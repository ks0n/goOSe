#[derive(Debug)]
pub enum AllocatorError {
    OutOfMemory,
    InvalidFree,
}
