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
mod kernel_serial;
mod mm;
mod utils;

#[cfg(test)]
mod kernel_tests;

use drivers::ns16550::*;
use drivers::plic;

use arch::Architecture;

#[no_mangle]
fn k_main() -> ! {
    #[cfg(test)]
    ktests_launch();

    let mut arch = arch::new_arch();

    kprintln!("GoOSe is booting");

    arch.init_interrupts();

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

    loop {
        unsafe {
            asm!("wfi");
        }
    }
}
