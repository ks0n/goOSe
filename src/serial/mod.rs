pub struct Serial {
    port: usize,
}

impl Serial {
    pub fn new(port: usize) -> Self {
        Self { port }
    }

    pub fn write(&self, data: &str) {
        for byte in data.bytes() {
            self.write_byte(byte);
        }
    }

    fn write_byte(&self, byte: u8) {
        let addr = self.port as *mut u8;
        unsafe {
            *addr = byte;
        }
    }
}
