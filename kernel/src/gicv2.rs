use super::irq;

pub struct GicV2 {
    handlers: [Option<fn()>; 1022],

    gic: drivers::gicv2::GicV2,
}

impl GicV2 {
    pub fn new(gicd_base: usize, gicc_base: usize) -> Self {
        Self {
            handlers: [None; drivers::gicv2::GicV2::max_interrupts()],
            gic: drivers::gicv2::GicV2::new(gicd_base, gicc_base),
        }
    }

    pub fn enable_interrupts(&mut self) {
        self.gic.enable_interrupts();
    }
}

impl irq::IrqManager for GicV2 {
    fn enable(&mut self, line: irq::IrqLine, handler: fn()) {
        if line >= self.handlers.len() {
            return;
        }

        self.handlers[line] = Some(handler);
        self.gic.enable(line);
    }

    fn handle(&mut self, line: irq::IrqLine) {
        if line >= self.handlers.len() {
            return;
        }

        if let Some(handler) = self.handlers[line] {
            handler();
        }
    }
}
