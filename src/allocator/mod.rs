use crate::arch;
use crate::utils::external_symbol_address;

use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

/// Init the allocator. This needs to be called before any allocation can be performed.
pub fn init() {
    let heap_start = external_symbol_address(unsafe { &arch::HEAP_START });
    let heap_end = external_symbol_address(unsafe { &arch::HEAP_END });
    let heap_size = heap_end - heap_start;
    unsafe {
        ALLOCATOR.lock().init(heap_start, heap_size);
    }
}

mod test {
    use alloc::vec::Vec;

    #[test_case]
    fn simple_alloc() {
        crate::allocator::init();
        let mut vec: Vec<usize> = Vec::new();
        for i in 0..5 {
            vec.push(i as usize);
        }

        kassert!(true, "Simple allocation");
    }

    // TODO: Find a way to test out of memory case
}
