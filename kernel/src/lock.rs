use core::cell::UnsafeCell;

pub struct Lock<T: Sized> {
    data: UnsafeCell<T>,
}

impl<T> Lock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
        }
    }

    pub fn lock<'a, R>(&'a self, f: impl FnOnce(&'a mut T) -> R) -> R {
        // TODO: actually lock something...
        let data = unsafe { &mut *self.data.get() };

        f(data)
    }

    pub fn get(&self) -> &'static mut T {
        unsafe { &mut *self.data.get() }
    }
}

unsafe impl<T> Send for Lock<T> where T: Sized + Send {}
unsafe impl<T> Sync for Lock<T> where T: Sized + Send {}
