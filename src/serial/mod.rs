use core::fmt;

use crate::arch;

/*
const THR_OFF: u16 = 0x0;
const RBR_OFF: u16 = 0x0;
*/
const DLL_OFF: u16 = 0x0;
const DLH_OFF: u16 = 0x1;
/*
const IER_OFF: u16 = 0x1;
const IIR_OFF: u16 = 0x2;
const FCR_OFF: u16 = 0x2;
const LCR_OFF: u16 = 0x3;
const MCR_OFF: u16 = 0x4;
const LSR_OFF: u16 = 0x5;
const MSR_OFF: u16 = 0x6;
const SR_OFF: u16 = 0x7;
*/

// FIXME: Remove use of static mut
static SERIAL_PORT: Serial = Serial { port: arch::UART0 };

pub struct Serial {
    port: usize,
}

impl Serial {
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
    unsafe {
        SERIAL_PORT.write_fmt(args).unwrap();
    }
}
