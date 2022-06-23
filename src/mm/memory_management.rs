use crate::arch;
use crate::arch::ArchitectureMemory;
use crate::mm;
use crate::mm::{PAddr, Permissions, PhysicalMemoryManager, VAddr};

pub struct MemoryManagement<'alloc> {
    arch: &'alloc mut arch::MemoryImpl,
}

impl<'alloc> MemoryManagement<'alloc> {
    pub fn new(pmm: &mut PhysicalMemoryManager) -> Self {
        Self {
            arch: arch::MemoryImpl::new(pmm),
        }
    }

    pub fn map(
        &mut self,
        pmm: &mut PhysicalMemoryManager,
        phys: PAddr,
        virt: VAddr,
        perms: Permissions,
    ) {
        self.arch.map(pmm, phys.into(), virt.into(), perms)
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
