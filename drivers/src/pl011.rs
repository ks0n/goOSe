use super::Console;
use super::Driver;

pub struct Pl011 {
    base: usize,
}

impl Pl011 {
    pub fn new(base: usize) -> Self {
        Self { base }
    }

    fn read_flag_register(&self) -> u32 {
        unsafe { ((self.base + 0x18) as *mut u32).read_volatile() }
    }

    fn tx_fifo_full(&self) -> bool {
        self.read_flag_register() & (1 << 5) > 0
    }

    fn write_data_register(&mut self, b: u8) {
        let dr = self.base as *mut u32;
        unsafe { dr.write_volatile(b.into()) }
    }

    pub fn putc(&mut self, b: u8) {
        while self.tx_fifo_full() {}

        self.write_data_register(b);
    }
}

impl Driver for Pl011 {
    fn get_address_range(&self) -> Option<(usize, usize)> {
        // Base address, max register offset
        Some((self.base, 0xFFC))
    }
}

impl Console for Pl011 {
    fn write(&mut self, data: &str) {
        data.bytes().for_each(|b| self.putc(b))
    }
}
