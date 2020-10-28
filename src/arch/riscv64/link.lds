ENTRY(kstart)
EXTERN(ARCH);

MEMORY
{
    ram (rx) : ORIGIN = 0x80200000, LENGTH = 2048M
}

SECTIONS
{
    .text : { 
        TEXT_START = .;
        *(.text.kstart); # Warning: Should allways be at the top of all sections
        *(.text*);
        TEXT_END = .;
    } > ram

    .data : { 
        DATA_START = .;
        *(.data*);
        *(.rodata*);
        KEEP(*(.rodata.ARCH));
        *(.sdata*);
        DATA_END = .;
    } > ram

    .bss (NOLOAD) : {
        BSS_START = .;
        *(.bss*);
        BSS_END = .;
    } > ram

    . = ALIGN(4096);
    STACK_END = .;
    STACK_START = . + 1M;

    HEAP_START = .;
    HEAP_END = . + 25M;

    PROVIDE(MMU = .);
}
