use log::trace;

use super::{AddressRange, Error};

#[derive(Debug, Clone, Copy)]
pub struct VAddr {
    pub val: usize,
}

impl VAddr {
    pub fn new(val: usize) -> Self {
        Self { val }
    }
}

impl core::convert::From<usize> for VAddr {
    fn from(val: usize) -> Self {
        Self { val }
    }
}

impl core::convert::From<VAddr> for usize {
    fn from(va: VAddr) -> Self {
        va.val
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PAddr {
    pub val: usize,
}

impl PAddr {
    pub fn new(val: usize) -> Self {
        Self { val }
    }
    pub fn ptr_cast<T>(self) -> *mut T {
        self.val as *mut T
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy)]
    pub struct Permissions: u8 {
        const READ    = 0b00000001;
        const WRITE   = 0b00000010;
        const EXECUTE = 0b00000100;
        const USER    = 0b00001000;
    }
}

pub type PageAllocFn = fn(usize) -> PAddr;

#[derive(Debug)]
pub enum AllocatorError {
    NotEnoughMemoryForMetadata,
    OutOfMemory,
}

pub trait PageAlloc: Sync {
    fn alloc(&self, page_count: usize) -> Result<usize, AllocatorError>;
    fn dealloc(&self, base: usize, page_count: usize) -> Result<(), AllocatorError>;
    fn used_pages<F: FnMut(usize)>(&self, f: F);
}

pub struct NullPageAllocator;

impl PageAlloc for NullPageAllocator {
    fn alloc(&self, _page_count: usize) -> Result<usize, AllocatorError> {
        panic!("the null page allocator mustn't allocate");
    }

    fn dealloc(&self, _base: usize, _page_count: usize) -> Result<(), AllocatorError> {
        panic!("the null page allocator cannot deallocate");
    }

    fn used_pages<F: FnMut(usize)>(&self, _f: F) {
        panic!("obviously the null allocator has no pages that are in use");
    }
}

pub trait PageEntry {
    fn set_invalid(&mut self);
}

pub trait PageMap {
    const PAGE_SIZE: usize;
    type Entry: PageEntry;

    fn new(allocator: &impl PageAlloc) -> Result<&'static mut Self, Error>;

    fn map(
        &mut self,
        va: VAddr,
        pa: PAddr,
        perms: Permissions,
        allocator: &impl PageAlloc,
    ) -> Result<&mut Self::Entry, Error>;

    fn add_invalid_entry(&mut self, va: VAddr, allocator: &impl PageAlloc) -> Result<(), Error> {
        self.map(
            va,
            PAddr::new(0x0A0A_0A0A_0A0A_0A0A),
            Permissions::READ,
            allocator,
        )?
        .set_invalid();

        Ok(())
    }

    fn identity_map(
        &mut self,
        addr: VAddr,
        perms: Permissions,
        allocator: &impl PageAlloc,
    ) -> Result<(), Error> {
        self.map(addr, PAddr::new(addr.val), perms, allocator)
            .map(|_| ())
    }

    fn identity_map_range(
        &mut self,
        addr: VAddr,
        page_count: usize,
        perms: Permissions,
        allocator: &impl PageAlloc,
    ) -> Result<(), Error> {
        let start = addr.val;
        for i in 0..page_count {
            self.identity_map(VAddr::new(start + i * Self::PAGE_SIZE), perms, allocator)?;
        }

        Ok(())
    }

    fn add_invalid_entries(
        &mut self,
        range: AddressRange,
        allocator: &impl PageAlloc,
    ) -> Result<(), Error> {
        for page in range.iter_pages(Self::PAGE_SIZE) {
            self.add_invalid_entry(VAddr::new(page), allocator)?;
        }

        Ok(())
    }

    fn identity_map_addressrange(
        &mut self,
        range: AddressRange,
        perms: Permissions,
        allocator: &impl PageAlloc,
    ) -> Result<(), Error> {
        for page in range.iter_pages(Self::PAGE_SIZE) {
            self.identity_map(VAddr::new(page), perms, allocator)?;
        }

        Ok(())
    }
}

pub fn align_up(val: usize, page_sz: usize) -> usize {
    ((val + page_sz - 1) / page_sz) * page_sz
}

pub fn align_down(addr: usize, page_sz: usize) -> usize {
    // TODO: can this be more optimized ?
    // XXX: uh isn't this math wrong ?
    align_up(addr, page_sz) + page_sz
}

pub fn prefill_pagetable<P: PageMap + 'static>(
    r: impl Iterator<Item = AddressRange>,
    rw: impl Iterator<Item = AddressRange>,
    rwx: impl Iterator<Item = AddressRange>,
    pre_allocated: impl Iterator<Item = AddressRange>,
    allocator: &impl PageAlloc,
) -> Result<&'static mut P, Error> {
    trace!("hal_core::mm::prefill_pagetable");
    let pt: &'static mut P = P::new(allocator)?;
    let page_size = P::PAGE_SIZE;

    for range in pre_allocated {
        pt.add_invalid_entries(range, allocator)?;
    }

    for range in r {
        trace!("mapping as RO: {:X?}", range);
        pt.identity_map_addressrange(range, Permissions::READ, allocator)?;
    }

    for range in rw {
        trace!("mapping as RW: {:X?}", range);
        pt.identity_map_addressrange(range, Permissions::READ | Permissions::WRITE, allocator)?;
    }

    for range in rwx {
        trace!("mapping as RWX: {:X?}", range);
        pt.identity_map_addressrange(
            range,
            Permissions::READ | Permissions::WRITE | Permissions::EXECUTE,
            allocator,
        )?
    }

    Ok(pt)
}
