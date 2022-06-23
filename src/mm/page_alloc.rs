use crate::arch;
use crate::mm::PhysicalMemoryManager;

use spin::Mutex;

#[derive(Debug)]
pub enum AllocatorError {
    OutOfMemory,
    InvalidFree,
}
