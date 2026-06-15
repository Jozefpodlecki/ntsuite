use std::fs::File;
use std::path::Path;
use pdb::{FallibleIterator, PDB, SymbolData, TypeData, TypeIndex};
use pelite::pe::{Pe, PeFile};
use pelite::pe::debug::{Entry, CodeView};
use anyhow::Result;
use symsrv::SymsrvDownloader;

use crate::types::{Function, FunctionCategory};

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

fn extract_pdb_signature(pe_bytes: &[u8]) -> Result<(String, String)> {
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
                        format!("{}{:X}", image.Signature.to_string(), age)
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

pub fn build_functions(pdb_path: &Path) -> Result<Vec<Function>> {
    let file = File::open(pdb_path)?;
    let mut pdb = PDB::open(file)?;
    
    let symbol_table = pdb.global_symbols()?;
    let address_map = pdb.address_map()?;
    let type_info = pdb.type_information()?;
    let mut type_finder = type_info.finder();
    
    let mut type_iter = type_info.iter();
    while let Some(ty) = type_iter.next()? {
        type_finder.update(&type_iter);
    }
    
    let mut functions = Vec::new();
    let mut symbols = symbol_table.iter();
    
    while let Some(symbol) = symbols.next()? {

        let symbol_data = match symbol.parse() {
            Ok(value) => value,
            Err(err) => {
                println!("{}", err);
                continue;
            },
        };

        let (name, rva, return_type, parameters) = match symbol_data {
            SymbolData::Public(data) if data.function => {
                let rva = data.offset.to_rva(&address_map).map(|r| r.0);
                (data.name.to_string().to_string(), rva, None, Vec::new())
            }
            SymbolData::Procedure(data) => {
                let rva = data.offset.to_rva(&address_map).map(|r| r.0);
                let ty = type_finder.find(data.type_index).ok();
                let (ret, params) = ty.map_or((None, Vec::new()), |t| parse_signature(&type_finder, &t));
                (data.name.to_string().to_string(), rva, ret, params)
            }
            _ => {
                continue;
            },
        };
        
        if let Some(rva) = rva {
            functions.push(Function {
                name,
                rva,
                size: 0,
                category: FunctionCategory::Other,
                return_type,
                parameter_types: parameters,
            });
        }
    }
    
    Ok(functions)
}

fn parse_signature(
    type_finder: &pdb::TypeFinder<'_>,
    ty: &pdb::Type<'_>,
) -> (Option<String>, Vec<String>) {
    let type_data = match ty.parse() {
        Ok(TypeData::Procedure(proc)) => proc,
        _ => return (None, Vec::new()),
    };
    
    let return_type = type_data.return_type
        .and_then(|idx| resolve_type(type_finder, idx));
    
    let parameters = resolve_parameters(type_finder, type_data.argument_list);
    
    (return_type, parameters)
}

fn resolve_parameters(
    type_finder: &pdb::TypeFinder<'_>,
    arg_list_idx: TypeIndex,
) -> Vec<String> {
    let arg_list = match type_finder.find(arg_list_idx).and_then(|t| t.parse()) {
        Ok(TypeData::ArgumentList(args)) => args,
        _ => return Vec::new(),
    };
    
    arg_list.arguments
        .iter()
        .filter_map(|idx| resolve_type(type_finder, *idx))
        .collect()
}

fn resolve_type(type_finder: &pdb::TypeFinder<'_>, type_index: TypeIndex) -> Option<String> {
    let ty = type_finder.find(type_index).ok()?;
    let type_data = ty.parse().ok()?;
    
    match type_data {
        TypeData::Primitive(prim) => Some(format!("{:?}", prim)),
        TypeData::Pointer(ptr) => {
            let inner = resolve_type(type_finder, ptr.underlying_type);
            inner.map(|name| format!("{}*", name)).or(Some("void*".to_string()))
        }
        TypeData::Modifier(modifier) => {
            let inner = resolve_type(type_finder, modifier.underlying_type);
            let prefix = if modifier.constant { "const " } else { "" };
            inner.map(|name| format!("{}{}", prefix, name))
        }
        TypeData::Class(class) => Some(class.name.to_string().to_string()),
        TypeData::Enumeration(en) => Some(en.name.to_string().to_string()),
        _ => None,
    }
}