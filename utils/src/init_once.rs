use core::cell::UnsafeCell;
use core::sync::atomic::{self, AtomicBool};

pub struct InitOnce<T: Sized> {
    locked: AtomicBool,
    val: UnsafeCell<Option<T>>,
}

impl<T: Sync> InitOnce<T> {
    pub const fn new() -> Self {
        Self {
            locked: AtomicBool::new(false),
            val: UnsafeCell::new(None),
        }
    }

    pub fn set(&self, val: T) -> Result<(), &'static str> {
        if self.is_initialized() {
            return Err("the InitOnce is already initialized");
        }

        while self
            .locked
            .compare_exchange(
                false,
                true,
                atomic::Ordering::Acquire,
                atomic::Ordering::Relaxed,
            )
            .is_err()
        {}

        // Safety: We hold exclusive access to the UnsafeCell.
        unsafe {
            *self.val.get() = Some(val);
        }

        self.locked.store(false, atomic::Ordering::Release);

        Ok(())
    }

    pub fn is_initialized(&self) -> bool {
        while self
            .locked
            .compare_exchange(
                false,
                true,
                atomic::Ordering::Acquire,
                atomic::Ordering::Relaxed,
            )
            .is_err()
        {}

        // Safety: We hold exclusive access to the UnsafeCell.
        let initialized: bool = unsafe { &*self.val.get() }.is_some();

        self.locked.store(false, atomic::Ordering::Release);

        initialized
    }

    pub fn get<'a>(&'a self) -> Option<&'a T> {
        if !self.is_initialized() {
            return None;
        }

        // Safety: We know it is initialized some the UnsafeCell MUST contain an Option that
        // contains a value.
        // Since that value is Sync, we can safely pass references to it.
        let val: Option<&T> = unsafe { &*self.val.get() }.as_ref();
        val
    }
}

unsafe impl<T: Sync> Sync for InitOnce<T> {}
