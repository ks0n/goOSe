//! The panic handler is called when code aborts or encounters a panic expression.
//! The aim of the panic handler is simple: Showing some information about the error,
//! and making the panic easier to debug, for example by showing a backtrace of the
//! execution

#[cfg(not(test))]
use crate::println;
#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
#[doc(hidden)]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}
