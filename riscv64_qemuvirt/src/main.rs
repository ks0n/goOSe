#![no_std]
#![no_main]
#![feature(naked_functions)]

#[cfg(not(target_arch = "riscv64"))]
compile_error!("Must be compiled as riscv64");

use kernel::drivers::ns16550::*;

use log::info;

pub const UART_ADDR: usize = 0x1000_0000;
pub const UART_INTERRUPT_NUMBER: u16 = 10;

const LAUNCH_TESTS: bool = cfg!(feature = "launch_tests");

#[no_mangle]
extern "C" fn k_main(core_id: usize, device_tree_ptr: usize) -> ! {
    kernel::HAL.init_core_info(core_id);
    kernel::HAL.init_irqs();

    static NS16550: Ns16550 = Ns16550::new(UART_ADDR);
    kernel::kernel_console::set_earlyinit_console(&NS16550);

    kernel::kernel_console::init_logging().unwrap();

    assert_eq!(
        core_id, 0,
        "Kernel must be booted on the first core with id == 0"
    );
    info!("GoOSe is booting");

    let device_tree = kernel::device_tree::DeviceTree::new(device_tree_ptr).unwrap();
    kernel::generic_main::generic_main::<LAUNCH_TESTS>(device_tree, &[&NS16550]);
}
