#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "General purpose I/O"]
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
    #[doc = "Port configuration register low (GPIOn_CRL)"]
    #[inline(always)]
    pub const fn cr(self, n: usize) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "Port input data register (GPIOn_IDR)"]
    #[inline(always)]
    pub const fn idr(self) -> crate::common::Reg<regs::Idr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Port output data register (GPIOn_ODR)"]
    #[inline(always)]
    pub const fn odr(self) -> crate::common::Reg<regs::Odr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Port bit set/reset register (GPIOn_BSRR)"]
    #[inline(always)]
    pub const fn bsrr(self) -> crate::common::Reg<regs::Bsrr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Port bit reset register (GPIOn_BRR)"]
    #[inline(always)]
    pub const fn brr(self) -> crate::common::Reg<regs::Brr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Port configuration lock register"]
    #[inline(always)]
    pub const fn lckr(self) -> crate::common::Reg<regs::Lckr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
}
pub mod regs {
    #[doc = "Port bit reset register (GPIOn_BRR)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Brr(pub u32);
    impl Brr {
        #[doc = "Reset bit"]
        #[inline(always)]
        pub const fn br(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Reset bit"]
        #[inline(always)]
        pub fn set_br(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Brr {
        #[inline(always)]
        fn default() -> Brr {
            Brr(0)
        }
    }
    #[doc = "Port bit set/reset register (GPIOn_BSRR)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bsrr(pub u32);
    impl Bsrr {
        #[doc = "Set bit"]
        #[inline(always)]
        pub const fn bs(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Set bit"]
        #[inline(always)]
        pub fn set_bs(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Reset bit"]
        #[inline(always)]
        pub const fn br(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Reset bit"]
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
    #[doc = "Port configuration register (GPIOn_CRx)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Port n mode bits"]
        #[inline(always)]
        pub const fn mode(&self, n: usize) -> super::vals::Mode {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "Port n mode bits"]
        #[inline(always)]
        pub fn set_mode(&mut self, n: usize, val: super::vals::Mode) {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Port n configuration bits, for input mode"]
        #[inline(always)]
        pub const fn cnf_in(&self, n: usize) -> super::vals::CnfIn {
            assert!(n < 8usize);
            let offs = 2usize + n * 4usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::CnfIn::from_bits(val as u8)
        }
        #[doc = "Port n configuration bits, for input mode"]
        #[inline(always)]
        pub fn set_cnf_in(&mut self, n: usize, val: super::vals::CnfIn) {
            assert!(n < 8usize);
            let offs = 2usize + n * 4usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Port n configuration bits, for output mode"]
        #[inline(always)]
        pub const fn cnf_out(&self, n: usize) -> super::vals::CnfOut {
            assert!(n < 8usize);
            let offs = 2usize + n * 4usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::CnfOut::from_bits(val as u8)
        }
        #[doc = "Port n configuration bits, for output mode"]
        #[inline(always)]
        pub fn set_cnf_out(&mut self, n: usize, val: super::vals::CnfOut) {
            assert!(n < 8usize);
            let offs = 2usize + n * 4usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    #[doc = "Port input data register (GPIOn_IDR)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idr(pub u32);
    impl Idr {
        #[doc = "Port input data"]
        #[inline(always)]
        pub const fn idr(&self, n: usize) -> super::vals::Idr {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Idr::from_bits(val as u8)
        }
        #[doc = "Port input data"]
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
    #[doc = "Port configuration lock register"]
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
    #[doc = "Port output data register (GPIOn_ODR)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Odr(pub u32);
    impl Odr {
        #[doc = "Port output data"]
        #[inline(always)]
        pub const fn odr(&self, n: usize) -> super::vals::Odr {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Odr::from_bits(val as u8)
        }
        #[doc = "Port output data"]
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
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum CnfIn {
        #[doc = "Analog mode"]
        ANALOG = 0x0,
        #[doc = "Floating input (reset state)"]
        FLOATING = 0x01,
        #[doc = "Input with pull-up/pull-down"]
        PULL = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl CnfIn {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CnfIn {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CnfIn {
        #[inline(always)]
        fn from(val: u8) -> CnfIn {
            CnfIn::from_bits(val)
        }
    }
    impl From<CnfIn> for u8 {
        #[inline(always)]
        fn from(val: CnfIn) -> u8 {
            CnfIn::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum CnfOut {
        #[doc = "Push-Pull mode"]
        PUSHPULL = 0x0,
        #[doc = "Open Drain-Mode"]
        OPENDRAIN = 0x01,
        #[doc = "Alternate Function Push-Pull Mode"]
        ALTPUSHPULL = 0x02,
        #[doc = "Alternate Function Open-Drain Mode"]
        ALTOPENDRAIN = 0x03,
    }
    impl CnfOut {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CnfOut {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CnfOut {
        #[inline(always)]
        fn from(val: u8) -> CnfOut {
            CnfOut::from_bits(val)
        }
    }
    impl From<CnfOut> for u8 {
        #[inline(always)]
        fn from(val: CnfOut) -> u8 {
            CnfOut::to_bits(val)
        }
    }
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
    pub enum Mode {
        #[doc = "Input mode (reset state)"]
        INPUT = 0x0,
        #[doc = "Output mode 10 MHz"]
        OUTPUT10MHZ = 0x01,
        #[doc = "Output mode 2 MHz"]
        OUTPUT2MHZ = 0x02,
        #[doc = "Output mode 50 MHz"]
        OUTPUT50MHZ = 0x03,
    }
    impl Mode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mode {
        #[inline(always)]
        fn from(val: u8) -> Mode {
            Mode::from_bits(val)
        }
    }
    impl From<Mode> for u8 {
        #[inline(always)]
        fn from(val: Mode) -> u8 {
            Mode::to_bits(val)
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
}
