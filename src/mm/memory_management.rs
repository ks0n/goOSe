use crate::arch;

use crate::mm;
use crate::mm::{MemoryManager, PAddr, Permissions, VAddr};

pub struct MemoryManagement<'alloc, T: arch::ArchitectureMemory> {
    arch: &'alloc mut T,
}

impl<'alloc, T: arch::ArchitectureMemory> MemoryManagement<'alloc, T> {
    pub fn new() -> Self {
        let mut page_allocator = mm::get_global_allocator().lock();
        let arch_mem = T::new(&mut *page_allocator);

        Self { arch: arch_mem }
    }
}

impl<T: arch::ArchitectureMemory> MemoryManager for MemoryManagement<'_, T> {
    fn map(&mut self, phys: PAddr, virt: VAddr, perms: Permissions) {
        let mut page_allocator = mm::get_global_allocator().lock();

        self.arch
            .map(&mut *page_allocator, phys.into(), virt.into(), perms)
    }

    fn reload_page_table(&mut self) {
        self.arch.reload();
    }
}
