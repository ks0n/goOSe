//! This module stores all drivers strictly necessary for the kernel.

#![no_std]

pub mod ns16550;
pub mod pl011;
pub mod qemuexit;

pub mod gicv2;

#[cfg(target_arch = "riscv64")]
pub mod plic;

pub trait Driver {
    fn get_address_range(&self) -> Option<(usize, usize)>;
}

pub trait Console {
    fn write(&mut self, data: &str);
}
