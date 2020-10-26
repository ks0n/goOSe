use crate::println;
use bitfield::bitfield;

use core::mem;

pub const PAGE_SIZE: usize = 4096;
const ENTRIES_PER_PAGE: usize = PAGE_SIZE / mem::size_of::<PageEntry>();
const MAX_LEVEL: usize = 3;

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
    pub entries: [PageEntry; ENTRIES_PER_PAGE],
}

impl PageEntry {
    pub fn new() -> PageEntry {
        PageEntry {
            ..Default::default()
        }
    }

    pub fn init(&mut self, level: usize, addr: usize) -> usize {
        let mut addr = addr + PAGE_SIZE;
        println!(
            "Init a PageEntry of level {} and which point to {:#x}",
            level, addr
        );

        self.set_ppn(addr);
        self.set_valid(true);

        if level == MAX_LEVEL {
            self.set_read(true);
            self.set_write(true);
            self.set_execute(true);
        } else {
            self.set_read(false);
            self.set_write(false);
            self.set_execute(false);

            let page_table = addr as *mut PageTable;
            addr = unsafe { (*page_table).init(level + 1, addr) }
        }

        addr
    }

    pub fn set_ppn(&mut self, ppn: usize) {
        self.set_ppn2((ppn >> 18) as u64);
        self.set_ppn1(((ppn >> 9) & 0x1ff) as u64); // 0x1ff for queping only the 9 last bits
        self.set_ppn1(((ppn) & 0x1ff) as u64);
    }
}

impl PageTable {
    pub fn init(&mut self, level: usize, addr: usize) -> usize {
        println!("Init a PageTable at addres: {:p} and level {}", self, level);
        let mut addr = addr;
        for page_entry in &mut self.entries {
            addr = page_entry.init(level, addr)
        }

        addr
    }
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

pub fn new(addr: usize) -> PageTable {
    println!("MMU Initialization");
    println!("Root PageTable = {:#x}", addr);

    let root_pagetable = addr as *mut PageTable;
    let level = 0;

    unsafe {
        (*root_pagetable).init(level, addr);
    }

    let root: &mut PageTable = unsafe { &mut *root_pagetable };
    mem::take(root)
}
