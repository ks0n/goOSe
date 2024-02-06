use core::arch::asm;

#[derive(Default)]
pub struct Registers {
    x: [usize; 32],
}

pub fn set_sstatus_sie() {
    unsafe {
        asm!("csrrs zero, sstatus, {}", in(reg)1 << 1);
    }
}

pub fn set_sie_ssie() {
    unsafe {
        asm!("csrrs zero, sie, {}", in(reg)1 << 1);
    }
}

pub fn set_sie_seie() {
    unsafe {
        asm!("csrrs zero, sie, {}", in(reg)1 << 9);
    }
}

pub fn set_sie_stie() {
    unsafe {
        asm!("csrrs zero, sie, {}", in(reg)1 << 5);
    }
}

pub fn set_stvec(addr: usize) {
    unsafe {
        asm!("csrw stvec, {}", in(reg)(addr));
    }
}
