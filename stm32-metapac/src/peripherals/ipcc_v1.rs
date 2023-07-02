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
        unsafe { IpccCpu::from_ptr(self.ptr.add(0usize + n * 16usize) as _) }
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
    pub const fn cr(self) -> crate::common::Reg<regs::C1cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Mask register CPUx"]
    #[inline(always)]
    pub const fn mr(self) -> crate::common::Reg<regs::C1mr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Status Set or Clear register CPU1"]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::C1scr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "CPU1 to CPU2 status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::C1to2sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
}
pub mod regs {
    #[doc = "Control register CPU1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1cr(pub u32);
    impl C1cr {
        #[doc = "processor 1 Receive channel occupied interrupt enable"]
        #[inline(always)]
        pub const fn rxoie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "processor 1 Receive channel occupied interrupt enable"]
        #[inline(always)]
        pub fn set_rxoie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "processor 1 Transmit channel free interrupt enable"]
        #[inline(always)]
        pub const fn txfie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "processor 1 Transmit channel free interrupt enable"]
        #[inline(always)]
        pub fn set_txfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for C1cr {
        #[inline(always)]
        fn default() -> C1cr {
            C1cr(0)
        }
    }
    #[doc = "Mask register CPU1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1mr(pub u32);
    impl C1mr {
        #[doc = "processor 1 Receive channel x occupied interrupt enable"]
        #[inline(always)]
        pub const fn chom(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "processor 1 Receive channel x occupied interrupt enable"]
        #[inline(always)]
        pub fn set_chom(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "processor 1 Transmit channel x free interrupt mask"]
        #[inline(always)]
        pub const fn chfm(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "processor 1 Transmit channel x free interrupt mask"]
        #[inline(always)]
        pub fn set_chfm(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for C1mr {
        #[inline(always)]
        fn default() -> C1mr {
            C1mr(0)
        }
    }
    #[doc = "Status Set or Clear register CPU1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1scr(pub u32);
    impl C1scr {
        #[doc = "processor 1 Receive channel x status clear"]
        #[inline(always)]
        pub const fn chc(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "processor 1 Receive channel x status clear"]
        #[inline(always)]
        pub fn set_chc(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "processor 1 Transmit channel x status set"]
        #[inline(always)]
        pub const fn chs(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "processor 1 Transmit channel x status set"]
        #[inline(always)]
        pub fn set_chs(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for C1scr {
        #[inline(always)]
        fn default() -> C1scr {
            C1scr(0)
        }
    }
    #[doc = "CPU1 to CPU2 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1to2sr(pub u32);
    impl C1to2sr {
        #[doc = "processor 1 transmit to process 2 Receive channel x status flag"]
        #[inline(always)]
        pub const fn chf(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "processor 1 transmit to process 2 Receive channel x status flag"]
        #[inline(always)]
        pub fn set_chf(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for C1to2sr {
        #[inline(always)]
        fn default() -> C1to2sr {
            C1to2sr(0)
        }
    }
    #[doc = "Control register CPU2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C2cr(pub u32);
    impl C2cr {
        #[doc = "processor 2 Receive channel occupied interrupt enable"]
        #[inline(always)]
        pub const fn rxoie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "processor 2 Receive channel occupied interrupt enable"]
        #[inline(always)]
        pub fn set_rxoie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "processor 2 Transmit channel free interrupt enable"]
        #[inline(always)]
        pub const fn txfie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "processor 2 Transmit channel free interrupt enable"]
        #[inline(always)]
        pub fn set_txfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for C2cr {
        #[inline(always)]
        fn default() -> C2cr {
            C2cr(0)
        }
    }
    #[doc = "Mask register CPU2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C2mr(pub u32);
    impl C2mr {
        #[doc = "processor 2 Receive channel x occupied interrupt enable"]
        #[inline(always)]
        pub const fn chom(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "processor 2 Receive channel x occupied interrupt enable"]
        #[inline(always)]
        pub fn set_chom(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "processor 2 Transmit channel 1 free interrupt mask"]
        #[inline(always)]
        pub const fn chfm(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "processor 2 Transmit channel 1 free interrupt mask"]
        #[inline(always)]
        pub fn set_chfm(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for C2mr {
        #[inline(always)]
        fn default() -> C2mr {
            C2mr(0)
        }
    }
    #[doc = "Status Set or Clear register CPU2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C2scr(pub u32);
    impl C2scr {
        #[doc = "processor 2 Receive channel x status clear"]
        #[inline(always)]
        pub const fn chc(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "processor 2 Receive channel x status clear"]
        #[inline(always)]
        pub fn set_chc(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "processor 2 Transmit channel 1 status set"]
        #[inline(always)]
        pub const fn chs(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "processor 2 Transmit channel 1 status set"]
        #[inline(always)]
        pub fn set_chs(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for C2scr {
        #[inline(always)]
        fn default() -> C2scr {
            C2scr(0)
        }
    }
    #[doc = "CPU2 to CPU1 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C2toc1sr(pub u32);
    impl C2toc1sr {
        #[doc = "processor 2 transmit to process 1 Receive channel x status flag"]
        #[inline(always)]
        pub const fn chf(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "processor 2 transmit to process 1 Receive channel x status flag"]
        #[inline(always)]
        pub fn set_chf(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for C2toc1sr {
        #[inline(always)]
        fn default() -> C2toc1sr {
            C2toc1sr(0)
        }
    }
}
