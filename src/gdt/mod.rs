#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct Segment {
    base: u32,
    limit: u32,
    flag: u16,
}

impl Segment {
    pub fn new(base: u32, limit: u32, flag: u16) -> Segment {
        let new_s = Segment {
            base: base,
            limit: limit,
            flag: flag,
        };

        new_s
    }
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct GlobalDescriptorTable {}

impl GlobalDescriptorTable {
    pub fn init() -> GlobalDescriptorTable {
        // asm_wrappers::cli();
        // asm_wrappers::sti();

        todo!();
    }

    // FIXME: Use seg
    pub fn add_entry(_seg: Segment) {
        todo!();
    }

    pub fn load(&self) {
        /* Use mem::transmute ? */
        todo!();
    }
}
