use crate::kmain;

pub static UART0: usize = 0x3f8;

#[no_mangle]
#[link_section = ".start"]
pub unsafe extern "C" fn _start() -> () {
    // asm!("mov sp, _stack");

    kmain();
}

pub fn outb(addr: usize, byte: u8) {
    unsafe {
        asm!("out dx, al", in("al")byte, in("dx")addr);
    }
}

pub fn busy_loop() -> ! {
    unsafe {
        asm!("cli");
    }

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
