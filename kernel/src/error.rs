use super::mm;

use crate::utils::init_once;

#[derive(Debug)]
pub enum Error {
    DeviceNotFound(&'static str),
    NoMatchingDriver(&'static str),
    InvalidFdtNode,
    FdtError(fdt::FdtError),
    InitOnce(init_once::Error),
    Allocator(mm::AllocatorError),
    Hal(hal_core::Error),
    SetLoggerError(log::SetLoggerError),
}

impl From<init_once::Error> for Error {
    fn from(e: init_once::Error) -> Self {
        Self::InitOnce(e)
    }
}

impl From<fdt::FdtError> for Error {
    fn from(e: fdt::FdtError) -> Self {
        Self::FdtError(e)
    }
}

impl From<mm::AllocatorError> for Error {
    fn from(e: mm::AllocatorError) -> Self {
        Self::Allocator(e)
    }
}

impl From<hal_core::Error> for Error {
    fn from(e: hal_core::Error) -> Self {
        Self::Hal(e)
    }
}

impl From<log::SetLoggerError> for Error {
    fn from(e: log::SetLoggerError) -> Self {
        Self::SetLoggerError(e)
    }
}
