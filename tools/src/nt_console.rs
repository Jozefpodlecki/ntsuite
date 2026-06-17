
use core::fmt::{self, Write};
use ntapi::{ntdef::*, ntioapi::{IO_STATUS_BLOCK, NtWriteFile}, ntpebteb::PEB};

use crate::U16CStackString;

#[unsafe(naked)]
pub unsafe fn get_peb() -> *mut PEB {
    core::arch::naked_asm!(
        "mov rax, gs:[0x60]",
        "ret"
    );
}

pub fn get_std_handle(n_std_handle: u32) -> HANDLE {
    unsafe {
        let peb_ptr = get_peb();
        let peb = &*peb_ptr;
        let process_params_ptr = peb.ProcessParameters;
        
        if process_params_ptr.is_null() {
            return core::ptr::null_mut();
        }

        let process_params = &*process_params_ptr;
        
        match n_std_handle {
            STD_INPUT_HANDLE => process_params.StandardInput,
            STD_OUTPUT_HANDLE => process_params.StandardOutput,
            STD_ERROR_HANDLE => process_params.StandardError,
            _ => core::ptr::null_mut(),
        }
    }
}

fn write_console_utf16_with_nt_write(
    handle: HANDLE,
    buffer: *const u16,
    chars_to_write: u32,
    chars_written: *mut u32,
) -> NTSTATUS {
    unsafe {
        let mut io_status_block: IO_STATUS_BLOCK = core::mem::zeroed();
        
        if handle.is_null() || buffer.is_null() || chars_to_write == 0 {
            return STATUS_INVALID_HANDLE;
        }
        
        let bytes_to_write = chars_to_write * 2;
        
        let status = NtWriteFile(
            handle,
            core::ptr::null_mut(),
            None,
            core::ptr::null_mut(),
            &mut io_status_block,
            buffer as PVOID,
            bytes_to_write,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        );
        
        if status >= 0 && !chars_written.is_null() {
            let bytes_written = io_status_block.Information as u32;
            *chars_written = bytes_written / 2;
        }
        
        status
    }
}

pub struct NtConsole {
    handle: HANDLE,
}

impl NtConsole {
    pub fn new() -> Self {
        Self {
            handle: get_std_handle(STD_OUTPUT_HANDLE),
        }
    }

    pub fn with_handle(handle: HANDLE) -> Self {
        Self { handle }
    }

    pub fn writeln(&self, text: &str) -> Result<u32, NTSTATUS> {
        let written = self.write(text)?;
        let newline_written = self.write("\r\n")?;
        Ok(written + newline_written)
    }

    pub fn write(&self, text: &str) -> Result<u32, NTSTATUS> {
        let utf16 = match U16CStackString::<260>::from_str(text) {
            Some(s) => s,
            None => return Err(STATUS_INVALID_HANDLE),
        };
        
        let mut written = 0;
        let status = unsafe {
            write_console_utf16_with_nt_write(
                self.handle,
                utf16.as_ptr(),
                utf16.len() as u32,
                &mut written,
            )
        };
        
        if status >= 0 {
            Ok(written)
        } else {
            Err(status)
        }
    }

}

impl Clone for NtConsole {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle,
        }
    }
}

impl Write for NtConsole {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s).map_err(|_| fmt::Error)?;
        Ok(())
    }
}