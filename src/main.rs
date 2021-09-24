#![no_std]
#![no_main]
#![feature(asm)]
#![feature(naked_functions)]

mod arch;
mod serial;

#[allow(unused_imports)]
use panic_halt;

use serial::Serial;

#[no_mangle]
fn k_main() -> ! {
    let _arch = arch::new_arch();

    let serial = Serial::new(0x10000000);
    serial.write("GoOSe is booting\n\r");

    loop {}
}
