#[cfg(target_arch = "arm")]
pub mod arm32;
#[cfg(target_arch = "riscv64")]
pub mod riscv64;
#[cfg(target_arch = "aarch64")]
pub mod aarch64;

pub trait Architecture {
    unsafe extern "C" fn _start() -> !;

    fn new(info: usize) -> Self;

    fn for_all_memory_regions<F: FnMut(&mut dyn Iterator<Item = (usize, usize)>)>(&self, f: F);
    fn for_all_reserved_memory_regions<F: FnMut(&mut dyn Iterator<Item = (usize, usize)>)>(
        &self,
        f: F,
    );
}

#[cfg(target = "riscv64")]
use crate::mm;
#[cfg(target = "riscv64")]
pub trait ArchitectureMemory {
    fn new<'alloc>(allocator: &mut mm::PhysicalMemoryManager) -> &'alloc mut Self;

    fn get_page_size() -> usize;

    fn align_down(addr: usize) -> usize {
        let page_size = Self::get_page_size();
        let page_mask = !(page_size - 1);

        addr & page_mask
    }

    fn align_up(addr: usize) -> usize {
        let page_size = Self::get_page_size();
        ((addr + page_size - 1) / page_size) * page_size
    }

    fn map(
        &mut self,
        allocator: &mut mm::PhysicalMemoryManager,
        pa: mm::PAddr,
        va: mm::VAddr,
        perms: mm::Permissions,
    );

    fn reload(&mut self);
    fn disable(&mut self);
}

pub trait ArchitectureInterrupts {
    fn new() -> Self;
    fn init_interrupts(&mut self);
    fn set_timer(&mut self, delay: usize);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel_tests::TestContext;

    struct ArchitectureMemoryDummy {}
    impl ArchitectureMemory for ArchitectureMemoryDummy {
        fn new<'alloc>(_allocator: &mut mm::PhysicalMemoryManager) -> &'alloc mut Self {
            unreachable!("We will never use this, we just need the compiler to be happy");
        }

        fn get_page_size() -> usize {
            4096
        }

        fn map(
            &mut self,
            _allocator: &mut mm::PhysicalMemoryManager,
            _pa: mm::PAddr,
            _va: mm::VAddr,
            _perms: mm::Permissions,
        ) {
        }

        fn reload(&mut self) {}
        fn disable(&mut self) {}
    }

    #[test_case]
    fn align_down(_ctx: &mut TestContext) {
        assert!(ArchitectureMemoryDummy::align_down(0x1042) == 0x1000);
    }

    #[test_case]
    fn align_up(_ctx: &mut TestContext) {
        assert!(ArchitectureMemoryDummy::align_up(0x1042) == 0x2000);
    }
}
