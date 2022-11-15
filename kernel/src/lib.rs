#![no_std]
#![feature(naked_functions)]
#![feature(fn_align)]

pub mod arch;
pub mod device_tree;
pub mod kernel_console;
pub mod mm;
pub mod paging;
mod utils;

// TODO: redo the unit tests with Mockall
// pub mod kernel_tests;

pub use drivers;

// TODO: cleanup how we handle features

cfg_if::cfg_if! {
    if  #[cfg(target_arch = "aarch64")] {
        pub use arch::aarch64::{PagingImpl, ArchImpl, InterruptsImpl};
        pub type ConsoleImpl = drivers::pl011::Pl011;
    } else if #[cfg(target_arch = "riscv64")] {
        pub use arch::riscv64::{PagingImpl, ArchImpl, InterruptsImpl};
        pub type ConsoleImpl = drivers::ns16550::Ns16550;

        pub mod interrupt_manager;
    }
}

static_assertions::assert_impl_all!(ArchImpl: arch::Architecture);
static_assertions::assert_impl_all!(InterruptsImpl: arch::ArchitectureInterrupts);
static_assertions::assert_impl_all!(PagingImpl: paging::PagingImpl);
