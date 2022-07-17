pub type IrqLine = usize;

pub trait IrqManager {
    fn enable(&mut self, line: IrqLine, handler: fn());
    fn handle(&mut self, line: IrqLine);
}
