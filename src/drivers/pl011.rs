pub struct Pl011 {
    base: usize,
}

fn tamer() {}

impl Pl011 {
    pub fn new(base: usize) -> Self {
        Self { base }
    }

    pub fn putc(&mut self, b: u8) {
        let thr = self.base as *mut u64;
        unsafe { thr.write_volatile(b.into()) }
    }
}

impl crate::kernel_console::Console for Pl011 {
    fn write(&mut self, data: &str) {
        data
            .bytes()
            .for_each(|b| self.putc(b))
    }
}
