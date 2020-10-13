ENTRY(kstart)
EXTERN(ARCH);

MEMORY
{
    ram (rx) : ORIGIN = 0x80200000, LENGTH = 256M
}

SECTIONS
{
    .start : {
        START_START = .;
        *(.start*);
        START_END = .;
    } > ram

    .text : { 
        TEXT_START = .;
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
    STACK_START = BSS_END + 1M;
}
