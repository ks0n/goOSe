mod page_alloc;
mod physical_memory_manager;
pub use physical_memory_manager::PhysicalMemoryManager;

use crate::device_tree::DeviceTree;
use crate::paging;
use crate::paging::PagingImpl as _;
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

pub fn is_reserved_page(base: usize, device_tree: &DeviceTree) -> bool {
    let mut is_res = false;

    device_tree.for_all_reserved_memory_regions(|regions| {
        is_res = regions
            .map(|(start, size)| (start, size)) // this is a weird hack to fix a type error.
            .any(|(region_start, region_size)| {
                base >= region_start && base <= (region_start + region_size)
            })
    });

    is_res
}

fn map_kernel_rwx(mm: &mut crate::PagingImpl, pmm: &mut PhysicalMemoryManager, page_size: usize) {
    let kernel_start = unsafe { utils::external_symbol_value(&KERNEL_START) };
    let kernel_end = unsafe { utils::external_symbol_value(&KERNEL_END) };
    let kernel_end_align = ((kernel_end + page_size - 1) / page_size) * page_size;

    for addr in (kernel_start..kernel_end_align).step_by(page_size) {
        if let Err(e) = mm.map(
            pmm,
            PAddr::from(addr),
            VAddr::from(addr),
            Permissions::READ | Permissions::WRITE | Permissions::EXECUTE,
        ) {
            panic!("Failed to map address space: {:?}", e);
        }
    }
}

pub struct KernelPageTable(&'static mut crate::PagingImpl);

impl KernelPageTable {
    pub fn identity_map(&mut self, addr: usize, perms: Permissions) -> Result<(), paging::Error> {
        self.0
            .map_noalloc(PAddr::from(addr), VAddr::from(addr), perms)
    }

    pub fn reload(&mut self) {
        self.0.reload()
    }
    pub fn disable(&mut self) {
        self.0.disable()
    }
}

pub fn map_address_space(
    device_tree: &DeviceTree,
    pmm: &mut PhysicalMemoryManager,
    drivers: &[&dyn drivers::Driver],
) -> KernelPageTable {
    let page_table = crate::PagingImpl::new(pmm);
    let page_size = pmm.page_size();

    device_tree.for_all_memory_regions(|regions| {
        regions
            .flat_map(|(base, size)| (base..base + size).step_by(page_size))
            .for_each(|page_base| {
                if let Err(e) = page_table.add_invalid_entry(pmm, VAddr::from(page_base)) {
                    panic!("Failed to map address space: {:?}", e);
                }
            })
    });

    map_kernel_rwx(page_table, pmm, page_size);

    let metadata_pages = pmm.metadata_pages();
    metadata_pages.for_each(|page| {
        if let Err(e) = page_table.map(
            pmm,
            PAddr::from(page),
            VAddr::from(page),
            Permissions::READ | Permissions::WRITE,
        ) {
            panic!("Failed to map address space: {:?}", e);
        }
    });

    drivers
        .iter()
        .map(|drv| drv.get_address_range())
        .flat_map(|(base, len)| (base..(base + len)).step_by(page_size))
        .for_each(|page| {
            if let Err(e) = page_table.map(
                pmm,
                PAddr::from(page),
                VAddr::from(page),
                Permissions::READ | Permissions::WRITE,
            ) {
                panic!("Failed to map address space: {:?}", e);
            }
        });

    page_table.reload();

    KernelPageTable(page_table)
}
