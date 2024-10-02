use core::arch::naked_asm;
use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};

use crate::cpu;
use hal_core::{Error, TimerCallbackFn};

use crate::devices::gicv2::GicV2;

use hal_core::mm::PageAlloc;
use hal_core::IrqOps;

use core::cell::OnceCell;

use cortex_a::registers::*;
use tock_registers::interfaces::{ReadWriteable, Writeable};

const PHYSICAL_TIMER_LINE: u32 = 30;

macro_rules! gen_isr_stub {
    () => {
        concat!(
            r#"
            .balign 0x80
            msr spsel, xzr
            stp x0, x1, [sp, #-16]!
            stp x2, x3, [sp, #-16]!
            stp x4, x5, [sp, #-16]!
            stp x6, x7, [sp, #-16]!
            stp x8, x9, [sp, #-16]!
            stp x10, x11, [sp, #-16]!
            stp x12, x13, [sp, #-16]!
            stp x14, x15, [sp, #-16]!
            stp x16, x17, [sp, #-16]!
            stp x18, x29, [sp, #-16]!
            stp x30, xzr, [sp, #-16]!

            mov x0, . - el1_vector_table
            bl aarch64_common_trap

            ldp x30, xzr, [sp], #16
            ldp x18, x29, [sp], #16
            ldp x16, x17, [sp], #16
            ldp x14, x15, [sp], #16
            ldp x12, x13, [sp], #16
            ldp x10, x11, [sp], #16
            ldp x8, x9, [sp], #16
            ldp x6, x7, [sp], #16
            ldp x4, x5, [sp], #16
            ldp x2, x3, [sp], #16
            ldp x0, x1, [sp], #16
            eret
            "#
        )
    };
}

#[naked]
#[no_mangle]
#[repr(align(0x800))]
unsafe extern "C" fn el1_vector_table() {
    naked_asm!(
        gen_isr_stub!(),
        gen_isr_stub!(),
        gen_isr_stub!(),
        gen_isr_stub!(),
        gen_isr_stub!(),
        gen_isr_stub!(),
        gen_isr_stub!(),
        gen_isr_stub!(),
        gen_isr_stub!(),
        gen_isr_stub!(),
        gen_isr_stub!(),
        gen_isr_stub!(),
    );
}

#[repr(u64)]
#[derive(Debug)]
enum InterruptType {
    // Current EL with SP0
    SyncCurrentElSp0,
    IrqCurrentElSp0,
    FiqCurrentElSp0,
    SerrorCurrentElSp0,
    // Current EL with SPx
    SyncCurrentElSpx,
    IrqCurrentElSpx,
    FiqCurrentElSpx,
    SerrorCurrentElSpx,
    // Lower EL
    SyncLowerEl,
    IrqLowerEl,
    FiqLowerEl,
    SerrorLowerEl,
    // Lower EL with aarch32
    SyncLowerElAarch32,
    IrqLowerElAarch32,
    FiqLowerElAarch32,
    SerrorLowerElAarch32,
}

static mut IRQS: core::cell::OnceCell<&Aarch64Irqs> = core::cell::OnceCell::new();

#[no_mangle]
unsafe extern "C" fn aarch64_common_trap(offset: u64) {
    log::debug!("aarch64_common_trap(0x{:x})", offset);

    let int_type = match offset {
        0x000..=0x07f => InterruptType::SyncCurrentElSp0,
        0x080..=0x0ff => InterruptType::IrqCurrentElSp0,
        0x100..=0x17f => InterruptType::FiqCurrentElSp0,
        0x180..=0x1ff => InterruptType::SerrorCurrentElSp0,
        0x200..=0x27f => InterruptType::SyncCurrentElSpx,
        0x280..=0x2ff => InterruptType::IrqCurrentElSpx,
        0x300..=0x37f => InterruptType::FiqCurrentElSpx,
        0x380..=0x3ff => InterruptType::SerrorCurrentElSpx,
        0x400..=0x47f => InterruptType::SyncLowerEl,
        0x480..=0x4ff => InterruptType::IrqLowerEl,
        0x500..=0x57f => InterruptType::FiqLowerEl,
        0x580..=0x5ff => InterruptType::SerrorLowerEl,
        0x600..=0x67f => InterruptType::SyncLowerElAarch32,
        0x680..=0x6ff => InterruptType::IrqLowerElAarch32,
        0x700..=0x77f => InterruptType::FiqLowerElAarch32,
        0x780..=0x7ff => InterruptType::SerrorLowerElAarch32,
        _ => unreachable!(),
    };

    IRQS.get()
        .expect("no one has init'ed the aarch64 hal yet...")
        .handler(int_type);
}

#[derive(Debug)]
pub struct Aarch64Irqs {
    irq_chip: OnceCell<GicV2>,
    timer_callback: AtomicPtr<TimerCallbackFn>,
}

/// Safety: I know what I'm doing :D
unsafe impl Sync for Aarch64Irqs {}

impl Aarch64Irqs {
    pub const fn new() -> Self {
        Self {
            irq_chip: OnceCell::new(),
            timer_callback: AtomicPtr::new(ptr::null_mut()),
        }
    }

    fn irq_chip(&self) -> &GicV2 {
        self.irq_chip.get().expect("something is trying to program the IRQ chip but `init_irq_chip` has not been called yet")
    }

    fn handler(&self, int: InterruptType) {
        match int {
            InterruptType::IrqCurrentElSp0 => {
                let int = self.irq_chip().get_int();

                match int {
                    Ok(PHYSICAL_TIMER_LINE) => {
                        // Clear the timer in order to EOI it.
                        self.clear_timer();

                        let timer_cb = self.timer_callback.load(Ordering::Relaxed);
                        if !timer_cb.is_null() {
                            unsafe {
                                // Cannot simply dereference TIMER_CALLBACK here.
                                // We are using an AtomicPtr and TIMER_CALLBACK already holds the fn().
                                #[allow(clippy::crosspointer_transmute)]
                                core::mem::transmute::<_, fn()>(timer_cb)();
                            }
                        }

                        self.irq_chip().clear_int(int.unwrap());
                    }
                    _ => panic!("got an irq but fuck knows"),
                }
            }
            _ => panic!("unhandled int {:?}", int),
        }
    }
}

impl IrqOps for Aarch64Irqs {
    fn init(&'static self) {
        cortex_a::registers::VBAR_EL1.set(el1_vector_table as usize as u64);
        unsafe {
            IRQS.set(self)
                .expect("looks like init has already been called")
        };
    }

    fn init_irq_chip(&self, _allocator: &impl PageAlloc) -> Result<(), Error> {
        let (gicd_base, gicc_base) = (0x800_0000, 0x801_0000);
        self.irq_chip
            .set(GicV2::new(gicd_base, gicc_base))
            .expect("init_irq_chip has already been called");
        Ok(())
    }

    fn unmask_interrupts(&self) {
        cpu::unmask_interrupts();
    }

    fn set_timer_handler(&self, h: TimerCallbackFn) {
        self.timer_callback.store(h as *mut _, Ordering::Relaxed);
    }

    fn set_timer(&self, ticks: usize) -> Result<(), Error> {
        self.irq_chip().enable_line(PHYSICAL_TIMER_LINE)?;
        super::cpu::set_physical_timer(ticks);
        super::cpu::unmask_interrupts();

        Ok(())
    }

    fn clear_timer(&self) {
        CNTP_CTL_EL0.modify(CNTP_CTL_EL0::ENABLE::CLEAR);
    }
}
