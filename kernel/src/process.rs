use crate::hal::{self, Registers};

use crate::mm::alloc_pages_user;

use log::{debug, info, trace};

pub struct Process {
    registers: Registers,
    stack: &'static mut [u8],
}

impl Process {
    pub fn new(entry: usize, stack_size: usize) -> Self {
        let stack = alloc_pages_user(stack_size).unwrap();
        let mut registers = Registers::default();
        registers.pc = entry;

        Process { registers, stack }
    }
}

pub fn run_in_userland(process: &mut Process) {
    debug!("run_in_userland {}", process.stack.len());
    hal::context::switch_userland(process.registers.pc, unsafe { process.stack.as_mut_ptr().byte_add(process.stack.len()) });
}

pub fn run(process: &mut Process) {
    hal::context::switch(process.registers.pc);
}
