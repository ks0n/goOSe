/// The datasheet used to write this is:
/// http://caro.su/msx/ocm_de1/16550.pdf

pub const QEMU_VIRT_BASE_ADDRESS: usize = 0x10000000;

pub struct Ns16550 {
    base_register_address: usize,
}

impl Ns16550 {
    pub fn new(base_register_address: usize) -> Self {
        Self {
            base_register_address,
        }
    }

    pub fn write(&self, data: &str) {
        for byte in data.bytes() {
            self.write_transmitter_holding_reg(byte);
        }
    }

    fn write_transmitter_holding_reg(&self, byte: u8) {
        // Transmitter Holding Register: +0 offset
        let addr = (self.base_register_address) as *mut u8;

        unsafe { core::ptr::write_volatile(addr, byte) }
    }
}
