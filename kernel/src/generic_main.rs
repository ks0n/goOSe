use super::device_tree::DeviceTree;
use super::drivers::qemuexit::QemuExit;
use super::drivers::Driver;
use super::globals;

use crate::mm;
use crate::HAL;

use crate::tests::{self, TestResult};

use log::info;

use hal_core::mm::{PageMap, Permissions, VAddr};

pub fn generic_main<const LAUNCH_TESTS: bool>(dt: DeviceTree, hacky_devices: &[&dyn Driver]) -> ! {
    info!("Entered generic_main");
    let qemu_exit = QemuExit::new();
    let qemu_exit_slice = [&qemu_exit as &dyn Driver];

    let devices = hacky_devices.iter().chain(&qemu_exit_slice);

    // Memory init
    globals::PHYSICAL_MEMORY_MANAGER
        .init_from_device_tree(&dt)
        .unwrap();
    mm::map_address_space(&dt, devices).expect("failed to map the addres space");

    // Driver stuff
    // let _drvmgr = DriverManager::with_devices(&dt).unwrap();

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "aarch64")] {
            // XXX: Ideally we'd read the device tree but we're not doing for now...
            log::trace!("mapping gic pages");
            let (gicd_base, gicc_base) = (0x800_0000, 0x801_0000);
            HAL.kpt().lock().identity_map_range(
                VAddr::new(gicd_base),
                0x0001_0000 / HAL.page_size(),
                Permissions::READ | Permissions::WRITE,
                &globals::PHYSICAL_MEMORY_MANAGER
            ).unwrap();
            HAL.kpt().lock().identity_map_range(
                VAddr::new(gicc_base),
                0x0001_0000 / HAL.page_size(),
                Permissions::READ | Permissions::WRITE,
                &globals::PHYSICAL_MEMORY_MANAGER
            ).unwrap();
        } else if #[cfg(target_arch = "riscv64")] {
            let base = 0xc000000;
            let max_offset = 0x3FFFFFC;
            HAL.kpt().lock().identity_map_range(
                VAddr::new(base),
                max_offset / HAL.page_size() + 1,
                Permissions::READ | Permissions::WRITE,
                &globals::PHYSICAL_MEMORY_MANAGER,
            ).unwrap();
        }
    }

    log::trace!("initializing irq chip");

    crate::HAL
        .init_irq_chip(&globals::PHYSICAL_MEMORY_MANAGER)
        .expect("initialization of irq chip failed");

    HAL.unmask_interrupts();

    if LAUNCH_TESTS {
        match tests::launch() {
            TestResult::Success => qemu_exit.exit_success(),
            TestResult::Failure => qemu_exit.exit_failure(),
        }
    } else {
        panic!("no scheduler to launch yet...");
    }
}
