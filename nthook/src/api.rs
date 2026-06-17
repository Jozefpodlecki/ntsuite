use anyhow::{Result, bail};
use ntapi::{ntdef::{ACCESS_MASK, HANDLE, OBJECT_ATTRIBUTES}, ntpsapi::NtOpenProcess, ntrtl::CLIENT_ID};

#[unsafe(naked)]
pub unsafe fn get_current_process_id() -> u32 {
    core::arch::naked_asm!(
        "mov rax, gs:[0x30]",
        "mov eax, [rax + 0x40]",
        "ret"
    );
}

pub fn open_process(pid: u32, desired_access: ACCESS_MASK) -> Result<HANDLE> {
    unsafe {
        let mut handle = core::ptr::null_mut();
        let mut attrs: OBJECT_ATTRIBUTES = core::mem::zeroed();
        let mut client_id: CLIENT_ID = core::mem::zeroed();

        client_id.UniqueProcess = pid as _;
        client_id.UniqueThread = core::ptr::null_mut();

        let status = NtOpenProcess(
            &mut handle,
            desired_access,
            &mut attrs,
            &mut client_id);

        if status < 0 {
            bail!("NtOpenProcess failed: {status:?}");
        }

        Ok(handle)
    }
}