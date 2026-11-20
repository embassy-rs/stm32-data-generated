#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Comparators."]
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
    #[doc = "COMP control and status register."]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
}
pub mod regs {
    #[doc = "COMP control and status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "10 kOhm pull-up resistor."]
        #[must_use]
        #[inline(always)]
        pub const fn pu10k(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "10 kOhm pull-up resistor."]
        #[inline(always)]
        pub const fn set_pu10k(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "400 kOhm pull-up resistor."]
        #[must_use]
        #[inline(always)]
        pub const fn pu400k(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "400 kOhm pull-up resistor."]
        #[inline(always)]
        pub const fn set_pu400k(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "10 kOhm pull-down resistor."]
        #[must_use]
        #[inline(always)]
        pub const fn pd10k(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "10 kOhm pull-down resistor."]
        #[inline(always)]
        pub const fn set_pd10k(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "400 kOhm pull-down resistor."]
        #[must_use]
        #[inline(always)]
        pub const fn pd400k(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "400 kOhm pull-down resistor."]
        #[inline(always)]
        pub const fn set_pd400k(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Comparator 1 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp1en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator 1 enable."]
        #[inline(always)]
        pub const fn set_cmp1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "COMP1_SW1 analog switch enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sw1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "COMP1_SW1 analog switch enable."]
        #[inline(always)]
        pub const fn set_sw1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Comparator 1 output."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp1out(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator 1 output."]
        #[inline(always)]
        pub const fn set_cmp1out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Comparator 2 speed mode."]
        #[must_use]
        #[inline(always)]
        pub const fn speed(&self) -> super::vals::Speed {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Speed::from_bits(val as u8)
        }
        #[doc = "Comparator 2 speed mode."]
        #[inline(always)]
        pub const fn set_speed(&mut self, val: super::vals::Speed) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "Comparator 2 output."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp2out(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator 2 output."]
        #[inline(always)]
        pub const fn set_cmp2out(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "VREFINT output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn vrefouten(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "VREFINT output enable."]
        #[inline(always)]
        pub const fn set_vrefouten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Window mode enable."]
        #[must_use]
        #[inline(always)]
        pub const fn wndwe(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Window mode enable."]
        #[inline(always)]
        pub const fn set_wndwe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Comparator 2 inverting input selection."]
        #[must_use]
        #[inline(always)]
        pub const fn insel(&self) -> super::vals::Insel {
            let val = (self.0 >> 18usize) & 0x07;
            super::vals::Insel::from_bits(val as u8)
        }
        #[doc = "Comparator 2 inverting input selection."]
        #[inline(always)]
        pub const fn set_insel(&mut self, val: super::vals::Insel) {
            self.0 = (self.0 & !(0x07 << 18usize)) | (((val.to_bits() as u32) & 0x07) << 18usize);
        }
        #[doc = "Comparator 2 output redirection."]
        #[must_use]
        #[inline(always)]
        pub const fn outsel(&self) -> super::vals::Outsel {
            let val = (self.0 >> 21usize) & 0x07;
            super::vals::Outsel::from_bits(val as u8)
        }
        #[doc = "Comparator 2 output redirection."]
        #[inline(always)]
        pub const fn set_outsel(&mut self, val: super::vals::Outsel) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
        }
        #[doc = "Select GPIO port PA3 as fast ADC input channel CH3."]
        #[must_use]
        #[inline(always)]
        pub const fn fch3(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Select GPIO port PA3 as fast ADC input channel CH3."]
        #[inline(always)]
        pub const fn set_fch3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Select GPIO port PB0 as fast ADC input channel CH8."]
        #[must_use]
        #[inline(always)]
        pub const fn fch8(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Select GPIO port PB0 as fast ADC input channel CH8."]
        #[inline(always)]
        pub const fn set_fch8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Select GPIO port PC3 as re-routed ADC input channel CH13."]
        #[must_use]
        #[inline(always)]
        pub const fn rch13(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Select GPIO port PC3 as re-routed ADC input channel CH13."]
        #[inline(always)]
        pub const fn set_rch13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Channel acquisition interrupt enable / clear."]
        #[must_use]
        #[inline(always)]
        pub const fn caie(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Channel acquisition interrupt enable / clear."]
        #[inline(always)]
        pub const fn set_caie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Channel acquisition interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn caif(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Channel acquisition interrupt flag."]
        #[inline(always)]
        pub const fn set_caif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Suspend Timer Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tsusp(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Suspend Timer Mode."]
        #[inline(always)]
        pub const fn set_tsusp(&mut self, val: bool) {
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
                .field("pu10k", &self.pu10k())
                .field("pu400k", &self.pu400k())
                .field("pd10k", &self.pd10k())
                .field("pd400k", &self.pd400k())
                .field("cmp1en", &self.cmp1en())
                .field("sw1", &self.sw1())
                .field("cmp1out", &self.cmp1out())
                .field("speed", &self.speed())
                .field("cmp2out", &self.cmp2out())
                .field("vrefouten", &self.vrefouten())
                .field("wndwe", &self.wndwe())
                .field("insel", &self.insel())
                .field("outsel", &self.outsel())
                .field("fch3", &self.fch3())
                .field("fch8", &self.fch8())
                .field("rch13", &self.rch13())
                .field("caie", &self.caie())
                .field("caif", &self.caif())
                .field("tsusp", &self.tsusp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csr {{ pu10k: {=bool:?}, pu400k: {=bool:?}, pd10k: {=bool:?}, pd400k: {=bool:?}, cmp1en: {=bool:?}, sw1: {=bool:?}, cmp1out: {=bool:?}, speed: {:?}, cmp2out: {=bool:?}, vrefouten: {=bool:?}, wndwe: {=bool:?}, insel: {:?}, outsel: {:?}, fch3: {=bool:?}, fch8: {=bool:?}, rch13: {=bool:?}, caie: {=bool:?}, caif: {=bool:?}, tsusp: {=bool:?} }}",
                self.pu10k(),
                self.pu400k(),
                self.pd10k(),
                self.pd400k(),
                self.cmp1en(),
                self.sw1(),
                self.cmp1out(),
                self.speed(),
                self.cmp2out(),
                self.vrefouten(),
                self.wndwe(),
                self.insel(),
                self.outsel(),
                self.fch3(),
                self.fch8(),
                self.rch13(),
                self.caie(),
                self.caif(),
                self.tsusp()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Insel {
        None = 0x0,
        Pb3 = 0x01,
        Vrefint = 0x02,
        ThreeQuarterVrefint = 0x03,
        HalfVrefint = 0x04,
        QuarterVrefint = 0x05,
        DacOut1 = 0x06,
        DacOut2 = 0x07,
    }
    impl Insel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Insel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Insel {
        #[inline(always)]
        fn from(val: u8) -> Insel {
            Insel::from_bits(val)
        }
    }
    impl From<Insel> for u8 {
        #[inline(always)]
        fn from(val: Insel) -> u8 {
            Insel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Outsel {
        Tim2ic4 = 0x0,
        Tim2ocrefClr = 0x01,
        Tim3ic4 = 0x02,
        Tim3ocrefClr = 0x03,
        Tim4ic4 = 0x04,
        Tim4ocrefClr = 0x05,
        Tim10ic1 = 0x06,
        NoRedirection = 0x07,
    }
    impl Outsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Outsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Outsel {
        #[inline(always)]
        fn from(val: u8) -> Outsel {
            Outsel::from_bits(val)
        }
    }
    impl From<Outsel> for u8 {
        #[inline(always)]
        fn from(val: Outsel) -> u8 {
            Outsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Speed {
        #[doc = "Slow speed."]
        Slow = 0x0,
        #[doc = "Fast speed."]
        Fast = 0x01,
    }
    impl Speed {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Speed {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Speed {
        #[inline(always)]
        fn from(val: u8) -> Speed {
            Speed::from_bits(val)
        }
    }
    impl From<Speed> for u8 {
        #[inline(always)]
        fn from(val: Speed) -> u8 {
            Speed::to_bits(val)
        }
    }
}
