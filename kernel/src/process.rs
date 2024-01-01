use crate::hal::{self, mm::PageTable};
use crate::{error::Error, globals};
use hal_core::mm::PageMap;

pub struct Process<'a> {
    pub pagetable: &'a mut PageTable,
}

impl<'a> Process<'a> {
    pub fn new() -> Result<Self, Error> {
        Ok(Self {
            pagetable: PageTable::new(&globals::PHYSICAL_MEMORY_MANAGER)?,
        })
    }
}
