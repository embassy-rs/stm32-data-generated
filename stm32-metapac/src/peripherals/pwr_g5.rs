#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "register block."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwr {
    ptr: *mut u8,
}
unsafe impl Send for Pwr {}
unsafe impl Sync for Pwr {}
impl Pwr {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "PWR low power mode control register."]
    #[inline(always)]
    pub const fn lpmcr(self) -> crate::common::Reg<regs::Lpmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "PWR status register."]
    #[inline(always)]
    pub const fn pmsr(self) -> crate::common::Reg<regs::Pmsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "PWR voltage scaling control register."]
    #[inline(always)]
    pub const fn voscr(self) -> crate::common::Reg<regs::Voscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "PWR voltage scaling status register."]
    #[inline(always)]
    pub const fn vossr(self) -> crate::common::Reg<regs::Vossr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "PWR tamper monitoring control register."]
    #[inline(always)]
    pub const fn tmcr(self) -> crate::common::Reg<regs::Tmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "PWR RTC domain control register."]
    #[inline(always)]
    pub const fn rtccr(self) -> crate::common::Reg<regs::Rtccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "PWR tamper monitoring status register."]
    #[inline(always)]
    pub const fn tmsr(self) -> crate::common::Reg<regs::Tmsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "PWR USB Type-C power delivery register."]
    #[inline(always)]
    pub const fn ucpdr(self) -> crate::common::Reg<regs::Ucpdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "PWR supply configuration control register."]
    #[inline(always)]
    pub const fn sccr(self) -> crate::common::Reg<regs::Sccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "PWR voltage monitor control register."]
    #[inline(always)]
    pub const fn vmcr(self) -> crate::common::Reg<regs::Vmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "PWR USB supply control register."]
    #[inline(always)]
    pub const fn usbscr(self) -> crate::common::Reg<regs::Usbscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "PWR voltage monitor status register."]
    #[inline(always)]
    pub const fn vmsr(self) -> crate::common::Reg<regs::Vmsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "PWR wake-up status clear register."]
    #[inline(always)]
    pub const fn wuscr(self) -> crate::common::Reg<regs::Wuscr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "PWR wake-up status register."]
    #[inline(always)]
    pub const fn wusr(self) -> crate::common::Reg<regs::Wusr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "PWR wake-up configuration register."]
    #[inline(always)]
    pub const fn wucr(self) -> crate::common::Reg<regs::Wucr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "PWR I/O retention register."]
    #[inline(always)]
    pub const fn ioretr(self) -> crate::common::Reg<regs::Ioretr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "PWR security configuration register."]
    #[inline(always)]
    pub const fn seccfgr(self) -> crate::common::Reg<regs::Seccfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "PWR privilege configuration register."]
    #[inline(always)]
    pub const fn privcfgr(self) -> crate::common::Reg<regs::Privcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
}
pub mod regs {
    #[doc = "PWR I/O retention register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ioretr(pub u32);
    impl Ioretr {
        #[doc = "IO retention enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ioreten(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IO retention enable."]
        #[inline(always)]
        pub const fn set_ioreten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "JTAG IOs retention enable."]
        #[must_use]
        #[inline(always)]
        pub const fn jtagioreten(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "JTAG IOs retention enable."]
        #[inline(always)]
        pub const fn set_jtagioreten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Ioretr {
        #[inline(always)]
        fn default() -> Ioretr {
            Ioretr(0)
        }
    }
    impl core::fmt::Debug for Ioretr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ioretr")
                .field("ioreten", &self.ioreten())
                .field("jtagioreten", &self.jtagioreten())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ioretr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ioretr {{ ioreten: {=bool:?}, jtagioreten: {=bool:?} }}",
                self.ioreten(),
                self.jtagioreten()
            )
        }
    }
    #[doc = "PWR low power mode control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lpmcr(pub u32);
    impl Lpmcr {
        #[doc = "low-power mode selection."]
        #[must_use]
        #[inline(always)]
        pub const fn lpms(&self) -> super::vals::Lpms {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Lpms::from_bits(val as u8)
        }
        #[doc = "low-power mode selection."]
        #[inline(always)]
        pub const fn set_lpms(&mut self, val: super::vals::Lpms) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "clear Standby and Stop flags (always read as 0)."]
        #[must_use]
        #[inline(always)]
        pub const fn cssf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "clear Standby and Stop flags (always read as 0)."]
        #[inline(always)]
        pub const fn set_cssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "flash memory low-power mode in Stop mode."]
        #[must_use]
        #[inline(always)]
        pub const fn flps(&self) -> super::vals::Flps {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Flps::from_bits(val as u8)
        }
        #[doc = "flash memory low-power mode in Stop mode."]
        #[inline(always)]
        pub const fn set_flps(&mut self, val: super::vals::Flps) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "AHB SRAM3 power-down in Stop mode."]
        #[must_use]
        #[inline(always)]
        pub const fn sram3pds(&self) -> super::vals::Srampds {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Srampds::from_bits(val as u8)
        }
        #[doc = "AHB SRAM3 power-down in Stop mode."]
        #[inline(always)]
        pub const fn set_sram3pds(&mut self, val: super::vals::Srampds) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "AHB SRAM2 page 1 power-down in Stop mode."]
        #[must_use]
        #[inline(always)]
        pub const fn sram2pds1(&self) -> super::vals::Srampds {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Srampds::from_bits(val as u8)
        }
        #[doc = "AHB SRAM2 page 1 power-down in Stop mode."]
        #[inline(always)]
        pub const fn set_sram2pds1(&mut self, val: super::vals::Srampds) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
        #[doc = "AHB SRAM2 page 2 power-down in Stop mode."]
        #[must_use]
        #[inline(always)]
        pub const fn sram2pds2(&self) -> super::vals::Srampds {
            let val = (self.0 >> 25usize) & 0x01;
            super::vals::Srampds::from_bits(val as u8)
        }
        #[doc = "AHB SRAM2 page 2 power-down in Stop mode."]
        #[inline(always)]
        pub const fn set_sram2pds2(&mut self, val: super::vals::Srampds) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
        }
        #[doc = "AHB SRAM1 power-down in Stop mode."]
        #[must_use]
        #[inline(always)]
        pub const fn sram1pds(&self) -> super::vals::Srampds {
            let val = (self.0 >> 26usize) & 0x01;
            super::vals::Srampds::from_bits(val as u8)
        }
        #[doc = "AHB SRAM1 power-down in Stop mode."]
        #[inline(always)]
        pub const fn set_sram1pds(&mut self, val: super::vals::Srampds) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Lpmcr {
        #[inline(always)]
        fn default() -> Lpmcr {
            Lpmcr(0)
        }
    }
    impl core::fmt::Debug for Lpmcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lpmcr")
                .field("lpms", &self.lpms())
                .field("cssf", &self.cssf())
                .field("flps", &self.flps())
                .field("sram3pds", &self.sram3pds())
                .field("sram2pds1", &self.sram2pds1())
                .field("sram2pds2", &self.sram2pds2())
                .field("sram1pds", &self.sram1pds())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lpmcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Lpmcr {{ lpms: {:?}, cssf: {=bool:?}, flps: {:?}, sram3pds: {:?}, sram2pds1: {:?}, sram2pds2: {:?}, sram1pds: {:?} }}",
                self.lpms(),
                self.cssf(),
                self.flps(),
                self.sram3pds(),
                self.sram2pds1(),
                self.sram2pds2(),
                self.sram1pds()
            )
        }
    }
    #[doc = "PWR status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pmsr(pub u32);
    impl Pmsr {
        #[doc = "Stop flag."]
        #[must_use]
        #[inline(always)]
        pub const fn stopf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Stop flag."]
        #[inline(always)]
        pub const fn set_stopf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "System standby flag."]
        #[must_use]
        #[inline(always)]
        pub const fn sbf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "System standby flag."]
        #[inline(always)]
        pub const fn set_sbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Pmsr {
        #[inline(always)]
        fn default() -> Pmsr {
            Pmsr(0)
        }
    }
    impl core::fmt::Debug for Pmsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pmsr")
                .field("stopf", &self.stopf())
                .field("sbf", &self.sbf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pmsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pmsr {{ stopf: {=bool:?}, sbf: {=bool:?} }}",
                self.stopf(),
                self.sbf()
            )
        }
    }
    #[doc = "PWR privilege configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcfgr(pub u32);
    impl Privcfgr {
        #[doc = "PWR secure functions privilege configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn spriv(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PWR secure functions privilege configuration."]
        #[inline(always)]
        pub const fn set_spriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "PWR nonsecure functions privilege configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn nspriv(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "PWR nonsecure functions privilege configuration."]
        #[inline(always)]
        pub const fn set_nspriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
                .field("spriv", &self.spriv())
                .field("nspriv", &self.nspriv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Privcfgr {{ spriv: {=bool:?}, nspriv: {=bool:?} }}",
                self.spriv(),
                self.nspriv()
            )
        }
    }
    #[doc = "PWR RTC domain control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rtccr(pub u32);
    impl Rtccr {
        #[doc = "Disable RTC domain write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn drtcp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Disable RTC domain write protection."]
        #[inline(always)]
        pub const fn set_drtcp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Rtccr {
        #[inline(always)]
        fn default() -> Rtccr {
            Rtccr(0)
        }
    }
    impl core::fmt::Debug for Rtccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rtccr").field("drtcp", &self.drtcp()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rtccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rtccr {{ drtcp: {=bool:?} }}", self.drtcp())
        }
    }
    #[doc = "PWR supply configuration control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sccr(pub u32);
    impl Sccr {
        #[doc = "power management unit bypass."]
        #[must_use]
        #[inline(always)]
        pub const fn bypass(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "power management unit bypass."]
        #[inline(always)]
        pub const fn set_bypass(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LDO enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ldoen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "LDO enable."]
        #[inline(always)]
        pub const fn set_ldoen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "SMPS enable."]
        #[must_use]
        #[inline(always)]
        pub const fn smpsen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SMPS enable."]
        #[inline(always)]
        pub const fn set_smpsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Sccr {
        #[inline(always)]
        fn default() -> Sccr {
            Sccr(0)
        }
    }
    impl core::fmt::Debug for Sccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sccr")
                .field("bypass", &self.bypass())
                .field("ldoen", &self.ldoen())
                .field("smpsen", &self.smpsen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sccr {{ bypass: {=bool:?}, ldoen: {=bool:?}, smpsen: {=bool:?} }}",
                self.bypass(),
                self.ldoen(),
                self.smpsen()
            )
        }
    }
    #[doc = "PWR security configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccfgr(pub u32);
    impl Seccfgr {
        #[doc = "WUPx secure protection (x=8 to 4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wup1sec(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "WUPx secure protection (x=8 to 4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_wup1sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "WUPx secure protection (x=8 to 4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wup2sec(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "WUPx secure protection (x=8 to 4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_wup2sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "WUPx secure protection (x=8 to 4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wup4sec(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "WUPx secure protection (x=8 to 4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_wup4sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "WUPx secure protection (x=8 to 4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wup5sec(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "WUPx secure protection (x=8 to 4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_wup5sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "WUPx secure protection (x=8 to 4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wup6sec(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "WUPx secure protection (x=8 to 4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_wup6sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "WUPx secure protection (x=8 to 4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wup7sec(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "WUPx secure protection (x=8 to 4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_wup7sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "WUPx secure protection (x=8 to 4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wup8sec(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "WUPx secure protection (x=8 to 4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_wup8sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "retention secure protection."]
        #[must_use]
        #[inline(always)]
        pub const fn retsec(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "retention secure protection."]
        #[inline(always)]
        pub const fn set_retsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "low-power modes secure protection."]
        #[must_use]
        #[inline(always)]
        pub const fn lpmsec(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "low-power modes secure protection."]
        #[inline(always)]
        pub const fn set_lpmsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "supply configuration and monitoring secure protection."]
        #[must_use]
        #[inline(always)]
        pub const fn scmsec(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "supply configuration and monitoring secure protection."]
        #[inline(always)]
        pub const fn set_scmsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "RTC domain and tamper monitoring secure protection."]
        #[must_use]
        #[inline(always)]
        pub const fn rtmsec(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "RTC domain and tamper monitoring secure protection."]
        #[inline(always)]
        pub const fn set_rtmsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "voltage USB secure protection."]
        #[must_use]
        #[inline(always)]
        pub const fn vusbsec(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "voltage USB secure protection."]
        #[inline(always)]
        pub const fn set_vusbsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
                .field("wup1sec", &self.wup1sec())
                .field("wup2sec", &self.wup2sec())
                .field("wup4sec", &self.wup4sec())
                .field("wup5sec", &self.wup5sec())
                .field("wup6sec", &self.wup6sec())
                .field("wup7sec", &self.wup7sec())
                .field("wup8sec", &self.wup8sec())
                .field("retsec", &self.retsec())
                .field("lpmsec", &self.lpmsec())
                .field("scmsec", &self.scmsec())
                .field("rtmsec", &self.rtmsec())
                .field("vusbsec", &self.vusbsec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Seccfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Seccfgr {{ wup1sec: {=bool:?}, wup2sec: {=bool:?}, wup4sec: {=bool:?}, wup5sec: {=bool:?}, wup6sec: {=bool:?}, wup7sec: {=bool:?}, wup8sec: {=bool:?}, retsec: {=bool:?}, lpmsec: {=bool:?}, scmsec: {=bool:?}, rtmsec: {=bool:?}, vusbsec: {=bool:?} }}",
                self.wup1sec(),
                self.wup2sec(),
                self.wup4sec(),
                self.wup5sec(),
                self.wup6sec(),
                self.wup7sec(),
                self.wup8sec(),
                self.retsec(),
                self.lpmsec(),
                self.scmsec(),
                self.rtmsec(),
                self.vusbsec()
            )
        }
    }
    #[doc = "PWR tamper monitoring control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tmcr(pub u32);
    impl Tmcr {
        #[doc = "VDD voltage and temperature monitoring enable."]
        #[must_use]
        #[inline(always)]
        pub const fn monen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "VDD voltage and temperature monitoring enable."]
        #[inline(always)]
        pub const fn set_monen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Tmcr {
        #[inline(always)]
        fn default() -> Tmcr {
            Tmcr(0)
        }
    }
    impl core::fmt::Debug for Tmcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tmcr").field("monen", &self.monen()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tmcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Tmcr {{ monen: {=bool:?} }}", self.monen())
        }
    }
    #[doc = "PWR tamper monitoring status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tmsr(pub u32);
    impl Tmsr {
        #[doc = "VDD voltage level monitoring versus high threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn vddh(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "VDD voltage level monitoring versus high threshold."]
        #[inline(always)]
        pub const fn set_vddh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "temperature level monitoring versus low threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn templ(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "temperature level monitoring versus low threshold."]
        #[inline(always)]
        pub const fn set_templ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "temperature level monitoring versus high threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn temph(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "temperature level monitoring versus high threshold."]
        #[inline(always)]
        pub const fn set_temph(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Tmsr {
        #[inline(always)]
        fn default() -> Tmsr {
            Tmsr(0)
        }
    }
    impl core::fmt::Debug for Tmsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tmsr")
                .field("vddh", &self.vddh())
                .field("templ", &self.templ())
                .field("temph", &self.temph())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tmsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Tmsr {{ vddh: {=bool:?}, templ: {=bool:?}, temph: {=bool:?} }}",
                self.vddh(),
                self.templ(),
                self.temph()
            )
        }
    }
    #[doc = "PWR USB Type-C power delivery register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ucpdr(pub u32);
    impl Ucpdr {
        #[doc = "USB Type-C and power delivery dead battery disable."]
        #[must_use]
        #[inline(always)]
        pub const fn ucpd_dbdis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USB Type-C and power delivery dead battery disable."]
        #[inline(always)]
        pub const fn set_ucpd_dbdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "USB Type-c and Power delivery Standby mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ucpd_stby(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USB Type-c and Power delivery Standby mode."]
        #[inline(always)]
        pub const fn set_ucpd_stby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Ucpdr {
        #[inline(always)]
        fn default() -> Ucpdr {
            Ucpdr(0)
        }
    }
    impl core::fmt::Debug for Ucpdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ucpdr")
                .field("ucpd_dbdis", &self.ucpd_dbdis())
                .field("ucpd_stby", &self.ucpd_stby())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ucpdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ucpdr {{ ucpd_dbdis: {=bool:?}, ucpd_stby: {=bool:?} }}",
                self.ucpd_dbdis(),
                self.ucpd_stby()
            )
        }
    }
    #[doc = "PWR USB supply control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Usbscr(pub u32);
    impl Usbscr {
        #[doc = "USB supply valid."]
        #[must_use]
        #[inline(always)]
        pub const fn usb33sv(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "USB supply valid."]
        #[inline(always)]
        pub const fn set_usb33sv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Usbscr {
        #[inline(always)]
        fn default() -> Usbscr {
            Usbscr(0)
        }
    }
    impl core::fmt::Debug for Usbscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Usbscr").field("usb33sv", &self.usb33sv()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Usbscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Usbscr {{ usb33sv: {=bool:?} }}", self.usb33sv())
        }
    }
    #[doc = "PWR voltage monitor control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vmcr(pub u32);
    impl Vmcr {
        #[doc = "PVD enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pvde(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PVD enable."]
        #[inline(always)]
        pub const fn set_pvde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "programmable voltage detector (PVD) level selection."]
        #[must_use]
        #[inline(always)]
        pub const fn pls(&self) -> super::vals::Pls {
            let val = (self.0 >> 1usize) & 0x03;
            super::vals::Pls::from_bits(val as u8)
        }
        #[doc = "programmable voltage detector (PVD) level selection."]
        #[inline(always)]
        pub const fn set_pls(&mut self, val: super::vals::Pls) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
        }
        #[doc = "peripheral voltage monitor on VDD voltage enable."]
        #[must_use]
        #[inline(always)]
        pub const fn avden(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "peripheral voltage monitor on VDD voltage enable."]
        #[inline(always)]
        pub const fn set_avden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "analog voltage detector (AVD) level selection."]
        #[must_use]
        #[inline(always)]
        pub const fn als(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "analog voltage detector (AVD) level selection."]
        #[inline(always)]
        pub const fn set_als(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Vmcr {
        #[inline(always)]
        fn default() -> Vmcr {
            Vmcr(0)
        }
    }
    impl core::fmt::Debug for Vmcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vmcr")
                .field("pvde", &self.pvde())
                .field("pls", &self.pls())
                .field("avden", &self.avden())
                .field("als", &self.als())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vmcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Vmcr {{ pvde: {=bool:?}, pls: {:?}, avden: {=bool:?}, als: {=bool:?} }}",
                self.pvde(),
                self.pls(),
                self.avden(),
                self.als()
            )
        }
    }
    #[doc = "PWR voltage monitor status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vmsr(pub u32);
    impl Vmsr {
        #[doc = "analog voltage detector output on VDD voltage."]
        #[must_use]
        #[inline(always)]
        pub const fn avdo(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "analog voltage detector output on VDD voltage."]
        #[inline(always)]
        pub const fn set_avdo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "programmable voltage detector output."]
        #[must_use]
        #[inline(always)]
        pub const fn pvdo(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "programmable voltage detector output."]
        #[inline(always)]
        pub const fn set_pvdo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Vmsr {
        #[inline(always)]
        fn default() -> Vmsr {
            Vmsr(0)
        }
    }
    impl core::fmt::Debug for Vmsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vmsr")
                .field("avdo", &self.avdo())
                .field("pvdo", &self.pvdo())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vmsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Vmsr {{ avdo: {=bool:?}, pvdo: {=bool:?} }}",
                self.avdo(),
                self.pvdo()
            )
        }
    }
    #[doc = "PWR voltage scaling control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Voscr(pub u32);
    impl Voscr {
        #[doc = "voltage scaling selection according to performance."]
        #[must_use]
        #[inline(always)]
        pub const fn vos(&self) -> super::vals::Vos {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Vos::from_bits(val as u8)
        }
        #[doc = "voltage scaling selection according to performance."]
        #[inline(always)]
        pub const fn set_vos(&mut self, val: super::vals::Vos) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
    }
    impl Default for Voscr {
        #[inline(always)]
        fn default() -> Voscr {
            Voscr(0)
        }
    }
    impl core::fmt::Debug for Voscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Voscr").field("vos", &self.vos()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Voscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Voscr {{ vos: {:?} }}", self.vos())
        }
    }
    #[doc = "PWR voltage scaling status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vossr(pub u32);
    impl Vossr {
        #[doc = "SRAM memories operation at low speed."]
        #[must_use]
        #[inline(always)]
        pub const fn sramls(&self) -> super::vals::Sramls {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Sramls::from_bits(val as u8)
        }
        #[doc = "SRAM memories operation at low speed."]
        #[inline(always)]
        pub const fn set_sramls(&mut self, val: super::vals::Sramls) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Ready bit for VCORE voltage scaling output selection."]
        #[must_use]
        #[inline(always)]
        pub const fn vosrdy(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Ready bit for VCORE voltage scaling output selection."]
        #[inline(always)]
        pub const fn set_vosrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Current VOS voltage level ready."]
        #[must_use]
        #[inline(always)]
        pub const fn actvosrdy(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Current VOS voltage level ready."]
        #[inline(always)]
        pub const fn set_actvosrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Current voltage scaling range."]
        #[must_use]
        #[inline(always)]
        pub const fn actvos(&self) -> super::vals::Vos {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::Vos::from_bits(val as u8)
        }
        #[doc = "Current voltage scaling range."]
        #[inline(always)]
        pub const fn set_actvos(&mut self, val: super::vals::Vos) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
    }
    impl Default for Vossr {
        #[inline(always)]
        fn default() -> Vossr {
            Vossr(0)
        }
    }
    impl core::fmt::Debug for Vossr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vossr")
                .field("sramls", &self.sramls())
                .field("vosrdy", &self.vosrdy())
                .field("actvosrdy", &self.actvosrdy())
                .field("actvos", &self.actvos())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vossr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Vossr {{ sramls: {:?}, vosrdy: {=bool:?}, actvosrdy: {=bool:?}, actvos: {:?} }}",
                self.sramls(),
                self.vosrdy(),
                self.actvosrdy(),
                self.actvos()
            )
        }
    }
    #[doc = "PWR wake-up configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wucr(pub u32);
    impl Wucr {
        #[doc = "enable wake-up pin WUPx (x=8 to 4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wupen1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable wake-up pin WUPx (x=8 to 4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_wupen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "enable wake-up pin WUPx (x=8 to 4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wupen2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "enable wake-up pin WUPx (x=8 to 4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_wupen2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "enable wake-up pin WUPx (x=8 to 4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wupen4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable wake-up pin WUPx (x=8 to 4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_wupen4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "enable wake-up pin WUPx (x=8 to 4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wupen5(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "enable wake-up pin WUPx (x=8 to 4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_wupen5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "enable wake-up pin WUPx (x=8 to 4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wupen6(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "enable wake-up pin WUPx (x=8 to 4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_wupen6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "enable wake-up pin WUPx (x=8 to 4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wupen7(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "enable wake-up pin WUPx (x=8 to 4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_wupen7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "enable wake-up pin WUPx (x=8 to 4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wupen8(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "enable wake-up pin WUPx (x=8 to 4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_wupen8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "wake-up pin polarity bit for WUPx (x=8 to 4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wupp1(&self) -> super::vals::Wupp {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Wupp::from_bits(val as u8)
        }
        #[doc = "wake-up pin polarity bit for WUPx (x=8 to 4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_wupp1(&mut self, val: super::vals::Wupp) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "wake-up pin polarity bit for WUPx (x=8 to 4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wupp2(&self) -> super::vals::Wupp {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Wupp::from_bits(val as u8)
        }
        #[doc = "wake-up pin polarity bit for WUPx (x=8 to 4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_wupp2(&mut self, val: super::vals::Wupp) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "wake-up pin polarity bit for WUPx (x=8 to 4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wupp4(&self) -> super::vals::Wupp {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Wupp::from_bits(val as u8)
        }
        #[doc = "wake-up pin polarity bit for WUPx (x=8 to 4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_wupp4(&mut self, val: super::vals::Wupp) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "wake-up pin polarity bit for WUPx (x=8 to 4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wupp5(&self) -> super::vals::Wupp {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Wupp::from_bits(val as u8)
        }
        #[doc = "wake-up pin polarity bit for WUPx (x=8 to 4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_wupp5(&mut self, val: super::vals::Wupp) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "wake-up pin polarity bit for WUPx (x=8 to 4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wupp6(&self) -> super::vals::Wupp {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::Wupp::from_bits(val as u8)
        }
        #[doc = "wake-up pin polarity bit for WUPx (x=8 to 4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_wupp6(&mut self, val: super::vals::Wupp) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
        #[doc = "wake-up pin polarity bit for WUPx (x=8 to 4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wupp7(&self) -> super::vals::Wupp {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Wupp::from_bits(val as u8)
        }
        #[doc = "wake-up pin polarity bit for WUPx (x=8 to 4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_wupp7(&mut self, val: super::vals::Wupp) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "wake-up pin polarity bit for WUPx (x=8 to 4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wupp8(&self) -> super::vals::Wupp {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Wupp::from_bits(val as u8)
        }
        #[doc = "wake-up pin polarity bit for WUPx (x=8 to 4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_wupp8(&mut self, val: super::vals::Wupp) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "wake-up pin pull configuration for WKUPx (x=8 to 4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wuppupd1(&self) -> super::vals::Wuppupd {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Wuppupd::from_bits(val as u8)
        }
        #[doc = "wake-up pin pull configuration for WKUPx (x=8 to 4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_wuppupd1(&mut self, val: super::vals::Wuppupd) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "wake-up pin pull configuration for WKUPx (x=8 to 4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wuppupd2(&self) -> super::vals::Wuppupd {
            let val = (self.0 >> 18usize) & 0x03;
            super::vals::Wuppupd::from_bits(val as u8)
        }
        #[doc = "wake-up pin pull configuration for WKUPx (x=8 to 4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_wuppupd2(&mut self, val: super::vals::Wuppupd) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
        }
        #[doc = "wake-up pin pull configuration for WKUPx (x=8 to 4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wuppupd4(&self) -> super::vals::Wuppupd {
            let val = (self.0 >> 22usize) & 0x03;
            super::vals::Wuppupd::from_bits(val as u8)
        }
        #[doc = "wake-up pin pull configuration for WKUPx (x=8 to 4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_wuppupd4(&mut self, val: super::vals::Wuppupd) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
        }
        #[doc = "wake-up pin pull configuration for WKUPx (x=8 to 4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wuppupd5(&self) -> super::vals::Wuppupd {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Wuppupd::from_bits(val as u8)
        }
        #[doc = "wake-up pin pull configuration for WKUPx (x=8 to 4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_wuppupd5(&mut self, val: super::vals::Wuppupd) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "wake-up pin pull configuration for WKUPx (x=8 to 4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wuppupd6(&self) -> super::vals::Wuppupd {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::Wuppupd::from_bits(val as u8)
        }
        #[doc = "wake-up pin pull configuration for WKUPx (x=8 to 4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_wuppupd6(&mut self, val: super::vals::Wuppupd) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "wake-up pin pull configuration for WKUPx (x=8 to 4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wuppupd7(&self) -> super::vals::Wuppupd {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Wuppupd::from_bits(val as u8)
        }
        #[doc = "wake-up pin pull configuration for WKUPx (x=8 to 4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_wuppupd7(&mut self, val: super::vals::Wuppupd) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "wake-up pin pull configuration for WKUPx (x=8 to 4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wuppupd8(&self) -> super::vals::Wuppupd {
            let val = (self.0 >> 30usize) & 0x03;
            super::vals::Wuppupd::from_bits(val as u8)
        }
        #[doc = "wake-up pin pull configuration for WKUPx (x=8 to 4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_wuppupd8(&mut self, val: super::vals::Wuppupd) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
        }
    }
    impl Default for Wucr {
        #[inline(always)]
        fn default() -> Wucr {
            Wucr(0)
        }
    }
    impl core::fmt::Debug for Wucr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wucr")
                .field("wupen1", &self.wupen1())
                .field("wupen2", &self.wupen2())
                .field("wupen4", &self.wupen4())
                .field("wupen5", &self.wupen5())
                .field("wupen6", &self.wupen6())
                .field("wupen7", &self.wupen7())
                .field("wupen8", &self.wupen8())
                .field("wupp1", &self.wupp1())
                .field("wupp2", &self.wupp2())
                .field("wupp4", &self.wupp4())
                .field("wupp5", &self.wupp5())
                .field("wupp6", &self.wupp6())
                .field("wupp7", &self.wupp7())
                .field("wupp8", &self.wupp8())
                .field("wuppupd1", &self.wuppupd1())
                .field("wuppupd2", &self.wuppupd2())
                .field("wuppupd4", &self.wuppupd4())
                .field("wuppupd5", &self.wuppupd5())
                .field("wuppupd6", &self.wuppupd6())
                .field("wuppupd7", &self.wuppupd7())
                .field("wuppupd8", &self.wuppupd8())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wucr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wucr {{ wupen1: {=bool:?}, wupen2: {=bool:?}, wupen4: {=bool:?}, wupen5: {=bool:?}, wupen6: {=bool:?}, wupen7: {=bool:?}, wupen8: {=bool:?}, wupp1: {:?}, wupp2: {:?}, wupp4: {:?}, wupp5: {:?}, wupp6: {:?}, wupp7: {:?}, wupp8: {:?}, wuppupd1: {:?}, wuppupd2: {:?}, wuppupd4: {:?}, wuppupd5: {:?}, wuppupd6: {:?}, wuppupd7: {:?}, wuppupd8: {:?} }}",
                self.wupen1(),
                self.wupen2(),
                self.wupen4(),
                self.wupen5(),
                self.wupen6(),
                self.wupen7(),
                self.wupen8(),
                self.wupp1(),
                self.wupp2(),
                self.wupp4(),
                self.wupp5(),
                self.wupp6(),
                self.wupp7(),
                self.wupp8(),
                self.wuppupd1(),
                self.wuppupd2(),
                self.wuppupd4(),
                self.wuppupd5(),
                self.wuppupd6(),
                self.wuppupd7(),
                self.wuppupd8()
            )
        }
    }
    #[doc = "PWR wake-up status clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wuscr(pub u32);
    impl Wuscr {
        #[doc = "clear wake-up pin flag for WUFx (x=8 to4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn cwuf1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "clear wake-up pin flag for WUFx (x=8 to4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_cwuf1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "clear wake-up pin flag for WUFx (x=8 to4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn cwuf2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "clear wake-up pin flag for WUFx (x=8 to4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_cwuf2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "clear wake-up pin flag for WUFx (x=8 to4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn cwuf4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "clear wake-up pin flag for WUFx (x=8 to4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_cwuf4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "clear wake-up pin flag for WUFx (x=8 to4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn cwuf5(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "clear wake-up pin flag for WUFx (x=8 to4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_cwuf5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "clear wake-up pin flag for WUFx (x=8 to4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn cwuf6(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "clear wake-up pin flag for WUFx (x=8 to4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_cwuf6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "clear wake-up pin flag for WUFx (x=8 to4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn cwuf7(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "clear wake-up pin flag for WUFx (x=8 to4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_cwuf7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "clear wake-up pin flag for WUFx (x=8 to4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn cwuf8(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "clear wake-up pin flag for WUFx (x=8 to4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_cwuf8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Wuscr {
        #[inline(always)]
        fn default() -> Wuscr {
            Wuscr(0)
        }
    }
    impl core::fmt::Debug for Wuscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wuscr")
                .field("cwuf1", &self.cwuf1())
                .field("cwuf2", &self.cwuf2())
                .field("cwuf4", &self.cwuf4())
                .field("cwuf5", &self.cwuf5())
                .field("cwuf6", &self.cwuf6())
                .field("cwuf7", &self.cwuf7())
                .field("cwuf8", &self.cwuf8())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wuscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wuscr {{ cwuf1: {=bool:?}, cwuf2: {=bool:?}, cwuf4: {=bool:?}, cwuf5: {=bool:?}, cwuf6: {=bool:?}, cwuf7: {=bool:?}, cwuf8: {=bool:?} }}",
                self.cwuf1(),
                self.cwuf2(),
                self.cwuf4(),
                self.cwuf5(),
                self.cwuf6(),
                self.cwuf7(),
                self.cwuf8()
            )
        }
    }
    #[doc = "PWR wake-up status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wusr(pub u32);
    impl Wusr {
        #[doc = "wake-up pin WUFx flag (x = 8 to 4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wuf1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "wake-up pin WUFx flag (x = 8 to 4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_wuf1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "wake-up pin WUFx flag (x = 8 to 4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wuf2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "wake-up pin WUFx flag (x = 8 to 4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_wuf2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "wake-up pin WUFx flag (x = 8 to 4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wuf4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "wake-up pin WUFx flag (x = 8 to 4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_wuf4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "wake-up pin WUFx flag (x = 8 to 4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wuf5(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "wake-up pin WUFx flag (x = 8 to 4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_wuf5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "wake-up pin WUFx flag (x = 8 to 4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wuf6(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "wake-up pin WUFx flag (x = 8 to 4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_wuf6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "wake-up pin WUFx flag (x = 8 to 4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wuf7(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "wake-up pin WUFx flag (x = 8 to 4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_wuf7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "wake-up pin WUFx flag (x = 8 to 4, 2 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wuf8(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "wake-up pin WUFx flag (x = 8 to 4, 2 to 1)."]
        #[inline(always)]
        pub const fn set_wuf8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Wusr {
        #[inline(always)]
        fn default() -> Wusr {
            Wusr(0)
        }
    }
    impl core::fmt::Debug for Wusr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wusr")
                .field("wuf1", &self.wuf1())
                .field("wuf2", &self.wuf2())
                .field("wuf4", &self.wuf4())
                .field("wuf5", &self.wuf5())
                .field("wuf6", &self.wuf6())
                .field("wuf7", &self.wuf7())
                .field("wuf8", &self.wuf8())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wusr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wusr {{ wuf1: {=bool:?}, wuf2: {=bool:?}, wuf4: {=bool:?}, wuf5: {=bool:?}, wuf6: {=bool:?}, wuf7: {=bool:?}, wuf8: {=bool:?} }}",
                self.wuf1(),
                self.wuf2(),
                self.wuf4(),
                self.wuf5(),
                self.wuf6(),
                self.wuf7(),
                self.wuf8()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Flps {
        #[doc = "flash memory remains in normal mode when the system enters Stop mode (quickrestart time)."]
        Normal = 0x0,
        #[doc = "flash memory enters low-power mode when the system enters Stop mode(low-power consumption)."]
        LowPower = 0x01,
    }
    impl Flps {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Flps {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Flps {
        #[inline(always)]
        fn from(val: u8) -> Flps {
            Flps::from_bits(val)
        }
    }
    impl From<Flps> for u8 {
        #[inline(always)]
        fn from(val: Flps) -> u8 {
            Flps::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lpms {
        #[doc = "keeps Stop mode when entering DeepSleep."]
        Stop = 0x0,
        #[doc = "allows Standby mode when entering DeepSleep."]
        Standby = 0x01,
    }
    impl Lpms {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lpms {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lpms {
        #[inline(always)]
        fn from(val: u8) -> Lpms {
            Lpms::from_bits(val)
        }
    }
    impl From<Lpms> for u8 {
        #[inline(always)]
        fn from(val: Lpms) -> u8 {
            Lpms::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pls {
        #[doc = "PVD level0."]
        Level0 = 0x0,
        #[doc = "PVD level1."]
        Level1 = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "PVD_IN pin."]
        PvdInPin = 0x03,
    }
    impl Pls {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pls {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pls {
        #[inline(always)]
        fn from(val: u8) -> Pls {
            Pls::from_bits(val)
        }
    }
    impl From<Pls> for u8 {
        #[inline(always)]
        fn from(val: Pls) -> u8 {
            Pls::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sramls {
        #[doc = "SRAMs operating at high speed (VOS1 or VOS0 voltage scaling)."]
        HighSpeed = 0x0,
        #[doc = "SRAMs operating at low speed (VOS2 voltage scaling)."]
        LowSpeed = 0x01,
    }
    impl Sramls {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sramls {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sramls {
        #[inline(always)]
        fn from(val: u8) -> Sramls {
            Sramls::from_bits(val)
        }
    }
    impl From<Sramls> for u8 {
        #[inline(always)]
        fn from(val: Sramls) -> u8 {
            Sramls::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Srampds {
        #[doc = "AHB RAM1 content is kept in Stop mode."]
        Preserved = 0x0,
        #[doc = "AHB RAM1 content is lost in Stop mode."]
        Lost = 0x01,
    }
    impl Srampds {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Srampds {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Srampds {
        #[inline(always)]
        fn from(val: u8) -> Srampds {
            Srampds::from_bits(val)
        }
    }
    impl From<Srampds> for u8 {
        #[inline(always)]
        fn from(val: Srampds) -> u8 {
            Srampds::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Vos {
        _RESERVED_0 = 0x0,
        #[doc = "range 2 (VOS2, default): Low-power range."]
        Scale2 = 0x01,
        #[doc = "range 1: Medium-performance range."]
        Scale1 = 0x02,
        #[doc = "range 0: High-performance range."]
        Scale0 = 0x03,
    }
    impl Vos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vos {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vos {
        #[inline(always)]
        fn from(val: u8) -> Vos {
            Vos::from_bits(val)
        }
    }
    impl From<Vos> for u8 {
        #[inline(always)]
        fn from(val: Vos) -> u8 {
            Vos::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wupp {
        #[doc = "detection on high level (rising edge)."]
        RisingEdge = 0x0,
        #[doc = "detection on low level (falling edge)."]
        FallingEdge = 0x01,
    }
    impl Wupp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wupp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wupp {
        #[inline(always)]
        fn from(val: u8) -> Wupp {
            Wupp::from_bits(val)
        }
    }
    impl From<Wupp> for u8 {
        #[inline(always)]
        fn from(val: Wupp) -> u8 {
            Wupp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wuppupd {
        NoPullUp = 0x0,
        PullUp = 0x01,
        PullDown = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Wuppupd {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wuppupd {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wuppupd {
        #[inline(always)]
        fn from(val: u8) -> Wuppupd {
            Wuppupd::from_bits(val)
        }
    }
    impl From<Wuppupd> for u8 {
        #[inline(always)]
        fn from(val: Wuppupd) -> u8 {
            Wuppupd::to_bits(val)
        }
    }
}
