use hal_core::{
    mm::{self, PageAlloc, PageMap},
    AddressRange, Error,
};

pub mod pgt48;

use pgt48::PageTable;

pub type EntryType = usize;

pub const PAGE_SIZE: usize = PageTable::PAGE_SIZE;

use core::cell::OnceCell;

static mut GPT: OnceCell<&'static mut PageTable> = OnceCell::new();

pub fn is_pagetable_installed() -> bool {
    unsafe { GPT.get_mut().is_some() }
}

pub fn prefill_pagetable(
    r: impl Iterator<Item = AddressRange>,
    rw: impl Iterator<Item = AddressRange>,
    rwx: impl Iterator<Item = AddressRange>,
    pre_allocated: impl Iterator<Item = AddressRange>,
    allocator: &impl PageAlloc,
) -> Result<(), Error> {
    let pt = hal_core::mm::prefill_pagetable::<PageTable>(r, rw, rwx, pre_allocated, allocator)?;

    // TODO: put into into the hal_core::Error
    unsafe {
        if GPT.set(pt).is_err() {
            panic!("GPT is already set ?");
        }
    };

    Ok(())
}

pub fn align_up(addr: usize) -> usize {
    mm::align_up(addr, PAGE_SIZE)
}

pub fn align_down(addr: usize) -> usize {
    mm::align_down(addr, PAGE_SIZE)
}
