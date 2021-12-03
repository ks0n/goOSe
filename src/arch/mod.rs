#[cfg(target_arch = "arm")]
mod arm32;
#[cfg(target_arch = "riscv64")]
mod riscv64;

use cfg_if::cfg_if;

use crate::mm;

#[cfg(target_arch = "riscv64")]
pub type MemoryImpl = riscv64::sv39::PageTable;

pub fn new_arch() -> impl Architecture {
    cfg_if! {
        if #[cfg(target_arch = "riscv64")] {
            riscv64::Riscv64::new()
        } else if #[cfg(target_arch = "arm")] {
            arm32::Arm32::new()
        } else {
            core::compile_error!("Architecture not supported! Did you run `gen_cargo.sh`?");
        }
    }
}

pub trait Architecture {
    unsafe extern "C" fn _start() -> !;

    fn init_interrupts(&mut self);
}

pub trait ArchitectureMemory {
    fn new<'alloc>(allocator: &mut mm::SimplePageAllocator<'alloc>) -> &'alloc mut Self;
    fn get_page_size() -> usize;
    fn map(
        &mut self,
        allocator: &mut mm::SimplePageAllocator,
        to: usize,
        from: usize,
        perms: mm::Permissions,
    );
    fn reload(&mut self);
}
