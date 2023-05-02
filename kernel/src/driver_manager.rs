use alloc::{boxed::Box, collections::LinkedList, sync::Arc};

use super::device_tree::DeviceTree;
use super::drivers::{self, Matcher};
use super::error::Error;
use super::globals;
use drivers::{Console, Driver};
use fdt::node::FdtNode;

use crate::hal;
use crate::mm::alloc_pages_for_hal;
use hal_core::mm::{PageMap, Permissions};

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

        if let Some(cons_driver) =
            self.find_driver::<dyn Console + Sync + Send>(&cons_node, drivers::CONSOLE_MATCHERS)
        {
            self.register_console(cons_driver)?;
            Ok(())
        } else {
            unmap_dt_regions(&cons_node)?;
            Err(Error::NoMatchingDriver("console"))
        }
    }

    fn extract_compatibles<'a>(node: &'a FdtNode) -> impl Iterator<Item = &'a str> {
        let compatible = node
            .properties()
            .find(|prop| prop.name == "compatible")
            .and_then(|some_prop| some_prop.as_str())
            .unwrap_or("");
        compatible.split('\0')
    }

    pub fn find_driver<T: ?Sized>(
        &self,
        node: &FdtNode,
        matchers: &[&Matcher<T>],
    ) -> Option<Box<T>> {
        for compat in Self::extract_compatibles(node) {
            let matching_constructor = matchers
                .iter()
                .find(|matcher| matcher.matches(compat))
                .map(|matcher| matcher.constructor);
            if let Some(constructor) = matching_constructor {
                if let Ok(driver) = constructor(&mut node.reg()?) {
                    return Some(driver);
                }
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

fn map_dt_regions(node: &FdtNode) -> Result<(), Error> {
    if let Some(reg) = node.reg() {
        for memory_region in reg {
            let start = memory_region.starting_address as usize;
            let size = memory_region.size.ok_or(Error::InvalidFdtNode)?;

            assert!(size % hal::mm::PAGE_SIZE == 0);
            hal::mm::current().identity_map_range(start.into(), size / hal::mm::PAGE_SIZE, Permissions::READ | Permissions::WRITE, alloc_pages_for_hal)?;
        }
    }

    Ok(())
}

fn unmap_dt_regions(node: &FdtNode) -> Result<(), Error> {
    let pagesize = hal::mm::PAGE_SIZE;

    if let Some(reg) = node.reg() {
        for memory_region in reg {
            let start = memory_region.starting_address as usize;
            let size = memory_region.size.ok_or(Error::InvalidFdtNode)?;
            assert!(size % hal::mm::PAGE_SIZE == 0);

            let kernel_pt = hal::mm::current();
            for page in (start..start + size).step_by(pagesize) {
                kernel_pt.add_invalid_entry(page.into(), |_| unreachable!()).unwrap();
            }
        }
    }

    Ok(())
}
