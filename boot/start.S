.section .boot

.global _start

_start:
        mrs        x4, mpidr_el1
        and        x4, x4, #3
        cbz        x4, _init
0:
        wfe
        b            0b

_init:  ldr     x4, =_start
        mov     sp, x4

        ldr     x4, =__bss_start
        ldr     w5, =__bss_size
_bss_clear:
        cbz     w5, kernel_main
        str     xzr, [x4], #8
        sub     w5, w5, #1
        cbnz    w5, _bss_clear

        b       kernel_main