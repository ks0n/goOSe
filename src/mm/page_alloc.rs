use crate::arch;
use crate::mm::{PAddr, FirstFitPageAllocator};

#[derive(Debug)]
pub enum AllocatorError {
    OutOfMemory,
    InvalidFree,
}

pub trait PageAllocator {
    fn alloc_pages(&mut self, page_count: usize) -> Result<PAddr, AllocatorError>;
    fn dealloc_pages(&mut self, ptr: PAddr) -> Result<(), AllocatorError>;

    fn page_size(&self) -> usize;
}

static mut GLOBAL_ALLOCATOR: Option<FirstFitPageAllocator> = None;

pub fn init_global_allocator(arch: &impl arch::Architecture, page_size: usize) {
    unsafe {
        if !GLOBAL_ALLOCATOR.is_none() {
            panic!("[ERROR] Tried to init global page allocator BUT it has already been initialized !");
        }

        GLOBAL_ALLOCATOR = Some(FirstFitPageAllocator::from_arch_info(arch, page_size));
    }
}

pub fn get_global_allocator() -> &'static mut impl PageAllocator {
    unsafe {
        if GLOBAL_ALLOCATOR.is_none() {
            panic!("[ERROR] Tried to access the global page allocator before it has been initialized !");
        }

        (&mut GLOBAL_ALLOCATOR).as_mut().unwrap()
    }
}
