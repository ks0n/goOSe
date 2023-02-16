use super::device_tree::DeviceTree;
use super::drivers::qemuexit::QemuExit;
use super::globals;
use super::drivers::{
    Driver,
    Console,
    pl011::Pl011,
    gicv2::GicV2
};
use super::irq::{IrqChip, Interrupt};
use super::paging::PagingImpl as _;
use super::mm::{self, Permissions};
use super::arch::Architecture;
use super::driver_manager::DriverManager;

use alloc::sync::Arc;

fn do_irq_chip() -> Arc<dyn IrqChip + Sync + Send> {
    globals::KERNEL_PAGETABLE
        .lock(|pagetable| {
            pagetable.map(
                0x0800_0000.into(),
                0x0800_0000.into(),
                Permissions::READ | Permissions::WRITE,
            ).unwrap();
            pagetable.map(
                0x0801_0000.into(),
                0x0801_0000.into(),
                Permissions::READ | Permissions::WRITE,
            ).unwrap();
        });
    Arc::new(GicV2::new(0x800_0000, 0x801_0000))
}

pub fn generic_main<Arch: Architecture>(dt: DeviceTree, hacky_devices: &[&dyn Driver]) {
    // Memory init
    globals::PHYSICAL_MEMORY_MANAGER
        .lock(|pmm| pmm.init_from_device_tree(&dt, 4096))
        .unwrap();
    mm::map_address_space(&dt, hacky_devices);

    // Driver stuff
    let _drvmgr = DriverManager::with_devices(&dt).unwrap();

    // Should be done by the driver manager, but no time for now.
    let irq_chip = do_irq_chip();
    irq_chip.enable(Interrupt::PhysicalTimer);
    globals::IRQ_CHIP.set(irq_chip);


    // Shit-tier testing
    test_timer_interrupt::<Arch>();
    test_pagetable_remap();

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
