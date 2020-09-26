#![no_std]
#![no_main]
#![feature(asm)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::utest::runner)]
#![reexport_test_harness_main = "utests_launch"]
#![allow(dead_code)] // FIXME: Eww

mod arch;
mod panic;
mod serial;
mod utest;

extern crate rlibc;

static GREET: &str = "Talk to me, Goose !";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    utests_launch();

    println!("{}", GREET);

    loop {}
}
