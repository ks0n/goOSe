//! The `Unit Test` module provides two macros: `kassert` and `kassert_eq`, which can
//! be used like `assert` and `assert_eq` in classic Rust code.

use crate::arch;
use crate::print;
use crate::println;
use core::panic::PanicInfo;
use qemu_exit::QEMUExit;

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
        print!("{} ... ", core::any::type_name::<T>());
        self();
        println!("{}", UTEST_SUCESS);
    }
}

#[doc(hidden)]
pub fn runner(tests: &[&dyn Testable]) {
    println!("\nRunning goOSe tests... Amount: {}\n", tests.len());

    for test in tests {
        test.run();
    }

    end_utests();
}

fn end_utests() {
    arch::QEMU_EXIT.exit_success();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[{}]", UTEST_FAILURE);
    println!("{}", info);

    end_utests();

    loop {}
}

#[test_case]
fn test_slef_check() {
    assert!(true);
}
