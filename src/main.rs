#![no_std]
#![no_main]
#![feature(asm)]
#![feature(fn_align)]
#![feature(naked_functions)]

mod arch;
mod drivers;

#[allow(unused_imports)]
use panic_halt;

use arch::Architecture;

#[no_mangle]
fn k_main() -> ! {
    let mut arch = arch::new_arch();

    let serial = drivers::ns16550::Ns16550::new(drivers::ns16550::QEMU_VIRT_BASE_ADDRESS);
    serial.write("GoOSe is booting\n\r");

    arch.init_interrupts();

    serial.enable_data_ready_interrupt();

    let mut plic = drivers::plic::Plic::new(drivers::plic::QEMU_VIRT_PLIC_BASE_ADDRESS);
    plic.set_priority(10, 1);
    plic.enable_interrupt(10);
    plic.set_threshold(0);

    loop {
        unsafe {
            asm!("wfi");
        }
    }
}
