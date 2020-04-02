pub fn outb(port: u16, byte: u8) {
    unsafe {
        asm!("outb %al, %dx" ::
         "{dx}"(port), "{al}"(byte) ::
         "volatile");
    }
}

pub fn inb(port: u16) -> u8 {
    let mut byte: u8 = 0x0;

    unsafe {
        asm!("inb %dx, %al" ::
         "{dx}"(port), "{al}"(byte) ::
         "volatile");
    }

    byte
}

pub fn cli() {
    unsafe {
        asm!("cli");
    }
}

pub fn sti() {
    unsafe {
        asm!("sti");
    }
}
