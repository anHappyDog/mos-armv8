pub mod gic400;

pub trait Gic {
    /// 初始化GIC硬件
    fn init(&mut self);

    /// 启用特定中断
    /// - `irq`: 中断号（如SGI 0-15, PPI 16-31, SPI 32-1019）
    fn enable_interrupt(&mut self, irq: u32);

    /// 禁用特定中断
    fn disable_interrupt(&mut self, irq: u32);

    /// 设置中断优先级
    /// - `irq`: 中断号
    /// - `priority`: 优先级值（通常0-255，0最高）
    fn set_priority(&mut self, irq: u32, priority: u8);

    /// 设置中断分组
    /// - `irq`: 中断号
    /// - `group`: 分组（0 = 安全, 1 = 非安全）
    fn set_group(&mut self, irq: u32, group: u8);

    /// 设置中断目标CPU（仅适用于SPI）
    /// - `irq`: 中断号
    /// - `target`: CPU位图（如0x1表示CPU0）
    fn set_target(&mut self, irq: u32, target: u8);

    /// 获取当前激活的中断ID
    fn get_interrupt_id(&self) -> u32;

    /// 标记中断处理结束
    /// - `irq`: 中断号
    fn end_interrupt(&mut self, irq: u32);

    /// 接收中断 : pending -> active
    fn accept_interrupt(&mut self) -> u32;

    /// 设置优先级掩码
    /// - `priority`: 最低优先级阈值
    fn set_priority_mask(&mut self, priority: u8);

    /// 检查中断是否挂起
    fn is_pending(&self, irq: u32) -> bool;

    /// 触发软件中断（SGI）
    /// - `irq`: SGI中断号（0-15）
    /// - `target`: 目标CPU位图
    fn send_sgi(&mut self, irq: u32, target: u8);
}
