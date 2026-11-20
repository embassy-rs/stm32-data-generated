#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Comparator 1."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Comp1 {
    ptr: *mut u8,
}
unsafe impl Send for Comp1 {}
unsafe impl Sync for Comp1 {}
impl Comp1 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "COMP1 control and status register."]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
}
#[doc = "Comparator 2."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Comp2 {
    ptr: *mut u8,
}
unsafe impl Send for Comp2 {}
unsafe impl Sync for Comp2 {}
impl Comp2 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "COMP2 control and status register."]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
}
pub mod regs {
    #[doc = "COMP1 control and status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr1(pub u32);
    impl Csr1 {
        #[doc = "COMP1 enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "COMP1 enable bit."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "COMP1 input minus selection."]
        #[must_use]
        #[inline(always)]
        pub const fn innsel(&self) -> super::vals::Comp1Innsel {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Comp1Innsel::from_bits(val as u8)
        }
        #[doc = "COMP1 input minus selection."]
        #[inline(always)]
        pub const fn set_innsel(&mut self, val: super::vals::Comp1Innsel) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "COMP1 window mode selection bit."]
        #[must_use]
        #[inline(always)]
        pub const fn wm(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "COMP1 window mode selection bit."]
        #[inline(always)]
        pub const fn set_wm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "COMP1 LPTIM1 input 1 propagation bit."]
        #[must_use]
        #[inline(always)]
        pub const fn lptimin1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "COMP1 LPTIM1 input 1 propagation bit."]
        #[inline(always)]
        pub const fn set_lptimin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "COMP1 output polarity selection bit."]
        #[must_use]
        #[inline(always)]
        pub const fn polarity(&self) -> super::vals::Polarity {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Polarity::from_bits(val as u8)
        }
        #[doc = "COMP1 output polarity selection bit."]
        #[inline(always)]
        pub const fn set_polarity(&mut self, val: super::vals::Polarity) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "COMP1 output status bit."]
        #[must_use]
        #[inline(always)]
        pub const fn value(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "COMP1 output status bit."]
        #[inline(always)]
        pub const fn set_value(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "COMP1_CSR register lock bit."]
        #[must_use]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "COMP1_CSR register lock bit."]
        #[inline(always)]
        pub const fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Csr1 {
        #[inline(always)]
        fn default() -> Csr1 {
            Csr1(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Csr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csr1")
                .field("en", &self.en())
                .field("innsel", &self.innsel())
                .field("wm", &self.wm())
                .field("lptimin1", &self.lptimin1())
                .field("polarity", &self.polarity())
                .field("value", &self.value())
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csr1 {{ en: {=bool:?}, innsel: {:?}, wm: {=bool:?}, lptimin1: {=bool:?}, polarity: {:?}, value: {=bool:?}, lock: {=bool:?} }}",
                self.en(),
                self.innsel(),
                self.wm(),
                self.lptimin1(),
                self.polarity(),
                self.value(),
                self.lock()
            )
        }
    }
    #[doc = "COMP2 control and status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr2(pub u32);
    impl Csr2 {
        #[doc = "COMP2 enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "COMP2 enable bit."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "COMP2 power mode selection bit."]
        #[must_use]
        #[inline(always)]
        pub const fn speed(&self) -> super::vals::Speed {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Speed::from_bits(val as u8)
        }
        #[doc = "COMP2 power mode selection bit."]
        #[inline(always)]
        pub const fn set_speed(&mut self, val: super::vals::Speed) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "COMP2 input minus selection."]
        #[must_use]
        #[inline(always)]
        pub const fn innsel(&self) -> super::vals::Comp2Innsel {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Comp2Innsel::from_bits(val as u8)
        }
        #[doc = "COMP2 input minus selection."]
        #[inline(always)]
        pub const fn set_innsel(&mut self, val: super::vals::Comp2Innsel) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "COMP2 input plus selection."]
        #[must_use]
        #[inline(always)]
        pub const fn inpsel(&self) -> super::vals::Comp2Inpsel {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Comp2Inpsel::from_bits(val as u8)
        }
        #[doc = "COMP2 input plus selection."]
        #[inline(always)]
        pub const fn set_inpsel(&mut self, val: super::vals::Comp2Inpsel) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "COMP2 LPTIM1 input 2 propagation bit."]
        #[must_use]
        #[inline(always)]
        pub const fn lptimin2(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "COMP2 LPTIM1 input 2 propagation bit."]
        #[inline(always)]
        pub const fn set_lptimin2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "COMP2 LPTIM1 input 1 propagation bit."]
        #[must_use]
        #[inline(always)]
        pub const fn lptimin1(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "COMP2 LPTIM1 input 1 propagation bit."]
        #[inline(always)]
        pub const fn set_lptimin1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "COMP2 output polarity selection bit."]
        #[must_use]
        #[inline(always)]
        pub const fn polarity(&self) -> super::vals::Polarity {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Polarity::from_bits(val as u8)
        }
        #[doc = "COMP2 output polarity selection bit."]
        #[inline(always)]
        pub const fn set_polarity(&mut self, val: super::vals::Polarity) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "COMP2 output status bit."]
        #[must_use]
        #[inline(always)]
        pub const fn value(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "COMP2 output status bit."]
        #[inline(always)]
        pub const fn set_value(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "COMP2_CSR register lock bit."]
        #[must_use]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "COMP2_CSR register lock bit."]
        #[inline(always)]
        pub const fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Csr2 {
        #[inline(always)]
        fn default() -> Csr2 {
            Csr2(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Csr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csr2")
                .field("en", &self.en())
                .field("speed", &self.speed())
                .field("innsel", &self.innsel())
                .field("inpsel", &self.inpsel())
                .field("lptimin2", &self.lptimin2())
                .field("lptimin1", &self.lptimin1())
                .field("polarity", &self.polarity())
                .field("value", &self.value())
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csr2 {{ en: {=bool:?}, speed: {:?}, innsel: {:?}, inpsel: {:?}, lptimin2: {=bool:?}, lptimin1: {=bool:?}, polarity: {:?}, value: {=bool:?}, lock: {=bool:?} }}",
                self.en(),
                self.speed(),
                self.innsel(),
                self.inpsel(),
                self.lptimin2(),
                self.lptimin1(),
                self.polarity(),
                self.value(),
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
    pub enum Comp1Innsel {
        Vrefint = 0x0,
        Pa0 = 0x01,
        Pa4 = 0x02,
        Pa5 = 0x03,
    }
    impl Comp1Innsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Comp1Innsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Comp1Innsel {
        #[inline(always)]
        fn from(val: u8) -> Comp1Innsel {
            Comp1Innsel::from_bits(val)
        }
    }
    impl From<Comp1Innsel> for u8 {
        #[inline(always)]
        fn from(val: Comp1Innsel) -> u8 {
            Comp1Innsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Comp2Innsel {
        Vrefint = 0x0,
        Pa2 = 0x01,
        Pa4 = 0x02,
        Pa5 = 0x03,
        QuarterVrefint = 0x04,
        HalfVrefint = 0x05,
        ThreeQuarterVrefint = 0x06,
        Pb3 = 0x07,
    }
    impl Comp2Innsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Comp2Innsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Comp2Innsel {
        #[inline(always)]
        fn from(val: u8) -> Comp2Innsel {
            Comp2Innsel::from_bits(val)
        }
    }
    impl From<Comp2Innsel> for u8 {
        #[inline(always)]
        fn from(val: Comp2Innsel) -> u8 {
            Comp2Innsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Comp2Inpsel {
        Pa3 = 0x0,
        Pb4 = 0x01,
        Pb5 = 0x02,
        Pb6 = 0x03,
        Pb7 = 0x04,
        Pa7 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Comp2Inpsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Comp2Inpsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Comp2Inpsel {
        #[inline(always)]
        fn from(val: u8) -> Comp2Inpsel {
            Comp2Inpsel::from_bits(val)
        }
    }
    impl From<Comp2Inpsel> for u8 {
        #[inline(always)]
        fn from(val: Comp2Inpsel) -> u8 {
            Comp2Inpsel::to_bits(val)
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
