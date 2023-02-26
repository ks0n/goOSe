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

pub fn generic_timer_irq() -> Result<(), Error> {
    // TODO: this is where all calls from a timer irq will land, I guess this where well will
    // schedule new tasks etc...
    crate::kprintln!("timer irq");

    Ok(())
}
