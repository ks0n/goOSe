use core::fmt::{self, Write};

use crate::utils::init_once::InitOnce;
use crate::utils::init_cell::InitCell;
use crate::drivers::null_uart::NullUart;
use crate::drivers::Console;
use crate::Error;
use crate::hal;

use alloc::sync::Arc;

use log::{error, Level, LevelFilter, Metadata, Record};

static NULL_CONSOLE: NullUart = NullUart::new();

pub static EARLYINIT_CONSOLE: InitCell<&'static (dyn Console + Sync)> =
    InitCell::new(&NULL_CONSOLE);
pub static CONSOLE: InitOnce<Arc<dyn Console + Sync + Send>> = InitOnce::new();

pub fn set_earlyinit_console(new_console: &'static (dyn Console + Sync)) {
    EARLYINIT_CONSOLE.set(|console| *console = new_console);
}

pub fn set_console(new_console: Arc<dyn Console + Sync + Send>) -> Result<(), Error> {
    CONSOLE.set(new_console)?;

    Ok(())
}

fn write(data: &str) {
    if CONSOLE.is_initialized() {
        // Safety: we know CONSOLE has something because it is initialized.
        CONSOLE.get().unwrap().write(data);
    } else {
        EARLYINIT_CONSOLE.get().write(data);
    }
}

#[derive(Debug)]
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

impl log::Log for KernelConsoleWriter {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Trace
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            crate::kprintln!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static KERNEL_CONSOLE_WRITER: KernelConsoleWriter = KernelConsoleWriter;

pub fn init_logging() -> Result<(), Error> {
    log::set_logger(&KERNEL_CONSOLE_WRITER)?;
    log::set_max_level(LevelFilter::Trace);

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
