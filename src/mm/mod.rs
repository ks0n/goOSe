mod page_alloc;
mod first_fit_page_allocator;

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

pub struct MemoryManagement<'alloc, T: arch::ArchitectureMemory> {
    page_allocator: FirstFitPageAllocator<'alloc>,
    arch: &'alloc mut T,
}

impl<'alloc, T: arch::ArchitectureMemory> MemoryManagement<'alloc, T> {
    pub fn new(arch: &impl arch::Architecture) -> Self {
        let mut page_allocator =
            first_fit_page_allocator::FirstFitPageAllocator::from_arch_info(arch, T::get_page_size());
        let arch_mem = T::new(&mut page_allocator);

        Self { page_allocator, arch: arch_mem }
    }

    fn map_memory_rw(&mut self) {
        let un_self = self as *mut Self;

        for page in self.page_allocator.pages() {
            unsafe {
                (*un_self).map(
                    PAddr::from(page.base()),
                    VAddr::from(page.base()),
                    Permissions::READ | Permissions::WRITE,
                );
            }
        }
    }

    fn map_kernel_rwx(&mut self) {
        let kernel_start = unsafe { utils::external_symbol_value(&KERNEL_START) };
        let kernel_end = unsafe { utils::external_symbol_value(&KERNEL_END) };
        let page_size = self.page_allocator.page_size();
        let kernel_end_align = ((kernel_end + page_size - 1) / page_size) * page_size;

        for addr in (kernel_start..kernel_end_align).step_by(page_size) {
            self.map(
                PAddr::from(addr),
                VAddr::from(addr),
                Permissions::READ | Permissions::WRITE | Permissions::EXECUTE,
            );
        }
    }

    pub fn map_address_space(&mut self) {
        self.map_memory_rw();
        self.map_kernel_rwx();

        let serial_page = crate::drivers::ns16550::QEMU_VIRT_BASE_ADDRESS;
        self.map(
            PAddr::from(serial_page),
            VAddr::from(serial_page),
            Permissions::READ | Permissions::WRITE,
        );

        self.reload_page_table();
    }
}

impl<T: arch::ArchitectureMemory> MemoryManager for MemoryManagement<'_, T> {
    fn map(&mut self, phys: PAddr, virt: VAddr, perms: Permissions) {
        self.arch.map(&mut self.page_allocator, phys.into(), virt.into(), perms)
    }

    fn reload_page_table(&mut self) {
        self.arch.reload();
    }
}
