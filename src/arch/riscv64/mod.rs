use crate::kmain;
use cfg_if::cfg_if;

pub static UART0: usize = 0x10000000;

cfg_if! {
    if #[cfg(test)] {
        use qemu_exit;
        pub static QEMU_EXIT: qemu_exit::RISCV64 = qemu_exit::RISCV64::new(0x100000);
    }
}

#[no_mangle]
#[link_section = ".start"]
pub unsafe extern "C" fn _start() -> ! {
    asm!("la sp, _stack");

    kmain();
}

pub fn outb(addr: usize, byte: u8) {
    let addr = addr as *mut u8;
    unsafe {
        *addr = byte;
    }
}
