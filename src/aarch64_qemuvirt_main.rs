#![no_std]
#![no_main]
#![feature(naked_functions)]

#[cfg(not(target_arch = "aarch64"))]
compile_error!("Must be compiled as aarch64");

mod arch;
mod drivers;
mod kernel_console;

use arch::Architecture;
use crate::drivers::pl011::Pl011;

use core::arch::asm;

use cortex_a::{asm, registers::*};
use tock_registers::interfaces::Writeable;

pub type ConsoleImpl = Pl011;

#[no_mangle]
extern "C" fn k_main(_device_tree_ptr: usize) -> ! {
    // Disable trapping of FP instructions.
    // CPACR_EL1.write(CPACR_EL1::FPEN::TrapNothing);
    CPACR_EL1.set(0b11 << 20);

    kernel_console::init(Pl011::new(0x0900_0000));

    kprintln!("Kernel has been initialized");

    loop {}
}
