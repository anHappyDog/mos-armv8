include include.mk


DEVICE			:= raspi4b
MEMORY			:= 2g
SMP				:= 4
MODULES			:= boot driver kernel lib
OBJECTS         := $(addsuffix /*.o, $(MODULES))

TARGET_DIR		:= target
SCRIPT_DIR		:= script

LINK_SCRIPT		:= $(SCRIPT_DIR)/kernel.ld
DTB_FILE 		:= $(SCRIPT_DIR)/raspi4b.dtb
DTS_FILE 		:= $(TARGET_DIR)/raspi4b.dts
TARGET			:= $(TARGET_DIR)/mos

QEMU_FLAGS		:= -M $(DEVICE) -m $(MEMORY) -dtb $(DTB_FILE) -smp $(SMP) -nographic

.PHONY:all clean run dbg dbg-run $(MODULES) objdump

export CC CFLAGS LD LDFLAGS


$(TARGET_DIR):
	mkdir -p $(TARGET_DIR)

all: $(MODULES) $(TARGET_DIR)
	$(LD) $(LDFLAGS) -o $(TARGET) -N -T $(LINK_SCRIPT) $(OBJECTS)

$(MODULES):
	$(MAKE) --directory=$@

clean:
	for d in $(MODULES); do\
		if [ -f $$d/Makefile ]; then\
			$(MAKE) --directory=$$d clean;\
		fi;\
	done
	rm $(TARGET_DIR) -rf

run:
	$(QEMU) $(QEMU_FLAGS) -kernel $(TARGET)

dbg:
	$(QEMU) $(QEMU_FLAGS) -kernel $(TARGET) -s -S

dbg-run:
	$(GDB) -ex "target remote localhost:1234" -ex "file $(TARGET)"

fix-style: clean
	find ./ -name "*.c" -o -name "*.h" | xargs clang-format -i

objdump:
	$(OBJDUMP) -Ds $(TARGET) > $(TARGET).asm

dts:$(target_dir)
	dtc -I dtb -O dts $(DTB_FILE) -o $(DTS_FILE)



