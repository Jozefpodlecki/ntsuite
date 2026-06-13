#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused)]

mod mapped_file_iter;

pub use mapped_file_iter::*;

pub mod ntdef;

#[cfg(feature = "ntmmapi")]
pub mod ntmmapi;

#[cfg(feature = "ntobapi")]
pub mod ntobapi;

#[cfg(feature = "ntkeapi")]
pub mod ntkeapi;

#[cfg(feature = "ntafd")]
pub mod ntafd;

#[cfg(feature = "ntioapi")]
pub mod ntioapi;

#[cfg(not(target_os = "windows"))]
compile_error!("This module requires Windows target OS");
