/// The only role of the ARM32 module is to provide the startup function, which is
/// already implemented in the `cortex_m` crate. Later on, the [`Arm32`] crate may
/// keep references or instances of different devices available on the various platforms

use crate::Architecture;

use cortex_m_rt::entry;

pub struct Arm32;

impl Arm32 {
    pub fn new() -> Arm32 {
        Arm32 {}
    }
}

#[entry]
#[doc(hidden)]
fn start() -> ! {
    crate::k_main()
}

impl Architecture for Arm32 {
    /// This function isn't actually used for the ARM32 platform, as we rely instead
    /// of the #[entry] proc macro provided by the cortex_m crate
    unsafe extern "C" fn _start() -> ! {
        loop {}
    }

    fn init_interrupts(&mut self) {}
}
