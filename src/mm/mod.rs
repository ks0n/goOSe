mod simple_page_allocator;

pub use simple_page_allocator::SimplePageAllocator;

use crate::arch;

use crate::utils;
use bitflags::bitflags;

extern "C" {
    pub static KERNEL_START: usize;
    pub static KERNEL_END: usize;

    pub static HEAP_START: *mut u8;
    pub static HEAP_END: *const u8;
}

bitflags! {
    pub struct Permissions: u8 {
        const READ    = 0b00000001;
        const WRITE   = 0b00000010;
        const EXECUTE = 0b00000100;
    }
}

pub struct MemoryManager<'alloc, T: arch::ArchitectureMemory> {
    page_allocator: SimplePageAllocator<'alloc>,
    arch: &'alloc mut T,
}

impl<'alloc, T: arch::ArchitectureMemory> MemoryManager<'alloc, T> {
    pub fn new() -> Self {
        let (heap_start, heap_end) = unsafe {
            (
                utils::external_symbol_value(&HEAP_START),
                utils::external_symbol_value(&HEAP_END),
            )
        };

        let mut page_allocator =
            SimplePageAllocator::from_heap(heap_start, heap_end, T::get_page_size());
        let arch = T::new(&mut page_allocator);

        Self {
            page_allocator,
            arch,
        }
    }

    fn map(&mut self, to: usize, from: usize, perms: Permissions) {
        self.arch.map(&mut self.page_allocator, to, from, perms)
    }

    pub fn map_address_space(&mut self) {
        let kernel_start = unsafe { utils::external_symbol_value(&KERNEL_START) };
        let kernel_end = unsafe { utils::external_symbol_value(&KERNEL_END) };
        let page_size = self.page_allocator.page_size();
        let kernel_end_align = ((kernel_end + page_size - 1) / page_size) * page_size;
        let rwx = Permissions::READ | Permissions::WRITE | Permissions::EXECUTE;

        for addr in (kernel_start..kernel_end_align).step_by(page_size) {
            self.map(addr, addr, rwx);
        }

        let serial_page = crate::drivers::ns16550::QEMU_VIRT_BASE_ADDRESS;
        self.map(serial_page, serial_page, rwx);
    }
}
