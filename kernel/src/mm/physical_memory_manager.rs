use crate::device_tree::DeviceTree;
use crate::mm;
use crate::mm::PAddr;
use crate::Architecture;
use core::mem;

#[derive(Debug, PartialEq, Eq)]
pub enum PageKind {
    Metadata,
    /// For now, all reserved pages are owned by OpenSBI.
    Reserved,
    Kernel,
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

    fn is_reserved(&self) -> bool {
        self.kind == PageKind::Reserved
    }

    fn is_kernel(&self) -> bool {
        self.kind == PageKind::Reserved
    }

    fn is_allocated(&self) -> bool {
        self.kind == PageKind::Allocated
    }

    fn is_free(&self) -> bool {
        self.kind == PageKind::Free
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
pub enum AllocatorError {
    OutOfMemory,
}

pub struct PhysicalMemoryManager {
    metadata: &'static mut [PhysicalPage],
    page_size: usize,
}

impl PhysicalMemoryManager {
    fn count_pages(device_tree: &DeviceTree, page_size: usize) -> usize {
        let mut count = 0;

        device_tree.for_all_memory_regions(|regions| {
            count = regions
                .map(|(start, size)| (start as usize, size as usize))
                .flat_map(|(start, size)| (start..start + size).step_by(page_size))
                .count();
        });

        count
    }

    fn align_up(addr: usize, alignment: usize) -> usize {
        ((addr) + alignment - 1) & !(alignment - 1)
    }

    fn is_metadata_page(base: usize, metadata_start: usize, metadata_end: usize) -> bool {
        base >= metadata_start && base < metadata_end
    }

    fn phys_addr_to_physical_page(
        phys_addr: usize,
        metadata_start: usize,
        metadata_end: usize,
        device_tree: &DeviceTree,
    ) -> PhysicalPage {
        let kind = if mm::is_kernel_page(phys_addr) {
            PageKind::Kernel
        } else if mm::is_reserved_page(phys_addr, device_tree) {
            PageKind::Reserved
        } else if Self::is_metadata_page(phys_addr, metadata_start, metadata_end) {
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

    /// Look for `pages_needed` contiguous unused pages, beware of pages that are in use by the
    /// kernel or reserved by opensbi.
    fn find_contiguous_unused_pages(
        device_tree: &DeviceTree,
        pages_needed: usize,
        page_size: usize,
    ) -> Option<usize> {
        let mut found = None;

        device_tree.for_all_memory_regions(|regions| {
            let physical_pages = regions
                .flat_map(|(addr, size)| (addr..addr + size).step_by(page_size))
                .filter(|addr| !device_tree.is_used(*addr));

            let mut first_page_addr: usize = 0;
            let mut consecutive_pages: usize = 0;

            for page in physical_pages {
                if consecutive_pages == 0 {
                    first_page_addr = page;
                }

                if mm::is_kernel_page(page) || mm::is_reserved_page(page, device_tree) {
                    consecutive_pages = 0;
                    continue;
                }

                consecutive_pages += 1;

                if consecutive_pages == pages_needed {
                    found = Some(first_page_addr);
                    return;
                }
            }
        });

        found
    }

    /// TLDR: Initialize a [`PageAllocator`] from the device tree.
    /// How it works:
    /// - First count how many pages we can make out on the system, how much size we will need for
    /// metadata and align that up to a page size.
    /// - Second (in [`Self::find_contiguous_unused_pages`]), look through our pages for a contiguous
    /// space large enough to hold all our metadata.
    /// - Lastly store our metadata there, and mark pages as unused or kernel.
    pub fn from_device_tree(device_tree: &DeviceTree, page_size: usize) -> Self {
        let page_count = Self::count_pages(device_tree, page_size);
        let metadata_size = page_count * mem::size_of::<PhysicalPage>();
        let pages_needed = Self::align_up(metadata_size, page_size) / page_size;

        let metadata_addr =
            Self::find_contiguous_unused_pages(device_tree, pages_needed, page_size).unwrap();

        let metadata: &mut [PhysicalPage] =
            unsafe { core::slice::from_raw_parts_mut(metadata_addr as *mut _, page_count) };

        device_tree.for_all_memory_regions(|regions| {
            let physical_pages = regions
                .flat_map(|(start, size)| (start..start + size).step_by(page_size))
                .filter(|addr| !device_tree.is_used(*addr))
                .map(|base| {
                    Self::phys_addr_to_physical_page(
                        base,
                        metadata_addr,
                        metadata_addr + pages_needed * page_size,
                        device_tree,
                    )
                });

            for (i, page) in physical_pages.enumerate() {
                metadata[i] = page;
            }
        });

        Self {
            metadata,
            page_size,
        }
    }

    pub fn alloc_pages(&mut self, page_count: usize) -> Result<PAddr, AllocatorError> {
        let mut consecutive_pages: usize = 0;
        let mut first_page_index: usize = 0;

        for (i, page) in self.metadata.iter().enumerate() {
            if consecutive_pages == 0 {
                first_page_index = i;
            }

            if page.is_used() {
                consecutive_pages = 0;
                continue;
            }

            consecutive_pages += 1;

            if consecutive_pages == page_count {
                self.metadata[first_page_index..=i]
                    .iter_mut()
                    .for_each(|page| page.set_allocated());
                self.metadata[i].set_last();

                return Ok(PAddr::from(self.metadata[first_page_index].base));
            }
        }

        Err(AllocatorError::OutOfMemory)
    }

    pub fn page_size(&self) -> usize {
        self.page_size
    }

    pub fn metadata_pages(&self) -> impl core::iter::Iterator<Item = usize> {
        let metadata_start = (&self.metadata[0] as *const PhysicalPage) as usize;
        let metadata_last =
            (&self.metadata[self.metadata.len() - 1] as *const PhysicalPage) as usize;

        (metadata_start..=metadata_last).step_by(self.page_size())
    }

    pub fn allocated_pages(&self) -> impl core::iter::Iterator<Item = usize> + '_ {
        self.metadata
            .iter()
            .filter(|page| page.is_allocated())
            .map(|page| page.base)
    }
}
