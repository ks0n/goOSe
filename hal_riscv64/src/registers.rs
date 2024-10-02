use core::arch::asm;

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

pub fn set_sscratch(val: usize) {
    unsafe {
        asm!("csrw sscratch, {}", in(reg)(val));
    }
}

pub fn get_sscratch() -> usize {
    let mut val: usize;

    unsafe {
        asm!("csrr {}, sscratch", out(reg)(val));
    }

    val
}
