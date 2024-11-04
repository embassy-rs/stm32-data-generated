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
    #[doc = "SRAM2 write protection register"]
    #[inline(always)]
    pub const fn swpr(self) -> crate::common::Reg<regs::Swpr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "SKR"]
    #[inline(always)]
    pub const fn skr(self) -> crate::common::Reg<regs::Skr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "SRAM2 write protection register 2"]
    #[inline(always)]
    pub const fn swpr2(self) -> crate::common::Reg<regs::Swpr2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "CPU1 interrupt mask register 1"]
    #[inline(always)]
    pub const fn imr1(self) -> crate::common::Reg<regs::Imr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "CPU1 interrupt mask register 2"]
    #[inline(always)]
    pub const fn imr2(self) -> crate::common::Reg<regs::Imr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "CPU2 interrupt mask register 1"]
    #[inline(always)]
    pub const fn c2imr1(self) -> crate::common::Reg<regs::C2imr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "CPU2 interrupt mask register 1"]
    #[inline(always)]
    pub const fn c2imr2(self) -> crate::common::Reg<regs::C2imr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "secure IP control register"]
    #[inline(always)]
    pub const fn sipcr(self) -> crate::common::Reg<regs::Sipcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
}
pub mod regs {
    #[doc = "CPU2 interrupt mask register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C2imr1(pub u32);
    impl C2imr1 {
        #[doc = "Peripheral RTCSTAMP interrupt mask to CPU2"]
        #[inline(always)]
        pub const fn rtcstamp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral RTCSTAMP interrupt mask to CPU2"]
        #[inline(always)]
        pub fn set_rtcstamp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Peripheral RTCWKUP interrupt mask to CPU2"]
        #[inline(always)]
        pub const fn rtcwkup(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral RTCWKUP interrupt mask to CPU2"]
        #[inline(always)]
        pub fn set_rtcwkup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Peripheral RTCALARM interrupt mask to CPU2"]
        #[inline(always)]
        pub const fn rtcalarm(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral RTCALARM interrupt mask to CPU2"]
        #[inline(always)]
        pub fn set_rtcalarm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Peripheral RCC interrupt mask to CPU2"]
        #[inline(always)]
        pub const fn rcc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral RCC interrupt mask to CPU2"]
        #[inline(always)]
        pub fn set_rcc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Peripheral FLASH interrupt mask to CPU2"]
        #[inline(always)]
        pub const fn flash(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral FLASH interrupt mask to CPU2"]
        #[inline(always)]
        pub fn set_flash(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Peripheral PKA interrupt mask to CPU2"]
        #[inline(always)]
        pub const fn pka(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral PKA interrupt mask to CPU2"]
        #[inline(always)]
        pub fn set_pka(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Peripheral RNG interrupt mask to CPU2"]
        #[inline(always)]
        pub const fn rng(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral RNG interrupt mask to CPU2"]
        #[inline(always)]
        pub fn set_rng(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Peripheral AES1 interrupt mask to CPU2"]
        #[inline(always)]
        pub const fn aes1(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral AES1 interrupt mask to CPU2"]
        #[inline(always)]
        pub fn set_aes1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Peripheral COMP interrupt mask to CPU2"]
        #[inline(always)]
        pub const fn comp(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral COMP interrupt mask to CPU2"]
        #[inline(always)]
        pub fn set_comp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Peripheral ADC interrupt mask to CPU2"]
        #[inline(always)]
        pub const fn adc(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral ADC interrupt mask to CPU2"]
        #[inline(always)]
        pub fn set_adc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for C2imr1 {
        #[inline(always)]
        fn default() -> C2imr1 {
            C2imr1(0)
        }
    }
    #[doc = "CPU2 interrupt mask register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C2imr2(pub u32);
    impl C2imr2 {
        #[doc = "Peripheral DMA1 CH1 interrupt mask to CPU2"]
        #[inline(always)]
        pub const fn dma1_ch1_im(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral DMA1 CH1 interrupt mask to CPU2"]
        #[inline(always)]
        pub fn set_dma1_ch1_im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Peripheral DMA1 CH2 interrupt mask to CPU2"]
        #[inline(always)]
        pub const fn dma1_ch2_im(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral DMA1 CH2 interrupt mask to CPU2"]
        #[inline(always)]
        pub fn set_dma1_ch2_im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Peripheral DMA1 CH3 interrupt mask to CPU2"]
        #[inline(always)]
        pub const fn dma1_ch3_im(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral DMA1 CH3 interrupt mask to CPU2"]
        #[inline(always)]
        pub fn set_dma1_ch3_im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Peripheral DMA1 CH4 interrupt mask to CPU2"]
        #[inline(always)]
        pub const fn dma1_ch4_im(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral DMA1 CH4 interrupt mask to CPU2"]
        #[inline(always)]
        pub fn set_dma1_ch4_im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Peripheral DMA1 CH5 interrupt mask to CPU2"]
        #[inline(always)]
        pub const fn dma1_ch5_im(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral DMA1 CH5 interrupt mask to CPU2"]
        #[inline(always)]
        pub fn set_dma1_ch5_im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Peripheral DMA1 CH6 interrupt mask to CPU2"]
        #[inline(always)]
        pub const fn dma1_ch6_im(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral DMA1 CH6 interrupt mask to CPU2"]
        #[inline(always)]
        pub fn set_dma1_ch6_im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Peripheral DMA1 CH7 interrupt mask to CPU2"]
        #[inline(always)]
        pub const fn dma1_ch7_im(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral DMA1 CH7 interrupt mask to CPU2"]
        #[inline(always)]
        pub fn set_dma1_ch7_im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Peripheral DMA2 CH1 interrupt mask to CPU1"]
        #[inline(always)]
        pub const fn dma2_ch1_im(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral DMA2 CH1 interrupt mask to CPU1"]
        #[inline(always)]
        pub fn set_dma2_ch1_im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Peripheral DMA2 CH2 interrupt mask to CPU1"]
        #[inline(always)]
        pub const fn dma2_ch2_im(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral DMA2 CH2 interrupt mask to CPU1"]
        #[inline(always)]
        pub fn set_dma2_ch2_im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Peripheral DMA2 CH3 interrupt mask to CPU1"]
        #[inline(always)]
        pub const fn dma2_ch3_im(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral DMA2 CH3 interrupt mask to CPU1"]
        #[inline(always)]
        pub fn set_dma2_ch3_im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Peripheral DMA2 CH4 interrupt mask to CPU1"]
        #[inline(always)]
        pub const fn dma2_ch4_im(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral DMA2 CH4 interrupt mask to CPU1"]
        #[inline(always)]
        pub fn set_dma2_ch4_im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Peripheral DMA2 CH5 interrupt mask to CPU1"]
        #[inline(always)]
        pub const fn dma2_ch5_im(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral DMA2 CH5 interrupt mask to CPU1"]
        #[inline(always)]
        pub fn set_dma2_ch5_im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Peripheral DMA2 CH6 interrupt mask to CPU1"]
        #[inline(always)]
        pub const fn dma2_ch6_im(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral DMA2 CH6 interrupt mask to CPU1"]
        #[inline(always)]
        pub fn set_dma2_ch6_im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Peripheral DMA2 CH7 interrupt mask to CPU1"]
        #[inline(always)]
        pub const fn dma2_ch7_im(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral DMA2 CH7 interrupt mask to CPU1"]
        #[inline(always)]
        pub fn set_dma2_ch7_im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Peripheral DMAM UX1 interrupt mask to CPU1"]
        #[inline(always)]
        pub const fn dmam_ux1_im(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral DMAM UX1 interrupt mask to CPU1"]
        #[inline(always)]
        pub fn set_dmam_ux1_im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Peripheral PVM1IM interrupt mask to CPU1"]
        #[inline(always)]
        pub const fn pvm1im(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral PVM1IM interrupt mask to CPU1"]
        #[inline(always)]
        pub fn set_pvm1im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Peripheral PVM3IM interrupt mask to CPU1"]
        #[inline(always)]
        pub const fn pvm3im(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral PVM3IM interrupt mask to CPU1"]
        #[inline(always)]
        pub fn set_pvm3im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Peripheral PVDIM interrupt mask to CPU1"]
        #[inline(always)]
        pub const fn pvdim(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral PVDIM interrupt mask to CPU1"]
        #[inline(always)]
        pub fn set_pvdim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Peripheral TSCIM interrupt mask to CPU1"]
        #[inline(always)]
        pub const fn tscim(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral TSCIM interrupt mask to CPU1"]
        #[inline(always)]
        pub fn set_tscim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Peripheral LCDIM interrupt mask to CPU1"]
        #[inline(always)]
        pub const fn lcdim(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral LCDIM interrupt mask to CPU1"]
        #[inline(always)]
        pub fn set_lcdim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
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
        #[doc = "Floating Point Unit interrupts enable bits"]
        #[inline(always)]
        pub const fn fpu_ie(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x3f;
            val as u8
        }
        #[doc = "Floating Point Unit interrupts enable bits"]
        #[inline(always)]
        pub fn set_fpu_ie(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 26usize)) | (((val as u32) & 0x3f) << 26usize);
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
        #[doc = "Cortex-M4 LOCKUP (Hardfault) output enable bit"]
        #[inline(always)]
        pub const fn cll(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Cortex-M4 LOCKUP (Hardfault) output enable bit"]
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
    #[doc = "external interrupt configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exticr(pub u32);
    impl Exticr {
        #[doc = "EXTI 0 configuration bits"]
        #[inline(always)]
        pub const fn exti(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x07;
            val as u8
        }
        #[doc = "EXTI 0 configuration bits"]
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
    #[doc = "CPU1 interrupt mask register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Imr1(pub u32);
    impl Imr1 {
        #[doc = "Peripheral TIM1 interrupt mask to CPU1"]
        #[inline(always)]
        pub const fn tim1im(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral TIM1 interrupt mask to CPU1"]
        #[inline(always)]
        pub fn set_tim1im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Peripheral TIM16 interrupt mask to CPU1"]
        #[inline(always)]
        pub const fn tim16im(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral TIM16 interrupt mask to CPU1"]
        #[inline(always)]
        pub fn set_tim16im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Peripheral TIM17 interrupt mask to CPU1"]
        #[inline(always)]
        pub const fn tim17im(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral TIM17 interrupt mask to CPU1"]
        #[inline(always)]
        pub fn set_tim17im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Peripheral EXIT5 interrupt mask to CPU1"]
        #[inline(always)]
        pub const fn exit5im(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral EXIT5 interrupt mask to CPU1"]
        #[inline(always)]
        pub fn set_exit5im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Peripheral EXIT6 interrupt mask to CPU1"]
        #[inline(always)]
        pub const fn exit6im(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral EXIT6 interrupt mask to CPU1"]
        #[inline(always)]
        pub fn set_exit6im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Peripheral EXIT7 interrupt mask to CPU1"]
        #[inline(always)]
        pub const fn exit7im(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral EXIT7 interrupt mask to CPU1"]
        #[inline(always)]
        pub fn set_exit7im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Peripheral EXIT8 interrupt mask to CPU1"]
        #[inline(always)]
        pub const fn exit8im(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral EXIT8 interrupt mask to CPU1"]
        #[inline(always)]
        pub fn set_exit8im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Peripheral EXIT9 interrupt mask to CPU1"]
        #[inline(always)]
        pub const fn exit9im(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral EXIT9 interrupt mask to CPU1"]
        #[inline(always)]
        pub fn set_exit9im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Peripheral EXIT10 interrupt mask to CPU1"]
        #[inline(always)]
        pub const fn exit10im(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral EXIT10 interrupt mask to CPU1"]
        #[inline(always)]
        pub fn set_exit10im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Peripheral EXIT11 interrupt mask to CPU1"]
        #[inline(always)]
        pub const fn exit11im(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral EXIT11 interrupt mask to CPU1"]
        #[inline(always)]
        pub fn set_exit11im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Peripheral EXIT12 interrupt mask to CPU1"]
        #[inline(always)]
        pub const fn exit12im(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral EXIT12 interrupt mask to CPU1"]
        #[inline(always)]
        pub fn set_exit12im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Peripheral EXIT13 interrupt mask to CPU1"]
        #[inline(always)]
        pub const fn exit13im(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral EXIT13 interrupt mask to CPU1"]
        #[inline(always)]
        pub fn set_exit13im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Peripheral EXIT14 interrupt mask to CPU1"]
        #[inline(always)]
        pub const fn exit14im(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral EXIT14 interrupt mask to CPU1"]
        #[inline(always)]
        pub fn set_exit14im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Peripheral EXIT15 interrupt mask to CPU1"]
        #[inline(always)]
        pub const fn exit15im(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral EXIT15 interrupt mask to CPU1"]
        #[inline(always)]
        pub fn set_exit15im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Imr1 {
        #[inline(always)]
        fn default() -> Imr1 {
            Imr1(0)
        }
    }
    #[doc = "CPU1 interrupt mask register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Imr2(pub u32);
    impl Imr2 {
        #[doc = "Peripheral PVM1 interrupt mask to CPU1"]
        #[inline(always)]
        pub const fn pvm1im(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral PVM1 interrupt mask to CPU1"]
        #[inline(always)]
        pub fn set_pvm1im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Peripheral PVM3 interrupt mask to CPU1"]
        #[inline(always)]
        pub const fn pvm3im(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral PVM3 interrupt mask to CPU1"]
        #[inline(always)]
        pub fn set_pvm3im(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Peripheral PVD interrupt mask to CPU1"]
        #[inline(always)]
        pub const fn pvdim(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral PVD interrupt mask to CPU1"]
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
    #[doc = "SCSR"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scsr(pub u32);
    impl Scsr {
        #[doc = "SRAM2 Erase"]
        #[inline(always)]
        pub const fn sram2er(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 Erase"]
        #[inline(always)]
        pub fn set_sram2er(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SRAM2 busy by erase operation"]
        #[inline(always)]
        pub const fn sram2bsy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 busy by erase operation"]
        #[inline(always)]
        pub fn set_sram2bsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CPU2 SRAM fetch (execution) disable."]
        #[inline(always)]
        pub const fn c2rfd(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "CPU2 SRAM fetch (execution) disable."]
        #[inline(always)]
        pub fn set_c2rfd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Scsr {
        #[inline(always)]
        fn default() -> Scsr {
            Scsr(0)
        }
    }
    #[doc = "secure IP control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sipcr(pub u32);
    impl Sipcr {
        #[doc = "Enable AES1 KEY\\[7:0\\]
security."]
        #[inline(always)]
        pub const fn saes(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Enable AES1 KEY\\[7:0\\]
security."]
        #[inline(always)]
        pub fn set_saes(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Enable PKA security"]
        #[inline(always)]
        pub const fn spka(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Enable PKA security"]
        #[inline(always)]
        pub fn set_spka(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Enable True RNG security"]
        #[inline(always)]
        pub const fn srng(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Enable True RNG security"]
        #[inline(always)]
        pub fn set_srng(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Sipcr {
        #[inline(always)]
        fn default() -> Sipcr {
            Sipcr(0)
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
    #[doc = "SRAM2 write protection register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Swpr(pub u32);
    impl Swpr {
        #[doc = "P0WP"]
        #[inline(always)]
        pub const fn pwp(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "P0WP"]
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
    #[doc = "SRAM2 write protection register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Swpr2(pub u32);
    impl Swpr2 {
        #[doc = "P32WP"]
        #[inline(always)]
        pub const fn pwp(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "P32WP"]
        #[inline(always)]
        pub fn set_pwp(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Swpr2 {
        #[inline(always)]
        fn default() -> Swpr2 {
            Swpr2(0)
        }
    }
}
