#![no_main]
#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]
#![feature(fn_align)]

use core::panic::PanicInfo;
use core::arch::asm;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[inline(always)]
unsafe fn syscall(number: u32) -> ! {
    asm!("
        mv a7, {number}
        ecall
    ", number = in(reg) number, options(noreturn));
}

#[no_mangle]
#[repr(align(0x1000))]
pub extern "C" fn _start() -> ! {
    unsafe {syscall(60)};
}
