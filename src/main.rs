#![no_std]
#![no_main]
#![feature(asm)]
#![feature(fn_align)]
#![feature(naked_functions)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::kernel_tests::runner)]
#![reexport_test_harness_main = "ktests_launch"]

mod arch;
mod drivers;
mod kernel_serial;
mod mm;
mod utils;

#[cfg(test)]
mod kernel_tests;
use core::include_bytes;

use arch::Architecture;
use drivers::ns16550::*;
use drivers::plic;

use dtb;

static FDT_BYTES: &[u8] = include_bytes!("../virt.dtb");

fn print_node(items: &mut dtb::StructItems, level: usize) {
    loop {
        match items.next_item().unwrap() {
            dtb::StructItem::EndNode => {
                kprintln!("{: >width$}}}", "", width = level - 4);

                return;
            }
            dtb::StructItem::BeginNode { name } => {
                kprintln!("{: >width$}Node: {} {{", "", name, width = level);
                print_node(items, level + 4);
            }
            dtb::StructItem::Property { name, value } => {
                kprintln!(
                    "{: >width$}Property: {} {:?}",
                    "",
                    name,
                    value,
                    width = level
                );
            }
        }
    }
}

#[no_mangle]
fn k_main() -> ! {
    #[cfg(test)]
    ktests_launch();

    kprintln!("GoOSe is booting");

    let mut arch = arch::new_arch();

    let dtb = dtb::Reader::read(FDT_BYTES).unwrap();
    print_node(&mut dtb.struct_items(), 0);

    arch.init_interrupts();

    // Enable Serial interrupts
    plic::init(plic::QEMU_VIRT_PLIC_BASE_ADDRESS);
    let plic = plic::get();
    if let Err(e) = plic.set_priority(QEMU_VIRT_NS16550_INTERRUPT_NUMBER, 1) {
        kprintln!("{}", e);
    }
    if let Err(e) = plic.enable_interrupt(QEMU_VIRT_NS16550_INTERRUPT_NUMBER, 0) {
        kprintln!("{}", e);
    }
    plic.set_threshold(0);

    let mut memory = mm::MemoryManager::<arch::MemoryImpl>::new();
    memory.map_address_space();

    kprintln!("Virtual memory enabled!");

    loop {
        unsafe {
            asm!("wfi");
        }
    }
}
