#![no_std]
#![no_main]
#![feature(asm)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::utest::runner)]
#![reexport_test_harness_main = "utests_launch"]
#![allow(dead_code)] // FIXME: Eww

mod asm_wrappers;
mod gdt;
mod serial;
mod utest;
mod vga;

use core::panic::PanicInfo;

static GREET: &str = "Talk to me, Goose !";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    utests_launch();

    println!("{}", GREET);

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

mod tests {
    // FIXME: Add test for invalid characters: vga::write("HeWörld");
}
