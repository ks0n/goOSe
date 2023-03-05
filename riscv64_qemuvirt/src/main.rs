#![no_std]
#![no_main]
// #![feature(custom_test_frameworks)]
// #![test_runner(kernel::super::kernel_tests::runner)]
// #![reexport_test_harness_main = "ktests_launch"]

#[cfg(not(target_arch = "riscv64"))]
compile_error!("Must be compiled as riscv64");

use core::arch::asm;
use kernel::drivers::ns16550::*;
use kernel::drivers::plic;
use kernel::drivers::qemuexit::QemuExit;
use kernel::arch::riscv64::Riscv64;

pub const UART_ADDR: usize = 0x1000_0000;
pub const UART_INTERRUPT_NUMBER: u16 = 10;

#[no_mangle]
extern "C" fn k_main(_core_id: usize, device_tree_ptr: usize) -> ! {
    static NS16550: Ns16550 = Ns16550::new(UART_ADDR);
    kernel::globals::set_earlyinit_console(&NS16550);

    kernel::kprintln!("GoOSe is booting");

    #[cfg(test)]
    {
        kernel::kernel_tests::init(device_tree_ptr);
        ktests_launch();
    }

    let mut arch = kernel::arch::riscv64::Riscv64::new();
    arch.init_trap_handlers();
    let device_tree = kernel::device_tree::DeviceTree::new(device_tree_ptr).unwrap();
    kernel::generic_main::generic_main::<Riscv64>(device_tree, &[&NS16550]);

    unreachable!();

    // let qemu_exit = QemuExit::new();
    //
    // // Enable Serial interrupts
    // plic::init(plic::QEMU_VIRT_PLIC_BASE_ADDRESS);
    // let plic = plic::get();
    // if let Err(e) = plic.set_priority(UART_INTERRUPT_NUMBER, 1) {
    //     kernel::kprintln!("{}", e);
    // }
    // if let Err(e) = plic.enable_interrupt(UART_INTERRUPT_NUMBER, 0) {
    //     kernel::kprintln!("{}", e);
    // }
    // plic.set_threshold(0);
    //
    // kernel::globals::PHYSICAL_MEMORY_MANAGER
    //     .lock(|pmm| pmm.init_from_device_tree(&device_tree, 4096))
    //     .unwrap();
    // kernel::mm::map_address_space(&device_tree, &[&NS16550, &qemu_exit]);
    //
    // kernel::kprintln!("[OK] Setup virtual memory");
    //
    // let _drvmgr = kernel::driver_manager::DriverManager::with_devices(&device_tree).unwrap();
    //
    // let mut interrupts = kernel::interrupt_manager::InterruptManager::new();
    // interrupts.init_interrupts();
    //
    // kernel::kprintln!("[OK] Enable interrupts");
    //
    // kernel::kprintln!("[OK] GoOSe shuting down, bye bye!");
    // qemu_exit.exit_success();
}
