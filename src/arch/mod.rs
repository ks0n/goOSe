#[cfg(target_arch = "riscv64")]
mod riscv64;

use cfg_if::cfg_if;

pub fn new_arch() -> impl Architecture {
    cfg_if! {
        if #[cfg(target_arch = "riscv64")] {
            return riscv64::Riscv64::new();
        } else {
            core::compile_error!("Architecture not supported! Did you run `gen_cargo.sh`?");
        }
    }
}

pub trait Architecture {
    unsafe extern "C" fn _start() -> !;

    fn init_interrupts(&mut self);
}
