//! GoOSe aims to be a generic OS for embedded devices

#![no_std]
#![no_main]
#![feature(asm)]
#![feature(global_asm)]
#![feature(naked_functions)]
#![warn(missing_docs)]
#![feature(custom_test_frameworks)]
#![feature(default_alloc_error_handler)]
#![test_runner(crate::utest::runner)]
#![reexport_test_harness_main = "utests_launch"]

cfg_if! {
    if #[cfg(test)] {
        #[macro_use]
        mod utest;
    }
}

extern crate alloc;

mod allocator;
mod arch;
mod panic;
mod serial;
mod utils;

use alloc::vec::Vec;
use cfg_if::cfg_if;

/// After all architecture specific initialization for correct rust execution is done,
/// this is the "real" kernel entry point.
pub fn kmain() -> ! {
    #[cfg(test)]
    utests_launch();

    println!("Kernel Entry");

    print_sections_info();

    allocator::init();
    println!("Allocator initialized");

    let mut vec: Vec<usize> = Vec::new();
    for i in 0..5 {
        vec.push(i as usize);
    }

    for i in 0..vec.len() {
        println!("vec[{}] = {}", i, vec[i]);
    }

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
    println!("BSS: {:p} -> {:p}", unsafe { &arch::BSS_START }, unsafe {
        &arch::BSS_END
    });
    println!("HEAP: {:p} -> {:p}", unsafe { &arch::HEAP_START }, unsafe {
        &arch::HEAP_END
    });
    println!(
        "STACK: {:p} -> {:p}",
        unsafe { &arch::STACK_START },
        unsafe { &arch::STACK_END }
    );
}
