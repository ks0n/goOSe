#![no_std]
#![no_main]

#[cfg(not(target_arch = "aarch64"))]
compile_error!("Must be compiled as aarch64");

use core::arch::asm;
use kernel::arch::aarch64::Aarch64;
use kernel::drivers::pl011::Pl011;

const DTB_ADDR: usize = 0x4000_0000;

#[no_mangle]
extern "C" fn k_main(_device_tree_ptr: usize) -> ! {
    kernel::arch::aarch64::Aarch64::disable_fp_trap();

    static PL011: Pl011 = Pl011::new(0x0900_0000);
    kernel::globals::set_earlyinit_console(&PL011);

    kernel::kprintln!("hello, I am a goOSe! proud member of the gagelen !!!");

    unsafe {
        kernel::arch::aarch64::Aarch64::init_el1_exception_handlers();
    }

    unsafe {
        asm!("isb SY");
        asm!("dmb SY");
    }

    let device_tree = kernel::device_tree::DeviceTree::new(DTB_ADDR).unwrap();

    kernel::generic_main::generic_main::<Aarch64>(device_tree, &[&PL011]);

    unreachable!();
}
