use core::arch::asm;
use core::fmt::{self, Write};

use crate::drivers::ns16550::{Ns16550, QEMU_VIRT_BASE_ADDRESS};

static mut KERNEL_SERIAL: Option<Ns16550> = None;

fn check_init() {
    unsafe {
        match &KERNEL_SERIAL {
            None => {
                KERNEL_SERIAL = Some(Ns16550::new(QEMU_VIRT_BASE_ADDRESS));
            }
            Some(_) => {}
        }
    }
}

fn get() -> &'static Ns16550 {
    check_init();
    unsafe { KERNEL_SERIAL.as_ref().unwrap() }
}

fn write(data: &str) {
    get().write(data);
}

struct KernelSerialWriter;

impl fmt::Write for KernelSerialWriter {
    fn write_str(&mut self, data: &str) -> fmt::Result {
        write(data);

        Ok(())
    }
}

pub fn print_fmt(args: fmt::Arguments) {
    KernelSerialWriter.write_fmt(args).unwrap()
}

#[panic_handler]
#[cfg(not(test))]
fn panic(info: &core::panic::PanicInfo) -> ! {
    crate::kprintln!("\x1b[31mkernel panic\x1b[0m: {}", info);

    loop {
        unsafe { asm!("wfi") }
    }
}

#[macro_export]
macro_rules! kprint {
    ($($args:tt)*) => ($crate::kernel_serial::print_fmt(format_args!($($args)*)))
}

#[macro_export]
macro_rules! kprintln {
    () => ($crate::kprint!("\r\n"));
    ($($args:tt)*) => ($crate::kprint!("{}\r\n", format_args!($($args)*)))
}

#[macro_export]
macro_rules! dbg {
    () => {
        $crate::kprintln!("[{}:{}]", core::file!(), core::line!())
    };
    ($expr:expr) => {
        $crate::kprintln!(
            "[{}:{}] {} = {:#?}",
            core::file!(),
            core::line!(),
            core::stringify!($expr),
            &$expr
        )
    };
}
