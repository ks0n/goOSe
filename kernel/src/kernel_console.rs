use core::fmt::{self, Write};

use crate::globals;

fn write(data: &str) {
    if globals::CONSOLE.is_initialized() {
        // Safety: we know CONSOLE has something because it is initialized.
        globals::CONSOLE.get().unwrap().write(data);
    } else {
        globals::get_earlyinit_console().write(data);
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
    KernelConsoleWriter.write_fmt(args).unwrap();
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
