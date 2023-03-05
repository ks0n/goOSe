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

pub struct Riscv64 {}

impl Riscv64 {
    pub fn new() -> Self {
        Self {}
    }

    pub fn init_trap_handlers(&mut self) {
        // Set the trap handler
        registers::set_stvec(interrupts::trap_dispatch as usize);
    }
}

impl Architecture for Riscv64 {
    #[naked]
    #[no_mangle]
    unsafe extern "C" fn _start() -> ! {
        asm!("la sp, STACK_START", "call k_main", options(noreturn));
    }

    fn set_timer(delay: usize) {
        let target_time = riscv::register::time::read() + delay;
        sbi::timer::set_timer(target_time as u64).unwrap();
    }

    fn disable_timer() {
        todo!()
    }

    fn unmask_interrupts() {
        registers::set_sstatus_sie();
        registers::set_sie_ssie();
        registers::set_sie_seie();
        registers::set_sie_stie();
    }
}
