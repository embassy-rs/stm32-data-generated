#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "System configuration controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syscfg {
    ptr: *mut u8,
}
unsafe impl Send for Syscfg {}
unsafe impl Sync for Syscfg {}
impl Syscfg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "memory remap register"]
    #[inline(always)]
    pub const fn memrmp(self) -> crate::common::Reg<regs::Memrmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "configuration register 1"]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::Cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "external interrupt configuration register 1"]
    #[inline(always)]
    pub const fn exticr(self, n: usize) -> crate::common::Reg<regs::Exticr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 4usize) as _) }
    }
    #[doc = "SCSR"]
    #[inline(always)]
    pub const fn scsr(self) -> crate::common::Reg<regs::Scsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "CFGR2"]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "SWPR"]
    #[inline(always)]
    pub const fn swpr(self) -> crate::common::Reg<regs::Swpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "SKR"]
    #[inline(always)]
    pub const fn skr(self) -> crate::common::Reg<regs::Skr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "SYSCFG CPU1 interrupt mask register 1"]
    #[inline(always)]
    pub const fn imr1(self) -> crate::common::Reg<regs::Imr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "SYSCFG CPU1 interrupt mask register 2"]
    #[inline(always)]
    pub const fn imr2(self) -> crate::common::Reg<regs::Imr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "SYSCFG CPU2 interrupt mask register 1"]
    #[inline(always)]
    pub const fn c2imr1(self) -> crate::common::Reg<regs::C2imr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "SYSCFG CPU2 interrupt mask register 2"]
    #[inline(always)]
    pub const fn c2imr2(self) -> crate::common::Reg<regs::C2imr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "radio debug control register"]
    #[inline(always)]
    pub const fn rfdcr(self) -> crate::common::Reg<regs::Rfdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0208usize) as _) }
    }
}
pub mod regs {
    #[doc = "SYSCFG CPU2 interrupt mask register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C2imr1(pub u32);
    impl C2imr1 {
        #[doc = "RTCSTAMPTAMPLSECSSIM"]
        #[inline(always)]
        pub const fn rtcstamptamplsecssim(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RTCSTAMPTAMPLSECSSIM"]
        #[inline(always)]
        pub fn set_rtcstamptamplsecssim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RTCALARMIM"]
        #[inline(always)]
        pub const fn rtcalarmim(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RTCALARMIM"]
        #[inline(always)]
        pub fn set_rtcalarmim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "RTCSSRUIM"]
        #[inline(always)]
        pub const fn rtcssruim(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RTCSSRUIM"]
        #[inline(always)]
        pub fn set_rtcssruim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "RTCWKUPIM"]
        #[inline(always)]
        pub const fn rtcwkupim(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RTCWKUPIM"]
        #[inline(always)]
        pub fn set_rtcwkupim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "RCCIM"]
        #[inline(always)]
        pub const fn rccim(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RCCIM"]
        #[inline(always)]
        pub fn set_rccim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FLASHIM"]
        #[inline(always)]
        pub const fn flashim(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "FLASHIM"]
        #[inline(always)]
        pub fn set_flashim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "PKAIM"]
        #[inline(always)]
        pub const fn pkaim(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "PKAIM"]
        #[inline(always)]
        pub fn set_pkaim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "AESIM"]
        #[inline(always)]
        pub const fn aesim(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "AESIM"]
        #[inline(always)]
        pub fn set_aesim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "COMPIM"]
        #[inline(always)]
        pub const fn compim(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "COMPIM"]
        #[inline(always)]
        pub fn set_compim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "ADCIM"]
        #[inline(always)]
        pub const fn adcim(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "ADCIM"]
        #[inline(always)]
        pub fn set_adcim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "DACIM"]
        #[inline(always)]
        pub const fn dacim(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "DACIM"]
        #[inline(always)]
        pub fn set_dacim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "EXTI0IM"]
        #[inline(always)]
        pub const fn exti0im(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI0IM"]
        #[inline(always)]
        pub fn set_exti0im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "EXTI1IM"]
        #[inline(always)]
        pub const fn exti1im(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI1IM"]
        #[inline(always)]
        pub fn set_exti1im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "EXTI2IM"]
        #[inline(always)]
        pub const fn exti2im(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI2IM"]
        #[inline(always)]
        pub fn set_exti2im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "EXTI3IM"]
        #[inline(always)]
        pub const fn exti3im(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI3IM"]
        #[inline(always)]
        pub fn set_exti3im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "EXTI4IM"]
        #[inline(always)]
        pub const fn exti4im(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI4IM"]
        #[inline(always)]
        pub fn set_exti4im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "EXTI5IM"]
        #[inline(always)]
        pub const fn exti5im(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI5IM"]
        #[inline(always)]
        pub fn set_exti5im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "EXTI6IM"]
        #[inline(always)]
        pub const fn exti6im(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI6IM"]
        #[inline(always)]
        pub fn set_exti6im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "EXTI7IM"]
        #[inline(always)]
        pub const fn exti7im(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI7IM"]
        #[inline(always)]
        pub fn set_exti7im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "EXTI8IM"]
        #[inline(always)]
        pub const fn exti8im(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI8IM"]
        #[inline(always)]
        pub fn set_exti8im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "EXTI9IM"]
        #[inline(always)]
        pub const fn exti9im(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI9IM"]
        #[inline(always)]
        pub fn set_exti9im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "EXTI10IM"]
        #[inline(always)]
        pub const fn exti10im(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI10IM"]
        #[inline(always)]
        pub fn set_exti10im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "EXTI11IM"]
        #[inline(always)]
        pub const fn exti11im(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI11IM"]
        #[inline(always)]
        pub fn set_exti11im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "EXTI12IM"]
        #[inline(always)]
        pub const fn exti12im(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI12IM"]
        #[inline(always)]
        pub fn set_exti12im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "EXTI13IM"]
        #[inline(always)]
        pub const fn exti13im(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI13IM"]
        #[inline(always)]
        pub fn set_exti13im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "EXTI14IM"]
        #[inline(always)]
        pub const fn exti14im(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI14IM"]
        #[inline(always)]
        pub fn set_exti14im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "EXTI15IM"]
        #[inline(always)]
        pub const fn exti15im(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI15IM"]
        #[inline(always)]
        pub fn set_exti15im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for C2imr1 {
        #[inline(always)]
        fn default() -> C2imr1 {
            C2imr1(0)
        }
    }
    #[doc = "SYSCFG CPU2 interrupt mask register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C2imr2(pub u32);
    impl C2imr2 {
        #[doc = "DMA1CH1IM"]
        #[inline(always)]
        pub const fn dma1ch1im(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1CH1IM"]
        #[inline(always)]
        pub fn set_dma1ch1im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA1CH2IM"]
        #[inline(always)]
        pub const fn dma1ch2im(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1CH2IM"]
        #[inline(always)]
        pub fn set_dma1ch2im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DMA1CH3IM"]
        #[inline(always)]
        pub const fn dma1ch3im(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1CH3IM"]
        #[inline(always)]
        pub fn set_dma1ch3im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "DMA1CH4IM"]
        #[inline(always)]
        pub const fn dma1ch4im(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1CH4IM"]
        #[inline(always)]
        pub fn set_dma1ch4im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DMA1CH5IM"]
        #[inline(always)]
        pub const fn dma1ch5im(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1CH5IM"]
        #[inline(always)]
        pub fn set_dma1ch5im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "DMA1CH6IM"]
        #[inline(always)]
        pub const fn dma1ch6im(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1CH6IM"]
        #[inline(always)]
        pub fn set_dma1ch6im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "DMA1CH7IM"]
        #[inline(always)]
        pub const fn dma1ch7im(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1CH7IM"]
        #[inline(always)]
        pub fn set_dma1ch7im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "DMA2CH1IM"]
        #[inline(always)]
        pub const fn dma2ch1im(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2CH1IM"]
        #[inline(always)]
        pub fn set_dma2ch1im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "DMA2CH2IM"]
        #[inline(always)]
        pub const fn dma2ch2im(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2CH2IM"]
        #[inline(always)]
        pub fn set_dma2ch2im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "DMA2CH3IM"]
        #[inline(always)]
        pub const fn dma2ch3im(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2CH3IM"]
        #[inline(always)]
        pub fn set_dma2ch3im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "DMA2CH4IM"]
        #[inline(always)]
        pub const fn dma2ch4im(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2CH4IM"]
        #[inline(always)]
        pub fn set_dma2ch4im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "DMA2CH5IM"]
        #[inline(always)]
        pub const fn dma2ch5im(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2CH5IM"]
        #[inline(always)]
        pub fn set_dma2ch5im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "DMA2CH6IM"]
        #[inline(always)]
        pub const fn dma2ch6im(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2CH6IM"]
        #[inline(always)]
        pub fn set_dma2ch6im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "DMA2CH7IM"]
        #[inline(always)]
        pub const fn dma2ch7im(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2CH7IM"]
        #[inline(always)]
        pub fn set_dma2ch7im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "DMAMUX1IM"]
        #[inline(always)]
        pub const fn dmamux1im(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "DMAMUX1IM"]
        #[inline(always)]
        pub fn set_dmamux1im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "PVM3IM"]
        #[inline(always)]
        pub const fn pvm3im(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PVM3IM"]
        #[inline(always)]
        pub fn set_pvm3im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PVDIM"]
        #[inline(always)]
        pub const fn pvdim(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "PVDIM"]
        #[inline(always)]
        pub fn set_pvdim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for C2imr2 {
        #[inline(always)]
        fn default() -> C2imr2 {
            C2imr2(0)
        }
    }
    #[doc = "configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr1(pub u32);
    impl Cfgr1 {
        #[doc = "I/O analog switch voltage booster enable"]
        #[inline(always)]
        pub const fn boosten(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "I/O analog switch voltage booster enable"]
        #[inline(always)]
        pub fn set_boosten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Fast-mode Plus (Fm+) driving capability activation on PB6"]
        #[inline(always)]
        pub const fn i2c_pb6_fmp(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus (Fm+) driving capability activation on PB6"]
        #[inline(always)]
        pub fn set_i2c_pb6_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Fast-mode Plus (Fm+) driving capability activation on PB7"]
        #[inline(always)]
        pub const fn i2c_pb7_fmp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus (Fm+) driving capability activation on PB7"]
        #[inline(always)]
        pub fn set_i2c_pb7_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Fast-mode Plus (Fm+) driving capability activation on PB8"]
        #[inline(always)]
        pub const fn i2c_pb8_fmp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus (Fm+) driving capability activation on PB8"]
        #[inline(always)]
        pub fn set_i2c_pb8_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Fast-mode Plus (Fm+) driving capability activation on PB9"]
        #[inline(always)]
        pub const fn i2c_pb9_fmp(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus (Fm+) driving capability activation on PB9"]
        #[inline(always)]
        pub fn set_i2c_pb9_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "I2C1 Fast-mode Plus driving capability activation"]
        #[inline(always)]
        pub const fn i2c1_fmp(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 Fast-mode Plus driving capability activation"]
        #[inline(always)]
        pub fn set_i2c1_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C2 Fast-mode Plus driving capability activation"]
        #[inline(always)]
        pub const fn i2c2_fmp(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 Fast-mode Plus driving capability activation"]
        #[inline(always)]
        pub fn set_i2c2_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C3 Fast-mode Plus driving capability activation"]
        #[inline(always)]
        pub const fn i2c3_fmp(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 Fast-mode Plus driving capability activation"]
        #[inline(always)]
        pub fn set_i2c3_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Cfgr1 {
        #[inline(always)]
        fn default() -> Cfgr1 {
            Cfgr1(0)
        }
    }
    #[doc = "CFGR2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "CPU1 LOCKUP (Hardfault) output enable bit"]
        #[inline(always)]
        pub const fn cll(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CPU1 LOCKUP (Hardfault) output enable bit"]
        #[inline(always)]
        pub fn set_cll(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SRAM2 parity lock bit"]
        #[inline(always)]
        pub const fn spl(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 parity lock bit"]
        #[inline(always)]
        pub fn set_spl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PVD lock enable bit"]
        #[inline(always)]
        pub const fn pvdl(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PVD lock enable bit"]
        #[inline(always)]
        pub fn set_pvdl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ECC Lock"]
        #[inline(always)]
        pub const fn eccl(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ECC Lock"]
        #[inline(always)]
        pub fn set_eccl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SRAM2 parity error flag"]
        #[inline(always)]
        pub const fn spf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 parity error flag"]
        #[inline(always)]
        pub fn set_spf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Cfgr2 {
        #[inline(always)]
        fn default() -> Cfgr2 {
            Cfgr2(0)
        }
    }
    #[doc = "external interrupt configuration register 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exticr(pub u32);
    impl Exticr {
        #[doc = "EXTI12 configuration bits"]
        #[inline(always)]
        pub const fn exti(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x07;
            val as u8
        }
        #[doc = "EXTI12 configuration bits"]
        #[inline(always)]
        pub fn set_exti(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val as u32) & 0x07) << offs);
        }
    }
    impl Default for Exticr {
        #[inline(always)]
        fn default() -> Exticr {
            Exticr(0)
        }
    }
    #[doc = "SYSCFG CPU1 interrupt mask register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Imr1(pub u32);
    impl Imr1 {
        #[doc = "RTCSTAMPTAMPLSECSSIM"]
        #[inline(always)]
        pub const fn rtcstamptamplsecssim(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RTCSTAMPTAMPLSECSSIM"]
        #[inline(always)]
        pub fn set_rtcstamptamplsecssim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RTCSSRUIM"]
        #[inline(always)]
        pub const fn rtcssruim(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RTCSSRUIM"]
        #[inline(always)]
        pub fn set_rtcssruim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "EXTI5IM"]
        #[inline(always)]
        pub const fn exti5im(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI5IM"]
        #[inline(always)]
        pub fn set_exti5im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "EXTI6IM"]
        #[inline(always)]
        pub const fn exti6im(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI6IM"]
        #[inline(always)]
        pub fn set_exti6im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "EXTI7IM"]
        #[inline(always)]
        pub const fn exti7im(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI7IM"]
        #[inline(always)]
        pub fn set_exti7im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "EXTI8IM"]
        #[inline(always)]
        pub const fn exti8im(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI8IM"]
        #[inline(always)]
        pub fn set_exti8im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "EXTI9IM"]
        #[inline(always)]
        pub const fn exti9im(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI9IM"]
        #[inline(always)]
        pub fn set_exti9im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "EXTI10IM"]
        #[inline(always)]
        pub const fn exti10im(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI10IM"]
        #[inline(always)]
        pub fn set_exti10im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "EXTI11IM"]
        #[inline(always)]
        pub const fn exti11im(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI11IM"]
        #[inline(always)]
        pub fn set_exti11im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "EXTI12IM"]
        #[inline(always)]
        pub const fn exti12im(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI12IM"]
        #[inline(always)]
        pub fn set_exti12im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "EXTI13IM"]
        #[inline(always)]
        pub const fn exti13im(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI13IM"]
        #[inline(always)]
        pub fn set_exti13im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "EXTI14IM"]
        #[inline(always)]
        pub const fn exti14im(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI14IM"]
        #[inline(always)]
        pub fn set_exti14im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "EXTI15IM"]
        #[inline(always)]
        pub const fn exti15im(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI15IM"]
        #[inline(always)]
        pub fn set_exti15im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Imr1 {
        #[inline(always)]
        fn default() -> Imr1 {
            Imr1(0)
        }
    }
    #[doc = "SYSCFG CPU1 interrupt mask register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Imr2(pub u32);
    impl Imr2 {
        #[doc = "PVM3IM"]
        #[inline(always)]
        pub const fn pvm3im(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PVM3IM"]
        #[inline(always)]
        pub fn set_pvm3im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PVDIM"]
        #[inline(always)]
        pub const fn pvdim(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "PVDIM"]
        #[inline(always)]
        pub fn set_pvdim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Imr2 {
        #[inline(always)]
        fn default() -> Imr2 {
            Imr2(0)
        }
    }
    #[doc = "memory remap register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Memrmp(pub u32);
    impl Memrmp {
        #[doc = "Memory mapping selection"]
        #[inline(always)]
        pub const fn mem_mode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Memory mapping selection"]
        #[inline(always)]
        pub fn set_mem_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Memrmp {
        #[inline(always)]
        fn default() -> Memrmp {
            Memrmp(0)
        }
    }
    #[doc = "radio debug control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rfdcr(pub u32);
    impl Rfdcr {
        #[doc = "radio debug test bus selection"]
        #[inline(always)]
        pub const fn rftbsel(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "radio debug test bus selection"]
        #[inline(always)]
        pub fn set_rftbsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Rfdcr {
        #[inline(always)]
        fn default() -> Rfdcr {
            Rfdcr(0)
        }
    }
    #[doc = "SCSR"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scsr(pub u32);
    impl Scsr {
        #[doc = "SRAM2 erase"]
        #[inline(always)]
        pub const fn sram2er(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 erase"]
        #[inline(always)]
        pub fn set_sram2er(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SRAM1, SRAM2 and PKA SRAM busy by erase operation"]
        #[inline(always)]
        pub const fn srambsy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1, SRAM2 and PKA SRAM busy by erase operation"]
        #[inline(always)]
        pub fn set_srambsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PKA SRAM busy by erase operation"]
        #[inline(always)]
        pub const fn pkasrambsy(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "PKA SRAM busy by erase operation"]
        #[inline(always)]
        pub fn set_pkasrambsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Scsr {
        #[inline(always)]
        fn default() -> Scsr {
            Scsr(0)
        }
    }
    #[doc = "SKR"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Skr(pub u32);
    impl Skr {
        #[doc = "SRAM2 write protection key for software erase"]
        #[inline(always)]
        pub const fn key(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "SRAM2 write protection key for software erase"]
        #[inline(always)]
        pub fn set_key(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Skr {
        #[inline(always)]
        fn default() -> Skr {
            Skr(0)
        }
    }
    #[doc = "SWPR"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Swpr(pub u32);
    impl Swpr {
        #[doc = "SRAM2 1Kbyte page 0 write protection"]
        #[inline(always)]
        pub const fn pwp(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1Kbyte page 0 write protection"]
        #[inline(always)]
        pub fn set_pwp(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Swpr {
        #[inline(always)]
        fn default() -> Swpr {
            Swpr(0)
        }
    }
}
