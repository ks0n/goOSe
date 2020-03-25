#![no_std]
#![no_main]

mod panic_handler;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
