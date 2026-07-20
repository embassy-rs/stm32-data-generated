#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DataRegs {
    ptr: *mut u8,
}
unsafe impl Send for DataRegs {}
unsafe impl Sync for DataRegs {}
impl DataRegs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "I3C transmit data byte register."]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "I3C transmit data word register."]
    #[inline(always)]
    pub const fn dwr(self) -> crate::common::Reg<regs::Dwr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
}
#[doc = "Improved inter-integrated circuit."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I3c {
    ptr: *mut u8,
}
unsafe impl Send for I3c {}
unsafe impl Sync for I3c {}
impl I3c {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "I3C message control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "I3C message control register."]
    #[inline(always)]
    pub const fn cr_alternate(self) -> crate::common::Reg<regs::CrAlternate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "I3C configuration register."]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn rx_data_regs(self) -> DataRegs {
        unsafe { DataRegs::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn tx_data_regs(self) -> DataRegs {
        unsafe { DataRegs::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "I3C IBI payload data register."]
    #[inline(always)]
    pub const fn ibidr(self) -> crate::common::Reg<regs::Ibidr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "I3C target transmit configuration register."]
    #[inline(always)]
    pub const fn tgttdr(self) -> crate::common::Reg<regs::Tgttdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "I3C status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "I3C status error register."]
    #[inline(always)]
    pub const fn ser(self) -> crate::common::Reg<regs::Ser, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "I3C received message register."]
    #[inline(always)]
    pub const fn rmr(self) -> crate::common::Reg<regs::Rmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "I3C event register."]
    #[inline(always)]
    pub const fn evr(self) -> crate::common::Reg<regs::Evr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "I3C interrupt enable register."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "I3C clear event register."]
    #[inline(always)]
    pub const fn cevr(self) -> crate::common::Reg<regs::Cevr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "I3C own device characteristics register."]
    #[inline(always)]
    pub const fn devr0(self) -> crate::common::Reg<regs::Devr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "I3C device 1 characteristics register."]
    #[inline(always)]
    pub const fn devr(self, n: usize) -> crate::common::Reg<regs::Devr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize + n * 4usize) as _) }
    }
    #[doc = "I3C maximum read length register."]
    #[inline(always)]
    pub const fn maxrlr(self) -> crate::common::Reg<regs::Maxrlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "I3C maximum write length register."]
    #[inline(always)]
    pub const fn maxwlr(self) -> crate::common::Reg<regs::Maxwlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "I3C timing register 0."]
    #[inline(always)]
    pub const fn timingr0(self) -> crate::common::Reg<regs::Timingr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "I3C timing register 1."]
    #[inline(always)]
    pub const fn timingr1(self) -> crate::common::Reg<regs::Timingr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "I3C timing register 2."]
    #[inline(always)]
    pub const fn timingr2(self) -> crate::common::Reg<regs::Timingr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "I3C bus characteristics register."]
    #[inline(always)]
    pub const fn bcr(self) -> crate::common::Reg<regs::Bcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "I3C device characteristics register."]
    #[inline(always)]
    pub const fn dcr(self) -> crate::common::Reg<regs::Dcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "I3C get capability register."]
    #[inline(always)]
    pub const fn getcapr(self) -> crate::common::Reg<regs::Getcapr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "I3C controller-role capability register."]
    #[inline(always)]
    pub const fn crcapr(self) -> crate::common::Reg<regs::Crcapr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "I3C get capability register."]
    #[inline(always)]
    pub const fn getmxdsr(self) -> crate::common::Reg<regs::Getmxdsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "I3C extended provisioned ID register."]
    #[inline(always)]
    pub const fn epidr(self) -> crate::common::Reg<regs::Epidr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
}
pub mod regs {
    #[doc = "I3C bus characteristics register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bcr(pub u32);
    impl Bcr {
        #[doc = "max data speed limitation."]
        #[must_use]
        #[inline(always)]
        pub const fn bcr0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "max data speed limitation."]
        #[inline(always)]
        pub const fn set_bcr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "in-band interrupt (IBI) payload."]
        #[must_use]
        #[inline(always)]
        pub const fn bcr2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "in-band interrupt (IBI) payload."]
        #[inline(always)]
        pub const fn set_bcr2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Controller capable."]
        #[must_use]
        #[inline(always)]
        pub const fn bcr6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Controller capable."]
        #[inline(always)]
        pub const fn set_bcr6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Bcr {
        #[inline(always)]
        fn default() -> Bcr {
            Bcr(0)
        }
    }
    impl core::fmt::Debug for Bcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bcr")
                .field("bcr0", &self.bcr0())
                .field("bcr2", &self.bcr2())
                .field("bcr6", &self.bcr6())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bcr {{ bcr0: {=bool:?}, bcr2: {=bool:?}, bcr6: {=bool:?} }}",
                self.bcr0(),
                self.bcr2(),
                self.bcr6()
            )
        }
    }
    #[doc = "I3C clear event register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cevr(pub u32);
    impl Cevr {
        #[doc = "Clear frame complete flag (whatever the I3C acts as controller/target)."]
        #[must_use]
        #[inline(always)]
        pub const fn cfcf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Clear frame complete flag (whatever the I3C acts as controller/target)."]
        #[inline(always)]
        pub const fn set_cfcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Clear target-initiated read end flag (when the I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn crxtgtendf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Clear target-initiated read end flag (when the I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_crxtgtendf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Clear error flag (whatever the I3C acts as controller/target)."]
        #[must_use]
        #[inline(always)]
        pub const fn cerrf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Clear error flag (whatever the I3C acts as controller/target)."]
        #[inline(always)]
        pub const fn set_cerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Clear IBI request flag (when the I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn cibif(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Clear IBI request flag (when the I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_cibif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Clear IBI end flag (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn cibiendf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Clear IBI end flag (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_cibiendf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Clear controller-role request flag (when the I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn ccrf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Clear controller-role request flag (when the I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_ccrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Clear controller-role update flag (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn ccrupdf(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Clear controller-role update flag (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_ccrupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Clear hot-join flag (when the I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn chjf(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Clear hot-join flag (when the I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_chjf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Clear wake-up flag (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn cwkpf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Clear wake-up flag (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_cwkpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Clear GETxxx CCC flag (except GETSTATUS of format 1) (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn cgetf(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Clear GETxxx CCC flag (except GETSTATUS of format 1) (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_cgetf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Clear format 1 GETSTATUS CCC flag (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn cstaf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Clear format 1 GETSTATUS CCC flag (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_cstaf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Clear ENTDAA/RSTDAA/SETNEWDA CCC flag (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn cdaupdf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ENTDAA/RSTDAA/SETNEWDA CCC flag (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_cdaupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Clear SETMWL CCC flag (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn cmwlupdf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Clear SETMWL CCC flag (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_cmwlupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Clear SETMRL CCC flag (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn cmrlupdf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Clear SETMRL CCC flag (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_cmrlupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Clear reset pattern flag (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn crstf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Clear reset pattern flag (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_crstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Clear ENTASx CCC flag (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn casupdf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ENTASx CCC flag (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_casupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Clear ENEC/DISEC CCC flag (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn cintupdf(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ENEC/DISEC CCC flag (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_cintupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Clear DEFTGTS CCC flag (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn cdeff(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Clear DEFTGTS CCC flag (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_cdeff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Clear DEFGRPA CCC flag (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn cgrpf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Clear DEFGRPA CCC flag (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_cgrpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cevr {
        #[inline(always)]
        fn default() -> Cevr {
            Cevr(0)
        }
    }
    impl core::fmt::Debug for Cevr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cevr")
                .field("cfcf", &self.cfcf())
                .field("crxtgtendf", &self.crxtgtendf())
                .field("cerrf", &self.cerrf())
                .field("cibif", &self.cibif())
                .field("cibiendf", &self.cibiendf())
                .field("ccrf", &self.ccrf())
                .field("ccrupdf", &self.ccrupdf())
                .field("chjf", &self.chjf())
                .field("cwkpf", &self.cwkpf())
                .field("cgetf", &self.cgetf())
                .field("cstaf", &self.cstaf())
                .field("cdaupdf", &self.cdaupdf())
                .field("cmwlupdf", &self.cmwlupdf())
                .field("cmrlupdf", &self.cmrlupdf())
                .field("crstf", &self.crstf())
                .field("casupdf", &self.casupdf())
                .field("cintupdf", &self.cintupdf())
                .field("cdeff", &self.cdeff())
                .field("cgrpf", &self.cgrpf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cevr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cevr {{ cfcf: {=bool:?}, crxtgtendf: {=bool:?}, cerrf: {=bool:?}, cibif: {=bool:?}, cibiendf: {=bool:?}, ccrf: {=bool:?}, ccrupdf: {=bool:?}, chjf: {=bool:?}, cwkpf: {=bool:?}, cgetf: {=bool:?}, cstaf: {=bool:?}, cdaupdf: {=bool:?}, cmwlupdf: {=bool:?}, cmrlupdf: {=bool:?}, crstf: {=bool:?}, casupdf: {=bool:?}, cintupdf: {=bool:?}, cdeff: {=bool:?}, cgrpf: {=bool:?} }}",
                self.cfcf(),
                self.crxtgtendf(),
                self.cerrf(),
                self.cibif(),
                self.cibiendf(),
                self.ccrf(),
                self.ccrupdf(),
                self.chjf(),
                self.cwkpf(),
                self.cgetf(),
                self.cstaf(),
                self.cdaupdf(),
                self.cmwlupdf(),
                self.cmrlupdf(),
                self.crstf(),
                self.casupdf(),
                self.cintupdf(),
                self.cdeff(),
                self.cgrpf()
            )
        }
    }
    #[doc = "I3C configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "I3C enable (whatever I3C acts as controller/target)."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "I3C enable (whatever I3C acts as controller/target)."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Initial controller/target role."]
        #[must_use]
        #[inline(always)]
        pub const fn crinit(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Initial controller/target role."]
        #[inline(always)]
        pub const fn set_crinit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "No arbitrable header after a start (when I3C acts as a controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn noarbh(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "No arbitrable header after a start (when I3C acts as a controller)."]
        #[inline(always)]
        pub const fn set_noarbh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HDR reset pattern enable (when I3C acts as a controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn rstptrn(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HDR reset pattern enable (when I3C acts as a controller)."]
        #[inline(always)]
        pub const fn set_rstptrn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HDR exit pattern enable (when I3C acts as a controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn exitptrn(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HDR exit pattern enable (when I3C acts as a controller)."]
        #[inline(always)]
        pub const fn set_exitptrn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "High-keeper enable on SDA line (when I3C acts as a controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn hksdaen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "High-keeper enable on SDA line (when I3C acts as a controller)."]
        #[inline(always)]
        pub const fn set_hksdaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Hot-join request acknowledge (when I3C acts as a controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn hjack(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Hot-join request acknowledge (when I3C acts as a controller)."]
        #[inline(always)]
        pub const fn set_hjack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "RX-FIFO DMA request enable (whatever I3C acts as controller/target)."]
        #[must_use]
        #[inline(always)]
        pub const fn rxdmaen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "RX-FIFO DMA request enable (whatever I3C acts as controller/target)."]
        #[inline(always)]
        pub const fn set_rxdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "RX-FIFO flush (whatever I3C acts as controller/target)."]
        #[must_use]
        #[inline(always)]
        pub const fn rxflush(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "RX-FIFO flush (whatever I3C acts as controller/target)."]
        #[inline(always)]
        pub const fn set_rxflush(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "RX-FIFO threshold (whatever I3C acts as controller/target)."]
        #[must_use]
        #[inline(always)]
        pub const fn rxthres(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "RX-FIFO threshold (whatever I3C acts as controller/target)."]
        #[inline(always)]
        pub const fn set_rxthres(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "TX-FIFO DMA request enable (whatever I3C acts as controller/target)."]
        #[must_use]
        #[inline(always)]
        pub const fn txdmaen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "TX-FIFO DMA request enable (whatever I3C acts as controller/target)."]
        #[inline(always)]
        pub const fn set_txdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TX-FIFO flush (whatever I3C acts as controller/target)."]
        #[must_use]
        #[inline(always)]
        pub const fn txflush(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TX-FIFO flush (whatever I3C acts as controller/target)."]
        #[inline(always)]
        pub const fn set_txflush(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "TX-FIFO threshold (whatever I3C acts as controller/target)."]
        #[must_use]
        #[inline(always)]
        pub const fn txthres(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "TX-FIFO threshold (whatever I3C acts as controller/target)."]
        #[inline(always)]
        pub const fn set_txthres(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "S-FIFO DMA request enable (when I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn sdmaen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "S-FIFO DMA request enable (when I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_sdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "S-FIFO flush (when I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn sflush(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "S-FIFO flush (when I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_sflush(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "S-FIFO enable / status receive mode (when I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn smode(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "S-FIFO enable / status receive mode (when I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_smode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Transmit mode (when I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn tmode(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit mode (when I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_tmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "C-FIFO DMA request enable (when I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn cdmaen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "C-FIFO DMA request enable (when I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_cdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "C-FIFO flush (when I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn cflush(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "C-FIFO flush (when I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_cflush(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Frame transfer set (software trigger) (when I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn tsfset(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Frame transfer set (software trigger) (when I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_tsfset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
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
                .field("en", &self.en())
                .field("crinit", &self.crinit())
                .field("noarbh", &self.noarbh())
                .field("rstptrn", &self.rstptrn())
                .field("exitptrn", &self.exitptrn())
                .field("hksdaen", &self.hksdaen())
                .field("hjack", &self.hjack())
                .field("rxdmaen", &self.rxdmaen())
                .field("rxflush", &self.rxflush())
                .field("rxthres", &self.rxthres())
                .field("txdmaen", &self.txdmaen())
                .field("txflush", &self.txflush())
                .field("txthres", &self.txthres())
                .field("sdmaen", &self.sdmaen())
                .field("sflush", &self.sflush())
                .field("smode", &self.smode())
                .field("tmode", &self.tmode())
                .field("cdmaen", &self.cdmaen())
                .field("cflush", &self.cflush())
                .field("tsfset", &self.tsfset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfgr {{ en: {=bool:?}, crinit: {=bool:?}, noarbh: {=bool:?}, rstptrn: {=bool:?}, exitptrn: {=bool:?}, hksdaen: {=bool:?}, hjack: {=bool:?}, rxdmaen: {=bool:?}, rxflush: {=bool:?}, rxthres: {=bool:?}, txdmaen: {=bool:?}, txflush: {=bool:?}, txthres: {=bool:?}, sdmaen: {=bool:?}, sflush: {=bool:?}, smode: {=bool:?}, tmode: {=bool:?}, cdmaen: {=bool:?}, cflush: {=bool:?}, tsfset: {=bool:?} }}",
                self.en(),
                self.crinit(),
                self.noarbh(),
                self.rstptrn(),
                self.exitptrn(),
                self.hksdaen(),
                self.hjack(),
                self.rxdmaen(),
                self.rxflush(),
                self.rxthres(),
                self.txdmaen(),
                self.txflush(),
                self.txthres(),
                self.sdmaen(),
                self.sflush(),
                self.smode(),
                self.tmode(),
                self.cdmaen(),
                self.cflush(),
                self.tsfset()
            )
        }
    }
    #[doc = "I3C message control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Count of data to transfer during a read or write message, in bytes (whatever I3C acts as controller/target)."]
        #[must_use]
        #[inline(always)]
        pub const fn dcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Count of data to transfer during a read or write message, in bytes (whatever I3C acts as controller/target)."]
        #[inline(always)]
        pub const fn set_dcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Read / non-write message (when I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn rnw(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Read / non-write message (when I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_rnw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "7-bit I3C dynamic / Iless thansup>2less than/sup>C static target address (when I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn add(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x7f;
            val as u8
        }
        #[doc = "7-bit I3C dynamic / Iless thansup>2less than/sup>C static target address (when I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_add(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
        }
        #[doc = "Message type (whatever I3C acts as controller/target)."]
        #[must_use]
        #[inline(always)]
        pub const fn mtype(&self) -> u8 {
            let val = (self.0 >> 27usize) & 0x0f;
            val as u8
        }
        #[doc = "Message type (whatever I3C acts as controller/target)."]
        #[inline(always)]
        pub const fn set_mtype(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 27usize)) | (((val as u32) & 0x0f) << 27usize);
        }
        #[doc = "Message end type / last message of a frame (when the I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn mend(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Message end type / last message of a frame (when the I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_mend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("dcnt", &self.dcnt())
                .field("rnw", &self.rnw())
                .field("add", &self.add())
                .field("mtype", &self.mtype())
                .field("mend", &self.mend())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ dcnt: {=u16:?}, rnw: {=bool:?}, add: {=u8:?}, mtype: {=u8:?}, mend: {=bool:?} }}",
                self.dcnt(),
                self.rnw(),
                self.add(),
                self.mtype(),
                self.mend()
            )
        }
    }
    #[doc = "I3C message control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CrAlternate(pub u32);
    impl CrAlternate {
        #[doc = "Count of related data to the CCC command to transfer as CCC defining bytes, or CCC sub-command bytes, or CCC data bytes, in bytes."]
        #[must_use]
        #[inline(always)]
        pub const fn dcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Count of related data to the CCC command to transfer as CCC defining bytes, or CCC sub-command bytes, or CCC data bytes, in bytes."]
        #[inline(always)]
        pub const fn set_dcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "8-bit CCC code (when I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn ccc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "8-bit CCC code (when I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_ccc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Message type (when I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn mtype(&self) -> u8 {
            let val = (self.0 >> 27usize) & 0x0f;
            val as u8
        }
        #[doc = "Message type (when I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_mtype(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 27usize)) | (((val as u32) & 0x0f) << 27usize);
        }
        #[doc = "Message end type / last message of a frame (when I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn mend(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Message end type / last message of a frame (when I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_mend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for CrAlternate {
        #[inline(always)]
        fn default() -> CrAlternate {
            CrAlternate(0)
        }
    }
    impl core::fmt::Debug for CrAlternate {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CrAlternate")
                .field("dcnt", &self.dcnt())
                .field("ccc", &self.ccc())
                .field("mtype", &self.mtype())
                .field("mend", &self.mend())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for CrAlternate {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "CrAlternate {{ dcnt: {=u16:?}, ccc: {=u8:?}, mtype: {=u8:?}, mend: {=bool:?} }}",
                self.dcnt(),
                self.ccc(),
                self.mtype(),
                self.mend()
            )
        }
    }
    #[doc = "I3C controller-role capability register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crcapr(pub u32);
    impl Crcapr {
        #[doc = "delayed controller-role hand-off."]
        #[must_use]
        #[inline(always)]
        pub const fn capdhoff(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "delayed controller-role hand-off."]
        #[inline(always)]
        pub const fn set_capdhoff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "group management support (when acting as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn capgrp(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "group management support (when acting as controller)."]
        #[inline(always)]
        pub const fn set_capgrp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Crcapr {
        #[inline(always)]
        fn default() -> Crcapr {
            Crcapr(0)
        }
    }
    impl core::fmt::Debug for Crcapr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Crcapr")
                .field("capdhoff", &self.capdhoff())
                .field("capgrp", &self.capgrp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Crcapr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Crcapr {{ capdhoff: {=bool:?}, capgrp: {=bool:?} }}",
                self.capdhoff(),
                self.capgrp()
            )
        }
    }
    #[doc = "I3C device characteristics register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dcr(pub u32);
    impl Dcr {
        #[doc = "device characteristics ID."]
        #[must_use]
        #[inline(always)]
        pub const fn dcr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "device characteristics ID."]
        #[inline(always)]
        pub const fn set_dcr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Dcr {
        #[inline(always)]
        fn default() -> Dcr {
            Dcr(0)
        }
    }
    impl core::fmt::Debug for Dcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dcr").field("dcr", &self.dcr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dcr {{ dcr: {=u8:?} }}", self.dcr())
        }
    }
    #[doc = "I3C device 1 characteristics register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Devr(pub u32);
    impl Devr {
        #[doc = "Assigned I3C dynamic address to target x (when the I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn da(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "Assigned I3C dynamic address to target x (when the I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_da(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
        #[doc = "IBI request acknowledge (when the I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn ibiack(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "IBI request acknowledge (when the I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_ibiack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Controller-role request acknowledge (when the I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn crack(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Controller-role request acknowledge (when the I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_crack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "IBI data enable (when the I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn ibiden(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "IBI data enable (when the I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_ibiden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Suspend/stop I3C transfer on received IBI (when the I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn susp(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Suspend/stop I3C transfer on received IBI (when the I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_susp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "DA\\[6:0\\]
write disabled (when the I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn dis(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "DA\\[6:0\\]
write disabled (when the I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Devr {
        #[inline(always)]
        fn default() -> Devr {
            Devr(0)
        }
    }
    impl core::fmt::Debug for Devr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Devr")
                .field("da", &self.da())
                .field("ibiack", &self.ibiack())
                .field("crack", &self.crack())
                .field("ibiden", &self.ibiden())
                .field("susp", &self.susp())
                .field("dis", &self.dis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Devr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Devr {{ da: {=u8:?}, ibiack: {=bool:?}, crack: {=bool:?}, ibiden: {=bool:?}, susp: {=bool:?}, dis: {=bool:?} }}",
                self.da(),
                self.ibiack(),
                self.crack(),
                self.ibiden(),
                self.susp(),
                self.dis()
            )
        }
    }
    #[doc = "I3C own device characteristics register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Devr0(pub u32);
    impl Devr0 {
        #[doc = "Dynamic address is valid (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn daval(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Dynamic address is valid (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_daval(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "7-bit dynamic address."]
        #[must_use]
        #[inline(always)]
        pub const fn da(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "7-bit dynamic address."]
        #[inline(always)]
        pub const fn set_da(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
        #[doc = "IBI request enable (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn ibien(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "IBI request enable (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_ibien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Controller-role request enable (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn cren(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Controller-role request enable (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_cren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Hot-join request enable (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn hjen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Hot-join request enable (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_hjen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Activity state (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn as_(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Activity state (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_as_(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "Reset action/level on received reset pattern (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn rstact(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "Reset action/level on received reset pattern (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_rstact(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "Reset action is valid (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn rstval(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Reset action is valid (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_rstval(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Devr0 {
        #[inline(always)]
        fn default() -> Devr0 {
            Devr0(0)
        }
    }
    impl core::fmt::Debug for Devr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Devr0")
                .field("daval", &self.daval())
                .field("da", &self.da())
                .field("ibien", &self.ibien())
                .field("cren", &self.cren())
                .field("hjen", &self.hjen())
                .field("as_", &self.as_())
                .field("rstact", &self.rstact())
                .field("rstval", &self.rstval())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Devr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Devr0 {{ daval: {=bool:?}, da: {=u8:?}, ibien: {=bool:?}, cren: {=bool:?}, hjen: {=bool:?}, as_: {=u8:?}, rstact: {=u8:?}, rstval: {=bool:?} }}",
                self.daval(),
                self.da(),
                self.ibien(),
                self.cren(),
                self.hjen(),
                self.as_(),
                self.rstact(),
                self.rstval()
            )
        }
    }
    #[doc = "I3C receive data byte register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr(pub u32);
    impl Dr {
        #[doc = "8-bit received data on I3C bus."]
        #[must_use]
        #[inline(always)]
        pub const fn db(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "8-bit received data on I3C bus."]
        #[inline(always)]
        pub const fn set_db(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Dr {
        #[inline(always)]
        fn default() -> Dr {
            Dr(0)
        }
    }
    impl core::fmt::Debug for Dr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dr").field("db", &self.db()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dr {{ db: {=u8:?} }}", self.db())
        }
    }
    #[doc = "I3C receive data word register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dwr(pub u32);
    impl Dwr {
        #[doc = "8-bit received data (earliest byte on I3C bus)."]
        #[must_use]
        #[inline(always)]
        pub const fn db(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0xff;
            val as u8
        }
        #[doc = "8-bit received data (earliest byte on I3C bus)."]
        #[inline(always)]
        pub const fn set_db(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0xff << offs)) | (((val as u32) & 0xff) << offs);
        }
    }
    impl Default for Dwr {
        #[inline(always)]
        fn default() -> Dwr {
            Dwr(0)
        }
    }
    impl core::fmt::Debug for Dwr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dwr")
                .field("db[0]", &self.db(0usize))
                .field("db[1]", &self.db(1usize))
                .field("db[2]", &self.db(2usize))
                .field("db[3]", &self.db(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dwr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dwr {{ db[0]: {=u8:?}, db[1]: {=u8:?}, db[2]: {=u8:?}, db[3]: {=u8:?} }}",
                self.db(0usize),
                self.db(1usize),
                self.db(2usize),
                self.db(3usize)
            )
        }
    }
    #[doc = "I3C extended provisioned ID register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Epidr(pub u32);
    impl Epidr {
        #[doc = "4-bit MIPI Instance ID."]
        #[must_use]
        #[inline(always)]
        pub const fn mipiid(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "4-bit MIPI Instance ID."]
        #[inline(always)]
        pub const fn set_mipiid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "provisioned ID type selector."]
        #[must_use]
        #[inline(always)]
        pub const fn idtsel(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "provisioned ID type selector."]
        #[inline(always)]
        pub const fn set_idtsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "15-bit MIPI manufacturer ID."]
        #[must_use]
        #[inline(always)]
        pub const fn mipimid(&self) -> u16 {
            let val = (self.0 >> 17usize) & 0x7fff;
            val as u16
        }
        #[doc = "15-bit MIPI manufacturer ID."]
        #[inline(always)]
        pub const fn set_mipimid(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 17usize)) | (((val as u32) & 0x7fff) << 17usize);
        }
    }
    impl Default for Epidr {
        #[inline(always)]
        fn default() -> Epidr {
            Epidr(0)
        }
    }
    impl core::fmt::Debug for Epidr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Epidr")
                .field("mipiid", &self.mipiid())
                .field("idtsel", &self.idtsel())
                .field("mipimid", &self.mipimid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Epidr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Epidr {{ mipiid: {=u8:?}, idtsel: {=bool:?}, mipimid: {=u16:?} }}",
                self.mipiid(),
                self.idtsel(),
                self.mipimid()
            )
        }
    }
    #[doc = "I3C event register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Evr(pub u32);
    impl Evr {
        #[doc = "C-FIFO empty flag (whatever the I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn cfef(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "C-FIFO empty flag (whatever the I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_cfef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TX-FIFO empty flag (whatever the I3C acts as controller/target)."]
        #[must_use]
        #[inline(always)]
        pub const fn txfef(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TX-FIFO empty flag (whatever the I3C acts as controller/target)."]
        #[inline(always)]
        pub const fn set_txfef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "C-FIFO not full flag (when the I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn cfnff(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "C-FIFO not full flag (when the I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_cfnff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "S-FIFO not empty flag (when the I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn sfnef(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "S-FIFO not empty flag (when the I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_sfnef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TX-FIFO not full flag (whatever the I3C acts as controller/target)."]
        #[must_use]
        #[inline(always)]
        pub const fn txfnff(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TX-FIFO not full flag (whatever the I3C acts as controller/target)."]
        #[inline(always)]
        pub const fn set_txfnff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "RX-FIFO not empty flag (whatever the I3C acts as controller/target)."]
        #[must_use]
        #[inline(always)]
        pub const fn rxfnef(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RX-FIFO not empty flag (whatever the I3C acts as controller/target)."]
        #[inline(always)]
        pub const fn set_rxfnef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Last written data byte/word flag (whatever the I3C acts as controller/target)."]
        #[must_use]
        #[inline(always)]
        pub const fn txlastf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Last written data byte/word flag (whatever the I3C acts as controller/target)."]
        #[inline(always)]
        pub const fn set_txlastf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Last read data byte/word flag (when the I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn rxlastf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Last read data byte/word flag (when the I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_rxlastf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Frame complete flag (whatever the I3C acts as controller/target)."]
        #[must_use]
        #[inline(always)]
        pub const fn fcf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Frame complete flag (whatever the I3C acts as controller/target)."]
        #[inline(always)]
        pub const fn set_fcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Target-initiated read end flag (when the I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn rxtgtendf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Target-initiated read end flag (when the I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_rxtgtendf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Flag (whatever the I3C acts as controller/target)."]
        #[must_use]
        #[inline(always)]
        pub const fn errf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Flag (whatever the I3C acts as controller/target)."]
        #[inline(always)]
        pub const fn set_errf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "IBI flag (when the I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn ibif(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "IBI flag (when the I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_ibif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "IBI end flag (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn ibiendf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "IBI end flag (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_ibiendf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Controller-role request flag (when the I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn crf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Controller-role request flag (when the I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_crf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Controller-role update flag (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn crupdf(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Controller-role update flag (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_crupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Hot-join flag (when the I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn hjf(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Hot-join flag (when the I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_hjf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Wake-up/missed start flag (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn wkpf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-up/missed start flag (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_wkpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Get flag (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn getf(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Get flag (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_getf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Get status flag (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn staf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Get status flag (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_staf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Dynamic address update flag (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn daupdf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Dynamic address update flag (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_daupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Maximum write length update flag (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn mwlupdf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Maximum write length update flag (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_mwlupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Maximum read length update flag (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn mrlupdf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Maximum read length update flag (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_mrlupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Reset pattern flag (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn rstf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Reset pattern flag (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_rstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Activity state update flag (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn asupdf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Activity state update flag (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_asupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Interrupt/controller-role/hot-join update flag (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn intupdf(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt/controller-role/hot-join update flag (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_intupdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "DEFTGTS flag (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn deff(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "DEFTGTS flag (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_deff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Group addressing flag (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn grpf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Group addressing flag (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_grpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Evr {
        #[inline(always)]
        fn default() -> Evr {
            Evr(0)
        }
    }
    impl core::fmt::Debug for Evr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Evr")
                .field("cfef", &self.cfef())
                .field("txfef", &self.txfef())
                .field("cfnff", &self.cfnff())
                .field("sfnef", &self.sfnef())
                .field("txfnff", &self.txfnff())
                .field("rxfnef", &self.rxfnef())
                .field("txlastf", &self.txlastf())
                .field("rxlastf", &self.rxlastf())
                .field("fcf", &self.fcf())
                .field("rxtgtendf", &self.rxtgtendf())
                .field("errf", &self.errf())
                .field("ibif", &self.ibif())
                .field("ibiendf", &self.ibiendf())
                .field("crf", &self.crf())
                .field("crupdf", &self.crupdf())
                .field("hjf", &self.hjf())
                .field("wkpf", &self.wkpf())
                .field("getf", &self.getf())
                .field("staf", &self.staf())
                .field("daupdf", &self.daupdf())
                .field("mwlupdf", &self.mwlupdf())
                .field("mrlupdf", &self.mrlupdf())
                .field("rstf", &self.rstf())
                .field("asupdf", &self.asupdf())
                .field("intupdf", &self.intupdf())
                .field("deff", &self.deff())
                .field("grpf", &self.grpf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Evr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Evr {{ cfef: {=bool:?}, txfef: {=bool:?}, cfnff: {=bool:?}, sfnef: {=bool:?}, txfnff: {=bool:?}, rxfnef: {=bool:?}, txlastf: {=bool:?}, rxlastf: {=bool:?}, fcf: {=bool:?}, rxtgtendf: {=bool:?}, errf: {=bool:?}, ibif: {=bool:?}, ibiendf: {=bool:?}, crf: {=bool:?}, crupdf: {=bool:?}, hjf: {=bool:?}, wkpf: {=bool:?}, getf: {=bool:?}, staf: {=bool:?}, daupdf: {=bool:?}, mwlupdf: {=bool:?}, mrlupdf: {=bool:?}, rstf: {=bool:?}, asupdf: {=bool:?}, intupdf: {=bool:?}, deff: {=bool:?}, grpf: {=bool:?} }}",
                self.cfef(),
                self.txfef(),
                self.cfnff(),
                self.sfnef(),
                self.txfnff(),
                self.rxfnef(),
                self.txlastf(),
                self.rxlastf(),
                self.fcf(),
                self.rxtgtendf(),
                self.errf(),
                self.ibif(),
                self.ibiendf(),
                self.crf(),
                self.crupdf(),
                self.hjf(),
                self.wkpf(),
                self.getf(),
                self.staf(),
                self.daupdf(),
                self.mwlupdf(),
                self.mrlupdf(),
                self.rstf(),
                self.asupdf(),
                self.intupdf(),
                self.deff(),
                self.grpf()
            )
        }
    }
    #[doc = "I3C get capability register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Getcapr(pub u32);
    impl Getcapr {
        #[doc = "IBI MDB support for pending read notification."]
        #[must_use]
        #[inline(always)]
        pub const fn cappend(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "IBI MDB support for pending read notification."]
        #[inline(always)]
        pub const fn set_cappend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Getcapr {
        #[inline(always)]
        fn default() -> Getcapr {
            Getcapr(0)
        }
    }
    impl core::fmt::Debug for Getcapr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Getcapr").field("cappend", &self.cappend()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Getcapr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Getcapr {{ cappend: {=bool:?} }}", self.cappend())
        }
    }
    #[doc = "I3C get capability register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Getmxdsr(pub u32);
    impl Getmxdsr {
        #[doc = "Controller hand-off activity state."]
        #[must_use]
        #[inline(always)]
        pub const fn hoffas(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Controller hand-off activity state."]
        #[inline(always)]
        pub const fn set_hoffas(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "GETMXDS CCC format."]
        #[must_use]
        #[inline(always)]
        pub const fn fmt(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "GETMXDS CCC format."]
        #[inline(always)]
        pub const fn set_fmt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "programmed byte of the 3-byte MaxRdTurn (maximum read turnaround byte)."]
        #[must_use]
        #[inline(always)]
        pub const fn rdturn(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "programmed byte of the 3-byte MaxRdTurn (maximum read turnaround byte)."]
        #[inline(always)]
        pub const fn set_rdturn(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "clock-to-data turnaround time (tless thansub>SCOless than/sub>)."]
        #[must_use]
        #[inline(always)]
        pub const fn tsco(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "clock-to-data turnaround time (tless thansub>SCOless than/sub>)."]
        #[inline(always)]
        pub const fn set_tsco(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Getmxdsr {
        #[inline(always)]
        fn default() -> Getmxdsr {
            Getmxdsr(0)
        }
    }
    impl core::fmt::Debug for Getmxdsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Getmxdsr")
                .field("hoffas", &self.hoffas())
                .field("fmt", &self.fmt())
                .field("rdturn", &self.rdturn())
                .field("tsco", &self.tsco())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Getmxdsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Getmxdsr {{ hoffas: {=u8:?}, fmt: {=u8:?}, rdturn: {=u8:?}, tsco: {=bool:?} }}",
                self.hoffas(),
                self.fmt(),
                self.rdturn(),
                self.tsco()
            )
        }
    }
    #[doc = "I3C IBI payload data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ibidr(pub u32);
    impl Ibidr {
        #[doc = "8-bit IBI payload data (earliest byte on I3C bus, MDB\\[7:0\\]
mandatory data byte)."]
        #[must_use]
        #[inline(always)]
        pub const fn ibidb(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0xff;
            val as u8
        }
        #[doc = "8-bit IBI payload data (earliest byte on I3C bus, MDB\\[7:0\\]
mandatory data byte)."]
        #[inline(always)]
        pub const fn set_ibidb(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0xff << offs)) | (((val as u32) & 0xff) << offs);
        }
    }
    impl Default for Ibidr {
        #[inline(always)]
        fn default() -> Ibidr {
            Ibidr(0)
        }
    }
    impl core::fmt::Debug for Ibidr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ibidr")
                .field("ibidb[0]", &self.ibidb(0usize))
                .field("ibidb[1]", &self.ibidb(1usize))
                .field("ibidb[2]", &self.ibidb(2usize))
                .field("ibidb[3]", &self.ibidb(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ibidr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ibidr {{ ibidb[0]: {=u8:?}, ibidb[1]: {=u8:?}, ibidb[2]: {=u8:?}, ibidb[3]: {=u8:?} }}",
                self.ibidb(0usize),
                self.ibidb(1usize),
                self.ibidb(2usize),
                self.ibidb(3usize)
            )
        }
    }
    #[doc = "I3C interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "C-FIFO not full interrupt enable (whatever the I3C acts as controller/target)."]
        #[must_use]
        #[inline(always)]
        pub const fn cfnfie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "C-FIFO not full interrupt enable (whatever the I3C acts as controller/target)."]
        #[inline(always)]
        pub const fn set_cfnfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "S-FIFO not empty interrupt enable (whatever the I3C acts as controller/target)."]
        #[must_use]
        #[inline(always)]
        pub const fn sfneie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "S-FIFO not empty interrupt enable (whatever the I3C acts as controller/target)."]
        #[inline(always)]
        pub const fn set_sfneie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TX-FIFO not full interrupt enable (whatever the I3C acts as controller/target)."]
        #[must_use]
        #[inline(always)]
        pub const fn txfnfie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TX-FIFO not full interrupt enable (whatever the I3C acts as controller/target)."]
        #[inline(always)]
        pub const fn set_txfnfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "RX-FIFO not empty interrupt enable (whatever the I3C acts as controller/target)."]
        #[must_use]
        #[inline(always)]
        pub const fn rxfneie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RX-FIFO not empty interrupt enable (whatever the I3C acts as controller/target)."]
        #[inline(always)]
        pub const fn set_rxfneie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "frame complete interrupt enable (whatever the I3C acts as controller/target)."]
        #[must_use]
        #[inline(always)]
        pub const fn fcie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "frame complete interrupt enable (whatever the I3C acts as controller/target)."]
        #[inline(always)]
        pub const fn set_fcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "target-initiated read end interrupt enable (when the I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn rxtgtendie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "target-initiated read end interrupt enable (when the I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_rxtgtendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "error interrupt enable (whatever the I3C acts as controller/target)."]
        #[must_use]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "error interrupt enable (whatever the I3C acts as controller/target)."]
        #[inline(always)]
        pub const fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "IBI request interrupt enable (when the I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn ibiie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "IBI request interrupt enable (when the I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_ibiie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "IBI end interrupt enable (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn ibiendie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "IBI end interrupt enable (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_ibiendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Controller-role request interrupt enable (when the I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn crie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Controller-role request interrupt enable (when the I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_crie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Controller-role update interrupt enable (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn crupdie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Controller-role update interrupt enable (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_crupdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Hot-join interrupt enable (when the I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn hjie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Hot-join interrupt enable (when the I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_hjie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Wake-up interrupt enable (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn wkpie(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-up interrupt enable (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_wkpie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "GETxxx CCC interrupt enable (except GETSTATUS of format 1) (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn getie(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "GETxxx CCC interrupt enable (except GETSTATUS of format 1) (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_getie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "format 1 GETSTATUS CCC interrupt enable (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn staie(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "format 1 GETSTATUS CCC interrupt enable (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_staie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "ENTDAA/RSTDAA/SETNEWDA CCC interrupt enable (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn daupdie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ENTDAA/RSTDAA/SETNEWDA CCC interrupt enable (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_daupdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "SETMWL CCC interrupt enable (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn mwlupdie(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "SETMWL CCC interrupt enable (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_mwlupdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "SETMRL CCC interrupt enable (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn mrlupdie(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "SETMRL CCC interrupt enable (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_mrlupdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "reset pattern interrupt enable (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn rstie(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "reset pattern interrupt enable (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_rstie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "ENTASx CCC interrupt enable (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn asupdie(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "ENTASx CCC interrupt enable (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_asupdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "ENEC/DISEC CCC interrupt enable (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn intupdie(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "ENEC/DISEC CCC interrupt enable (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_intupdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "DEFTGTS CCC interrupt enable (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn defie(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "DEFTGTS CCC interrupt enable (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_defie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "DEFGRPA CCC interrupt enable (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn grpie(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "DEFGRPA CCC interrupt enable (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_grpie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("cfnfie", &self.cfnfie())
                .field("sfneie", &self.sfneie())
                .field("txfnfie", &self.txfnfie())
                .field("rxfneie", &self.rxfneie())
                .field("fcie", &self.fcie())
                .field("rxtgtendie", &self.rxtgtendie())
                .field("errie", &self.errie())
                .field("ibiie", &self.ibiie())
                .field("ibiendie", &self.ibiendie())
                .field("crie", &self.crie())
                .field("crupdie", &self.crupdie())
                .field("hjie", &self.hjie())
                .field("wkpie", &self.wkpie())
                .field("getie", &self.getie())
                .field("staie", &self.staie())
                .field("daupdie", &self.daupdie())
                .field("mwlupdie", &self.mwlupdie())
                .field("mrlupdie", &self.mrlupdie())
                .field("rstie", &self.rstie())
                .field("asupdie", &self.asupdie())
                .field("intupdie", &self.intupdie())
                .field("defie", &self.defie())
                .field("grpie", &self.grpie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ier {{ cfnfie: {=bool:?}, sfneie: {=bool:?}, txfnfie: {=bool:?}, rxfneie: {=bool:?}, fcie: {=bool:?}, rxtgtendie: {=bool:?}, errie: {=bool:?}, ibiie: {=bool:?}, ibiendie: {=bool:?}, crie: {=bool:?}, crupdie: {=bool:?}, hjie: {=bool:?}, wkpie: {=bool:?}, getie: {=bool:?}, staie: {=bool:?}, daupdie: {=bool:?}, mwlupdie: {=bool:?}, mrlupdie: {=bool:?}, rstie: {=bool:?}, asupdie: {=bool:?}, intupdie: {=bool:?}, defie: {=bool:?}, grpie: {=bool:?} }}",
                self.cfnfie(),
                self.sfneie(),
                self.txfnfie(),
                self.rxfneie(),
                self.fcie(),
                self.rxtgtendie(),
                self.errie(),
                self.ibiie(),
                self.ibiendie(),
                self.crie(),
                self.crupdie(),
                self.hjie(),
                self.wkpie(),
                self.getie(),
                self.staie(),
                self.daupdie(),
                self.mwlupdie(),
                self.mrlupdie(),
                self.rstie(),
                self.asupdie(),
                self.intupdie(),
                self.defie(),
                self.grpie()
            )
        }
    }
    #[doc = "I3C maximum read length register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maxrlr(pub u32);
    impl Maxrlr {
        #[doc = "Maximum data read length (when I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn ml(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Maximum data read length (when I3C acts as target)."]
        #[inline(always)]
        pub const fn set_ml(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "IBI payload data maximum size, in bytes (when I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn ibip(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "IBI payload data maximum size, in bytes (when I3C acts as target)."]
        #[inline(always)]
        pub const fn set_ibip(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for Maxrlr {
        #[inline(always)]
        fn default() -> Maxrlr {
            Maxrlr(0)
        }
    }
    impl core::fmt::Debug for Maxrlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maxrlr")
                .field("ml", &self.ml())
                .field("ibip", &self.ibip())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Maxrlr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Maxrlr {{ ml: {=u16:?}, ibip: {=u8:?} }}", self.ml(), self.ibip())
        }
    }
    #[doc = "I3C maximum write length register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maxwlr(pub u32);
    impl Maxwlr {
        #[doc = "Maximum data write length (when I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn ml(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Maximum data write length (when I3C acts as target)."]
        #[inline(always)]
        pub const fn set_ml(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Maxwlr {
        #[inline(always)]
        fn default() -> Maxwlr {
            Maxwlr(0)
        }
    }
    impl core::fmt::Debug for Maxwlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maxwlr").field("ml", &self.ml()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Maxwlr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Maxwlr {{ ml: {=u16:?} }}", self.ml())
        }
    }
    #[doc = "I3C received message register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rmr(pub u32);
    impl Rmr {
        #[doc = "IBI received payload data count (when the I3C is configured as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn ibirdcnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "IBI received payload data count (when the I3C is configured as controller)."]
        #[inline(always)]
        pub const fn set_ibirdcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Received CCC code (when the I3C is configured as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn rcode(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Received CCC code (when the I3C is configured as target)."]
        #[inline(always)]
        pub const fn set_rcode(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Received target address (when the I3C is configured as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn radd(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x7f;
            val as u8
        }
        #[doc = "Received target address (when the I3C is configured as controller)."]
        #[inline(always)]
        pub const fn set_radd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
        }
    }
    impl Default for Rmr {
        #[inline(always)]
        fn default() -> Rmr {
            Rmr(0)
        }
    }
    impl core::fmt::Debug for Rmr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rmr")
                .field("ibirdcnt", &self.ibirdcnt())
                .field("rcode", &self.rcode())
                .field("radd", &self.radd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rmr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rmr {{ ibirdcnt: {=u8:?}, rcode: {=u8:?}, radd: {=u8:?} }}",
                self.ibirdcnt(),
                self.rcode(),
                self.radd()
            )
        }
    }
    #[doc = "I3C status error register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ser(pub u32);
    impl Ser {
        #[doc = "Protocol error code/type."]
        #[must_use]
        #[inline(always)]
        pub const fn coderr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Protocol error code/type."]
        #[inline(always)]
        pub const fn set_coderr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Protocol error."]
        #[must_use]
        #[inline(always)]
        pub const fn perr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Protocol error."]
        #[inline(always)]
        pub const fn set_perr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SCL stall error (when the I3C acts as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SCL stall error (when the I3C acts as target)."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "RX-FIFO overrun or TX-FIFO underrun."]
        #[must_use]
        #[inline(always)]
        pub const fn dovr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RX-FIFO overrun or TX-FIFO underrun."]
        #[inline(always)]
        pub const fn set_dovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "C-FIFO underrun or S-FIFO overrun (when the I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn covr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "C-FIFO underrun or S-FIFO overrun (when the I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_covr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Address not acknowledged (when the I3C is configured as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn anack(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Address not acknowledged (when the I3C is configured as controller)."]
        #[inline(always)]
        pub const fn set_anack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Data not acknowledged (when the I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn dnack(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Data not acknowledged (when the I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_dnack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Data error (when the I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn derr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Data error (when the I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_derr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Ser {
        #[inline(always)]
        fn default() -> Ser {
            Ser(0)
        }
    }
    impl core::fmt::Debug for Ser {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ser")
                .field("coderr", &self.coderr())
                .field("perr", &self.perr())
                .field("stall", &self.stall())
                .field("dovr", &self.dovr())
                .field("covr", &self.covr())
                .field("anack", &self.anack())
                .field("dnack", &self.dnack())
                .field("derr", &self.derr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ser {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ser {{ coderr: {=u8:?}, perr: {=bool:?}, stall: {=bool:?}, dovr: {=bool:?}, covr: {=bool:?}, anack: {=bool:?}, dnack: {=bool:?}, derr: {=bool:?} }}",
                self.coderr(),
                self.perr(),
                self.stall(),
                self.dovr(),
                self.covr(),
                self.anack(),
                self.dnack(),
                self.derr()
            )
        }
    }
    #[doc = "I3C status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Data counter."]
        #[must_use]
        #[inline(always)]
        pub const fn xdcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Data counter."]
        #[inline(always)]
        pub const fn set_xdcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "A private read message is ended prematurely by the target (when the I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn abt(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "A private read message is ended prematurely by the target (when the I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_abt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Message direction."]
        #[must_use]
        #[inline(always)]
        pub const fn dir(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Message direction."]
        #[inline(always)]
        pub const fn set_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Message identifier/counter of a given frame (when the I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn mid(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Message identifier/counter of a given frame (when the I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_mid(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
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
                .field("xdcnt", &self.xdcnt())
                .field("abt", &self.abt())
                .field("dir", &self.dir())
                .field("mid", &self.mid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ xdcnt: {=u16:?}, abt: {=bool:?}, dir: {=bool:?}, mid: {=u8:?} }}",
                self.xdcnt(),
                self.abt(),
                self.dir(),
                self.mid()
            )
        }
    }
    #[doc = "I3C target transmit configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tgttdr(pub u32);
    impl Tgttdr {
        #[doc = "Transmit data counter, in bytes (when I3C is configured as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn tgttdcnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Transmit data counter, in bytes (when I3C is configured as target)."]
        #[inline(always)]
        pub const fn set_tgttdcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Preload of the TX-FIFO (when I3C is configured as target)."]
        #[must_use]
        #[inline(always)]
        pub const fn preload(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Preload of the TX-FIFO (when I3C is configured as target)."]
        #[inline(always)]
        pub const fn set_preload(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Tgttdr {
        #[inline(always)]
        fn default() -> Tgttdr {
            Tgttdr(0)
        }
    }
    impl core::fmt::Debug for Tgttdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tgttdr")
                .field("tgttdcnt", &self.tgttdcnt())
                .field("preload", &self.preload())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tgttdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Tgttdr {{ tgttdcnt: {=u16:?}, preload: {=bool:?} }}",
                self.tgttdcnt(),
                self.preload()
            )
        }
    }
    #[doc = "I3C timing register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timingr0(pub u32);
    impl Timingr0 {
        #[doc = "SCL low duration in I3C push-pull phases, in number of kernel clocks cycles:."]
        #[must_use]
        #[inline(always)]
        pub const fn scll_pp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "SCL low duration in I3C push-pull phases, in number of kernel clocks cycles:."]
        #[inline(always)]
        pub const fn set_scll_pp(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "SCL high duration, used for I3C messages (both in push-pull and open-drain phases), in number of kernel clocks cycles:."]
        #[must_use]
        #[inline(always)]
        pub const fn sclh_i3c(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "SCL high duration, used for I3C messages (both in push-pull and open-drain phases), in number of kernel clocks cycles:."]
        #[inline(always)]
        pub const fn set_sclh_i3c(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "SCL low duration in open-drain phases, used for legacy Iless thansup>2less than/sup>C messages and for I3C open-drain phases (address phase following a start, ACK phase during controller-initiated messages, and T bit phase during direct/private/IBI payload), in number of kernel clocks cycles:."]
        #[must_use]
        #[inline(always)]
        pub const fn scll_od(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "SCL low duration in open-drain phases, used for legacy Iless thansup>2less than/sup>C messages and for I3C open-drain phases (address phase following a start, ACK phase during controller-initiated messages, and T bit phase during direct/private/IBI payload), in number of kernel clocks cycles:."]
        #[inline(always)]
        pub const fn set_scll_od(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "SCL high duration, used for legacy Iless thansup>2less than/sup>C messages, in number of kernel clocks cycles:."]
        #[must_use]
        #[inline(always)]
        pub const fn sclh_i2c(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "SCL high duration, used for legacy Iless thansup>2less than/sup>C messages, in number of kernel clocks cycles:."]
        #[inline(always)]
        pub const fn set_sclh_i2c(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Timingr0 {
        #[inline(always)]
        fn default() -> Timingr0 {
            Timingr0(0)
        }
    }
    impl core::fmt::Debug for Timingr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timingr0")
                .field("scll_pp", &self.scll_pp())
                .field("sclh_i3c", &self.sclh_i3c())
                .field("scll_od", &self.scll_od())
                .field("sclh_i2c", &self.sclh_i2c())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timingr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timingr0 {{ scll_pp: {=u8:?}, sclh_i3c: {=u8:?}, scll_od: {=u8:?}, sclh_i2c: {=u8:?} }}",
                self.scll_pp(),
                self.sclh_i3c(),
                self.scll_od(),
                self.sclh_i2c()
            )
        }
    }
    #[doc = "I3C timing register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timingr1(pub u32);
    impl Timingr1 {
        #[doc = "Number of kernel clock cycles to set a time unit of 1 s, whatever I3C acts as controller or target."]
        #[must_use]
        #[inline(always)]
        pub const fn aval(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Number of kernel clock cycles to set a time unit of 1 s, whatever I3C acts as controller or target."]
        #[inline(always)]
        pub const fn set_aval(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Activity state of the new controller (when I3C acts as active controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn asncr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Activity state of the new controller (when I3C acts as active controller)."]
        #[inline(always)]
        pub const fn set_asncr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "Number of kernel clocks cycles that is used to set some MIPI timings like bus free condition time (when the I3C acts as controller)."]
        #[must_use]
        #[inline(always)]
        pub const fn free(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Number of kernel clocks cycles that is used to set some MIPI timings like bus free condition time (when the I3C acts as controller)."]
        #[inline(always)]
        pub const fn set_free(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "SDA hold time (when the I3C acts as controller), in number of kernel clocks cycles (refer to MIPI timing SDA hold time in push-pull tless thansub>HD_PPless than/sub>):."]
        #[must_use]
        #[inline(always)]
        pub const fn sda_hd(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "SDA hold time (when the I3C acts as controller), in number of kernel clocks cycles (refer to MIPI timing SDA hold time in push-pull tless thansub>HD_PPless than/sub>):."]
        #[inline(always)]
        pub const fn set_sda_hd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Timingr1 {
        #[inline(always)]
        fn default() -> Timingr1 {
            Timingr1(0)
        }
    }
    impl core::fmt::Debug for Timingr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timingr1")
                .field("aval", &self.aval())
                .field("asncr", &self.asncr())
                .field("free", &self.free())
                .field("sda_hd", &self.sda_hd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timingr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timingr1 {{ aval: {=u8:?}, asncr: {=u8:?}, free: {=u8:?}, sda_hd: {=bool:?} }}",
                self.aval(),
                self.asncr(),
                self.free(),
                self.sda_hd()
            )
        }
    }
    #[doc = "I3C timing register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timingr2(pub u32);
    impl Timingr2 {
        #[doc = "Controller clock stall enable on T-bit phase of data (and on the ACK/NACK phase of data byte of a legacy Iless thansup>2less than/sup>C read)."]
        #[must_use]
        #[inline(always)]
        pub const fn stallt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Controller clock stall enable on T-bit phase of data (and on the ACK/NACK phase of data byte of a legacy Iless thansup>2less than/sup>C read)."]
        #[inline(always)]
        pub const fn set_stallt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Controller clock stall enable on PAR phase of Data."]
        #[must_use]
        #[inline(always)]
        pub const fn stalld(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Controller clock stall enable on PAR phase of Data."]
        #[inline(always)]
        pub const fn set_stalld(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Controller clock stall enable on PAR phase of CCC."]
        #[must_use]
        #[inline(always)]
        pub const fn stallc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Controller clock stall enable on PAR phase of CCC."]
        #[inline(always)]
        pub const fn set_stallc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Controller clock stall enable on ACK phase."]
        #[must_use]
        #[inline(always)]
        pub const fn stalla(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Controller clock stall enable on ACK phase."]
        #[inline(always)]
        pub const fn set_stalla(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Controller clock stall time, in number of kernel clock cycles."]
        #[must_use]
        #[inline(always)]
        pub const fn stall(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Controller clock stall time, in number of kernel clock cycles."]
        #[inline(always)]
        pub const fn set_stall(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Timingr2 {
        #[inline(always)]
        fn default() -> Timingr2 {
            Timingr2(0)
        }
    }
    impl core::fmt::Debug for Timingr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timingr2")
                .field("stallt", &self.stallt())
                .field("stalld", &self.stalld())
                .field("stallc", &self.stallc())
                .field("stalla", &self.stalla())
                .field("stall", &self.stall())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Timingr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Timingr2 {{ stallt: {=bool:?}, stalld: {=bool:?}, stallc: {=bool:?}, stalla: {=bool:?}, stall: {=u8:?} }}",
                self.stallt(),
                self.stalld(),
                self.stallc(),
                self.stalla(),
                self.stall()
            )
        }
    }
}
