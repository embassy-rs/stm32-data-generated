#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "PWR address block description"]
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
    #[doc = "PWR control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "PWR control register 3"]
    #[inline(always)]
    pub const fn cr3(self) -> crate::common::Reg<regs::Cr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "PWR control register 4"]
    #[inline(always)]
    pub const fn cr4(self) -> crate::common::Reg<regs::Cr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "PWR status register 1"]
    #[inline(always)]
    pub const fn sr1(self) -> crate::common::Reg<regs::Sr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "PWR status register 2"]
    #[inline(always)]
    pub const fn sr2(self) -> crate::common::Reg<regs::Sr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "PWR status clear register"]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::Scr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "PWR Port pull-up control register"]
    #[inline(always)]
    pub const fn pucr(self, n: usize) -> crate::common::Reg<regs::Pcr, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize + n * 8usize) as _) }
    }
    #[doc = "PWR Port pull-down control register"]
    #[inline(always)]
    pub const fn pdcr(self, n: usize) -> crate::common::Reg<regs::Pcr, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize + n * 8usize) as _) }
    }
}
pub mod regs {
    #[doc = "PWR control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "Low-power mode selection These bits select the low-power mode entered when CPU enters deepsleep mode. 1XX: Shutdown mode"]
        #[inline(always)]
        pub const fn lpms(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Low-power mode selection These bits select the low-power mode entered when CPU enters deepsleep mode. 1XX: Shutdown mode"]
        #[inline(always)]
        pub fn set_lpms(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Flash memory powered down during Stop mode This bit determines whether the Flash memory is put in power-down mode or remains in idle mode when the device enters Stop mode."]
        #[inline(always)]
        pub const fn fpd_stop(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Flash memory powered down during Stop mode This bit determines whether the Flash memory is put in power-down mode or remains in idle mode when the device enters Stop mode."]
        #[inline(always)]
        pub fn set_fpd_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Flash memory powered down during Sleep mode This bit determines whether the Flash memory is put in power-down mode or remains in idle mode when the device enters Sleep mode."]
        #[inline(always)]
        pub const fn fpd_slp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Flash memory powered down during Sleep mode This bit determines whether the Flash memory is put in power-down mode or remains in idle mode when the device enters Sleep mode."]
        #[inline(always)]
        pub fn set_fpd_slp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("lpms", &self.lpms())
                .field("fpd_stop", &self.fpd_stop())
                .field("fpd_slp", &self.fpd_slp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cr1 {
                lpms: u8,
                fpd_stop: bool,
                fpd_slp: bool,
            }
            let proxy = Cr1 {
                lpms: self.lpms(),
                fpd_stop: self.fpd_stop(),
                fpd_slp: self.fpd_slp(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "PWR control register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr3(pub u32);
    impl Cr3 {
        #[doc = "Enable Wakeup pin"]
        #[inline(always)]
        pub const fn ewup(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Enable Wakeup pin"]
        #[inline(always)]
        pub fn set_ewup(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Apply pull-up and pull-down configuration This bit determines whether the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied."]
        #[inline(always)]
        pub const fn apc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Apply pull-up and pull-down configuration This bit determines whether the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied."]
        #[inline(always)]
        pub fn set_apc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Enable internal wakeup line When set, a rising edge on the internal wakeup line triggers a wakeup event."]
        #[inline(always)]
        pub const fn eiwul(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Enable internal wakeup line When set, a rising edge on the internal wakeup line triggers a wakeup event."]
        #[inline(always)]
        pub fn set_eiwul(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Cr3 {
        #[inline(always)]
        fn default() -> Cr3 {
            Cr3(0)
        }
    }
    impl core::fmt::Debug for Cr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr3")
                .field(
                    "ewup",
                    &[
                        self.ewup(0usize),
                        self.ewup(1usize),
                        self.ewup(2usize),
                        self.ewup(3usize),
                        self.ewup(4usize),
                        self.ewup(5usize),
                    ],
                )
                .field("apc", &self.apc())
                .field("eiwul", &self.eiwul())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr3 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cr3 {
                ewup: [bool; 6usize],
                apc: bool,
                eiwul: bool,
            }
            let proxy = Cr3 {
                ewup: [
                    self.ewup(0usize),
                    self.ewup(1usize),
                    self.ewup(2usize),
                    self.ewup(3usize),
                    self.ewup(4usize),
                    self.ewup(5usize),
                ],
                apc: self.apc(),
                eiwul: self.eiwul(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "PWR control register 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr4(pub u32);
    impl Cr4 {
        #[doc = "Wakeup pin WKUP1 polarity"]
        #[inline(always)]
        pub const fn wp(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Wakeup pin WKUP1 polarity"]
        #[inline(always)]
        pub fn set_wp(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Cr4 {
        #[inline(always)]
        fn default() -> Cr4 {
            Cr4(0)
        }
    }
    impl core::fmt::Debug for Cr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr4")
                .field(
                    "wp",
                    &[
                        self.wp(0usize),
                        self.wp(1usize),
                        self.wp(2usize),
                        self.wp(3usize),
                        self.wp(4usize),
                        self.wp(5usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr4 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cr4 {
                wp: [bool; 6usize],
            }
            let proxy = Cr4 {
                wp: [
                    self.wp(0usize),
                    self.wp(1usize),
                    self.wp(2usize),
                    self.wp(3usize),
                    self.wp(4usize),
                    self.wp(5usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Power Port pull control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcr(pub u32);
    impl Pcr {
        #[doc = "Port pull bit y (y=0..15)"]
        #[inline(always)]
        pub const fn p(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Port pull bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_p(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Pcr {
        #[inline(always)]
        fn default() -> Pcr {
            Pcr(0)
        }
    }
    impl core::fmt::Debug for Pcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcr")
                .field(
                    "p",
                    &[
                        self.p(0usize),
                        self.p(1usize),
                        self.p(2usize),
                        self.p(3usize),
                        self.p(4usize),
                        self.p(5usize),
                        self.p(6usize),
                        self.p(7usize),
                        self.p(8usize),
                        self.p(9usize),
                        self.p(10usize),
                        self.p(11usize),
                        self.p(12usize),
                        self.p(13usize),
                        self.p(14usize),
                        self.p(15usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pcr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Pcr {
                p: [bool; 16usize],
            }
            let proxy = Pcr {
                p: [
                    self.p(0usize),
                    self.p(1usize),
                    self.p(2usize),
                    self.p(3usize),
                    self.p(4usize),
                    self.p(5usize),
                    self.p(6usize),
                    self.p(7usize),
                    self.p(8usize),
                    self.p(9usize),
                    self.p(10usize),
                    self.p(11usize),
                    self.p(12usize),
                    self.p(13usize),
                    self.p(14usize),
                    self.p(15usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "PWR status clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scr(pub u32);
    impl Scr {
        #[doc = "Clear Wakeup flag"]
        #[inline(always)]
        pub const fn cwuf(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Clear Wakeup flag"]
        #[inline(always)]
        pub fn set_cwuf(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Clear standby flag Setting this bit clears the SBF flag in the PWR_SR1 register."]
        #[inline(always)]
        pub const fn csbf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Clear standby flag Setting this bit clears the SBF flag in the PWR_SR1 register."]
        #[inline(always)]
        pub fn set_csbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Scr {
        #[inline(always)]
        fn default() -> Scr {
            Scr(0)
        }
    }
    impl core::fmt::Debug for Scr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scr")
                .field(
                    "cwuf",
                    &[
                        self.cwuf(0usize),
                        self.cwuf(1usize),
                        self.cwuf(2usize),
                        self.cwuf(3usize),
                        self.cwuf(4usize),
                        self.cwuf(5usize),
                    ],
                )
                .field("csbf", &self.csbf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Scr {
                cwuf: [bool; 6usize],
                csbf: bool,
            }
            let proxy = Scr {
                cwuf: [
                    self.cwuf(0usize),
                    self.cwuf(1usize),
                    self.cwuf(2usize),
                    self.cwuf(3usize),
                    self.cwuf(4usize),
                    self.cwuf(5usize),
                ],
                csbf: self.csbf(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "PWR status register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr1(pub u32);
    impl Sr1 {
        #[doc = "Wakeup flag"]
        #[inline(always)]
        pub const fn wuf(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Wakeup flag"]
        #[inline(always)]
        pub fn set_wuf(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Standby/Shutdown flag This bit is set by hardware when the device enters Standby or Shutdown mode and is cleared by setting the CSBF bit in the PWR_SCR register, or by a power-on reset. It is not cleared by the system reset."]
        #[inline(always)]
        pub const fn sbf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Standby/Shutdown flag This bit is set by hardware when the device enters Standby or Shutdown mode and is cleared by setting the CSBF bit in the PWR_SCR register, or by a power-on reset. It is not cleared by the system reset."]
        #[inline(always)]
        pub fn set_sbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Wakeup flag internal This bit is set when a wakeup condition is detected on the internal wakeup line. It is cleared when all internal wakeup sources are cleared."]
        #[inline(always)]
        pub const fn wufi(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup flag internal This bit is set when a wakeup condition is detected on the internal wakeup line. It is cleared when all internal wakeup sources are cleared."]
        #[inline(always)]
        pub fn set_wufi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
                .field(
                    "wuf",
                    &[
                        self.wuf(0usize),
                        self.wuf(1usize),
                        self.wuf(2usize),
                        self.wuf(3usize),
                        self.wuf(4usize),
                        self.wuf(5usize),
                    ],
                )
                .field("sbf", &self.sbf())
                .field("wufi", &self.wufi())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Sr1 {
                wuf: [bool; 6usize],
                sbf: bool,
                wufi: bool,
            }
            let proxy = Sr1 {
                wuf: [
                    self.wuf(0usize),
                    self.wuf(1usize),
                    self.wuf(2usize),
                    self.wuf(3usize),
                    self.wuf(4usize),
                    self.wuf(5usize),
                ],
                sbf: self.sbf(),
                wufi: self.wufi(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "PWR status register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr2(pub u32);
    impl Sr2 {
        #[doc = "Flash ready flag This bit is set by hardware to indicate when the Flash memory is ready to be accessed after wakeup from power-down. To place the Flash memory in power-down, set either FPD_SLP or FPD_STP bit. Note: If the system boots from SRAM, the user application must wait till FLASH_RDY bit is set, prior to jumping to Flash memory."]
        #[inline(always)]
        pub const fn flash_rdy(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Flash ready flag This bit is set by hardware to indicate when the Flash memory is ready to be accessed after wakeup from power-down. To place the Flash memory in power-down, set either FPD_SLP or FPD_STP bit. Note: If the system boots from SRAM, the user application must wait till FLASH_RDY bit is set, prior to jumping to Flash memory."]
        #[inline(always)]
        pub fn set_flash_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Sr2 {
        #[inline(always)]
        fn default() -> Sr2 {
            Sr2(0)
        }
    }
    impl core::fmt::Debug for Sr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr2").field("flash_rdy", &self.flash_rdy()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Sr2 {
                flash_rdy: bool,
            }
            let proxy = Sr2 {
                flash_rdy: self.flash_rdy(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
