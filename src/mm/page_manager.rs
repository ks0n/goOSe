use super::page_alloc::{AllocatorError, PageAllocator};
use crate::kprintln;
use crate::mm;
use core::mem;

#[derive(Debug, PartialEq)]
pub enum PageKind {
    /// For now, all reserved pages are owned by OpenSBI.
    Reserved,
    Kernel,
    Used,
    Unused,
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
    fn is_unused(&self) -> bool {
        self.kind == PageKind::Unused
    }

    fn is_used(&self) -> bool {
        self.kind != PageKind::Unused
    }

    fn set_used(&mut self) {
        self.kind = PageKind::Used;
    }

    fn _is_last(&self) -> bool {
        self.last
    }

    fn set_last(&mut self) {
        self.last = true;
    }

    pub fn base(&self) -> usize {
        self.base
    }
}

pub struct PageManager<'a> {
    metadata: &'a mut [PhysicalPage],
    page_size: usize,
}

impl<'a> PageManager<'a> {
    fn count_pages(
        regions: impl Iterator<Item = fdt::standard_nodes::MemoryRegion>,
        page_size: usize,
    ) -> usize {
        regions
            .map(|region| {
                (region.starting_address, unsafe {
                    region.starting_address.add(region.size.unwrap_or(0))
                })
            })
            .map(|(start, end)| (start as usize, end as usize))
            .flat_map(|(start, end)| (start..end).step_by(page_size))
            .count()
    }

    fn align_up(addr: usize, alignment: usize) -> usize {
        ((addr) + alignment - 1) & !(alignment - 1)
    }

    fn phys_addr_to_physical_page(
        phys_addr: usize,
        device_tree: &fdt::Fdt,
        page_size: usize,
    ) -> PhysicalPage {
        let kind = if mm::is_kernel_page(phys_addr) {
            PageKind::Kernel
        } else if mm::is_reserved_page(phys_addr, device_tree) {
            PageKind::Reserved
        } else {
            PageKind::Unused
        };

        PhysicalPage {
            kind,
            base: phys_addr,
            last: false,
        }
    }

    /// Look for `pages_needed` contiguous unused pages, beware of pages that are in use by the
    /// kernel.
    fn find_contiguous_unused_pages(
        device_tree: &fdt::Fdt,
        pages_needed: usize,
        page_size: usize,
    ) -> Option<usize> {
        let memory = device_tree.memory();

        let physical_pages = memory
            .regions()
            .map(|region| {
                (region.starting_address, unsafe {
                    region.starting_address.add(region.size.unwrap_or(0))
                })
            })
            .map(|(start, end)| (start as usize, end as usize))
            .flat_map(|(start, end)| (start..end).step_by(page_size))
            .map(|base| Self::phys_addr_to_physical_page(base, device_tree, page_size));

        let mut first_page_addr: usize = 0;
        let mut consecutive_pages: usize = 0;

        for page in physical_pages {
            if consecutive_pages == 0 {
                first_page_addr = page.base;
            }

            if page.is_used() {
                consecutive_pages = 0;
                continue;
            }

            consecutive_pages += 1;

            if consecutive_pages == pages_needed {
                return Some(first_page_addr);
            }
        }

        None
    }

    /// TLDR: Initialize a [`PageAllocator`] from the device tree.
    /// How it works:
    /// - First count how many pages we can make out on the system, how much size we will need for
    /// metadata and align that up to a page size.
    /// - Second (in [`Self::find_contiguous_unused_pages`]), look through our pages for a contiguous
    /// space large enough to hold all our metadata.
    /// - Lastly store our metadata there, and mark pages as unused or kernel.
    pub fn from_device_tree(device_tree: &fdt::Fdt, page_size: usize) -> Self {
        let memory_node = device_tree.memory();

        let page_count = Self::count_pages(memory_node.regions(), page_size);
        let metadata_size = page_count * mem::size_of::<PhysicalPage>();
        let pages_needed = Self::align_up(metadata_size, page_size) / page_size;
        kprintln!("total of {:?} pages", page_count);
        kprintln!("metadata_size: {:?}", metadata_size);
        kprintln!("pages_needed: {:?}", pages_needed);

        let metadata_addr =
            Self::find_contiguous_unused_pages(device_tree, pages_needed, page_size).unwrap();
        kprintln!("metadata_addr: {:X}", metadata_addr);

        let metadata: &mut [PhysicalPage] =
            unsafe { core::slice::from_raw_parts_mut(metadata_addr as *mut _, page_count) };

        let physical_pages = memory_node
            .regions()
            .map(|region| {
                (region.starting_address, unsafe {
                    region.starting_address.add(region.size.unwrap_or(0))
                })
            })
            .map(|(start, end)| (start as usize, end as usize))
            .flat_map(|(start, end)| (start..end).step_by(page_size))
            .map(|base| Self::phys_addr_to_physical_page(base, device_tree, page_size));

        for (i, page) in physical_pages.enumerate() {
            metadata[i] = page;
        }

        return Self {
            metadata,
            page_size,
        };
    }

    pub fn page_size(&self) -> usize {
        self.page_size
    }

    pub fn pages(&self) -> impl Iterator<Item = &PhysicalPage> + '_ {
        self.metadata.iter()
    }
}

impl PageAllocator for PageManager<'_> {
    fn alloc_pages(&mut self, page_count: usize) -> Result<*mut u8, AllocatorError> {
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
                    .for_each(|page| page.set_used());
                self.metadata[i].set_last();

                return Ok(self.metadata[first_page_index].base as *mut u8);
            }
        }

        Err(AllocatorError::OutOfMemory)
    }

    fn dealloc_pages(&mut self, _ptr: *mut u8) -> Result<(), AllocatorError> {
        Err(AllocatorError::InvalidFree)
    }
}
