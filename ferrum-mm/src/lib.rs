#![cfg_attr(not(any(test, feature = "fuzz")), no_std)]
#![feature(ptr_alignment_type)]

pub mod allocator;
pub mod gfp;
pub mod early;
pub mod arch;
pub mod memory_block;
pub mod page;
pub mod physical_address;
pub mod virtual_address;
pub mod virtual_memory_area;

pub use physical_address::PhysicalAddress;
pub use virtual_address::VirtualAddress;
