use core::fmt;

use crate::arch;

pub static COM1: u16 = 0x3f8;

/* Avoid unused code, but might be useful later
pub static COM2: u16 = 0x2f8;
pub static COM3: u16 = 0x3e8;
pub static COM4: u16 = 0x2e8;

pub static THR_OFF: u16 = 0x0;
pub static RBR_OFF: u16 = 0x0;
*/
pub static DLL_OFF: u16 = 0x0;
pub static DLH_OFF: u16 = 0x1;
/*
pub static IER_OFF: u16 = 0x1;
pub static IIR_OFF: u16 = 0x2;
pub static FCR_OFF: u16 = 0x2;
pub static LCR_OFF: u16 = 0x3;
pub static MCR_OFF: u16 = 0x4;
pub static LSR_OFF: u16 = 0x5;
pub static MSR_OFF: u16 = 0x6;
pub static SR_OFF: u16 = 0x7;
*/

// FIXME: Remove use of static mut
static mut SERIAL_PORT: Serial = Serial { port: COM1 };

pub struct Serial {
    port: u16,
}

impl Serial {
    pub fn init(port: u16) -> Serial {
        let new_s = Serial { port: port };

        arch::outb(port + 3, 0b01000000);

        /* We initialize the Baude Rate of the port to 38400 bps */
        arch::outb(port + DLL_OFF, 0x3);
        arch::outb(port + DLH_OFF, 0x0);

        new_s
    }

    fn _write_str(&self, data: &str) {
        for byte in data.bytes() {
            if byte == '\n' as u8 {
                arch::outb(self.port, '\r' as u8);
                arch::outb(self.port, '\n' as u8);
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

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::serial::print_fmt(format_args!($($arg)*)))
}

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
