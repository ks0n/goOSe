ENTRY(kstart)
EXTERN(ARCH);

SECTIONS
{
    . = 0x80200000;

    .text : {
        TEXT_START = .;
        *(.text.kstart); # Warning: Should allways be at the top of all sections
        *(.text*);
        TEXT_END = .;
    }

    .data : {
        DATA_START = .;
        *(.data*);
        *(.rodata*);
        KEEP(*(.rodata.ARCH));
        *(.sdata*);
        DATA_END = .;
    }

    .bss (NOLOAD) : {
        BSS_START = .;
        *(.bss*);
        BSS_END = .;
    }

    . = ALIGN(4096);
    HEAP_START = .;
    HEAP_END = HEAP_START + 1M;

    . = ALIGN(4096);
    STACK_END = .;
    STACK_START = STACK_END + 1M;

    . = ALIGN(4096);
    TRAP_STACK_END = .;
    TRAP_STACK_START = TRAP_STACK_END + 1K;
}
