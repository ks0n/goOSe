CC := clang

ASFLAGS += -Wall -Wextra -nostdlib -nostartfiles -static -mno-relax
LDFLAGS += -fuse-ld=lld

EXEC = small

.PHONY = all clean

all:
	echo 'You forgot to sepcify your rust target'
	exit 1

riscv64gc-unknown-none-elf: ASFLAGS+=--target=riscv64-unknown-none-elf-riscv64gc
riscv64gc-unknown-none-elf: $(EXEC)

clean:
	rm -f *.o
	rm -f $(EXEC)


