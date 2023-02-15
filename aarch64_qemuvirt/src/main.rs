#![no_std]
#![no_main]

#[cfg(not(target_arch = "aarch64"))]
compile_error!("Must be compiled as aarch64");

// use kernel::drivers::gicv2::GicV2;
use kernel::drivers::pl011::Pl011;
use kernel::drivers::qemuexit::QemuExit;
use kernel::drivers::Console;
use kernel::paging::PagingImpl;
use kernel::irq::{self, IrqChip, Interrupt};
use kernel::drivers::gicv2::GicV2;
use kernel::alloc::sync::Arc;

use core::arch::asm;
use cortex_a::asm;
use cortex_a::registers::*;
use tock_registers::interfaces::{ReadWriteable, Writeable};

const DTB_ADDR: usize = 0x4000_0000;

#[no_mangle]
extern "C" fn k_main(_device_tree_ptr: usize) -> ! {
    kernel::arch::aarch64::Aarch64::disable_fp_trap();

    static PL011: Pl011 = Pl011::new(0x0900_0000);
    kernel::globals::set_earlyinit_console(&PL011);

    kernel::kprintln!("hello, I am GoOSe!");

    unsafe {
        kernel::arch::aarch64::Aarch64::init_el1_exception_handlers();
    }

    unsafe {
        asm::barrier::isb(asm::barrier::SY);
        asm::barrier::dmb(asm::barrier::SY);
        asm::barrier::dsb(asm::barrier::SY);
    };

    let device_tree = kernel::device_tree::DeviceTree::new(DTB_ADDR).unwrap();
    let qemu_exit = QemuExit::new();

    kernel::globals::PHYSICAL_MEMORY_MANAGER
        .lock(|pmm| pmm.init_from_device_tree(&device_tree, 4096))
        .unwrap();
    kernel::mm::map_address_space(&device_tree, &[&PL011]);

    kernel::kprintln!("PMM has been initialized with the device tree... check");
    kernel::kprintln!(
        "kernel's address space has been mapped into the kernel's pagetable... check"
    );
    kernel::kprintln!("Kernel bootstrap should be about done.");

    let _drvmgr = kernel::driver_manager::DriverManager::with_devices(&device_tree).unwrap();
    // drvmgr.get_console();
    // drvmgr.get_irq_manager();

    kernel::globals::KERNEL_PAGETABLE
        .lock(|pagetable| {
            pagetable.map(
                0x0800_0000.into(),
                0x0800_0000.into(),
                kernel::mm::Permissions::READ | kernel::mm::Permissions::WRITE,
            ).unwrap();
            pagetable.map(
                0x0801_0000.into(),
                0x0801_0000.into(),
                kernel::mm::Permissions::READ | kernel::mm::Permissions::WRITE,
            ).unwrap();
        });
    let mut gic = Arc::new(GicV2::new(0x800_0000, 0x801_0000));
    gic.enable(Interrupt::PhysicalTimer);
    gic.enable_interrupts();
    kernel::globals::IRQ_CHIP.set(gic);

    if true {
        // IRQ
        kernel::arch::aarch64::Aarch64::unmask_interrupts();
        kernel::arch::aarch64::Aarch64::set_timer(50_000);
    } else {
        // Synchronous exception
        unsafe {
            asm!("svc 42");
        }
    }

    kernel::kprintln!("Testing the remapping capabilities of our pagetable...");
    kernel::globals::KERNEL_PAGETABLE
        .lock(|pagetable| {
            pagetable.map(
                0x0900_0000.into(),
                0x0450_0000.into(),
                kernel::mm::Permissions::READ | kernel::mm::Permissions::WRITE,
            )
        })
        .unwrap();
    let uart = Pl011::new(0x0450_0000);
    uart.write("Uart remaped, if you see this, it works !!!\n");

    kernel::kprintln!("[OK] GoOSe shuting down, bye!");
    qemu_exit.exit_success();
}
