#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Power control"]
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
    #[doc = "PWR power mode control register"]
    #[inline(always)]
    pub const fn pmcr(self) -> crate::common::Reg<regs::Pmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "PWR status register"]
    #[inline(always)]
    pub const fn pmsr(self) -> crate::common::Reg<regs::Pmsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "PWR voltage scaling control register"]
    #[inline(always)]
    pub const fn voscr(self) -> crate::common::Reg<regs::Voscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "PWR voltage scaling status register"]
    #[inline(always)]
    pub const fn vossr(self) -> crate::common::Reg<regs::Vossr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "PWR Backup domain control register"]
    #[inline(always)]
    pub const fn bdcr(self) -> crate::common::Reg<regs::Bdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "PWR disable backup protection control register"]
    #[inline(always)]
    pub const fn dbpcr(self) -> crate::common::Reg<regs::Dbpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "PWR Backup domain status register"]
    #[inline(always)]
    pub const fn bdsr(self) -> crate::common::Reg<regs::Bdsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "PWR supply configuration control register"]
    #[inline(always)]
    pub const fn sccr(self) -> crate::common::Reg<regs::Sccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "PWR voltage monitor control register"]
    #[inline(always)]
    pub const fn vmcr(self) -> crate::common::Reg<regs::Vmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "PWR voltage monitor status register"]
    #[inline(always)]
    pub const fn vmsr(self) -> crate::common::Reg<regs::Vmsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "PWR wakeup status clear register"]
    #[inline(always)]
    pub const fn wuscr(self) -> crate::common::Reg<regs::Wuscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "PWR wakeup status register"]
    #[inline(always)]
    pub const fn wusr(self) -> crate::common::Reg<regs::Wusr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "PWR wakeup configuration register"]
    #[inline(always)]
    pub const fn wucr(self) -> crate::common::Reg<regs::Wucr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "PWR I/O retention register"]
    #[inline(always)]
    pub const fn ioretr(self) -> crate::common::Reg<regs::Ioretr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "PWR privilege configuration register"]
    #[inline(always)]
    pub const fn privcfgr(self) -> crate::common::Reg<regs::Privcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
}
pub mod regs {
    #[doc = "PWR Backup domain control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdcr(pub u32);
    impl Bdcr {
        #[doc = "Backup RAM retention in Standby and V<sub>BAT</sub> modes When this bit set, the backup regulator (used to maintain the backup RAM content in Standby and V<sub>BAT</sub> modes) is enabled. If BREN is cleared, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However its content is lost in Standby and V<sub>BAT</sub> modes. If BREN is set, the application must wait till the backup regulator ready flag (BRRDY) is set to indicate that the data written into the SRAM is maintained in Standby and V<sub>BAT</sub> modes."]
        #[inline(always)]
        pub const fn bren(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Backup RAM retention in Standby and V<sub>BAT</sub> modes When this bit set, the backup regulator (used to maintain the backup RAM content in Standby and V<sub>BAT</sub> modes) is enabled. If BREN is cleared, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However its content is lost in Standby and V<sub>BAT</sub> modes. If BREN is set, the application must wait till the backup regulator ready flag (BRRDY) is set to indicate that the data written into the SRAM is maintained in Standby and V<sub>BAT</sub> modes."]
        #[inline(always)]
        pub fn set_bren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Backup domain voltage and temperature monitoring enable"]
        #[inline(always)]
        pub const fn monen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Backup domain voltage and temperature monitoring enable"]
        #[inline(always)]
        pub fn set_monen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "V<sub>BAT</sub> charging enable Note: Reset only by POR,."]
        #[inline(always)]
        pub const fn vbe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "V<sub>BAT</sub> charging enable Note: Reset only by POR,."]
        #[inline(always)]
        pub fn set_vbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "V<sub>BAT</sub> charging resistor selection"]
        #[inline(always)]
        pub const fn vbrs(&self) -> super::vals::Vbrs {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Vbrs::from_bits(val as u8)
        }
        #[doc = "V<sub>BAT</sub> charging resistor selection"]
        #[inline(always)]
        pub fn set_vbrs(&mut self, val: super::vals::Vbrs) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Bdcr {
        #[inline(always)]
        fn default() -> Bdcr {
            Bdcr(0)
        }
    }
    #[doc = "PWR Backup domain status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdsr(pub u32);
    impl Bdsr {
        #[doc = "backup regulator ready This bit is set by hardware to indicate that the backup regulator is ready."]
        #[inline(always)]
        pub const fn brrdy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "backup regulator ready This bit is set by hardware to indicate that the backup regulator is ready."]
        #[inline(always)]
        pub fn set_brrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "V<sub>BAT</sub> level monitoring versus low threshold"]
        #[inline(always)]
        pub const fn vbatl(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "V<sub>BAT</sub> level monitoring versus low threshold"]
        #[inline(always)]
        pub fn set_vbatl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "V<sub>BAT</sub> level monitoring versus high threshold"]
        #[inline(always)]
        pub const fn vbath(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "V<sub>BAT</sub> level monitoring versus high threshold"]
        #[inline(always)]
        pub fn set_vbath(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "temperature level monitoring versus low threshold"]
        #[inline(always)]
        pub const fn templ(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "temperature level monitoring versus low threshold"]
        #[inline(always)]
        pub fn set_templ(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "temperature level monitoring versus high threshold"]
        #[inline(always)]
        pub const fn temph(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "temperature level monitoring versus high threshold"]
        #[inline(always)]
        pub fn set_temph(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Bdsr {
        #[inline(always)]
        fn default() -> Bdsr {
            Bdsr(0)
        }
    }
    #[doc = "PWR disable backup protection control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dbpcr(pub u32);
    impl Dbpcr {
        #[doc = "Disable Backup domain write protection In reset state, all registers and SRAM in Backup domain are protected against parasitic write access. This bit must be set to enable write access to these registers."]
        #[inline(always)]
        pub const fn dbp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Disable Backup domain write protection In reset state, all registers and SRAM in Backup domain are protected against parasitic write access. This bit must be set to enable write access to these registers."]
        #[inline(always)]
        pub fn set_dbp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Dbpcr {
        #[inline(always)]
        fn default() -> Dbpcr {
            Dbpcr(0)
        }
    }
    #[doc = "PWR I/O retention register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ioretr(pub u32);
    impl Ioretr {
        #[doc = "IO retention enable: When entering into standby mode, the output is sampled, and applied to the output IO during the standby power mode. Note: the IO state is not retained if the DBG_STANDBY bit is set in DBGMCU_CR register."]
        #[inline(always)]
        pub const fn ioreten(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IO retention enable: When entering into standby mode, the output is sampled, and applied to the output IO during the standby power mode. Note: the IO state is not retained if the DBG_STANDBY bit is set in DBGMCU_CR register."]
        #[inline(always)]
        pub fn set_ioreten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IO retention enable for JTAG IOs when entering into standby mode, the output is sampled, and applied to the output IO during the standby power mode"]
        #[inline(always)]
        pub const fn jtagioreten(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "IO retention enable for JTAG IOs when entering into standby mode, the output is sampled, and applied to the output IO during the standby power mode"]
        #[inline(always)]
        pub fn set_jtagioreten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Ioretr {
        #[inline(always)]
        fn default() -> Ioretr {
            Ioretr(0)
        }
    }
    #[doc = "PWR power mode control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pmcr(pub u32);
    impl Pmcr {
        #[doc = "low-power mode selection This bit defines the Deepsleep mode."]
        #[inline(always)]
        pub const fn lpms(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "low-power mode selection This bit defines the Deepsleep mode."]
        #[inline(always)]
        pub fn set_lpms(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "system Stop mode voltage scaling selection These bits control the V<sub>CORE</sub> voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance."]
        #[inline(always)]
        pub const fn svos(&self) -> super::vals::Svos {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Svos::from_bits(val as u8)
        }
        #[doc = "system Stop mode voltage scaling selection These bits control the V<sub>CORE</sub> voltage level in system Stop mode, to obtain the best trade-off between power consumption and performance."]
        #[inline(always)]
        pub fn set_svos(&mut self, val: super::vals::Svos) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "clear Standby and Stop flags (always read as 0) This bit is cleared to 0 by hardware."]
        #[inline(always)]
        pub const fn cssf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "clear Standby and Stop flags (always read as 0) This bit is cleared to 0 by hardware."]
        #[inline(always)]
        pub fn set_cssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Flash memory low-power mode in Stop mode This bit is used to obtain the best trade-off between low-power consumption and restart time when exiting from Stop mode. When it is set, the Flash memory enters low-power mode when the CPU domain is in Stop mode. Note: When system enters stop mode with SVOS5 enabled, Flash memory is automatically forced in low-power mode."]
        #[inline(always)]
        pub const fn flps(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Flash memory low-power mode in Stop mode This bit is used to obtain the best trade-off between low-power consumption and restart time when exiting from Stop mode. When it is set, the Flash memory enters low-power mode when the CPU domain is in Stop mode. Note: When system enters stop mode with SVOS5 enabled, Flash memory is automatically forced in low-power mode."]
        #[inline(always)]
        pub fn set_flps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "analog switch V<sub>BOOST</sub> control This bit enables the booster to guarantee the analog switch AC performance when the V<sub>DD</sub> supply voltage is below 2.7 V (reduction of the total harmonic distortion to have the same switch performance over the full supply voltage range) The V<sub>DD</sub> supply voltage can be monitored through the PVD and the PLS bits."]
        #[inline(always)]
        pub const fn booste(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "analog switch V<sub>BOOST</sub> control This bit enables the booster to guarantee the analog switch AC performance when the V<sub>DD</sub> supply voltage is below 2.7 V (reduction of the total harmonic distortion to have the same switch performance over the full supply voltage range) The V<sub>DD</sub> supply voltage can be monitored through the PVD and the PLS bits."]
        #[inline(always)]
        pub fn set_booste(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "analog voltage ready This bit is only used when the analog switch boost needs to be enabled (see BOOSTE bit). It must be set by software when the expected V<sub>DDA</sub> analog supply level is available. The correct analog supply level is indicated by the AVDO bit (PWR_VMSR register) after setting the AVDEN bit (PWR_VMCR register) and selecting the supply level to be monitored (ALS bits)."]
        #[inline(always)]
        pub const fn avd_ready(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "analog voltage ready This bit is only used when the analog switch boost needs to be enabled (see BOOSTE bit). It must be set by software when the expected V<sub>DDA</sub> analog supply level is available. The correct analog supply level is indicated by the AVDO bit (PWR_VMSR register) after setting the AVDEN bit (PWR_VMCR register) and selecting the supply level to be monitored (ALS bits)."]
        #[inline(always)]
        pub fn set_avd_ready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "AHB SRAM2 shut-off in Stop mode."]
        #[inline(always)]
        pub const fn sram2so(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "AHB SRAM2 shut-off in Stop mode."]
        #[inline(always)]
        pub fn set_sram2so(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "AHB SRAM1 shut-off in Stop mode"]
        #[inline(always)]
        pub const fn sram1so(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "AHB SRAM1 shut-off in Stop mode"]
        #[inline(always)]
        pub fn set_sram1so(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Pmcr {
        #[inline(always)]
        fn default() -> Pmcr {
            Pmcr(0)
        }
    }
    #[doc = "PWR status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pmsr(pub u32);
    impl Pmsr {
        #[doc = "Stop flag This bit is set by hardware and cleared only by any reset or by setting the CSSF bit."]
        #[inline(always)]
        pub const fn stopf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Stop flag This bit is set by hardware and cleared only by any reset or by setting the CSSF bit."]
        #[inline(always)]
        pub fn set_stopf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "System standby flag This bit is set by hardware and cleared only by a POR or by setting the CSSF bit."]
        #[inline(always)]
        pub const fn sbf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "System standby flag This bit is set by hardware and cleared only by a POR or by setting the CSSF bit."]
        #[inline(always)]
        pub fn set_sbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Pmsr {
        #[inline(always)]
        fn default() -> Pmsr {
            Pmsr(0)
        }
    }
    #[doc = "PWR privilege configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcfgr(pub u32);
    impl Privcfgr {
        #[doc = "PWR functions privilege configuration Set and reset by software. This bit can be written only by privileged access."]
        #[inline(always)]
        pub const fn nspriv(&self) -> super::vals::Priv {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Priv::from_bits(val as u8)
        }
        #[doc = "PWR functions privilege configuration Set and reset by software. This bit can be written only by privileged access."]
        #[inline(always)]
        pub fn set_nspriv(&mut self, val: super::vals::Priv) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Privcfgr {
        #[inline(always)]
        fn default() -> Privcfgr {
            Privcfgr(0)
        }
    }
    #[doc = "PWR supply configuration control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sccr(pub u32);
    impl Sccr {
        #[doc = "power management unit bypass"]
        #[inline(always)]
        pub const fn bypass(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "power management unit bypass"]
        #[inline(always)]
        pub fn set_bypass(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LDO enable The value is set by hardware when the package uses the LDO regulator."]
        #[inline(always)]
        pub const fn ldoen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "LDO enable The value is set by hardware when the package uses the LDO regulator."]
        #[inline(always)]
        pub fn set_ldoen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Sccr {
        #[inline(always)]
        fn default() -> Sccr {
            Sccr(0)
        }
    }
    #[doc = "PWR voltage monitor control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vmcr(pub u32);
    impl Vmcr {
        #[doc = "PVD enable"]
        #[inline(always)]
        pub const fn pvde(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PVD enable"]
        #[inline(always)]
        pub fn set_pvde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "programmable voltage detector (PVD) level selection These bits select the voltage threshold detected by the PVD."]
        #[inline(always)]
        pub const fn pls(&self) -> super::vals::Pls {
            let val = (self.0 >> 1usize) & 0x07;
            super::vals::Pls::from_bits(val as u8)
        }
        #[doc = "programmable voltage detector (PVD) level selection These bits select the voltage threshold detected by the PVD."]
        #[inline(always)]
        pub fn set_pls(&mut self, val: super::vals::Pls) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
        }
        #[doc = "peripheral voltage monitor on V<sub>DDA</sub> enable"]
        #[inline(always)]
        pub const fn avden(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "peripheral voltage monitor on V<sub>DDA</sub> enable"]
        #[inline(always)]
        pub fn set_avden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "analog voltage detector (AVD) level selection These bits select the voltage threshold detected by the AVD."]
        #[inline(always)]
        pub const fn als(&self) -> super::vals::Als {
            let val = (self.0 >> 9usize) & 0x03;
            super::vals::Als::from_bits(val as u8)
        }
        #[doc = "analog voltage detector (AVD) level selection These bits select the voltage threshold detected by the AVD."]
        #[inline(always)]
        pub fn set_als(&mut self, val: super::vals::Als) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
        }
    }
    impl Default for Vmcr {
        #[inline(always)]
        fn default() -> Vmcr {
            Vmcr(0)
        }
    }
    #[doc = "PWR voltage monitor status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vmsr(pub u32);
    impl Vmsr {
        #[doc = "analog voltage detector output on V<sub>DDA</sub> This bit is set and cleared by hardware. It is valid only if AVD on VDDA is enabled by the AVDEN bit. Note: Since the AVD is disabled in Standby mode, this bit is equal to 0 after standby or reset until the AVDEN bit is set."]
        #[inline(always)]
        pub const fn avdo(&self) -> super::vals::Avdo {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Avdo::from_bits(val as u8)
        }
        #[doc = "analog voltage detector output on V<sub>DDA</sub> This bit is set and cleared by hardware. It is valid only if AVD on VDDA is enabled by the AVDEN bit. Note: Since the AVD is disabled in Standby mode, this bit is equal to 0 after standby or reset until the AVDEN bit is set."]
        #[inline(always)]
        pub fn set_avdo(&mut self, val: super::vals::Avdo) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "voltage detector output on V<sub>DDIO2</sub> This bit is set and cleared by hardware."]
        #[inline(always)]
        pub const fn vddio2rdy(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "voltage detector output on V<sub>DDIO2</sub> This bit is set and cleared by hardware."]
        #[inline(always)]
        pub fn set_vddio2rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "programmable voltage detect output This bit is set and cleared by hardware. It is valid only if the PVD has been enabled by the PVDE bit. Note: Since the PVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the PVDE bit is set."]
        #[inline(always)]
        pub const fn pvdo(&self) -> super::vals::Pvdo {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Pvdo::from_bits(val as u8)
        }
        #[doc = "programmable voltage detect output This bit is set and cleared by hardware. It is valid only if the PVD has been enabled by the PVDE bit. Note: Since the PVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the PVDE bit is set."]
        #[inline(always)]
        pub fn set_pvdo(&mut self, val: super::vals::Pvdo) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Vmsr {
        #[inline(always)]
        fn default() -> Vmsr {
            Vmsr(0)
        }
    }
    #[doc = "PWR voltage scaling control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Voscr(pub u32);
    impl Voscr {
        #[doc = "voltage scaling selection according to performance These bits control the V<sub>CORE</sub> voltage level and allow to obtain the best trade-off between power consumption and performance: - In bypass mode, these bits must also be set according to the external provided core voltage level and related performance. - When increasing the performance, the voltage scaling must be changed before increasing the system frequency. - When decreasing performance, the system frequency must first be decreased before changing the voltage scaling."]
        #[inline(always)]
        pub const fn vos(&self) -> super::vals::Vos {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Vos::from_bits(val as u8)
        }
        #[doc = "voltage scaling selection according to performance These bits control the V<sub>CORE</sub> voltage level and allow to obtain the best trade-off between power consumption and performance: - In bypass mode, these bits must also be set according to the external provided core voltage level and related performance. - When increasing the performance, the voltage scaling must be changed before increasing the system frequency. - When decreasing performance, the system frequency must first be decreased before changing the voltage scaling."]
        #[inline(always)]
        pub fn set_vos(&mut self, val: super::vals::Vos) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
    }
    impl Default for Voscr {
        #[inline(always)]
        fn default() -> Voscr {
            Voscr(0)
        }
    }
    #[doc = "PWR voltage scaling status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vossr(pub u32);
    impl Vossr {
        #[doc = "Ready bit for V<sub>CORE</sub> voltage scaling output selection."]
        #[inline(always)]
        pub const fn vosrdy(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Ready bit for V<sub>CORE</sub> voltage scaling output selection."]
        #[inline(always)]
        pub fn set_vosrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Voltage level ready for currently used VOS"]
        #[inline(always)]
        pub const fn actvosrdy(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Voltage level ready for currently used VOS"]
        #[inline(always)]
        pub fn set_actvosrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "voltage output scaling currently applied to V<sub>CORE</sub> This field provides the last VOS value."]
        #[inline(always)]
        pub const fn actvos(&self) -> super::vals::Actvos {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::Actvos::from_bits(val as u8)
        }
        #[doc = "voltage output scaling currently applied to V<sub>CORE</sub> This field provides the last VOS value."]
        #[inline(always)]
        pub fn set_actvos(&mut self, val: super::vals::Actvos) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
    }
    impl Default for Vossr {
        #[inline(always)]
        fn default() -> Vossr {
            Vossr(0)
        }
    }
    #[doc = "PWR wakeup configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wucr(pub u32);
    impl Wucr {
        #[doc = "enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
        #[inline(always)]
        pub const fn wupen(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "enable wakeup pin WUPx These bits are set and cleared by software. Note: an additional wakeup event is detected if WUPx pin is enabled (by setting the WUPENx bit) when WUPx pin level is already high when WUPPx selects rising edge, or low when WUPPx selects falling edge."]
        #[inline(always)]
        pub fn set_wupen(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
        #[inline(always)]
        pub const fn wupp(&self, n: usize) -> super::vals::Wupp {
            assert!(n < 5usize);
            let offs = 8usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Wupp::from_bits(val as u8)
        }
        #[doc = "wakeup pin polarity bit for WUPx These bits define the polarity used for event detection on WUPx external wakeup pin."]
        #[inline(always)]
        pub fn set_wupp(&mut self, n: usize, val: super::vals::Wupp) {
            assert!(n < 5usize);
            let offs = 8usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
        #[inline(always)]
        pub const fn wuppupd(&self, n: usize) -> super::vals::Wuppupd {
            assert!(n < 5usize);
            let offs = 16usize + n * 2usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::Wuppupd::from_bits(val as u8)
        }
        #[doc = "wakeup pin pull configuration for WKUPx These bits define the I/O pad pull configuration used when WUPENx = 1. The associated GPIO port pull configuration must be set to the same value or to 00. The wakeup pin pull configuration is kept in Standby mode."]
        #[inline(always)]
        pub fn set_wuppupd(&mut self, n: usize, val: super::vals::Wuppupd) {
            assert!(n < 5usize);
            let offs = 16usize + n * 2usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
    }
    impl Default for Wucr {
        #[inline(always)]
        fn default() -> Wucr {
            Wucr(0)
        }
    }
    #[doc = "PWR wakeup status clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wuscr(pub u32);
    impl Wuscr {
        #[doc = "clear wakeup pin flag for WUFx These bits are always read as 0."]
        #[inline(always)]
        pub const fn cwuf(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "clear wakeup pin flag for WUFx These bits are always read as 0."]
        #[inline(always)]
        pub fn set_cwuf(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Wuscr {
        #[inline(always)]
        fn default() -> Wuscr {
            Wuscr(0)
        }
    }
    #[doc = "PWR wakeup status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wusr(pub u32);
    impl Wusr {
        #[doc = "wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
        #[inline(always)]
        pub const fn wuf(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "wakeup pin WUFx flag This bit is set by hardware and cleared only by a RESET pin or by setting the CWUFx bit in PWR_WUSCR register."]
        #[inline(always)]
        pub fn set_wuf(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Wusr {
        #[inline(always)]
        fn default() -> Wusr {
            Wusr(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Actvos {
        #[doc = "VOS3 (lowest power)"]
        B_0X0 = 0x0,
        #[doc = "VOS2"]
        B_0X1 = 0x01,
        #[doc = "VOS1"]
        B_0X2 = 0x02,
        #[doc = "VOS0 (highest frequency)"]
        B_0X3 = 0x03,
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Als {
        #[doc = "1.7 V"]
        B_0X0 = 0x0,
        #[doc = "2.1 V"]
        B_0X1 = 0x01,
        #[doc = "2.5 V"]
        B_0X2 = 0x02,
        #[doc = "2.8 V"]
        B_0X3 = 0x03,
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Avdo {
        #[doc = "V<sub>DDA</sub> is equal or higher than the AVD threshold selected with the ALS\\[2:0\\]
bits."]
        B_0X0 = 0x0,
        #[doc = "V<sub>DDA</sub> is lower than the AVD threshold selected with the ALS\\[2:0\\]
bits."]
        B_0X1 = 0x01,
    }
    impl Avdo {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Avdo {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Avdo {
        #[inline(always)]
        fn from(val: u8) -> Avdo {
            Avdo::from_bits(val)
        }
    }
    impl From<Avdo> for u8 {
        #[inline(always)]
        fn from(val: Avdo) -> u8 {
            Avdo::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Pls {
        #[doc = "1.95 V"]
        B_0X0 = 0x0,
        #[doc = "2.1 V"]
        B_0X1 = 0x01,
        #[doc = "2.25 V"]
        B_0X2 = 0x02,
        #[doc = "2.4 V"]
        B_0X3 = 0x03,
        #[doc = "2.55 V"]
        B_0X4 = 0x04,
        #[doc = "2.7 V"]
        B_0X5 = 0x05,
        #[doc = "2.85 V"]
        B_0X6 = 0x06,
        #[doc = "PVD_IN pin"]
        B_0X7 = 0x07,
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Priv {
        #[doc = "Read and write to PWR functions can be done by privileged or unprivileged access."]
        B_0X0 = 0x0,
        #[doc = "Read and write to PWR functions can be done by privileged access only."]
        B_0X1 = 0x01,
    }
    impl Priv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Priv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Priv {
        #[inline(always)]
        fn from(val: u8) -> Priv {
            Priv::from_bits(val)
        }
    }
    impl From<Priv> for u8 {
        #[inline(always)]
        fn from(val: Priv) -> u8 {
            Priv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Pvdo {
        #[doc = "V<sub>DD</sub> is equal or higher than the PVD threshold selected through the PLS\\[2:0\\]
bits."]
        B_0X0 = 0x0,
        #[doc = "V<sub>DD</sub> is lower than the PVD threshold selected through the PLS\\[2:0\\]
bits."]
        B_0X1 = 0x01,
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Svos {
        #[doc = "reserved"]
        B_0X0 = 0x0,
        #[doc = "SVOS5 scale 5"]
        B_0X1 = 0x01,
        #[doc = "SVOS4 scale 4"]
        B_0X2 = 0x02,
        #[doc = "SVOS3 scale 3 (default)."]
        B_0X3 = 0x03,
    }
    impl Svos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Svos {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Svos {
        #[inline(always)]
        fn from(val: u8) -> Svos {
            Svos::from_bits(val)
        }
    }
    impl From<Svos> for u8 {
        #[inline(always)]
        fn from(val: Svos) -> u8 {
            Svos::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Vbrs {
        #[doc = "Charge V<sub>BAT</sub> through a 5 kΩ resistor."]
        B_0X0 = 0x0,
        #[doc = "Charge V<sub>BAT</sub> through a 1.5 kΩ resistor."]
        B_0X1 = 0x01,
    }
    impl Vbrs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vbrs {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vbrs {
        #[inline(always)]
        fn from(val: u8) -> Vbrs {
            Vbrs::from_bits(val)
        }
    }
    impl From<Vbrs> for u8 {
        #[inline(always)]
        fn from(val: Vbrs) -> u8 {
            Vbrs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Vos {
        #[doc = "scale 3 (default)"]
        SCALE3 = 0x0,
        #[doc = "scale 2"]
        SCALE2 = 0x01,
        #[doc = "scale 1"]
        SCALE1 = 0x02,
        #[doc = "scale 0"]
        SCALE0 = 0x03,
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Wupp {
        #[doc = "detection on high level (rising edge)"]
        B_0X0 = 0x0,
        #[doc = "detection on low level (falling edge)"]
        B_0X1 = 0x01,
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Wuppupd {
        #[doc = "no pull-up"]
        B_0X0 = 0x0,
        #[doc = "pull-up"]
        B_0X1 = 0x01,
        #[doc = "pull-down"]
        B_0X2 = 0x02,
        #[doc = "reserved"]
        B_0X3 = 0x03,
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
