#[cfg(target_arch = "riscv64")]
mod riscv64;

pub fn new_arch() -> impl Architecture {
    #[cfg(target_arch = "riscv64")]
    return riscv64::Riscv64::new();

    #[allow(unreachable_code)]
    {
        // Block is needed as `unreachable!` will expand to multiple lines
        unreachable!("Architecture not supported! Did you run `gen_cargo.sh`?");
    }
}

pub trait Architecture {}
