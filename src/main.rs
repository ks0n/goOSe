//! GoOSe aims to be a generic OS for embedded devices

#![no_std]
#![no_main]
#![feature(asm)]
#![warn(missing_docs)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::utest::runner)]
#![reexport_test_harness_main = "utests_launch"]

cfg_if! {
    if #[cfg(test)] {
        #[macro_use]
        mod utest;
    }
}

mod arch;
mod panic;
mod serial;
mod utils;

use cfg_if::cfg_if;

#[doc(hidden)]
static GREET: &str = "Talk to me, Goose !";

/// After all architecture specific initialization for correct rust execution is done,
/// this is the "real" kernel entry point.
pub fn kmain() -> ! {
    #[cfg(test)]
    utests_launch();

    println!("Kernel Entry");

    print_sections_info();

    println!("{}", GREET);

    loop {}
}

/// Print some infos about our diferent sections.
fn print_sections_info() {
    // println!(
    //     "START: {:p} -> {:p}",
    //     unsafe { &arch::START_START },
    //     unsafe { &arch::START_END }
    // );
    println!("TEXT: {:p} -> {:p}", unsafe { &arch::TEXT_START }, unsafe {
        &arch::TEXT_END
    });
    println!("DATA: {:p} -> {:p}", unsafe { &arch::DATA_START }, unsafe {
        &arch::DATA_END
    });
    println!(
        "STACK: {:p} -> {:p}",
        unsafe { &arch::STACK_START },
        unsafe { &arch::STACK_END }
    );
}
