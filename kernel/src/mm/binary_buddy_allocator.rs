use crate::globals;

use core::alloc::{GlobalAlloc, Layout};

pub struct BinaryBuddyAllocator;

unsafe impl Sync for BinaryBuddyAllocator {}

unsafe impl GlobalAlloc for BinaryBuddyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if unsafe { globals::STATE.is_earlyinit() } {
            panic!("Something tried to allocate before earlyinit is over o_O");
        }

        assert!(layout.size() > 0);

        // The keen eye might notice this is just giving away pages ^^
        // TODO: some much to be done
        //   - actually implement a buddy allocator ^^
        //   - be thread-safe
        //   - disable interrupts when entering, then re-enable

        globals::PHYSICAL_MEMORY_MANAGER.lock(|pmm| {
            let page_count = if layout.size() <= pmm.page_size() {
                1
            } else {
                layout.size() / pmm.page_size() + 1
            };
            pmm.alloc_rw_pages(page_count)
                .unwrap_or(0usize.into()) as *mut u8
        })
    }

    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {
        crate::kprintln!("[WARNING] dealloc is not implemented yet, freeing memory isn't supported by the allocator");
    }
}

#[global_allocator]
static HEAP: BinaryBuddyAllocator = BinaryBuddyAllocator;

#[alloc_error_handler]
fn binary_buddy_oom(_layout: Layout) -> ! {
    panic!("The binary buddy allocator has oomed, this system has gobbled up all the RAM\nWe are dOOMed !!!");
}
