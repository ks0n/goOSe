use super::mm;
use super::paging;
use crate::utils::init_once;
use super::irq;

#[derive(Debug)]
pub enum Error {
    DeviceNotFound(&'static str),
    NoMatchingDriver(&'static str),
    InvalidFdtNode,
    FdtError(fdt::FdtError),
    InitOnce(init_once::Error),
    Allocator(mm::AllocatorError),
    InvalidIrqLine(irq::IrqLine),
    IrqAlreadyEnabled(irq::IrqLine),
    UnexpectedIrq(irq::IrqLine),
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
