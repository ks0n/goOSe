use crate::kmain;
use crate::println;
use cfg_if::cfg_if;

pub static UART0: usize = 0x10000000;

cfg_if! {
    if #[cfg(test)] {
        use qemu_exit;
        pub static QEMU_EXIT: qemu_exit::RISCV64 = qemu_exit::RISCV64::new(0x100000);
    }
}

extern "C" {
    pub static START_START: ();
    pub static START_END: ();
    pub static TEXT_START: ();
    pub static TEXT_END: ();
    pub static DATA_START: ();
    pub static DATA_END: ();
    pub static BSS_START: ();
    pub static BSS_END: ();
    pub static STACK_START: ();
    pub static STACK_END: ();
}

// pub static RODATA_START: usize = &_rodata_start;
// pub static RODATA_END: *const !;
// pub static STACK_START: *const !;
// pub static STACK_END: *const !;

#[no_mangle]
#[link_section = ".start"]
pub unsafe extern "C" fn _start() -> ! {
    asm!(
        "la sp, STACK_START
          call init"
    );

    kmain();
}

#[no_mangle]
extern "C" fn init() {
    println!("\nRISCV64 Init"); // Separation from OpenSBI boot info
    clear_bss();
}

fn clear_bss() {
    let _BSS_START = unsafe { (&BSS_START as *const ()) as usize };
    let _BSS_END = unsafe { (&BSS_END as *const ()) as usize };

    for addr in _BSS_START.._BSS_END {
        let addr = addr as *mut u8;
        unsafe {
            *addr = 0;
        }
    }

    println!("BSS cleared ({:#X} -> {:#X})", _BSS_START, _BSS_END);
}

pub fn outb(addr: usize, byte: u8) {
    let addr = addr as *mut u8;
    unsafe {
        *addr = byte;
    }
}
