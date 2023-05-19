use crate::lock::Lock;

use crate::mm;

use crate::drivers;
use crate::utils;
use utils::init_cell::InitCell;
use utils::init_once::InitOnce;

use alloc::sync::Arc;

pub static PHYSICAL_MEMORY_MANAGER: Lock<mm::PhysicalMemoryManager> =
    Lock::new(mm::PhysicalMemoryManager::new());

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
