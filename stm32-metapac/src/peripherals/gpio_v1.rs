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
    impl core::fmt::Debug for Brr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Brr")
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
    impl defmt::Format for Brr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Brr {
                br: [bool; 16usize],
            }
            let proxy = Brr {
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
    impl core::fmt::Debug for Cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr")
                .field(
                    "mode",
                    &[
                        self.mode(0usize),
                        self.mode(1usize),
                        self.mode(2usize),
                        self.mode(3usize),
                        self.mode(4usize),
                        self.mode(5usize),
                        self.mode(6usize),
                        self.mode(7usize),
                    ],
                )
                .field(
                    "cnf_in",
                    &[
                        self.cnf_in(0usize),
                        self.cnf_in(1usize),
                        self.cnf_in(2usize),
                        self.cnf_in(3usize),
                        self.cnf_in(4usize),
                        self.cnf_in(5usize),
                        self.cnf_in(6usize),
                        self.cnf_in(7usize),
                    ],
                )
                .field(
                    "cnf_out",
                    &[
                        self.cnf_out(0usize),
                        self.cnf_out(1usize),
                        self.cnf_out(2usize),
                        self.cnf_out(3usize),
                        self.cnf_out(4usize),
                        self.cnf_out(5usize),
                        self.cnf_out(6usize),
                        self.cnf_out(7usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cr {
                mode: [super::vals::Mode; 8usize],
                cnf_in: [super::vals::CnfIn; 8usize],
                cnf_out: [super::vals::CnfOut; 8usize],
            }
            let proxy = Cr {
                mode: [
                    self.mode(0usize),
                    self.mode(1usize),
                    self.mode(2usize),
                    self.mode(3usize),
                    self.mode(4usize),
                    self.mode(5usize),
                    self.mode(6usize),
                    self.mode(7usize),
                ],
                cnf_in: [
                    self.cnf_in(0usize),
                    self.cnf_in(1usize),
                    self.cnf_in(2usize),
                    self.cnf_in(3usize),
                    self.cnf_in(4usize),
                    self.cnf_in(5usize),
                    self.cnf_in(6usize),
                    self.cnf_in(7usize),
                ],
                cnf_out: [
                    self.cnf_out(0usize),
                    self.cnf_out(1usize),
                    self.cnf_out(2usize),
                    self.cnf_out(3usize),
                    self.cnf_out(4usize),
                    self.cnf_out(5usize),
                    self.cnf_out(6usize),
                    self.cnf_out(7usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
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
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum CnfOut {
        #[doc = "Push-Pull mode"]
        PUSH_PULL = 0x0,
        #[doc = "Open Drain-Mode"]
        OPEN_DRAIN = 0x01,
        #[doc = "Alternate Function Push-Pull Mode"]
        ALT_PUSH_PULL = 0x02,
        #[doc = "Alternate Function Open-Drain Mode"]
        ALT_OPEN_DRAIN = 0x03,
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
}
