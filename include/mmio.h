#ifndef __IO_H__
#define __IO_H__

// this file comes from https://wiki.osdev.org/Raspberry_Pi_Bare_Bones
// for more information please go to this web.
#include <types.h>

// PI 4
#define MMIO_BASE 0xFE000000

enum {
	// The offsets for reach register.
	GPIO_BASE = 0x200000,

	// Controls actuation of pull up/down to ALL GPIO pins.
	GPPUD = (GPIO_BASE + 0x94),

	// Controls actuation of pull up/down for specific GPIO pin.
	GPPUDCLK0 = (GPIO_BASE + 0x98),

	// The base address for UART.
	UART0_BASE =
	    (GPIO_BASE +
	     0x1000), // for raspi4 0xFE201000, raspi2 & 3 0x3F201000, and 0x20201000 for raspi1

	// The offsets for reach register for the UART.
	UART0_DR = (UART0_BASE + 0x00),
	UART0_RSRECR = (UART0_BASE + 0x04),
	UART0_FR = (UART0_BASE + 0x18),
	UART0_ILPR = (UART0_BASE + 0x20),
	UART0_IBRD = (UART0_BASE + 0x24),
	UART0_FBRD = (UART0_BASE + 0x28),
	UART0_LCRH = (UART0_BASE + 0x2C),
	UART0_CR = (UART0_BASE + 0x30),
	UART0_IFLS = (UART0_BASE + 0x34),
	UART0_IMSC = (UART0_BASE + 0x38),
	UART0_RIS = (UART0_BASE + 0x3C),
	UART0_MIS = (UART0_BASE + 0x40),
	UART0_ICR = (UART0_BASE + 0x44),
	UART0_DMACR = (UART0_BASE + 0x48),
	UART0_ITCR = (UART0_BASE + 0x80),
	UART0_ITIP = (UART0_BASE + 0x84),
	UART0_ITOP = (UART0_BASE + 0x88),
	UART0_TDR = (UART0_BASE + 0x8C),

	// The offsets for Mailbox registers
	MBOX_BASE = 0xB880,
	MBOX_READ = (MBOX_BASE + 0x00),
	MBOX_STATUS = (MBOX_BASE + 0x18),
	MBOX_WRITE = (MBOX_BASE + 0x20)
};

// A Mailbox message with set clock rate of PL011 to 3MHz tag
extern volatile unsigned int __attribute__((aligned(16)));

static inline void mmio_write(uint32_t reg, uint32_t data) {
	*(volatile uint32_t *)(MMIO_BASE + reg) = data;
}

static inline uint32_t mmio_read(uint32_t reg) {
	return *(volatile uint32_t *)(MMIO_BASE + reg);
}

static inline void delay(int32_t count) {
	asm volatile("__delay_%=: subs %[count], %[count], #1; bne __delay_%=\n"
		     : "=r"(count)
		     : [count] "0"(count)
		     : "cc");
}

#endif // __IO_H__