use super::Architecture;
use super::ArchitectureInterrupts;
use crate::globals;
use crate::irq::{self, Interrupt};
use core::arch::asm;
use cortex_a::{asm, registers::*};
use tock_registers::interfaces::{ReadWriteable, Writeable};

cfg_if::cfg_if! {
    if #[cfg(feature = "aarch64_pgt48oa")] {
        pub mod pgt48;
        pub type PagingImpl = pgt48::PageTable;
    }
}

pub type ArchImpl = Aarch64;
pub type InterruptsImpl = Aarch64;

pub struct Aarch64 {}

impl Architecture for Aarch64 {
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

    fn unmask_interrupts() {
        DAIF.write(DAIF::A::Unmasked + DAIF::I::Unmasked + DAIF::F::Unmasked);
    }

    fn set_timer(delay: usize) {
        CNTP_TVAL_EL0.set(delay as u64);

        unsafe { asm::barrier::isb(asm::barrier::SY) };

        CNTP_CTL_EL0.write(
            CNTP_CTL_EL0::ENABLE::SET + CNTP_CTL_EL0::IMASK::CLEAR + CNTP_CTL_EL0::ISTATUS::CLEAR,
        );
    }

    fn disable_timer() {
        CNTP_CTL_EL0.modify(CNTP_CTL_EL0::ENABLE::CLEAR);
    }
}

impl Aarch64 {
    pub fn disable_fp_trap() {
        // Disable trapping of FP instructions.
        // CPACR_EL1.write(CPACR_EL1::FPEN::TrapNothing);
        CPACR_EL1.set(0b11 << 20);
    }
    pub unsafe fn init_el1_exception_handlers() {
        extern "Rust" {
            static el1_vector_table: core::cell::UnsafeCell<()>;
        }
        cortex_a::registers::VBAR_EL1.set(el1_vector_table.get() as u64);
    }
}

core::arch::global_asm!(include_str!("exceptions.S"));

#[no_mangle]
extern "C" fn sync_current_el_sp0() {
    panic!("hit sync_current_el_sp0");
}

#[no_mangle]
extern "C" fn irq_current_el_sp0() {
    let irq_mgr = globals::IRQ_CHIP.get().unwrap();

    let int = irq_mgr.get_int().unwrap();

    match int {
        Interrupt::PhysicalTimer => {
            // Disable the timer in order to EOI it.
            Aarch64::disable_timer();

            irq::generic_timer_irq().unwrap();

            irq_mgr.clear_int(int);

            // Re-arm the timer.
            Aarch64::set_timer(50_000);
            CNTP_CTL_EL0.modify(CNTP_CTL_EL0::ENABLE::SET);
        }
    }
}
#[no_mangle]
extern "C" fn fiq_current_el_sp0() {
    panic!("hit fiq_current_el_sp0");
}

#[no_mangle]
extern "C" fn serror_current_el_sp0() {
    panic!("hit serror_current_el_sp0");
}

#[no_mangle]
extern "C" fn sync_current_el_spx() {
    panic!("hit sync_current_el_spx");
}

#[no_mangle]
extern "C" fn irq_current_el_spx() {
    panic!("hit irq_current_el_spx");
}

#[no_mangle]
extern "C" fn fiq_current_el_spx() {
    panic!("hit fiq_current_el_spx");
}

#[no_mangle]
extern "C" fn serror_current_el_spx() {
    panic!("hit serror_current_el_spx");
}

#[no_mangle]
extern "C" fn sync_lower_el() {
    panic!("hit sync_lower_el");
}

#[no_mangle]
extern "C" fn irq_lower_el() {
    panic!("hit irq_lower_el");
}

#[no_mangle]
extern "C" fn fiq_lower_el() {
    panic!("hit fiq_lower_el");
}

#[no_mangle]
extern "C" fn serror_lower_el() {
    panic!("hit serror_lower_el");
}

#[no_mangle]
extern "C" fn sync_lower_el_aarch32() {
    panic!("hit sync_lower_el_aarch32");
}

#[no_mangle]
extern "C" fn irq_lower_el_aarch32() {
    panic!("hit irq_lower_el_aarch32");
}

#[no_mangle]
extern "C" fn fiq_lower_el_aarch32() {
    panic!("hit fiq_lower_el_aarch32");
}

#[no_mangle]
extern "C" fn serror_lower_el_aarch32() {
    panic!("hit serror_lower_el_aarch32");
}
