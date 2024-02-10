use log::debug;

use core::arch::asm;

pub fn switch(entry: usize) {
    unsafe { core::mem::transmute::<_, extern "C" fn()>(entry)() };
}

pub fn switch_userland(entry: usize, stack: *mut u8) {
    debug!("Inside switch_userland");

    const u_mode_mask: usize = 1 << 8;
    unsafe {
        asm!("
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

            csrw sscratch, sp    #Save kernel stack
            csrc sstatus, {0}    #Select u mode
            csrw sepc, {1}       #Set jump address

            mv sp, {2}

            sret
        ",
            in(reg) u_mode_mask,
            in(reg) entry,
            in(reg) stack
        );
    }
}
