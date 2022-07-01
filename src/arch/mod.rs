#[cfg(target_arch = "arm")]
pub mod arm32;
#[cfg(target_arch = "riscv64")]
pub mod riscv64;
#[cfg(target_arch = "aarch64")]
pub mod aarch64;

pub trait Architecture {
    unsafe extern "C" fn _start() -> !;

    fn new(info: usize) -> Self;

    fn for_all_memory_regions<F: FnMut(&mut dyn Iterator<Item = (usize, usize)>)>(&self, f: F);
    fn for_all_reserved_memory_regions<F: FnMut(&mut dyn Iterator<Item = (usize, usize)>)>(
        &self,
        f: F,
    );
}

pub trait ArchitectureInterrupts {
    fn new() -> Self;
    fn init_interrupts(&mut self);
    fn set_timer(&mut self, delay: usize);
}
