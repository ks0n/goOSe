use super::Architecture;

#[no_mangle]
static mut g_higher_trap_handler: *const () = 0 as *const ();

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

    fn set_sip_ssip(&self) {
        unsafe {
            asm!("csrrs zero, sip, {}", in(reg)1 << 1);
        }
    }

    fn set_stvec(&self, addr: u64) {
        unsafe {
            asm!("csrw stvec, {}", in(reg)(addr));
        }
    }

    fn set_higher_trap_handler(&mut self, higher_trap_handler: fn()) {
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
<<<<<<< HEAD
        self.set_stvec(Self::trap_handler as u64);
=======
        self.set_sie_seie();
        self.set_stvec(trap_handler as u64);
>>>>>>> b1ac19e (Add alignment to trap_handler)

        self.set_higher_trap_handler(|| {
            // well fuck we can't do anything without a context (like print a message on serial,
            // but we can't since we don't have acces to the serial object.
            // Should `init_interrupts` take a context parameters of some sort ?
            // And don't you think taking in closures as trap handlers is pretty fucking sick ?!?
            unsafe {
                asm!("wfi");
            }
        });

        self.set_sip_ssip();
    }
}

#[naked]
#[no_mangle]
#[repr(align(4))]
unsafe extern "C" fn trap_handler() -> ! {
    // FIXME: When going to g_higher_trap_handler, we do not find the `wfi` opcode
<<<<<<< HEAD
    asm!("jalr {}", in(reg)g_higher_trap_handler, options(noreturn));
=======
    asm!("call g_higher_trap_handler", options(noreturn));
>>>>>>> b1ac19e (Add alignment to trap_handler)
    // Obviously this isn't done, we need to jump back to the previous context before the
    // interrupt using mpp/spp and mepc/sepc.
}
