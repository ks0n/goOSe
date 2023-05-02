use super::drivers::Driver;
use super::Error;

pub type IrqLine = usize;

pub enum Interrupt {
    PhysicalTimer,
}

pub trait IrqChip: Driver {
    fn enable(&self, int: Interrupt) -> Result<(), Error>;
    fn get_int(&self) -> Result<Interrupt, Error>;
    fn clear_int(&self, int: Interrupt);
}
