use alloc::{boxed::Box, collections::LinkedList, sync::Arc};

use super::device_tree::DeviceTree;
use super::error::Error;
use super::globals;
use super::mm;
use super::paging::PagingImpl as _;
use super::drivers;
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

    pub fn with_devices(dt: &DeviceTree) -> Result<Self, Error> {
        let mut mgr = Self::new();

        mgr.do_console(dt)?;

        Ok(mgr)
    }

    fn do_console(&mut self, dt: &DeviceTree) -> Result<(), Error> {
        let cons_node = dt.console_node().ok_or(Error::DeviceNotFound(
            "dtb doesn't contain a console node...",
        ))?;

        map_dt_regions(&cons_node)?;

        if let Some(cons_driver) = self.find_console(&cons_node) {
            self.register_console(cons_driver)?;
            Ok(())
        } else {
            unmap_dt_regions(&cons_node)?;
            Err(Error::NoMatchingDriver("console"))
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

    fn register_console(&mut self, cons: Box<dyn Console + Sync + Send>) -> Result<(), Error> {
        let cons: Arc<dyn Console + Sync + Send> = Arc::from(cons);
        self.register_driver(cons.clone());
        globals::CONSOLE.set(cons.clone())?;

        Ok(())
    }

    fn register_driver(&mut self, drv: Arc<dyn Driver>) {
        self.drivers.push_back(drv);
    }
}

fn map_dt_regions(node: &fdt::node::FdtNode) -> Result<(), Error> {
    let pagesize = crate::PagingImpl::get_page_size();

    if let Some(reg) = node.reg() {
        for memory_region in reg {
            let start = memory_region.starting_address as usize;
            let size = memory_region.size.ok_or(Error::InvalidFdtNode)?;

            for page in (start..start + size).step_by(pagesize) {
                globals::KERNEL_PAGETABLE.lock(|pagetable| {
                    pagetable.map(
                        page.into(),
                        page.into(),
                        mm::Permissions::READ | mm::Permissions::WRITE,
                    )
                })?;
                // TODO: if the above fails, we should just try? but also unmap the already mapped
                // stuff before returning an Error.
            }
        }
    }

    Ok(())
}

fn unmap_dt_regions(node: &fdt::node::FdtNode) -> Result<(), Error> {
    let pagesize = crate::PagingImpl::get_page_size();

    if let Some(reg) = node.reg() {
        for memory_region in reg {
            let start = memory_region.starting_address as usize;
            let size = memory_region.size.ok_or(Error::InvalidFdtNode)?;

            let kernel_pt = globals::KERNEL_PAGETABLE.lock(|pt| pt);
            for page in (start..start + size).step_by(pagesize) {
                kernel_pt.add_invalid_entry(page.into())?;
            }
        }
    }

    Ok(())
}
