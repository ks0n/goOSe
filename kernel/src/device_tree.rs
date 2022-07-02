pub struct DeviceTree {
    dtb: fdt::Fdt<'static>,
}

impl DeviceTree {
    pub fn new(device_tree_ptr: usize) -> Self {
        let dtb = unsafe { fdt::Fdt::from_ptr(device_tree_ptr as *const u8).unwrap() };

        Self { dtb }
    }

    pub fn for_all_memory_regions<F: FnMut(&mut dyn Iterator<Item = (usize, usize)>)>(&self, mut f: F) {
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
        let reserved_memory = self.dtb.find_node("/reserved-memory").unwrap();

        let mut regions = reserved_memory
            .children()
            .flat_map(|child| child.reg().unwrap())
            .map(|region| (region.starting_address as usize, region.size.unwrap_or(0)));

        f(&mut regions);
    }
}
