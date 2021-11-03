#[derive(Debug)]
pub enum AllocError {
    OutOfMemory,
    InvalidFree,
}

#[derive(PartialEq)]
enum PageUsage {
    Empty,
    Used,
}

// TODO: We can optimize this by just keeping an u8 flag or a repr(u8) enum
struct Page {
    used: PageUsage,
    /// When you have a contiguous allocation of multiple pages, Last is used to indicate the Last page of the allocation.
    is_last: bool,
}

impl Page {
    pub fn is_empty(&self) -> bool {
        self.used == PageUsage::Empty
    }

    #[allow(dead_code)]
    pub fn is_used(&self) -> bool {
        self.used == PageUsage::Used
    }

    #[allow(dead_code)]
    pub fn is_last(&self) -> bool {
        self.is_last
    }

    #[allow(dead_code)]
    pub fn set_empty(&mut self) {
        self.used = PageUsage::Empty;
    }

    pub fn set_used(&mut self) {
        self.used = PageUsage::Used;
    }

    pub fn set_last(&mut self) {
        self.is_last = true;
    }

    pub fn clear(&mut self) {
        self.used = PageUsage::Empty;
        self.is_last = false;
    }
}

pub struct SimplePageAllocator<'a> {
    metadata: &'a mut [Page],
    page_size: usize,
    heap: *mut u8,
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

        let heap = unsafe { start.add(metadata_size) };
        let heap = unsafe { heap.add(heap.align_offset(4096)) };

        Self {
            metadata,
            page_size,
            heap,
        }
    }

    pub fn from_heap(heap_start: usize, heap_end: usize, page_size: usize) -> Self {
        let heap_size = heap_end - heap_start;

        Self::new(heap_start as *mut u8, heap_size, page_size)
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
                        self.metadata[i].set_used();
                        self.metadata[i..j]
                            .iter_mut()
                            .for_each(|page| page.set_used());
                        self.metadata[j].set_last();

                        return unsafe { Ok(self.heap.add(i * self.page_size)) };
                    }

                    if !self.metadata[j].is_empty() {
                        break;
                    }

                    consecutive_pages += 1;
                    j += 1;
                }
            }
            i += 1;
        }

        Err(AllocError::OutOfMemory)
    }

    #[allow(dead_code)]
    pub fn dealloc_pages(&mut self, ptr: *mut u8) -> Result<(), AllocError> {
        // FIXME: Make sure that the pointer is aligned
        let start = (ptr as usize - self.heap as usize) / self.page_size;

        for page in self.metadata[start..].iter_mut() {
            if !page.is_used() {
                return Err(AllocError::InvalidFree);
            }

            page.set_empty();

            if page.is_last() {
                page.clear();
                page.set_empty();
                break;
            }
        }

        Ok(())
    }

    pub fn page_size(&self) -> usize {
        self.page_size
    }
}