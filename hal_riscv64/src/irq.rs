use hal_core::{
    mm::{PageAlloc, PageMap, Permissions, VAddr},
    Error, TimerCallbackFn,
};

use super::mm;
use super::plic::Plic;
use super::registers;

use core::arch::asm;
use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};

use riscv;
use sbi;

pub fn init_exception_handlers() {
    registers::set_stvec(trap_handler as usize);
}

static mut IRQ_CHIP: Option<Plic> = None;

pub fn init_irq_chip(_dt_node: (), allocator: &impl PageAlloc) -> Result<(), Error> {
    // TODO map the dt_node
    let base = 0xc000000;
    let max_offset = 0x3FFFFFC;

    mm::current().identity_map_range(
        VAddr::new(base),
        max_offset / mm::PAGE_SIZE + 1,
        Permissions::READ | Permissions::WRITE,
        allocator,
    )?;
    unsafe {
        IRQ_CHIP = Some(Plic::new(base));
    }

    Ok(())
}

static TIMER_CALLBACK: AtomicPtr<TimerCallbackFn> = AtomicPtr::new(ptr::null_mut());

pub fn set_timer_handler(h: TimerCallbackFn) {
    TIMER_CALLBACK.store(h as *mut _, Ordering::Relaxed);
}

pub fn set_timer(ticks: usize) -> Result<(), Error> {
    let target_time = riscv::register::time::read() + ticks;
    sbi::timer::set_timer(target_time as u64).unwrap();

    Ok(())
}

#[derive(Debug, Copy, Clone)]
enum InterruptType {
    Reserved,
    SupervisorSoftware,
    SupervisorTimer,
    SupervisorExternal,
    Platform(u64),
}

impl InterruptType {
    fn is_asynchronous(&self) -> bool {
        matches!(self, Self::SupervisorTimer)
    }
}

impl From<u64> for InterruptType {
    fn from(code: u64) -> Self {
        match code {
            0 | 2..=4 | 6..=8 | 10..=15 => Self::Reserved,
            1 => Self::SupervisorSoftware,
            5 => Self::SupervisorTimer,
            9 => Self::SupervisorExternal,
            _ => Self::Platform(code),
        }
    }
}

