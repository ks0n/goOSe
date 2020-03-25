#![no_std]
#![no_main]

mod panic_handler;

static GREET: &[u8] = b"Talk to me, Goose";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer_addr = 0xb8000 as *mut u8;

    unsafe {
        for character in GREET {
            core::ptr::write_volatile(vga_buffer_addr, *character as u8);
        }
    }

    loop {}
}
