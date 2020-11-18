use crate::println;
use crate::utils;

use riscv;

const PLIC_ADDR: usize = 0xc000000;
const PLIC_ENABLE_OFFSET: usize = 0x002000;
const UART_IRQ_NB: usize = 10;

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
        riscv::register::sie::set_ssoft();
        riscv::register::sie::clear_utimer();
        riscv::register::sie::set_stimer();
        riscv::register::sie::clear_uext();
        riscv::register::sie::set_sext();
    }

    let test_addr = unsafe { stvec_base as usize };
    println!("Vector base addr: ${:x}", test_addr);
    unsafe {
        riscv::register::stvec::write(test_addr, riscv::register::stvec::TrapMode::Direct);
    }

    //TODO: Setup PLIC?
    // let plic_uart_priority = (PLIC_ADDR + 4 * UART_IRQ_NB) as *mut u32;
    // let plic_uart_enable = (PLIC_ADDR + PLIC_ENABLE_OFFSET + UART_IRQ_NB / 32) as *mut u32;
    // unsafe {
    //     *plic_uart_priority = 1;
    //     *plic_uart_enable = 1 << 10;
    // }

    set(InterruptsState::ENABLE);

    let v = 0x0 as *mut u64;
    unsafe {
        v.write_volatile(0);
    }
}

extern "C" {
    fn stvec_base();
}

global_asm!(
    "
    .align 4
    .global stvec_base
    stvec_base:
      call test
    "
);

#[naked] //Make sure we can safely jump directly to stvec_base
#[no_mangle]
pub extern "C" fn test() {
    println!("I got trapped!");
    loop {}
}
