use crate::kmain;

pub static UART0: usize = 0x10000000;

#[no_mangle]
#[link_section = ".start"]
pub unsafe extern "C" fn _start() -> () {
    asm!("la sp, _stack");

    kmain();
}

pub fn outb(addr: usize, byte: u8) {
    unsafe {
        let reg = addr as *mut u8;
        reg.add(0).write_volatile(byte);
    }
}
