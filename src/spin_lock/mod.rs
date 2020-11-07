use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicU8, Ordering};

#[repr(u8)]
enum SpinLockState {
    FREE = 0,
    USED = 1,
}

pub struct SpinLock<T> {
    cell: UnsafeCell<T>,
    state: AtomicU8,
}

unsafe impl<T> Send for SpinLock<T> {}
unsafe impl<T> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
    pub fn new(data: T) -> Self {
        Self {
            cell: UnsafeCell::new(data),
            state: AtomicU8::new(SpinLockState::FREE as u8),
        }
    }

    fn lock(&mut self) {
        while self.state.swap(SpinLockState::USED as u8, Ordering::SeqCst)
            == SpinLockState::USED as u8
        {}
    }

    fn unlock(&mut self) {
        self.state
            .store(SpinLockState::FREE as u8, Ordering::SeqCst);
    }

    pub fn borrow<R>(&mut self, f: impl FnOnce(&mut T) -> R) -> R {
        self.lock();

        let ret = f(unsafe { self.cell.get().as_mut().unwrap() });

        self.unlock();

        ret
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utest::uassert_eq;

    #[test_case]
    fn spin_lock_test() {
        let val = 0;
        let mut lock = SpinLock::new(val);

        lock.borrow(|data| {
            *data = 1;
        });

        lock.borrow(|data| {
            kassert!(*data == 1, "SpinLock test");
        });
    }
}
