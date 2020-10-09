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

    // println!(
    //     "START: {:#X} -> {:#X}",
    //     unsafe { arch::START_START },
    //     unsafe { arch::START_END }
    // );

    println!(
        "TEXT: {:#X} -> {:#X}",
        unsafe { arch::TEXT_START },
        unsafe { arch::TEXT_END }
    );

    println!("{}", GREET);

    loop {}
}
