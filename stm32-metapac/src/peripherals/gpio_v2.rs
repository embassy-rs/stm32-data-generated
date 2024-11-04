#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "General-purpose I/Os"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio {
    ptr: *mut u8,
}
unsafe impl Send for Gpio {}
unsafe impl Sync for Gpio {}
impl Gpio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GPIO port mode register"]
    #[inline(always)]
    pub const fn moder(self) -> crate::common::Reg<regs::Moder, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "GPIO port output type register"]
    #[inline(always)]
    pub const fn otyper(self) -> crate::common::Reg<regs::Otyper, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "GPIO port output speed register"]
    #[inline(always)]
    pub const fn ospeedr(self) -> crate::common::Reg<regs::Ospeedr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "GPIO port pull-up/pull-down register"]
    #[inline(always)]
    pub const fn pupdr(self) -> crate::common::Reg<regs::Pupdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "GPIO port input data register"]
    #[inline(always)]
    pub const fn idr(self) -> crate::common::Reg<regs::Idr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "GPIO port output data register"]
    #[inline(always)]
    pub const fn odr(self) -> crate::common::Reg<regs::Odr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "GPIO port bit set/reset register"]
    #[inline(always)]
    pub const fn bsrr(self) -> crate::common::Reg<regs::Bsrr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "GPIO port configuration lock register"]
    #[inline(always)]
    pub const fn lckr(self) -> crate::common::Reg<regs::Lckr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "GPIO alternate function registers. The register described in the datasheet as AFRL is index 0 in this array, and AFRH is index 1. Note that when operating on AFRH, you need to subtract 8 from any operations on the field array it contains -- the alternate function for pin 9 is at index 1, for instance."]
    #[inline(always)]
    pub const fn afr(self, n: usize) -> crate::common::Reg<regs::Afr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "GPIO alternate function register. This contains an array of 8 fields, which correspond to pins 0-7 of the port (for AFRL) or pins 8-15 of the port (for AFRH)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Afr(pub u32);
    impl Afr {
        #[doc = "Alternate function selection for one of the pins controlled by this register (0-7)."]
        #[inline(always)]
        pub const fn afr(&self, n: usize) -> u8 {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "Alternate function selection for one of the pins controlled by this register (0-7)."]
        #[inline(always)]
        pub fn set_afr(&mut self, n: usize, val: u8) {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
    }
    impl Default for Afr {
        #[inline(always)]
        fn default() -> Afr {
            Afr(0)
        }
    }
    #[doc = "GPIO port bit set/reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bsrr(pub u32);
    impl Bsrr {
        #[doc = "Port x set bit y (y= 0..15)"]
        #[inline(always)]
        pub const fn bs(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Port x set bit y (y= 0..15)"]
        #[inline(always)]
        pub fn set_bs(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Port x set bit y (y= 0..15)"]
        #[inline(always)]
        pub const fn br(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Port x set bit y (y= 0..15)"]
        #[inline(always)]
        pub fn set_br(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Bsrr {
        #[inline(always)]
        fn default() -> Bsrr {
            Bsrr(0)
        }
    }
    #[doc = "GPIO port input data register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idr(pub u32);
    impl Idr {
        #[doc = "Port input data (y = 0..15)"]
        #[inline(always)]
        pub const fn idr(&self, n: usize) -> super::vals::Idr {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Idr::from_bits(val as u8)
        }
        #[doc = "Port input data (y = 0..15)"]
        #[inline(always)]
        pub fn set_idr(&mut self, n: usize, val: super::vals::Idr) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
    }
    impl Default for Idr {
        #[inline(always)]
        fn default() -> Idr {
            Idr(0)
        }
    }
    #[doc = "GPIO port configuration lock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lckr(pub u32);
    impl Lckr {
        #[doc = "Port configuration locked"]
        #[inline(always)]
        pub const fn lck(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Port configuration locked"]
        #[inline(always)]
        pub fn set_lck(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Port configuration lock key active"]
        #[inline(always)]
        pub const fn lckk(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Port configuration lock key active"]
        #[inline(always)]
        pub fn set_lckk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Lckr {
        #[inline(always)]
        fn default() -> Lckr {
            Lckr(0)
        }
    }
    #[doc = "GPIO port mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Moder(pub u32);
    impl Moder {
        #[doc = "Port x configuration bits (y = 0..15)"]
        #[inline(always)]
        pub const fn moder(&self, n: usize) -> super::vals::Moder {
            assert!(n < 16usize);
            let offs = 0usize + n * 2usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::Moder::from_bits(val as u8)
        }
        #[doc = "Port x configuration bits (y = 0..15)"]
        #[inline(always)]
        pub fn set_moder(&mut self, n: usize, val: super::vals::Moder) {
            assert!(n < 16usize);
            let offs = 0usize + n * 2usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
    }
    impl Default for Moder {
        #[inline(always)]
        fn default() -> Moder {
            Moder(0)
        }
    }
    #[doc = "GPIO port output data register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Odr(pub u32);
    impl Odr {
        #[doc = "Port output data (y = 0..15)"]
        #[inline(always)]
        pub const fn odr(&self, n: usize) -> super::vals::Odr {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Odr::from_bits(val as u8)
        }
        #[doc = "Port output data (y = 0..15)"]
        #[inline(always)]
        pub fn set_odr(&mut self, n: usize, val: super::vals::Odr) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
    }
    impl Default for Odr {
        #[inline(always)]
        fn default() -> Odr {
            Odr(0)
        }
    }
    #[doc = "GPIO port output speed register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ospeedr(pub u32);
    impl Ospeedr {
        #[doc = "Port x configuration bits (y = 0..15)"]
        #[inline(always)]
        pub const fn ospeedr(&self, n: usize) -> super::vals::Ospeedr {
            assert!(n < 16usize);
            let offs = 0usize + n * 2usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::Ospeedr::from_bits(val as u8)
        }
        #[doc = "Port x configuration bits (y = 0..15)"]
        #[inline(always)]
        pub fn set_ospeedr(&mut self, n: usize, val: super::vals::Ospeedr) {
            assert!(n < 16usize);
            let offs = 0usize + n * 2usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
    }
    impl Default for Ospeedr {
        #[inline(always)]
        fn default() -> Ospeedr {
            Ospeedr(0)
        }
    }
    #[doc = "GPIO port output type register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Otyper(pub u32);
    impl Otyper {
        #[doc = "Port x configuration bits (y = 0..15)"]
        #[inline(always)]
        pub const fn ot(&self, n: usize) -> super::vals::Ot {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Ot::from_bits(val as u8)
        }
        #[doc = "Port x configuration bits (y = 0..15)"]
        #[inline(always)]
        pub fn set_ot(&mut self, n: usize, val: super::vals::Ot) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
    }
    impl Default for Otyper {
        #[inline(always)]
        fn default() -> Otyper {
            Otyper(0)
        }
    }
    #[doc = "GPIO port pull-up/pull-down register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pupdr(pub u32);
    impl Pupdr {
        #[doc = "Port x configuration bits (y = 0..15)"]
        #[inline(always)]
        pub const fn pupdr(&self, n: usize) -> super::vals::Pupdr {
            assert!(n < 16usize);
            let offs = 0usize + n * 2usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::Pupdr::from_bits(val as u8)
        }
        #[doc = "Port x configuration bits (y = 0..15)"]
        #[inline(always)]
        pub fn set_pupdr(&mut self, n: usize, val: super::vals::Pupdr) {
            assert!(n < 16usize);
            let offs = 0usize + n * 2usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
    }
    impl Default for Pupdr {
        #[inline(always)]
        fn default() -> Pupdr {
            Pupdr(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Idr {
        #[doc = "Input is logic low"]
        LOW = 0x0,
        #[doc = "Input is logic high"]
        HIGH = 0x01,
    }
    impl Idr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Idr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Idr {
        #[inline(always)]
        fn from(val: u8) -> Idr {
            Idr::from_bits(val)
        }
    }
    impl From<Idr> for u8 {
        #[inline(always)]
        fn from(val: Idr) -> u8 {
            Idr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Moder {
        #[doc = "Input mode (reset state)"]
        INPUT = 0x0,
        #[doc = "General purpose output mode"]
        OUTPUT = 0x01,
        #[doc = "Alternate function mode"]
        ALTERNATE = 0x02,
        #[doc = "Analog mode"]
        ANALOG = 0x03,
    }
    impl Moder {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Moder {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Moder {
        #[inline(always)]
        fn from(val: u8) -> Moder {
            Moder::from_bits(val)
        }
    }
    impl From<Moder> for u8 {
        #[inline(always)]
        fn from(val: Moder) -> u8 {
            Moder::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Odr {
        #[doc = "Set output to logic low"]
        LOW = 0x0,
        #[doc = "Set output to logic high"]
        HIGH = 0x01,
    }
    impl Odr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Odr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Odr {
        #[inline(always)]
        fn from(val: u8) -> Odr {
            Odr::from_bits(val)
        }
    }
    impl From<Odr> for u8 {
        #[inline(always)]
        fn from(val: Odr) -> u8 {
            Odr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ospeedr {
        #[doc = "Low speed"]
        LOWSPEED = 0x0,
        #[doc = "Medium speed"]
        MEDIUMSPEED = 0x01,
        #[doc = "High speed"]
        HIGHSPEED = 0x02,
        #[doc = "Very high speed"]
        VERYHIGHSPEED = 0x03,
    }
    impl Ospeedr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ospeedr {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ospeedr {
        #[inline(always)]
        fn from(val: u8) -> Ospeedr {
            Ospeedr::from_bits(val)
        }
    }
    impl From<Ospeedr> for u8 {
        #[inline(always)]
        fn from(val: Ospeedr) -> u8 {
            Ospeedr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ot {
        #[doc = "Output push-pull (reset state)"]
        PUSHPULL = 0x0,
        #[doc = "Output open-drain"]
        OPENDRAIN = 0x01,
    }
    impl Ot {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ot {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ot {
        #[inline(always)]
        fn from(val: u8) -> Ot {
            Ot::from_bits(val)
        }
    }
    impl From<Ot> for u8 {
        #[inline(always)]
        fn from(val: Ot) -> u8 {
            Ot::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Pupdr {
        #[doc = "No pull-up, pull-down"]
        FLOATING = 0x0,
        #[doc = "Pull-up"]
        PULLUP = 0x01,
        #[doc = "Pull-down"]
        PULLDOWN = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Pupdr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pupdr {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pupdr {
        #[inline(always)]
        fn from(val: u8) -> Pupdr {
            Pupdr::from_bits(val)
        }
    }
    impl From<Pupdr> for u8 {
        #[inline(always)]
        fn from(val: Pupdr) -> u8 {
            Pupdr::to_bits(val)
        }
    }
}
