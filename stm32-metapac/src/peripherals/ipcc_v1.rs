#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "IPCC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipcc {
    ptr: *mut u8,
}
unsafe impl Send for Ipcc {}
unsafe impl Sync for Ipcc {}
impl Ipcc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "CPU specific registers"]
    #[inline(always)]
    pub const fn cpu(self, n: usize) -> IpccCpu {
        assert!(n < 2usize);
        unsafe { IpccCpu::from_ptr(self.ptr.add(0x0usize + n * 16usize) as _) }
    }
}
#[doc = "IPCC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IpccCpu {
    ptr: *mut u8,
}
unsafe impl Send for IpccCpu {}
unsafe impl Sync for IpccCpu {}
impl IpccCpu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control register CPUx"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::CxCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Mask register CPUx"]
    #[inline(always)]
    pub const fn mr(self) -> crate::common::Reg<regs::CxMr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Status Set or Clear register CPUx"]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::CxScr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "CPUx to CPUy status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::CxToySr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Control register CPUx"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CxCr(pub u32);
    impl CxCr {
        #[doc = "processor x Receive channel occupied interrupt enable"]
        #[inline(always)]
        pub const fn rxoie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "processor x Receive channel occupied interrupt enable"]
        #[inline(always)]
        pub fn set_rxoie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "processor x Transmit channel free interrupt enable"]
        #[inline(always)]
        pub const fn txfie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "processor x Transmit channel free interrupt enable"]
        #[inline(always)]
        pub fn set_txfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for CxCr {
        #[inline(always)]
        fn default() -> CxCr {
            CxCr(0)
        }
    }
    #[doc = "Mask register CPUx"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CxMr(pub u32);
    impl CxMr {
        #[doc = "processor x Receive channel y occupied interrupt enable"]
        #[inline(always)]
        pub const fn chom(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "processor x Receive channel y occupied interrupt enable"]
        #[inline(always)]
        pub fn set_chom(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "processor x Transmit channel y free interrupt mask"]
        #[inline(always)]
        pub const fn chfm(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "processor x Transmit channel y free interrupt mask"]
        #[inline(always)]
        pub fn set_chfm(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for CxMr {
        #[inline(always)]
        fn default() -> CxMr {
            CxMr(0)
        }
    }
    #[doc = "Status Set or Clear register CPUx"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CxScr(pub u32);
    impl CxScr {
        #[doc = "processor x Receive channel y status clear"]
        #[inline(always)]
        pub const fn chc(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "processor x Receive channel y status clear"]
        #[inline(always)]
        pub fn set_chc(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "processor x Transmit channel y status set"]
        #[inline(always)]
        pub const fn chs(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "processor x Transmit channel y status set"]
        #[inline(always)]
        pub fn set_chs(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for CxScr {
        #[inline(always)]
        fn default() -> CxScr {
            CxScr(0)
        }
    }
    #[doc = "CPUx to CPUy status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CxToySr(pub u32);
    impl CxToySr {
        #[doc = "processor x transmit to process y Receive channel z status flag"]
        #[inline(always)]
        pub const fn chf(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "processor x transmit to process y Receive channel z status flag"]
        #[inline(always)]
        pub fn set_chf(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for CxToySr {
        #[inline(always)]
        fn default() -> CxToySr {
            CxToySr(0)
        }
    }
}
