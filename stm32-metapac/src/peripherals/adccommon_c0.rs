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
    #[doc = "common configuration register"]
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::Ccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
}
pub mod regs {
    #[doc = "common configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr(pub u32);
    impl Ccr {
        #[doc = "prescaler"]
        #[must_use]
        #[inline(always)]
        pub const fn presc(&self) -> super::vals::Presc {
            let val = (self.0 >> 18usize) & 0x0f;
            super::vals::Presc::from_bits(val as u8)
        }
        #[doc = "prescaler"]
        #[inline(always)]
        pub const fn set_presc(&mut self, val: super::vals::Presc) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val.to_bits() as u32) & 0x0f) << 18usize);
        }
        #[doc = "VREFINT enable"]
        #[must_use]
        #[inline(always)]
        pub const fn vrefen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "VREFINT enable"]
        #[inline(always)]
        pub const fn set_vrefen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Temperature sensor enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tsen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Temperature sensor enable"]
        #[inline(always)]
        pub const fn set_tsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Ccr {
        #[inline(always)]
        fn default() -> Ccr {
            Ccr(0)
        }
    }
    impl core::fmt::Debug for Ccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccr")
                .field("presc", &self.presc())
                .field("vrefen", &self.vrefen())
                .field("tsen", &self.tsen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccr {{ presc: {:?}, vrefen: {=bool:?}, tsen: {=bool:?} }}",
                self.presc(),
                self.vrefen(),
                self.tsen()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Presc {
        #[doc = "adc_ker_ck_input not divided"]
        Div1 = 0x0,
        #[doc = "adc_ker_ck_input divided by 2"]
        Div2 = 0x01,
        #[doc = "adc_ker_ck_input divided by 4"]
        Div4 = 0x02,
        #[doc = "adc_ker_ck_input divided by 6"]
        Div6 = 0x03,
        #[doc = "adc_ker_ck_input divided by 8"]
        Div8 = 0x04,
        #[doc = "adc_ker_ck_input divided by 10"]
        Div10 = 0x05,
        #[doc = "adc_ker_ck_input divided by 12"]
        Div12 = 0x06,
        #[doc = "adc_ker_ck_input divided by 16"]
        Div16 = 0x07,
        #[doc = "adc_ker_ck_input divided by 32"]
        Div32 = 0x08,
        #[doc = "adc_ker_ck_input divided by 64"]
        Div64 = 0x09,
        #[doc = "adc_ker_ck_input divided by 128"]
        Div128 = 0x0a,
        #[doc = "adc_ker_ck_input divided by 256"]
        Div256 = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Presc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Presc {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Presc {
        #[inline(always)]
        fn from(val: u8) -> Presc {
            Presc::from_bits(val)
        }
    }
    impl From<Presc> for u8 {
        #[inline(always)]
        fn from(val: Presc) -> u8 {
            Presc::to_bits(val)
        }
    }
}
