/// Driver fot the RISC-V Platform-Level Interrupt Controller
/// Documentation: https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc

pub const QEMU_VIRT_PLIC_BASE_ADDRESS: usize = 0xc000000;

const PLIC_ENABLE_OFFSET: usize = 0x002080;
const PLIC_THRESHOLD_OFFSET: usize = 0x201000;
const PLIC_NUMBER_SOURCES: u16 = 1024;
const PLIC_NUMBER_INTERRUPT_SOURCE_BY_REGISTER: u8 = 32;
const PLIC_NUMBER_SOURCE_REGISTER: u16 =
    PLIC_NUMBER_SOURCES / PLIC_NUMBER_INTERRUPT_SOURCE_BY_REGISTER as u16;
const PLIC_MAX_CONTEXT: u16 = 0x3e00;

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

        unsafe {
            let addr = (self.base_register_address + PLIC_THRESHOLD_OFFSET) as *mut u32;
            addr.write_volatile(threshold as u32);
        }
    }

    pub fn enable_interrupt(&self, id: u16, hart: u16) -> Result<(), &str> {
        if id >= PLIC_NUMBER_SOURCES {
            return Err("enable_interrupt: Id is higher than PLIC_MAX_INTERRUPT_SOURCE");
        }

        if hart >= PLIC_MAX_CONTEXT {
            return Err("enable_interrupt: hart is higher than PLIC_MAX_CONTEXT");
        }

        let source_offset = (id / PLIC_NUMBER_INTERRUPT_SOURCE_BY_REGISTER as u16
            + hart * PLIC_NUMBER_SOURCE_REGISTER) as usize;
        let id_shift = 1 << (id % PLIC_NUMBER_INTERRUPT_SOURCE_BY_REGISTER as u16);

        unsafe {
            let addr =
                (self.base_register_address + PLIC_ENABLE_OFFSET + source_offset) as *mut u32;
            let current_interrupt = core::ptr::read_volatile(addr);
            addr.write_volatile(current_interrupt | id_shift);
        }

        Ok(())
    }

    pub fn set_priority(&self, id: u16, priority: u32) -> Result<(), &str> {
        if id >= PLIC_NUMBER_SOURCES {
            return Err("set_priority: Id is higher than PLIC_MAX_INTERRUPT_SOURCE");
        }

        unsafe {
            let addr = (self.base_register_address + (id * 4) as usize) as *mut u32;
            addr.write_volatile(priority);
        }

        Ok(())
    }
}
