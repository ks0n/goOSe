use crate::globals;
use crate::mm;
use crate::paging;
use crate::paging::PagingImpl;
use crate::Error;
use core::arch::asm;
use core::convert::TryInto;
use modular_bitfield::{bitfield, prelude::*};

#[repr(C)]
pub struct VAddr {
    addr: u64,
}

impl From<mm::VAddr> for VAddr {
    fn from(vaddr: mm::VAddr) -> Self {
        Self::from_u64(usize::from(vaddr).try_into().unwrap())
    }
}

impl VAddr {
    pub fn from_u64(addr: u64) -> VAddr {
        let mut vaddr = Self { addr };

        let bit38 = (vaddr.vpn(2) >> 8) != 0;

        vaddr.extend_38th_bit(bit38);

        vaddr
    }

    pub fn vpn(&self, nb: usize) -> u16 {
        let vpn = self.addr >> 12;

        ((vpn >> (nb * 9)) & 0x1ff) as u16
    }

    fn extend_38th_bit(&mut self, bit: bool) {
        let mask = ((!1u64) >> 38) << 38;

        if bit {
            self.addr |= mask; // set all the bits above the 38th
        } else {
            self.addr &= !mask; // clear all the bits above the 38th
        }
    }
}

#[repr(C)]
pub struct PAddr {
    addr: u64,
}

impl From<mm::PAddr> for PAddr {
    fn from(paddr: mm::PAddr) -> Self {
        Self::from_u64(usize::from(paddr).try_into().unwrap())
    }
}

impl PAddr {
    pub fn from_u64(addr: u64) -> Self {
        let mut paddr = Self { addr };

        let bit55 = (paddr.ppn(2) >> 8) != 0;

        paddr.extend_55th_bit(bit55);

        paddr
    }

    fn ppn(&self, nb: usize) -> u64 {
        let ppn = self.addr >> 12;

        if nb == 2 {
            (ppn >> (nb * 9)) & 0x3fffff
        } else {
            (ppn >> (nb * 9)) & 0x1ff
        }
    }

    fn extend_55th_bit(&mut self, bit: bool) {
        let mask = ((!0u64) >> 55) << 55;

        if bit {
            self.addr |= mask; // set all the bits above the 55th
        } else {
            self.addr &= !mask; // clear all the bits above the 55th
        }
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
    #[skip]
    g: B1,
    #[skip]
    a: B1,
    #[skip]
    d: B1,
    #[skip]
    rsw: B2,
    ppn0: B9,
    ppn1: B9,
    ppn2: B26,
    #[skip]
    reserved: B10,
}

impl PageTableEntry {
    const fn new_invalid() -> Self {
        let mut pte = Self::new();
        pte.clear();

        pte
    }

    const fn clear(&mut self) {
        *self = PageTableEntry::from_bytes([0u8; 8])
    }

    fn is_valid(&self) -> bool {
        self.v() == 1
    }

    fn set_valid(&mut self) {
        self.set_v(1)
    }

    fn set_paddr(&mut self, paddr: &PAddr) {
        self.set_ppn2(paddr.ppn(2) as u32);
        self.set_ppn1(paddr.ppn(1) as u16);
        self.set_ppn0(paddr.ppn(0) as u16);
    }

    fn set_target(&mut self, pt: *mut PageTable) {
        let addr = pt as u64;
        self.set_paddr(&PAddr::from_u64(addr))
    }

    fn get_target(&mut self) -> &mut PageTable {
        let addr =
            ((self.ppn2() as u64) << 18 | (self.ppn1() as u64) << 9 | self.ppn0() as u64) * 4096u64;
        unsafe { (addr as *mut PageTable).as_mut().unwrap() }
    }

