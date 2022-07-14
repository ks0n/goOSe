use crate::arch::ArchitectureInterrupts;

use core::marker::PhantomData;

pub struct InterruptManager<I> {
    arch_interrupts: I,
}

impl<I: ArchitectureInterrupts> InterruptManager<I> {
    pub fn new(arch_interrupts: I) -> Self {
        Self { arch_interrupts }
    }

    pub fn init_interrupts(&mut self) {
        self.arch_interrupts.init_interrupts();
    }

    pub fn set_timer(&mut self, delay: usize) {
        self.arch_interrupts.set_timer(delay);
    }
}

// TODO: Test set_timer
