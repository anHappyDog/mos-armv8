
include include.mk

MODULES := boot


$(KERNEL) : $(TARGET_DIR)
	$(MAKE) -C boot CC=$(CC) CFLAGS="$(CFLAGS)"
	$(LD) $(LDFLAGS) -o $(KERNEL) $(wildcard boot/*.o)


$(TARGET_DIR):
	mkdir -p $(TARGET_DIR)

all: $(KERNEL)

run:$(KERNEL)
	$(QEMU) $(QEMUFLAGS) -kernel $(KERNEL)

.PHONY: clean
clean:
	$(MAKE) -C boot clean
	rm -rf $(TARGET_DIR)
