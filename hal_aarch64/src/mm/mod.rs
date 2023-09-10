use hal_core::{
    mm::{self, PageAllocFn, PageMap},
    AddressRange, Error,
};

use cortex_a::asm::barrier;
use cortex_a::registers::*;
use tock_registers::interfaces::{ReadWriteable, Writeable};

mod pgt48;

use pgt48::PageTable;

pub type EntryType = usize;

pub const PAGE_SIZE: usize = PageTable::PAGE_SIZE;

use core::cell::OnceCell;

static mut GPT: OnceCell<&'static mut PageTable> = OnceCell::new();

pub fn current() -> &'static mut PageTable {
    unsafe { GPT.get_mut().unwrap() }
}

pub fn init_paging(
    r: impl Iterator<Item = AddressRange>,
    rw: impl Iterator<Item = AddressRange>,
    rwx: impl Iterator<Item = AddressRange>,
    pre_allocated: impl Iterator<Item = AddressRange>,
    alloc: PageAllocFn,
) -> Result<(), Error> {
    hal_core::mm::init_paging::<PageTable>(r, rw, rwx, pre_allocated, alloc, |pt| {
        // TODO: put into into the hal_core::Error
        unsafe {
            if GPT.set(pt).is_err() {
                panic!("GPT is already set ?");
            }
        };
        unsafe {
            load_pagetable(current());
        };
    })?;

    Ok(())
}

unsafe fn load_pagetable(pt: &'static mut PageTable) {
    MAIR_EL1.write(
        // Attribute 0 - NonCacheable normal DRAM. FIXME: enable cache?
        MAIR_EL1::Attr0_Normal_Outer::NonCacheable + MAIR_EL1::Attr0_Normal_Inner::NonCacheable,
    );
    TTBR0_EL1.set_baddr((pt as *const PageTable) as u64);
    TCR_EL1.write(
        TCR_EL1::TBI0::Used
        + TCR_EL1::IPS::Bits_48
        + TCR_EL1::TG0::KiB_4
        // + TCR_EL1::SH0::Inner
        + TCR_EL1::SH0::None
        // + TCR_EL1::ORGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable
        + TCR_EL1::ORGN0::NonCacheable
        // + TCR_EL1::IRGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable
        + TCR_EL1::IRGN0::NonCacheable
        + TCR_EL1::EPD0::EnableTTBR0Walks
        + TCR_EL1::A1::TTBR0
        + TCR_EL1::T0SZ.val(16)
        + TCR_EL1::EPD1::DisableTTBR1Walks,
    );

    barrier::isb(barrier::SY);

    SCTLR_EL1.modify(SCTLR_EL1::M::Enable);

    barrier::isb(barrier::SY);
}

pub fn align_up(addr: usize) -> usize {
    mm::align_up(addr, PAGE_SIZE)
}

pub fn align_down(addr: usize) -> usize {
    mm::align_down(addr, PAGE_SIZE)
}
