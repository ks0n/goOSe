#![no_std]
#![feature(fn_align)]
#![feature(naked_functions)]
#![feature(once_cell)]

pub mod cpu;
pub mod irq;
pub mod mm;
mod plic;
mod registers;

use core::arch::asm;

pub fn panic_info() -> () {
    ()
}

#[naked]
#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    asm!("la sp, STACK_START", "call k_main", options(noreturn));
}
