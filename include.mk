VERSION ?= debug
ARCH ?= arm64
ENDIAN ?= little


ifeq ($(ARCH), arm64)
	CROSS_COMPILE := aarch64-none-elf-
	QEMU := qemu-system-aarch64
else
	CROSS_COMPILE := arm-none-eabi-
	QEMU := qemu-system-arm
endif

CROSS_COMPILE := aarch64-none-elf-

CC := $(CROSS_COMPILE)gcc
LD := $(CROSS_COMPILE)ld
OBJCOPY := $(CROSS_COMPILE)objcopy
OBJDUMP := $(CROSS_COMPILE)objdump
GDB := $(CROSS_COMPILE)gdb

LD_SCRIPT := kernel.ld

CFLAGS := -Wall -nostdinc -nostdlib -nostartfiles -ffreestanding

LDFLAGS := -T $(LD_SCRIPT) -nostdlib

QEMUFLAGS := -M raspi4b -m 256M -nographic


ifeq ($(ENDIAN), little)
	CFLAGS += -mlittle-endian -DLITTLE_ENDIAN
else
	CFLAGS += -mbig-endian -DBIG_ENDIAN
endif

ifeq ($(VERSION), debug)
	CFLAGS += -g
else
	CFLAGS += -O2
endif


TARGET_DIR := target

KERNEL := $(TARGET_DIR)/kernel.elf




