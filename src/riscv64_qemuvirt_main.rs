#![no_std]
#![no_main]
#![feature(doc_cfg)]
#![feature(fn_align)]
#![feature(bench_black_box)]
#![feature(naked_functions)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::kernel_tests::runner)]
#![reexport_test_harness_main = "ktests_launch"]

#[cfg(not(target_arch = "riscv64"))]
compile_error!("Must be compiled as riscv64");

mod arch;
mod drivers;
mod executable;
mod interrupt_manager;
mod kernel_serial;
#[cfg(test)]
mod kernel_tests;
mod mm;
mod utils;

use core::arch::asm;
use drivers::ns16550::*;
use drivers::plic;

use arch::Architecture;
use arch::ArchitectureMemory;

pub type ArchImpl = arch::riscv64::Riscv64;
pub type InterruptsImpl = arch::riscv64::interrupts::Interrupts;
pub type MemoryImpl = arch::riscv64::sv39::PageTable;

#[no_mangle]
extern "C" fn k_main(_core_id: usize, device_tree_ptr: usize) -> ! {
    #[cfg(test)]
    {
        crate::kernel_tests::init(device_tree_ptr);
        ktests_launch();
    }

    let arch = arch::new_arch(device_tree_ptr);

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

    let mut pmm =
        mm::PhysicalMemoryManager::from_arch_info(&arch, crate::MemoryImpl::get_page_size());
    let page_table = crate::MemoryImpl::new(&mut pmm);
    mm::map_address_space(&arch, page_table, &mut pmm);

    kprintln!("[OK] Setup virtual memory");

    let mut interrupts = interrupt_manager::InterruptManager::new();
    interrupts.init_interrupts();

    kprintln!("[OK] Enable interrupts");

    loop {
        unsafe {
            asm!("wfi");
        }
    }
}
