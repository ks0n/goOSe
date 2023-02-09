.section .text

.macro save_regs
    stp x0, x1, [sp, #-16]!
    stp x2, x3, [sp, #-16]!
    stp x4, x5, [sp, #-16]!
    stp x6, x7, [sp, #-16]!
    stp x8, x9, [sp, #-16]!
    stp x10, x11, [sp, #-16]!
    stp x12, x13, [sp, #-16]!
    stp x14, x15, [sp, #-16]!
    stp x16, x17, [sp, #-16]!
    stp x18, x29, [sp, #-16]!
    stp x30, xzr, [sp, #-16]!
.endm

.macro restore_regs
    ldp x30, xzr, [sp], #16
    ldp x18, x29, [sp], #16
    ldp x16, x17, [sp], #16
    ldp x14, x15, [sp], #16
    ldp x12, x13, [sp], #16
    ldp x10, x11, [sp], #16
    ldp x8, x9, [sp], #16
    ldp x6, x7, [sp], #16
    ldp x4, x5, [sp], #16
    ldp x2, x3, [sp], #16
    ldp x0, x1, [sp], #16
.endm

.macro gen_stub func
.balign 0x80
asm_\func:
    msr spsel, xzr
    save_regs
    bl \func
    restore_regs
    eret
.endm

.balign 0x800
el1_vector_table:

// Current EL with SP0
gen_stub sync_current_el_sp0
gen_stub irq_current_el_sp0
gen_stub fiq_current_el_sp0
gen_stub serror_current_el_sp0

// Current EL with SPx
gen_stub sync_current_el_spx
gen_stub irq_current_el_spx
gen_stub fiq_current_el_spx
gen_stub serror_current_el_spx

// Lower EL
gen_stub sync_lower_el
gen_stub irq_lower_el
gen_stub fiq_lower_el
gen_stub serror_lower_el

// Lower EL with aarch32
gen_stub sync_lower_el_aarch32
gen_stub irq_lower_el_aarch32
gen_stub fiq_lower_el_aarch32
gen_stub serror_lower_el_aarch32
