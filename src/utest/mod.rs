#![test_runner(crate::utest_runner)]

#[cfg(test)]
fn utest_runner(tests: &[&dyn Fn()]) {
    for test in tests {
        test();
    }
}
