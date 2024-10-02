mod physical_memory_manager;
pub use physical_memory_manager::PhysicalMemoryManager;

mod binary_buddy_allocator;

use crate::device_tree::DeviceTree;
use crate::globals;

use crate::Error;
use crate::HAL;
use hal_core::mm::{NullPageAllocator, PageAlloc, PageMap, Permissions, VAddr};
use hal_core::AddressRange;

use crate::drivers;
use drivers::Driver;

use arrayvec::ArrayVec;
use core::iter;

use log::debug;

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

pub fn kernel_memory_region() -> AddressRange {
    let (start, end) = unsafe {
        (
            crate::utils::external_symbol_value(&KERNEL_START),
            crate::utils::external_symbol_value(&KERNEL_END),
        )
    };

    AddressRange::new(start..end)
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
    impl Iterator<Item = AddressRange>,
    impl Iterator<Item = AddressRange>,
    impl Iterator<Item = AddressRange>,
) {
    let page_size = HAL.page_size();
    let kernel_start = unsafe { crate::utils::external_symbol_value(&KERNEL_START) };
    let kernel_end = unsafe { crate::utils::external_symbol_value(&KERNEL_END) };
    let kernel_end_align = ((kernel_end + page_size - 1) / page_size) * page_size;

    let rwx_entries = iter::once(AddressRange::new(kernel_start..kernel_end_align));

    (iter::empty(), iter::empty(), rwx_entries)
}

pub fn map_address_space<'a, I: Iterator<Item = &'a &'a dyn Driver>>(
    device_tree: &DeviceTree,
    drivers: I,
) -> Result<(), Error> {
    let mut r_entries = ArrayVec::<AddressRange, 128>::new();
    let mut rw_entries = ArrayVec::<AddressRange, 128>::new();
    let mut rwx_entries = ArrayVec::<AddressRange, 128>::new();
    let mut pre_allocated_entries = ArrayVec::<AddressRange, 1024>::new();

    // Add entries/descriptors in the pagetable for all of accessible memory regions.
    // That way in the future, mapping those entries won't require any memory allocations,
    // just settings the entry to valid and filling up the bits.
    device_tree.for_all_memory_regions(|regions| {
        regions.for_each(|(base, size)| {
            pre_allocated_entries
                .try_push(AddressRange::with_size(base, size))
                .unwrap();
        });
    });
    debug!(
        "adding region containing the device tree to rw entries {:X?}",
        device_tree.memory_region()
    );
    rw_entries
        .try_push(
            device_tree
                .memory_region()
                .round_up_to_page(HAL.page_size()),
        )
        .unwrap();

    let (kernel_r, kernel_rw, kernel_rwx) = map_kernel_rwx();
    r_entries.extend(kernel_r);
    rw_entries.extend(kernel_rw);
    rwx_entries.extend(kernel_rwx);

    for drv in drivers {
        if let Some((base, len)) = drv.get_address_range() {
            let len = HAL.align_up(len);
            debug!(
                "adding driver memory region to RW entries: [{:X}; {:X}]",
                base,
                base + len
            );
            rw_entries
                .try_push(AddressRange::with_size(base, len))
                .unwrap();
        }
    }

    debug!("r_entries: {:X?}", r_entries);
    debug!("rw_entries: {:X?}", rw_entries);
    debug!("rwx_entries: {:X?}", rwx_entries);
    debug!("pre_allocated_entries: {:X?}", pre_allocated_entries);

    HAL.init_kpt(
        r_entries.into_iter(),
        rw_entries.into_iter(),
        rwx_entries.into_iter(),
        pre_allocated_entries.into_iter(),
        &globals::PHYSICAL_MEMORY_MANAGER,
    )?;

    // All pmm pages are located in DRAM so they are already in the pagetable (they are part of
    // the pre_allocated_entries).
    // Therefore no allocations will be made, pass the NullPageAllocator.
    globals::PHYSICAL_MEMORY_MANAGER.used_pages(|page| {
        HAL.kpt()
            .lock()
            .identity_map(
                VAddr::new(page),
                Permissions::READ | Permissions::WRITE,
                &NullPageAllocator,
            )
            .unwrap();
    });

    log::trace!("going to enable paging...");
    HAL.enable_paging()?;
    log::trace!("enabled paging !");

    unsafe { globals::STATE = globals::KernelState::MmuEnabledInit };

    Ok(())
}
