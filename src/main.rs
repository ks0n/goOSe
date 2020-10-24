#![no_std]
#![no_main]
#![feature(asm)]
#![feature(never_type)]
#![feature(custom_test_frameworks)]
#![feature(const_raw_ptr_to_usize_cast)]
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

    println!("Kernel Entry");

    print_sections_info();

    println!("{}", GREET);

    loop {}
}

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
