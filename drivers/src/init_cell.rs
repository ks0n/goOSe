use core::cell::UnsafeCell;

pub struct InitCell<T: Sized> {
    data: UnsafeCell<T>,
}

impl<T> InitCell<T> {
    pub const fn new(data: T) -> Self {
        Self { data: UnsafeCell::new(data) }
    }

    pub fn set<'a>(&'a self, f: impl FnOnce(&'a mut T)) {
        let data = unsafe { &mut *self.data.get() };
        f(data)
    }

    pub fn get<'a>(&'a self) -> &'a T {
        unsafe { &*self.data.get() }
    }
}

unsafe impl<T: Send> Send for InitCell<T> {}
unsafe impl<T: Send> Sync for InitCell<T> {}
