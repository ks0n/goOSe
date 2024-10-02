#![no_std]
#![feature(naked_functions)]
#![feature(fn_align)]
#![feature(const_mut_refs)]
#![feature(slice_ptr_get)]
#![feature(const_ptr_as_ref)]
#![feature(const_slice_from_raw_parts_mut)]
#![feature(iterator_try_collect)]
#![feature(const_for)]
#![feature(alloc_error_handler)]
#![feature(trait_upcasting)]

pub extern crate alloc;

pub mod driver_manager;
pub mod drivers;
mod utils;

pub mod error;
pub use error::Error;

pub mod device_tree;
pub mod executable;
pub mod generic_main;
pub mod globals;
pub mod kernel_console;
pub mod mm;
mod panic;
mod tests;

// TODO: redo the unit tests with Mockall
// pub mod kernel_tests;

// TODO: cleanup how we handle features
cfg_if::cfg_if! {
    if #[cfg(target_arch = "aarch64")] {
        pub type ConsoleImpl = drivers::pl011::Pl011;

        use hal_aarch64::{
            mm::pgt48,
            irq::Aarch64Irqs,
            Aarch64CoreInfo,
        };
        use hal_core::Hal;
        pub static HAL: Hal<pgt48::PageTable, Aarch64Irqs, Aarch64CoreInfo> = Hal::new(Aarch64Irqs::new());
    } else if #[cfg(target_arch = "riscv64")] {
        pub type ConsoleImpl = drivers::ns16550::Ns16550;
        use hal_riscv64::{
            mm::sv39,
            irq::Riscv64Irqs,
            Riscv64CoreInfo,
        };
        use hal_core::Hal;
        pub static HAL: Hal<sv39::PageTable, Riscv64Irqs, Riscv64CoreInfo> = Hal::new(Riscv64Irqs::new());
    }
}
