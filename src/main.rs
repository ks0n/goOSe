#![no_std]
#![no_main]
#![feature(asm)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::utest::runner)]
#![reexport_test_harness_main = "utests_launch"]

mod asm_wrappers;
mod gdt;
mod panic_handler;
mod serial;
mod utest;
mod vga;

use core::fmt::Write;

static GREET: &str = "Talk to me, Goose !";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    utests_launch();

    /// FIXME
    // let mut vga_buffer = vga::buffer::Buffer::new();
    // vga::write(&mut vga_buffer, GREET);
    let mut serial = serial::Serial::init_com1();

    serial.write_str("Hey there, this is on serial\nNewlines !\n");

    write!(serial, "Hi there, this is serial");

    println!("Hey there, that's a macro");

    loop {}
}

mod tests {
    // FIXME: Add test for invalid characters: vga::write("HeWörld");
}
