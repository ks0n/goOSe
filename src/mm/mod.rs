mod simple_page_allocator;
mod page_allocator;

pub use simple_page_allocator::SimplePageAllocator;

use crate::arch;
use crate::utils;
use bitflags::bitflags;

extern "C" {
    pub static KERNEL_START: usize;
    pub static KERNEL_END: usize;
}

bitflags! {
    pub struct Permissions: u8 {
        const READ    = 0b00000001;
        const WRITE   = 0b00000010;
        const EXECUTE = 0b00000100;
    }
}

pub fn is_kernel_page(base: usize) -> bool {
    let (kernel_start, kernel_end) = unsafe {
        (
            utils::external_symbol_value(&KERNEL_START),
            utils::external_symbol_value(&KERNEL_END),
        )
    };

    base >= kernel_start && base < kernel_end
}

pub struct MemoryManager<'alloc, T: arch::ArchitectureMemory> {
    page_allocator: SimplePageAllocator<'alloc>,
    arch: &'alloc mut T,
}

impl<'alloc, T: arch::ArchitectureMemory> MemoryManager<'alloc, T> {


    pub fn new(device_tree: &fdt::Fdt) -> Self {
        let memory_node = device_tree.memory();




        // for page in tamer {
        //     kprintln!("{:X?}", page);
        // }

        // for reservation in device_tree.memory_reservations() {
        //     kprintln!("{:?}", reservation);
        // }

        let pa = page_allocator::PageAllocator::from_memory_node(&memory_node, T::get_page_size());
        let mut simple_page_allocator = SimplePageAllocator::from_heap(0, 0, T::get_page_size());
        let arch = T::new(&mut simple_page_allocator);

        Self {
            page_allocator: simple_page_allocator,
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
