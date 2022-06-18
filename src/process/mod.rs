use crate::executable;
use crate::mm;

struct Process<'alloc> {
    memory: mm::MemoryManagement<'alloc>,
}

impl<'alloc> Process<'alloc> {
    fn new() -> Self {
        Self {
            memory: mm::MemoryManagement::new(),
        }
    }

    pub fn from_elf<'a>(data: &'a [u8]) -> Self {
        let elf = executable::elf::Elf::from_bytes(data);
        let mut process = Self::new();

        elf.map(&mut process.memory);

        process
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel_tests::*;

    static ELF_BYTES: &[u8] = core::include_bytes!("../../fixtures/small");

    #[test_case]
    fn from_elf(_ctx: &mut TestContext) {
        let _process = Process::from_elf(ELF_BYTES);
    }

    #[test_case]
    fn run(_ctx: &mut TestContext) {
        let process = Process::from_elf(ELF_BYTES);
    }
}
