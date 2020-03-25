use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // FIXME: Add functionality to panic handler
    loop {}
}
