use core::arch::asm;

use super::Architecture;

pub struct Aarch64 {}

impl Architecture for Aarch64 {
    #[naked]
    #[no_mangle]
    unsafe extern "C" fn _start() -> ! {
        asm!("adrp x0, STACK_START", "mov sp, x0", "b k_main", options(noreturn));
    }
}
