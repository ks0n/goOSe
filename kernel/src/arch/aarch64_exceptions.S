.section .text

save_regs:
    stp x0, x1, [sp, #-16]!
    stp x2, x3, [sp, #-16]!
    stp x4, x5, [sp, #-16]!
    stp x6, x7, [sp, #-16]!
    stp x8, x9, [sp, #-16]!
    stp x10, x11, [sp, #-16]!
    stp x12, x13, [sp, #-16]!
    stp x14, x15, [sp, #-16]!
    stp x16, x17, [sp, #-16]!
    stp x18, x19, [sp, #-16]!
    stp x20, x21, [sp, #-16]!
    stp x22, x23, [sp, #-16]!
    stp x24, x25, [sp, #-16]!
    stp x26, x27, [sp, #-16]!
    stp x28, x29, [sp, #-16]!
    str x30, [sp, #-8]!
    ret

restore_regs:
    ldr x30, [sp], #8
    ldp x28, x29, [sp], #16
    ldp x26, x27, [sp], #16
    ldp x24, x25, [sp], #16
    ldp x22, x23, [sp], #16
    ldp x20, x21, [sp], #16
    ldp x18, x19, [sp], #16
    ldp x16, x17, [sp], #16
    ldp x14, x15, [sp], #16
    ldp x12, x13, [sp], #16
    ldp x10, x11, [sp], #16
    ldp x8, x9, [sp], #16
    ldp x6, x7, [sp], #16
    ldp x4, x5, [sp], #16
    ldp x2, x3, [sp], #16
    ldp x0, x1, [sp], #16
    ret

.macro call_with_context_switch func
    msr spsel, xzr
    bl save_regs
    bl \func
    bl restore_regs
    eret
.endm

.balign 0x800
el1_vector_table:

// Current EL with SP0
asm_sync_current_el_sp0:
    call_with_context_switch sync_current_el_sp0

.balign 0x80
asm_irq_current_el_sp0:
    call_with_context_switch irq_current_el_sp0

.balign 0x80
asm_fiq_current_el_sp0:
    call_with_context_switch fiq_current_el_sp0

.balign 0x80
asm_serror_current_el_sp0:
    call_with_context_switch serror_current_el_sp0

// Current EL with SPx
.balign 0x80
asm_sync_current_el_spx:
    call_with_context_switch sync_current_el_spx

.balign 0x80
asm_irq_current_el_spx:
    call_with_context_switch irq_current_el_spx

.balign 0x80
asm_fiq_current_el_spx:
    call_with_context_switch fiq_current_el_spx

.balign 0x80
asm_serror_current_el_spx:
    call_with_context_switch serror_current_el_spx

// Lower EL
.balign 0x80
asm_sync_lower_el:
    call_with_context_switch sync_lower_el

.balign 0x80
asm_irq_lower_el:
    call_with_context_switch irq_lower_el

.balign 0x80
asm_fiq_lower_el:
    call_with_context_switch fiq_lower_el

.balign 0x80
asm_serror_lower_el:
    call_with_context_switch serror_lower_el

// Lower EL with aarch32
.balign 0x80
asm_sync_lower_el_aarch32:
    call_with_context_switch sync_lower_el_aarch32

.balign 0x80
asm_irq_lower_el_aarch32:
    call_with_context_switch irq_lower_el_aarch32

.balign 0x80
asm_fiq_lower_el_aarch32:
    call_with_context_switch fiq_lower_el_aarch32

.balign 0x80
asm_serror_lower_el_aarch32:
    call_with_context_switch serror_lower_el_aarch32


