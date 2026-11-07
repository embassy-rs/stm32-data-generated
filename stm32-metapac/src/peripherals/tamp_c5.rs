#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "TAMP register block."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tamp {
    ptr: *mut u8,
}
unsafe impl Send for Tamp {}
unsafe impl Sync for Tamp {}
impl Tamp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "TAMP control register 1."]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "TAMP control register 2."]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "TAMP control register 3."]
    #[inline(always)]
    pub const fn cr3(self) -> crate::common::Reg<regs::Cr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "TAMP filter control register."]
    #[inline(always)]
    pub const fn fltcr(self) -> crate::common::Reg<regs::Fltcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "TAMP configuration register."]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "TAMP privilege configuration register."]
    #[inline(always)]
    pub const fn privcfgr(self) -> crate::common::Reg<regs::Privcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "TAMP interrupt enable register."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "TAMP status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "TAMP masked interrupt status register."]
    #[inline(always)]
    pub const fn misr(self) -> crate::common::Reg<regs::Misr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "TAMP status clear register."]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::Scr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "TAMP option register."]
    #[inline(always)]
    pub const fn or(self) -> crate::common::Reg<regs::Or, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "TAMP backup 0 register."]
    #[inline(always)]
    pub const fn bkpr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "TAMP configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "Backup registers read/write protection offset."]
        #[must_use]
        #[inline(always)]
        pub const fn bkprw(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Backup registers read/write protection offset."]
        #[inline(always)]
        pub const fn set_bkprw(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Backup registers write protection offset."]
        #[must_use]
        #[inline(always)]
        pub const fn bkpw(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Backup registers write protection offset."]
        #[inline(always)]
        pub const fn set_bkpw(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Cfgr {
        #[inline(always)]
        fn default() -> Cfgr {
            Cfgr(0)
        }
    }
    impl core::fmt::Debug for Cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr")
                .field("bkprw", &self.bkprw())
                .field("bkpw", &self.bkpw())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cfgr {{ bkprw: {=u8:?}, bkpw: {=u8:?} }}", self.bkprw(), self.bkpw())
        }
    }
    #[doc = "TAMP control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "Tamper detection on TAMP_IN1 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tampe(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper detection on TAMP_IN1 enable."]
        #[inline(always)]
        pub const fn set_tampe(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Internal tamper 3 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn itamp3e(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 3 enable."]
        #[inline(always)]
        pub const fn set_itamp3e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Internal tamper 4 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn itamp4e(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 4 enable."]
        #[inline(always)]
        pub const fn set_itamp4e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Internal tamper 5 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn itamp5e(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 5 enable."]
        #[inline(always)]
        pub const fn set_itamp5e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Internal tamper 6 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn itamp6e(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 6 enable."]
        #[inline(always)]
        pub const fn set_itamp6e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Internal tamper 9 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn itamp9e(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 9 enable."]
        #[inline(always)]
        pub const fn set_itamp9e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Internal tamper 11 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn itamp11e(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 11 enable."]
        #[inline(always)]
        pub const fn set_itamp11e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Cr1 {
        #[inline(always)]
        fn default() -> Cr1 {
            Cr1(0)
        }
    }
    impl core::fmt::Debug for Cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr1")
                .field("tampe[0]", &self.tampe(0usize))
                .field("tampe[1]", &self.tampe(1usize))
                .field("tampe[2]", &self.tampe(2usize))
                .field("itamp3e", &self.itamp3e())
                .field("itamp4e", &self.itamp4e())
                .field("itamp5e", &self.itamp5e())
                .field("itamp6e", &self.itamp6e())
                .field("itamp9e", &self.itamp9e())
                .field("itamp11e", &self.itamp11e())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr1 {{ tampe[0]: {=bool:?}, tampe[1]: {=bool:?}, tampe[2]: {=bool:?}, itamp3e: {=bool:?}, itamp4e: {=bool:?}, itamp5e: {=bool:?}, itamp6e: {=bool:?}, itamp9e: {=bool:?}, itamp11e: {=bool:?} }}",
                self.tampe(0usize),
                self.tampe(1usize),
                self.tampe(2usize),
                self.itamp3e(),
                self.itamp4e(),
                self.itamp5e(),
                self.itamp6e(),
                self.itamp9e(),
                self.itamp11e()
            )
        }
    }
    #[doc = "TAMP control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "Tamper 1 potential mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tamppom(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper 1 potential mode."]
        #[inline(always)]
        pub const fn set_tamppom(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Tamper 1 mask."]
        #[must_use]
        #[inline(always)]
        pub const fn tampmsk(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper 1 mask."]
        #[inline(always)]
        pub const fn set_tampmsk(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Backup registers and device secretsless thansup>(1)less than/sup> access blocked."]
        #[must_use]
        #[inline(always)]
        pub const fn bkblock(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Backup registers and device secretsless thansup>(1)less than/sup> access blocked."]
        #[inline(always)]
        pub const fn set_bkblock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Backup registers and device secretsless thansup>(1)less than/sup> erase."]
        #[must_use]
        #[inline(always)]
        pub const fn bkerase(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Backup registers and device secretsless thansup>(1)less than/sup> erase."]
        #[inline(always)]
        pub const fn set_bkerase(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Active level for tamper 1 input."]
        #[must_use]
        #[inline(always)]
        pub const fn tamptrg(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 24usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Active level for tamper 1 input."]
        #[inline(always)]
        pub const fn set_tamptrg(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 24usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Cr2 {
        #[inline(always)]
        fn default() -> Cr2 {
            Cr2(0)
        }
    }
    impl core::fmt::Debug for Cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr2")
                .field("tamppom[0]", &self.tamppom(0usize))
                .field("tamppom[1]", &self.tamppom(1usize))
                .field("tamppom[2]", &self.tamppom(2usize))
                .field("tampmsk[0]", &self.tampmsk(0usize))
                .field("tampmsk[1]", &self.tampmsk(1usize))
                .field("tampmsk[2]", &self.tampmsk(2usize))
                .field("bkblock", &self.bkblock())
                .field("bkerase", &self.bkerase())
                .field("tamptrg[0]", &self.tamptrg(0usize))
                .field("tamptrg[1]", &self.tamptrg(1usize))
                .field("tamptrg[2]", &self.tamptrg(2usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr2 {{ tamppom[0]: {=bool:?}, tamppom[1]: {=bool:?}, tamppom[2]: {=bool:?}, tampmsk[0]: {=bool:?}, tampmsk[1]: {=bool:?}, tampmsk[2]: {=bool:?}, bkblock: {=bool:?}, bkerase: {=bool:?}, tamptrg[0]: {=bool:?}, tamptrg[1]: {=bool:?}, tamptrg[2]: {=bool:?} }}",
                self.tamppom(0usize),
                self.tamppom(1usize),
                self.tamppom(2usize),
                self.tampmsk(0usize),
                self.tampmsk(1usize),
                self.tampmsk(2usize),
                self.bkblock(),
                self.bkerase(),
                self.tamptrg(0usize),
                self.tamptrg(1usize),
                self.tamptrg(2usize)
            )
        }
    }
    #[doc = "TAMP control register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr3(pub u32);
    impl Cr3 {
        #[doc = "Internal tamper 3 potential mode."]
        #[must_use]
        #[inline(always)]
        pub const fn itamp3pom(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 3 potential mode."]
        #[inline(always)]
        pub const fn set_itamp3pom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Internal tamper 4 potential mode."]
        #[must_use]
        #[inline(always)]
        pub const fn itamp4pom(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 4 potential mode."]
        #[inline(always)]
        pub const fn set_itamp4pom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Internal tamper 5 potential mode."]
        #[must_use]
        #[inline(always)]
        pub const fn itamp5pom(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 5 potential mode."]
        #[inline(always)]
        pub const fn set_itamp5pom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Internal tamper 6 potential mode."]
        #[must_use]
        #[inline(always)]
        pub const fn itamp6pom(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 6 potential mode."]
        #[inline(always)]
        pub const fn set_itamp6pom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Internal tamper 9 potential mode."]
        #[must_use]
        #[inline(always)]
        pub const fn itamp9pom(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 9 potential mode."]
        #[inline(always)]
        pub const fn set_itamp9pom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Internal tamper 11 potential mode."]
        #[must_use]
        #[inline(always)]
        pub const fn itamp11pom(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 11 potential mode."]
        #[inline(always)]
        pub const fn set_itamp11pom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Cr3 {
        #[inline(always)]
        fn default() -> Cr3 {
            Cr3(0)
        }
    }
    impl core::fmt::Debug for Cr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr3")
                .field("itamp3pom", &self.itamp3pom())
                .field("itamp4pom", &self.itamp4pom())
                .field("itamp5pom", &self.itamp5pom())
                .field("itamp6pom", &self.itamp6pom())
                .field("itamp9pom", &self.itamp9pom())
                .field("itamp11pom", &self.itamp11pom())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr3 {{ itamp3pom: {=bool:?}, itamp4pom: {=bool:?}, itamp5pom: {=bool:?}, itamp6pom: {=bool:?}, itamp9pom: {=bool:?}, itamp11pom: {=bool:?} }}",
                self.itamp3pom(),
                self.itamp4pom(),
                self.itamp5pom(),
                self.itamp6pom(),
                self.itamp9pom(),
                self.itamp11pom()
            )
        }
    }
    #[doc = "TAMP filter control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fltcr(pub u32);
    impl Fltcr {
        #[doc = "Tamper sampling frequency."]
        #[must_use]
        #[inline(always)]
        pub const fn tampfreq(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Tamper sampling frequency."]
        #[inline(always)]
        pub const fn set_tampfreq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "TAMP_INx filter count."]
        #[must_use]
        #[inline(always)]
        pub const fn tampflt(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "TAMP_INx filter count."]
        #[inline(always)]
        pub const fn set_tampflt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "TAMP_INx precharge duration."]
        #[must_use]
        #[inline(always)]
        pub const fn tampprch(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "TAMP_INx precharge duration."]
        #[inline(always)]
        pub const fn set_tampprch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "TAMP_INx pull-up disable."]
        #[must_use]
        #[inline(always)]
        pub const fn tamppudis(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TAMP_INx pull-up disable."]
        #[inline(always)]
        pub const fn set_tamppudis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Fltcr {
        #[inline(always)]
        fn default() -> Fltcr {
            Fltcr(0)
        }
    }
    impl core::fmt::Debug for Fltcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fltcr")
                .field("tampfreq", &self.tampfreq())
                .field("tampflt", &self.tampflt())
                .field("tampprch", &self.tampprch())
                .field("tamppudis", &self.tamppudis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fltcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Fltcr {{ tampfreq: {=u8:?}, tampflt: {=u8:?}, tampprch: {=u8:?}, tamppudis: {=bool:?} }}",
                self.tampfreq(),
                self.tampflt(),
                self.tampprch(),
                self.tamppudis()
            )
        }
    }
    #[doc = "TAMP interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "Tamper 1 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tampie(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper 1 interrupt enable."]
        #[inline(always)]
        pub const fn set_tampie(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Internal tamper 3 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn itamp3ie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 3 interrupt enable."]
        #[inline(always)]
        pub const fn set_itamp3ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Internal tamper 4 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn itamp4ie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 4 interrupt enable."]
        #[inline(always)]
        pub const fn set_itamp4ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Internal tamper 5 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn itamp5ie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 5 interrupt enable."]
        #[inline(always)]
        pub const fn set_itamp5ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Internal tamper 6 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn itamp6ie(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 6 interrupt enable."]
        #[inline(always)]
        pub const fn set_itamp6ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Internal tamper 9 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn itamp9ie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 9 interrupt enable."]
        #[inline(always)]
        pub const fn set_itamp9ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Internal tamper 11 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn itamp11ie(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 11 interrupt enable."]
        #[inline(always)]
        pub const fn set_itamp11ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
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
                .field("tampie[0]", &self.tampie(0usize))
                .field("tampie[1]", &self.tampie(1usize))
                .field("tampie[2]", &self.tampie(2usize))
                .field("itamp3ie", &self.itamp3ie())
                .field("itamp4ie", &self.itamp4ie())
                .field("itamp5ie", &self.itamp5ie())
                .field("itamp6ie", &self.itamp6ie())
                .field("itamp9ie", &self.itamp9ie())
                .field("itamp11ie", &self.itamp11ie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ier {{ tampie[0]: {=bool:?}, tampie[1]: {=bool:?}, tampie[2]: {=bool:?}, itamp3ie: {=bool:?}, itamp4ie: {=bool:?}, itamp5ie: {=bool:?}, itamp6ie: {=bool:?}, itamp9ie: {=bool:?}, itamp11ie: {=bool:?} }}",
                self.tampie(0usize),
                self.tampie(1usize),
                self.tampie(2usize),
                self.itamp3ie(),
                self.itamp4ie(),
                self.itamp5ie(),
                self.itamp6ie(),
                self.itamp9ie(),
                self.itamp11ie()
            )
        }
    }
    #[doc = "TAMP masked interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Misr(pub u32);
    impl Misr {
        #[doc = "TAMP1 interrupt masked flag."]
        #[must_use]
        #[inline(always)]
        pub const fn tampmf(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "TAMP1 interrupt masked flag."]
        #[inline(always)]
        pub const fn set_tampmf(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Internal tamper 3 interrupt masked flag."]
        #[must_use]
        #[inline(always)]
        pub const fn itamp3mf(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 3 interrupt masked flag."]
        #[inline(always)]
        pub const fn set_itamp3mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Internal tamper 4 interrupt masked flag."]
        #[must_use]
        #[inline(always)]
        pub const fn itamp4mf(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 4 interrupt masked flag."]
        #[inline(always)]
        pub const fn set_itamp4mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Internal tamper 5 interrupt masked flag."]
        #[must_use]
        #[inline(always)]
        pub const fn itamp5mf(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 5 interrupt masked flag."]
        #[inline(always)]
        pub const fn set_itamp5mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Internal tamper 6 interrupt masked flag."]
        #[must_use]
        #[inline(always)]
        pub const fn itamp6mf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 6 interrupt masked flag."]
        #[inline(always)]
        pub const fn set_itamp6mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "internal tamper 9 interrupt masked flag."]
        #[must_use]
        #[inline(always)]
        pub const fn itamp9mf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "internal tamper 9 interrupt masked flag."]
        #[inline(always)]
        pub const fn set_itamp9mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "internal tamper 11 interrupt masked flag."]
        #[must_use]
        #[inline(always)]
        pub const fn itamp11mf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "internal tamper 11 interrupt masked flag."]
        #[inline(always)]
        pub const fn set_itamp11mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
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
                .field("tampmf[0]", &self.tampmf(0usize))
                .field("tampmf[1]", &self.tampmf(1usize))
                .field("tampmf[2]", &self.tampmf(2usize))
                .field("itamp3mf", &self.itamp3mf())
                .field("itamp4mf", &self.itamp4mf())
                .field("itamp5mf", &self.itamp5mf())
                .field("itamp6mf", &self.itamp6mf())
                .field("itamp9mf", &self.itamp9mf())
                .field("itamp11mf", &self.itamp11mf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Misr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Misr {{ tampmf[0]: {=bool:?}, tampmf[1]: {=bool:?}, tampmf[2]: {=bool:?}, itamp3mf: {=bool:?}, itamp4mf: {=bool:?}, itamp5mf: {=bool:?}, itamp6mf: {=bool:?}, itamp9mf: {=bool:?}, itamp11mf: {=bool:?} }}",
                self.tampmf(0usize),
                self.tampmf(1usize),
                self.tampmf(2usize),
                self.itamp3mf(),
                self.itamp4mf(),
                self.itamp5mf(),
                self.itamp6mf(),
                self.itamp9mf(),
                self.itamp11mf()
            )
        }
    }
    #[doc = "TAMP option register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Or(pub u32);
    impl Or {
        #[doc = "TAMP_IN2 pin select."]
        #[must_use]
        #[inline(always)]
        pub const fn in2_rmp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TAMP_IN2 pin select."]
        #[inline(always)]
        pub const fn set_in2_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TAMP_IN3 pin select."]
        #[must_use]
        #[inline(always)]
        pub const fn in3_rmp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TAMP_IN3 pin select."]
        #[inline(always)]
        pub const fn set_in3_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Or {
        #[inline(always)]
        fn default() -> Or {
            Or(0)
        }
    }
    impl core::fmt::Debug for Or {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Or")
                .field("in2_rmp", &self.in2_rmp())
                .field("in3_rmp", &self.in3_rmp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Or {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Or {{ in2_rmp: {=bool:?}, in3_rmp: {=bool:?} }}",
                self.in2_rmp(),
                self.in3_rmp()
            )
        }
    }
    #[doc = "TAMP privilege configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcfgr(pub u32);
    impl Privcfgr {
        #[doc = "Backup registers zone 1 privilege protection."]
        #[must_use]
        #[inline(always)]
        pub const fn bkprwpriv(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Backup registers zone 1 privilege protection."]
        #[inline(always)]
        pub const fn set_bkprwpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Backup registers zone 2 privilege protection."]
        #[must_use]
        #[inline(always)]
        pub const fn bkpwpriv(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Backup registers zone 2 privilege protection."]
        #[inline(always)]
        pub const fn set_bkpwpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Tamper privilege protection (excluding backup registers)."]
        #[must_use]
        #[inline(always)]
        pub const fn tamppriv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Tamper privilege protection (excluding backup registers)."]
        #[inline(always)]
        pub const fn set_tamppriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("bkprwpriv", &self.bkprwpriv())
                .field("bkpwpriv", &self.bkpwpriv())
                .field("tamppriv", &self.tamppriv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Privcfgr {{ bkprwpriv: {=bool:?}, bkpwpriv: {=bool:?}, tamppriv: {=bool:?} }}",
                self.bkprwpriv(),
                self.bkpwpriv(),
                self.tamppriv()
            )
        }
    }
    #[doc = "TAMP status clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scr(pub u32);
    impl Scr {
        #[doc = "Clear TAMP1 detection flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ctampf(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Clear TAMP1 detection flag."]
        #[inline(always)]
        pub const fn set_ctampf(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Clear ITAMP3 detection flag."]
        #[must_use]
        #[inline(always)]
        pub const fn citamp3f(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ITAMP3 detection flag."]
        #[inline(always)]
        pub const fn set_citamp3f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Clear ITAMP4 detection flag."]
        #[must_use]
        #[inline(always)]
        pub const fn citamp4f(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ITAMP4 detection flag."]
        #[inline(always)]
        pub const fn set_citamp4f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Clear ITAMP5 detection flag."]
        #[must_use]
        #[inline(always)]
        pub const fn citamp5f(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ITAMP5 detection flag."]
        #[inline(always)]
        pub const fn set_citamp5f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Clear ITAMP6 detection flag."]
        #[must_use]
        #[inline(always)]
        pub const fn citamp6f(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ITAMP6 detection flag."]
        #[inline(always)]
        pub const fn set_citamp6f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Clear ITAMP9 detection flag."]
        #[must_use]
        #[inline(always)]
        pub const fn citamp9f(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ITAMP9 detection flag."]
        #[inline(always)]
        pub const fn set_citamp9f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Clear ITAMP11 detection flag."]
        #[must_use]
        #[inline(always)]
        pub const fn citamp11f(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ITAMP11 detection flag."]
        #[inline(always)]
        pub const fn set_citamp11f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Scr {
        #[inline(always)]
        fn default() -> Scr {
            Scr(0)
        }
    }
    impl core::fmt::Debug for Scr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scr")
                .field("ctampf[0]", &self.ctampf(0usize))
                .field("ctampf[1]", &self.ctampf(1usize))
                .field("ctampf[2]", &self.ctampf(2usize))
                .field("citamp3f", &self.citamp3f())
                .field("citamp4f", &self.citamp4f())
                .field("citamp5f", &self.citamp5f())
                .field("citamp6f", &self.citamp6f())
                .field("citamp9f", &self.citamp9f())
                .field("citamp11f", &self.citamp11f())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Scr {{ ctampf[0]: {=bool:?}, ctampf[1]: {=bool:?}, ctampf[2]: {=bool:?}, citamp3f: {=bool:?}, citamp4f: {=bool:?}, citamp5f: {=bool:?}, citamp6f: {=bool:?}, citamp9f: {=bool:?}, citamp11f: {=bool:?} }}",
                self.ctampf(0usize),
                self.ctampf(1usize),
                self.ctampf(2usize),
                self.citamp3f(),
                self.citamp4f(),
                self.citamp5f(),
                self.citamp6f(),
                self.citamp9f(),
                self.citamp11f()
            )
        }
    }
    #[doc = "TAMP status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "TAMP1 detection flag."]
        #[must_use]
        #[inline(always)]
        pub const fn tampf(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "TAMP1 detection flag."]
        #[inline(always)]
        pub const fn set_tampf(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Internal tamper 3 flag."]
        #[must_use]
        #[inline(always)]
        pub const fn itamp3f(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 3 flag."]
        #[inline(always)]
        pub const fn set_itamp3f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Internal tamper 4 flag."]
        #[must_use]
        #[inline(always)]
        pub const fn itamp4f(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 4 flag."]
        #[inline(always)]
        pub const fn set_itamp4f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Internal tamper 5 flag."]
        #[must_use]
        #[inline(always)]
        pub const fn itamp5f(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 5 flag."]
        #[inline(always)]
        pub const fn set_itamp5f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Internal tamper 6 flag."]
        #[must_use]
        #[inline(always)]
        pub const fn itamp6f(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 6 flag."]
        #[inline(always)]
        pub const fn set_itamp6f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Internal tamper 9 flag."]
        #[must_use]
        #[inline(always)]
        pub const fn itamp9f(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 9 flag."]
        #[inline(always)]
        pub const fn set_itamp9f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Internal tamper 11 flag."]
        #[must_use]
        #[inline(always)]
        pub const fn itamp11f(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 11 flag."]
        #[inline(always)]
        pub const fn set_itamp11f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
    impl core::fmt::Debug for Sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr")
                .field("tampf[0]", &self.tampf(0usize))
                .field("tampf[1]", &self.tampf(1usize))
                .field("tampf[2]", &self.tampf(2usize))
                .field("itamp3f", &self.itamp3f())
                .field("itamp4f", &self.itamp4f())
                .field("itamp5f", &self.itamp5f())
                .field("itamp6f", &self.itamp6f())
                .field("itamp9f", &self.itamp9f())
                .field("itamp11f", &self.itamp11f())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ tampf[0]: {=bool:?}, tampf[1]: {=bool:?}, tampf[2]: {=bool:?}, itamp3f: {=bool:?}, itamp4f: {=bool:?}, itamp5f: {=bool:?}, itamp6f: {=bool:?}, itamp9f: {=bool:?}, itamp11f: {=bool:?} }}",
                self.tampf(0usize),
                self.tampf(1usize),
                self.tampf(2usize),
                self.itamp3f(),
                self.itamp4f(),
                self.itamp5f(),
                self.itamp6f(),
                self.itamp9f(),
                self.itamp11f()
            )
        }
    }
}
