VERSION			:= debug
ARCH 			:= aarch64
QEMU			:= qemu-system-aarch64
CROSS_COMPILE	:= aarch64-linux-gnu-

CC				:= $(CROSS_COMPILE)gcc
LD				:= $(CROSS_COMPILE)ld
OBJDUMP			:= $(CROSS_COMPILE)objdump
OBJCOPY 		:= $(CROSS_COMPILE)objcopy
GDB				:= gdb-multiarch

CFLAGS         += -ffreestanding  -Wall -Wextra
LD_FLAGS		:= -static -n -nostdlib --fatal-warnings

ifeq (${VERSION},release)
	CFLAGS 		+= -O2
else
	CFLAGS		+= -O0 -g
endif

ifeq (${ARCH},aarch64)
	CFLAGS		+= -DAARCH64
endif