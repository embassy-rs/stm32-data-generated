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
    #[doc = "PWR control register 1."]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "PWR status register 1."]
    #[inline(always)]
    pub const fn sr1(self) -> crate::common::Reg<regs::Sr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "PWR control status register 1."]
    #[inline(always)]
    pub const fn csr1(self) -> crate::common::Reg<regs::Csr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "PWR control status register 2."]
    #[inline(always)]
    pub const fn csr2(self) -> crate::common::Reg<regs::Csr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "PWR CPU control status register 3."]
    #[inline(always)]
    pub const fn csr3(self) -> crate::common::Reg<regs::Csr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "PWR control status register 4."]
    #[inline(always)]
    pub const fn csr4(self) -> crate::common::Reg<regs::Csr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "PWR wakeup clear register."]
    #[inline(always)]
    pub const fn wkupcr(self) -> crate::common::Reg<regs::Wkupcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "PWR wakeup flag register."]
    #[inline(always)]
    pub const fn wkupfr(self) -> crate::common::Reg<regs::Wkupfr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "PWR wakeup enable and polarity register."]
    #[inline(always)]
    pub const fn wkupepr(self) -> crate::common::Reg<regs::Wkupepr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "PWR USB Type-C and power delivery register."]
    #[inline(always)]
    pub const fn ucpdr(self) -> crate::common::Reg<regs::Ucpdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "PWR apply pull register."]
    #[inline(always)]
    pub const fn apcr(self) -> crate::common::Reg<regs::Apcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "PWR I3C pull-up control register."]
    #[inline(always)]
    pub const fn i3cpucr(self) -> crate::common::Reg<regs::I3cpucr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "PWR XSPI pull-up control register."]
    #[inline(always)]
    pub const fn xspipucr(self) -> crate::common::Reg<regs::Xspipucr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "PWR XSPI pull-down control register."]
    #[inline(always)]
    pub const fn xspipdcr(self) -> crate::common::Reg<regs::Xspipdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
}
pub mod regs {
    #[doc = "PWR apply pull register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apcr(pub u32);
    impl Apcr {
        #[doc = "Applypull-upandpull-downconfiguration."]
        #[must_use]
        #[inline(always)]
        pub const fn apc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Applypull-upandpull-downconfiguration."]
        #[inline(always)]
        pub const fn set_apc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Apcr {
        #[inline(always)]
        fn default() -> Apcr {
            Apcr(0)
        }
    }
    impl core::fmt::Debug for Apcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apcr").field("apc", &self.apc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Apcr {{ apc: {=bool:?} }}", self.apc())
        }
    }
    #[doc = "PWR control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "System Stop mode voltage scaling selection."]
        #[must_use]
        #[inline(always)]
        pub const fn svos(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "System Stop mode voltage scaling selection."]
        #[inline(always)]
        pub const fn set_svos(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Programmable voltage detector enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pvde(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Programmable voltage detector enable."]
        #[inline(always)]
        pub const fn set_pvde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Programmable voltage detector level selection."]
        #[must_use]
        #[inline(always)]
        pub const fn pls(&self) -> super::vals::Pls {
            let val = (self.0 >> 5usize) & 0x07;
            super::vals::Pls::from_bits(val as u8)
        }
        #[doc = "Programmable voltage detector level selection."]
        #[inline(always)]
        pub const fn set_pls(&mut self, val: super::vals::Pls) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
        }
        #[doc = "Disable backup domain write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn dbp(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Disable backup domain write protection."]
        #[inline(always)]
        pub const fn set_dbp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Flash low-power mode in Stop mode."]
        #[must_use]
        #[inline(always)]
        pub const fn flps(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Flash low-power mode in Stop mode."]
        #[inline(always)]
        pub const fn set_flps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Analog switch VBoost control."]
        #[must_use]
        #[inline(always)]
        pub const fn booste(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Analog switch VBoost control."]
        #[inline(always)]
        pub const fn set_booste(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Analog voltage ready."]
        #[must_use]
        #[inline(always)]
        pub const fn avdready(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Analog voltage ready."]
        #[inline(always)]
        pub const fn set_avdready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Peripheral voltage monitor on VDDA enable."]
        #[must_use]
        #[inline(always)]
        pub const fn avden(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral voltage monitor on VDDA enable."]
        #[inline(always)]
        pub const fn set_avden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Analog voltage detector level selection."]
        #[must_use]
        #[inline(always)]
        pub const fn als(&self) -> super::vals::Als {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::Als::from_bits(val as u8)
        }
        #[doc = "Analog voltage detector level selection."]
        #[inline(always)]
        pub const fn set_als(&mut self, val: super::vals::Als) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
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
                .field("svos", &self.svos())
                .field("pvde", &self.pvde())
                .field("pls", &self.pls())
                .field("dbp", &self.dbp())
                .field("flps", &self.flps())
                .field("booste", &self.booste())
                .field("avdready", &self.avdready())
                .field("avden", &self.avden())
                .field("als", &self.als())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr1 {{ svos: {=bool:?}, pvde: {=bool:?}, pls: {:?}, dbp: {=bool:?}, flps: {=bool:?}, booste: {=bool:?}, avdready: {=bool:?}, avden: {=bool:?}, als: {:?} }}",
                self.svos(),
                self.pvde(),
                self.pls(),
                self.dbp(),
                self.flps(),
                self.booste(),
                self.avdready(),
                self.avden(),
                self.als()
            )
        }
    }
    #[doc = "PWR control status register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr1(pub u32);
    impl Csr1 {
        #[doc = "Backup regulator enable."]
        #[must_use]
        #[inline(always)]
        pub const fn bren(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Backup regulator enable."]
        #[inline(always)]
        pub const fn set_bren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Vless thansub>BATless than/sub> and temperature monitoring enable."]
        #[must_use]
        #[inline(always)]
        pub const fn monen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Vless thansub>BATless than/sub> and temperature monitoring enable."]
        #[inline(always)]
        pub const fn set_monen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Backup regulator ready."]
        #[must_use]
        #[inline(always)]
        pub const fn brrdy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Backup regulator ready."]
        #[inline(always)]
        pub const fn set_brrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Vless thansub>BATless than/sub> level monitoring versus low threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn vbatl(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Vless thansub>BATless than/sub> level monitoring versus low threshold."]
        #[inline(always)]
        pub const fn set_vbatl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Vless thansub>BATless than/sub> level monitoring versus high threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn vbath(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Vless thansub>BATless than/sub> level monitoring versus high threshold."]
        #[inline(always)]
        pub const fn set_vbath(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Temperature level monitoring versus low threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn templ(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Temperature level monitoring versus low threshold."]
        #[inline(always)]
        pub const fn set_templ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Temperature level monitoring versus high threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn temph(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Temperature level monitoring versus high threshold."]
        #[inline(always)]
        pub const fn set_temph(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Csr1 {
        #[inline(always)]
        fn default() -> Csr1 {
            Csr1(0)
        }
    }
    impl core::fmt::Debug for Csr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csr1")
                .field("bren", &self.bren())
                .field("monen", &self.monen())
                .field("brrdy", &self.brrdy())
                .field("vbatl", &self.vbatl())
                .field("vbath", &self.vbath())
                .field("templ", &self.templ())
                .field("temph", &self.temph())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csr1 {{ bren: {=bool:?}, monen: {=bool:?}, brrdy: {=bool:?}, vbatl: {=bool:?}, vbath: {=bool:?}, templ: {=bool:?}, temph: {=bool:?} }}",
                self.bren(),
                self.monen(),
                self.brrdy(),
                self.vbatl(),
                self.vbath(),
                self.templ(),
                self.temph()
            )
        }
    }
    #[doc = "PWR control status register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr2(pub u32);
    impl Csr2 {
        #[doc = "Power management unit bypass."]
        #[must_use]
        #[inline(always)]
        pub const fn bypass(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Power management unit bypass."]
        #[inline(always)]
        pub const fn set_bypass(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Integrated low drop-out regulator enable (read only)."]
        #[must_use]
        #[inline(always)]
        pub const fn ldoen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Integrated low drop-out regulator enable (read only)."]
        #[inline(always)]
        pub const fn set_ldoen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Integrated SMPS step-down converter enable (read only)."]
        #[must_use]
        #[inline(always)]
        pub const fn smpsen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Integrated SMPS step-down converter enable (read only)."]
        #[inline(always)]
        pub const fn set_smpsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "VBAT charging enable."]
        #[must_use]
        #[inline(always)]
        pub const fn vbe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "VBAT charging enable."]
        #[inline(always)]
        pub const fn set_vbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "VBAT charging resistor selection."]
        #[must_use]
        #[inline(always)]
        pub const fn vbrs(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "VBAT charging resistor selection."]
        #[inline(always)]
        pub const fn set_vbrs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Integrated SMPS step-down converter supply ready."]
        #[must_use]
        #[inline(always)]
        pub const fn sdrdy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Integrated SMPS step-down converter supply ready."]
        #[inline(always)]
        pub const fn set_sdrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Csr2 {
        #[inline(always)]
        fn default() -> Csr2 {
            Csr2(0)
        }
    }
    impl core::fmt::Debug for Csr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csr2")
                .field("bypass", &self.bypass())
                .field("ldoen", &self.ldoen())
                .field("smpsen", &self.smpsen())
                .field("vbe", &self.vbe())
                .field("vbrs", &self.vbrs())
                .field("sdrdy", &self.sdrdy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csr2 {{ bypass: {=bool:?}, ldoen: {=bool:?}, smpsen: {=bool:?}, vbe: {=bool:?}, vbrs: {=bool:?}, sdrdy: {=bool:?} }}",
                self.bypass(),
                self.ldoen(),
                self.smpsen(),
                self.vbe(),
                self.vbrs(),
                self.sdrdy()
            )
        }
    }
    #[doc = "PWR CPU control status register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr3(pub u32);
    impl Csr3 {
        #[doc = "Power-down Deepsleep."]
        #[must_use]
        #[inline(always)]
        pub const fn pdds(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Power-down Deepsleep."]
        #[inline(always)]
        pub const fn set_pdds(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear Standby and Stop flags (always read as 0)."]
        #[must_use]
        #[inline(always)]
        pub const fn cssf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear Standby and Stop flags (always read as 0)."]
        #[inline(always)]
        pub const fn set_cssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Stop flag."]
        #[must_use]
        #[inline(always)]
        pub const fn stopf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Stop flag."]
        #[inline(always)]
        pub const fn set_stopf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "System Standby flag."]
        #[must_use]
        #[inline(always)]
        pub const fn sbf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "System Standby flag."]
        #[inline(always)]
        pub const fn set_sbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Csr3 {
        #[inline(always)]
        fn default() -> Csr3 {
            Csr3(0)
        }
    }
    impl core::fmt::Debug for Csr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csr3")
                .field("pdds", &self.pdds())
                .field("cssf", &self.cssf())
                .field("stopf", &self.stopf())
                .field("sbf", &self.sbf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csr3 {{ pdds: {=bool:?}, cssf: {=bool:?}, stopf: {=bool:?}, sbf: {=bool:?} }}",
                self.pdds(),
                self.cssf(),
                self.stopf(),
                self.sbf()
            )
        }
    }
    #[doc = "PWR control status register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr4(pub u32);
    impl Csr4 {
        #[doc = "Voltage scaling selection according to performance."]
        #[must_use]
        #[inline(always)]
        pub const fn vos(&self) -> super::vals::Vos {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Vos::from_bits(val as u8)
        }
        #[doc = "Voltage scaling selection according to performance."]
        #[inline(always)]
        pub const fn set_vos(&mut self, val: super::vals::Vos) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "VOS ready bit for Vless thansub>COREless than/sub> voltage scaling output selection."]
        #[must_use]
        #[inline(always)]
        pub const fn vosrdy(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "VOS ready bit for Vless thansub>COREless than/sub> voltage scaling output selection."]
        #[inline(always)]
        pub const fn set_vosrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Csr4 {
        #[inline(always)]
        fn default() -> Csr4 {
            Csr4(0)
        }
    }
    impl core::fmt::Debug for Csr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csr4")
                .field("vos", &self.vos())
                .field("vosrdy", &self.vosrdy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Csr4 {{ vos: {:?}, vosrdy: {=bool:?} }}", self.vos(), self.vosrdy())
        }
    }
    #[doc = "PWR I3C pull-up control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct I3cpucr(pub u32);
    impl I3cpucr {
        #[doc = "Port PA15 I3C pull-up bit."]
        #[must_use]
        #[inline(always)]
        pub const fn i3cpa15_pu(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Port PA15 I3C pull-up bit."]
        #[inline(always)]
        pub const fn set_i3cpa15_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Port PB3 I3C pull-up bit."]
        #[must_use]
        #[inline(always)]
        pub const fn i3cpb3_pu(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Port PB3 I3C pull-up bit."]
        #[inline(always)]
        pub const fn set_i3cpb3_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Port PB4 I3C pull-up bit."]
        #[must_use]
        #[inline(always)]
        pub const fn i3cpb4_pu(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Port PB4 I3C pull-up bit."]
        #[inline(always)]
        pub const fn set_i3cpb4_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Port PB8 I3C pull-up bit."]
        #[must_use]
        #[inline(always)]
        pub const fn i3cpb8_pu(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Port PB8 I3C pull-up bit."]
        #[inline(always)]
        pub const fn set_i3cpb8_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Port PB9 I3C pull-up bit."]
        #[must_use]
        #[inline(always)]
        pub const fn i3cpb9_pu(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Port PB9 I3C pull-up bit."]
        #[inline(always)]
        pub const fn set_i3cpb9_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Port PC8 I3C pull-up bit."]
        #[must_use]
        #[inline(always)]
        pub const fn i3cpc8_pu(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Port PC8 I3C pull-up bit."]
        #[inline(always)]
        pub const fn set_i3cpc8_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Port PC9 I3C pull-up bit."]
        #[must_use]
        #[inline(always)]
        pub const fn i3cpc9_pu(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Port PC9 I3C pull-up bit."]
        #[inline(always)]
        pub const fn set_i3cpc9_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Port PD0 I3C pull-up bit."]
        #[must_use]
        #[inline(always)]
        pub const fn i3cpd0_pu(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Port PD0 I3C pull-up bit."]
        #[inline(always)]
        pub const fn set_i3cpd0_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Port PD1 I3C pull-up bit."]
        #[must_use]
        #[inline(always)]
        pub const fn i3cpd1_pu(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Port PD1 I3C pull-up bit."]
        #[inline(always)]
        pub const fn set_i3cpd1_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Port PD6 I3C pull-up bit."]
        #[must_use]
        #[inline(always)]
        pub const fn i3cpd6_pu(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Port PD6 I3C pull-up bit."]
        #[inline(always)]
        pub const fn set_i3cpd6_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Port PD7 I3C pull-up bit."]
        #[must_use]
        #[inline(always)]
        pub const fn i3cpd7_pu(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Port PD7 I3C pull-up bit."]
        #[inline(always)]
        pub const fn set_i3cpd7_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Port PE5 I3C pull-up bit."]
        #[must_use]
        #[inline(always)]
        pub const fn i3cpe5_pu(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Port PE5 I3C pull-up bit."]
        #[inline(always)]
        pub const fn set_i3cpe5_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Port PE6 I3C pull-up bit."]
        #[must_use]
        #[inline(always)]
        pub const fn i3cpe6_pu(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Port PE6 I3C pull-up bit."]
        #[inline(always)]
        pub const fn set_i3cpe6_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Port PG13 I3C pull-up bit."]
        #[must_use]
        #[inline(always)]
        pub const fn i3cpg13_pu(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Port PG13 I3C pull-up bit."]
        #[inline(always)]
        pub const fn set_i3cpg13_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Port PG14 I3C pull-up bit."]
        #[must_use]
        #[inline(always)]
        pub const fn i3cpg14_pu(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Port PG14 I3C pull-up bit."]
        #[inline(always)]
        pub const fn set_i3cpg14_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Port PH11 I3C pull-up bit."]
        #[must_use]
        #[inline(always)]
        pub const fn i3cph11_pu(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Port PH11 I3C pull-up bit."]
        #[inline(always)]
        pub const fn set_i3cph11_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Port PH12 I3C pull-up bit."]
        #[must_use]
        #[inline(always)]
        pub const fn i3cph12_pu(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Port PH12 I3C pull-up bit."]
        #[inline(always)]
        pub const fn set_i3cph12_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Port PI2 I3C pull-up bit."]
        #[must_use]
        #[inline(always)]
        pub const fn i3cpi2_pu(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Port PI2 I3C pull-up bit."]
        #[inline(always)]
        pub const fn set_i3cpi2_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for I3cpucr {
        #[inline(always)]
        fn default() -> I3cpucr {
            I3cpucr(0)
        }
    }
    impl core::fmt::Debug for I3cpucr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("I3cpucr")
                .field("i3cpa15_pu", &self.i3cpa15_pu())
                .field("i3cpb3_pu", &self.i3cpb3_pu())
                .field("i3cpb4_pu", &self.i3cpb4_pu())
                .field("i3cpb8_pu", &self.i3cpb8_pu())
                .field("i3cpb9_pu", &self.i3cpb9_pu())
                .field("i3cpc8_pu", &self.i3cpc8_pu())
                .field("i3cpc9_pu", &self.i3cpc9_pu())
                .field("i3cpd0_pu", &self.i3cpd0_pu())
                .field("i3cpd1_pu", &self.i3cpd1_pu())
                .field("i3cpd6_pu", &self.i3cpd6_pu())
                .field("i3cpd7_pu", &self.i3cpd7_pu())
                .field("i3cpe5_pu", &self.i3cpe5_pu())
                .field("i3cpe6_pu", &self.i3cpe6_pu())
                .field("i3cpg13_pu", &self.i3cpg13_pu())
                .field("i3cpg14_pu", &self.i3cpg14_pu())
                .field("i3cph11_pu", &self.i3cph11_pu())
                .field("i3cph12_pu", &self.i3cph12_pu())
                .field("i3cpi2_pu", &self.i3cpi2_pu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for I3cpucr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "I3cpucr {{ i3cpa15_pu: {=bool:?}, i3cpb3_pu: {=bool:?}, i3cpb4_pu: {=bool:?}, i3cpb8_pu: {=bool:?}, i3cpb9_pu: {=bool:?}, i3cpc8_pu: {=bool:?}, i3cpc9_pu: {=bool:?}, i3cpd0_pu: {=bool:?}, i3cpd1_pu: {=bool:?}, i3cpd6_pu: {=bool:?}, i3cpd7_pu: {=bool:?}, i3cpe5_pu: {=bool:?}, i3cpe6_pu: {=bool:?}, i3cpg13_pu: {=bool:?}, i3cpg14_pu: {=bool:?}, i3cph11_pu: {=bool:?}, i3cph12_pu: {=bool:?}, i3cpi2_pu: {=bool:?} }}",
                self.i3cpa15_pu(),
                self.i3cpb3_pu(),
                self.i3cpb4_pu(),
                self.i3cpb8_pu(),
                self.i3cpb9_pu(),
                self.i3cpc8_pu(),
                self.i3cpc9_pu(),
                self.i3cpd0_pu(),
                self.i3cpd1_pu(),
                self.i3cpd6_pu(),
                self.i3cpd7_pu(),
                self.i3cpe5_pu(),
                self.i3cpe6_pu(),
                self.i3cpg13_pu(),
                self.i3cpg14_pu(),
                self.i3cph11_pu(),
                self.i3cph12_pu(),
                self.i3cpi2_pu()
            )
        }
    }
    #[doc = "PWR status register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr1(pub u32);
    impl Sr1 {
        #[doc = "VOS currently applied for Vless thansub>COREless than/sub> voltage scaling selection."]
        #[must_use]
        #[inline(always)]
        pub const fn actvos(&self) -> super::vals::Actvos {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Actvos::from_bits(val as u8)
        }
        #[doc = "VOS currently applied for Vless thansub>COREless than/sub> voltage scaling selection."]
        #[inline(always)]
        pub const fn set_actvos(&mut self, val: super::vals::Actvos) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Voltage levels ready bit for currently used ACTVOS."]
        #[must_use]
        #[inline(always)]
        pub const fn actvosrdy(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Voltage levels ready bit for currently used ACTVOS."]
        #[inline(always)]
        pub const fn set_actvosrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Programmable voltage detect output."]
        #[must_use]
        #[inline(always)]
        pub const fn pvdo(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Programmable voltage detect output."]
        #[inline(always)]
        pub const fn set_pvdo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Analog voltage detector output on VDDA."]
        #[must_use]
        #[inline(always)]
        pub const fn avdo(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Analog voltage detector output on VDDA."]
        #[inline(always)]
        pub const fn set_avdo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Sr1 {
        #[inline(always)]
        fn default() -> Sr1 {
            Sr1(0)
        }
    }
    impl core::fmt::Debug for Sr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr1")
                .field("actvos", &self.actvos())
                .field("actvosrdy", &self.actvosrdy())
                .field("pvdo", &self.pvdo())
                .field("avdo", &self.avdo())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr1 {{ actvos: {:?}, actvosrdy: {=bool:?}, pvdo: {=bool:?}, avdo: {=bool:?} }}",
                self.actvos(),
                self.actvosrdy(),
                self.pvdo(),
                self.avdo()
            )
        }
    }
    #[doc = "PWR USB Type-C and power delivery register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ucpdr(pub u32);
    impl Ucpdr {
        #[doc = "UCPDdeadbatterydisable."]
        #[must_use]
        #[inline(always)]
        pub const fn ucpd_dbdis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "UCPDdeadbatterydisable."]
        #[inline(always)]
        pub const fn set_ucpd_dbdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "UCPDStandbymode."]
        #[must_use]
        #[inline(always)]
        pub const fn ucpd_stby(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "UCPDStandbymode."]
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
    #[doc = "PWR wakeup clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wkupcr(pub u32);
    impl Wkupcr {
        #[doc = "Clear wakeup pin flag for WKUP1."]
        #[must_use]
        #[inline(always)]
        pub const fn wkupc1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear wakeup pin flag for WKUP1."]
        #[inline(always)]
        pub const fn set_wkupc1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear wakeup pin flag for WKUP2."]
        #[must_use]
        #[inline(always)]
        pub const fn wkupc2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear wakeup pin flag for WKUP2."]
        #[inline(always)]
        pub const fn set_wkupc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Clear wakeup pin flag for WKUP3."]
        #[must_use]
        #[inline(always)]
        pub const fn wkupc3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Clear wakeup pin flag for WKUP3."]
        #[inline(always)]
        pub const fn set_wkupc3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Clear wakeup pin flag for WKUP4."]
        #[must_use]
        #[inline(always)]
        pub const fn wkupc4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Clear wakeup pin flag for WKUP4."]
        #[inline(always)]
        pub const fn set_wkupc4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Clear wakeup pin flag for WKUP5."]
        #[must_use]
        #[inline(always)]
        pub const fn wkupc5(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Clear wakeup pin flag for WKUP5."]
        #[inline(always)]
        pub const fn set_wkupc5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Clear wakeup pin flag for WKUP6."]
        #[must_use]
        #[inline(always)]
        pub const fn wkupc6(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Clear wakeup pin flag for WKUP6."]
        #[inline(always)]
        pub const fn set_wkupc6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Clear wakeup pin flag for WKUP7."]
        #[must_use]
        #[inline(always)]
        pub const fn wkupc7(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Clear wakeup pin flag for WKUP7."]
        #[inline(always)]
        pub const fn set_wkupc7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Clear wakeup pin flag for WKUP8."]
        #[must_use]
        #[inline(always)]
        pub const fn wkupc8(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Clear wakeup pin flag for WKUP8."]
        #[inline(always)]
        pub const fn set_wkupc8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Wkupcr {
        #[inline(always)]
        fn default() -> Wkupcr {
            Wkupcr(0)
        }
    }
    impl core::fmt::Debug for Wkupcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wkupcr")
                .field("wkupc1", &self.wkupc1())
                .field("wkupc2", &self.wkupc2())
                .field("wkupc3", &self.wkupc3())
                .field("wkupc4", &self.wkupc4())
                .field("wkupc5", &self.wkupc5())
                .field("wkupc6", &self.wkupc6())
                .field("wkupc7", &self.wkupc7())
                .field("wkupc8", &self.wkupc8())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wkupcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wkupcr {{ wkupc1: {=bool:?}, wkupc2: {=bool:?}, wkupc3: {=bool:?}, wkupc4: {=bool:?}, wkupc5: {=bool:?}, wkupc6: {=bool:?}, wkupc7: {=bool:?}, wkupc8: {=bool:?} }}",
                self.wkupc1(),
                self.wkupc2(),
                self.wkupc3(),
                self.wkupc4(),
                self.wkupc5(),
                self.wkupc6(),
                self.wkupc7(),
                self.wkupc8()
            )
        }
    }
    #[doc = "PWR wakeup enable and polarity register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wkupepr(pub u32);
    impl Wkupepr {
        #[doc = "Enable wakeup pin WKUPn."]
        #[must_use]
        #[inline(always)]
        pub const fn wkupen1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable wakeup pin WKUPn."]
        #[inline(always)]
        pub const fn set_wkupen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Enable wakeup pin WKUPn."]
        #[must_use]
        #[inline(always)]
        pub const fn wkupen2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Enable wakeup pin WKUPn."]
        #[inline(always)]
        pub const fn set_wkupen2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Enable wakeup pin WKUPn."]
        #[must_use]
        #[inline(always)]
        pub const fn wkupen3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Enable wakeup pin WKUPn."]
        #[inline(always)]
        pub const fn set_wkupen3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Enable wakeup pin WKUPn."]
        #[must_use]
        #[inline(always)]
        pub const fn wkupen4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Enable wakeup pin WKUPn."]
        #[inline(always)]
        pub const fn set_wkupen4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Enable wakeup pin WKUPn."]
        #[must_use]
        #[inline(always)]
        pub const fn wkupen5(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Enable wakeup pin WKUPn."]
        #[inline(always)]
        pub const fn set_wkupen5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Enable wakeup pin WKUPn."]
        #[must_use]
        #[inline(always)]
        pub const fn wkupen6(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Enable wakeup pin WKUPn."]
        #[inline(always)]
        pub const fn set_wkupen6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Enable wakeup pin WKUPn."]
        #[must_use]
        #[inline(always)]
        pub const fn wkupen7(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable wakeup pin WKUPn."]
        #[inline(always)]
        pub const fn set_wkupen7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Enable wakeup pin WKUPn."]
        #[must_use]
        #[inline(always)]
        pub const fn wkupen8(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Enable wakeup pin WKUPn."]
        #[inline(always)]
        pub const fn set_wkupen8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Wakeup pin polarity bit for WKUPn."]
        #[must_use]
        #[inline(always)]
        pub const fn wkupp1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup pin polarity bit for WKUPn."]
        #[inline(always)]
        pub const fn set_wkupp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Wakeup pin polarity bit for WKUPn."]
        #[must_use]
        #[inline(always)]
        pub const fn wkupp2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup pin polarity bit for WKUPn."]
        #[inline(always)]
        pub const fn set_wkupp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Wakeup pin polarity bit for WKUPn."]
        #[must_use]
        #[inline(always)]
        pub const fn wkupp3(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup pin polarity bit for WKUPn."]
        #[inline(always)]
        pub const fn set_wkupp3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Wakeup pin polarity bit for WKUPn."]
        #[must_use]
        #[inline(always)]
        pub const fn wkupp4(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup pin polarity bit for WKUPn."]
        #[inline(always)]
        pub const fn set_wkupp4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Wakeup pin polarity bit for WKUPn."]
        #[must_use]
        #[inline(always)]
        pub const fn wkupp5(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup pin polarity bit for WKUPn."]
        #[inline(always)]
        pub const fn set_wkupp5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Wakeup pin polarity bit for WKUPn."]
        #[must_use]
        #[inline(always)]
        pub const fn wkupp6(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup pin polarity bit for WKUPn."]
        #[inline(always)]
        pub const fn set_wkupp6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Wakeup pin polarity bit for WKUPn."]
        #[must_use]
        #[inline(always)]
        pub const fn wkupp7(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup pin polarity bit for WKUPn."]
        #[inline(always)]
        pub const fn set_wkupp7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Wakeup pin polarity bit for WKUPn."]
        #[must_use]
        #[inline(always)]
        pub const fn wkupp8(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup pin polarity bit for WKUPn."]
        #[inline(always)]
        pub const fn set_wkupp8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Wakeup pin pull configuration for WKUPn."]
        #[must_use]
        #[inline(always)]
        pub const fn wkuppupd1(&self) -> super::vals::Wkuppupd1 {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Wkuppupd1::from_bits(val as u8)
        }
        #[doc = "Wakeup pin pull configuration for WKUPn."]
        #[inline(always)]
        pub const fn set_wkuppupd1(&mut self, val: super::vals::Wkuppupd1) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Wakeup pin pull configuration for WKUPn."]
        #[must_use]
        #[inline(always)]
        pub const fn wkuppupd2(&self) -> super::vals::Wkuppupd2 {
            let val = (self.0 >> 18usize) & 0x03;
            super::vals::Wkuppupd2::from_bits(val as u8)
        }
        #[doc = "Wakeup pin pull configuration for WKUPn."]
        #[inline(always)]
        pub const fn set_wkuppupd2(&mut self, val: super::vals::Wkuppupd2) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
        }
        #[doc = "Wakeup pin pull configuration for WKUPn."]
        #[must_use]
        #[inline(always)]
        pub const fn wkuppupd3(&self) -> super::vals::Wkuppupd3 {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::Wkuppupd3::from_bits(val as u8)
        }
        #[doc = "Wakeup pin pull configuration for WKUPn."]
        #[inline(always)]
        pub const fn set_wkuppupd3(&mut self, val: super::vals::Wkuppupd3) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "Wakeup pin pull configuration for WKUPn."]
        #[must_use]
        #[inline(always)]
        pub const fn wkuppupd4(&self) -> super::vals::Wkuppupd4 {
            let val = (self.0 >> 22usize) & 0x03;
            super::vals::Wkuppupd4::from_bits(val as u8)
        }
        #[doc = "Wakeup pin pull configuration for WKUPn."]
        #[inline(always)]
        pub const fn set_wkuppupd4(&mut self, val: super::vals::Wkuppupd4) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
        }
        #[doc = "Wakeup pin pull configuration for WKUPn."]
        #[must_use]
        #[inline(always)]
        pub const fn wkuppupd5(&self) -> super::vals::Wkuppupd5 {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Wkuppupd5::from_bits(val as u8)
        }
        #[doc = "Wakeup pin pull configuration for WKUPn."]
        #[inline(always)]
        pub const fn set_wkuppupd5(&mut self, val: super::vals::Wkuppupd5) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "Wakeup pin pull configuration for WKUPn."]
        #[must_use]
        #[inline(always)]
        pub const fn wkuppupd6(&self) -> super::vals::Wkuppupd6 {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::Wkuppupd6::from_bits(val as u8)
        }
        #[doc = "Wakeup pin pull configuration for WKUPn."]
        #[inline(always)]
        pub const fn set_wkuppupd6(&mut self, val: super::vals::Wkuppupd6) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "Wakeup pin pull configuration for WKUPn."]
        #[must_use]
        #[inline(always)]
        pub const fn wkuppupd7(&self) -> super::vals::Wkuppupd7 {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Wkuppupd7::from_bits(val as u8)
        }
        #[doc = "Wakeup pin pull configuration for WKUPn."]
        #[inline(always)]
        pub const fn set_wkuppupd7(&mut self, val: super::vals::Wkuppupd7) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "Wakeup pin pull configuration for WKUPn."]
        #[must_use]
        #[inline(always)]
        pub const fn wkuppupd8(&self) -> super::vals::Wkuppupd8 {
            let val = (self.0 >> 30usize) & 0x03;
            super::vals::Wkuppupd8::from_bits(val as u8)
        }
        #[doc = "Wakeup pin pull configuration for WKUPn."]
        #[inline(always)]
        pub const fn set_wkuppupd8(&mut self, val: super::vals::Wkuppupd8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
        }
    }
    impl Default for Wkupepr {
        #[inline(always)]
        fn default() -> Wkupepr {
            Wkupepr(0)
        }
    }
    impl core::fmt::Debug for Wkupepr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wkupepr")
                .field("wkupen1", &self.wkupen1())
                .field("wkupen2", &self.wkupen2())
                .field("wkupen3", &self.wkupen3())
                .field("wkupen4", &self.wkupen4())
                .field("wkupen5", &self.wkupen5())
                .field("wkupen6", &self.wkupen6())
                .field("wkupen7", &self.wkupen7())
                .field("wkupen8", &self.wkupen8())
                .field("wkupp1", &self.wkupp1())
                .field("wkupp2", &self.wkupp2())
                .field("wkupp3", &self.wkupp3())
                .field("wkupp4", &self.wkupp4())
                .field("wkupp5", &self.wkupp5())
                .field("wkupp6", &self.wkupp6())
                .field("wkupp7", &self.wkupp7())
                .field("wkupp8", &self.wkupp8())
                .field("wkuppupd1", &self.wkuppupd1())
                .field("wkuppupd2", &self.wkuppupd2())
                .field("wkuppupd3", &self.wkuppupd3())
                .field("wkuppupd4", &self.wkuppupd4())
                .field("wkuppupd5", &self.wkuppupd5())
                .field("wkuppupd6", &self.wkuppupd6())
                .field("wkuppupd7", &self.wkuppupd7())
                .field("wkuppupd8", &self.wkuppupd8())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wkupepr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wkupepr {{ wkupen1: {=bool:?}, wkupen2: {=bool:?}, wkupen3: {=bool:?}, wkupen4: {=bool:?}, wkupen5: {=bool:?}, wkupen6: {=bool:?}, wkupen7: {=bool:?}, wkupen8: {=bool:?}, wkupp1: {=bool:?}, wkupp2: {=bool:?}, wkupp3: {=bool:?}, wkupp4: {=bool:?}, wkupp5: {=bool:?}, wkupp6: {=bool:?}, wkupp7: {=bool:?}, wkupp8: {=bool:?}, wkuppupd1: {:?}, wkuppupd2: {:?}, wkuppupd3: {:?}, wkuppupd4: {:?}, wkuppupd5: {:?}, wkuppupd6: {:?}, wkuppupd7: {:?}, wkuppupd8: {:?} }}",
                self.wkupen1(),
                self.wkupen2(),
                self.wkupen3(),
                self.wkupen4(),
                self.wkupen5(),
                self.wkupen6(),
                self.wkupen7(),
                self.wkupen8(),
                self.wkupp1(),
                self.wkupp2(),
                self.wkupp3(),
                self.wkupp4(),
                self.wkupp5(),
                self.wkupp6(),
                self.wkupp7(),
                self.wkupp8(),
                self.wkuppupd1(),
                self.wkuppupd2(),
                self.wkuppupd3(),
                self.wkuppupd4(),
                self.wkuppupd5(),
                self.wkuppupd6(),
                self.wkuppupd7(),
                self.wkuppupd8()
            )
        }
    }
    #[doc = "PWR wakeup flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wkupfr(pub u32);
    impl Wkupfr {
        #[doc = "Wakeup pin WKUP1 flag."]
        #[must_use]
        #[inline(always)]
        pub const fn wkupf1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup pin WKUP1 flag."]
        #[inline(always)]
        pub const fn set_wkupf1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Wakeup pin WKUP2 flag."]
        #[must_use]
        #[inline(always)]
        pub const fn wkupf2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup pin WKUP2 flag."]
        #[inline(always)]
        pub const fn set_wkupf2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Wakeup pin WKUP3 flag."]
        #[must_use]
        #[inline(always)]
        pub const fn wkupf3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup pin WKUP3 flag."]
        #[inline(always)]
        pub const fn set_wkupf3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Wakeup pin WKUP4 flag."]
        #[must_use]
        #[inline(always)]
        pub const fn wkupf4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup pin WKUP4 flag."]
        #[inline(always)]
        pub const fn set_wkupf4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Wakeup pin WKUP5 flag."]
        #[must_use]
        #[inline(always)]
        pub const fn wkupf5(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup pin WKUP5 flag."]
        #[inline(always)]
        pub const fn set_wkupf5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Wakeup pin WKUP6 flag."]
        #[must_use]
        #[inline(always)]
        pub const fn wkupf6(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup pin WKUP6 flag."]
        #[inline(always)]
        pub const fn set_wkupf6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Wakeup pin WKUP7 flag."]
        #[must_use]
        #[inline(always)]
        pub const fn wkupf7(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup pin WKUP7 flag."]
        #[inline(always)]
        pub const fn set_wkupf7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Wakeup pin WKUP8 flag."]
        #[must_use]
        #[inline(always)]
        pub const fn wkupf8(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup pin WKUP8 flag."]
        #[inline(always)]
        pub const fn set_wkupf8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Wkupfr {
        #[inline(always)]
        fn default() -> Wkupfr {
            Wkupfr(0)
        }
    }
    impl core::fmt::Debug for Wkupfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wkupfr")
                .field("wkupf1", &self.wkupf1())
                .field("wkupf2", &self.wkupf2())
                .field("wkupf3", &self.wkupf3())
                .field("wkupf4", &self.wkupf4())
                .field("wkupf5", &self.wkupf5())
                .field("wkupf6", &self.wkupf6())
                .field("wkupf7", &self.wkupf7())
                .field("wkupf8", &self.wkupf8())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wkupfr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wkupfr {{ wkupf1: {=bool:?}, wkupf2: {=bool:?}, wkupf3: {=bool:?}, wkupf4: {=bool:?}, wkupf5: {=bool:?}, wkupf6: {=bool:?}, wkupf7: {=bool:?}, wkupf8: {=bool:?} }}",
                self.wkupf1(),
                self.wkupf2(),
                self.wkupf3(),
                self.wkupf4(),
                self.wkupf5(),
                self.wkupf6(),
                self.wkupf7(),
                self.wkupf8()
            )
        }
    }
    #[doc = "PWR XSPI pull-down control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Xspipdcr(pub u32);
    impl Xspipdcr {
        #[doc = "Port PG10 XSPI pull-down bit."]
        #[must_use]
        #[inline(always)]
        pub const fn xspipg10_pd(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Port PG10 XSPI pull-down bit."]
        #[inline(always)]
        pub const fn set_xspipg10_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Port PD3 XSPI pull-down bit."]
        #[must_use]
        #[inline(always)]
        pub const fn xspipd3_pd(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Port PD3 XSPI pull-down bit."]
        #[inline(always)]
        pub const fn set_xspipd3_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Port PD2 XSPI pull-down bit."]
        #[must_use]
        #[inline(always)]
        pub const fn xspipd2_pd(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Port PD2 XSPI pull-down bit."]
        #[inline(always)]
        pub const fn set_xspipd2_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Port PD1 XSPI pull-down bit."]
        #[must_use]
        #[inline(always)]
        pub const fn xspipd1_pd(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Port PD1 XSPI pull-down bit."]
        #[inline(always)]
        pub const fn set_xspipd1_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Port PD0 XSPI pull-down bit."]
        #[must_use]
        #[inline(always)]
        pub const fn xspipd0_pd(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Port PD0 XSPI pull-down bit."]
        #[inline(always)]
        pub const fn set_xspipd0_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Port PB2 XSPI pull-down bit."]
        #[must_use]
        #[inline(always)]
        pub const fn xspipb2_pd(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Port PB2 XSPI pull-down bit."]
        #[inline(always)]
        pub const fn set_xspipb2_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Port PB1 XSPI pull-down bit."]
        #[must_use]
        #[inline(always)]
        pub const fn xspipb1_pd(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Port PB1 XSPI pull-down bit."]
        #[inline(always)]
        pub const fn set_xspipb1_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Port PA5 XSPI pull-down bit."]
        #[must_use]
        #[inline(always)]
        pub const fn xspipa5_pd(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Port PA5 XSPI pull-down bit."]
        #[inline(always)]
        pub const fn set_xspipa5_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Port PA4 XSPI pull-down bit."]
        #[must_use]
        #[inline(always)]
        pub const fn xspipa4_pd(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Port PA4 XSPI pull-down bit."]
        #[inline(always)]
        pub const fn set_xspipa4_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Port PA3 XSPI pull-down bit."]
        #[must_use]
        #[inline(always)]
        pub const fn xspipa3_pd(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Port PA3 XSPI pull-down bit."]
        #[inline(always)]
        pub const fn set_xspipa3_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Port PA2 XSPI pull-down bit."]
        #[must_use]
        #[inline(always)]
        pub const fn xspipa2_pd(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Port PA2 XSPI pull-down bit."]
        #[inline(always)]
        pub const fn set_xspipa2_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Port PA0 XSPI pull-down bit."]
        #[must_use]
        #[inline(always)]
        pub const fn xspipa0_pd(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Port PA0 XSPI pull-down bit."]
        #[inline(always)]
        pub const fn set_xspipa0_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Port PF2 XSPI pull-down bit."]
        #[must_use]
        #[inline(always)]
        pub const fn xspipf2_pd(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Port PF2 XSPI pull-down bit."]
        #[inline(always)]
        pub const fn set_xspipf2_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Port PH1 XSPI pull-down bit."]
        #[must_use]
        #[inline(always)]
        pub const fn xspiph1_pd(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Port PH1 XSPI pull-down bit."]
        #[inline(always)]
        pub const fn set_xspiph1_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Port PH0 XSPI pull-down bit."]
        #[must_use]
        #[inline(always)]
        pub const fn xspiph0_pd(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Port PH0 XSPI pull-down bit."]
        #[inline(always)]
        pub const fn set_xspiph0_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Xspipdcr {
        #[inline(always)]
        fn default() -> Xspipdcr {
            Xspipdcr(0)
        }
    }
    impl core::fmt::Debug for Xspipdcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Xspipdcr")
                .field("xspipg10_pd", &self.xspipg10_pd())
                .field("xspipd3_pd", &self.xspipd3_pd())
                .field("xspipd2_pd", &self.xspipd2_pd())
                .field("xspipd1_pd", &self.xspipd1_pd())
                .field("xspipd0_pd", &self.xspipd0_pd())
                .field("xspipb2_pd", &self.xspipb2_pd())
                .field("xspipb1_pd", &self.xspipb1_pd())
                .field("xspipa5_pd", &self.xspipa5_pd())
                .field("xspipa4_pd", &self.xspipa4_pd())
                .field("xspipa3_pd", &self.xspipa3_pd())
                .field("xspipa2_pd", &self.xspipa2_pd())
                .field("xspipa0_pd", &self.xspipa0_pd())
                .field("xspipf2_pd", &self.xspipf2_pd())
                .field("xspiph1_pd", &self.xspiph1_pd())
                .field("xspiph0_pd", &self.xspiph0_pd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Xspipdcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Xspipdcr {{ xspipg10_pd: {=bool:?}, xspipd3_pd: {=bool:?}, xspipd2_pd: {=bool:?}, xspipd1_pd: {=bool:?}, xspipd0_pd: {=bool:?}, xspipb2_pd: {=bool:?}, xspipb1_pd: {=bool:?}, xspipa5_pd: {=bool:?}, xspipa4_pd: {=bool:?}, xspipa3_pd: {=bool:?}, xspipa2_pd: {=bool:?}, xspipa0_pd: {=bool:?}, xspipf2_pd: {=bool:?}, xspiph1_pd: {=bool:?}, xspiph0_pd: {=bool:?} }}",
                self.xspipg10_pd(),
                self.xspipd3_pd(),
                self.xspipd2_pd(),
                self.xspipd1_pd(),
                self.xspipd0_pd(),
                self.xspipb2_pd(),
                self.xspipb1_pd(),
                self.xspipa5_pd(),
                self.xspipa4_pd(),
                self.xspipa3_pd(),
                self.xspipa2_pd(),
                self.xspipa0_pd(),
                self.xspipf2_pd(),
                self.xspiph1_pd(),
                self.xspiph0_pd()
            )
        }
    }
    #[doc = "PWR XSPI pull-up control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Xspipucr(pub u32);
    impl Xspipucr {
        #[doc = "Port PG10 XSPI pull-up bit."]
        #[must_use]
        #[inline(always)]
        pub const fn xspipg10_pu(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Port PG10 XSPI pull-up bit."]
        #[inline(always)]
        pub const fn set_xspipg10_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Port PD3 XSPI pull-up bit."]
        #[must_use]
        #[inline(always)]
        pub const fn xspipd3_pu(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Port PD3 XSPI pull-up bit."]
        #[inline(always)]
        pub const fn set_xspipd3_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Port PD2 XSPI pull-up bit."]
        #[must_use]
        #[inline(always)]
        pub const fn xspipd2_pu(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Port PD2 XSPI pull-up bit."]
        #[inline(always)]
        pub const fn set_xspipd2_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Port PD1 XSPI pull-up bit."]
        #[must_use]
        #[inline(always)]
        pub const fn xspipd1_pu(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Port PD1 XSPI pull-up bit."]
        #[inline(always)]
        pub const fn set_xspipd1_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Port PD0 XSPI pull-up bit."]
        #[must_use]
        #[inline(always)]
        pub const fn xspipd0_pu(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Port PD0 XSPI pull-up bit."]
        #[inline(always)]
        pub const fn set_xspipd0_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Port PB2 XSPI pull-up bit."]
        #[must_use]
        #[inline(always)]
        pub const fn xspipb2_pu(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Port PB2 XSPI pull-up bit."]
        #[inline(always)]
        pub const fn set_xspipb2_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Port PB1 XSPI pull-up bit."]
        #[must_use]
        #[inline(always)]
        pub const fn xspipb1_pu(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Port PB1 XSPI pull-up bit."]
        #[inline(always)]
        pub const fn set_xspipb1_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Port PA5 XSPI pull-up bit."]
        #[must_use]
        #[inline(always)]
        pub const fn xspipa5_pu(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Port PA5 XSPI pull-up bit."]
        #[inline(always)]
        pub const fn set_xspipa5_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Port PA4 XSPI pull-up bit."]
        #[must_use]
        #[inline(always)]
        pub const fn xspipa4_pu(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Port PA4 XSPI pull-up bit."]
        #[inline(always)]
        pub const fn set_xspipa4_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Port PA3 XSPI pull-up bit."]
        #[must_use]
        #[inline(always)]
        pub const fn xspipa3_pu(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Port PA3 XSPI pull-up bit."]
        #[inline(always)]
        pub const fn set_xspipa3_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Port PA2 XSPI pull-up bit."]
        #[must_use]
        #[inline(always)]
        pub const fn xspipa2_pu(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Port PA2 XSPI pull-up bit."]
        #[inline(always)]
        pub const fn set_xspipa2_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Port PA0 XSPI pull-up bit."]
        #[must_use]
        #[inline(always)]
        pub const fn xspipa0_pu(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Port PA0 XSPI pull-up bit."]
        #[inline(always)]
        pub const fn set_xspipa0_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Port PF2 XSPI pull-up bit."]
        #[must_use]
        #[inline(always)]
        pub const fn xspipf2_pu(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Port PF2 XSPI pull-up bit."]
        #[inline(always)]
        pub const fn set_xspipf2_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Port PH1 XSPI pull-up bit."]
        #[must_use]
        #[inline(always)]
        pub const fn xspiph1_pu(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Port PH1 XSPI pull-up bit."]
        #[inline(always)]
        pub const fn set_xspiph1_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Port PH0 XSPI pull-up bit."]
        #[must_use]
        #[inline(always)]
        pub const fn xspiph0_pu(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Port PH0 XSPI pull-up bit."]
        #[inline(always)]
        pub const fn set_xspiph0_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Xspipucr {
        #[inline(always)]
        fn default() -> Xspipucr {
            Xspipucr(0)
        }
    }
    impl core::fmt::Debug for Xspipucr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Xspipucr")
                .field("xspipg10_pu", &self.xspipg10_pu())
                .field("xspipd3_pu", &self.xspipd3_pu())
                .field("xspipd2_pu", &self.xspipd2_pu())
                .field("xspipd1_pu", &self.xspipd1_pu())
                .field("xspipd0_pu", &self.xspipd0_pu())
                .field("xspipb2_pu", &self.xspipb2_pu())
                .field("xspipb1_pu", &self.xspipb1_pu())
                .field("xspipa5_pu", &self.xspipa5_pu())
                .field("xspipa4_pu", &self.xspipa4_pu())
                .field("xspipa3_pu", &self.xspipa3_pu())
                .field("xspipa2_pu", &self.xspipa2_pu())
                .field("xspipa0_pu", &self.xspipa0_pu())
                .field("xspipf2_pu", &self.xspipf2_pu())
                .field("xspiph1_pu", &self.xspiph1_pu())
                .field("xspiph0_pu", &self.xspiph0_pu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Xspipucr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Xspipucr {{ xspipg10_pu: {=bool:?}, xspipd3_pu: {=bool:?}, xspipd2_pu: {=bool:?}, xspipd1_pu: {=bool:?}, xspipd0_pu: {=bool:?}, xspipb2_pu: {=bool:?}, xspipb1_pu: {=bool:?}, xspipa5_pu: {=bool:?}, xspipa4_pu: {=bool:?}, xspipa3_pu: {=bool:?}, xspipa2_pu: {=bool:?}, xspipa0_pu: {=bool:?}, xspipf2_pu: {=bool:?}, xspiph1_pu: {=bool:?}, xspiph0_pu: {=bool:?} }}",
                self.xspipg10_pu(),
                self.xspipd3_pu(),
                self.xspipd2_pu(),
                self.xspipd1_pu(),
                self.xspipd0_pu(),
                self.xspipb2_pu(),
                self.xspipb1_pu(),
                self.xspipa5_pu(),
                self.xspipa4_pu(),
                self.xspipa3_pu(),
                self.xspipa2_pu(),
                self.xspipa0_pu(),
                self.xspipf2_pu(),
                self.xspiph1_pu(),
                self.xspiph0_pu()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Actvos {
        _RESERVED_0 = 0x0,
        #[doc = "VOS2."]
        Vos2 = 0x01,
        #[doc = "VOS1."]
        Vos1 = 0x02,
        #[doc = "VOS0."]
        Vos0 = 0x03,
    }
    impl Actvos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Actvos {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Actvos {
        #[inline(always)]
        fn from(val: u8) -> Actvos {
            Actvos::from_bits(val)
        }
    }
    impl From<Actvos> for u8 {
        #[inline(always)]
        fn from(val: Actvos) -> u8 {
            Actvos::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Als {
        #[doc = "AVD level 1."]
        Level1 = 0x0,
        #[doc = "AVD level 2."]
        Level2 = 0x01,
        #[doc = "AVD level 3."]
        Level3 = 0x02,
        #[doc = "AVD level 4."]
        Level4 = 0x03,
    }
    impl Als {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Als {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Als {
        #[inline(always)]
        fn from(val: u8) -> Als {
            Als::from_bits(val)
        }
    }
    impl From<Als> for u8 {
        #[inline(always)]
        fn from(val: Als) -> u8 {
            Als::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pls {
        #[doc = "VDD less than PVD level 1."]
        Level1 = 0x0,
        #[doc = "VDD less than PVD level 2."]
        Level2 = 0x01,
        #[doc = "VDD less than PVD level 3."]
        Level3 = 0x02,
        #[doc = "VDD less than PVD level 4."]
        Level4 = 0x03,
        #[doc = "VDD less than PVD level 5."]
        Level5 = 0x04,
        #[doc = "VDD less than PVD level 6."]
        Level6 = 0x05,
        #[doc = "VDD less than PVD level 7."]
        Level7 = 0x06,
        #[doc = "PVD_IN less than VREFINT."]
        PvdVrefint = 0x07,
    }
    impl Pls {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pls {
            unsafe { core::mem::transmute(val & 0x07) }
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
    pub enum Vos {
        _RESERVED_0 = 0x0,
        #[doc = "VOS2 (lowest power)."]
        Vos2 = 0x01,
        #[doc = "VOS1 (default)."]
        Vos1 = 0x02,
        #[doc = "VOS0 (highest frequency)."]
        Vos0 = 0x03,
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
    pub enum Wkuppupd1 {
        #[doc = "No pull."]
        None = 0x0,
        #[doc = "Pull-up."]
        Pullup = 0x01,
        #[doc = "Pull-down."]
        Pulldown = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Wkuppupd1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wkuppupd1 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wkuppupd1 {
        #[inline(always)]
        fn from(val: u8) -> Wkuppupd1 {
            Wkuppupd1::from_bits(val)
        }
    }
    impl From<Wkuppupd1> for u8 {
        #[inline(always)]
        fn from(val: Wkuppupd1) -> u8 {
            Wkuppupd1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wkuppupd2 {
        #[doc = "No pull."]
        None = 0x0,
        #[doc = "Pull-up."]
        Pullup = 0x01,
        #[doc = "Pull-down."]
        Pulldown = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Wkuppupd2 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wkuppupd2 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wkuppupd2 {
        #[inline(always)]
        fn from(val: u8) -> Wkuppupd2 {
            Wkuppupd2::from_bits(val)
        }
    }
    impl From<Wkuppupd2> for u8 {
        #[inline(always)]
        fn from(val: Wkuppupd2) -> u8 {
            Wkuppupd2::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wkuppupd3 {
        #[doc = "No pull."]
        None = 0x0,
        #[doc = "Pull-up."]
        Pullup = 0x01,
        #[doc = "Pull-down."]
        Pulldown = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Wkuppupd3 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wkuppupd3 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wkuppupd3 {
        #[inline(always)]
        fn from(val: u8) -> Wkuppupd3 {
            Wkuppupd3::from_bits(val)
        }
    }
    impl From<Wkuppupd3> for u8 {
        #[inline(always)]
        fn from(val: Wkuppupd3) -> u8 {
            Wkuppupd3::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wkuppupd4 {
        #[doc = "No pull."]
        None = 0x0,
        #[doc = "Pull-up."]
        Pullup = 0x01,
        #[doc = "Pull-down."]
        Pulldown = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Wkuppupd4 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wkuppupd4 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wkuppupd4 {
        #[inline(always)]
        fn from(val: u8) -> Wkuppupd4 {
            Wkuppupd4::from_bits(val)
        }
    }
    impl From<Wkuppupd4> for u8 {
        #[inline(always)]
        fn from(val: Wkuppupd4) -> u8 {
            Wkuppupd4::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wkuppupd5 {
        #[doc = "No pull."]
        None = 0x0,
        #[doc = "Pull-up."]
        Pullup = 0x01,
        #[doc = "Pull-down."]
        Pulldown = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Wkuppupd5 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wkuppupd5 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wkuppupd5 {
        #[inline(always)]
        fn from(val: u8) -> Wkuppupd5 {
            Wkuppupd5::from_bits(val)
        }
    }
    impl From<Wkuppupd5> for u8 {
        #[inline(always)]
        fn from(val: Wkuppupd5) -> u8 {
            Wkuppupd5::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wkuppupd6 {
        #[doc = "No pull."]
        None = 0x0,
        #[doc = "Pull-up."]
        Pullup = 0x01,
        #[doc = "Pull-down."]
        Pulldown = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Wkuppupd6 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wkuppupd6 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wkuppupd6 {
        #[inline(always)]
        fn from(val: u8) -> Wkuppupd6 {
            Wkuppupd6::from_bits(val)
        }
    }
    impl From<Wkuppupd6> for u8 {
        #[inline(always)]
        fn from(val: Wkuppupd6) -> u8 {
            Wkuppupd6::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wkuppupd7 {
        #[doc = "No pull."]
        None = 0x0,
        #[doc = "Pull-up."]
        Pullup = 0x01,
        #[doc = "Pull-down."]
        Pulldown = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Wkuppupd7 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wkuppupd7 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wkuppupd7 {
        #[inline(always)]
        fn from(val: u8) -> Wkuppupd7 {
            Wkuppupd7::from_bits(val)
        }
    }
    impl From<Wkuppupd7> for u8 {
        #[inline(always)]
        fn from(val: Wkuppupd7) -> u8 {
            Wkuppupd7::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wkuppupd8 {
        #[doc = "No pull."]
        None = 0x0,
        #[doc = "Pull-up."]
        Pullup = 0x01,
        #[doc = "Pull-down."]
        Pulldown = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Wkuppupd8 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wkuppupd8 {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wkuppupd8 {
        #[inline(always)]
        fn from(val: u8) -> Wkuppupd8 {
            Wkuppupd8::from_bits(val)
        }
    }
    impl From<Wkuppupd8> for u8 {
        #[inline(always)]
        fn from(val: Wkuppupd8) -> u8 {
            Wkuppupd8::to_bits(val)
        }
    }
}
