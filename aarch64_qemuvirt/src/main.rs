#![no_std]
#![no_main]

#[cfg(not(target_arch = "aarch64"))]
compile_error!("Must be compiled as aarch64");

use kernel::drivers::gicv2::GicV2;
use kernel::drivers::pl011::Pl011;
use kernel::drivers::qemuexit::QemuExit;
use kernel::drivers::Console;
use kernel::paging::PagingImpl;

use cortex_a::asm;

const DTB_ADDR: usize = 0x4000_0000;

#[no_mangle]
extern "C" fn k_main(_device_tree_ptr: usize) -> ! {
    kernel::arch::aarch64::Aarch64::disable_fp_trap();

    static PL011: Pl011 = Pl011::new(0x0900_0000);
    kernel::globals::set_earlyinit_console(&PL011);

    kernel::kprintln!("hello, I am GoOSe!");

    let device_tree = kernel::device_tree::DeviceTree::new(DTB_ADDR);

    kernel::globals::PHYSICAL_MEMORY_MANAGER
        .lock(|pmm| pmm.init_from_device_tree(&device_tree, 4096));
    kernel::mm::map_address_space(&device_tree, &[&PL011]);

    kernel::kprintln!("PMM has been initialized with the device tree... check");
    kernel::kprintln!(
        "kernel's address space has been mapped into the kernel's pagetable... check"
    );
    kernel::kprintln!("Kernel initialization should be about done.");

    let mut gic = GicV2::new(0x8000000, 0x8010000);
    gic.enable(30); // Physical timer
    gic.enable_interrupts();

    unsafe {
        kernel::arch::aarch64::Aarch64::init_el1_interrupts();
    }

    unsafe {
        asm::barrier::isb(asm::barrier::SY);
        asm::barrier::dmb(asm::barrier::SY);
        asm::barrier::dsb(asm::barrier::SY);
    };

    let device_tree = kernel::device_tree::DeviceTree::new(DTB_ADDR);
    let qemu_exit = QemuExit::new();

    kernel::globals::PHYSICAL_MEMORY_MANAGER
        .lock(|pmm| pmm.init_from_device_tree(&device_tree, 4096));
    kernel::mm::map_address_space(&device_tree, &[&PL011]);

    kernel::kprintln!("PMM has been initialized with the device tree... check");
    kernel::kprintln!(
        "kernel's address space has been mapped into the kernel's pagetable... check"
    );
    kernel::kprintln!("Kernel initialization should be about done.");

    kernel::kprintln!("Testing the remapping capabilities of our pagetable...");
    kernel::globals::KERNEL_PAGETABLE.lock(|pagetable| {
        pagetable.map(
            0x0900_0000.into(),
            0x0450_0000.into(),
            kernel::mm::Permissions::READ | kernel::mm::Permissions::WRITE,
        );
    });
    let uart = Pl011::new(0x0450_0000);
    uart.write("Uart remaped, if you see this, it works !!!\n");

    kernel::kprintln!("[OK] GoOSe shuting down, bye bye!");
    qemu_exit.exit_success();
}
