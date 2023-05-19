use core::fmt::{self, Write};

use crate::drivers::Console;
use crate::Error;
use crate::hal;

use alloc::sync::Arc;

use log::{error, Level, LevelFilter, Metadata, Record};
use spin::Mutex;

struct KernelConsole {
    earlyinit_console: Option<&'static (dyn Console + Sync)>,
    console: Option<Arc<dyn Console + Sync + Send>>,
}

impl KernelConsole {
    const fn new() -> Self {
        Self {
            earlyinit_console: None,
            console: None,
        }
    }
}

static KERNEL_CONSOLE: Mutex<KernelConsole> = Mutex::new(KernelConsole::new());

impl fmt::Write for KernelConsole {
    fn write_str(&mut self, data: &str) -> fmt::Result {
        if let Some(console) = &self.console {
            console.write(data);
        } else if let Some(earlyinit_console) = self.earlyinit_console {
            earlyinit_console.write(data);
        }
        // We should return an Error in the `else` case but it not like we can tell the user with a
        // print...

        Ok(())
    }
}

fn print_fmt(args: fmt::Arguments) {
    KERNEL_CONSOLE.lock().write_fmt(args).unwrap();
}

struct KernelLogger;

impl log::Log for KernelLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            // kprintln will call into the KERNEL_CONSOLE
            crate::kprintln!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static KERNEL_LOGGER: KernelLogger = KernelLogger;

pub fn init_logging() -> Result<(), Error> {
    log::set_logger(&KERNEL_LOGGER)?;
    log::set_max_level(LevelFilter::Trace);

    Ok(())
}

pub fn set_earlyinit_console(new_console: &'static (dyn Console + Sync)) {
    KERNEL_CONSOLE.lock().earlyinit_console = Some(new_console);
}

pub fn set_console(new_console: Arc<dyn Console + Sync + Send>) -> Result<(), Error> {
    KERNEL_CONSOLE.lock().console = Some(new_console);

    // TODO: return an error if the error already was some (unless we consider it is ok)
    Ok(())
}

#[panic_handler]
#[cfg(not(test))]
fn panic(info: &core::panic::PanicInfo) -> ! {
    error!("\x1b[31mkernel panic\x1b[0m: {}", info);

    error!("hal panic info: {:X?}", hal::panic_info());

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
