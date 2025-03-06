use int::enable_timer_interrupt;

use crate::{
    dev::{
        gic::{Gic, gic400::GIC_400},
        mmio::MMIO,
    },
    log,
    register::el1::{CNTFRQ_EL0, CNTP_TVAL_EL0, MPIDR_EL1, VBAR_EL1},
};
use core::arch;

pub mod int;

#[repr(C)]
struct Trapframe {
    regs: [usize; 31],
    sp: usize,
    esr_el1: usize,
    far_el1: usize,
    spsr_el1: usize,
    elr_el1: usize,
}

pub fn init() {
    VBAR_EL1::write(trap_vector as u64);
    let mut gic_400 = GIC_400.lock();
    gic_400.init();
    gic_400.enable_interrupt(30);
    gic_400.set_priority(30, 0x20);
    log!("mipdr_el1 is {:x}\n", MPIDR_EL1::read());
    gic_400.set_target(30, 1 << (MPIDR_EL1::read() & 0x3));
    gic_400.set_group(30, 1);
    gic_400.set_priority_mask(0xff);
    drop(gic_400);
    enable_timer_interrupt();
}

#[unsafe(no_mangle)]
extern "C" fn cur_syn_exception_handler(context: &Trapframe) {
    panic!("cur syn exception happened.\n");
}

#[unsafe(no_mangle)]
extern "C" fn cur_fiq_handler(context: &Trapframe) {
    panic!("cur irq happened.\n");
}

#[unsafe(no_mangle)]
extern "C" fn cur_irq_handler(context: &Trapframe) {
    let mut gic = GIC_400.lock();
    let iar = gic.accept_interrupt(); // 确认中断
    log!("IRQ: {}\n", iar & 0x3ff);
    let frq = CNTFRQ_EL0::read();
    CNTP_TVAL_EL0::write(frq);
    gic.end_interrupt(iar & 0x3ffu32);
}

