use qemu_exit;
use qemu_exit::QEMUExit;

use crate::Driver;

const RISCV64_BASE_ADDRESS: usize = 0x100000;

pub struct QemuExit {
    address: usize,
    #[cfg(target_arch = "riscv64")]
    inner: qemu_exit::RISCV64,
}

impl QemuExit {
    pub fn new() -> Self {
        #[cfg(target_arch = "riscv64")]
        Self {
            address: RISCV64_BASE_ADDRESS,
            inner: qemu_exit::RISCV64::new(RISCV64_BASE_ADDRESS as u64),
        }
    }

    pub fn exit_success(&self) {
        self.inner.exit_success();
    }

    pub fn exit_failure(&self) {
        self.inner.exit_failure();
    }
}

impl Default for QemuExit {
    fn default() -> Self {
        Self::new()
    }
}

impl Driver for QemuExit {
    fn get_address_range(&self) -> (usize, usize) {
        (self.address, 1)
    }
}
