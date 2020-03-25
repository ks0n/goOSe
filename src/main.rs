#![no_std]
#![no_main]
#![feature(asm)]

mod panic_handler;

static GREET: &[u8] = b"Talk to me, Goose !";

#[no_mangle]
pub extern "C" fn _start() -> ! {
unsafe {
    let vga_buffer_addr = 0xb8000;
    let mut i = 0;

    for character in GREET {
        core::ptr::write_volatile((vga_buffer_addr + i) as *mut u8, *character as u8);
        core::ptr::write_volatile((vga_buffer_addr + i + 1) as *mut u8, 0xa);
        i += 2;
    }

    asm!("cli");

    loop {}
}
}
