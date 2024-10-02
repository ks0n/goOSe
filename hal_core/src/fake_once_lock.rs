use core::cell::OnceCell;

pub struct FakeOnceLock<T> {
    cell: OnceCell<T>,
}

impl<T> FakeOnceLock<T> {
    pub const fn new() -> FakeOnceLock<T> {
        Self {
            cell: OnceCell::new(),
        }
    }

    pub fn get(&self) -> Option<&T> {
        self.cell.get()
    }

    pub fn set(&self, value: T) -> Result<(), T> {
        self.cell.set(value)
    }
}

/// Safety: it is not safe...
unsafe impl<T> Sync for FakeOnceLock<T> {}
unsafe impl<T> Send for FakeOnceLock<T> {}
