#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Comparator with one channel."]
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
    #[doc = "Comparator status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::SrSingle, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Comparator interrupt clear flag register."]
    #[inline(always)]
    pub const fn icfr(self) -> crate::common::Reg<regs::IcfrSingle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Comparator configuration register."]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::CfgrSingle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
#[doc = "Comparator channel."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CompChannel {
    ptr: *mut u8,
}
unsafe impl Send for CompChannel {}
unsafe impl Sync for CompChannel {}
impl CompChannel {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Comparator configuration register."]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::CfgrDual, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
}
#[doc = "Common register block for a pair of comparators."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CompCommon {
    ptr: *mut u8,
}
unsafe impl Send for CompCommon {}
unsafe impl Sync for CompCommon {}
impl CompCommon {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Comparator status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::SrDual, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Comparator interrupt clear flag register."]
    #[inline(always)]
    pub const fn icfr(self) -> crate::common::Reg<regs::IcfrDual, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Comparator configuration register."]
    #[inline(always)]
    pub const fn cfgr(self, n: usize) -> crate::common::Reg<regs::CfgrDual, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "Comparator configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CfgrDual(pub u32);
    impl CfgrDual {
        #[doc = "Comparator channel enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator channel enable."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Scaler bridge enable."]
        #[must_use]
        #[inline(always)]
        pub const fn brgen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Scaler bridge enable."]
        #[inline(always)]
        pub const fn set_brgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Voltage scaler enable."]
        #[must_use]
        #[inline(always)]
        pub const fn scalen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Voltage scaler enable."]
        #[inline(always)]
        pub const fn set_scalen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Comparator channel polarity selection."]
        #[must_use]
        #[inline(always)]
        pub const fn polarity(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator channel polarity selection."]
        #[inline(always)]
        pub const fn set_polarity(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Comparator non-inverting input selector for window mode."]
        #[must_use]
        #[inline(always)]
        pub const fn winmode(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator non-inverting input selector for window mode."]
        #[inline(always)]
        pub const fn set_winmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Comparator channel interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn iten(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator channel interrupt enable."]
        #[inline(always)]
        pub const fn set_iten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Comparator channel hysteresis selection."]
        #[must_use]
        #[inline(always)]
        pub const fn hyst(&self) -> super::vals::Hyst {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Hyst::from_bits(val as u8)
        }
        #[doc = "Comparator channel hysteresis selection."]
        #[inline(always)]
        pub const fn set_hyst(&mut self, val: super::vals::Hyst) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "Power mode of the comparator channel."]
        #[must_use]
        #[inline(always)]
        pub const fn pwrmode(&self) -> super::vals::Pwrmode {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Pwrmode::from_bits(val as u8)
        }
        #[doc = "Power mode of the comparator channel."]
        #[inline(always)]
        pub const fn set_pwrmode(&mut self, val: super::vals::Pwrmode) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "Comparator output selector for window mode."]
        #[must_use]
        #[inline(always)]
        pub const fn winout(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator output selector for window mode."]
        #[inline(always)]
        pub const fn set_winout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Comparator channel inverting input selection."]
        #[must_use]
        #[inline(always)]
        pub const fn inmsel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Comparator channel inverting input selection."]
        #[inline(always)]
        pub const fn set_inmsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Comparator channel non-inverting input selection."]
        #[must_use]
        #[inline(always)]
        pub const fn inpsel(&self) -> u8 {
            let mut val = 0;
            val += (((self.0 >> 20usize) & 0x01) << 0usize);
            val += (((self.0 >> 22usize) & 0x01) << 1usize);
            val as u8
        }
        #[doc = "Comparator channel non-inverting input selection."]
        #[inline(always)]
        pub const fn set_inpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32 >> 0usize) & 0x01) << 20usize);
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32 >> 1usize) & 0x01) << 22usize);
        }
        #[doc = "Comparator channel blanking source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn blanking(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Comparator channel blanking source selection."]
        #[inline(always)]
        pub const fn set_blanking(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Comparator configuration register lock."]
        #[must_use]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator configuration register lock."]
        #[inline(always)]
        pub const fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CfgrDual {
        #[inline(always)]
        fn default() -> CfgrDual {
            CfgrDual(0)
        }
    }
    impl core::fmt::Debug for CfgrDual {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CfgrDual")
                .field("en", &self.en())
                .field("brgen", &self.brgen())
                .field("scalen", &self.scalen())
                .field("polarity", &self.polarity())
                .field("winmode", &self.winmode())
                .field("iten", &self.iten())
                .field("hyst", &self.hyst())
                .field("pwrmode", &self.pwrmode())
                .field("winout", &self.winout())
                .field("inmsel", &self.inmsel())
                .field("inpsel", &self.inpsel())
                .field("blanking", &self.blanking())
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CfgrDual {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CfgrDual {{ en: {=bool:?}, brgen: {=bool:?}, scalen: {=bool:?}, polarity: {=bool:?}, winmode: {=bool:?}, iten: {=bool:?}, hyst: {:?}, pwrmode: {:?}, winout: {=bool:?}, inmsel: {=u8:?}, inpsel: {=u8:?}, blanking: {=u8:?}, lock: {=bool:?} }}",
                self.en(),
                self.brgen(),
                self.scalen(),
                self.polarity(),
                self.winmode(),
                self.iten(),
                self.hyst(),
                self.pwrmode(),
                self.winout(),
                self.inmsel(),
                self.inpsel(),
                self.blanking(),
                self.lock()
            )
        }
    }
    #[doc = "Comparator configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CfgrSingle(pub u32);
    impl CfgrSingle {
        #[doc = "Comparator channel enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator channel enable."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Scaler bridge enable."]
        #[must_use]
        #[inline(always)]
        pub const fn brgen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Scaler bridge enable."]
        #[inline(always)]
        pub const fn set_brgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Voltage scaler enable."]
        #[must_use]
        #[inline(always)]
        pub const fn scalen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Voltage scaler enable."]
        #[inline(always)]
        pub const fn set_scalen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Comparator channel polarity selection."]
        #[must_use]
        #[inline(always)]
        pub const fn polarity(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator channel polarity selection."]
        #[inline(always)]
        pub const fn set_polarity(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Comparator channel interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn iten(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator channel interrupt enable."]
        #[inline(always)]
        pub const fn set_iten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Comparator channel hysteresis selection."]
        #[must_use]
        #[inline(always)]
        pub const fn hyst(&self) -> super::vals::Hyst {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Hyst::from_bits(val as u8)
        }
        #[doc = "Comparator channel hysteresis selection."]
        #[inline(always)]
        pub const fn set_hyst(&mut self, val: super::vals::Hyst) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "Power mode of the comparator channel."]
        #[must_use]
        #[inline(always)]
        pub const fn pwrmode(&self) -> super::vals::Pwrmode {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Pwrmode::from_bits(val as u8)
        }
        #[doc = "Power mode of the comparator channel."]
        #[inline(always)]
        pub const fn set_pwrmode(&mut self, val: super::vals::Pwrmode) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "Comparator channel inverting input selection."]
        #[must_use]
        #[inline(always)]
        pub const fn inmsel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Comparator channel inverting input selection."]
        #[inline(always)]
        pub const fn set_inmsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Comparator channel non-inverting input selection."]
        #[must_use]
        #[inline(always)]
        pub const fn inpsel(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Comparator channel non-inverting input selection."]
        #[inline(always)]
        pub const fn set_inpsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "Comparator channel blanking source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn blanking(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Comparator channel blanking source selection."]
        #[inline(always)]
        pub const fn set_blanking(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Comparator configuration register lock."]
        #[must_use]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator configuration register lock."]
        #[inline(always)]
        pub const fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CfgrSingle {
        #[inline(always)]
        fn default() -> CfgrSingle {
            CfgrSingle(0)
        }
    }
    impl core::fmt::Debug for CfgrSingle {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CfgrSingle")
                .field("en", &self.en())
                .field("brgen", &self.brgen())
                .field("scalen", &self.scalen())
                .field("polarity", &self.polarity())
                .field("iten", &self.iten())
                .field("hyst", &self.hyst())
                .field("pwrmode", &self.pwrmode())
                .field("inmsel", &self.inmsel())
                .field("inpsel", &self.inpsel())
                .field("blanking", &self.blanking())
                .field("lock", &self.lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CfgrSingle {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CfgrSingle {{ en: {=bool:?}, brgen: {=bool:?}, scalen: {=bool:?}, polarity: {=bool:?}, iten: {=bool:?}, hyst: {:?}, pwrmode: {:?}, inmsel: {=u8:?}, inpsel: {=u8:?}, blanking: {=u8:?}, lock: {=bool:?} }}",
                self.en(),
                self.brgen(),
                self.scalen(),
                self.polarity(),
                self.iten(),
                self.hyst(),
                self.pwrmode(),
                self.inmsel(),
                self.inpsel(),
                self.blanking(),
                self.lock()
            )
        }
    }
    #[doc = "Comparator interrupt clear flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IcfrDual(pub u32);
    impl IcfrDual {
        #[doc = "Clear comparator channel interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ccif(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Clear comparator channel interrupt flag."]
        #[inline(always)]
        pub const fn set_ccif(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for IcfrDual {
        #[inline(always)]
        fn default() -> IcfrDual {
            IcfrDual(0)
        }
    }
    impl core::fmt::Debug for IcfrDual {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IcfrDual")
                .field("ccif[0]", &self.ccif(0usize))
                .field("ccif[1]", &self.ccif(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IcfrDual {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IcfrDual {{ ccif[0]: {=bool:?}, ccif[1]: {=bool:?} }}",
                self.ccif(0usize),
                self.ccif(1usize)
            )
        }
    }
    #[doc = "Comparator interrupt clear flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IcfrSingle(pub u32);
    impl IcfrSingle {
        #[doc = "Clear comparator channel interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ccif(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Clear comparator channel interrupt flag."]
        #[inline(always)]
        pub const fn set_ccif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for IcfrSingle {
        #[inline(always)]
        fn default() -> IcfrSingle {
            IcfrSingle(0)
        }
    }
    impl core::fmt::Debug for IcfrSingle {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IcfrSingle").field("ccif", &self.ccif()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IcfrSingle {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "IcfrSingle {{ ccif: {=bool:?} }}", self.ccif())
        }
    }
    #[doc = "Comparator status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SrDual(pub u32);
    impl SrDual {
        #[doc = "Comparator channel output status."]
        #[must_use]
        #[inline(always)]
        pub const fn cval(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Comparator channel output status."]
        #[inline(always)]
        pub const fn set_cval(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Comparator channel interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cif(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Comparator channel interrupt flag."]
        #[inline(always)]
        pub const fn set_cif(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for SrDual {
        #[inline(always)]
        fn default() -> SrDual {
            SrDual(0)
        }
    }
    impl core::fmt::Debug for SrDual {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SrDual")
                .field("cval[0]", &self.cval(0usize))
                .field("cval[1]", &self.cval(1usize))
                .field("cif[0]", &self.cif(0usize))
                .field("cif[1]", &self.cif(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SrDual {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SrDual {{ cval[0]: {=bool:?}, cval[1]: {=bool:?}, cif[0]: {=bool:?}, cif[1]: {=bool:?} }}",
                self.cval(0usize),
                self.cval(1usize),
                self.cif(0usize),
                self.cif(1usize)
            )
        }
    }
    #[doc = "Comparator status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SrSingle(pub u32);
    impl SrSingle {
        #[doc = "Comparator channel output status."]
        #[must_use]
        #[inline(always)]
        pub const fn cval(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator channel output status."]
        #[inline(always)]
        pub const fn set_cval(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Comparator channel interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cif(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Comparator channel interrupt flag."]
        #[inline(always)]
        pub const fn set_cif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for SrSingle {
        #[inline(always)]
        fn default() -> SrSingle {
            SrSingle(0)
        }
    }
    impl core::fmt::Debug for SrSingle {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SrSingle")
                .field("cval", &self.cval())
                .field("cif", &self.cif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SrSingle {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SrSingle {{ cval: {=bool:?}, cif: {=bool:?} }}",
                self.cval(),
                self.cif()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hyst {
        None = 0x0,
        Low = 0x01,
        Medium = 0x02,
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
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pwrmode {
        #[doc = "High speed and high power."]
        HighSpeed = 0x0,
        #[doc = "Medium speed and medium power."]
        MediumSpeed = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Pwrmode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pwrmode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pwrmode {
        #[inline(always)]
        fn from(val: u8) -> Pwrmode {
            Pwrmode::from_bits(val)
        }
    }
    impl From<Pwrmode> for u8 {
        #[inline(always)]
        fn from(val: Pwrmode) -> u8 {
            Pwrmode::to_bits(val)
        }
    }
}
