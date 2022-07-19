use core::arch::asm;

use super::{Architecture, PerCoreContext};

pub mod interrupts;
pub mod registers;
pub mod sv39;

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

    fn get_core_local_storage() -> &'static mut PerCoreContext<'static> {
        let mut sscratch = 0;

        unsafe {
            asm!("csrww sscratch, {}", out(reg) sscratch);
            &mut *(sscratch as *mut PerCoreContext)
        }
    }

    fn set_core_local_storage(p: &mut PerCoreContext) {
        unsafe { asm!("csrrw {}, sscratch", in(reg) (p as *mut PerCoreContext) as u64) };
    }
}
