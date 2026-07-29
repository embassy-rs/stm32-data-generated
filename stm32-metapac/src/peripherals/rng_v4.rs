#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Random number generator"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rng {
    ptr: *mut u8,
}
unsafe impl Send for Rng {}
unsafe impl Sync for Rng {}
impl Rng {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "data register"]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "noise source control register."]
    #[inline(always)]
    pub const fn nscr(self) -> crate::common::Reg<regs::Nscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "health test control register"]
    #[inline(always)]
    pub const fn htcr(self, n: usize) -> crate::common::Reg<regs::Htcr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize + n * 4usize) as _) }
    }
    #[doc = "health test status register 0."]
    #[inline(always)]
    pub const fn htsr0(self) -> crate::common::Reg<regs::Htsr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "health test status register 1."]
    #[inline(always)]
    pub const fn htsr1(self) -> crate::common::Reg<regs::Htsr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "noise source mask register."]
    #[inline(always)]
    pub const fn nsmr(self) -> crate::common::Reg<regs::Nsmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
}
pub mod regs {
    #[doc = "control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Random number generator enable"]
        #[must_use]
        #[inline(always)]
        pub const fn rngen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Random number generator enable"]
        #[inline(always)]
        pub const fn set_rngen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt enable"]
        #[inline(always)]
        pub const fn set_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Clock error detection"]
        #[must_use]
        #[inline(always)]
        pub const fn ced(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Clock error detection"]
        #[inline(always)]
        pub const fn set_ced(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Auto reset disable"]
        #[must_use]
        #[inline(always)]
        pub const fn ardis(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Auto reset disable"]
        #[inline(always)]
        pub const fn set_ardis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "configuration 3"]
        #[must_use]
        #[inline(always)]
        pub const fn rng_config3(&self) -> super::vals::RngConfig3 {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::RngConfig3::from_bits(val as u8)
        }
        #[doc = "configuration 3"]
        #[inline(always)]
        pub const fn set_rng_config3(&mut self, val: super::vals::RngConfig3) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
        }
        #[doc = "NIST custom"]
        #[must_use]
        #[inline(always)]
        pub const fn nistc(&self) -> super::vals::Nistc {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Nistc::from_bits(val as u8)
        }
        #[doc = "NIST custom"]
        #[inline(always)]
        pub const fn set_nistc(&mut self, val: super::vals::Nistc) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "configuration 2"]
        #[must_use]
        #[inline(always)]
        pub const fn rng_config2(&self) -> super::vals::RngConfig2 {
            let val = (self.0 >> 13usize) & 0x07;
            super::vals::RngConfig2::from_bits(val as u8)
        }
        #[doc = "configuration 2"]
        #[inline(always)]
        pub const fn set_rng_config2(&mut self, val: super::vals::RngConfig2) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val.to_bits() as u32) & 0x07) << 13usize);
        }
        #[doc = "Clock divider factor"]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> super::vals::Clkdiv {
            let val = (self.0 >> 16usize) & 0x0f;
            super::vals::Clkdiv::from_bits(val as u8)
        }
        #[doc = "Clock divider factor"]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: super::vals::Clkdiv) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val.to_bits() as u32) & 0x0f) << 16usize);
        }
        #[doc = "configuration 1"]
        #[must_use]
        #[inline(always)]
        pub const fn rng_config1(&self) -> super::vals::RngConfig1 {
            let val = (self.0 >> 20usize) & 0xff;
            super::vals::RngConfig1::from_bits(val as u8)
        }
        #[doc = "configuration 1"]
        #[inline(always)]
        pub const fn set_rng_config1(&mut self, val: super::vals::RngConfig1) {
            self.0 = (self.0 & !(0xff << 20usize)) | (((val.to_bits() as u32) & 0xff) << 20usize);
        }
        #[doc = "Conditioning soft reset"]
        #[must_use]
        #[inline(always)]
        pub const fn condrst(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Conditioning soft reset"]
        #[inline(always)]
        pub const fn set_condrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Config Lock"]
        #[must_use]
        #[inline(always)]
        pub const fn configlock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Config Lock"]
        #[inline(always)]
        pub const fn set_configlock(&mut self, val: bool) {
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
                .field("rngen", &self.rngen())
                .field("ie", &self.ie())
                .field("ced", &self.ced())
                .field("ardis", &self.ardis())
                .field("rng_config3", &self.rng_config3())
                .field("nistc", &self.nistc())
                .field("rng_config2", &self.rng_config2())
                .field("clkdiv", &self.clkdiv())
                .field("rng_config1", &self.rng_config1())
                .field("condrst", &self.condrst())
                .field("configlock", &self.configlock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ rngen: {=bool:?}, ie: {=bool:?}, ced: {=bool:?}, ardis: {=bool:?}, rng_config3: {:?}, nistc: {:?}, rng_config2: {:?}, clkdiv: {:?}, rng_config1: {:?}, condrst: {=bool:?}, configlock: {=bool:?} }}",
                self.rngen(),
                self.ie(),
                self.ced(),
                self.ardis(),
                self.rng_config3(),
                self.nistc(),
                self.rng_config2(),
                self.clkdiv(),
                self.rng_config1(),
                self.condrst(),
                self.configlock()
            )
        }
    }
    #[doc = "Health test control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Htcr(pub u32);
    impl Htcr {
        #[doc = "Health test configuration"]
        #[must_use]
        #[inline(always)]
        pub const fn htcfg(&self) -> super::vals::Htcfg {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            super::vals::Htcfg::from_bits(val as u32)
        }
        #[doc = "Health test configuration"]
        #[inline(always)]
        pub const fn set_htcfg(&mut self, val: super::vals::Htcfg) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Htcr {
        #[inline(always)]
        fn default() -> Htcr {
            Htcr(0)
        }
    }
    impl core::fmt::Debug for Htcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Htcr").field("htcfg", &self.htcfg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Htcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Htcr {{ htcfg: {:?} }}", self.htcfg())
        }
    }
    #[doc = "health test status register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Htsr0(pub u32);
    impl Htsr0 {
        #[doc = "Repetitive error after the XOR."]
        #[must_use]
        #[inline(always)]
        pub const fn rperrx(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Repetitive error after the XOR."]
        #[inline(always)]
        pub const fn set_rperrx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Repetitive error for oscillator i."]
        #[must_use]
        #[inline(always)]
        pub const fn rperr(&self, n: usize) -> bool {
            assert!(n < 9usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Repetitive error for oscillator i."]
        #[inline(always)]
        pub const fn set_rperr(&mut self, n: usize, val: bool) {
            assert!(n < 9usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Htsr0 {
        #[inline(always)]
        fn default() -> Htsr0 {
            Htsr0(0)
        }
    }
    impl core::fmt::Debug for Htsr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Htsr0")
                .field("rperrx", &self.rperrx())
                .field("rperr[0]", &self.rperr(0usize))
                .field("rperr[1]", &self.rperr(1usize))
                .field("rperr[2]", &self.rperr(2usize))
                .field("rperr[3]", &self.rperr(3usize))
                .field("rperr[4]", &self.rperr(4usize))
                .field("rperr[5]", &self.rperr(5usize))
                .field("rperr[6]", &self.rperr(6usize))
                .field("rperr[7]", &self.rperr(7usize))
                .field("rperr[8]", &self.rperr(8usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Htsr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Htsr0 {{ rperrx: {=bool:?}, rperr[0]: {=bool:?}, rperr[1]: {=bool:?}, rperr[2]: {=bool:?}, rperr[3]: {=bool:?}, rperr[4]: {=bool:?}, rperr[5]: {=bool:?}, rperr[6]: {=bool:?}, rperr[7]: {=bool:?}, rperr[8]: {=bool:?} }}",
                self.rperrx(),
                self.rperr(0usize),
                self.rperr(1usize),
                self.rperr(2usize),
                self.rperr(3usize),
                self.rperr(4usize),
                self.rperr(5usize),
                self.rperr(6usize),
                self.rperr(7usize),
                self.rperr(8usize)
            )
        }
    }
    #[doc = "health test status register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Htsr1(pub u32);
    impl Htsr1 {
        #[doc = "Adaptative error after the XOR."]
        #[must_use]
        #[inline(always)]
        pub const fn aderrx(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Adaptative error after the XOR."]
        #[inline(always)]
        pub const fn set_aderrx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Adaptative error for oscillator i."]
        #[must_use]
        #[inline(always)]
        pub const fn aderr(&self, n: usize) -> bool {
            assert!(n < 9usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Adaptative error for oscillator i."]
        #[inline(always)]
        pub const fn set_aderr(&mut self, n: usize, val: bool) {
            assert!(n < 9usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Htsr1 {
        #[inline(always)]
        fn default() -> Htsr1 {
            Htsr1(0)
        }
    }
    impl core::fmt::Debug for Htsr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Htsr1")
                .field("aderrx", &self.aderrx())
                .field("aderr[0]", &self.aderr(0usize))
                .field("aderr[1]", &self.aderr(1usize))
                .field("aderr[2]", &self.aderr(2usize))
                .field("aderr[3]", &self.aderr(3usize))
                .field("aderr[4]", &self.aderr(4usize))
                .field("aderr[5]", &self.aderr(5usize))
                .field("aderr[6]", &self.aderr(6usize))
                .field("aderr[7]", &self.aderr(7usize))
                .field("aderr[8]", &self.aderr(8usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Htsr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Htsr1 {{ aderrx: {=bool:?}, aderr[0]: {=bool:?}, aderr[1]: {=bool:?}, aderr[2]: {=bool:?}, aderr[3]: {=bool:?}, aderr[4]: {=bool:?}, aderr[5]: {=bool:?}, aderr[6]: {=bool:?}, aderr[7]: {=bool:?}, aderr[8]: {=bool:?} }}",
                self.aderrx(),
                self.aderr(0usize),
                self.aderr(1usize),
                self.aderr(2usize),
                self.aderr(3usize),
                self.aderr(4usize),
                self.aderr(5usize),
                self.aderr(6usize),
                self.aderr(7usize),
                self.aderr(8usize)
            )
        }
    }
    #[doc = "noise source control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nscr(pub u32);
    impl Nscr {
        #[doc = "When the RNG is enabled (RNGEN bit set), each bit of this bitfield enables one of the three inputs from the oscillator instance number x."]
        #[must_use]
        #[inline(always)]
        pub const fn en_osc(&self, n: usize) -> u8 {
            assert!(n < 3usize);
            let offs = 0usize + n * 3usize;
            let val = (self.0 >> offs) & 0x07;
            val as u8
        }
        #[doc = "When the RNG is enabled (RNGEN bit set), each bit of this bitfield enables one of the three inputs from the oscillator instance number x."]
        #[inline(always)]
        pub const fn set_en_osc(&mut self, n: usize, val: u8) {
            assert!(n < 3usize);
            let offs = 0usize + n * 3usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val as u32) & 0x07) << offs);
        }
    }
    impl Default for Nscr {
        #[inline(always)]
        fn default() -> Nscr {
            Nscr(0)
        }
    }
    impl core::fmt::Debug for Nscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nscr")
                .field("en_osc[0]", &self.en_osc(0usize))
                .field("en_osc[1]", &self.en_osc(1usize))
                .field("en_osc[2]", &self.en_osc(2usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Nscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Nscr {{ en_osc[0]: {=u8:?}, en_osc[1]: {=u8:?}, en_osc[2]: {=u8:?} }}",
                self.en_osc(0usize),
                self.en_osc(1usize),
                self.en_osc(2usize)
            )
        }
    }
    #[doc = "noise source mask register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nsmr(pub u32);
    impl Nsmr {
        #[doc = "Mask oscillator i."]
        #[must_use]
        #[inline(always)]
        pub const fn mosc(&self, n: usize) -> bool {
            assert!(n < 9usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Mask oscillator i."]
        #[inline(always)]
        pub const fn set_mosc(&mut self, n: usize, val: bool) {
            assert!(n < 9usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Nsmr {
        #[inline(always)]
        fn default() -> Nsmr {
            Nsmr(0)
        }
    }
    impl core::fmt::Debug for Nsmr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nsmr")
                .field("mosc[0]", &self.mosc(0usize))
                .field("mosc[1]", &self.mosc(1usize))
                .field("mosc[2]", &self.mosc(2usize))
                .field("mosc[3]", &self.mosc(3usize))
                .field("mosc[4]", &self.mosc(4usize))
                .field("mosc[5]", &self.mosc(5usize))
                .field("mosc[6]", &self.mosc(6usize))
                .field("mosc[7]", &self.mosc(7usize))
                .field("mosc[8]", &self.mosc(8usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Nsmr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Nsmr {{ mosc[0]: {=bool:?}, mosc[1]: {=bool:?}, mosc[2]: {=bool:?}, mosc[3]: {=bool:?}, mosc[4]: {=bool:?}, mosc[5]: {=bool:?}, mosc[6]: {=bool:?}, mosc[7]: {=bool:?}, mosc[8]: {=bool:?} }}",
                self.mosc(0usize),
                self.mosc(1usize),
                self.mosc(2usize),
                self.mosc(3usize),
                self.mosc(4usize),
                self.mosc(5usize),
                self.mosc(6usize),
                self.mosc(7usize),
                self.mosc(8usize)
            )
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Data ready"]
        #[must_use]
        #[inline(always)]
        pub const fn drdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Data ready"]
        #[inline(always)]
        pub const fn set_drdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clock error current status"]
        #[must_use]
        #[inline(always)]
        pub const fn cecs(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clock error current status"]
        #[inline(always)]
        pub const fn set_cecs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Seed error current status"]
        #[must_use]
        #[inline(always)]
        pub const fn secs(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Seed error current status"]
        #[inline(always)]
        pub const fn set_secs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Busy status"]
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Busy status"]
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Clock error interrupt status"]
        #[must_use]
        #[inline(always)]
        pub const fn ceis(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Clock error interrupt status"]
        #[inline(always)]
        pub const fn set_ceis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Seed error interrupt status"]
        #[must_use]
        #[inline(always)]
        pub const fn seis(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Seed error interrupt status"]
        #[inline(always)]
        pub const fn set_seis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
                .field("drdy", &self.drdy())
                .field("cecs", &self.cecs())
                .field("secs", &self.secs())
                .field("busy", &self.busy())
                .field("ceis", &self.ceis())
                .field("seis", &self.seis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ drdy: {=bool:?}, cecs: {=bool:?}, secs: {=bool:?}, busy: {=bool:?}, ceis: {=bool:?}, seis: {=bool:?} }}",
                self.drdy(),
                self.cecs(),
                self.secs(),
                self.busy(),
                self.ceis(),
                self.seis()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Clkdiv {
        #[doc = "Internal RNG clock after divider is similar to incoming RNG clock"]
        NoDiv = 0x0,
        #[doc = "Divide RNG clock by 2^1"]
        Div21 = 0x01,
        #[doc = "Divide RNG clock by 2^2"]
        Div22 = 0x02,
        #[doc = "Divide RNG clock by 2^3"]
        Div23 = 0x03,
        #[doc = "Divide RNG clock by 2^4"]
        Div24 = 0x04,
        #[doc = "Divide RNG clock by 2^5"]
        Div25 = 0x05,
        #[doc = "Divide RNG clock by 2^6"]
        Div26 = 0x06,
        #[doc = "Divide RNG clock by 2^7"]
        Div27 = 0x07,
        #[doc = "Divide RNG clock by 2^8"]
        Div28 = 0x08,
        #[doc = "Divide RNG clock by 2^9"]
        Div29 = 0x09,
        #[doc = "Divide RNG clock by 2^10"]
        Div210 = 0x0a,
        #[doc = "Divide RNG clock by 2^11"]
        Div211 = 0x0b,
        #[doc = "Divide RNG clock by 2^12"]
        Div212 = 0x0c,
        #[doc = "Divide RNG clock by 2^13"]
        Div213 = 0x0d,
        #[doc = "Divide RNG clock by 2^14"]
        Div214 = 0x0e,
        #[doc = "Divide RNG clock by 2^15"]
        Div215 = 0x0f,
    }
    impl Clkdiv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Clkdiv {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Clkdiv {
        #[inline(always)]
        fn from(val: u8) -> Clkdiv {
            Clkdiv::from_bits(val)
        }
    }
    impl From<Clkdiv> for u8 {
        #[inline(always)]
        fn from(val: Clkdiv) -> u8 {
            Clkdiv::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Htcfg(u32);
    impl Htcfg {
        #[doc = "Recommended value for RNG certification (0x0000_AAC7)"]
        pub const Recommended: Self = Self(0xaac7);
    }
    impl Htcfg {
        pub const fn from_bits(val: u32) -> Htcfg {
            Self(val & 0xffff_ffff)
        }
        pub const fn to_bits(self) -> u32 {
            self.0
        }
    }
    impl core::fmt::Debug for Htcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0xaac7 => f.write_str("Recommended"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Htcfg {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0xaac7 => defmt::write!(f, "Recommended"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u32> for Htcfg {
        #[inline(always)]
        fn from(val: u32) -> Htcfg {
            Htcfg::from_bits(val)
        }
    }
    impl From<Htcfg> for u32 {
        #[inline(always)]
        fn from(val: Htcfg) -> u32 {
            Htcfg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Nistc {
        #[doc = "Hardware default values for NIST compliant RNG"]
        Default = 0x0,
        #[doc = "Custom values for NIST compliant RNG"]
        Custom = 0x01,
    }
    impl Nistc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Nistc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Nistc {
        #[inline(always)]
        fn from(val: u8) -> Nistc {
            Nistc::from_bits(val)
        }
    }
    impl From<Nistc> for u8 {
        #[inline(always)]
        fn from(val: Nistc) -> u8 {
            Nistc::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct RngConfig1(u8);
    impl RngConfig1 {
        #[doc = "Recommended value for config B (not NIST certifiable)"]
        pub const ConfigB: Self = Self(0x83);
        #[doc = "Recommended value for config A (NIST certifiable) and C (not NIST certifiable)"]
        pub const ConfigAC: Self = Self(0x84);
    }
    impl RngConfig1 {
        pub const fn from_bits(val: u8) -> RngConfig1 {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for RngConfig1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x83 => f.write_str("ConfigB"),
                0x84 => f.write_str("ConfigAC"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for RngConfig1 {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x83 => defmt::write!(f, "ConfigB"),
                0x84 => defmt::write!(f, "ConfigAC"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for RngConfig1 {
        #[inline(always)]
        fn from(val: u8) -> RngConfig1 {
            RngConfig1::from_bits(val)
        }
    }
    impl From<RngConfig1> for u8 {
        #[inline(always)]
        fn from(val: RngConfig1) -> u8 {
            RngConfig1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum RngConfig2 {
        #[doc = "Recommended value for config A, B and C"]
        Recommended = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl RngConfig2 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> RngConfig2 {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for RngConfig2 {
        #[inline(always)]
        fn from(val: u8) -> RngConfig2 {
            RngConfig2::from_bits(val)
        }
    }
    impl From<RngConfig2> for u8 {
        #[inline(always)]
        fn from(val: RngConfig2) -> u8 {
            RngConfig2::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum RngConfig3 {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        #[doc = "Recommended value for config A (NIST certifiable) and B, C (not NIST certifiable)"]
        Recommended = 0x0f,
    }
    impl RngConfig3 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> RngConfig3 {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for RngConfig3 {
        #[inline(always)]
        fn from(val: u8) -> RngConfig3 {
            RngConfig3::from_bits(val)
        }
    }
    impl From<RngConfig3> for u8 {
        #[inline(always)]
        fn from(val: RngConfig3) -> u8 {
            RngConfig3::to_bits(val)
        }
    }
}
