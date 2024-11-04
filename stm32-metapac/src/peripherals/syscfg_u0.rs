#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "SYSCFG register block"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syscfg {
    ptr: *mut u8,
}
unsafe impl Send for Syscfg {}
unsafe impl Sync for Syscfg {}
impl Syscfg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SYSCFG configuration register 1"]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::Cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "SYSCFG configuration register 2"]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "SYSCFG SRAM2 control and status register"]
    #[inline(always)]
    pub const fn scsr(self) -> crate::common::Reg<regs::Scsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "SYSCFG SRAM2 key register"]
    #[inline(always)]
    pub const fn skr(self) -> crate::common::Reg<regs::Skr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "SYSCFG TSC comparator register"]
    #[inline(always)]
    pub const fn tsccr(self) -> crate::common::Reg<regs::Tsccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "SYSCFG interrupt line 0 status register"]
    #[inline(always)]
    pub const fn itline0(self) -> crate::common::Reg<regs::Itline0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "SYSCFG interrupt line 1 status register"]
    #[inline(always)]
    pub const fn itline1(self) -> crate::common::Reg<regs::Itline1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "SYSCFG interrupt line 2 status register"]
    #[inline(always)]
    pub const fn itline2(self) -> crate::common::Reg<regs::Itline2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "SYSCFG interrupt line 3 status register"]
    #[inline(always)]
    pub const fn itline3(self) -> crate::common::Reg<regs::Itline3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "SYSCFG interrupt line 4 status register"]
    #[inline(always)]
    pub const fn itline4(self) -> crate::common::Reg<regs::Itline4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "SYSCFG interrupt line 5 status register"]
    #[inline(always)]
    pub const fn itline5(self) -> crate::common::Reg<regs::Itline5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "SYSCFG interrupt line 6 status register"]
    #[inline(always)]
    pub const fn itline6(self) -> crate::common::Reg<regs::Itline6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "SYSCFG interrupt line 7 status register"]
    #[inline(always)]
    pub const fn itline7(self) -> crate::common::Reg<regs::Itline7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "SYSCFG interrupt line 8 status register"]
    #[inline(always)]
    pub const fn itline8(self) -> crate::common::Reg<regs::Itline8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "SYSCFG interrupt line 9 status register"]
    #[inline(always)]
    pub const fn itline9(self) -> crate::common::Reg<regs::Itline9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "SYSCFG interrupt line 10 status register"]
    #[inline(always)]
    pub const fn itline10(self) -> crate::common::Reg<regs::Itline10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "SYSCFG interrupt line 11 status register"]
    #[inline(always)]
    pub const fn itline11(self) -> crate::common::Reg<regs::Itline11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "SYSCFG interrupt line 12 status register"]
    #[inline(always)]
    pub const fn itline12(self) -> crate::common::Reg<regs::Itline12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "SYSCFG interrupt line 13 status register"]
    #[inline(always)]
    pub const fn itline13(self) -> crate::common::Reg<regs::Itline13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[doc = "SYSCFG interrupt line 14 status register"]
    #[inline(always)]
    pub const fn itline14(self) -> crate::common::Reg<regs::Itline14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[doc = "SYSCFG interrupt line 15 status register"]
    #[inline(always)]
    pub const fn itline15(self) -> crate::common::Reg<regs::Itline15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[doc = "SYSCFG interrupt line 16 status register"]
    #[inline(always)]
    pub const fn itline16(self) -> crate::common::Reg<regs::Itline16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "SYSCFG interrupt line 17 status register"]
    #[inline(always)]
    pub const fn itline17(self) -> crate::common::Reg<regs::Itline17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "SYSCFG interrupt line 18 status register"]
    #[inline(always)]
    pub const fn itline18(self) -> crate::common::Reg<regs::Itline18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[doc = "SYSCFG interrupt line 19 status register"]
    #[inline(always)]
    pub const fn itline19(self) -> crate::common::Reg<regs::Itline19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[doc = "SYSCFG interrupt line 20 status register"]
    #[inline(always)]
    pub const fn itline20(self) -> crate::common::Reg<regs::Itline20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[doc = "SYSCFG interrupt line 21 status register"]
    #[inline(always)]
    pub const fn itline21(self) -> crate::common::Reg<regs::Itline21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[doc = "SYSCFG interrupt line 22 status register"]
    #[inline(always)]
    pub const fn itline22(self) -> crate::common::Reg<regs::Itline22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[doc = "SYSCFG interrupt line 23 status register"]
    #[inline(always)]
    pub const fn itline23(self) -> crate::common::Reg<regs::Itline23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[doc = "SYSCFG interrupt line 24 status register"]
    #[inline(always)]
    pub const fn itline24(self) -> crate::common::Reg<regs::Itline24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[doc = "SYSCFG interrupt line 25 status register"]
    #[inline(always)]
    pub const fn itline25(self) -> crate::common::Reg<regs::Itline25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
    }
    #[doc = "SYSCFG interrupt line 26 status register"]
    #[inline(always)]
    pub const fn itline26(self) -> crate::common::Reg<regs::Itline26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe8usize) as _) }
    }
    #[doc = "SYSCFG interrupt line 27 status register"]
    #[inline(always)]
    pub const fn itline27(self) -> crate::common::Reg<regs::Itline27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xecusize) as _) }
    }
    #[doc = "SYSCFG interrupt line 28 status register"]
    #[inline(always)]
    pub const fn itline28(self) -> crate::common::Reg<regs::Itline28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[doc = "SYSCFG interrupt line 29 status register"]
    #[inline(always)]
    pub const fn itline29(self) -> crate::common::Reg<regs::Itline29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[doc = "SYSCFG interrupt line 30 status register"]
    #[inline(always)]
    pub const fn itline30(self) -> crate::common::Reg<regs::Itline30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
    #[doc = "SYSCFG interrupt line 31 status register"]
    #[inline(always)]
    pub const fn itline31(self) -> crate::common::Reg<regs::Itline31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
}
pub mod regs {
    #[doc = "SYSCFG configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr1(pub u32);
    impl Cfgr1 {
        #[doc = "Memory mapping selection bits These bits are set and cleared by software. They control the memory internal mapping at address 0x000010000. After reset these bits take on the value selected by the actual boot mode configuration. Refer to Section12.5: Boot configuration for more details. X0: Main flash memory mapped at 0x000010000"]
        #[inline(always)]
        pub const fn mem_mode(&self) -> super::vals::MemMode {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::MemMode::from_bits(val as u8)
        }
        #[doc = "Memory mapping selection bits These bits are set and cleared by software. They control the memory internal mapping at address 0x000010000. After reset these bits take on the value selected by the actual boot mode configuration. Refer to Section12.5: Boot configuration for more details. X0: Main flash memory mapped at 0x000010000"]
        #[inline(always)]
        pub fn set_mem_mode(&mut self, val: super::vals::MemMode) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "PA11 pin remapping This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port. 0: No remap (PA11) 1: Remap (PA9)"]
        #[inline(always)]
        pub const fn pa11_rmp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PA11 pin remapping This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port. 0: No remap (PA11) 1: Remap (PA9)"]
        #[inline(always)]
        pub fn set_pa11_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "PA12 pin remapping This bit is set and cleared by software. When set, it remaps the PA12 pin to operate as PA10 GPIO port, instead as PA12 GPIO port. 0: No remap (PA12) 1: Remap (PA10)"]
        #[inline(always)]
        pub const fn pa12_rmp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PA12 pin remapping This bit is set and cleared by software. When set, it remaps the PA12 pin to operate as PA10 GPIO port, instead as PA12 GPIO port. 0: No remap (PA12) 1: Remap (PA10)"]
        #[inline(always)]
        pub fn set_pa12_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "IR output polarity selection"]
        #[inline(always)]
        pub const fn ir_pol(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "IR output polarity selection"]
        #[inline(always)]
        pub fn set_ir_pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "IR Modulation Envelope signal selection This bitfield selects the signal for IR modulation envelope:"]
        #[inline(always)]
        pub const fn ir_mod(&self) -> super::vals::IrMod {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::IrMod::from_bits(val as u8)
        }
        #[doc = "IR Modulation Envelope signal selection This bitfield selects the signal for IR modulation envelope:"]
        #[inline(always)]
        pub fn set_ir_mod(&mut self, val: super::vals::IrMod) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "I/O analog switch voltage booster enable This bit selects the way of supplying I/O analog switches: When using the analog inputs , setting to 0 is recommended for high V<sub>DD</sub>, setting to 1 for low V<sub>DD</sub> (less than 2.4 V)."]
        #[inline(always)]
        pub const fn boosten(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "I/O analog switch voltage booster enable This bit selects the way of supplying I/O analog switches: When using the analog inputs , setting to 0 is recommended for high V<sub>DD</sub>, setting to 1 for low V<sub>DD</sub> (less than 2.4 V)."]
        #[inline(always)]
        pub fn set_boosten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Fast Mode Plus (FM+) enable for PB6 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB6 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead. 0: Disable 1: Enable"]
        #[inline(always)]
        pub const fn i2c_pb6_fmp(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Fast Mode Plus (FM+) enable for PB6 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB6 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead. 0: Disable 1: Enable"]
        #[inline(always)]
        pub fn set_i2c_pb6_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Fast Mode Plus (FM+) enable for PB7 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB7 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead. 0: Disable 1: Enable"]
        #[inline(always)]
        pub const fn i2c_pb7_fmp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Fast Mode Plus (FM+) enable for PB7 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB7 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead. 0: Disable 1: Enable"]
        #[inline(always)]
        pub fn set_i2c_pb7_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Fast Mode Plus (FM+) enable for PB8 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB8 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead. 0: Disable 1: Enable"]
        #[inline(always)]
        pub const fn i2c_pb8_fmp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Fast Mode Plus (FM+) enable for PB8 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB8 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead. 0: Disable 1: Enable"]
        #[inline(always)]
        pub fn set_i2c_pb8_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Fast Mode Plus (FM+) enable for PB9 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB9 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead. 0: Disable 1: Enable"]
        #[inline(always)]
        pub const fn i2c_pb9_fmp(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Fast Mode Plus (FM+) enable for PB9 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PB9 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead. 0: Disable 1: Enable"]
        #[inline(always)]
        pub fn set_i2c_pb9_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Fast Mode Plus (FM+) enable for PA9 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PA9 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead. 0: Disable 1: Enable"]
        #[inline(always)]
        pub const fn i2c_pa9_fmp(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Fast Mode Plus (FM+) enable for PA9 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PA9 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead. 0: Disable 1: Enable"]
        #[inline(always)]
        pub fn set_i2c_pa9_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Fast Mode Plus (FM+) enable for PA10 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PA10 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead. 0: Disable 1: Enable"]
        #[inline(always)]
        pub const fn i2c_pa10_fmp(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Fast Mode Plus (FM+) enable for PA10 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on PA10 I/O port. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead. 0: Disable 1: Enable"]
        #[inline(always)]
        pub fn set_i2c_pa10_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Fast Mode Plus (FM+) enable for I2C3 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on I/O ports configured as I2C3 through GPIOx_AFR registers. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on I/O ports configured as I2C3 can be enabled through their corresponding I2Cx_FMP bit. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead. 0: Disable 1: Enable"]
        #[inline(always)]
        pub const fn i2c3_fmp(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Fast Mode Plus (FM+) enable for I2C3 This bit is set and cleared by software. It enables I<sup>2</sup>C FM+ driving capability on I/O ports configured as I2C3 through GPIOx_AFR registers. With this bit in disable state, the I<sup>2</sup>C FM+ driving capability on I/O ports configured as I2C3 can be enabled through their corresponding I2Cx_FMP bit. When I<sup>2</sup>C FM+ is enabled, the speed control is ignored. Note: This control bit is kept for legacy reasons. It is recommended to use the FMP bit of the I2Cx_CR1 register instead. 0: Disable 1: Enable"]
        #[inline(always)]
        pub fn set_i2c3_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Cfgr1 {
        #[inline(always)]
        fn default() -> Cfgr1 {
            Cfgr1(0)
        }
    }
    #[doc = "SYSCFG configuration register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "Cortex<Superscript>1<Default 1 Font>-M0+ LOCKUP bit enable bit This bit is set by software and cleared by a system reset. It can be use to enable and lock the connection of Cortex<Superscript>1<Default 1 Font>-M0+ LOCKUP (Hardfault) output to TIM1/15/16 Break input."]
        #[inline(always)]
        pub const fn ccl(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Cortex<Superscript>1<Default 1 Font>-M0+ LOCKUP bit enable bit This bit is set by software and cleared by a system reset. It can be use to enable and lock the connection of Cortex<Superscript>1<Default 1 Font>-M0+ LOCKUP (Hardfault) output to TIM1/15/16 Break input."]
        #[inline(always)]
        pub fn set_ccl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SRAM1 parity lock bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the SRAM1 parity error signal connection to TIM1/15/16 Break input."]
        #[inline(always)]
        pub const fn spl(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 parity lock bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the SRAM1 parity error signal connection to TIM1/15/16 Break input."]
        #[inline(always)]
        pub fn set_spl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PVD lock enable bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the PVD connection to TIM1/15/16 Break input, as well as the PVDE and PLS\\[2:0\\]
in the PWR_CR register."]
        #[inline(always)]
        pub const fn pvdl(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PVD lock enable bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the PVD connection to TIM1/15/16 Break input, as well as the PVDE and PLS\\[2:0\\]
in the PWR_CR register."]
        #[inline(always)]
        pub fn set_pvdl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ECC error lock bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the flash ECC 2-bit error detection signal connection to TIM1/15/16 Break input."]
        #[inline(always)]
        pub const fn eccl(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ECC error lock bit This bit is set by software and cleared by a system reset. It can be used to enable and lock the flash ECC 2-bit error detection signal connection to TIM1/15/16 Break input."]
        #[inline(always)]
        pub fn set_eccl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Backup SRAM2 parity lock This bit is set by software and cleared by a system reset. It can be used to enable and lock the SRAM2 parity error signal connection to TIM1/15/16 Break input."]
        #[inline(always)]
        pub const fn bkpl(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Backup SRAM2 parity lock This bit is set by software and cleared by a system reset. It can be used to enable and lock the SRAM2 parity error signal connection to TIM1/15/16 Break input."]
        #[inline(always)]
        pub fn set_bkpl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Backup SRAM2 parity error flag This bit is set by hardware when an SRAM2 parity error is detected. It is cleared by software by writing 1."]
        #[inline(always)]
        pub const fn bkpf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Backup SRAM2 parity error flag This bit is set by hardware when an SRAM2 parity error is detected. It is cleared by software by writing 1."]
        #[inline(always)]
        pub fn set_bkpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "SRAM1 parity error flag This bit is set by hardware when an SRAM1 parity error is detected. It is cleared by software by writing 1."]
        #[inline(always)]
        pub const fn spf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 parity error flag This bit is set by hardware when an SRAM1 parity error is detected. It is cleared by software by writing 1."]
        #[inline(always)]
        pub fn set_spf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Cfgr2 {
        #[inline(always)]
        fn default() -> Cfgr2 {
            Cfgr2(0)
        }
    }
    #[doc = "SYSCFG interrupt line 0 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline0(pub u32);
    impl Itline0 {
        #[doc = "Window watchdog interrupt pending flag"]
        #[inline(always)]
        pub const fn wwdg(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog interrupt pending flag"]
        #[inline(always)]
        pub fn set_wwdg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline0 {
        #[inline(always)]
        fn default() -> Itline0 {
            Itline0(0)
        }
    }
    #[doc = "SYSCFG interrupt line 1 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline1(pub u32);
    impl Itline1 {
        #[doc = "PVD supply monitoring interrupt request pending (EXTI line 16)."]
        #[inline(always)]
        pub const fn pvdout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PVD supply monitoring interrupt request pending (EXTI line 16)."]
        #[inline(always)]
        pub fn set_pvdout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "V<sub>DDUSB</sub> supply monitoring interrupt request pending (EXTI line 19)"]
        #[inline(always)]
        pub const fn pvmout1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "V<sub>DDUSB</sub> supply monitoring interrupt request pending (EXTI line 19)"]
        #[inline(always)]
        pub fn set_pvmout1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ADC supply monitoring interrupt request pending (EXTI line 20)"]
        #[inline(always)]
        pub const fn pvmout3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ADC supply monitoring interrupt request pending (EXTI line 20)"]
        #[inline(always)]
        pub fn set_pvmout3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "DAC supply monitoring interrupt request pending (EXTI line 21)"]
        #[inline(always)]
        pub const fn pvmout4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DAC supply monitoring interrupt request pending (EXTI line 21)"]
        #[inline(always)]
        pub fn set_pvmout4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Itline1 {
        #[inline(always)]
        fn default() -> Itline1 {
            Itline1(0)
        }
    }
    #[doc = "SYSCFG interrupt line 10 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline10(pub u32);
    impl Itline10 {
        #[doc = "DMA1 channel 2 interrupt request pending"]
        #[inline(always)]
        pub const fn dma1_ch2(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 channel 2 interrupt request pending"]
        #[inline(always)]
        pub fn set_dma1_ch2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA1 channel 3 interrupt request pending"]
        #[inline(always)]
        pub const fn dma1_ch3(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 channel 3 interrupt request pending"]
        #[inline(always)]
        pub fn set_dma1_ch3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Itline10 {
        #[inline(always)]
        fn default() -> Itline10 {
            Itline10(0)
        }
    }
    #[doc = "SYSCFG interrupt line 11 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline11(pub u32);
    impl Itline11 {
        #[doc = "DMAMUX interrupt request pending"]
        #[inline(always)]
        pub const fn dmamux(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMAMUX interrupt request pending"]
        #[inline(always)]
        pub fn set_dmamux(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA1 channel 4 interrupt request pending"]
        #[inline(always)]
        pub const fn dma1_ch4(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 channel 4 interrupt request pending"]
        #[inline(always)]
        pub fn set_dma1_ch4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DMA1 channel 5 interrupt request pending"]
        #[inline(always)]
        pub const fn dma1_ch5(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 channel 5 interrupt request pending"]
        #[inline(always)]
        pub fn set_dma1_ch5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "DMA1 channel 6 interrupt request pending"]
        #[inline(always)]
        pub const fn dma1_ch6(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 channel 6 interrupt request pending"]
        #[inline(always)]
        pub fn set_dma1_ch6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DMA1 channel 7 interrupt request pending"]
        #[inline(always)]
        pub const fn dma1_ch7(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 channel 7 interrupt request pending"]
        #[inline(always)]
        pub fn set_dma1_ch7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "DMA2 channel 1 interrupt request pending"]
        #[inline(always)]
        pub const fn dma2_ch1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 channel 1 interrupt request pending"]
        #[inline(always)]
        pub fn set_dma2_ch1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "DMA2 channel 2 interrupt request pending"]
        #[inline(always)]
        pub const fn dma2_ch2(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 channel 2 interrupt request pending"]
        #[inline(always)]
        pub fn set_dma2_ch2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "DMA2 channel 3 interrupt request pending"]
        #[inline(always)]
        pub const fn dma2_ch3(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 channel 3 interrupt request pending"]
        #[inline(always)]
        pub fn set_dma2_ch3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "DMA2 channel 4 interrupt request pending"]
        #[inline(always)]
        pub const fn dma2_ch4(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 channel 4 interrupt request pending"]
        #[inline(always)]
        pub fn set_dma2_ch4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "DMA2 channel 5 interrupt request pending"]
        #[inline(always)]
        pub const fn dma2_ch5(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 channel 5 interrupt request pending"]
        #[inline(always)]
        pub fn set_dma2_ch5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Itline11 {
        #[inline(always)]
        fn default() -> Itline11 {
            Itline11(0)
        }
    }
    #[doc = "SYSCFG interrupt line 12 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline12(pub u32);
    impl Itline12 {
        #[doc = "ADC interrupt request pending"]
        #[inline(always)]
        pub const fn adc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADC interrupt request pending"]
        #[inline(always)]
        pub fn set_adc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Comparator 1 interrupt request pending (EXTI line 17)"]
        #[inline(always)]
        pub const fn comp1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator 1 interrupt request pending (EXTI line 17)"]
        #[inline(always)]
        pub fn set_comp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Comparator 2 interrupt request pending (EXTI line 18)"]
        #[inline(always)]
        pub const fn comp2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator 2 interrupt request pending (EXTI line 18)"]
        #[inline(always)]
        pub fn set_comp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Itline12 {
        #[inline(always)]
        fn default() -> Itline12 {
            Itline12(0)
        }
    }
    #[doc = "SYSCFG interrupt line 13 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline13(pub u32);
    impl Itline13 {
        #[doc = "Timer 1 commutation interrupt request pending"]
        #[inline(always)]
        pub const fn tim1_ccu(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 1 commutation interrupt request pending"]
        #[inline(always)]
        pub fn set_tim1_ccu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timer 1 trigger interrupt request pending"]
        #[inline(always)]
        pub const fn tim1_trg(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 1 trigger interrupt request pending"]
        #[inline(always)]
        pub fn set_tim1_trg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer 1 update interrupt request pending"]
        #[inline(always)]
        pub const fn tim1_upd(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 1 update interrupt request pending"]
        #[inline(always)]
        pub fn set_tim1_upd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Timer 1 break interrupt request pending"]
        #[inline(always)]
        pub const fn tim1_brk(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 1 break interrupt request pending"]
        #[inline(always)]
        pub fn set_tim1_brk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Itline13 {
        #[inline(always)]
        fn default() -> Itline13 {
            Itline13(0)
        }
    }
    #[doc = "SYSCFG interrupt line 14 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline14(pub u32);
    impl Itline14 {
        #[doc = "Timer 1 capture compare 1 interrupt request pending"]
        #[inline(always)]
        pub const fn tim1_cc1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 1 capture compare 1 interrupt request pending"]
        #[inline(always)]
        pub fn set_tim1_cc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timer 1 capture compare 2 interrupt request pending"]
        #[inline(always)]
        pub const fn tim1_cc2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 1 capture compare 2 interrupt request pending"]
        #[inline(always)]
        pub fn set_tim1_cc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer 1 capture compare 3 interrupt request pending"]
        #[inline(always)]
        pub const fn tim1_cc3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 1 capture compare 3 interrupt request pending"]
        #[inline(always)]
        pub fn set_tim1_cc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Timer 1 capture compare 4 interrupt request pending"]
        #[inline(always)]
        pub const fn tim1_cc4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 1 capture compare 4 interrupt request pending"]
        #[inline(always)]
        pub fn set_tim1_cc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Itline14 {
        #[inline(always)]
        fn default() -> Itline14 {
            Itline14(0)
        }
    }
    #[doc = "SYSCFG interrupt line 15 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline15(pub u32);
    impl Itline15 {
        #[doc = "Timer 2 interrupt request pending"]
        #[inline(always)]
        pub const fn tim2(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 2 interrupt request pending"]
        #[inline(always)]
        pub fn set_tim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline15 {
        #[inline(always)]
        fn default() -> Itline15 {
            Itline15(0)
        }
    }
    #[doc = "SYSCFG interrupt line 16 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline16(pub u32);
    impl Itline16 {
        #[doc = "Timer 3 interrupt request pending"]
        #[inline(always)]
        pub const fn tim3(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 3 interrupt request pending"]
        #[inline(always)]
        pub fn set_tim3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline16 {
        #[inline(always)]
        fn default() -> Itline16 {
            Itline16(0)
        }
    }
    #[doc = "SYSCFG interrupt line 17 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline17(pub u32);
    impl Itline17 {
        #[doc = "Timer 6 interrupt request pending"]
        #[inline(always)]
        pub const fn tim6(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 6 interrupt request pending"]
        #[inline(always)]
        pub fn set_tim6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DAC underrun interrupt request pending"]
        #[inline(always)]
        pub const fn dac(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DAC underrun interrupt request pending"]
        #[inline(always)]
        pub fn set_dac(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Low-power timer 1 interrupt request pending (EXTI line 29)"]
        #[inline(always)]
        pub const fn lptim1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power timer 1 interrupt request pending (EXTI line 29)"]
        #[inline(always)]
        pub fn set_lptim1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Itline17 {
        #[inline(always)]
        fn default() -> Itline17 {
            Itline17(0)
        }
    }
    #[doc = "SYSCFG interrupt line 18 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline18(pub u32);
    impl Itline18 {
        #[doc = "Timer 7 interrupt request pending"]
        #[inline(always)]
        pub const fn tim7(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 7 interrupt request pending"]
        #[inline(always)]
        pub fn set_tim7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Low-power timer 2 interrupt request pending (EXTI line 30)"]
        #[inline(always)]
        pub const fn lptim2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power timer 2 interrupt request pending (EXTI line 30)"]
        #[inline(always)]
        pub fn set_lptim2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Itline18 {
        #[inline(always)]
        fn default() -> Itline18 {
            Itline18(0)
        }
    }
    #[doc = "SYSCFG interrupt line 19 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline19(pub u32);
    impl Itline19 {
        #[doc = "Timer 15 interrupt request pending"]
        #[inline(always)]
        pub const fn tim15(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 15 interrupt request pending"]
        #[inline(always)]
        pub fn set_tim15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Low-power timer 3 interrupt request pending"]
        #[inline(always)]
        pub const fn lptim3(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power timer 3 interrupt request pending"]
        #[inline(always)]
        pub fn set_lptim3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Itline19 {
        #[inline(always)]
        fn default() -> Itline19 {
            Itline19(0)
        }
    }
    #[doc = "SYSCFG interrupt line 2 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline2(pub u32);
    impl Itline2 {
        #[doc = "Tamper interrupt request pending (EXTI line 21)"]
        #[inline(always)]
        pub const fn tamp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Tamper interrupt request pending (EXTI line 21)"]
        #[inline(always)]
        pub fn set_tamp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RTC interrupt request pending (EXTI line 19)"]
        #[inline(always)]
        pub const fn rtc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RTC interrupt request pending (EXTI line 19)"]
        #[inline(always)]
        pub fn set_rtc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Itline2 {
        #[inline(always)]
        fn default() -> Itline2 {
            Itline2(0)
        }
    }
    #[doc = "SYSCFG interrupt line 20 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline20(pub u32);
    impl Itline20 {
        #[doc = "Timer 16 interrupt request pending"]
        #[inline(always)]
        pub const fn tim16(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 16 interrupt request pending"]
        #[inline(always)]
        pub fn set_tim16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline20 {
        #[inline(always)]
        fn default() -> Itline20 {
            Itline20(0)
        }
    }
    #[doc = "SYSCFG interrupt line 21 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline21(pub u32);
    impl Itline21 {
        #[doc = "TSC max count error interrupt request pending"]
        #[inline(always)]
        pub const fn tsc_mce(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TSC max count error interrupt request pending"]
        #[inline(always)]
        pub fn set_tsc_mce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TSC end of acquisition interrupt request pending"]
        #[inline(always)]
        pub const fn tsc_eoa(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TSC end of acquisition interrupt request pending"]
        #[inline(always)]
        pub fn set_tsc_eoa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Itline21 {
        #[inline(always)]
        fn default() -> Itline21 {
            Itline21(0)
        }
    }
    #[doc = "SYSCFG interrupt line 22 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline22(pub u32);
    impl Itline22 {
        #[doc = "LCD interrupt request pending"]
        #[inline(always)]
        pub const fn lcd(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LCD interrupt request pending"]
        #[inline(always)]
        pub fn set_lcd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline22 {
        #[inline(always)]
        fn default() -> Itline22 {
            Itline22(0)
        }
    }
    #[doc = "SYSCFG interrupt line 23 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline23(pub u32);
    impl Itline23 {
        #[doc = "I2C1 interrupt request pending (EXTI line 33)"]
        #[inline(always)]
        pub const fn i2c1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 interrupt request pending (EXTI line 33)"]
        #[inline(always)]
        pub fn set_i2c1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline23 {
        #[inline(always)]
        fn default() -> Itline23 {
            Itline23(0)
        }
    }
    #[doc = "SYSCFG interrupt line 24 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline24(pub u32);
    impl Itline24 {
        #[doc = "I2C2 interrupt request pending"]
        #[inline(always)]
        pub const fn i2c2(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 interrupt request pending"]
        #[inline(always)]
        pub fn set_i2c2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "I2C4 interrupt request pending"]
        #[inline(always)]
        pub const fn i2c4(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 interrupt request pending"]
        #[inline(always)]
        pub fn set_i2c4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "I2C3 interrupt request pending (EXTI line 23)"]
        #[inline(always)]
        pub const fn i2c3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 interrupt request pending (EXTI line 23)"]
        #[inline(always)]
        pub fn set_i2c3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Itline24 {
        #[inline(always)]
        fn default() -> Itline24 {
            Itline24(0)
        }
    }
    #[doc = "SYSCFG interrupt line 25 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline25(pub u32);
    impl Itline25 {
        #[doc = "SPI1 interrupt request pending"]
        #[inline(always)]
        pub const fn spi1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 interrupt request pending"]
        #[inline(always)]
        pub fn set_spi1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline25 {
        #[inline(always)]
        fn default() -> Itline25 {
            Itline25(0)
        }
    }
    #[doc = "SYSCFG interrupt line 26 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline26(pub u32);
    impl Itline26 {
        #[doc = "SPI2 interrupt request pending"]
        #[inline(always)]
        pub const fn spi2(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 interrupt request pending"]
        #[inline(always)]
        pub fn set_spi2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SPI3 interrupt request pending"]
        #[inline(always)]
        pub const fn spi3(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 interrupt request pending"]
        #[inline(always)]
        pub fn set_spi3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Itline26 {
        #[inline(always)]
        fn default() -> Itline26 {
            Itline26(0)
        }
    }
    #[doc = "SYSCFG interrupt line 27 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline27(pub u32);
    impl Itline27 {
        #[doc = "USART1 interrupt request pending, combined with EXTI line 25"]
        #[inline(always)]
        pub const fn usart1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 interrupt request pending, combined with EXTI line 25"]
        #[inline(always)]
        pub fn set_usart1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline27 {
        #[inline(always)]
        fn default() -> Itline27 {
            Itline27(0)
        }
    }
    #[doc = "SYSCFG interrupt line 28 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline28(pub u32);
    impl Itline28 {
        #[doc = "USART2 interrupt request pending (EXTI line 35)"]
        #[inline(always)]
        pub const fn usart2(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 interrupt request pending (EXTI line 35)"]
        #[inline(always)]
        pub fn set_usart2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LPUART2 interrupt request pending (EXTI line 31)"]
        #[inline(always)]
        pub const fn lpuart2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART2 interrupt request pending (EXTI line 31)"]
        #[inline(always)]
        pub fn set_lpuart2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Itline28 {
        #[inline(always)]
        fn default() -> Itline28 {
            Itline28(0)
        }
    }
    #[doc = "SYSCFG interrupt line 29 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline29(pub u32);
    impl Itline29 {
        #[doc = "USART3 interrupt request pending"]
        #[inline(always)]
        pub const fn usart3(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 interrupt request pending"]
        #[inline(always)]
        pub fn set_usart3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LPUART1 interrupt request pending (EXTI line 30)"]
        #[inline(always)]
        pub const fn lpuart1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 interrupt request pending (EXTI line 30)"]
        #[inline(always)]
        pub fn set_lpuart1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Itline29 {
        #[inline(always)]
        fn default() -> Itline29 {
            Itline29(0)
        }
    }
    #[doc = "SYSCFG interrupt line 3 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline3(pub u32);
    impl Itline3 {
        #[doc = "Flash interface interrupt request pending"]
        #[inline(always)]
        pub const fn flash_itf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Flash interface interrupt request pending"]
        #[inline(always)]
        pub fn set_flash_itf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Flash interface ECC interrupt request pending"]
        #[inline(always)]
        pub const fn flash_ecc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Flash interface ECC interrupt request pending"]
        #[inline(always)]
        pub fn set_flash_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Itline3 {
        #[inline(always)]
        fn default() -> Itline3 {
            Itline3(0)
        }
    }
    #[doc = "SYSCFG interrupt line 30 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline30(pub u32);
    impl Itline30 {
        #[doc = "USART4 interrupt request pending"]
        #[inline(always)]
        pub const fn usart4(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USART4 interrupt request pending"]
        #[inline(always)]
        pub fn set_usart4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LPUART3 interrupt request pending (EXTI line 32)"]
        #[inline(always)]
        pub const fn lpuart3(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART3 interrupt request pending (EXTI line 32)"]
        #[inline(always)]
        pub fn set_lpuart3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Itline30 {
        #[inline(always)]
        fn default() -> Itline30 {
            Itline30(0)
        }
    }
    #[doc = "SYSCFG interrupt line 31 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline31(pub u32);
    impl Itline31 {
        #[doc = "RNG interrupt request pending"]
        #[inline(always)]
        pub const fn rng(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RNG interrupt request pending"]
        #[inline(always)]
        pub fn set_rng(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "AES interrupt request pending"]
        #[inline(always)]
        pub const fn aes(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "AES interrupt request pending"]
        #[inline(always)]
        pub fn set_aes(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Itline31 {
        #[inline(always)]
        fn default() -> Itline31 {
            Itline31(0)
        }
    }
    #[doc = "SYSCFG interrupt line 4 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline4(pub u32);
    impl Itline4 {
        #[doc = "Reset and clock control interrupt request pending"]
        #[inline(always)]
        pub const fn rcc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Reset and clock control interrupt request pending"]
        #[inline(always)]
        pub fn set_rcc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CRS interrupt request pending"]
        #[inline(always)]
        pub const fn crs(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CRS interrupt request pending"]
        #[inline(always)]
        pub fn set_crs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Itline4 {
        #[inline(always)]
        fn default() -> Itline4 {
            Itline4(0)
        }
    }
    #[doc = "SYSCFG interrupt line 5 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline5(pub u32);
    impl Itline5 {
        #[doc = "EXTI line 0 interrupt request pending"]
        #[inline(always)]
        pub const fn exti0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI line 0 interrupt request pending"]
        #[inline(always)]
        pub fn set_exti0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "EXTI line 1 interrupt request pending"]
        #[inline(always)]
        pub const fn exti1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI line 1 interrupt request pending"]
        #[inline(always)]
        pub fn set_exti1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Itline5 {
        #[inline(always)]
        fn default() -> Itline5 {
            Itline5(0)
        }
    }
    #[doc = "SYSCFG interrupt line 6 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline6(pub u32);
    impl Itline6 {
        #[doc = "EXTI line 2 interrupt request pending"]
        #[inline(always)]
        pub const fn exti2(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI line 2 interrupt request pending"]
        #[inline(always)]
        pub fn set_exti2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "EXTI line 3 interrupt request pending"]
        #[inline(always)]
        pub const fn exti3(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI line 3 interrupt request pending"]
        #[inline(always)]
        pub fn set_exti3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Itline6 {
        #[inline(always)]
        fn default() -> Itline6 {
            Itline6(0)
        }
    }
    #[doc = "SYSCFG interrupt line 7 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline7(pub u32);
    impl Itline7 {
        #[doc = "EXTI line 4 interrupt request pending"]
        #[inline(always)]
        pub const fn exti4(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI line 4 interrupt request pending"]
        #[inline(always)]
        pub fn set_exti4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "EXTI line 5 interrupt request pending"]
        #[inline(always)]
        pub const fn exti5(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI line 5 interrupt request pending"]
        #[inline(always)]
        pub fn set_exti5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "EXTI line 6 interrupt request pending"]
        #[inline(always)]
        pub const fn exti6(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI line 6 interrupt request pending"]
        #[inline(always)]
        pub fn set_exti6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "EXTI line 7 interrupt request pending"]
        #[inline(always)]
        pub const fn exti7(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI line 7 interrupt request pending"]
        #[inline(always)]
        pub fn set_exti7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "EXTI line 8 interrupt request pending"]
        #[inline(always)]
        pub const fn exti8(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI line 8 interrupt request pending"]
        #[inline(always)]
        pub fn set_exti8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "EXTI line 9 interrupt request pending"]
        #[inline(always)]
        pub const fn exti9(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI line 9 interrupt request pending"]
        #[inline(always)]
        pub fn set_exti9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "EXTI line 10 interrupt request pending"]
        #[inline(always)]
        pub const fn exti10(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI line 10 interrupt request pending"]
        #[inline(always)]
        pub fn set_exti10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "EXTI line 11 interrupt request pending"]
        #[inline(always)]
        pub const fn exti11(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI line 11 interrupt request pending"]
        #[inline(always)]
        pub fn set_exti11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "EXTI line 12 interrupt request pending"]
        #[inline(always)]
        pub const fn exti12(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI line 12 interrupt request pending"]
        #[inline(always)]
        pub fn set_exti12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "EXTI line 13 interrupt request pending"]
        #[inline(always)]
        pub const fn exti13(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI line 13 interrupt request pending"]
        #[inline(always)]
        pub fn set_exti13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "EXTI line 14 interrupt request pending"]
        #[inline(always)]
        pub const fn exti14(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI line 14 interrupt request pending"]
        #[inline(always)]
        pub fn set_exti14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "EXTI line 15 interrupt request pending"]
        #[inline(always)]
        pub const fn exti15(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI line 15 interrupt request pending"]
        #[inline(always)]
        pub fn set_exti15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for Itline7 {
        #[inline(always)]
        fn default() -> Itline7 {
            Itline7(0)
        }
    }
    #[doc = "SYSCFG interrupt line 8 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline8(pub u32);
    impl Itline8 {
        #[doc = "USB interrupt request pending"]
        #[inline(always)]
        pub const fn usb(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USB interrupt request pending"]
        #[inline(always)]
        pub fn set_usb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline8 {
        #[inline(always)]
        fn default() -> Itline8 {
            Itline8(0)
        }
    }
    #[doc = "SYSCFG interrupt line 9 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itline9(pub u32);
    impl Itline9 {
        #[doc = "DMA1 channel 1 interrupt request pending"]
        #[inline(always)]
        pub const fn dma1_ch1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 channel 1 interrupt request pending"]
        #[inline(always)]
        pub fn set_dma1_ch1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itline9 {
        #[inline(always)]
        fn default() -> Itline9 {
            Itline9(0)
        }
    }
    #[doc = "SYSCFG SRAM2 control and status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scsr(pub u32);
    impl Scsr {
        #[doc = "SRAM2 erase Setting this bit starts a hardware SRAM2 erase operation. This bit is automatically cleared at the end of the SRAM2 erase operation. Note: This bit is write-protected: setting this bit is possible only after the correct key sequence is written in the SYSCFG_SKR register."]
        #[inline(always)]
        pub const fn sram2er(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 erase Setting this bit starts a hardware SRAM2 erase operation. This bit is automatically cleared at the end of the SRAM2 erase operation. Note: This bit is write-protected: setting this bit is possible only after the correct key sequence is written in the SYSCFG_SKR register."]
        #[inline(always)]
        pub fn set_sram2er(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SRAM2 busy by erase operation"]
        #[inline(always)]
        pub const fn sram2bsy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 busy by erase operation"]
        #[inline(always)]
        pub fn set_sram2bsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Scsr {
        #[inline(always)]
        fn default() -> Scsr {
            Scsr(0)
        }
    }
    #[doc = "SYSCFG SRAM2 key register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Skr(pub u32);
    impl Skr {
        #[doc = "SRAM2 write protection key for software erase The following steps are required to unlock the write protection of the SRAM2ER bit in the SYSCFG_CFGR2 register: Write 0xCA into KEY\\[7:0\\]
Write 0x53 into KEY\\[7:0\\]
Writing a wrong key reactivates the write protection."]
        #[inline(always)]
        pub const fn key(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "SRAM2 write protection key for software erase The following steps are required to unlock the write protection of the SRAM2ER bit in the SYSCFG_CFGR2 register: Write 0xCA into KEY\\[7:0\\]
Write 0x53 into KEY\\[7:0\\]
Writing a wrong key reactivates the write protection."]
        #[inline(always)]
        pub fn set_key(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Skr {
        #[inline(always)]
        fn default() -> Skr {
            Skr(0)
        }
    }
    #[doc = "SYSCFG TSC comparator register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tsccr(pub u32);
    impl Tsccr {
        #[doc = "Comparator mode for group 2 on I/O 1"]
        #[inline(always)]
        pub const fn g2_io1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator mode for group 2 on I/O 1"]
        #[inline(always)]
        pub fn set_g2_io1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Comparator mode for group 2 on I/O 3"]
        #[inline(always)]
        pub const fn g2_io3(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator mode for group 2 on I/O 3"]
        #[inline(always)]
        pub fn set_g2_io3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Comparator mode for group 4 on I/O 3"]
        #[inline(always)]
        pub const fn g4_io3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator mode for group 4 on I/O 3"]
        #[inline(always)]
        pub fn set_g4_io3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Comparator mode for group 6 on I/O 1"]
        #[inline(always)]
        pub const fn g6_io1(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator mode for group 6 on I/O 1"]
        #[inline(always)]
        pub fn set_g6_io1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Comparator mode for group 7 on I/O 1"]
        #[inline(always)]
        pub const fn g7_io1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator mode for group 7 on I/O 1"]
        #[inline(always)]
        pub fn set_g7_io1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "I/O control in comparator mode The I/O control in comparator mode can be overwritten by hardware."]
        #[inline(always)]
        pub const fn tsc_ioctrl(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "I/O control in comparator mode The I/O control in comparator mode can be overwritten by hardware."]
        #[inline(always)]
        pub fn set_tsc_ioctrl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Tsccr {
        #[inline(always)]
        fn default() -> Tsccr {
            Tsccr(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum IrMod {
        #[doc = "TIM16"]
        TIM16 = 0x0,
        #[doc = "USART1"]
        USART1 = 0x01,
        #[doc = "USART2"]
        USART2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl IrMod {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> IrMod {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for IrMod {
        #[inline(always)]
        fn from(val: u8) -> IrMod {
            IrMod::from_bits(val)
        }
    }
    impl From<IrMod> for u8 {
        #[inline(always)]
        fn from(val: IrMod) -> u8 {
            IrMod::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum MemMode {
        _RESERVED_0 = 0x0,
        #[doc = "System flash memory mapped at 0x000010000"]
        SYSTEM_FLASH = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "Embedded SRAM mapped at 0x000010000"]
        SRAM = 0x03,
    }
    impl MemMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> MemMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for MemMode {
        #[inline(always)]
        fn from(val: u8) -> MemMode {
            MemMode::from_bits(val)
        }
    }
    impl From<MemMode> for u8 {
        #[inline(always)]
        fn from(val: MemMode) -> u8 {
            MemMode::to_bits(val)
        }
    }
}
