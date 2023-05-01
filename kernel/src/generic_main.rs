use super::arch::Architecture;
use super::device_tree::DeviceTree;
use super::driver_manager::DriverManager;
use super::drivers::qemuexit::QemuExit;
use super::drivers::{
    pl011::Pl011,
    // gicv2::GicV2
    Console,
    Driver,
};
use super::globals;
use super::irq::{Interrupt, IrqChip};
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::hal;
use crate::hal::mm;
use crate::mm::{alloc_pages_for_hal, map_address_space};
use hal_core::mm::{PageMap, Permissions};


pub fn generic_main(dt: DeviceTree, hacky_devices: &[&dyn Driver]) {
    // Memory init
    globals::PHYSICAL_MEMORY_MANAGER
        .lock(|pmm| pmm.init_from_device_tree(&dt, 4096))
        .unwrap();
    map_address_space(&dt, hacky_devices);

    // Driver stuff
    let _drvmgr = DriverManager::with_devices(&dt).unwrap();

    hal::irq::init_irq_chip((), alloc_pages_for_hal);

    // Shit-tier testing
    test_timer_interrupt();
    // #[cfg(target = "aarch64")]
    test_pagetable_remap();

    crate::kprintln!("TESTS FINISHED SUCCESSFULY ✅");

    QemuExit::new().exit_success();
}

fn test_timer_interrupt() {
    if true {
        // IRQ
        static cnt: AtomicUsize = AtomicUsize::new(0);
        const NUM_INTERRUPTS: usize = 3;

        crate::kprintln!("Testing timer interrupts, waiting for {} interrupts", NUM_INTERRUPTS);

        hal::irq::set_timer(50_000);

        hal::irq::set_timer_handler(|| {
            crate::kprintln!(".");

            if cnt.fetch_add(1, Ordering::Relaxed) < NUM_INTERRUPTS {
                hal::irq::set_timer(50_000);
            }
        });

        while cnt.load(Ordering::Relaxed) < NUM_INTERRUPTS {
        }

        // TODO: restore the timer handler
        hal::cpu::clear_physical_timer();
        crate::kprintln!("test_timer_interrupts ✅");
    } else {
        // // Synchronous exception
        // unsafe {
        //     asm!("svc 42");
        // }
    }
}

fn test_pagetable_remap() {
    crate::kprintln!("Testing the remapping capabilities of our pagetable...");
    hal::mm::current().map(
                hal_core::mm::VAddr::new(0x0450_0000),
                hal_core::mm::PAddr::new(0x0900_0000),
                Permissions::READ | Permissions::WRITE,
                alloc_pages_for_hal
    ).unwrap();
    let uart = Pl011::new(0x0450_0000);
    uart.write("Uart remaped, if you see this, it works !!!\n");
    crate::kprintln!("test_pagetable_remap ✅");
}
