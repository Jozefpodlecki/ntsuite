use std::fs::File;
use std::path::Path;
use pdb::{FallibleIterator, PDB, SymbolData};
use pelite::pe::{Pe, PeFile};
use pelite::pe::debug::{Entry, CodeView};
use anyhow::Result;
use symsrv::SymsrvDownloader;

pub async fn download_ntdll_pdb(pe_data: &[u8]) -> Result<std::path::PathBuf> {
    let (signature, filename) = extract_pdb_signature(pe_data)?;
    let clean_signature = signature
        .replace('{', "")
        .replace('}', "")
        .replace('-', "")
        .to_uppercase();
    
    let symbol_path = "srv**https://msdl.microsoft.com/download/symbols";
    let parsed = symsrv::parse_nt_symbol_path(symbol_path);
    let mut downloader = SymsrvDownloader::new(parsed);
    downloader.set_default_downstream_store(symsrv::get_home_sym_dir());
    
    downloader.get_file(&filename, &clean_signature).await.map_err(Into::into)
}

pub fn extract_pdb_signature(pe_bytes: &[u8]) -> Result<(String, String)> {
    let pe = PeFile::from_bytes(pe_bytes)?;
    let debug = pe.debug()?;
    
    for dir in debug.iter() {
        let entry = dir.entry()?;
        
        if let Entry::CodeView(cv) = entry {
            let pdb_filename = cv.pdb_file_name().to_str()?.to_string();
            let format = cv.format();
            let age = cv.age();
            
            let signature = match format {
                "RSDS" => {
                    if let CodeView::Cv70 { image, .. } = cv {
                        image.Signature.to_string()
                    } else {
                        anyhow::bail!("Expected Cv70 format")
                    }
                },
                "NB10" => {
                    if let CodeView::Cv20 { image, .. } = cv {
                        format!("{:08X}{:X}", image.TimeDateStamp, age)
                    } else {
                        anyhow::bail!("Expected Cv20 format")
                    }
                },
                _ => anyhow::bail!("Unknown CodeView format: {}", format),
            };
            
            return Ok((signature, pdb_filename));
        }
    }
    
    anyhow::bail!("No CodeView debug entry found")
}

pub fn build_rva_to_function_map(pdb_path: &Path) -> Result<Vec<(u32, String)>> {
    let file = File::open(pdb_path)?;
    let mut pdb = PDB::open(file)?;
    
    let symbol_table = pdb.global_symbols()?;
    let address_map = pdb.address_map()?;
    
    let mut functions = Vec::new();
    let mut symbols = symbol_table.iter();
    
    while let Some(symbol) = symbols.next()? {
        match symbol.parse() {
            Ok(SymbolData::Public(data)) if data.function => {
                if let Some(rva) = data.offset.to_rva(&address_map) {
                    functions.push((rva.0, data.name.to_string().to_string()));
                }
            }
            Ok(SymbolData::Procedure(data)) => {
                if let Some(rva) = data.offset.to_rva(&address_map) {
                    functions.push((rva.0, data.name.to_string().to_string()));
                }
            }
            _ => {}
        }
    }
    
    Ok(functions)
}