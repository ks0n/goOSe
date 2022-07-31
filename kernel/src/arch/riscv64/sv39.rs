use crate::mm;
use crate::paging;
use crate::paging::Error;
use crate::paging::PagingImpl;
use core::arch::asm;
use core::convert::TryInto;
use core::ops::DerefMut;
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
    fn clear(&mut self) {
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

pub struct PageTable {
    entries: [PageTableEntry; 512],
}

impl PageTable {
    fn map_inner(
        &mut self,
        mut kernel_page_table: Option<&mut Self>,
        mut allocator: Option<&mut mm::PhysicalMemoryManager>,
        paddr: PAddr,
        vaddr: VAddr,
        perms: mm::Permissions,
    ) -> Result<&mut PageTableEntry, paging::Error> {
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
                if let Some(alloc) = allocator.as_mut() {
                    let new_page_table = PageTable::new(kernel_page_table.as_deref_mut(), alloc);

                    // Set new PageTable as target of this entry
                    pte.set_target(new_page_table as *mut PageTable);
                    pte.set_valid();
                } else {
                    // We need to allocate but we did not have an allocator
                    return Err(paging::Error::CannotMapNoAlloc);
                }
            }

            // Get the next level PageTable
            pagetable = pte.get_target();
        }

        unreachable!("We should have returned by now");
    }
}

impl PagingImpl for PageTable {
    fn new<'alloc>(
        kernel_pagetable: Option<&mut Self>,
        allocator: &mut mm::PhysicalMemoryManager,
    ) -> &'alloc mut Self {
        // FIXME: No unwrap here
        let page = allocator.alloc_pages(1).unwrap();
        let page_table: *mut PageTable = page.into();

        if let Some(kernel_pagetable) = kernel_pagetable {
            let page_table_addr = page_table as usize;
            // FIXME: No unwrap here
            kernel_pagetable
                .map_noalloc(
                    page_table_addr.into(),
                    page_table_addr.into(),
                    mm::Permissions::READ | mm::Permissions::WRITE,
                )
                .unwrap();
        }

        // FIXME: Do not unwrap either
        let page_table = unsafe { page_table.as_mut().unwrap() };

        page_table.entries.iter_mut().for_each(|pte| pte.clear());

        page_table
    }

    fn get_page_size() -> usize {
        4096
    }

    fn get_uppermost_address() -> usize {
        0x7fffffffff
    }

    fn map(
        &mut self,
        kernel_page_table: Option<&mut Self>,
        allocator: &mut mm::PhysicalMemoryManager,
        pa: mm::PAddr,
        va: mm::VAddr,
        perms: mm::Permissions,
    ) -> Result<(), Error> {
        self.map_inner(
            kernel_page_table,
            Some(allocator),
            pa.into(),
            va.into(),
            perms,
        )?;

        Ok(())
    }

    fn map_noalloc(
        &mut self,
        pa: mm::PAddr,
        va: mm::VAddr,
        perms: mm::Permissions,
    ) -> Result<(), paging::Error> {
        self.map_inner(None, None, pa.into(), va.into(), perms)?;

        Ok(())
    }

    fn add_invalid_entry(
        &mut self,
        allocator: &mut mm::PhysicalMemoryManager,
        vaddr: mm::VAddr,
    ) -> Result<(), Error> {
        let pte = self.map_inner(
            None,
            Some(allocator),
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
