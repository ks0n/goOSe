ENTRY(_start)

MEMORY
{
    ram (rx) : ORIGIN = 0x200000, LENGTH = 256M
}

SECTIONS
{
    .start : { *(.start*) } > ram
    .text : { *(.text*) } > ram
    .data : { *(.data*) } > ram
    .sdata : { *(.sdata*) } > ram
    .rodata : { *(.rodata*) } > ram
    .got : { *(.got*) } > ram
    .bss (NOLOAD) : {
        *(.bss*)
        PROVIDE(_bss_end = .);
    } > ram

    PROVIDE(_stack = _bss_end + 1M);
}
