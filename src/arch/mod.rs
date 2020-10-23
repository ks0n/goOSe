use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        compile_error!("X86 is not supported");
    }
    else if #[cfg(target_arch = "riscv64")] {
        pub mod riscv64;
        pub use riscv64::*;
    }
}

/// Warning: Symbol only, do NOT use the value directly
extern "Rust" {
    pub static TEXT_START: ();
    pub static TEXT_END: ();
    pub static DATA_START: ();
    pub static DATA_END: ();
    pub static BSS_START: ();
    pub static BSS_END: ();
    pub static STACK_START: ();
    pub static STACK_END: ();
}
