use ntapi::ntdef::*;
use ntapi::ntobapi::*;
use ntapi::ntrtl::*;
use ntapi::ntstatus::STATUS_SUCCESS;
use core::ptr;
use core::mem;

pub fn query_dos_device(device_name: &str) -> Option<String> {

    let full_path = format!("\\??\\{}", device_name);
    let wide_path: Vec<u16> = full_path.encode_utf16().chain(Some(0)).collect();

    let mut path_string: UNICODE_STRING = unsafe { mem::zeroed() };
    unsafe {
        RtlInitUnicodeString(&mut path_string, wide_path.as_ptr());
    }

    let mut symlink_handle: HANDLE = ptr::null_mut();
    let mut obj_attr: OBJECT_ATTRIBUTES = unsafe { mem::zeroed() };
    obj_attr.Length = mem::size_of::<OBJECT_ATTRIBUTES>() as u32;
    obj_attr.ObjectName = &mut path_string;
    obj_attr.Attributes = OBJ_CASE_INSENSITIVE;

    let status = unsafe {
        NtOpenSymbolicLinkObject(
            &mut symlink_handle,
            SYMBOLIC_LINK_QUERY,
            &mut obj_attr,
        )
    };
    if status != STATUS_SUCCESS {
        return None;
    }

    let mut target: UNICODE_STRING = unsafe { mem::zeroed() };
    let mut buffer = [0u16; 1024];
    target.Buffer = buffer.as_mut_ptr();
    target.MaximumLength = buffer.len() as u16;
    target.Length = 0;

    let mut returned_length: u32 = 0;
    let status = unsafe {
        NtQuerySymbolicLinkObject(
            symlink_handle,
            &mut target,
            &mut returned_length,
        )
    };
    unsafe { NtClose(symlink_handle); }

    if status != STATUS_SUCCESS {
        return None;
    }

    let slice = unsafe {
        core::slice::from_raw_parts(
            target.Buffer,
            (target.Length / 2) as usize,
        )
    };
    Some(String::from_utf16_lossy(slice))
}

pub fn dos_to_device_path(dos_path: &str) -> Option<String> {
    if dos_path.len() < 2 || &dos_path[1..2] != ":" {
        return None;
    }
    let drive = &dos_path[0..2];
    let rest = &dos_path[2..];

    let device = query_dos_device(drive)?;
    Some(format!("{}{}", device, rest))
}

fn main() {
    let device = query_dos_device("C:").unwrap();
    println!("C: -> {:?}", device);
}