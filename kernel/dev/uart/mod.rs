pub mod pl011;

pub trait Uart {
    fn init(&self) -> Result<(), &str>;
    fn putc(&self, c: u32);
    fn getc(&self) -> Option<u32>;
}
