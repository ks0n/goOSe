#![no_main]
#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]
#![feature(fn_align)]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[no_mangle]
#[repr(align(0x1000))]
pub extern "C" fn _start() -> u8 {
    0u8
}
