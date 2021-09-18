ENTRY(_start)

SECTIONS
{
    . = 0x80200000;

    .text : {
        *(.text._start); # _start should allways be at the top of all sections
        *(.text*);
    }

    .data : {
        *(.data*);
    }

    .rodata : {
        *(.rodata*);
    }
}
