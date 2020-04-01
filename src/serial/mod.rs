/// Given a specific Baud Rate `ubbr`,
/// One has to set the following bytes
/// UBBR0H = ubbr >> 8
/// UBBR0L = ubbr
///
/// The RX and TX buffers then need to be set-up

static COM1: u16 = 0x3f8;
static COM2: u16 = 0x2f8;
static COM3: u16 = 0x3e8;
static COM4: u16 = 0x2e8;

fn outb(port: u16, byte: u8) {
    unsafe {
        asm!("outb %al, %dx" ::
         "{dx}"(port), "{al}"(byte) ::
         "volatile");
    }
}

pub fn test_write(data: &str) {
    for byte in data.bytes() {
        outb(COM1, byte);
    }
}
