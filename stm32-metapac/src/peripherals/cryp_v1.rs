#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Cryptographic processor."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cryp {
    ptr: *mut u8,
}
unsafe impl Send for Cryp {}
unsafe impl Sync for Cryp {}
impl Cryp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "data input register."]
    #[inline(always)]
    pub const fn din(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "data output register."]
    #[inline(always)]
    pub const fn dout(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "DMA control register."]
    #[inline(always)]
    pub const fn dmacr(self) -> crate::common::Reg<regs::Dmacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "interrupt mask set/clear register."]
    #[inline(always)]
    pub const fn imscr(self) -> crate::common::Reg<regs::Imscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "raw interrupt status register."]
    #[inline(always)]
    pub const fn risr(self) -> crate::common::Reg<regs::Risr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "masked interrupt status register."]
    #[inline(always)]
    pub const fn misr(self) -> crate::common::Reg<regs::Misr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Cluster KEY%s, containing K?LR, K?RR."]
    #[inline(always)]
    pub const fn key(self, n: usize) -> Key {
        assert!(n < 4usize);
        unsafe { Key::from_ptr(self.ptr.add(0x20usize + n * 8usize) as _) }
    }
    #[doc = "Cluster INIT%s, containing IV?LR, IV?RR."]
    #[inline(always)]
    pub const fn init(self, n: usize) -> Init {
        assert!(n < 2usize);
        unsafe { Init::from_ptr(self.ptr.add(0x40usize + n * 8usize) as _) }
    }
}
#[doc = "Cluster INIT%s, containing IV?LR, IV?RR."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Init {
    ptr: *mut u8,
}
unsafe impl Send for Init {}
unsafe impl Sync for Init {}
impl Init {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "initialization vector registers."]
    #[inline(always)]
    pub const fn ivlr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "initialization vector registers."]
    #[inline(always)]
    pub const fn ivrr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
#[doc = "Cluster KEY%s, containing K?LR, K?RR."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key {
    ptr: *mut u8,
}
unsafe impl Send for Key {}
unsafe impl Sync for Key {}
impl Key {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "key registers."]
    #[inline(always)]
    pub const fn klr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "key registers."]
    #[inline(always)]
    pub const fn krr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
pub mod regs {
    #[doc = "control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Algorithm direction."]
        #[inline(always)]
        pub const fn algodir(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Algorithm direction."]
        #[inline(always)]
        pub fn set_algodir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Algorithm mode."]
        #[inline(always)]
        pub const fn algomode(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x07;
            val as u8
        }
        #[doc = "Algorithm mode."]
        #[inline(always)]
        pub fn set_algomode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
        }
        #[doc = "Data type selection."]
        #[inline(always)]
        pub const fn datatype(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Data type selection."]
        #[inline(always)]
        pub fn set_datatype(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "Key size selection (AES mode only)."]
        #[inline(always)]
        pub const fn keysize(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Key size selection (AES mode only)."]
        #[inline(always)]
        pub fn set_keysize(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "FIFO flush."]
        #[inline(always)]
        pub const fn fflush(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO flush."]
        #[inline(always)]
        pub fn set_fflush(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Cryptographic processor enable."]
        #[inline(always)]
        pub const fn crypen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Cryptographic processor enable."]
        #[inline(always)]
        pub fn set_crypen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    #[doc = "DMA control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmacr(pub u32);
    impl Dmacr {
        #[doc = "DMA input enable."]
        #[inline(always)]
        pub const fn dien(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA input enable."]
        #[inline(always)]
        pub fn set_dien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA output enable."]
        #[inline(always)]
        pub const fn doen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA output enable."]
        #[inline(always)]
        pub fn set_doen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Dmacr {
        #[inline(always)]
        fn default() -> Dmacr {
            Dmacr(0)
        }
    }
    #[doc = "interrupt mask set/clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Imscr(pub u32);
    impl Imscr {
        #[doc = "Input FIFO service interrupt mask."]
        #[inline(always)]
        pub const fn inim(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Input FIFO service interrupt mask."]
        #[inline(always)]
        pub fn set_inim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Output FIFO service interrupt mask."]
        #[inline(always)]
        pub const fn outim(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Output FIFO service interrupt mask."]
        #[inline(always)]
        pub fn set_outim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Imscr {
        #[inline(always)]
        fn default() -> Imscr {
            Imscr(0)
        }
    }
    #[doc = "masked interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Misr(pub u32);
    impl Misr {
        #[doc = "Input FIFO service masked interrupt status."]
        #[inline(always)]
        pub const fn inmis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Input FIFO service masked interrupt status."]
        #[inline(always)]
        pub fn set_inmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Output FIFO service masked interrupt status."]
        #[inline(always)]
        pub const fn outmis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Output FIFO service masked interrupt status."]
        #[inline(always)]
        pub fn set_outmis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Misr {
        #[inline(always)]
        fn default() -> Misr {
            Misr(0)
        }
    }
    #[doc = "raw interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Risr(pub u32);
    impl Risr {
        #[doc = "Input FIFO service raw interrupt status."]
        #[inline(always)]
        pub const fn inris(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Input FIFO service raw interrupt status."]
        #[inline(always)]
        pub fn set_inris(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Output FIFO service raw interrupt status."]
        #[inline(always)]
        pub const fn outris(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Output FIFO service raw interrupt status."]
        #[inline(always)]
        pub fn set_outris(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Risr {
        #[inline(always)]
        fn default() -> Risr {
            Risr(0)
        }
    }
    #[doc = "status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Input FIFO empty."]
        #[inline(always)]
        pub const fn ifem(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Input FIFO empty."]
        #[inline(always)]
        pub fn set_ifem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Input FIFO not full."]
        #[inline(always)]
        pub const fn ifnf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Input FIFO not full."]
        #[inline(always)]
        pub fn set_ifnf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Output FIFO not empty."]
        #[inline(always)]
        pub const fn ofne(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Output FIFO not empty."]
        #[inline(always)]
        pub fn set_ofne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Output FIFO full."]
        #[inline(always)]
        pub const fn offu(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Output FIFO full."]
        #[inline(always)]
        pub fn set_offu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Busy bit."]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Busy bit."]
        #[inline(always)]
        pub fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
}
