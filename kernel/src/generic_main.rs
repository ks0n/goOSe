use super::device_tree::DeviceTree;
use super::driver_manager::DriverManager;
use super::drivers::qemuexit::QemuExit;
use super::drivers::Driver;
use super::globals;

use crate::mm;
use crate::HAL;

use crate::tests::{self, TestResult};

use log::info;

pub fn generic_main<const LAUNCH_TESTS: bool>(dt: DeviceTree, hacky_devices: &[&dyn Driver]) -> ! {
    info!("Entered generic_main");
    let qemu_exit = QemuExit::new();
    let qemu_exit_slice = [&qemu_exit as &dyn Driver];

    let devices = hacky_devices.iter().chain(&qemu_exit_slice);

    // Memory init
    globals::PHYSICAL_MEMORY_MANAGER
        .init_from_device_tree(&dt)
        .unwrap();
    // Below this comment, using alloc is possible.
    mm::map_address_space(&dt, devices).expect("failed to map the addres space");

    DriverManager::do_console(&dt).expect("couldn't initialize a console from the device tree");
    DriverManager::map_irq_chip(&dt).expect("failed to map irq chip from the device tree");

    log::trace!("initializing irq chip");
    HAL.init_irq_chip(&globals::PHYSICAL_MEMORY_MANAGER)
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
