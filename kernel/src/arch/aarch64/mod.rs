use super::Architecture;
use super::ArchitectureInterrupts;
use core::arch::asm;
use cortex_a::{asm, registers::*};
use tock_registers::interfaces::Writeable;

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
}

impl Aarch64 {
    pub fn disable_fp_trap() {
        // Disable trapping of FP instructions.
        // CPACR_EL1.write(CPACR_EL1::FPEN::TrapNothing);
        CPACR_EL1.set(0b11 << 20);
    }
    pub unsafe fn init_el1_interrupts() {
        extern "Rust" {
            static el1_vector_table: core::cell::UnsafeCell<()>;
        }
        cortex_a::registers::VBAR_EL1.set(el1_vector_table.get() as u64);
    }
}

impl ArchitectureInterrupts for Aarch64 {
    fn new() -> Self {
        Self {}
    }

    fn init_interrupts(&mut self) {
        unsafe {
            Self::init_el1_interrupts();
        };
    }

    fn set_timer(&mut self, delay: usize) {
        CNTP_TVAL_EL0.set(delay as u64);

        unsafe { asm::barrier::isb(asm::barrier::SY) };

        CNTP_CTL_EL0.write(
            CNTP_CTL_EL0::ENABLE::SET + CNTP_CTL_EL0::IMASK::CLEAR + CNTP_CTL_EL0::ISTATUS::CLEAR,
        );
    }
}

core::arch::global_asm!(include_str!("exceptions.S"));

#[no_mangle]
extern "C" fn sync_current_el_sp0() {
    panic!("hit sync_current_el_sp0");
}

#[no_mangle]
extern "C" fn irq_current_el_sp0() {
    panic!("hit irq_current_el_sp0");
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
