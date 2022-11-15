use core::arch::asm;

use super::Architecture;

pub mod interrupts;
pub mod registers;

cfg_if::cfg_if! {
    if  #[cfg(feature = "riscv64_sv39")] {
        pub mod sv39;
        pub type PagingImpl = sv39::PageTable;
    }
}

pub type ArchImpl = Riscv64;
pub type InterruptsImpl = interrupts::Interrupts;

pub struct Riscv64 {}

impl Riscv64 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Architecture for Riscv64 {
    #[naked]
    #[no_mangle]
    unsafe extern "C" fn _start() -> ! {
        asm!("la sp, STACK_START", "call k_main", options(noreturn));
    }
}
