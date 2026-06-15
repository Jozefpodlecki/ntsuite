use ntapi::ntmmapi::*;
use ntapi::ntobapi::*;
use ntapi::ntpsapi::*;
use ntapi::ntdef::*;
use ntapi::ntrtl::CLIENT_ID;
use std::ptr;
use std::mem;

extern "system" fn thread_entry(param: *mut core::ffi::c_void) -> u32 {
    println!("Thread running! param: {:p}", param);
    0
}

fn main() {
    unsafe {
        let mut thread_handle: HANDLE = ptr::null_mut();
        let process_handle = usize::MAX as _;
        
        let desired_access = THREAD_ALL_ACCESS;
        let object_attributes: POBJECT_ATTRIBUTES = ptr::null_mut();
        let mut client_id: CLIENT_ID = mem::zeroed();
        
        let mut context: CONTEXT = mem::zeroed();
        context.ContextFlags = CONTEXT_FULL;
        context.Rip = thread_entry as u64;
        context.Rcx = 0;
        
        let mut stack_base: PVOID = ptr::null_mut();
        let mut stack_size: SIZE_T = 0x10000;
        
        let status = NtAllocateVirtualMemory(
            process_handle,
            &mut stack_base,
            0,
            &mut stack_size,
            MEM_COMMIT,
            PAGE_READWRITE,
        );
        
        if status != STATUS_SUCCESS {
            println!("Failed to allocate stack: 0x{:08X}", status);
            return;
        }
        
        context.Rsp = (stack_base as u64) + stack_size;
        
        let initial_teb: PINITIAL_TEB = ptr::null_mut();
        
        let status = NtCreateThread(
            &mut thread_handle,
            desired_access,
            object_attributes,
            process_handle,
            &mut client_id,
            &mut context,
            initial_teb,
            0,
        );
        
        if status == STATUS_SUCCESS {
            println!("Thread created successfully: {:p}", thread_handle);
            
            NtWaitForSingleObject(thread_handle, 0, ptr::null_mut());
            
            NtClose(thread_handle);
        } else {
            println!("NtCreateThread failed with status: 0x{:X}", status);
        }
        
        NtFreeVirtualMemory(process_handle, &mut stack_base, &mut stack_size, MEM_RELEASE);
    }
}