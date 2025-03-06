use crate::{dev::mmio::MMIO, log, sync::spin::Spinlock};

use super::Gic;

const GICD_CTLR: u32 = 0x0;
const GICD_TYPER: u32 = 0x4;
const GICD_IIDR: u32 = 0x8;
const GICD_IGROUP_N: u32 = 0x80;
const GICD_ISENABLER_N: u32 = 0x100;
const GICD_ICENABLER_N: u32 = 0x180;
const GICD_ISPENDR_N: u32 = 0x200;
const GICD_ICPENDR_N: u32 = 0x280;
const GICD_ISACTIVER_N: u32 = 0x300;
const GICD_ICACTIVER_N: u32 = 0x380;
const GICD_IPRIORITYR_N: u32 = 0x400;
const GICD_ITARGETSR_N: u32 = 0x800;
const GICD_ICFGR_N: u32 = 0xc00;
const GICD_NSACR_N: u32 = 0xe00;
const GICD_SGIR: u32 = 0xf00;
const GICD_CPENDSGIR_N: u32 = 0xf10;
const GICD_SPENDSGIR_N: u32 = 0xf20;

pub static GIC_400: Spinlock<Gic400> = Spinlock::new(Gic400::new(0xff841000, 0xff842000));

pub struct Gic400 {
    pub gicd_base: usize,
    pub gicc_base: usize,
}

impl Gic400 {
    pub const fn new(gicd_base: usize, gicc_base: usize) -> Self {
        Self {
            gicc_base,
            gicd_base,
        }
    }
}

impl Gic for Gic400 {
    fn init(&mut self) {
        // 初始化GICD
        let mut ctlr = MMIO::read::<u32>(self.gicd_base + GICD_CTLR as usize); // GICD_CTLR
        ctlr |= 0x3; // 启用Group 0 and 1
        MMIO::write(self.gicd_base + GICD_CTLR as usize, ctlr);

        // 初始化GICC
        MMIO::write(self.gicc_base, 7u32); // GICC_CTLR，启用Group 1与1
        MMIO::write(self.gicc_base + 0x4, 0xFFu32); // GICC_PMR，优先级掩码
    }

    fn enable_interrupt(&mut self, irq: u32) {
        let reg_idx = irq / 32;
        let bit_idx = irq % 32;
        let offset = GICD_ISENABLER_N as usize + (reg_idx as usize) * 4; // GICD_ISENABLERn
        let mut value = MMIO::read::<u32>(self.gicd_base + offset);
        value |= 1 << bit_idx;
        MMIO::write(self.gicd_base + offset, value);
    }

    fn disable_interrupt(&mut self, irq: u32) {
        let offset = GICD_ICENABLER_N + (irq / 32) * 4; // GICD_ICENABLER<n>
        let bit = 1 << (irq % 32);
        MMIO::write(self.gicd_base + offset as usize, bit as u32);
    }

    fn set_priority(&mut self, irq: u32, priority: u8) {
        let offset = GICD_IPRIORITYR_N + (irq / 4) * 4; // GICD_IPRIORITYR<n>
        let mut former: u32 = MMIO::read(self.gicd_base + offset as usize);
        former &= !(0xFF << ((irq % 4) * 8));
        former |= (priority as u32) << ((irq % 4) * 8);
        MMIO::write(self.gicd_base + offset as usize, former);
    }

    fn accept_interrupt(&mut self) -> u32 {
        MMIO::read(self.gicc_base + 0xc)
    }

    fn set_group(&mut self, irq: u32, group: u8) {
        let offset = GICD_IGROUP_N + (irq / 32) * 4; // GICD_IGROUPR<n>
        let mut value = MMIO::read::<u32>(self.gicd_base + offset as usize);
        value &= !(1 << (irq % 32));
        value |= (u32::from(group) & 0x1) << (irq % 32);
        MMIO::write(self.gicd_base + offset as usize, value);
    }

    fn set_target(&mut self, irq: u32, target: u8) {
        let offset = GICD_ITARGETSR_N + (irq / 4) * 4; // GICD_ITARGETSR<n>
        let shift = (irq % 4) * 8; // 每个 IRQ 占 8 位
        let current_value: u32 = MMIO::read(self.gicd_base + offset as usize); // 读取当前值
        let mask = 0xFF << shift; // 目标 8 位字段的掩码
        let new_value = (current_value & !mask) | ((target as u32) << shift); // 更新目标字段
        MMIO::write(self.gicd_base + offset as usize, new_value);
    }

    fn get_interrupt_id(&self) -> u32 {
        MMIO::read(self.gicc_base + 0xC) // GICC_IAR
    }

    fn end_interrupt(&mut self, irq: u32) {
        MMIO::write(self.gicc_base + 0x10, irq); // GICC_EOIR
    }

    fn set_priority_mask(&mut self, priority: u8) {
        MMIO::write(self.gicc_base + 0x4, priority as u32); // GICC_PMR
    }

    fn is_pending(&self, irq: u32) -> bool {
        let offset = GICD_ISPENDR_N + (irq / 32) * 4; // GICD_ISPENDR<n>
        let bit = 1 << (irq % 32);
        let status = MMIO::read::<u32>(self.gicd_base + offset as usize);
        (status & bit) != 0
    }

    fn send_sgi(&mut self, irq: u32, target: u8) {
        let sgi = (target as u32) << 16 | (irq & 0xF);
        MMIO::write(self.gicd_base + GICD_SGIR as usize, sgi); // GICD_SGIR
    }
}
