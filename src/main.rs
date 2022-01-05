#![no_std]
#![no_main]
#![feature(doc_cfg)]
#![feature(fn_align)]
#![feature(naked_functions)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::kernel_tests::runner)]
#![reexport_test_harness_main = "ktests_launch"]

mod arch;
mod drivers;
mod interrupt_manager;
mod kernel_serial;
mod mm;
mod utils;

#[cfg(test)]
mod kernel_tests;

use core::arch::asm;
use drivers::ns16550::*;
use drivers::plic;

#[no_mangle]
fn k_main(_core_id: usize, device_tree_ptr: usize) -> ! {
    #[cfg(test)]
    ktests_launch();

    let _arch = arch::new_arch();

    kprintln!("GoOSe is booting");

    // Enable Serial interrupts
    plic::init(plic::QEMU_VIRT_PLIC_BASE_ADDRESS);
    let plic = plic::get();
    if let Err(e) = plic.set_priority(QEMU_VIRT_NS16550_INTERRUPT_NUMBER, 1) {
        kprintln!("{}", e);
    }
    if let Err(e) = plic.enable_interrupt(QEMU_VIRT_NS16550_INTERRUPT_NUMBER, 0) {
        kprintln!("{}", e);
    }
    plic.set_threshold(0);

    let device_tree = unsafe { fdt::Fdt::from_ptr(device_tree_ptr as * const u8).unwrap() };
    let mut memory = mm::MemoryManager::<arch::MemoryImpl>::new(&device_tree);
    memory.map_address_space();

    kprintln!("[OK] Setup virtual memory");

    let interrupts = interrupt_manager::InterruptManager::<arch::InterruptsImpl>::new();
    interrupts.init_interrupts();

    kprintln!("[OK] Enable interrupts");

    loop {
        unsafe {
            asm!("wfi");
        }
    }
}
