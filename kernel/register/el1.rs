macro_rules! define_register {
    ($name:ident, $reg:tt) => {
        #[allow(non_camel_case_types)]
        pub struct $name;

        impl $name {
            #[inline]
            pub fn read() -> u64 {
                let value: u64;
                unsafe {
                    core::arch::asm!(concat!("mrs {}, ", $reg), out(reg) value);
                }
                value
            }
        }
    };
    ($name:ident, $reg:tt, read_write) => {
        pub struct $name;

        impl $name {
            #[inline]
            pub fn read() -> u64 {
                let value: u64;
                unsafe {
                    core::arch::asm!(concat!("mrs {}, ", $reg), out(reg) value);
                }
                value
            }

            #[inline]
            pub fn write(value: u64) {
                unsafe {
                    core::arch::asm!(concat!("msr ", $reg, ", {}"), in(reg) value);
                }
            }
        }
    };
}

// 异常相关寄存器
define_register!(VBAR_EL1, "VBAR_EL1", read_write);
define_register!(SPSR_EL1, "SPSR_EL1", read_write);
define_register!(ELR_EL1, "ELR_EL1", read_write);
define_register!(ESR_EL1, "ESR_EL1");
define_register!(FAR_EL1, "FAR_EL1", read_write);

// 线程和核心管理
define_register!(TPIDR_EL1, "TPIDR_EL1", read_write);
define_register!(MPIDR_EL1, "MPIDR_EL1");

// 系统控制
define_register!(SCTLR_EL1, "SCTLR_EL1", read_write);
define_register!(TCR_EL1, "TCR_EL1", read_write);
define_register!(TTBR0_EL1, "TTBR0_EL1", read_write);
define_register!(TTBR1_EL1, "TTBR1_EL1", read_write);
define_register!(CURRENT_EL, "CurrentEL");
define_register!(CNTFRQ_EL0, "CNTFRQ_EL0");
define_register!(CNTP_TVAL_EL0, "CNTP_TVAL_EL0", read_write);
define_register!(CNTP_CTL_EL0, "CNTP_CTL_EL0", read_write);
