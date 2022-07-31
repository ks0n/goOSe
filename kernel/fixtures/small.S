#ifdef __riscv
.text
.global _start
.type _start,@function

_start:
	li a7, 42
	ecall
#else
	#error This example is not available for the selected target
#endif
