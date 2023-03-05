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
use super::mm::{self, Permissions};
use super::paging::PagingImpl as _;

use alloc::sync::Arc;

// fn do_irq_chip() -> Arc<dyn IrqChip + Sync + Send> {
//     globals::KERNEL_PAGETABLE
//         .lock(|pagetable| {
//             pagetable.map(
//                 0x0800_0000.into(),
//                 0x0800_0000.into(),
//                 Permissions::READ | Permissions::WRITE,
//             ).unwrap();
//             pagetable.map(
//                 0x0801_0000.into(),
//                 0x0801_0000.into(),
//                 Permissions::READ | Permissions::WRITE,
//             ).unwrap();
//         });
//     Arc::new(GicV2::new(0x800_0000, 0x801_0000))
// }

pub fn generic_main<Arch: Architecture>(dt: DeviceTree, hacky_devices: &[&dyn Driver]) {
    // Memory init
    globals::PHYSICAL_MEMORY_MANAGER
        .lock(|pmm| pmm.init_from_device_tree(&dt, 4096))
        .unwrap();
    mm::map_address_space(&dt, hacky_devices);

    // Driver stuff
    let _drvmgr = DriverManager::with_devices(&dt).unwrap();

    globals::IRQ_CHIP
        .get()
        .unwrap()
        .enable(Interrupt::PhysicalTimer);

    // Shit-tier testing
    test_timer_interrupt::<Arch>();
    #[cfg(target = "aarch64")]
    test_pagetable_remap();

    crate::kprintln!("after last test");
    loop {}

    QemuExit::new().exit_success();
}

fn test_timer_interrupt<Arch: Architecture>() {
    if true {
        // IRQ
        Arch::unmask_interrupts();
        Arch::set_timer(50_000);
    } else {
        // // Synchronous exception
        // unsafe {
        //     asm!("svc 42");
        // }
    }
}

fn test_pagetable_remap() {
    crate::kprintln!("Testing the remapping capabilities of our pagetable...");
    globals::KERNEL_PAGETABLE
        .lock(|pagetable| {
            pagetable.map(
                0x0900_0000.into(),
                0x0450_0000.into(),
                Permissions::READ | Permissions::WRITE,
            )
        })
        .unwrap();
    let uart = Pl011::new(0x0450_0000);
    uart.write("Uart remaped, if you see this, it works !!!\n");
}
