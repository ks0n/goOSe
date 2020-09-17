pub fn outb(port: u16, byte: u8) {
    unsafe {
        asm!("out dx, al", in("al")byte, in("dx")port);
    }
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
