use riscv;

#[derive(Debug)]
pub enum InterruptsState {
    DISABLE = 0,
    ENABLE = 1,
}

pub fn state() -> InterruptsState {
    match riscv::register::sstatus::read().sie() {
        false => InterruptsState::DISABLE,
        true => InterruptsState::ENABLE,
    }
}

pub fn set(status: InterruptsState) {
    match status {
        InterruptsState::DISABLE => unsafe { riscv::register::sstatus::clear_sie() },
        InterruptsState::ENABLE => unsafe { riscv::register::sstatus::set_sie() },
    }
}
