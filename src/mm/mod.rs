mod memory_management;
mod page_alloc;
mod first_fit_page_allocator;

pub use memory_management::MemoryManagement;
pub use page_alloc::PageAllocator;
use first_fit_page_allocator::FirstFitPageAllocator;


use crate::arch;
use crate::utils;
use bitflags::bitflags;

extern "C" {
    pub static KERNEL_START: usize;
    pub static KERNEL_END: usize;
}

bitflags! {
    pub struct Permissions: u8 {
        const READ    = 0b00000001;
        const WRITE   = 0b00000010;
        const EXECUTE = 0b00000100;
    }
}

pub struct VAddr {
    addr: usize,
}

impl core::convert::From<usize> for VAddr {
    fn from(val: usize) -> Self {
        Self { addr: val }
    }
}

impl core::convert::From<VAddr> for usize {
    fn from(val: VAddr) -> Self {
        val.addr
    }
}

pub struct PAddr {
    addr: usize,
}

impl core::convert::From<usize> for PAddr {
    fn from(val: usize) -> Self {
        Self { addr: val }
    }
}

impl core::convert::From<PAddr> for usize {
    fn from(val: PAddr) -> Self {
        val.addr
    }
}

impl<T> core::convert::From<PAddr> for *mut T {
    fn from(val: PAddr) -> Self {
        val.addr as *mut T
    }
}

pub trait MemoryManager {
    fn map(&mut self, phys: PAddr, virt: VAddr, perms: Permissions);
    fn reload_page_table(&mut self);

}

pub fn is_kernel_page(base: usize) -> bool {
    let (kernel_start, kernel_end) = unsafe {
        (
            utils::external_symbol_value(&KERNEL_START),
            utils::external_symbol_value(&KERNEL_END),
        )
    };

    base >= kernel_start && base < kernel_end
}

pub fn is_reserved_page(base: usize, arch: &impl arch::Architecture) -> bool {
    let mut is_res = false;

    arch.for_all_reserved_memory_regions(|regions| {
        is_res = regions
            .map(|(start, size)| (start, size)) // this is a weird hack to fix a type error.
            .any(|(region_start, region_size)| {
            base >= region_start && base <= (region_start + region_size)
        })
    });

    return is_res;
}
