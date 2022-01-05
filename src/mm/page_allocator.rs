use crate::kprintln;
use crate::mm;
use core::mem;

#[derive(Debug, PartialEq)]
enum PageKind {
    Kernel,
    Unused,
}

/// Holds data about each physical page in the system.
#[derive(Debug)]
struct PhysicalPage {
    kind: PageKind,
    base: usize,
}

impl PhysicalPage {
    fn is_unused(self) -> bool{
        self.kind == PageKind::Unused
    }
}


pub struct PageAllocator<'a> {
    metadata: &'a mut [PhysicalPage],
}

impl<'a> PageAllocator<'a> {

    fn count_pages(regions: impl Iterator<Item = fdt::standard_nodes::MemoryRegion>, page_size: usize) -> usize {
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

    /// Look for `pages_needed` contiguous unused pages, beware of pages that are in use by the
    /// kernel.
    fn find_contiguous_unused_pages(regions: impl Iterator<Item = fdt::standard_nodes::MemoryRegion>,
                                    pages_needed: usize, page_size: usize) -> Option<usize> {
        let physical_pages = regions
            .map(|region| {
                (region.starting_address, unsafe {
                    region.starting_address.add(region.size.unwrap_or(0))
                })
            })
            .map(|(start, end)| (start as usize, end as usize))
            .flat_map(|(start, end)| (start..end).step_by(page_size))
            .map(|base| {
                if mm::is_kernel_page(base) {
                    PhysicalPage {
                        kind: PageKind::Kernel,
                        base,
                    }
                } else {
                    PhysicalPage {
                        kind: PageKind::Unused,
                        base,
                    }
                }
            });

        let mut first_page_addr: usize = 0;
        let mut consecutive_pages: usize = 0;

        for page in physical_pages {
            if consecutive_pages == 0 {
                first_page_addr = page.base;
            }

            if !page.is_unused() {
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

/// TLDR: Initialize a [`PageAllocator`] from the device tree's memory node (*/memory*).
/// How it works
/// First count how many pages we can make out on the system, how much size we will need for
/// metadata and align that up to a page size.
/// Second (in [`Self::find_contiguous_unused_pages`]), look through our pages for a contiguous
/// space large enough to hold all our metadata.
/// Lastly store our metadata there, and mark pages as unused or kernel.
    pub fn from_memory_node(memory_node: &fdt::standard_nodes::Memory, page_size: usize) -> Self {
        let page_count = Self::count_pages(memory_node.regions(), page_size);
        let metadata_size = page_count * mem::size_of::<PhysicalPage>();
        let pages_needed = Self::align_up(metadata_size, page_size) / page_size;
        kprintln!("total of {:?} pages", page_count);
        kprintln!("metadata_size: {:?}", metadata_size);
        kprintln!("pages_needed: {:?}", pages_needed);

        let metadata_addr = Self::find_contiguous_unused_pages(memory_node.regions(), pages_needed, page_size).unwrap();
        kprintln!("metadata_addr: {:X}", metadata_addr);

        let metadata: &mut [PhysicalPage] = unsafe {
            core::slice::from_raw_parts_mut(metadata_addr as *mut _, page_count)
        };

        let physical_pages = memory_node.regions()
            .map(|region| {
                (region.starting_address, unsafe {
                    region.starting_address.add(region.size.unwrap_or(0))
                })
            })
            .map(|(start, end)| (start as usize, end as usize))
            .flat_map(|(start, end)| (start..end).step_by(page_size))
            .map(|base| {
                if mm::is_kernel_page(base) {
                    PhysicalPage {
                        kind: PageKind::Kernel,
                        base,
                    }
                } else {
                    PhysicalPage {
                        kind: PageKind::Unused,
                        base,
                    }
                }
            });

        for (i, page) in physical_pages.enumerate() {
            metadata[i] = page;
        }

        return Self { metadata }
    }
}
