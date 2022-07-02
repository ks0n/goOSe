TARGET = $(shell awk '/target =/ { print substr($$3, 2, length($$3) - 2) }' ../.cargo/config)
CC := clang

ASFLAGS += -Wall -Wextra -nostdlib -nostartfiles -static -mno-relax
LDFLAGS += -fuse-ld=lld

EXEC = small

.PHONY = all clean

all: $(TARGET) $(EXEC)

riscv64gc-unknown-none-elf:
	$(eval ASFLAGS+=--target=riscv64-unknown-none-elf-riscv64gc)

clean:
	rm -f *.o
	rm -f $(EXEC)


