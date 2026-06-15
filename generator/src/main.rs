use flexi_logger::{Duplicate, FileSpec, Logger};
use anyhow::{Result, bail};

use crate::{categorizer::*, code_generator::*, pdb_extractor::*, pe_parser::*};

mod types;
mod categorizer;
mod pe_parser;
mod pdb_extractor;
mod code_generator;
mod data;

#[tokio::main]
async fn main() -> Result<()> {
    init_logger()?;
    
    let ntdll_path = r#"C:\Windows\System32\ntdll.dll"#;
    let pe_data = load_pe_file(ntdll_path)?;
    let pdb_path = download_ntdll_pdb(&pe_data).await?;
    
    let functions = build_functions(&pdb_path)?;
    let categorized = categorize_functions(&pe_data, functions)?;
    let version = extract_ntdll_version(&pe_data)?;

    generate_code(version, categorized)?;
    
    Ok(())
}

fn init_logger() -> Result<()> {
    let logger = Logger::try_with_str("info")?
        .log_to_file(FileSpec::default())
        .duplicate_to_stdout(Duplicate::All);
    logger.start()?;
    Ok(())
}