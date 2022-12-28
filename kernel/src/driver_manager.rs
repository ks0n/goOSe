use alloc::{boxed::Box, collections::LinkedList, sync::Arc};

use super::device_tree::DeviceTree;
use super::globals;
use super::mm;
use super::paging::PagingImpl as _;
use drivers::{Console, Driver};

pub struct DriverManager {
    drivers: LinkedList<Arc<dyn Driver>>,
}

impl DriverManager {
    fn new() -> Self {
        Self {
            drivers: LinkedList::new(),
        }
    }

    pub fn with_devices(dt: &DeviceTree) -> Self {
        let mut mgr = Self::new();

        mgr.do_console(dt).unwrap();

        mgr
    }

    fn do_console(&mut self, dt: &DeviceTree) -> Result<(), &'static str> {
        let cons_node = dt.console_node();
        if cons_node.is_none() {
            return Err("Device tree doesn't even contain a console node...");
        }
        let cons_node = cons_node.unwrap();

        map_dt_regions(&cons_node);

        if let Some(cons_driver) = self.find_console(&cons_node) {
            self.register_console(cons_driver);
            Ok(())
        } else {
            unmap_dt_regions(&cons_node);
            Err("Failed to find a matching driver for the console...")
        }
    }

    pub fn find_console(
        &self,
        cons_node: &fdt::node::FdtNode,
    ) -> Option<Box<dyn Console + Send + Sync>> {
        let console_base = cons_node.reg()?.next()?.starting_address as usize;

        // TODO: I'm not sure we are handling/parsing the compat string very well ^^
        let compatible = cons_node
            .properties()
            .find(|prop| prop.name == "compatible")
            .and_then(|some_prop| some_prop.as_str())
            .unwrap_or("");
        let compatibles = compatible.split('\0');

        for compatible in compatibles {
            if let Some(cons_driver) = drivers::matching_console_driver(compatible) {
                return Some(cons_driver(console_base));
            }
        }

        None
    }

    fn register_console(&mut self, cons: Box<dyn Console + Sync + Send>) {
        let cons: Arc<dyn Console + Sync + Send> = Arc::from(cons);
        self.register_driver(cons.clone());
        globals::CONSOLE.set(cons.clone()).unwrap();
    }

    fn register_driver(&mut self, drv: Arc<dyn Driver>) {
        self.drivers.push_back(drv);
    }
}

fn map_dt_regions(node: &fdt::node::FdtNode) {
    let pagesize = crate::PagingImpl::get_page_size();

    if let Some(reg) = node.reg() {
        for memory_region in reg {
            let start = memory_region.starting_address as usize;
            let size = memory_region.size.unwrap();

            for page in (start..start + size).step_by(pagesize) {
                globals::KERNEL_PAGETABLE.lock(|pagetable| {
                    pagetable
                        .map(
                            page.into(),
                            page.into(),
                            mm::Permissions::READ | mm::Permissions::WRITE,
                        )
                        .unwrap();
                });
            }
        }
    }
}

fn unmap_dt_regions(node: &fdt::node::FdtNode) {
    let pagesize = crate::PagingImpl::get_page_size();

    if let Some(reg) = node.reg() {
        for memory_region in reg {
            let start = memory_region.starting_address as usize;
            let size = memory_region.size.unwrap_or(pagesize);

            for page in (start..start + size).step_by(pagesize) {
                globals::KERNEL_PAGETABLE.lock(|pagetable| {
                    pagetable.add_invalid_entry(page.into()).unwrap();
                });
            }
        }
    }
}
