mod simple_page_allocator;
mod page_allocator;

pub use simple_page_allocator::SimplePageAllocator;
pub use page_allocator::PageAllocator;

use page_allocator::PhysicalPage;

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

// TODO: shall this be moved to arch/riscv (in the MemoryImpl) ?
pub fn is_reserved_page(base: usize, device_tree: &fdt::Fdt, page_size: usize) -> bool {
    let reserved_memory = device_tree.find_node("/reserved-memory").unwrap();
    let mut reserved_pages = reserved_memory
        .children()
        .flat_map(|child| child.reg().unwrap())
        .map(|region| (region.starting_address as usize, region.size.unwrap_or(0)))
        .flat_map(|(start, size)| (start..start+size).step_by(page_size));

    reserved_pages.any(|page_start| base >= page_start && base <= (page_start + page_size))
}

pub struct MemoryManager<'alloc, T: arch::ArchitectureMemory> {
    page_allocator: PageAllocator<'alloc>,
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

        let mut pa = page_allocator::PageAllocator::from_device_tree(&device_tree, T::get_page_size());
        let arch = T::new(&mut pa);

        Self {
            page_allocator: pa,
            arch,
        }
    }

    fn map(&mut self, to: usize, from: usize, perms: Permissions) {
        self.arch.map(&mut self.page_allocator, to, from, perms)
    }

    fn map_memory_rw(&mut self) {
        let un_self = self as *mut Self;

        for page in self.page_allocator.pages() {
            unsafe {
                (*un_self).map(page.base(), page.base(), Permissions::READ | Permissions::WRITE);
            }
        }
    }

    fn map_kernel_rwx(&mut self) {
        let kernel_start = unsafe { utils::external_symbol_value(&KERNEL_START) };
        let kernel_end = unsafe { utils::external_symbol_value(&KERNEL_END) };
        let page_size = self.page_allocator.page_size();
        let kernel_end_align = ((kernel_end + page_size - 1) / page_size) * page_size;

        for addr in (kernel_start..kernel_end_align).step_by(page_size) {
            self.map(addr, addr, Permissions::READ | Permissions::WRITE | Permissions::EXECUTE);
        }
    }

    pub fn map_address_space(&mut self) {
        let page_size = self.page_allocator.page_size();
        let rw = Permissions::READ | Permissions::WRITE;

        self.map_memory_rw();
        self.map_kernel_rwx();

        let serial_page = crate::drivers::ns16550::QEMU_VIRT_BASE_ADDRESS;
        self.map(serial_page, serial_page, rw);

        self.arch.reload();
    }
}
