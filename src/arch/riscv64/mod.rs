use super::Architecture;
use crate::drivers::plic::plic_handler;

pub mod sv39;

#[no_mangle]
static mut g_higher_trap_handler: *const () = 0 as *const ();

static mut INTERRUPT_VECTOR: &[extern "C" fn()] = &[
    undefined_handler,
    undefined_handler,
    undefined_handler,
    undefined_handler,
    undefined_handler,
    undefined_handler,
    undefined_handler,
    undefined_handler,
    undefined_handler,
    plic_handler,
];

pub struct Riscv64 {}

impl Riscv64 {
    pub fn new() -> Self {
        Self {}
    }

    fn set_sstatus_sie(&self) {
        unsafe {
            asm!("csrrs zero, sstatus, {}", in(reg)1 << 1);
        }
    }

    fn set_sie_ssie(&self) {
        unsafe {
            asm!("csrrs zero, sie, {}", in(reg)1 << 1);
        }
    }

    fn set_sie_seie(&self) {
        unsafe {
            asm!("csrrs zero, sie, {}", in(reg)1 << 9);
        }
    }

    fn set_stvec(&self, addr: usize) {
        unsafe {
            asm!("csrw stvec, {}", in(reg)(addr));
        }
    }

    fn set_higher_trap_handler(&mut self, higher_trap_handler: fn(cause: usize)) {
        unsafe {
            g_higher_trap_handler = higher_trap_handler as *const ();
        }
    }
}

impl Architecture for Riscv64 {
    #[naked]
    #[no_mangle]
    unsafe extern "C" fn _start() -> ! {
        asm!("la sp, STACK_START", "call k_main", options(noreturn));
    }

    fn init_interrupts(&mut self) {
        self.set_sstatus_sie();
        self.set_sie_ssie();
        self.set_sie_seie();
        self.set_stvec(trap_handler as usize);
        self.set_higher_trap_handler(trap_dispatch);
    }
}

fn is_interrupt(cause: usize) -> bool {
    (cause >> 63) == 1
}

fn trap_dispatch(cause: usize) {
    if is_interrupt(cause) {
        let exception_code = cause & !(1 << 63);
        unsafe {
            INTERRUPT_VECTOR[exception_code]();
        }
    } else {
        panic!("Exception not implemented yet");
    }
}

extern "C" fn undefined_handler() {
    panic!("Interruption is not handled yet");
}

#[naked]
#[no_mangle]
#[repr(align(4))]
unsafe extern "C" fn trap_handler() {
    asm!(
        "
        addi sp, sp, -0x100

        sd x31, 0x100(sp)
        sd x30, 0xf8(sp)
        sd x29, 0xf0(sp)
        sd x28, 0xd8(sp)
        sd x27, 0xd0(sp)
        sd x26, 0xc8(sp)
        sd x25, 0xc0(sp)
        sd x24, 0xb8(sp)
        sd x23, 0xb0(sp)
        sd x22, 0xa8(sp)
        sd x21, 0xa0(sp)
        sd x20, 0x98(sp)
        sd x19, 0x90(sp)
        sd x18, 0x88(sp)
        sd x17, 0x80(sp)
        sd x16, 0x78(sp)
        sd x15, 0x70(sp)
        sd x14, 0x68(sp)
        sd x13, 0x60(sp)
        sd x12, 0x58(sp)
        sd x11, 0x50(sp)
        sd x10, 0x48(sp)
        sd x9, 0x40(sp)
        sd x8, 0x38(sp)
        sd x7, 0x30(sp)
        sd x6, 0x28(sp)
        sd x5, 0x20(sp)
        sd x4, 0x18(sp)
        sd x3, 0x10(sp)
        sd x2, 0x8(sp)
        sd x1, 0x0(sp)

        // mv a0, sp // Pointer on stack for the register struct
        // csrr a1, sepc
        // csrr a2, stval
        // csrr a3, scause
        // csrr a5, sstatus

        csrr a0, scause
        ld t0, g_higher_trap_handler
        jalr t0

        csrr t0, sepc
        addi t0, t0, 4
        csrw sepc, t0

        ld x1, 0x0(sp)
        ld x2, 0x8(sp)
        ld x3, 0x10(sp)
        ld x4, 0x18(sp)
        ld x5, 0x20(sp)
        ld x6, 0x28(sp)
        ld x7, 0x30(sp)
        ld x8, 0x38(sp)
        ld x9, 0x40(sp)
        ld x10, 0x48(sp)
        ld x11, 0x50(sp)
        ld x12, 0x58(sp)
        ld x13, 0x60(sp)
        ld x14, 0x68(sp)
        ld x15, 0x70(sp)
        ld x16, 0x78(sp)
        ld x17, 0x80(sp)
        ld x18, 0x88(sp)
        ld x19, 0x90(sp)
        ld x20, 0x98(sp)
        ld x21, 0xa0(sp)
        ld x22, 0xa8(sp)
        ld x23, 0xb0(sp)
        ld x24, 0xb8(sp)
        ld x25, 0xc0(sp)
        ld x26, 0xc8(sp)
        ld x27, 0xd0(sp)
        ld x28, 0xd8(sp)
        ld x29, 0xf0(sp)
        ld x30, 0xf8(sp)
        ld x31, 0x100(sp)

        addi sp, sp, 0x100

        sret",
        options(noreturn)
    );
    // Obviously this isn't done, we need to jump back to the previous context before the
    // interrupt using mpp/spp and mepc/sepc.
}
