ENTRY(_start)

SECTIONS
{
    /* . = 0x80200000; */
    . = 0x40100000;

    KERNEL_START = . ;
    .text : {
        *(.text._start); # _start should allways be at the top of all sections
        *(.text*);
    }

    .data : {
        *(.data*);
    }

    .sdata : {
        *(.sdata*);
    }

    .rodata : {
        *(.rodata*);
    }

    .eh_frame : {
        *(.eh_frame*);
    }

    .bss : {
        *(.bss*);
    }

    .sbss : {
        *(.sbss*);
    }

    . = ALIGN(4096);
    STACK_END = . ;
    . = . + 1M;
    STACK_START = . ;

    KERNEL_END = . ;
}
