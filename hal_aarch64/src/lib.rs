#![no_std]
#![feature(naked_functions)]

use cortex_a::registers::*;
use tock_registers::interfaces::Readable;

use core::arch::asm;

pub mod cpu;
pub mod irq;
pub mod mm;
pub mod context;

mod devices;

#[allow(dead_code)]
#[derive(Debug)]
pub struct PanicInfo {
    esr_el1: u64,
    elr_el1: u64,
}

pub fn panic_info() -> PanicInfo {
    PanicInfo {
        esr_el1: ESR_EL1.get(),
        elr_el1: ELR_EL1.get(),
    }
}

#[naked]
#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    asm!(
        "
        adrp x9, STACK_START
        msr spsel, xzr
        mov sp, x9
        b k_main
        ",
        options(noreturn)
    );
}
