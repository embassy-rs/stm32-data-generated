#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "PWR register block."]
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
    #[doc = "PWR power mode control register."]
    #[inline(always)]
    pub const fn pmcr(self) -> crate::common::Reg<regs::Pmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "PWR status register."]
    #[inline(always)]
    pub const fn pmsr(self) -> crate::common::Reg<regs::Pmsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "PWR RTC domain control register."]
    #[inline(always)]
    pub const fn rtccr(self) -> crate::common::Reg<regs::Rtccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "PWR voltage monitor control register."]
    #[inline(always)]
    pub const fn vmcr(self) -> crate::common::Reg<regs::Vmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
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
        pub const fn ioreten(&self) -> super::vals::Reten {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Reten::from_bits(val as u8)
        }
        #[doc = "IO retention enable."]
        #[inline(always)]
        pub const fn set_ioreten(&mut self, val: super::vals::Reten) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "IO retention enable for JTAG I/Os."]
        #[must_use]
        #[inline(always)]
        pub const fn jtagioreten(&self) -> super::vals::Reten {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Reten::from_bits(val as u8)
        }
        #[doc = "IO retention enable for JTAG I/Os."]
        #[inline(always)]
        pub const fn set_jtagioreten(&mut self, val: super::vals::Reten) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
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
                "Ioretr {{ ioreten: {:?}, jtagioreten: {:?} }}",
                self.ioreten(),
                self.jtagioreten()
            )
        }
    }
    #[doc = "PWR power mode control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pmcr(pub u32);
    impl Pmcr {
        #[doc = "low-power mode selection."]
        #[must_use]
        #[inline(always)]
        pub const fn lpms(&self) -> super::vals::Lpms {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Lpms::from_bits(val as u8)
        }
        #[doc = "low-power mode selection."]
        #[inline(always)]
        pub const fn set_lpms(&mut self, val: super::vals::Lpms) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Clear Standby and Stop flags (always read as 0)."]
        #[must_use]
        #[inline(always)]
        pub const fn cssf(&self) -> super::vals::Cssf {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Cssf::from_bits(val as u8)
        }
        #[doc = "Clear Standby and Stop flags (always read as 0)."]
        #[inline(always)]
        pub const fn set_cssf(&mut self, val: super::vals::Cssf) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "Flash memory low-power mode in Stop mode."]
        #[must_use]
        #[inline(always)]
        pub const fn flps(&self) -> super::vals::Flps {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Flps::from_bits(val as u8)
        }
        #[doc = "Flash memory low-power mode in Stop mode."]
        #[inline(always)]
        pub const fn set_flps(&mut self, val: super::vals::Flps) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "AHB SRAM2 block 3 shut-off in Stop mode."]
        #[must_use]
        #[inline(always)]
        pub const fn sram2_3_so(&self) -> super::vals::SramSo {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::SramSo::from_bits(val as u8)
        }
        #[doc = "AHB SRAM2 block 3 shut-off in Stop mode."]
        #[inline(always)]
        pub const fn set_sram2_3_so(&mut self, val: super::vals::SramSo) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "AHB SRAM2 block 1 shut-off in Stop mode."]
        #[must_use]
        #[inline(always)]
        pub const fn sram2_1_so(&self) -> super::vals::SramSo {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::SramSo::from_bits(val as u8)
        }
        #[doc = "AHB SRAM2 block 1 shut-off in Stop mode."]
        #[inline(always)]
        pub const fn set_sram2_1_so(&mut self, val: super::vals::SramSo) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
        #[doc = "AHB SRAM2 16-Kbyte shut-off in Stop mode (lower address)."]
        #[must_use]
        #[inline(always)]
        pub const fn sram2_lso(&self) -> super::vals::SramSo {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::SramSo::from_bits(val as u8)
        }
        #[doc = "AHB SRAM2 16-Kbyte shut-off in Stop mode (lower address)."]
        #[inline(always)]
        pub const fn set_sram2_lso(&mut self, val: super::vals::SramSo) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
        #[doc = "AHB SRAM2 block 2 shut-off in Stop mode."]
        #[must_use]
        #[inline(always)]
        pub const fn sram2_2_so(&self) -> super::vals::SramSo {
            let val = (self.0 >> 25usize) & 0x01;
            super::vals::SramSo::from_bits(val as u8)
        }
        #[doc = "AHB SRAM2 block 2 shut-off in Stop mode."]
        #[inline(always)]
        pub const fn set_sram2_2_so(&mut self, val: super::vals::SramSo) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
        }
        #[doc = "AHB SRAM2 48-Kbyte shut-off in Stop mode (higher address)."]
        #[must_use]
        #[inline(always)]
        pub const fn sram2_hso(&self) -> super::vals::SramSo {
            let val = (self.0 >> 25usize) & 0x01;
            super::vals::SramSo::from_bits(val as u8)
        }
        #[doc = "AHB SRAM2 48-Kbyte shut-off in Stop mode (higher address)."]
        #[inline(always)]
        pub const fn set_sram2_hso(&mut self, val: super::vals::SramSo) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
        }
        #[doc = "AHB SRAM1 block 1shut-off in Stop mode."]
        #[must_use]
        #[inline(always)]
        pub const fn sram1so(&self) -> super::vals::SramSo {
            let val = (self.0 >> 26usize) & 0x01;
            super::vals::SramSo::from_bits(val as u8)
        }
        #[doc = "AHB SRAM1 block 1shut-off in Stop mode."]
        #[inline(always)]
        pub const fn set_sram1so(&mut self, val: super::vals::SramSo) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Pmcr {
        #[inline(always)]
        fn default() -> Pmcr {
            Pmcr(0)
        }
    }
    impl core::fmt::Debug for Pmcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pmcr")
                .field("lpms", &self.lpms())
                .field("cssf", &self.cssf())
                .field("flps", &self.flps())
                .field("sram2_3_so", &self.sram2_3_so())
                .field("sram2_1_so", &self.sram2_1_so())
                .field("sram2_lso", &self.sram2_lso())
                .field("sram2_2_so", &self.sram2_2_so())
                .field("sram2_hso", &self.sram2_hso())
                .field("sram1so", &self.sram1so())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pmcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Pmcr {{ lpms: {:?}, cssf: {:?}, flps: {:?}, sram2_3_so: {:?}, sram2_1_so: {:?}, sram2_lso: {:?}, sram2_2_so: {:?}, sram2_hso: {:?}, sram1so: {:?} }}" , self . lpms () , self . cssf () , self . flps () , self . sram2_3_so () , self . sram2_1_so () , self . sram2_lso () , self . sram2_2_so () , self . sram2_hso () , self . sram1so ())
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
        #[doc = "PWR nonsecure functions privilege configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn priv_(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "PWR nonsecure functions privilege configuration."]
        #[inline(always)]
        pub const fn set_priv_(&mut self, val: bool) {
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
            f.debug_struct("Privcfgr").field("priv_", &self.priv_()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Privcfgr {{ priv_: {=bool:?} }}", self.priv_())
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
    }
    impl Default for Vmcr {
        #[inline(always)]
        fn default() -> Vmcr {
            Vmcr(0)
        }
    }
    impl core::fmt::Debug for Vmcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vmcr").field("pvde", &self.pvde()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vmcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Vmcr {{ pvde: {=bool:?} }}", self.pvde())
        }
    }
    #[doc = "PWR voltage monitor status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vmsr(pub u32);
    impl Vmsr {
        #[doc = "programmable voltage detect output."]
        #[must_use]
        #[inline(always)]
        pub const fn pvdo(&self) -> super::vals::Pvdo {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Pvdo::from_bits(val as u8)
        }
        #[doc = "programmable voltage detect output."]
        #[inline(always)]
        pub const fn set_pvdo(&mut self, val: super::vals::Pvdo) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
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
            f.debug_struct("Vmsr").field("pvdo", &self.pvdo()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vmsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Vmsr {{ pvdo: {:?} }}", self.pvdo())
        }
    }
    #[doc = "PWR wake-up configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wucr(pub u32);
    impl Wucr {
        #[doc = "Enable wake-up pin WKUPx (x=5to1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wupen1(&self) -> super::vals::Wupen {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Wupen::from_bits(val as u8)
        }
        #[doc = "Enable wake-up pin WKUPx (x=5to1)."]
        #[inline(always)]
        pub const fn set_wupen1(&mut self, val: super::vals::Wupen) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Enable wake-up pin WKUPx (x=5to1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wupen2(&self) -> super::vals::Wupen {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Wupen::from_bits(val as u8)
        }
        #[doc = "Enable wake-up pin WKUPx (x=5to1)."]
        #[inline(always)]
        pub const fn set_wupen2(&mut self, val: super::vals::Wupen) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Enable wake-up pin WKUPx (x=7to1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wupen3(&self) -> super::vals::Wupen {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Wupen::from_bits(val as u8)
        }
        #[doc = "Enable wake-up pin WKUPx (x=7to1)."]
        #[inline(always)]
        pub const fn set_wupen3(&mut self, val: super::vals::Wupen) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Enable wake-up pin WKUPx (x=5to1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wupen4(&self) -> super::vals::Wupen {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Wupen::from_bits(val as u8)
        }
        #[doc = "Enable wake-up pin WKUPx (x=5to1)."]
        #[inline(always)]
        pub const fn set_wupen4(&mut self, val: super::vals::Wupen) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Enable wake-up pin WKUPx (x=5to1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wupen5(&self) -> super::vals::Wupen {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Wupen::from_bits(val as u8)
        }
        #[doc = "Enable wake-up pin WKUPx (x=5to1)."]
        #[inline(always)]
        pub const fn set_wupen5(&mut self, val: super::vals::Wupen) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Enable wake-up pin WKUPx (x=7to1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wupen6(&self) -> super::vals::Wupen {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Wupen::from_bits(val as u8)
        }
        #[doc = "Enable wake-up pin WKUPx (x=7to1)."]
        #[inline(always)]
        pub const fn set_wupen6(&mut self, val: super::vals::Wupen) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "Enable wake-up pin WKUPx (x=7to1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wupen7(&self) -> super::vals::Wupen {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Wupen::from_bits(val as u8)
        }
        #[doc = "Enable wake-up pin WKUPx (x=7to1)."]
        #[inline(always)]
        pub const fn set_wupen7(&mut self, val: super::vals::Wupen) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "Wake-up pin polarity bit for WKUPx (x=5to1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wupp1(&self) -> super::vals::Wupp {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Wupp::from_bits(val as u8)
        }
        #[doc = "Wake-up pin polarity bit for WKUPx (x=5to1)."]
        #[inline(always)]
        pub const fn set_wupp1(&mut self, val: super::vals::Wupp) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "Wake-up pin polarity bit for WKUPx (x=5to1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wupp2(&self) -> super::vals::Wupp {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Wupp::from_bits(val as u8)
        }
        #[doc = "Wake-up pin polarity bit for WKUPx (x=5to1)."]
        #[inline(always)]
        pub const fn set_wupp2(&mut self, val: super::vals::Wupp) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "Wake-up pin polarity bit for WKUPx (x=7to1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wupp3(&self) -> super::vals::Wupp {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Wupp::from_bits(val as u8)
        }
        #[doc = "Wake-up pin polarity bit for WKUPx (x=7to1)."]
        #[inline(always)]
        pub const fn set_wupp3(&mut self, val: super::vals::Wupp) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "Wake-up pin polarity bit for WKUPx (x=5to1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wupp4(&self) -> super::vals::Wupp {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Wupp::from_bits(val as u8)
        }
        #[doc = "Wake-up pin polarity bit for WKUPx (x=5to1)."]
        #[inline(always)]
        pub const fn set_wupp4(&mut self, val: super::vals::Wupp) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Wake-up pin polarity bit for WKUPx (x=5to1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wupp5(&self) -> super::vals::Wupp {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Wupp::from_bits(val as u8)
        }
        #[doc = "Wake-up pin polarity bit for WKUPx (x=5to1)."]
        #[inline(always)]
        pub const fn set_wupp5(&mut self, val: super::vals::Wupp) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "Wake-up pin polarity bit for WKUPx (x=7to1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wupp6(&self) -> super::vals::Wupp {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::Wupp::from_bits(val as u8)
        }
        #[doc = "Wake-up pin polarity bit for WKUPx (x=7to1)."]
        #[inline(always)]
        pub const fn set_wupp6(&mut self, val: super::vals::Wupp) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
        #[doc = "Wake-up pin polarity bit for WKUPx (x=7to1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wupp7(&self) -> super::vals::Wupp {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Wupp::from_bits(val as u8)
        }
        #[doc = "Wake-up pin polarity bit for WKUPx (x=7to1)."]
        #[inline(always)]
        pub const fn set_wupp7(&mut self, val: super::vals::Wupp) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "Wake-up pin pull configuration for WKUPx (x=5to1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wuppupd1(&self) -> super::vals::Wuppupd {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Wuppupd::from_bits(val as u8)
        }
        #[doc = "Wake-up pin pull configuration for WKUPx (x=5to1)."]
        #[inline(always)]
        pub const fn set_wuppupd1(&mut self, val: super::vals::Wuppupd) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Wake-up pin pull configuration for WKUPx (x=5to1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wuppupd2(&self) -> super::vals::Wuppupd {
            let val = (self.0 >> 18usize) & 0x03;
            super::vals::Wuppupd::from_bits(val as u8)
        }
        #[doc = "Wake-up pin pull configuration for WKUPx (x=5to1)."]
        #[inline(always)]
        pub const fn set_wuppupd2(&mut self, val: super::vals::Wuppupd) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
        }
        #[doc = "Wake-up pin pull configuration for WKUPx (x=7to1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wuppupd3(&self) -> super::vals::Wuppupd {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::Wuppupd::from_bits(val as u8)
        }
        #[doc = "Wake-up pin pull configuration for WKUPx (x=7to1)."]
        #[inline(always)]
        pub const fn set_wuppupd3(&mut self, val: super::vals::Wuppupd) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "Wake-up pin pull configuration for WKUPx (x=5to1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wuppupd4(&self) -> super::vals::Wuppupd {
            let val = (self.0 >> 22usize) & 0x03;
            super::vals::Wuppupd::from_bits(val as u8)
        }
        #[doc = "Wake-up pin pull configuration for WKUPx (x=5to1)."]
        #[inline(always)]
        pub const fn set_wuppupd4(&mut self, val: super::vals::Wuppupd) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
        }
        #[doc = "Wake-up pin pull configuration for WKUPx (x=5to1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wuppupd5(&self) -> super::vals::Wuppupd {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Wuppupd::from_bits(val as u8)
        }
        #[doc = "Wake-up pin pull configuration for WKUPx (x=5to1)."]
        #[inline(always)]
        pub const fn set_wuppupd5(&mut self, val: super::vals::Wuppupd) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "Wake-up pin pull configuration for WKUPx (x=7to1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wuppupd6(&self) -> super::vals::Wuppupd {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::Wuppupd::from_bits(val as u8)
        }
        #[doc = "Wake-up pin pull configuration for WKUPx (x=7to1)."]
        #[inline(always)]
        pub const fn set_wuppupd6(&mut self, val: super::vals::Wuppupd) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "Wake-up pin pull configuration for WKUPx (x=7to1)."]
        #[must_use]
        #[inline(always)]
        pub const fn wuppupd7(&self) -> super::vals::Wuppupd {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Wuppupd::from_bits(val as u8)
        }
        #[doc = "Wake-up pin pull configuration for WKUPx (x=7to1)."]
        #[inline(always)]
        pub const fn set_wuppupd7(&mut self, val: super::vals::Wuppupd) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
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
                .field("wupen3", &self.wupen3())
                .field("wupen4", &self.wupen4())
                .field("wupen5", &self.wupen5())
                .field("wupen6", &self.wupen6())
                .field("wupen7", &self.wupen7())
                .field("wupp1", &self.wupp1())
                .field("wupp2", &self.wupp2())
                .field("wupp3", &self.wupp3())
                .field("wupp4", &self.wupp4())
                .field("wupp5", &self.wupp5())
                .field("wupp6", &self.wupp6())
                .field("wupp7", &self.wupp7())
                .field("wuppupd1", &self.wuppupd1())
                .field("wuppupd2", &self.wuppupd2())
                .field("wuppupd3", &self.wuppupd3())
                .field("wuppupd4", &self.wuppupd4())
                .field("wuppupd5", &self.wuppupd5())
                .field("wuppupd6", &self.wuppupd6())
                .field("wuppupd7", &self.wuppupd7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wucr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Wucr {{ wupen1: {:?}, wupen2: {:?}, wupen3: {:?}, wupen4: {:?}, wupen5: {:?}, wupen6: {:?}, wupen7: {:?}, wupp1: {:?}, wupp2: {:?}, wupp3: {:?}, wupp4: {:?}, wupp5: {:?}, wupp6: {:?}, wupp7: {:?}, wuppupd1: {:?}, wuppupd2: {:?}, wuppupd3: {:?}, wuppupd4: {:?}, wuppupd5: {:?}, wuppupd6: {:?}, wuppupd7: {:?} }}" , self . wupen1 () , self . wupen2 () , self . wupen3 () , self . wupen4 () , self . wupen5 () , self . wupen6 () , self . wupen7 () , self . wupp1 () , self . wupp2 () , self . wupp3 () , self . wupp4 () , self . wupp5 () , self . wupp6 () , self . wupp7 () , self . wuppupd1 () , self . wuppupd2 () , self . wuppupd3 () , self . wuppupd4 () , self . wuppupd5 () , self . wuppupd6 () , self . wuppupd7 ())
        }
    }
    #[doc = "PWR wake-up status clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wuscr(pub u32);
    impl Wuscr {
        #[doc = "clear wake-up pin flag for WUFx (x=5 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn cwuf1(&self) -> super::vals::Cwuf {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Cwuf::from_bits(val as u8)
        }
        #[doc = "clear wake-up pin flag for WUFx (x=5 to 1)."]
        #[inline(always)]
        pub const fn set_cwuf1(&mut self, val: super::vals::Cwuf) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "clear wake-up pin flag for WUFx (x=5 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn cwuf2(&self) -> super::vals::Cwuf {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Cwuf::from_bits(val as u8)
        }
        #[doc = "clear wake-up pin flag for WUFx (x=5 to 1)."]
        #[inline(always)]
        pub const fn set_cwuf2(&mut self, val: super::vals::Cwuf) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "clear wake-up pin flag for WUFx (x=7 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn cwuf3(&self) -> super::vals::Cwuf {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Cwuf::from_bits(val as u8)
        }
        #[doc = "clear wake-up pin flag for WUFx (x=7 to 1)."]
        #[inline(always)]
        pub const fn set_cwuf3(&mut self, val: super::vals::Cwuf) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "clear wake-up pin flag for WUFx (x=5 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn cwuf4(&self) -> super::vals::Cwuf {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Cwuf::from_bits(val as u8)
        }
        #[doc = "clear wake-up pin flag for WUFx (x=5 to 1)."]
        #[inline(always)]
        pub const fn set_cwuf4(&mut self, val: super::vals::Cwuf) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "clear wake-up pin flag for WUFx (x=5 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn cwuf5(&self) -> super::vals::Cwuf {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Cwuf::from_bits(val as u8)
        }
        #[doc = "clear wake-up pin flag for WUFx (x=5 to 1)."]
        #[inline(always)]
        pub const fn set_cwuf5(&mut self, val: super::vals::Cwuf) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "clear wake-up pin flag for WUFx (x=7 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn cwuf6(&self) -> super::vals::Cwuf {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Cwuf::from_bits(val as u8)
        }
        #[doc = "clear wake-up pin flag for WUFx (x=7 to 1)."]
        #[inline(always)]
        pub const fn set_cwuf6(&mut self, val: super::vals::Cwuf) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "clear wake-up pin flag for WUFx (x=7 to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn cwuf7(&self) -> super::vals::Cwuf {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Cwuf::from_bits(val as u8)
        }
        #[doc = "clear wake-up pin flag for WUFx (x=7 to 1)."]
        #[inline(always)]
        pub const fn set_cwuf7(&mut self, val: super::vals::Cwuf) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
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
                .field("cwuf3", &self.cwuf3())
                .field("cwuf4", &self.cwuf4())
                .field("cwuf5", &self.cwuf5())
                .field("cwuf6", &self.cwuf6())
                .field("cwuf7", &self.cwuf7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wuscr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wuscr {{ cwuf1: {:?}, cwuf2: {:?}, cwuf3: {:?}, cwuf4: {:?}, cwuf5: {:?}, cwuf6: {:?}, cwuf7: {:?} }}",
                self.cwuf1(),
                self.cwuf2(),
                self.cwuf3(),
                self.cwuf4(),
                self.cwuf5(),
                self.cwuf6(),
                self.cwuf7()
            )
        }
    }
    #[doc = "PWR wake-up status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wusr(pub u32);
    impl Wusr {
        #[doc = "wake-up pin WUFx flag."]
        #[must_use]
        #[inline(always)]
        pub const fn wuf1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "wake-up pin WUFx flag."]
        #[inline(always)]
        pub const fn set_wuf1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "wake-up pin WUFx flag."]
        #[must_use]
        #[inline(always)]
        pub const fn wuf2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "wake-up pin WUFx flag."]
        #[inline(always)]
        pub const fn set_wuf2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "wake-up pin WUFx flag."]
        #[must_use]
        #[inline(always)]
        pub const fn wuf3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "wake-up pin WUFx flag."]
        #[inline(always)]
        pub const fn set_wuf3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "wake-up pin WUFx flag."]
        #[must_use]
        #[inline(always)]
        pub const fn wuf4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "wake-up pin WUFx flag."]
        #[inline(always)]
        pub const fn set_wuf4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "wake-up pin WUFx flag."]
        #[must_use]
        #[inline(always)]
        pub const fn wuf5(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "wake-up pin WUFx flag."]
        #[inline(always)]
        pub const fn set_wuf5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "wake-up pin WUFx flag."]
        #[must_use]
        #[inline(always)]
        pub const fn wuf6(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "wake-up pin WUFx flag."]
        #[inline(always)]
        pub const fn set_wuf6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "wake-up pin WUFx flag."]
        #[must_use]
        #[inline(always)]
        pub const fn wuf7(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "wake-up pin WUFx flag."]
        #[inline(always)]
        pub const fn set_wuf7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
                .field("wuf3", &self.wuf3())
                .field("wuf4", &self.wuf4())
                .field("wuf5", &self.wuf5())
                .field("wuf6", &self.wuf6())
                .field("wuf7", &self.wuf7())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wusr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Wusr {{ wuf1: {=bool:?}, wuf2: {=bool:?}, wuf3: {=bool:?}, wuf4: {=bool:?}, wuf5: {=bool:?}, wuf6: {=bool:?}, wuf7: {=bool:?} }}" , self . wuf1 () , self . wuf2 () , self . wuf3 () , self . wuf4 () , self . wuf5 () , self . wuf6 () , self . wuf7 ())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cssf {
        _RESERVED_0 = 0x0,
        #[doc = "STOPF and SBF flags cleared."]
        Clear = 0x01,
    }
    impl Cssf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cssf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cssf {
        #[inline(always)]
        fn from(val: u8) -> Cssf {
            Cssf::from_bits(val)
        }
    }
    impl From<Cssf> for u8 {
        #[inline(always)]
        fn from(val: Cssf) -> u8 {
            Cssf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cwuf {
        _RESERVED_0 = 0x0,
        #[doc = "Writing 1 clears the WUFx wake-up pin flag (bit is cleared to 0 by hardware)."]
        Clear = 0x01,
    }
    impl Cwuf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cwuf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cwuf {
        #[inline(always)]
        fn from(val: u8) -> Cwuf {
            Cwuf::from_bits(val)
        }
    }
    impl From<Cwuf> for u8 {
        #[inline(always)]
        fn from(val: Cwuf) -> u8 {
            Cwuf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Flps {
        #[doc = "Flash memory remains in normal mode when the system enters Stop mode (quickrestart time)."]
        Normal = 0x0,
        #[doc = "Flash memory enters low-power mode when the system enters Stop mode(low-power consumption)."]
        Stop = 0x01,
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
        #[doc = "Stop 0 mode when entering DeepSleep."]
        Stop0 = 0x0,
        #[doc = "Stop 1mode when entering DeepSleep.."]
        Stop1 = 0x01,
        #[doc = "Standby mode when entering DeepSleep."]
        Standby = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Lpms {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lpms {
            unsafe { core::mem::transmute(val & 0x03) }
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
    pub enum Pvdo {
        #[doc = "Vless thansub>DDless than/sub> is equal or higher than the PVD threshold."]
        Above = 0x0,
        #[doc = "Vless thansub>DDless than/sub> is lower than the PVD threshold."]
        Below = 0x01,
    }
    impl Pvdo {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pvdo {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pvdo {
        #[inline(always)]
        fn from(val: u8) -> Pvdo {
            Pvdo::from_bits(val)
        }
    }
    impl From<Pvdo> for u8 {
        #[inline(always)]
        fn from(val: Pvdo) -> u8 {
            Pvdo::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Reten {
        #[doc = "I/O retention mode is disabled."]
        Disabled = 0x0,
        #[doc = "I/O retention mode is enabling for all I/Os except the I/O support the standby functionality and PA13, PA14, PA15, and PB4."]
        Retained = 0x01,
    }
    impl Reten {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Reten {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Reten {
        #[inline(always)]
        fn from(val: u8) -> Reten {
            Reten::from_bits(val)
        }
    }
    impl From<Reten> for u8 {
        #[inline(always)]
        fn from(val: Reten) -> u8 {
            Reten::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum SramSo {
        #[doc = "RAM content is kept in Stop mode."]
        Retained = 0x0,
        #[doc = "RAM content is lost in Stop mode."]
        Disabled = 0x01,
    }
    impl SramSo {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SramSo {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SramSo {
        #[inline(always)]
        fn from(val: u8) -> SramSo {
            SramSo::from_bits(val)
        }
    }
    impl From<SramSo> for u8 {
        #[inline(always)]
        fn from(val: SramSo) -> u8 {
            SramSo::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wupen {
        #[doc = "An event on WUPx pin does not wake-up the system from Standby mode."]
        Enabled = 0x0,
        #[doc = "A rising or falling edge on WUPx pin wakes up the system from Standby mode."]
        Disabled = 0x01,
    }
    impl Wupen {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wupen {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wupen {
        #[inline(always)]
        fn from(val: u8) -> Wupen {
            Wupen::from_bits(val)
        }
    }
    impl From<Wupen> for u8 {
        #[inline(always)]
        fn from(val: Wupen) -> u8 {
            Wupen::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wupp {
        #[doc = "Detection on high level (rising edge)."]
        Rising = 0x0,
        #[doc = "Detection on low level (falling edge)."]
        Falling = 0x01,
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
        #[doc = "No pull-up."]
        Disabled = 0x0,
        #[doc = "Pull-up."]
        PullUp = 0x01,
        #[doc = "Pull-down."]
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
