#ifndef _TYPES_H_
#define _TYPES_H_

#include <stddef.h>
#include <stdint.h>

typedef unsigned char uchar;
typedef unsigned short ushort;
typedef unsigned int uint;
typedef unsigned long ulong;

#define MIN(_a, _b)                                                                                \
	({                                                                                         \
		typeof(_a) __a = (_a);                                                             \
		typeof(_b) __b = (_b);                                                             \
		__a <= __b ? __a : __b;                                                            \
	})

/* Rounding; only works for n = power of two */
#define ROUND(a, n) (((((u_long)(a)) + (n)-1)) & ~((n)-1))
#define ROUNDDOWN(a, n) (((u_long)(a)) & ~((n)-1))

#endif /* !_INC_TYPES_H_ */
