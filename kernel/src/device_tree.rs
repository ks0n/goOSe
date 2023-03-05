use super::Error;

use fdt::node::FdtNode;

pub struct DeviceTree {
    addr: usize,
    dtb: fdt::Fdt<'static>,
    total_size: usize,
}

impl DeviceTree {
    pub fn new(device_tree_ptr: usize) -> Result<Self, Error> {
        let dtb = unsafe { fdt::Fdt::from_ptr(device_tree_ptr as *const u8)? };

        Ok(Self {
            addr: device_tree_ptr,
            dtb,
            total_size: dtb.total_size(),
        })
    }

    pub fn memory_region(&self) -> (usize, usize) {
        (self.addr, self.addr + self.total_size)
    }

    pub fn for_all_memory_regions<F: FnMut(&mut dyn Iterator<Item = (usize, usize)>)>(
        &self,
        mut f: F,
    ) {
        let memory = self.dtb.memory();
        let mut regions = memory
            .regions()
            .map(|region| (region.starting_address as usize, region.size.unwrap_or(0)));

        f(&mut regions);
    }

    pub fn for_all_reserved_memory_regions<F: FnMut(&mut dyn Iterator<Item = (usize, usize)>)>(
        &self,
        mut f: F,
    ) {
        match self.dtb.find_node("/reserved-memory") {
            None => (),
            Some(reserved_memory) => {
                let mut regions = reserved_memory
                    .children()
                    .flat_map(|child| child.reg())
                    .flatten()
                    .map(|region| (region.starting_address as usize, region.size.unwrap_or(0)));

                f(&mut regions);
            }
        }
    }

    pub fn console_node(&self) -> Option<FdtNode> {
        let chosen = self.dtb.chosen();
        chosen.stdout()
    }

    pub fn interrupt_controller(&self) -> Option<FdtNode> {
        // This is a funny one.
        // There can be multiple interrupt controllers:
        //   - on a "reguler" Aarch64 board, you just have a gic
        //   - on a riscv board, you have a "root" irq chip that's part of the cpu and there is a
        //     soc level interrupt controller "plic/aplic" (similar to gic).
        //  Handling this properly requires more code which we will do in the future, but for
        //  now... don't do anything particular to take care of the root irqchip and use a
        //  heuristic to find the soc level interrupt controller.
        let mut interrupt_controllers = self.dtb.all_nodes().filter(|node| node.property("interrupt-controller").is_some());

        // The heuristic, the root irq chip doesn't have a reg property.
        // Works on aarch64 and riscv64.
        let interrupt_controller = interrupt_controllers.find(|intc| intc.reg().is_some());

        interrupt_controller
    }
}
