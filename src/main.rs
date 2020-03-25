#![no_std]
#![no_main]
#![feature(asm)]

mod vga;
mod panic_handler;

static GREET: &[u8] = b"Talk to me, Goose !";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga::write_bytes(GREET);

    loop {}
}
