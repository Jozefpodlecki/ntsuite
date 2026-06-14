use std::{fs::{self, File}, io::Read, path::Path};
use codegen::Scope;
use flexi_logger::{Duplicate, FileSpec, Logger};
use iced_x86::{Code, DecoderOptions};
use log::*;
use pdb::{FallibleIterator, PDB, SymbolData};
use pelite::pe::{Pe, PeFile, debug::{CodeView, Entry}, exports::Export};
use anyhow::{Result, bail};
use symsrv::SymsrvDownloader;

use crate::{categorizer::categorize_functions, code_generator::generate_code, pdb_extractor::{build_rva_to_function_map, download_ntdll_pdb}, pe_parser::load_pe_file};

mod types;
mod categorizer;
mod pe_parser;
mod pdb_extractor;
mod code_generator;

#[tokio::main]
async fn main() -> Result<()> {
    init_logger()?;
    
    let ntdll_path = r#"C:\Windows\System32\ntdll.dll"#;
    let pe_data = load_pe_file(ntdll_path)?;
    let pdb_path = download_ntdll_pdb(&pe_data).await?;
    
    let functions = build_rva_to_function_map(&pdb_path)?;
    let categorized = categorize_functions(&pe_data, functions)?;
    
    generate_code(categorized)?;
    
    Ok(())
}

fn init_logger() -> Result<()> {
    let logger = Logger::try_with_str("info")?
        .log_to_file(FileSpec::default())
        .duplicate_to_stdout(Duplicate::All);
    logger.start()?;
    Ok(())
}

// fn extract_pdb_signature(pe: &PeFile<'_>) -> Result<(String, String)> {
//     let debug = pe.debug()?;
    
//     for dir in debug.iter() {
//         let entry = dir.entry()?;
        
//         if let Entry::CodeView(cv) = entry {
//             let pdb_filename = cv.pdb_file_name().to_str()?.to_string();
//             let format = cv.format();
//             let age = cv.age();
            
//             let signature = match format {
//                 "RSDS" => {
//                     // For Cv70 (modern PDBs), we need the GUID
//                     if let CodeView::Cv70 { image, .. } = cv {
//                         // image.Signature is [u8; 16] - the GUID
//                         let guid_hex: String = image.Signature.to_string();
//                         format!("{}{:X}", guid_hex, age)
//                     } else {
//                         anyhow::bail!("Expected Cv70 format")
//                     }
//                 },
//                 "NB10" => {
//                     if let CodeView::Cv20 { image, .. } = cv {
//                         format!("{:08X}{:X}", image.TimeDateStamp, age)
//                     } else {
//                         anyhow::bail!("Expected Cv20 format")
//                     }
//                 },
//                 _ => anyhow::bail!("Unknown CodeView format: {}", format),
//             };
            
//             return Ok((signature, pdb_filename));
//         }
//     }
    
//     anyhow::bail!("No CodeView debug entry found")
// }

// fn build_rva_to_function_map(pdb_path: &Path) -> Result<Vec<(u32, String)>> {
//     let file = File::open(pdb_path)?;
//     let mut pdb = PDB::open(file)?;
    
//     let symbol_table = pdb.global_symbols()?;
//     let address_map = pdb.address_map()?;
    
//     let mut functions = Vec::new();
//     let mut symbols = symbol_table.iter();
    
//     while let Some(symbol) = symbols.next()? {
//         match symbol.parse() {
//             Ok(SymbolData::Public(data)) if data.function => {
//                 if let Some(rva) = data.offset.to_rva(&address_map) {
//                     functions.push((rva.0, data.name.to_string().to_string()));
//                 }
//             }
//             Ok(SymbolData::Procedure(data)) => {
//                 if let Some(rva) = data.offset.to_rva(&address_map) {
//                     functions.push((rva.0, data.name.to_string().to_string()));
//                 }
//             }
//             _ => {}
//         }
//     }
    
//     Ok(functions)
// }

// #[tokio::main]
// async fn main() -> Result<()> {
//     let mut logger = Logger::try_with_str("debug")?;
//     logger = logger.log_to_file(FileSpec::default());
//     logger = logger.duplicate_to_stdout(Duplicate::All);
//     logger.start()?;

//     let symbol_path_env = symsrv::get_symbol_path_from_environment();
//     let symbol_path =
//     symbol_path_env.as_deref().unwrap_or("srv**https://msdl.microsoft.com/download/symbols");
//     let parsed_symbol_path = symsrv::parse_nt_symbol_path(symbol_path);

//     let mut downloader = SymsrvDownloader::new(parsed_symbol_path);
//     downloader.set_default_downstream_store(symsrv::get_home_sym_dir());

