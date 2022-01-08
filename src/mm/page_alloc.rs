#[derive(Debug)]
pub enum AllocError {
    OutOfMemory,
    InvalidFree,
}

pub trait PageAllocator {
    fn alloc_pages(&mut self, page_count: usize) -> Result<*mut u8, AllocError>;
    fn dealloc_pages(&mut self, ptr: *mut u8) -> Result<(), AllocError>;
}

