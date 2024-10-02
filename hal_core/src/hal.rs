use super::mm::{self, Mmu, PageAlloc, PageMap};
use super::once_lock::OnceLock;
use super::AddressRange;
use super::Error;
use super::ReentrantSpinlock;
use super::{IrqOps, TimerCallbackFn};

pub struct Hal<P: PageMap + 'static, I: IrqOps> {
    kpt: OnceLock<ReentrantSpinlock<&'static mut P>>,
    irq_ops: I,
}

impl<P: PageMap + Mmu + 'static, I: IrqOps> Hal<P, I> {
    pub const fn new(irq_ops: I) -> Hal<P, I> {
        Self {
            kpt: OnceLock::new(),
            irq_ops,
        }
    }

    pub fn init_irqs(&'static self) {
        self.irq_ops.init();
    }

    pub fn init_irq_chip(&self, allocator: &impl PageAlloc) -> Result<(), Error> {
        self.irq_ops.init_irq_chip(allocator)
    }

    pub fn unmask_interrupts(&self) {
        self.irq_ops.unmask_interrupts();
    }

    pub fn set_timer_handler(&self, h: TimerCallbackFn) {
        log::trace!("Hal::set_timer_handler(0x{:x})", h as usize);
        self.irq_ops.set_timer_handler(h);
    }

    pub fn set_timer(&self, ticks: usize) -> Result<(), Error> {
        log::trace!("Hal::set_timer({})", ticks);
        self.irq_ops.set_timer(ticks)
    }

    pub fn clear_timer(&self) {
        log::trace!("Hal::clear_timer()");
        self.irq_ops.clear_timer();
    }

    pub fn init_kpt(
        &self,
        r: impl Iterator<Item = AddressRange>,
        rw: impl Iterator<Item = AddressRange>,
        rwx: impl Iterator<Item = AddressRange>,
        pre_allocated: impl Iterator<Item = AddressRange>,
        allocator: &impl PageAlloc,
    ) -> Result<(), Error> {
        if self
            .kpt
            .set(ReentrantSpinlock::new(crate::mm::prefill_pagetable::<P>(
                r,
                rw,
                rwx,
                pre_allocated,
                allocator,
            )?))
            .is_err()
        {
            panic!("kpt has already been set in the hal...");
        }

        log::debug!("hal::init_kpt finished");
        Ok(())
    }

    pub fn enable_paging(&self) -> Result<(), Error> {
        let kpt = self.kpt.get().unwrap().lock();

        P::mmu_on(*kpt);

        Ok(())
    }

    pub const fn page_size(&self) -> usize {
        P::PAGE_SIZE
    }

    pub const fn align_up(&self, val: usize) -> usize {
        mm::align_up(val, self.page_size())
    }

    pub const fn align_down(&self, val: usize) -> usize {
        mm::align_down(val, self.page_size())
    }

    pub fn kpt(&'static self) -> &ReentrantSpinlock<&'static mut P> {
        self.kpt.get().unwrap()
    }
}
