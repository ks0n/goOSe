use super::{Console, Driver};

#[derive(Debug)]
pub struct NullUart;

impl NullUart {
    pub const fn new() -> Self {
        Self
    }
}

impl Console for NullUart {
    fn write(&self, _data: &str) {
        // Does nothing, just a placeholder while a real uart is not in place.
    }
}

impl Driver for NullUart {
    fn get_address_range(&self) -> Option<(usize, usize)> {
        None
    }
}
