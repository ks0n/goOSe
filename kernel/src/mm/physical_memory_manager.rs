use crate::device_tree::DeviceTree;
use crate::globals;
use crate::hal;
use crate::mm;
use core::mem;
use hal_core::{
    mm::{AllocatorError, NullPageAllocator, PageAlloc, PageMap, Permissions, VAddr},
    AddressRange,
};

use hal::mm::PAGE_SIZE;

use log::debug;
use spin::mutex::Mutex;

#[derive(Debug, PartialEq, Eq)]
pub enum PageKind {
    Metadata,
    Allocated,
    Free,
}

/// Holds data about each physical page in the system.
#[derive(Debug)]
pub struct PhysicalPage {
    kind: PageKind,
    base: usize,

    /// Page is last of a contiguous allocation of pages.
    last: bool,
}

impl PhysicalPage {
    fn is_used(&self) -> bool {
        self.kind != PageKind::Free
    }

    fn is_allocated(&self) -> bool {
        self.kind == PageKind::Allocated
    }

    fn set_allocated(&mut self) {
        self.kind = PageKind::Allocated;
    }

    fn _is_last(&self) -> bool {
        self.last
    }

    fn set_last(&mut self) {
        self.last = true;
    }
}

#[derive(Debug)]
pub struct PhysicalMemoryManager {
    metadata: Mutex<&'static mut [PhysicalPage]>,
}

impl PhysicalMemoryManager {
    fn count_pages(regions: &[Option<AddressRange>]) -> usize {
        let total_memory_bytes: usize = regions
            .iter()
            .filter_map(|maybe_region| maybe_region.map(|region| region.size()))
            .sum();

        total_memory_bytes / PAGE_SIZE
    }

    fn find_large_region(regions: &[Option<AddressRange>], minimum_size: usize) -> Option<usize> {
        regions
            .iter()
            .flatten()
            .find(|region| region.size() >= minimum_size)
            .map(|region| region.start)
    }

    fn is_metadata_page(base: usize, metadata_start: usize, metadata_end: usize) -> bool {
        base >= metadata_start && base < metadata_end
    }

    fn phys_addr_to_physical_page(
        phys_addr: usize,
        metadata_start: usize,
        metadata_end: usize,
    ) -> PhysicalPage {
        let kind = if Self::is_metadata_page(phys_addr, metadata_start, metadata_end) {
            PageKind::Metadata
        } else {
            PageKind::Free
        };

        PhysicalPage {
            kind,
            base: phys_addr,
            last: false,
        }
    }

    fn exclude_range<const MAX_REGIONS: usize>(
        regions: &mut [Option<AddressRange>; MAX_REGIONS],
        excluded: AddressRange,
    ) {
        let (excl_start, excl_end) = (excluded.start, excluded.end);

        assert!(excl_start < excl_end);

        for i in 0..MAX_REGIONS {
            if regions[i].is_none() {
                continue;
            }
            let region = regions[i].unwrap();

            if region.start == excl_start && region.end == excl_end {
                // Perfect overlap between the region to be excluded and the current region, just remove the region.
                regions[i] = None;
            } else if (region.start < excl_start && excl_start < region.end)
                && (region.start < excl_end && excl_end < region.end)
            {
                // Region to be excluded is the middle of the current region.
                let new_region = AddressRange {
                    start: excl_end,
                    end: region.end,
                };
                regions[i] = Some(AddressRange::new(region.start..excl_start));

                // The exclusion in the middle causes a split of the current region, put the new region (the end part) somewhere there is a none.
                *regions
                    .iter_mut()
                    .find(|maybe_region| maybe_region.is_none())
                    .expect("regions array is too small, increase MAX_REGIONS") = Some(new_region);
            } else if region.contains(excl_end) {
                // Region to be removed is at the left (at the beginning) of the current region.
                regions[i] = Some(AddressRange::new(excl_end..region.end));
            } else if region.contains(excl_start) {
                // Region to be removed is at the right (at the end) of the current region.
                regions[i] = Some(AddressRange::new(region.start..excl_start));
            }
        }
    }

