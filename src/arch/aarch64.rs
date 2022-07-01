use core::arch::asm;

use super::Architecture;

pub struct Aarch64 {
    device_tree: fdt::Fdt<'static>,
}

impl Architecture for Aarch64 {
    #[naked]
    #[no_mangle]
    unsafe extern "C" fn _start() -> ! {
        asm!("adrp x0, STACK_START", "mov sp, x0", "b k_main", options(noreturn));
    }

    fn new(info: usize) -> Self {
        if let Ok(device_tree) = unsafe { fdt::Fdt::from_ptr(info as *const u8) } {
            return Self { device_tree }
        }

        loop {}
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
