#![no_std]
#![no_main]
#![feature(asm)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::utest::runner)]
#![reexport_test_harness_main = "utests_launch"]
#![allow(dead_code)] // FIXME: Eww

mod arch;
mod gdt;
mod panic;
mod serial;
mod utest;
mod vga;

static GREET: &str = "Talk to me, Goose !";

#[no_mangle]
fn kmain () -> !
{
    // #[cfg(test)]
    // utests_launch();

    println!("{}", GREET);

    arch::busy_loop();
}
