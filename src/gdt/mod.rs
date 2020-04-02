use crate::asm_wrappers;

#[derive(Clone, Copy)]
#[repr(packed)]
pub struct Segment {
}

impl Segment {
    pub fn new() -> Segment {
        todo!();
    }
}

#[repr(packed)]
pub struct GlobalDescriptorTable {
}

impl GlobalDescriptorTable {
    pub fn init() -> GlobalDescriptorTable {
        asm_wrappers::cli();

        todo!();

        asm_wrappers::sti();
    }


    pub fn add_entry(seg: Segment) {
        todo!();
    }
}
