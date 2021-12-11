use crate::arch;

pub struct InterruptManager<T: arch::ArchitectureInterrupts> {
    arch: T,
}

impl<T: arch::ArchitectureInterrupts> InterruptManager<T> {
    pub fn new() -> Self {
        let arch = T::new();

        Self { arch }
    }

    pub fn init_interrupts(&self) {
        self.arch.init_interrupts()
    }
}
