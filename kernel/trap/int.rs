use core::arch;

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
