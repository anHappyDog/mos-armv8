#![no_std]
#![no_main]
#![feature(naked_functions)]

mod dev;
mod log;
mod mem;
mod register;
mod sync;
mod trap;

use core::arch;

use log::log_init;
use register::el1::CURRENT_EL;

#[unsafe(no_mangle)]
fn _init(_dtb_ptr32: usize, _x1: usize, _x2: usize, _x3: usize) {
    trap::init();
    log_init();
    log!("current el is {:x}\n", CURRENT_EL::read());
    unsafe {
        *(0xdeadbeef as *mut u8) = 1;
    }
    log!("HELLO,aarch64 world!\n");
    loop {}
}

#[naked]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.boot")]
unsafe extern "C" fn _start() {
    unsafe {
        arch::naked_asm!(
            "mrs x4, CPACR_EL1",        // 读取当前CPACR_EL1的值到x0
            "orr x4, x4, #(0x3 << 20)", // 将FPEN字段（位[21:20]）设置为0b11
            "msr CPACR_EL1, x4",        // 写回CPACR_EL1
            "dsb sy",                   // 数据同步屏障，确保写操作完成
            "isb",                      // 指令同步屏障，确保后续指令使用新配置
            "mov x4, #(1 << 10)",
            "msr scr_el3,x4",
            "mov x4,#(0b0101)",
            "msr spsr_el3,x4",
            "adr x4,3f",
            "msr elr_el3,x4",
            "eret",
            "3:",
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
