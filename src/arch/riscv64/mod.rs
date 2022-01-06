use core::arch::asm;

use super::Architecture;
use crate::drivers::plic::plic_handler;

pub mod interrupts;
pub mod registers;
pub mod sv39;

pub struct Riscv64 {}

impl Riscv64 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Architecture for Riscv64 {
    #[naked]
    #[no_mangle]
    unsafe extern "C" fn _start() -> () {
        asm!("la sp, STACK_START", "call k_main", options(noreturn));
    }
}
