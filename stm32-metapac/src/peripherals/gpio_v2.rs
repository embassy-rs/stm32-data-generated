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
    impl core::fmt::Debug for Afr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Afr")
                .field(
                    "afr",
                    &[
                        self.afr(0usize),
                        self.afr(1usize),
                        self.afr(2usize),
                        self.afr(3usize),
                        self.afr(4usize),
                        self.afr(5usize),
                        self.afr(6usize),
                        self.afr(7usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Afr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Afr {
                afr: [u8; 8usize],
            }
            let proxy = Afr {
                afr: [
                    self.afr(0usize),
                    self.afr(1usize),
                    self.afr(2usize),
                    self.afr(3usize),
                    self.afr(4usize),
                    self.afr(5usize),
                    self.afr(6usize),
                    self.afr(7usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
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
    impl core::fmt::Debug for Bsrr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bsrr")
                .field(
                    "bs",
                    &[
                        self.bs(0usize),
                        self.bs(1usize),
                        self.bs(2usize),
                        self.bs(3usize),
                        self.bs(4usize),
                        self.bs(5usize),
                        self.bs(6usize),
                        self.bs(7usize),
                        self.bs(8usize),
                        self.bs(9usize),
                        self.bs(10usize),
                        self.bs(11usize),
                        self.bs(12usize),
                        self.bs(13usize),
                        self.bs(14usize),
                        self.bs(15usize),
                    ],
                )
                .field(
                    "br",
                    &[
                        self.br(0usize),
                        self.br(1usize),
                        self.br(2usize),
                        self.br(3usize),
                        self.br(4usize),
                        self.br(5usize),
                        self.br(6usize),
                        self.br(7usize),
                        self.br(8usize),
                        self.br(9usize),
                        self.br(10usize),
                        self.br(11usize),
                        self.br(12usize),
                        self.br(13usize),
                        self.br(14usize),
                        self.br(15usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bsrr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Bsrr {
                bs: [bool; 16usize],
                br: [bool; 16usize],
            }
            let proxy = Bsrr {
                bs: [
                    self.bs(0usize),
                    self.bs(1usize),
                    self.bs(2usize),
                    self.bs(3usize),
                    self.bs(4usize),
                    self.bs(5usize),
                    self.bs(6usize),
                    self.bs(7usize),
                    self.bs(8usize),
                    self.bs(9usize),
                    self.bs(10usize),
                    self.bs(11usize),
                    self.bs(12usize),
                    self.bs(13usize),
                    self.bs(14usize),
                    self.bs(15usize),
                ],
                br: [
                    self.br(0usize),
                    self.br(1usize),
                    self.br(2usize),
                    self.br(3usize),
                    self.br(4usize),
                    self.br(5usize),
                    self.br(6usize),
                    self.br(7usize),
                    self.br(8usize),
                    self.br(9usize),
                    self.br(10usize),
                    self.br(11usize),
                    self.br(12usize),
                    self.br(13usize),
                    self.br(14usize),
                    self.br(15usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
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
    impl core::fmt::Debug for Idr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Idr")
                .field(
                    "idr",
                    &[
                        self.idr(0usize),
                        self.idr(1usize),
                        self.idr(2usize),
                        self.idr(3usize),
                        self.idr(4usize),
                        self.idr(5usize),
                        self.idr(6usize),
                        self.idr(7usize),
                        self.idr(8usize),
                        self.idr(9usize),
                        self.idr(10usize),
                        self.idr(11usize),
                        self.idr(12usize),
                        self.idr(13usize),
                        self.idr(14usize),
                        self.idr(15usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Idr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Idr {
                idr: [super::vals::Idr; 16usize],
            }
            let proxy = Idr {
                idr: [
                    self.idr(0usize),
                    self.idr(1usize),
                    self.idr(2usize),
                    self.idr(3usize),
                    self.idr(4usize),
                    self.idr(5usize),
                    self.idr(6usize),
                    self.idr(7usize),
                    self.idr(8usize),
                    self.idr(9usize),
                    self.idr(10usize),
                    self.idr(11usize),
                    self.idr(12usize),
                    self.idr(13usize),
                    self.idr(14usize),
                    self.idr(15usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
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
    impl core::fmt::Debug for Lckr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lckr")
                .field(
                    "lck",
                    &[
                        self.lck(0usize),
                        self.lck(1usize),
                        self.lck(2usize),
                        self.lck(3usize),
                        self.lck(4usize),
                        self.lck(5usize),
                        self.lck(6usize),
                        self.lck(7usize),
                        self.lck(8usize),
                        self.lck(9usize),
                        self.lck(10usize),
                        self.lck(11usize),
                        self.lck(12usize),
                        self.lck(13usize),
                        self.lck(14usize),
                        self.lck(15usize),
                    ],
                )
                .field("lckk", &self.lckk())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lckr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Lckr {
                lck: [bool; 16usize],
                lckk: bool,
            }
            let proxy = Lckr {
                lck: [
                    self.lck(0usize),
                    self.lck(1usize),
                    self.lck(2usize),
                    self.lck(3usize),
                    self.lck(4usize),
                    self.lck(5usize),
                    self.lck(6usize),
                    self.lck(7usize),
                    self.lck(8usize),
                    self.lck(9usize),
                    self.lck(10usize),
                    self.lck(11usize),
                    self.lck(12usize),
                    self.lck(13usize),
                    self.lck(14usize),
                    self.lck(15usize),
                ],
                lckk: self.lckk(),
            };
            defmt::write!(f, "{}", proxy)
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
    impl core::fmt::Debug for Moder {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Moder")
                .field(
                    "moder",
                    &[
                        self.moder(0usize),
                        self.moder(1usize),
                        self.moder(2usize),
                        self.moder(3usize),
                        self.moder(4usize),
                        self.moder(5usize),
                        self.moder(6usize),
                        self.moder(7usize),
                        self.moder(8usize),
                        self.moder(9usize),
                        self.moder(10usize),
                        self.moder(11usize),
                        self.moder(12usize),
                        self.moder(13usize),
                        self.moder(14usize),
                        self.moder(15usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Moder {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Moder {
                moder: [super::vals::Moder; 16usize],
            }
            let proxy = Moder {
                moder: [
                    self.moder(0usize),
                    self.moder(1usize),
                    self.moder(2usize),
                    self.moder(3usize),
                    self.moder(4usize),
                    self.moder(5usize),
                    self.moder(6usize),
                    self.moder(7usize),
                    self.moder(8usize),
                    self.moder(9usize),
                    self.moder(10usize),
                    self.moder(11usize),
                    self.moder(12usize),
                    self.moder(13usize),
                    self.moder(14usize),
                    self.moder(15usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
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
    impl core::fmt::Debug for Odr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Odr")
                .field(
                    "odr",
                    &[
                        self.odr(0usize),
                        self.odr(1usize),
                        self.odr(2usize),
                        self.odr(3usize),
                        self.odr(4usize),
                        self.odr(5usize),
                        self.odr(6usize),
                        self.odr(7usize),
                        self.odr(8usize),
                        self.odr(9usize),
                        self.odr(10usize),
                        self.odr(11usize),
                        self.odr(12usize),
                        self.odr(13usize),
                        self.odr(14usize),
                        self.odr(15usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Odr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Odr {
                odr: [super::vals::Odr; 16usize],
            }
            let proxy = Odr {
                odr: [
                    self.odr(0usize),
                    self.odr(1usize),
                    self.odr(2usize),
                    self.odr(3usize),
                    self.odr(4usize),
                    self.odr(5usize),
                    self.odr(6usize),
                    self.odr(7usize),
                    self.odr(8usize),
                    self.odr(9usize),
                    self.odr(10usize),
                    self.odr(11usize),
                    self.odr(12usize),
                    self.odr(13usize),
                    self.odr(14usize),
                    self.odr(15usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
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
    impl core::fmt::Debug for Ospeedr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ospeedr")
                .field(
                    "ospeedr",
                    &[
                        self.ospeedr(0usize),
                        self.ospeedr(1usize),
                        self.ospeedr(2usize),
                        self.ospeedr(3usize),
                        self.ospeedr(4usize),
                        self.ospeedr(5usize),
                        self.ospeedr(6usize),
                        self.ospeedr(7usize),
                        self.ospeedr(8usize),
                        self.ospeedr(9usize),
                        self.ospeedr(10usize),
                        self.ospeedr(11usize),
                        self.ospeedr(12usize),
                        self.ospeedr(13usize),
                        self.ospeedr(14usize),
                        self.ospeedr(15usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ospeedr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ospeedr {
                ospeedr: [super::vals::Ospeedr; 16usize],
            }
            let proxy = Ospeedr {
                ospeedr: [
                    self.ospeedr(0usize),
                    self.ospeedr(1usize),
                    self.ospeedr(2usize),
                    self.ospeedr(3usize),
                    self.ospeedr(4usize),
                    self.ospeedr(5usize),
                    self.ospeedr(6usize),
                    self.ospeedr(7usize),
                    self.ospeedr(8usize),
                    self.ospeedr(9usize),
                    self.ospeedr(10usize),
                    self.ospeedr(11usize),
                    self.ospeedr(12usize),
                    self.ospeedr(13usize),
                    self.ospeedr(14usize),
                    self.ospeedr(15usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
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
    impl core::fmt::Debug for Otyper {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Otyper")
                .field(
                    "ot",
                    &[
                        self.ot(0usize),
                        self.ot(1usize),
                        self.ot(2usize),
                        self.ot(3usize),
                        self.ot(4usize),
                        self.ot(5usize),
                        self.ot(6usize),
                        self.ot(7usize),
                        self.ot(8usize),
                        self.ot(9usize),
                        self.ot(10usize),
                        self.ot(11usize),
                        self.ot(12usize),
                        self.ot(13usize),
                        self.ot(14usize),
                        self.ot(15usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Otyper {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Otyper {
                ot: [super::vals::Ot; 16usize],
            }
            let proxy = Otyper {
                ot: [
                    self.ot(0usize),
                    self.ot(1usize),
                    self.ot(2usize),
                    self.ot(3usize),
                    self.ot(4usize),
                    self.ot(5usize),
                    self.ot(6usize),
                    self.ot(7usize),
                    self.ot(8usize),
                    self.ot(9usize),
                    self.ot(10usize),
                    self.ot(11usize),
                    self.ot(12usize),
                    self.ot(13usize),
                    self.ot(14usize),
                    self.ot(15usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
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
    impl core::fmt::Debug for Pupdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pupdr")
                .field(
                    "pupdr",
                    &[
                        self.pupdr(0usize),
                        self.pupdr(1usize),
                        self.pupdr(2usize),
                        self.pupdr(3usize),
                        self.pupdr(4usize),
                        self.pupdr(5usize),
                        self.pupdr(6usize),
                        self.pupdr(7usize),
                        self.pupdr(8usize),
                        self.pupdr(9usize),
                        self.pupdr(10usize),
                        self.pupdr(11usize),
                        self.pupdr(12usize),
                        self.pupdr(13usize),
                        self.pupdr(14usize),
                        self.pupdr(15usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pupdr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Pupdr {
                pupdr: [super::vals::Pupdr; 16usize],
            }
            let proxy = Pupdr {
                pupdr: [
                    self.pupdr(0usize),
                    self.pupdr(1usize),
                    self.pupdr(2usize),
                    self.pupdr(3usize),
                    self.pupdr(4usize),
                    self.pupdr(5usize),
                    self.pupdr(6usize),
                    self.pupdr(7usize),
                    self.pupdr(8usize),
                    self.pupdr(9usize),
                    self.pupdr(10usize),
                    self.pupdr(11usize),
                    self.pupdr(12usize),
                    self.pupdr(13usize),
                    self.pupdr(14usize),
                    self.pupdr(15usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ospeedr {
        #[doc = "Low speed"]
        LOW_SPEED = 0x0,
        #[doc = "Medium speed"]
        MEDIUM_SPEED = 0x01,
        #[doc = "High speed"]
        HIGH_SPEED = 0x02,
        #[doc = "Very high speed"]
        VERY_HIGH_SPEED = 0x03,
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ot {
        #[doc = "Output push-pull (reset state)"]
        PUSH_PULL = 0x0,
        #[doc = "Output open-drain"]
        OPEN_DRAIN = 0x01,
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pupdr {
        #[doc = "No pull-up, pull-down"]
        FLOATING = 0x0,
        #[doc = "Pull-up"]
        PULL_UP = 0x01,
        #[doc = "Pull-down"]
        PULL_DOWN = 0x02,
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
