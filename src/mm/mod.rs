mod page_alloc;
mod physical_memory_manager;

pub use physical_memory_manager::PhysicalMemoryManager;

use crate::arch;
use crate::arch::ArchitectureMemory;
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

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
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

impl<T> core::convert::From<&PAddr> for *mut T {
    fn from(val: &PAddr) -> Self {
        val.addr as *mut T
    }
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

fn map_memory_rw(
    arch: &impl arch::Architecture,
    page_table: &mut arch::MemoryImpl,
    pmm: &mut PhysicalMemoryManager,
    page_size: usize,
) {
    arch.for_all_memory_regions(|regions| {
        regions
            .flat_map(|(base, size)| (base..base + size).step_by(page_size))
            .for_each(|page_base| {
                if !is_reserved_page(page_base, arch) {
                    page_table.map(
                        pmm,
                        PAddr::from(page_base),
                        VAddr::from(page_base),
                        Permissions::READ | Permissions::WRITE,
                    );
                }
            });
    });
}

fn map_kernel_rwx(mm: &mut arch::MemoryImpl, pmm: &mut PhysicalMemoryManager, page_size: usize) {
    let kernel_start = unsafe { utils::external_symbol_value(&KERNEL_START) };
    let kernel_end = unsafe { utils::external_symbol_value(&KERNEL_END) };
    let kernel_end_align = ((kernel_end + page_size - 1) / page_size) * page_size;

    for addr in (kernel_start..kernel_end_align).step_by(page_size) {
        mm.map(
            pmm,
            PAddr::from(addr),
            VAddr::from(addr),
            Permissions::READ | Permissions::WRITE | Permissions::EXECUTE,
        );
    }
}

pub fn map_address_space(
    arch: &impl arch::Architecture,
    page_table: &mut arch::MemoryImpl,
    pmm: &mut PhysicalMemoryManager,
) {
    let page_size = pmm.page_size();

    map_memory_rw(arch, page_table, pmm, page_size);
    map_kernel_rwx(page_table, pmm, page_size);

    let serial_page = crate::drivers::ns16550::QEMU_VIRT_BASE_ADDRESS;
    page_table.map(
        pmm,
        PAddr::from(serial_page),
        VAddr::from(serial_page),
        Permissions::READ | Permissions::WRITE,
    );

    page_table.reload();
}
