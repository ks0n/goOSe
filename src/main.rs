#![no_std]
#![no_main]
#![feature(asm)]
#![feature(fn_align)]
#![feature(naked_functions)]

mod arch;
mod drivers;

extern crate panic_halt;

use drivers::ns16550::*;
use drivers::plic;

use arch::Architecture;

static mut KERNEL_SERIAL: Ns16550 = Ns16550::new(0);

fn serial_write(data: &str) {
    unsafe { KERNEL_SERIAL.write(data) };
}
fn serial_read() -> u8 {
    unsafe { KERNEL_SERIAL.read() }
}

fn serial_interrupt_handler() {
    let byte = serial_read();
    let arr = [byte; 1];
    serial_write(core::str::from_utf8(&arr).unwrap());
}

#[no_mangle]
fn k_main() -> ! {
    let mut arch = arch::new_arch();

    unsafe {
        KERNEL_SERIAL = Ns16550::new(QEMU_VIRT_BASE_ADDRESS);
    }

    serial_write("GoOSe is booting\n\r");

    arch.init_interrupts();

    // Enable Serial interrupts
    plic::init(plic::QEMU_VIRT_PLIC_BASE_ADDRESS);
    let plic = plic::get();
    if let Err(e) = plic.set_priority(QEMU_VIRT_NS16550_INTERRUPT_NUMBER, 1) {
        serial_write(e);
        serial_write("\n\r");
    }
    if let Err(e) = plic.enable_interrupt(QEMU_VIRT_NS16550_INTERRUPT_NUMBER, 0) {
        serial_write(e);
        serial_write("\n\r");
    }
    plic.set_threshold(0);

    unsafe {
        KERNEL_SERIAL.enable_data_ready_interrupt();
    }
    plic.register_handler(10, serial_interrupt_handler);

    loop {
        unsafe {
            asm!("wfi");
        }
    }
}
