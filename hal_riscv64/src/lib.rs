#![no_std]
#![feature(fn_align)]
#![feature(naked_functions)]

pub mod irq;
pub mod mm;
mod plic;
mod registers;

use core::arch::naked_asm;

pub fn panic_info() {}

#[naked]
#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    naked_asm!("la sp, STACK_START", "call k_main");
}

pub struct Riscv64CoreInfo;

impl hal_core::CoreInfo for Riscv64CoreInfo {
    fn init(core_id: usize) {
        // The core_id is the value in the mhartid CSR we got from the machine-level firmware.
        registers::set_sscratch(core_id);
    }
    fn core_id() -> usize {
        // Early kernel code called Self::init and putthe core_id argument into the sscratch.
        registers::get_sscratch()
    }
}
