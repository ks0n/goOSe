use cfg_if::cfg_if;
use qemu_exit::QEMUExit;

use core::panic::PanicInfo;

use crate::arch;
use crate::arch::Architecture;
use crate::arch::ArchitectureInterrupts;
use crate::arch_mem::ArchitectureMemory;
use crate::mm;
use crate::{kprint, kprintln};

static UTEST_SUCESS: &str = "\x1b[32mok\x1b[0m";
static UTEST_FAILURE: &str = "\x1b[31mFAILED\x1b[0m";

static mut TEST_CONTEXT: Option<TestContext> = None;

pub struct TestContext<'alloc> {
    device_tree_address: usize,
    pub arch: crate::ArchImpl,
    pub arch_interrupts: crate::InterruptsImpl,
    pub pmm: mm::PhysicalMemoryManager,
    pub page_table: &'alloc mut crate::MemoryImpl,
}

impl<'alloc> TestContext<'alloc> {
    pub fn new(device_tree_address: usize) -> Self {
        let (arch, pmm, page_table) = TestContext::build_context_data(device_tree_address);

        TestContext {
            device_tree_address,
            arch,
            arch_interrupts: crate::InterruptsImpl::new(),
            pmm,
            page_table,
        }
    }

    pub fn reset(&mut self) {
        // We will recreate a global allocator from scratch. Currently loaded page table is
        // allocated via the global allocator. Let's disable pagination to avoiding access fault
        self.page_table.disable();

        let (arch, pmm, page_table) = TestContext::build_context_data(self.device_tree_address);

        self.arch = arch;
        self.pmm = pmm;
        self.page_table = page_table;
    }

    fn build_context_data(
        device_tree_address: usize,
    ) -> (
        crate::ArchImpl,
        mm::PhysicalMemoryManager,
        &'alloc mut crate::MemoryImpl,
    ) {
        let arch = crate::ArchImpl::new(device_tree_address);
        let mut pmm =
            mm::PhysicalMemoryManager::from_arch_info(&arch, crate::MemoryImpl::get_page_size());

        let page_table = crate::MemoryImpl::new(&mut pmm);
        mm::map_address_space(&arch, page_table, &mut pmm);

        (arch, pmm, page_table)
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
    kprintln!("\nRunning goOSe tests... Amount: {}\n", tests.len());

    let ctx = unsafe { (&mut TEST_CONTEXT).as_mut().unwrap() };

    for test in tests {
        test.run(ctx);
    }

    end_utests();
}

fn end_utests() {
    let ctx = unsafe { (&mut TEST_CONTEXT).as_mut().unwrap() };

    cfg_if! {
        if #[cfg(target_arch = "riscv64")] {
            ctx.page_table.map(&mut ctx.pmm, mm::PAddr::from(0x100000), mm::VAddr::from(0x100000),
            mm::Permissions::READ | mm::Permissions::WRITE);
            qemu_exit::RISCV64::new(0x100000).exit_success()
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kprintln!("[{}]", UTEST_FAILURE);
    kprintln!("{}", info);

    end_utests();

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
