use crate::println;
use crate::utils;

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

fn set(status: InterruptsState) {
    match status {
        InterruptsState::DISABLE => unsafe { riscv::register::sstatus::clear_sie() },
        InterruptsState::ENABLE => unsafe { riscv::register::sstatus::set_sie() },
    }
}

pub fn init() {
    set(InterruptsState::DISABLE);

    // Enable only Supervisor-level external interrupts
    unsafe {
        riscv::register::sie::clear_usoft();
        riscv::register::sie::clear_ssoft();
        riscv::register::sie::clear_utimer();
        riscv::register::sie::clear_stimer();
        riscv::register::sie::clear_uext();
        riscv::register::sie::set_sext();
    }

    let test_addr = unsafe { utils::external_symbol_address(stvec_base) };
    println!("Vector base addr: ${:x}", test_addr);
    unsafe {
        riscv::register::stvec::write(test_addr, riscv::register::stvec::TrapMode::Direct);
    }

    //TODO: Setup PLIC?

    set(InterruptsState::ENABLE);
}

extern "Rust" {
    static stvec_base: ();
}

#[naked] //Make sure we can safely jump directly to stvec_base
pub extern "C" fn test() {
    unsafe {
        asm!(
            ".align 4
            .global stvec_base
            stvec_base:"
        );
    }
    println!("I've been trapped!");
}
