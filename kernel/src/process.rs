use crate::arch::Architecture;
use crate::executable::elf::Elf;
use crate::mm;

const STACK_PAGES: usize = 1;

pub struct Process<'a> {
    elf: Elf<'a>,
    pagetable: mm::UserPageTable,
    stack_base: usize,
}

impl<'a> Process<'a> {
    pub fn from_elf(
        elf_bytes: &'a [u8],
        kernel_pagetable: &mut mm::KernelPageTable,
        pmm: &mut mm::PhysicalMemoryManager,
    ) -> Self {
        let mut user_pagetable = kernel_pagetable.fork_user_page_table(pmm).unwrap(); // TODO: No
                                                                                      // unwrap
                                                                                      //
        let stack_pages = pmm.alloc_pages(STACK_PAGES).unwrap();
        let stack_base = user_pagetable.get_uppermost_address();
        let stack_base_page = user_pagetable.align_down(stack_base);
        user_pagetable
            .map(
                kernel_pagetable,
                pmm,
                stack_pages.into(),
                stack_base_page,
                mm::Permissions::READ | mm::Permissions::WRITE | mm::Permissions::USER,
            )
            .unwrap();

        let elf = Elf::from_bytes(elf_bytes);
        elf.load(kernel_pagetable, &mut user_pagetable, pmm);

        Self {
            elf,
            pagetable: user_pagetable,
            stack_base,
        }
    }

    pub fn execute(&mut self, arch: &mut crate::ArchImpl) {
        let entry = self.elf.get_entry_point();
        self.pagetable.reload();

        arch.jump_to_userland(entry, self.stack_base);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arch::riscv64::interrupts::TrapReturnValues; // FIXME: Oh noooo, arch specific
    use crate::arch::ArchitectureInterrupts;
    use crate::kernel_tests::*;
    use core::arch::asm;

    #[test_case]
    fn process_load_and_execute_userland(ctx: &mut TestContext) {
        ctx.reset();

        static mut EXIT_CODE: usize = 0;

        extern "C" fn exit_trap_handler(_cause: u64) -> TrapReturnValues {
            let val: usize;
            unsafe {
                asm!("mv {}, a7", out(reg) val);
                EXIT_CODE = val;
            }

            TrapReturnValues {
                need_pc_increment: 0,
                abort_to_kernel: 1,
            }
        }

        ctx.arch_interrupts.init_interrupts();
        ctx.arch_interrupts
            .set_higher_trap_handler(exit_trap_handler);

        let elf_bytes = core::include_bytes!("../fixtures/small");
        let mut process = Process::from_elf(elf_bytes, &mut ctx.page_table, &mut ctx.pmm);

        process.execute(&mut ctx.arch);

        // Restore kernel pagetable
        ctx.page_table.reload();

        assert!(unsafe { EXIT_CODE } == 42);
    }
}
