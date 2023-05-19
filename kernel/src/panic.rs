use crate::hal;
use core::arch::asm;
use log::error;

#[panic_handler]
#[cfg(not(test))]
fn panic(info: &core::panic::PanicInfo) -> ! {
    error!("\x1b[31mkernel panic\x1b[0m: {}", info);

    error!("hal panic info: {:X?}", hal::panic_info());

    loop {
        unsafe { asm!("wfi") }
    }
}
