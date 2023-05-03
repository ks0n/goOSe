use cfg_if::cfg_if;

use core::panic::PanicInfo;

use crate::arch;
use crate::arch::Architecture;
use crate::arch::ArchitectureInterrupts;
use crate::mm;
use crate::paging::PagingImpl;
use crate::{kprint, kprintln};

use drivers::qemuexit::QemuExit;

static UTEST_SUCESS: &str = "\x1b[32mok\x1b[0m";
static UTEST_FAILURE: &str = "\x1b[31mFAILED\x1b[0m";

static mut TEST_CONTEXT: Option<TestContext> = None;

pub struct TestContext {
    device_tree_address: usize,
    pub arch: crate::ArchImpl,
    pub arch_interrupts: crate::InterruptsImpl,
    pub mm: mm::MemoryManager,
}

impl TestContext {
    pub fn new(device_tree_address: usize) -> Self {
        let (arch, mm) = TestContext::build_context_data(device_tree_address);

        TestContext {
            device_tree_address,
            arch,
            arch_interrupts: crate::InterruptsImpl {},
            mm,
        }
    }

    pub fn reset(&mut self) {
        // We will recreate a global allocator from scratch. Currently loaded page table is
        // allocated via the global allocator. Let's disable pagination to avoiding access fault
        self.mm.get_kernel_pagetable().unwrap().disable();

        let (arch, mm) = TestContext::build_context_data(self.device_tree_address);

        self.arch = arch;
        self.mm = mm;
    }

    fn build_context_data(device_tree_address: usize) -> (crate::ArchImpl, mm::MemoryManager) {
        let arch = crate::ArchImpl::new();
        let device_tree = crate::device_tree::DeviceTree::new(device_tree_address);
        let pmm = mm::PhysicalMemoryManager::from_device_tree(
            &device_tree,
            crate::PagingImpl::get_page_size(),
        );

        let mut mm = mm::MemoryManager::new(pmm);

        let pagetable = mm::map_address_space(
            &device_tree,
            &mut mm,
            &[crate::kernel_console::get_console(), &QemuExit::new()],
        );

        mm.set_kernel_pagetable(pagetable);

        (arch, mm)
    }
}

pub trait Testable {
    fn run(&self, ctx: &mut TestContext) -> ();
}

impl<T> Testable for T
where
    T: Fn(&mut TestContext),
{
    fn run(&self, ctx: &mut TestContext) {
        kprint!("{} ... ", core::any::type_name::<T>());
        self(ctx);
        kprintln!("{}", UTEST_SUCESS);
    }
}

pub fn init(device_tree_address: usize) {
    let ctx = TestContext::new(device_tree_address);

    unsafe {
        TEST_CONTEXT = Some(ctx);
    }

    kprintln!("[OK] Test context initialization");
}

#[doc(hidden)]
pub fn runner(tests: &[&dyn Testable]) {
    kprintln!("\nRunning Gagelen tests... Amount: {}\n", tests.len());

    let ctx = unsafe { TEST_CONTEXT.as_mut().unwrap() };

    for test in tests {
        test.run(ctx);
    }

    end_utests();
}

fn end_utests() {
    let ctx = unsafe { (&mut TEST_CONTEXT).as_mut().unwrap() };

    QemuExit::new().exit_success();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprintln!("[{}]", UTEST_FAILURE);
    kprintln!("{}", info);

    end_utests();

    QemuExit::new().exit_failure();

    loop {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn assert_true(_ctx: &mut TestContext) {
        assert!(true)
    }
}
