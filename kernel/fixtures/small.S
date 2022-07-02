#ifdef __riscv
.text
.global _start
.type _start,@function

_start:
	addi a0, x0, 42
	ret
#else
	#error This example is not available for the selected target
#endif
