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

#ifndef __FDT_H_
#define __FDT_H_

#include <types.h>

#define FDT_MAGIC 0xd00dfeed
#define FDT_VERSION 17

struct fdt_header {
	uint magic;
	uint totalsize;
	uint off_dt_struct;
	uint off_dt_strings;
	uint off_mem_rsvmap;
	uint version;
	uint last_comp_version; /* <= 17 */
	uint boot_cpuid_phys;
	uint size_dt_strings;
	uint size_dt_struct;
};

#define FDT_BEGIN_NODE 1
#define FDT_END_NODE 2
#define FDT_PROP 3
#define FDT_NOP 4
#define FDT_END 9

struct fdt_scan_node {
	const struct fdt_scan_node *parent;
	const char *name;
	int address_cells;
	int size_cells;
};

struct fdt_scan_prop {
	const struct fdt_scan_node *node;
	const char *name;
	uint *value;
	int len; // in bytes of value
};

struct fdt_cb {
	void (*open)(const struct fdt_scan_node *node, void *extra);
	void (*prop)(const struct fdt_scan_prop *prop, void *extra);
	void (*done)(const struct fdt_scan_node *node, void *extra); // last property was seen
	int (*close)(const struct fdt_scan_node *node,
		     void *extra); // -1 => delete the node + children
	void *extra;
};

void fdt_scan(uint fdt, const struct fdt_cb *cb);

// Extract fields
const uint *fdt_get_address(const struct fdt_scan_node *node, const uint *base, uint *value);
const uint *fdt_get_size(const struct fdt_scan_node *node, const uint *base, uint *value);

void query_mem_base_size(uint fdt, uint *base, uint *size);

#endif