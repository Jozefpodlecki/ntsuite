#![allow(non_camel_case_types)]
#![allow(unused)]
#![feature(allocator_api)]
#![feature(naked_functions_rustic_abi)]
#![no_std]

mod nt_allocator;
mod rtl_allocator;

pub use nt_allocator::*;
pub use rtl_allocator::*;