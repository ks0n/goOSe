//! This module implements a "platform agnosticity" layer. No matter what platform you're trying
//! to get goOSe working on, all of the conditions defined here must be met. On top of
//! that, using `use arch` in the code will automatically dispatch to the architecture
//! you're using at compile time. The arch module uses external symbols that should
//! be defined by the linker script for any architecture.

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
    pub static HEAP_START: ();
    pub static HEAP_END: ();
    pub static STACK_START: ();
    pub static STACK_END: ();
}
