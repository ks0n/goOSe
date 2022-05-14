#[cfg(target_arch = "arm")]
mod arm32;
#[cfg(target_arch = "riscv64")]
mod riscv64;

use cfg_if::cfg_if;

use crate::mm;

#[cfg(target_arch = "riscv64")]
pub type ArchImpl = riscv64::Riscv64;
#[cfg(target_arch = "riscv64")]
pub type ArchInterruptsImpl = riscv64::interrupts::Interrupts;
#[cfg(target_arch = "riscv64")]
pub type MemoryImpl = riscv64::sv39::PageTable;
#[cfg(target_arch = "riscv64")]
pub type InterruptsImpl = riscv64::interrupts::Interrupts;

pub fn new_arch(info: usize) -> impl Architecture {
    cfg_if! {
        if #[cfg(target_arch = "riscv64")] {
            riscv64::Riscv64::new(info)
        } else if #[cfg(target_arch = "arm")] {
            arm32::Arm32::new()
        } else {
            core::compile_error!("Architecture not supported! Did you run `gen_cargo.sh`?");
        }
    }
}

pub trait Architecture {
    unsafe extern "C" fn _start() -> !;

    fn new(info: usize) -> Self;

    fn for_all_memory_regions<F: FnMut(&mut dyn Iterator<Item = (usize, usize)>)>(&self, f: F);
    fn for_all_reserved_memory_regions<F: FnMut(&mut dyn Iterator<Item = (usize, usize)>)>(
        &self,
        f: F,
    );
}

pub trait ArchitectureMemory {
    fn new<'alloc>(allocator: &mut impl mm::PageAllocator) -> &'alloc mut Self;

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
        allocator: &mut impl mm::PageAllocator,
        to: usize,
        from: usize,
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
        fn new<'alloc>(_allocator: &mut impl mm::PageAllocator) -> &'alloc mut Self {
            // We will never use this, we just need the compiler to be happy
            unsafe { (0 as *mut Self).as_mut().unwrap() }
        }

        fn get_page_size() -> usize {
            4096
        }

        fn map(
            &mut self,
            _allocator: &mut impl mm::PageAllocator,
            _to: usize,
            _from: usize,
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
