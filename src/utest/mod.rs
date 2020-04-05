use crate::println;
use crate::print;

use crate::asm_wrappers;

static UTEST_SUCESS: &str = "\x1b[32mOK\x1b[0m";
static QEMU_EXIT_PORT: u16 = 0xf4;

/* We need a custom exit code in order to not interfere with QEMU
 * This will cause a 253 exit code on success */
static QEMU_SUCCESS_CODE: u8 = 0xfe;

fn end_utests() {
    asm_wrappers::outb(QEMU_EXIT_PORT, QEMU_SUCCESS_CODE);
}

#[cfg(test)]
pub fn runner(tests: &[&dyn Fn()]) {
    println!("Running goOSe tests... Amount: {}\n", tests.len());

    for test in tests {
        test();
    }

    end_utests();
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(0, 1);
    println!("[{}]", UTEST_SUCESS);
}
