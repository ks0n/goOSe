use cfg_if::cfg_if;
use qemu_exit::QEMUExit;

use core::panic::PanicInfo;

use crate::{kprint, kprintln};

static UTEST_SUCESS: &str = "\x1b[32mok\x1b[0m";
static UTEST_FAILURE: &str = "\x1b[31mFAILED\x1b[0m";

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        kprint!("{} ... ", core::any::type_name::<T>());
        self();
        kprintln!("{}", UTEST_SUCESS);
    }
}

#[doc(hidden)]
pub fn runner(tests: &[&dyn Testable]) {
    kprintln!("\nRunning goOSe tests... Amount: {}\n", tests.len());

    for test in tests {
        test.run();
    }

    end_utests();
}

fn end_utests() {
    cfg_if! {
        if #[cfg(target_arch = "riscv64")] {
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
    #[test_case]
    fn assert_true() {
        assert!(true)
    }
}
