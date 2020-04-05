pub fn outb(port: u16, byte: u8) {
    unsafe {
        asm!("outb %al, %dx"
            :
            : "{dx}"(port), "{al}"(byte)
            :
            : "volatile");
    }
}

pub fn inb(port: u16) -> u8 {
    let ret_byte: u8;

    unsafe {
        asm!("inb %dx, %al"
            : "={al}"(ret_byte)
            : "{dx}"(port)
            :
            : "volatile");
    }

    ret_byte
}

pub fn lgdt(gdt: u64) {
    unsafe {
        asm!("lgdt ($0)"
             :
             : "r"(gdt)
             :
             :);
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
