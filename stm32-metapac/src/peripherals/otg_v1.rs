#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "USB OTG core by Synopsys (more docs at <https://www.intel.com/content/www/us/en/programmable/hps/agilex5/index_frames.html>)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Otg {
    ptr: *mut u8,
}
unsafe impl Send for Otg {}
unsafe impl Sync for Otg {}
impl Otg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control and status register"]
    #[inline(always)]
    pub const fn gotgctl(self) -> crate::common::Reg<regs::Gotgctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Interrupt register"]
    #[inline(always)]
    pub const fn gotgint(self) -> crate::common::Reg<regs::Gotgint, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "AHB configuration register"]
    #[inline(always)]
    pub const fn gahbcfg(self) -> crate::common::Reg<regs::Gahbcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "USB configuration register"]
    #[inline(always)]
    pub const fn gusbcfg(self) -> crate::common::Reg<regs::Gusbcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Reset register"]
    #[inline(always)]
    pub const fn grstctl(self) -> crate::common::Reg<regs::Grstctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Core interrupt register"]
    #[inline(always)]
    pub const fn gintsts(self) -> crate::common::Reg<regs::Gintsts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Interrupt mask register"]
    #[inline(always)]
    pub const fn gintmsk(self) -> crate::common::Reg<regs::Gintmsk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Receive status debug read register"]
    #[inline(always)]
    pub const fn grxstsr(self) -> crate::common::Reg<regs::Grxsts, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Status read and pop register"]
    #[inline(always)]
    pub const fn grxstsp(self) -> crate::common::Reg<regs::Grxsts, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Receive FIFO size register"]
    #[inline(always)]
    pub const fn grxfsiz(self) -> crate::common::Reg<regs::Grxfsiz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Endpoint 0 transmit FIFO size register (device mode)"]
    #[inline(always)]
    pub const fn dieptxf0(self) -> crate::common::Reg<regs::Fsiz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Non-periodic transmit FIFO size register (host mode)"]
    #[inline(always)]
    pub const fn hnptxfsiz(self) -> crate::common::Reg<regs::Fsiz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Non-periodic transmit FIFO/queue status register (host mode)"]
    #[inline(always)]
    pub const fn hnptxsts(self) -> crate::common::Reg<regs::Hnptxsts, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "OTG I2C access register"]
    #[inline(always)]
    pub const fn gi2cctl(self) -> crate::common::Reg<regs::Gi2cctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "General core configuration register, for core_id 0x0000_1xxx"]
    #[inline(always)]
    pub const fn gccfg_v1(self) -> crate::common::Reg<regs::GccfgV1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "General core configuration register, for core_id 0x0000_\\[23\\]xxx"]
    #[inline(always)]
    pub const fn gccfg_v2(self) -> crate::common::Reg<regs::GccfgV2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "General core configuration register, for core_id 0x0000_5xxx"]
    #[inline(always)]
    pub const fn gccfg_v3(self) -> crate::common::Reg<regs::GccfgV3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "Core ID register"]
    #[inline(always)]
    pub const fn cid(self) -> crate::common::Reg<regs::Cid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "Synopsis ID Register"]
    #[inline(always)]
    pub const fn snpsid(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "User HW Config 1 Register"]
    #[inline(always)]
    pub const fn hwcfg1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "User HW Config 2 Register"]
    #[inline(always)]
    pub const fn hwcfg2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "User HW Config 3 Register"]
    #[inline(always)]
    pub const fn hwcfg3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "User HW Config 4 Register"]
    #[inline(always)]
    pub const fn hwcfg4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "OTG core LPM configuration register"]
    #[inline(always)]
    pub const fn glpmcfg(self) -> crate::common::Reg<regs::Glpmcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "Global PowerDn Register"]
    #[inline(always)]
    pub const fn gpwrdn(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "Global DFIFO SW Config Register"]
    #[inline(always)]
    pub const fn gdfifocfg(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "ADP (Attach Detection Protocol) Control Register"]
    #[inline(always)]
    pub const fn adpctl(self) -> crate::common::Reg<regs::Adpctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "Host periodic transmit FIFO size register"]
    #[inline(always)]
    pub const fn hptxfsiz(self) -> crate::common::Reg<regs::Fsiz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Device IN endpoint transmit FIFO size register"]
    #[inline(always)]
    pub const fn dieptxf(self, n: usize) -> crate::common::Reg<regs::Fsiz, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize + n * 4usize) as _) }
    }
    #[doc = "Host configuration register"]
    #[inline(always)]
    pub const fn hcfg(self) -> crate::common::Reg<regs::Hcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[doc = "Host frame interval register"]
    #[inline(always)]
    pub const fn hfir(self) -> crate::common::Reg<regs::Hfir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0404usize) as _) }
    }
    #[doc = "Host frame number/frame time remaining register"]
    #[inline(always)]
    pub const fn hfnum(self) -> crate::common::Reg<regs::Hfnum, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0408usize) as _) }
    }
    #[doc = "Periodic transmit FIFO/queue status register"]
    #[inline(always)]
    pub const fn hptxsts(self) -> crate::common::Reg<regs::Hptxsts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0410usize) as _) }
    }
    #[doc = "Host all channels interrupt register"]
    #[inline(always)]
    pub const fn haint(self) -> crate::common::Reg<regs::Haint, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0414usize) as _) }
    }
    #[doc = "Host all channels interrupt mask register"]
    #[inline(always)]
    pub const fn haintmsk(self) -> crate::common::Reg<regs::Haintmsk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0418usize) as _) }
    }
    #[doc = "Host Frame Scheduling List Register"]
    #[inline(always)]
    pub const fn hflbaddr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x041cusize) as _) }
    }
    #[doc = "Host port control and status register"]
    #[inline(always)]
    pub const fn hprt(self) -> crate::common::Reg<regs::Hprt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0440usize) as _) }
    }
    #[doc = "Host channel characteristics register"]
    #[inline(always)]
    pub const fn hcchar(self, n: usize) -> crate::common::Reg<regs::Hcchar, crate::common::RW> {
        assert!(n < 12usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize + n * 32usize) as _) }
    }
    #[doc = "Host channel split control register"]
    #[inline(always)]
    pub const fn hcsplt(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 12usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize + n * 32usize) as _) }
    }
    #[doc = "Host channel interrupt register"]
    #[inline(always)]
    pub const fn hcint(self, n: usize) -> crate::common::Reg<regs::Hcint, crate::common::RW> {
        assert!(n < 12usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize + n * 32usize) as _) }
    }
    #[doc = "Host channel mask register"]
    #[inline(always)]
    pub const fn hcintmsk(self, n: usize) -> crate::common::Reg<regs::Hcintmsk, crate::common::RW> {
        assert!(n < 12usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize + n * 32usize) as _) }
    }
    #[doc = "Host channel transfer size register"]
    #[inline(always)]
    pub const fn hctsiz(self, n: usize) -> crate::common::Reg<regs::Hctsiz, crate::common::RW> {
        assert!(n < 12usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize + n * 32usize) as _) }
    }
    #[doc = "Host channel DMA address register (config for scatter/gather)"]
    #[inline(always)]
    pub const fn hcdma(self, n: usize) -> crate::common::Reg<regs::Hcdma, crate::common::RW> {
        assert!(n < 12usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize + n * 32usize) as _) }
    }
    #[doc = "Host channel DMA address register (address for current transfer; debug)"]
    #[inline(always)]
    pub const fn hcdmab(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 12usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize + n * 32usize) as _) }
    }
    #[doc = "Device configuration register"]
    #[inline(always)]
    pub const fn dcfg(self) -> crate::common::Reg<regs::Dcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0800usize) as _) }
    }
    #[doc = "Device control register"]
    #[inline(always)]
    pub const fn dctl(self) -> crate::common::Reg<regs::Dctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0804usize) as _) }
    }
    #[doc = "Device status register"]
    #[inline(always)]
    pub const fn dsts(self) -> crate::common::Reg<regs::Dsts, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0808usize) as _) }
    }
    #[doc = "Device IN endpoint common interrupt mask register"]
    #[inline(always)]
    pub const fn diepmsk(self) -> crate::common::Reg<regs::Diepmsk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0810usize) as _) }
    }
    #[doc = "Device OUT endpoint common interrupt mask register"]
    #[inline(always)]
    pub const fn doepmsk(self) -> crate::common::Reg<regs::Doepmsk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0814usize) as _) }
    }
    #[doc = "Device all endpoints interrupt register"]
    #[inline(always)]
    pub const fn daint(self) -> crate::common::Reg<regs::Daint, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0818usize) as _) }
    }
    #[doc = "All endpoints interrupt mask register"]
    #[inline(always)]
    pub const fn daintmsk(self) -> crate::common::Reg<regs::Daintmsk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x081cusize) as _) }
    }
    #[doc = "Device VBUS discharge time register"]
    #[inline(always)]
    pub const fn dvbusdis(self) -> crate::common::Reg<regs::Dvbusdis, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0828usize) as _) }
    }
    #[doc = "Device VBUS pulsing time register"]
    #[inline(always)]
    pub const fn dvbuspulse(self) -> crate::common::Reg<regs::Dvbuspulse, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x082cusize) as _) }
    }
    #[doc = "Device IN endpoint FIFO empty interrupt mask register"]
    #[inline(always)]
    pub const fn diepempmsk(self) -> crate::common::Reg<regs::Diepempmsk, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0834usize) as _) }
    }
    #[doc = "Device IN endpoint control register"]
    #[inline(always)]
    pub const fn diepctl(self, n: usize) -> crate::common::Reg<regs::Diepctl, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0900usize + n * 32usize) as _) }
    }
    #[doc = "Device IN endpoint interrupt register"]
    #[inline(always)]
    pub const fn diepint(self, n: usize) -> crate::common::Reg<regs::Diepint, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0908usize + n * 32usize) as _) }
    }
    #[doc = "Device IN endpoint transfer size register"]
    #[inline(always)]
    pub const fn dieptsiz(self, n: usize) -> crate::common::Reg<regs::Dieptsiz, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0910usize + n * 32usize) as _) }
    }
    #[doc = "Device IN endpoint transmit FIFO status register"]
    #[inline(always)]
    pub const fn dtxfsts(self, n: usize) -> crate::common::Reg<regs::Dtxfsts, crate::common::R> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0918usize + n * 32usize) as _) }
    }
    #[doc = "Device OUT endpoint control register"]
    #[inline(always)]
    pub const fn doepctl(self, n: usize) -> crate::common::Reg<regs::Doepctl, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b00usize + n * 32usize) as _) }
    }
    #[doc = "Device OUT endpoint interrupt register"]
    #[inline(always)]
    pub const fn doepint(self, n: usize) -> crate::common::Reg<regs::Doepint, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b08usize + n * 32usize) as _) }
    }
    #[doc = "Device OUT endpoint transfer size register"]
    #[inline(always)]
    pub const fn doeptsiz(self, n: usize) -> crate::common::Reg<regs::Doeptsiz, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b10usize + n * 32usize) as _) }
    }
    #[doc = "Device OUT/IN endpoint DMA address register"]
    #[inline(always)]
    pub const fn doepdma(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b14usize + n * 32usize) as _) }
    }
    #[doc = "Power and clock gating control register"]
    #[inline(always)]
    pub const fn pcgcctl(self) -> crate::common::Reg<regs::Pcgcctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0e00usize) as _) }
    }
    #[doc = "Device endpoint / host channel FIFO register"]
    #[inline(always)]
    pub const fn fifo(self, n: usize) -> crate::common::Reg<regs::Fifo, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1000usize + n * 4096usize) as _) }
    }
}
pub mod regs {
    #[doc = "ADP (Attach Detection Protocol) Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Adpctl(pub u32);
    impl Adpctl {
        #[doc = "Probe Discharge time (times for TADP_DSCHG)"]
        #[inline(always)]
        pub const fn prb_dschg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Probe Discharge time (times for TADP_DSCHG)"]
        #[inline(always)]
        pub fn set_prb_dschg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Probe Delta (resolution for RTIM)"]
        #[inline(always)]
        pub const fn prb_delta(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Probe Delta (resolution for RTIM)"]
        #[inline(always)]
        pub fn set_prb_delta(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Probe Period (TADP_PRD)"]
        #[inline(always)]
        pub const fn prb_per(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Probe Period (TADP_PRD)"]
        #[inline(always)]
        pub fn set_prb_per(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Probe Period (TADP_PRD)"]
        #[inline(always)]
        pub const fn rtim(&self) -> u16 {
            let val = (self.0 >> 6usize) & 0x07ff;
            val as u16
        }
        #[doc = "Probe Period (TADP_PRD)"]
        #[inline(always)]
        pub fn set_rtim(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 6usize)) | (((val as u32) & 0x07ff) << 6usize);
        }
        #[doc = "Enable Probe"]
        #[inline(always)]
        pub const fn enaprb(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Probe"]
        #[inline(always)]
        pub fn set_enaprb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Enable Sense"]
        #[inline(always)]
        pub const fn enasns(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Sense"]
        #[inline(always)]
        pub fn set_enasns(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "ADP Reset"]
        #[inline(always)]
        pub const fn adpres(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "ADP Reset"]
        #[inline(always)]
        pub fn set_adpres(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "ADP Enable"]
        #[inline(always)]
        pub const fn adpen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "ADP Enable"]
        #[inline(always)]
        pub fn set_adpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "ADP Probe Interrupt Enable"]
        #[inline(always)]
        pub const fn adp_prb_int(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "ADP Probe Interrupt Enable"]
        #[inline(always)]
        pub fn set_adp_prb_int(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ADP Sense Interrupt Enable"]
        #[inline(always)]
        pub const fn adp_sns_int(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "ADP Sense Interrupt Enable"]
        #[inline(always)]
        pub fn set_adp_sns_int(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "ADP Timeout Interrupt Enable"]
        #[inline(always)]
        pub const fn adp_tmout_int(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "ADP Timeout Interrupt Enable"]
        #[inline(always)]
        pub fn set_adp_tmout_int(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "ADP Probe Interrupt Mask"]
        #[inline(always)]
        pub const fn adp_prb_msk(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ADP Probe Interrupt Mask"]
        #[inline(always)]
        pub fn set_adp_prb_msk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "ADP Timeout Interrupt Mask"]
        #[inline(always)]
        pub const fn adp_tmout_msk(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "ADP Timeout Interrupt Mask"]
        #[inline(always)]
        pub fn set_adp_tmout_msk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Access Request"]
        #[inline(always)]
        pub const fn ar(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Access Request"]
        #[inline(always)]
        pub fn set_ar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Adpctl {
        #[inline(always)]
        fn default() -> Adpctl {
            Adpctl(0)
        }
    }
    impl core::fmt::Debug for Adpctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Adpctl")
                .field("prb_dschg", &self.prb_dschg())
                .field("prb_delta", &self.prb_delta())
                .field("prb_per", &self.prb_per())
                .field("rtim", &self.rtim())
                .field("enaprb", &self.enaprb())
                .field("enasns", &self.enasns())
                .field("adpres", &self.adpres())
                .field("adpen", &self.adpen())
                .field("adp_prb_int", &self.adp_prb_int())
                .field("adp_sns_int", &self.adp_sns_int())
                .field("adp_tmout_int", &self.adp_tmout_int())
                .field("adp_prb_msk", &self.adp_prb_msk())
                .field("adp_tmout_msk", &self.adp_tmout_msk())
                .field("ar", &self.ar())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Adpctl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Adpctl {{ prb_dschg: {=u8:?}, prb_delta: {=u8:?}, prb_per: {=u8:?}, rtim: {=u16:?}, enaprb: {=bool:?}, enasns: {=bool:?}, adpres: {=bool:?}, adpen: {=bool:?}, adp_prb_int: {=bool:?}, adp_sns_int: {=bool:?}, adp_tmout_int: {=bool:?}, adp_prb_msk: {=bool:?}, adp_tmout_msk: {=bool:?}, ar: {=bool:?} }}" , self . prb_dschg () , self . prb_delta () , self . prb_per () , self . rtim () , self . enaprb () , self . enasns () , self . adpres () , self . adpen () , self . adp_prb_int () , self . adp_sns_int () , self . adp_tmout_int () , self . adp_prb_msk () , self . adp_tmout_msk () , self . ar ())
        }
    }
    #[doc = "Core ID register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cid(pub u32);
    impl Cid {
        #[doc = "Product ID field"]
        #[inline(always)]
        pub const fn product_id(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Product ID field"]
        #[inline(always)]
        pub fn set_product_id(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cid {
        #[inline(always)]
        fn default() -> Cid {
            Cid(0)
        }
    }
    impl core::fmt::Debug for Cid {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cid").field("product_id", &self.product_id()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cid {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cid {{ product_id: {=u32:?} }}", self.product_id())
        }
    }
    #[doc = "Device all endpoints interrupt register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Daint(pub u32);
    impl Daint {
        #[doc = "IN endpoint interrupt bits"]
        #[inline(always)]
        pub const fn iepint(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "IN endpoint interrupt bits"]
        #[inline(always)]
        pub fn set_iepint(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "OUT endpoint interrupt bits"]
        #[inline(always)]
        pub const fn oepint(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "OUT endpoint interrupt bits"]
        #[inline(always)]
        pub fn set_oepint(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Daint {
        #[inline(always)]
        fn default() -> Daint {
            Daint(0)
        }
    }
    impl core::fmt::Debug for Daint {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Daint")
                .field("iepint", &self.iepint())
                .field("oepint", &self.oepint())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Daint {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Daint {{ iepint: {=u16:?}, oepint: {=u16:?} }}",
                self.iepint(),
                self.oepint()
            )
        }
    }
    #[doc = "All endpoints interrupt mask register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Daintmsk(pub u32);
    impl Daintmsk {
        #[doc = "IN EP interrupt mask bits"]
        #[inline(always)]
        pub const fn iepm(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "IN EP interrupt mask bits"]
        #[inline(always)]
        pub fn set_iepm(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "OUT EP interrupt mask bits"]
        #[inline(always)]
        pub const fn oepm(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "OUT EP interrupt mask bits"]
        #[inline(always)]
        pub fn set_oepm(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Daintmsk {
        #[inline(always)]
        fn default() -> Daintmsk {
            Daintmsk(0)
        }
    }
    impl core::fmt::Debug for Daintmsk {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Daintmsk")
                .field("iepm", &self.iepm())
                .field("oepm", &self.oepm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Daintmsk {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Daintmsk {{ iepm: {=u16:?}, oepm: {=u16:?} }}",
                self.iepm(),
                self.oepm()
            )
        }
    }
    #[doc = "Device configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dcfg(pub u32);
    impl Dcfg {
        #[doc = "Device speed"]
        #[inline(always)]
        pub const fn dspd(&self) -> super::vals::Dspd {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Dspd::from_bits(val as u8)
        }
        #[doc = "Device speed"]
        #[inline(always)]
        pub fn set_dspd(&mut self, val: super::vals::Dspd) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Non-zero-length status OUT handshake"]
        #[inline(always)]
        pub const fn nzlsohsk(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Non-zero-length status OUT handshake"]
        #[inline(always)]
        pub fn set_nzlsohsk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Device address"]
        #[inline(always)]
        pub const fn dad(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x7f;
            val as u8
        }
        #[doc = "Device address"]
        #[inline(always)]
        pub fn set_dad(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 4usize)) | (((val as u32) & 0x7f) << 4usize);
        }
        #[doc = "Periodic frame interval"]
        #[inline(always)]
        pub const fn pfivl(&self) -> super::vals::Pfivl {
            let val = (self.0 >> 11usize) & 0x03;
            super::vals::Pfivl::from_bits(val as u8)
        }
        #[doc = "Periodic frame interval"]
        #[inline(always)]
        pub fn set_pfivl(&mut self, val: super::vals::Pfivl) {
            self.0 = (self.0 & !(0x03 << 11usize)) | (((val.to_bits() as u32) & 0x03) << 11usize);
        }
        #[doc = "Transceiver delay"]
        #[inline(always)]
        pub const fn xcvrdly(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Transceiver delay"]
        #[inline(always)]
        pub fn set_xcvrdly(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Dcfg {
        #[inline(always)]
        fn default() -> Dcfg {
            Dcfg(0)
        }
    }
    impl core::fmt::Debug for Dcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dcfg")
                .field("dspd", &self.dspd())
                .field("nzlsohsk", &self.nzlsohsk())
                .field("dad", &self.dad())
                .field("pfivl", &self.pfivl())
                .field("xcvrdly", &self.xcvrdly())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dcfg {{ dspd: {:?}, nzlsohsk: {=bool:?}, dad: {=u8:?}, pfivl: {:?}, xcvrdly: {=bool:?} }}",
                self.dspd(),
                self.nzlsohsk(),
                self.dad(),
                self.pfivl(),
                self.xcvrdly()
            )
        }
    }
    #[doc = "Device control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dctl(pub u32);
    impl Dctl {
        #[doc = "Remote wakeup signaling"]
        #[inline(always)]
        pub const fn rwusig(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Remote wakeup signaling"]
        #[inline(always)]
        pub fn set_rwusig(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Soft disconnect"]
        #[inline(always)]
        pub const fn sdis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Soft disconnect"]
        #[inline(always)]
        pub fn set_sdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Global IN NAK status"]
        #[inline(always)]
        pub const fn ginsts(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Global IN NAK status"]
        #[inline(always)]
        pub fn set_ginsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Global OUT NAK status"]
        #[inline(always)]
        pub const fn gonsts(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Global OUT NAK status"]
        #[inline(always)]
        pub fn set_gonsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Test control"]
        #[inline(always)]
        pub const fn tctl(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Test control"]
        #[inline(always)]
        pub fn set_tctl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Set global IN NAK"]
        #[inline(always)]
        pub const fn sginak(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Set global IN NAK"]
        #[inline(always)]
        pub fn set_sginak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Clear global IN NAK"]
        #[inline(always)]
        pub const fn cginak(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Clear global IN NAK"]
        #[inline(always)]
        pub fn set_cginak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Set global OUT NAK"]
        #[inline(always)]
        pub const fn sgonak(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Set global OUT NAK"]
        #[inline(always)]
        pub fn set_sgonak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Clear global OUT NAK"]
        #[inline(always)]
        pub const fn cgonak(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Clear global OUT NAK"]
        #[inline(always)]
        pub fn set_cgonak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Power-on programming done"]
        #[inline(always)]
        pub const fn poprgdne(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Power-on programming done"]
        #[inline(always)]
        pub fn set_poprgdne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for Dctl {
        #[inline(always)]
        fn default() -> Dctl {
            Dctl(0)
        }
    }
    impl core::fmt::Debug for Dctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dctl")
                .field("rwusig", &self.rwusig())
                .field("sdis", &self.sdis())
                .field("ginsts", &self.ginsts())
                .field("gonsts", &self.gonsts())
                .field("tctl", &self.tctl())
                .field("sginak", &self.sginak())
                .field("cginak", &self.cginak())
                .field("sgonak", &self.sgonak())
                .field("cgonak", &self.cgonak())
                .field("poprgdne", &self.poprgdne())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dctl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Dctl {{ rwusig: {=bool:?}, sdis: {=bool:?}, ginsts: {=bool:?}, gonsts: {=bool:?}, tctl: {=u8:?}, sginak: {=bool:?}, cginak: {=bool:?}, sgonak: {=bool:?}, cgonak: {=bool:?}, poprgdne: {=bool:?} }}" , self . rwusig () , self . sdis () , self . ginsts () , self . gonsts () , self . tctl () , self . sginak () , self . cginak () , self . sgonak () , self . cgonak () , self . poprgdne ())
        }
    }
    #[doc = "Device endpoint control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepctl(pub u32);
    impl Diepctl {
        #[doc = "MPSIZ"]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "MPSIZ"]
        #[inline(always)]
        pub fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "USBAEP"]
        #[inline(always)]
        pub const fn usbaep(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "USBAEP"]
        #[inline(always)]
        pub fn set_usbaep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "EONUM/DPID"]
        #[inline(always)]
        pub const fn eonum_dpid(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "EONUM/DPID"]
        #[inline(always)]
        pub fn set_eonum_dpid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "NAKSTS"]
        #[inline(always)]
        pub const fn naksts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "NAKSTS"]
        #[inline(always)]
        pub fn set_naksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "EPTYP"]
        #[inline(always)]
        pub const fn eptyp(&self) -> super::vals::Eptyp {
            let val = (self.0 >> 18usize) & 0x03;
            super::vals::Eptyp::from_bits(val as u8)
        }
        #[doc = "EPTYP"]
        #[inline(always)]
        pub fn set_eptyp(&mut self, val: super::vals::Eptyp) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
        }
        #[doc = "SNPM"]
        #[inline(always)]
        pub const fn snpm(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SNPM"]
        #[inline(always)]
        pub fn set_snpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "STALL"]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "STALL"]
        #[inline(always)]
        pub fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "TXFNUM"]
        #[inline(always)]
        pub const fn txfnum(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x0f;
            val as u8
        }
        #[doc = "TXFNUM"]
        #[inline(always)]
        pub fn set_txfnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 22usize)) | (((val as u32) & 0x0f) << 22usize);
        }
        #[doc = "CNAK"]
        #[inline(always)]
        pub const fn cnak(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "CNAK"]
        #[inline(always)]
        pub fn set_cnak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "SNAK"]
        #[inline(always)]
        pub const fn snak(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "SNAK"]
        #[inline(always)]
        pub fn set_snak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "SD0PID/SEVNFRM"]
        #[inline(always)]
        pub const fn sd0pid_sevnfrm(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "SD0PID/SEVNFRM"]
        #[inline(always)]
        pub fn set_sd0pid_sevnfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "SODDFRM/SD1PID"]
        #[inline(always)]
        pub const fn soddfrm_sd1pid(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "SODDFRM/SD1PID"]
        #[inline(always)]
        pub fn set_soddfrm_sd1pid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "EPDIS"]
        #[inline(always)]
        pub const fn epdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "EPDIS"]
        #[inline(always)]
        pub fn set_epdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "EPENA"]
        #[inline(always)]
        pub const fn epena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "EPENA"]
        #[inline(always)]
        pub fn set_epena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Diepctl {
        #[inline(always)]
        fn default() -> Diepctl {
            Diepctl(0)
        }
    }
    impl core::fmt::Debug for Diepctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepctl")
                .field("mpsiz", &self.mpsiz())
                .field("usbaep", &self.usbaep())
                .field("eonum_dpid", &self.eonum_dpid())
                .field("naksts", &self.naksts())
                .field("eptyp", &self.eptyp())
                .field("snpm", &self.snpm())
                .field("stall", &self.stall())
                .field("txfnum", &self.txfnum())
                .field("cnak", &self.cnak())
                .field("snak", &self.snak())
                .field("sd0pid_sevnfrm", &self.sd0pid_sevnfrm())
                .field("soddfrm_sd1pid", &self.soddfrm_sd1pid())
                .field("epdis", &self.epdis())
                .field("epena", &self.epena())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Diepctl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Diepctl {{ mpsiz: {=u16:?}, usbaep: {=bool:?}, eonum_dpid: {=bool:?}, naksts: {=bool:?}, eptyp: {:?}, snpm: {=bool:?}, stall: {=bool:?}, txfnum: {=u8:?}, cnak: {=bool:?}, snak: {=bool:?}, sd0pid_sevnfrm: {=bool:?}, soddfrm_sd1pid: {=bool:?}, epdis: {=bool:?}, epena: {=bool:?} }}" , self . mpsiz () , self . usbaep () , self . eonum_dpid () , self . naksts () , self . eptyp () , self . snpm () , self . stall () , self . txfnum () , self . cnak () , self . snak () , self . sd0pid_sevnfrm () , self . soddfrm_sd1pid () , self . epdis () , self . epena ())
        }
    }
    #[doc = "Device IN endpoint FIFO empty interrupt mask register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepempmsk(pub u32);
    impl Diepempmsk {
        #[doc = "IN EP Tx FIFO empty interrupt mask bits"]
        #[inline(always)]
        pub const fn ineptxfem(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "IN EP Tx FIFO empty interrupt mask bits"]
        #[inline(always)]
        pub fn set_ineptxfem(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Diepempmsk {
        #[inline(always)]
        fn default() -> Diepempmsk {
            Diepempmsk(0)
        }
    }
    impl core::fmt::Debug for Diepempmsk {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepempmsk")
                .field("ineptxfem", &self.ineptxfem())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Diepempmsk {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Diepempmsk {{ ineptxfem: {=u16:?} }}", self.ineptxfem())
        }
    }
    #[doc = "Device endpoint interrupt register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepint(pub u32);
    impl Diepint {
        #[doc = "XFRC"]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "XFRC"]
        #[inline(always)]
        pub fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "EPDISD"]
        #[inline(always)]
        pub const fn epdisd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "EPDISD"]
        #[inline(always)]
        pub fn set_epdisd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TOC"]
        #[inline(always)]
        pub const fn toc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TOC"]
        #[inline(always)]
        pub fn set_toc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "ITTXFE"]
        #[inline(always)]
        pub const fn ittxfe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ITTXFE"]
        #[inline(always)]
        pub fn set_ittxfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "INEPNE"]
        #[inline(always)]
        pub const fn inepne(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "INEPNE"]
        #[inline(always)]
        pub fn set_inepne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TXFE"]
        #[inline(always)]
        pub const fn txfe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TXFE"]
        #[inline(always)]
        pub fn set_txfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Diepint {
        #[inline(always)]
        fn default() -> Diepint {
            Diepint(0)
        }
    }
    impl core::fmt::Debug for Diepint {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepint")
                .field("xfrc", &self.xfrc())
                .field("epdisd", &self.epdisd())
                .field("toc", &self.toc())
                .field("ittxfe", &self.ittxfe())
                .field("inepne", &self.inepne())
                .field("txfe", &self.txfe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Diepint {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Diepint {{ xfrc: {=bool:?}, epdisd: {=bool:?}, toc: {=bool:?}, ittxfe: {=bool:?}, inepne: {=bool:?}, txfe: {=bool:?} }}" , self . xfrc () , self . epdisd () , self . toc () , self . ittxfe () , self . inepne () , self . txfe ())
        }
    }
    #[doc = "Device IN endpoint common interrupt mask register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Diepmsk(pub u32);
    impl Diepmsk {
        #[doc = "Transfer completed interrupt mask"]
        #[inline(always)]
        pub const fn xfrcm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer completed interrupt mask"]
        #[inline(always)]
        pub fn set_xfrcm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Endpoint disabled interrupt mask"]
        #[inline(always)]
        pub const fn epdm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint disabled interrupt mask"]
        #[inline(always)]
        pub fn set_epdm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timeout condition mask (Non-isochronous endpoints)"]
        #[inline(always)]
        pub const fn tom(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Timeout condition mask (Non-isochronous endpoints)"]
        #[inline(always)]
        pub fn set_tom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "IN token received when TxFIFO empty mask"]
        #[inline(always)]
        pub const fn ittxfemsk(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IN token received when TxFIFO empty mask"]
        #[inline(always)]
        pub fn set_ittxfemsk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "IN token received with EP mismatch mask"]
        #[inline(always)]
        pub const fn inepnmm(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "IN token received with EP mismatch mask"]
        #[inline(always)]
        pub fn set_inepnmm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "IN endpoint NAK effective mask"]
        #[inline(always)]
        pub const fn inepnem(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "IN endpoint NAK effective mask"]
        #[inline(always)]
        pub fn set_inepnem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Diepmsk {
        #[inline(always)]
        fn default() -> Diepmsk {
            Diepmsk(0)
        }
    }
    impl core::fmt::Debug for Diepmsk {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Diepmsk")
                .field("xfrcm", &self.xfrcm())
                .field("epdm", &self.epdm())
                .field("tom", &self.tom())
                .field("ittxfemsk", &self.ittxfemsk())
                .field("inepnmm", &self.inepnmm())
                .field("inepnem", &self.inepnem())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Diepmsk {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Diepmsk {{ xfrcm: {=bool:?}, epdm: {=bool:?}, tom: {=bool:?}, ittxfemsk: {=bool:?}, inepnmm: {=bool:?}, inepnem: {=bool:?} }}" , self . xfrcm () , self . epdm () , self . tom () , self . ittxfemsk () , self . inepnmm () , self . inepnem ())
        }
    }
    #[doc = "Device endpoint transfer size register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dieptsiz(pub u32);
    impl Dieptsiz {
        #[doc = "Transfer size"]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "Transfer size"]
        #[inline(always)]
        pub fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "Packet count"]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "Packet count"]
        #[inline(always)]
        pub fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "Multi count"]
        #[inline(always)]
        pub const fn mcnt(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "Multi count"]
        #[inline(always)]
        pub fn set_mcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Dieptsiz {
        #[inline(always)]
        fn default() -> Dieptsiz {
            Dieptsiz(0)
        }
    }
    impl core::fmt::Debug for Dieptsiz {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dieptsiz")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("mcnt", &self.mcnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dieptsiz {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dieptsiz {{ xfrsiz: {=u32:?}, pktcnt: {=u16:?}, mcnt: {=u8:?} }}",
                self.xfrsiz(),
                self.pktcnt(),
                self.mcnt()
            )
        }
    }
    #[doc = "Device endpoint control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepctl(pub u32);
    impl Doepctl {
        #[doc = "MPSIZ"]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "MPSIZ"]
        #[inline(always)]
        pub fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "USBAEP"]
        #[inline(always)]
        pub const fn usbaep(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "USBAEP"]
        #[inline(always)]
        pub fn set_usbaep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "EONUM/DPID"]
        #[inline(always)]
        pub const fn eonum_dpid(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "EONUM/DPID"]
        #[inline(always)]
        pub fn set_eonum_dpid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "NAKSTS"]
        #[inline(always)]
        pub const fn naksts(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "NAKSTS"]
        #[inline(always)]
        pub fn set_naksts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "EPTYP"]
        #[inline(always)]
        pub const fn eptyp(&self) -> super::vals::Eptyp {
            let val = (self.0 >> 18usize) & 0x03;
            super::vals::Eptyp::from_bits(val as u8)
        }
        #[doc = "EPTYP"]
        #[inline(always)]
        pub fn set_eptyp(&mut self, val: super::vals::Eptyp) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
        }
        #[doc = "SNPM"]
        #[inline(always)]
        pub const fn snpm(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SNPM"]
        #[inline(always)]
        pub fn set_snpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "STALL"]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "STALL"]
        #[inline(always)]
        pub fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "CNAK"]
        #[inline(always)]
        pub const fn cnak(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "CNAK"]
        #[inline(always)]
        pub fn set_cnak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "SNAK"]
        #[inline(always)]
        pub const fn snak(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "SNAK"]
        #[inline(always)]
        pub fn set_snak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "SD0PID/SEVNFRM"]
        #[inline(always)]
        pub const fn sd0pid_sevnfrm(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "SD0PID/SEVNFRM"]
        #[inline(always)]
        pub fn set_sd0pid_sevnfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "SODDFRM"]
        #[inline(always)]
        pub const fn soddfrm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "SODDFRM"]
        #[inline(always)]
        pub fn set_soddfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "EPDIS"]
        #[inline(always)]
        pub const fn epdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "EPDIS"]
        #[inline(always)]
        pub fn set_epdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "EPENA"]
        #[inline(always)]
        pub const fn epena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "EPENA"]
        #[inline(always)]
        pub fn set_epena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Doepctl {
        #[inline(always)]
        fn default() -> Doepctl {
            Doepctl(0)
        }
    }
    impl core::fmt::Debug for Doepctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepctl")
                .field("mpsiz", &self.mpsiz())
                .field("usbaep", &self.usbaep())
                .field("eonum_dpid", &self.eonum_dpid())
                .field("naksts", &self.naksts())
                .field("eptyp", &self.eptyp())
                .field("snpm", &self.snpm())
                .field("stall", &self.stall())
                .field("cnak", &self.cnak())
                .field("snak", &self.snak())
                .field("sd0pid_sevnfrm", &self.sd0pid_sevnfrm())
                .field("soddfrm", &self.soddfrm())
                .field("epdis", &self.epdis())
                .field("epena", &self.epena())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Doepctl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Doepctl {{ mpsiz: {=u16:?}, usbaep: {=bool:?}, eonum_dpid: {=bool:?}, naksts: {=bool:?}, eptyp: {:?}, snpm: {=bool:?}, stall: {=bool:?}, cnak: {=bool:?}, snak: {=bool:?}, sd0pid_sevnfrm: {=bool:?}, soddfrm: {=bool:?}, epdis: {=bool:?}, epena: {=bool:?} }}" , self . mpsiz () , self . usbaep () , self . eonum_dpid () , self . naksts () , self . eptyp () , self . snpm () , self . stall () , self . cnak () , self . snak () , self . sd0pid_sevnfrm () , self . soddfrm () , self . epdis () , self . epena ())
        }
    }
    #[doc = "Device endpoint interrupt register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepint(pub u32);
    impl Doepint {
        #[doc = "XFRC"]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "XFRC"]
        #[inline(always)]
        pub fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "EPDISD"]
        #[inline(always)]
        pub const fn epdisd(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "EPDISD"]
        #[inline(always)]
        pub fn set_epdisd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "STUP"]
        #[inline(always)]
        pub const fn stup(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "STUP"]
        #[inline(always)]
        pub fn set_stup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "OTEPDIS"]
        #[inline(always)]
        pub const fn otepdis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OTEPDIS"]
        #[inline(always)]
        pub fn set_otepdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "B2BSTUP"]
        #[inline(always)]
        pub const fn b2bstup(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "B2BSTUP"]
        #[inline(always)]
        pub fn set_b2bstup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Doepint {
        #[inline(always)]
        fn default() -> Doepint {
            Doepint(0)
        }
    }
    impl core::fmt::Debug for Doepint {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepint")
                .field("xfrc", &self.xfrc())
                .field("epdisd", &self.epdisd())
                .field("stup", &self.stup())
                .field("otepdis", &self.otepdis())
                .field("b2bstup", &self.b2bstup())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Doepint {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Doepint {{ xfrc: {=bool:?}, epdisd: {=bool:?}, stup: {=bool:?}, otepdis: {=bool:?}, b2bstup: {=bool:?} }}" , self . xfrc () , self . epdisd () , self . stup () , self . otepdis () , self . b2bstup ())
        }
    }
    #[doc = "Device OUT endpoint common interrupt mask register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doepmsk(pub u32);
    impl Doepmsk {
        #[doc = "Transfer completed interrupt mask"]
        #[inline(always)]
        pub const fn xfrcm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer completed interrupt mask"]
        #[inline(always)]
        pub fn set_xfrcm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Endpoint disabled interrupt mask"]
        #[inline(always)]
        pub const fn epdm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint disabled interrupt mask"]
        #[inline(always)]
        pub fn set_epdm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SETUP phase done mask"]
        #[inline(always)]
        pub const fn stupm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "SETUP phase done mask"]
        #[inline(always)]
        pub fn set_stupm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "OUT token received when endpoint disabled mask"]
        #[inline(always)]
        pub const fn otepdm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OUT token received when endpoint disabled mask"]
        #[inline(always)]
        pub fn set_otepdm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Doepmsk {
        #[inline(always)]
        fn default() -> Doepmsk {
            Doepmsk(0)
        }
    }
    impl core::fmt::Debug for Doepmsk {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doepmsk")
                .field("xfrcm", &self.xfrcm())
                .field("epdm", &self.epdm())
                .field("stupm", &self.stupm())
                .field("otepdm", &self.otepdm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Doepmsk {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Doepmsk {{ xfrcm: {=bool:?}, epdm: {=bool:?}, stupm: {=bool:?}, otepdm: {=bool:?} }}",
                self.xfrcm(),
                self.epdm(),
                self.stupm(),
                self.otepdm()
            )
        }
    }
    #[doc = "Device OUT endpoint transfer size register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doeptsiz(pub u32);
    impl Doeptsiz {
        #[doc = "Transfer size"]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "Transfer size"]
        #[inline(always)]
        pub fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "Packet count"]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "Packet count"]
        #[inline(always)]
        pub fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "Received data PID/SETUP packet count"]
        #[inline(always)]
        pub const fn rxdpid_stupcnt(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "Received data PID/SETUP packet count"]
        #[inline(always)]
        pub fn set_rxdpid_stupcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
    }
    impl Default for Doeptsiz {
        #[inline(always)]
        fn default() -> Doeptsiz {
            Doeptsiz(0)
        }
    }
    impl core::fmt::Debug for Doeptsiz {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Doeptsiz")
                .field("xfrsiz", &self.xfrsiz())
                .field("pktcnt", &self.pktcnt())
                .field("rxdpid_stupcnt", &self.rxdpid_stupcnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Doeptsiz {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Doeptsiz {{ xfrsiz: {=u32:?}, pktcnt: {=u16:?}, rxdpid_stupcnt: {=u8:?} }}",
                self.xfrsiz(),
                self.pktcnt(),
                self.rxdpid_stupcnt()
            )
        }
    }
    #[doc = "Device status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dsts(pub u32);
    impl Dsts {
        #[doc = "Suspend status"]
        #[inline(always)]
        pub const fn suspsts(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Suspend status"]
        #[inline(always)]
        pub fn set_suspsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Enumerated speed"]
        #[inline(always)]
        pub const fn enumspd(&self) -> super::vals::Dspd {
            let val = (self.0 >> 1usize) & 0x03;
            super::vals::Dspd::from_bits(val as u8)
        }
        #[doc = "Enumerated speed"]
        #[inline(always)]
        pub fn set_enumspd(&mut self, val: super::vals::Dspd) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
        }
        #[doc = "Erratic error"]
        #[inline(always)]
        pub const fn eerr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Erratic error"]
        #[inline(always)]
        pub fn set_eerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Frame number of the received SOF"]
        #[inline(always)]
        pub const fn fnsof(&self) -> u16 {
            let val = (self.0 >> 8usize) & 0x3fff;
            val as u16
        }
        #[doc = "Frame number of the received SOF"]
        #[inline(always)]
        pub fn set_fnsof(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 8usize)) | (((val as u32) & 0x3fff) << 8usize);
        }
    }
    impl Default for Dsts {
        #[inline(always)]
        fn default() -> Dsts {
            Dsts(0)
        }
    }
    impl core::fmt::Debug for Dsts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dsts")
                .field("suspsts", &self.suspsts())
                .field("enumspd", &self.enumspd())
                .field("eerr", &self.eerr())
                .field("fnsof", &self.fnsof())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dsts {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dsts {{ suspsts: {=bool:?}, enumspd: {:?}, eerr: {=bool:?}, fnsof: {=u16:?} }}",
                self.suspsts(),
                self.enumspd(),
                self.eerr(),
                self.fnsof()
            )
        }
    }
    #[doc = "Device IN endpoint transmit FIFO status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dtxfsts(pub u32);
    impl Dtxfsts {
        #[doc = "IN endpoint TxFIFO space available"]
        #[inline(always)]
        pub const fn ineptfsav(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "IN endpoint TxFIFO space available"]
        #[inline(always)]
        pub fn set_ineptfsav(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dtxfsts {
        #[inline(always)]
        fn default() -> Dtxfsts {
            Dtxfsts(0)
        }
    }
    impl core::fmt::Debug for Dtxfsts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dtxfsts").field("ineptfsav", &self.ineptfsav()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dtxfsts {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dtxfsts {{ ineptfsav: {=u16:?} }}", self.ineptfsav())
        }
    }
    #[doc = "Device VBUS discharge time register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dvbusdis(pub u32);
    impl Dvbusdis {
        #[doc = "Device VBUS discharge time"]
        #[inline(always)]
        pub const fn vbusdt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Device VBUS discharge time"]
        #[inline(always)]
        pub fn set_vbusdt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dvbusdis {
        #[inline(always)]
        fn default() -> Dvbusdis {
            Dvbusdis(0)
        }
    }
    impl core::fmt::Debug for Dvbusdis {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dvbusdis").field("vbusdt", &self.vbusdt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dvbusdis {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dvbusdis {{ vbusdt: {=u16:?} }}", self.vbusdt())
        }
    }
    #[doc = "Device VBUS pulsing time register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dvbuspulse(pub u32);
    impl Dvbuspulse {
        #[doc = "Device VBUS pulsing time"]
        #[inline(always)]
        pub const fn dvbusp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Device VBUS pulsing time"]
        #[inline(always)]
        pub fn set_dvbusp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Dvbuspulse {
        #[inline(always)]
        fn default() -> Dvbuspulse {
            Dvbuspulse(0)
        }
    }
    impl core::fmt::Debug for Dvbuspulse {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dvbuspulse").field("dvbusp", &self.dvbusp()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dvbuspulse {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dvbuspulse {{ dvbusp: {=u16:?} }}", self.dvbusp())
        }
    }
    #[doc = "FIFO register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fifo(pub u32);
    impl Fifo {
        #[doc = "Data"]
        #[inline(always)]
        pub const fn data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Data"]
        #[inline(always)]
        pub fn set_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Fifo {
        #[inline(always)]
        fn default() -> Fifo {
            Fifo(0)
        }
    }
    impl core::fmt::Debug for Fifo {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fifo").field("data", &self.data()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fifo {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Fifo {{ data: {=u32:?} }}", self.data())
        }
    }
    #[doc = "FIFO size register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fsiz(pub u32);
    impl Fsiz {
        #[doc = "RAM start address"]
        #[inline(always)]
        pub const fn sa(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "RAM start address"]
        #[inline(always)]
        pub fn set_sa(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "FIFO depth"]
        #[inline(always)]
        pub const fn fd(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "FIFO depth"]
        #[inline(always)]
        pub fn set_fd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Fsiz {
        #[inline(always)]
        fn default() -> Fsiz {
            Fsiz(0)
        }
    }
    impl core::fmt::Debug for Fsiz {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fsiz")
                .field("sa", &self.sa())
                .field("fd", &self.fd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fsiz {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Fsiz {{ sa: {=u16:?}, fd: {=u16:?} }}", self.sa(), self.fd())
        }
    }
    #[doc = "AHB configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gahbcfg(pub u32);
    impl Gahbcfg {
        #[doc = "Global interrupt mask"]
        #[inline(always)]
        pub const fn gint(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Global interrupt mask"]
        #[inline(always)]
        pub fn set_gint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Burst length/type"]
        #[inline(always)]
        pub const fn hbstlen(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x0f;
            val as u8
        }
        #[doc = "Burst length/type"]
        #[inline(always)]
        pub fn set_hbstlen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
        }
        #[doc = "DMA enable"]
        #[inline(always)]
        pub const fn dmaen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "DMA enable"]
        #[inline(always)]
        pub fn set_dmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TxFIFO empty level"]
        #[inline(always)]
        pub const fn txfelvl(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TxFIFO empty level"]
        #[inline(always)]
        pub fn set_txfelvl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Periodic TxFIFO empty level"]
        #[inline(always)]
        pub const fn ptxfelvl(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Periodic TxFIFO empty level"]
        #[inline(always)]
        pub fn set_ptxfelvl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Gahbcfg {
        #[inline(always)]
        fn default() -> Gahbcfg {
            Gahbcfg(0)
        }
    }
    impl core::fmt::Debug for Gahbcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gahbcfg")
                .field("gint", &self.gint())
                .field("hbstlen", &self.hbstlen())
                .field("dmaen", &self.dmaen())
                .field("txfelvl", &self.txfelvl())
                .field("ptxfelvl", &self.ptxfelvl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gahbcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Gahbcfg {{ gint: {=bool:?}, hbstlen: {=u8:?}, dmaen: {=bool:?}, txfelvl: {=bool:?}, ptxfelvl: {=bool:?} }}" , self . gint () , self . hbstlen () , self . dmaen () , self . txfelvl () , self . ptxfelvl ())
        }
    }
    #[doc = "General core configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GccfgV1(pub u32);
    impl GccfgV1 {
        #[doc = "Power down"]
        #[inline(always)]
        pub const fn pwrdwn(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Power down"]
        #[inline(always)]
        pub fn set_pwrdwn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Enable the VBUS \"A\" sensing device"]
        #[inline(always)]
        pub const fn vbusasen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Enable the VBUS \"A\" sensing device"]
        #[inline(always)]
        pub fn set_vbusasen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Enable the VBUS \"B\" sensing device"]
        #[inline(always)]
        pub const fn vbusbsen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Enable the VBUS \"B\" sensing device"]
        #[inline(always)]
        pub fn set_vbusbsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "SOF output enable"]
        #[inline(always)]
        pub const fn sofouten(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SOF output enable"]
        #[inline(always)]
        pub fn set_sofouten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "VBUS sensing disable"]
        #[inline(always)]
        pub const fn novbussens(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "VBUS sensing disable"]
        #[inline(always)]
        pub fn set_novbussens(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for GccfgV1 {
        #[inline(always)]
        fn default() -> GccfgV1 {
            GccfgV1(0)
        }
    }
    impl core::fmt::Debug for GccfgV1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GccfgV1")
                .field("pwrdwn", &self.pwrdwn())
                .field("vbusasen", &self.vbusasen())
                .field("vbusbsen", &self.vbusbsen())
                .field("sofouten", &self.sofouten())
                .field("novbussens", &self.novbussens())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GccfgV1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "GccfgV1 {{ pwrdwn: {=bool:?}, vbusasen: {=bool:?}, vbusbsen: {=bool:?}, sofouten: {=bool:?}, novbussens: {=bool:?} }}" , self . pwrdwn () , self . vbusasen () , self . vbusbsen () , self . sofouten () , self . novbussens ())
        }
    }
    #[doc = "General core configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GccfgV2(pub u32);
    impl GccfgV2 {
        #[doc = "Data contact detection (DCD) status"]
        #[inline(always)]
        pub const fn dcdet(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Data contact detection (DCD) status"]
        #[inline(always)]
        pub fn set_dcdet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Primary detection (PD) status"]
        #[inline(always)]
        pub const fn pdet(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Primary detection (PD) status"]
        #[inline(always)]
        pub fn set_pdet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Secondary detection (SD) status"]
        #[inline(always)]
        pub const fn sdet(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Secondary detection (SD) status"]
        #[inline(always)]
        pub fn set_sdet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "DM pull-up detection status"]
        #[inline(always)]
        pub const fn ps2det(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DM pull-up detection status"]
        #[inline(always)]
        pub fn set_ps2det(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Power down"]
        #[inline(always)]
        pub const fn pwrdwn(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Power down"]
        #[inline(always)]
        pub fn set_pwrdwn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Battery charging detector (BCD) enable"]
        #[inline(always)]
        pub const fn bcden(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Battery charging detector (BCD) enable"]
        #[inline(always)]
        pub fn set_bcden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Data contact detection (DCD) mode enable"]
        #[inline(always)]
        pub const fn dcden(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Data contact detection (DCD) mode enable"]
        #[inline(always)]
        pub fn set_dcden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Primary detection (PD) mode enable"]
        #[inline(always)]
        pub const fn pden(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Primary detection (PD) mode enable"]
        #[inline(always)]
        pub fn set_pden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Secondary detection (SD) mode enable"]
        #[inline(always)]
        pub const fn sden(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Secondary detection (SD) mode enable"]
        #[inline(always)]
        pub fn set_sden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "USB VBUS detection enable"]
        #[inline(always)]
        pub const fn vbden(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "USB VBUS detection enable"]
        #[inline(always)]
        pub fn set_vbden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Internal high-speed PHY enable."]
        #[inline(always)]
        pub const fn phyhsen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Internal high-speed PHY enable."]
        #[inline(always)]
        pub fn set_phyhsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for GccfgV2 {
        #[inline(always)]
        fn default() -> GccfgV2 {
            GccfgV2(0)
        }
    }
    impl core::fmt::Debug for GccfgV2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GccfgV2")
                .field("dcdet", &self.dcdet())
                .field("pdet", &self.pdet())
                .field("sdet", &self.sdet())
                .field("ps2det", &self.ps2det())
                .field("pwrdwn", &self.pwrdwn())
                .field("bcden", &self.bcden())
                .field("dcden", &self.dcden())
                .field("pden", &self.pden())
                .field("sden", &self.sden())
                .field("vbden", &self.vbden())
                .field("phyhsen", &self.phyhsen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GccfgV2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "GccfgV2 {{ dcdet: {=bool:?}, pdet: {=bool:?}, sdet: {=bool:?}, ps2det: {=bool:?}, pwrdwn: {=bool:?}, bcden: {=bool:?}, dcden: {=bool:?}, pden: {=bool:?}, sden: {=bool:?}, vbden: {=bool:?}, phyhsen: {=bool:?} }}" , self . dcdet () , self . pdet () , self . sdet () , self . ps2det () , self . pwrdwn () , self . bcden () , self . dcden () , self . pden () , self . sden () , self . vbden () , self . phyhsen ())
        }
    }
    #[doc = "OTG general core configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GccfgV3(pub u32);
    impl GccfgV3 {
        #[doc = "Charger detection, result of the current mode (primary or secondary)."]
        #[inline(always)]
        pub const fn chgdet(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Charger detection, result of the current mode (primary or secondary)."]
        #[inline(always)]
        pub fn set_chgdet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Single-Ended DP indicator This bit gives the voltage level on DP (also result of the comparison with V<sub>LGC</sub> threshold as defined in BC v1.2 standard)."]
        #[inline(always)]
        pub const fn fsvplus(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Single-Ended DP indicator This bit gives the voltage level on DP (also result of the comparison with V<sub>LGC</sub> threshold as defined in BC v1.2 standard)."]
        #[inline(always)]
        pub fn set_fsvplus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Single-Ended DM indicator This bit gives the voltage level on DM (also result of the comparison with V<sub>LGC</sub> threshold as defined in BC v1.2 standard)."]
        #[inline(always)]
        pub const fn fsvminus(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Single-Ended DM indicator This bit gives the voltage level on DM (also result of the comparison with V<sub>LGC</sub> threshold as defined in BC v1.2 standard)."]
        #[inline(always)]
        pub fn set_fsvminus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "VBUS session indicator Indicates if VBUS is above VBUS session threshold."]
        #[inline(always)]
        pub const fn sessvld(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "VBUS session indicator Indicates if VBUS is above VBUS session threshold."]
        #[inline(always)]
        pub fn set_sessvld(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Host CDP behavior enable."]
        #[inline(always)]
        pub const fn hcdpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Host CDP behavior enable."]
        #[inline(always)]
        pub fn set_hcdpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Host CDP port voltage detector enable on DP."]
        #[inline(always)]
        pub const fn hcdpdeten(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Host CDP port voltage detector enable on DP."]
        #[inline(always)]
        pub fn set_hcdpdeten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Host CDP port Voltage source enable on DM."]
        #[inline(always)]
        pub const fn hvdmsrcen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Host CDP port Voltage source enable on DM."]
        #[inline(always)]
        pub fn set_hvdmsrcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Data Contact Detection enable."]
        #[inline(always)]
        pub const fn dcden(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Data Contact Detection enable."]
        #[inline(always)]
        pub fn set_dcden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Primary detection enable."]
        #[inline(always)]
        pub const fn pden(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Primary detection enable."]
        #[inline(always)]
        pub fn set_pden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "VBUS detection enable Enables VBUS Sensing Comparators in order to detect VBUS presence and/or perform OTG operation."]
        #[inline(always)]
        pub const fn vbden(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "VBUS detection enable Enables VBUS Sensing Comparators in order to detect VBUS presence and/or perform OTG operation."]
        #[inline(always)]
        pub fn set_vbden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Secondary detection enable."]
        #[inline(always)]
        pub const fn sden(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Secondary detection enable."]
        #[inline(always)]
        pub fn set_sden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Software override value of the VBUS B-session detection."]
        #[inline(always)]
        pub const fn vbvaloval(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Software override value of the VBUS B-session detection."]
        #[inline(always)]
        pub fn set_vbvaloval(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Enables a software override of the VBUS B-session detection."]
        #[inline(always)]
        pub const fn vbvaloven(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Enables a software override of the VBUS B-session detection."]
        #[inline(always)]
        pub fn set_vbvaloven(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Force host mode pull-downs If the ID pin functions are enabled, the host mode pull-downs on DP and DM activate automatically. However, whenever that is not the case, yet host mode is required, this bit must be used to force the pull-downs active."]
        #[inline(always)]
        pub const fn forcehostpd(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Force host mode pull-downs If the ID pin functions are enabled, the host mode pull-downs on DP and DM activate automatically. However, whenever that is not the case, yet host mode is required, this bit must be used to force the pull-downs active."]
        #[inline(always)]
        pub fn set_forcehostpd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for GccfgV3 {
        #[inline(always)]
        fn default() -> GccfgV3 {
            GccfgV3(0)
        }
    }
    impl core::fmt::Debug for GccfgV3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GccfgV3")
                .field("chgdet", &self.chgdet())
                .field("fsvplus", &self.fsvplus())
                .field("fsvminus", &self.fsvminus())
                .field("sessvld", &self.sessvld())
                .field("hcdpen", &self.hcdpen())
                .field("hcdpdeten", &self.hcdpdeten())
                .field("hvdmsrcen", &self.hvdmsrcen())
                .field("dcden", &self.dcden())
                .field("pden", &self.pden())
                .field("vbden", &self.vbden())
                .field("sden", &self.sden())
                .field("vbvaloval", &self.vbvaloval())
                .field("vbvaloven", &self.vbvaloven())
                .field("forcehostpd", &self.forcehostpd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for GccfgV3 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "GccfgV3 {{ chgdet: {=bool:?}, fsvplus: {=bool:?}, fsvminus: {=bool:?}, sessvld: {=bool:?}, hcdpen: {=bool:?}, hcdpdeten: {=bool:?}, hvdmsrcen: {=bool:?}, dcden: {=bool:?}, pden: {=bool:?}, vbden: {=bool:?}, sden: {=bool:?}, vbvaloval: {=bool:?}, vbvaloven: {=bool:?}, forcehostpd: {=bool:?} }}" , self . chgdet () , self . fsvplus () , self . fsvminus () , self . sessvld () , self . hcdpen () , self . hcdpdeten () , self . hvdmsrcen () , self . dcden () , self . pden () , self . vbden () , self . sden () , self . vbvaloval () , self . vbvaloven () , self . forcehostpd ())
        }
    }
    #[doc = "I2C access register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gi2cctl(pub u32);
    impl Gi2cctl {
        #[doc = "I2C Read/Write Data"]
        #[inline(always)]
        pub const fn rwdata(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "I2C Read/Write Data"]
        #[inline(always)]
        pub fn set_rwdata(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "I2C Register Address"]
        #[inline(always)]
        pub const fn regaddr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "I2C Register Address"]
        #[inline(always)]
        pub fn set_regaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "I2C Address"]
        #[inline(always)]
        pub const fn addr(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "I2C Address"]
        #[inline(always)]
        pub fn set_addr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "I2C Enable"]
        #[inline(always)]
        pub const fn i2cen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C Enable"]
        #[inline(always)]
        pub fn set_i2cen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "I2C ACK"]
        #[inline(always)]
        pub const fn ack(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "I2C ACK"]
        #[inline(always)]
        pub fn set_ack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "I2C Device Address"]
        #[inline(always)]
        pub const fn i2cdevadr(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "I2C Device Address"]
        #[inline(always)]
        pub fn set_i2cdevadr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "I2C DatSe0 USB mode"]
        #[inline(always)]
        pub const fn i2cdatse0(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "I2C DatSe0 USB mode"]
        #[inline(always)]
        pub fn set_i2cdatse0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Read/Write Indicator"]
        #[inline(always)]
        pub const fn rw(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Read/Write Indicator"]
        #[inline(always)]
        pub fn set_rw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "I2C Busy/Done"]
        #[inline(always)]
        pub const fn bsydne(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "I2C Busy/Done"]
        #[inline(always)]
        pub fn set_bsydne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Gi2cctl {
        #[inline(always)]
        fn default() -> Gi2cctl {
            Gi2cctl(0)
        }
    }
    impl core::fmt::Debug for Gi2cctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gi2cctl")
                .field("rwdata", &self.rwdata())
                .field("regaddr", &self.regaddr())
                .field("addr", &self.addr())
                .field("i2cen", &self.i2cen())
                .field("ack", &self.ack())
                .field("i2cdevadr", &self.i2cdevadr())
                .field("i2cdatse0", &self.i2cdatse0())
                .field("rw", &self.rw())
                .field("bsydne", &self.bsydne())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gi2cctl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Gi2cctl {{ rwdata: {=u8:?}, regaddr: {=u8:?}, addr: {=u8:?}, i2cen: {=bool:?}, ack: {=bool:?}, i2cdevadr: {=u8:?}, i2cdatse0: {=bool:?}, rw: {=bool:?}, bsydne: {=bool:?} }}" , self . rwdata () , self . regaddr () , self . addr () , self . i2cen () , self . ack () , self . i2cdevadr () , self . i2cdatse0 () , self . rw () , self . bsydne ())
        }
    }
    #[doc = "Interrupt mask register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gintmsk(pub u32);
    impl Gintmsk {
        #[doc = "Mode mismatch interrupt mask"]
        #[inline(always)]
        pub const fn mmism(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Mode mismatch interrupt mask"]
        #[inline(always)]
        pub fn set_mmism(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "OTG interrupt mask"]
        #[inline(always)]
        pub const fn otgint(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "OTG interrupt mask"]
        #[inline(always)]
        pub fn set_otgint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Start of frame mask"]
        #[inline(always)]
        pub const fn sofm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Start of frame mask"]
        #[inline(always)]
        pub fn set_sofm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Receive FIFO non-empty mask"]
        #[inline(always)]
        pub const fn rxflvlm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Receive FIFO non-empty mask"]
        #[inline(always)]
        pub fn set_rxflvlm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Non-periodic TxFIFO empty mask"]
        #[inline(always)]
        pub const fn nptxfem(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Non-periodic TxFIFO empty mask"]
        #[inline(always)]
        pub fn set_nptxfem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Global non-periodic IN NAK effective mask"]
        #[inline(always)]
        pub const fn ginakeffm(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Global non-periodic IN NAK effective mask"]
        #[inline(always)]
        pub fn set_ginakeffm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Global OUT NAK effective mask"]
        #[inline(always)]
        pub const fn gonakeffm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Global OUT NAK effective mask"]
        #[inline(always)]
        pub fn set_gonakeffm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Early suspend mask"]
        #[inline(always)]
        pub const fn esuspm(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Early suspend mask"]
        #[inline(always)]
        pub fn set_esuspm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "USB suspend mask"]
        #[inline(always)]
        pub const fn usbsuspm(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "USB suspend mask"]
        #[inline(always)]
        pub fn set_usbsuspm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "USB reset mask"]
        #[inline(always)]
        pub const fn usbrst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "USB reset mask"]
        #[inline(always)]
        pub fn set_usbrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Enumeration done mask"]
        #[inline(always)]
        pub const fn enumdnem(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Enumeration done mask"]
        #[inline(always)]
        pub fn set_enumdnem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Isochronous OUT packet dropped interrupt mask"]
        #[inline(always)]
        pub const fn isoodrpm(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Isochronous OUT packet dropped interrupt mask"]
        #[inline(always)]
        pub fn set_isoodrpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "End of periodic frame interrupt mask"]
        #[inline(always)]
        pub const fn eopfm(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "End of periodic frame interrupt mask"]
        #[inline(always)]
        pub fn set_eopfm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Endpoint mismatch interrupt mask"]
        #[inline(always)]
        pub const fn epmism(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint mismatch interrupt mask"]
        #[inline(always)]
        pub fn set_epmism(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "IN endpoints interrupt mask"]
        #[inline(always)]
        pub const fn iepint(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "IN endpoints interrupt mask"]
        #[inline(always)]
        pub fn set_iepint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "OUT endpoints interrupt mask"]
        #[inline(always)]
        pub const fn oepint(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "OUT endpoints interrupt mask"]
        #[inline(always)]
        pub fn set_oepint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Incomplete isochronous IN transfer mask"]
        #[inline(always)]
        pub const fn iisoixfrm(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Incomplete isochronous IN transfer mask"]
        #[inline(always)]
        pub fn set_iisoixfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Incomplete periodic transfer mask (host mode) / Incomplete isochronous OUT transfer mask (device mode)"]
        #[inline(always)]
        pub const fn ipxfrm_iisooxfrm(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Incomplete periodic transfer mask (host mode) / Incomplete isochronous OUT transfer mask (device mode)"]
        #[inline(always)]
        pub fn set_ipxfrm_iisooxfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Data fetch suspended mask"]
        #[inline(always)]
        pub const fn fsuspm(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Data fetch suspended mask"]
        #[inline(always)]
        pub fn set_fsuspm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Reset detected interrupt mask"]
        #[inline(always)]
        pub const fn rstde(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Reset detected interrupt mask"]
        #[inline(always)]
        pub fn set_rstde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Host port interrupt mask"]
        #[inline(always)]
        pub const fn prtim(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Host port interrupt mask"]
        #[inline(always)]
        pub fn set_prtim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Host channels interrupt mask"]
        #[inline(always)]
        pub const fn hcim(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Host channels interrupt mask"]
        #[inline(always)]
        pub fn set_hcim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Periodic TxFIFO empty mask"]
        #[inline(always)]
        pub const fn ptxfem(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Periodic TxFIFO empty mask"]
        #[inline(always)]
        pub fn set_ptxfem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "LPM interrupt mask"]
        #[inline(always)]
        pub const fn lpmintm(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "LPM interrupt mask"]
        #[inline(always)]
        pub fn set_lpmintm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Connector ID status change mask"]
        #[inline(always)]
        pub const fn cidschgm(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Connector ID status change mask"]
        #[inline(always)]
        pub fn set_cidschgm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Disconnect detected interrupt mask"]
        #[inline(always)]
        pub const fn discint(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Disconnect detected interrupt mask"]
        #[inline(always)]
        pub fn set_discint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Session request/new session detected interrupt mask"]
        #[inline(always)]
        pub const fn srqim(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Session request/new session detected interrupt mask"]
        #[inline(always)]
        pub fn set_srqim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Resume/remote wakeup detected interrupt mask"]
        #[inline(always)]
        pub const fn wuim(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Resume/remote wakeup detected interrupt mask"]
        #[inline(always)]
        pub fn set_wuim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Gintmsk {
        #[inline(always)]
        fn default() -> Gintmsk {
            Gintmsk(0)
        }
    }
    impl core::fmt::Debug for Gintmsk {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gintmsk")
                .field("mmism", &self.mmism())
                .field("otgint", &self.otgint())
                .field("sofm", &self.sofm())
                .field("rxflvlm", &self.rxflvlm())
                .field("nptxfem", &self.nptxfem())
                .field("ginakeffm", &self.ginakeffm())
                .field("gonakeffm", &self.gonakeffm())
                .field("esuspm", &self.esuspm())
                .field("usbsuspm", &self.usbsuspm())
                .field("usbrst", &self.usbrst())
                .field("enumdnem", &self.enumdnem())
                .field("isoodrpm", &self.isoodrpm())
                .field("eopfm", &self.eopfm())
                .field("epmism", &self.epmism())
                .field("iepint", &self.iepint())
                .field("oepint", &self.oepint())
                .field("iisoixfrm", &self.iisoixfrm())
                .field("ipxfrm_iisooxfrm", &self.ipxfrm_iisooxfrm())
                .field("fsuspm", &self.fsuspm())
                .field("rstde", &self.rstde())
                .field("prtim", &self.prtim())
                .field("hcim", &self.hcim())
                .field("ptxfem", &self.ptxfem())
                .field("lpmintm", &self.lpmintm())
                .field("cidschgm", &self.cidschgm())
                .field("discint", &self.discint())
                .field("srqim", &self.srqim())
                .field("wuim", &self.wuim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gintmsk {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Gintmsk {{ mmism: {=bool:?}, otgint: {=bool:?}, sofm: {=bool:?}, rxflvlm: {=bool:?}, nptxfem: {=bool:?}, ginakeffm: {=bool:?}, gonakeffm: {=bool:?}, esuspm: {=bool:?}, usbsuspm: {=bool:?}, usbrst: {=bool:?}, enumdnem: {=bool:?}, isoodrpm: {=bool:?}, eopfm: {=bool:?}, epmism: {=bool:?}, iepint: {=bool:?}, oepint: {=bool:?}, iisoixfrm: {=bool:?}, ipxfrm_iisooxfrm: {=bool:?}, fsuspm: {=bool:?}, rstde: {=bool:?}, prtim: {=bool:?}, hcim: {=bool:?}, ptxfem: {=bool:?}, lpmintm: {=bool:?}, cidschgm: {=bool:?}, discint: {=bool:?}, srqim: {=bool:?}, wuim: {=bool:?} }}" , self . mmism () , self . otgint () , self . sofm () , self . rxflvlm () , self . nptxfem () , self . ginakeffm () , self . gonakeffm () , self . esuspm () , self . usbsuspm () , self . usbrst () , self . enumdnem () , self . isoodrpm () , self . eopfm () , self . epmism () , self . iepint () , self . oepint () , self . iisoixfrm () , self . ipxfrm_iisooxfrm () , self . fsuspm () , self . rstde () , self . prtim () , self . hcim () , self . ptxfem () , self . lpmintm () , self . cidschgm () , self . discint () , self . srqim () , self . wuim ())
        }
    }
    #[doc = "Core interrupt register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gintsts(pub u32);
    impl Gintsts {
        #[doc = "Current mode of operation"]
        #[inline(always)]
        pub const fn cmod(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Current mode of operation"]
        #[inline(always)]
        pub fn set_cmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Mode mismatch interrupt"]
        #[inline(always)]
        pub const fn mmis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Mode mismatch interrupt"]
        #[inline(always)]
        pub fn set_mmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "OTG interrupt"]
        #[inline(always)]
        pub const fn otgint(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "OTG interrupt"]
        #[inline(always)]
        pub fn set_otgint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Start of frame"]
        #[inline(always)]
        pub const fn sof(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Start of frame"]
        #[inline(always)]
        pub fn set_sof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "RxFIFO non-empty"]
        #[inline(always)]
        pub const fn rxflvl(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RxFIFO non-empty"]
        #[inline(always)]
        pub fn set_rxflvl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Non-periodic TxFIFO empty"]
        #[inline(always)]
        pub const fn nptxfe(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Non-periodic TxFIFO empty"]
        #[inline(always)]
        pub fn set_nptxfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Global IN non-periodic NAK effective"]
        #[inline(always)]
        pub const fn ginakeff(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Global IN non-periodic NAK effective"]
        #[inline(always)]
        pub fn set_ginakeff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Global OUT NAK effective"]
        #[inline(always)]
        pub const fn goutnakeff(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Global OUT NAK effective"]
        #[inline(always)]
        pub fn set_goutnakeff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Early suspend"]
        #[inline(always)]
        pub const fn esusp(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Early suspend"]
        #[inline(always)]
        pub fn set_esusp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "USB suspend"]
        #[inline(always)]
        pub const fn usbsusp(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "USB suspend"]
        #[inline(always)]
        pub fn set_usbsusp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "USB reset"]
        #[inline(always)]
        pub const fn usbrst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "USB reset"]
        #[inline(always)]
        pub fn set_usbrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Enumeration done"]
        #[inline(always)]
        pub const fn enumdne(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Enumeration done"]
        #[inline(always)]
        pub fn set_enumdne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Isochronous OUT packet dropped interrupt"]
        #[inline(always)]
        pub const fn isoodrp(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Isochronous OUT packet dropped interrupt"]
        #[inline(always)]
        pub fn set_isoodrp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "End of periodic frame interrupt"]
        #[inline(always)]
        pub const fn eopf(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "End of periodic frame interrupt"]
        #[inline(always)]
        pub fn set_eopf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "IN endpoint interrupt"]
        #[inline(always)]
        pub const fn iepint(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "IN endpoint interrupt"]
        #[inline(always)]
        pub fn set_iepint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "OUT endpoint interrupt"]
        #[inline(always)]
        pub const fn oepint(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "OUT endpoint interrupt"]
        #[inline(always)]
        pub fn set_oepint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Incomplete isochronous IN transfer"]
        #[inline(always)]
        pub const fn iisoixfr(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Incomplete isochronous IN transfer"]
        #[inline(always)]
        pub fn set_iisoixfr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Incomplete periodic transfer (host mode) / Incomplete isochronous OUT transfer (device mode)"]
        #[inline(always)]
        pub const fn ipxfr_incompisoout(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Incomplete periodic transfer (host mode) / Incomplete isochronous OUT transfer (device mode)"]
        #[inline(always)]
        pub fn set_ipxfr_incompisoout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Data fetch suspended"]
        #[inline(always)]
        pub const fn datafsusp(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Data fetch suspended"]
        #[inline(always)]
        pub fn set_datafsusp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Reset detected"]
        #[inline(always)]
        pub const fn resetdet(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Reset detected"]
        #[inline(always)]
        pub fn set_resetdet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Host port interrupt"]
        #[inline(always)]
        pub const fn hprtint(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Host port interrupt"]
        #[inline(always)]
        pub fn set_hprtint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Host channels interrupt"]
        #[inline(always)]
        pub const fn hcint(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Host channels interrupt"]
        #[inline(always)]
        pub fn set_hcint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Periodic TxFIFO empty"]
        #[inline(always)]
        pub const fn ptxfe(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Periodic TxFIFO empty"]
        #[inline(always)]
        pub fn set_ptxfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Connector ID status change"]
        #[inline(always)]
        pub const fn cidschg(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Connector ID status change"]
        #[inline(always)]
        pub fn set_cidschg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Disconnect detected interrupt"]
        #[inline(always)]
        pub const fn discint(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Disconnect detected interrupt"]
        #[inline(always)]
        pub fn set_discint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Session request/new session detected interrupt"]
        #[inline(always)]
        pub const fn srqint(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Session request/new session detected interrupt"]
        #[inline(always)]
        pub fn set_srqint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Resume/remote wakeup detected interrupt"]
        #[inline(always)]
        pub const fn wkupint(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Resume/remote wakeup detected interrupt"]
        #[inline(always)]
        pub fn set_wkupint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Gintsts {
        #[inline(always)]
        fn default() -> Gintsts {
            Gintsts(0)
        }
    }
    impl core::fmt::Debug for Gintsts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gintsts")
                .field("cmod", &self.cmod())
                .field("mmis", &self.mmis())
                .field("otgint", &self.otgint())
                .field("sof", &self.sof())
                .field("rxflvl", &self.rxflvl())
                .field("nptxfe", &self.nptxfe())
                .field("ginakeff", &self.ginakeff())
                .field("goutnakeff", &self.goutnakeff())
                .field("esusp", &self.esusp())
                .field("usbsusp", &self.usbsusp())
                .field("usbrst", &self.usbrst())
                .field("enumdne", &self.enumdne())
                .field("isoodrp", &self.isoodrp())
                .field("eopf", &self.eopf())
                .field("iepint", &self.iepint())
                .field("oepint", &self.oepint())
                .field("iisoixfr", &self.iisoixfr())
                .field("ipxfr_incompisoout", &self.ipxfr_incompisoout())
                .field("datafsusp", &self.datafsusp())
                .field("resetdet", &self.resetdet())
                .field("hprtint", &self.hprtint())
                .field("hcint", &self.hcint())
                .field("ptxfe", &self.ptxfe())
                .field("cidschg", &self.cidschg())
                .field("discint", &self.discint())
                .field("srqint", &self.srqint())
                .field("wkupint", &self.wkupint())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gintsts {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Gintsts {{ cmod: {=bool:?}, mmis: {=bool:?}, otgint: {=bool:?}, sof: {=bool:?}, rxflvl: {=bool:?}, nptxfe: {=bool:?}, ginakeff: {=bool:?}, goutnakeff: {=bool:?}, esusp: {=bool:?}, usbsusp: {=bool:?}, usbrst: {=bool:?}, enumdne: {=bool:?}, isoodrp: {=bool:?}, eopf: {=bool:?}, iepint: {=bool:?}, oepint: {=bool:?}, iisoixfr: {=bool:?}, ipxfr_incompisoout: {=bool:?}, datafsusp: {=bool:?}, resetdet: {=bool:?}, hprtint: {=bool:?}, hcint: {=bool:?}, ptxfe: {=bool:?}, cidschg: {=bool:?}, discint: {=bool:?}, srqint: {=bool:?}, wkupint: {=bool:?} }}" , self . cmod () , self . mmis () , self . otgint () , self . sof () , self . rxflvl () , self . nptxfe () , self . ginakeff () , self . goutnakeff () , self . esusp () , self . usbsusp () , self . usbrst () , self . enumdne () , self . isoodrp () , self . eopf () , self . iepint () , self . oepint () , self . iisoixfr () , self . ipxfr_incompisoout () , self . datafsusp () , self . resetdet () , self . hprtint () , self . hcint () , self . ptxfe () , self . cidschg () , self . discint () , self . srqint () , self . wkupint ())
        }
    }
    #[doc = "Core LPM configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Glpmcfg(pub u32);
    impl Glpmcfg {
        #[doc = "LPM support enable"]
        #[inline(always)]
        pub const fn lpmen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LPM support enable"]
        #[inline(always)]
        pub fn set_lpmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LPM token acknowledge enable"]
        #[inline(always)]
        pub const fn lpmack(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LPM token acknowledge enable"]
        #[inline(always)]
        pub fn set_lpmack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Best effort service latency"]
        #[inline(always)]
        pub const fn besl(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x0f;
            val as u8
        }
        #[doc = "Best effort service latency"]
        #[inline(always)]
        pub fn set_besl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
        }
        #[doc = "bRemoteWake value"]
        #[inline(always)]
        pub const fn remwake(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "bRemoteWake value"]
        #[inline(always)]
        pub fn set_remwake(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "L1 Shallow Sleep enable"]
        #[inline(always)]
        pub const fn l1ssen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "L1 Shallow Sleep enable"]
        #[inline(always)]
        pub fn set_l1ssen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "BESL threshold"]
        #[inline(always)]
        pub const fn beslthrs(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "BESL threshold"]
        #[inline(always)]
        pub fn set_beslthrs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "L1 deep sleep enable"]
        #[inline(always)]
        pub const fn l1dsen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "L1 deep sleep enable"]
        #[inline(always)]
        pub fn set_l1dsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "LPM response"]
        #[inline(always)]
        pub const fn lpmrst(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[doc = "LPM response"]
        #[inline(always)]
        pub fn set_lpmrst(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
        #[doc = "Port sleep status"]
        #[inline(always)]
        pub const fn slpsts(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Port sleep status"]
        #[inline(always)]
        pub fn set_slpsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Sleep State Resume OK"]
        #[inline(always)]
        pub const fn l1rsmok(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Sleep State Resume OK"]
        #[inline(always)]
        pub fn set_l1rsmok(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "LPM Channel Index"]
        #[inline(always)]
        pub const fn lpmchidx(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x0f;
            val as u8
        }
        #[doc = "LPM Channel Index"]
        #[inline(always)]
        pub fn set_lpmchidx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 17usize)) | (((val as u32) & 0x0f) << 17usize);
        }
        #[doc = "LPM retry count"]
        #[inline(always)]
        pub const fn lpmrcnt(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x07;
            val as u8
        }
        #[doc = "LPM retry count"]
        #[inline(always)]
        pub fn set_lpmrcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
        }
        #[doc = "Send LPM transaction"]
        #[inline(always)]
        pub const fn sndlpm(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Send LPM transaction"]
        #[inline(always)]
        pub fn set_sndlpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "LPM retry count status"]
        #[inline(always)]
        pub const fn lpmrcntsts(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x07;
            val as u8
        }
        #[doc = "LPM retry count status"]
        #[inline(always)]
        pub fn set_lpmrcntsts(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 25usize)) | (((val as u32) & 0x07) << 25usize);
        }
        #[doc = "Enable best effort service latency"]
        #[inline(always)]
        pub const fn enbesl(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Enable best effort service latency"]
        #[inline(always)]
        pub fn set_enbesl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Glpmcfg {
        #[inline(always)]
        fn default() -> Glpmcfg {
            Glpmcfg(0)
        }
    }
    impl core::fmt::Debug for Glpmcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Glpmcfg")
                .field("lpmen", &self.lpmen())
                .field("lpmack", &self.lpmack())
                .field("besl", &self.besl())
                .field("remwake", &self.remwake())
                .field("l1ssen", &self.l1ssen())
                .field("beslthrs", &self.beslthrs())
                .field("l1dsen", &self.l1dsen())
                .field("lpmrst", &self.lpmrst())
                .field("slpsts", &self.slpsts())
                .field("l1rsmok", &self.l1rsmok())
                .field("lpmchidx", &self.lpmchidx())
                .field("lpmrcnt", &self.lpmrcnt())
                .field("sndlpm", &self.sndlpm())
                .field("lpmrcntsts", &self.lpmrcntsts())
                .field("enbesl", &self.enbesl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Glpmcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Glpmcfg {{ lpmen: {=bool:?}, lpmack: {=bool:?}, besl: {=u8:?}, remwake: {=bool:?}, l1ssen: {=bool:?}, beslthrs: {=u8:?}, l1dsen: {=bool:?}, lpmrst: {=u8:?}, slpsts: {=bool:?}, l1rsmok: {=bool:?}, lpmchidx: {=u8:?}, lpmrcnt: {=u8:?}, sndlpm: {=bool:?}, lpmrcntsts: {=u8:?}, enbesl: {=bool:?} }}" , self . lpmen () , self . lpmack () , self . besl () , self . remwake () , self . l1ssen () , self . beslthrs () , self . l1dsen () , self . lpmrst () , self . slpsts () , self . l1rsmok () , self . lpmchidx () , self . lpmrcnt () , self . sndlpm () , self . lpmrcntsts () , self . enbesl ())
        }
    }
    #[doc = "Control and status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gotgctl(pub u32);
    impl Gotgctl {
        #[doc = "Session request success"]
        #[inline(always)]
        pub const fn srqscs(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Session request success"]
        #[inline(always)]
        pub fn set_srqscs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Session request"]
        #[inline(always)]
        pub const fn srq(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Session request"]
        #[inline(always)]
        pub fn set_srq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "VBUS valid override enable"]
        #[inline(always)]
        pub const fn vbvaloen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "VBUS valid override enable"]
        #[inline(always)]
        pub fn set_vbvaloen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "VBUS valid override value"]
        #[inline(always)]
        pub const fn vbvaloval(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "VBUS valid override value"]
        #[inline(always)]
        pub fn set_vbvaloval(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "A-peripheral session valid override enable"]
        #[inline(always)]
        pub const fn avaloen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "A-peripheral session valid override enable"]
        #[inline(always)]
        pub fn set_avaloen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "A-peripheral session valid override value"]
        #[inline(always)]
        pub const fn avaloval(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "A-peripheral session valid override value"]
        #[inline(always)]
        pub fn set_avaloval(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "B-peripheral session valid override enable"]
        #[inline(always)]
        pub const fn bvaloen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "B-peripheral session valid override enable"]
        #[inline(always)]
        pub fn set_bvaloen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "B-peripheral session valid override value"]
        #[inline(always)]
        pub const fn bvaloval(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "B-peripheral session valid override value"]
        #[inline(always)]
        pub fn set_bvaloval(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Host negotiation success"]
        #[inline(always)]
        pub const fn hngscs(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Host negotiation success"]
        #[inline(always)]
        pub fn set_hngscs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "HNP request"]
        #[inline(always)]
        pub const fn hnprq(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "HNP request"]
        #[inline(always)]
        pub fn set_hnprq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Host set HNP enable"]
        #[inline(always)]
        pub const fn hshnpen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Host set HNP enable"]
        #[inline(always)]
        pub fn set_hshnpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Device HNP enabled"]
        #[inline(always)]
        pub const fn dhnpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Device HNP enabled"]
        #[inline(always)]
        pub fn set_dhnpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Embedded host enable"]
        #[inline(always)]
        pub const fn ehen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Embedded host enable"]
        #[inline(always)]
        pub fn set_ehen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Connector ID status"]
        #[inline(always)]
        pub const fn cidsts(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Connector ID status"]
        #[inline(always)]
        pub fn set_cidsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Long/short debounce time"]
        #[inline(always)]
        pub const fn dbct(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Long/short debounce time"]
        #[inline(always)]
        pub fn set_dbct(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "A-session valid"]
        #[inline(always)]
        pub const fn asvld(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "A-session valid"]
        #[inline(always)]
        pub fn set_asvld(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "B-session valid"]
        #[inline(always)]
        pub const fn bsvld(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "B-session valid"]
        #[inline(always)]
        pub fn set_bsvld(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for Gotgctl {
        #[inline(always)]
        fn default() -> Gotgctl {
            Gotgctl(0)
        }
    }
    impl core::fmt::Debug for Gotgctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gotgctl")
                .field("srqscs", &self.srqscs())
                .field("srq", &self.srq())
                .field("vbvaloen", &self.vbvaloen())
                .field("vbvaloval", &self.vbvaloval())
                .field("avaloen", &self.avaloen())
                .field("avaloval", &self.avaloval())
                .field("bvaloen", &self.bvaloen())
                .field("bvaloval", &self.bvaloval())
                .field("hngscs", &self.hngscs())
                .field("hnprq", &self.hnprq())
                .field("hshnpen", &self.hshnpen())
                .field("dhnpen", &self.dhnpen())
                .field("ehen", &self.ehen())
                .field("cidsts", &self.cidsts())
                .field("dbct", &self.dbct())
                .field("asvld", &self.asvld())
                .field("bsvld", &self.bsvld())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gotgctl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Gotgctl {{ srqscs: {=bool:?}, srq: {=bool:?}, vbvaloen: {=bool:?}, vbvaloval: {=bool:?}, avaloen: {=bool:?}, avaloval: {=bool:?}, bvaloen: {=bool:?}, bvaloval: {=bool:?}, hngscs: {=bool:?}, hnprq: {=bool:?}, hshnpen: {=bool:?}, dhnpen: {=bool:?}, ehen: {=bool:?}, cidsts: {=bool:?}, dbct: {=bool:?}, asvld: {=bool:?}, bsvld: {=bool:?} }}" , self . srqscs () , self . srq () , self . vbvaloen () , self . vbvaloval () , self . avaloen () , self . avaloval () , self . bvaloen () , self . bvaloval () , self . hngscs () , self . hnprq () , self . hshnpen () , self . dhnpen () , self . ehen () , self . cidsts () , self . dbct () , self . asvld () , self . bsvld ())
        }
    }
    #[doc = "Interrupt register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gotgint(pub u32);
    impl Gotgint {
        #[doc = "Session end detected"]
        #[inline(always)]
        pub const fn sedet(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Session end detected"]
        #[inline(always)]
        pub fn set_sedet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Session request success status change"]
        #[inline(always)]
        pub const fn srsschg(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Session request success status change"]
        #[inline(always)]
        pub fn set_srsschg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Host negotiation success status change"]
        #[inline(always)]
        pub const fn hnsschg(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Host negotiation success status change"]
        #[inline(always)]
        pub fn set_hnsschg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Host negotiation detected"]
        #[inline(always)]
        pub const fn hngdet(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Host negotiation detected"]
        #[inline(always)]
        pub fn set_hngdet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "A-device timeout change"]
        #[inline(always)]
        pub const fn adtochg(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "A-device timeout change"]
        #[inline(always)]
        pub fn set_adtochg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Debounce done"]
        #[inline(always)]
        pub const fn dbcdne(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Debounce done"]
        #[inline(always)]
        pub fn set_dbcdne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "ID input pin changed"]
        #[inline(always)]
        pub const fn idchng(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "ID input pin changed"]
        #[inline(always)]
        pub fn set_idchng(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Gotgint {
        #[inline(always)]
        fn default() -> Gotgint {
            Gotgint(0)
        }
    }
    impl core::fmt::Debug for Gotgint {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gotgint")
                .field("sedet", &self.sedet())
                .field("srsschg", &self.srsschg())
                .field("hnsschg", &self.hnsschg())
                .field("hngdet", &self.hngdet())
                .field("adtochg", &self.adtochg())
                .field("dbcdne", &self.dbcdne())
                .field("idchng", &self.idchng())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gotgint {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Gotgint {{ sedet: {=bool:?}, srsschg: {=bool:?}, hnsschg: {=bool:?}, hngdet: {=bool:?}, adtochg: {=bool:?}, dbcdne: {=bool:?}, idchng: {=bool:?} }}" , self . sedet () , self . srsschg () , self . hnsschg () , self . hngdet () , self . adtochg () , self . dbcdne () , self . idchng ())
        }
    }
    #[doc = "Reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Grstctl(pub u32);
    impl Grstctl {
        #[doc = "Core soft reset"]
        #[inline(always)]
        pub const fn csrst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Core soft reset"]
        #[inline(always)]
        pub fn set_csrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "HCLK soft reset"]
        #[inline(always)]
        pub const fn hsrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "HCLK soft reset"]
        #[inline(always)]
        pub fn set_hsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Host frame counter reset"]
        #[inline(always)]
        pub const fn fcrst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Host frame counter reset"]
        #[inline(always)]
        pub fn set_fcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "RxFIFO flush"]
        #[inline(always)]
        pub const fn rxfflsh(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RxFIFO flush"]
        #[inline(always)]
        pub fn set_rxfflsh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TxFIFO flush"]
        #[inline(always)]
        pub const fn txfflsh(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TxFIFO flush"]
        #[inline(always)]
        pub fn set_txfflsh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TxFIFO number"]
        #[inline(always)]
        pub const fn txfnum(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x1f;
            val as u8
        }
        #[doc = "TxFIFO number"]
        #[inline(always)]
        pub fn set_txfnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
        }
        #[doc = "DMA request signal enabled for USB OTG HS"]
        #[inline(always)]
        pub const fn dmareq(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "DMA request signal enabled for USB OTG HS"]
        #[inline(always)]
        pub fn set_dmareq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "AHB master idle"]
        #[inline(always)]
        pub const fn ahbidl(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "AHB master idle"]
        #[inline(always)]
        pub fn set_ahbidl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Grstctl {
        #[inline(always)]
        fn default() -> Grstctl {
            Grstctl(0)
        }
    }
    impl core::fmt::Debug for Grstctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Grstctl")
                .field("csrst", &self.csrst())
                .field("hsrst", &self.hsrst())
                .field("fcrst", &self.fcrst())
                .field("rxfflsh", &self.rxfflsh())
                .field("txfflsh", &self.txfflsh())
                .field("txfnum", &self.txfnum())
                .field("dmareq", &self.dmareq())
                .field("ahbidl", &self.ahbidl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Grstctl {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Grstctl {{ csrst: {=bool:?}, hsrst: {=bool:?}, fcrst: {=bool:?}, rxfflsh: {=bool:?}, txfflsh: {=bool:?}, txfnum: {=u8:?}, dmareq: {=bool:?}, ahbidl: {=bool:?} }}" , self . csrst () , self . hsrst () , self . fcrst () , self . rxfflsh () , self . txfflsh () , self . txfnum () , self . dmareq () , self . ahbidl ())
        }
    }
    #[doc = "Receive FIFO size register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Grxfsiz(pub u32);
    impl Grxfsiz {
        #[doc = "RxFIFO depth"]
        #[inline(always)]
        pub const fn rxfd(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "RxFIFO depth"]
        #[inline(always)]
        pub fn set_rxfd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Grxfsiz {
        #[inline(always)]
        fn default() -> Grxfsiz {
            Grxfsiz(0)
        }
    }
    impl core::fmt::Debug for Grxfsiz {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Grxfsiz").field("rxfd", &self.rxfd()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Grxfsiz {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Grxfsiz {{ rxfd: {=u16:?} }}", self.rxfd())
        }
    }
    #[doc = "Status read and pop register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Grxsts(pub u32);
    impl Grxsts {
        #[doc = "Endpoint number (device mode) / Channel number (host mode)"]
        #[inline(always)]
        pub const fn epnum(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Endpoint number (device mode) / Channel number (host mode)"]
        #[inline(always)]
        pub fn set_epnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Byte count"]
        #[inline(always)]
        pub const fn bcnt(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x07ff;
            val as u16
        }
        #[doc = "Byte count"]
        #[inline(always)]
        pub fn set_bcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 4usize)) | (((val as u32) & 0x07ff) << 4usize);
        }
        #[doc = "Data PID"]
        #[inline(always)]
        pub const fn dpid(&self) -> super::vals::Dpid {
            let val = (self.0 >> 15usize) & 0x03;
            super::vals::Dpid::from_bits(val as u8)
        }
        #[doc = "Data PID"]
        #[inline(always)]
        pub fn set_dpid(&mut self, val: super::vals::Dpid) {
            self.0 = (self.0 & !(0x03 << 15usize)) | (((val.to_bits() as u32) & 0x03) << 15usize);
        }
        #[doc = "Packet status (device mode)"]
        #[inline(always)]
        pub const fn pktstsd(&self) -> super::vals::Pktstsd {
            let val = (self.0 >> 17usize) & 0x0f;
            super::vals::Pktstsd::from_bits(val as u8)
        }
        #[doc = "Packet status (device mode)"]
        #[inline(always)]
        pub fn set_pktstsd(&mut self, val: super::vals::Pktstsd) {
            self.0 = (self.0 & !(0x0f << 17usize)) | (((val.to_bits() as u32) & 0x0f) << 17usize);
        }
        #[doc = "Packet status (host mode)"]
        #[inline(always)]
        pub const fn pktstsh(&self) -> super::vals::Pktstsh {
            let val = (self.0 >> 17usize) & 0x0f;
            super::vals::Pktstsh::from_bits(val as u8)
        }
        #[doc = "Packet status (host mode)"]
        #[inline(always)]
        pub fn set_pktstsh(&mut self, val: super::vals::Pktstsh) {
            self.0 = (self.0 & !(0x0f << 17usize)) | (((val.to_bits() as u32) & 0x0f) << 17usize);
        }
        #[doc = "Frame number (device mode)"]
        #[inline(always)]
        pub const fn frmnum(&self) -> u8 {
            let val = (self.0 >> 21usize) & 0x0f;
            val as u8
        }
        #[doc = "Frame number (device mode)"]
        #[inline(always)]
        pub fn set_frmnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 21usize)) | (((val as u32) & 0x0f) << 21usize);
        }
    }
    impl Default for Grxsts {
        #[inline(always)]
        fn default() -> Grxsts {
            Grxsts(0)
        }
    }
    impl core::fmt::Debug for Grxsts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Grxsts")
                .field("epnum", &self.epnum())
                .field("bcnt", &self.bcnt())
                .field("dpid", &self.dpid())
                .field("pktstsd", &self.pktstsd())
                .field("pktstsh", &self.pktstsh())
                .field("frmnum", &self.frmnum())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Grxsts {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Grxsts {{ epnum: {=u8:?}, bcnt: {=u16:?}, dpid: {:?}, pktstsd: {:?}, pktstsh: {:?}, frmnum: {=u8:?} }}" , self . epnum () , self . bcnt () , self . dpid () , self . pktstsd () , self . pktstsh () , self . frmnum ())
        }
    }
    #[doc = "USB configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gusbcfg(pub u32);
    impl Gusbcfg {
        #[doc = "FS timeout calibration"]
        #[inline(always)]
        pub const fn tocal(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "FS timeout calibration"]
        #[inline(always)]
        pub fn set_tocal(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Full-speed internal serial transceiver enable"]
        #[inline(always)]
        pub const fn physel(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Full-speed internal serial transceiver enable"]
        #[inline(always)]
        pub fn set_physel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "SRP-capable"]
        #[inline(always)]
        pub const fn srpcap(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRP-capable"]
        #[inline(always)]
        pub fn set_srpcap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "HNP-capable"]
        #[inline(always)]
        pub const fn hnpcap(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "HNP-capable"]
        #[inline(always)]
        pub fn set_hnpcap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "USB turnaround time"]
        #[inline(always)]
        pub const fn trdt(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x0f;
            val as u8
        }
        #[doc = "USB turnaround time"]
        #[inline(always)]
        pub fn set_trdt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 10usize)) | (((val as u32) & 0x0f) << 10usize);
        }
        #[doc = "PHY Low-power clock select"]
        #[inline(always)]
        pub const fn phylpcs(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "PHY Low-power clock select"]
        #[inline(always)]
        pub fn set_phylpcs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "ULPI FS/LS select"]
        #[inline(always)]
        pub const fn ulpifsls(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "ULPI FS/LS select"]
        #[inline(always)]
        pub fn set_ulpifsls(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "ULPI Auto-resume"]
        #[inline(always)]
        pub const fn ulpiar(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "ULPI Auto-resume"]
        #[inline(always)]
        pub fn set_ulpiar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "ULPI Clock SuspendM"]
        #[inline(always)]
        pub const fn ulpicsm(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "ULPI Clock SuspendM"]
        #[inline(always)]
        pub fn set_ulpicsm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "ULPI External VBUS Drive"]
        #[inline(always)]
        pub const fn ulpievbusd(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "ULPI External VBUS Drive"]
        #[inline(always)]
        pub fn set_ulpievbusd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "ULPI external VBUS indicator"]
        #[inline(always)]
        pub const fn ulpievbusi(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "ULPI external VBUS indicator"]
        #[inline(always)]
        pub fn set_ulpievbusi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "TermSel DLine pulsing selection"]
        #[inline(always)]
        pub const fn tsdps(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "TermSel DLine pulsing selection"]
        #[inline(always)]
        pub fn set_tsdps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Indicator complement"]
        #[inline(always)]
        pub const fn pcci(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Indicator complement"]
        #[inline(always)]
        pub fn set_pcci(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Indicator pass through"]
        #[inline(always)]
        pub const fn ptci(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Indicator pass through"]
        #[inline(always)]
        pub fn set_ptci(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "ULPI interface protect disable"]
        #[inline(always)]
        pub const fn ulpiipd(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "ULPI interface protect disable"]
        #[inline(always)]
        pub fn set_ulpiipd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Force host mode"]
        #[inline(always)]
        pub const fn fhmod(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Force host mode"]
        #[inline(always)]
        pub fn set_fhmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Force device mode"]
        #[inline(always)]
        pub const fn fdmod(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Force device mode"]
        #[inline(always)]
        pub fn set_fdmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Corrupt Tx packet"]
        #[inline(always)]
        pub const fn ctxpkt(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Corrupt Tx packet"]
        #[inline(always)]
        pub fn set_ctxpkt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Gusbcfg {
        #[inline(always)]
        fn default() -> Gusbcfg {
            Gusbcfg(0)
        }
    }
    impl core::fmt::Debug for Gusbcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gusbcfg")
                .field("tocal", &self.tocal())
                .field("physel", &self.physel())
                .field("srpcap", &self.srpcap())
                .field("hnpcap", &self.hnpcap())
                .field("trdt", &self.trdt())
                .field("phylpcs", &self.phylpcs())
                .field("ulpifsls", &self.ulpifsls())
                .field("ulpiar", &self.ulpiar())
                .field("ulpicsm", &self.ulpicsm())
                .field("ulpievbusd", &self.ulpievbusd())
                .field("ulpievbusi", &self.ulpievbusi())
                .field("tsdps", &self.tsdps())
                .field("pcci", &self.pcci())
                .field("ptci", &self.ptci())
                .field("ulpiipd", &self.ulpiipd())
                .field("fhmod", &self.fhmod())
                .field("fdmod", &self.fdmod())
                .field("ctxpkt", &self.ctxpkt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gusbcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Gusbcfg {{ tocal: {=u8:?}, physel: {=bool:?}, srpcap: {=bool:?}, hnpcap: {=bool:?}, trdt: {=u8:?}, phylpcs: {=bool:?}, ulpifsls: {=bool:?}, ulpiar: {=bool:?}, ulpicsm: {=bool:?}, ulpievbusd: {=bool:?}, ulpievbusi: {=bool:?}, tsdps: {=bool:?}, pcci: {=bool:?}, ptci: {=bool:?}, ulpiipd: {=bool:?}, fhmod: {=bool:?}, fdmod: {=bool:?}, ctxpkt: {=bool:?} }}" , self . tocal () , self . physel () , self . srpcap () , self . hnpcap () , self . trdt () , self . phylpcs () , self . ulpifsls () , self . ulpiar () , self . ulpicsm () , self . ulpievbusd () , self . ulpievbusi () , self . tsdps () , self . pcci () , self . ptci () , self . ulpiipd () , self . fhmod () , self . fdmod () , self . ctxpkt ())
        }
    }
    #[doc = "Host all channels interrupt register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Haint(pub u32);
    impl Haint {
        #[doc = "Channel interrupts"]
        #[inline(always)]
        pub const fn haint(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Channel interrupts"]
        #[inline(always)]
        pub fn set_haint(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Haint {
        #[inline(always)]
        fn default() -> Haint {
            Haint(0)
        }
    }
    impl core::fmt::Debug for Haint {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Haint").field("haint", &self.haint()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Haint {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Haint {{ haint: {=u16:?} }}", self.haint())
        }
    }
    #[doc = "Host all channels interrupt mask register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Haintmsk(pub u32);
    impl Haintmsk {
        #[doc = "Channel interrupt mask"]
        #[inline(always)]
        pub const fn haintm(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Channel interrupt mask"]
        #[inline(always)]
        pub fn set_haintm(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Haintmsk {
        #[inline(always)]
        fn default() -> Haintmsk {
            Haintmsk(0)
        }
    }
    impl core::fmt::Debug for Haintmsk {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Haintmsk").field("haintm", &self.haintm()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Haintmsk {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Haintmsk {{ haintm: {=u16:?} }}", self.haintm())
        }
    }
    #[doc = "Host channel characteristics register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcchar(pub u32);
    impl Hcchar {
        #[doc = "Maximum packet size"]
        #[inline(always)]
        pub const fn mpsiz(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Maximum packet size"]
        #[inline(always)]
        pub fn set_mpsiz(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Endpoint number"]
        #[inline(always)]
        pub const fn epnum(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x0f;
            val as u8
        }
        #[doc = "Endpoint number"]
        #[inline(always)]
        pub fn set_epnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
        }
        #[doc = "Endpoint direction"]
        #[inline(always)]
        pub const fn epdir(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint direction"]
        #[inline(always)]
        pub fn set_epdir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Low-speed device"]
        #[inline(always)]
        pub const fn lsdev(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Low-speed device"]
        #[inline(always)]
        pub fn set_lsdev(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Endpoint type"]
        #[inline(always)]
        pub const fn eptyp(&self) -> super::vals::Eptyp {
            let val = (self.0 >> 18usize) & 0x03;
            super::vals::Eptyp::from_bits(val as u8)
        }
        #[doc = "Endpoint type"]
        #[inline(always)]
        pub fn set_eptyp(&mut self, val: super::vals::Eptyp) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
        }
        #[doc = "Multicount"]
        #[inline(always)]
        pub const fn mcnt(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Multicount"]
        #[inline(always)]
        pub fn set_mcnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "Device address"]
        #[inline(always)]
        pub const fn dad(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x7f;
            val as u8
        }
        #[doc = "Device address"]
        #[inline(always)]
        pub fn set_dad(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 22usize)) | (((val as u32) & 0x7f) << 22usize);
        }
        #[doc = "Odd frame (request iso/interrupt transaction to be performed on odd micro-frame)"]
        #[inline(always)]
        pub const fn oddfrm(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Odd frame (request iso/interrupt transaction to be performed on odd micro-frame)"]
        #[inline(always)]
        pub fn set_oddfrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Channel disable"]
        #[inline(always)]
        pub const fn chdis(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Channel disable"]
        #[inline(always)]
        pub fn set_chdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Channel enable"]
        #[inline(always)]
        pub const fn chena(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Channel enable"]
        #[inline(always)]
        pub fn set_chena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hcchar {
        #[inline(always)]
        fn default() -> Hcchar {
            Hcchar(0)
        }
    }
    impl core::fmt::Debug for Hcchar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcchar")
                .field("mpsiz", &self.mpsiz())
                .field("epnum", &self.epnum())
                .field("epdir", &self.epdir())
                .field("lsdev", &self.lsdev())
                .field("eptyp", &self.eptyp())
                .field("mcnt", &self.mcnt())
                .field("dad", &self.dad())
                .field("oddfrm", &self.oddfrm())
                .field("chdis", &self.chdis())
                .field("chena", &self.chena())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hcchar {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Hcchar {{ mpsiz: {=u16:?}, epnum: {=u8:?}, epdir: {=bool:?}, lsdev: {=bool:?}, eptyp: {:?}, mcnt: {=u8:?}, dad: {=u8:?}, oddfrm: {=bool:?}, chdis: {=bool:?}, chena: {=bool:?} }}" , self . mpsiz () , self . epnum () , self . epdir () , self . lsdev () , self . eptyp () , self . mcnt () , self . dad () , self . oddfrm () , self . chdis () , self . chena ())
        }
    }
    #[doc = "Host channel DMA config register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcdma(pub u32);
    impl Hcdma {
        #[doc = "QTD list base address"]
        #[inline(always)]
        pub const fn qtdaddr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QTD list base address"]
        #[inline(always)]
        pub fn set_qtdaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
        #[doc = "Current QTD (transfer descriptor) index"]
        #[inline(always)]
        pub const fn cqtd(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x3f;
            val as u8
        }
        #[doc = "Current QTD (transfer descriptor) index"]
        #[inline(always)]
        pub fn set_cqtd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 3usize)) | (((val as u32) & 0x3f) << 3usize);
        }
    }
    impl Default for Hcdma {
        #[inline(always)]
        fn default() -> Hcdma {
            Hcdma(0)
        }
    }
    impl core::fmt::Debug for Hcdma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcdma")
                .field("qtdaddr", &self.qtdaddr())
                .field("cqtd", &self.cqtd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hcdma {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hcdma {{ qtdaddr: {=u32:?}, cqtd: {=u8:?} }}",
                self.qtdaddr(),
                self.cqtd()
            )
        }
    }
    #[doc = "Host configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcfg(pub u32);
    impl Hcfg {
        #[doc = "FS/LS PHY clock select"]
        #[inline(always)]
        pub const fn fslspcs(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "FS/LS PHY clock select"]
        #[inline(always)]
        pub fn set_fslspcs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "FS- and LS-only support"]
        #[inline(always)]
        pub const fn fslss(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "FS- and LS-only support"]
        #[inline(always)]
        pub fn set_fslss(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Descriptor DMA-mode enable (qtd)"]
        #[inline(always)]
        pub const fn descdma(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Descriptor DMA-mode enable (qtd)"]
        #[inline(always)]
        pub fn set_descdma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Frame list length"]
        #[inline(always)]
        pub const fn frlistlen(&self) -> super::vals::Frlistlen {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Frlistlen::from_bits(val as u8)
        }
        #[doc = "Frame list length"]
        #[inline(always)]
        pub fn set_frlistlen(&mut self, val: super::vals::Frlistlen) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "Period scheduling enable"]
        #[inline(always)]
        pub const fn perschedena(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Period scheduling enable"]
        #[inline(always)]
        pub fn set_perschedena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Hcfg {
        #[inline(always)]
        fn default() -> Hcfg {
            Hcfg(0)
        }
    }
    impl core::fmt::Debug for Hcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcfg")
                .field("fslspcs", &self.fslspcs())
                .field("fslss", &self.fslss())
                .field("descdma", &self.descdma())
                .field("frlistlen", &self.frlistlen())
                .field("perschedena", &self.perschedena())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Hcfg {{ fslspcs: {=u8:?}, fslss: {=bool:?}, descdma: {=bool:?}, frlistlen: {:?}, perschedena: {=bool:?} }}" , self . fslspcs () , self . fslss () , self . descdma () , self . frlistlen () , self . perschedena ())
        }
    }
    #[doc = "Host channel interrupt register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcint(pub u32);
    impl Hcint {
        #[doc = "Transfer completed"]
        #[inline(always)]
        pub const fn xfrc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer completed"]
        #[inline(always)]
        pub fn set_xfrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Channel halted"]
        #[inline(always)]
        pub const fn chh(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Channel halted"]
        #[inline(always)]
        pub fn set_chh(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "STALL response received interrupt"]
        #[inline(always)]
        pub const fn stall(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "STALL response received interrupt"]
        #[inline(always)]
        pub fn set_stall(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "NAK response received interrupt"]
        #[inline(always)]
        pub const fn nak(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "NAK response received interrupt"]
        #[inline(always)]
        pub fn set_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ACK response received/transmitted interrupt"]
        #[inline(always)]
        pub const fn ack(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ACK response received/transmitted interrupt"]
        #[inline(always)]
        pub fn set_ack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Transaction error"]
        #[inline(always)]
        pub const fn txerr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Transaction error"]
        #[inline(always)]
        pub fn set_txerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Babble error"]
        #[inline(always)]
        pub const fn bberr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Babble error"]
        #[inline(always)]
        pub fn set_bberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Frame overrun"]
        #[inline(always)]
        pub const fn frmor(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Frame overrun"]
        #[inline(always)]
        pub fn set_frmor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Data toggle error"]
        #[inline(always)]
        pub const fn dterr(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Data toggle error"]
        #[inline(always)]
        pub fn set_dterr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcint {
        #[inline(always)]
        fn default() -> Hcint {
            Hcint(0)
        }
    }
    impl core::fmt::Debug for Hcint {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcint")
                .field("xfrc", &self.xfrc())
                .field("chh", &self.chh())
                .field("stall", &self.stall())
                .field("nak", &self.nak())
                .field("ack", &self.ack())
                .field("txerr", &self.txerr())
                .field("bberr", &self.bberr())
                .field("frmor", &self.frmor())
                .field("dterr", &self.dterr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hcint {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Hcint {{ xfrc: {=bool:?}, chh: {=bool:?}, stall: {=bool:?}, nak: {=bool:?}, ack: {=bool:?}, txerr: {=bool:?}, bberr: {=bool:?}, frmor: {=bool:?}, dterr: {=bool:?} }}" , self . xfrc () , self . chh () , self . stall () , self . nak () , self . ack () , self . txerr () , self . bberr () , self . frmor () , self . dterr ())
        }
    }
    #[doc = "Host channel mask register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hcintmsk(pub u32);
    impl Hcintmsk {
        #[doc = "Transfer completed mask"]
        #[inline(always)]
        pub const fn xfrcm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer completed mask"]
        #[inline(always)]
        pub fn set_xfrcm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Channel halted mask"]
        #[inline(always)]
        pub const fn chhm(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Channel halted mask"]
        #[inline(always)]
        pub fn set_chhm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "STALL response received interrupt mask"]
        #[inline(always)]
        pub const fn stallm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "STALL response received interrupt mask"]
        #[inline(always)]
        pub fn set_stallm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "NAK response received interrupt mask"]
        #[inline(always)]
        pub const fn nakm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "NAK response received interrupt mask"]
        #[inline(always)]
        pub fn set_nakm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ACK response received/transmitted interrupt mask"]
        #[inline(always)]
        pub const fn ackm(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ACK response received/transmitted interrupt mask"]
        #[inline(always)]
        pub fn set_ackm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Response received interrupt mask"]
        #[inline(always)]
        pub const fn nyet(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Response received interrupt mask"]
        #[inline(always)]
        pub fn set_nyet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Transaction error mask"]
        #[inline(always)]
        pub const fn txerrm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Transaction error mask"]
        #[inline(always)]
        pub fn set_txerrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Babble error mask"]
        #[inline(always)]
        pub const fn bberrm(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Babble error mask"]
        #[inline(always)]
        pub fn set_bberrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Frame overrun mask"]
        #[inline(always)]
        pub const fn frmorm(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Frame overrun mask"]
        #[inline(always)]
        pub fn set_frmorm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Data toggle error mask"]
        #[inline(always)]
        pub const fn dterrm(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Data toggle error mask"]
        #[inline(always)]
        pub fn set_dterrm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Hcintmsk {
        #[inline(always)]
        fn default() -> Hcintmsk {
            Hcintmsk(0)
        }
    }
    impl core::fmt::Debug for Hcintmsk {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hcintmsk")
                .field("xfrcm", &self.xfrcm())
                .field("chhm", &self.chhm())
                .field("stallm", &self.stallm())
                .field("nakm", &self.nakm())
                .field("ackm", &self.ackm())
                .field("nyet", &self.nyet())
                .field("txerrm", &self.txerrm())
                .field("bberrm", &self.bberrm())
                .field("frmorm", &self.frmorm())
                .field("dterrm", &self.dterrm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hcintmsk {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Hcintmsk {{ xfrcm: {=bool:?}, chhm: {=bool:?}, stallm: {=bool:?}, nakm: {=bool:?}, ackm: {=bool:?}, nyet: {=bool:?}, txerrm: {=bool:?}, bberrm: {=bool:?}, frmorm: {=bool:?}, dterrm: {=bool:?} }}" , self . xfrcm () , self . chhm () , self . stallm () , self . nakm () , self . ackm () , self . nyet () , self . txerrm () , self . bberrm () , self . frmorm () , self . dterrm ())
        }
    }
    #[doc = "Host channel transfer size register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hctsiz(pub u32);
    impl Hctsiz {
        #[doc = "Schedule info for isochronuous & interrupt pipes (xfrsiz\\[7:0\\])"]
        #[inline(always)]
        pub const fn schedinfo(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Schedule info for isochronuous & interrupt pipes (xfrsiz\\[7:0\\])"]
        #[inline(always)]
        pub fn set_schedinfo(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Transfer size for non-isochronuous/interrupt pipes"]
        #[inline(always)]
        pub const fn xfrsiz(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "Transfer size for non-isochronuous/interrupt pipes"]
        #[inline(always)]
        pub fn set_xfrsiz(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
        #[doc = "NTD descriptor list length for isochronuous & interrupt pipes (xfrsiz\\[15:8\\], note val+1 is actual length)"]
        #[inline(always)]
        pub const fn ntdl(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "NTD descriptor list length for isochronuous & interrupt pipes (xfrsiz\\[15:8\\], note val+1 is actual length)"]
        #[inline(always)]
        pub fn set_ntdl(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Packet count"]
        #[inline(always)]
        pub const fn pktcnt(&self) -> u16 {
            let val = (self.0 >> 19usize) & 0x03ff;
            val as u16
        }
        #[doc = "Packet count"]
        #[inline(always)]
        pub fn set_pktcnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 19usize)) | (((val as u32) & 0x03ff) << 19usize);
        }
        #[doc = "Data PID"]
        #[inline(always)]
        pub const fn dpid(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x03;
            val as u8
        }
        #[doc = "Data PID"]
        #[inline(always)]
        pub fn set_dpid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 29usize)) | (((val as u32) & 0x03) << 29usize);
        }
        #[doc = "Do Ping"]
        #[inline(always)]
        pub const fn doping(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Do Ping"]
        #[inline(always)]
        pub fn set_doping(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Hctsiz {
        #[inline(always)]
        fn default() -> Hctsiz {
            Hctsiz(0)
        }
    }
    impl core::fmt::Debug for Hctsiz {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hctsiz")
                .field("schedinfo", &self.schedinfo())
                .field("xfrsiz", &self.xfrsiz())
                .field("ntdl", &self.ntdl())
                .field("pktcnt", &self.pktcnt())
                .field("dpid", &self.dpid())
                .field("doping", &self.doping())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hctsiz {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Hctsiz {{ schedinfo: {=u8:?}, xfrsiz: {=u32:?}, ntdl: {=u8:?}, pktcnt: {=u16:?}, dpid: {=u8:?}, doping: {=bool:?} }}" , self . schedinfo () , self . xfrsiz () , self . ntdl () , self . pktcnt () , self . dpid () , self . doping ())
        }
    }
    #[doc = "Host frame interval register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfir(pub u32);
    impl Hfir {
        #[doc = "Frame interval"]
        #[inline(always)]
        pub const fn frivl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Frame interval"]
        #[inline(always)]
        pub fn set_frivl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Dynamic Loading Control"]
        #[inline(always)]
        pub const fn rldctrl(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Dynamic Loading Control"]
        #[inline(always)]
        pub fn set_rldctrl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Hfir {
        #[inline(always)]
        fn default() -> Hfir {
            Hfir(0)
        }
    }
    impl core::fmt::Debug for Hfir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfir")
                .field("frivl", &self.frivl())
                .field("rldctrl", &self.rldctrl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfir {{ frivl: {=u16:?}, rldctrl: {=bool:?} }}",
                self.frivl(),
                self.rldctrl()
            )
        }
    }
    #[doc = "Host frame number/frame time remaining register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hfnum(pub u32);
    impl Hfnum {
        #[doc = "Frame number"]
        #[inline(always)]
        pub const fn frnum(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Frame number"]
        #[inline(always)]
        pub fn set_frnum(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Frame time remaining"]
        #[inline(always)]
        pub const fn ftrem(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Frame time remaining"]
        #[inline(always)]
        pub fn set_ftrem(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Hfnum {
        #[inline(always)]
        fn default() -> Hfnum {
            Hfnum(0)
        }
    }
    impl core::fmt::Debug for Hfnum {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hfnum")
                .field("frnum", &self.frnum())
                .field("ftrem", &self.ftrem())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hfnum {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hfnum {{ frnum: {=u16:?}, ftrem: {=u16:?} }}",
                self.frnum(),
                self.ftrem()
            )
        }
    }
    #[doc = "Non-periodic transmit FIFO/queue status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hnptxsts(pub u32);
    impl Hnptxsts {
        #[doc = "Non-periodic TxFIFO space available"]
        #[inline(always)]
        pub const fn nptxfsav(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Non-periodic TxFIFO space available"]
        #[inline(always)]
        pub fn set_nptxfsav(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Non-periodic transmit request queue space available"]
        #[inline(always)]
        pub const fn nptqxsav(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Non-periodic transmit request queue space available"]
        #[inline(always)]
        pub fn set_nptqxsav(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Top of the non-periodic transmit request queue"]
        #[inline(always)]
        pub const fn nptxqtop(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x7f;
            val as u8
        }
        #[doc = "Top of the non-periodic transmit request queue"]
        #[inline(always)]
        pub fn set_nptxqtop(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
        }
    }
    impl Default for Hnptxsts {
        #[inline(always)]
        fn default() -> Hnptxsts {
            Hnptxsts(0)
        }
    }
    impl core::fmt::Debug for Hnptxsts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hnptxsts")
                .field("nptxfsav", &self.nptxfsav())
                .field("nptqxsav", &self.nptqxsav())
                .field("nptxqtop", &self.nptxqtop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hnptxsts {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hnptxsts {{ nptxfsav: {=u16:?}, nptqxsav: {=u8:?}, nptxqtop: {=u8:?} }}",
                self.nptxfsav(),
                self.nptqxsav(),
                self.nptxqtop()
            )
        }
    }
    #[doc = "Host port control and status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hprt(pub u32);
    impl Hprt {
        #[doc = "Port connect status"]
        #[inline(always)]
        pub const fn pcsts(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port connect status"]
        #[inline(always)]
        pub fn set_pcsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Port connect detected"]
        #[inline(always)]
        pub const fn pcdet(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port connect detected"]
        #[inline(always)]
        pub fn set_pcdet(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Port enable (W1C)"]
        #[inline(always)]
        pub const fn pena(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Port enable (W1C)"]
        #[inline(always)]
        pub fn set_pena(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Port enable/disable change"]
        #[inline(always)]
        pub const fn penchng(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Port enable/disable change"]
        #[inline(always)]
        pub fn set_penchng(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Port overcurrent active"]
        #[inline(always)]
        pub const fn poca(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Port overcurrent active"]
        #[inline(always)]
        pub fn set_poca(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Port overcurrent change"]
        #[inline(always)]
        pub const fn pocchng(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Port overcurrent change"]
        #[inline(always)]
        pub fn set_pocchng(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Port resume"]
        #[inline(always)]
        pub const fn pres(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Port resume"]
        #[inline(always)]
        pub fn set_pres(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Port suspend"]
        #[inline(always)]
        pub const fn psusp(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Port suspend"]
        #[inline(always)]
        pub fn set_psusp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Port reset"]
        #[inline(always)]
        pub const fn prst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Port reset"]
        #[inline(always)]
        pub fn set_prst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Port line status"]
        #[inline(always)]
        pub const fn plsts(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "Port line status"]
        #[inline(always)]
        pub fn set_plsts(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "Port power"]
        #[inline(always)]
        pub const fn ppwr(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Port power"]
        #[inline(always)]
        pub fn set_ppwr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Port test control"]
        #[inline(always)]
        pub const fn ptctl(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x0f;
            val as u8
        }
        #[doc = "Port test control"]
        #[inline(always)]
        pub fn set_ptctl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 13usize)) | (((val as u32) & 0x0f) << 13usize);
        }
        #[doc = "Port speed"]
        #[inline(always)]
        pub const fn pspd(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x03;
            val as u8
        }
        #[doc = "Port speed"]
        #[inline(always)]
        pub fn set_pspd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
        }
    }
    impl Default for Hprt {
        #[inline(always)]
        fn default() -> Hprt {
            Hprt(0)
        }
    }
    impl core::fmt::Debug for Hprt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hprt")
                .field("pcsts", &self.pcsts())
                .field("pcdet", &self.pcdet())
                .field("pena", &self.pena())
                .field("penchng", &self.penchng())
                .field("poca", &self.poca())
                .field("pocchng", &self.pocchng())
                .field("pres", &self.pres())
                .field("psusp", &self.psusp())
                .field("prst", &self.prst())
                .field("plsts", &self.plsts())
                .field("ppwr", &self.ppwr())
                .field("ptctl", &self.ptctl())
                .field("pspd", &self.pspd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hprt {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Hprt {{ pcsts: {=bool:?}, pcdet: {=bool:?}, pena: {=bool:?}, penchng: {=bool:?}, poca: {=bool:?}, pocchng: {=bool:?}, pres: {=bool:?}, psusp: {=bool:?}, prst: {=bool:?}, plsts: {=u8:?}, ppwr: {=bool:?}, ptctl: {=u8:?}, pspd: {=u8:?} }}" , self . pcsts () , self . pcdet () , self . pena () , self . penchng () , self . poca () , self . pocchng () , self . pres () , self . psusp () , self . prst () , self . plsts () , self . ppwr () , self . ptctl () , self . pspd ())
        }
    }
    #[doc = "Periodic transmit FIFO/queue status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hptxsts(pub u32);
    impl Hptxsts {
        #[doc = "Periodic transmit data FIFO space available"]
        #[inline(always)]
        pub const fn ptxfsavl(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Periodic transmit data FIFO space available"]
        #[inline(always)]
        pub fn set_ptxfsavl(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Periodic transmit request queue space available"]
        #[inline(always)]
        pub const fn ptxqsav(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Periodic transmit request queue space available"]
        #[inline(always)]
        pub fn set_ptxqsav(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Top of the periodic transmit request queue"]
        #[inline(always)]
        pub const fn ptxqtop(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Top of the periodic transmit request queue"]
        #[inline(always)]
        pub fn set_ptxqtop(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Hptxsts {
        #[inline(always)]
        fn default() -> Hptxsts {
            Hptxsts(0)
        }
    }
    impl core::fmt::Debug for Hptxsts {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hptxsts")
                .field("ptxfsavl", &self.ptxfsavl())
                .field("ptxqsav", &self.ptxqsav())
                .field("ptxqtop", &self.ptxqtop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hptxsts {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hptxsts {{ ptxfsavl: {=u16:?}, ptxqsav: {=u8:?}, ptxqtop: {=u8:?} }}",
                self.ptxfsavl(),
                self.ptxqsav(),
                self.ptxqtop()
            )
        }
    }
    #[doc = "Power and clock gating control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcgcctl(pub u32);
    impl Pcgcctl {
        #[doc = "Stop PHY clock"]
        #[inline(always)]
        pub const fn stppclk(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Stop PHY clock"]
        #[inline(always)]
        pub fn set_stppclk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Gate HCLK"]
        #[inline(always)]
        pub const fn gatehclk(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Gate HCLK"]
        #[inline(always)]
        pub fn set_gatehclk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PHY Suspended"]
        #[inline(always)]
        pub const fn physusp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PHY Suspended"]
        #[inline(always)]
        pub fn set_physusp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Pcgcctl {
        #[inline(always)]
        fn default() -> Pcgcctl {
            Pcgcctl(0)
        }
    }
    impl core::fmt::Debug for Pcgcctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcgcctl")
                .field("stppclk", &self.stppclk())
                .field("gatehclk", &self.gatehclk())
                .field("physusp", &self.physusp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pcgcctl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pcgcctl {{ stppclk: {=bool:?}, gatehclk: {=bool:?}, physusp: {=bool:?} }}",
                self.stppclk(),
                self.gatehclk(),
                self.physusp()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dpid {
        DATA0 = 0x0,
        DATA2 = 0x01,
        DATA1 = 0x02,
        MDATA = 0x03,
    }
    impl Dpid {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dpid {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dpid {
        #[inline(always)]
        fn from(val: u8) -> Dpid {
            Dpid::from_bits(val)
        }
    }
    impl From<Dpid> for u8 {
        #[inline(always)]
        fn from(val: Dpid) -> u8 {
            Dpid::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dspd {
        #[doc = "High speed"]
        HIGH_SPEED = 0x0,
        #[doc = "Full speed using external ULPI PHY"]
        FULL_SPEED_EXTERNAL = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "Full speed using internal embedded PHY"]
        FULL_SPEED_INTERNAL = 0x03,
    }
    impl Dspd {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dspd {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dspd {
        #[inline(always)]
        fn from(val: u8) -> Dspd {
            Dspd::from_bits(val)
        }
    }
    impl From<Dspd> for u8 {
        #[inline(always)]
        fn from(val: Dspd) -> u8 {
            Dspd::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Eptyp {
        CONTROL = 0x0,
        ISOCHRONOUS = 0x01,
        BULK = 0x02,
        INTERRUPT = 0x03,
    }
    impl Eptyp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Eptyp {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Eptyp {
        #[inline(always)]
        fn from(val: u8) -> Eptyp {
            Eptyp::from_bits(val)
        }
    }
    impl From<Eptyp> for u8 {
        #[inline(always)]
        fn from(val: Eptyp) -> u8 {
            Eptyp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Frlistlen {
        #[doc = "Length = 8"]
        LEN8 = 0x0,
        #[doc = "Length = 16"]
        LEN16 = 0x01,
        #[doc = "Length = 32"]
        LEN32 = 0x02,
        #[doc = "Length = 64"]
        LEN64 = 0x03,
    }
    impl Frlistlen {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Frlistlen {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Frlistlen {
        #[inline(always)]
        fn from(val: u8) -> Frlistlen {
            Frlistlen::from_bits(val)
        }
    }
    impl From<Frlistlen> for u8 {
        #[inline(always)]
        fn from(val: Frlistlen) -> u8 {
            Frlistlen::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pfivl {
        #[doc = "80% of the frame interval"]
        FRAME_INTERVAL_80 = 0x0,
        #[doc = "85% of the frame interval"]
        FRAME_INTERVAL_85 = 0x01,
        #[doc = "90% of the frame interval"]
        FRAME_INTERVAL_90 = 0x02,
        #[doc = "95% of the frame interval"]
        FRAME_INTERVAL_95 = 0x03,
    }
    impl Pfivl {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pfivl {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pfivl {
        #[inline(always)]
        fn from(val: u8) -> Pfivl {
            Pfivl::from_bits(val)
        }
    }
    impl From<Pfivl> for u8 {
        #[inline(always)]
        fn from(val: Pfivl) -> u8 {
            Pfivl::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pktstsd {
        _RESERVED_0 = 0x0,
        #[doc = "Global OUT NAK (triggers an interrupt)"]
        OUT_NAK = 0x01,
        #[doc = "OUT data packet received"]
        OUT_DATA_RX = 0x02,
        #[doc = "OUT transfer completed (triggers an interrupt)"]
        OUT_DATA_DONE = 0x03,
        #[doc = "SETUP transaction completed (triggers an interrupt)"]
        SETUP_DATA_DONE = 0x04,
        _RESERVED_5 = 0x05,
        #[doc = "SETUP data packet received"]
        SETUP_DATA_RX = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Pktstsd {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pktstsd {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pktstsd {
        #[inline(always)]
        fn from(val: u8) -> Pktstsd {
            Pktstsd::from_bits(val)
        }
    }
    impl From<Pktstsd> for u8 {
        #[inline(always)]
        fn from(val: Pktstsd) -> u8 {
            Pktstsd::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pktstsh {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "IN data packet received"]
        IN_DATA_RX = 0x02,
        #[doc = "IN transfer completed (triggers an interrupt)"]
        IN_DATA_DONE = 0x03,
        _RESERVED_4 = 0x04,
        #[doc = "Data toggle error (triggers an interrupt)"]
        DATA_TOGGLE_ERR = 0x05,
        _RESERVED_6 = 0x06,
        #[doc = "Channel halted (triggers an interrupt)"]
        CHANNEL_HALTED = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Pktstsh {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pktstsh {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pktstsh {
        #[inline(always)]
        fn from(val: u8) -> Pktstsh {
            Pktstsh::from_bits(val)
        }
    }
    impl From<Pktstsh> for u8 {
        #[inline(always)]
        fn from(val: Pktstsh) -> u8 {
            Pktstsh::to_bits(val)
        }
    }
}
