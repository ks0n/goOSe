//! This module stores all drivers strictly necessary for the kernel.

#![no_std]

extern crate alloc;
use alloc::boxed::Box;

pub mod ns16550;
pub mod null_uart;
pub mod pl011;
pub mod qemuexit;

pub mod gicv2;

#[cfg(target_arch = "riscv64")]
pub mod plic;

pub trait Driver {
    fn get_address_range(&self) -> Option<(usize, usize)>;
}

pub trait Console: Driver {
    fn write(&self, data: &str);
}

struct ConsoleMatcher {
    compatibles: &'static [&'static str],
    constructor: fn(usize) -> Box<dyn Console + Send + Sync>,
}

impl ConsoleMatcher {
    fn matches(&self, compatible: &str) -> bool {
        self.compatibles
            .iter()
            .find(|&s| s == &compatible)
            .is_some()
    }
}

pub fn matching_console_driver(
    compatible: &str,
) -> Option<fn(usize) -> Box<dyn Console + Send + Sync>> {
    //Option<Box<dyn Console + Send + Sync>> {
    static MATCHERS: [&ConsoleMatcher; 2] = [&pl011::MATCHER, &ns16550::MATCHER];

    MATCHERS
        .iter()
        .find(|m| m.matches(compatible))
        .map(|some| some.constructor)
}
