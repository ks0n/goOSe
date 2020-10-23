use bitfield::bitfield;
use crate::println;

use core::mem;

bitfield!{
    // #[repr(packed())]
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

impl PageEntry {
    pub fn new() -> PageEntry {
        let test = 0xffffffffffffffff as u64;

        let entry: PageEntry = unsafe {mem::transmute::<u64, PageEntry>(test)};
        println!("{:#X}", entry.reserved());
        entry
    }
}
