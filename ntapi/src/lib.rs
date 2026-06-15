#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused)]

pub mod ntdef;
pub mod ntstatus;

#[cfg(feature = "ntpnpapi")]
pub mod ntpnpapi;

#[cfg(feature = "evntrace")]
pub mod evntrace;

#[cfg(feature = "ntdbg")]
pub mod ntdbg;

#[cfg(feature = "ntmisc")]
pub mod ntmisc;

#[cfg(feature = "ntwmi")]
pub mod ntwmi;

#[cfg(feature = "ntregapi")]
pub mod ntregapi;

#[cfg(feature = "ntzwapi")]
pub mod ntzwapi;

#[cfg(feature = "nttmapi")]
pub mod nttmapi;

#[cfg(feature = "ntseapi")]
pub mod ntseapi;

#[cfg(feature = "ntpoapi")]
pub mod ntpoapi;

#[cfg(feature = "ntexapi")]
pub mod ntexapi;

#[cfg(feature = "ntlpcapi")]
pub mod ntlpcapi;

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