use crate::kmain;

pub static UART0: usize = 0x3f8;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    kmain();
}

pub fn outb(port: usize, byte: u8) {
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
