#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "DLYBSD address block description."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dlybsd {
    ptr: *mut u8,
}
unsafe impl Send for Dlybsd {}
unsafe impl Sync for Dlybsd {}
impl Dlybsd {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Delay block SDMMC DLL configuration."]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Delay block SDMMC DLL status."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
}
pub mod regs {
    #[doc = "Delay block SDMMC DLL configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg(pub u32);
    impl Cfg {
        #[doc = "DLL enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sdmmc_dll_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DLL enable."]
        #[inline(always)]
        pub const fn set_sdmmc_dll_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "selection of RX delay."]
        #[must_use]
        #[inline(always)]
        pub const fn sdmmc_rx_tap_sel(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x3f;
            val as u8
        }
        #[doc = "selection of RX delay."]
        #[inline(always)]
        pub const fn set_sdmmc_rx_tap_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 1usize)) | (((val as u32) & 0x3f) << 1usize);
        }
        #[doc = "DLL configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn sdmmc_dll_byp_en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "DLL configuration."]
        #[inline(always)]
        pub const fn set_sdmmc_dll_byp_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Bypass command."]
        #[must_use]
        #[inline(always)]
        pub const fn sdmmc_dll_byp_cmd(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x1f;
            val as u8
        }
        #[doc = "Bypass command."]
        #[inline(always)]
        pub const fn set_sdmmc_dll_byp_cmd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 17usize)) | (((val as u32) & 0x1f) << 17usize);
        }
        #[doc = "Antiglitch logic enabled when 1."]
        #[must_use]
        #[inline(always)]
        pub const fn sdmmc_dll_antiglitch_en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Antiglitch logic enabled when 1."]
        #[inline(always)]
        pub const fn set_sdmmc_dll_antiglitch_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Cfg {
        #[inline(always)]
        fn default() -> Cfg {
            Cfg(0)
        }
    }
    impl core::fmt::Debug for Cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfg")
                .field("sdmmc_dll_en", &self.sdmmc_dll_en())
                .field("sdmmc_rx_tap_sel", &self.sdmmc_rx_tap_sel())
                .field("sdmmc_dll_byp_en", &self.sdmmc_dll_byp_en())
                .field("sdmmc_dll_byp_cmd", &self.sdmmc_dll_byp_cmd())
                .field("sdmmc_dll_antiglitch_en", &self.sdmmc_dll_antiglitch_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfg {{ sdmmc_dll_en: {=bool:?}, sdmmc_rx_tap_sel: {=u8:?}, sdmmc_dll_byp_en: {=bool:?}, sdmmc_dll_byp_cmd: {=u8:?}, sdmmc_dll_antiglitch_en: {=bool:?} }}",
                self.sdmmc_dll_en(),
                self.sdmmc_rx_tap_sel(),
                self.sdmmc_dll_byp_en(),
                self.sdmmc_dll_byp_cmd(),
                self.sdmmc_dll_antiglitch_en()
            )
        }
    }
    #[doc = "Delay block SDMMC DLL status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "SDMMC DLL lock."]
        #[must_use]
        #[inline(always)]
        pub const fn sdmmc_dll_lock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC DLL lock."]
        #[inline(always)]
        pub const fn set_sdmmc_dll_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SDMMC RX delay selection acknowledge."]
        #[must_use]
        #[inline(always)]
        pub const fn sdmmc_rx_tap_sel_ack(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC RX delay selection acknowledge."]
        #[inline(always)]
        pub const fn set_sdmmc_rx_tap_sel_ack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Status {
        #[inline(always)]
        fn default() -> Status {
            Status(0)
        }
    }
    impl core::fmt::Debug for Status {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Status")
                .field("sdmmc_dll_lock", &self.sdmmc_dll_lock())
                .field("sdmmc_rx_tap_sel_ack", &self.sdmmc_rx_tap_sel_ack())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Status {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Status {{ sdmmc_dll_lock: {=bool:?}, sdmmc_rx_tap_sel_ack: {=bool:?} }}",
                self.sdmmc_dll_lock(),
                self.sdmmc_rx_tap_sel_ack()
            )
        }
    }
}
