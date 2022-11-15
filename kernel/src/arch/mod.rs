#[cfg(all(feature = "aarch64", feature = "riscv64"))]
compile_error!("Only a single architecture feature must enabled at once !");

#[cfg(target_arch = "aarch64")]
pub mod aarch64;

#[cfg(target_arch = "arm")]
pub mod arm32;

#[cfg(target_arch = "riscv64")]
pub mod riscv64;

pub trait Architecture {
    unsafe extern "C" fn _start() -> !;
}

pub trait ArchitectureInterrupts {
    fn new() -> Self;
    fn init_interrupts(&mut self);
    fn set_timer(&mut self, delay: usize);
}
