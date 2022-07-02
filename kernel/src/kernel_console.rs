use core::fmt::{self, Write};

use drivers::Console;

pub static mut STDOUT_UART: Option<crate::ConsoleImpl> = None;


pub fn init(uart: crate::ConsoleImpl) {
    unsafe { STDOUT_UART = Some(uart) };
}

fn write(data: &str) {
    if let Some(console) = unsafe { &mut STDOUT_UART } {
        console.write(data);
    }
}

struct KernelConsoleWriter;

impl fmt::Write for KernelConsoleWriter {
    fn write_str(&mut self, data: &str) -> fmt::Result {
        write(data);

        Ok(())
    }
}

pub fn print_fmt(args: fmt::Arguments) {
    KernelConsoleWriter.write_fmt(args).unwrap_or(());
}

#[panic_handler]
#[cfg(not(test))]
fn panic(info: &core::panic::PanicInfo) -> ! {
    crate::kprintln!("\x1b[31mkernel panic\x1b[0m: {}", info);

    loop {
        use core::arch::asm;
        unsafe { asm!("wfi") }
    }
}

#[macro_export]
macro_rules! kprint {
    ($($args:tt)*) => ($crate::kernel_console::print_fmt(format_args!($($args)*)))
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
