use crate::arch;
use crate::mm::PhysicalMemoryManager;

use spin::Mutex;

static mut GLOBAL_ALLOCATOR: Option<Mutex<PhysicalMemoryManager>> = None;

#[derive(Debug)]
pub enum AllocatorError {
    OutOfMemory,
    InvalidFree,
}

pub fn init_global_allocator(arch: &impl arch::Architecture, page_size: usize) {
    unsafe {
        // Test might need a clean context so we need to reset the global allocator
        #[cfg(not(test))]
        if GLOBAL_ALLOCATOR.is_some() {
            panic!(
                "[ERROR] Tried to init global page allocator BUT it has already been initialized !"
            );
        }

        GLOBAL_ALLOCATOR = Some(Mutex::new(PhysicalMemoryManager::from_arch_info(
            arch, page_size,
        )));
    }
}

pub fn get_physical_memory_manager() -> &'static mut Mutex<PhysicalMemoryManager<'static>> {
    unsafe {
        if GLOBAL_ALLOCATOR.is_none() {
            panic!("[ERROR] Tried to access the global page allocator before it has been initialized !");
        }

        (&mut GLOBAL_ALLOCATOR).as_mut().unwrap()
    }
}
