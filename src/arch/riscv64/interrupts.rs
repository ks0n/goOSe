/// Enum for interrupts states
#[derive(Debug)]
pub enum InterruptsState {
    /// No interrupt can be triggered
    DISABLE = 0,
    /// Supervisor interrupts are enabled
    ENABLE = 1,
}

/// Retrieve current state of supervisor interrupts
pub fn state() -> InterruptsState {
    match riscv::register::sstatus::read().sie() {
        false => InterruptsState::DISABLE,
        true => InterruptsState::ENABLE,
    }
}

fn set(status: InterruptsState) {
    match status {
        InterruptsState::DISABLE => unsafe { riscv::register::sstatus::clear_sie() },
        InterruptsState::ENABLE => unsafe { riscv::register::sstatus::set_sie() },
    }
}

/// Initialize supervisor interrupts
pub fn init() {
    set(InterruptsState::DISABLE);

    // At the moment, no need to activate any interrupts
    unsafe {
        riscv::register::sie::clear_usoft();
        riscv::register::sie::clear_ssoft();
        riscv::register::sie::clear_utimer();
        riscv::register::sie::clear_stimer();
        riscv::register::sie::clear_uext();
        riscv::register::sie::clear_sext();
    }

    set(InterruptsState::ENABLE);
}

/// Handle interrupt
pub fn handle(id: usize) {
    panic!("Interrupt {} not handled yet!", id);
}
