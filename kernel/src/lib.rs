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

pub mod drivers;
mod utils;

pub mod error;
pub use error::Error;

pub mod arch;
pub mod device_tree;
pub mod driver_manager;
pub mod generic_main;
pub mod globals;
pub mod irq;
pub mod kernel_console;
mod lock;
pub mod mm;
pub mod paging;

// TODO: redo the unit tests with Mockall
// pub mod kernel_tests;

// TODO: cleanup how we handle features
cfg_if::cfg_if! {
    if  #[cfg(target_arch = "aarch64")] {
        // pub use arch::aarch64::{PagingImpl, ArchImpl};
        pub type ConsoleImpl = drivers::pl011::Pl011;
        pub use hal_aarch64 as hal;
    } else if #[cfg(target_arch = "riscv64")] {
        pub use arch::riscv64::{PagingImpl, ArchImpl};
        pub type ConsoleImpl = drivers::ns16550::Ns16550;

        // pub mod interrupt_manager;
    }
}

// static_assertions::assert_impl_all!(ArchImpl: arch::Architecture);
// static_assertions::assert_impl_all!(InterruptsImpl: arch::ArchitectureInterrupts);
// static_assertions::assert_impl_all!(PagingImpl: paging::PagingImpl);
// static_assertions::assert_impl_all!(ConsoleImpl: drivers::Console);
