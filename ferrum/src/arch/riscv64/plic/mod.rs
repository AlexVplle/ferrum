mod constants;

use constants::{
    CLAIM_COMPLETE_OFFSET, ENABLE_BASE, ENABLE_STRIDE_PER_CONTEXT, PENDING_BASE,
    PER_CONTEXT_STRIDE, SOURCE_PRIORITY_BASE, THRESHOLD_BASE,
};
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::memory_management::physical_address::PhysicalAddress;
use crate::memory_management::virtual_address::VirtualAddress;

use super::boot;

static PLATFORM_LEVEL_INTERRUPT_CONTROLLER_BASE: AtomicUsize = AtomicUsize::new(0);

pub fn init() {
    let physical_base: usize = boot::platform_level_interrupt_controller_address()
        .expect("PLIC not found in FDT");
    let virtual_base: VirtualAddress =
        super::physical_to_virtual(unsafe { PhysicalAddress::new_unchecked(physical_base) });
    PLATFORM_LEVEL_INTERRUPT_CONTROLLER_BASE.store(virtual_base.as_usize(), Ordering::Release);
    set_threshold(0);
}

fn base() -> usize {
    PLATFORM_LEVEL_INTERRUPT_CONTROLLER_BASE.load(Ordering::Acquire)
}

fn context() -> usize {
    boot::hart_id() as usize * 2 + 1
}

pub fn set_source_priority(source: usize, priority: u32) {
    let address: usize = base() + SOURCE_PRIORITY_BASE + 4 * source;
    unsafe { core::ptr::write_volatile(address as *mut u32, priority) };
}

pub fn is_pending(source: usize) -> bool {
    let address: usize = base() + PENDING_BASE + (source / 32) * 4;
    let value: u32 = unsafe { core::ptr::read_volatile(address as *const u32) };
    value & (1 << (source % 32)) != 0
}

pub fn enable_source(source: usize) {
    let address: usize =
        base() + ENABLE_BASE + ENABLE_STRIDE_PER_CONTEXT * context() + (source / 32) * 4;
    unsafe {
        let pointer: *mut u32 = address as *mut u32;
        let value: u32 = core::ptr::read_volatile(pointer);
        core::ptr::write_volatile(pointer, value | (1 << (source % 32)));
    }
}

pub fn set_threshold(threshold: u32) {
    let address: usize = base() + THRESHOLD_BASE + PER_CONTEXT_STRIDE * context();
    unsafe { core::ptr::write_volatile(address as *mut u32, threshold) };
}

pub fn claim() -> u32 {
    let address: usize =
        base() + THRESHOLD_BASE + PER_CONTEXT_STRIDE * context() + CLAIM_COMPLETE_OFFSET;
    unsafe { core::ptr::read_volatile(address as *const u32) }
}

pub fn complete(source: u32) {
    let address: usize =
        base() + THRESHOLD_BASE + PER_CONTEXT_STRIDE * context() + CLAIM_COMPLETE_OFFSET;
    unsafe { core::ptr::write_volatile(address as *mut u32, source) };
}
