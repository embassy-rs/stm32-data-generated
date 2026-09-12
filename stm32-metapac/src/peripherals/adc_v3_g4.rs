#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Analog-to-digital converter"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc {
    ptr: *mut u8,
}
unsafe impl Send for Adc {}
unsafe impl Sync for Adc {}
impl Adc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Interrupt and status register"]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Interrupt enable register"]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Configuration register"]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Configuration register 2"]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Sample time register. SMPR\\[i\\]
holds the sample times of channels 10*i to 10*i+9."]
    #[inline(always)]
    pub const fn smpr(self, n: usize) -> crate::common::Reg<regs::Smpr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize + n * 4usize) as _) }
    }
    #[doc = "Analog watchdog threshold register. TR\\[1\\]
and TR\\[2\\]
(watchdogs 2 and 3) only use the 8 most significant bits of each threshold."]
    #[inline(always)]
    pub const fn tr(self, n: usize) -> crate::common::Reg<regs::Tr, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _) }
    }
    #[doc = "Regular sequence register 1 (length and conversions 1 to 4)"]
    #[inline(always)]
    pub const fn sqr1(self) -> crate::common::Reg<regs::Sqr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "Regular sequence register 2 (conversions 5 to 9)"]
    #[inline(always)]
    pub const fn sqr2(self) -> crate::common::Reg<regs::Sqr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "Regular sequence register 3 (conversions 10 to 14)"]
    #[inline(always)]
    pub const fn sqr3(self) -> crate::common::Reg<regs::Sqr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Regular sequence register 4 (conversions 15 to 16)"]
    #[inline(always)]
    pub const fn sqr4(self) -> crate::common::Reg<regs::Sqr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Regular data register"]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Injected sequence register"]
    #[inline(always)]
    pub const fn jsqr(self) -> crate::common::Reg<regs::Jsqr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Offset register"]
    #[inline(always)]
    pub const fn ofr(self, n: usize) -> crate::common::Reg<regs::Ofr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize + n * 4usize) as _) }
    }
    #[doc = "Injected data register"]
    #[inline(always)]
    pub const fn jdr(self, n: usize) -> crate::common::Reg<regs::Jdr, crate::common::R> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _) }
    }
    #[doc = "Analog watchdog 2 configuration register"]
    #[inline(always)]
    pub const fn awd2cr(self) -> crate::common::Reg<regs::Awd2cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "Analog watchdog 3 configuration register"]
    #[inline(always)]
    pub const fn awd3cr(self) -> crate::common::Reg<regs::Awd3cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "Differential mode selection register"]
    #[inline(always)]
    pub const fn difsel(self) -> crate::common::Reg<regs::Difsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "Calibration factors register"]
    #[inline(always)]
    pub const fn calfact(self) -> crate::common::Reg<regs::Calfact, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "Gain compensation register (STM32G4 only)"]
    #[inline(always)]
    pub const fn gcomp(self) -> crate::common::Reg<regs::Gcomp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "Option register (STM32H5 and STM32H7RS only)"]
    #[inline(always)]
    pub const fn or(self) -> crate::common::Reg<regs::Or, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
}
pub mod regs {
    #[doc = "Analog watchdog 2 configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awd2cr(pub u32);
    impl Awd2cr {
        #[doc = "Analog watchdog 2 channel-x selection"]
        #[must_use]
        #[inline(always)]
        pub const fn awd2ch(&self, n: usize) -> bool {
            assert!(n < 20usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog 2 channel-x selection"]
        #[inline(always)]
        pub const fn set_awd2ch(&mut self, n: usize, val: bool) {
            assert!(n < 20usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Awd2cr {
        #[inline(always)]
        fn default() -> Awd2cr {
            Awd2cr(0)
        }
    }
    impl core::fmt::Debug for Awd2cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Awd2cr")
                .field("awd2ch[0]", &self.awd2ch(0usize))
                .field("awd2ch[1]", &self.awd2ch(1usize))
                .field("awd2ch[2]", &self.awd2ch(2usize))
                .field("awd2ch[3]", &self.awd2ch(3usize))
                .field("awd2ch[4]", &self.awd2ch(4usize))
                .field("awd2ch[5]", &self.awd2ch(5usize))
                .field("awd2ch[6]", &self.awd2ch(6usize))
                .field("awd2ch[7]", &self.awd2ch(7usize))
                .field("awd2ch[8]", &self.awd2ch(8usize))
                .field("awd2ch[9]", &self.awd2ch(9usize))
                .field("awd2ch[10]", &self.awd2ch(10usize))
                .field("awd2ch[11]", &self.awd2ch(11usize))
                .field("awd2ch[12]", &self.awd2ch(12usize))
                .field("awd2ch[13]", &self.awd2ch(13usize))
                .field("awd2ch[14]", &self.awd2ch(14usize))
                .field("awd2ch[15]", &self.awd2ch(15usize))
                .field("awd2ch[16]", &self.awd2ch(16usize))
                .field("awd2ch[17]", &self.awd2ch(17usize))
                .field("awd2ch[18]", &self.awd2ch(18usize))
                .field("awd2ch[19]", &self.awd2ch(19usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Awd2cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Awd2cr {{ awd2ch[0]: {=bool:?}, awd2ch[1]: {=bool:?}, awd2ch[2]: {=bool:?}, awd2ch[3]: {=bool:?}, awd2ch[4]: {=bool:?}, awd2ch[5]: {=bool:?}, awd2ch[6]: {=bool:?}, awd2ch[7]: {=bool:?}, awd2ch[8]: {=bool:?}, awd2ch[9]: {=bool:?}, awd2ch[10]: {=bool:?}, awd2ch[11]: {=bool:?}, awd2ch[12]: {=bool:?}, awd2ch[13]: {=bool:?}, awd2ch[14]: {=bool:?}, awd2ch[15]: {=bool:?}, awd2ch[16]: {=bool:?}, awd2ch[17]: {=bool:?}, awd2ch[18]: {=bool:?}, awd2ch[19]: {=bool:?} }}",
                self.awd2ch(0usize),
                self.awd2ch(1usize),
                self.awd2ch(2usize),
                self.awd2ch(3usize),
                self.awd2ch(4usize),
                self.awd2ch(5usize),
                self.awd2ch(6usize),
                self.awd2ch(7usize),
                self.awd2ch(8usize),
                self.awd2ch(9usize),
                self.awd2ch(10usize),
                self.awd2ch(11usize),
                self.awd2ch(12usize),
                self.awd2ch(13usize),
                self.awd2ch(14usize),
                self.awd2ch(15usize),
                self.awd2ch(16usize),
                self.awd2ch(17usize),
                self.awd2ch(18usize),
                self.awd2ch(19usize)
            )
        }
    }
    #[doc = "Analog watchdog 3 configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awd3cr(pub u32);
    impl Awd3cr {
        #[doc = "Analog watchdog 3 channel-x selection"]
        #[must_use]
        #[inline(always)]
        pub const fn awd3ch(&self, n: usize) -> bool {
            assert!(n < 20usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog 3 channel-x selection"]
        #[inline(always)]
        pub const fn set_awd3ch(&mut self, n: usize, val: bool) {
            assert!(n < 20usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Awd3cr {
        #[inline(always)]
        fn default() -> Awd3cr {
            Awd3cr(0)
        }
    }
    impl core::fmt::Debug for Awd3cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Awd3cr")
                .field("awd3ch[0]", &self.awd3ch(0usize))
                .field("awd3ch[1]", &self.awd3ch(1usize))
                .field("awd3ch[2]", &self.awd3ch(2usize))
                .field("awd3ch[3]", &self.awd3ch(3usize))
                .field("awd3ch[4]", &self.awd3ch(4usize))
                .field("awd3ch[5]", &self.awd3ch(5usize))
                .field("awd3ch[6]", &self.awd3ch(6usize))
                .field("awd3ch[7]", &self.awd3ch(7usize))
                .field("awd3ch[8]", &self.awd3ch(8usize))
                .field("awd3ch[9]", &self.awd3ch(9usize))
                .field("awd3ch[10]", &self.awd3ch(10usize))
                .field("awd3ch[11]", &self.awd3ch(11usize))
                .field("awd3ch[12]", &self.awd3ch(12usize))
                .field("awd3ch[13]", &self.awd3ch(13usize))
                .field("awd3ch[14]", &self.awd3ch(14usize))
                .field("awd3ch[15]", &self.awd3ch(15usize))
                .field("awd3ch[16]", &self.awd3ch(16usize))
                .field("awd3ch[17]", &self.awd3ch(17usize))
                .field("awd3ch[18]", &self.awd3ch(18usize))
                .field("awd3ch[19]", &self.awd3ch(19usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Awd3cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Awd3cr {{ awd3ch[0]: {=bool:?}, awd3ch[1]: {=bool:?}, awd3ch[2]: {=bool:?}, awd3ch[3]: {=bool:?}, awd3ch[4]: {=bool:?}, awd3ch[5]: {=bool:?}, awd3ch[6]: {=bool:?}, awd3ch[7]: {=bool:?}, awd3ch[8]: {=bool:?}, awd3ch[9]: {=bool:?}, awd3ch[10]: {=bool:?}, awd3ch[11]: {=bool:?}, awd3ch[12]: {=bool:?}, awd3ch[13]: {=bool:?}, awd3ch[14]: {=bool:?}, awd3ch[15]: {=bool:?}, awd3ch[16]: {=bool:?}, awd3ch[17]: {=bool:?}, awd3ch[18]: {=bool:?}, awd3ch[19]: {=bool:?} }}",
                self.awd3ch(0usize),
                self.awd3ch(1usize),
                self.awd3ch(2usize),
                self.awd3ch(3usize),
                self.awd3ch(4usize),
                self.awd3ch(5usize),
                self.awd3ch(6usize),
                self.awd3ch(7usize),
                self.awd3ch(8usize),
                self.awd3ch(9usize),
                self.awd3ch(10usize),
                self.awd3ch(11usize),
                self.awd3ch(12usize),
                self.awd3ch(13usize),
                self.awd3ch(14usize),
                self.awd3ch(15usize),
                self.awd3ch(16usize),
                self.awd3ch(17usize),
                self.awd3ch(18usize),
                self.awd3ch(19usize)
            )
        }
    }
    #[doc = "Calibration factors register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Calfact(pub u32);
    impl Calfact {
        #[doc = "Calibration factor in single-ended mode"]
        #[must_use]
        #[inline(always)]
        pub const fn calfact_s(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Calibration factor in single-ended mode"]
        #[inline(always)]
        pub const fn set_calfact_s(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Calibration factor in differential mode"]
        #[must_use]
        #[inline(always)]
        pub const fn calfact_d(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Calibration factor in differential mode"]
        #[inline(always)]
        pub const fn set_calfact_d(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for Calfact {
        #[inline(always)]
        fn default() -> Calfact {
            Calfact(0)
        }
    }
    impl core::fmt::Debug for Calfact {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Calfact")
                .field("calfact_s", &self.calfact_s())
                .field("calfact_d", &self.calfact_d())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Calfact {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Calfact {{ calfact_s: {=u8:?}, calfact_d: {=u8:?} }}",
                self.calfact_s(),
                self.calfact_d()
            )
        }
    }
    #[doc = "Configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "Direct memory access enable"]
        #[must_use]
        #[inline(always)]
        pub const fn dmaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Direct memory access enable"]
        #[inline(always)]
        pub const fn set_dmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Direct memory access configuration"]
        #[must_use]
        #[inline(always)]
        pub const fn dmacfg(&self) -> super::vals::Dmacfg {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Dmacfg::from_bits(val as u8)
        }
        #[doc = "Direct memory access configuration"]
        #[inline(always)]
        pub const fn set_dmacfg(&mut self, val: super::vals::Dmacfg) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Digital filter (ADF) configuration (STM32H7RS only)"]
        #[must_use]
        #[inline(always)]
        pub const fn adfcfg(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Digital filter (ADF) configuration (STM32H7RS only)"]
        #[inline(always)]
        pub const fn set_adfcfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Data resolution"]
        #[must_use]
        #[inline(always)]
        pub const fn res(&self) -> super::vals::Res {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Res::from_bits(val as u8)
        }
        #[doc = "Data resolution"]
        #[inline(always)]
        pub const fn set_res(&mut self, val: super::vals::Res) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "External trigger selection for regular group"]
        #[must_use]
        #[inline(always)]
        pub const fn extsel(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x1f;
            val as u8
        }
        #[doc = "External trigger selection for regular group"]
        #[inline(always)]
        pub const fn set_extsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
        }
        #[doc = "External trigger enable and polarity selection for regular channels"]
        #[must_use]
        #[inline(always)]
        pub const fn exten(&self) -> super::vals::Exten {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::Exten::from_bits(val as u8)
        }
        #[doc = "External trigger enable and polarity selection for regular channels"]
        #[inline(always)]
        pub const fn set_exten(&mut self, val: super::vals::Exten) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
        #[doc = "Overrun mode (0: ADC_DR is preserved when an overrun is detected, 1: ADC_DR is overwritten with the last conversion result)"]
        #[must_use]
        #[inline(always)]
        pub const fn ovrmod(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun mode (0: ADC_DR is preserved when an overrun is detected, 1: ADC_DR is overwritten with the last conversion result)"]
        #[inline(always)]
        pub const fn set_ovrmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Continuous conversion mode for regular conversions"]
        #[must_use]
        #[inline(always)]
        pub const fn cont(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Continuous conversion mode for regular conversions"]
        #[inline(always)]
        pub const fn set_cont(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Delayed conversion mode"]
        #[must_use]
        #[inline(always)]
        pub const fn autdly(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Delayed conversion mode"]
        #[inline(always)]
        pub const fn set_autdly(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Data alignment"]
        #[must_use]
        #[inline(always)]
        pub const fn align(&self) -> super::vals::Align {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Align::from_bits(val as u8)
        }
        #[doc = "Data alignment"]
        #[inline(always)]
        pub const fn set_align(&mut self, val: super::vals::Align) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "Discontinuous mode for regular channels"]
        #[must_use]
        #[inline(always)]
        pub const fn discen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Discontinuous mode for regular channels"]
        #[inline(always)]
        pub const fn set_discen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Discontinuous mode channel count"]
        #[must_use]
        #[inline(always)]
        pub const fn discnum(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[doc = "Discontinuous mode channel count"]
        #[inline(always)]
        pub const fn set_discnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
        }
        #[doc = "Discontinuous mode on injected channels"]
        #[must_use]
        #[inline(always)]
        pub const fn jdiscen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Discontinuous mode on injected channels"]
        #[inline(always)]
        pub const fn set_jdiscen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "JSQR queue mode"]
        #[must_use]
        #[inline(always)]
        pub const fn jqm(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "JSQR queue mode"]
        #[inline(always)]
        pub const fn set_jqm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Enable the analog watchdog 1 on a single channel (1) or on all channels (0)"]
        #[must_use]
        #[inline(always)]
        pub const fn awd1sgl(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Enable the analog watchdog 1 on a single channel (1) or on all channels (0)"]
        #[inline(always)]
        pub const fn set_awd1sgl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Analog watchdog 1 enable on regular channels"]
        #[must_use]
        #[inline(always)]
        pub const fn awd1en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog 1 enable on regular channels"]
        #[inline(always)]
        pub const fn set_awd1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Analog watchdog 1 enable on injected channels"]
        #[must_use]
        #[inline(always)]
        pub const fn jawd1en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog 1 enable on injected channels"]
        #[inline(always)]
        pub const fn set_jawd1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Automatic injected group conversion"]
        #[must_use]
        #[inline(always)]
        pub const fn jauto(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic injected group conversion"]
        #[inline(always)]
        pub const fn set_jauto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Analog watchdog 1 channel selection"]
        #[must_use]
        #[inline(always)]
        pub const fn awd1ch(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x1f;
            val as u8
        }
        #[doc = "Analog watchdog 1 channel selection"]
        #[inline(always)]
        pub const fn set_awd1ch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
        }
        #[doc = "Injected queue disable"]
        #[must_use]
        #[inline(always)]
        pub const fn jqdis(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Injected queue disable"]
        #[inline(always)]
        pub const fn set_jqdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cfgr {
        #[inline(always)]
        fn default() -> Cfgr {
            Cfgr(0)
        }
    }
    impl core::fmt::Debug for Cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr")
                .field("dmaen", &self.dmaen())
                .field("dmacfg", &self.dmacfg())
                .field("adfcfg", &self.adfcfg())
                .field("res", &self.res())
                .field("extsel", &self.extsel())
                .field("exten", &self.exten())
                .field("ovrmod", &self.ovrmod())
                .field("cont", &self.cont())
                .field("autdly", &self.autdly())
                .field("align", &self.align())
                .field("discen", &self.discen())
                .field("discnum", &self.discnum())
                .field("jdiscen", &self.jdiscen())
                .field("jqm", &self.jqm())
                .field("awd1sgl", &self.awd1sgl())
                .field("awd1en", &self.awd1en())
                .field("jawd1en", &self.jawd1en())
                .field("jauto", &self.jauto())
                .field("awd1ch", &self.awd1ch())
                .field("jqdis", &self.jqdis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfgr {{ dmaen: {=bool:?}, dmacfg: {:?}, adfcfg: {=bool:?}, res: {:?}, extsel: {=u8:?}, exten: {:?}, ovrmod: {=bool:?}, cont: {=bool:?}, autdly: {=bool:?}, align: {:?}, discen: {=bool:?}, discnum: {=u8:?}, jdiscen: {=bool:?}, jqm: {=bool:?}, awd1sgl: {=bool:?}, awd1en: {=bool:?}, jawd1en: {=bool:?}, jauto: {=bool:?}, awd1ch: {=u8:?}, jqdis: {=bool:?} }}",
                self.dmaen(),
                self.dmacfg(),
                self.adfcfg(),
                self.res(),
                self.extsel(),
                self.exten(),
                self.ovrmod(),
                self.cont(),
                self.autdly(),
                self.align(),
                self.discen(),
                self.discnum(),
                self.jdiscen(),
                self.jqm(),
                self.awd1sgl(),
                self.awd1en(),
                self.jawd1en(),
                self.jauto(),
                self.awd1ch(),
                self.jqdis()
            )
        }
    }
    #[doc = "Configuration register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "Regular oversampling enable"]
        #[must_use]
        #[inline(always)]
        pub const fn rovse(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Regular oversampling enable"]
        #[inline(always)]
        pub const fn set_rovse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Injected oversampling enable"]
        #[must_use]
        #[inline(always)]
        pub const fn jovse(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Injected oversampling enable"]
        #[inline(always)]
        pub const fn set_jovse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Oversampling ratio"]
        #[must_use]
        #[inline(always)]
        pub const fn ovsr(&self) -> super::vals::Ovsr {
            let val = (self.0 >> 2usize) & 0x07;
            super::vals::Ovsr::from_bits(val as u8)
        }
        #[doc = "Oversampling ratio"]
        #[inline(always)]
        pub const fn set_ovsr(&mut self, val: super::vals::Ovsr) {
            self.0 = (self.0 & !(0x07 << 2usize)) | (((val.to_bits() as u32) & 0x07) << 2usize);
        }
        #[doc = "Oversampling shift (number of bits to right-shift the accumulated result)"]
        #[must_use]
        #[inline(always)]
        pub const fn ovss(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x0f;
            val as u8
        }
        #[doc = "Oversampling shift (number of bits to right-shift the accumulated result)"]
        #[inline(always)]
        pub const fn set_ovss(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 5usize)) | (((val as u32) & 0x0f) << 5usize);
        }
        #[doc = "Triggered regular oversampling"]
        #[must_use]
        #[inline(always)]
        pub const fn trovs(&self) -> super::vals::Trovs {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Trovs::from_bits(val as u8)
        }
        #[doc = "Triggered regular oversampling"]
        #[inline(always)]
        pub const fn set_trovs(&mut self, val: super::vals::Trovs) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "Regular oversampling mode"]
        #[must_use]
        #[inline(always)]
        pub const fn rovsm(&self) -> super::vals::Rovsm {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Rovsm::from_bits(val as u8)
        }
        #[doc = "Regular oversampling mode"]
        #[inline(always)]
        pub const fn set_rovsm(&mut self, val: super::vals::Rovsm) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "Gain compensation mode"]
        #[must_use]
        #[inline(always)]
        pub const fn gcomp(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Gain compensation mode"]
        #[inline(always)]
        pub const fn set_gcomp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Software trigger bit for sampling time control trigger mode"]
        #[must_use]
        #[inline(always)]
        pub const fn swtrig(&self) -> super::vals::Swtrig {
            let val = (self.0 >> 25usize) & 0x01;
            super::vals::Swtrig::from_bits(val as u8)
        }
        #[doc = "Software trigger bit for sampling time control trigger mode"]
        #[inline(always)]
        pub const fn set_swtrig(&mut self, val: super::vals::Swtrig) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
        }
        #[doc = "Bulb sampling mode"]
        #[must_use]
        #[inline(always)]
        pub const fn bulb(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Bulb sampling mode"]
        #[inline(always)]
        pub const fn set_bulb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Sampling time control trigger mode"]
        #[must_use]
        #[inline(always)]
        pub const fn smptrig(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Sampling time control trigger mode"]
        #[inline(always)]
        pub const fn set_smptrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Low frequency trigger mode enable (STM32H5 only)"]
        #[must_use]
        #[inline(always)]
        pub const fn lftrig(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Low frequency trigger mode enable (STM32H5 only)"]
        #[inline(always)]
        pub const fn set_lftrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Cfgr2 {
        #[inline(always)]
        fn default() -> Cfgr2 {
            Cfgr2(0)
        }
    }
    impl core::fmt::Debug for Cfgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr2")
                .field("rovse", &self.rovse())
                .field("jovse", &self.jovse())
                .field("ovsr", &self.ovsr())
                .field("ovss", &self.ovss())
                .field("trovs", &self.trovs())
                .field("rovsm", &self.rovsm())
                .field("gcomp", &self.gcomp())
                .field("swtrig", &self.swtrig())
                .field("bulb", &self.bulb())
                .field("smptrig", &self.smptrig())
                .field("lftrig", &self.lftrig())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfgr2 {{ rovse: {=bool:?}, jovse: {=bool:?}, ovsr: {:?}, ovss: {=u8:?}, trovs: {:?}, rovsm: {:?}, gcomp: {=bool:?}, swtrig: {:?}, bulb: {=bool:?}, smptrig: {=bool:?}, lftrig: {=bool:?} }}",
                self.rovse(),
                self.jovse(),
                self.ovsr(),
                self.ovss(),
                self.trovs(),
                self.rovsm(),
                self.gcomp(),
                self.swtrig(),
                self.bulb(),
                self.smptrig(),
                self.lftrig()
            )
        }
    }
    #[doc = "Control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "ADC enable"]
        #[must_use]
        #[inline(always)]
        pub const fn aden(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADC enable"]
        #[inline(always)]
        pub const fn set_aden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ADC disable"]
        #[must_use]
        #[inline(always)]
        pub const fn addis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ADC disable"]
        #[inline(always)]
        pub const fn set_addis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Regular conversions start"]
        #[must_use]
        #[inline(always)]
        pub const fn adstart(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Regular conversions start"]
        #[inline(always)]
        pub const fn set_adstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Injected conversions start"]
        #[must_use]
        #[inline(always)]
        pub const fn jadstart(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Injected conversions start"]
        #[inline(always)]
        pub const fn set_jadstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Regular conversions stop"]
        #[must_use]
        #[inline(always)]
        pub const fn adstp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Regular conversions stop"]
        #[inline(always)]
        pub const fn set_adstp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Injected conversions stop"]
        #[must_use]
        #[inline(always)]
        pub const fn jadstp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Injected conversions stop"]
        #[inline(always)]
        pub const fn set_jadstp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ADC voltage regulator enable"]
        #[must_use]
        #[inline(always)]
        pub const fn advregen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "ADC voltage regulator enable"]
        #[inline(always)]
        pub const fn set_advregen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Deep-power-down enable"]
        #[must_use]
        #[inline(always)]
        pub const fn deeppwd(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Deep-power-down enable"]
        #[inline(always)]
        pub const fn set_deeppwd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Differential mode for calibration (0: single-ended, 1: differential)"]
        #[must_use]
        #[inline(always)]
        pub const fn adcaldif(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Differential mode for calibration (0: single-ended, 1: differential)"]
        #[inline(always)]
        pub const fn set_adcaldif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "ADC calibration"]
        #[must_use]
        #[inline(always)]
        pub const fn adcal(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "ADC calibration"]
        #[inline(always)]
        pub const fn set_adcal(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    impl core::fmt::Debug for Cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr")
                .field("aden", &self.aden())
                .field("addis", &self.addis())
                .field("adstart", &self.adstart())
                .field("jadstart", &self.jadstart())
                .field("adstp", &self.adstp())
                .field("jadstp", &self.jadstp())
                .field("advregen", &self.advregen())
                .field("deeppwd", &self.deeppwd())
                .field("adcaldif", &self.adcaldif())
                .field("adcal", &self.adcal())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ aden: {=bool:?}, addis: {=bool:?}, adstart: {=bool:?}, jadstart: {=bool:?}, adstp: {=bool:?}, jadstp: {=bool:?}, advregen: {=bool:?}, deeppwd: {=bool:?}, adcaldif: {=bool:?}, adcal: {=bool:?} }}",
                self.aden(),
                self.addis(),
                self.adstart(),
                self.jadstart(),
                self.adstp(),
                self.jadstp(),
                self.advregen(),
                self.deeppwd(),
                self.adcaldif(),
                self.adcal()
            )
        }
    }
    #[doc = "Differential mode selection register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Difsel(pub u32);
    impl Difsel {
        #[doc = "Differential mode for channel x (0: single-ended, 1: differential)"]
        #[must_use]
        #[inline(always)]
        pub const fn difsel(&self, n: usize) -> bool {
            assert!(n < 20usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Differential mode for channel x (0: single-ended, 1: differential)"]
        #[inline(always)]
        pub const fn set_difsel(&mut self, n: usize, val: bool) {
            assert!(n < 20usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Difsel {
        #[inline(always)]
        fn default() -> Difsel {
            Difsel(0)
        }
    }
    impl core::fmt::Debug for Difsel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Difsel")
                .field("difsel[0]", &self.difsel(0usize))
                .field("difsel[1]", &self.difsel(1usize))
                .field("difsel[2]", &self.difsel(2usize))
                .field("difsel[3]", &self.difsel(3usize))
                .field("difsel[4]", &self.difsel(4usize))
                .field("difsel[5]", &self.difsel(5usize))
                .field("difsel[6]", &self.difsel(6usize))
                .field("difsel[7]", &self.difsel(7usize))
                .field("difsel[8]", &self.difsel(8usize))
                .field("difsel[9]", &self.difsel(9usize))
                .field("difsel[10]", &self.difsel(10usize))
                .field("difsel[11]", &self.difsel(11usize))
                .field("difsel[12]", &self.difsel(12usize))
                .field("difsel[13]", &self.difsel(13usize))
                .field("difsel[14]", &self.difsel(14usize))
                .field("difsel[15]", &self.difsel(15usize))
                .field("difsel[16]", &self.difsel(16usize))
                .field("difsel[17]", &self.difsel(17usize))
                .field("difsel[18]", &self.difsel(18usize))
                .field("difsel[19]", &self.difsel(19usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Difsel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Difsel {{ difsel[0]: {=bool:?}, difsel[1]: {=bool:?}, difsel[2]: {=bool:?}, difsel[3]: {=bool:?}, difsel[4]: {=bool:?}, difsel[5]: {=bool:?}, difsel[6]: {=bool:?}, difsel[7]: {=bool:?}, difsel[8]: {=bool:?}, difsel[9]: {=bool:?}, difsel[10]: {=bool:?}, difsel[11]: {=bool:?}, difsel[12]: {=bool:?}, difsel[13]: {=bool:?}, difsel[14]: {=bool:?}, difsel[15]: {=bool:?}, difsel[16]: {=bool:?}, difsel[17]: {=bool:?}, difsel[18]: {=bool:?}, difsel[19]: {=bool:?} }}",
                self.difsel(0usize),
                self.difsel(1usize),
                self.difsel(2usize),
                self.difsel(3usize),
                self.difsel(4usize),
                self.difsel(5usize),
                self.difsel(6usize),
                self.difsel(7usize),
                self.difsel(8usize),
                self.difsel(9usize),
                self.difsel(10usize),
                self.difsel(11usize),
                self.difsel(12usize),
                self.difsel(13usize),
                self.difsel(14usize),
                self.difsel(15usize),
                self.difsel(16usize),
                self.difsel(17usize),
                self.difsel(18usize),
                self.difsel(19usize)
            )
        }
    }
    #[doc = "Regular data register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr(pub u32);
    impl Dr {
        #[doc = "Regular data converted"]
        #[must_use]
        #[inline(always)]
        pub const fn rdata(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Regular data converted"]
        #[inline(always)]
        pub const fn set_rdata(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dr {
        #[inline(always)]
        fn default() -> Dr {
            Dr(0)
        }
    }
    impl core::fmt::Debug for Dr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dr").field("rdata", &self.rdata()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dr {{ rdata: {=u16:?} }}", self.rdata())
        }
    }
    #[doc = "Gain compensation register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gcomp(pub u32);
    impl Gcomp {
        #[doc = "Gain compensation coefficient"]
        #[must_use]
        #[inline(always)]
        pub const fn gcompcoeff(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "Gain compensation coefficient"]
        #[inline(always)]
        pub const fn set_gcompcoeff(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
    }
    impl Default for Gcomp {
        #[inline(always)]
        fn default() -> Gcomp {
            Gcomp(0)
        }
    }
    impl core::fmt::Debug for Gcomp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gcomp").field("gcompcoeff", &self.gcompcoeff()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gcomp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Gcomp {{ gcompcoeff: {=u16:?} }}", self.gcompcoeff())
        }
    }
    #[doc = "Interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "ADC ready interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn adrdyie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADC ready interrupt enable"]
        #[inline(always)]
        pub const fn set_adrdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "End of sampling interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn eosmpie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "End of sampling interrupt enable"]
        #[inline(always)]
        pub const fn set_eosmpie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "End of regular conversion interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn eocie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "End of regular conversion interrupt enable"]
        #[inline(always)]
        pub const fn set_eocie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "End of regular sequence interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn eosie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "End of regular sequence interrupt enable"]
        #[inline(always)]
        pub const fn set_eosie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Overrun interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ovrie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun interrupt enable"]
        #[inline(always)]
        pub const fn set_ovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "End of injected conversion interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn jeocie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "End of injected conversion interrupt enable"]
        #[inline(always)]
        pub const fn set_jeocie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "End of injected sequence interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn jeosie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "End of injected sequence interrupt enable"]
        #[inline(always)]
        pub const fn set_jeosie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Analog watchdog interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn awdie(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog interrupt enable"]
        #[inline(always)]
        pub const fn set_awdie(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Injected context queue overflow interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn jqovfie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Injected context queue overflow interrupt enable"]
        #[inline(always)]
        pub const fn set_jqovfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Ier {
        #[inline(always)]
        fn default() -> Ier {
            Ier(0)
        }
    }
    impl core::fmt::Debug for Ier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ier")
                .field("adrdyie", &self.adrdyie())
                .field("eosmpie", &self.eosmpie())
                .field("eocie", &self.eocie())
                .field("eosie", &self.eosie())
                .field("ovrie", &self.ovrie())
                .field("jeocie", &self.jeocie())
                .field("jeosie", &self.jeosie())
                .field("awdie[0]", &self.awdie(0usize))
                .field("awdie[1]", &self.awdie(1usize))
                .field("awdie[2]", &self.awdie(2usize))
                .field("jqovfie", &self.jqovfie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ier {{ adrdyie: {=bool:?}, eosmpie: {=bool:?}, eocie: {=bool:?}, eosie: {=bool:?}, ovrie: {=bool:?}, jeocie: {=bool:?}, jeosie: {=bool:?}, awdie[0]: {=bool:?}, awdie[1]: {=bool:?}, awdie[2]: {=bool:?}, jqovfie: {=bool:?} }}",
                self.adrdyie(),
                self.eosmpie(),
                self.eocie(),
                self.eosie(),
                self.ovrie(),
                self.jeocie(),
                self.jeosie(),
                self.awdie(0usize),
                self.awdie(1usize),
                self.awdie(2usize),
                self.jqovfie()
            )
        }
    }
    #[doc = "Interrupt and status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "ADC ready flag"]
        #[must_use]
        #[inline(always)]
        pub const fn adrdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADC ready flag"]
        #[inline(always)]
        pub const fn set_adrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "End of sampling flag"]
        #[must_use]
        #[inline(always)]
        pub const fn eosmp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "End of sampling flag"]
        #[inline(always)]
        pub const fn set_eosmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "End of regular conversion flag"]
        #[must_use]
        #[inline(always)]
        pub const fn eoc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "End of regular conversion flag"]
        #[inline(always)]
        pub const fn set_eoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "End of regular sequence flag"]
        #[must_use]
        #[inline(always)]
        pub const fn eos(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "End of regular sequence flag"]
        #[inline(always)]
        pub const fn set_eos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Overrun flag"]
        #[must_use]
        #[inline(always)]
        pub const fn ovr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun flag"]
        #[inline(always)]
        pub const fn set_ovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "End of injected conversion flag"]
        #[must_use]
        #[inline(always)]
        pub const fn jeoc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "End of injected conversion flag"]
        #[inline(always)]
        pub const fn set_jeoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "End of injected sequence flag"]
        #[must_use]
        #[inline(always)]
        pub const fn jeos(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "End of injected sequence flag"]
        #[inline(always)]
        pub const fn set_jeos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Analog watchdog flag"]
        #[must_use]
        #[inline(always)]
        pub const fn awd(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog flag"]
        #[inline(always)]
        pub const fn set_awd(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Injected context queue overflow flag"]
        #[must_use]
        #[inline(always)]
        pub const fn jqovf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Injected context queue overflow flag"]
        #[inline(always)]
        pub const fn set_jqovf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Isr {
        #[inline(always)]
        fn default() -> Isr {
            Isr(0)
        }
    }
    impl core::fmt::Debug for Isr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Isr")
                .field("adrdy", &self.adrdy())
                .field("eosmp", &self.eosmp())
                .field("eoc", &self.eoc())
                .field("eos", &self.eos())
                .field("ovr", &self.ovr())
                .field("jeoc", &self.jeoc())
                .field("jeos", &self.jeos())
                .field("awd[0]", &self.awd(0usize))
                .field("awd[1]", &self.awd(1usize))
                .field("awd[2]", &self.awd(2usize))
                .field("jqovf", &self.jqovf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Isr {{ adrdy: {=bool:?}, eosmp: {=bool:?}, eoc: {=bool:?}, eos: {=bool:?}, ovr: {=bool:?}, jeoc: {=bool:?}, jeos: {=bool:?}, awd[0]: {=bool:?}, awd[1]: {=bool:?}, awd[2]: {=bool:?}, jqovf: {=bool:?} }}",
                self.adrdy(),
                self.eosmp(),
                self.eoc(),
                self.eos(),
                self.ovr(),
                self.jeoc(),
                self.jeos(),
                self.awd(0usize),
                self.awd(1usize),
                self.awd(2usize),
                self.jqovf()
            )
        }
    }
    #[doc = "Injected data register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jdr(pub u32);
    impl Jdr {
        #[doc = "Injected data"]
        #[must_use]
        #[inline(always)]
        pub const fn jdata(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Injected data"]
        #[inline(always)]
        pub const fn set_jdata(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Jdr {
        #[inline(always)]
        fn default() -> Jdr {
            Jdr(0)
        }
    }
    impl core::fmt::Debug for Jdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Jdr").field("jdata", &self.jdata()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Jdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Jdr {{ jdata: {=u16:?} }}", self.jdata())
        }
    }
    #[doc = "Injected sequence register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jsqr(pub u32);
    impl Jsqr {
        #[doc = "Injected channel sequence length"]
        #[must_use]
        #[inline(always)]
        pub const fn jl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Injected channel sequence length"]
        #[inline(always)]
        pub const fn set_jl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "External trigger selection for injected group"]
        #[must_use]
        #[inline(always)]
        pub const fn jextsel(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x1f;
            val as u8
        }
        #[doc = "External trigger selection for injected group"]
        #[inline(always)]
        pub const fn set_jextsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 2usize)) | (((val as u32) & 0x1f) << 2usize);
        }
        #[doc = "External trigger enable and polarity selection for injected channels"]
        #[must_use]
        #[inline(always)]
        pub const fn jexten(&self) -> super::vals::Exten {
            let val = (self.0 >> 7usize) & 0x03;
            super::vals::Exten::from_bits(val as u8)
        }
        #[doc = "External trigger enable and polarity selection for injected channels"]
        #[inline(always)]
        pub const fn set_jexten(&mut self, val: super::vals::Exten) {
            self.0 = (self.0 & !(0x03 << 7usize)) | (((val.to_bits() as u32) & 0x03) << 7usize);
        }
        #[doc = "Conversion in the injected sequence"]
        #[must_use]
        #[inline(always)]
        pub const fn jsq(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 9usize + n * 6usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "Conversion in the injected sequence"]
        #[inline(always)]
        pub const fn set_jsq(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 9usize + n * 6usize;
            self.0 = (self.0 & !(0x1f << offs)) | (((val as u32) & 0x1f) << offs);
        }
    }
    impl Default for Jsqr {
        #[inline(always)]
        fn default() -> Jsqr {
            Jsqr(0)
        }
    }
    impl core::fmt::Debug for Jsqr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Jsqr")
                .field("jl", &self.jl())
                .field("jextsel", &self.jextsel())
                .field("jexten", &self.jexten())
                .field("jsq[0]", &self.jsq(0usize))
                .field("jsq[1]", &self.jsq(1usize))
                .field("jsq[2]", &self.jsq(2usize))
                .field("jsq[3]", &self.jsq(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Jsqr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Jsqr {{ jl: {=u8:?}, jextsel: {=u8:?}, jexten: {:?}, jsq[0]: {=u8:?}, jsq[1]: {=u8:?}, jsq[2]: {=u8:?}, jsq[3]: {=u8:?} }}",
                self.jl(),
                self.jextsel(),
                self.jexten(),
                self.jsq(0usize),
                self.jsq(1usize),
                self.jsq(2usize),
                self.jsq(3usize)
            )
        }
    }
    #[doc = "Offset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ofr(pub u32);
    impl Ofr {
        #[doc = "Data offset y for the channel programmed into OFFSET_CH"]
        #[must_use]
        #[inline(always)]
        pub const fn offset(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Data offset y for the channel programmed into OFFSET_CH"]
        #[inline(always)]
        pub const fn set_offset(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Positive offset (0: the offset is subtracted, 1: the offset is added)"]
        #[must_use]
        #[inline(always)]
        pub const fn offsetpos(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Positive offset (0: the offset is subtracted, 1: the offset is added)"]
        #[inline(always)]
        pub const fn set_offsetpos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Saturation enable"]
        #[must_use]
        #[inline(always)]
        pub const fn saten(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Saturation enable"]
        #[inline(always)]
        pub const fn set_saten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Channel selection for the data offset y"]
        #[must_use]
        #[inline(always)]
        pub const fn offset_ch(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x1f;
            val as u8
        }
        #[doc = "Channel selection for the data offset y"]
        #[inline(always)]
        pub const fn set_offset_ch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
        }
        #[doc = "Offset y enable"]
        #[must_use]
        #[inline(always)]
        pub const fn offset_en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Offset y enable"]
        #[inline(always)]
        pub const fn set_offset_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ofr {
        #[inline(always)]
        fn default() -> Ofr {
            Ofr(0)
        }
    }
    impl core::fmt::Debug for Ofr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ofr")
                .field("offset", &self.offset())
                .field("offsetpos", &self.offsetpos())
                .field("saten", &self.saten())
                .field("offset_ch", &self.offset_ch())
                .field("offset_en", &self.offset_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ofr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ofr {{ offset: {=u16:?}, offsetpos: {=bool:?}, saten: {=bool:?}, offset_ch: {=u8:?}, offset_en: {=bool:?} }}",
                self.offset(),
                self.offsetpos(),
                self.saten(),
                self.offset_ch(),
                self.offset_en()
            )
        }
    }
    #[doc = "Option register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Or(pub u32);
    impl Or {
        #[doc = "Option bit 0 (INP0/INN1 GPIO switch control on ADC1, VDDCORE channel enable on ADC2)"]
        #[must_use]
        #[inline(always)]
        pub const fn op0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Option bit 0 (INP0/INN1 GPIO switch control on ADC1, VDDCORE channel enable on ADC2)"]
        #[inline(always)]
        pub const fn set_op0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Option bit 1"]
        #[must_use]
        #[inline(always)]
        pub const fn op1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Option bit 1"]
        #[inline(always)]
        pub const fn set_op1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Or {
        #[inline(always)]
        fn default() -> Or {
            Or(0)
        }
    }
    impl core::fmt::Debug for Or {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Or")
                .field("op0", &self.op0())
                .field("op1", &self.op1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Or {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Or {{ op0: {=bool:?}, op1: {=bool:?} }}", self.op0(), self.op1())
        }
    }
    #[doc = "Sample time register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smpr(pub u32);
    impl Smpr {
        #[doc = "Channel x sampling time selection"]
        #[must_use]
        #[inline(always)]
        pub const fn smp(&self, n: usize) -> super::vals::SampleTime {
            assert!(n < 10usize);
            let offs = 0usize + n * 3usize;
            let val = (self.0 >> offs) & 0x07;
            super::vals::SampleTime::from_bits(val as u8)
        }
        #[doc = "Channel x sampling time selection"]
        #[inline(always)]
        pub const fn set_smp(&mut self, n: usize, val: super::vals::SampleTime) {
            assert!(n < 10usize);
            let offs = 0usize + n * 3usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val.to_bits() as u32) & 0x07) << offs);
        }
        #[doc = "Addition of one clock cycle to the sampling time (SMPR\\[0\\]
only)"]
        #[must_use]
        #[inline(always)]
        pub const fn smpplus(&self) -> super::vals::Smpplus {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Smpplus::from_bits(val as u8)
        }
        #[doc = "Addition of one clock cycle to the sampling time (SMPR\\[0\\]
only)"]
        #[inline(always)]
        pub const fn set_smpplus(&mut self, val: super::vals::Smpplus) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Smpr {
        #[inline(always)]
        fn default() -> Smpr {
            Smpr(0)
        }
    }
    impl core::fmt::Debug for Smpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Smpr")
                .field("smp[0]", &self.smp(0usize))
                .field("smp[1]", &self.smp(1usize))
                .field("smp[2]", &self.smp(2usize))
                .field("smp[3]", &self.smp(3usize))
                .field("smp[4]", &self.smp(4usize))
                .field("smp[5]", &self.smp(5usize))
                .field("smp[6]", &self.smp(6usize))
                .field("smp[7]", &self.smp(7usize))
                .field("smp[8]", &self.smp(8usize))
                .field("smp[9]", &self.smp(9usize))
                .field("smpplus", &self.smpplus())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Smpr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Smpr {{ smp[0]: {:?}, smp[1]: {:?}, smp[2]: {:?}, smp[3]: {:?}, smp[4]: {:?}, smp[5]: {:?}, smp[6]: {:?}, smp[7]: {:?}, smp[8]: {:?}, smp[9]: {:?}, smpplus: {:?} }}",
                self.smp(0usize),
                self.smp(1usize),
                self.smp(2usize),
                self.smp(3usize),
                self.smp(4usize),
                self.smp(5usize),
                self.smp(6usize),
                self.smp(7usize),
                self.smp(8usize),
                self.smp(9usize),
                self.smpplus()
            )
        }
    }
    #[doc = "Regular sequence register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqr1(pub u32);
    impl Sqr1 {
        #[doc = "Regular channel sequence length"]
        #[must_use]
        #[inline(always)]
        pub const fn l(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Regular channel sequence length"]
        #[inline(always)]
        pub const fn set_l(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Conversion in regular sequence"]
        #[must_use]
        #[inline(always)]
        pub const fn sq(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 6usize + n * 6usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "Conversion in regular sequence"]
        #[inline(always)]
        pub const fn set_sq(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 6usize + n * 6usize;
            self.0 = (self.0 & !(0x1f << offs)) | (((val as u32) & 0x1f) << offs);
        }
    }
    impl Default for Sqr1 {
        #[inline(always)]
        fn default() -> Sqr1 {
            Sqr1(0)
        }
    }
    impl core::fmt::Debug for Sqr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sqr1")
                .field("l", &self.l())
                .field("sq[0]", &self.sq(0usize))
                .field("sq[1]", &self.sq(1usize))
                .field("sq[2]", &self.sq(2usize))
                .field("sq[3]", &self.sq(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sqr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sqr1 {{ l: {=u8:?}, sq[0]: {=u8:?}, sq[1]: {=u8:?}, sq[2]: {=u8:?}, sq[3]: {=u8:?} }}",
                self.l(),
                self.sq(0usize),
                self.sq(1usize),
                self.sq(2usize),
                self.sq(3usize)
            )
        }
    }
    #[doc = "Regular sequence register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqr2(pub u32);
    impl Sqr2 {
        #[doc = "Conversion in regular sequence"]
        #[must_use]
        #[inline(always)]
        pub const fn sq(&self, n: usize) -> u8 {
            assert!(n < 5usize);
            let offs = 0usize + n * 6usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "Conversion in regular sequence"]
        #[inline(always)]
        pub const fn set_sq(&mut self, n: usize, val: u8) {
            assert!(n < 5usize);
            let offs = 0usize + n * 6usize;
            self.0 = (self.0 & !(0x1f << offs)) | (((val as u32) & 0x1f) << offs);
        }
    }
    impl Default for Sqr2 {
        #[inline(always)]
        fn default() -> Sqr2 {
            Sqr2(0)
        }
    }
    impl core::fmt::Debug for Sqr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sqr2")
                .field("sq[0]", &self.sq(0usize))
                .field("sq[1]", &self.sq(1usize))
                .field("sq[2]", &self.sq(2usize))
                .field("sq[3]", &self.sq(3usize))
                .field("sq[4]", &self.sq(4usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sqr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sqr2 {{ sq[0]: {=u8:?}, sq[1]: {=u8:?}, sq[2]: {=u8:?}, sq[3]: {=u8:?}, sq[4]: {=u8:?} }}",
                self.sq(0usize),
                self.sq(1usize),
                self.sq(2usize),
                self.sq(3usize),
                self.sq(4usize)
            )
        }
    }
    #[doc = "Regular sequence register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqr3(pub u32);
    impl Sqr3 {
        #[doc = "Conversion in regular sequence"]
        #[must_use]
        #[inline(always)]
        pub const fn sq(&self, n: usize) -> u8 {
            assert!(n < 5usize);
            let offs = 0usize + n * 6usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "Conversion in regular sequence"]
        #[inline(always)]
        pub const fn set_sq(&mut self, n: usize, val: u8) {
            assert!(n < 5usize);
            let offs = 0usize + n * 6usize;
            self.0 = (self.0 & !(0x1f << offs)) | (((val as u32) & 0x1f) << offs);
        }
    }
    impl Default for Sqr3 {
        #[inline(always)]
        fn default() -> Sqr3 {
            Sqr3(0)
        }
    }
    impl core::fmt::Debug for Sqr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sqr3")
                .field("sq[0]", &self.sq(0usize))
                .field("sq[1]", &self.sq(1usize))
                .field("sq[2]", &self.sq(2usize))
                .field("sq[3]", &self.sq(3usize))
                .field("sq[4]", &self.sq(4usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sqr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sqr3 {{ sq[0]: {=u8:?}, sq[1]: {=u8:?}, sq[2]: {=u8:?}, sq[3]: {=u8:?}, sq[4]: {=u8:?} }}",
                self.sq(0usize),
                self.sq(1usize),
                self.sq(2usize),
                self.sq(3usize),
                self.sq(4usize)
            )
        }
    }
    #[doc = "Regular sequence register 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sqr4(pub u32);
    impl Sqr4 {
        #[doc = "Conversion in regular sequence"]
        #[must_use]
        #[inline(always)]
        pub const fn sq(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 0usize + n * 6usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "Conversion in regular sequence"]
        #[inline(always)]
        pub const fn set_sq(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 0usize + n * 6usize;
            self.0 = (self.0 & !(0x1f << offs)) | (((val as u32) & 0x1f) << offs);
        }
    }
    impl Default for Sqr4 {
        #[inline(always)]
        fn default() -> Sqr4 {
            Sqr4(0)
        }
    }
    impl core::fmt::Debug for Sqr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sqr4")
                .field("sq[0]", &self.sq(0usize))
                .field("sq[1]", &self.sq(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sqr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sqr4 {{ sq[0]: {=u8:?}, sq[1]: {=u8:?} }}",
                self.sq(0usize),
                self.sq(1usize)
            )
        }
    }
    #[doc = "Analog watchdog threshold register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tr(pub u32);
    impl Tr {
        #[doc = "Analog watchdog lower threshold"]
        #[must_use]
        #[inline(always)]
        pub const fn lt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Analog watchdog lower threshold"]
        #[inline(always)]
        pub const fn set_lt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Analog watchdog filtering parameter (TR\\[0\\]
only)"]
        #[must_use]
        #[inline(always)]
        pub const fn awdfilt(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "Analog watchdog filtering parameter (TR\\[0\\]
only)"]
        #[inline(always)]
        pub const fn set_awdfilt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "Analog watchdog higher threshold"]
        #[must_use]
        #[inline(always)]
        pub const fn ht(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Analog watchdog higher threshold"]
        #[inline(always)]
        pub const fn set_ht(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Tr {
        #[inline(always)]
        fn default() -> Tr {
            Tr(0)
        }
    }
    impl core::fmt::Debug for Tr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tr")
                .field("lt", &self.lt())
                .field("awdfilt", &self.awdfilt())
                .field("ht", &self.ht())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Tr {{ lt: {=u16:?}, awdfilt: {=u8:?}, ht: {=u16:?} }}",
                self.lt(),
                self.awdfilt(),
                self.ht()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Align {
        #[doc = "Right alignment"]
        Right = 0x0,
        #[doc = "Left alignment"]
        Left = 0x01,
    }
    impl Align {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Align {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Align {
        #[inline(always)]
        fn from(val: u8) -> Align {
            Align::from_bits(val)
        }
    }
    impl From<Align> for u8 {
        #[inline(always)]
        fn from(val: Align) -> u8 {
            Align::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dmacfg {
        #[doc = "DMA one shot mode selected"]
        OneShot = 0x0,
        #[doc = "DMA circular mode selected"]
        Circular = 0x01,
    }
    impl Dmacfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dmacfg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dmacfg {
        #[inline(always)]
        fn from(val: u8) -> Dmacfg {
            Dmacfg::from_bits(val)
        }
    }
    impl From<Dmacfg> for u8 {
        #[inline(always)]
        fn from(val: Dmacfg) -> u8 {
            Dmacfg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Exten {
        #[doc = "Hardware trigger detection disabled (conversions can be launched by software)"]
        Disabled = 0x0,
        #[doc = "Hardware trigger detection on the rising edge"]
        RisingEdge = 0x01,
        #[doc = "Hardware trigger detection on the falling edge"]
        FallingEdge = 0x02,
        #[doc = "Hardware trigger detection on both the rising and falling edges"]
        BothEdges = 0x03,
    }
    impl Exten {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Exten {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Exten {
        #[inline(always)]
        fn from(val: u8) -> Exten {
            Exten::from_bits(val)
        }
    }
    impl From<Exten> for u8 {
        #[inline(always)]
        fn from(val: Exten) -> u8 {
            Exten::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ovsr {
        #[doc = "2x oversampling ratio"]
        Mul2 = 0x0,
        #[doc = "4x oversampling ratio"]
        Mul4 = 0x01,
        #[doc = "8x oversampling ratio"]
        Mul8 = 0x02,
        #[doc = "16x oversampling ratio"]
        Mul16 = 0x03,
        #[doc = "32x oversampling ratio"]
        Mul32 = 0x04,
        #[doc = "64x oversampling ratio"]
        Mul64 = 0x05,
        #[doc = "128x oversampling ratio"]
        Mul128 = 0x06,
        #[doc = "256x oversampling ratio"]
        Mul256 = 0x07,
    }
    impl Ovsr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ovsr {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ovsr {
        #[inline(always)]
        fn from(val: u8) -> Ovsr {
            Ovsr::from_bits(val)
        }
    }
    impl From<Ovsr> for u8 {
        #[inline(always)]
        fn from(val: Ovsr) -> u8 {
            Ovsr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Res {
        #[doc = "12-bit"]
        Bits12 = 0x0,
        #[doc = "10-bit"]
        Bits10 = 0x01,
        #[doc = "8-bit"]
        Bits8 = 0x02,
        #[doc = "6-bit"]
        Bits6 = 0x03,
    }
    impl Res {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Res {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Res {
        #[inline(always)]
        fn from(val: u8) -> Res {
            Res::from_bits(val)
        }
    }
    impl From<Res> for u8 {
        #[inline(always)]
        fn from(val: Res) -> u8 {
            Res::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rovsm {
        #[doc = "Oversampling is temporarily stopped and continued after the injection sequence (intermediate results are kept)"]
        Continued = 0x0,
        #[doc = "Oversampling is aborted and resumed from start after the injection sequence (intermediate results are discarded)"]
        Resumed = 0x01,
    }
    impl Rovsm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rovsm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rovsm {
        #[inline(always)]
        fn from(val: u8) -> Rovsm {
            Rovsm::from_bits(val)
        }
    }
    impl From<Rovsm> for u8 {
        #[inline(always)]
        fn from(val: Rovsm) -> u8 {
            Rovsm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum SampleTime {
        #[doc = "2.5 ADC clock cycles"]
        Cycles25 = 0x0,
        #[doc = "6.5 ADC clock cycles"]
        Cycles65 = 0x01,
        #[doc = "12.5 ADC clock cycles"]
        Cycles125 = 0x02,
        #[doc = "24.5 ADC clock cycles"]
        Cycles245 = 0x03,
        #[doc = "47.5 ADC clock cycles"]
        Cycles475 = 0x04,
        #[doc = "92.5 ADC clock cycles"]
        Cycles925 = 0x05,
        #[doc = "247.5 ADC clock cycles"]
        Cycles2475 = 0x06,
        #[doc = "640.5 ADC clock cycles"]
        Cycles6405 = 0x07,
    }
    impl SampleTime {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SampleTime {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SampleTime {
        #[inline(always)]
        fn from(val: u8) -> SampleTime {
            SampleTime::from_bits(val)
        }
    }
    impl From<SampleTime> for u8 {
        #[inline(always)]
        fn from(val: SampleTime) -> u8 {
            SampleTime::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Smpplus {
        #[doc = "SMP = 000 selects 2.5 ADC clock cycles"]
        Cycles25 = 0x0,
        #[doc = "SMP = 000 selects 3.5 ADC clock cycles"]
        Cycles35 = 0x01,
    }
    impl Smpplus {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Smpplus {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Smpplus {
        #[inline(always)]
        fn from(val: u8) -> Smpplus {
            Smpplus::from_bits(val)
        }
    }
    impl From<Smpplus> for u8 {
        #[inline(always)]
        fn from(val: Smpplus) -> u8 {
            Smpplus::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Swtrig {
        #[doc = "Software trigger starts the conversion"]
        Conversion = 0x0,
        #[doc = "Software trigger starts the sampling phase (the conversion starts when the trigger is cleared)"]
        Sampling = 0x01,
    }
    impl Swtrig {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Swtrig {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Swtrig {
        #[inline(always)]
        fn from(val: u8) -> Swtrig {
            Swtrig::from_bits(val)
        }
    }
    impl From<Swtrig> for u8 {
        #[inline(always)]
        fn from(val: Swtrig) -> u8 {
            Swtrig::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Trovs {
        #[doc = "All oversampled conversions for a channel are done consecutively after a trigger"]
        Automatic = 0x0,
        #[doc = "Each oversampled conversion for a channel needs a trigger"]
        Triggered = 0x01,
    }
    impl Trovs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Trovs {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Trovs {
        #[inline(always)]
        fn from(val: u8) -> Trovs {
            Trovs::from_bits(val)
        }
    }
    impl From<Trovs> for u8 {
        #[inline(always)]
        fn from(val: Trovs) -> u8 {
            Trovs::to_bits(val)
        }
    }
}
