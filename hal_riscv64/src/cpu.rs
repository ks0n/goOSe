use super::registers;

pub fn unmask_interrupts() {
    registers::set_sstatus_sie();
    registers::set_sie_ssie();
    registers::set_sie_seie();
    registers::set_sie_stie();
}

pub fn clear_physical_timer() {
    sbi::timer::set_timer(u64::MAX).unwrap();
}
