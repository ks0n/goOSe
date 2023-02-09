use super::{Error, Range};

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
}

pub fn align_up(val: usize, page_sz: usize) -> usize {
    ((val + page_sz - 1) / page_sz) * page_sz
}

pub fn init_paging<P: PageMap + 'static>(
    r: impl Iterator<Item = Range>,
    rw: impl Iterator<Item = Range>,
    rwx: impl Iterator<Item = Range>,
    pre_allocated: impl Iterator<Item = Range>,
    alloc: PageAllocFn,
    store_pagetable: impl FnOnce(&'static mut P),
) -> Result<(), Error> {
    let pt: &'static mut P = P::new(alloc)?;
    let page_size = P::PAGE_SIZE;

    for (addr, len) in pre_allocated {
        let len = align_up(len, page_size);
        for base in (addr..=addr + len).step_by(page_size) {
            pt.add_invalid_entry(VAddr::new(base), alloc)?;
        }
    }

    for (addr, len) in r {
        let page_count = align_up(len, page_size) / page_size;
        pt.identity_map_range(VAddr::new(addr), page_count, Permissions::READ, alloc)?;
    }

    for (addr, len) in rw {
        let page_count = align_up(len, page_size) / page_size;
        pt.identity_map_range(
            VAddr::new(addr),
            page_count,
            Permissions::READ | Permissions::WRITE,
            alloc,
        )?;
    }

    for (addr, len) in rwx {
        let page_count = align_up(len, page_size) / page_size;
        pt.identity_map_range(
            VAddr::new(addr),
            page_count,
            Permissions::READ | Permissions::WRITE | Permissions::EXECUTE,
            alloc,
        )?
    }

    store_pagetable(pt);

    Ok(())
}
