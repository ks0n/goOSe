use crate::lock::Lock;

use crate::mm;

use drivers::init_cell::InitCell;
use drivers::Console;

static NULL_CONSOLE: drivers::null_uart::NullUart = drivers::null_uart::NullUart::new();

pub static CONSOLE: InitCell<&'static (dyn drivers::Console + Sync)> = InitCell::new(&NULL_CONSOLE);
pub static PHYSICAL_MEMORY_MANAGER: Lock<mm::PhysicalMemoryManager> =
    Lock::new(mm::PhysicalMemoryManager::new());

pub static KERNEL_PAGETABLE: Lock<crate::PagingImpl> = Lock::new(crate::PagingImpl::zeroed());

pub enum KernelState {
    EarlyInit,
    MmuEnabledInit,
}

impl KernelState {
    pub fn is_mmu_enabled(&self) -> bool {
        matches!(self, Self::MmuEnabledInit)
    }
}

pub static mut STATE: KernelState = KernelState::EarlyInit;

pub fn set_console(new_console: &'static (dyn drivers::Console + Sync)) {
    CONSOLE.set(|console| *console = new_console);
}

pub fn get_console() -> &'static dyn drivers::Console {
    *CONSOLE.get()
}
