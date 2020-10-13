use super::*;
use crate::println;
use cfg_if::cfg_if;

pub static UART0: usize = 0x10000000;

cfg_if! {
    if #[cfg(test)] {
        use qemu_exit;
        pub static QEMU_EXIT: qemu_exit::RISCV64 = qemu_exit::RISCV64::new(0x100000);
    }
}

pub struct RISCV64 {}

impl Arch for RISCV64 {
    fn init(&self) {
        println!("\nRISCV64 Init"); // Separation from OpenSBI boot info
        clear_bss();
    }
}

fn clear_bss() {
    let _bss_start = unsafe { (&BSS_START as *const ()) as usize };
    let _bss_end = unsafe { (&BSS_END as *const ()) as usize };

    for addr in _bss_start.._bss_end {
        let addr = addr as *mut u8;
        unsafe {
            *addr = 0;
        }
    }

    println!("BSS cleared ({:#X} -> {:#X})", _bss_start, _bss_end);
}

pub fn outb(addr: usize, byte: u8) {
    let addr = addr as *mut u8;
    unsafe {
        *addr = byte;
    }
}
