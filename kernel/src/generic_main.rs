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
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::hal;
use crate::mm::{alloc_pages_for_hal, map_address_space};
use hal_core::mm::{PageMap, Permissions};

use crate::executable::elf::Elf;
use align_data::include_aligned;
use align_data::Align4K;

use log::{info, trace};

pub fn generic_main<const LAUNCH_TESTS: bool>(dt: DeviceTree, hacky_devices: &[&dyn Driver]) -> ! {
    info!("Entered generic_main");
    let qemu_exit = QemuExit::new();
    let qemu_exit_slice = [&qemu_exit as &dyn Driver];

    let devices = hacky_devices.into_iter().chain(&qemu_exit_slice);

    // Memory init
    globals::PHYSICAL_MEMORY_MANAGER
        .lock(|pmm| pmm.init_from_device_tree(&dt, 4096))
        .unwrap();
    map_address_space(&dt, devices).expect("failed to map the addres space");

    // Driver stuff
    // let _drvmgr = DriverManager::with_devices(&dt).unwrap();

    hal::irq::init_irq_chip((), alloc_pages_for_hal).expect("initialization of irq chip failed");

    hal::cpu::unmask_interrupts();

    if LAUNCH_TESTS {
        info!("Launching tests...");
        // Shit-tier testing
        test_timer_interrupt();
        #[cfg(target_arch = "aarch64")]
        test_pagetable_remap();
        test_elf_loader_basic();

        info!("TESTS FINISHED SUCCESSFULY ✅");

        qemu_exit.exit_success();
    } else {
        panic!("no scheduler to launch yet...");
    }
}

fn test_timer_interrupt() {
    if true {
        // IRQ
        static CNT: AtomicUsize = AtomicUsize::new(0);
        const NUM_INTERRUPTS: usize = 3;

        info!(
            "Testing timer interrupts, waiting for {} interrupts",
            NUM_INTERRUPTS
        );

        hal::cpu::clear_physical_timer();

        hal::irq::set_timer_handler(|| {
            trace!(".");

            if CNT.fetch_add(1, Ordering::Relaxed) < NUM_INTERRUPTS {
                hal::irq::set_timer(50_000)
                    .expect("failed to set timer in the timer handler of the test");
            }
        });

        hal::irq::set_timer(50_000).expect("failed to set timer for test");

        while CNT.load(Ordering::Relaxed) < NUM_INTERRUPTS {}

        // TODO: restore the timer handler
        hal::cpu::clear_physical_timer();
        info!("test_timer_interrupts ✅");
    } else {
        // // Synchronous exception
        // unsafe {
        //     asm!("svc 42");
        // }
    }
}

fn test_pagetable_remap() {
    info!("Testing the remapping capabilities of our pagetable...");
    hal::mm::current()
        .map(
            hal_core::mm::VAddr::new(0x0450_0000),
            hal_core::mm::PAddr::new(0x0900_0000),
            Permissions::READ | Permissions::WRITE,
            alloc_pages_for_hal,
        )
        .unwrap();
    let uart = Pl011::new(0x0450_0000);
    uart.write("Uart remaped, if you see this, it works !!!\n");
    info!("test_pagetable_remap ✅");
}

fn test_elf_loader_basic() {
    static TEST_BIN: &[u8] = include_aligned!(Align4K, env!("CARGO_BIN_FILE_TESTS"));

    let test_bin = Elf::from_bytes(TEST_BIN);
    info!("[OK] Elf from_bytes {}", env!("CARGO_BIN_FILE_TESTS"));
    test_bin.load().unwrap();
    info!("[OK] Elf loaded");
    let entry_point: extern "C" fn() -> u8 =
        unsafe { core::mem::transmute(test_bin.get_entry_point()) };
    info!("[OK] Elf loaded, entry point is {:?}", entry_point);
    entry_point();
    info!("[OK] Returned for Elf");
}
