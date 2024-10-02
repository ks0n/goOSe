#![no_std]
#![feature(const_mut_refs)]
#![feature(never_type)]

use core::convert::Into;
use core::ops::Range;

mod hal;
pub use hal::Hal;

pub mod once_lock;

mod reentrant_spinlock;
pub use reentrant_spinlock::ReentrantSpinlock;

pub trait IrqOps {
    fn init(&'static self);
    fn init_irq_chip(&self, allocator: &impl mm::PageAlloc) -> Result<(), Error>;
    fn unmask_interrupts(&self);
    fn set_timer_handler(&self, h: TimerCallbackFn);
    fn set_timer(&self, ticks: usize) -> Result<(), Error>;
    fn clear_timer(&self);
}

pub mod mm;

#[derive(Debug)]
pub enum Error {
    Alloc(mm::AllocatorError),
}

impl From<mm::AllocatorError> for Error {
    fn from(e: mm::AllocatorError) -> Self {
        Self::Alloc(e)
    }
}

pub type TimerCallbackFn = fn();

/// A range similar to core::ops::Range but that is copyable.
/// The range is half-open, inclusive below, exclusive above, ie. [start; end[
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AddressRange {
    pub start: usize,
    pub end: usize,
}

impl AddressRange {
    pub fn new<T: Into<usize>>(range: Range<T>) -> Self {
        let (start, end) = (range.start.into(), range.end.into());
        // assert!(range.start % page_size == 0);
        // assert_eq!(range.end, mm::align_up(range.end, page_size));

        assert!(start < end);

        Self { start, end }
    }

    pub fn with_size(start: usize, size: usize) -> Self {
        Self::new(start..start + size)
    }

    pub fn round_up_to_page(self, page_size: usize) -> Self {
        Self {
            start: self.start,
            end: mm::align_up(self.end, page_size),
        }
    }

    pub fn iter_pages(self, page_size: usize) -> impl Iterator<Item = usize> {
        assert_eq!(self.end, mm::align_up(self.end, page_size));

        (self.start..self.end).step_by(page_size)
    }

    pub fn count_pages(&self, page_size: usize) -> usize {
        mm::align_up(self.size(), page_size) / page_size
    }

    pub fn contains(&self, val: usize) -> bool {
        self.start <= val && val < self.end
    }

    pub fn size(&self) -> usize {
        self.end - self.start
    }
}
