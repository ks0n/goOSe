#[cfg(target_arch = "aarch64")]
pub mod aarch64;
#[cfg(target_arch = "arm")]
pub mod arm32;
#[cfg(target_arch = "riscv64")]
pub mod riscv64;

pub trait Architecture {
    unsafe extern "C" fn _start() -> !;
    fn get_core_local_storage() -> &'static mut PerCoreContext;
    fn set_core_local_storage(p: &mut PerCoreContext);
}

/// Each core has its own copy of this, accessible using Architecutre::{get_core_local_storage, set_core_local_storage}.
/// Technically accesses to irq_manager should be protected with a Mutex.
pub struct PerCoreContext {
    pub coreid: u32,
}

pub trait ArchitectureInterrupts {
    fn init_interrupts(&mut self);
    fn set_timer(&mut self, delay: usize);
}
