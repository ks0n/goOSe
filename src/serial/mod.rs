static COM1: u16 = 0x3f8;

fn outb(port: u16, byte: u8) {
    unsafe {
        asm!("outb %al, %dx" ::
         "{dx}"(port), "{al}"(byte) ::
         "volatile");
    }
}

pub fn write_str(data: &str) {
    for byte in data.bytes() {
        outb(COM1, byte);
    }
}
