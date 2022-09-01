//! Driver for the NS16550 UART chip.
//! The datasheet used to write this is: <http://caro.su/msx/ocm_de1/16550.pdf>

use super::Console;
use super::Driver;

const TRANSMITTER_HOLDING_REGISTER: usize = 0;
const _INTERRUPT_ENABLE_REGISTER: usize = 1;

pub struct Ns16550 {
    base_register_address: usize,
}

impl Ns16550 {
    pub const fn new(base_register_address: usize) -> Self {
        Self {
            base_register_address,
        }
    }

    pub fn _read(&self) -> u8 {
        self._read_transmitter_holding_reg()
    }

    pub fn _enable_data_ready_interrupt(&self) {
        // Data ready is the first bit of the Interrupt Enable Register
        unsafe {
            let addr = (self.base_register_address as *mut u8).add(_INTERRUPT_ENABLE_REGISTER);
            addr.write_volatile(1 << 0)
        }
    }

    fn write_transmitter_holding_reg(&self, byte: u8) {
        unsafe {
            let addr = (self.base_register_address as *mut u8).add(TRANSMITTER_HOLDING_REGISTER);
            addr.write_volatile(byte);
        }
    }

    fn _read_transmitter_holding_reg(&self) -> u8 {
        unsafe {
            let addr = (self.base_register_address as *mut u8).add(TRANSMITTER_HOLDING_REGISTER);
            addr.read_volatile()
        }
    }
}

impl Driver for Ns16550 {
    fn get_address_range(&self) -> Option<(usize, usize)> {
        // Base address + max register offset
        Some((self.base_register_address, 0b111))
    }
}

impl Console for Ns16550 {
    fn write(&mut self, data: &str) {
        for byte in data.bytes() {
            self.write_transmitter_holding_reg(byte);
        }
    }
}
