//! The `Unit Test` module provides two macros: `kassert` and `kassert_eq`, which can
//! be used like `assert` and `assert_eq` in classic Rust code.

use crate::arch;
use crate::print;
use crate::println;
use core::panic::PanicInfo;
use qemu_exit::QEMUExit;

static UTEST_SUCESS: &str = "\x1b[32mOK\x1b[0m";
static UTEST_FAILURE: &str = "\x1b[31mKO\x1b[0m";

/// Assert the equality of two elements
macro_rules! kassert_eq {
    ($l_exp: expr, $r_exp: expr) => {{
        crate::utest::uassert_eq($l_exp, $r_exp, "anonymous test")
    }};
    ($l_exp: expr, $r_exp: expr, $name: tt) => {{
        crate::utest::uassert_eq($l_exp, $r_exp, $name)
    }};
}

/// Assert the validity of a statement
macro_rules! kassert {
    ($stmt: expr) => {{
        crate::utest::uassert_eq($stmt, true, "anonymous test")
    }};
    ($stmt: expr, $name: tt) => {{
        crate::utest::uassert_eq($stmt, true, $name)
    }};
}

#[doc(hidden)]
pub fn runner(tests: &[&dyn Fn()]) {
    println!("Running goOSe tests... Amount: {}\n", tests.len());

    for test in tests {
        test();
    }

    end_utests();
}

#[doc(hidden)]
pub fn uassert_eq<T: PartialEq + core::fmt::Debug>(lhs: T, rhs: T, test_name: &str) {
    print!("{}... ", test_name);
    assert_eq!(lhs, rhs);
    println!("[{}]", UTEST_SUCESS);
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
fn utests_test() {
    uassert_eq(true, true, "utest framework initialization");
    kassert_eq!(true, true);
    kassert!(true);
    kassert_eq!(true, true, "utest framework initialization complete");
}
