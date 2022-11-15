#![no_std]
#![no_main]
// #![feature(custom_test_frameworks)]
// #![test_runner(crate::kernel_tests::runner)]
// #![reexport_test_harness_main = "ktests_launch"]

#[cfg(not(target_arch = "aarch64"))]
compile_error!("Must be compiled as aarch64");

use kernel::drivers::gicv2::GicV2;
use kernel::drivers::pl011::Pl011;
use kernel::drivers::Console;

use core::arch::asm;

use cortex_a::asm;

const DTB_ADDR: usize = 0x4000_0000;

#[no_mangle]
extern "C" fn k_main(_device_tree_ptr: usize) -> ! {
    kernel::arch::aarch64::Aarch64::disable_fp_trap();

    kernel::kernel_console::init(Pl011::new(0x0900_0000));
    kernel::kprintln!("hello!");

    #[cfg(test)]
    {
        kernel_tests::init(DTB_ADDR);
        ktests_launch();
    }

    let mut gic = GicV2::new(0x8000000, 0x8010000);
    gic.enable(30); // Physical timer
    gic.enable_interrupts();

    unsafe {
        kernel::arch::aarch64::Aarch64::init_el1_interrupts();
    }

    unsafe {
        asm::barrier::isb(asm::barrier::SY);
        asm::barrier::dmb(asm::barrier::SY);
        asm::barrier::dsb(asm::barrier::SY);
    };

    let device_tree_ptr = DTB_ADDR;
    let device_tree = kernel::device_tree::DeviceTree::new(device_tree_ptr);
    //
    let pmm = kernel::mm::PhysicalMemoryManager::from_device_tree(&device_tree, 4096);
    let mut mm = kernel::mm::MemoryManager::new(pmm);
    let pagetable = kernel::mm::map_address_space(
        &device_tree,
        &mut mm,
        &[kernel::kernel_console::get_console()],
    );
    mm.set_kernel_pagetable(pagetable);

    // ???:
    // why do we pass the pagetable after into the mm ?
    //     (can we pass it at initialization)
    // - also when we map, we reload the page table, enabling the MMU (SCTLR_EL1.M)
    //   shouldn't that be separate ? ie map just put it in the pagetable, then we can manually reload it / enable paging ?

    kernel::kprintln!("Kernel has been initialized");

    mm.kernel_map(
        0x0900_0000,
        0x0450_0000,
        kernel::mm::Permissions::READ | kernel::mm::Permissions::WRITE,
    );
    let mut uart = Pl011::new(0x0450_0000);
    uart.write("Uart remaped");

    loop {}
}
