use core::arch::asm;

use super::Architecture;

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

    fn jump_to_userland(&mut self, addr: usize, stack: usize) {
        unsafe {
            riscv::register::sstatus::set_spp(riscv::register::sstatus::SPP::User);
            riscv::register::sepc::write(addr);

            asm!(
                "
                // Put return address on the stack
                addi sp, sp, -8
                la t0, 1f
                sd t0, 0x8(sp)

                // Store kernel stack address in sscratch
                csrrw x0, sscratch, sp

                // Put userland stack address in sp
                mv sp, {}

                // Jump to to userland!
                sret

                1:
                // When returning here sp will hold the same value as before 'mv sp, ...'
                addi sp, sp, 8
                ", in(reg) stack
            );
        }
    }
}
