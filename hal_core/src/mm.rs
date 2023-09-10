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

pub trait PageEntry {
    fn set_invalid(&mut self);
}

pub trait PageMap {
    const PAGE_SIZE: usize;
    type Entry: PageEntry;

    fn new(alloc: PageAllocFn) -> Result<&'static mut Self, Error>;

    fn map(
        &mut self,
        va: VAddr,
        pa: PAddr,
        perms: Permissions,
        alloc: PageAllocFn,
    ) -> Result<&mut Self::Entry, Error>;

    fn add_invalid_entry(&mut self, va: VAddr, alloc: PageAllocFn) -> Result<(), Error> {
        self.map(
            va,
            PAddr::new(0x0A0A_0A0A_0A0A_0A0A),
            Permissions::READ,
            alloc,
        )?
        .set_invalid();

        Ok(())
    }

    fn identity_map(
        &mut self,
        addr: VAddr,
        perms: Permissions,
        alloc: PageAllocFn,
    ) -> Result<(), Error> {
        self.map(addr, PAddr::new(addr.val), perms, alloc)
            .map(|_| ())
    }

    fn identity_map_range(
        &mut self,
        addr: VAddr,
        page_count: usize,
        perms: Permissions,
        alloc: PageAllocFn,
    ) -> Result<(), Error> {
        let start = addr.val;
        for i in 0..page_count {
            self.identity_map(VAddr::new(start + i * Self::PAGE_SIZE), perms, alloc)?;
        }

        Ok(())
    }

    fn add_invalid_entries(
        &mut self,
        range: AddressRange,
        alloc: PageAllocFn,
    ) -> Result<(), Error> {
        for page in range.iter_pages(Self::PAGE_SIZE) {
            self.add_invalid_entry(VAddr::new(page), alloc)?;
        }

        Ok(())
    }

    fn identity_map_addressrange(
        &mut self,
        range: AddressRange,
        perms: Permissions,
        alloc: PageAllocFn,
    ) -> Result<(), Error> {
        for page in range.iter_pages(Self::PAGE_SIZE) {
            self.identity_map(VAddr::new(page), perms, alloc)?;
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

pub fn init_paging<P: PageMap + 'static>(
    r: impl Iterator<Item = AddressRange>,
    rw: impl Iterator<Item = AddressRange>,
    rwx: impl Iterator<Item = AddressRange>,
    pre_allocated: impl Iterator<Item = AddressRange>,
    alloc: PageAllocFn,
    store_pagetable: impl FnOnce(&'static mut P),
) -> Result<(), Error> {
    let pt: &'static mut P = P::new(alloc)?;
    let page_size = P::PAGE_SIZE;

    for range in pre_allocated {
        pt.add_invalid_entries(range, alloc)?;
    }

    for range in r {
        trace!("mapping as RO: {:X?}", range);
        pt.identity_map_addressrange(range, Permissions::READ, alloc)?;
    }

    for range in rw {
        trace!("mapping as RW: {:X?}", range);
        pt.identity_map_addressrange(range, Permissions::READ | Permissions::WRITE, alloc)?;
    }

    for range in rwx {
        trace!("mapping as RWX: {:X?}", range);
        pt.identity_map_addressrange(
            range,
            Permissions::READ | Permissions::WRITE | Permissions::EXECUTE,
            alloc,
        )?
    }

    store_pagetable(pt);

    Ok(())
}
