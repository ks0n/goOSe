use super::device_tree::DeviceTree;
use super::drivers::qemuexit::QemuExit;
use super::drivers::Driver;
use super::globals;

use crate::hal;
use crate::mm::{alloc_pages_for_hal, map_address_space};

use crate::tests::{self, TestResult};

use log::info;

pub fn generic_main<const LAUNCH_TESTS: bool>(dt: DeviceTree, hacky_devices: &[&dyn Driver]) -> ! {
    info!("Entered generic_main");
    let qemu_exit = QemuExit::new();
    let qemu_exit_slice = [&qemu_exit as &dyn Driver];

    let devices = hacky_devices.into_iter().chain(&qemu_exit_slice);

    // Memory init
    globals::PHYSICAL_MEMORY_MANAGER
        .lock(|pmm| pmm.init_from_device_tree(&dt))
        .unwrap();
    map_address_space(&dt, devices).expect("failed to map the addres space");

    // Driver stuff
    // let _drvmgr = DriverManager::with_devices(&dt).unwrap();

    hal::irq::init_irq_chip((), alloc_pages_for_hal).expect("initialization of irq chip failed");

    hal::cpu::unmask_interrupts();

    if LAUNCH_TESTS {
        match tests::launch() {
            TestResult::Success => qemu_exit.exit_success(),
            TestResult::Failure => qemu_exit.exit_failure(),
        }
    } else {
        panic!("no scheduler to launch yet...");
    }
}
