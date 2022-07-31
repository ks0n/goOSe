mod physical_memory_manager;
pub use physical_memory_manager::{AllocatorError, PhysicalMemoryManager};

mod binary_buddy_allocator;

use crate::device_tree::DeviceTree;
use crate::globals;
use crate::paging;
use crate::paging::PagingImpl as _;
use crate::Error;
use drivers::Driver;

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
        const USER    = 0b00001000;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VAddr {
    pub addr: usize,
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

pub fn kernel_memory_region() -> (usize, usize) {
    unsafe {
        (
            utils::external_symbol_value(&KERNEL_START),
            utils::external_symbol_value(&KERNEL_END),
        )
    }
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

fn map_kernel_rwx(pagetable: &mut crate::PagingImpl) {
    let page_size = crate::PagingImpl::get_page_size();
    let kernel_start = unsafe { utils::external_symbol_value(&KERNEL_START) };
    let kernel_end = unsafe { utils::external_symbol_value(&KERNEL_END) };
    let kernel_end_align = ((kernel_end + page_size - 1) / page_size) * page_size;

    for addr in (kernel_start..kernel_end_align).step_by(page_size) {
        if let Err(e) = pagetable.map(
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
    pub fn identity_map(&mut self, addr: usize, perms: Permissions) -> Result<(), Error> {
        self.0.map(PAddr::from(addr), VAddr::from(addr), perms)
    }

    pub fn map(&mut self, paddr: usize, vaddr: usize, perms: Permissions) -> Result<(), Error> {
        self.0.map(PAddr::from(paddr), VAddr::from(vaddr), perms)
    }

    pub fn fork_user_page_table(
        &mut self,
        pmm: &mut PhysicalMemoryManager,
    ) -> Result<UserPageTable, Error> {
        let page_table = crate::PagingImpl::new()?;
        let page_table_addr = (page_table as *mut crate::PagingImpl) as usize;
        self.identity_map(page_table_addr, Permissions::READ | Permissions::WRITE)?;

        map_kernel_rwx(page_table);

        Ok(UserPageTable(page_table))
    }

    pub fn reload(&mut self) {
        self.0.reload()
    }
    pub fn disable(&mut self) {
        self.0.disable()
    }
}

pub struct UserPageTable(&'static mut crate::PagingImpl);

impl UserPageTable {
    pub fn map(
        &mut self,
        kernel_page_table: &mut KernelPageTable,
        allocator: &mut PhysicalMemoryManager,
        paddr: usize,
        vaddr: usize,
        perms: Permissions,
    ) -> Result<(), Error> {
        self.0.map(PAddr::from(paddr), VAddr::from(vaddr), perms)
    }

    pub fn align_down(&self, addr: usize) -> usize {
        crate::PagingImpl::align_down(addr)
    }

    pub fn align_up(&self, addr: usize) -> usize {
        crate::PagingImpl::align_up(addr)
    }

    pub fn reload(&mut self) {
        self.0.reload()
    }
    pub fn disable(&mut self) {
        self.0.disable()
    }
}

pub fn map_address_space(device_tree: &DeviceTree, drivers: &[&dyn Driver]) -> Result<(), Error> {
    let page_table: &mut crate::PagingImpl = globals::KERNEL_PAGETABLE.lock(|pt| pt);

    let page_size = crate::PagingImpl::get_page_size();

    // Add entries/descriptors in the pagetable for all of accessible memory regions.
    // That way in the future, mapping those entries won't require any memory allocations,
    // just settings the entry to valid and filling up the bits.
    device_tree.for_all_memory_regions(|regions| {
        regions
            .flat_map(|(base, size)| (base..base + size).step_by(page_size))
            .for_each(|page_base| {
                if let Err(e) = page_table.add_invalid_entry(VAddr::from(page_base)) {
                    panic!("Failed to map address space: {:?}", e);
                }
            })
    });

    let (dt_start, dt_end) = device_tree.memory_region();
    for base in (dt_start..dt_end).step_by(page_size) {
        page_table.map(
            PAddr::from(base),
            VAddr::from(base),
            Permissions::READ | Permissions::WRITE,
        )?;
    }

    map_kernel_rwx(page_table);

    drivers
        .iter()
        .flat_map(|drv| drv.get_address_range())
        .flat_map(|(base, len)| (base..(base + len)).step_by(page_size))
        .for_each(|page| {
            if let Err(e) = page_table.map(
                PAddr::from(page),
                VAddr::from(page),
                Permissions::READ | Permissions::WRITE,
            ) {
                panic!("Failed to map address space: {:?}", e);
            }
        });

    globals::PHYSICAL_MEMORY_MANAGER.lock(|pmm| {
        let metadata_pages = pmm.metadata_pages();
        let allocated_pages = pmm.allocated_pages();
        let pmm_pages = metadata_pages.chain(allocated_pages);
        pmm_pages.for_each(|page| {
            // All pmm pages are part of DRAM so they are already in the pagetable.
            // Therefore no allocations should be made.
            page_table
                .map(
                    PAddr::from(page),
                    VAddr::from(page),
                    Permissions::READ | Permissions::WRITE,
                )
                .unwrap()
        });
    });

    unsafe { globals::STATE = globals::KernelState::MmuEnabledInit };
    page_table.reload();

    Ok(())
}
