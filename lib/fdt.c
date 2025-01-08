/* Copyright (c) 2013, The Regents of the University of California (Regents).
All Rights Reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:
1. Redistributions of source code must retain the above copyright
   notice, this list of conditions and the following disclaimer.
2. Redistributions in binary form must reproduce the above copyright
   notice, this list of conditions and the following disclaimer in the
   documentation and/or other materials provided with the distribution.
3. Neither the name of the Regents nor the
   names of its contributors may be used to endorse or promote products
   derived from this software without specific prior written permission.

IN NO EVENT SHALL REGENTS BE LIABLE TO ANY PARTY FOR DIRECT, INDIRECT,
SPECIAL, INCIDENTAL, OR CONSEQUENTIAL DAMAGES, INCLUDING LOST PROFITS, ARISING
OUT OF THE USE OF THIS SOFTWARE AND ITS DOCUMENTATION, EVEN IF REGENTS HAS
BEEN ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

REGENTS SPECIFICALLY DISCLAIMS ANY WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
PURPOSE. THE SOFTWARE AND ACCOMPANYING DOCUMENTATION, IF ANY, PROVIDED
HEREUNDER IS PROVIDED "AS IS". REGENTS HAS NO OBLIGATION TO PROVIDE
MAINTENANCE, SUPPORT, UPDATES, ENHANCEMENTS, OR MODIFICATIONS. */

#include <fdt.h>
#include <string.h>
#include <types.h>

static inline uint bswap(uint x) {
	uint y = (x & 0x00FF00FF) << 8 | (x & 0xFF00FF00) >> 8;
	uint z = (y & 0x0000FFFF) << 16 | (y & 0xFFFF0000) >> 16;
	return z;
}

static uint *fdt_scan_helper(uint *lex, const char *strings, struct fdt_scan_node *node,
			     const struct fdt_cb *cb) {
	struct fdt_scan_node child;
	struct fdt_scan_prop prop;
	int last = 0;

	child.parent = node;
	// these are the default cell counts, as per the FDT spec
	child.address_cells = 2;
	child.size_cells = 1;
	prop.node = node;

	while (1) {
		switch (bswap(lex[0])) {
		case FDT_NOP: {
			lex += 1;
			break;
		}
		case FDT_PROP: {
			prop.name = strings + bswap(lex[2]);
			prop.len = bswap(lex[1]);
			prop.value = lex + 3;
			if (node && !strcmp(prop.name, "#address-cells")) {
				node->address_cells = bswap(lex[3]);
			}
			if (node && !strcmp(prop.name, "#size-cells")) {
				node->size_cells = bswap(lex[3]);
			}
			lex += 3 + (prop.len + 3) / 4;
			cb->prop(&prop, cb->extra);
			break;
		}
		case FDT_BEGIN_NODE: {
			uint *lex_next;
			if (!last && node && cb->done) {
				cb->done(node, cb->extra);
			}
			last = 1;
			child.name = (const char *)(lex + 1);
			if (cb->open) {
				cb->open(&child, cb->extra);
			}
			lex_next =
			    fdt_scan_helper(lex + 2 + strlen(child.name) / 4, strings, &child, cb);
			if (cb->close && cb->close(&child, cb->extra) == -1) {
				while (lex != lex_next) {
					*lex++ = bswap(FDT_NOP);
				}
			}
			lex = lex_next;
			break;
		}
		case FDT_END_NODE: {
			if (!last && node && cb->done) {
				cb->done(node, cb->extra);
			}
			return lex + 1;
		}
		default: { // FDT_END
			if (!last && node && cb->done) {
				cb->done(node, cb->extra);
			}
			return lex;
		}
		}
	}
}

void fdt_scan(uint fdt, const struct fdt_cb *cb) {
	struct fdt_header *header = (struct fdt_header *)((size_t)fdt);

	// Only process FDT that we understand
	if (bswap(header->magic) != FDT_MAGIC || bswap(header->last_comp_version) > FDT_VERSION) {
		return;
	}

	const char *strings = (const char *)((size_t)(fdt + bswap(header->off_dt_strings)));
	uint *lex = (uint *)((size_t)(fdt + bswap(header->off_dt_struct)));

	fdt_scan_helper(lex, strings, 0, cb);
}

const uint *fdt_get_address(const struct fdt_scan_node *node, const uint *value, uint *result) {
	*result = 0;
	for (int cells = node->address_cells; cells > 0; --cells) {
		*result = ((size_t)*result << 32) + bswap(*value++);
	}
	return value;
}

const uint *fdt_get_size(const struct fdt_scan_node *node, const uint *value, uint *result) {
	*result = 0;
	for (int cells = node->size_cells; cells > 0; --cells) {
		*result = ((size_t)*result << 32) + bswap(*value++);
	}
	return value;
}

//////////////////////////////////////////// MEMORY SCAN /////////////////////////////////////////

static uint mem_size;
static uint mem_base;

struct mem_scan {
	int memory;
	const uint *reg_value;
	int reg_len;
};

static void mem_open(const struct fdt_scan_node *node, void *extra) {
	struct mem_scan *scan = (struct mem_scan *)extra;
	memset(scan, 0, sizeof(*scan));
}

static void mem_prop(const struct fdt_scan_prop *prop, void *extra) {
	struct mem_scan *scan = (struct mem_scan *)extra;
	if (!strcmp(prop->name, "device_type") && !strcmp((const char *)prop->value, "memory")) {
		scan->memory = 1;
	} else if (!strcmp(prop->name, "reg")) {
		scan->reg_value = prop->value;
		scan->reg_len = prop->len;
	}
}

static void mem_done(const struct fdt_scan_node *node, void *extra) {
	struct mem_scan *scan = (struct mem_scan *)extra;
	const uint *value = scan->reg_value;
	const uint *end = value + scan->reg_len / 4;
	size_t self = (size_t)mem_done;

	if (!scan->memory) {
		return;
	}

	while (end - value > 0) {
		uint base, size;
		value = fdt_get_address(node->parent, value, &base);
		value = fdt_get_size(node->parent, value, &size);
		if (base <= self && self <= base + size) {
			mem_size = size;
			mem_base = base;
		}
	}
}

void query_mem_base_size(uint fdt, uint *base, uint *size) {
	struct fdt_cb cb;
	struct mem_scan scan;

	memset(&cb, 0, sizeof(cb));
	cb.open = mem_open;
	cb.prop = mem_prop;
	cb.done = mem_done;
	cb.extra = &scan;

	mem_size = 0;
	fdt_scan(fdt, &cb);
	*base = mem_base;
	*size = mem_size;
}