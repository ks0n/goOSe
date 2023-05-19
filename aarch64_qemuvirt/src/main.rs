#![no_std]
#![no_main]
#![feature(naked_functions)]

#[cfg(not(target_arch = "aarch64"))]
compile_error!("Must be compiled as aarch64");

use core::arch::asm;
use kernel::drivers::pl011::Pl011;

const DTB_ADDR: usize = 0x4000_0000;

#[no_mangle]
extern "C" fn k_main(_device_tree_ptr: usize) -> ! {
    kernel::hal::cpu::disable_fp_trapping();

    static PL011: Pl011 = Pl011::new(0x0900_0000);
    kernel::kernel_console::set_earlyinit_console(&PL011);

    kernel::kprintln!("hello, I am a goOSe! proud member of the gagelen !!!");

    unsafe {
        kernel::hal::irq::init_el1_exception_handlers();
    }

    unsafe {
        asm!("isb SY");
        asm!("dmb SY");
    }

    let device_tree = kernel::device_tree::DeviceTree::new(DTB_ADDR).unwrap();

    kernel::generic_main::generic_main(device_tree, &[&PL011]);

    unreachable!();
}
