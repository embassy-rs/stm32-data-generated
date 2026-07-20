#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Low-power timer."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lptim {
    ptr: *mut u8,
}
unsafe impl Send for Lptim {}
unsafe impl Sync for Lptim {}
impl Lptim {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "LPTIM2 interrupt and status register \\[alternate\\]."]
    #[inline(always)]
    pub const fn isr_input(self) -> crate::common::Reg<regs::IsrInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "LPTIM2 interrupt and status register \\[alternate\\]."]
    #[inline(always)]
    pub const fn isr_output(self) -> crate::common::Reg<regs::IsrOutput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "LPTIM2 interrupt clear register \\[alternate\\]."]
    #[inline(always)]
    pub const fn icr_input(self) -> crate::common::Reg<regs::IcrInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "LPTIM2 interrupt clear register \\[alternate\\]."]
    #[inline(always)]
    pub const fn icr_output(self) -> crate::common::Reg<regs::IcrOutput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "LPTIM1 interrupt enable register \\[alternate\\]."]
    #[inline(always)]
    pub const fn dier_input(self) -> crate::common::Reg<regs::DierInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "LPTIM2 interrupt enable register \\[alternate\\]."]
    #[inline(always)]
    pub const fn dier_output(self) -> crate::common::Reg<regs::DierOutput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "LPTIM2 configuration register."]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "LPTIM2 control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "LPTIM2 compare register 1."]
    #[inline(always)]
    pub const fn ccr(self, n: usize) -> crate::common::Reg<regs::Ccr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize + n * 32usize) as _) }
    }
    #[doc = "LPTIM2 autoreload register."]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::Arr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "LPTIM2 counter register."]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::Cnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "LPTIM2 configuration register 2."]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "LPTIM2 repetition register."]
    #[inline(always)]
    pub const fn rcr(self) -> crate::common::Reg<regs::Rcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "LPTIM2 capture/compare mode register 1."]
    #[inline(always)]
    pub const fn ccmr1(self) -> crate::common::Reg<regs::Ccmr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
}
pub mod regs {
    #[doc = "LPTIM2 autoreload register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Arr(pub u32);
    impl Arr {
        #[doc = "Auto reload value."]
        #[must_use]
        #[inline(always)]
        pub const fn arr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Auto reload value."]
        #[inline(always)]
        pub const fn set_arr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Arr {
        #[inline(always)]
        fn default() -> Arr {
            Arr(0)
        }
    }
    impl core::fmt::Debug for Arr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Arr").field("arr", &self.arr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Arr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Arr {{ arr: {=u16:?} }}", self.arr())
        }
    }
    #[doc = "LPTIM2 capture/compare mode register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccmr1(pub u32);
    impl Ccmr1 {
        #[doc = "Capture/compare 1 selection."]
        #[must_use]
        #[inline(always)]
        pub const fn ccsel(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 16usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 1 selection."]
        #[inline(always)]
        pub const fn set_ccsel(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 16usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/compare 1 output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cce(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 1usize + n * 16usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 1 output enable."]
        #[inline(always)]
        pub const fn set_cce(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 1usize + n * 16usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/compare 1 output polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn ccp(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 2usize + n * 16usize;
            let val = (self.0 >> offs) & 0x03;
            val as u8
        }
        #[doc = "Capture/compare 1 output polarity."]
        #[inline(always)]
        pub const fn set_ccp(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 2usize + n * 16usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
        }
        #[doc = "Input capture 1 prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn icpsc(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 8usize + n * 16usize;
            let val = (self.0 >> offs) & 0x03;
            val as u8
        }
        #[doc = "Input capture 1 prescaler."]
        #[inline(always)]
        pub const fn set_icpsc(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 8usize + n * 16usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
        }
        #[doc = "Input capture 1 filter."]
        #[must_use]
        #[inline(always)]
        pub const fn icf(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 12usize + n * 16usize;
            let val = (self.0 >> offs) & 0x03;
            val as u8
        }
        #[doc = "Input capture 1 filter."]
        #[inline(always)]
        pub const fn set_icf(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 12usize + n * 16usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
        }
    }
    impl Default for Ccmr1 {
        #[inline(always)]
        fn default() -> Ccmr1 {
            Ccmr1(0)
        }
    }
    impl core::fmt::Debug for Ccmr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccmr1")
                .field("ccsel[0]", &self.ccsel(0usize))
                .field("ccsel[1]", &self.ccsel(1usize))
                .field("cce[0]", &self.cce(0usize))
                .field("cce[1]", &self.cce(1usize))
                .field("ccp[0]", &self.ccp(0usize))
                .field("ccp[1]", &self.ccp(1usize))
                .field("icpsc[0]", &self.icpsc(0usize))
                .field("icpsc[1]", &self.icpsc(1usize))
                .field("icf[0]", &self.icf(0usize))
                .field("icf[1]", &self.icf(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccmr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccmr1 {{ ccsel[0]: {=bool:?}, ccsel[1]: {=bool:?}, cce[0]: {=bool:?}, cce[1]: {=bool:?}, ccp[0]: {=u8:?}, ccp[1]: {=u8:?}, icpsc[0]: {=u8:?}, icpsc[1]: {=u8:?}, icf[0]: {=u8:?}, icf[1]: {=u8:?} }}",
                self.ccsel(0usize),
                self.ccsel(1usize),
                self.cce(0usize),
                self.cce(1usize),
                self.ccp(0usize),
                self.ccp(1usize),
                self.icpsc(0usize),
                self.icpsc(1usize),
                self.icf(0usize),
                self.icf(1usize)
            )
        }
    }
    #[doc = "LPTIM2 compare register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr(pub u32);
    impl Ccr {
        #[doc = "Capture/compare 1 value."]
        #[must_use]
        #[inline(always)]
        pub const fn ccr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Capture/compare 1 value."]
        #[inline(always)]
        pub const fn set_ccr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ccr {
        #[inline(always)]
        fn default() -> Ccr {
            Ccr(0)
        }
    }
    impl core::fmt::Debug for Ccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccr").field("ccr", &self.ccr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ccr {{ ccr: {=u16:?} }}", self.ccr())
        }
    }
    #[doc = "LPTIM2 configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "Clock selector."]
        #[must_use]
        #[inline(always)]
        pub const fn cksel(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clock selector."]
        #[inline(always)]
        pub const fn set_cksel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clock Polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn ckpol(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "Clock Polarity."]
        #[inline(always)]
        pub const fn set_ckpol(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "Configurable digital filter for external clock."]
        #[must_use]
        #[inline(always)]
        pub const fn ckflt(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "Configurable digital filter for external clock."]
        #[inline(always)]
        pub const fn set_ckflt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "Configurable digital filter for trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn trgflt(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Configurable digital filter for trigger."]
        #[inline(always)]
        pub const fn set_trgflt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "Clock prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn presc(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x07;
            val as u8
        }
        #[doc = "Clock prescaler."]
        #[inline(always)]
        pub const fn set_presc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
        }
        #[doc = "Trigger selector."]
        #[must_use]
        #[inline(always)]
        pub const fn trigsel(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "Trigger selector."]
        #[inline(always)]
        pub const fn set_trigsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
        }
        #[doc = "Trigger enable and polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn trigen(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x03;
            val as u8
        }
        #[doc = "Trigger enable and polarity."]
        #[inline(always)]
        pub const fn set_trigen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
        }
        #[doc = "Timeout enable."]
        #[must_use]
        #[inline(always)]
        pub const fn timout(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout enable."]
        #[inline(always)]
        pub const fn set_timout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Waveform shape."]
        #[must_use]
        #[inline(always)]
        pub const fn wave(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Waveform shape."]
        #[inline(always)]
        pub const fn set_wave(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Waveform shape polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn wavpol(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Waveform shape polarity."]
        #[inline(always)]
        pub const fn set_wavpol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Registers update mode."]
        #[must_use]
        #[inline(always)]
        pub const fn preload(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Registers update mode."]
        #[inline(always)]
        pub const fn set_preload(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "counter mode enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn countmode(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "counter mode enabled."]
        #[inline(always)]
        pub const fn set_countmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Encoder mode enable."]
        #[must_use]
        #[inline(always)]
        pub const fn enc(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Encoder mode enable."]
        #[inline(always)]
        pub const fn set_enc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
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
                .field("cksel", &self.cksel())
                .field("ckpol", &self.ckpol())
                .field("ckflt", &self.ckflt())
                .field("trgflt", &self.trgflt())
                .field("presc", &self.presc())
                .field("trigsel", &self.trigsel())
                .field("trigen", &self.trigen())
                .field("timout", &self.timout())
                .field("wave", &self.wave())
                .field("wavpol", &self.wavpol())
                .field("preload", &self.preload())
                .field("countmode", &self.countmode())
                .field("enc", &self.enc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfgr {{ cksel: {=bool:?}, ckpol: {=u8:?}, ckflt: {=u8:?}, trgflt: {=u8:?}, presc: {=u8:?}, trigsel: {=u8:?}, trigen: {=u8:?}, timout: {=bool:?}, wave: {=bool:?}, wavpol: {=bool:?}, preload: {=bool:?}, countmode: {=bool:?}, enc: {=bool:?} }}",
                self.cksel(),
                self.ckpol(),
                self.ckflt(),
                self.trgflt(),
                self.presc(),
                self.trigsel(),
                self.trigen(),
                self.timout(),
                self.wave(),
                self.wavpol(),
                self.preload(),
                self.countmode(),
                self.enc()
            )
        }
    }
    #[doc = "LPTIM2 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "LPTIM input 1 selection."]
        #[must_use]
        #[inline(always)]
        pub const fn insel(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x03;
            val as u8
        }
        #[doc = "LPTIM input 1 selection."]
        #[inline(always)]
        pub const fn set_insel(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
        }
        #[doc = "LPTIM input capture 1 selection."]
        #[must_use]
        #[inline(always)]
        pub const fn icsel(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 16usize + n * 4usize;
            let val = (self.0 >> offs) & 0x03;
            val as u8
        }
        #[doc = "LPTIM input capture 1 selection."]
        #[inline(always)]
        pub const fn set_icsel(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 16usize + n * 4usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
        }
    }
    impl Default for Cfgr2 {
        #[inline(always)]
        fn default() -> Cfgr2 {
            Cfgr2(0)
        }
    }
    impl core::fmt::Debug for Cfgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr2")
                .field("insel[0]", &self.insel(0usize))
                .field("insel[1]", &self.insel(1usize))
                .field("icsel[0]", &self.icsel(0usize))
                .field("icsel[1]", &self.icsel(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfgr2 {{ insel[0]: {=u8:?}, insel[1]: {=u8:?}, icsel[0]: {=u8:?}, icsel[1]: {=u8:?} }}",
                self.insel(0usize),
                self.insel(1usize),
                self.icsel(0usize),
                self.icsel(1usize)
            )
        }
    }
    #[doc = "LPTIM2 counter register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cnt(pub u32);
    impl Cnt {
        #[doc = "Counter value."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Counter value."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cnt {
        #[inline(always)]
        fn default() -> Cnt {
            Cnt(0)
        }
    }
    impl core::fmt::Debug for Cnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cnt").field("cnt", &self.cnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cnt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cnt {{ cnt: {=u16:?} }}", self.cnt())
        }
    }
    #[doc = "LPTIM2 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "LPTIM enable."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM enable."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LPTIM start in Single mode."]
        #[must_use]
        #[inline(always)]
        pub const fn sngstrt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM start in Single mode."]
        #[inline(always)]
        pub const fn set_sngstrt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer start in Continuous mode."]
        #[must_use]
        #[inline(always)]
        pub const fn cntstrt(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Timer start in Continuous mode."]
        #[inline(always)]
        pub const fn set_cntstrt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Counter reset."]
        #[must_use]
        #[inline(always)]
        pub const fn countrst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Counter reset."]
        #[inline(always)]
        pub const fn set_countrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Reset after read enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rstare(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Reset after read enable."]
        #[inline(always)]
        pub const fn set_rstare(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
                .field("enable", &self.enable())
                .field("sngstrt", &self.sngstrt())
                .field("cntstrt", &self.cntstrt())
                .field("countrst", &self.countrst())
                .field("rstare", &self.rstare())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ enable: {=bool:?}, sngstrt: {=bool:?}, cntstrt: {=bool:?}, countrst: {=bool:?}, rstare: {=bool:?} }}",
                self.enable(),
                self.sngstrt(),
                self.cntstrt(),
                self.countrst(),
                self.rstare()
            )
        }
    }
    #[doc = "LPTIM1 interrupt enable register \\[alternate\\]."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DierInput(pub u32);
    impl DierInput {
        #[doc = "Capture/compare 1 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ccie(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 9usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 1 interrupt enable."]
        #[inline(always)]
        pub const fn set_ccie(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 9usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Autoreload match Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn arrmie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Autoreload match Interrupt Enable."]
        #[inline(always)]
        pub const fn set_arrmie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "External trigger valid edge Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn exttrigie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "External trigger valid edge Interrupt Enable."]
        #[inline(always)]
        pub const fn set_exttrigie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Autoreload register update OK Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn arrokie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Autoreload register update OK Interrupt Enable."]
        #[inline(always)]
        pub const fn set_arrokie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Direction change to UP Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn upie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Direction change to UP Interrupt Enable."]
        #[inline(always)]
        pub const fn set_upie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Direction change to down Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn downie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Direction change to down Interrupt Enable."]
        #[inline(always)]
        pub const fn set_downie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Update event interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ueie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Update event interrupt enable."]
        #[inline(always)]
        pub const fn set_ueie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Repetition register update OK interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn repokie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Repetition register update OK interrupt Enable."]
        #[inline(always)]
        pub const fn set_repokie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Capture/compare 1 over-capture interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ccoie(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 12usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 1 over-capture interrupt enable."]
        #[inline(always)]
        pub const fn set_ccoie(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 12usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/compare 1 DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ccde(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 16usize + n * 9usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 1 DMA request enable."]
        #[inline(always)]
        pub const fn set_ccde(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 16usize + n * 9usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Update event DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uede(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Update event DMA request enable."]
        #[inline(always)]
        pub const fn set_uede(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for DierInput {
        #[inline(always)]
        fn default() -> DierInput {
            DierInput(0)
        }
    }
    impl core::fmt::Debug for DierInput {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DierInput")
                .field("ccie[0]", &self.ccie(0usize))
                .field("ccie[1]", &self.ccie(1usize))
                .field("arrmie", &self.arrmie())
                .field("exttrigie", &self.exttrigie())
                .field("arrokie", &self.arrokie())
                .field("upie", &self.upie())
                .field("downie", &self.downie())
                .field("ueie", &self.ueie())
                .field("repokie", &self.repokie())
                .field("ccoie[0]", &self.ccoie(0usize))
                .field("ccoie[1]", &self.ccoie(1usize))
                .field("ccde[0]", &self.ccde(0usize))
                .field("ccde[1]", &self.ccde(1usize))
                .field("uede", &self.uede())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DierInput {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DierInput {{ ccie[0]: {=bool:?}, ccie[1]: {=bool:?}, arrmie: {=bool:?}, exttrigie: {=bool:?}, arrokie: {=bool:?}, upie: {=bool:?}, downie: {=bool:?}, ueie: {=bool:?}, repokie: {=bool:?}, ccoie[0]: {=bool:?}, ccoie[1]: {=bool:?}, ccde[0]: {=bool:?}, ccde[1]: {=bool:?}, uede: {=bool:?} }}",
                self.ccie(0usize),
                self.ccie(1usize),
                self.arrmie(),
                self.exttrigie(),
                self.arrokie(),
                self.upie(),
                self.downie(),
                self.ueie(),
                self.repokie(),
                self.ccoie(0usize),
                self.ccoie(1usize),
                self.ccde(0usize),
                self.ccde(1usize),
                self.uede()
            )
        }
    }
    #[doc = "LPTIM2 interrupt enable register \\[alternate\\]."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DierOutput(pub u32);
    impl DierOutput {
        #[doc = "Capture/compare 1 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ccie(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 9usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 1 interrupt enable."]
        #[inline(always)]
        pub const fn set_ccie(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 9usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Autoreload match Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn arrmie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Autoreload match Interrupt Enable."]
        #[inline(always)]
        pub const fn set_arrmie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "External trigger valid edge Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn exttrigie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "External trigger valid edge Interrupt Enable."]
        #[inline(always)]
        pub const fn set_exttrigie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Compare register 1 update OK interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpokie(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 3usize + n * 16usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Compare register 1 update OK interrupt enable."]
        #[inline(always)]
        pub const fn set_cmpokie(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 3usize + n * 16usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Autoreload register update OK Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn arrokie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Autoreload register update OK Interrupt Enable."]
        #[inline(always)]
        pub const fn set_arrokie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Direction change to UP Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn upie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Direction change to UP Interrupt Enable."]
        #[inline(always)]
        pub const fn set_upie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Direction change to down Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn downie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Direction change to down Interrupt Enable."]
        #[inline(always)]
        pub const fn set_downie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Update event interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ueie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Update event interrupt enable."]
        #[inline(always)]
        pub const fn set_ueie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Repetition register update OK interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn repokie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Repetition register update OK interrupt Enable."]
        #[inline(always)]
        pub const fn set_repokie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Update event DMA request enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uede(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Update event DMA request enable."]
        #[inline(always)]
        pub const fn set_uede(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for DierOutput {
        #[inline(always)]
        fn default() -> DierOutput {
            DierOutput(0)
        }
    }
    impl core::fmt::Debug for DierOutput {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DierOutput")
                .field("ccie[0]", &self.ccie(0usize))
                .field("ccie[1]", &self.ccie(1usize))
                .field("arrmie", &self.arrmie())
                .field("exttrigie", &self.exttrigie())
                .field("cmpokie[0]", &self.cmpokie(0usize))
                .field("cmpokie[1]", &self.cmpokie(1usize))
                .field("arrokie", &self.arrokie())
                .field("upie", &self.upie())
                .field("downie", &self.downie())
                .field("ueie", &self.ueie())
                .field("repokie", &self.repokie())
                .field("uede", &self.uede())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DierOutput {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DierOutput {{ ccie[0]: {=bool:?}, ccie[1]: {=bool:?}, arrmie: {=bool:?}, exttrigie: {=bool:?}, cmpokie[0]: {=bool:?}, cmpokie[1]: {=bool:?}, arrokie: {=bool:?}, upie: {=bool:?}, downie: {=bool:?}, ueie: {=bool:?}, repokie: {=bool:?}, uede: {=bool:?} }}",
                self.ccie(0usize),
                self.ccie(1usize),
                self.arrmie(),
                self.exttrigie(),
                self.cmpokie(0usize),
                self.cmpokie(1usize),
                self.arrokie(),
                self.upie(),
                self.downie(),
                self.ueie(),
                self.repokie(),
                self.uede()
            )
        }
    }
    #[doc = "LPTIM2 interrupt clear register \\[alternate\\]."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IcrInput(pub u32);
    impl IcrInput {
        #[doc = "Capture/compare 1 clear flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cccf(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 9usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 1 clear flag."]
        #[inline(always)]
        pub const fn set_cccf(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 9usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Autoreload match clear flag."]
        #[must_use]
        #[inline(always)]
        pub const fn arrmcf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Autoreload match clear flag."]
        #[inline(always)]
        pub const fn set_arrmcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "External trigger valid edge clear flag."]
        #[must_use]
        #[inline(always)]
        pub const fn exttrigcf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "External trigger valid edge clear flag."]
        #[inline(always)]
        pub const fn set_exttrigcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Autoreload register update OK clear flag."]
        #[must_use]
        #[inline(always)]
        pub const fn arrokcf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Autoreload register update OK clear flag."]
        #[inline(always)]
        pub const fn set_arrokcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Direction change to UP clear flag."]
        #[must_use]
        #[inline(always)]
        pub const fn upcf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Direction change to UP clear flag."]
        #[inline(always)]
        pub const fn set_upcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Direction change to down clear flag."]
        #[must_use]
        #[inline(always)]
        pub const fn downcf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Direction change to down clear flag."]
        #[inline(always)]
        pub const fn set_downcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Update event clear flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uecf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Update event clear flag."]
        #[inline(always)]
        pub const fn set_uecf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Repetition register update OK clear flag."]
        #[must_use]
        #[inline(always)]
        pub const fn repokcf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Repetition register update OK clear flag."]
        #[inline(always)]
        pub const fn set_repokcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Capture/compare 1 over-capture clear flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ccocf(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 12usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 1 over-capture clear flag."]
        #[inline(always)]
        pub const fn set_ccocf(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 12usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Interrupt enable register update OK clear flag."]
        #[must_use]
        #[inline(always)]
        pub const fn dierokcf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt enable register update OK clear flag."]
        #[inline(always)]
        pub const fn set_dierokcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for IcrInput {
        #[inline(always)]
        fn default() -> IcrInput {
            IcrInput(0)
        }
    }
    impl core::fmt::Debug for IcrInput {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IcrInput")
                .field("cccf[0]", &self.cccf(0usize))
                .field("cccf[1]", &self.cccf(1usize))
                .field("arrmcf", &self.arrmcf())
                .field("exttrigcf", &self.exttrigcf())
                .field("arrokcf", &self.arrokcf())
                .field("upcf", &self.upcf())
                .field("downcf", &self.downcf())
                .field("uecf", &self.uecf())
                .field("repokcf", &self.repokcf())
                .field("ccocf[0]", &self.ccocf(0usize))
                .field("ccocf[1]", &self.ccocf(1usize))
                .field("dierokcf", &self.dierokcf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IcrInput {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IcrInput {{ cccf[0]: {=bool:?}, cccf[1]: {=bool:?}, arrmcf: {=bool:?}, exttrigcf: {=bool:?}, arrokcf: {=bool:?}, upcf: {=bool:?}, downcf: {=bool:?}, uecf: {=bool:?}, repokcf: {=bool:?}, ccocf[0]: {=bool:?}, ccocf[1]: {=bool:?}, dierokcf: {=bool:?} }}",
                self.cccf(0usize),
                self.cccf(1usize),
                self.arrmcf(),
                self.exttrigcf(),
                self.arrokcf(),
                self.upcf(),
                self.downcf(),
                self.uecf(),
                self.repokcf(),
                self.ccocf(0usize),
                self.ccocf(1usize),
                self.dierokcf()
            )
        }
    }
    #[doc = "LPTIM2 interrupt clear register \\[alternate\\]."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IcrOutput(pub u32);
    impl IcrOutput {
        #[doc = "Capture/compare 1 clear flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cccf(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 9usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 1 clear flag."]
        #[inline(always)]
        pub const fn set_cccf(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 9usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Autoreload match clear flag."]
        #[must_use]
        #[inline(always)]
        pub const fn arrmcf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Autoreload match clear flag."]
        #[inline(always)]
        pub const fn set_arrmcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "External trigger valid edge clear flag."]
        #[must_use]
        #[inline(always)]
        pub const fn exttrigcf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "External trigger valid edge clear flag."]
        #[inline(always)]
        pub const fn set_exttrigcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Compare register 1 update OK clear flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpokcf(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 3usize + n * 16usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Compare register 1 update OK clear flag."]
        #[inline(always)]
        pub const fn set_cmpokcf(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 3usize + n * 16usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Autoreload register update OK clear flag."]
        #[must_use]
        #[inline(always)]
        pub const fn arrokcf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Autoreload register update OK clear flag."]
        #[inline(always)]
        pub const fn set_arrokcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Direction change to UP clear flag."]
        #[must_use]
        #[inline(always)]
        pub const fn upcf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Direction change to UP clear flag."]
        #[inline(always)]
        pub const fn set_upcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Direction change to down clear flag."]
        #[must_use]
        #[inline(always)]
        pub const fn downcf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Direction change to down clear flag."]
        #[inline(always)]
        pub const fn set_downcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Update event clear flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uecf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Update event clear flag."]
        #[inline(always)]
        pub const fn set_uecf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Repetition register update OK clear flag."]
        #[must_use]
        #[inline(always)]
        pub const fn repokcf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Repetition register update OK clear flag."]
        #[inline(always)]
        pub const fn set_repokcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Interrupt enable register update OK clear flag."]
        #[must_use]
        #[inline(always)]
        pub const fn dierokcf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt enable register update OK clear flag."]
        #[inline(always)]
        pub const fn set_dierokcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for IcrOutput {
        #[inline(always)]
        fn default() -> IcrOutput {
            IcrOutput(0)
        }
    }
    impl core::fmt::Debug for IcrOutput {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IcrOutput")
                .field("cccf[0]", &self.cccf(0usize))
                .field("cccf[1]", &self.cccf(1usize))
                .field("arrmcf", &self.arrmcf())
                .field("exttrigcf", &self.exttrigcf())
                .field("cmpokcf[0]", &self.cmpokcf(0usize))
                .field("cmpokcf[1]", &self.cmpokcf(1usize))
                .field("arrokcf", &self.arrokcf())
                .field("upcf", &self.upcf())
                .field("downcf", &self.downcf())
                .field("uecf", &self.uecf())
                .field("repokcf", &self.repokcf())
                .field("dierokcf", &self.dierokcf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IcrOutput {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IcrOutput {{ cccf[0]: {=bool:?}, cccf[1]: {=bool:?}, arrmcf: {=bool:?}, exttrigcf: {=bool:?}, cmpokcf[0]: {=bool:?}, cmpokcf[1]: {=bool:?}, arrokcf: {=bool:?}, upcf: {=bool:?}, downcf: {=bool:?}, uecf: {=bool:?}, repokcf: {=bool:?}, dierokcf: {=bool:?} }}",
                self.cccf(0usize),
                self.cccf(1usize),
                self.arrmcf(),
                self.exttrigcf(),
                self.cmpokcf(0usize),
                self.cmpokcf(1usize),
                self.arrokcf(),
                self.upcf(),
                self.downcf(),
                self.uecf(),
                self.repokcf(),
                self.dierokcf()
            )
        }
    }
    #[doc = "LPTIM2 interrupt and status register \\[alternate\\]."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IsrInput(pub u32);
    impl IsrInput {
        #[doc = "capture 1 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ccif(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 9usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "capture 1 interrupt flag."]
        #[inline(always)]
        pub const fn set_ccif(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 9usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Autoreload match."]
        #[must_use]
        #[inline(always)]
        pub const fn arrm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Autoreload match."]
        #[inline(always)]
        pub const fn set_arrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "External trigger edge event."]
        #[must_use]
        #[inline(always)]
        pub const fn exttrig(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "External trigger edge event."]
        #[inline(always)]
        pub const fn set_exttrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Autoreload register update OK."]
        #[must_use]
        #[inline(always)]
        pub const fn arrok(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Autoreload register update OK."]
        #[inline(always)]
        pub const fn set_arrok(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Counter direction change down to up."]
        #[must_use]
        #[inline(always)]
        pub const fn up(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Counter direction change down to up."]
        #[inline(always)]
        pub const fn set_up(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Counter direction change up to down."]
        #[must_use]
        #[inline(always)]
        pub const fn down(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Counter direction change up to down."]
        #[inline(always)]
        pub const fn set_down(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LPTIM update event occurred."]
        #[must_use]
        #[inline(always)]
        pub const fn ue(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM update event occurred."]
        #[inline(always)]
        pub const fn set_ue(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Repetition register update OK."]
        #[must_use]
        #[inline(always)]
        pub const fn repok(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Repetition register update OK."]
        #[inline(always)]
        pub const fn set_repok(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Capture 1 over-capture flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ccof(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 12usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture 1 over-capture flag."]
        #[inline(always)]
        pub const fn set_ccof(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 12usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Interrupt enable register update OK."]
        #[must_use]
        #[inline(always)]
        pub const fn dierok(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt enable register update OK."]
        #[inline(always)]
        pub const fn set_dierok(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for IsrInput {
        #[inline(always)]
        fn default() -> IsrInput {
            IsrInput(0)
        }
    }
    impl core::fmt::Debug for IsrInput {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IsrInput")
                .field("ccif[0]", &self.ccif(0usize))
                .field("ccif[1]", &self.ccif(1usize))
                .field("arrm", &self.arrm())
                .field("exttrig", &self.exttrig())
                .field("arrok", &self.arrok())
                .field("up", &self.up())
                .field("down", &self.down())
                .field("ue", &self.ue())
                .field("repok", &self.repok())
                .field("ccof[0]", &self.ccof(0usize))
                .field("ccof[1]", &self.ccof(1usize))
                .field("dierok", &self.dierok())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IsrInput {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IsrInput {{ ccif[0]: {=bool:?}, ccif[1]: {=bool:?}, arrm: {=bool:?}, exttrig: {=bool:?}, arrok: {=bool:?}, up: {=bool:?}, down: {=bool:?}, ue: {=bool:?}, repok: {=bool:?}, ccof[0]: {=bool:?}, ccof[1]: {=bool:?}, dierok: {=bool:?} }}",
                self.ccif(0usize),
                self.ccif(1usize),
                self.arrm(),
                self.exttrig(),
                self.arrok(),
                self.up(),
                self.down(),
                self.ue(),
                self.repok(),
                self.ccof(0usize),
                self.ccof(1usize),
                self.dierok()
            )
        }
    }
    #[doc = "LPTIM2 interrupt and status register \\[alternate\\]."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IsrOutput(pub u32);
    impl IsrOutput {
        #[doc = "Compare 1 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ccif(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 9usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Compare 1 interrupt flag."]
        #[inline(always)]
        pub const fn set_ccif(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 9usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Autoreload match."]
        #[must_use]
        #[inline(always)]
        pub const fn arrm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Autoreload match."]
        #[inline(always)]
        pub const fn set_arrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "External trigger edge event."]
        #[must_use]
        #[inline(always)]
        pub const fn exttrig(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "External trigger edge event."]
        #[inline(always)]
        pub const fn set_exttrig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Compare register 1 update OK."]
        #[must_use]
        #[inline(always)]
        pub const fn cmpok(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 3usize + n * 16usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Compare register 1 update OK."]
        #[inline(always)]
        pub const fn set_cmpok(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 3usize + n * 16usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Autoreload register update OK."]
        #[must_use]
        #[inline(always)]
        pub const fn arrok(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Autoreload register update OK."]
        #[inline(always)]
        pub const fn set_arrok(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Counter direction change down to up."]
        #[must_use]
        #[inline(always)]
        pub const fn up(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Counter direction change down to up."]
        #[inline(always)]
        pub const fn set_up(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Counter direction change up to down."]
        #[must_use]
        #[inline(always)]
        pub const fn down(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Counter direction change up to down."]
        #[inline(always)]
        pub const fn set_down(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LPTIM update event occurred."]
        #[must_use]
        #[inline(always)]
        pub const fn ue(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM update event occurred."]
        #[inline(always)]
        pub const fn set_ue(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Repetition register update OK."]
        #[must_use]
        #[inline(always)]
        pub const fn repok(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Repetition register update OK."]
        #[inline(always)]
        pub const fn set_repok(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Interrupt enable register update OK."]
        #[must_use]
        #[inline(always)]
        pub const fn dierok(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt enable register update OK."]
        #[inline(always)]
        pub const fn set_dierok(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for IsrOutput {
        #[inline(always)]
        fn default() -> IsrOutput {
            IsrOutput(0)
        }
    }
    impl core::fmt::Debug for IsrOutput {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IsrOutput")
                .field("ccif[0]", &self.ccif(0usize))
                .field("ccif[1]", &self.ccif(1usize))
                .field("arrm", &self.arrm())
                .field("exttrig", &self.exttrig())
                .field("cmpok[0]", &self.cmpok(0usize))
                .field("cmpok[1]", &self.cmpok(1usize))
                .field("arrok", &self.arrok())
                .field("up", &self.up())
                .field("down", &self.down())
                .field("ue", &self.ue())
                .field("repok", &self.repok())
                .field("dierok", &self.dierok())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IsrOutput {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IsrOutput {{ ccif[0]: {=bool:?}, ccif[1]: {=bool:?}, arrm: {=bool:?}, exttrig: {=bool:?}, cmpok[0]: {=bool:?}, cmpok[1]: {=bool:?}, arrok: {=bool:?}, up: {=bool:?}, down: {=bool:?}, ue: {=bool:?}, repok: {=bool:?}, dierok: {=bool:?} }}",
                self.ccif(0usize),
                self.ccif(1usize),
                self.arrm(),
                self.exttrig(),
                self.cmpok(0usize),
                self.cmpok(1usize),
                self.arrok(),
                self.up(),
                self.down(),
                self.ue(),
                self.repok(),
                self.dierok()
            )
        }
    }
    #[doc = "LPTIM2 repetition register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rcr(pub u32);
    impl Rcr {
        #[doc = "Repetition register value."]
        #[must_use]
        #[inline(always)]
        pub const fn rep(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Repetition register value."]
        #[inline(always)]
        pub const fn set_rep(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Rcr {
        #[inline(always)]
        fn default() -> Rcr {
            Rcr(0)
        }
    }
    impl core::fmt::Debug for Rcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rcr").field("rep", &self.rep()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rcr {{ rep: {=u8:?} }}", self.rep())
        }
    }
}
