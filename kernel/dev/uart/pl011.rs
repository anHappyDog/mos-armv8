use crate::dev::{
    mailbox::VC_MBOX,
    mmio::{self, GPIO_BASE, GPPUD, GPPUDCLK0, MMIO},
};

use super::Uart;
pub const UART0_BASE: usize = GPIO_BASE + 0x1000;
const UART0_DR: usize = 0x00;
const UART0_RSRECR: usize = 0x04;
const UART0_FR: usize = 0x18;
const UART0_ILPR: usize = 0x20;
const UART0_IBRD: usize = 0x24;
const UART0_FBRD: usize = 0x28;
const UART0_LCRH: usize = 0x2C;
const UART0_CR: usize = 0x30;
const UART0_IFLS: usize = 0x34;
const UART0_IMSC: usize = 0x38;
const UART0_RIS: usize = 0x3C;
const UART0_MIS: usize = 0x40;
const UART0_ICR: usize = 0x44;
const UART0_DMACR: usize = 0x48;
const UART0_ITCR: usize = 0x80;
const UART0_ITIP: usize = 0x84;
const UART0_ITOP: usize = 0x88;
const UART0_TDR: usize = 0x8C;

#[repr(C, align(16))]
struct MboxBuffer {
    inner: [u32; 9],
}

static MBOX_BUFFER: MboxBuffer = MboxBuffer {
    inner: [9 * 4, 0, 0x38002, 12, 8, 2, 3000000, 0, 0],
};

pub struct Pl011 {
    base: usize,
}

impl Pl011 {
    pub const fn new(base: usize) -> Self {
        Self { base }
    }
}

impl Uart for Pl011 {
    fn init(&self) -> Result<(), &str> {
        // 1. 禁用 UART0
        MMIO::write::<u32>(self.base + UART0_CR, 0x00000000);

        // 2. 配置 GPIO 引脚 14 和 15 (TXD0 和 RXD0)
        // 禁用所有 GPIO 引脚的上下拉
        MMIO::write::<u32>(GPPUD, 0x00000000);
        mmio::delay(150);

        // 为引脚 14 和 15 禁用上下拉
        MMIO::write::<u32>(GPPUDCLK0, (1 << 14) | (1 << 15));
        mmio::delay(150);

        MMIO::write::<u32>(GPPUDCLK0, 0x00000000);

        MMIO::write::<u32>(self.base + UART0_ICR, 0x7FF);

        VC_MBOX.send(&MBOX_BUFFER.inner, 8);
        while VC_MBOX.empty() || !VC_MBOX.hold(&MBOX_BUFFER.inner, 8) {}

        if MBOX_BUFFER.inner[1] != 0x80000000 {
            return Err("Failed to set UART clock via Mailbox");
        }

        // Divider = 3000000 / (16 * 115200) = 1.627 ≈ 1
        MMIO::write::<u32>(self.base + UART0_IBRD, 1);
        // Fractional = (.627 * 64) + 0.5 = 40.6 ≈ 40
        MMIO::write::<u32>(self.base + UART0_FBRD, 40);

        MMIO::write::<u32>(self.base + UART0_LCRH, (1 << 4) | (1 << 5) | (1 << 6));

        MMIO::write::<u32>(
            self.base + UART0_IMSC,
            (1 << 1) | (1 << 4) | (1 << 5) | (1 << 6) | (1 << 7) | (1 << 8) | (1 << 9) | (1 << 10),
        );

        MMIO::write::<u32>(self.base + UART0_CR, (1 << 0) | (1 << 8) | (1 << 9));

        Ok(())
    }

    fn putc(&self, c: u32) {
        while MMIO::read::<u32>(self.base + UART0_FR) & (1 << 5) != 0 {}
        MMIO::write::<u32>(self.base + UART0_DR, c);
    }

    fn getc(&self) -> Option<u32> {
        if MMIO::read::<u32>(self.base + UART0_FR) & (1 << 4) != 0 {
            return None;
        }
        Some(MMIO::read::<u32>(self.base + UART0_DR))
    }
}