    fn available_memory_regions<const MAX_REGIONS: usize>(
        device_tree: &DeviceTree,
    ) -> [Option<AddressRange>; MAX_REGIONS] {
        // First put all regions in the array.
        let mut all_regions = [None; MAX_REGIONS];
        device_tree.for_all_memory_regions(|regions| {
            regions.enumerate().for_each(|(i, (base, size))| {
                if i == MAX_REGIONS - 1 {
                    panic!(
                        "found more regions in the device tree than this has been compiled to fit"
                    );
                }

                all_regions[i] = Some(AddressRange {
                    start: base,
                    end: base + size,
                });
            });
        });

        Self::exclude_range(&mut all_regions, mm::kernel_memory_region());

        Self::exclude_range(&mut all_regions, device_tree.memory_region());

        device_tree.for_all_reserved_memory_regions(|reserved_regions| {
            reserved_regions.for_each(|(base, size)| {
                Self::exclude_range(&mut all_regions, AddressRange::with_size(base, size))
            })
        });

        // Re-align the regions, for exemple things we exclude are not always aligned to a page boundary.
        all_regions.iter_mut().for_each(|maybe_region| {
            if let Some(region) = maybe_region {
                region.start = hal::mm::align_down(region.start);
                region.end = hal::mm::align_up(region.end);

                *maybe_region = if region.size() > 0 {
                    Some(*region)
                } else {
                    None
                };
            }
        });

        all_regions
    }

    pub const fn new() -> Self {
        let metadata = unsafe {
            core::slice::from_raw_parts_mut(
                core::ptr::NonNull::<PhysicalPage>::dangling().as_ptr(),
                0,
            )
        };

        Self {
            metadata: Mutex::new(metadata),
        }
    }

    /// Initialize a [`PageAllocator`] from the device tree.
    pub fn init_from_device_tree(&self, device_tree: &DeviceTree) -> Result<(), AllocatorError> {
        let available_regions = Self::available_memory_regions::<10>(device_tree);

        assert!(
            available_regions
                .iter()
                .flatten()
                .all(
                    |region| region.start == hal::mm::align_up(region.start)
                        && region.end == hal::mm::align_up(region.end)
                ),
            "Expected region bounds to be aligned to the page size (won't be possible to allocate pages otherwise)"
        );

        for (i, reg) in available_regions.iter().flatten().enumerate() {
            debug!("region {}: {:X?}", i, reg);
        }

        let page_count = Self::count_pages(&available_regions);
        let metadata_size = page_count * mem::size_of::<PhysicalPage>();
        let pages_needed = hal::mm::align_up(metadata_size) / PAGE_SIZE;

        let metadata_addr = Self::find_large_region(&available_regions, metadata_size)
            .ok_or(AllocatorError::NotEnoughMemoryForMetadata)?;

        let metadata: &mut [PhysicalPage] =
            unsafe { core::slice::from_raw_parts_mut(metadata_addr as *mut _, page_count) };

        let physical_pages = available_regions
            .iter()
            .flatten()
            .flat_map(|region| region.iter_pages(PAGE_SIZE))
            .map(|base| {
                Self::phys_addr_to_physical_page(
                    base,
                    metadata_addr,
                    metadata_addr + pages_needed * PAGE_SIZE,
                )
            });

        let mut count = 0;
        for (i, page) in physical_pages.enumerate() {
            metadata[i] = page;
            count += 1;
        }
        assert!(count == page_count);

        *self.metadata.lock() = metadata;

        Ok(())
    }

    pub fn alloc_pages(&self, page_count: usize) -> Result<usize, AllocatorError> {
        let mut consecutive_pages: usize = 0;
        let mut first_page_index: usize = 0;
        let mut last_page_base: usize = 0;

        let mut metadata = self.metadata.lock();

        for (i, page) in metadata.iter().enumerate() {
            if consecutive_pages == 0 {
                first_page_index = i;
                last_page_base = page.base;
            }

            if page.is_used() {
                consecutive_pages = 0;
                continue;
            }

            if consecutive_pages > 0 && page.base != last_page_base + PAGE_SIZE {
                consecutive_pages = 0;
                continue;
            }

            consecutive_pages += 1;
            last_page_base = page.base;

            if consecutive_pages == page_count {
                metadata[first_page_index..=i]
                    .iter_mut()
                    .for_each(|page| page.set_allocated());
                metadata[i].set_last();

                return Ok(metadata[first_page_index].base);
            }
        }

        Err(AllocatorError::OutOfMemory)
    }
}

