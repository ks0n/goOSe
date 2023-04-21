use super::Error;

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

    fn map(&mut self, va: VAddr, pa: PAddr, perms: Permissions, alloc: PageAllocFn) -> Result<&mut Self::Entry, Error>;

    fn add_invalid_entry(&mut self, va: VAddr, alloc: PageAllocFn) -> Result<(), Error>;

    fn identity_map(&mut self, addr: VAddr, perms: Permissions, alloc: PageAllocFn) -> Result<(), Error> {
        self.map(addr, PAddr::new(addr.val), perms, alloc)
            .map(|_| ())
    }

    fn identity_map_range(&mut self, addr: VAddr, page_count: usize, perms: Permissions, alloc: PageAllocFn) -> Result<(), Error> {
        let start = addr.val;
        for i in 0..page_count {
            self.identity_map(VAddr::new(start + i*Self::PAGE_SIZE), perms, alloc)?;
        }

        Ok(())
    }
}
