use super::Architecture;

pub struct Riscv64 {}

impl Riscv64 {
    pub fn new() -> Self {
        Self {}
    }
}

impl Architecture for Riscv64 {
    #[naked]
    #[no_mangle]
    unsafe extern "C" fn _start() -> ! {
        asm!("la sp, STACK_START", "call k_main", options(noreturn));
    }
}