impl PageAlloc for PhysicalMemoryManager {
    fn alloc(&self, page_count: usize) -> Result<usize, AllocatorError> {
        // If there is a kernel pagetable, identity map the pages.
        let first_page = self.alloc_pages(page_count)?;

        if unsafe { globals::STATE.is_mmu_enabled() } {
            // The mmu is enabled, therefore we already mapped all DRAM into the kernel's pagetable
            // as invalid entries.
            // Pagetable must only modify existing entries and not allocate.
            hal::mm::current()
                .identity_map_range(
                    VAddr::new(first_page),
                    page_count,
                    Permissions::READ | Permissions::WRITE,
                    &NullPageAllocator,
                )
                .unwrap();
        }

        Ok(first_page)
    }

    fn dealloc(&self, _base: usize, _page_count: usize) -> Result<(), AllocatorError> {
        // TODO:
        //  - if MMU is on, unmap the page
        //  - set as free
        log::warn!("PMM dealloc not yet implemented...");
        Ok(())
    }

    fn used_pages<F: FnMut(usize)>(&self, f: F) {
        let metadata = self.metadata.lock();

        let metadata_start = (&metadata[0] as *const PhysicalPage) as usize;
        let metadata_last = (&metadata[metadata.len() - 1] as *const PhysicalPage) as usize;

        let metadata_pages = (metadata_start..=metadata_last).step_by(PAGE_SIZE);
        let allocated_pages = metadata
            .iter()
            .filter(|page| page.is_allocated())
            .map(|page| page.base);

        metadata_pages.chain(allocated_pages).for_each(f);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel_tests::*;

    #[test_case]
    fn exclude_range_remove_in_the_middle(_ctx: &mut TestContext) {
        let mut ranges = [Some(AddressRange::new(0x0..0x1000)), None];
        PhysicalMemoryManager::exclude_range(&mut ranges, (0x500, 0x600));

        assert_eq!(ranges[0], Some(AddressRange::new(0x0..0x500)));
        assert_eq!(ranges[1], Some(AddressRange::new(0x600, 0x1000)));
    }

    #[test_case]
    fn exclude_range_remove_beginning(_ctx: &mut TestContext) {
        let mut ranges = [Some(AddressRange::new(0x100..0x1000)), None];
        PhysicalMemoryManager::exclude_range(&mut ranges, (0x0, 0x200));

        assert_eq!(ranges[0], Some(AddressRange::new(0x200..0x1000)));
        assert!(ranges[1].is_none());
    }

    #[test_case]
    fn exclude_range_remove_ending(_ctx: &mut TestContext) {
        let mut ranges = [Some(AddressRange::new(0x100..0x1000)), None];
        PhysicalMemoryManager::exclude_range(&mut ranges, (0x800, 0x1000));

        assert_eq!(ranges[0], Some(AddressRange::new(0x100..0x800)));
        assert!(ranges[1].is_none());
    }

    #[test_case]
    fn exclude_range_overlaps_exactly(_ctx: &mut TestContext) {
        let mut ranges = [Some(AddressRange::new(0x400_000..0x800_000)), None];
        PhysicalMemoryManager::exclude_range(&mut ranges, (0x400_000, 0x800_000));

        assert!(ranges[0].is_none());
        assert!(ranges[1].is_none());
    }

    #[test_case]
    fn exclude_range_overlap_with_exact_beginning(_ctx: &mut TestContext) {
        let mut ranges = [Some(AddressRange::new(0x400_000..0x800_000)), None];
        PhysicalMemoryManager::exclude_range(&mut ranges, (0x400_000, 0x401_000));

        assert_eq!(ranges[0], Some(AddressRange::new(0x401_000..0x800_000)));
        assert!(ranges[1].is_none());
    }
}
