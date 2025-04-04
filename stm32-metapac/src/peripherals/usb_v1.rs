#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Universal serial bus full-speed device interface"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb {
    ptr: *mut u8,
}
unsafe impl Send for Usb {}
unsafe impl Sync for Usb {}
impl Usb {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "endpoint register"]
    #[inline(always)]
    pub const fn epr(self, n: usize) -> crate::common::Reg<regs::Epr, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "control register"]
    #[inline(always)]
    pub const fn cntr(self) -> crate::common::Reg<regs::Cntr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "interrupt status register"]
    #[inline(always)]
    pub const fn istr(self) -> crate::common::Reg<regs::Istr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "frame number register"]
    #[inline(always)]
    pub const fn fnr(self) -> crate::common::Reg<regs::Fnr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "device address"]
    #[inline(always)]
    pub const fn daddr(self) -> crate::common::Reg<regs::Daddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "Buffer table address"]
    #[inline(always)]
    pub const fn btable(self) -> crate::common::Reg<regs::Btable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
}
pub mod regs {
    #[doc = "Buffer table address"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Btable(pub u32);
    impl Btable {
        #[doc = "BTABLE"]
        #[inline(always)]
        pub const fn btable(&self) -> u16 {
            let val = (self.0 >> 3usize) & 0x1fff;
            val as u16
        }
        #[doc = "BTABLE"]
        #[inline(always)]
        pub fn set_btable(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 3usize)) | (((val as u32) & 0x1fff) << 3usize);
        }
    }
    impl Default for Btable {
        #[inline(always)]
        fn default() -> Btable {
            Btable(0)
        }
    }
    impl core::fmt::Debug for Btable {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Btable").field("btable", &self.btable()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Btable {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Btable {{ btable: {=u16:?} }}", self.btable())
        }
    }
    #[doc = "control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cntr(pub u32);
    impl Cntr {
        #[doc = "Force a reset of the USB peripheral, exactly like a RESET signaling on the USB"]
        #[inline(always)]
        pub const fn fres(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Force a reset of the USB peripheral, exactly like a RESET signaling on the USB"]
        #[inline(always)]
        pub fn set_fres(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Enter power down mode"]
        #[inline(always)]
        pub const fn pdwn(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Enter power down mode"]
        #[inline(always)]
        pub fn set_pdwn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Enter low-power mode"]
        #[inline(always)]
        pub const fn lpmode(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Enter low-power mode"]
        #[inline(always)]
        pub fn set_lpmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Enter suspend mode. Clocks and static power dissipation in the analog transceiver are left unaffected"]
        #[inline(always)]
        pub const fn fsusp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Enter suspend mode. Clocks and static power dissipation in the analog transceiver are left unaffected"]
        #[inline(always)]
        pub fn set_fsusp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Resume request"]
        #[inline(always)]
        pub const fn resume(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Resume request"]
        #[inline(always)]
        pub fn set_resume(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ESOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
        #[inline(always)]
        pub const fn esofm(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "ESOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
        #[inline(always)]
        pub fn set_esofm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "SOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
        #[inline(always)]
        pub const fn sofm(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SOF Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
        #[inline(always)]
        pub fn set_sofm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "RESET Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
        #[inline(always)]
        pub const fn resetm(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "RESET Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
        #[inline(always)]
        pub fn set_resetm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "SUSP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
        #[inline(always)]
        pub const fn suspm(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "SUSP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
        #[inline(always)]
        pub fn set_suspm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "WKUP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
        #[inline(always)]
        pub const fn wkupm(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "WKUP Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
        #[inline(always)]
        pub fn set_wkupm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "ERR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
        #[inline(always)]
        pub const fn errm(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "ERR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
        #[inline(always)]
        pub fn set_errm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "PMAOVR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
        #[inline(always)]
        pub const fn pmaovrm(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "PMAOVR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
        #[inline(always)]
        pub fn set_pmaovrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "CTR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
        #[inline(always)]
        pub const fn ctrm(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "CTR Interrupt enabled, an interrupt request is generated when the corresponding bit in the USB_ISTR register is set"]
        #[inline(always)]
        pub fn set_ctrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Cntr {
        #[inline(always)]
        fn default() -> Cntr {
            Cntr(0)
        }
    }
    impl core::fmt::Debug for Cntr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cntr")
                .field("fres", &self.fres())
                .field("pdwn", &self.pdwn())
                .field("lpmode", &self.lpmode())
                .field("fsusp", &self.fsusp())
                .field("resume", &self.resume())
                .field("esofm", &self.esofm())
                .field("sofm", &self.sofm())
                .field("resetm", &self.resetm())
                .field("suspm", &self.suspm())
                .field("wkupm", &self.wkupm())
                .field("errm", &self.errm())
                .field("pmaovrm", &self.pmaovrm())
                .field("ctrm", &self.ctrm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cntr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cntr {{ fres: {=bool:?}, pdwn: {=bool:?}, lpmode: {=bool:?}, fsusp: {=bool:?}, resume: {=bool:?}, esofm: {=bool:?}, sofm: {=bool:?}, resetm: {=bool:?}, suspm: {=bool:?}, wkupm: {=bool:?}, errm: {=bool:?}, pmaovrm: {=bool:?}, ctrm: {=bool:?} }}" , self . fres () , self . pdwn () , self . lpmode () , self . fsusp () , self . resume () , self . esofm () , self . sofm () , self . resetm () , self . suspm () , self . wkupm () , self . errm () , self . pmaovrm () , self . ctrm ())
        }
    }
    #[doc = "device address"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Daddr(pub u32);
    impl Daddr {
        #[doc = "device address"]
        #[inline(always)]
        pub const fn add(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "device address"]
        #[inline(always)]
        pub fn set_add(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "USB device enabled"]
        #[inline(always)]
        pub const fn ef(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "USB device enabled"]
        #[inline(always)]
        pub fn set_ef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Daddr {
        #[inline(always)]
        fn default() -> Daddr {
            Daddr(0)
        }
    }
    impl core::fmt::Debug for Daddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Daddr")
                .field("add", &self.add())
                .field("ef", &self.ef())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Daddr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Daddr {{ add: {=u8:?}, ef: {=bool:?} }}", self.add(), self.ef())
        }
    }
    #[doc = "endpoint register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Epr(pub u32);
    impl Epr {
        #[doc = "EA"]
        #[inline(always)]
        pub const fn ea(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "EA"]
        #[inline(always)]
        pub fn set_ea(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "STAT_TX"]
        #[inline(always)]
        pub const fn stat_tx(&self) -> super::vals::Stat {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Stat::from_bits(val as u8)
        }
        #[doc = "STAT_TX"]
        #[inline(always)]
        pub fn set_stat_tx(&mut self, val: super::vals::Stat) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "DTOG_TX"]
        #[inline(always)]
        pub const fn dtog_tx(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "DTOG_TX"]
        #[inline(always)]
        pub fn set_dtog_tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "CTR_TX"]
        #[inline(always)]
        pub const fn ctr_tx(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CTR_TX"]
        #[inline(always)]
        pub fn set_ctr_tx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "EP_KIND"]
        #[inline(always)]
        pub const fn ep_kind(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "EP_KIND"]
        #[inline(always)]
        pub fn set_ep_kind(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "EPTYPE"]
        #[inline(always)]
        pub const fn ep_type(&self) -> super::vals::EpType {
            let val = (self.0 >> 9usize) & 0x03;
            super::vals::EpType::from_bits(val as u8)
        }
        #[doc = "EPTYPE"]
        #[inline(always)]
        pub fn set_ep_type(&mut self, val: super::vals::EpType) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
        }
        #[doc = "SETUP"]
        #[inline(always)]
        pub const fn setup(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "SETUP"]
        #[inline(always)]
        pub fn set_setup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "STAT_RX"]
        #[inline(always)]
        pub const fn stat_rx(&self) -> super::vals::Stat {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Stat::from_bits(val as u8)
        }
        #[doc = "STAT_RX"]
        #[inline(always)]
        pub fn set_stat_rx(&mut self, val: super::vals::Stat) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "DTOG_RX"]
        #[inline(always)]
        pub const fn dtog_rx(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "DTOG_RX"]
        #[inline(always)]
        pub fn set_dtog_rx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "CTR_RX"]
        #[inline(always)]
        pub const fn ctr_rx(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "CTR_RX"]
        #[inline(always)]
        pub fn set_ctr_rx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Epr {
        #[inline(always)]
        fn default() -> Epr {
            Epr(0)
        }
    }
    impl core::fmt::Debug for Epr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Epr")
                .field("ea", &self.ea())
                .field("stat_tx", &self.stat_tx())
                .field("dtog_tx", &self.dtog_tx())
                .field("ctr_tx", &self.ctr_tx())
                .field("ep_kind", &self.ep_kind())
                .field("ep_type", &self.ep_type())
                .field("setup", &self.setup())
                .field("stat_rx", &self.stat_rx())
                .field("dtog_rx", &self.dtog_rx())
                .field("ctr_rx", &self.ctr_rx())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Epr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Epr {{ ea: {=u8:?}, stat_tx: {:?}, dtog_tx: {=bool:?}, ctr_tx: {=bool:?}, ep_kind: {=bool:?}, ep_type: {:?}, setup: {=bool:?}, stat_rx: {:?}, dtog_rx: {=bool:?}, ctr_rx: {=bool:?} }}" , self . ea () , self . stat_tx () , self . dtog_tx () , self . ctr_tx () , self . ep_kind () , self . ep_type () , self . setup () , self . stat_rx () , self . dtog_rx () , self . ctr_rx ())
        }
    }
    #[doc = "frame number register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fnr(pub u32);
    impl Fnr {
        #[doc = "FN"]
        #[inline(always)]
        pub const fn fn_(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "FN"]
        #[inline(always)]
        pub fn set_fn_(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "LSOF"]
        #[inline(always)]
        pub const fn lsof(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x03;
            val as u8
        }
        #[doc = "LSOF"]
        #[inline(always)]
        pub fn set_lsof(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 11usize)) | (((val as u32) & 0x03) << 11usize);
        }
        #[doc = "the frame timer remains in this state until an USB reset or USB suspend event occurs"]
        #[inline(always)]
        pub const fn lck(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "the frame timer remains in this state until an USB reset or USB suspend event occurs"]
        #[inline(always)]
        pub fn set_lck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "received data minus upstream port data line"]
        #[inline(always)]
        pub const fn rxdm(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "received data minus upstream port data line"]
        #[inline(always)]
        pub fn set_rxdm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "received data plus upstream port data line"]
        #[inline(always)]
        pub const fn rxdp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "received data plus upstream port data line"]
        #[inline(always)]
        pub fn set_rxdp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Fnr {
        #[inline(always)]
        fn default() -> Fnr {
            Fnr(0)
        }
    }
    impl core::fmt::Debug for Fnr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fnr")
                .field("fn_", &self.fn_())
                .field("lsof", &self.lsof())
                .field("lck", &self.lck())
                .field("rxdm", &self.rxdm())
                .field("rxdp", &self.rxdp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fnr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Fnr {{ fn_: {=u16:?}, lsof: {=u8:?}, lck: {=bool:?}, rxdm: {=bool:?}, rxdp: {=bool:?} }}",
                self.fn_(),
                self.lsof(),
                self.lck(),
                self.rxdm(),
                self.rxdp()
            )
        }
    }
    #[doc = "interrupt status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Istr(pub u32);
    impl Istr {
        #[doc = "EP_ID"]
        #[inline(always)]
        pub const fn ep_id(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "EP_ID"]
        #[inline(always)]
        pub fn set_ep_id(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "DIR"]
        #[inline(always)]
        pub const fn dir(&self) -> super::vals::Dir {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Dir::from_bits(val as u8)
        }
        #[doc = "DIR"]
        #[inline(always)]
        pub fn set_dir(&mut self, val: super::vals::Dir) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "an SOF packet is expected but not received"]
        #[inline(always)]
        pub const fn esof(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "an SOF packet is expected but not received"]
        #[inline(always)]
        pub fn set_esof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "beginning of a new USB frame and it is set when a SOF packet arrives through the USB bus"]
        #[inline(always)]
        pub const fn sof(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "beginning of a new USB frame and it is set when a SOF packet arrives through the USB bus"]
        #[inline(always)]
        pub fn set_sof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "peripheral detects an active USB RESET signal at its inputs"]
        #[inline(always)]
        pub const fn reset(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "peripheral detects an active USB RESET signal at its inputs"]
        #[inline(always)]
        pub fn set_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "no traffic has been received for 3 ms, indicating a suspend mode request from the USB bus"]
        #[inline(always)]
        pub const fn susp(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "no traffic has been received for 3 ms, indicating a suspend mode request from the USB bus"]
        #[inline(always)]
        pub fn set_susp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "activity is detected that wakes up the USB peripheral"]
        #[inline(always)]
        pub const fn wkup(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "activity is detected that wakes up the USB peripheral"]
        #[inline(always)]
        pub fn set_wkup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "One of No ANSwer, Cyclic Redundancy Check, Bit Stuffing or Framing format Violation error occurred"]
        #[inline(always)]
        pub const fn err(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "One of No ANSwer, Cyclic Redundancy Check, Bit Stuffing or Framing format Violation error occurred"]
        #[inline(always)]
        pub fn set_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "microcontroller has not been able to respond in time to an USB memory request"]
        #[inline(always)]
        pub const fn pmaovr(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "microcontroller has not been able to respond in time to an USB memory request"]
        #[inline(always)]
        pub fn set_pmaovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "endpoint has successfully completed a transaction"]
        #[inline(always)]
        pub const fn ctr(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint has successfully completed a transaction"]
        #[inline(always)]
        pub fn set_ctr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Istr {
        #[inline(always)]
        fn default() -> Istr {
            Istr(0)
        }
    }
    impl core::fmt::Debug for Istr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Istr")
                .field("ep_id", &self.ep_id())
                .field("dir", &self.dir())
                .field("esof", &self.esof())
                .field("sof", &self.sof())
                .field("reset", &self.reset())
                .field("susp", &self.susp())
                .field("wkup", &self.wkup())
                .field("err", &self.err())
                .field("pmaovr", &self.pmaovr())
                .field("ctr", &self.ctr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Istr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Istr {{ ep_id: {=u8:?}, dir: {:?}, esof: {=bool:?}, sof: {=bool:?}, reset: {=bool:?}, susp: {=bool:?}, wkup: {=bool:?}, err: {=bool:?}, pmaovr: {=bool:?}, ctr: {=bool:?} }}" , self . ep_id () , self . dir () , self . esof () , self . sof () , self . reset () , self . susp () , self . wkup () , self . err () , self . pmaovr () , self . ctr ())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dir {
        #[doc = "data transmitted by the USB peripheral to the host PC"]
        TO = 0x0,
        #[doc = "data received by the USB peripheral from the host PC"]
        FROM = 0x01,
    }
    impl Dir {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dir {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dir {
        #[inline(always)]
        fn from(val: u8) -> Dir {
            Dir::from_bits(val)
        }
    }
    impl From<Dir> for u8 {
        #[inline(always)]
        fn from(val: Dir) -> u8 {
            Dir::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum EpType {
        #[doc = "Bulk endpoint"]
        BULK = 0x0,
        #[doc = "Control endpoint"]
        CONTROL = 0x01,
        #[doc = "Iso endpoint"]
        ISO = 0x02,
        #[doc = "Interrupt endpoint"]
        INTERRUPT = 0x03,
    }
    impl EpType {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> EpType {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for EpType {
        #[inline(always)]
        fn from(val: u8) -> EpType {
            EpType::from_bits(val)
        }
    }
    impl From<EpType> for u8 {
        #[inline(always)]
        fn from(val: EpType) -> u8 {
            EpType::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Stat {
        #[doc = "all requests addressed to this endpoint are ignored"]
        DISABLED = 0x0,
        #[doc = "the endpoint is stalled and all requests result in a STALL handshake"]
        STALL = 0x01,
        #[doc = "the endpoint is naked and all requests result in a NAK handshake"]
        NAK = 0x02,
        #[doc = "this endpoint is enabled, requests are ACKed"]
        VALID = 0x03,
    }
    impl Stat {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Stat {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Stat {
        #[inline(always)]
        fn from(val: u8) -> Stat {
            Stat::from_bits(val)
        }
    }
    impl From<Stat> for u8 {
        #[inline(always)]
        fn from(val: Stat) -> u8 {
            Stat::to_bits(val)
        }
    }
}
