#![no_std]
#![no_main]

#![feature(asm)]
#![feature(custom_test_frameworks)]

#![reexport_test_harness_main = "utests_launch"]

mod panic_handler;
mod vga;
mod utest;

static GREET: &str = "Talk to me, Goose !";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    utests_launch();

    vga::write(GREET);

    loop {}
}
