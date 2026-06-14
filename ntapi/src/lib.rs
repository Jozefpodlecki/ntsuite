#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused)]

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

#[cfg(feature = "ntrtl")]
pub mod ntrtl;

#[cfg(feature = "ntpebteb")]
pub mod ntpebteb;

#[cfg(feature = "ntpsapi")]
pub mod ntpsapi;


#[cfg(not(target_os = "windows"))]
compile_error!("This module requires Windows target OS");