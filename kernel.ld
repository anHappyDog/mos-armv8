ENTRY(_start)
HIDDEN(STACK_SZ = 0x4000);
HIDDEN(HEAP_SZ=0x10000);
SECTIONS
{
    /* Starts at LOADER_ADDR. */
    /* For AArch64, use . = 0x80000; */
    . = 0x80000;
    __start = .;
    __text_start = .;
    .text :
    {
        KEEP(*(.text.boot))
        *(.text .text.*)
    }
    . = ALIGN(4096); /* align to page size */
    __text_end = .;

    __rodata_start = .;
    .rodata :
    {
        *(.rodata .rodata.*)
    }
    . = ALIGN(4096); /* align to page size */
    __rodata_end = .;

    __data_start = .;
    .data :
    {
        *(.data .data.*)
    }
    . = ALIGN(4096); /* align to page size */
    __data_end = .;

    __stack_end = .;
    .stack : {
        . += STACK_SZ;
    }
    __stack_top = .;
    __heap_begin = .;
    .heap : {
        . += HEAP_SZ;
    }
    __heap_end  = .;
    __heap_size = __heap_end - __heap_begin;
    __bss_start = .;
    .bss :
    {
        bss = .;
        *(.bss .bss.*)
    }
    . = ALIGN(4096); /* align to page size */
    __bss_end = .;
    __bss_size = __bss_end - __bss_start;
    __end = .;
}