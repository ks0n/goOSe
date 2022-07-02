//! This module stores all drivers strictly necessary for the kernel.

#![no_std]

pub mod ns16550;
pub mod pl011;

#[cfg(target_arch = "riscv64")]
pub mod plic;

pub trait Console {
    fn write(&mut self, data: &str);
}