#[naked]
#[unsafe(link_section = ".text.trap_vector")]
unsafe extern "C" fn trap_vector() {
    unsafe {
        arch::naked_asm!(
            // synchronous exception with sp_el0
            ".align 8",
            "stp x0, x1,   [sp, #-288]", // regs[0] = x0, regs[1] = x1
            "adr x0, 3f",
            "b 1f",
            "3:",
            "mov x0,sp",
            "bl cur_syn_exception_handler ",
            "b 2f",
            // irq with sp_el0
            ".align 7",
            "stp x0, x1,   [sp, #-288]", // regs[0] = x0, regs[1] = x1
            "adr x0, 3f",
            "b 1f",
            "3:",
            "mov x0,sp",
            "bl cur_irq_handler",
            "b 2f",
            // fiq with sp_el0
            ".align 7",
            "stp x0, x1,   [sp, #-288]", // regs[0] = x0, regs[1] = x1
            "adr x0, 3f",
            "b 1f",
            "3:",
            "mov x0,sp",
            "bl cur_fiq_handler",
            "b 2f",
            // synchronous exception with sp_elx
            ".align 8",
            "stp x0, x1,   [sp, #-288]", // regs[0] = x0, regs[1] = x1
            "adr x0, 3f",
            "b 1f",
            "3:",
            "mov x0,sp",
            "bl cur_syn_exception_handler ",
            "b 2f",
            // irq with sp_elx
            ".align 7",
            "stp x0, x1,   [sp, #-288]", // regs[0] = x0, regs[1] = x1
            "adr x0, 3f",
            "b 1f",
            "3:",
            "mov x0,sp",
            "bl cur_irq_handler",
            "b 2f",
            // fiq with sp_elx
            ".align 7",
            "stp x0, x1,   [sp, #-288]", // regs[0] = x0, regs[1] = x1
            "adr x0, 3f",
            "b 1f",
            "3:",
            "mov x0,sp",
            "bl cur_fiq_handler",
            "b 2f",
            // lower exception level using aarch64
            // synchronous exceptions
            ".align 8",
            "",
            //irq with lower exception level
            ".align 7",
            "",
            // fiq with lower exception level
            ".align 7",
            "",
            // lower exception in aarch32
            ".align 8",
            "",
            // irq
            ".align 7",
            "",
            ".align 7",
            "",
            // save context
            "1:",
            "stp x2, x3,   [sp, #-272]", // regs[2], regs[3]
            "stp x4, x5,   [sp, #-256]", // regs[4], regs[5]
            "stp x6, x7,   [sp, #-240]", // regs[6], regs[7]
            "stp x8, x9,   [sp, #-224]", // regs[8], regs[9]
            "stp x10, x11, [sp, #-208]", // regs[10], regs[11]
            "stp x12, x13, [sp, #-192]", // regs[12], regs[13]
            "stp x14, x15, [sp, #-176]", // regs[14], regs[15]
            "stp x16, x17, [sp, #-160]", // regs[16], regs[17]
            "stp x18, x19, [sp, #-144]", // regs[18], regs[19]
            "stp x20, x21, [sp, #-128]", // regs[20], regs[21]
            "stp x22, x23, [sp, #-112]", // regs[22], regs[23]
            "stp x24, x25, [sp, #-96]",  // regs[24], regs[25]
            "stp x26, x27, [sp, #-80]",  // regs[26], regs[27]
            "stp x28, x29, [sp, #-64]",  // regs[28], regs[29]
            "str x30,      [sp, #-48]",  // regs[30]
            // 存储当前 sp 值
            // "str sp,       [sp, #-40]", // sp 字段
            // 存储特殊寄存器
            "mrs x1, esr_el1",
            "str x1,       [sp, #-32]", // esr_el1
            "mrs x1, far_el1",
            "str x1,       [sp, #-24]", // far_el1
            "mrs x1, spsr_el1",
            "str x1,       [sp, #-16]", // spsr_el1
            "mrs x1, elr_el1",
            "str x1,       [sp, #-8]", // elr_el1
            // 更新 sp（减小 288 字节）
            "sub sp, sp, #288",
            "ret x0",
            //restore contex
            "2:",
            // 恢复特殊寄存器
            "ldr x0,       [sp, #256]", // 加载 esr_el1
            "msr esr_el1, x0",
            "ldr x0,       [sp, #264]", // 加载 far_el1
            "msr far_el1, x0",
            "ldr x0,       [sp, #272]", // 加载 spsr_el1
            "msr spsr_el1, x0",
            "ldr x0,       [sp, #280]", // 加载 elr_el1
            "msr elr_el1, x0",
            "ldp x0, x1,   [sp, #0]",   // regs[0], regs[1]
            "ldp x2, x3,   [sp, #16]",  // regs[2], regs[3]
            "ldp x4, x5,   [sp, #32]",  // regs[4], regs[5]
            "ldp x6, x7,   [sp, #48]",  // regs[6], regs[7]
            "ldp x8, x9,   [sp, #64]",  // regs[8], regs[9]
            "ldp x10, x11, [sp, #80]",  // regs[10], regs[11]
            "ldp x12, x13, [sp, #96]",  // regs[12], regs[13]
            "ldp x14, x15, [sp, #112]", // regs[14], regs[15]
            "ldp x16, x17, [sp, #128]", // regs[16], regs[17]
            "ldp x18, x19, [sp, #144]", // regs[18], regs[19]
            "ldp x20, x21, [sp, #160]", // regs[20], regs[21]
            "ldp x22, x23, [sp, #176]", // regs[22], regs[23]
            "ldp x24, x25, [sp, #192]", // regs[24], regs[25]
            "ldp x26, x27, [sp, #208]", // regs[26], regs[27]
            "ldp x28, x29, [sp, #224]", // regs[28], regs[29]
            "ldr x30,      [sp, #240]", // regs[30]
            "eret",
            options(raw)
        );
    }
}
