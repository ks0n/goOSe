#[cfg(target_arch = "arm")]
mod arm32;
#[cfg(target_arch = "riscv64")]
mod riscv64;

use cfg_if::cfg_if;

use crate::mm;

#[cfg(target_arch = "riscv64")]
pub type MemoryImpl = riscv64::sv39::PageTable;
#[cfg(target_arch = "riscv64")]
pub type InterruptsImpl = riscv64::interrupts::Interrupts;

pub fn new_arch(info: usize) -> impl Architecture {
    cfg_if! {
        if #[cfg(target_arch = "riscv64")] {
            riscv64::Riscv64::new(info)
        } else if #[cfg(target_arch = "arm")] {
            arm32::Arm32::new()
        } else {
            core::compile_error!("Architecture not supported! Did you run `gen_cargo.sh`?");
        }
    }
}

pub trait Architecture {
    unsafe extern "C" fn _start() -> !;

    fn new(info: usize) -> Self;

    fn for_all_memory_regions<F: FnMut(&mut dyn Iterator<Item = (usize, usize)>)>(&self, f: F);
    fn for_all_reserved_memory_regions<F: FnMut(&mut dyn Iterator<Item = (usize, usize)>)>(
        &self,
        f: F,
    );
}

pub trait ArchitectureMemory {
    fn new<'alloc>(allocator: &mut impl mm::PageAllocator) -> &'alloc mut Self;
    fn get_page_size() -> usize;
    fn map(
        &mut self,
        allocator: &mut impl mm::PageAllocator,
        to: usize,
        from: usize,
        perms: mm::Permissions,
    );
    fn reload(&mut self);
}

pub trait ArchitectureInterrupts {
    fn new() -> Self;
    fn init_interrupts(&self);
}
