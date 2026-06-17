#![feature(naked_functions_rustic_abi)]
#![no_std]

mod api;

use anyhow::Result;
use ntapi::{ntdef::HANDLE, ntmmapi::NtReadVirtualMemory, ntrtl::IMAGE_NT_HEADERS64};
use pelite::image::IMAGE_EXPORT_DIRECTORY;
use tools::*;
use crate::api::open_process;

pub const NT_CURRENT_PROCESS: HANDLE = usize::MAX as _;

#[repr(C)]
pub struct IMAGE_DOS_HEADER {
    pub e_magic: u16,
    pub e_cblp: u16,
    pub e_cp: u16,
    pub e_crlc: u16,
    pub e_cparhdr: u16,
    pub e_minalloc: u16,
    pub e_maxalloc: u16,
    pub e_ss: u16,
    pub e_sp: u16,
    pub e_csum: u16,
    pub e_ip: u16,
    pub e_cs: u16,
    pub e_lfarlc: u16,
    pub e_ovno: u16,
    pub e_res: [u16; 4],
    pub e_oemid: u16,
    pub e_oeminfo: u16,
    pub e_res2: [u16; 10],
    pub e_lfanew: i32,
}

pub const IMAGE_DOS_SIGNATURE: u16 = 0x5A4D;
pub const IMAGE_NT_SIGNATURE: u32 = 0x00004550;

#[repr(C)]
pub struct IMAGE_FILE_HEADER {
    pub Machine: u16,
    pub NumberOfSections: u16,
    pub TimeDateStamp: u32,
    pub PointerToSymbolTable: u32,
    pub NumberOfSymbols: u32,
    pub SizeOfOptionalHeader: u16,
    pub Characteristics: u16,
}

#[repr(C)]
pub struct IMAGE_DATA_DIRECTORY {
    pub VirtualAddress: u32,
    pub Size: u32,
}

pub const IMAGE_NUMBEROF_DIRECTORY_ENTRIES: usize = 16;
pub const IMAGE_DIRECTORY_ENTRY_EXPORT: usize = 0;

pub fn is_system_call() -> bool {

}

pub struct NtHook;

impl NtHook {
    pub fn hook_current_process() -> Result<()> {
        let ntdll_base = Self::get_ntdll_base();
        
        unsafe {
            let dos_header = (ntdll_base as *const IMAGE_DOS_HEADER).as_ref().unwrap();
            if dos_header.e_magic != IMAGE_DOS_SIGNATURE {
                panic!("Invalid DOS header");
            }
            
            let nt_headers = (ntdll_base as usize + dos_header.e_lfanew as usize) as *const IMAGE_NT_HEADERS64;
            let nt = nt_headers.as_ref().unwrap();
            if nt.Signature != IMAGE_NT_SIGNATURE {
                panic!("Invalid NT signature");
            }
            
            let export_rva = nt.OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_EXPORT].VirtualAddress;
            if export_rva == 0 {
                panic!("No export directory");
            }
            
            let export_dir = (ntdll_base as usize + export_rva as usize) as *const IMAGE_EXPORT_DIRECTORY;
            let exports = export_dir.as_ref().unwrap();

            let names_rva = exports.AddressOfNames;
            let ordinals_rva = exports.AddressOfNameOrdinals;
            let functions_rva = exports.AddressOfFunctions;
            
            let names = (ntdll_base as usize + names_rva as usize) as *const u32;
            let ordinals = (ntdll_base as usize + ordinals_rva as usize) as *const u16;
            let functions = (ntdll_base as usize + functions_rva as usize) as *const u32;
            let mut console_mut = NtConsole::new();

            for i in 0..exports.NumberOfNames {
                let name_rva = *names.add(i as usize);
                let name_ptr = (ntdll_base as usize + name_rva as usize) as *const u8;
                let name = AStackString::<260>::from_ptr(name_ptr).unwrap();

                console_mut.writeln(name.as_str()).map_err(|status| anyhow::anyhow!("Error {status}"))?;


                
                // let ordinal = *ordinals.add(i as usize);
                // let function_rva = *functions.add(ordinal as usize);
                
                // if function_rva != 0 {
                //     let function_addr = ntdll_base as usize + function_rva as usize;
                //     core::mem::forget(name);
                // }
            }
        }
        
        Ok(())
    }

    fn get_ntdll_base() -> usize {
        let handle = usize::MAX as _;
        
        for module in MappedFileIterator::new(handle) {
            if module.name_contains_utf8(b"ntdll.dll") {
                return module.base_address();
            }
        }
        
        panic!("Could not find ntdll base?")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterator_finds_ntdll() {
        NtHook::hook_current_process().unwrap();
    }
}