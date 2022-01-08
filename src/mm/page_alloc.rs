#[derive(Debug)]
pub enum AllocatorError {
    OutOfMemory,
    InvalidFree,
}

pub trait PageAllocator {
    fn alloc_pages(&mut self, page_count: usize) -> Result<*mut u8, AllocatorError>;
    fn dealloc_pages(&mut self, ptr: *mut u8) -> Result<(), AllocatorError>;
}
