#[doc = "Random number generator"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rng {
    ptr: *mut u8,
}
unsafe impl Send for Rng {}
unsafe impl Sync for Rng {}
impl Rng {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "data register"]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "health test control register"]
    #[inline(always)]
    pub const fn htcr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
}
pub mod regs {
    #[doc = "control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "True random number generator enable"]
        #[inline(always)]
        pub const fn rngen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "True random number generator enable"]
        #[inline(always)]
        pub fn set_rngen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Interrupt Enable"]
        #[inline(always)]
        pub const fn ie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt Enable"]
        #[inline(always)]
        pub fn set_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Clock error detection"]
        #[inline(always)]
        pub const fn ced(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Clock error detection"]
        #[inline(always)]
        pub fn set_ced(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "RNG configuration 3"]
        #[inline(always)]
        pub const fn rng_config3(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "RNG configuration 3"]
        #[inline(always)]
        pub fn set_rng_config3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Non NIST compliant"]
        #[inline(always)]
        pub const fn nistc(&self) -> super::vals::Nistc {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Nistc::from_bits(val as u8)
        }
        #[doc = "Non NIST compliant"]
        #[inline(always)]
        pub fn set_nistc(&mut self, val: super::vals::Nistc) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "RNG configuration 2"]
        #[inline(always)]
        pub const fn rng_config2(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "RNG configuration 2"]
        #[inline(always)]
        pub fn set_rng_config2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[doc = "Clock divider factor"]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Clock divider factor"]
        #[inline(always)]
        pub fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "RNG configuration 1"]
        #[inline(always)]
        pub const fn rng_config1(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x3f;
            val as u8
        }
        #[doc = "RNG configuration 1"]
        #[inline(always)]
        pub fn set_rng_config1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 20usize)) | (((val as u32) & 0x3f) << 20usize);
        }
        #[doc = "Conditioning soft reset"]
        #[inline(always)]
        pub const fn condrst(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Conditioning soft reset"]
        #[inline(always)]
        pub fn set_condrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Config Lock"]
        #[inline(always)]
        pub const fn configlock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Config Lock"]
        #[inline(always)]
        pub fn set_configlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    #[doc = "health test control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Htcr(pub u32);
    impl Htcr {
        #[doc = "health test configuration"]
        #[inline(always)]
        pub const fn htcfg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "health test configuration"]
        #[inline(always)]
        pub fn set_htcfg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Htcr {
        #[inline(always)]
        fn default() -> Htcr {
            Htcr(0)
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Data Ready"]
        #[inline(always)]
        pub const fn drdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Data Ready"]
        #[inline(always)]
        pub fn set_drdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clock error current status"]
        #[inline(always)]
        pub const fn cecs(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clock error current status"]
        #[inline(always)]
        pub fn set_cecs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Seed error current status"]
        #[inline(always)]
        pub const fn secs(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Seed error current status"]
        #[inline(always)]
        pub fn set_secs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Clock error interrupt status"]
        #[inline(always)]
        pub const fn ceis(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Clock error interrupt status"]
        #[inline(always)]
        pub fn set_ceis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Seed error interrupt status"]
        #[inline(always)]
        pub const fn seis(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Seed error interrupt status"]
        #[inline(always)]
        pub fn set_seis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Nistc {
        #[doc = "Hardware default values for NIST compliant RNG. In this configuration per 128-bit output two conditioning loops are performed and 256 bits of noise source are used"]
        DEFAULT = 0,
        #[doc = "Custom values for NIST compliant RNG"]
        CUSTOM = 0x01,
    }
    impl Nistc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Nistc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Nistc {
        #[inline(always)]
        fn from(val: u8) -> Nistc {
            Nistc::from_bits(val)
        }
    }
    impl From<Nistc> for u8 {
        #[inline(always)]
        fn from(val: Nistc) -> u8 {
            Nistc::to_bits(val)
        }
    }
}
