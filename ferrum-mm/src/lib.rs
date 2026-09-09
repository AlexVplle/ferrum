#![cfg_attr(not(any(test, feature = "fuzz")), no_std)]
#![feature(ptr_alignment_type)]

#[cfg(not(any(test, feature = "fuzz")))]
extern crate alloc;

pub mod allocator;
pub mod init;
pub mod gfp;
pub mod memory_descriptor;
pub mod early;
pub mod arch;
pub mod memory_block;
pub mod page;
pub mod physical_address;
pub mod virtual_address;
pub mod virtual_memory_area;

pub use physical_address::PhysicalAddress;
pub use virtual_address::VirtualAddress;
