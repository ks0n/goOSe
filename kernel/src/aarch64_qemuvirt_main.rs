#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::kernel_tests::runner)]
#![reexport_test_harness_main = "ktests_launch"]

#[cfg(not(target_arch = "aarch64"))]
compile_error!("Must be compiled as aarch64");

mod arch;
mod device_tree;
mod kernel_console;
#[cfg(test)]
mod kernel_tests;
mod mm;
mod paging;
mod utils;

use drivers::gicv2::GicV2;
use drivers::pl011::Pl011;

use core::arch::asm;

use paging::PagingImpl as _;

use cortex_a::asm;
use cortex_a::registers::*;
use tock_registers::interfaces::Writeable;

pub type ArchImpl = arch::aarch64::Aarch64;
pub type Architecture = arch::aarch64::Aarch64;
pub type InterruptsImpl = arch::aarch64::Aarch64;
pub type ConsoleImpl = Pl011;
pub type PagingImpl = arch::aarch64::pgt48::PageTable;

const DTB_ADDR: usize = 0x4000_0000;

#[no_mangle]
extern "C" fn k_main(_device_tree_ptr: usize) -> ! {
    // Disable trapping of FP instructions.
    // CPACR_EL1.write(CPACR_EL1::FPEN::TrapNothing);
    CPACR_EL1.set(0b11 << 20);

    kernel_console::init(Pl011::new(0x0900_0000));

    #[cfg(test)]
    {
        kernel_tests::init(DTB_ADDR);
        ktests_launch();
    }

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

    let device_tree_ptr = DTB_ADDR;
    let device_tree = device_tree::DeviceTree::new(device_tree_ptr);

    let pmm =
        mm::PhysicalMemoryManager::from_device_tree(&device_tree, PagingImpl::get_page_size());
    let mut mm = mm::MemoryManager::new(pmm);
    let pagetable = mm::map_address_space(
        &device_tree,
        &mut mm,
        &[crate::kernel_console::get_console()],
    );
    mm.set_kernel_pagetable(pagetable);

    // ???:
    // why do we pass the pagetable after into the mm ?
    //     (can we pass it at initialization)
    // - also when we map, we reload the page table, enabling the MMU (SCTLR_EL1.M)
    //   shouldn't that be separate ? ie map just put it in the pagetable, then we can manually reload it / enable paging ?

    kprintln!("Kernel has been initialized");

    mm.kernel_map(
        0x0900_0000,
        0x0450_0000,
        mm::Permissions::READ | mm::Permissions::WRITE,
    );
    let mut uart = Pl011::new(0x0450_0000);
    use drivers::Console;
    uart.write("Uart remaped");

    loop {}
}
