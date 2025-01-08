#include <fdt.h>
#include <printk.h>

volatile unsigned int __attribute__((aligned(16)))
mbox[9] = {9 * 4, 0, 0x38002, 12, 8, 2, 3000000, 0, 0};

#ifdef AARCH64
// arguments for AArch64
void kernel_main(uint64_t dtb_ptr32, uint64_t x1, uint64_t x2, uint64_t x3)
#else
// arguments for AArch32
void kernel_main(uint32_t r0, uint32_t r1, uint32_t atags)
#endif
{
	uint32_t base = 0, size = 0;
	query_mem_base_size(dtb_ptr32, &base, &size);
	printk("hello,mos in aarch64, the mem base is %08x, size is %08x\n", base, size);
	while (1) {
		;
	}
}