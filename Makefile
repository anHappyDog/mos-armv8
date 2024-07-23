
include include.mk


MODULES := boot lib

export CC CFLAGS LD LDFLAGS

$(MODULES):
	$(MAKE) --directory=$@ all

$(KERNEL) : $(TARGET_DIR) $(MODULES)
	$(LD) $(LDFLAGS) -o $(KERNEL) $(foreach module, $(MODULES), $(shell find $(module) -type f -name '*.o'))

$(TARGET_DIR):
	mkdir -p $(TARGET_DIR)

all: $(KERNEL)

run:$(KERNEL)
	$(QEMU) $(QEMUFLAGS) -kernel $(KERNEL)

.PHONY: clean $(MODULES)
clean:
	$(MAKE) -C boot clean
	rm -rf $(TARGET_DIR)