impl From<InterruptType> for u64 {
    fn from(itype: InterruptType) -> Self {
        match itype {
            InterruptType::Reserved => {
                unreachable!()
            }
            InterruptType::Platform(code) => code,
            InterruptType::SupervisorSoftware => 1,
            InterruptType::SupervisorTimer => 5,
            InterruptType::SupervisorExternal => 9,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum ExceptionType {
    Reserved,
    Custom(u64),
    InstructionAddressMisaligned,
    InstructionAccessFault,
    IllegalInstruction,
    Breakpoint,
    LoadAddressMisaligned,
    LoadAccessFault,
    StoreAMOAddressMisaligned,
    StoreAMOAccessFault,
    EnvironmentCallUMode,
    EnvironmentCallSMode,
    InstructionPageFault,
    LoadPageFault,
    StoreAMOPageFault,
}

impl From<u64> for ExceptionType {
    fn from(code: u64) -> Self {
        match code {
            10..=11 | 14 | 16..=23 | 32..=47 => Self::Reserved,
            c if c >= 64 => Self::Reserved,
            24..=31 | 48..=63 => Self::Custom(code),
            0 => Self::InstructionAddressMisaligned,
            1 => Self::InstructionAccessFault,
            2 => Self::IllegalInstruction,
            3 => Self::Breakpoint,
            4 => Self::LoadAddressMisaligned,
            5 => Self::LoadAccessFault,
            6 => Self::StoreAMOAddressMisaligned,
            7 => Self::StoreAMOAccessFault,
            8 => Self::EnvironmentCallUMode,
            9 => Self::EnvironmentCallSMode,
            12 => Self::InstructionPageFault,
            13 => Self::LoadPageFault,
            15 => Self::StoreAMOPageFault,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone)]
enum TrapType {
    Interrupt(InterruptType),
    Exception(ExceptionType),
}

impl TrapType {
    fn is_interrupt(cause: u64) -> bool {
        (cause >> 63) == 1
    }
}

impl From<u64> for TrapType {
    fn from(cause: u64) -> Self {
        let exception_code = cause & !(1 << 63);

        if Self::is_interrupt(cause) {
            Self::Interrupt(InterruptType::from(exception_code))
        } else {
            Self::Exception(ExceptionType::from(exception_code))
        }
    }
}

static mut INTERRUPT_VECTOR: &[extern "C" fn()] = &[
    undefined_handler,
    undefined_handler,
    undefined_handler,
    undefined_handler,
    undefined_handler,
    timer_handler,
    undefined_handler,
    undefined_handler,
    undefined_handler,
    supervisor_external_interrupt_handler,
];

/// Dispatch interrupts and exceptions
/// Returns 0 if it was synchronous, 1 otherwise
#[no_mangle]
extern "C" fn trap_dispatch(cause: u64) -> u64 {
    match TrapType::from(cause) {
        TrapType::Interrupt(itype) => {
            let exception_code: u64 = itype.into();
            unsafe { INTERRUPT_VECTOR[exception_code as usize]() };

            if itype.is_asynchronous() {
                1
            } else {
                0
            }
        }
        TrapType::Exception(etype) => {
            panic!("Exception '{:?}' not implemented yet", etype)
        }
    }
}

extern "C" fn supervisor_external_interrupt_handler() {
    todo!("fwd the external int to the irq_chip or smthing...");
}

extern "C" fn undefined_handler() {
    panic!("Interruption is not handled yet");
}

extern "C" fn timer_handler() {
    let timer_cb = TIMER_CALLBACK.load(Ordering::Relaxed);
    if !timer_cb.is_null() {
        unsafe {
            core::mem::transmute::<_, fn()>(timer_cb)();
        }
    }
}

#[naked]
#[no_mangle]
#[repr(align(4))]
unsafe extern "C" fn trap_handler() {
    asm!(
        "
        addi sp, sp, -0x100

        sd x31, 0x100(sp)
        sd x30, 0xf8(sp)
        sd x29, 0xf0(sp)
        sd x28, 0xd8(sp)
        sd x27, 0xd0(sp)
        sd x26, 0xc8(sp)
        sd x25, 0xc0(sp)
        sd x24, 0xb8(sp)
        sd x23, 0xb0(sp)
        sd x22, 0xa8(sp)
        sd x21, 0xa0(sp)
        sd x20, 0x98(sp)
        sd x19, 0x90(sp)
        sd x18, 0x88(sp)
        sd x17, 0x80(sp)
        sd x16, 0x78(sp)
        sd x15, 0x70(sp)
        sd x14, 0x68(sp)
        sd x13, 0x60(sp)
        sd x12, 0x58(sp)
        sd x11, 0x50(sp)
        sd x10, 0x48(sp)
        sd x9, 0x40(sp)
        sd x8, 0x38(sp)
        sd x7, 0x30(sp)
        sd x6, 0x28(sp)
        sd x5, 0x20(sp)
        sd x4, 0x18(sp)
        sd x3, 0x10(sp)
        sd x2, 0x8(sp)
        sd x1, 0x0(sp)

        // mv a0, sp // Pointer on stack for the register struct
        // csrr a1, sepc
        // csrr a2, stval
        // csrr a3, scause
        // csrr a5, sstatus

        csrr a0, scause
        jal trap_dispatch

        bne a0, x0, 1f

        csrr t0, sepc
        addi t0, t0, 4
        csrw sepc, t0

1:
        ld x1, 0x0(sp)
        ld x2, 0x8(sp)
        ld x3, 0x10(sp)
        ld x4, 0x18(sp)
        ld x5, 0x20(sp)
        ld x6, 0x28(sp)
        ld x7, 0x30(sp)
        ld x8, 0x38(sp)
        ld x9, 0x40(sp)
        ld x10, 0x48(sp)
        ld x11, 0x50(sp)
        ld x12, 0x58(sp)
        ld x13, 0x60(sp)
        ld x14, 0x68(sp)
        ld x15, 0x70(sp)
        ld x16, 0x78(sp)
        ld x17, 0x80(sp)
        ld x18, 0x88(sp)
        ld x19, 0x90(sp)
        ld x20, 0x98(sp)
        ld x21, 0xa0(sp)
        ld x22, 0xa8(sp)
        ld x23, 0xb0(sp)
        ld x24, 0xb8(sp)
        ld x25, 0xc0(sp)
        ld x26, 0xc8(sp)
        ld x27, 0xd0(sp)
        ld x28, 0xd8(sp)
        ld x29, 0xf0(sp)
        ld x30, 0xf8(sp)
        ld x31, 0x100(sp)

        addi sp, sp, 0x100

        sret",
        options(noreturn)
    );
    // Obviously this isn't done, we need to jump back to the previous context before the
    // interrupt using mpp/spp and mepc/sepc.
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::kernel_tests::*;

    #[test_case]
    fn arch_timer(ctx: &mut TestContext) {
        static mut TRAPTYPE: Option<TrapType> = None;

        extern "C" fn test_trap_handler(cause: u64) -> u64 {
            unsafe { TRAPTYPE = Some(TrapType::from(cause)) };

            // "Disable" timer
            sbi::timer::set_timer(u64::MAX).unwrap();
            1
        }

        ctx.arch_interrupts.init_interrupts();
        ctx.arch_interrupts
            .set_higher_trap_handler(test_trap_handler);

        ctx.arch_interrupts.set_timer(10000);

        // Wait for the some time for the timer interrupt to arrive
        for i in 0..1000000 {
            // This is just to avoid the whole loop optimized out
            core::hint::black_box(i);

            if let Some(ttype) = unsafe { TRAPTYPE } {
                if matches!(ttype, TrapType::Interrupt(InterruptType::SupervisorTimer)) {
                    return;
                }

                // There was an interrupt but it was not the timer
                assert!(false);
            }
        }

        // The interrupt was never tirggered
        assert!(false)
    }
}
