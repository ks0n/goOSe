use core::arch::asm;
use core::cell::OnceCell;

use hal_core::{
    mm::{self, PageAllocFn, PageMap, Permissions, VAddr},
    Error, Range,
};

mod sv39;
use sv39::{PageTable, Satp, SatpMode};

pub const PAGE_SIZE: usize = PageTable::PAGE_SIZE;

static mut GPT: OnceCell<&'static mut PageTable> = OnceCell::new();

pub fn current() -> &'static mut PageTable {
    unsafe { *GPT.get_mut().unwrap() }
}

pub fn init_paging(
    r: impl Iterator<Item = Range>,
    rw: impl Iterator<Item = Range>,
    rwx: impl Iterator<Item = Range>,
    pre_allocated: impl Iterator<Item = Range>,
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
    let pt_addr = pt as *mut PageTable as usize;
    let ppn = pt_addr >> 12;

    let satp = Satp::with_values(ppn as u64, 0, SatpMode::Sv39);

    unsafe {
        asm!("csrw satp, {}", in(reg)u64::from(satp));
        asm!("sfence.vma");
    }
}
