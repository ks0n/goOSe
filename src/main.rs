#![no_std]
#![no_main]

mod arch;

#[allow(unused_imports)]
use panic_halt;

#[no_mangle]
fn _start() -> ! {
    let _arch = arch::new_arch();
    loop {}
}
