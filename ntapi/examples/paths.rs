use core::ptr;
use core::mem;
use ntapi::{ntdef::UNICODE_STRING, ntrtl::*};


fn dos_to_nt_path(dos_path: &str) -> Option<UNICODE_STRING> {
    let wide_path: Vec<u16> = dos_path.encode_utf16().chain(Some(0)).collect();
    
    let mut nt_path: UNICODE_STRING = unsafe { mem::zeroed() };
    let mut file_part: *mut u16 = ptr::null_mut();
    
    unsafe {
        let result = RtlDosPathNameToNtPathName_U(
            wide_path.as_ptr(),
            &mut nt_path,
            &mut file_part,
            ptr::null_mut(),
        );
        
        if result != 0 {
            Some(nt_path)
        } else {
            None
        }
    }
}

fn nt_to_dos_path(nt_path_str: &str) -> Option<String> {
    let wide_path: Vec<u16> = nt_path_str.encode_utf16().chain(Some(0)).collect();
    
    let mut buffer: RTL_UNICODE_STRING_BUFFER = unsafe { mem::zeroed() };
    buffer.String.Buffer = wide_path.as_ptr() as *mut u16;
    buffer.String.Length = (wide_path.len() * 2 - 2) as u16;
    buffer.String.MaximumLength = (wide_path.len() * 2) as u16;
    
    let mut disposition: u32 = 0;
    let mut file_part: *mut u16 = ptr::null_mut();
    
    unsafe {
        let status = RtlNtPathNameToDosPathName(
            0,
            &mut buffer,
            &mut disposition,
            &mut file_part,
        );
        
        if status == 0 {
            let slice = std::slice::from_raw_parts(
                buffer.String.Buffer,
                (buffer.String.Length / 2) as usize,
            );
            let result = String::from_utf16_lossy(slice);
            Some(result)
        } else {
            None
        }
    }
}

fn main() {
    let dos = r"C:\Windows\System32";
    
    if let Some(mut nt_path) = dos_to_nt_path(dos) {
        let slice = unsafe {
            std::slice::from_raw_parts(
                nt_path.Buffer,
                (nt_path.Length / 2) as usize,
            )
        };
        let nt_path_str = String::from_utf16_lossy(slice);
        println!("NT Path: {}", nt_path_str);

        let device_path = nt_path_str.trim_start_matches(r"\??\");
        println!("Device Path: {}", device_path);
        
        if let Some(converted_back) = nt_to_dos_path(device_path) {
            println!("Converted back to DOS: {}", converted_back);
        } else {
            println!("Failed to convert back");
        }

        unsafe {
            RtlFreeUnicodeString(&mut nt_path);
        }
    } else {
        println!("Failed to convert path");
    }
}