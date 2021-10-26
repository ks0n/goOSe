ENTRY(_start)

SECTIONS
{
    . = 0x80200000;

    KERNEL_START = . ;
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
    KERNEL_END = . ;

    . = ALIGN(4096);
    STACK_END = .;
    STACK_START = STACK_END + 1M;

    . = ALIGN(4096);
    HEAP_START = .;
    HEAP_END = HEAP_START + 4M;
}
