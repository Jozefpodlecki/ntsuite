#![feature(naked_functions_rustic_abi)]

use ntalloc::*;

#[unsafe(naked)]
pub unsafe fn get_process_heap() -> *mut std::ffi::c_void {
    core::arch::naked_asm!(
        "mov rax, gs:[0x60]",
        "mov rax, [rax + 0x30]",
        "ret"
    );
}

#[global_allocator]
pub static GLOBAL_ALLOCATOR: RtlAlloc = RtlAlloc;
// pub static GLOBAL_ALLOCATOR: NtAlloc = NtAlloc;

fn main() {
    unsafe {
        // RtlAlloc::init();
        // let answer = Box::new(42);
        // let answer = Box::new([0; 42]);

        let heap = get_process_heap();
        // println!("{:X}", heap as usize);
    }
}