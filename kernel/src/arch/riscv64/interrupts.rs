use crate::arch::riscv64::registers;
use crate::arch::ArchitectureInterrupts;
use crate::kprintln;
use drivers::plic::plic_handler;

use core::arch::asm;

use riscv;
use sbi;

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

#[repr(C)]
pub struct TrapReturnValues {
    pub need_pc_increment: u64,
    pub abort_to_kernel: u64,
}

#[no_mangle]
static mut g_higher_trap_handler: *const () = 0 as *const ();

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
    plic_handler,
];

pub struct Interrupts {}

impl Interrupts {
    pub fn set_higher_trap_handler(
        &mut self,
        higher_trap_handler: extern "C" fn(cause: u64, syscall_number: usize) -> TrapReturnValues,
    ) {
        unsafe {
            g_higher_trap_handler = higher_trap_handler as *const ();
        }
    }
}

impl ArchitectureInterrupts for Interrupts {
    fn new() -> Self {
        Self {}
    }

    fn init_interrupts(&mut self) {
        extern "Rust" {
            static trap_handler: core::cell::UnsafeCell<()>;
        }

        // Set the trap handler
        self.set_higher_trap_handler(trap_dispatch);
        unsafe {
            registers::set_stvec(trap_handler.get() as usize);
        }

        // Then enable the interrupts sources
        registers::set_sstatus_sie();
        registers::set_sie_ssie();
        registers::set_sie_seie();
        registers::set_sie_stie();
    }

    fn set_timer(&mut self, delay: usize) {
        let target_time = riscv::register::time::read() + delay;
        sbi::timer::set_timer(target_time as u64).unwrap();
    }
}

/// Dispatch interrupts and exceptions
/// Returns 0 if it was synchronous, 1 otherwise
extern "C" fn trap_dispatch(cause: u64, _syscall_number: usize) -> TrapReturnValues {
    match TrapType::from(cause) {
        TrapType::Interrupt(itype) => {
            kprintln!("Interrupt '{:#?}' triggered", itype);

            let exception_code: u64 = itype.into();
            unsafe { INTERRUPT_VECTOR[exception_code as usize]() };

            if itype.is_asynchronous() {
                TrapReturnValues {
                    need_pc_increment: 1,
                    abort_to_kernel: 0,
                }
            } else {
                TrapReturnValues {
                    need_pc_increment: 0,
                    abort_to_kernel: 0,
                }
            }
        }
        TrapType::Exception(etype) => {
            panic!("Exception '{:?}' not implemented yet", etype)
        }
    }
}

extern "C" fn undefined_handler() {
    panic!("Interruption is not handled yet");
}

extern "C" fn timer_handler() {
    kprintln!("Timer!!");
    sbi::timer::set_timer(u64::MAX).unwrap();
}

core::arch::global_asm!(include_str!("interrupt.S"));

#[cfg(test)]
mod test {
    use super::*;
    use crate::kernel_tests::*;

    #[test_case]
    fn arch_timer(ctx: &mut TestContext) {
        static mut TRAPTYPE: Option<TrapType> = None;

        extern "C" fn test_trap_handler(cause: u64, _syscall_number: usize) -> TrapReturnValues {
            unsafe { TRAPTYPE = Some(TrapType::from(cause)) };

            // "Disable" timer
            sbi::timer::set_timer(u64::MAX).unwrap();
            TrapReturnValues {
                need_pc_increment: 1,
                abort_to_kernel: 0,
            }
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
