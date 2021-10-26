use super::SimplePageAllocator;
use modular_bitfield::{bitfield, prelude::*};

#[repr(C)]
#[bitfield]
pub struct VAddr {
    page_offset: B12,
    vpn0: B9,
    vpn1: B9,
    vpn2: B9,
    unused: B25,
}

impl VAddr {
    fn try_from_addr(addr: u64) -> VAddr {
        let mut vaddr = VAddr::from_bytes(addr.to_le_bytes());

        let bit38 = vaddr.vpn2() >> 8;

        if bit38 == 1 {
            vaddr.set_unused(!0);
        } else {
            vaddr.set_unused(0);
        }

        vaddr
    }

    pub fn get_vpn(&self, nb: usize) -> u16 {
        match nb {
            0 => self.vpn0(),
            1 => self.vpn1(),
            2 => self.vpn2(),
            _ => unreachable!(),
        }
    }
}

#[repr(C)]
#[bitfield]
pub struct PAddr {
    page_offset: B12,
    ppn0: B9,
    ppn1: B9,
    ppn2: B26,
    unused: B8,
}

impl PAddr {
    fn addr(&self) -> u64 {
        ((self.ppn2() as u64) << 18 | (self.ppn1() as u64) << 9 | self.ppn0() as u64)
            * 4096u64
    }
}

#[repr(u64)]
#[bitfield]
struct PageTableEntry {
    v: B1,
    r: B1,
    w: B1,
    x: B1,
    u: B1,
    g: B1,
    a: B1,
    d: B1,
    rsw: B2,
    ppn0: B9,
    ppn1: B9,
    ppn2: B26,
    reserved: B10,
}

impl PageTableEntry {
    fn clear(&mut self) {
        *self = PageTableEntry::from_bytes([0u8; 8])
    }

    fn is_valid(&self) -> bool {
        self.v() == 1
    }

    fn set_target(&mut self, target: u64) {
        self.set_ppn2(((target / 4096u64) >> 18) as u32);
        self.set_ppn1(((target / 4096u64) >> 9) as u16);
        self.set_ppn0((target / 4096u64) as u16);
    }

    fn get_target(&mut self) -> &mut PageTable {
        let addr = ((self.ppn2() as u64) << 18 | (self.ppn1() as u64) << 9 | self.ppn0() as u64)
            * 4096u64;
        unsafe { (addr as *mut PageTable).as_mut().unwrap() }
    }
}

#[repr(u8)]
pub enum SatpMode {
    Bare = 0,
    Sv39 = 8,
    Sv48 = 9,
    Sv57 = 10,
    Sv64 = 11,
}

pub struct Satp {
    ppn: B44,
    asid: B16,
    mode: B4,
}

pub struct PageTable {
    entries: [PageTableEntry; 512],
}

impl PageTable {
    pub fn new<'alloc>(allocator: &mut SimplePageAllocator<'alloc>) -> &'alloc mut PageTable {
        // FIXME: No unwrap here
        let page = allocator.alloc_pages(1).unwrap();
        let page_table = page as *mut PageTable;
        // FIXME: Do not unwrap either
        let page_table = unsafe { page_table.as_mut().unwrap() };

        page_table.entries.iter_mut().for_each(|pte| pte.clear());

        page_table
    }

    fn map_inner(
        &mut self,
        allocator: &mut SimplePageAllocator,
        paddr: PAddr,
        vaddr: VAddr,
        level: usize,
    ) {
        let vpn = vaddr.get_vpn(level);

        let entry = &mut self.entries[vpn as usize];

        if level == 0 {
            entry.set_target(paddr.addr());
        }

        if !entry.is_valid() {
            let new_page_table = PageTable::new(allocator);
            entry.set_target(new_page_table as *mut PageTable as u64);
        }

        entry.get_target().map_inner(allocator, paddr, vaddr, level - 1);
    }

    pub fn map(
        &mut self,
        allocator: &mut SimplePageAllocator,
        paddr: PAddr,
        vaddr: VAddr,
    ) {
        self.map_inner(allocator, paddr, vaddr, 2)
    }
}
