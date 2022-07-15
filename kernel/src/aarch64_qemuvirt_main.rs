#![no_std]
#![no_main]
#![feature(naked_functions)]

#[cfg(not(target_arch = "aarch64"))]
compile_error!("Must be compiled as aarch64");

mod arch;
mod kernel_console;

use drivers::gicv2::GicV2;
use drivers::pl011::Pl011;

use core::arch::asm;

use cortex_a::asm;
use cortex_a::registers::*;
use tock_registers::interfaces::Writeable;

pub type ConsoleImpl = Pl011;

#[no_mangle]
extern "C" fn k_main(_device_tree_ptr: usize) -> ! {
    // Disable trapping of FP instructions.
    // CPACR_EL1.write(CPACR_EL1::FPEN::TrapNothing);
    CPACR_EL1.set(0b11 << 20);

    kernel_console::init(Pl011::new(0x0900_0000));

    let mut gic = GicV2::new(0x8000000, 0x8010000);
    gic.enable(30); // Physical timer
    gic.enable_interrupts();

    unsafe {
        arch::aarch64::Aarch64::init_el1_interrupts();
    }

    unsafe {
        asm::barrier::isb(asm::barrier::SY);
        asm::barrier::dmb(asm::barrier::SY);
        asm::barrier::dsb(asm::barrier::SY);
    };

    kprintln!("Kernel has been initialized");

    if false {
        // IRQ
        DAIF.write(DAIF::D::Unmasked + DAIF::A::Unmasked + DAIF::I::Unmasked + DAIF::F::Unmasked);
        CNTP_CTL_EL0.write(
            CNTP_CTL_EL0::ENABLE::SET + CNTP_CTL_EL0::IMASK::CLEAR + CNTP_CTL_EL0::ISTATUS::CLEAR,
        );

        unsafe { asm!("msr CNTP_CVAL_EL0, xzr") };
        CNTP_TVAL_EL0.set(10000);
    } else {
        // Synchronous exception
        unsafe {
            asm!("svc 42");
        }
    }

    loop {}
}
