#![no_std]
#![no_main]
#![feature(asm)]
#![feature(fn_align)]
#![feature(naked_functions)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::kernel_tests::runner)]
#![reexport_test_harness_main = "ktests_launch"]

mod arch;
mod drivers;
mod interrupt_manager;
mod kernel_serial;
mod mm;
mod utils;

#[cfg(test)]
mod kernel_tests;

use drivers::ns16550::*;
use drivers::plic;

#[no_mangle]
fn k_main() -> ! {
    #[cfg(test)]
    ktests_launch();

    let _arch = arch::new_arch();

    kprintln!("GoOSe is booting");

    // Enable Serial interrupts
    plic::init(plic::QEMU_VIRT_PLIC_BASE_ADDRESS);
    let plic = plic::get();
    if let Err(e) = plic.set_priority(QEMU_VIRT_NS16550_INTERRUPT_NUMBER, 1) {
        kprintln!("{}", e);
    }
    if let Err(e) = plic.enable_interrupt(QEMU_VIRT_NS16550_INTERRUPT_NUMBER, 0) {
        kprintln!("{}", e);
    }
    plic.set_threshold(0);

    let mut memory = mm::MemoryManager::<arch::MemoryImpl>::new();
    memory.map_address_space();

    kprintln!("[OK] Setup virtual memory");

    let interrupts = interrupt_manager::InterruptManager::<arch::InterruptsImpl>::new();
    interrupts.init_interrupts();

    kprintln!("[OK] Enable interrupts");

    loop {
        unsafe {
            asm!("wfi");
        }
    }
}
