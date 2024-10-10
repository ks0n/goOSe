use super::Console;
use super::ConsoleMatcher;
use super::Driver;

use crate::utils::lock::Lock;

pub extern crate alloc;
use alloc::boxed::Box;

pub struct Pl011 {
    inner: Lock<Pl011Inner>,
}

struct Pl011Inner {
    base: usize,
}

impl Pl011Inner {
    pub const fn new(base: usize) -> Self {
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
        self.inner.lock(|pl011| Some((pl011.base, 0xFFC)))
    }
}

impl Console for Pl011 {
    fn write(&self, data: &str) {
        self.inner
            .lock(|pl011| data.bytes().for_each(|b| pl011.putc(b)));
    }
}

impl Pl011 {
    pub const fn new(base: usize) -> Self {
        Self {
            inner: Lock::new(Pl011Inner::new(base)),
        }
    }
}

pub(super) static MATCHER: ConsoleMatcher = ConsoleMatcher {
    compatibles: &["arm,pl011"],
    constructor: |reg| {
        Ok(Box::new(Pl011::new(
            reg.next().unwrap().starting_address as usize,
        )))
    },
};
