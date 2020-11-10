// use crate::arch;
extern crate spin;

use crate::*;

use core::mem;
use core::slice;

use spin::RwLock;

lazy_static! {
    pub static ref PAGE_ALLOC: RwLock<PageAllocator> = RwLock::new(PageAllocator::new());
}

#[repr(u8)]
#[derive(Debug, PartialEq, Copy, Clone)]
enum UsageFlags {
    Used = 1,
    Free = 0,
}

#[derive(Debug, Default)]
pub struct PageAllocator {
    usage: &'static mut [UsageFlags],
    number_of_pages: usize,
    first_page: usize,
}

impl PageAllocator {
    pub fn new() -> PageAllocator {
        let heap_start: usize = unsafe { &arch::HEAP_START as *const () } as usize;
        let heap_end: usize = unsafe { &arch::HEAP_END as *const () } as usize;

        let heap_pages_count: usize = (heap_end - heap_start) / arch::PAGE_SIZE;
        let metadata_overhead: usize = (heap_pages_count) * mem::size_of::<UsageFlags>();
        // Calculate the address of the first page after heap_start + metadata
        let first_allocatable_page: usize =
            ((heap_start + metadata_overhead + arch::PAGE_SIZE - 1) / arch::PAGE_SIZE)
                * arch::PAGE_SIZE;
        let allocatable_pages_count =
            heap_pages_count - (first_allocatable_page - heap_start) / arch::PAGE_SIZE;

        // println!("heap_start = {}\nheap_end = {}\nheap_pages_count = {}\nmetadata_overhead = {}\nfirst_allocatable_page = {:#x}\nallocatable_pages_count= {}", heap_start, heap_end, heap_pages_count, metadata_overhead, first_allocatable_page, allocatable_pages_count);

        PageAllocator {
            usage: unsafe {
                slice::from_raw_parts_mut(
                    first_allocatable_page as *mut UsageFlags,
                    allocatable_pages_count,
                )
            },
            number_of_pages: allocatable_pages_count,
            first_page: first_allocatable_page,
        }
    }

    pub fn page_alloc(&mut self) -> Option<usize> {
        for index in 0..self.number_of_pages {
            match self.usage[index] {
                UsageFlags::Free => {
                    self.usage[index] = UsageFlags::Used;
                    let addr = self.first_page + index * arch::PAGE_SIZE;
                    return Some(addr);
                }
                UsageFlags::Used => (),
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utest::uassert_eq;

    #[test_case]
    fn page_alloc_one() {
        let page = PAGE_ALLOC.write().page_alloc();
        kassert!(page.is_some(), "Page alloc one page test");
    }

    #[test_case]
    fn page_alloc_out_of_memory() {
        let mut allocator = PAGE_ALLOC.write();
        while allocator.page_alloc().is_some() {
            // At some point we will not have any free pages left
        }

        kassert!(true, "Page alloc out of memory");
    }
}
