#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "CSI-2 Host."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csi {
    ptr: *mut u8,
}
unsafe impl Send for Csi {}
unsafe impl Sync for Csi {}
impl Csi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "CSI-2 Host control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "CSI-2 Host DPHY_RX control register."]
    #[inline(always)]
    pub const fn pcr(self) -> crate::common::Reg<regs::Pcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "CSI-2 Host virtual channel 0 configuration register 1."]
    #[inline(always)]
    pub const fn vc0cfgr1(self) -> crate::common::Reg<regs::Vc0cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "CSI-2 Host virtual channel 0 configuration register 2."]
    #[inline(always)]
    pub const fn vc0cfgr2(self) -> crate::common::Reg<regs::Vc0cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "CSI-2 Host virtual channel 0 configuration register 3."]
    #[inline(always)]
    pub const fn vc0cfgr3(self) -> crate::common::Reg<regs::Vc0cfgr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "CSI-2 Host virtual channel 0 configuration register 4."]
    #[inline(always)]
    pub const fn vc0cfgr4(self) -> crate::common::Reg<regs::Vc0cfgr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "CSI-2 Host virtual channel 1 configuration register 1."]
    #[inline(always)]
    pub const fn vc1cfgr1(self) -> crate::common::Reg<regs::Vc1cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "CSI-2 Host virtual channel 1 configuration register 2."]
    #[inline(always)]
    pub const fn vc1cfgr2(self) -> crate::common::Reg<regs::Vc1cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "CSI-2 Host virtual channel 1 configuration register 3."]
    #[inline(always)]
    pub const fn vc1cfgr3(self) -> crate::common::Reg<regs::Vc1cfgr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "CSI-2 Host virtual channel 1 configuration register 4."]
    #[inline(always)]
    pub const fn vc1cfgr4(self) -> crate::common::Reg<regs::Vc1cfgr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "CSI-2 Host virtual channel 2 configuration register 1."]
    #[inline(always)]
    pub const fn vc2cfgr1(self) -> crate::common::Reg<regs::Vc2cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "CSI-2 Host virtual channel 2 configuration register 2."]
    #[inline(always)]
    pub const fn vc2cfgr2(self) -> crate::common::Reg<regs::Vc2cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "CSI-2 Host virtual channel 2 configuration register 3."]
    #[inline(always)]
    pub const fn vc2cfgr3(self) -> crate::common::Reg<regs::Vc2cfgr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "CSI-2 Host virtual channel 2 configuration register 4."]
    #[inline(always)]
    pub const fn vc2cfgr4(self) -> crate::common::Reg<regs::Vc2cfgr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "CSI-2 Host virtual channel 3 configuration register 1."]
    #[inline(always)]
    pub const fn vc3cfgr1(self) -> crate::common::Reg<regs::Vc3cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "CSI-2 Host virtual channel 3 configuration register 2."]
    #[inline(always)]
    pub const fn vc3cfgr2(self) -> crate::common::Reg<regs::Vc3cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "CSI-2 Host virtual channel 3 configuration register 3."]
    #[inline(always)]
    pub const fn vc3cfgr3(self) -> crate::common::Reg<regs::Vc3cfgr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "CSI-2 Host virtual channel 3 configuration register 4."]
    #[inline(always)]
    pub const fn vc3cfgr4(self) -> crate::common::Reg<regs::Vc3cfgr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "CSI-2 Host line byte 0 configuration register."]
    #[inline(always)]
    pub const fn lb0cfgr(self) -> crate::common::Reg<regs::Lb0cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "CSI-2 Host line byte 1 configuration register."]
    #[inline(always)]
    pub const fn lb1cfgr(self) -> crate::common::Reg<regs::Lb1cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "CSI-2 Host line byte 2 configuration register."]
    #[inline(always)]
    pub const fn lb2cfgr(self) -> crate::common::Reg<regs::Lb2cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "CSI-2 Host line byte 3 configuration register."]
    #[inline(always)]
    pub const fn lb3cfgr(self) -> crate::common::Reg<regs::Lb3cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "CSI-2 Host timer 0 configuration register."]
    #[inline(always)]
    pub const fn tim0cfgr(self) -> crate::common::Reg<regs::Tim0cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "CSI-2 Host timer 1 configuration register."]
    #[inline(always)]
    pub const fn tim1cfgr(self) -> crate::common::Reg<regs::Tim1cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "CSI-2 Host timer 2 configuration register."]
    #[inline(always)]
    pub const fn tim2cfgr(self) -> crate::common::Reg<regs::Tim2cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "CSI-2 Host timer 3 configuration register."]
    #[inline(always)]
    pub const fn tim3cfgr(self) -> crate::common::Reg<regs::Tim3cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "CSI-2 Host lane merger configuration register."]
    #[inline(always)]
    pub const fn lmcfgr(self) -> crate::common::Reg<regs::Lmcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "CSI-2 Host program interrupt register."]
    #[inline(always)]
    pub const fn prgitr(self) -> crate::common::Reg<regs::Prgitr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "CSI-2 Host watchdog register."]
    #[inline(always)]
    pub const fn wdr(self) -> crate::common::Reg<regs::Wdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "CSI-2 Host interrupt enable register 0."]
    #[inline(always)]
    pub const fn ier0(self) -> crate::common::Reg<regs::Ier0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "CSI-2 Host interrupt enable register 1."]
    #[inline(always)]
    pub const fn ier1(self) -> crate::common::Reg<regs::Ier1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "CSI-2 Host status register 0."]
    #[inline(always)]
    pub const fn sr0(self) -> crate::common::Reg<regs::Sr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "CSI-2 Host status register 1."]
    #[inline(always)]
    pub const fn sr1(self) -> crate::common::Reg<regs::Sr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "CSI-2 Host flag clear register 0."]
    #[inline(always)]
    pub const fn fcr0(self) -> crate::common::Reg<regs::Fcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "CSI-2 Host flag clear register 1."]
    #[inline(always)]
    pub const fn fcr1(self) -> crate::common::Reg<regs::Fcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "CSI-2 Host short packet data field register."]
    #[inline(always)]
    pub const fn spdfr(self) -> crate::common::Reg<regs::Spdfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "CSI-2 Host error register 1."]
    #[inline(always)]
    pub const fn err1(self) -> crate::common::Reg<regs::Err1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "CSI-2 Host error register 2."]
    #[inline(always)]
    pub const fn err2(self) -> crate::common::Reg<regs::Err2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "CSI PHY reset control register."]
    #[inline(always)]
    pub const fn prcr(self) -> crate::common::Reg<regs::Prcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1000usize) as _) }
    }
    #[doc = "CSI PHY mode control register."]
    #[inline(always)]
    pub const fn pmcr(self) -> crate::common::Reg<regs::Pmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1004usize) as _) }
    }
    #[doc = "CSI PHY frequency control register."]
    #[inline(always)]
    pub const fn pfcr(self) -> crate::common::Reg<regs::Pfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1008usize) as _) }
    }
    #[doc = "CSI PHY test control register 0."]
    #[inline(always)]
    pub const fn ptcr0(self) -> crate::common::Reg<regs::Ptcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1010usize) as _) }
    }
    #[doc = "CSI PHY test control register 1."]
    #[inline(always)]
    pub const fn ptcr1(self) -> crate::common::Reg<regs::Ptcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1014usize) as _) }
    }
    #[doc = "CSI PHY test status register."]
    #[inline(always)]
    pub const fn ptsr(self) -> crate::common::Reg<regs::Ptsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1018usize) as _) }
    }
}
pub mod regs {
    #[doc = "CSI-2 Host control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "CSI-2 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn csien(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CSI-2 enable."]
        #[inline(always)]
        pub const fn set_csien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Virtual channel 0 start."]
        #[must_use]
        #[inline(always)]
        pub const fn vc0start(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Virtual channel 0 start."]
        #[inline(always)]
        pub const fn set_vc0start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Virtual channel 0 stop."]
        #[must_use]
        #[inline(always)]
        pub const fn vc0stop(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Virtual channel 0 stop."]
        #[inline(always)]
        pub const fn set_vc0stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Virtual channel 1 start."]
        #[must_use]
        #[inline(always)]
        pub const fn vc1start(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Virtual channel 1 start."]
        #[inline(always)]
        pub const fn set_vc1start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Virtual channel 1 stop."]
        #[must_use]
        #[inline(always)]
        pub const fn vc1stop(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Virtual channel 1 stop."]
        #[inline(always)]
        pub const fn set_vc1stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Virtual channel 2 start."]
        #[must_use]
        #[inline(always)]
        pub const fn vc2start(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Virtual channel 2 start."]
        #[inline(always)]
        pub const fn set_vc2start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Virtual channel 2 stop."]
        #[must_use]
        #[inline(always)]
        pub const fn vc2stop(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Virtual channel 2 stop."]
        #[inline(always)]
        pub const fn set_vc2stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Virtual channel 3 start."]
        #[must_use]
        #[inline(always)]
        pub const fn vc3start(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Virtual channel 3 start."]
        #[inline(always)]
        pub const fn set_vc3start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Virtual channel 3 stop."]
        #[must_use]
        #[inline(always)]
        pub const fn vc3stop(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Virtual channel 3 stop."]
        #[inline(always)]
        pub const fn set_vc3stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
                .field("csien", &self.csien())
                .field("vc0start", &self.vc0start())
                .field("vc0stop", &self.vc0stop())
                .field("vc1start", &self.vc1start())
                .field("vc1stop", &self.vc1stop())
                .field("vc2start", &self.vc2start())
                .field("vc2stop", &self.vc2stop())
                .field("vc3start", &self.vc3start())
                .field("vc3stop", &self.vc3stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ csien: {=bool:?}, vc0start: {=bool:?}, vc0stop: {=bool:?}, vc1start: {=bool:?}, vc1stop: {=bool:?}, vc2start: {=bool:?}, vc2stop: {=bool:?}, vc3start: {=bool:?}, vc3stop: {=bool:?} }}" , self . csien () , self . vc0start () , self . vc0stop () , self . vc1start () , self . vc1stop () , self . vc2start () , self . vc2stop () , self . vc3start () , self . vc3stop ())
        }
    }
    #[doc = "CSI-2 Host error register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Err1(pub u32);
    impl Err1 {
        #[doc = "Data type having a CRC error."]
        #[must_use]
        #[inline(always)]
        pub const fn crcdterr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type having a CRC error."]
        #[inline(always)]
        pub const fn set_crcdterr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Virtual channel having a CRC error."]
        #[must_use]
        #[inline(always)]
        pub const fn crcvcerr(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Virtual channel having a CRC error."]
        #[inline(always)]
        pub const fn set_crcvcerr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "Data type having a corrected ECC error."]
        #[must_use]
        #[inline(always)]
        pub const fn ceccdterr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type having a corrected ECC error."]
        #[inline(always)]
        pub const fn set_ceccdterr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "Virtual channel having a corrected ECC error."]
        #[must_use]
        #[inline(always)]
        pub const fn ceccvcerr(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Virtual channel having a corrected ECC error."]
        #[inline(always)]
        pub const fn set_ceccvcerr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Data type in error."]
        #[must_use]
        #[inline(always)]
        pub const fn iddterr(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type in error."]
        #[inline(always)]
        pub const fn set_iddterr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "Virtual channel having ID error."]
        #[must_use]
        #[inline(always)]
        pub const fn idvcerr(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "Virtual channel having ID error."]
        #[inline(always)]
        pub const fn set_idvcerr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
    }
    impl Default for Err1 {
        #[inline(always)]
        fn default() -> Err1 {
            Err1(0)
        }
    }
    impl core::fmt::Debug for Err1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Err1")
                .field("crcdterr", &self.crcdterr())
                .field("crcvcerr", &self.crcvcerr())
                .field("ceccdterr", &self.ceccdterr())
                .field("ceccvcerr", &self.ceccvcerr())
                .field("iddterr", &self.iddterr())
                .field("idvcerr", &self.idvcerr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Err1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Err1 {{ crcdterr: {=u8:?}, crcvcerr: {=u8:?}, ceccdterr: {=u8:?}, ceccvcerr: {=u8:?}, iddterr: {=u8:?}, idvcerr: {=u8:?} }}" , self . crcdterr () , self . crcvcerr () , self . ceccdterr () , self . ceccvcerr () , self . iddterr () , self . idvcerr ())
        }
    }
    #[doc = "CSI-2 Host error register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Err2(pub u32);
    impl Err2 {
        #[doc = "Data type having a short packet error."]
        #[must_use]
        #[inline(always)]
        pub const fn spktdterr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type having a short packet error."]
        #[inline(always)]
        pub const fn set_spktdterr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Virtual channel having a short packet error."]
        #[must_use]
        #[inline(always)]
        pub const fn spktvcerr(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Virtual channel having a short packet error."]
        #[inline(always)]
        pub const fn set_spktvcerr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "Virtual channel having a watchdog error."]
        #[must_use]
        #[inline(always)]
        pub const fn wdvcerr(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "Virtual channel having a watchdog error."]
        #[inline(always)]
        pub const fn set_wdvcerr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "Virtual channel having synchronization error."]
        #[must_use]
        #[inline(always)]
        pub const fn syncvcerr(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "Virtual channel having synchronization error."]
        #[inline(always)]
        pub const fn set_syncvcerr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
    }
    impl Default for Err2 {
        #[inline(always)]
        fn default() -> Err2 {
            Err2(0)
        }
    }
    impl core::fmt::Debug for Err2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Err2")
                .field("spktdterr", &self.spktdterr())
                .field("spktvcerr", &self.spktvcerr())
                .field("wdvcerr", &self.wdvcerr())
                .field("syncvcerr", &self.syncvcerr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Err2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Err2 {{ spktdterr: {=u8:?}, spktvcerr: {=u8:?}, wdvcerr: {=u8:?}, syncvcerr: {=u8:?} }}",
                self.spktdterr(),
                self.spktvcerr(),
                self.wdvcerr(),
                self.syncvcerr()
            )
        }
    }
    #[doc = "CSI-2 Host flag clear register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fcr0(pub u32);
    impl Fcr0 {
        #[doc = "Clear line/byte counter 0 flag."]
        #[must_use]
        #[inline(always)]
        pub const fn clb0f(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear line/byte counter 0 flag."]
        #[inline(always)]
        pub const fn set_clb0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear line/byte counter 1 flag."]
        #[must_use]
        #[inline(always)]
        pub const fn clb1f(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear line/byte counter 1 flag."]
        #[inline(always)]
        pub const fn set_clb1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Clear line/byte counter 2 flag."]
        #[must_use]
        #[inline(always)]
        pub const fn clb2f(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Clear line/byte counter 2 flag."]
        #[inline(always)]
        pub const fn set_clb2f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Clear line/byte counter 3 flag."]
        #[must_use]
        #[inline(always)]
        pub const fn clb3f(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Clear line/byte counter 3 flag."]
        #[inline(always)]
        pub const fn set_clb3f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Clear timer 0 flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ctim0f(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Clear timer 0 flag."]
        #[inline(always)]
        pub const fn set_ctim0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Clear timer 1 flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ctim1f(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Clear timer 1 flag."]
        #[inline(always)]
        pub const fn set_ctim1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Clear timer 2 flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ctim2f(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Clear timer 2 flag."]
        #[inline(always)]
        pub const fn set_ctim2f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Clear timer 3 flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ctim3f(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Clear timer 3 flag."]
        #[inline(always)]
        pub const fn set_ctim3f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Clear SOF flag for virtual channel 0."]
        #[must_use]
        #[inline(always)]
        pub const fn csof0f(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Clear SOF flag for virtual channel 0."]
        #[inline(always)]
        pub const fn set_csof0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Clear SOF flag for virtual channel 1."]
        #[must_use]
        #[inline(always)]
        pub const fn csof1f(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Clear SOF flag for virtual channel 1."]
        #[inline(always)]
        pub const fn set_csof1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Clear SOF flag for virtual channel 2."]
        #[must_use]
        #[inline(always)]
        pub const fn csof2f(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Clear SOF flag for virtual channel 2."]
        #[inline(always)]
        pub const fn set_csof2f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Clear SOF flag for virtual channel 3."]
        #[must_use]
        #[inline(always)]
        pub const fn csof3f(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Clear SOF flag for virtual channel 3."]
        #[inline(always)]
        pub const fn set_csof3f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Clear EOF flag for virtual channel 0."]
        #[must_use]
        #[inline(always)]
        pub const fn ceof0f(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Clear EOF flag for virtual channel 0."]
        #[inline(always)]
        pub const fn set_ceof0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Clear EOF flag for virtual channel 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ceof1f(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Clear EOF flag for virtual channel 1."]
        #[inline(always)]
        pub const fn set_ceof1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Clear EOF flag for virtual channel 2."]
        #[must_use]
        #[inline(always)]
        pub const fn ceof2f(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Clear EOF flag for virtual channel 2."]
        #[inline(always)]
        pub const fn set_ceof2f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Clear EOF flag for virtual channel 3."]
        #[must_use]
        #[inline(always)]
        pub const fn ceof3f(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Clear EOF flag for virtual channel 3."]
        #[inline(always)]
        pub const fn set_ceof3f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Clear short packet flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cspktf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Clear short packet flag."]
        #[inline(always)]
        pub const fn set_cspktf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Clear clock changer FIFO full flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cccfifoff(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Clear clock changer FIFO full flag."]
        #[inline(always)]
        pub const fn set_cccfifoff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Clear CRC error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ccrcerrf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Clear CRC error flag."]
        #[inline(always)]
        pub const fn set_ccrcerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Clear ECC error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ceccerrf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ECC error flag."]
        #[inline(always)]
        pub const fn set_ceccerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Clear corrected ECC error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cceccerrf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Clear corrected ECC error flag."]
        #[inline(always)]
        pub const fn set_cceccerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Clear data type ID error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ciderrf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Clear data type ID error flag."]
        #[inline(always)]
        pub const fn set_ciderrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Clear short packet error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cspkterrf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Clear short packet error flag."]
        #[inline(always)]
        pub const fn set_cspkterrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Clear watchdog error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cwderrf(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Clear watchdog error flag."]
        #[inline(always)]
        pub const fn set_cwderrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Clear invalid synchronization error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn csyncerrf(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Clear invalid synchronization error flag."]
        #[inline(always)]
        pub const fn set_csyncerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Fcr0 {
        #[inline(always)]
        fn default() -> Fcr0 {
            Fcr0(0)
        }
    }
    impl core::fmt::Debug for Fcr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fcr0")
                .field("clb0f", &self.clb0f())
                .field("clb1f", &self.clb1f())
                .field("clb2f", &self.clb2f())
                .field("clb3f", &self.clb3f())
                .field("ctim0f", &self.ctim0f())
                .field("ctim1f", &self.ctim1f())
                .field("ctim2f", &self.ctim2f())
                .field("ctim3f", &self.ctim3f())
                .field("csof0f", &self.csof0f())
                .field("csof1f", &self.csof1f())
                .field("csof2f", &self.csof2f())
                .field("csof3f", &self.csof3f())
                .field("ceof0f", &self.ceof0f())
                .field("ceof1f", &self.ceof1f())
                .field("ceof2f", &self.ceof2f())
                .field("ceof3f", &self.ceof3f())
                .field("cspktf", &self.cspktf())
                .field("cccfifoff", &self.cccfifoff())
                .field("ccrcerrf", &self.ccrcerrf())
                .field("ceccerrf", &self.ceccerrf())
                .field("cceccerrf", &self.cceccerrf())
                .field("ciderrf", &self.ciderrf())
                .field("cspkterrf", &self.cspkterrf())
                .field("cwderrf", &self.cwderrf())
                .field("csyncerrf", &self.csyncerrf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fcr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Fcr0 {{ clb0f: {=bool:?}, clb1f: {=bool:?}, clb2f: {=bool:?}, clb3f: {=bool:?}, ctim0f: {=bool:?}, ctim1f: {=bool:?}, ctim2f: {=bool:?}, ctim3f: {=bool:?}, csof0f: {=bool:?}, csof1f: {=bool:?}, csof2f: {=bool:?}, csof3f: {=bool:?}, ceof0f: {=bool:?}, ceof1f: {=bool:?}, ceof2f: {=bool:?}, ceof3f: {=bool:?}, cspktf: {=bool:?}, cccfifoff: {=bool:?}, ccrcerrf: {=bool:?}, ceccerrf: {=bool:?}, cceccerrf: {=bool:?}, ciderrf: {=bool:?}, cspkterrf: {=bool:?}, cwderrf: {=bool:?}, csyncerrf: {=bool:?} }}" , self . clb0f () , self . clb1f () , self . clb2f () , self . clb3f () , self . ctim0f () , self . ctim1f () , self . ctim2f () , self . ctim3f () , self . csof0f () , self . csof1f () , self . csof2f () , self . csof3f () , self . ceof0f () , self . ceof1f () , self . ceof2f () , self . ceof3f () , self . cspktf () , self . cccfifoff () , self . ccrcerrf () , self . ceccerrf () , self . cceccerrf () , self . ciderrf () , self . cspkterrf () , self . cwderrf () , self . csyncerrf ())
        }
    }
    #[doc = "CSI-2 Host flag clear register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fcr1(pub u32);
    impl Fcr1 {
        #[doc = "Clear SOT error flag on lane 0."]
        #[must_use]
        #[inline(always)]
        pub const fn cesotdl0f(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear SOT error flag on lane 0."]
        #[inline(always)]
        pub const fn set_cesotdl0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear SOT synchronization error flag on lane 0."]
        #[must_use]
        #[inline(always)]
        pub const fn cesotsyncdl0f(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear SOT synchronization error flag on lane 0."]
        #[inline(always)]
        pub const fn set_cesotsyncdl0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Clear D-PHY_RX lane 0 escape entry error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ceescdl0f(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Clear D-PHY_RX lane 0 escape entry error flag."]
        #[inline(always)]
        pub const fn set_ceescdl0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Clear D-PHY_RX lane 0 low-power data transmission synchronization error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cesyncescdl0f(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Clear D-PHY_RX lane 0 low-power data transmission synchronization error flag."]
        #[inline(always)]
        pub const fn set_cesyncescdl0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Clear D-PHY_RX lane 0 control error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cectrldl0f(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Clear D-PHY_RX lane 0 control error flag."]
        #[inline(always)]
        pub const fn set_cectrldl0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Clear SOT error flag on lane 1."]
        #[must_use]
        #[inline(always)]
        pub const fn cesotdl1f(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Clear SOT error flag on lane 1."]
        #[inline(always)]
        pub const fn set_cesotdl1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Clear SOT synchronization error flag on lane 1."]
        #[must_use]
        #[inline(always)]
        pub const fn cesotsyncdl1f(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Clear SOT synchronization error flag on lane 1."]
        #[inline(always)]
        pub const fn set_cesotsyncdl1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Clear D-PHY_RX lane 1 escape entry error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ceescdl1f(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Clear D-PHY_RX lane 1 escape entry error flag."]
        #[inline(always)]
        pub const fn set_ceescdl1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Clear D-PHY_RX lane 1 low-power data transmission synchronization error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cesyncescdl1f(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Clear D-PHY_RX lane 1 low-power data transmission synchronization error flag."]
        #[inline(always)]
        pub const fn set_cesyncescdl1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Clear D-PHY_RX lane 1 control error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cectrldl1f(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Clear D-PHY_RX lane 1 control error flag."]
        #[inline(always)]
        pub const fn set_cectrldl1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Fcr1 {
        #[inline(always)]
        fn default() -> Fcr1 {
            Fcr1(0)
        }
    }
    impl core::fmt::Debug for Fcr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fcr1")
                .field("cesotdl0f", &self.cesotdl0f())
                .field("cesotsyncdl0f", &self.cesotsyncdl0f())
                .field("ceescdl0f", &self.ceescdl0f())
                .field("cesyncescdl0f", &self.cesyncescdl0f())
                .field("cectrldl0f", &self.cectrldl0f())
                .field("cesotdl1f", &self.cesotdl1f())
                .field("cesotsyncdl1f", &self.cesotsyncdl1f())
                .field("ceescdl1f", &self.ceescdl1f())
                .field("cesyncescdl1f", &self.cesyncescdl1f())
                .field("cectrldl1f", &self.cectrldl1f())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fcr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Fcr1 {{ cesotdl0f: {=bool:?}, cesotsyncdl0f: {=bool:?}, ceescdl0f: {=bool:?}, cesyncescdl0f: {=bool:?}, cectrldl0f: {=bool:?}, cesotdl1f: {=bool:?}, cesotsyncdl1f: {=bool:?}, ceescdl1f: {=bool:?}, cesyncescdl1f: {=bool:?}, cectrldl1f: {=bool:?} }}" , self . cesotdl0f () , self . cesotsyncdl0f () , self . ceescdl0f () , self . cesyncescdl0f () , self . cectrldl0f () , self . cesotdl1f () , self . cesotsyncdl1f () , self . ceescdl1f () , self . cesyncescdl1f () , self . cectrldl1f ())
        }
    }
    #[doc = "CSI-2 Host interrupt enable register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier0(pub u32);
    impl Ier0 {
        #[doc = "Line/byte counter 0 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lb0ie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Line/byte counter 0 interrupt enable."]
        #[inline(always)]
        pub const fn set_lb0ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Line/byte counter 1 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lb1ie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Line/byte counter 1 interrupt enable."]
        #[inline(always)]
        pub const fn set_lb1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Line/byte counter 2 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lb2ie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Line/byte counter 2 interrupt enable."]
        #[inline(always)]
        pub const fn set_lb2ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Line/byte counter 3 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lb3ie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Line/byte counter 3 interrupt enable."]
        #[inline(always)]
        pub const fn set_lb3ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Timer 0 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim0ie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 0 interrupt enable."]
        #[inline(always)]
        pub const fn set_tim0ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Timer 1 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim1ie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 1 interrupt enable."]
        #[inline(always)]
        pub const fn set_tim1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Timer 2 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim2ie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 2 interrupt enable."]
        #[inline(always)]
        pub const fn set_tim2ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Timer 3 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim3ie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 3 interrupt enable."]
        #[inline(always)]
        pub const fn set_tim3ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "SOF for virtual channel 0 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sof0ie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SOF for virtual channel 0 interrupt enable."]
        #[inline(always)]
        pub const fn set_sof0ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "SOF for virtual channel 1 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sof1ie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SOF for virtual channel 1 interrupt enable."]
        #[inline(always)]
        pub const fn set_sof1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "SOF for virtual channel 2 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sof2ie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "SOF for virtual channel 2 interrupt enable."]
        #[inline(always)]
        pub const fn set_sof2ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "SOF for virtual channel 3 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sof3ie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "SOF for virtual channel 3 interrupt enable."]
        #[inline(always)]
        pub const fn set_sof3ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "EOF for virtual channel 0 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn eof0ie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "EOF for virtual channel 0 interrupt enable."]
        #[inline(always)]
        pub const fn set_eof0ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "EOF for virtual channel 1 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn eof1ie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "EOF for virtual channel 1 interrupt enable."]
        #[inline(always)]
        pub const fn set_eof1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "EOF for virtual channel 2 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn eof2ie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "EOF for virtual channel 2 interrupt enable."]
        #[inline(always)]
        pub const fn set_eof2ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "EOF for virtual channel 3 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn eof3ie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "EOF for virtual channel 3 interrupt enable."]
        #[inline(always)]
        pub const fn set_eof3ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Short packet interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn spktie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Short packet interrupt enable."]
        #[inline(always)]
        pub const fn set_spktie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Clock changer FIFO full interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ccfifofie(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Clock changer FIFO full interrupt enable."]
        #[inline(always)]
        pub const fn set_ccfifofie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "CRC error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn crcerrie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "CRC error interrupt enable."]
        #[inline(always)]
        pub const fn set_crcerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "ECC error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn eccerrie(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "ECC error interrupt enable."]
        #[inline(always)]
        pub const fn set_eccerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Corrected ECC error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ceccerrie(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Corrected ECC error interrupt enable."]
        #[inline(always)]
        pub const fn set_ceccerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Data type ID error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn iderrie(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Data type ID error interrupt enable."]
        #[inline(always)]
        pub const fn set_iderrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Short packet error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn spkterrie(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Short packet error interrupt enable."]
        #[inline(always)]
        pub const fn set_spkterrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Watchdog error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn wderrie(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog error interrupt enable."]
        #[inline(always)]
        pub const fn set_wderrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Invalid synchronization error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn syncerrie(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Invalid synchronization error interrupt enable."]
        #[inline(always)]
        pub const fn set_syncerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Ier0 {
        #[inline(always)]
        fn default() -> Ier0 {
            Ier0(0)
        }
    }
    impl core::fmt::Debug for Ier0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ier0")
                .field("lb0ie", &self.lb0ie())
                .field("lb1ie", &self.lb1ie())
                .field("lb2ie", &self.lb2ie())
                .field("lb3ie", &self.lb3ie())
                .field("tim0ie", &self.tim0ie())
                .field("tim1ie", &self.tim1ie())
                .field("tim2ie", &self.tim2ie())
                .field("tim3ie", &self.tim3ie())
                .field("sof0ie", &self.sof0ie())
                .field("sof1ie", &self.sof1ie())
                .field("sof2ie", &self.sof2ie())
                .field("sof3ie", &self.sof3ie())
                .field("eof0ie", &self.eof0ie())
                .field("eof1ie", &self.eof1ie())
                .field("eof2ie", &self.eof2ie())
                .field("eof3ie", &self.eof3ie())
                .field("spktie", &self.spktie())
                .field("ccfifofie", &self.ccfifofie())
                .field("crcerrie", &self.crcerrie())
                .field("eccerrie", &self.eccerrie())
                .field("ceccerrie", &self.ceccerrie())
                .field("iderrie", &self.iderrie())
                .field("spkterrie", &self.spkterrie())
                .field("wderrie", &self.wderrie())
                .field("syncerrie", &self.syncerrie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ier0 {{ lb0ie: {=bool:?}, lb1ie: {=bool:?}, lb2ie: {=bool:?}, lb3ie: {=bool:?}, tim0ie: {=bool:?}, tim1ie: {=bool:?}, tim2ie: {=bool:?}, tim3ie: {=bool:?}, sof0ie: {=bool:?}, sof1ie: {=bool:?}, sof2ie: {=bool:?}, sof3ie: {=bool:?}, eof0ie: {=bool:?}, eof1ie: {=bool:?}, eof2ie: {=bool:?}, eof3ie: {=bool:?}, spktie: {=bool:?}, ccfifofie: {=bool:?}, crcerrie: {=bool:?}, eccerrie: {=bool:?}, ceccerrie: {=bool:?}, iderrie: {=bool:?}, spkterrie: {=bool:?}, wderrie: {=bool:?}, syncerrie: {=bool:?} }}" , self . lb0ie () , self . lb1ie () , self . lb2ie () , self . lb3ie () , self . tim0ie () , self . tim1ie () , self . tim2ie () , self . tim3ie () , self . sof0ie () , self . sof1ie () , self . sof2ie () , self . sof3ie () , self . eof0ie () , self . eof1ie () , self . eof2ie () , self . eof3ie () , self . spktie () , self . ccfifofie () , self . crcerrie () , self . eccerrie () , self . ceccerrie () , self . iderrie () , self . spkterrie () , self . wderrie () , self . syncerrie ())
        }
    }
    #[doc = "CSI-2 Host interrupt enable register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier1(pub u32);
    impl Ier1 {
        #[doc = "SOT error interrupt enable on lane 0."]
        #[must_use]
        #[inline(always)]
        pub const fn esotdl0ie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SOT error interrupt enable on lane 0."]
        #[inline(always)]
        pub const fn set_esotdl0ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SOT synchronization interrupt error enable on lane 0."]
        #[must_use]
        #[inline(always)]
        pub const fn esotsyncdl0ie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SOT synchronization interrupt error enable on lane 0."]
        #[inline(always)]
        pub const fn set_esotsyncdl0ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "D-PHY_RX lane 0 escape entry error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn eescdl0ie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "D-PHY_RX lane 0 escape entry error interrupt enable."]
        #[inline(always)]
        pub const fn set_eescdl0ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "D-PHY_RX lane 0 low power data transmission synchronization error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn esyncescdl0ie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "D-PHY_RX lane 0 low power data transmission synchronization error interrupt enable."]
        #[inline(always)]
        pub const fn set_esyncescdl0ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "D-PHY_RX lane 0 control error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ectrldl0ie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "D-PHY_RX lane 0 control error interrupt enable."]
        #[inline(always)]
        pub const fn set_ectrldl0ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SOT error interrupt enable on lane 1."]
        #[must_use]
        #[inline(always)]
        pub const fn esotdl1ie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SOT error interrupt enable on lane 1."]
        #[inline(always)]
        pub const fn set_esotdl1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "SOT synchronization interrupt error enable on lane 1."]
        #[must_use]
        #[inline(always)]
        pub const fn esotsyncdl1ie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SOT synchronization interrupt error enable on lane 1."]
        #[inline(always)]
        pub const fn set_esotsyncdl1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "D-PHY_RX lane 1 escape entry error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn eescdl1ie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "D-PHY_RX lane 1 escape entry error interrupt enable."]
        #[inline(always)]
        pub const fn set_eescdl1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "D-PHY_RX lane 1 low-power data transmission synchronization error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn esyncescdl1ie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "D-PHY_RX lane 1 low-power data transmission synchronization error interrupt enable."]
        #[inline(always)]
        pub const fn set_esyncescdl1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "D-PHY_RX lane 1 control error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ectrldl1ie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "D-PHY_RX lane 1 control error interrupt enable."]
        #[inline(always)]
        pub const fn set_ectrldl1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Ier1 {
        #[inline(always)]
        fn default() -> Ier1 {
            Ier1(0)
        }
    }
    impl core::fmt::Debug for Ier1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ier1")
                .field("esotdl0ie", &self.esotdl0ie())
                .field("esotsyncdl0ie", &self.esotsyncdl0ie())
                .field("eescdl0ie", &self.eescdl0ie())
                .field("esyncescdl0ie", &self.esyncescdl0ie())
                .field("ectrldl0ie", &self.ectrldl0ie())
                .field("esotdl1ie", &self.esotdl1ie())
                .field("esotsyncdl1ie", &self.esotsyncdl1ie())
                .field("eescdl1ie", &self.eescdl1ie())
                .field("esyncescdl1ie", &self.esyncescdl1ie())
                .field("ectrldl1ie", &self.ectrldl1ie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ier1 {{ esotdl0ie: {=bool:?}, esotsyncdl0ie: {=bool:?}, eescdl0ie: {=bool:?}, esyncescdl0ie: {=bool:?}, ectrldl0ie: {=bool:?}, esotdl1ie: {=bool:?}, esotsyncdl1ie: {=bool:?}, eescdl1ie: {=bool:?}, esyncescdl1ie: {=bool:?}, ectrldl1ie: {=bool:?} }}" , self . esotdl0ie () , self . esotsyncdl0ie () , self . eescdl0ie () , self . esyncescdl0ie () , self . ectrldl0ie () , self . esotdl1ie () , self . esotsyncdl1ie () , self . eescdl1ie () , self . esyncescdl1ie () , self . ectrldl1ie ())
        }
    }
    #[doc = "CSI-2 Host line byte 0 configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lb0cfgr(pub u32);
    impl Lb0cfgr {
        #[doc = "Byte counter."]
        #[must_use]
        #[inline(always)]
        pub const fn bytecnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Byte counter."]
        #[inline(always)]
        pub const fn set_bytecnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Line counter."]
        #[must_use]
        #[inline(always)]
        pub const fn linecnt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Line counter."]
        #[inline(always)]
        pub const fn set_linecnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Lb0cfgr {
        #[inline(always)]
        fn default() -> Lb0cfgr {
            Lb0cfgr(0)
        }
    }
    impl core::fmt::Debug for Lb0cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lb0cfgr")
                .field("bytecnt", &self.bytecnt())
                .field("linecnt", &self.linecnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lb0cfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Lb0cfgr {{ bytecnt: {=u16:?}, linecnt: {=u16:?} }}",
                self.bytecnt(),
                self.linecnt()
            )
        }
    }
    #[doc = "CSI-2 Host line byte 1 configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lb1cfgr(pub u32);
    impl Lb1cfgr {
        #[doc = "Byte counter."]
        #[must_use]
        #[inline(always)]
        pub const fn bytecnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Byte counter."]
        #[inline(always)]
        pub const fn set_bytecnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Line counter."]
        #[must_use]
        #[inline(always)]
        pub const fn linecnt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Line counter."]
        #[inline(always)]
        pub const fn set_linecnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Lb1cfgr {
        #[inline(always)]
        fn default() -> Lb1cfgr {
            Lb1cfgr(0)
        }
    }
    impl core::fmt::Debug for Lb1cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lb1cfgr")
                .field("bytecnt", &self.bytecnt())
                .field("linecnt", &self.linecnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lb1cfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Lb1cfgr {{ bytecnt: {=u16:?}, linecnt: {=u16:?} }}",
                self.bytecnt(),
                self.linecnt()
            )
        }
    }
    #[doc = "CSI-2 Host line byte 2 configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lb2cfgr(pub u32);
    impl Lb2cfgr {
        #[doc = "Byte counter."]
        #[must_use]
        #[inline(always)]
        pub const fn bytecnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Byte counter."]
        #[inline(always)]
        pub const fn set_bytecnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Line counter."]
        #[must_use]
        #[inline(always)]
        pub const fn linecnt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Line counter."]
        #[inline(always)]
        pub const fn set_linecnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Lb2cfgr {
        #[inline(always)]
        fn default() -> Lb2cfgr {
            Lb2cfgr(0)
        }
    }
    impl core::fmt::Debug for Lb2cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lb2cfgr")
                .field("bytecnt", &self.bytecnt())
                .field("linecnt", &self.linecnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lb2cfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Lb2cfgr {{ bytecnt: {=u16:?}, linecnt: {=u16:?} }}",
                self.bytecnt(),
                self.linecnt()
            )
        }
    }
    #[doc = "CSI-2 Host line byte 3 configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lb3cfgr(pub u32);
    impl Lb3cfgr {
        #[doc = "Byte counter."]
        #[must_use]
        #[inline(always)]
        pub const fn bytecnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Byte counter."]
        #[inline(always)]
        pub const fn set_bytecnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Line counter."]
        #[must_use]
        #[inline(always)]
        pub const fn linecnt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Line counter."]
        #[inline(always)]
        pub const fn set_linecnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Lb3cfgr {
        #[inline(always)]
        fn default() -> Lb3cfgr {
            Lb3cfgr(0)
        }
    }
    impl core::fmt::Debug for Lb3cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lb3cfgr")
                .field("bytecnt", &self.bytecnt())
                .field("linecnt", &self.linecnt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lb3cfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Lb3cfgr {{ bytecnt: {=u16:?}, linecnt: {=u16:?} }}",
                self.bytecnt(),
                self.linecnt()
            )
        }
    }
    #[doc = "CSI-2 Host lane merger configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lmcfgr(pub u32);
    impl Lmcfgr {
        #[doc = "Number of lanes."]
        #[must_use]
        #[inline(always)]
        pub const fn lanenb(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Number of lanes."]
        #[inline(always)]
        pub const fn set_lanenb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "Physical mapping of logical data lane 0."]
        #[must_use]
        #[inline(always)]
        pub const fn dl0map(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Physical mapping of logical data lane 0."]
        #[inline(always)]
        pub const fn set_dl0map(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "Physical mapping of logical data lane 1."]
        #[must_use]
        #[inline(always)]
        pub const fn dl1map(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "Physical mapping of logical data lane 1."]
        #[inline(always)]
        pub const fn set_dl1map(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
    }
    impl Default for Lmcfgr {
        #[inline(always)]
        fn default() -> Lmcfgr {
            Lmcfgr(0)
        }
    }
    impl core::fmt::Debug for Lmcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lmcfgr")
                .field("lanenb", &self.lanenb())
                .field("dl0map", &self.dl0map())
                .field("dl1map", &self.dl1map())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lmcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Lmcfgr {{ lanenb: {=u8:?}, dl0map: {=u8:?}, dl1map: {=u8:?} }}",
                self.lanenb(),
                self.dl0map(),
                self.dl1map()
            )
        }
    }
    #[doc = "CSI-2 Host DPHY_RX control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcr(pub u32);
    impl Pcr {
        #[doc = "Power down."]
        #[must_use]
        #[inline(always)]
        pub const fn pwrdown(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Power down."]
        #[inline(always)]
        pub const fn set_pwrdown(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clock lane enable."]
        #[must_use]
        #[inline(always)]
        pub const fn clen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clock lane enable."]
        #[inline(always)]
        pub const fn set_clen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "D-PHY_RX data lane 0 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dl0en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "D-PHY_RX data lane 0 enable."]
        #[inline(always)]
        pub const fn set_dl0en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "D-PHY_RX data lane 1 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dl1en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "D-PHY_RX data lane 1 enable."]
        #[inline(always)]
        pub const fn set_dl1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
                .field("pwrdown", &self.pwrdown())
                .field("clen", &self.clen())
                .field("dl0en", &self.dl0en())
                .field("dl1en", &self.dl1en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pcr {{ pwrdown: {=bool:?}, clen: {=bool:?}, dl0en: {=bool:?}, dl1en: {=bool:?} }}",
                self.pwrdown(),
                self.clen(),
                self.dl0en(),
                self.dl1en()
            )
        }
    }
    #[doc = "CSI PHY frequency control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfcr(pub u32);
    impl Pfcr {
        #[doc = "Configuration clock frequency range selection."]
        #[must_use]
        #[inline(always)]
        pub const fn ccfr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Configuration clock frequency range selection."]
        #[inline(always)]
        pub const fn set_ccfr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "PHY high-speed frequency range selection."]
        #[must_use]
        #[inline(always)]
        pub const fn hsfr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "PHY high-speed frequency range selection."]
        #[inline(always)]
        pub const fn set_hsfr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "Data lane direction of lane 0."]
        #[must_use]
        #[inline(always)]
        pub const fn dld(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Data lane direction of lane 0."]
        #[inline(always)]
        pub const fn set_dld(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Pfcr {
        #[inline(always)]
        fn default() -> Pfcr {
            Pfcr(0)
        }
    }
    impl core::fmt::Debug for Pfcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pfcr")
                .field("ccfr", &self.ccfr())
                .field("hsfr", &self.hsfr())
                .field("dld", &self.dld())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pfcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pfcr {{ ccfr: {=u8:?}, hsfr: {=u8:?}, dld: {=bool:?} }}",
                self.ccfr(),
                self.hsfr(),
                self.dld()
            )
        }
    }
    #[doc = "CSI PHY mode control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pmcr(pub u32);
    impl Pmcr {
        #[doc = "Force to Rx mode the data lane 0."]
        #[must_use]
        #[inline(always)]
        pub const fn frxmdl0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Force to Rx mode the data lane 0."]
        #[inline(always)]
        pub const fn set_frxmdl0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Force to Rx mode the data lane 1."]
        #[must_use]
        #[inline(always)]
        pub const fn frxmdl1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Force to Rx mode the data lane 1."]
        #[inline(always)]
        pub const fn set_frxmdl1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Force to Tx Stop mode the data lane 0."]
        #[must_use]
        #[inline(always)]
        pub const fn ftxsmdl0(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Force to Tx Stop mode the data lane 0."]
        #[inline(always)]
        pub const fn set_ftxsmdl0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Disable turn-around data lane 0."]
        #[must_use]
        #[inline(always)]
        pub const fn dtdl(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Disable turn-around data lane 0."]
        #[inline(always)]
        pub const fn set_dtdl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Turn-around request data lane 0."]
        #[must_use]
        #[inline(always)]
        pub const fn rtdl0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Turn-around request data lane 0."]
        #[inline(always)]
        pub const fn set_rtdl0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Tx ULP escape-mode data lane 0."]
        #[must_use]
        #[inline(always)]
        pub const fn tuesdl0(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Tx ULP escape-mode data lane 0."]
        #[inline(always)]
        pub const fn set_tuesdl0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Tx ULP exit sequence data lane 0."]
        #[must_use]
        #[inline(always)]
        pub const fn tuexdl0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Tx ULP exit sequence data lane 0."]
        #[inline(always)]
        pub const fn set_tuexdl0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
                .field("frxmdl0", &self.frxmdl0())
                .field("frxmdl1", &self.frxmdl1())
                .field("ftxsmdl0", &self.ftxsmdl0())
                .field("dtdl", &self.dtdl())
                .field("rtdl0", &self.rtdl0())
                .field("tuesdl0", &self.tuesdl0())
                .field("tuexdl0", &self.tuexdl0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pmcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Pmcr {{ frxmdl0: {=bool:?}, frxmdl1: {=bool:?}, ftxsmdl0: {=bool:?}, dtdl: {=bool:?}, rtdl0: {=bool:?}, tuesdl0: {=bool:?}, tuexdl0: {=bool:?} }}" , self . frxmdl0 () , self . frxmdl1 () , self . ftxsmdl0 () , self . dtdl () , self . rtdl0 () , self . tuesdl0 () , self . tuexdl0 ())
        }
    }
    #[doc = "CSI PHY reset control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Prcr(pub u32);
    impl Prcr {
        #[doc = "When set to 0, this bit places the digital section of the D-PHY in the reset state."]
        #[must_use]
        #[inline(always)]
        pub const fn pen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "When set to 0, this bit places the digital section of the D-PHY in the reset state."]
        #[inline(always)]
        pub const fn set_pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Prcr {
        #[inline(always)]
        fn default() -> Prcr {
            Prcr(0)
        }
    }
    impl core::fmt::Debug for Prcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Prcr").field("pen", &self.pen()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Prcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Prcr {{ pen: {=bool:?} }}", self.pen())
        }
    }
    #[doc = "CSI-2 Host program interrupt register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Prgitr(pub u32);
    impl Prgitr {
        #[doc = "Line/byte counter 0 linked to a virtual channel."]
        #[must_use]
        #[inline(always)]
        pub const fn lb0vc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Line/byte counter 0 linked to a virtual channel."]
        #[inline(always)]
        pub const fn set_lb0vc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Line/byte 0 counter enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lb0en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Line/byte 0 counter enable."]
        #[inline(always)]
        pub const fn set_lb0en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Line/byte counter 1 linked to a virtual channel."]
        #[must_use]
        #[inline(always)]
        pub const fn lb1vc(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Line/byte counter 1 linked to a virtual channel."]
        #[inline(always)]
        pub const fn set_lb1vc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Line/byte 1 counter enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lb1en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Line/byte 1 counter enable."]
        #[inline(always)]
        pub const fn set_lb1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Line/byte counter 2 linked to a virtual channel."]
        #[must_use]
        #[inline(always)]
        pub const fn lb2vc(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Line/byte counter 2 linked to a virtual channel."]
        #[inline(always)]
        pub const fn set_lb2vc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "Line/byte 2 counter enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lb2en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Line/byte 2 counter enable."]
        #[inline(always)]
        pub const fn set_lb2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Line/byte counter 3 linked to a virtual channel."]
        #[must_use]
        #[inline(always)]
        pub const fn lb3vc(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Line/byte counter 3 linked to a virtual channel."]
        #[inline(always)]
        pub const fn set_lb3vc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Line/byte 3 counter enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lb3en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Line/byte 3 counter enable."]
        #[inline(always)]
        pub const fn set_lb3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "TIM0 base time linked to a virtual channel."]
        #[must_use]
        #[inline(always)]
        pub const fn tim0vc(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "TIM0 base time linked to a virtual channel."]
        #[inline(always)]
        pub const fn set_tim0vc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "TIM0 base time starting from the EOF."]
        #[must_use]
        #[inline(always)]
        pub const fn tim0eof(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM0 base time starting from the EOF."]
        #[inline(always)]
        pub const fn set_tim0eof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "TIM0 base time enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim0en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "TIM0 base time enable."]
        #[inline(always)]
        pub const fn set_tim0en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "TIM1 base time linked to a virtual channel."]
        #[must_use]
        #[inline(always)]
        pub const fn tim1vc(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "TIM1 base time linked to a virtual channel."]
        #[inline(always)]
        pub const fn set_tim1vc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "TIM1 base time starting from the EOF."]
        #[must_use]
        #[inline(always)]
        pub const fn tim1eof(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 base time starting from the EOF."]
        #[inline(always)]
        pub const fn set_tim1eof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "TIM1 base time enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim1en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 base time enable."]
        #[inline(always)]
        pub const fn set_tim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "TIM2 base time linked to a virtual channel."]
        #[must_use]
        #[inline(always)]
        pub const fn tim2vc(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "TIM2 base time linked to a virtual channel."]
        #[inline(always)]
        pub const fn set_tim2vc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "TIM2 base time starting from the EOF."]
        #[must_use]
        #[inline(always)]
        pub const fn tim2eof(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 base time starting from the EOF."]
        #[inline(always)]
        pub const fn set_tim2eof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "TIM2 base time enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim2en(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 base time enable."]
        #[inline(always)]
        pub const fn set_tim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "TIM3 base time linked to a virtual channel."]
        #[must_use]
        #[inline(always)]
        pub const fn tim3vc(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "TIM3 base time linked to a virtual channel."]
        #[inline(always)]
        pub const fn set_tim3vc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
        #[doc = "TIM3 base time starting from the EOF."]
        #[must_use]
        #[inline(always)]
        pub const fn tim3eof(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 base time starting from the EOF."]
        #[inline(always)]
        pub const fn set_tim3eof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "TIM3 base time enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim3en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 base time enable."]
        #[inline(always)]
        pub const fn set_tim3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Prgitr {
        #[inline(always)]
        fn default() -> Prgitr {
            Prgitr(0)
        }
    }
    impl core::fmt::Debug for Prgitr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Prgitr")
                .field("lb0vc", &self.lb0vc())
                .field("lb0en", &self.lb0en())
                .field("lb1vc", &self.lb1vc())
                .field("lb1en", &self.lb1en())
                .field("lb2vc", &self.lb2vc())
                .field("lb2en", &self.lb2en())
                .field("lb3vc", &self.lb3vc())
                .field("lb3en", &self.lb3en())
                .field("tim0vc", &self.tim0vc())
                .field("tim0eof", &self.tim0eof())
                .field("tim0en", &self.tim0en())
                .field("tim1vc", &self.tim1vc())
                .field("tim1eof", &self.tim1eof())
                .field("tim1en", &self.tim1en())
                .field("tim2vc", &self.tim2vc())
                .field("tim2eof", &self.tim2eof())
                .field("tim2en", &self.tim2en())
                .field("tim3vc", &self.tim3vc())
                .field("tim3eof", &self.tim3eof())
                .field("tim3en", &self.tim3en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Prgitr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Prgitr {{ lb0vc: {=u8:?}, lb0en: {=bool:?}, lb1vc: {=u8:?}, lb1en: {=bool:?}, lb2vc: {=u8:?}, lb2en: {=bool:?}, lb3vc: {=u8:?}, lb3en: {=bool:?}, tim0vc: {=u8:?}, tim0eof: {=bool:?}, tim0en: {=bool:?}, tim1vc: {=u8:?}, tim1eof: {=bool:?}, tim1en: {=bool:?}, tim2vc: {=u8:?}, tim2eof: {=bool:?}, tim2en: {=bool:?}, tim3vc: {=u8:?}, tim3eof: {=bool:?}, tim3en: {=bool:?} }}" , self . lb0vc () , self . lb0en () , self . lb1vc () , self . lb1en () , self . lb2vc () , self . lb2en () , self . lb3vc () , self . lb3en () , self . tim0vc () , self . tim0eof () , self . tim0en () , self . tim1vc () , self . tim1eof () , self . tim1en () , self . tim2vc () , self . tim2eof () , self . tim2en () , self . tim3vc () , self . tim3eof () , self . tim3en ())
        }
    }
    #[doc = "CSI PHY test control register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptcr0(pub u32);
    impl Ptcr0 {
        #[doc = "Test-interface clock enable for the TDI bus into the PHY."]
        #[must_use]
        #[inline(always)]
        pub const fn tcken(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Test-interface clock enable for the TDI bus into the PHY."]
        #[inline(always)]
        pub const fn set_tcken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Test-interface reset enable for the TDI bus into the PHY."]
        #[must_use]
        #[inline(always)]
        pub const fn trsen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Test-interface reset enable for the TDI bus into the PHY."]
        #[inline(always)]
        pub const fn set_trsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Ptcr0 {
        #[inline(always)]
        fn default() -> Ptcr0 {
            Ptcr0(0)
        }
    }
    impl core::fmt::Debug for Ptcr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ptcr0")
                .field("tcken", &self.tcken())
                .field("trsen", &self.trsen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ptcr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ptcr0 {{ tcken: {=bool:?}, trsen: {=bool:?} }}",
                self.tcken(),
                self.trsen()
            )
        }
    }
    #[doc = "CSI PHY test control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptcr1(pub u32);
    impl Ptcr1 {
        #[doc = "Test-interface data in."]
        #[must_use]
        #[inline(always)]
        pub const fn tdi(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Test-interface data in."]
        #[inline(always)]
        pub const fn set_tdi(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Test-interface write mode selector."]
        #[must_use]
        #[inline(always)]
        pub const fn twm(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Test-interface write mode selector."]
        #[inline(always)]
        pub const fn set_twm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Ptcr1 {
        #[inline(always)]
        fn default() -> Ptcr1 {
            Ptcr1(0)
        }
    }
    impl core::fmt::Debug for Ptcr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ptcr1")
                .field("tdi", &self.tdi())
                .field("twm", &self.twm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ptcr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ptcr1 {{ tdi: {=u8:?}, twm: {=bool:?} }}", self.tdi(), self.twm())
        }
    }
    #[doc = "CSI PHY test status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptsr(pub u32);
    impl Ptsr {
        #[doc = "CSI PHY test interface data output bus for read-back and internal probing functionalities."]
        #[must_use]
        #[inline(always)]
        pub const fn tdo(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "CSI PHY test interface data output bus for read-back and internal probing functionalities."]
        #[inline(always)]
        pub const fn set_tdo(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Ptsr {
        #[inline(always)]
        fn default() -> Ptsr {
            Ptsr(0)
        }
    }
    impl core::fmt::Debug for Ptsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ptsr").field("tdo", &self.tdo()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ptsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ptsr {{ tdo: {=u8:?} }}", self.tdo())
        }
    }
    #[doc = "CSI-2 Host short packet data field register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Spdfr(pub u32);
    impl Spdfr {
        #[doc = "Data field."]
        #[must_use]
        #[inline(always)]
        pub const fn datafield(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Data field."]
        #[inline(always)]
        pub const fn set_datafield(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Data type class."]
        #[must_use]
        #[inline(always)]
        pub const fn datatype(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type class."]
        #[inline(always)]
        pub const fn set_datatype(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "Virtual channel."]
        #[must_use]
        #[inline(always)]
        pub const fn vchannel(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "Virtual channel."]
        #[inline(always)]
        pub const fn set_vchannel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
    }
    impl Default for Spdfr {
        #[inline(always)]
        fn default() -> Spdfr {
            Spdfr(0)
        }
    }
    impl core::fmt::Debug for Spdfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Spdfr")
                .field("datafield", &self.datafield())
                .field("datatype", &self.datatype())
                .field("vchannel", &self.vchannel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Spdfr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Spdfr {{ datafield: {=u16:?}, datatype: {=u8:?}, vchannel: {=u8:?} }}",
                self.datafield(),
                self.datatype(),
                self.vchannel()
            )
        }
    }
    #[doc = "CSI-2 Host status register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr0(pub u32);
    impl Sr0 {
        #[doc = "Line/byte counter 0 flag."]
        #[must_use]
        #[inline(always)]
        pub const fn lb0f(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Line/byte counter 0 flag."]
        #[inline(always)]
        pub const fn set_lb0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Line/byte counter 1 flag."]
        #[must_use]
        #[inline(always)]
        pub const fn lb1f(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Line/byte counter 1 flag."]
        #[inline(always)]
        pub const fn set_lb1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Line/byte counter 2 flag."]
        #[must_use]
        #[inline(always)]
        pub const fn lb2f(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Line/byte counter 2 flag."]
        #[inline(always)]
        pub const fn set_lb2f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Line/byte counter 3 flag."]
        #[must_use]
        #[inline(always)]
        pub const fn lb3f(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Line/byte counter 3 flag."]
        #[inline(always)]
        pub const fn set_lb3f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Timer 0 flag."]
        #[must_use]
        #[inline(always)]
        pub const fn tim0f(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 0 flag."]
        #[inline(always)]
        pub const fn set_tim0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Timer 1 flag."]
        #[must_use]
        #[inline(always)]
        pub const fn tim1f(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 1 flag."]
        #[inline(always)]
        pub const fn set_tim1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Timer 2 flag."]
        #[must_use]
        #[inline(always)]
        pub const fn tim2f(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 2 flag."]
        #[inline(always)]
        pub const fn set_tim2f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Timer 3 flag."]
        #[must_use]
        #[inline(always)]
        pub const fn tim3f(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 3 flag."]
        #[inline(always)]
        pub const fn set_tim3f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "SOF flag for virtual channel 0."]
        #[must_use]
        #[inline(always)]
        pub const fn sof0f(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SOF flag for virtual channel 0."]
        #[inline(always)]
        pub const fn set_sof0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "SOF flag for virtual channel 1."]
        #[must_use]
        #[inline(always)]
        pub const fn sof1f(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SOF flag for virtual channel 1."]
        #[inline(always)]
        pub const fn set_sof1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "SOF flag for virtual channel 2."]
        #[must_use]
        #[inline(always)]
        pub const fn sof2f(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "SOF flag for virtual channel 2."]
        #[inline(always)]
        pub const fn set_sof2f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "SOF flag for virtual channel 3."]
        #[must_use]
        #[inline(always)]
        pub const fn sof3f(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "SOF flag for virtual channel 3."]
        #[inline(always)]
        pub const fn set_sof3f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "EOF flag for virtual channel 0."]
        #[must_use]
        #[inline(always)]
        pub const fn eof0f(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "EOF flag for virtual channel 0."]
        #[inline(always)]
        pub const fn set_eof0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "EOF flag for virtual channel 1."]
        #[must_use]
        #[inline(always)]
        pub const fn eof1f(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "EOF flag for virtual channel 1."]
        #[inline(always)]
        pub const fn set_eof1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "EOF flag for virtual channel 2."]
        #[must_use]
        #[inline(always)]
        pub const fn eof2f(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "EOF flag for virtual channel 2."]
        #[inline(always)]
        pub const fn set_eof2f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "EOF flag for virtual channel 3."]
        #[must_use]
        #[inline(always)]
        pub const fn eof3f(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "EOF flag for virtual channel 3."]
        #[inline(always)]
        pub const fn set_eof3f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Short packet flag."]
        #[must_use]
        #[inline(always)]
        pub const fn spktf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Short packet flag."]
        #[inline(always)]
        pub const fn set_spktf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Virtual channel 0 state flag."]
        #[must_use]
        #[inline(always)]
        pub const fn vc0statef(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Virtual channel 0 state flag."]
        #[inline(always)]
        pub const fn set_vc0statef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Virtual channel 1 state flag."]
        #[must_use]
        #[inline(always)]
        pub const fn vc1statef(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Virtual channel 1 state flag."]
        #[inline(always)]
        pub const fn set_vc1statef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Virtual channel 2 state flag."]
        #[must_use]
        #[inline(always)]
        pub const fn vc2statef(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Virtual channel 2 state flag."]
        #[inline(always)]
        pub const fn set_vc2statef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Virtual channel 3 state flag."]
        #[must_use]
        #[inline(always)]
        pub const fn vc3statef(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Virtual channel 3 state flag."]
        #[inline(always)]
        pub const fn set_vc3statef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Clock changer FIFO full flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ccfifoff(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Clock changer FIFO full flag."]
        #[inline(always)]
        pub const fn set_ccfifoff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "CRC error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn crcerrf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "CRC error flag."]
        #[inline(always)]
        pub const fn set_crcerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "ECC error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn eccerrf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "ECC error flag."]
        #[inline(always)]
        pub const fn set_eccerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Corrected ECC error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ceccerrf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Corrected ECC error flag."]
        #[inline(always)]
        pub const fn set_ceccerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Data type ID error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn iderrf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Data type ID error flag."]
        #[inline(always)]
        pub const fn set_iderrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Short packet error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn spkterrf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Short packet error flag."]
        #[inline(always)]
        pub const fn set_spkterrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Watchdog error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn wderrf(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog error flag."]
        #[inline(always)]
        pub const fn set_wderrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Invalid synchronization error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn syncerrf(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Invalid synchronization error flag."]
        #[inline(always)]
        pub const fn set_syncerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Sr0 {
        #[inline(always)]
        fn default() -> Sr0 {
            Sr0(0)
        }
    }
    impl core::fmt::Debug for Sr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr0")
                .field("lb0f", &self.lb0f())
                .field("lb1f", &self.lb1f())
                .field("lb2f", &self.lb2f())
                .field("lb3f", &self.lb3f())
                .field("tim0f", &self.tim0f())
                .field("tim1f", &self.tim1f())
                .field("tim2f", &self.tim2f())
                .field("tim3f", &self.tim3f())
                .field("sof0f", &self.sof0f())
                .field("sof1f", &self.sof1f())
                .field("sof2f", &self.sof2f())
                .field("sof3f", &self.sof3f())
                .field("eof0f", &self.eof0f())
                .field("eof1f", &self.eof1f())
                .field("eof2f", &self.eof2f())
                .field("eof3f", &self.eof3f())
                .field("spktf", &self.spktf())
                .field("vc0statef", &self.vc0statef())
                .field("vc1statef", &self.vc1statef())
                .field("vc2statef", &self.vc2statef())
                .field("vc3statef", &self.vc3statef())
                .field("ccfifoff", &self.ccfifoff())
                .field("crcerrf", &self.crcerrf())
                .field("eccerrf", &self.eccerrf())
                .field("ceccerrf", &self.ceccerrf())
                .field("iderrf", &self.iderrf())
                .field("spkterrf", &self.spkterrf())
                .field("wderrf", &self.wderrf())
                .field("syncerrf", &self.syncerrf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr0 {{ lb0f: {=bool:?}, lb1f: {=bool:?}, lb2f: {=bool:?}, lb3f: {=bool:?}, tim0f: {=bool:?}, tim1f: {=bool:?}, tim2f: {=bool:?}, tim3f: {=bool:?}, sof0f: {=bool:?}, sof1f: {=bool:?}, sof2f: {=bool:?}, sof3f: {=bool:?}, eof0f: {=bool:?}, eof1f: {=bool:?}, eof2f: {=bool:?}, eof3f: {=bool:?}, spktf: {=bool:?}, vc0statef: {=bool:?}, vc1statef: {=bool:?}, vc2statef: {=bool:?}, vc3statef: {=bool:?}, ccfifoff: {=bool:?}, crcerrf: {=bool:?}, eccerrf: {=bool:?}, ceccerrf: {=bool:?}, iderrf: {=bool:?}, spkterrf: {=bool:?}, wderrf: {=bool:?}, syncerrf: {=bool:?} }}" , self . lb0f () , self . lb1f () , self . lb2f () , self . lb3f () , self . tim0f () , self . tim1f () , self . tim2f () , self . tim3f () , self . sof0f () , self . sof1f () , self . sof2f () , self . sof3f () , self . eof0f () , self . eof1f () , self . eof2f () , self . eof3f () , self . spktf () , self . vc0statef () , self . vc1statef () , self . vc2statef () , self . vc3statef () , self . ccfifoff () , self . crcerrf () , self . eccerrf () , self . ceccerrf () , self . iderrf () , self . spkterrf () , self . wderrf () , self . syncerrf ())
        }
    }
    #[doc = "CSI-2 Host status register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr1(pub u32);
    impl Sr1 {
        #[doc = "SOT error flag on lane 0."]
        #[must_use]
        #[inline(always)]
        pub const fn esotdl0f(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SOT error flag on lane 0."]
        #[inline(always)]
        pub const fn set_esotdl0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SOT synchronization error flag on lane 0."]
        #[must_use]
        #[inline(always)]
        pub const fn esotsyncdl0f(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SOT synchronization error flag on lane 0."]
        #[inline(always)]
        pub const fn set_esotsyncdl0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "D-PHY_RX lane 0 escape entry error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn eescdl0f(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "D-PHY_RX lane 0 escape entry error flag."]
        #[inline(always)]
        pub const fn set_eescdl0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "D-PHY_RX lane 0 low-power data transmission synchronization error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn esyncescdl0f(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "D-PHY_RX lane 0 low-power data transmission synchronization error flag."]
        #[inline(always)]
        pub const fn set_esyncescdl0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "D-PHY_RX lane 0 control error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ectrldl0f(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "D-PHY_RX lane 0 control error flag."]
        #[inline(always)]
        pub const fn set_ectrldl0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SOT error flag on lane 1."]
        #[must_use]
        #[inline(always)]
        pub const fn esotdl1f(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SOT error flag on lane 1."]
        #[inline(always)]
        pub const fn set_esotdl1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "SOT synchronization error flag on lane 1."]
        #[must_use]
        #[inline(always)]
        pub const fn esotsyncdl1f(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SOT synchronization error flag on lane 1."]
        #[inline(always)]
        pub const fn set_esotsyncdl1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "D-PHY_RX lane 1 escape entry error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn eescdl1f(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "D-PHY_RX lane 1 escape entry error flag."]
        #[inline(always)]
        pub const fn set_eescdl1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "D-PHY_RX lane 1 low-power data transmission synchronization error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn esyncescdl1f(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "D-PHY_RX lane 1 low-power data transmission synchronization error flag."]
        #[inline(always)]
        pub const fn set_esyncescdl1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "D-PHY_RX lane 1 control error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ectrldl1f(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "D-PHY_RX lane 1 control error flag."]
        #[inline(always)]
        pub const fn set_ectrldl1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "D-PHY_RX lane 0 high-speed reception active."]
        #[must_use]
        #[inline(always)]
        pub const fn actdl0f(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "D-PHY_RX lane 0 high-speed reception active."]
        #[inline(always)]
        pub const fn set_actdl0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "D-PHY_RX lane 0 receiver synchronization observed."]
        #[must_use]
        #[inline(always)]
        pub const fn syncdl0f(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "D-PHY_RX lane 0 receiver synchronization observed."]
        #[inline(always)]
        pub const fn set_syncdl0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "D-PHY_RX lane 0 high-speed skew calibration."]
        #[must_use]
        #[inline(always)]
        pub const fn skcaldl0f(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "D-PHY_RX lane 0 high-speed skew calibration."]
        #[inline(always)]
        pub const fn set_skcaldl0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "D-PHY_RX receiver data lane 0 in stop state."]
        #[must_use]
        #[inline(always)]
        pub const fn stopdl0f(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "D-PHY_RX receiver data lane 0 in stop state."]
        #[inline(always)]
        pub const fn set_stopdl0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "D-PHY_RX receiver ultra-low-power state (not) active on data lane 0."]
        #[must_use]
        #[inline(always)]
        pub const fn ulpndl0f(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "D-PHY_RX receiver ultra-low-power state (not) active on data lane 0."]
        #[inline(always)]
        pub const fn set_ulpndl0f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "D-PHY_RX lane 1 high-speed reception active."]
        #[must_use]
        #[inline(always)]
        pub const fn actdl1f(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "D-PHY_RX lane 1 high-speed reception active."]
        #[inline(always)]
        pub const fn set_actdl1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "D-PHY_RX lane 1 receiver synchronization observed."]
        #[must_use]
        #[inline(always)]
        pub const fn syncdl1f(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "D-PHY_RX lane 1 receiver synchronization observed."]
        #[inline(always)]
        pub const fn set_syncdl1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "D-PHY_RX lane 1 high-speed skew calibration."]
        #[must_use]
        #[inline(always)]
        pub const fn skcaldl1f(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "D-PHY_RX lane 1 high-speed skew calibration."]
        #[inline(always)]
        pub const fn set_skcaldl1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "D-PHY_RX receiver data lane 1 in stop state."]
        #[must_use]
        #[inline(always)]
        pub const fn stopdl1f(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "D-PHY_RX receiver data lane 1 in stop state."]
        #[inline(always)]
        pub const fn set_stopdl1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "D-PHY_RX receiver ultra-low-power state (not) active on data lane 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ulpndl1f(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "D-PHY_RX receiver ultra-low-power state (not) active on data lane 1."]
        #[inline(always)]
        pub const fn set_ulpndl1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "D-PHY_RX receiver in stop state for the clock lane."]
        #[must_use]
        #[inline(always)]
        pub const fn stopclf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "D-PHY_RX receiver in stop state for the clock lane."]
        #[inline(always)]
        pub const fn set_stopclf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "D-PHY_RX receiver ULP state (not) active."]
        #[must_use]
        #[inline(always)]
        pub const fn ulpnactf(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "D-PHY_RX receiver ULP state (not) active."]
        #[inline(always)]
        pub const fn set_ulpnactf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "D-PHY_RX receiver Ultra-Low power state (not) on clock lane."]
        #[must_use]
        #[inline(always)]
        pub const fn ulpnclf(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "D-PHY_RX receiver Ultra-Low power state (not) on clock lane."]
        #[inline(always)]
        pub const fn set_ulpnclf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "D-PHY_RX receiver clock active flag."]
        #[must_use]
        #[inline(always)]
        pub const fn actclf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "D-PHY_RX receiver clock active flag."]
        #[inline(always)]
        pub const fn set_actclf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("esotdl0f", &self.esotdl0f())
                .field("esotsyncdl0f", &self.esotsyncdl0f())
                .field("eescdl0f", &self.eescdl0f())
                .field("esyncescdl0f", &self.esyncescdl0f())
                .field("ectrldl0f", &self.ectrldl0f())
                .field("esotdl1f", &self.esotdl1f())
                .field("esotsyncdl1f", &self.esotsyncdl1f())
                .field("eescdl1f", &self.eescdl1f())
                .field("esyncescdl1f", &self.esyncescdl1f())
                .field("ectrldl1f", &self.ectrldl1f())
                .field("actdl0f", &self.actdl0f())
                .field("syncdl0f", &self.syncdl0f())
                .field("skcaldl0f", &self.skcaldl0f())
                .field("stopdl0f", &self.stopdl0f())
                .field("ulpndl0f", &self.ulpndl0f())
                .field("actdl1f", &self.actdl1f())
                .field("syncdl1f", &self.syncdl1f())
                .field("skcaldl1f", &self.skcaldl1f())
                .field("stopdl1f", &self.stopdl1f())
                .field("ulpndl1f", &self.ulpndl1f())
                .field("stopclf", &self.stopclf())
                .field("ulpnactf", &self.ulpnactf())
                .field("ulpnclf", &self.ulpnclf())
                .field("actclf", &self.actclf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Sr1 {{ esotdl0f: {=bool:?}, esotsyncdl0f: {=bool:?}, eescdl0f: {=bool:?}, esyncescdl0f: {=bool:?}, ectrldl0f: {=bool:?}, esotdl1f: {=bool:?}, esotsyncdl1f: {=bool:?}, eescdl1f: {=bool:?}, esyncescdl1f: {=bool:?}, ectrldl1f: {=bool:?}, actdl0f: {=bool:?}, syncdl0f: {=bool:?}, skcaldl0f: {=bool:?}, stopdl0f: {=bool:?}, ulpndl0f: {=bool:?}, actdl1f: {=bool:?}, syncdl1f: {=bool:?}, skcaldl1f: {=bool:?}, stopdl1f: {=bool:?}, ulpndl1f: {=bool:?}, stopclf: {=bool:?}, ulpnactf: {=bool:?}, ulpnclf: {=bool:?}, actclf: {=bool:?} }}" , self . esotdl0f () , self . esotsyncdl0f () , self . eescdl0f () , self . esyncescdl0f () , self . ectrldl0f () , self . esotdl1f () , self . esotsyncdl1f () , self . eescdl1f () , self . esyncescdl1f () , self . ectrldl1f () , self . actdl0f () , self . syncdl0f () , self . skcaldl0f () , self . stopdl0f () , self . ulpndl0f () , self . actdl1f () , self . syncdl1f () , self . skcaldl1f () , self . stopdl1f () , self . ulpndl1f () , self . stopclf () , self . ulpnactf () , self . ulpnclf () , self . actclf ())
        }
    }
    #[doc = "CSI-2 Host timer 0 configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tim0cfgr(pub u32);
    impl Tim0cfgr {
        #[doc = "Clock cycle counter."]
        #[must_use]
        #[inline(always)]
        pub const fn count(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "Clock cycle counter."]
        #[inline(always)]
        pub const fn set_count(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
        }
    }
    impl Default for Tim0cfgr {
        #[inline(always)]
        fn default() -> Tim0cfgr {
            Tim0cfgr(0)
        }
    }
    impl core::fmt::Debug for Tim0cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tim0cfgr").field("count", &self.count()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tim0cfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Tim0cfgr {{ count: {=u32:?} }}", self.count())
        }
    }
    #[doc = "CSI-2 Host timer 1 configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tim1cfgr(pub u32);
    impl Tim1cfgr {
        #[doc = "Clock cycle counter."]
        #[must_use]
        #[inline(always)]
        pub const fn count(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "Clock cycle counter."]
        #[inline(always)]
        pub const fn set_count(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
        }
    }
    impl Default for Tim1cfgr {
        #[inline(always)]
        fn default() -> Tim1cfgr {
            Tim1cfgr(0)
        }
    }
    impl core::fmt::Debug for Tim1cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tim1cfgr").field("count", &self.count()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tim1cfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Tim1cfgr {{ count: {=u32:?} }}", self.count())
        }
    }
    #[doc = "CSI-2 Host timer 2 configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tim2cfgr(pub u32);
    impl Tim2cfgr {
        #[doc = "Clock cycle counter."]
        #[must_use]
        #[inline(always)]
        pub const fn count(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "Clock cycle counter."]
        #[inline(always)]
        pub const fn set_count(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
        }
    }
    impl Default for Tim2cfgr {
        #[inline(always)]
        fn default() -> Tim2cfgr {
            Tim2cfgr(0)
        }
    }
    impl core::fmt::Debug for Tim2cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tim2cfgr").field("count", &self.count()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tim2cfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Tim2cfgr {{ count: {=u32:?} }}", self.count())
        }
    }
    #[doc = "CSI-2 Host timer 3 configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tim3cfgr(pub u32);
    impl Tim3cfgr {
        #[doc = "Clock cycle counter."]
        #[must_use]
        #[inline(always)]
        pub const fn count(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "Clock cycle counter."]
        #[inline(always)]
        pub const fn set_count(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
        }
    }
    impl Default for Tim3cfgr {
        #[inline(always)]
        fn default() -> Tim3cfgr {
            Tim3cfgr(0)
        }
    }
    impl core::fmt::Debug for Tim3cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tim3cfgr").field("count", &self.count()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tim3cfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Tim3cfgr {{ count: {=u32:?} }}", self.count())
        }
    }
    #[doc = "CSI-2 Host virtual channel 0 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vc0cfgr1(pub u32);
    impl Vc0cfgr1 {
        #[doc = "All data types enable for the virtual channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn alldt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "All data types enable for the virtual channel x."]
        #[inline(always)]
        pub const fn set_alldt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Data type 0 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dt0en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Data type 0 enable."]
        #[inline(always)]
        pub const fn set_dt0en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Data type 1 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dt1en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Data type 1 enable."]
        #[inline(always)]
        pub const fn set_dt1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Data type 2 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dt2en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Data type 2 enable."]
        #[inline(always)]
        pub const fn set_dt2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Data type 3 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dt3en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Data type 3 enable."]
        #[inline(always)]
        pub const fn set_dt3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Data type 4 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dt4en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Data type 4 enable."]
        #[inline(always)]
        pub const fn set_dt4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Data type 5 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dt5en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Data type 5 enable."]
        #[inline(always)]
        pub const fn set_dt5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Data type 6 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dt6en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Data type 6 enable."]
        #[inline(always)]
        pub const fn set_dt6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Common format for all data types."]
        #[must_use]
        #[inline(always)]
        pub const fn cdtft(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Common format for all data types."]
        #[inline(always)]
        pub const fn set_cdtft(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Data type 0 class selection for virtual channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn dt0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type 0 class selection for virtual channel x."]
        #[inline(always)]
        pub const fn set_dt0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "Data type 0 format."]
        #[must_use]
        #[inline(always)]
        pub const fn dt0ft(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Data type 0 format."]
        #[inline(always)]
        pub const fn set_dt0ft(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for Vc0cfgr1 {
        #[inline(always)]
        fn default() -> Vc0cfgr1 {
            Vc0cfgr1(0)
        }
    }
    impl core::fmt::Debug for Vc0cfgr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vc0cfgr1")
                .field("alldt", &self.alldt())
                .field("dt0en", &self.dt0en())
                .field("dt1en", &self.dt1en())
                .field("dt2en", &self.dt2en())
                .field("dt3en", &self.dt3en())
                .field("dt4en", &self.dt4en())
                .field("dt5en", &self.dt5en())
                .field("dt6en", &self.dt6en())
                .field("cdtft", &self.cdtft())
                .field("dt0", &self.dt0())
                .field("dt0ft", &self.dt0ft())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vc0cfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Vc0cfgr1 {{ alldt: {=bool:?}, dt0en: {=bool:?}, dt1en: {=bool:?}, dt2en: {=bool:?}, dt3en: {=bool:?}, dt4en: {=bool:?}, dt5en: {=bool:?}, dt6en: {=bool:?}, cdtft: {=u8:?}, dt0: {=u8:?}, dt0ft: {=u8:?} }}" , self . alldt () , self . dt0en () , self . dt1en () , self . dt2en () , self . dt3en () , self . dt4en () , self . dt5en () , self . dt6en () , self . cdtft () , self . dt0 () , self . dt0ft ())
        }
    }
    #[doc = "CSI-2 Host virtual channel 0 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vc0cfgr2(pub u32);
    impl Vc0cfgr2 {
        #[doc = "Data type 1 class selection for virtual channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn dt1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type 1 class selection for virtual channel x."]
        #[inline(always)]
        pub const fn set_dt1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Data type 1 format."]
        #[must_use]
        #[inline(always)]
        pub const fn dt1ft(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Data type 1 format."]
        #[inline(always)]
        pub const fn set_dt1ft(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Data type 2 class selection for virtual channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn dt2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type 2 class selection for virtual channel x."]
        #[inline(always)]
        pub const fn set_dt2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "Data type 2 format."]
        #[must_use]
        #[inline(always)]
        pub const fn dt2ft(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Data type 2 format."]
        #[inline(always)]
        pub const fn set_dt2ft(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for Vc0cfgr2 {
        #[inline(always)]
        fn default() -> Vc0cfgr2 {
            Vc0cfgr2(0)
        }
    }
    impl core::fmt::Debug for Vc0cfgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vc0cfgr2")
                .field("dt1", &self.dt1())
                .field("dt1ft", &self.dt1ft())
                .field("dt2", &self.dt2())
                .field("dt2ft", &self.dt2ft())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vc0cfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Vc0cfgr2 {{ dt1: {=u8:?}, dt1ft: {=u8:?}, dt2: {=u8:?}, dt2ft: {=u8:?} }}",
                self.dt1(),
                self.dt1ft(),
                self.dt2(),
                self.dt2ft()
            )
        }
    }
    #[doc = "CSI-2 Host virtual channel 0 configuration register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vc0cfgr3(pub u32);
    impl Vc0cfgr3 {
        #[doc = "Data type 3 class selection for virtual channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn dt3(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type 3 class selection for virtual channel x."]
        #[inline(always)]
        pub const fn set_dt3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Data type 3 format."]
        #[must_use]
        #[inline(always)]
        pub const fn dt3ft(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Data type 3 format."]
        #[inline(always)]
        pub const fn set_dt3ft(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Data type 4 class selection for virtual channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn dt4(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type 4 class selection for virtual channel x."]
        #[inline(always)]
        pub const fn set_dt4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "Data type 4 format."]
        #[must_use]
        #[inline(always)]
        pub const fn dt4ft(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Data type 4 format."]
        #[inline(always)]
        pub const fn set_dt4ft(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for Vc0cfgr3 {
        #[inline(always)]
        fn default() -> Vc0cfgr3 {
            Vc0cfgr3(0)
        }
    }
    impl core::fmt::Debug for Vc0cfgr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vc0cfgr3")
                .field("dt3", &self.dt3())
                .field("dt3ft", &self.dt3ft())
                .field("dt4", &self.dt4())
                .field("dt4ft", &self.dt4ft())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vc0cfgr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Vc0cfgr3 {{ dt3: {=u8:?}, dt3ft: {=u8:?}, dt4: {=u8:?}, dt4ft: {=u8:?} }}",
                self.dt3(),
                self.dt3ft(),
                self.dt4(),
                self.dt4ft()
            )
        }
    }
    #[doc = "CSI-2 Host virtual channel 0 configuration register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vc0cfgr4(pub u32);
    impl Vc0cfgr4 {
        #[doc = "Data type 5 class selection for virtual channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn dt5(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type 5 class selection for virtual channel x."]
        #[inline(always)]
        pub const fn set_dt5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Data type 5 format."]
        #[must_use]
        #[inline(always)]
        pub const fn dt5ft(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Data type 5 format."]
        #[inline(always)]
        pub const fn set_dt5ft(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Data type 6 class selection for virtual channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn dt6(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type 6 class selection for virtual channel x."]
        #[inline(always)]
        pub const fn set_dt6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "Data type 6 format."]
        #[must_use]
        #[inline(always)]
        pub const fn dt6ft(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Data type 6 format."]
        #[inline(always)]
        pub const fn set_dt6ft(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for Vc0cfgr4 {
        #[inline(always)]
        fn default() -> Vc0cfgr4 {
            Vc0cfgr4(0)
        }
    }
    impl core::fmt::Debug for Vc0cfgr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vc0cfgr4")
                .field("dt5", &self.dt5())
                .field("dt5ft", &self.dt5ft())
                .field("dt6", &self.dt6())
                .field("dt6ft", &self.dt6ft())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vc0cfgr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Vc0cfgr4 {{ dt5: {=u8:?}, dt5ft: {=u8:?}, dt6: {=u8:?}, dt6ft: {=u8:?} }}",
                self.dt5(),
                self.dt5ft(),
                self.dt6(),
                self.dt6ft()
            )
        }
    }
    #[doc = "CSI-2 Host virtual channel 1 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vc1cfgr1(pub u32);
    impl Vc1cfgr1 {
        #[doc = "All data types enable for the virtual channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn alldt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "All data types enable for the virtual channel x."]
        #[inline(always)]
        pub const fn set_alldt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Data type 0 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dt0en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Data type 0 enable."]
        #[inline(always)]
        pub const fn set_dt0en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Data type 1 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dt1en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Data type 1 enable."]
        #[inline(always)]
        pub const fn set_dt1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Data type 2 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dt2en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Data type 2 enable."]
        #[inline(always)]
        pub const fn set_dt2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Data type 3 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dt3en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Data type 3 enable."]
        #[inline(always)]
        pub const fn set_dt3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Data type 4 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dt4en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Data type 4 enable."]
        #[inline(always)]
        pub const fn set_dt4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Data type 5 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dt5en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Data type 5 enable."]
        #[inline(always)]
        pub const fn set_dt5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Data type 6 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dt6en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Data type 6 enable."]
        #[inline(always)]
        pub const fn set_dt6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Common format for all data types."]
        #[must_use]
        #[inline(always)]
        pub const fn cdtft(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Common format for all data types."]
        #[inline(always)]
        pub const fn set_cdtft(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Data type 0 class selection for virtual channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn dt0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type 0 class selection for virtual channel x."]
        #[inline(always)]
        pub const fn set_dt0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "Data type 0 format."]
        #[must_use]
        #[inline(always)]
        pub const fn dt0ft(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Data type 0 format."]
        #[inline(always)]
        pub const fn set_dt0ft(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for Vc1cfgr1 {
        #[inline(always)]
        fn default() -> Vc1cfgr1 {
            Vc1cfgr1(0)
        }
    }
    impl core::fmt::Debug for Vc1cfgr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vc1cfgr1")
                .field("alldt", &self.alldt())
                .field("dt0en", &self.dt0en())
                .field("dt1en", &self.dt1en())
                .field("dt2en", &self.dt2en())
                .field("dt3en", &self.dt3en())
                .field("dt4en", &self.dt4en())
                .field("dt5en", &self.dt5en())
                .field("dt6en", &self.dt6en())
                .field("cdtft", &self.cdtft())
                .field("dt0", &self.dt0())
                .field("dt0ft", &self.dt0ft())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vc1cfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Vc1cfgr1 {{ alldt: {=bool:?}, dt0en: {=bool:?}, dt1en: {=bool:?}, dt2en: {=bool:?}, dt3en: {=bool:?}, dt4en: {=bool:?}, dt5en: {=bool:?}, dt6en: {=bool:?}, cdtft: {=u8:?}, dt0: {=u8:?}, dt0ft: {=u8:?} }}" , self . alldt () , self . dt0en () , self . dt1en () , self . dt2en () , self . dt3en () , self . dt4en () , self . dt5en () , self . dt6en () , self . cdtft () , self . dt0 () , self . dt0ft ())
        }
    }
    #[doc = "CSI-2 Host virtual channel 1 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vc1cfgr2(pub u32);
    impl Vc1cfgr2 {
        #[doc = "Data type 1 class selection for virtual channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn dt1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type 1 class selection for virtual channel x."]
        #[inline(always)]
        pub const fn set_dt1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Data type 1 format."]
        #[must_use]
        #[inline(always)]
        pub const fn dt1ft(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Data type 1 format."]
        #[inline(always)]
        pub const fn set_dt1ft(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Data type 2 class selection for virtual channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn dt2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type 2 class selection for virtual channel x."]
        #[inline(always)]
        pub const fn set_dt2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "Data type 2 format."]
        #[must_use]
        #[inline(always)]
        pub const fn dt2ft(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Data type 2 format."]
        #[inline(always)]
        pub const fn set_dt2ft(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for Vc1cfgr2 {
        #[inline(always)]
        fn default() -> Vc1cfgr2 {
            Vc1cfgr2(0)
        }
    }
    impl core::fmt::Debug for Vc1cfgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vc1cfgr2")
                .field("dt1", &self.dt1())
                .field("dt1ft", &self.dt1ft())
                .field("dt2", &self.dt2())
                .field("dt2ft", &self.dt2ft())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vc1cfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Vc1cfgr2 {{ dt1: {=u8:?}, dt1ft: {=u8:?}, dt2: {=u8:?}, dt2ft: {=u8:?} }}",
                self.dt1(),
                self.dt1ft(),
                self.dt2(),
                self.dt2ft()
            )
        }
    }
    #[doc = "CSI-2 Host virtual channel 1 configuration register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vc1cfgr3(pub u32);
    impl Vc1cfgr3 {
        #[doc = "Data type 3 class selection for virtual channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn dt3(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type 3 class selection for virtual channel x."]
        #[inline(always)]
        pub const fn set_dt3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Data type 3 format."]
        #[must_use]
        #[inline(always)]
        pub const fn dt3ft(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Data type 3 format."]
        #[inline(always)]
        pub const fn set_dt3ft(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Data type 4 class selection for virtual channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn dt4(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type 4 class selection for virtual channel x."]
        #[inline(always)]
        pub const fn set_dt4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "Data type 4 format."]
        #[must_use]
        #[inline(always)]
        pub const fn dt4ft(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Data type 4 format."]
        #[inline(always)]
        pub const fn set_dt4ft(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for Vc1cfgr3 {
        #[inline(always)]
        fn default() -> Vc1cfgr3 {
            Vc1cfgr3(0)
        }
    }
    impl core::fmt::Debug for Vc1cfgr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vc1cfgr3")
                .field("dt3", &self.dt3())
                .field("dt3ft", &self.dt3ft())
                .field("dt4", &self.dt4())
                .field("dt4ft", &self.dt4ft())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vc1cfgr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Vc1cfgr3 {{ dt3: {=u8:?}, dt3ft: {=u8:?}, dt4: {=u8:?}, dt4ft: {=u8:?} }}",
                self.dt3(),
                self.dt3ft(),
                self.dt4(),
                self.dt4ft()
            )
        }
    }
    #[doc = "CSI-2 Host virtual channel 1 configuration register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vc1cfgr4(pub u32);
    impl Vc1cfgr4 {
        #[doc = "Data type 5 class selection for virtual channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn dt5(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type 5 class selection for virtual channel x."]
        #[inline(always)]
        pub const fn set_dt5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Data type 5 format."]
        #[must_use]
        #[inline(always)]
        pub const fn dt5ft(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Data type 5 format."]
        #[inline(always)]
        pub const fn set_dt5ft(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Data type 6 class selection for virtual channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn dt6(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type 6 class selection for virtual channel x."]
        #[inline(always)]
        pub const fn set_dt6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "Data type 6 format."]
        #[must_use]
        #[inline(always)]
        pub const fn dt6ft(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Data type 6 format."]
        #[inline(always)]
        pub const fn set_dt6ft(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for Vc1cfgr4 {
        #[inline(always)]
        fn default() -> Vc1cfgr4 {
            Vc1cfgr4(0)
        }
    }
    impl core::fmt::Debug for Vc1cfgr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vc1cfgr4")
                .field("dt5", &self.dt5())
                .field("dt5ft", &self.dt5ft())
                .field("dt6", &self.dt6())
                .field("dt6ft", &self.dt6ft())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vc1cfgr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Vc1cfgr4 {{ dt5: {=u8:?}, dt5ft: {=u8:?}, dt6: {=u8:?}, dt6ft: {=u8:?} }}",
                self.dt5(),
                self.dt5ft(),
                self.dt6(),
                self.dt6ft()
            )
        }
    }
    #[doc = "CSI-2 Host virtual channel 2 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vc2cfgr1(pub u32);
    impl Vc2cfgr1 {
        #[doc = "All data types enable for the virtual channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn alldt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "All data types enable for the virtual channel x."]
        #[inline(always)]
        pub const fn set_alldt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Data type 0 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dt0en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Data type 0 enable."]
        #[inline(always)]
        pub const fn set_dt0en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Data type 1 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dt1en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Data type 1 enable."]
        #[inline(always)]
        pub const fn set_dt1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Data type 2 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dt2en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Data type 2 enable."]
        #[inline(always)]
        pub const fn set_dt2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Data type 3 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dt3en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Data type 3 enable."]
        #[inline(always)]
        pub const fn set_dt3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Data type 4 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dt4en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Data type 4 enable."]
        #[inline(always)]
        pub const fn set_dt4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Data type 5 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dt5en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Data type 5 enable."]
        #[inline(always)]
        pub const fn set_dt5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Data type 6 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dt6en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Data type 6 enable."]
        #[inline(always)]
        pub const fn set_dt6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Common format for all data types."]
        #[must_use]
        #[inline(always)]
        pub const fn cdtft(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Common format for all data types."]
        #[inline(always)]
        pub const fn set_cdtft(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Data type 0 class selection for virtual channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn dt0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type 0 class selection for virtual channel x."]
        #[inline(always)]
        pub const fn set_dt0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "Data type 0 format."]
        #[must_use]
        #[inline(always)]
        pub const fn dt0ft(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Data type 0 format."]
        #[inline(always)]
        pub const fn set_dt0ft(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for Vc2cfgr1 {
        #[inline(always)]
        fn default() -> Vc2cfgr1 {
            Vc2cfgr1(0)
        }
    }
    impl core::fmt::Debug for Vc2cfgr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vc2cfgr1")
                .field("alldt", &self.alldt())
                .field("dt0en", &self.dt0en())
                .field("dt1en", &self.dt1en())
                .field("dt2en", &self.dt2en())
                .field("dt3en", &self.dt3en())
                .field("dt4en", &self.dt4en())
                .field("dt5en", &self.dt5en())
                .field("dt6en", &self.dt6en())
                .field("cdtft", &self.cdtft())
                .field("dt0", &self.dt0())
                .field("dt0ft", &self.dt0ft())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vc2cfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Vc2cfgr1 {{ alldt: {=bool:?}, dt0en: {=bool:?}, dt1en: {=bool:?}, dt2en: {=bool:?}, dt3en: {=bool:?}, dt4en: {=bool:?}, dt5en: {=bool:?}, dt6en: {=bool:?}, cdtft: {=u8:?}, dt0: {=u8:?}, dt0ft: {=u8:?} }}" , self . alldt () , self . dt0en () , self . dt1en () , self . dt2en () , self . dt3en () , self . dt4en () , self . dt5en () , self . dt6en () , self . cdtft () , self . dt0 () , self . dt0ft ())
        }
    }
    #[doc = "CSI-2 Host virtual channel 2 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vc2cfgr2(pub u32);
    impl Vc2cfgr2 {
        #[doc = "Data type 1 class selection for virtual channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn dt1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type 1 class selection for virtual channel x."]
        #[inline(always)]
        pub const fn set_dt1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Data type 1 format."]
        #[must_use]
        #[inline(always)]
        pub const fn dt1ft(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Data type 1 format."]
        #[inline(always)]
        pub const fn set_dt1ft(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Data type 2 class selection for virtual channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn dt2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type 2 class selection for virtual channel x."]
        #[inline(always)]
        pub const fn set_dt2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "Data type 2 format."]
        #[must_use]
        #[inline(always)]
        pub const fn dt2ft(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Data type 2 format."]
        #[inline(always)]
        pub const fn set_dt2ft(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for Vc2cfgr2 {
        #[inline(always)]
        fn default() -> Vc2cfgr2 {
            Vc2cfgr2(0)
        }
    }
    impl core::fmt::Debug for Vc2cfgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vc2cfgr2")
                .field("dt1", &self.dt1())
                .field("dt1ft", &self.dt1ft())
                .field("dt2", &self.dt2())
                .field("dt2ft", &self.dt2ft())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vc2cfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Vc2cfgr2 {{ dt1: {=u8:?}, dt1ft: {=u8:?}, dt2: {=u8:?}, dt2ft: {=u8:?} }}",
                self.dt1(),
                self.dt1ft(),
                self.dt2(),
                self.dt2ft()
            )
        }
    }
    #[doc = "CSI-2 Host virtual channel 2 configuration register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vc2cfgr3(pub u32);
    impl Vc2cfgr3 {
        #[doc = "Data type 3 class selection for virtual channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn dt3(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type 3 class selection for virtual channel x."]
        #[inline(always)]
        pub const fn set_dt3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Data type 3 format."]
        #[must_use]
        #[inline(always)]
        pub const fn dt3ft(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Data type 3 format."]
        #[inline(always)]
        pub const fn set_dt3ft(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Data type 4 class selection for virtual channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn dt4(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type 4 class selection for virtual channel x."]
        #[inline(always)]
        pub const fn set_dt4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "Data type 4 format."]
        #[must_use]
        #[inline(always)]
        pub const fn dt4ft(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Data type 4 format."]
        #[inline(always)]
        pub const fn set_dt4ft(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for Vc2cfgr3 {
        #[inline(always)]
        fn default() -> Vc2cfgr3 {
            Vc2cfgr3(0)
        }
    }
    impl core::fmt::Debug for Vc2cfgr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vc2cfgr3")
                .field("dt3", &self.dt3())
                .field("dt3ft", &self.dt3ft())
                .field("dt4", &self.dt4())
                .field("dt4ft", &self.dt4ft())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vc2cfgr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Vc2cfgr3 {{ dt3: {=u8:?}, dt3ft: {=u8:?}, dt4: {=u8:?}, dt4ft: {=u8:?} }}",
                self.dt3(),
                self.dt3ft(),
                self.dt4(),
                self.dt4ft()
            )
        }
    }
    #[doc = "CSI-2 Host virtual channel 2 configuration register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vc2cfgr4(pub u32);
    impl Vc2cfgr4 {
        #[doc = "Data type 5 class selection for virtual channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn dt5(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type 5 class selection for virtual channel x."]
        #[inline(always)]
        pub const fn set_dt5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Data type 5 format."]
        #[must_use]
        #[inline(always)]
        pub const fn dt5ft(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Data type 5 format."]
        #[inline(always)]
        pub const fn set_dt5ft(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Data type 6 class selection for virtual channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn dt6(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type 6 class selection for virtual channel x."]
        #[inline(always)]
        pub const fn set_dt6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "Data type 6 format."]
        #[must_use]
        #[inline(always)]
        pub const fn dt6ft(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Data type 6 format."]
        #[inline(always)]
        pub const fn set_dt6ft(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for Vc2cfgr4 {
        #[inline(always)]
        fn default() -> Vc2cfgr4 {
            Vc2cfgr4(0)
        }
    }
    impl core::fmt::Debug for Vc2cfgr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vc2cfgr4")
                .field("dt5", &self.dt5())
                .field("dt5ft", &self.dt5ft())
                .field("dt6", &self.dt6())
                .field("dt6ft", &self.dt6ft())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vc2cfgr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Vc2cfgr4 {{ dt5: {=u8:?}, dt5ft: {=u8:?}, dt6: {=u8:?}, dt6ft: {=u8:?} }}",
                self.dt5(),
                self.dt5ft(),
                self.dt6(),
                self.dt6ft()
            )
        }
    }
    #[doc = "CSI-2 Host virtual channel 3 configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vc3cfgr1(pub u32);
    impl Vc3cfgr1 {
        #[doc = "All data types enable for the virtual channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn alldt(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "All data types enable for the virtual channel x."]
        #[inline(always)]
        pub const fn set_alldt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Data type 0 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dt0en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Data type 0 enable."]
        #[inline(always)]
        pub const fn set_dt0en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Data type 1 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dt1en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Data type 1 enable."]
        #[inline(always)]
        pub const fn set_dt1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Data type 2 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dt2en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Data type 2 enable."]
        #[inline(always)]
        pub const fn set_dt2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Data type 3 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dt3en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Data type 3 enable."]
        #[inline(always)]
        pub const fn set_dt3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Data type 4 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dt4en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Data type 4 enable."]
        #[inline(always)]
        pub const fn set_dt4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Data type 5 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dt5en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Data type 5 enable."]
        #[inline(always)]
        pub const fn set_dt5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Data type 6 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dt6en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Data type 6 enable."]
        #[inline(always)]
        pub const fn set_dt6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Common format for all data types."]
        #[must_use]
        #[inline(always)]
        pub const fn cdtft(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Common format for all data types."]
        #[inline(always)]
        pub const fn set_cdtft(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Data type 0 class selection for virtual channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn dt0(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type 0 class selection for virtual channel x."]
        #[inline(always)]
        pub const fn set_dt0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "Data type 0 format."]
        #[must_use]
        #[inline(always)]
        pub const fn dt0ft(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Data type 0 format."]
        #[inline(always)]
        pub const fn set_dt0ft(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for Vc3cfgr1 {
        #[inline(always)]
        fn default() -> Vc3cfgr1 {
            Vc3cfgr1(0)
        }
    }
    impl core::fmt::Debug for Vc3cfgr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vc3cfgr1")
                .field("alldt", &self.alldt())
                .field("dt0en", &self.dt0en())
                .field("dt1en", &self.dt1en())
                .field("dt2en", &self.dt2en())
                .field("dt3en", &self.dt3en())
                .field("dt4en", &self.dt4en())
                .field("dt5en", &self.dt5en())
                .field("dt6en", &self.dt6en())
                .field("cdtft", &self.cdtft())
                .field("dt0", &self.dt0())
                .field("dt0ft", &self.dt0ft())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vc3cfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Vc3cfgr1 {{ alldt: {=bool:?}, dt0en: {=bool:?}, dt1en: {=bool:?}, dt2en: {=bool:?}, dt3en: {=bool:?}, dt4en: {=bool:?}, dt5en: {=bool:?}, dt6en: {=bool:?}, cdtft: {=u8:?}, dt0: {=u8:?}, dt0ft: {=u8:?} }}" , self . alldt () , self . dt0en () , self . dt1en () , self . dt2en () , self . dt3en () , self . dt4en () , self . dt5en () , self . dt6en () , self . cdtft () , self . dt0 () , self . dt0ft ())
        }
    }
    #[doc = "CSI-2 Host virtual channel 3 configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vc3cfgr2(pub u32);
    impl Vc3cfgr2 {
        #[doc = "Data type 1 class selection for virtual channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn dt1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type 1 class selection for virtual channel x."]
        #[inline(always)]
        pub const fn set_dt1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Data type 1 format."]
        #[must_use]
        #[inline(always)]
        pub const fn dt1ft(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Data type 1 format."]
        #[inline(always)]
        pub const fn set_dt1ft(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Data type 2 class selection for virtual channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn dt2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type 2 class selection for virtual channel x."]
        #[inline(always)]
        pub const fn set_dt2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "Data type 2 format."]
        #[must_use]
        #[inline(always)]
        pub const fn dt2ft(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Data type 2 format."]
        #[inline(always)]
        pub const fn set_dt2ft(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for Vc3cfgr2 {
        #[inline(always)]
        fn default() -> Vc3cfgr2 {
            Vc3cfgr2(0)
        }
    }
    impl core::fmt::Debug for Vc3cfgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vc3cfgr2")
                .field("dt1", &self.dt1())
                .field("dt1ft", &self.dt1ft())
                .field("dt2", &self.dt2())
                .field("dt2ft", &self.dt2ft())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vc3cfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Vc3cfgr2 {{ dt1: {=u8:?}, dt1ft: {=u8:?}, dt2: {=u8:?}, dt2ft: {=u8:?} }}",
                self.dt1(),
                self.dt1ft(),
                self.dt2(),
                self.dt2ft()
            )
        }
    }
    #[doc = "CSI-2 Host virtual channel 3 configuration register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vc3cfgr3(pub u32);
    impl Vc3cfgr3 {
        #[doc = "Data type 3 class selection for virtual channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn dt3(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type 3 class selection for virtual channel x."]
        #[inline(always)]
        pub const fn set_dt3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Data type 3 format."]
        #[must_use]
        #[inline(always)]
        pub const fn dt3ft(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Data type 3 format."]
        #[inline(always)]
        pub const fn set_dt3ft(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Data type 4 class selection for virtual channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn dt4(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type 4 class selection for virtual channel x."]
        #[inline(always)]
        pub const fn set_dt4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "Data type 4 format."]
        #[must_use]
        #[inline(always)]
        pub const fn dt4ft(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Data type 4 format."]
        #[inline(always)]
        pub const fn set_dt4ft(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for Vc3cfgr3 {
        #[inline(always)]
        fn default() -> Vc3cfgr3 {
            Vc3cfgr3(0)
        }
    }
    impl core::fmt::Debug for Vc3cfgr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vc3cfgr3")
                .field("dt3", &self.dt3())
                .field("dt3ft", &self.dt3ft())
                .field("dt4", &self.dt4())
                .field("dt4ft", &self.dt4ft())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vc3cfgr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Vc3cfgr3 {{ dt3: {=u8:?}, dt3ft: {=u8:?}, dt4: {=u8:?}, dt4ft: {=u8:?} }}",
                self.dt3(),
                self.dt3ft(),
                self.dt4(),
                self.dt4ft()
            )
        }
    }
    #[doc = "CSI-2 Host virtual channel 3 configuration register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vc3cfgr4(pub u32);
    impl Vc3cfgr4 {
        #[doc = "Data type 5 class selection for virtual channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn dt5(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type 5 class selection for virtual channel x."]
        #[inline(always)]
        pub const fn set_dt5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
        #[doc = "Data type 5 format."]
        #[must_use]
        #[inline(always)]
        pub const fn dt5ft(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Data type 5 format."]
        #[inline(always)]
        pub const fn set_dt5ft(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Data type 6 class selection for virtual channel x."]
        #[must_use]
        #[inline(always)]
        pub const fn dt6(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "Data type 6 class selection for virtual channel x."]
        #[inline(always)]
        pub const fn set_dt6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "Data type 6 format."]
        #[must_use]
        #[inline(always)]
        pub const fn dt6ft(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x1f;
            val as u8
        }
        #[doc = "Data type 6 format."]
        #[inline(always)]
        pub const fn set_dt6ft(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
        }
    }
    impl Default for Vc3cfgr4 {
        #[inline(always)]
        fn default() -> Vc3cfgr4 {
            Vc3cfgr4(0)
        }
    }
    impl core::fmt::Debug for Vc3cfgr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vc3cfgr4")
                .field("dt5", &self.dt5())
                .field("dt5ft", &self.dt5ft())
                .field("dt6", &self.dt6())
                .field("dt6ft", &self.dt6ft())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vc3cfgr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Vc3cfgr4 {{ dt5: {=u8:?}, dt5ft: {=u8:?}, dt6: {=u8:?}, dt6ft: {=u8:?} }}",
                self.dt5(),
                self.dt5ft(),
                self.dt6(),
                self.dt6ft()
            )
        }
    }
    #[doc = "CSI-2 Host watchdog register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wdr(pub u32);
    impl Wdr {
        #[doc = "Watchdog counter."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Watchdog counter."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Wdr {
        #[inline(always)]
        fn default() -> Wdr {
            Wdr(0)
        }
    }
    impl core::fmt::Debug for Wdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wdr").field("cnt", &self.cnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wdr {{ cnt: {=u32:?} }}", self.cnt())
        }
    }
}
