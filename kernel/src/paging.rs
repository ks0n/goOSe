use super::mm;
use core::num::TryFromIntError;

pub trait PagingImpl {
    fn new() -> Result<&'static mut Self, crate::Error>;

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
        pa: mm::PAddr,
        va: mm::VAddr,
        perms: mm::Permissions,
    ) -> Result<(), crate::Error>;

    fn add_invalid_entry(&mut self, vaddr: mm::VAddr) -> Result<(), crate::Error>;

    fn reload(&mut self);
    fn disable(&mut self);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel_tests::TestContext;

    struct PagingImplDummy {}
    impl PagingImpl for PagingImplDummy {
        fn new<'alloc>() -> &'alloc mut Self {
            unreachable!("We will never use this, we just need the compiler to be happy");
        }

        fn get_page_size() -> usize {
            4096
        }

        fn map(
            &mut self,
            _pa: mm::PAddr,
            _va: mm::VAddr,
            _perms: mm::Permissions,
        ) -> Result<(), Error> {
            unreachable!("We will never use this, we just need the compiler to be happy");
        }

        fn add_invalid_entry(
            &mut self,
            _mm: &mut mm::MemoryManager,
            _vaddr: mm::VAddr,
        ) -> Result<(), Error> {
            unreachable!("We will never use this, we just need the compiler to be happy");
        }

        fn reload(&mut self) {}
        fn disable(&mut self) {}
    }

    #[test_case]
    fn align_down(_ctx: &mut TestContext) {
        assert!(PagingImplDummy::align_down(0x1042) == 0x1000);
    }

    #[test_case]
    fn align_up(_ctx: &mut TestContext) {
        assert!(PagingImplDummy::align_up(0x1042) == 0x2000);
    }
}
