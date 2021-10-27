#![no_std]
#![no_main]
#![feature(asm)]
#![feature(fn_align)]
#![feature(naked_functions)]

mod arch;
mod drivers;
mod kernel_serial;

extern crate panic_halt;

use drivers::ns16550::*;
use drivers::plic;

use arch::Architecture;

#[no_mangle]
fn k_main() -> ! {
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
    plic.register_handler(10, kernel_serial::interrupt_handler);

    loop {
        unsafe {
            asm!("wfi");
        }
    }
}
