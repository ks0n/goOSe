use crate::mm::PAddr;

#[derive(Debug)]
pub enum AllocatorError {
    OutOfMemory,
    InvalidFree,
}

pub trait PageAllocator {
    fn alloc_pages(&mut self, page_count: usize) -> Result<PAddr, AllocatorError>;
    fn dealloc_pages(&mut self, ptr: PAddr) -> Result<(), AllocatorError>;
}
