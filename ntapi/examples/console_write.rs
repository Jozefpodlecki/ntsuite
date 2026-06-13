#![feature(naked_functions_rustic_abi)]

use ntapi::{ntdef::*, ntioapi::{IO_STATUS_BLOCK, NtDeviceIoControlFile}};

const IOCTL_CONSOLE_WRITE: ULONG = 0x500016;

#[unsafe(naked)]
pub unsafe fn is_console_attached() -> bool {
    core::arch::naked_asm!(
        "mov rax, gs:[0x60]",
        "mov rax, [rax + 0x20]",
        "cmp qword ptr [rax + 0x10], 0",
        "setne al",
        "ret"
    );
}

#[repr(C)]
struct CONSOLE_WRITE_INPUT {
    pub Unknown0: ULONG,
    pub Field08: ULONG, 
    pub Field0C: ULONG, 
    pub Field10: ULONG, 
    pub BufferPtr: *mut u8,
    pub BufferLen: ULONG, 
}

pub fn write_console_internal(
    handle: HANDLE,
    buffer: *const u16,
    chars_to_write: u32,
    chars_written: *mut u32,
    is_unicode: bool,
) -> NTSTATUS {
    unsafe {
        let mut io_status_block: IO_STATUS_BLOCK = core::mem::zeroed();
        
        let actual_chars = if is_unicode {
            chars_to_write * 2
        } else {
            chars_to_write
        };
        
        if handle.is_null() || buffer.is_null() || chars_to_write == 0 {
            return 0xC0000008u32 as i32;
        }
        
        if !is_console_attached() {
            return 0xC0000008u32 as i32;
        }
        
        let mut input = core::mem::zeroed::<CONSOLE_WRITE_INPUT>();
        input.Unknown0 = 0x1000006;
        input.Field08 = 2;
        input.Field0C = 1;
        input.Field10 = 0x10;
        input.BufferPtr = buffer as *mut u8;
        input.BufferLen = actual_chars;
        
        let status = NtDeviceIoControlFile(
            handle,
            core::ptr::null_mut(),
            None,
            core::ptr::null_mut(),
            &mut io_status_block,
            IOCTL_CONSOLE_WRITE,
            &raw mut input as PVOID,
            core::mem::size_of::<CONSOLE_WRITE_INPUT>() as ULONG,
            core::ptr::null_mut(),
            0,
        );
        
        if status >= 0 && !chars_written.is_null() {
            let bytes_written = io_status_block.Information as u32;
            *chars_written = if is_unicode {
                bytes_written / 2
            } else {
                bytes_written
            };
        }
        
        println!("status {:X}", status);
        status
    }
}

pub fn write_console_w(
    handle: HANDLE,
    buffer: *const u16,
    chars_to_write: u32,
    chars_written: *mut u32,
    _reserved: *mut core::ffi::c_void,
) -> bool {
    unsafe {
        let status = write_console_internal(handle, buffer, chars_to_write, chars_written, true);
        status >= 0
    }
}

#[unsafe(naked)]
pub unsafe fn get_std_handle(n_std_handle: u32) -> HANDLE {
    core::arch::naked_asm!(
        "cmp ecx, 0xFFFFFFF6",
        "jz 1f",
        "cmp ecx, 0xFFFFFFF5",
        "jnz 2f",
        "mov rax, gs:[0x60]",
        "mov rax, [rax + 0x20]",
        "test dword ptr [rax + 0xA4], 0x400",
        "jnz 3f",
        "mov rax, gs:[0x60]",
        "mov rax, [rax + 0x20]",
        "mov rbx, [rax + 0x28]",
        "jmp 4f",
        "1:",
        "mov rax, gs:[0x60]",
        "mov rax, [rax + 0x20]",
        "test dword ptr [rax + 0xA4], 0x200",
        "jnz 3f",
        "mov rax, gs:[0x60]",
        "mov rax, [rax + 0x20]",
        "mov rbx, [rax + 0x20]",
        "jmp 4f",
        "2:",
        "cmp ecx, 0xFFFFFFF4",
        "jnz 5f",
        "mov rax, gs:[0x60]",
        "mov rax, [rax + 0x20]",
        "mov rbx, [rax + 0x30]",
        "jmp 4f",
        "3:",
        "xor ebx, ebx",
        "jmp 4f",
        "4:",
        "cmp rbx, -1",
        "jnz 6f",
        "mov ecx, 0xC0000008",
        "call BaseSetLastNTError",
        "6:",
        "mov rax, rbx",
        "ret",
        "5:",
        "mov rbx, -1",
        "mov ecx, 0xC0000008",
        "call BaseSetLastNTError",
        "mov rax, rbx",
        "ret",
    );
}

pub const STD_OUTPUT_HANDLE: u32 = 0xFFFFFFF5u32;
pub const STD_INPUT_HANDLE: u32 = 0xFFFFFFF6u32;
pub const STD_ERROR_HANDLE: u32 = 0xFFFFFFF4u32;

fn main() {
    unsafe {
        let handle = get_std_handle(STD_OUTPUT_HANDLE);
        let msg = "Hello, World!\r\n";
        let mut utf16: [u16; 256] = [0; 256];
        let mut len = 0;
        
        for (i, ch) in msg.encode_utf16().enumerate() {
            utf16[i] = ch;
            len = i + 1;
        }
        
        let mut written = 0;
        write_console_w(handle, utf16.as_ptr(), len as u32, &mut written, core::ptr::null_mut());
    }
}