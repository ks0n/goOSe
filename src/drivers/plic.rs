pub const QEMU_VIRT_PLIC_BASE_ADDRESS: usize = 0xc000000;
pub const PLIC_ENABLE_OFFSET: usize = 0x002080;
pub const PLIC_THRESHOLD_OFFSET: usize = 0x201000;

pub struct Plic {
    base_register_address: usize,
    threshold: u8,
}

impl Plic {
    pub fn new(base_register_address: usize) -> Self {
        Self {
            base_register_address,
            threshold: 0,
        }
    }

    pub fn set_threshold(&mut self, threshold: u8) {
        self.threshold = threshold;

        let addr = (self.base_register_address + PLIC_THRESHOLD_OFFSET) as *mut u32;

        unsafe {
            core::ptr::write_volatile(addr, threshold as u32);
        }
    }

    pub fn enable_interrupt(&self, id: u16) {
        // FIXME: This only work if id <= 31 and the interrupt will only be on hart0
        // See https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc#3-memory-map
        let addr = (self.base_register_address + PLIC_ENABLE_OFFSET) as *mut u32;
        let id_shift = 1 << id;

        unsafe {
            let current_interrupt = core::ptr::read_volatile(addr);
            core::ptr::write_volatile(addr, current_interrupt | id_shift);
        }
    }

    pub fn set_priority(&self, id: u16, priority: u32) {
        let addr = (self.base_register_address + (id * 4) as usize) as *mut u32;
        unsafe {
            core::ptr::write_volatile(addr, priority);
        }
    }
}
