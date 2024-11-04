#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "ADC common registers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdcCommon {
    ptr: *mut u8,
}
unsafe impl Send for AdcCommon {}
unsafe impl Sync for AdcCommon {}
impl AdcCommon {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "ADC Common status register"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "ADC common control register"]
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::Ccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "ADC common regular data register for dual and triple modes"]
    #[inline(always)]
    pub const fn cdr(self) -> crate::common::Reg<regs::Cdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
pub mod regs {
    #[doc = "ADC common control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr(pub u32);
    impl Ccr {
        #[doc = "Multi ADC mode selection"]
        #[inline(always)]
        pub const fn multi(&self) -> super::vals::Multi {
            let val = (self.0 >> 0usize) & 0x1f;
            super::vals::Multi::from_bits(val as u8)
        }
        #[doc = "Multi ADC mode selection"]
        #[inline(always)]
        pub fn set_multi(&mut self, val: super::vals::Multi) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
        }
        #[doc = "Delay between 2 sampling phases"]
        #[inline(always)]
        pub const fn delay(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Delay between 2 sampling phases"]
        #[inline(always)]
        pub fn set_delay(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "DMA disable selection for multi-ADC mode"]
        #[inline(always)]
        pub const fn dds(&self) -> super::vals::Dds {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::Dds::from_bits(val as u8)
        }
        #[doc = "DMA disable selection for multi-ADC mode"]
        #[inline(always)]
        pub fn set_dds(&mut self, val: super::vals::Dds) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
        #[doc = "Direct memory access mode for multi ADC mode"]
        #[inline(always)]
        pub const fn dma(&self) -> super::vals::Dma {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::Dma::from_bits(val as u8)
        }
        #[doc = "Direct memory access mode for multi ADC mode"]
        #[inline(always)]
        pub fn set_dma(&mut self, val: super::vals::Dma) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
        #[doc = "ADC prescaler"]
        #[inline(always)]
        pub const fn adcpre(&self) -> super::vals::Adcpre {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Adcpre::from_bits(val as u8)
        }
        #[doc = "ADC prescaler"]
        #[inline(always)]
        pub fn set_adcpre(&mut self, val: super::vals::Adcpre) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "VBAT enable"]
        #[inline(always)]
        pub const fn vbate(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "VBAT enable"]
        #[inline(always)]
        pub fn set_vbate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Temperature sensor and VREFINT enable"]
        #[inline(always)]
        pub const fn tsvrefe(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Temperature sensor and VREFINT enable"]
        #[inline(always)]
        pub fn set_tsvrefe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Ccr {
        #[inline(always)]
        fn default() -> Ccr {
            Ccr(0)
        }
    }
    #[doc = "ADC common regular data register for dual and triple modes"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cdr(pub u32);
    impl Cdr {
        #[doc = "1st data item of a pair of regular conversions"]
        #[inline(always)]
        pub const fn data(&self, n: usize) -> u16 {
            assert!(n < 2usize);
            let offs = 0usize + n * 16usize;
            let val = (self.0 >> offs) & 0xffff;
            val as u16
        }
        #[doc = "1st data item of a pair of regular conversions"]
        #[inline(always)]
        pub fn set_data(&mut self, n: usize, val: u16) {
            assert!(n < 2usize);
            let offs = 0usize + n * 16usize;
            self.0 = (self.0 & !(0xffff << offs)) | (((val as u32) & 0xffff) << offs);
        }
    }
    impl Default for Cdr {
        #[inline(always)]
        fn default() -> Cdr {
            Cdr(0)
        }
    }
    #[doc = "ADC common status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "Analog watchdog event occurred"]
        #[inline(always)]
        pub const fn awd(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog event occurred"]
        #[inline(always)]
        pub fn set_awd(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "End of conversion of ADC"]
        #[inline(always)]
        pub const fn eoc(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 1usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "End of conversion of ADC"]
        #[inline(always)]
        pub fn set_eoc(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 1usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Injected channel end of conversion of ADC"]
        #[inline(always)]
        pub const fn jeoc(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 2usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Injected channel end of conversion of ADC"]
        #[inline(always)]
        pub fn set_jeoc(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 2usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Injected channel conversion started"]
        #[inline(always)]
        pub const fn jstrt(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 3usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Injected channel conversion started"]
        #[inline(always)]
        pub fn set_jstrt(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 3usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "regular channel conversion started"]
        #[inline(always)]
        pub const fn strt(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 4usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "regular channel conversion started"]
        #[inline(always)]
        pub fn set_strt(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 4usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Overrun occurred"]
        #[inline(always)]
        pub const fn ovr(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 5usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Overrun occurred"]
        #[inline(always)]
        pub fn set_ovr(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 5usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Csr {
        #[inline(always)]
        fn default() -> Csr {
            Csr(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Adcpre {
        #[doc = "PCLK2 divided by 2"]
        DIV2 = 0x0,
        #[doc = "PCLK2 divided by 4"]
        DIV4 = 0x01,
        #[doc = "PCLK2 divided by 6"]
        DIV6 = 0x02,
        #[doc = "PCLK2 divided by 8"]
        DIV8 = 0x03,
    }
    impl Adcpre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adcpre {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adcpre {
        #[inline(always)]
        fn from(val: u8) -> Adcpre {
            Adcpre::from_bits(val)
        }
    }
    impl From<Adcpre> for u8 {
        #[inline(always)]
        fn from(val: Adcpre) -> u8 {
            Adcpre::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Dds {
        #[doc = "No new DMA request is issued after the last transfer"]
        SINGLE = 0x0,
        #[doc = "DMA requests are issued as long as data are converted and DMA=01, 10 or 11"]
        CONTINUOUS = 0x01,
    }
    impl Dds {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dds {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dds {
        #[inline(always)]
        fn from(val: u8) -> Dds {
            Dds::from_bits(val)
        }
    }
    impl From<Dds> for u8 {
        #[inline(always)]
        fn from(val: Dds) -> u8 {
            Dds::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Dma {
        #[doc = "DMA mode disabled"]
        DISABLED = 0x0,
        #[doc = "DMA mode 1 enabled (2 / 3 half-words one by one - 1 then 2 then 3)"]
        MODE1 = 0x01,
        #[doc = "DMA mode 2 enabled (2 / 3 half-words by pairs - 2&1 then 1&3 then 3&2)"]
        MODE2 = 0x02,
        #[doc = "DMA mode 3 enabled (2 / 3 half-words by pairs - 2&1 then 1&3 then 3&2)"]
        MODE3 = 0x03,
    }
    impl Dma {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dma {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dma {
        #[inline(always)]
        fn from(val: u8) -> Dma {
            Dma::from_bits(val)
        }
    }
    impl From<Dma> for u8 {
        #[inline(always)]
        fn from(val: Dma) -> u8 {
            Dma::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Multi {
        #[doc = "All the ADCs independent: independent mode"]
        INDEPENDENT = 0x0,
        #[doc = "Dual ADC1 and ADC2, combined regular and injected simultaneous mode"]
        DUALRJ = 0x01,
        #[doc = "Dual ADC1 and ADC2, combined regular and alternate trigger mode"]
        DUALRA = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        #[doc = "Dual ADC1 and ADC2, injected simultaneous mode only"]
        DUALJ = 0x05,
        #[doc = "Dual ADC1 and ADC2, regular simultaneous mode only"]
        DUALR = 0x06,
        #[doc = "Dual ADC1 and ADC2, interleaved mode only"]
        DUALI = 0x07,
        _RESERVED_8 = 0x08,
        #[doc = "Dual ADC1 and ADC2, alternate trigger mode only"]
        DUALA = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
        _RESERVED_10 = 0x10,
        #[doc = "Triple ADC, regular and injected simultaneous mode"]
        TRIPLERJ = 0x11,
        #[doc = "Triple ADC, regular and alternate trigger mode"]
        TRIPLERA = 0x12,
        _RESERVED_13 = 0x13,
        _RESERVED_14 = 0x14,
        #[doc = "Triple ADC, injected simultaneous mode only"]
        TRIPLEJ = 0x15,
        #[doc = "Triple ADC, regular simultaneous mode only"]
        TRIPLER = 0x16,
        #[doc = "Triple ADC, interleaved mode only"]
        TRIPLEI = 0x17,
        #[doc = "Triple ADC, alternate trigger mode only"]
        TRIPLEA = 0x18,
        _RESERVED_19 = 0x19,
        _RESERVED_1a = 0x1a,
        _RESERVED_1b = 0x1b,
        _RESERVED_1c = 0x1c,
        _RESERVED_1d = 0x1d,
        _RESERVED_1e = 0x1e,
        _RESERVED_1f = 0x1f,
    }
    impl Multi {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Multi {
            unsafe { core::mem::transmute(val & 0x1f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Multi {
        #[inline(always)]
        fn from(val: u8) -> Multi {
            Multi::from_bits(val)
        }
    }
    impl From<Multi> for u8 {
        #[inline(always)]
        fn from(val: Multi) -> u8 {
            Multi::to_bits(val)
        }
    }
}
