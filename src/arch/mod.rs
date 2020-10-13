use crate::kmain;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        mod x86_64;
        pub use x86_64::*;
        extern crate rlibc;
    }
    else if #[cfg(target_arch = "riscv64")] {
        pub mod riscv64;
        pub use riscv64::*;

        #[used]
        pub static ARCH: riscv64::RISCV64 = riscv64::RISCV64{};
    }
}

/// Warning: Symbol only, do NOT use the value directly
extern "Rust" {
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

#[no_mangle]
#[link_section = ".start"]
unsafe extern "C" fn kstart() -> ! {
    #[cfg(target_arch = "riscv64")]
    asm!(
        "la sp, STACK_START
          call init"
    );

    kmain();
}

#[no_mangle]
fn init() {
    ARCH.init();
}

trait Arch {
    fn init(&self);
}
