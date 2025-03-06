use core::arch::{self, naked_asm};

use crate::{
    dev::{
        gic::{Gic, gic400::GIC_400},
        mmio::MMIO,
    },
    log,
    register::el1::{CNTFRQ_EL0, CNTP_CTL_EL0, CNTP_TVAL_EL0},
};

pub fn enable_timer_interrupt() {
    let frq = CNTFRQ_EL0::read();
    CNTP_TVAL_EL0::write(frq);
    CNTP_CTL_EL0::write(0x1);
    unsafe {
        arch::asm!(" msr DAIFClr, #2");
    }
    loop {}
}

#[inline]
pub fn save_and_disable_interrupts() -> u64 {
    let mut flags: u64;
    unsafe {
        arch::asm!(
            "mrs {0}, daif",
            "msr daifset, #0b0011",
            out(reg) flags,
            options(nostack, nomem)
        );
    }
    flags
}

#[inline]
pub fn restore_interrupts(flags: u64) {
    unsafe {
        arch::asm!(
            "msr daif, {0}",
            in(reg) flags,
            options(nostack, nomem)
        );
    }
}
