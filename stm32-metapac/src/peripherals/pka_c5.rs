#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "PKA register blank and RAM."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pka {
    ptr: *mut u8,
}
unsafe impl Send for Pka {}
unsafe impl Sync for Pka {}
impl Pka {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "PKA control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "PKA status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "PKA clear flag register."]
    #[inline(always)]
    pub const fn clrfr(self) -> crate::common::Reg<regs::Clrfr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "PKA internal memory."]
    #[inline(always)]
    pub const fn ram(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 1334usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "PKA clear flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Clrfr(pub u32);
    impl Clrfr {
        #[doc = "Clear PKA end of operation flag."]
        #[must_use]
        #[inline(always)]
        pub const fn procendfc(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Clear PKA end of operation flag."]
        #[inline(always)]
        pub const fn set_procendfc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Clear PKA RAM error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ramerrfc(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Clear PKA RAM error flag."]
        #[inline(always)]
        pub const fn set_ramerrfc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Clear address error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn addrerrfc(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Clear address error flag."]
        #[inline(always)]
        pub const fn set_addrerrfc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Clear operation error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn operrfc(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Clear operation error flag."]
        #[inline(always)]
        pub const fn set_operrfc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Clear chaining mode flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cmfc(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Clear chaining mode flag."]
        #[inline(always)]
        pub const fn set_cmfc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Clrfr {
        #[inline(always)]
        fn default() -> Clrfr {
            Clrfr(0)
        }
    }
    impl core::fmt::Debug for Clrfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Clrfr")
                .field("procendfc", &self.procendfc())
                .field("ramerrfc", &self.ramerrfc())
                .field("addrerrfc", &self.addrerrfc())
                .field("operrfc", &self.operrfc())
                .field("cmfc", &self.cmfc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Clrfr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Clrfr {{ procendfc: {=bool:?}, ramerrfc: {=bool:?}, addrerrfc: {=bool:?}, operrfc: {=bool:?}, cmfc: {=bool:?} }}",
                self.procendfc(),
                self.ramerrfc(),
                self.addrerrfc(),
                self.operrfc(),
                self.cmfc()
            )
        }
    }
    #[doc = "PKA control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "PKA enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PKA enable."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "start the operation."]
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "start the operation."]
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PKA operation code."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "PKA operation code."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "End of operation interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn procendie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "End of operation interrupt enable."]
        #[inline(always)]
        pub const fn set_procendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "RAM error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ramerrie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "RAM error interrupt enable."]
        #[inline(always)]
        pub const fn set_ramerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Address error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn addrerrie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Address error interrupt enable."]
        #[inline(always)]
        pub const fn set_addrerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Operation error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn operrie(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Operation error interrupt enable."]
        #[inline(always)]
        pub const fn set_operrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Chaining mode flags interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmfie(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Chaining mode flags interrupt enable."]
        #[inline(always)]
        pub const fn set_cmfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
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
                .field("en", &self.en())
                .field("start", &self.start())
                .field("mode", &self.mode())
                .field("procendie", &self.procendie())
                .field("ramerrie", &self.ramerrie())
                .field("addrerrie", &self.addrerrie())
                .field("operrie", &self.operrie())
                .field("cmfie", &self.cmfie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ en: {=bool:?}, start: {=bool:?}, mode: {=u8:?}, procendie: {=bool:?}, ramerrie: {=bool:?}, addrerrie: {=bool:?}, operrie: {=bool:?}, cmfie: {=bool:?} }}",
                self.en(),
                self.start(),
                self.mode(),
                self.procendie(),
                self.ramerrie(),
                self.addrerrie(),
                self.operrie(),
                self.cmfie()
            )
        }
    }
    #[doc = "PKA status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "PKA initialization OK."]
        #[must_use]
        #[inline(always)]
        pub const fn initok(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PKA initialization OK."]
        #[inline(always)]
        pub const fn set_initok(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Limited mode flag."]
        #[must_use]
        #[inline(always)]
        pub const fn lmf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Limited mode flag."]
        #[inline(always)]
        pub const fn set_lmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Coupling and chaining mode enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ccen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Coupling and chaining mode enable."]
        #[inline(always)]
        pub const fn set_ccen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "RNG OK flag."]
        #[must_use]
        #[inline(always)]
        pub const fn rngokf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "RNG OK flag."]
        #[inline(always)]
        pub const fn set_rngokf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Data OK flag."]
        #[must_use]
        #[inline(always)]
        pub const fn dataokf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Data OK flag."]
        #[inline(always)]
        pub const fn set_dataokf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Increment error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn increrrf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Increment error flag."]
        #[inline(always)]
        pub const fn set_increrrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Data 0-ed error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn datazf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Data 0-ed error flag."]
        #[inline(always)]
        pub const fn set_datazf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Trailing 0s error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn trzerrf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Trailing 0s error flag."]
        #[inline(always)]
        pub const fn set_trzerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Mode error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn mderrf(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Mode error flag."]
        #[inline(always)]
        pub const fn set_mderrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "RNG error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn rngerrf(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "RNG error flag."]
        #[inline(always)]
        pub const fn set_rngerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Busy flag."]
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Busy flag."]
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "PKA end of operation flag."]
        #[must_use]
        #[inline(always)]
        pub const fn procendf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "PKA end of operation flag."]
        #[inline(always)]
        pub const fn set_procendf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "PKA RAM error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ramerrf(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "PKA RAM error flag."]
        #[inline(always)]
        pub const fn set_ramerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Address error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn addrerrf(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Address error flag."]
        #[inline(always)]
        pub const fn set_addrerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Operation error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn operrf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Operation error flag."]
        #[inline(always)]
        pub const fn set_operrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Chaining mode flags."]
        #[must_use]
        #[inline(always)]
        pub const fn cmf(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Chaining mode flags."]
        #[inline(always)]
        pub const fn set_cmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
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
                .field("initok", &self.initok())
                .field("lmf", &self.lmf())
                .field("ccen", &self.ccen())
                .field("rngokf", &self.rngokf())
                .field("dataokf", &self.dataokf())
                .field("increrrf", &self.increrrf())
                .field("datazf", &self.datazf())
                .field("trzerrf", &self.trzerrf())
                .field("mderrf", &self.mderrf())
                .field("rngerrf", &self.rngerrf())
                .field("busy", &self.busy())
                .field("procendf", &self.procendf())
                .field("ramerrf", &self.ramerrf())
                .field("addrerrf", &self.addrerrf())
                .field("operrf", &self.operrf())
                .field("cmf", &self.cmf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ initok: {=bool:?}, lmf: {=bool:?}, ccen: {=bool:?}, rngokf: {=bool:?}, dataokf: {=bool:?}, increrrf: {=bool:?}, datazf: {=bool:?}, trzerrf: {=bool:?}, mderrf: {=bool:?}, rngerrf: {=bool:?}, busy: {=bool:?}, procendf: {=bool:?}, ramerrf: {=bool:?}, addrerrf: {=bool:?}, operrf: {=bool:?}, cmf: {=bool:?} }}",
                self.initok(),
                self.lmf(),
                self.ccen(),
                self.rngokf(),
                self.dataokf(),
                self.increrrf(),
                self.datazf(),
                self.trzerrf(),
                self.mderrf(),
                self.rngerrf(),
                self.busy(),
                self.procendf(),
                self.ramerrf(),
                self.addrerrf(),
                self.operrf(),
                self.cmf()
            )
        }
    }
}
