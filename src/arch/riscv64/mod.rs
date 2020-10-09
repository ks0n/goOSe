use crate::kmain;
use cfg_if::cfg_if;

pub static UART0: usize = 0x10000000;

cfg_if! {
    if #[cfg(test)] {
        use qemu_exit;
        pub static QEMU_EXIT: qemu_exit::RISCV64 = qemu_exit::RISCV64::new(0x100000);
    }
}

extern "C" {
    pub static START_START: usize;
    pub static START_END: usize;
    pub static TEXT_START: usize;
    pub static TEXT_END: usize;
    pub static DATA_START: usize;
    pub static DATA_END: usize;
    pub static RODATA_START: usize;
    pub static RODATA_END: usize;
    pub static BSS_START: usize;
    pub static BSS_END: usize;
    pub static STACK_START: usize;
    pub static STACK_END: usize;
}

#[no_mangle]
#[link_section = ".start"]
pub unsafe extern "C" fn _start() -> ! {
    asm!("la sp, STACK_START");

    clear_bss();

    kmain();
}

pub unsafe fn clear_bss() {
    for addr in BSS_START..BSS_END {
        let addr = addr as *mut u8;
        *addr = 0;
    }
}

pub fn outb(addr: usize, byte: u8) {
    let addr = addr as *mut u8;
    unsafe {
        *addr = byte;
    }
}
