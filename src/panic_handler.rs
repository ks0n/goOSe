use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    // FIXME: Add functionality to panic handler
    loop {}
}
