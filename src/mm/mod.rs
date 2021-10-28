mod paging;
mod simple_page_allocator;

use crate::utils;
pub use paging::{load_pt, PAddr, PageTable, VAddr};
pub use simple_page_allocator::SimplePageAllocator;

// use crate::arch::ArchitectureMemory;

// use fixedvec::FixedVec;

extern "C" {
    pub static KERNEL_START: usize;
    pub static KERNEL_END: usize;

    pub static HEAP_START: *mut u8;
    pub static HEAP_END: *const u8;
}

pub fn map_address_space(root: &mut PageTable, allocator: &mut SimplePageAllocator) {
    let kernel_start = unsafe { utils::external_symbol_value(&KERNEL_START) };
    let kernel_end = unsafe { utils::external_symbol_value(&KERNEL_END) };
    let page_size = allocator.page_size();
    let kernel_end_align = ((kernel_end + page_size - 1) / page_size) * page_size;

    for addr in (kernel_start..kernel_end_align).step_by(page_size) {
        let addr = addr as u64;
        root.map(allocator, PAddr::from_u64(addr), VAddr::from_u64(addr));
    }

    let serial_page = unsafe { crate::drivers::ns16550::QEMU_VIRT_BASE_ADDRESS };
    root.map(
        allocator,
        PAddr::from_u64(serial_page as u64),
        VAddr::from_u64(serial_page as u64),
    );
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
