use core::fmt::{self, Write};

use crate::{
    dev::uart::{
        Uart,
        pl011::{Pl011, UART0_BASE},
    },
    sync::spin::Spinlock,
};

static PL011: Pl011 = Pl011::new(UART0_BASE);

struct Stdout;

impl fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            PL011.putc(c as u32);
        }
        Ok(())
    }
}

static STDOUT: Spinlock<Stdout> = Spinlock::new(Stdout);

pub fn _print(color_code: &str, args: fmt::Arguments) {
    let mut stdout = STDOUT.lock();
    stdout.write_str(color_code).unwrap();
    stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! log {
        ($($arg:tt)*) => {
            $crate::log::_print("\x1b[32m",format_args!($($arg)*));
        };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::log::_print("\x1b[33m",format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! err {
    ($($arg:tt)*) => {
        $crate::log::_print("\x1b[31m",format_args!($($arg)*));
    };
}

macro_rules! print {
    ($($arg:tt)*) => {
        $crate::log::_print("\x1b[0m",format_args!($($arg)*));
    };
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    print!("{:?}",_info);
    loop {}
}

pub fn log_init() {
    let _ = PL011.init();
}
