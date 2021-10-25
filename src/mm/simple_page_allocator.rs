use crate::mm;

#[repr(u8)]
#[derive(PartialEq)]
enum PageFlags {
    Empty,
    Used,
    Last,  /// When you have a contiguous allocation of multiple pages, Last is used to indicate the Last page of the allocation.
}

struct Page {
    flags: u8, // flags is used as a bitfield of PageFlag
}

impl Page {
    pub fn is_empty(&self) -> bool {
        self.flags & PageFlags::Empty.val() != 0
    }

    pub fn is_used(&self) -> bool {
        self.flags & PageFlags::Used.val() != 0
    }

    pub fn clear(&mut self) {
        self.flags = 0;
    }
}

pub struct SimplePageAllocator<'a> {
    metadata: &'a mut [Page],
    heap: *mut u8,
}

fn external_symbol_value<T>(sym: &T) -> usize {
    (sym as *const T) as usize
}

impl<'a> SimplePageAllocator<'a> {
    fn new(start: *mut u8, size: usize, page_size: usize) -> Self {
        let possible_num_pages = size / page_size;
        let possible_metadata_size = possible_num_pages * core::mem::size_of::<Page>();

        let num_pages = (size - possible_metadata_size) / page_size;

        let metadata: &mut [Page] = unsafe {
            core::slice::from_raw_parts_mut::<Page>(start as *mut Page, num_pages)
        };

        metadata.for_each(|page| page.clear());

        Self { metadata, heap: start }
    }

    pub fn from_heap() -> Self {
        let (heap_start, heap_end) = unsafe {
            (
                external_symbol_value(&mm::HEAP_START),
                external_symbol_value(&mm::HEAP_END)
            )
        };
        let heap_size = heap_end - heap_start;

        Self::new(heap_start as *mut u8, heap_size, 4096) // TODO: don't hardcode the 4096, get it from MemoryManager or something (it doesn't really exist as I have removed it for now
    }

    pub fn alloc_pages(&mut self, page_count: usize) -> *mut u8 {
        for i, page in self.pages.iter().enumerate() {
            if page.is_empty() {
            }
        }
    }
}
