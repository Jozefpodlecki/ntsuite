use nthook::NtHook;
use ntapi::{ntdef::*, ntioapi::{IO_STATUS_BLOCK, NtCreateFile}};


fn create_file() -> HANDLE {

    unsafe {
        // let path = r"C:\repos\ntsuite\test.txt";
        let path = r"\??\C:\repos\ntsuite\test.txt";
        let wide_path: Vec<u16> = path.encode_utf16().chain(Some(0)).collect();
        
        let mut uni = UNICODE_STRING {
            Length: (wide_path.len() * 2 - 2) as u16,
            MaximumLength: (wide_path.len() * 2) as u16,
            Buffer: wide_path.as_ptr() as *mut u16,
        };
        
        let mut attrs = core::mem::zeroed::<OBJECT_ATTRIBUTES>();
        attrs.Length = core::mem::size_of::<OBJECT_ATTRIBUTES>() as u32;
        attrs.ObjectName = &mut uni;
        attrs.Attributes = 0x40;

        let mut file_handle: HANDLE = core::ptr::null_mut();
        let mut io_status: IO_STATUS_BLOCK = core::mem::zeroed();
        
        let status = NtCreateFile(
            &mut file_handle,
            FILE_GENERIC_WRITE | SYNCHRONIZE | FILE_READ_ATTRIBUTES,
            &mut attrs,
            &mut io_status,
            core::ptr::null_mut(),
            FILE_ATTRIBUTE_NORMAL,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            OPEN_EXISTING,
            FILE_SYNCHRONOUS_IO_NONALERT | FILE_NON_DIRECTORY_FILE,
            core::ptr::null_mut(),
            0,
        );
        
        println!("NtCreateFile status: 0x{:X}", status);
        println!("File handle: 0x{:X}", file_handle as u64);
        println!("IO Status: 0x{:X}", io_status.u.Status);
        
        // if status >= 0 {
        //     NtClose(file_handle);
        //     println!("File created: {path}");
        // }

        file_handle
    }
}

fn main() {
    NtHook::hook_current_process();

    create_file();
}