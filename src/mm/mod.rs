mod paging;
mod simple_page_allocator;

pub use simple_page_allocator::SimplePageAllocator;
pub use paging::{PAddr, VAddr, PageTable};

// use crate::arch::ArchitectureMemory;

// use fixedvec::FixedVec;

extern "C" {
    pub static KERNEL_START: usize;
    pub static KERNEL_END: usize;

    pub static HEAP_START: *mut u8;
    pub static HEAP_END: *const u8;
}

// #[derive(Copy, Clone)]
// pub struct Vma {
//     addr: usize,
//     size: usize,
// }

// pub struct MemoryManager<T: ArchitectureMemory> {
//     mem: T,
//     vmas: FixedVec<'static, Vma>,
// }

// fn external_symbol_value<T>(sym: &T) -> usize {
//     (sym as *const T) as usize
// }

// impl<T: ArchitectureMemory> MemoryManager<T> {
//     pub fn new(mem: T) -> Self {
//         let mut heap_slice: &'static mut [Vma] = unsafe {
//             let heap_start = external_symbol_value(&HEAP_START);
//             let heap_end = external_symbol_value(&HEAP_END);

//             let vma_count = (heap_end - heap_start) / core::mem::size_of::<Vma>();
//             core::slice::from_raw_parts_mut(heap_start as *mut Vma, vma_count)
//         };
//         let vmas = FixedVec::new(heap_slice);

//         vma.push(initial_vma).unwrap();

//         Self { mem, free_vma_pool }
//     }

//     pub fn get_page_size(&self) -> usize {
//         self.mem.get_page_size()
//     }

//     pub fn first_free_page(&mut self) -> Vma {
//         Vma { addr: 0, size: self.get_page_size() }
//     }
// }
