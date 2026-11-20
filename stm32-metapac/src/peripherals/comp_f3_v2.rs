#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "General purpose comparators."]
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
    #[doc = "control and status register."]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
}
pub mod regs {
    #[doc = "control and status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "Comparator enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator enable."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Comparator non-inverting input connection to DAC output."]
        #[must_use]
        #[inline(always)]
        pub const fn inp_dac(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator non-inverting input connection to DAC output."]
        #[inline(always)]
        pub const fn set_inp_dac(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Comparator mode (encoding differs between COMP1/2 and COMP3..7; see reference manual)."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Comparator mode (encoding differs between COMP1/2 and COMP3..7; see reference manual)."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Comparator inverting input selection."]
        #[must_use]
        #[inline(always)]
        pub const fn inmsel(&self) -> u8 {
            let mut val = 0;
            val += (((self.0 >> 4usize) & 0x07) << 0usize);
            val += (((self.0 >> 22usize) & 0x01) << 3usize);
            val as u8
        }
        #[doc = "Comparator inverting input selection."]
        #[inline(always)]
        pub const fn set_inmsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32 >> 0usize) & 0x07) << 4usize);
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32 >> 3usize) & 0x01) << 22usize);
        }
        #[doc = "Comparator non-inverting input selection."]
        #[must_use]
        #[inline(always)]
        pub const fn inpsel(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator non-inverting input selection."]
        #[inline(always)]
        pub const fn set_inpsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Window mode enable (available on even comparators)."]
        #[must_use]
        #[inline(always)]
        pub const fn winmode(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Window mode enable (available on even comparators)."]
        #[inline(always)]
        pub const fn set_winmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Comparator output selection."]
        #[must_use]
        #[inline(always)]
        pub const fn outsel(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x0f;
            val as u8
        }
        #[doc = "Comparator output selection."]
        #[inline(always)]
        pub const fn set_outsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 10usize)) | (((val as u32) & 0x0f) << 10usize);
        }
        #[doc = "Comparator output polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn pol(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator output polarity."]
        #[inline(always)]
        pub const fn set_pol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Comparator hysteresis."]
        #[must_use]
        #[inline(always)]
        pub const fn hyst(&self) -> super::vals::Hyst {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Hyst::from_bits(val as u8)
        }
        #[doc = "Comparator hysteresis."]
        #[inline(always)]
        pub const fn set_hyst(&mut self, val: super::vals::Hyst) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Comparator blanking source."]
        #[must_use]
        #[inline(always)]
        pub const fn blanking(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x07;
            val as u8
        }
        #[doc = "Comparator blanking source."]
        #[inline(always)]
        pub const fn set_blanking(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
        }
        #[doc = "Comparator output."]
        #[must_use]
        #[inline(always)]
        pub const fn out(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator output."]
        #[inline(always)]
        pub const fn set_out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Comparator lock."]
        #[must_use]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator lock."]
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
                .field("inp_dac", &self.inp_dac())
                .field("mode", &self.mode())
                .field("inmsel", &self.inmsel())
                .field("inpsel", &self.inpsel())
                .field("winmode", &self.winmode())
                .field("outsel", &self.outsel())
                .field("pol", &self.pol())
                .field("hyst", &self.hyst())
                .field("blanking", &self.blanking())
                .field("out", &self.out())
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csr {{ en: {=bool:?}, inp_dac: {=bool:?}, mode: {=u8:?}, inmsel: {=u8:?}, inpsel: {=bool:?}, winmode: {=bool:?}, outsel: {=u8:?}, pol: {=bool:?}, hyst: {:?}, blanking: {=u8:?}, out: {=bool:?}, lock: {=bool:?} }}",
                self.en(),
                self.inp_dac(),
                self.mode(),
                self.inmsel(),
                self.inpsel(),
                self.winmode(),
                self.outsel(),
                self.pol(),
                self.hyst(),
                self.blanking(),
                self.out(),
                self.lock()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hyst {
        None = 0x0,
        #[doc = "Low hysteresis"]
        Low = 0x01,
        #[doc = "Medium hysteresis"]
        Medium = 0x02,
        #[doc = "High hysteresis"]
        High = 0x03,
    }
    impl Hyst {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hyst {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hyst {
        #[inline(always)]
        fn from(val: u8) -> Hyst {
            Hyst::from_bits(val)
        }
    }
    impl From<Hyst> for u8 {
        #[inline(always)]
        fn from(val: Hyst) -> u8 {
            Hyst::to_bits(val)
        }
    }
}
