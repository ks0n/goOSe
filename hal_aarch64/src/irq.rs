use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};

use crate::cpu;
use hal_core::{Error, TimerCallbackFn};

use crate::devices::gicv2::GicV2;

use crate::mm;
use hal_core::mm::{PageAlloc, PageMap, Permissions, VAddr};

use tock_registers::interfaces::Writeable;

const PHYSICAL_TIMER_LINE: u32 = 30;

pub unsafe fn init_el1_exception_handlers() {
    extern "Rust" {
        static el1_vector_table: core::cell::UnsafeCell<()>;
    }
    cortex_a::registers::VBAR_EL1.set(el1_vector_table.get() as u64);
}

static TIMER_CALLBACK: AtomicPtr<TimerCallbackFn> = AtomicPtr::new(ptr::null_mut());

pub fn set_timer_handler(h: TimerCallbackFn) {
    TIMER_CALLBACK.store(h as *mut _, Ordering::Relaxed);
}

pub fn set_timer(ticks: usize) -> Result<(), Error> {
    enable_line(PHYSICAL_TIMER_LINE)?;
    super::cpu::set_physical_timer(ticks);
    super::cpu::unmask_interrupts();

    Ok(())
}

enum IrqChip {
    NoChip,
    GicV2(GicV2),
}

impl IrqChip {
    fn get_int(&mut self) -> Result<u32, Error> {
        match self {
            Self::NoChip => unreachable!("does not support this"),
            Self::GicV2(gic) => gic.get_int(),
        }
    }

    fn clear_int(&mut self, int: u32) {
        match self {
            Self::NoChip => unreachable!("does not support this"),
            Self::GicV2(gic) => gic.clear_int(int),
        }
    }

    fn enable_int(&mut self, int: u32) -> Result<(), Error> {
        match self {
            Self::NoChip => unreachable!("does not support"),
            Self::GicV2(gic) => gic.enable_line(int),
        }
    }
}

static mut IRQ_CHIP: IrqChip = IrqChip::NoChip;

pub fn init_irq_chip(_dt_node: (), allocator: &mut impl PageAlloc) -> Result<(), Error> {
    let (gicd_base, gicc_base) = (0x800_0000, 0x801_0000);
    mm::current().identity_map_range(
        VAddr::new(gicd_base),
        0x0001_0000 / mm::PAGE_SIZE,
        Permissions::READ | Permissions::WRITE,
        allocator,
    )?;
    mm::current().identity_map_range(
        VAddr::new(gicc_base),
        0x0001_0000 / mm::PAGE_SIZE,
        Permissions::READ | Permissions::WRITE,
        allocator,
    )?;

    unsafe {
        IRQ_CHIP = IrqChip::GicV2(GicV2::new(gicd_base, gicc_base));
    }
    Ok(())
}

fn enable_line(line: u32) -> Result<(), Error> {
    unsafe { IRQ_CHIP.enable_int(line) }
}

#[no_mangle]
extern "C" fn sync_current_el_sp0() {
    panic!("hit sync_current_el_sp0");
}

#[no_mangle]
extern "C" fn irq_current_el_sp0() {
    let int = unsafe { IRQ_CHIP.get_int() };

    match int {
        Ok(PHYSICAL_TIMER_LINE) => {
            // Clear the timer in order to EOI it.
            cpu::clear_physical_timer();

            let timer_ptr = TIMER_CALLBACK.load(Ordering::Relaxed);
            if !timer_ptr.is_null() {
                unsafe {
                    let timer: fn() = core::mem::transmute::<_, fn()>(timer_ptr);
                    timer();
                }
            }

            unsafe { IRQ_CHIP.clear_int(int.unwrap()) };
        }
        _ => panic!("got an irq but fuck knows"),
    }
}
#[no_mangle]
extern "C" fn fiq_current_el_sp0() {
    panic!("hit fiq_current_el_sp0");
}

#[no_mangle]
extern "C" fn serror_current_el_sp0() {
    panic!("hit serror_current_el_sp0");
}

#[no_mangle]
extern "C" fn sync_current_el_spx() {
    panic!("hit sync_current_el_spx");
}

#[no_mangle]
extern "C" fn irq_current_el_spx() {
    panic!("hit irq_current_el_spx");
}

#[no_mangle]
extern "C" fn fiq_current_el_spx() {
    panic!("hit fiq_current_el_spx");
}

#[no_mangle]
extern "C" fn serror_current_el_spx() {
    panic!("hit serror_current_el_spx");
}

#[no_mangle]
extern "C" fn sync_lower_el() {
    panic!("hit sync_lower_el");
}

#[no_mangle]
extern "C" fn irq_lower_el() {
    panic!("hit irq_lower_el");
}

#[no_mangle]
extern "C" fn fiq_lower_el() {
    panic!("hit fiq_lower_el");
}

#[no_mangle]
extern "C" fn serror_lower_el() {
    panic!("hit serror_lower_el");
}

#[no_mangle]
extern "C" fn sync_lower_el_aarch32() {
    panic!("hit sync_lower_el_aarch32");
}

#[no_mangle]
extern "C" fn irq_lower_el_aarch32() {
    panic!("hit irq_lower_el_aarch32");
}

#[no_mangle]
extern "C" fn fiq_lower_el_aarch32() {
    panic!("hit fiq_lower_el_aarch32");
}

#[no_mangle]
extern "C" fn serror_lower_el_aarch32() {
    panic!("hit serror_lower_el_aarch32");
}

core::arch::global_asm!(include_str!("exceptions.S"));
