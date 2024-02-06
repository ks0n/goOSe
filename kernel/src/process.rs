use hal::Registers;

pub struct Process {
    registers: Registers,
}

impl Process {
    pub fn new(entry: usize, stack: usize) -> Self {
        Process { 
            register: Registers::default(),
        }
    }
}
    // entry: extern "C" fn() -> u8, stack: *mut u8
