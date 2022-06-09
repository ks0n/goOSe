use crate::arch;
use crate::arch::ArchitectureMemory;
use crate::mm;
use crate::mm::{MemoryManager, PAddr, Permissions, VAddr};

pub struct MemoryManagement<'alloc> {
    arch: &'alloc mut arch::MemoryImpl,
}

impl<'alloc> MemoryManagement<'alloc> {
    pub fn new() -> Self {
        let mut page_allocator = mm::get_global_allocator().lock();
        Self {
            arch: arch::MemoryImpl::new(&mut *page_allocator),
        }
    }
}

impl<'alloc> MemoryManager for MemoryManagement<'alloc> {
    fn map(&mut self, phys: PAddr, virt: VAddr, perms: Permissions) {
        let mut page_allocator = mm::get_global_allocator().lock();

        self.arch
            .map(&mut *page_allocator, phys.into(), virt.into(), perms)
    }

    fn reload_page_table(&mut self) {
        self.arch.reload();
    }

    fn disable_page_table(&mut self) {
        self.arch.disable();
    }

    fn page_size(&self) -> usize {
        arch::MemoryImpl::get_page_size()
    }

    fn align_down(&self, addr: usize) -> usize {
        arch::MemoryImpl::align_down(addr)
    }

    fn align_up(&self, addr: usize) -> usize {
        arch::MemoryImpl::align_up(addr)
    }
}
