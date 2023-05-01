mod physical_memory_manager;
pub use physical_memory_manager::{AllocatorError, PhysicalMemoryManager};

mod binary_buddy_allocator;

use crate::device_tree::DeviceTree;
use crate::globals;

use crate::Error;
use crate::hal;
use hal_core::mm::{PageMap, Permissions, VAddr};

use crate::drivers;
use drivers::Driver;


use core::{iter, slice};
use staticvec::StaticVec;

extern "C" {
    pub static KERNEL_START: usize;
    pub static KERNEL_END: usize;
}

pub fn is_kernel_page(base: usize) -> bool {
    let (kernel_start, kernel_end) = unsafe {
        (
            crate::utils::external_symbol_value(&KERNEL_START),
            crate::utils::external_symbol_value(&KERNEL_END),
        )
    };

    base >= kernel_start && base < kernel_end
}

pub fn kernel_memory_region() -> (usize, usize) {
    unsafe {
        (
            crate::utils::external_symbol_value(&KERNEL_START),
            crate::utils::external_symbol_value(&KERNEL_END),
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

fn map_kernel_rwx() -> (
    impl Iterator<Item=(usize, usize)>,
    impl Iterator<Item=(usize, usize)>,
    impl Iterator<Item=(usize, usize)>,
) {
    let page_size = hal::mm::PAGE_SIZE;
    let kernel_start = unsafe { crate::utils::external_symbol_value(&KERNEL_START) };
    let kernel_end = unsafe { crate::utils::external_symbol_value(&KERNEL_END) };
    let kernel_end_align = ((kernel_end + page_size - 1) / page_size) * page_size;

    let rwx_entries = iter::once((kernel_start, kernel_end_align));

    (iter::empty(), iter::empty(), rwx_entries)
}

pub fn alloc_pages_raw(count: usize) -> Result<hal_core::mm::PAddr, Error> {
    // If there is a kernel pagetable, identity map the pages.
    // Not sure this is the best idea, but I would say it follows the
    // principle of least astonishment.
    // TODO: remove unwrap
    let start = globals::PHYSICAL_MEMORY_MANAGER.lock(|pmm| pmm.alloc_rw_pages(count).unwrap());
    let addr: usize = start.into();

    if unsafe { globals::STATE.is_mmu_enabled() } {
        hal::mm::current().identity_map_range(VAddr::new(addr), count, Permissions::READ | Permissions::WRITE, |_| {
            // The mmu is enabled, therefore we already mapped all DRAM into the kernel's pagetable as
            // invalid entries.
            // Pagetable must only modify existing entries and not allocate.
            panic!("alloc_rw_pages: pagetable tried to allocate memory when mapping it's rw_pages")
        }).expect("mapping in this case should never fail as illustrated by the comment above...");
    }

    Ok(hal_core::mm::PAddr::new(addr))
}

pub fn alloc_pages(count: usize) -> Result<&'static mut [u8], Error> {
    let addr = alloc_pages_raw(count)?;
    let page_size = hal::mm::PAGE_SIZE;

    Ok(unsafe { slice::from_raw_parts_mut(addr.val as *mut _, count * page_size) })
}

pub fn alloc_pages_for_hal(count: usize) -> hal_core::mm::PAddr {
    alloc_pages_raw(count).expect("page allocation function passed to the hal has failed, critical...")
}

pub fn map_address_space(device_tree: &DeviceTree, drivers: &[&dyn Driver]) -> Result<(), Error> {
    let mut r_entries = StaticVec::<(usize, usize), 128>::new();
    let mut rw_entries = StaticVec::<(usize, usize), 128>::new();
    let mut rwx_entries = StaticVec::<(usize, usize), 128>::new();
    let mut pre_allocated_entries = StaticVec::<(usize, usize), 1024>::new();

    // Add entries/descriptors in the pagetable for all of accessible memory regions.
    // That way in the future, mapping those entries won't require any memory allocations,
    // just settings the entry to valid and filling up the bits.
    device_tree.for_all_memory_regions(|regions| {
        regions.for_each(|region| pre_allocated_entries.try_push(region).unwrap());

        // TODO: this needs to be done differently, we're mapping all DRAM as rw...
        regions.for_each(|region| rw_entries.try_push(region).unwrap());
    });
    let (dt_start, dt_end) = device_tree.memory_region();
    rw_entries.try_push((dt_start, dt_end-dt_start));

    let (kernel_r, kernel_rw, kernel_rwx) = map_kernel_rwx();
    kernel_r.for_each(|entry| r_entries.try_push(entry).unwrap());
    kernel_rw.for_each(|entry| rw_entries.try_push(entry).unwrap());
    kernel_rwx.for_each(|entry| rwx_entries.try_push(entry).unwrap());

    for drv in drivers.iter() {
        if let Some((base, len)) = drv.get_address_range() {
            rw_entries.try_push((base, len)).unwrap();
        }
    }

    // rw_entries = rw_entries.chain(
    //     globals::PHYSICAL_MEMORY_MANAGER.lock(|pmm| {
    //         let metadata_pages = pmm.metadata_pages();
    //         let allocated_pages = pmm.allocated_pages();
    //         let pmm_pages = metadata_pages.chain(allocated_pages);
    //     }),
    // );
    globals::PHYSICAL_MEMORY_MANAGER.lock(|pmm| {
        // All pmm pages are located in DRAM so they are already in the pagetable (they are part of
        // the pre_allocated_entries).
        // Therefore no allocations will be made.
        let _pmm_pages = iter::Iterator::chain(pmm.metadata_pages(), pmm.allocated_pages());
        // XXX: put the pages as range in to rw_entries 
        //      for now just crammed all memory regions as rw_entries a bit higher in the function.
    });


    hal::mm::init_paging(r_entries.into_iter(), rw_entries.into_iter(), rwx_entries.into_iter(), pre_allocated_entries.into_iter(), |count| {
        alloc_pages_raw(count).expect("failure on page allocator passed to init_paging")
    });
    unsafe { globals::STATE = globals::KernelState::MmuEnabledInit };

    Ok(())
}
