.text
.global _start
.type _start,@function

_start:
#ifdef __riscv
	addi a0, x0, 42
	ret
#elif __aarch64__
	mov w8, 42
	svc 0
#else
	#error This example is not available for the selected target
#endif
