use crate::register::el1::VBAR_EL1;
use core::arch;

pub mod int;

#[repr(C)]
struct Trapframe {

    regs: [usize; 31],
    esr_el1: usize,
    far_el1: usize,
    spsr_el1: usize,
    elr_el1: usize,
    mpidr_el1: usize,
}

pub fn init() {
    VBAR_EL1::write(trap_vector as u64);
}

#[naked]
#[unsafe(link_section = ".text.trap_vector")]
unsafe extern "C" fn trap_vector() {
    unsafe {
        arch::naked_asm!("");
    }
}
