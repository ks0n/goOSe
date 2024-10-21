use alloc::boxed::Box;

use super::device_tree::DeviceTree;
use super::drivers::{self, Matcher};
use super::error::Error;
use super::kernel_console;
use drivers::Console;
use fdt::{node::FdtNode, standard_nodes::MemoryRegion};

use crate::globals;
use crate::HAL;
use hal_core::mm::{NullPageAllocator, PageMap, Permissions};

pub struct DriverManager;

impl DriverManager {
    pub fn do_console(dt: &DeviceTree) -> Result<(), Error> {
        let cons_node = dt.console_node().ok_or(Error::DeviceNotFound(
            "dtb doesn't contain a console node...",
        ))?;

        map_dt_regions(&cons_node)?;

        if let Some(cons_driver) =
            Self::find_driver::<dyn Console + Sync + Send>(&cons_node, drivers::CONSOLE_MATCHERS)
        {
            kernel_console::set_console(cons_driver)?;
            Ok(())
        } else {
            unmap_dt_regions(&cons_node)?;
            Err(Error::NoMatchingDriver("console"))
        }
    }

    pub fn map_irq_chip(dt: &DeviceTree) -> Result<(), Error> {
        let intc = dt
            .interrupt_controller()
            .expect("device tree has no interrupt-controller node...");

        map_dt_regions(&intc)?;

        Ok(())
    }

    fn extract_compatibles<'a>(node: &'a FdtNode) -> impl Iterator<Item = &'a str> {
        let compatible = node
            .properties()
            .find(|prop| prop.name == "compatible")
            .and_then(|some_prop| some_prop.as_str())
            .unwrap_or("");
        compatible.split('\0')
    }

    pub fn find_driver<T: ?Sized>(node: &FdtNode, matchers: &[&Matcher<T>]) -> Option<Box<T>> {
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
}

fn num_pages(memory_region: &MemoryRegion) -> Result<usize, Error> {
    let size = memory_region.size.ok_or(Error::InvalidFdtNode)?;

    if size < HAL.page_size() {
        Ok(1)
    } else {
        Ok(size / HAL.page_size())
    }
}

fn map_dt_regions(node: &FdtNode) -> Result<(), Error> {
    if let Some(reg) = node.reg() {
        let mut kpt = HAL.kpt().lock();
        for memory_region in reg {
            let start = memory_region.starting_address as usize;
            let num_pages = num_pages(&memory_region)?;

            kpt.identity_map_range(
                start.into(),
                num_pages,
                Permissions::READ | Permissions::WRITE,
                &globals::PHYSICAL_MEMORY_MANAGER,
            )?;
        }
    }

    Ok(())
}

fn unmap_dt_regions(node: &FdtNode) -> Result<(), Error> {
    if let Some(reg) = node.reg() {
        for memory_region in reg {
            let start = memory_region.starting_address as usize;
            let size = memory_region.size.ok_or(Error::InvalidFdtNode)?;
            assert!(size % HAL.page_size() == 0);

            let mut kernel_pt = HAL.kpt().lock();
            for page in (start..start + size).step_by(HAL.page_size()) {
                kernel_pt
                    .add_invalid_entry(page.into(), &NullPageAllocator)
                    .unwrap();
            }
        }
    }

    Ok(())
}
