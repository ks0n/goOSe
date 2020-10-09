ENTRY(_start)

MEMORY
{
    ram (rx) : ORIGIN = 0x80200000, LENGTH = 256M
}

SECTIONS
{
    PROVIDE(START_START = 0x8020000);
    .start : { *(.start*) } > ram
    PROVIDE(START_END = .);

    PROVIDE(TEXT_START = .);
    .text : { *(.text*) } > ram
    PROVIDE(TEXT_END = .);

    PROVIDE(data_start = .);
    .data : { *(.data*) } > ram
    PROVIDE(DATA_END = .);

    PROVIDE(SDATA_START = .);
    .sdata : { *(.sdata*) } > ram
    PROVIDE(SDATA_END = .);

    PROVIDE(RODATA_START = .);
    .rodata : { *(.rodata*) } > ram
    PROVIDE(RODATA_END = .);

    PROVIDE(BSS_START = .);
    .bss (NOLOAD) : {
        *(.bss*)
    } > ram
    PROVIDE(BSS_END = .);

    PROVIDE(STACK_END = BSS_END);
    PROVIDE(STACK_START = BSS_END + 1M);
}
