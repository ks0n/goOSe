use crate::println;
use crate::utils;
use core::mem::size_of;

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

    // let v = 0x0 as *mut u64;
    // unsafe {
    //     v.write_volatile(0);
    // }
    //
    // println!("We can't be here");
}

extern "C" {
    fn stvec_base();
}

#[naked]
#[no_mangle]
/// This function will never get called. It's only a wrapper around stvec_base which is aligned to
/// 4 as needed for the base field in stvec register
pub extern "C" fn stvec_base_wrapper() {
    unsafe {
        // As rust does not provide a way to have align a function
        // [yet](https://github.com/rust-lang/rust/issues/75072), we do it here and use
        // stvec_base address in stvec register
        asm!(
            "
            .align 4
            .global stvec_base
            stvec_base:

            csrw sscratch, sp

            // Save registers
            addi sp, sp, -248
            sd x1, 0(sp)
            // push original sp
            csrr x1, sscratch
            sd x1, 8(sp)
            // restore x1's value
            ld x1, 0(sp)
            sd x3, 16(sp)
            sd x4, 24(sp)
            sd x5, 32(sp)
            sd x6, 40(sp)
            sd x7, 48(sp)
            sd x8, 56(sp)
            sd x9, 64(sp)
            sd x10, 72(sp)
            sd x11, 80(sp)
            sd x12, 88(sp)
            sd x13, 96(sp)
            sd x14, 104(sp)
            sd x15, 112(sp)
            sd x16, 120(sp)
            sd x17, 128(sp)
            sd x18, 136(sp)
            sd x19, 144(sp)
            sd x20, 152(sp)
            sd x21, 160(sp)
            sd x22, 168(sp)
            sd x23, 176(sp)
            sd x24, 184(sp)
            sd x25, 192(sp)
            sd x26, 200(sp)
            sd x27, 208(sp)
            sd x28, 216(sp)
            sd x29, 224(sp)
            sd x30, 232(sp)
            sd x31, 240(sp)

            csrr a1, scause
            csrr a2, stval
            csrr a3, sepc
            call trap_handler

            // Restore registers
            ld x1, 0(sp)
            // Skip x2 as its the stack pointer
            ld x3, 16(sp)
            ld x4, 24(sp)
            ld x5, 32(sp)
            ld x6, 40(sp)
            ld x7, 48(sp)
            ld x8, 56(sp)
            ld x9, 64(sp)
            ld x10, 72(sp)
            ld x11, 80(sp)
            ld x12, 88(sp)
            ld x13, 96(sp)
            ld x14, 104(sp)
            ld x15, 112(sp)
            ld x16, 120(sp)
            ld x17, 128(sp)
            ld x18, 136(sp)
            ld x19, 144(sp)
            ld x20, 152(sp)
            ld x21, 160(sp)
            ld x22, 168(sp)
            ld x23, 176(sp)
            ld x24, 184(sp)
            ld x25, 192(sp)
            ld x26, 200(sp)
            ld x27, 208(sp)
            ld x28, 216(sp)
            ld x29, 224(sp)
            ld x30, 232(sp)
            ld x31, 240(sp)

            // Resotre user stack
            csrr sp, sscratch

            sret
            "
        );
    }

    panic!("Return from trap failed!");
}

/// Act as a multiplexer, routing traps to their respective handler
#[no_mangle]
extern "C" fn trap_handler(scause: usize, sval: usize, sepc: usize) {
    let is_interrupt = (scause >> (size_of::<usize>() - 1)) == 1;

    match is_interrupt {
        true => panic!("Interrupt {} not handled yet!", sval),
        false => match sval {
            2 => {
                panic!("Illegal instruction at {:#x}!", sepc)
            }
            _ => panic!(
                "Exception {} not handled yet!. Occured at {:#x}",
                sval, sepc
            ),
        },
    }
}
