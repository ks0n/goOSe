use crate::arch;
use crate::utils;

use crate::mm::{
    FirstFitPageAllocator,
    PAddr,
    VAddr,
    Permissions,
    MemoryManager,
    KERNEL_START,
    KERNEL_END,
};

pub struct MemoryManagement<'alloc, T: arch::ArchitectureMemory> {
    page_allocator: FirstFitPageAllocator<'alloc>,
    arch: &'alloc mut T,
}

impl<'alloc, T: arch::ArchitectureMemory> MemoryManagement<'alloc, T> {
    pub fn new(arch: &impl arch::Architecture) -> Self {
        let mut page_allocator =
            FirstFitPageAllocator::from_arch_info(arch, T::get_page_size());
        let arch_mem = T::new(&mut page_allocator);

        Self { page_allocator, arch: arch_mem }
    }

    fn map_memory_rw(&mut self) {
        let un_self = self as *mut Self;

        for page in self.page_allocator.pages() {
            unsafe {
                (*un_self).map(
                    PAddr::from(page.base()),
                    VAddr::from(page.base()),
                    Permissions::READ | Permissions::WRITE,
                );
            }
        }
    }

    fn map_kernel_rwx(&mut self) {
        let kernel_start = unsafe { utils::external_symbol_value(&KERNEL_START) };
        let kernel_end = unsafe { utils::external_symbol_value(&KERNEL_END) };
        let page_size = self.page_allocator.page_size();
        let kernel_end_align = ((kernel_end + page_size - 1) / page_size) * page_size;

        for addr in (kernel_start..kernel_end_align).step_by(page_size) {
            self.map(
                PAddr::from(addr),
                VAddr::from(addr),
                Permissions::READ | Permissions::WRITE | Permissions::EXECUTE,
            );
        }
    }

    pub fn map_address_space(&mut self) {
        self.map_memory_rw();
        self.map_kernel_rwx();

        let serial_page = crate::drivers::ns16550::QEMU_VIRT_BASE_ADDRESS;
        self.map(
            PAddr::from(serial_page),
            VAddr::from(serial_page),
            Permissions::READ | Permissions::WRITE,
        );

        self.reload_page_table();
    }
}

impl<T: arch::ArchitectureMemory> MemoryManager for MemoryManagement<'_, T> {
    fn map(&mut self, phys: PAddr, virt: VAddr, perms: Permissions) {
        self.arch.map(&mut self.page_allocator, phys.into(), virt.into(), perms)
    }

    fn reload_page_table(&mut self) {
        self.arch.reload();
    }
}