//     let path = r#"C:\Windows\System32\ntdll.dll"#;
    
//     let mut file = std::fs::File::open(&path)?;
//     let mut bytes = Vec::new();
//     file.read_to_end(&mut bytes)?;
//     let file = PeFile::from_bytes(&bytes)?;
//     let text_section = file.section_headers().by_name(".text").unwrap();
//     let bytes = file.get_section_bytes(text_section)?;

//     let (signature, filename) = extract_pdb_signature(&file)?;

//     let clean_signature = signature
//         .replace('{', "")
//         .replace('}', "")
//         .replace('-', "")
//         .to_uppercase(); 

//     let pdb_path = downloader.get_file("ntdll.pdb", &clean_signature).await?;
//     let pdb_functions = build_rva_to_function_map(&pdb_path)?;

//     let mut functions = vec![];

//     for (rva, name) in pdb_functions {
//         // info!("{:X} {}", rva, func);
//         let function = NtFunction {
//             name: name.to_string(),
//             category: get_category(bytes, rva, name),
//             size: get_size(bytes, rva),
//             rva: rva
//         };
//         functions.push(function);
//     }

//     let mut scope = Scope::new();

//     for function in functions {
//         match function.category {
//             NtFunctionCategory::Syscall(code) => {
//                 let func = scope.new_fn(function.name);
//                 func.line("");
//             },
//             _ => continue,
//         }
//     }

//     let gen_path = "generated.rs";
//     fs::write(gen_path, scope.to_string());

//     // let exports = file.exports()?;
//     // let by = exports.by()?;
    
//     // for (name_result, export_result) in by.iter_names() {
//     //     if let (Ok(name), Ok(export)) = (name_result, export_result) {
//     //         let name_str = name.to_str().unwrap_or("<invalid UTF-8>");
//     //         match export {
//     //             Export::Symbol(rva) => {
//     //                 info!("{} -> RVA: 0x{:08X}", name_str, rva);
//     //             }
//     //             Export::Forward(fwd) => {
//     //                 if let Ok(fwd_str) = fwd.to_str() {
//     //                     info!("{} -> Forwarded to {}", name_str, fwd_str);
//     //                 }
//     //             }
//     //         }
//     //     }
//     // }

//     // let exception = file.exception()?;

//     // for runtime_function in exception.functions() {
//     //     let runtime_function = runtime_function.image();
//     //     let start_rva = runtime_function.BeginAddress;
//     //     let end_rva = runtime_function.EndAddress;
//     //     let unwind_rva = runtime_function.UnwindData;
        
//     //     let syscall_num = extract_syscall_number(&bytes, start_rva);
        
//     //     info!("Function at 0x{:08X}-0x{:08X} Unwind: 0x{:08X} Syscall: 0x{:X} ({})", 
//     //           start_rva, end_rva, unwind_rva, syscall_num, syscall_num);
//     // }

//     Ok(())
// }

// fn get_category(bytes: &[u8], rva: u32, name: &str) -> Result<NtFunctionCategory> {

//     if !name.contains("Nt") && !name.contains("Zw") {
//         return Ok(NtFunctionCategory::Other)
//     }

//     let mut decoder = iced_x86::Decoder::new(64, bytes, DecoderOptions::NONE);
//     let mut code = None;
//     let mut is_syscall = false;

//     loop {
//         let instr = decoder.decode();

//         if instr.is_invalid() {
//             bail!("Invalid")
//         }

//         if instr.code() == Code::Mov_AL_moffs8 { //  i.e  mov     eax, 0ECh
//             // get sys code
//             code = Some(123);
//         }

//         if instr.code() == Code::Syscall {
//             // definitely syscall
//             is_syscall = true;
//         }

//         if instr.code() == Code::Retnw
//             || instr.code() == Code::Retnq
//             || instr.code() == Code::Retnd {
//             break;
//         }
//     }

//     match is_syscall {
//         true => Ok(NtFunctionCategory::Syscall(code.unwrap())),
//         false => Ok(NtFunctionCategory::Other),
//     }
// }

// fn get_size(bytes: &[u8], rva: u32) -> Result<u32> {
//     let mut decoder = iced_x86::Decoder::new(64, bytes, DecoderOptions::NONE);
//     let mut size = 0;

//     loop {
//         let instr = decoder.decode();

//         if instr.is_invalid() {
//             bail!("Invalid")
//         }

//         if instr.code() == Code::Retnw
//             || instr.code() == Code::Retnq
//             || instr.code() == Code::Retnd {
//             break;
//         }

//         size += instr.len() as u32;
//     }

//     Ok(size)
// }