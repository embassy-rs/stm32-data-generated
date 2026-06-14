#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Secure digital input/output MultiMediaCard interface."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdmmc {
    ptr: *mut u8,
}
unsafe impl Send for Sdmmc {}
unsafe impl Sync for Sdmmc {}
impl Sdmmc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SDMMC power control register."]
    #[inline(always)]
    pub const fn power(self) -> crate::common::Reg<regs::Power, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "SDMMC clock control register."]
    #[inline(always)]
    pub const fn clkcr(self) -> crate::common::Reg<regs::Clkcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "SDMMC argument register."]
    #[inline(always)]
    pub const fn argr(self) -> crate::common::Reg<regs::Argr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "SDMMC command register."]
    #[inline(always)]
    pub const fn cmdr(self) -> crate::common::Reg<regs::Cmdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "SDMMC command response register."]
    #[inline(always)]
    pub const fn respcmdr(self) -> crate::common::Reg<regs::Respcmdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "SDMMC response 1 register."]
    #[inline(always)]
    pub const fn respr(self, n: usize) -> crate::common::Reg<regs::ResPxR, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize + n * 4usize) as _) }
    }
    #[doc = "SDMMC data timer register."]
    #[inline(always)]
    pub const fn dtimer(self) -> crate::common::Reg<regs::Dtimer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "SDMMC data length register."]
    #[inline(always)]
    pub const fn dlenr(self) -> crate::common::Reg<regs::Dlenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "SDMMC data control register."]
    #[inline(always)]
    pub const fn dctrl(self) -> crate::common::Reg<regs::Dctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "SDMMC data counter register."]
    #[inline(always)]
    pub const fn dcntr(self) -> crate::common::Reg<regs::Dcntr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "SDMMC status register."]
    #[inline(always)]
    pub const fn star(self) -> crate::common::Reg<regs::Star, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "SDMMC interrupt clear register."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "SDMMC mask register."]
    #[inline(always)]
    pub const fn maskr(self) -> crate::common::Reg<regs::Maskr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "SDMMC acknowledgment timer register."]
    #[inline(always)]
    pub const fn acktimer(self) -> crate::common::Reg<regs::Acktimer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "SDMMC data FIFO threshold register."]
    #[inline(always)]
    pub const fn fifothrr(self) -> crate::common::Reg<regs::Fifothrr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "SDMMC DMA control register."]
    #[inline(always)]
    pub const fn idmactrlr(self) -> crate::common::Reg<regs::Idmactrlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "SDMMC IDMA buffer size register."]
    #[inline(always)]
    pub const fn idmabsizer(self) -> crate::common::Reg<regs::Idmabsizer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "SDMMC IDMA buffer base address register."]
    #[inline(always)]
    pub const fn idmabaser(self) -> crate::common::Reg<regs::Idmabaser, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "SDMMC IDMA linked list address register."]
    #[inline(always)]
    pub const fn idmalar(self) -> crate::common::Reg<regs::Idmalar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "SDMMC IDMA linked list memory base register."]
    #[inline(always)]
    pub const fn idmabar(self) -> crate::common::Reg<regs::Idmabar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "SDMMC data FIFO registers 0."]
    #[inline(always)]
    pub const fn fifor(self, n: usize) -> crate::common::Reg<regs::Fifor, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "SDMMC acknowledgment timer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Acktimer(pub u32);
    impl Acktimer {
        #[doc = "Boot acknowledgment timeout period."]
        #[must_use]
        #[inline(always)]
        pub const fn acktime(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "Boot acknowledgment timeout period."]
        #[inline(always)]
        pub const fn set_acktime(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
        }
    }
    impl Default for Acktimer {
        #[inline(always)]
        fn default() -> Acktimer {
            Acktimer(0)
        }
    }
    impl core::fmt::Debug for Acktimer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Acktimer").field("acktime", &self.acktime()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Acktimer {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Acktimer {{ acktime: {=u32:?} }}", self.acktime())
        }
    }
    #[doc = "SDMMC argument register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Argr(pub u32);
    impl Argr {
        #[doc = "Command argument."]
        #[must_use]
        #[inline(always)]
        pub const fn cmdarg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Command argument."]
        #[inline(always)]
        pub const fn set_cmdarg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Argr {
        #[inline(always)]
        fn default() -> Argr {
            Argr(0)
        }
    }
    impl core::fmt::Debug for Argr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Argr").field("cmdarg", &self.cmdarg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Argr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Argr {{ cmdarg: {=u32:?} }}", self.cmdarg())
        }
    }
    #[doc = "SDMMC clock control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Clkcr(pub u32);
    impl Clkcr {
        #[doc = "Clock divide factor."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Clock divide factor."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Power saving configuration bit."]
        #[must_use]
        #[inline(always)]
        pub const fn pwrsav(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Power saving configuration bit."]
        #[inline(always)]
        pub const fn set_pwrsav(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Wide bus mode enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn widbus(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Wide bus mode enable bit."]
        #[inline(always)]
        pub const fn set_widbus(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "SDMMC_CK dephasing selection bit for data and command."]
        #[must_use]
        #[inline(always)]
        pub const fn negedge(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC_CK dephasing selection bit for data and command."]
        #[inline(always)]
        pub const fn set_negedge(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Hardware flow control enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hwfc_en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Hardware flow control enable."]
        #[inline(always)]
        pub const fn set_hwfc_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Data rate signaling selection."]
        #[must_use]
        #[inline(always)]
        pub const fn ddr(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Data rate signaling selection."]
        #[inline(always)]
        pub const fn set_ddr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Bus speed for selection of SDMMC operating modes."]
        #[must_use]
        #[inline(always)]
        pub const fn busspeed(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Bus speed for selection of SDMMC operating modes."]
        #[inline(always)]
        pub const fn set_busspeed(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Receive clock selection."]
        #[must_use]
        #[inline(always)]
        pub const fn selclkrx(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Receive clock selection."]
        #[inline(always)]
        pub const fn set_selclkrx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
    }
    impl Default for Clkcr {
        #[inline(always)]
        fn default() -> Clkcr {
            Clkcr(0)
        }
    }
    impl core::fmt::Debug for Clkcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Clkcr")
                .field("clkdiv", &self.clkdiv())
                .field("pwrsav", &self.pwrsav())
                .field("widbus", &self.widbus())
                .field("negedge", &self.negedge())
                .field("hwfc_en", &self.hwfc_en())
                .field("ddr", &self.ddr())
                .field("busspeed", &self.busspeed())
                .field("selclkrx", &self.selclkrx())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Clkcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Clkcr {{ clkdiv: {=u16:?}, pwrsav: {=bool:?}, widbus: {=u8:?}, negedge: {=bool:?}, hwfc_en: {=bool:?}, ddr: {=bool:?}, busspeed: {=bool:?}, selclkrx: {=u8:?} }}",
                self.clkdiv(),
                self.pwrsav(),
                self.widbus(),
                self.negedge(),
                self.hwfc_en(),
                self.ddr(),
                self.busspeed(),
                self.selclkrx()
            )
        }
    }
    #[doc = "SDMMC command register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmdr(pub u32);
    impl Cmdr {
        #[doc = "Command index."]
        #[must_use]
        #[inline(always)]
        pub const fn cmdindex(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Command index."]
        #[inline(always)]
        pub const fn set_cmdindex(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "The CPSM treats the command as a data transfer command, stops the interrupt period, and signals DataEnable to the DPSM."]
        #[must_use]
        #[inline(always)]
        pub const fn cmdtrans(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "The CPSM treats the command as a data transfer command, stops the interrupt period, and signals DataEnable to the DPSM."]
        #[inline(always)]
        pub const fn set_cmdtrans(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "The CPSM treats the command as a Stop Transmission command and signals abort to the DPSM."]
        #[must_use]
        #[inline(always)]
        pub const fn cmdstop(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The CPSM treats the command as a Stop Transmission command and signals abort to the DPSM."]
        #[inline(always)]
        pub const fn set_cmdstop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Wait for response bits."]
        #[must_use]
        #[inline(always)]
        pub const fn waitresp(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Wait for response bits."]
        #[inline(always)]
        pub const fn set_waitresp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "CPSM waits for interrupt request."]
        #[must_use]
        #[inline(always)]
        pub const fn waitint(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "CPSM waits for interrupt request."]
        #[inline(always)]
        pub const fn set_waitint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "CPSM waits for end of data transfer (CmdPend internal signal) from DPSM."]
        #[must_use]
        #[inline(always)]
        pub const fn waitpend(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "CPSM waits for end of data transfer (CmdPend internal signal) from DPSM."]
        #[inline(always)]
        pub const fn set_waitpend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Command path state machine (CPSM) enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cpsmen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Command path state machine (CPSM) enable bit."]
        #[inline(always)]
        pub const fn set_cpsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Hold new data block transmission and reception in the DPSM."]
        #[must_use]
        #[inline(always)]
        pub const fn dthold(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Hold new data block transmission and reception in the DPSM."]
        #[inline(always)]
        pub const fn set_dthold(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Select the boot mode procedure to be used."]
        #[must_use]
        #[inline(always)]
        pub const fn bootmode(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Select the boot mode procedure to be used."]
        #[inline(always)]
        pub const fn set_bootmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Enable boot mode procedure."]
        #[must_use]
        #[inline(always)]
        pub const fn booten(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Enable boot mode procedure."]
        #[inline(always)]
        pub const fn set_booten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "The CPSM treats the command as a Suspend or Resume command and signals interrupt period start/end."]
        #[must_use]
        #[inline(always)]
        pub const fn cmdsuspend(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "The CPSM treats the command as a Suspend or Resume command and signals interrupt period start/end."]
        #[inline(always)]
        pub const fn set_cmdsuspend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Cmdr {
        #[inline(always)]
        fn default() -> Cmdr {
            Cmdr(0)
        }
    }
    impl core::fmt::Debug for Cmdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmdr")
                .field("cmdindex", &self.cmdindex())
                .field("cmdtrans", &self.cmdtrans())
                .field("cmdstop", &self.cmdstop())
                .field("waitresp", &self.waitresp())
                .field("waitint", &self.waitint())
                .field("waitpend", &self.waitpend())
                .field("cpsmen", &self.cpsmen())
                .field("dthold", &self.dthold())
                .field("bootmode", &self.bootmode())
                .field("booten", &self.booten())
                .field("cmdsuspend", &self.cmdsuspend())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cmdr {{ cmdindex: {=u8:?}, cmdtrans: {=bool:?}, cmdstop: {=bool:?}, waitresp: {=u8:?}, waitint: {=bool:?}, waitpend: {=bool:?}, cpsmen: {=bool:?}, dthold: {=bool:?}, bootmode: {=bool:?}, booten: {=bool:?}, cmdsuspend: {=bool:?} }}",
                self.cmdindex(),
                self.cmdtrans(),
                self.cmdstop(),
                self.waitresp(),
                self.waitint(),
                self.waitpend(),
                self.cpsmen(),
                self.dthold(),
                self.bootmode(),
                self.booten(),
                self.cmdsuspend()
            )
        }
    }
    #[doc = "SDMMC data counter register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dcntr(pub u32);
    impl Dcntr {
        #[doc = "Data count value."]
        #[must_use]
        #[inline(always)]
        pub const fn datacount(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "Data count value."]
        #[inline(always)]
        pub const fn set_datacount(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
        }
    }
    impl Default for Dcntr {
        #[inline(always)]
        fn default() -> Dcntr {
            Dcntr(0)
        }
    }
    impl core::fmt::Debug for Dcntr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dcntr").field("datacount", &self.datacount()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dcntr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dcntr {{ datacount: {=u32:?} }}", self.datacount())
        }
    }
    #[doc = "SDMMC data control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dctrl(pub u32);
    impl Dctrl {
        #[doc = "Data transfer enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn dten(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Data transfer enable bit."]
        #[inline(always)]
        pub const fn set_dten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Data transfer direction selection."]
        #[must_use]
        #[inline(always)]
        pub const fn dtdir(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Data transfer direction selection."]
        #[inline(always)]
        pub const fn set_dtdir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Data transfer mode selection."]
        #[must_use]
        #[inline(always)]
        pub const fn dtmode(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Data transfer mode selection."]
        #[inline(always)]
        pub const fn set_dtmode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Data block size."]
        #[must_use]
        #[inline(always)]
        pub const fn dblocksize(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Data block size."]
        #[inline(always)]
        pub const fn set_dblocksize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Read Wait start."]
        #[must_use]
        #[inline(always)]
        pub const fn rwstart(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Read Wait start."]
        #[inline(always)]
        pub const fn set_rwstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Read Wait stop."]
        #[must_use]
        #[inline(always)]
        pub const fn rwstop(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Read Wait stop."]
        #[inline(always)]
        pub const fn set_rwstop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Read Wait mode."]
        #[must_use]
        #[inline(always)]
        pub const fn rwmod(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Read Wait mode."]
        #[inline(always)]
        pub const fn set_rwmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "SD I/O interrupt enable functions."]
        #[must_use]
        #[inline(always)]
        pub const fn sdioen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "SD I/O interrupt enable functions."]
        #[inline(always)]
        pub const fn set_sdioen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Enable the reception of the boot acknowledgment."]
        #[must_use]
        #[inline(always)]
        pub const fn bootacken(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Enable the reception of the boot acknowledgment."]
        #[inline(always)]
        pub const fn set_bootacken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "FIFO reset, flushes any remaining data."]
        #[must_use]
        #[inline(always)]
        pub const fn fiforst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO reset, flushes any remaining data."]
        #[inline(always)]
        pub const fn set_fiforst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Dctrl {
        #[inline(always)]
        fn default() -> Dctrl {
            Dctrl(0)
        }
    }
    impl core::fmt::Debug for Dctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dctrl")
                .field("dten", &self.dten())
                .field("dtdir", &self.dtdir())
                .field("dtmode", &self.dtmode())
                .field("dblocksize", &self.dblocksize())
                .field("rwstart", &self.rwstart())
                .field("rwstop", &self.rwstop())
                .field("rwmod", &self.rwmod())
                .field("sdioen", &self.sdioen())
                .field("bootacken", &self.bootacken())
                .field("fiforst", &self.fiforst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dctrl {{ dten: {=bool:?}, dtdir: {=bool:?}, dtmode: {=u8:?}, dblocksize: {=u8:?}, rwstart: {=bool:?}, rwstop: {=bool:?}, rwmod: {=bool:?}, sdioen: {=bool:?}, bootacken: {=bool:?}, fiforst: {=bool:?} }}",
                self.dten(),
                self.dtdir(),
                self.dtmode(),
                self.dblocksize(),
                self.rwstart(),
                self.rwstop(),
                self.rwmod(),
                self.sdioen(),
                self.bootacken(),
                self.fiforst()
            )
        }
    }
    #[doc = "SDMMC data length register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlenr(pub u32);
    impl Dlenr {
        #[doc = "Data length value."]
        #[must_use]
        #[inline(always)]
        pub const fn datalength(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "Data length value."]
        #[inline(always)]
        pub const fn set_datalength(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
        }
    }
    impl Default for Dlenr {
        #[inline(always)]
        fn default() -> Dlenr {
            Dlenr(0)
        }
    }
    impl core::fmt::Debug for Dlenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dlenr").field("datalength", &self.datalength()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dlenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dlenr {{ datalength: {=u32:?} }}", self.datalength())
        }
    }
    #[doc = "SDMMC data timer register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtimer(pub u32);
    impl Dtimer {
        #[doc = "Data and R1b busy timeout period."]
        #[must_use]
        #[inline(always)]
        pub const fn datatime(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Data and R1b busy timeout period."]
        #[inline(always)]
        pub const fn set_datatime(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dtimer {
        #[inline(always)]
        fn default() -> Dtimer {
            Dtimer(0)
        }
    }
    impl core::fmt::Debug for Dtimer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dtimer").field("datatime", &self.datatime()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dtimer {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dtimer {{ datatime: {=u32:?} }}", self.datatime())
        }
    }
    #[doc = "SDMMC data FIFO registers 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fifor(pub u32);
    impl Fifor {
        #[doc = "Receive and transmit FIFO data."]
        #[must_use]
        #[inline(always)]
        pub const fn fifodata(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Receive and transmit FIFO data."]
        #[inline(always)]
        pub const fn set_fifodata(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Fifor {
        #[inline(always)]
        fn default() -> Fifor {
            Fifor(0)
        }
    }
    impl core::fmt::Debug for Fifor {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fifor").field("fifodata", &self.fifodata()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fifor {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Fifor {{ fifodata: {=u32:?} }}", self.fifodata())
        }
    }
    #[doc = "SDMMC data FIFO threshold register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fifothrr(pub u32);
    impl Fifothrr {
        #[doc = "FIFO threshold."]
        #[must_use]
        #[inline(always)]
        pub const fn thr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "FIFO threshold."]
        #[inline(always)]
        pub const fn set_thr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Fifothrr {
        #[inline(always)]
        fn default() -> Fifothrr {
            Fifothrr(0)
        }
    }
    impl core::fmt::Debug for Fifothrr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fifothrr").field("thr", &self.thr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fifothrr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Fifothrr {{ thr: {=u8:?} }}", self.thr())
        }
    }
    #[doc = "SDMMC interrupt clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "CCRCFAIL flag clear bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ccrcfailc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CCRCFAIL flag clear bit."]
        #[inline(always)]
        pub const fn set_ccrcfailc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DCRCFAIL flag clear bit."]
        #[must_use]
        #[inline(always)]
        pub const fn dcrcfailc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DCRCFAIL flag clear bit."]
        #[inline(always)]
        pub const fn set_dcrcfailc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CTIMEOUT flag clear bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ctimeoutc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "CTIMEOUT flag clear bit."]
        #[inline(always)]
        pub const fn set_ctimeoutc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "DTIMEOUT flag clear bit."]
        #[must_use]
        #[inline(always)]
        pub const fn dtimeoutc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DTIMEOUT flag clear bit."]
        #[inline(always)]
        pub const fn set_dtimeoutc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TXUNDERR flag clear bit."]
        #[must_use]
        #[inline(always)]
        pub const fn txunderrc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TXUNDERR flag clear bit."]
        #[inline(always)]
        pub const fn set_txunderrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "RXOVERR flag clear bit."]
        #[must_use]
        #[inline(always)]
        pub const fn rxoverrc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RXOVERR flag clear bit."]
        #[inline(always)]
        pub const fn set_rxoverrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "CMDREND flag clear bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cmdrendc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "CMDREND flag clear bit."]
        #[inline(always)]
        pub const fn set_cmdrendc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "CMDSENT flag clear bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cmdsentc(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CMDSENT flag clear bit."]
        #[inline(always)]
        pub const fn set_cmdsentc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "DATAEND flag clear bit."]
        #[must_use]
        #[inline(always)]
        pub const fn dataendc(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "DATAEND flag clear bit."]
        #[inline(always)]
        pub const fn set_dataendc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "DHOLD flag clear bit."]
        #[must_use]
        #[inline(always)]
        pub const fn dholdc(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "DHOLD flag clear bit."]
        #[inline(always)]
        pub const fn set_dholdc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "DBCKEND flag clear bit."]
        #[must_use]
        #[inline(always)]
        pub const fn dbckendc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "DBCKEND flag clear bit."]
        #[inline(always)]
        pub const fn set_dbckendc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "DABORT flag clear bit."]
        #[must_use]
        #[inline(always)]
        pub const fn dabortc(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "DABORT flag clear bit."]
        #[inline(always)]
        pub const fn set_dabortc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "BUSYD0END flag clear bit."]
        #[must_use]
        #[inline(always)]
        pub const fn busyd0endc(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "BUSYD0END flag clear bit."]
        #[inline(always)]
        pub const fn set_busyd0endc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SDIOIT flag clear bit."]
        #[must_use]
        #[inline(always)]
        pub const fn sdioitc(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SDIOIT flag clear bit."]
        #[inline(always)]
        pub const fn set_sdioitc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "ACKFAIL flag clear bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ackfailc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "ACKFAIL flag clear bit."]
        #[inline(always)]
        pub const fn set_ackfailc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "ACKTIMEOUT flag clear bit."]
        #[must_use]
        #[inline(always)]
        pub const fn acktimeoutc(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ACKTIMEOUT flag clear bit."]
        #[inline(always)]
        pub const fn set_acktimeoutc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "VSWEND flag clear bit."]
        #[must_use]
        #[inline(always)]
        pub const fn vswendc(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "VSWEND flag clear bit."]
        #[inline(always)]
        pub const fn set_vswendc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "CKSTOP flag clear bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ckstopc(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "CKSTOP flag clear bit."]
        #[inline(always)]
        pub const fn set_ckstopc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "IDMA transfer error clear bit."]
        #[must_use]
        #[inline(always)]
        pub const fn idmatec(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "IDMA transfer error clear bit."]
        #[inline(always)]
        pub const fn set_idmatec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "IDMA buffer transfer complete clear bit."]
        #[must_use]
        #[inline(always)]
        pub const fn idmabtcc(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "IDMA buffer transfer complete clear bit."]
        #[inline(always)]
        pub const fn set_idmabtcc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Icr {
        #[inline(always)]
        fn default() -> Icr {
            Icr(0)
        }
    }
    impl core::fmt::Debug for Icr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icr")
                .field("ccrcfailc", &self.ccrcfailc())
                .field("dcrcfailc", &self.dcrcfailc())
                .field("ctimeoutc", &self.ctimeoutc())
                .field("dtimeoutc", &self.dtimeoutc())
                .field("txunderrc", &self.txunderrc())
                .field("rxoverrc", &self.rxoverrc())
                .field("cmdrendc", &self.cmdrendc())
                .field("cmdsentc", &self.cmdsentc())
                .field("dataendc", &self.dataendc())
                .field("dholdc", &self.dholdc())
                .field("dbckendc", &self.dbckendc())
                .field("dabortc", &self.dabortc())
                .field("busyd0endc", &self.busyd0endc())
                .field("sdioitc", &self.sdioitc())
                .field("ackfailc", &self.ackfailc())
                .field("acktimeoutc", &self.acktimeoutc())
                .field("vswendc", &self.vswendc())
                .field("ckstopc", &self.ckstopc())
                .field("idmatec", &self.idmatec())
                .field("idmabtcc", &self.idmabtcc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icr {{ ccrcfailc: {=bool:?}, dcrcfailc: {=bool:?}, ctimeoutc: {=bool:?}, dtimeoutc: {=bool:?}, txunderrc: {=bool:?}, rxoverrc: {=bool:?}, cmdrendc: {=bool:?}, cmdsentc: {=bool:?}, dataendc: {=bool:?}, dholdc: {=bool:?}, dbckendc: {=bool:?}, dabortc: {=bool:?}, busyd0endc: {=bool:?}, sdioitc: {=bool:?}, ackfailc: {=bool:?}, acktimeoutc: {=bool:?}, vswendc: {=bool:?}, ckstopc: {=bool:?}, idmatec: {=bool:?}, idmabtcc: {=bool:?} }}",
                self.ccrcfailc(),
                self.dcrcfailc(),
                self.ctimeoutc(),
                self.dtimeoutc(),
                self.txunderrc(),
                self.rxoverrc(),
                self.cmdrendc(),
                self.cmdsentc(),
                self.dataendc(),
                self.dholdc(),
                self.dbckendc(),
                self.dabortc(),
                self.busyd0endc(),
                self.sdioitc(),
                self.ackfailc(),
                self.acktimeoutc(),
                self.vswendc(),
                self.ckstopc(),
                self.idmatec(),
                self.idmabtcc()
            )
        }
    }
    #[doc = "SDMMC IDMA linked list memory base register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idmabar(pub u32);
    impl Idmabar {
        #[doc = "Word aligned Linked list memory base address."]
        #[must_use]
        #[inline(always)]
        pub const fn idmaba(&self) -> u32 {
            let val = (self.0 >> 2usize) & 0x3fff_ffff;
            val as u32
        }
        #[doc = "Word aligned Linked list memory base address."]
        #[inline(always)]
        pub const fn set_idmaba(&mut self, val: u32) {
            self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
        }
    }
    impl Default for Idmabar {
        #[inline(always)]
        fn default() -> Idmabar {
            Idmabar(0)
        }
    }
    impl core::fmt::Debug for Idmabar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Idmabar").field("idmaba", &self.idmaba()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Idmabar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Idmabar {{ idmaba: {=u32:?} }}", self.idmaba())
        }
    }
    #[doc = "SDMMC IDMA buffer base address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idmabaser(pub u32);
    impl Idmabaser {
        #[doc = "Buffer memory base address bits \\[31:2\\], must be word aligned (bit \\[1:0\\]
are always 0 and read only)."]
        #[must_use]
        #[inline(always)]
        pub const fn idmabase(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Buffer memory base address bits \\[31:2\\], must be word aligned (bit \\[1:0\\]
are always 0 and read only)."]
        #[inline(always)]
        pub const fn set_idmabase(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Idmabaser {
        #[inline(always)]
        fn default() -> Idmabaser {
            Idmabaser(0)
        }
    }
    impl core::fmt::Debug for Idmabaser {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Idmabaser").field("idmabase", &self.idmabase()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Idmabaser {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Idmabaser {{ idmabase: {=u32:?} }}", self.idmabase())
        }
    }
    #[doc = "SDMMC IDMA buffer size register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idmabsizer(pub u32);
    impl Idmabsizer {
        #[doc = "Number of bytes per buffer."]
        #[must_use]
        #[inline(always)]
        pub const fn idmabndt(&self) -> u16 {
            let val = (self.0 >> 6usize) & 0x07ff;
            val as u16
        }
        #[doc = "Number of bytes per buffer."]
        #[inline(always)]
        pub const fn set_idmabndt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 6usize)) | (((val as u32) & 0x07ff) << 6usize);
        }
    }
    impl Default for Idmabsizer {
        #[inline(always)]
        fn default() -> Idmabsizer {
            Idmabsizer(0)
        }
    }
    impl core::fmt::Debug for Idmabsizer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Idmabsizer")
                .field("idmabndt", &self.idmabndt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Idmabsizer {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Idmabsizer {{ idmabndt: {=u16:?} }}", self.idmabndt())
        }
    }
    #[doc = "SDMMC DMA control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idmactrlr(pub u32);
    impl Idmactrlr {
        #[doc = "IDMA enable."]
        #[must_use]
        #[inline(always)]
        pub const fn idmaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IDMA enable."]
        #[inline(always)]
        pub const fn set_idmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Buffer mode selection."]
        #[must_use]
        #[inline(always)]
        pub const fn idmabmode(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Buffer mode selection."]
        #[inline(always)]
        pub const fn set_idmabmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Idmactrlr {
        #[inline(always)]
        fn default() -> Idmactrlr {
            Idmactrlr(0)
        }
    }
    impl core::fmt::Debug for Idmactrlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Idmactrlr")
                .field("idmaen", &self.idmaen())
                .field("idmabmode", &self.idmabmode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Idmactrlr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Idmactrlr {{ idmaen: {=bool:?}, idmabmode: {=bool:?} }}",
                self.idmaen(),
                self.idmabmode()
            )
        }
    }
    #[doc = "SDMMC IDMA linked list address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idmalar(pub u32);
    impl Idmalar {
        #[doc = "Word aligned linked list item address offset."]
        #[must_use]
        #[inline(always)]
        pub const fn idmala(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x3fff;
            val as u16
        }
        #[doc = "Word aligned linked list item address offset."]
        #[inline(always)]
        pub const fn set_idmala(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
        }
        #[doc = "Acknowledge linked list buffer ready."]
        #[must_use]
        #[inline(always)]
        pub const fn abr(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledge linked list buffer ready."]
        #[inline(always)]
        pub const fn set_abr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Update SDMMC_IDMABSIZE from the next linked list when in linked list mode (SDMMC_IDMACTRLR.IDMABMODE select linked list mode and ULA = 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn uls(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Update SDMMC_IDMABSIZE from the next linked list when in linked list mode (SDMMC_IDMACTRLR.IDMABMODE select linked list mode and ULA = 1)."]
        #[inline(always)]
        pub const fn set_uls(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Update SDMMC_IDMALAR from linked list when in linked list mode (SDMMC_IDMACTRLR.IDMABMODE select linked list mode)."]
        #[must_use]
        #[inline(always)]
        pub const fn ula(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Update SDMMC_IDMALAR from linked list when in linked list mode (SDMMC_IDMACTRLR.IDMABMODE select linked list mode)."]
        #[inline(always)]
        pub const fn set_ula(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Idmalar {
        #[inline(always)]
        fn default() -> Idmalar {
            Idmalar(0)
        }
    }
    impl core::fmt::Debug for Idmalar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Idmalar")
                .field("idmala", &self.idmala())
                .field("abr", &self.abr())
                .field("uls", &self.uls())
                .field("ula", &self.ula())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Idmalar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Idmalar {{ idmala: {=u16:?}, abr: {=bool:?}, uls: {=bool:?}, ula: {=bool:?} }}",
                self.idmala(),
                self.abr(),
                self.uls(),
                self.ula()
            )
        }
    }
    #[doc = "SDMMC mask register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maskr(pub u32);
    impl Maskr {
        #[doc = "Command CRC fail interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ccrcfailie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Command CRC fail interrupt enable."]
        #[inline(always)]
        pub const fn set_ccrcfailie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Data CRC fail interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dcrcfailie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Data CRC fail interrupt enable."]
        #[inline(always)]
        pub const fn set_dcrcfailie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Command timeout interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ctimeoutie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Command timeout interrupt enable."]
        #[inline(always)]
        pub const fn set_ctimeoutie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Data timeout interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dtimeoutie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Data timeout interrupt enable."]
        #[inline(always)]
        pub const fn set_dtimeoutie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Tx FIFO underrun error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn txunderrie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO underrun error interrupt enable."]
        #[inline(always)]
        pub const fn set_txunderrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Rx FIFO overrun error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rxoverrie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO overrun error interrupt enable."]
        #[inline(always)]
        pub const fn set_rxoverrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Command response received interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmdrendie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Command response received interrupt enable."]
        #[inline(always)]
        pub const fn set_cmdrendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Command sent interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmdsentie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Command sent interrupt enable."]
        #[inline(always)]
        pub const fn set_cmdsentie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Data end interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dataendie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Data end interrupt enable."]
        #[inline(always)]
        pub const fn set_dataendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Data hold interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dholdie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Data hold interrupt enable."]
        #[inline(always)]
        pub const fn set_dholdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Data block end interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dbckendie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Data block end interrupt enable."]
        #[inline(always)]
        pub const fn set_dbckendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Data transfer aborted interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dabortie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Data transfer aborted interrupt enable."]
        #[inline(always)]
        pub const fn set_dabortie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Tx FIFO half empty interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn txfifoheie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO half empty interrupt enable."]
        #[inline(always)]
        pub const fn set_txfifoheie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Rx FIFO half full interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rxfifohfie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO half full interrupt enable."]
        #[inline(always)]
        pub const fn set_rxfifohfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Rx FIFO full interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rxfifofie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Rx FIFO full interrupt enable."]
        #[inline(always)]
        pub const fn set_rxfifofie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Tx FIFO empty interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn txfifoeie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Tx FIFO empty interrupt enable."]
        #[inline(always)]
        pub const fn set_txfifoeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "BUSYD0END interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn busyd0endie(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "BUSYD0END interrupt enable."]
        #[inline(always)]
        pub const fn set_busyd0endie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SDIO mode interrupt received interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sdioitie(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SDIO mode interrupt received interrupt enable."]
        #[inline(always)]
        pub const fn set_sdioitie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Acknowledgment Fail interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ackfailie(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledgment Fail interrupt enable."]
        #[inline(always)]
        pub const fn set_ackfailie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Acknowledgment timeout interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn acktimeoutie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Acknowledgment timeout interrupt enable."]
        #[inline(always)]
        pub const fn set_acktimeoutie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Voltage switch critical timing section completion interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn vswendie(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Voltage switch critical timing section completion interrupt enable."]
        #[inline(always)]
        pub const fn set_vswendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Voltage Switch clock stopped interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ckstopie(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Voltage Switch clock stopped interrupt enable."]
        #[inline(always)]
        pub const fn set_ckstopie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "IDMA buffer transfer complete interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn idmabtcie(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "IDMA buffer transfer complete interrupt enable."]
        #[inline(always)]
        pub const fn set_idmabtcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Maskr {
        #[inline(always)]
        fn default() -> Maskr {
            Maskr(0)
        }
    }
    impl core::fmt::Debug for Maskr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maskr")
                .field("ccrcfailie", &self.ccrcfailie())
                .field("dcrcfailie", &self.dcrcfailie())
                .field("ctimeoutie", &self.ctimeoutie())
                .field("dtimeoutie", &self.dtimeoutie())
                .field("txunderrie", &self.txunderrie())
                .field("rxoverrie", &self.rxoverrie())
                .field("cmdrendie", &self.cmdrendie())
                .field("cmdsentie", &self.cmdsentie())
                .field("dataendie", &self.dataendie())
                .field("dholdie", &self.dholdie())
                .field("dbckendie", &self.dbckendie())
                .field("dabortie", &self.dabortie())
                .field("txfifoheie", &self.txfifoheie())
                .field("rxfifohfie", &self.rxfifohfie())
                .field("rxfifofie", &self.rxfifofie())
                .field("txfifoeie", &self.txfifoeie())
                .field("busyd0endie", &self.busyd0endie())
                .field("sdioitie", &self.sdioitie())
                .field("ackfailie", &self.ackfailie())
                .field("acktimeoutie", &self.acktimeoutie())
                .field("vswendie", &self.vswendie())
                .field("ckstopie", &self.ckstopie())
                .field("idmabtcie", &self.idmabtcie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Maskr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Maskr {{ ccrcfailie: {=bool:?}, dcrcfailie: {=bool:?}, ctimeoutie: {=bool:?}, dtimeoutie: {=bool:?}, txunderrie: {=bool:?}, rxoverrie: {=bool:?}, cmdrendie: {=bool:?}, cmdsentie: {=bool:?}, dataendie: {=bool:?}, dholdie: {=bool:?}, dbckendie: {=bool:?}, dabortie: {=bool:?}, txfifoheie: {=bool:?}, rxfifohfie: {=bool:?}, rxfifofie: {=bool:?}, txfifoeie: {=bool:?}, busyd0endie: {=bool:?}, sdioitie: {=bool:?}, ackfailie: {=bool:?}, acktimeoutie: {=bool:?}, vswendie: {=bool:?}, ckstopie: {=bool:?}, idmabtcie: {=bool:?} }}",
                self.ccrcfailie(),
                self.dcrcfailie(),
                self.ctimeoutie(),
                self.dtimeoutie(),
                self.txunderrie(),
                self.rxoverrie(),
                self.cmdrendie(),
                self.cmdsentie(),
                self.dataendie(),
                self.dholdie(),
                self.dbckendie(),
                self.dabortie(),
                self.txfifoheie(),
                self.rxfifohfie(),
                self.rxfifofie(),
                self.txfifoeie(),
                self.busyd0endie(),
                self.sdioitie(),
                self.ackfailie(),
                self.acktimeoutie(),
                self.vswendie(),
                self.ckstopie(),
                self.idmabtcie()
            )
        }
    }
    #[doc = "SDMMC power control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Power(pub u32);
    impl Power {
        #[doc = "SDMMC state control bits."]
        #[must_use]
        #[inline(always)]
        pub const fn pwrctrl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "SDMMC state control bits."]
        #[inline(always)]
        pub const fn set_pwrctrl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Voltage switch sequence start."]
        #[must_use]
        #[inline(always)]
        pub const fn vswitch(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Voltage switch sequence start."]
        #[inline(always)]
        pub const fn set_vswitch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Voltage switch procedure enable."]
        #[must_use]
        #[inline(always)]
        pub const fn vswitchen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Voltage switch procedure enable."]
        #[inline(always)]
        pub const fn set_vswitchen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Data and command direction signals polarity selection."]
        #[must_use]
        #[inline(always)]
        pub const fn dirpol(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Data and command direction signals polarity selection."]
        #[inline(always)]
        pub const fn set_dirpol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Power {
        #[inline(always)]
        fn default() -> Power {
            Power(0)
        }
    }
    impl core::fmt::Debug for Power {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Power")
                .field("pwrctrl", &self.pwrctrl())
                .field("vswitch", &self.vswitch())
                .field("vswitchen", &self.vswitchen())
                .field("dirpol", &self.dirpol())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Power {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Power {{ pwrctrl: {=u8:?}, vswitch: {=bool:?}, vswitchen: {=bool:?}, dirpol: {=bool:?} }}",
                self.pwrctrl(),
                self.vswitch(),
                self.vswitchen(),
                self.dirpol()
            )
        }
    }
    #[doc = "SDMMC response 1 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ResPxR(pub u32);
    impl ResPxR {
        #[doc = "Card status according table below."]
        #[must_use]
        #[inline(always)]
        pub const fn cardstatus(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Card status according table below."]
        #[inline(always)]
        pub const fn set_cardstatus(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for ResPxR {
        #[inline(always)]
        fn default() -> ResPxR {
            ResPxR(0)
        }
    }
    impl core::fmt::Debug for ResPxR {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ResPxR")
                .field("cardstatus", &self.cardstatus())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ResPxR {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "ResPxR {{ cardstatus: {=u32:?} }}", self.cardstatus())
        }
    }
    #[doc = "SDMMC command response register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Respcmdr(pub u32);
    impl Respcmdr {
        #[doc = "Response command index."]
        #[must_use]
        #[inline(always)]
        pub const fn respcmd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Response command index."]
        #[inline(always)]
        pub const fn set_respcmd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for Respcmdr {
        #[inline(always)]
        fn default() -> Respcmdr {
            Respcmdr(0)
        }
    }
    impl core::fmt::Debug for Respcmdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Respcmdr").field("respcmd", &self.respcmd()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Respcmdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Respcmdr {{ respcmd: {=u8:?} }}", self.respcmd())
        }
    }
    #[doc = "SDMMC status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Star(pub u32);
    impl Star {
        #[doc = "Command response received (CRC check failed)."]
        #[must_use]
        #[inline(always)]
        pub const fn ccrcfail(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Command response received (CRC check failed)."]
        #[inline(always)]
        pub const fn set_ccrcfail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Data block sent/received (CRC check failed)."]
        #[must_use]
        #[inline(always)]
        pub const fn dcrcfail(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Data block sent/received (CRC check failed)."]
        #[inline(always)]
        pub const fn set_dcrcfail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Command response timeout."]
        #[must_use]
        #[inline(always)]
        pub const fn ctimeout(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Command response timeout."]
        #[inline(always)]
        pub const fn set_ctimeout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Data timeout."]
        #[must_use]
        #[inline(always)]
        pub const fn dtimeout(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Data timeout."]
        #[inline(always)]
        pub const fn set_dtimeout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Transmit FIFO underrun error (masked by hardware when IDMA is enabled)."]
        #[must_use]
        #[inline(always)]
        pub const fn txunderr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit FIFO underrun error (masked by hardware when IDMA is enabled)."]
        #[inline(always)]
        pub const fn set_txunderr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Received FIFO overrun error (masked by hardware when IDMA is enabled)."]
        #[must_use]
        #[inline(always)]
        pub const fn rxoverr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Received FIFO overrun error (masked by hardware when IDMA is enabled)."]
        #[inline(always)]
        pub const fn set_rxoverr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Command response received (CRC check passed, or no CRC)."]
        #[must_use]
        #[inline(always)]
        pub const fn cmdrend(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Command response received (CRC check passed, or no CRC)."]
        #[inline(always)]
        pub const fn set_cmdrend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Command sent (no response required)."]
        #[must_use]
        #[inline(always)]
        pub const fn cmdsent(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Command sent (no response required)."]
        #[inline(always)]
        pub const fn set_cmdsent(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Data transfer ended correctly."]
        #[must_use]
        #[inline(always)]
        pub const fn dataend(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Data transfer ended correctly."]
        #[inline(always)]
        pub const fn set_dataend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Data transfer Hold."]
        #[must_use]
        #[inline(always)]
        pub const fn dhold(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Data transfer Hold."]
        #[inline(always)]
        pub const fn set_dhold(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Data block sent/received."]
        #[must_use]
        #[inline(always)]
        pub const fn dbckend(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Data block sent/received."]
        #[inline(always)]
        pub const fn set_dbckend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Data transfer aborted by CMD12."]
        #[must_use]
        #[inline(always)]
        pub const fn dabort(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Data transfer aborted by CMD12."]
        #[inline(always)]
        pub const fn set_dabort(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Data path state machine active, i.e. not in Idle state."]
        #[must_use]
        #[inline(always)]
        pub const fn dpsmact(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Data path state machine active, i.e. not in Idle state."]
        #[inline(always)]
        pub const fn set_dpsmact(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Command path state machine active, i.e. not in Idle state."]
        #[must_use]
        #[inline(always)]
        pub const fn cpsmact(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Command path state machine active, i.e. not in Idle state."]
        #[inline(always)]
        pub const fn set_cpsmact(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Transmit FIFO half empty."]
        #[must_use]
        #[inline(always)]
        pub const fn txfifohe(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit FIFO half empty."]
        #[inline(always)]
        pub const fn set_txfifohe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Receive FIFO half full."]
        #[must_use]
        #[inline(always)]
        pub const fn rxfifohf(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Receive FIFO half full."]
        #[inline(always)]
        pub const fn set_rxfifohf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Transmit FIFO full."]
        #[must_use]
        #[inline(always)]
        pub const fn txfifof(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit FIFO full."]
        #[inline(always)]
        pub const fn set_txfifof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Receive FIFO full."]
        #[must_use]
        #[inline(always)]
        pub const fn rxfifof(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Receive FIFO full."]
        #[inline(always)]
        pub const fn set_rxfifof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Transmit FIFO empty."]
        #[must_use]
        #[inline(always)]
        pub const fn txfifoe(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit FIFO empty."]
        #[inline(always)]
        pub const fn set_txfifoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Receive FIFO empty."]
        #[must_use]
        #[inline(always)]
        pub const fn rxfifoe(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Receive FIFO empty."]
        #[inline(always)]
        pub const fn set_rxfifoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Inverted value of SDMMC_D0 line (Busy), sampled at the end of a CMD response and a second time 2 SDMMC_CK cycles after the CMD response."]
        #[must_use]
        #[inline(always)]
        pub const fn busyd0(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Inverted value of SDMMC_D0 line (Busy), sampled at the end of a CMD response and a second time 2 SDMMC_CK cycles after the CMD response."]
        #[inline(always)]
        pub const fn set_busyd0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "end of SDMMC_D0 Busy following a CMD response detected."]
        #[must_use]
        #[inline(always)]
        pub const fn busyd0end(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "end of SDMMC_D0 Busy following a CMD response detected."]
        #[inline(always)]
        pub const fn set_busyd0end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SDIO interrupt received."]
        #[must_use]
        #[inline(always)]
        pub const fn sdioit(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SDIO interrupt received."]
        #[inline(always)]
        pub const fn set_sdioit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Boot acknowledgment received (boot acknowledgment check fail)."]
        #[must_use]
        #[inline(always)]
        pub const fn ackfail(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Boot acknowledgment received (boot acknowledgment check fail)."]
        #[inline(always)]
        pub const fn set_ackfail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Boot acknowledgment timeout."]
        #[must_use]
        #[inline(always)]
        pub const fn acktimeout(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Boot acknowledgment timeout."]
        #[inline(always)]
        pub const fn set_acktimeout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Voltage switch critical timing section completion."]
        #[must_use]
        #[inline(always)]
        pub const fn vswend(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Voltage switch critical timing section completion."]
        #[inline(always)]
        pub const fn set_vswend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "SDMMC_CK stopped in Voltage switch procedure."]
        #[must_use]
        #[inline(always)]
        pub const fn ckstop(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC_CK stopped in Voltage switch procedure."]
        #[inline(always)]
        pub const fn set_ckstop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "IDMA transfer error."]
        #[must_use]
        #[inline(always)]
        pub const fn idmate(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "IDMA transfer error."]
        #[inline(always)]
        pub const fn set_idmate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "IDMA buffer transfer complete."]
        #[must_use]
        #[inline(always)]
        pub const fn idmabtc(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "IDMA buffer transfer complete."]
        #[inline(always)]
        pub const fn set_idmabtc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Star {
        #[inline(always)]
        fn default() -> Star {
            Star(0)
        }
    }
    impl core::fmt::Debug for Star {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Star")
                .field("ccrcfail", &self.ccrcfail())
                .field("dcrcfail", &self.dcrcfail())
                .field("ctimeout", &self.ctimeout())
                .field("dtimeout", &self.dtimeout())
                .field("txunderr", &self.txunderr())
                .field("rxoverr", &self.rxoverr())
                .field("cmdrend", &self.cmdrend())
                .field("cmdsent", &self.cmdsent())
                .field("dataend", &self.dataend())
                .field("dhold", &self.dhold())
                .field("dbckend", &self.dbckend())
                .field("dabort", &self.dabort())
                .field("dpsmact", &self.dpsmact())
                .field("cpsmact", &self.cpsmact())
                .field("txfifohe", &self.txfifohe())
                .field("rxfifohf", &self.rxfifohf())
                .field("txfifof", &self.txfifof())
                .field("rxfifof", &self.rxfifof())
                .field("txfifoe", &self.txfifoe())
                .field("rxfifoe", &self.rxfifoe())
                .field("busyd0", &self.busyd0())
                .field("busyd0end", &self.busyd0end())
                .field("sdioit", &self.sdioit())
                .field("ackfail", &self.ackfail())
                .field("acktimeout", &self.acktimeout())
                .field("vswend", &self.vswend())
                .field("ckstop", &self.ckstop())
                .field("idmate", &self.idmate())
                .field("idmabtc", &self.idmabtc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Star {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Star {{ ccrcfail: {=bool:?}, dcrcfail: {=bool:?}, ctimeout: {=bool:?}, dtimeout: {=bool:?}, txunderr: {=bool:?}, rxoverr: {=bool:?}, cmdrend: {=bool:?}, cmdsent: {=bool:?}, dataend: {=bool:?}, dhold: {=bool:?}, dbckend: {=bool:?}, dabort: {=bool:?}, dpsmact: {=bool:?}, cpsmact: {=bool:?}, txfifohe: {=bool:?}, rxfifohf: {=bool:?}, txfifof: {=bool:?}, rxfifof: {=bool:?}, txfifoe: {=bool:?}, rxfifoe: {=bool:?}, busyd0: {=bool:?}, busyd0end: {=bool:?}, sdioit: {=bool:?}, ackfail: {=bool:?}, acktimeout: {=bool:?}, vswend: {=bool:?}, ckstop: {=bool:?}, idmate: {=bool:?}, idmabtc: {=bool:?} }}",
                self.ccrcfail(),
                self.dcrcfail(),
                self.ctimeout(),
                self.dtimeout(),
                self.txunderr(),
                self.rxoverr(),
                self.cmdrend(),
                self.cmdsent(),
                self.dataend(),
                self.dhold(),
                self.dbckend(),
                self.dabort(),
                self.dpsmact(),
                self.cpsmact(),
                self.txfifohe(),
                self.rxfifohf(),
                self.txfifof(),
                self.rxfifof(),
                self.txfifoe(),
                self.rxfifoe(),
                self.busyd0(),
                self.busyd0end(),
                self.sdioit(),
                self.ackfail(),
                self.acktimeout(),
                self.vswend(),
                self.ckstop(),
                self.idmate(),
                self.idmabtc()
            )
        }
    }
}
