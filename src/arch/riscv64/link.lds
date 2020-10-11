ENTRY(_start)

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
        PROVIDE(DATA_START = .);
        *(.data*);
        PROVIDE(DATA_END = .);
    } > ram

    .sdata : {
        PROVIDE(_sdata_start = .);
        *(.sdata*);
        PROVIDE(_sdata_end = .);
    } > ram

    .rodata : {
        PROVIDE(_rodata_start = .);
        *(.rodata*);
        PROVIDE(_rodata_end = .);
    } > ram

    .bss (NOLOAD) : {
        PROVIDE(_bss_start = .);
        *(.bss*);
        PROVIDE(_bss_end = .);
    } > ram

    PROVIDE(_stack_end = _bss_end);
    PROVIDE(STACK_START = _bss_end + 1M);
}
