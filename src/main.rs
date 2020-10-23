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

cfg_if! {
    if #[cfg(test)] {
        mod utest;
        extern crate qemu_exit;
    }
}

use cfg_if::cfg_if;

static GREET: &str = "Talk to me, Goose !";

pub fn kmain() -> ! {
    #[cfg(test)]
    utests_launch();

    println!("{}", GREET);

    let entry = arch::mmu::PageEntry::new();

    loop {}
}
