#![no_std]
#![no_main]
#![feature(asm)]
#![feature(fn_align)]
#![feature(naked_functions)]

mod arch;
mod drivers;

extern crate panic_halt;

use drivers::ns16550::*;

use arch::Architecture;

#[no_mangle]
fn k_main() -> ! {
    let mut arch = arch::new_arch();

    let serial = Ns16550::new(QEMU_VIRT_BASE_ADDRESS);
    serial.write("GoOSe is booting\n\r");

    arch.init_interrupts();

    serial.enable_data_ready_interrupt();

    let mut plic = drivers::plic::Plic::new(drivers::plic::QEMU_VIRT_PLIC_BASE_ADDRESS);

    // Enable Serial interrupts
    if let Err(e) = plic.set_priority(QEMU_VIRT_NS16550_INTERRUPT_NUMBER, 1) {
        serial.write(e);
        serial.write("\n\r");
    }
    if let Err(e) = plic.enable_interrupt(QEMU_VIRT_NS16550_INTERRUPT_NUMBER, 0) {
        serial.write(e);
        serial.write("\n\r");
    }
    plic.set_threshold(0);

    loop {
        unsafe {
            asm!("wfi");
        }
    }
}
