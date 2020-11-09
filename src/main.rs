#![no_std]
#![no_main]
#![feature(asm)]
#![feature(never_type)]
#![feature(custom_test_frameworks)]
#![feature(const_raw_ptr_to_usize_cast)]
#![feature(const_in_array_repeat_expressions)]
#![test_runner(crate::utest::runner)]
#![reexport_test_harness_main = "utests_launch"]
#![allow(dead_code)] // FIXME: Eww

cfg_if! {
    if #[cfg(test)] {
        #[macro_use]
        mod utest;
        extern crate qemu_exit;
    }
}

#[macro_use]
extern crate lazy_static;

mod allocator;
mod arch;
mod panic;
mod serial;

use cfg_if::cfg_if;

pub fn kmain() -> ! {
    #[cfg(test)]
    utests_launch();

    println!("Kernel Entry");
    print_sections_info();

    // arch::mmu::new(unsafe { &arch::MMU as *const () } as usize);

    loop {}
}

fn print_sections_info() {
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
    println!("HEAP: {:p} -> {:p}", unsafe { &arch::HEAP_START }, unsafe {
        &arch::HEAP_END
    });
}
