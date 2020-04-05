use core::panic::PanicInfo;

use crate::println;
use crate::print;
use crate::asm_wrappers;

static UTEST_SUCESS: &str = "\x1b[32mOK\x1b[0m";
static UTEST_FAILURE: &str = "\x1b[31mKO\x1b[0m";
static QEMU_EXIT_PORT: u16 = 0xf4;

/* We need a custom exit code in order to not interfere with QEMU
 * This will cause a 253 exit code on success */
static QEMU_SUCCESS_CODE: u8 = 0xfe;
static QEMU_FAILURE_CODE: u8 = 0xbe;

/// Assert the equality of two elements
#[macro_export]
macro_rules! kassert_eq {
    ($l_exp: expr, $r_exp: expr) => ({
        // FIXME: Show function name
        uassert_eq($l_exp, $r_exp, "anonymous test")
    });
    ($l_exp: expr, $r_exp: expr, $name: tt) => ({
        uassert_eq($l_exp, $r_exp, $name)
    });
}

#[cfg(test)]
fn uassert_eq<T: Eq + core::fmt::Debug>(lhs: T, rhs: T, test_name: &str) {
    print!("{}... ", test_name);
    assert_eq!(lhs, rhs);
    println!("[{}]", UTEST_SUCESS);
}

#[cfg(test)]
pub fn runner(tests: &[&dyn Fn()]) {
    println!("Running goOSe tests... Amount: {}\n", tests.len());

    for test in tests {
        test();
    }

    end_utests();
}

#[cfg(test)]
fn end_utests() {
    asm_wrappers::outb(QEMU_EXIT_PORT, QEMU_SUCCESS_CODE);
}

#[cfg(test)]
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
    kassert_eq!(true, true, "utest framework initialization complete");
}
