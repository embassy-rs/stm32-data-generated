#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Comparator."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Comp {
    ptr: *mut u8,
}
unsafe impl Send for Comp {}
unsafe impl Sync for Comp {}
impl Comp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Comparator control and status register."]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Comparator filter configuration register."]
    #[inline(always)]
    pub const fn fltr(self) -> crate::common::Reg<regs::Fltr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
}
pub mod regs {
    #[doc = "Comparator control and status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "COMP enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "COMP enable bit."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Comparator signal selector for inverting input INM."]
        #[must_use]
        #[inline(always)]
        pub const fn inmsel(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Comparator signal selector for inverting input INM."]
        #[inline(always)]
        pub const fn set_inmsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Comparator signal selector for non-inverting input INP."]
        #[must_use]
        #[inline(always)]
        pub const fn inpsel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Comparator signal selector for non-inverting input INP."]
        #[inline(always)]
        pub const fn set_inpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "Comparator output selector."]
        #[must_use]
        #[inline(always)]
        pub const fn winout(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator output selector."]
        #[inline(always)]
        pub const fn set_winout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Comparator polarity selector."]
        #[must_use]
        #[inline(always)]
        pub const fn polarity(&self) -> super::vals::Polarity {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Polarity::from_bits(val as u8)
        }
        #[doc = "Comparator polarity selector."]
        #[inline(always)]
        pub const fn set_polarity(&mut self, val: super::vals::Polarity) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "Comparator hysteresis selector."]
        #[must_use]
        #[inline(always)]
        pub const fn hyst(&self) -> super::vals::Hysteresis {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::Hysteresis::from_bits(val as u8)
        }
        #[doc = "Comparator hysteresis selector."]
        #[inline(always)]
        pub const fn set_hyst(&mut self, val: super::vals::Hysteresis) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
        #[doc = "Comparator blanking source selector."]
        #[must_use]
        #[inline(always)]
        pub const fn blanksel(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x0f;
            val as u8
        }
        #[doc = "Comparator blanking source selector."]
        #[inline(always)]
        pub const fn set_blanksel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 19usize)) | (((val as u32) & 0x0f) << 19usize);
        }
        #[doc = "Vrefint resistor bridge enable."]
        #[must_use]
        #[inline(always)]
        pub const fn brgen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Vrefint resistor bridge enable."]
        #[inline(always)]
        pub const fn set_brgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Vrefint scaled input enable."]
        #[must_use]
        #[inline(always)]
        pub const fn scalen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Vrefint scaled input enable."]
        #[inline(always)]
        pub const fn set_scalen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Deglitcher enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dglt(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Deglitcher enable."]
        #[inline(always)]
        pub const fn set_dglt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Comparator output status."]
        #[must_use]
        #[inline(always)]
        pub const fn value(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator output status."]
        #[inline(always)]
        pub const fn set_value(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "CSR and FLTR register lock."]
        #[must_use]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "CSR and FLTR register lock."]
        #[inline(always)]
        pub const fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Csr {
        #[inline(always)]
        fn default() -> Csr {
            Csr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Csr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csr")
                .field("en", &self.en())
                .field("inmsel", &self.inmsel())
                .field("inpsel", &self.inpsel())
                .field("winout", &self.winout())
                .field("polarity", &self.polarity())
                .field("hyst", &self.hyst())
                .field("blanksel", &self.blanksel())
                .field("brgen", &self.brgen())
                .field("scalen", &self.scalen())
                .field("dglt", &self.dglt())
                .field("value", &self.value())
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csr {{ en: {=bool:?}, inmsel: {=u8:?}, inpsel: {=u8:?}, winout: {=bool:?}, polarity: {:?}, hyst: {:?}, blanksel: {=u8:?}, brgen: {=bool:?}, scalen: {=bool:?}, dglt: {=bool:?}, value: {=bool:?}, lock: {=bool:?} }}",
                self.en(),
                self.inmsel(),
                self.inpsel(),
                self.winout(),
                self.polarity(),
                self.hyst(),
                self.blanksel(),
                self.brgen(),
                self.scalen(),
                self.dglt(),
                self.value(),
                self.lock()
            )
        }
    }
    #[doc = "Comparator filter configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fltr(pub u32);
    impl Fltr {
        #[doc = "Digital filter clock prescaler selection."]
        #[must_use]
        #[inline(always)]
        pub const fn fpsc(&self) -> super::vals::Fpsc {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Fpsc::from_bits(val as u8)
        }
        #[doc = "Digital filter clock prescaler selection."]
        #[inline(always)]
        pub const fn set_fpsc(&mut self, val: super::vals::Fpsc) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Digital filter sampling and sample count selection."]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> super::vals::Fsel {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::Fsel::from_bits(val as u8)
        }
        #[doc = "Digital filter sampling and sample count selection."]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: super::vals::Fsel) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
        }
    }
    impl Default for Fltr {
        #[inline(always)]
        fn default() -> Fltr {
            Fltr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Fltr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fltr")
                .field("fpsc", &self.fpsc())
                .field("fsel", &self.fsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fltr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Fltr {{ fpsc: {:?}, fsel: {:?} }}", self.fpsc(), self.fsel())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fpsc {
        #[doc = "filter_clk = comp_pclk."]
        Div1 = 0x0,
        #[doc = "filter_clk = comp_pclk / 2."]
        Div2 = 0x01,
        #[doc = "filter_clk = comp_pclk / 4."]
        Div4 = 0x02,
        #[doc = "filter_clk = comp_pclk / 8."]
        Div8 = 0x03,
    }
    impl Fpsc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fpsc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fpsc {
        #[inline(always)]
        fn from(val: u8) -> Fpsc {
            Fpsc::from_bits(val)
        }
    }
    impl From<Fpsc> for u8 {
        #[inline(always)]
        fn from(val: Fpsc) -> u8 {
            Fpsc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fsel {
        #[doc = "Filter disabled."]
        Disabled = 0x0,
        #[doc = "filter_sampling = comp_pclk, N = 2."]
        PclkN2 = 0x01,
        #[doc = "filter_sampling = comp_pclk, N = 4."]
        PclkN4 = 0x02,
        #[doc = "filter_sampling = comp_pclk, N = 8."]
        PclkN8 = 0x03,
        #[doc = "filter_sampling = filter_clk / 2, N = 6."]
        Div2n6 = 0x04,
        #[doc = "filter_sampling = filter_clk / 2, N = 8."]
        Div2n8 = 0x05,
        #[doc = "filter_sampling = filter_clk / 4, N = 6."]
        Div4n6 = 0x06,
        #[doc = "filter_sampling = filter_clk / 4, N = 8."]
        Div4n8 = 0x07,
        #[doc = "filter_sampling = filter_clk / 8, N = 6."]
        Div8n6 = 0x08,
        #[doc = "filter_sampling = filter_clk / 8, N = 8."]
        Div8n8 = 0x09,
        #[doc = "filter_sampling = filter_clk / 16, N = 5."]
        Div16n5 = 0x0a,
        #[doc = "filter_sampling = filter_clk / 16, N = 6."]
        Div16n6 = 0x0b,
        #[doc = "filter_sampling = filter_clk / 16, N = 8."]
        Div16n8 = 0x0c,
        #[doc = "filter_sampling = filter_clk / 32, N = 5."]
        Div32n5 = 0x0d,
        #[doc = "filter_sampling = filter_clk / 32, N = 6."]
        Div32n6 = 0x0e,
        #[doc = "filter_sampling = filter_clk / 32, N = 8."]
        Div32n8 = 0x0f,
    }
    impl Fsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fsel {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fsel {
        #[inline(always)]
        fn from(val: u8) -> Fsel {
            Fsel::from_bits(val)
        }
    }
    impl From<Fsel> for u8 {
        #[inline(always)]
        fn from(val: Fsel) -> u8 {
            Fsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hysteresis {
        #[doc = "No hysteresis."]
        None = 0x0,
        #[doc = "10 mV hysteresis."]
        Hyst10m = 0x01,
        #[doc = "20 mV hysteresis."]
        Hyst20m = 0x02,
        #[doc = "30 mV hysteresis."]
        Hyst30m = 0x03,
        #[doc = "40 mV hysteresis."]
        Hyst40m = 0x04,
        #[doc = "50 mV hysteresis."]
        Hyst50m = 0x05,
        #[doc = "60 mV hysteresis."]
        Hyst60m = 0x06,
        #[doc = "70 mV hysteresis."]
        Hyst70m = 0x07,
    }
    impl Hysteresis {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hysteresis {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hysteresis {
        #[inline(always)]
        fn from(val: u8) -> Hysteresis {
            Hysteresis::from_bits(val)
        }
    }
    impl From<Hysteresis> for u8 {
        #[inline(always)]
        fn from(val: Hysteresis) -> u8 {
            Hysteresis::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Polarity {
        #[doc = "Output is not inverted."]
        NotInverted = 0x0,
        #[doc = "Output is inverted."]
        Inverted = 0x01,
    }
    impl Polarity {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Polarity {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Polarity {
        #[inline(always)]
        fn from(val: u8) -> Polarity {
            Polarity::from_bits(val)
        }
    }
    impl From<Polarity> for u8 {
        #[inline(always)]
        fn from(val: Polarity) -> u8 {
            Polarity::to_bits(val)
        }
    }
}
