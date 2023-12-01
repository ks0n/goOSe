use crate::mm;

pub static PHYSICAL_MEMORY_MANAGER: mm::PhysicalMemoryManager = mm::PhysicalMemoryManager::new();

pub enum KernelState {
    EarlyInit,
    MmuEnabledInit,
}

impl KernelState {
    pub fn is_earlyinit(&self) -> bool {
        matches!(self, Self::EarlyInit)
    }

    pub fn is_mmu_enabled(&self) -> bool {
        matches!(self, Self::MmuEnabledInit)
    }
}

pub static mut STATE: KernelState = KernelState::EarlyInit;
