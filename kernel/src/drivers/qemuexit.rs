use qemu_exit;
use qemu_exit::QEMUExit;

use super::Driver;

#[cfg(target_arch = "riscv64")]
const RISCV64_BASE_ADDRESS: usize = 0x100000;

pub struct QemuExit {
    #[cfg(target_arch = "riscv64")]
    address: usize,
    #[cfg(target_arch = "riscv64")]
    inner: qemu_exit::RISCV64,
    #[cfg(target_arch = "aarch64")]
    inner: qemu_exit::AArch64,
}

impl QemuExit {
    pub fn new() -> Self {
        #[cfg(target_arch = "riscv64")]
        return Self {
            address: RISCV64_BASE_ADDRESS,
            inner: qemu_exit::RISCV64::new(RISCV64_BASE_ADDRESS as u64),
        };

        #[cfg(target_arch = "aarch64")]
        return Self {
            inner: qemu_exit::AArch64::new(),
        };
    }

    pub fn exit_success(&self) -> ! {
        self.inner.exit_success();
    }

    pub fn exit_failure(&self) -> ! {
        self.inner.exit_failure();
    }
}

impl Default for QemuExit {
    fn default() -> Self {
        Self::new()
    }
}

impl Driver for QemuExit {
    fn get_address_range(&self) -> Option<(usize, usize)> {
        #[cfg(target_arch = "riscv64")]
        return Some((self.address, 1));

        #[cfg(target_arch = "aarch64")]
        return None;
    }
}
