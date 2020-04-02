use crate::asm_wrappers;

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

// FIXME: Needed ?
pub fn syscall_write(data: &[u8], count: usize) {
    for i in 0..count {
        asm_wrappers::outb(COM1, data[i]);
    }
}

pub fn init_serial(port: u16) {
    /* We initialize the Baude Rate of the port to 38400 bps */
    asm_wrappers::outb(port + DLL_OFF, 0x3);
    asm_wrappers::outb(port + DLH_OFF, 0x0);
}

pub fn init_com1() {
    init_serial(COM1);
}

pub fn write_str(data: &str) {
    for byte in data.bytes() {
        if byte == '\n' as u8 {
            asm_wrappers::outb(COM1, '\r' as u8);
            asm_wrappers::outb(COM1, '\n' as u8);
        } else {
            asm_wrappers::outb(COM1, byte);
        }
    }
}
