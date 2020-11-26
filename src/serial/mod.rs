//! The serial modules communicates over the architecture's serial port. It is defined
//! in the arch module, and is architecture dependent. Serial is used for basic text
//! input and output.

use crate::arch;
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

const DLL_OFF: u16 = 0x0;
const DLH_OFF: u16 = 0x1;

lazy_static! {
    static ref SERIAL_PORT: Mutex<Serial> = Mutex::new(Serial::init(arch::UART0));
}

/// Serial struct used to handle communication over a specific port
pub struct Serial {
    port: usize,
}

impl Serial {
    /// Initialize Serial communication over the given prot
    pub fn init(port: usize) -> Serial {
        let new_s = Serial { port };

        arch::outb(port + 3, 0b01000000);

        /* We initialize the Baude Rate of the port to 38400 bps */
        arch::outb(port + (DLL_OFF as usize), 0x3);
        arch::outb(port + (DLH_OFF as usize), 0x0);

        new_s
    }

    fn _write_str(&self, data: &str) {
        for byte in data.bytes() {
            if byte == b'\n' {
                arch::outb(self.port, b'\r');
                arch::outb(self.port, b'\n');
            } else {
                arch::outb(self.port, byte);
            }
        }
    }
}

impl fmt::Write for Serial {
    fn write_str(&mut self, data: &str) -> fmt::Result {
        self._write_str(data);
        Ok(())
    }
}

/// Prints to serial port
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::serial::print_fmt(format_args!($($arg)*)))
}

/// Prints to serial port with a newline
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn print_fmt(args: fmt::Arguments) {
    use core::fmt::Write;

    // FIXME: Change from mut static to lazy_static! or Mutex
    SERIAL_PORT.lock().write_fmt(args).unwrap();
}
