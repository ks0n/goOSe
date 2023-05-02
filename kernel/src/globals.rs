use crate::lock::Lock;

use crate::mm;

use crate::drivers;
use crate::utils;
use utils::init_cell::InitCell;
use utils::init_once::InitOnce;

use alloc::sync::Arc;

static NULL_CONSOLE: drivers::null_uart::NullUart = drivers::null_uart::NullUart::new();

pub static EARLYINIT_CONSOLE: InitCell<&'static (dyn drivers::Console + Sync)> =
    InitCell::new(&NULL_CONSOLE);
pub static CONSOLE: InitOnce<Arc<dyn drivers::Console + Sync + Send>> = InitOnce::new();
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

pub fn set_earlyinit_console(new_console: &'static (dyn drivers::Console + Sync)) {
    EARLYINIT_CONSOLE.set(|console| *console = new_console);
}

pub fn get_earlyinit_console() -> &'static dyn drivers::Console {
    *EARLYINIT_CONSOLE.get()
}
