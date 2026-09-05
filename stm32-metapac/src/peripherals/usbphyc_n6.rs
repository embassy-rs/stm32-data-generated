#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "USB high-speed PHY controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbphyc {
    ptr: *mut u8,
}
unsafe impl Send for Usbphyc {}
unsafe impl Sync for Usbphyc {}
impl Usbphyc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "USB PHY control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "USB PHY trimming control register 1"]
    #[inline(always)]
    pub const fn trim1cr(self) -> crate::common::Reg<regs::Trim1cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "USB PHY trimming control register 2"]
    #[inline(always)]
    pub const fn trim2cr(self) -> crate::common::Reg<regs::Trim2cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
}
pub mod regs {
    #[doc = "USB PHY control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Retention mode enable (active low)"]
        #[must_use]
        #[inline(always)]
        pub const fn retenablen1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Retention mode enable (active low)"]
        #[inline(always)]
        pub const fn set_retenablen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Automatic resume mode enable"]
        #[must_use]
        #[inline(always)]
        pub const fn autorsmenb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic resume mode enable"]
        #[inline(always)]
        pub const fn set_autorsmenb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Power-down control for analog blocks during suspend and sleep modes"]
        #[must_use]
        #[inline(always)]
        pub const fn cmn(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Power-down control for analog blocks during suspend and sleep modes"]
        #[inline(always)]
        pub const fn set_cmn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "PHY reference clock frequency selection"]
        #[must_use]
        #[inline(always)]
        pub const fn fsel(&self) -> super::vals::Fsel {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Fsel::from_bits(val as u8)
        }
        #[doc = "PHY reference clock frequency selection"]
        #[inline(always)]
        pub const fn set_fsel(&mut self, val: super::vals::Fsel) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "OTG disable control"]
        #[must_use]
        #[inline(always)]
        pub const fn otgdisable0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "OTG disable control"]
        #[inline(always)]
        pub const fn set_otgdisable0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "VBUS drive control"]
        #[must_use]
        #[inline(always)]
        pub const fn drvvbus0(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "VBUS drive control"]
        #[inline(always)]
        pub const fn set_drvvbus0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "High-speed PHY debug port selection. Only available in USBPHYC2."]
        #[must_use]
        #[inline(always)]
        pub const fn selotgdbg(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "High-speed PHY debug port selection. Only available in USBPHYC2."]
        #[inline(always)]
        pub const fn set_selotgdbg(&mut self, val: bool) {
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
                .field("retenablen1", &self.retenablen1())
                .field("autorsmenb1", &self.autorsmenb1())
                .field("cmn", &self.cmn())
                .field("fsel", &self.fsel())
                .field("otgdisable0", &self.otgdisable0())
                .field("drvvbus0", &self.drvvbus0())
                .field("selotgdbg", &self.selotgdbg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ retenablen1: {=bool:?}, autorsmenb1: {=bool:?}, cmn: {=bool:?}, fsel: {:?}, otgdisable0: {=bool:?}, drvvbus0: {=bool:?}, selotgdbg: {=bool:?} }}",
                self.retenablen1(),
                self.autorsmenb1(),
                self.cmn(),
                self.fsel(),
                self.otgdisable0(),
                self.drvvbus0(),
                self.selotgdbg()
            )
        }
    }
    #[doc = "USB PHY trimming control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Trim1cr(pub u32);
    impl Trim1cr {
        #[doc = "PHY PLL integral tuning"]
        #[must_use]
        #[inline(always)]
        pub const fn pllitune(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "PHY PLL integral tuning"]
        #[inline(always)]
        pub const fn set_pllitune(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "PHY PLL proportional tuning"]
        #[must_use]
        #[inline(always)]
        pub const fn pllptune(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x0f;
            val as u8
        }
        #[doc = "PHY PLL proportional tuning"]
        #[inline(always)]
        pub const fn set_pllptune(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
        }
        #[doc = "Disconnect threshold adjustment"]
        #[must_use]
        #[inline(always)]
        pub const fn compdistune(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x07;
            val as u8
        }
        #[doc = "Disconnect threshold adjustment"]
        #[inline(always)]
        pub const fn set_compdistune(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
        }
        #[doc = "Squelch threshold adjustment"]
        #[must_use]
        #[inline(always)]
        pub const fn sqrxtune(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x07;
            val as u8
        }
        #[doc = "Squelch threshold adjustment"]
        #[inline(always)]
        pub const fn set_sqrxtune(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
        }
        #[doc = "Data voltage reference tuning"]
        #[must_use]
        #[inline(always)]
        pub const fn vdatreftune(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Data voltage reference tuning"]
        #[inline(always)]
        pub const fn set_vdatreftune(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "VBUS valid threshold adjustment"]
        #[must_use]
        #[inline(always)]
        pub const fn otgtune(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x07;
            val as u8
        }
        #[doc = "VBUS valid threshold adjustment"]
        #[inline(always)]
        pub const fn set_otgtune(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 14usize)) | (((val as u32) & 0x07) << 14usize);
        }
        #[doc = "High-speed transmitter crossover adjustment"]
        #[must_use]
        #[inline(always)]
        pub const fn txhsxvtune(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x03;
            val as u8
        }
        #[doc = "High-speed transmitter crossover adjustment"]
        #[inline(always)]
        pub const fn set_txhsxvtune(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
        }
        #[doc = "Full-speed and low-speed transmitter tuning"]
        #[must_use]
        #[inline(always)]
        pub const fn txfslstune(&self) -> u8 {
            let val = (self.0 >> 19usize) & 0x0f;
            val as u8
        }
        #[doc = "Full-speed and low-speed transmitter tuning"]
        #[inline(always)]
        pub const fn set_txfslstune(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 19usize)) | (((val as u32) & 0x0f) << 19usize);
        }
        #[doc = "Transmitter voltage reference tuning"]
        #[must_use]
        #[inline(always)]
        pub const fn txvreftune(&self) -> u8 {
            let val = (self.0 >> 23usize) & 0x0f;
            val as u8
        }
        #[doc = "Transmitter voltage reference tuning"]
        #[inline(always)]
        pub const fn set_txvreftune(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 23usize)) | (((val as u32) & 0x0f) << 23usize);
        }
        #[doc = "Transmitter rise-time tuning"]
        #[must_use]
        #[inline(always)]
        pub const fn txrisetune(&self) -> u8 {
            let val = (self.0 >> 27usize) & 0x03;
            val as u8
        }
        #[doc = "Transmitter rise-time tuning"]
        #[inline(always)]
        pub const fn set_txrisetune(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 27usize)) | (((val as u32) & 0x03) << 27usize);
        }
        #[doc = "Transmitter resistance tuning"]
        #[must_use]
        #[inline(always)]
        pub const fn txrestune(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "Transmitter resistance tuning"]
        #[inline(always)]
        pub const fn set_txrestune(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Trim1cr {
        #[inline(always)]
        fn default() -> Trim1cr {
            Trim1cr(0)
        }
    }
    impl core::fmt::Debug for Trim1cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Trim1cr")
                .field("pllitune", &self.pllitune())
                .field("pllptune", &self.pllptune())
                .field("compdistune", &self.compdistune())
                .field("sqrxtune", &self.sqrxtune())
                .field("vdatreftune", &self.vdatreftune())
                .field("otgtune", &self.otgtune())
                .field("txhsxvtune", &self.txhsxvtune())
                .field("txfslstune", &self.txfslstune())
                .field("txvreftune", &self.txvreftune())
                .field("txrisetune", &self.txrisetune())
                .field("txrestune", &self.txrestune())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Trim1cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Trim1cr {{ pllitune: {=u8:?}, pllptune: {=u8:?}, compdistune: {=u8:?}, sqrxtune: {=u8:?}, vdatreftune: {=u8:?}, otgtune: {=u8:?}, txhsxvtune: {=u8:?}, txfslstune: {=u8:?}, txvreftune: {=u8:?}, txrisetune: {=u8:?}, txrestune: {=u8:?} }}",
                self.pllitune(),
                self.pllptune(),
                self.compdistune(),
                self.sqrxtune(),
                self.vdatreftune(),
                self.otgtune(),
                self.txhsxvtune(),
                self.txfslstune(),
                self.txvreftune(),
                self.txrisetune(),
                self.txrestune()
            )
        }
    }
    #[doc = "USB PHY trimming control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Trim2cr(pub u32);
    impl Trim2cr {
        #[doc = "High-speed transmitter pre-emphasis current control"]
        #[must_use]
        #[inline(always)]
        pub const fn txpreempamptune(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "High-speed transmitter pre-emphasis current control"]
        #[inline(always)]
        pub const fn set_txpreempamptune(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "High-speed transmitter pre-emphasis pulse control"]
        #[must_use]
        #[inline(always)]
        pub const fn txpreemppulsetune(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "High-speed transmitter pre-emphasis pulse control"]
        #[inline(always)]
        pub const fn set_txpreemppulsetune(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Trim2cr {
        #[inline(always)]
        fn default() -> Trim2cr {
            Trim2cr(0)
        }
    }
    impl core::fmt::Debug for Trim2cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Trim2cr")
                .field("txpreempamptune", &self.txpreempamptune())
                .field("txpreemppulsetune", &self.txpreemppulsetune())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Trim2cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Trim2cr {{ txpreempamptune: {=u8:?}, txpreemppulsetune: {=bool:?} }}",
                self.txpreempamptune(),
                self.txpreemppulsetune()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fsel {
        #[doc = "19.2 MHz PHY reference clock"]
        Mhz192 = 0x0,
        #[doc = "20 MHz PHY reference clock (default after reset)"]
        Mhz20 = 0x01,
        #[doc = "24 MHz PHY reference clock"]
        Mhz24 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Fsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fsel {
        #[inline(always)]
        fn from(val: u8) -> Fsel {
            Fsel::from_bits(val)
        }
    }
    impl From<Fsel> for u8 {
        #[inline(always)]
        fn from(val: Fsel) -> u8 {
            Fsel::to_bits(val)
        }
    }
}
