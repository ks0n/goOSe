#![no_std]

#![feature(once_cell)]

use cortex_a::{asm, registers::*};
use tock_registers::interfaces::{Readable, ReadWriteable, Writeable};


pub mod mm;
pub mod irq;

mod devices;

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

pub mod cpu {
    use cortex_a::{asm, registers::*};
    use tock_registers::interfaces::{ReadWriteable, Writeable};

    pub fn disable_fp_trapping() {
        // Disable trapping of FP instructions.
        // CPACR_EL1.write(CPACR_EL1::FPEN::TrapNothing);
        CPACR_EL1.set(0b11 << 20);
    }

    pub fn set_physical_timer(delay: usize) {
        CNTP_TVAL_EL0.set(delay as u64);

        unsafe { asm::barrier::isb(asm::barrier::SY) };

        CNTP_CTL_EL0.write(
            CNTP_CTL_EL0::ENABLE::SET + CNTP_CTL_EL0::IMASK::CLEAR + CNTP_CTL_EL0::ISTATUS::CLEAR,
        );
    }

    pub fn clear_physical_timer() {
        CNTP_CTL_EL0.modify(CNTP_CTL_EL0::ENABLE::CLEAR);
    }

    pub fn unmask_interrupts() {
        DAIF.write(DAIF::A::Unmasked + DAIF::I::Unmasked + DAIF::F::Unmasked);
    }
}
