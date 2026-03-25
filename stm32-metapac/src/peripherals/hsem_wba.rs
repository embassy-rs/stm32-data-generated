#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Hardware semaphore (HSEM), STM32WBA."]
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
    #[doc = "HSEM 2-step write lock and read-back registers R0–R15."]
    #[inline(always)]
    pub const fn r(self, n: usize) -> crate::common::Reg<regs::R, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "HSEM 1-step read lock registers RLR0–RLR15."]
    #[inline(always)]
    pub const fn rlr(self, n: usize) -> crate::common::Reg<regs::Rlr, crate::common::R> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 4usize) as _) }
    }
    #[doc = "HSEM interrupt enable register."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "HSEM interrupt clear register."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "HSEM interrupt status register."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "HSEM masked interrupt status register."]
    #[inline(always)]
    pub const fn misr(self) -> crate::common::Reg<regs::Misr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "HSEM secure interrupt enable register."]
    #[inline(always)]
    pub const fn sier(self) -> crate::common::Reg<regs::Sier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[doc = "HSEM secure interrupt clear register."]
    #[inline(always)]
    pub const fn sicr(self) -> crate::common::Reg<regs::Sicr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
    }
    #[doc = "HSEM secure interrupt status register."]
    #[inline(always)]
    pub const fn sisr(self) -> crate::common::Reg<regs::Sisr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize) as _) }
    }
    #[doc = "HSEM secure masked interrupt status register."]
    #[inline(always)]
    pub const fn smisr(self) -> crate::common::Reg<regs::Smisr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x018cusize) as _) }
    }
    #[doc = "HSEM security configuration register."]
    #[inline(always)]
    pub const fn seccfgr(self) -> crate::common::Reg<regs::Seccfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[doc = "HSEM privilege configuration register."]
    #[inline(always)]
    pub const fn privcfgr(self) -> crate::common::Reg<regs::Privcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0210usize) as _) }
    }
    #[doc = "HSEM semaphore clear register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0230usize) as _) }
    }
    #[doc = "HSEM semaphore clear key register."]
    #[inline(always)]
    pub const fn keyr(self) -> crate::common::Reg<regs::Keyr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0234usize) as _) }
    }
}
pub mod regs {
    #[doc = "HSEM semaphore clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "LOCKID of semaphores to be cleared."]
        #[inline(always)]
        pub const fn lockid(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "LOCKID of semaphores to be cleared."]
        #[inline(always)]
        pub fn set_lockid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "SEC value of semaphores to be cleared."]
        #[inline(always)]
        pub const fn sec(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SEC value of semaphores to be cleared."]
        #[inline(always)]
        pub fn set_sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "PRIV value of semaphores to be cleared."]
        #[inline(always)]
        pub const fn priv_(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "PRIV value of semaphores to be cleared."]
        #[inline(always)]
        pub fn set_priv_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Semaphore clear key."]
        #[inline(always)]
        pub const fn key(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Semaphore clear key."]
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
                .field("lockid", &self.lockid())
                .field("sec", &self.sec())
                .field("priv_", &self.priv_())
                .field("key", &self.key())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ lockid: {=u8:?}, sec: {=bool:?}, priv_: {=bool:?}, key: {=u16:?} }}",
                self.lockid(),
                self.sec(),
                self.priv_(),
                self.key()
            )
        }
    }
    #[doc = "HSEM interrupt clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "Interrupt semaphore n clear bit."]
        #[inline(always)]
        pub const fn isc(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Interrupt semaphore n clear bit."]
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
                .field("isc[0]", &self.isc(0usize))
                .field("isc[1]", &self.isc(1usize))
                .field("isc[2]", &self.isc(2usize))
                .field("isc[3]", &self.isc(3usize))
                .field("isc[4]", &self.isc(4usize))
                .field("isc[5]", &self.isc(5usize))
                .field("isc[6]", &self.isc(6usize))
                .field("isc[7]", &self.isc(7usize))
                .field("isc[8]", &self.isc(8usize))
                .field("isc[9]", &self.isc(9usize))
                .field("isc[10]", &self.isc(10usize))
                .field("isc[11]", &self.isc(11usize))
                .field("isc[12]", &self.isc(12usize))
                .field("isc[13]", &self.isc(13usize))
                .field("isc[14]", &self.isc(14usize))
                .field("isc[15]", &self.isc(15usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Icr {{ isc[0]: {=bool:?}, isc[1]: {=bool:?}, isc[2]: {=bool:?}, isc[3]: {=bool:?}, isc[4]: {=bool:?}, isc[5]: {=bool:?}, isc[6]: {=bool:?}, isc[7]: {=bool:?}, isc[8]: {=bool:?}, isc[9]: {=bool:?}, isc[10]: {=bool:?}, isc[11]: {=bool:?}, isc[12]: {=bool:?}, isc[13]: {=bool:?}, isc[14]: {=bool:?}, isc[15]: {=bool:?} }}" , self . isc (0usize) , self . isc (1usize) , self . isc (2usize) , self . isc (3usize) , self . isc (4usize) , self . isc (5usize) , self . isc (6usize) , self . isc (7usize) , self . isc (8usize) , self . isc (9usize) , self . isc (10usize) , self . isc (11usize) , self . isc (12usize) , self . isc (13usize) , self . isc (14usize) , self . isc (15usize))
        }
    }
    #[doc = "HSEM interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "Interrupt semaphore n enable bit."]
        #[inline(always)]
        pub const fn ise(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Interrupt semaphore n enable bit."]
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
                .field("ise[0]", &self.ise(0usize))
                .field("ise[1]", &self.ise(1usize))
                .field("ise[2]", &self.ise(2usize))
                .field("ise[3]", &self.ise(3usize))
                .field("ise[4]", &self.ise(4usize))
                .field("ise[5]", &self.ise(5usize))
                .field("ise[6]", &self.ise(6usize))
                .field("ise[7]", &self.ise(7usize))
                .field("ise[8]", &self.ise(8usize))
                .field("ise[9]", &self.ise(9usize))
                .field("ise[10]", &self.ise(10usize))
                .field("ise[11]", &self.ise(11usize))
                .field("ise[12]", &self.ise(12usize))
                .field("ise[13]", &self.ise(13usize))
                .field("ise[14]", &self.ise(14usize))
                .field("ise[15]", &self.ise(15usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ier {{ ise[0]: {=bool:?}, ise[1]: {=bool:?}, ise[2]: {=bool:?}, ise[3]: {=bool:?}, ise[4]: {=bool:?}, ise[5]: {=bool:?}, ise[6]: {=bool:?}, ise[7]: {=bool:?}, ise[8]: {=bool:?}, ise[9]: {=bool:?}, ise[10]: {=bool:?}, ise[11]: {=bool:?}, ise[12]: {=bool:?}, ise[13]: {=bool:?}, ise[14]: {=bool:?}, ise[15]: {=bool:?} }}" , self . ise (0usize) , self . ise (1usize) , self . ise (2usize) , self . ise (3usize) , self . ise (4usize) , self . ise (5usize) , self . ise (6usize) , self . ise (7usize) , self . ise (8usize) , self . ise (9usize) , self . ise (10usize) , self . ise (11usize) , self . ise (12usize) , self . ise (13usize) , self . ise (14usize) , self . ise (15usize))
        }
    }
    #[doc = "HSEM interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "Interrupt semaphore n status bit before enable (mask)."]
        #[inline(always)]
        pub const fn isf(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Interrupt semaphore n status bit before enable (mask)."]
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
                .field("isf[0]", &self.isf(0usize))
                .field("isf[1]", &self.isf(1usize))
                .field("isf[2]", &self.isf(2usize))
                .field("isf[3]", &self.isf(3usize))
                .field("isf[4]", &self.isf(4usize))
                .field("isf[5]", &self.isf(5usize))
                .field("isf[6]", &self.isf(6usize))
                .field("isf[7]", &self.isf(7usize))
                .field("isf[8]", &self.isf(8usize))
                .field("isf[9]", &self.isf(9usize))
                .field("isf[10]", &self.isf(10usize))
                .field("isf[11]", &self.isf(11usize))
                .field("isf[12]", &self.isf(12usize))
                .field("isf[13]", &self.isf(13usize))
                .field("isf[14]", &self.isf(14usize))
                .field("isf[15]", &self.isf(15usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Isr {{ isf[0]: {=bool:?}, isf[1]: {=bool:?}, isf[2]: {=bool:?}, isf[3]: {=bool:?}, isf[4]: {=bool:?}, isf[5]: {=bool:?}, isf[6]: {=bool:?}, isf[7]: {=bool:?}, isf[8]: {=bool:?}, isf[9]: {=bool:?}, isf[10]: {=bool:?}, isf[11]: {=bool:?}, isf[12]: {=bool:?}, isf[13]: {=bool:?}, isf[14]: {=bool:?}, isf[15]: {=bool:?} }}" , self . isf (0usize) , self . isf (1usize) , self . isf (2usize) , self . isf (3usize) , self . isf (4usize) , self . isf (5usize) , self . isf (6usize) , self . isf (7usize) , self . isf (8usize) , self . isf (9usize) , self . isf (10usize) , self . isf (11usize) , self . isf (12usize) , self . isf (13usize) , self . isf (14usize) , self . isf (15usize))
        }
    }
    #[doc = "HSEM semaphore clear key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Keyr(pub u32);
    impl Keyr {
        #[doc = "Semaphore clear key."]
        #[inline(always)]
        pub const fn key(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Semaphore clear key."]
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
            defmt::write!(f, "Keyr {{ key: {=u16:?} }}", self.key())
        }
    }
    #[doc = "HSEM masked interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Misr(pub u32);
    impl Misr {
        #[doc = "Masked interrupt semaphore n status bit after enable (mask)."]
        #[inline(always)]
        pub const fn misf(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Masked interrupt semaphore n status bit after enable (mask)."]
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
                .field("misf[0]", &self.misf(0usize))
                .field("misf[1]", &self.misf(1usize))
                .field("misf[2]", &self.misf(2usize))
                .field("misf[3]", &self.misf(3usize))
                .field("misf[4]", &self.misf(4usize))
                .field("misf[5]", &self.misf(5usize))
                .field("misf[6]", &self.misf(6usize))
                .field("misf[7]", &self.misf(7usize))
                .field("misf[8]", &self.misf(8usize))
                .field("misf[9]", &self.misf(9usize))
                .field("misf[10]", &self.misf(10usize))
                .field("misf[11]", &self.misf(11usize))
                .field("misf[12]", &self.misf(12usize))
                .field("misf[13]", &self.misf(13usize))
                .field("misf[14]", &self.misf(14usize))
                .field("misf[15]", &self.misf(15usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Misr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Misr {{ misf[0]: {=bool:?}, misf[1]: {=bool:?}, misf[2]: {=bool:?}, misf[3]: {=bool:?}, misf[4]: {=bool:?}, misf[5]: {=bool:?}, misf[6]: {=bool:?}, misf[7]: {=bool:?}, misf[8]: {=bool:?}, misf[9]: {=bool:?}, misf[10]: {=bool:?}, misf[11]: {=bool:?}, misf[12]: {=bool:?}, misf[13]: {=bool:?}, misf[14]: {=bool:?}, misf[15]: {=bool:?} }}" , self . misf (0usize) , self . misf (1usize) , self . misf (2usize) , self . misf (3usize) , self . misf (4usize) , self . misf (5usize) , self . misf (6usize) , self . misf (7usize) , self . misf (8usize) , self . misf (9usize) , self . misf (10usize) , self . misf (11usize) , self . misf (12usize) , self . misf (13usize) , self . misf (14usize) , self . misf (15usize))
        }
    }
    #[doc = "HSEM privilege configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcfgr(pub u32);
    impl Privcfgr {
        #[doc = "Semaphore n privilege attribute status bit."]
        #[inline(always)]
        pub const fn priv_(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Semaphore n privilege attribute status bit."]
        #[inline(always)]
        pub fn set_priv_(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Privcfgr {
        #[inline(always)]
        fn default() -> Privcfgr {
            Privcfgr(0)
        }
    }
    impl core::fmt::Debug for Privcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Privcfgr")
                .field("priv_[0]", &self.priv_(0usize))
                .field("priv_[1]", &self.priv_(1usize))
                .field("priv_[2]", &self.priv_(2usize))
                .field("priv_[3]", &self.priv_(3usize))
                .field("priv_[4]", &self.priv_(4usize))
                .field("priv_[5]", &self.priv_(5usize))
                .field("priv_[6]", &self.priv_(6usize))
                .field("priv_[7]", &self.priv_(7usize))
                .field("priv_[8]", &self.priv_(8usize))
                .field("priv_[9]", &self.priv_(9usize))
                .field("priv_[10]", &self.priv_(10usize))
                .field("priv_[11]", &self.priv_(11usize))
                .field("priv_[12]", &self.priv_(12usize))
                .field("priv_[13]", &self.priv_(13usize))
                .field("priv_[14]", &self.priv_(14usize))
                .field("priv_[15]", &self.priv_(15usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Privcfgr {{ priv_[0]: {=bool:?}, priv_[1]: {=bool:?}, priv_[2]: {=bool:?}, priv_[3]: {=bool:?}, priv_[4]: {=bool:?}, priv_[5]: {=bool:?}, priv_[6]: {=bool:?}, priv_[7]: {=bool:?}, priv_[8]: {=bool:?}, priv_[9]: {=bool:?}, priv_[10]: {=bool:?}, priv_[11]: {=bool:?}, priv_[12]: {=bool:?}, priv_[13]: {=bool:?}, priv_[14]: {=bool:?}, priv_[15]: {=bool:?} }}" , self . priv_ (0usize) , self . priv_ (1usize) , self . priv_ (2usize) , self . priv_ (3usize) , self . priv_ (4usize) , self . priv_ (5usize) , self . priv_ (6usize) , self . priv_ (7usize) , self . priv_ (8usize) , self . priv_ (9usize) , self . priv_ (10usize) , self . priv_ (11usize) , self . priv_ (12usize) , self . priv_ (13usize) , self . priv_ (14usize) , self . priv_ (15usize))
        }
    }
    #[doc = "HSEM semaphore register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R(pub u32);
    impl R {
        #[doc = "Semaphore process ID."]
        #[inline(always)]
        pub const fn procid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Semaphore process ID."]
        #[inline(always)]
        pub fn set_procid(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Semaphore lock ID."]
        #[inline(always)]
        pub const fn lockid(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Semaphore lock ID."]
        #[inline(always)]
        pub fn set_lockid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Semaphore secure bit."]
        #[inline(always)]
        pub const fn sec(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Semaphore secure bit."]
        #[inline(always)]
        pub fn set_sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Semaphore privilege bit."]
        #[inline(always)]
        pub const fn priv_(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Semaphore privilege bit."]
        #[inline(always)]
        pub fn set_priv_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
                .field("lockid", &self.lockid())
                .field("sec", &self.sec())
                .field("priv_", &self.priv_())
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for R {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "R {{ procid: {=u8:?}, lockid: {=u8:?}, sec: {=bool:?}, priv_: {=bool:?}, lock: {=bool:?} }}",
                self.procid(),
                self.lockid(),
                self.sec(),
                self.priv_(),
                self.lock()
            )
        }
    }
    #[doc = "HSEM read lock register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rlr(pub u32);
    impl Rlr {
        #[doc = "Semaphore process ID."]
        #[inline(always)]
        pub const fn procid(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Semaphore process ID."]
        #[inline(always)]
        pub fn set_procid(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Semaphore lock ID."]
        #[inline(always)]
        pub const fn lockid(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Semaphore lock ID."]
        #[inline(always)]
        pub fn set_lockid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Semaphore secure bit."]
        #[inline(always)]
        pub const fn sec(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Semaphore secure bit."]
        #[inline(always)]
        pub fn set_sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Semaphore privilege bit."]
        #[inline(always)]
        pub const fn priv_(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Semaphore privilege bit."]
        #[inline(always)]
        pub fn set_priv_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
                .field("lockid", &self.lockid())
                .field("sec", &self.sec())
                .field("priv_", &self.priv_())
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rlr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rlr {{ procid: {=u8:?}, lockid: {=u8:?}, sec: {=bool:?}, priv_: {=bool:?}, lock: {=bool:?} }}",
                self.procid(),
                self.lockid(),
                self.sec(),
                self.priv_(),
                self.lock()
            )
        }
    }
    #[doc = "HSEM security configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccfgr(pub u32);
    impl Seccfgr {
        #[doc = "Semaphore n secure attribute status bit."]
        #[inline(always)]
        pub const fn sec(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Semaphore n secure attribute status bit."]
        #[inline(always)]
        pub fn set_sec(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Seccfgr {
        #[inline(always)]
        fn default() -> Seccfgr {
            Seccfgr(0)
        }
    }
    impl core::fmt::Debug for Seccfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Seccfgr")
                .field("sec[0]", &self.sec(0usize))
                .field("sec[1]", &self.sec(1usize))
                .field("sec[2]", &self.sec(2usize))
                .field("sec[3]", &self.sec(3usize))
                .field("sec[4]", &self.sec(4usize))
                .field("sec[5]", &self.sec(5usize))
                .field("sec[6]", &self.sec(6usize))
                .field("sec[7]", &self.sec(7usize))
                .field("sec[8]", &self.sec(8usize))
                .field("sec[9]", &self.sec(9usize))
                .field("sec[10]", &self.sec(10usize))
                .field("sec[11]", &self.sec(11usize))
                .field("sec[12]", &self.sec(12usize))
                .field("sec[13]", &self.sec(13usize))
                .field("sec[14]", &self.sec(14usize))
                .field("sec[15]", &self.sec(15usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Seccfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Seccfgr {{ sec[0]: {=bool:?}, sec[1]: {=bool:?}, sec[2]: {=bool:?}, sec[3]: {=bool:?}, sec[4]: {=bool:?}, sec[5]: {=bool:?}, sec[6]: {=bool:?}, sec[7]: {=bool:?}, sec[8]: {=bool:?}, sec[9]: {=bool:?}, sec[10]: {=bool:?}, sec[11]: {=bool:?}, sec[12]: {=bool:?}, sec[13]: {=bool:?}, sec[14]: {=bool:?}, sec[15]: {=bool:?} }}" , self . sec (0usize) , self . sec (1usize) , self . sec (2usize) , self . sec (3usize) , self . sec (4usize) , self . sec (5usize) , self . sec (6usize) , self . sec (7usize) , self . sec (8usize) , self . sec (9usize) , self . sec (10usize) , self . sec (11usize) , self . sec (12usize) , self . sec (13usize) , self . sec (14usize) , self . sec (15usize))
        }
    }
    #[doc = "HSEM secure interrupt clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sicr(pub u32);
    impl Sicr {
        #[doc = "Secure interrupt semaphore n clear bit."]
        #[inline(always)]
        pub const fn isc(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Secure interrupt semaphore n clear bit."]
        #[inline(always)]
        pub fn set_isc(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Sicr {
        #[inline(always)]
        fn default() -> Sicr {
            Sicr(0)
        }
    }
    impl core::fmt::Debug for Sicr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sicr")
                .field("isc[0]", &self.isc(0usize))
                .field("isc[1]", &self.isc(1usize))
                .field("isc[2]", &self.isc(2usize))
                .field("isc[3]", &self.isc(3usize))
                .field("isc[4]", &self.isc(4usize))
                .field("isc[5]", &self.isc(5usize))
                .field("isc[6]", &self.isc(6usize))
                .field("isc[7]", &self.isc(7usize))
                .field("isc[8]", &self.isc(8usize))
                .field("isc[9]", &self.isc(9usize))
                .field("isc[10]", &self.isc(10usize))
                .field("isc[11]", &self.isc(11usize))
                .field("isc[12]", &self.isc(12usize))
                .field("isc[13]", &self.isc(13usize))
                .field("isc[14]", &self.isc(14usize))
                .field("isc[15]", &self.isc(15usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sicr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sicr {{ isc[0]: {=bool:?}, isc[1]: {=bool:?}, isc[2]: {=bool:?}, isc[3]: {=bool:?}, isc[4]: {=bool:?}, isc[5]: {=bool:?}, isc[6]: {=bool:?}, isc[7]: {=bool:?}, isc[8]: {=bool:?}, isc[9]: {=bool:?}, isc[10]: {=bool:?}, isc[11]: {=bool:?}, isc[12]: {=bool:?}, isc[13]: {=bool:?}, isc[14]: {=bool:?}, isc[15]: {=bool:?} }}" , self . isc (0usize) , self . isc (1usize) , self . isc (2usize) , self . isc (3usize) , self . isc (4usize) , self . isc (5usize) , self . isc (6usize) , self . isc (7usize) , self . isc (8usize) , self . isc (9usize) , self . isc (10usize) , self . isc (11usize) , self . isc (12usize) , self . isc (13usize) , self . isc (14usize) , self . isc (15usize))
        }
    }
    #[doc = "HSEM secure interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sier(pub u32);
    impl Sier {
        #[doc = "Secure interrupt semaphore n enable bit."]
        #[inline(always)]
        pub const fn ise(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Secure interrupt semaphore n enable bit."]
        #[inline(always)]
        pub fn set_ise(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Sier {
        #[inline(always)]
        fn default() -> Sier {
            Sier(0)
        }
    }
    impl core::fmt::Debug for Sier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sier")
                .field("ise[0]", &self.ise(0usize))
                .field("ise[1]", &self.ise(1usize))
                .field("ise[2]", &self.ise(2usize))
                .field("ise[3]", &self.ise(3usize))
                .field("ise[4]", &self.ise(4usize))
                .field("ise[5]", &self.ise(5usize))
                .field("ise[6]", &self.ise(6usize))
                .field("ise[7]", &self.ise(7usize))
                .field("ise[8]", &self.ise(8usize))
                .field("ise[9]", &self.ise(9usize))
                .field("ise[10]", &self.ise(10usize))
                .field("ise[11]", &self.ise(11usize))
                .field("ise[12]", &self.ise(12usize))
                .field("ise[13]", &self.ise(13usize))
                .field("ise[14]", &self.ise(14usize))
                .field("ise[15]", &self.ise(15usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sier {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sier {{ ise[0]: {=bool:?}, ise[1]: {=bool:?}, ise[2]: {=bool:?}, ise[3]: {=bool:?}, ise[4]: {=bool:?}, ise[5]: {=bool:?}, ise[6]: {=bool:?}, ise[7]: {=bool:?}, ise[8]: {=bool:?}, ise[9]: {=bool:?}, ise[10]: {=bool:?}, ise[11]: {=bool:?}, ise[12]: {=bool:?}, ise[13]: {=bool:?}, ise[14]: {=bool:?}, ise[15]: {=bool:?} }}" , self . ise (0usize) , self . ise (1usize) , self . ise (2usize) , self . ise (3usize) , self . ise (4usize) , self . ise (5usize) , self . ise (6usize) , self . ise (7usize) , self . ise (8usize) , self . ise (9usize) , self . ise (10usize) , self . ise (11usize) , self . ise (12usize) , self . ise (13usize) , self . ise (14usize) , self . ise (15usize))
        }
    }
    #[doc = "HSEM secure interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sisr(pub u32);
    impl Sisr {
        #[doc = "Secure interrupt semaphore n status bit before enable (mask)."]
        #[inline(always)]
        pub const fn isf(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Secure interrupt semaphore n status bit before enable (mask)."]
        #[inline(always)]
        pub fn set_isf(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Sisr {
        #[inline(always)]
        fn default() -> Sisr {
            Sisr(0)
        }
    }
    impl core::fmt::Debug for Sisr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sisr")
                .field("isf[0]", &self.isf(0usize))
                .field("isf[1]", &self.isf(1usize))
                .field("isf[2]", &self.isf(2usize))
                .field("isf[3]", &self.isf(3usize))
                .field("isf[4]", &self.isf(4usize))
                .field("isf[5]", &self.isf(5usize))
                .field("isf[6]", &self.isf(6usize))
                .field("isf[7]", &self.isf(7usize))
                .field("isf[8]", &self.isf(8usize))
                .field("isf[9]", &self.isf(9usize))
                .field("isf[10]", &self.isf(10usize))
                .field("isf[11]", &self.isf(11usize))
                .field("isf[12]", &self.isf(12usize))
                .field("isf[13]", &self.isf(13usize))
                .field("isf[14]", &self.isf(14usize))
                .field("isf[15]", &self.isf(15usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sisr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sisr {{ isf[0]: {=bool:?}, isf[1]: {=bool:?}, isf[2]: {=bool:?}, isf[3]: {=bool:?}, isf[4]: {=bool:?}, isf[5]: {=bool:?}, isf[6]: {=bool:?}, isf[7]: {=bool:?}, isf[8]: {=bool:?}, isf[9]: {=bool:?}, isf[10]: {=bool:?}, isf[11]: {=bool:?}, isf[12]: {=bool:?}, isf[13]: {=bool:?}, isf[14]: {=bool:?}, isf[15]: {=bool:?} }}" , self . isf (0usize) , self . isf (1usize) , self . isf (2usize) , self . isf (3usize) , self . isf (4usize) , self . isf (5usize) , self . isf (6usize) , self . isf (7usize) , self . isf (8usize) , self . isf (9usize) , self . isf (10usize) , self . isf (11usize) , self . isf (12usize) , self . isf (13usize) , self . isf (14usize) , self . isf (15usize))
        }
    }
    #[doc = "HSEM secure masked interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smisr(pub u32);
    impl Smisr {
        #[doc = "Secure masked interrupt semaphore n status bit after enable (mask)."]
        #[inline(always)]
        pub const fn misf(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Secure masked interrupt semaphore n status bit after enable (mask)."]
        #[inline(always)]
        pub fn set_misf(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Smisr {
        #[inline(always)]
        fn default() -> Smisr {
            Smisr(0)
        }
    }
    impl core::fmt::Debug for Smisr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Smisr")
                .field("misf[0]", &self.misf(0usize))
                .field("misf[1]", &self.misf(1usize))
                .field("misf[2]", &self.misf(2usize))
                .field("misf[3]", &self.misf(3usize))
                .field("misf[4]", &self.misf(4usize))
                .field("misf[5]", &self.misf(5usize))
                .field("misf[6]", &self.misf(6usize))
                .field("misf[7]", &self.misf(7usize))
                .field("misf[8]", &self.misf(8usize))
                .field("misf[9]", &self.misf(9usize))
                .field("misf[10]", &self.misf(10usize))
                .field("misf[11]", &self.misf(11usize))
                .field("misf[12]", &self.misf(12usize))
                .field("misf[13]", &self.misf(13usize))
                .field("misf[14]", &self.misf(14usize))
                .field("misf[15]", &self.misf(15usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Smisr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Smisr {{ misf[0]: {=bool:?}, misf[1]: {=bool:?}, misf[2]: {=bool:?}, misf[3]: {=bool:?}, misf[4]: {=bool:?}, misf[5]: {=bool:?}, misf[6]: {=bool:?}, misf[7]: {=bool:?}, misf[8]: {=bool:?}, misf[9]: {=bool:?}, misf[10]: {=bool:?}, misf[11]: {=bool:?}, misf[12]: {=bool:?}, misf[13]: {=bool:?}, misf[14]: {=bool:?}, misf[15]: {=bool:?} }}" , self . misf (0usize) , self . misf (1usize) , self . misf (2usize) , self . misf (3usize) , self . misf (4usize) , self . misf (5usize) , self . misf (6usize) , self . misf (7usize) , self . misf (8usize) , self . misf (9usize) , self . misf (10usize) , self . misf (11usize) , self . misf (12usize) , self . misf (13usize) , self . misf (14usize) , self . misf (15usize))
        }
    }
}
