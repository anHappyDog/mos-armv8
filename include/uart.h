#ifndef __UART_H__
#define __UART_H__
#include <mmio.h>

void uart_putc(unsigned char c);
unsigned char uart_getc();
void uart_puts(const char *str);
void uart_init(int raspi);

#endif //__UART_H__