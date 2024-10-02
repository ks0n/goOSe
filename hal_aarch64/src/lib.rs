#![no_std]
#![feature(naked_functions)]
#![feature(fn_align)]

use cortex_a::registers::*;
use tock_registers::interfaces::Readable;

use core::arch::naked_asm;

pub mod cpu;
pub mod irq;
pub mod mm;

mod devices;

#[allow(dead_code)]
#[derive(Debug)]
pub struct PanicInfo {
    esr_el1: u64,
    elr_el1: u64,
    far_el1: u64,
}

pub fn panic_info() -> PanicInfo {
    PanicInfo {
        esr_el1: ESR_EL1.get(),
        elr_el1: ELR_EL1.get(),
        far_el1: FAR_EL1.get(),
    }
}

#[naked]
#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    naked_asm!(
        "
        adrp x9, STACK_START
        msr spsel, xzr
        mov sp, x9
        b k_main
        ",
    );
}

pub struct Aarch64CoreInfo;

impl hal_core::CoreInfo for Aarch64CoreInfo {
    fn init(_core_id: usize) {
        // We just read MPIDR_EL1 on aarch64.
    }

    fn core_id() -> usize {
        let mpidr = MPIDR_EL1.get() as usize;
        assert!(mpidr < usize::MAX);

        mpidr
    }
}
