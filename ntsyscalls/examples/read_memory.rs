use ntapi::ntdef::HANDLE;
use ntapi::ntmmapi::{MEMORY_BASIC_INFORMATION, MEMORY_INFORMATION_CLASS};
use ntapi::ntstatus::STATUS_SUCCESS;
use ntsyscalls::v10_0_26100_8521::NtQueryVirtualMemory;
use std::ptr;

pub const NT_CURRENT_PROCESS: HANDLE = usize::MAX as _;

fn main() {
    unsafe {
        let process_handle = NT_CURRENT_PROCESS;
        let mut buffer = [0u8; 4096];
        let mut bytes_read: u64 = 0;
        
        let mut address: u64 = 0x10000;
        
        loop {
            let mut mbi: MEMORY_BASIC_INFORMATION = std::mem::zeroed();
            
            let status = NtQueryVirtualMemory(
                process_handle,
                address as _,
                MEMORY_INFORMATION_CLASS::MemoryBasicInformation,
                &mut mbi as *mut _ as _,
                std::mem::size_of::<MEMORY_BASIC_INFORMATION>() as u64,
                ptr::null_mut(),
            );
            
            if status != STATUS_SUCCESS {
                break;
            }

            println!("BaseAddress: {:p}", mbi.BaseAddress);
            println!("AllocationBase: {:p}", mbi.AllocationBase);
            println!("AllocationProtect: 0x{:X}", mbi.AllocationProtect);
            println!("RegionSize: 0x{:X}", mbi.RegionSize);
            println!("State: 0x{:X}", mbi.State);
            println!("Protect: 0x{:X}", mbi.Protect);
            println!("Type: 0x{:X}", mbi.Type);
            println!("---");
            
            address = mbi.BaseAddress as u64 + mbi.RegionSize as u64;
            
            if address > 0x7FFFFFFF0000 {
                break;
            }
        }
    }
}