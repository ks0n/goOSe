use crate::arch;
use crate::arch::ArchitectureInterrupts;

pub struct InterruptManager {
    arch: arch::InterruptsImpl,
}

impl InterruptManager {
    pub fn new() -> Self {
        let arch = arch::InterruptsImpl::new();

        Self { arch }
    }

    pub fn init_interrupts(&mut self) {
        self.arch.init_interrupts()
    }

    pub fn set_timer(&mut self, delay: usize) {
        self.arch.set_timer(delay);
    }
}
