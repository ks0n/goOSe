#![no_std]
#![no_main]
#![feature(asm)]

mod panic_handler;
mod vga;

static GREET: &str = "Talk to me, Goose !";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga::write(GREET);

    loop {}
}
