use crate::kmain;
use cfg_if::cfg_if;

pub static UART0: usize = 0x3f8;

cfg_if! {
    if #[cfg(test)] {
        use qemu_exit;
        pub static QEMU_EXIT: qemu_exit::X86 = qemu_exit::X86::new(0xf4, 253);
    }
}

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
