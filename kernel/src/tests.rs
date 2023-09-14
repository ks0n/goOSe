use log::{debug, info, trace};

use core::slice;
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::executable::elf::Elf;
use crate::globals;
use crate::hal::{self, mm::PAGE_SIZE};
use hal_core::mm::{PageAlloc, PageMap, Permissions};

use align_data::include_aligned;
use align_data::Align4K;

pub enum TestResult {
    Success,
    Failure,
}

struct Test {
    name: &'static str,
    test: fn() -> TestResult,
}

const TESTS: &[Test] = &[
    Test {
        name: "timer interrupts",
        test: test_timer_interrupt,
    },
    Test {
        name: "pagetable does remap",
        test: test_pagetable_remap,
    },
    Test {
        name: "basic elf loader",
        test: test_elf_loader_basic,
    },
];

pub fn launch() -> TestResult {
    let mut res = TestResult::Success;

    info!("Launching tests...");
    for (i, test) in TESTS.iter().enumerate() {
        info!("Test #{} \'{}\':", i, test.name);
        match (test.test)() {
            TestResult::Failure => {
                info!("#{} failed ❌", i);
                res = TestResult::Failure;
            }
            TestResult::Success => {
                info!("#{} passed ✅", i);
            }
        }
    }

    res
}

fn test_timer_interrupt() -> TestResult {
    if true {
        // IRQ
        static CNT: AtomicUsize = AtomicUsize::new(0);
        const NUM_INTERRUPTS: usize = 3;

        debug!(
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
        TestResult::Success
    } else {
        // // Synchronous exception
        // unsafe {
        //     asm!("svc 42");
        // }
        TestResult::Failure
    }
}

fn test_pagetable_remap() -> TestResult {
    info!("Testing the remapping capabilities of our pagetable...");

    let page_src = globals::PHYSICAL_MEMORY_MANAGER.alloc(1).unwrap();
    let page_src = unsafe { slice::from_raw_parts_mut(page_src as *mut u8, PAGE_SIZE) };
    let dst_addr = 0x0450_0000;
    let page_dst = unsafe { slice::from_raw_parts(dst_addr as *const u8, hal::mm::PAGE_SIZE) };
    let deadbeef = [0xDE, 0xAD, 0xBE, 0xEF];

    // Put data in source page

    page_src[0..deadbeef.len()].copy_from_slice(&deadbeef);

    // Remap source page to destination page
    hal::mm::current()
        .map(
            hal_core::mm::VAddr::new(dst_addr),
            hal_core::mm::PAddr::new(page_src.as_ptr() as usize),
            Permissions::READ | Permissions::WRITE,
            &globals::PHYSICAL_MEMORY_MANAGER,
        )
        .unwrap();

    // Readback from destination page
    for i in 0..deadbeef.len() {
        if page_dst[i] != deadbeef[i] {
            return TestResult::Failure;
        }
    }

    info!("Remapping works");

    TestResult::Success
}

fn test_elf_loader_basic() -> TestResult {
    static TEST_BIN: &[u8] = include_aligned!(Align4K, env!("CARGO_BIN_FILE_TESTS"));

    let test_bin = Elf::from_bytes(TEST_BIN);
    debug!("[OK] Elf from_bytes {}", env!("CARGO_BIN_FILE_TESTS"));
    test_bin.load().unwrap();
    debug!("[OK] Elf loaded");
    let entry_point: extern "C" fn() -> u8 =
        unsafe { core::mem::transmute(test_bin.get_entry_point()) };
    debug!("[OK] Elf loaded, entry point is {:?}", entry_point);
    entry_point();
    debug!("[OK] Returned for Elf");

    TestResult::Success
}
