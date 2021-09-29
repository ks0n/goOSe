/// The datasheet used to write this is:
/// http://caro.su/msx/ocm_de1/16550.pdf

pub const QEMU_VIRT_BASE_ADDRESS: usize = 0x10000000;

const TRANSMITTER_HOLDING_REGISTER: usize = 0;
const INTERRUPT_ENABLE_REGISTER: usize = 1;

pub struct Ns16550 {
    base_register_address: *mut u8,
}

impl Ns16550 {
    pub fn new(base_register_address: usize) -> Self {
        Self {
            base_register_address: base_register_address as *mut u8,
        }
    }

    pub fn write(&self, data: &str) {
        for byte in data.bytes() {
            self.write_transmitter_holding_reg(byte);
        }
    }

    pub fn enable_data_ready_interrupt(&self) {
        // Data ready is the first bit of the Interrupt Enable Register
        unsafe {
            let addr = self.base_register_address.add(INTERRUPT_ENABLE_REGISTER);
            addr.write_volatile(1 << 0)
        }
    }

    fn write_transmitter_holding_reg(&self, byte: u8) {
        unsafe {
            let addr = self.base_register_address.add(TRANSMITTER_HOLDING_REGISTER);
            addr.write_volatile(byte);
        }
    }
}
