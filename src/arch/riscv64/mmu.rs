use bitfield::bitfield;

use core::alloc::boxed;
use core::mem;

const PAGE_SIZE: usize = 4096;
const ENTRIES_PER_PAGE: usize = PAGE_SIZE / mem::size_of::<PageEntry>();

bitfield! {
    #[derive(Default, Copy, Clone)]
    pub struct PageEntry(u64);

    impl Debug;

    valid, set_valid: 0;
    read, set_read: 1;
    write, set_write: 2;
    execute, set_execute: 3;
    user, set_user: 4;
    global, set_global: 5;
    accessed, set_accessed: 6;
    dirty, set_dirty: 7;
    rsw, set_rsw: 9, 8;
    ppn0, set_ppn0: 18, 10;
    ppn1, set_ppn1: 27, 19;
    ppn2, set_ppn2: 53, 28;
    reserved, set_reserved: 63, 54;
}

pub struct PageTable {
    entries: [PageEntry; ENTRIES_PER_PAGE],
}

impl PageEntry {
    pub fn new() -> PageEntry {
        PageEntry {
            ..Default::default()
        }
    }

    pub fn set_ppn(&mut self, ppn: usize) {
        self.set_ppn2((ppn >> 18) as u64);
        self.set_ppn1(((ppn >> 9) & 0x1ff) as u64); // 0x1ff for queping only the 9 last bits
        self.set_ppn1(((ppn) & 0x1ff) as u64);
    }
}

impl PageTable {
    pub fn size() -> usize {
        mem::size_of::<PageTable>()
    }
}

impl Default for PageTable {
    fn default() -> PageTable {
        PageTable {
            entries: [Default::default(); ENTRIES_PER_PAGE],
        }
    }
}

// pub fn new(addr: usize) -> PageTable {
//     extern "Rust" {
//         mmu: PageTable,
//     }
//     let root_pagetable: PageTable = (addr as *mut PageTable).read();
//
//     root_pagetable
// }
