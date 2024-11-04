#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "OctoSPI IO Manager"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Octospim {
    ptr: *mut u8,
}
unsafe impl Send for Octospim {}
unsafe impl Sync for Octospim {}
impl Octospim {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "OctoSPI IO Manager Port 1 Configuration Register"]
    #[inline(always)]
    pub const fn p1cr(self) -> crate::common::Reg<regs::P1cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "OctoSPI IO Manager Port 2 Configuration Register"]
    #[inline(always)]
    pub const fn p2cr(self) -> crate::common::Reg<regs::P2cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
pub mod regs {
    #[doc = "control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Multiplexed mode enable"]
        #[inline(always)]
        pub const fn muxen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Multiplexed mode enable"]
        #[inline(always)]
        pub fn set_muxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "REQ to ACK time"]
        #[inline(always)]
        pub const fn req2ack_time(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "REQ to ACK time"]
        #[inline(always)]
        pub fn set_req2ack_time(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    #[doc = "OctoSPI IO Manager Port 1 Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P1cr(pub u32);
    impl P1cr {
        #[doc = "CLK/CLK Enable for Port"]
        #[inline(always)]
        pub const fn clken(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CLK/CLK Enable for Port"]
        #[inline(always)]
        pub fn set_clken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CLK/CLK Source for Port"]
        #[inline(always)]
        pub const fn clksrc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CLK/CLK Source for Port"]
        #[inline(always)]
        pub fn set_clksrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DQS Enable for Port"]
        #[inline(always)]
        pub const fn dqsen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DQS Enable for Port"]
        #[inline(always)]
        pub fn set_dqsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "DQS Source for Port"]
        #[inline(always)]
        pub const fn dqssrc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "DQS Source for Port"]
        #[inline(always)]
        pub fn set_dqssrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "CS Enable for Port"]
        #[inline(always)]
        pub const fn ncsen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "CS Enable for Port"]
        #[inline(always)]
        pub fn set_ncsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CS Source for Port"]
        #[inline(always)]
        pub const fn ncssrc(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "CS Source for Port"]
        #[inline(always)]
        pub fn set_ncssrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Enable for Port"]
        #[inline(always)]
        pub const fn iolen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Enable for Port"]
        #[inline(always)]
        pub fn set_iolen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Source for Port"]
        #[inline(always)]
        pub const fn iolsrc(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x03;
            val as u8
        }
        #[doc = "Source for Port"]
        #[inline(always)]
        pub fn set_iolsrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
        }
        #[doc = "Enable for Port n"]
        #[inline(always)]
        pub const fn iohen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Enable for Port n"]
        #[inline(always)]
        pub fn set_iohen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Source for Port"]
        #[inline(always)]
        pub const fn iohsrc(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x03;
            val as u8
        }
        #[doc = "Source for Port"]
        #[inline(always)]
        pub fn set_iohsrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
        }
    }
    impl Default for P1cr {
        #[inline(always)]
        fn default() -> P1cr {
            P1cr(0)
        }
    }
    #[doc = "OctoSPI IO Manager Port 2 Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct P2cr(pub u32);
    impl P2cr {
        #[doc = "CLK/CLK Enable for Port"]
        #[inline(always)]
        pub const fn clken(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CLK/CLK Enable for Port"]
        #[inline(always)]
        pub fn set_clken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CLK/CLK Source for Port"]
        #[inline(always)]
        pub const fn clksrc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CLK/CLK Source for Port"]
        #[inline(always)]
        pub fn set_clksrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DQS Enable for Port"]
        #[inline(always)]
        pub const fn dqsen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DQS Enable for Port"]
        #[inline(always)]
        pub fn set_dqsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "DQS Source for Port"]
        #[inline(always)]
        pub const fn dqssrc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "DQS Source for Port"]
        #[inline(always)]
        pub fn set_dqssrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "CS Enable for Port"]
        #[inline(always)]
        pub const fn ncsen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "CS Enable for Port"]
        #[inline(always)]
        pub fn set_ncsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CS Source for Port"]
        #[inline(always)]
        pub const fn ncssrc(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "CS Source for Port"]
        #[inline(always)]
        pub fn set_ncssrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Enable for Port"]
        #[inline(always)]
        pub const fn iolen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Enable for Port"]
        #[inline(always)]
        pub fn set_iolen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Source for Port"]
        #[inline(always)]
        pub const fn iolsrc(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x03;
            val as u8
        }
        #[doc = "Source for Port"]
        #[inline(always)]
        pub fn set_iolsrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 17usize)) | (((val as u32) & 0x03) << 17usize);
        }
        #[doc = "Enable for Port n"]
        #[inline(always)]
        pub const fn iohen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Enable for Port n"]
        #[inline(always)]
        pub fn set_iohen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Source for Port"]
        #[inline(always)]
        pub const fn iohsrc(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x03;
            val as u8
        }
        #[doc = "Source for Port"]
        #[inline(always)]
        pub fn set_iohsrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
        }
    }
    impl Default for P2cr {
        #[inline(always)]
        fn default() -> P2cr {
            P2cr(0)
        }
    }
}
