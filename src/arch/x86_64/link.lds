ENTRY(kstart)

SECTIONS
{
    . = ALIGN(4096);
    .text : { 
        TEXT_START = .;
        *(.text*);
        TEXT_END = .;
    }

    . = ALIGN(4096);
    .data : { 
        DATA_START = .;
        *(.data*);
        *(.rodata*);
        *(.sdata*);
        DATA_END = .;
    }

    . = ALIGN(4096);
    .bss (NOLOAD) : {
        BSS_START = .;
        *(.bss*);
        BSS_END = .;
    }

    . = ALIGN(4096);
    STACK_END = .;
    STACK_START = BSS_END + 1M;
}
