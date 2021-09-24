#![no_std]
#![no_main]
#![feature(asm)]
#![feature(naked_functions)]

mod arch;
mod drivers;

#[allow(unused_imports)]
use panic_halt;

#[no_mangle]
fn k_main() -> ! {
    let _arch = arch::new_arch();

    let serial = drivers::ns16550::Ns16550::new(drivers::ns16550::QEMU_VIRT_BASE_ADDRESS);
    serial.write("GoOSe is booting\n\r");

    loop {}
}