    fn set_perms(&mut self, perms: mm::Permissions) {
        self.set_r(perms.contains(mm::Permissions::READ) as u8);
        self.set_w(perms.contains(mm::Permissions::WRITE) as u8);
        self.set_x(perms.contains(mm::Permissions::EXECUTE) as u8);
        self.set_u(perms.contains(mm::Permissions::USER) as u8);
    }
}

#[repr(align(0x1000))]
pub struct PageTable {
    entries: [PageTableEntry; 512],
}

impl PageTable {
    pub const fn zeroed() -> Self {
        #[allow(clippy::uninit_assumed_init)]
        let mut entries: [PageTableEntry; 512] =
            unsafe { core::mem::MaybeUninit::uninit().assume_init() };
        let mut i = 0;
        while i < entries.len() {
            entries[i] = PageTableEntry::new_invalid();
            i += 1;
        }
        Self { entries }
    }

    fn map_inner(
        &mut self,
        paddr: PAddr,
        vaddr: VAddr,
        perms: mm::Permissions,
    ) -> Result<&mut PageTableEntry, Error> {
        let mut pagetable = self;

        for level in (0..=2).rev() {
            // Get offset for this vaddr
            let vpn = vaddr.vpn(level);
            // Get entry for this vaddr
            let pte = &mut pagetable.entries[vpn as usize];

            // If we are a leaf, add an entry for the paddr
            if level == 0 {
                pte.set_paddr(&paddr);
                pte.set_perms(perms);
                pte.set_valid();

                return Ok(pte);
            }

            // If the entry is not valid we will need to allocate a new PageTable
            if !pte.is_valid() {
                let new_page_table = PageTable::new();

                // Set new PageTable as target of this entry
                pte.set_target(new_page_table? as *mut PageTable);
                pte.set_valid();
            }

            // Get the next level PageTable
            pagetable = pte.get_target();
        }

        unreachable!("We should have returned by now");
    }
}

impl PagingImpl for PageTable {
    fn new() -> Result<&'static mut Self, Error> {
        // FIXME: No unwrap here
        let page = globals::PHYSICAL_MEMORY_MANAGER.lock(|pmm| pmm.alloc_rw_pages(1))?;

        let page_table: *mut PageTable = page.into();
        // Safety: the PMM gave us the memory, it should be a valid pointer.
        let page_table = unsafe { page_table.as_mut().unwrap() };

        page_table.entries.iter_mut().for_each(|pte| pte.clear());

        Ok(page_table)
    }

    fn get_page_size() -> usize {
        4096
    }

    fn map(&mut self, pa: mm::PAddr, va: mm::VAddr, perms: mm::Permissions) -> Result<(), Error> {
        self.map_inner(pa.into(), va.into(), perms)?;

        Ok(())
    }

    fn add_invalid_entry(&mut self, vaddr: mm::VAddr) -> Result<(), Error> {
        let pte = self.map_inner(
            PAddr::from_u64(0x0A0A_0A0A_0A0A_0A0A),
            vaddr.into(),
            mm::Permissions::READ,
        )?;

        pte.set_v(0);

        Ok(())
    }

    fn reload(&mut self) {
        load_pt(self)
    }

    fn disable(&mut self) {
        let satp = Satp::new()
            .with_ppn(0)
            .with_asid(0)
            .with_mode(SatpMode::Bare as u8);

        unsafe {
            asm!("csrw satp, {}", in(reg)u64::from(satp));
            asm!("sfence.vma");
        }
    }
}

#[repr(u8)]
pub enum SatpMode {
    Bare = 0,
    Sv39 = 8,
    _Sv48 = 9,
    _Sv57 = 10,
    _Sv64 = 11,
}

#[repr(u64)]
#[bitfield]
pub struct Satp {
    #[skip(getters)]
    ppn: B44,
    #[skip(getters)]
    asid: B16,
    #[skip(getters)]
    mode: B4,
}

pub fn load_pt(pt: &PageTable) {
    let pt_addr = pt as *const PageTable as usize;
    let ppn = pt_addr >> 12;

    let satp = Satp::new()
        .with_ppn(ppn as u64)
        .with_asid(0)
        .with_mode(SatpMode::Sv39 as u8);

    unsafe {
        asm!("csrw satp, {}", in(reg)u64::from(satp));
        asm!("sfence.vma");
    }
}
