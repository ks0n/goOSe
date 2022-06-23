use crate::arch;
use crate::arch::ArchitectureMemory;
use crate::mm;
use crate::mm::{PAddr, Permissions, VAddr};

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

    pub fn map(&mut self, phys: PAddr, virt: VAddr, perms: Permissions) {
        let mut page_allocator = mm::get_global_allocator().lock();

        self.arch
            .map(&mut *page_allocator, phys.into(), virt.into(), perms)
    }

    pub fn reload_page_table(&mut self) {
        self.arch.reload();
    }

    pub fn disable_page_table(&mut self) {
        self.arch.disable();
    }

    pub fn page_size(&self) -> usize {
        arch::MemoryImpl::get_page_size()
    }

    pub fn align_down(&self, addr: usize) -> usize {
        arch::MemoryImpl::align_down(addr)
    }

    pub fn align_up(&self, addr: usize) -> usize {
        arch::MemoryImpl::align_up(addr)
    }
}
