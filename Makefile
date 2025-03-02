VERSION 	:= debug
PLATFORM	:= aarch64-unknown-none
TARGET_DIR 	:= target/$(PLATFORM)/$(VERSION)
TARGET		:= $(TARGET_DIR)/evelyn
CARGO 		:= cargo
QEMU 		:= qemu-system-aarch64
GDB			:= gdb-multiarch
DTC 		:= dtc

SCRIPT_DIR	:= script

DTB_FILE 	:= raspi4b.dtb
DTS_FILE 	:= raspi4b.dts
QEMU_RUN_SCRIPT	:= $(SCRIPT_DIR)/run-qemu.sh
GDB_FLAGS	:= -ex "target remote localhost:1234" -ex "add-symbol-file $(TARGET)"
BUILD_FLAGS	:= 

ifeq ($(VERSION),release)
	BUILD_FLAGS += --release
endif


.PHONY:all clean run dbg_run dbg dts $(TARGET)

all:
	$(CARGO) build $(BUILD_FLAGS)

clean:
	@$(CARGO) clean
	@rm $(DTS_FILE) -f

run:
	$(QEMU_RUN_SCRIPT) $(TARGET)

dbg:
	QEMU_MODE=debug $(QEMU_RUN_SCRIPT) $(TARGET)

dbg_run:
	$(GDB) $(GDB_FLAGS)

dts:
	dtc -I dtb -O dts $(DTB_FILE) -o $(DTS_FILE)

$(TARGET):all	
