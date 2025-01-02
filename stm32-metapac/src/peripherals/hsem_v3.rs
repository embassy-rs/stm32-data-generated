#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Hardware semaphore."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hsem {
    ptr: *mut u8,
}
unsafe impl Send for Hsem {}
unsafe impl Sync for Hsem {}
impl Hsem {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "HSEM register HSEM_R%s HSEM_R31."]
    #[inline(always)]
    pub const fn r(self, n: usize) -> crate::common::Reg<regs::R, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "HSEM Read lock register."]
    #[inline(always)]
    pub const fn rlr(self, n: usize) -> crate::common::Reg<regs::Rlr, crate::common::R> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 4usize) as _) }
    }
    #[doc = "HSEM Interrupt enable register."]
    #[inline(always)]
    pub const fn ier(self, n: usize) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 16usize) as _) }
    }
    #[doc = "HSEM Interrupt clear register."]
    #[inline(always)]
    pub const fn icr(self, n: usize) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize + n * 16usize) as _) }
    }
    #[doc = "HSEM Interrupt status register."]
    #[inline(always)]
    pub const fn isr(self, n: usize) -> crate::common::Reg<regs::Isr, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize + n * 16usize) as _) }
    }
    #[doc = "HSEM Masked interrupt status register."]
    #[inline(always)]
    pub const fn misr(self, n: usize) -> crate::common::Reg<regs::Misr, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize + n * 16usize) as _) }
    }
    #[doc = "HSEM Clear register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "HSEM Interrupt clear register."]
    #[inline(always)]
    pub const fn keyr(self) -> crate::common::Reg<regs::Keyr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
}
pub mod regs {
    #[doc = "HSEM Clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "COREID."]
        #[inline(always)]
        pub const fn coreid(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "COREID."]
        #[inline(always)]
        pub fn set_coreid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Semaphore clear Key."]
        #[inline(always)]
        pub const fn key(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Semaphore clear Key."]
        #[inline(always)]
        pub fn set_key(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
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
                .field("coreid", &self.coreid())
                .field("key", &self.key())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cr {
                coreid: u8,
                key: u16,
            }
            let proxy = Cr {
                coreid: self.coreid(),
                key: self.key(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "HSEM Interrupt clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "Interrupt semaphore x clear bit."]
        #[inline(always)]
        pub const fn isc(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Interrupt semaphore x clear bit."]
        #[inline(always)]
        pub fn set_isc(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Icr {
        #[inline(always)]
        fn default() -> Icr {
            Icr(0)
        }
    }
    impl core::fmt::Debug for Icr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icr")
                .field(
                    "isc",
                    &[
                        self.isc(0usize),
                        self.isc(1usize),
                        self.isc(2usize),
                        self.isc(3usize),
                        self.isc(4usize),
                        self.isc(5usize),
                        self.isc(6usize),
                        self.isc(7usize),
                        self.isc(8usize),
                        self.isc(9usize),
                        self.isc(10usize),
                        self.isc(11usize),
                        self.isc(12usize),
                        self.isc(13usize),
                        self.isc(14usize),
                        self.isc(15usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Icr {
                isc: [bool; 16usize],
            }
            let proxy = Icr {
                isc: [
                    self.isc(0usize),
                    self.isc(1usize),
                    self.isc(2usize),
                    self.isc(3usize),
                    self.isc(4usize),
                    self.isc(5usize),
                    self.isc(6usize),
                    self.isc(7usize),
                    self.isc(8usize),
                    self.isc(9usize),
                    self.isc(10usize),
                    self.isc(11usize),
                    self.isc(12usize),
                    self.isc(13usize),
                    self.isc(14usize),
                    self.isc(15usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "HSEM Interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "Interrupt semaphore x enable bit."]
        #[inline(always)]
        pub const fn ise(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Interrupt semaphore x enable bit."]
        #[inline(always)]
        pub fn set_ise(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Ier {
        #[inline(always)]
        fn default() -> Ier {
            Ier(0)
        }
    }
    impl core::fmt::Debug for Ier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ier")
                .field(
                    "ise",
                    &[
                        self.ise(0usize),
                        self.ise(1usize),
                        self.ise(2usize),
                        self.ise(3usize),
                        self.ise(4usize),
                        self.ise(5usize),
                        self.ise(6usize),
                        self.ise(7usize),
                        self.ise(8usize),
                        self.ise(9usize),
                        self.ise(10usize),
                        self.ise(11usize),
                        self.ise(12usize),
                        self.ise(13usize),
                        self.ise(14usize),
                        self.ise(15usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ier {
                ise: [bool; 16usize],
            }
            let proxy = Ier {
                ise: [
                    self.ise(0usize),
                    self.ise(1usize),
                    self.ise(2usize),
                    self.ise(3usize),
                    self.ise(4usize),
                    self.ise(5usize),
                    self.ise(6usize),
                    self.ise(7usize),
                    self.ise(8usize),
                    self.ise(9usize),
                    self.ise(10usize),
                    self.ise(11usize),
                    self.ise(12usize),
                    self.ise(13usize),
                    self.ise(14usize),
                    self.ise(15usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "HSEM Interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "Interrupt semaphore x status bit before enable (mask)."]
        #[inline(always)]
        pub const fn isf(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Interrupt semaphore x status bit before enable (mask)."]
        #[inline(always)]
        pub fn set_isf(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Isr {
        #[inline(always)]
        fn default() -> Isr {
            Isr(0)
        }
    }
    impl core::fmt::Debug for Isr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Isr")
                .field(
                    "isf",
                    &[
                        self.isf(0usize),
                        self.isf(1usize),
                        self.isf(2usize),
                        self.isf(3usize),
                        self.isf(4usize),
                        self.isf(5usize),
                        self.isf(6usize),
                        self.isf(7usize),
                        self.isf(8usize),
                        self.isf(9usize),
                        self.isf(10usize),
                        self.isf(11usize),
                        self.isf(12usize),
                        self.isf(13usize),
                        self.isf(14usize),
                        self.isf(15usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Isr {
                isf: [bool; 16usize],
            }
            let proxy = Isr {
                isf: [
                    self.isf(0usize),
                    self.isf(1usize),
                    self.isf(2usize),
                    self.isf(3usize),
                    self.isf(4usize),
                    self.isf(5usize),
                    self.isf(6usize),
                    self.isf(7usize),
                    self.isf(8usize),
                    self.isf(9usize),
                    self.isf(10usize),
                    self.isf(11usize),
                    self.isf(12usize),
                    self.isf(13usize),
                    self.isf(14usize),
                    self.isf(15usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "HSEM Interrupt clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Keyr(pub u32);
    impl Keyr {
        #[doc = "Semaphore Clear Key."]
        #[inline(always)]
        pub const fn key(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Semaphore Clear Key."]
        #[inline(always)]
        pub fn set_key(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Keyr {
        #[inline(always)]
        fn default() -> Keyr {
            Keyr(0)
        }
    }
    impl core::fmt::Debug for Keyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Keyr").field("key", &self.key()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Keyr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Keyr {
                key: u16,
            }
            let proxy = Keyr { key: self.key() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "HSEM Masked interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Misr(pub u32);
    impl Misr {
        #[doc = "masked interrupt semaphore x status bit after enable (mask)."]
        #[inline(always)]
        pub const fn misf(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "masked interrupt semaphore x status bit after enable (mask)."]
        #[inline(always)]
        pub fn set_misf(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Misr {
        #[inline(always)]
        fn default() -> Misr {
            Misr(0)
        }
    }
    impl core::fmt::Debug for Misr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Misr")
                .field(
                    "misf",
                    &[
                        self.misf(0usize),
                        self.misf(1usize),
                        self.misf(2usize),
                        self.misf(3usize),
                        self.misf(4usize),
                        self.misf(5usize),
                        self.misf(6usize),
                        self.misf(7usize),
                        self.misf(8usize),
                        self.misf(9usize),
                        self.misf(10usize),
                        self.misf(11usize),
                        self.misf(12usize),
                        self.misf(13usize),
                        self.misf(14usize),
                        self.misf(15usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Misr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Misr {
                misf: [bool; 16usize],
            }
            let proxy = Misr {
                misf: [
                    self.misf(0usize),
                    self.misf(1usize),
                    self.misf(2usize),
                    self.misf(3usize),
                    self.misf(4usize),
                    self.misf(5usize),
                    self.misf(6usize),
                    self.misf(7usize),
                    self.misf(8usize),
                    self.misf(9usize),
                    self.misf(10usize),
                    self.misf(11usize),
                    self.misf(12usize),
                    self.misf(13usize),
                    self.misf(14usize),
                    self.misf(15usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "HSEM register HSEM_R%s HSEM_R31."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R(pub u32);
    impl R {
        #[doc = "Semaphore ProcessID."]
        #[inline(always)]
        pub const fn procid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Semaphore ProcessID."]
        #[inline(always)]
        pub fn set_procid(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "COREID."]
        #[inline(always)]
        pub const fn coreid(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "COREID."]
        #[inline(always)]
        pub fn set_coreid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Lock indication."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Lock indication."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for R {
        #[inline(always)]
        fn default() -> R {
            R(0)
        }
    }
    impl core::fmt::Debug for R {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("R")
                .field("procid", &self.procid())
                .field("coreid", &self.coreid())
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for R {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct R {
                procid: u8,
                coreid: u8,
                lock: bool,
            }
            let proxy = R {
                procid: self.procid(),
                coreid: self.coreid(),
                lock: self.lock(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "HSEM Read lock register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rlr(pub u32);
    impl Rlr {
        #[doc = "Semaphore ProcessID."]
        #[inline(always)]
        pub const fn procid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Semaphore ProcessID."]
        #[inline(always)]
        pub fn set_procid(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "COREID."]
        #[inline(always)]
        pub const fn coreid(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "COREID."]
        #[inline(always)]
        pub fn set_coreid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Lock indication."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Lock indication."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Rlr {
        #[inline(always)]
        fn default() -> Rlr {
            Rlr(0)
        }
    }
    impl core::fmt::Debug for Rlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rlr")
                .field("procid", &self.procid())
                .field("coreid", &self.coreid())
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rlr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Rlr {
                procid: u8,
                coreid: u8,
                lock: bool,
            }
            let proxy = Rlr {
                procid: self.procid(),
                coreid: self.coreid(),
                lock: self.lock(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
