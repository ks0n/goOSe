use core::arch::asm;

use super::Architecture;
use crate::drivers::plic::plic_handler;

pub mod interrupts;
pub mod registers;
pub mod sv39;

pub struct Riscv64 {
    device_tree: fdt::Fdt<'static>,
}

impl Architecture for Riscv64 {
    #[naked]
    #[no_mangle]
    unsafe extern "C" fn _start() -> ! {
        asm!("la sp, STACK_START", "call k_main", options(noreturn));
    }

    fn new(info: usize) -> Self {
        let device_tree = unsafe { fdt::Fdt::from_ptr(info as *const u8).unwrap() };

        Self { device_tree }
    }

    fn for_all_memory_regions<F: FnMut(&mut dyn Iterator<Item = (usize, usize)>)>(&self, mut f: F) {
        let memory = self.device_tree.memory();
        let mut regions = memory
            .regions()
            .map(|region| (region.starting_address as usize, region.size.unwrap_or(0)));

        f(&mut regions);
    }

    fn for_all_reserved_memory_regions<F: FnMut(&mut dyn Iterator<Item = (usize, usize)>)>(
        &self,
        mut f: F,
    ) {
        let reserved_memory = self.device_tree.find_node("/reserved-memory").unwrap();

        let mut regions = reserved_memory
            .children()
            .flat_map(|child| child.reg().unwrap())
            .map(|region| (region.starting_address as usize, region.size.unwrap_or(0)));

        f(&mut regions);
    }
}
