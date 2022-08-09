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
mod device_tree;
mod executable;
mod interrupt_manager;
mod kernel_console;
#[cfg(test)]
mod kernel_tests;
mod mm;
mod paging;
mod utils;

use core::arch::asm;
use drivers::ns16550::*;
use drivers::plic;

use arch::Architecture;
use paging::PagingImpl as _;

pub type ArchImpl = arch::riscv64::Riscv64;
pub type InterruptsImpl = arch::riscv64::interrupts::Interrupts;
pub type PagingImpl = arch::riscv64::sv39::PageTable;
pub type ConsoleImpl = Ns16550;

pub const UART_ADDR: usize = 0x1000_0000;
pub const UART_INTERRUPT_NUMBER: u16 = 10;

#[no_mangle]
extern "C" fn k_main(_core_id: usize, device_tree_ptr: usize) -> ! {
    kernel_console::init(Ns16550::new(UART_ADDR));

    kprintln!("GoOSe is booting");

    #[cfg(test)]
    {
        kernel_tests::init(device_tree_ptr);
        ktests_launch();
    }

    let arch = arch::riscv64::Riscv64::new();
    let device_tree = device_tree::DeviceTree::new(device_tree_ptr);

    // Enable Serial interrupts
    plic::init(plic::QEMU_VIRT_PLIC_BASE_ADDRESS);
    let plic = plic::get();
    if let Err(e) = plic.set_priority(UART_INTERRUPT_NUMBER, 1) {
        kprintln!("{}", e);
    }
    if let Err(e) = plic.enable_interrupt(UART_INTERRUPT_NUMBER, 0) {
        kprintln!("{}", e);
    }
    plic.set_threshold(0);

    let pmm =
        mm::PhysicalMemoryManager::from_device_tree(&device_tree, PagingImpl::get_page_size());
    let mut mm = mm::MemoryManager::new(pmm);
    let pagetable = mm::map_address_space(
        &device_tree,
        &mut mm,
        &[crate::kernel_console::get_console()],
    );
    mm.set_kernel_pagetable(pagetable);

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
