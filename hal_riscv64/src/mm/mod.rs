use core::arch::asm;
use core::cell::OnceCell;
use hal_core::{
    mm::{self, PageAlloc, PageMap},
    AddressRange, Error,
};

mod sv39;
use sv39::{PageTable, Satp, SatpMode};

pub const PAGE_SIZE: usize = PageTable::PAGE_SIZE;

static mut GPT: OnceCell<&'static mut PageTable> = OnceCell::new();

pub fn current() -> &'static mut PageTable {
    unsafe { *GPT.get_mut().unwrap() }
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

pub fn enable_paging() {
    unsafe {
        load_pagetable(current());
    }
}

unsafe fn load_pagetable(pt: &'static mut PageTable) {
    let pt_addr = pt as *mut PageTable as usize;
    let ppn = pt_addr >> 12;

    let satp = Satp::with_values(ppn as u64, 0, SatpMode::Sv39);

    unsafe {
        asm!("csrw satp, {}", in(reg)u64::from(satp));
        asm!("sfence.vma");
    }
}

pub fn align_down(addr: usize) -> usize {
    mm::align_down(addr, PageTable::PAGE_SIZE)
}

pub fn align_up(addr: usize) -> usize {
    mm::align_up(addr, PageTable::PAGE_SIZE)
}
