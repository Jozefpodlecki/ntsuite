use std::io::Read;

use anyhow::Result;
use pelite::{pe::{Pe, PeFile}, resources::version_info::VersionInfo};

pub fn load_pe_file(path: &str) -> Result<Vec<u8>> {
    let mut file = std::fs::File::open(path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(bytes)
}

pub fn extract_ntdll_version(pe_data: &[u8]) -> Result<String> {
    let pe = PeFile::from_bytes(pe_data)?;
    let resources = pe.resources()?;
    let version_info = resources.version_info()?;
    
    let fixed = version_info.fixed().ok_or_else(|| anyhow::anyhow!("No fixed version info found"))?;
    
    let version = format!("{}_{}_{}_{}", 
        fixed.dwFileVersion.Major,
        fixed.dwFileVersion.Minor, 
        fixed.dwFileVersion.Patch,
        fixed.dwFileVersion.Build
    );
    
    Ok(version)
}

pub fn parse_pe_file(bytes: &[u8]) -> Result<PeFile<'_>> {
    PeFile::from_bytes(bytes).map_err(Into::into)
}

pub fn extract_text_section(pe_data: &[u8]) -> Result<(&[u8], u32)> {
    let pe = PeFile::from_bytes(pe_data)?;
    let text_section = pe.section_headers()
        .by_name(".text")
        .ok_or_else(|| anyhow::anyhow!("No .text section found"))?;
    
    let bytes = pe.get_section_bytes(text_section)?;
    let rva = text_section.VirtualAddress;
    
    Ok((bytes, rva))
}