#![no_std]
#![no_main]
#![feature(naked_functions)]

#[cfg(not(target_arch = "aarch64"))]
compile_error!("Must be compiled as aarch64");

use core::arch::asm;
use kernel::drivers::pl011::Pl011;

const DTB_ADDR: usize = 0x4000_0000;

const LAUNCH_TESTS: bool = cfg!(feature = "launch_tests");

use log::info;

unsafe fn disable_fp_trapping() {
    asm!("msr CPACR_EL1, {:x}", in(reg) 0b11 << 20)
}

#[no_mangle]
extern "C" fn k_main(_device_tree_ptr: usize) -> ! {
    unsafe {
        disable_fp_trapping();
    }

    static PL011: Pl011 = Pl011::new(0x0900_0000);
    kernel::kernel_console::set_earlyinit_console(&PL011);

    kernel::kernel_console::init_logging().unwrap();

    info!("hello, I am a goOSe! proud member of the gagelen !!!");

    kernel::HAL.init_irqs();

    unsafe {
        asm!("isb SY");
        asm!("dmb SY");
    }

    let device_tree = kernel::device_tree::DeviceTree::new(DTB_ADDR).unwrap();

    kernel::generic_main::generic_main::<LAUNCH_TESTS>(device_tree, &[&PL011]);
}
