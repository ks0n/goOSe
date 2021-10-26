use crate::mm;

#[repr(u8)]
#[derive(PartialEq)]
enum PageFlags {
    Empty,
    Used,
    /// When you have a contiguous allocation of multiple pages, Last is used to indicate the Last page of the allocation.
    Last,
}

pub enum AllocError {
    OutOfMemory,
}

struct Page {
    flags: u8, // flags is used as a bitfield of PageFlag
}

impl Page {
    pub fn is_empty(&self) -> bool {
        self.flags & PageFlags::Empty as u8 != 0
    }

    pub fn is_used(&self) -> bool {
        self.flags & PageFlags::Used as u8 != 0
    }

    pub fn set_used(&mut self) {
        self.flags |= PageFlags::Used as u8;
    }

    pub fn set_last(&mut self) {
        self.flags |= PageFlags::Last as u8;
    }

    pub fn clear(&mut self) {
        self.flags = 0;
    }
}

pub struct SimplePageAllocator<'a> {
    metadata: &'a mut [Page],
    page_size: usize,
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

        let metadata: &mut [Page] =
            unsafe { core::slice::from_raw_parts_mut(start as *mut Page, num_pages) };

        metadata.iter_mut().for_each(|page| page.clear());

        let metadata_size = core::mem::size_of_val(metadata);

        Self {
            metadata,
            page_size,
            heap: unsafe { start.add(metadata_size) },
        }
    }

    pub fn from_heap() -> Self {
        let (heap_start, heap_end) = unsafe {
            (
                external_symbol_value(&mm::HEAP_START),
                external_symbol_value(&mm::HEAP_END),
            )
        };
        let heap_size = heap_end - heap_start;

        // TODO: don't hardcode the 4096, get it from MemoryManager or something (it doesn't really exist as I have removed it for now
        Self::new(heap_start as *mut u8, heap_size, 4096)
    }

    // FIXME: Unit test this
    pub fn alloc_pages(&mut self, page_count: usize) -> Result<*mut u8, AllocError> {
        let mut i = 0;
        while i < self.metadata.len() {
            if self.metadata[i].is_empty() {
                let mut j = i;
                let mut consecutive_pages = 1;
                while j < self.metadata.len() {
                    if consecutive_pages == page_count {
                        self.metadata[i..j].iter_mut().for_each(|page| page.set_used());
                        self.metadata[j - 1].set_last();

                        return unsafe { Ok(self.heap.add(i * self.page_size)) };
                    }

                    if !self.metadata[j].is_empty() {
                        break;
                    }

                    consecutive_pages += 1;
                    j += 1;
                }

                i += 1;
            }
        }

        Err(AllocError::OutOfMemory)
    }
}
