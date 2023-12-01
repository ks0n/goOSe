//! This module stores all drivers strictly necessary for the kernel.

extern crate alloc;
use alloc::boxed::Box;

pub mod ns16550;
pub mod null_uart;
pub mod pl011;
pub mod qemuexit;

use crate::Error;
use fdt::standard_nodes::MemoryRegion;

pub trait Driver {
    fn get_address_range(&self) -> Option<(usize, usize)>;
}

pub trait Console: Driver {
    fn write(&self, data: &str);
}

pub struct Matcher<T: ?Sized> {
    pub compatibles: &'static [&'static str],
    pub constructor: fn(&mut dyn Iterator<Item = MemoryRegion>) -> Result<Box<T>, Error>,
}

impl<T: ?Sized> Matcher<T> {
    pub fn matches(&self, compatible: &str) -> bool {
        self.compatibles.iter().any(|s| s == &compatible)
    }
}
type ConsoleMatcher = Matcher<dyn Console + Send + Sync>;

pub const CONSOLE_MATCHERS: &[&ConsoleMatcher] = &[&pl011::MATCHER, &ns16550::MATCHER];
