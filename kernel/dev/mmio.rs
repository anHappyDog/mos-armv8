use core::{arch, ptr};

// pi4
pub(super) const MMIO_BASE: usize = 0xFE000000;

pub(super) const GPIO_BASE: usize = MMIO_BASE + 0x200000;

pub(super) const GPPUD: usize = GPIO_BASE + 0x94;

pub(super) const GPPUDCLK0: usize = GPIO_BASE + 0x98;

pub trait MMIOType {}

impl MMIOType for u8 {}

impl MMIOType for u16 {}

impl MMIOType for u32 {}

#[allow(clippy::upper_case_acronyms)]
pub struct MMIO;

impl MMIO {
    pub fn read<T: MMIOType>(addr: usize) -> T {
        unsafe { ptr::read_volatile(addr as *const T) }
    }

    pub fn write<T: MMIOType>(addr: usize, val: T) {
        unsafe {
            ptr::write_volatile(addr as *mut T, val);
        }
    }
}

#[inline]
pub fn delay(count: i32) {
    unsafe {
        arch::asm!(
            "1: subs {0:w}, {0:w}, #1; bne 1b",
            in(reg) count ,
            options(nostack)
        );
    }
}
