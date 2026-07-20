#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch {
    ptr: *mut u8,
}
unsafe impl Send for Ch {}
unsafe impl Sync for Ch {}
impl Ch {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SAI configuration register 1."]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "SAI configuration register 2."]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "SAI frame configuration register."]
    #[inline(always)]
    pub const fn frcr(self) -> crate::common::Reg<regs::Frcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "SAI slot register."]
    #[inline(always)]
    pub const fn slotr(self) -> crate::common::Reg<regs::Slotr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "SAI interrupt mask register."]
    #[inline(always)]
    pub const fn im(self) -> crate::common::Reg<regs::Im, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "SAI status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "SAI clear flag register."]
    #[inline(always)]
    pub const fn clrfr(self) -> crate::common::Reg<regs::Clrfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "SAI data register."]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
}
#[doc = "Serial audio interface."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sai {
    ptr: *mut u8,
}
unsafe impl Send for Sai {}
unsafe impl Sync for Sai {}
impl Sai {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SAI global configuration register."]
    #[inline(always)]
    pub const fn gcr(self) -> crate::common::Reg<regs::Gcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn ch(self, n: usize) -> Ch {
        assert!(n < 2usize);
        unsafe { Ch::from_ptr(self.ptr.wrapping_add(0x04usize + n * 32usize) as _) }
    }
    #[doc = "SAI PDM control register."]
    #[inline(always)]
    pub const fn pdmcr(self) -> crate::common::Reg<regs::Pdmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "SAI PDM delay register."]
    #[inline(always)]
    pub const fn pdmdly(self) -> crate::common::Reg<regs::Pdmdly, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
}
pub mod regs {
    #[doc = "SAI clear flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Clrfr(pub u32);
    impl Clrfr {
        #[doc = "Clear overrun / underrun."]
        #[must_use]
        #[inline(always)]
        pub const fn covrudr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear overrun / underrun."]
        #[inline(always)]
        pub const fn set_covrudr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Mute detection flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cmutedet(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Mute detection flag."]
        #[inline(always)]
        pub const fn set_cmutedet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Clear wrong clock configuration flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cwckcfg(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Clear wrong clock configuration flag."]
        #[inline(always)]
        pub const fn set_cwckcfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Clear Codec not ready flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ccnrdy(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Clear Codec not ready flag."]
        #[inline(always)]
        pub const fn set_ccnrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Clear anticipated frame synchronization detection flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cafsdet(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Clear anticipated frame synchronization detection flag."]
        #[inline(always)]
        pub const fn set_cafsdet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Clear late frame synchronization detection flag."]
        #[must_use]
        #[inline(always)]
        pub const fn clfsdet(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Clear late frame synchronization detection flag."]
        #[inline(always)]
        pub const fn set_clfsdet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
                .field("covrudr", &self.covrudr())
                .field("cmutedet", &self.cmutedet())
                .field("cwckcfg", &self.cwckcfg())
                .field("ccnrdy", &self.ccnrdy())
                .field("cafsdet", &self.cafsdet())
                .field("clfsdet", &self.clfsdet())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Clrfr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Clrfr {{ covrudr: {=bool:?}, cmutedet: {=bool:?}, cwckcfg: {=bool:?}, ccnrdy: {=bool:?}, cafsdet: {=bool:?}, clfsdet: {=bool:?} }}",
                self.covrudr(),
                self.cmutedet(),
                self.cwckcfg(),
                self.ccnrdy(),
                self.cafsdet(),
                self.clfsdet()
            )
        }
    }
    #[doc = "SAI configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "SAIx audio block mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "SAIx audio block mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Protocol configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn prtcfg(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Protocol configuration."]
        #[inline(always)]
        pub const fn set_prtcfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Data size."]
        #[must_use]
        #[inline(always)]
        pub const fn ds(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x07;
            val as u8
        }
        #[doc = "Data size."]
        #[inline(always)]
        pub const fn set_ds(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
        }
        #[doc = "Least significant bit first."]
        #[must_use]
        #[inline(always)]
        pub const fn lsbfirst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Least significant bit first."]
        #[inline(always)]
        pub const fn set_lsbfirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Clock strobing edge."]
        #[must_use]
        #[inline(always)]
        pub const fn ckstr(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Clock strobing edge."]
        #[inline(always)]
        pub const fn set_ckstr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Synchronization enable."]
        #[must_use]
        #[inline(always)]
        pub const fn syncen(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "Synchronization enable."]
        #[inline(always)]
        pub const fn set_syncen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "Mono mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mono(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Mono mode."]
        #[inline(always)]
        pub const fn set_mono(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Output drive."]
        #[must_use]
        #[inline(always)]
        pub const fn outdriv(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Output drive."]
        #[inline(always)]
        pub const fn set_outdriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Audio block enable."]
        #[must_use]
        #[inline(always)]
        pub const fn saien(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Audio block enable."]
        #[inline(always)]
        pub const fn set_saien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "DMA enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dmaen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "DMA enable."]
        #[inline(always)]
        pub const fn set_dmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "No divider."]
        #[must_use]
        #[inline(always)]
        pub const fn nodiv(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "No divider."]
        #[inline(always)]
        pub const fn set_nodiv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Master clock divider."]
        #[must_use]
        #[inline(always)]
        pub const fn mckdiv(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x3f;
            val as u8
        }
        #[doc = "Master clock divider."]
        #[inline(always)]
        pub const fn set_mckdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 20usize)) | (((val as u32) & 0x3f) << 20usize);
        }
        #[doc = "Oversampling ratio for master clock."]
        #[must_use]
        #[inline(always)]
        pub const fn osr(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Oversampling ratio for master clock."]
        #[inline(always)]
        pub const fn set_osr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Master clock generation enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mcken(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Master clock generation enable."]
        #[inline(always)]
        pub const fn set_mcken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
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
                .field("mode", &self.mode())
                .field("prtcfg", &self.prtcfg())
                .field("ds", &self.ds())
                .field("lsbfirst", &self.lsbfirst())
                .field("ckstr", &self.ckstr())
                .field("syncen", &self.syncen())
                .field("mono", &self.mono())
                .field("outdriv", &self.outdriv())
                .field("saien", &self.saien())
                .field("dmaen", &self.dmaen())
                .field("nodiv", &self.nodiv())
                .field("mckdiv", &self.mckdiv())
                .field("osr", &self.osr())
                .field("mcken", &self.mcken())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr1 {{ mode: {=u8:?}, prtcfg: {=u8:?}, ds: {=u8:?}, lsbfirst: {=bool:?}, ckstr: {=bool:?}, syncen: {=u8:?}, mono: {=bool:?}, outdriv: {=bool:?}, saien: {=bool:?}, dmaen: {=bool:?}, nodiv: {=bool:?}, mckdiv: {=u8:?}, osr: {=bool:?}, mcken: {=bool:?} }}",
                self.mode(),
                self.prtcfg(),
                self.ds(),
                self.lsbfirst(),
                self.ckstr(),
                self.syncen(),
                self.mono(),
                self.outdriv(),
                self.saien(),
                self.dmaen(),
                self.nodiv(),
                self.mckdiv(),
                self.osr(),
                self.mcken()
            )
        }
    }
    #[doc = "SAI configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "FIFO threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn fth(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "FIFO threshold."]
        #[inline(always)]
        pub const fn set_fth(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "FIFO flush."]
        #[must_use]
        #[inline(always)]
        pub const fn fflush(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO flush."]
        #[inline(always)]
        pub const fn set_fflush(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Tristate management on data line."]
        #[must_use]
        #[inline(always)]
        pub const fn tris(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Tristate management on data line."]
        #[inline(always)]
        pub const fn set_tris(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Mute."]
        #[must_use]
        #[inline(always)]
        pub const fn mute(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Mute."]
        #[inline(always)]
        pub const fn set_mute(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Mute value."]
        #[must_use]
        #[inline(always)]
        pub const fn muteval(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Mute value."]
        #[inline(always)]
        pub const fn set_muteval(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Mute counter."]
        #[must_use]
        #[inline(always)]
        pub const fn mutecnt(&self) -> u8 {
            let val = (self.0 >> 7usize) & 0x3f;
            val as u8
        }
        #[doc = "Mute counter."]
        #[inline(always)]
        pub const fn set_mutecnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 7usize)) | (((val as u32) & 0x3f) << 7usize);
        }
        #[doc = "Complement bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cpl(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Complement bit."]
        #[inline(always)]
        pub const fn set_cpl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Companding mode."]
        #[must_use]
        #[inline(always)]
        pub const fn comp(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Companding mode."]
        #[inline(always)]
        pub const fn set_comp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
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
                .field("fth", &self.fth())
                .field("fflush", &self.fflush())
                .field("tris", &self.tris())
                .field("mute", &self.mute())
                .field("muteval", &self.muteval())
                .field("mutecnt", &self.mutecnt())
                .field("cpl", &self.cpl())
                .field("comp", &self.comp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr2 {{ fth: {=u8:?}, fflush: {=bool:?}, tris: {=bool:?}, mute: {=bool:?}, muteval: {=bool:?}, mutecnt: {=u8:?}, cpl: {=bool:?}, comp: {=u8:?} }}",
                self.fth(),
                self.fflush(),
                self.tris(),
                self.mute(),
                self.muteval(),
                self.mutecnt(),
                self.cpl(),
                self.comp()
            )
        }
    }
    #[doc = "SAI data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr(pub u32);
    impl Dr {
        #[doc = "Data."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Data."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
            f.debug_struct("Dr").field("data", &self.data()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dr {{ data: {=u32:?} }}", self.data())
        }
    }
    #[doc = "SAI frame configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Frcr(pub u32);
    impl Frcr {
        #[doc = "Frame length."]
        #[must_use]
        #[inline(always)]
        pub const fn frl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Frame length."]
        #[inline(always)]
        pub const fn set_frl(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Frame synchronization active level length."]
        #[must_use]
        #[inline(always)]
        pub const fn fsall(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Frame synchronization active level length."]
        #[inline(always)]
        pub const fn set_fsall(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "Frame synchronization definition."]
        #[must_use]
        #[inline(always)]
        pub const fn fsdef(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Frame synchronization definition."]
        #[inline(always)]
        pub const fn set_fsdef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Frame synchronization polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn fspol(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Frame synchronization polarity."]
        #[inline(always)]
        pub const fn set_fspol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Frame synchronization offset."]
        #[must_use]
        #[inline(always)]
        pub const fn fsoff(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Frame synchronization offset."]
        #[inline(always)]
        pub const fn set_fsoff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for Frcr {
        #[inline(always)]
        fn default() -> Frcr {
            Frcr(0)
        }
    }
    impl core::fmt::Debug for Frcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Frcr")
                .field("frl", &self.frl())
                .field("fsall", &self.fsall())
                .field("fsdef", &self.fsdef())
                .field("fspol", &self.fspol())
                .field("fsoff", &self.fsoff())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Frcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Frcr {{ frl: {=u8:?}, fsall: {=u8:?}, fsdef: {=bool:?}, fspol: {=bool:?}, fsoff: {=bool:?} }}",
                self.frl(),
                self.fsall(),
                self.fsdef(),
                self.fspol(),
                self.fsoff()
            )
        }
    }
    #[doc = "SAI global configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gcr(pub u32);
    impl Gcr {
        #[doc = "Synchronization outputs."]
        #[must_use]
        #[inline(always)]
        pub const fn syncin(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Synchronization outputs."]
        #[inline(always)]
        pub const fn set_syncin(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Synchronization outputs."]
        #[must_use]
        #[inline(always)]
        pub const fn syncout(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Synchronization outputs."]
        #[inline(always)]
        pub const fn set_syncout(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
    }
    impl Default for Gcr {
        #[inline(always)]
        fn default() -> Gcr {
            Gcr(0)
        }
    }
    impl core::fmt::Debug for Gcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gcr")
                .field("syncin", &self.syncin())
                .field("syncout", &self.syncout())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Gcr {{ syncin: {=u8:?}, syncout: {=u8:?} }}",
                self.syncin(),
                self.syncout()
            )
        }
    }
    #[doc = "SAI interrupt mask register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Im(pub u32);
    impl Im {
        #[doc = "Overrun/underrun interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ovrudrie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun/underrun interrupt enable."]
        #[inline(always)]
        pub const fn set_ovrudrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Mute detection interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mutedetie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Mute detection interrupt enable."]
        #[inline(always)]
        pub const fn set_mutedetie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Wrong clock configuration interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn wckcfgie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Wrong clock configuration interrupt enable."]
        #[inline(always)]
        pub const fn set_wckcfgie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "FIFO request interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn freqie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO request interrupt enable."]
        #[inline(always)]
        pub const fn set_freqie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Codec not ready interrupt enable (AC'97)."]
        #[must_use]
        #[inline(always)]
        pub const fn cnrdyie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Codec not ready interrupt enable (AC'97)."]
        #[inline(always)]
        pub const fn set_cnrdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Anticipated frame synchronization detection interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn afsdetie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Anticipated frame synchronization detection interrupt enable."]
        #[inline(always)]
        pub const fn set_afsdetie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Late frame synchronization detection interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lfsdetie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Late frame synchronization detection interrupt enable."]
        #[inline(always)]
        pub const fn set_lfsdetie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Im {
        #[inline(always)]
        fn default() -> Im {
            Im(0)
        }
    }
    impl core::fmt::Debug for Im {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Im")
                .field("ovrudrie", &self.ovrudrie())
                .field("mutedetie", &self.mutedetie())
                .field("wckcfgie", &self.wckcfgie())
                .field("freqie", &self.freqie())
                .field("cnrdyie", &self.cnrdyie())
                .field("afsdetie", &self.afsdetie())
                .field("lfsdetie", &self.lfsdetie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Im {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Im {{ ovrudrie: {=bool:?}, mutedetie: {=bool:?}, wckcfgie: {=bool:?}, freqie: {=bool:?}, cnrdyie: {=bool:?}, afsdetie: {=bool:?}, lfsdetie: {=bool:?} }}",
                self.ovrudrie(),
                self.mutedetie(),
                self.wckcfgie(),
                self.freqie(),
                self.cnrdyie(),
                self.afsdetie(),
                self.lfsdetie()
            )
        }
    }
    #[doc = "SAI PDM control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdmcr(pub u32);
    impl Pdmcr {
        #[doc = "PDM enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pdmen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PDM enable."]
        #[inline(always)]
        pub const fn set_pdmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Number of microphones."]
        #[must_use]
        #[inline(always)]
        pub const fn micnbr(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Number of microphones."]
        #[inline(always)]
        pub const fn set_micnbr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Clock enable of bitstream clock number 1."]
        #[must_use]
        #[inline(always)]
        pub const fn cken(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 8usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Clock enable of bitstream clock number 1."]
        #[inline(always)]
        pub const fn set_cken(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 8usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Pdmcr {
        #[inline(always)]
        fn default() -> Pdmcr {
            Pdmcr(0)
        }
    }
    impl core::fmt::Debug for Pdmcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdmcr")
                .field("pdmen", &self.pdmen())
                .field("micnbr", &self.micnbr())
                .field("cken[0]", &self.cken(0usize))
                .field("cken[1]", &self.cken(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pdmcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pdmcr {{ pdmen: {=bool:?}, micnbr: {=u8:?}, cken[0]: {=bool:?}, cken[1]: {=bool:?} }}",
                self.pdmen(),
                self.micnbr(),
                self.cken(0usize),
                self.cken(1usize)
            )
        }
    }
    #[doc = "SAI PDM delay register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdmdly(pub u32);
    impl Pdmdly {
        #[doc = "Delay line adjust for first microphone of pair 1."]
        #[must_use]
        #[inline(always)]
        pub const fn dlyml(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0x07;
            val as u8
        }
        #[doc = "Delay line adjust for first microphone of pair 1."]
        #[inline(always)]
        pub const fn set_dlyml(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val as u32) & 0x07) << offs);
        }
        #[doc = "Delay line adjust for second microphone of pair 1."]
        #[must_use]
        #[inline(always)]
        pub const fn dlymr(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 4usize + n * 8usize;
            let val = (self.0 >> offs) & 0x07;
            val as u8
        }
        #[doc = "Delay line adjust for second microphone of pair 1."]
        #[inline(always)]
        pub const fn set_dlymr(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 4usize + n * 8usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val as u32) & 0x07) << offs);
        }
    }
    impl Default for Pdmdly {
        #[inline(always)]
        fn default() -> Pdmdly {
            Pdmdly(0)
        }
    }
    impl core::fmt::Debug for Pdmdly {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pdmdly")
                .field("dlyml[0]", &self.dlyml(0usize))
                .field("dlyml[1]", &self.dlyml(1usize))
                .field("dlyml[2]", &self.dlyml(2usize))
                .field("dlyml[3]", &self.dlyml(3usize))
                .field("dlymr[0]", &self.dlymr(0usize))
                .field("dlymr[1]", &self.dlymr(1usize))
                .field("dlymr[2]", &self.dlymr(2usize))
                .field("dlymr[3]", &self.dlymr(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pdmdly {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pdmdly {{ dlyml[0]: {=u8:?}, dlyml[1]: {=u8:?}, dlyml[2]: {=u8:?}, dlyml[3]: {=u8:?}, dlymr[0]: {=u8:?}, dlymr[1]: {=u8:?}, dlymr[2]: {=u8:?}, dlymr[3]: {=u8:?} }}",
                self.dlyml(0usize),
                self.dlyml(1usize),
                self.dlyml(2usize),
                self.dlyml(3usize),
                self.dlymr(0usize),
                self.dlymr(1usize),
                self.dlymr(2usize),
                self.dlymr(3usize)
            )
        }
    }
    #[doc = "SAI slot register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Slotr(pub u32);
    impl Slotr {
        #[doc = "First bit offset."]
        #[must_use]
        #[inline(always)]
        pub const fn fboff(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "First bit offset."]
        #[inline(always)]
        pub const fn set_fboff(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Slot size."]
        #[must_use]
        #[inline(always)]
        pub const fn slotsz(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Slot size."]
        #[inline(always)]
        pub const fn set_slotsz(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "Number of slots in an audio frame."]
        #[must_use]
        #[inline(always)]
        pub const fn nbslot(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Number of slots in an audio frame."]
        #[inline(always)]
        pub const fn set_nbslot(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Slot enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sloten(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Slot enable."]
        #[inline(always)]
        pub const fn set_sloten(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Slotr {
        #[inline(always)]
        fn default() -> Slotr {
            Slotr(0)
        }
    }
    impl core::fmt::Debug for Slotr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Slotr")
                .field("fboff", &self.fboff())
                .field("slotsz", &self.slotsz())
                .field("nbslot", &self.nbslot())
                .field("sloten", &self.sloten())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Slotr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Slotr {{ fboff: {=u8:?}, slotsz: {=u8:?}, nbslot: {=u8:?}, sloten: {=u16:?} }}",
                self.fboff(),
                self.slotsz(),
                self.nbslot(),
                self.sloten()
            )
        }
    }
    #[doc = "SAI status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Overrun / underrun."]
        #[must_use]
        #[inline(always)]
        pub const fn ovrudr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun / underrun."]
        #[inline(always)]
        pub const fn set_ovrudr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Mute detection."]
        #[must_use]
        #[inline(always)]
        pub const fn mutedet(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Mute detection."]
        #[inline(always)]
        pub const fn set_mutedet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Wrong clock configuration flag."]
        #[must_use]
        #[inline(always)]
        pub const fn wckcfg(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Wrong clock configuration flag."]
        #[inline(always)]
        pub const fn set_wckcfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "FIFO request."]
        #[must_use]
        #[inline(always)]
        pub const fn freq(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO request."]
        #[inline(always)]
        pub const fn set_freq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Codec not ready."]
        #[must_use]
        #[inline(always)]
        pub const fn cnrdy(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Codec not ready."]
        #[inline(always)]
        pub const fn set_cnrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Anticipated frame synchronization detection."]
        #[must_use]
        #[inline(always)]
        pub const fn afsdet(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Anticipated frame synchronization detection."]
        #[inline(always)]
        pub const fn set_afsdet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Late frame synchronization detection."]
        #[must_use]
        #[inline(always)]
        pub const fn lfsdet(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Late frame synchronization detection."]
        #[inline(always)]
        pub const fn set_lfsdet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "FIFO level threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn flvl(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "FIFO level threshold."]
        #[inline(always)]
        pub const fn set_flvl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
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
                .field("ovrudr", &self.ovrudr())
                .field("mutedet", &self.mutedet())
                .field("wckcfg", &self.wckcfg())
                .field("freq", &self.freq())
                .field("cnrdy", &self.cnrdy())
                .field("afsdet", &self.afsdet())
                .field("lfsdet", &self.lfsdet())
                .field("flvl", &self.flvl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ ovrudr: {=bool:?}, mutedet: {=bool:?}, wckcfg: {=bool:?}, freq: {=bool:?}, cnrdy: {=bool:?}, afsdet: {=bool:?}, lfsdet: {=bool:?}, flvl: {=u8:?} }}",
                self.ovrudr(),
                self.mutedet(),
                self.wckcfg(),
                self.freq(),
                self.cnrdy(),
                self.afsdet(),
                self.lfsdet(),
                self.flvl()
            )
        }
    }
}
