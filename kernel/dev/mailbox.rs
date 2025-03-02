use super::mmio::{MMIO, MMIO_BASE};

pub const MBOX_BASE: usize = MMIO_BASE + 0xB880;
const MBOX_READ: usize = 0x00;
const MBOX_STATUS: usize = 0x18;
const MBOX_WRITE: usize = 0x20;

const MAILBOX_EMPTY: u32 = 0x4000_0000;
const MAILBOX_FULL: u32 = 0x8000_0000;

pub(super) struct Mbox {
    base: usize,
}

impl Mbox {
    const fn new(base: usize) -> Self {
        Self { base }
    }
}

impl Mbox {
    pub fn send(&self, buffer: &[u32], channel: u32) {
        let addr = buffer.as_ptr() as u32 & !0xF;
        let value = addr | (channel & 0xF);
        while self.status() & MAILBOX_FULL != 0 {}
        self.write(value);
    }

    pub fn status(&self) -> u32 {
        MMIO::read(self.base + MBOX_STATUS)
    }

    pub fn empty(&self) -> bool {
        self.status() & MAILBOX_EMPTY != 0
    }

    pub fn hold(&self, buffer: &[u32], channel: u32) -> bool {
        let addr = buffer.as_ptr() as u32 & !0xF;
        let value = addr | (channel & 0xF);
        MMIO::read::<u32>(self.base + MBOX_READ) == value
    }

    pub fn write(&self, val: u32) {
        MMIO::write(self.base + MBOX_WRITE, val);
    }
}

pub(super) static VC_MBOX: Mbox = Mbox::new(MBOX_BASE);
