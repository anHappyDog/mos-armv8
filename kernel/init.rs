#![no_std]
#![no_main]
#![feature(naked_functions)]

mod dev;
mod log;
mod mem;
mod sync;
mod trap;
mod register;

use core::arch;

use log::log_init;

#[unsafe(no_mangle)]
fn _init(_dtb_ptr32 : usize,_x1 : usize,_x2 : usize,_x3 : usize) {
    log_init();
    log!("HELLO,aarch64 world!\n");
    loop {}
}

#[naked]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.boot")]
unsafe extern "C" fn _start() {
    unsafe {
        arch::naked_asm!(
            "
            mrs     x4,mpidr_el1
            and     x4,x4,#3
            cbz     x4,1f
        0:
            wfe
            b       0b
        1:
            ldr     x4,=__stack_top
            mov     sp,x4
            ldr     x4,=__bss_start
            ldr     x5,=__bss_size
        2:
            cbz     w5,_init
            str     xzr,[x4],#8
            sub     w5,w5,#1
            cbnz    w5,2b
            
            b _init
        "
        );
    }
}
