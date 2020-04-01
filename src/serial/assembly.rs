pub static COM1: u16 = 0x3f8;

/* Avoid unused code, but might be useful later
pub static COM2: u16 = 0x2f8;
pub static COM3: u16 = 0x3e8;
pub static COM4: u16 = 0x2e8;

pub static THR_OFF: u16 = 0x0;
pub static RBR_OFF: u16 = 0x0;
*/
pub static DLL_OFF: u16 = 0x0;
pub static DLH_OFF: u16 = 0x1;
/*
pub static IER_OFF: u16 = 0x1;
pub static IIR_OFF: u16 = 0x2;
pub static FCR_OFF: u16 = 0x2;
pub static LCR_OFF: u16 = 0x3;
pub static MCR_OFF: u16 = 0x4;
pub static LSR_OFF: u16 = 0x5;
pub static MSR_OFF: u16 = 0x6;
pub static SR_OFF: u16 = 0x7;
*/

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
