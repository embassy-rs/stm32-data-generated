#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Reset and clock control"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcc {
    ptr: *mut u8,
}
unsafe impl Send for Rcc {}
unsafe impl Sync for Rcc {}
impl Rcc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Clock control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Internal clock sources calibration register"]
    #[inline(always)]
    pub const fn icscr(self) -> crate::common::Reg<regs::Icscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Clock configuration register"]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "PLL configuration register"]
    #[inline(always)]
    pub const fn pllcfgr(self) -> crate::common::Reg<regs::Pllcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "PLLSAI1 configuration register"]
    #[inline(always)]
    pub const fn pllsai1cfgr(self) -> crate::common::Reg<regs::Pllcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "PLLSAI2 configuration register"]
    #[inline(always)]
    pub const fn pllsai2cfgr(self) -> crate::common::Reg<regs::Pllcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Clock interrupt enable register"]
    #[inline(always)]
    pub const fn cier(self) -> crate::common::Reg<regs::Cier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Clock interrupt flag register"]
    #[inline(always)]
    pub const fn cifr(self) -> crate::common::Reg<regs::Cifr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Clock interrupt clear register"]
    #[inline(always)]
    pub const fn cicr(self) -> crate::common::Reg<regs::Cicr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "AHB1 peripheral reset register"]
    #[inline(always)]
    pub const fn ahb1rstr(self) -> crate::common::Reg<regs::Ahb1rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "AHB2 peripheral reset register"]
    #[inline(always)]
    pub const fn ahb2rstr(self) -> crate::common::Reg<regs::Ahb2rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "AHB3 peripheral reset register"]
    #[inline(always)]
    pub const fn ahb3rstr(self) -> crate::common::Reg<regs::Ahb3rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "APB1 peripheral reset register 1"]
    #[inline(always)]
    pub const fn apb1rstr1(self) -> crate::common::Reg<regs::Apb1rstr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "APB1 peripheral reset register 2"]
    #[inline(always)]
    pub const fn apb1rstr2(self) -> crate::common::Reg<regs::Apb1rstr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "APB2 peripheral reset register"]
    #[inline(always)]
    pub const fn apb2rstr(self) -> crate::common::Reg<regs::Apb2rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "AHB1 peripheral clock enable register"]
    #[inline(always)]
    pub const fn ahb1enr(self) -> crate::common::Reg<regs::Ahb1enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "AHB2 peripheral clock enable register"]
    #[inline(always)]
    pub const fn ahb2enr(self) -> crate::common::Reg<regs::Ahb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "AHB3 peripheral clock enable register"]
    #[inline(always)]
    pub const fn ahb3enr(self) -> crate::common::Reg<regs::Ahb3enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "APB1ENR1"]
    #[inline(always)]
    pub const fn apb1enr1(self) -> crate::common::Reg<regs::Apb1enr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "APB1 peripheral clock enable register 2"]
    #[inline(always)]
    pub const fn apb1enr2(self) -> crate::common::Reg<regs::Apb1enr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "APB2ENR"]
    #[inline(always)]
    pub const fn apb2enr(self) -> crate::common::Reg<regs::Apb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "AHB1 peripheral clocks enable in Sleep and Stop modes register"]
    #[inline(always)]
    pub const fn ahb1smenr(self) -> crate::common::Reg<regs::Ahb1smenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "AHB2 peripheral clocks enable in Sleep and Stop modes register"]
    #[inline(always)]
    pub const fn ahb2smenr(self) -> crate::common::Reg<regs::Ahb2smenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "AHB3 peripheral clocks enable in Sleep and Stop modes register"]
    #[inline(always)]
    pub const fn ahb3smenr(self) -> crate::common::Reg<regs::Ahb3smenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "APB1SMENR1"]
    #[inline(always)]
    pub const fn apb1smenr1(self) -> crate::common::Reg<regs::Apb1smenr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "APB1 peripheral clocks enable in Sleep and Stop modes register 2"]
    #[inline(always)]
    pub const fn apb1smenr2(self) -> crate::common::Reg<regs::Apb1smenr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "APB2SMENR"]
    #[inline(always)]
    pub const fn apb2smenr(self) -> crate::common::Reg<regs::Apb2smenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "CCIPR"]
    #[inline(always)]
    pub const fn ccipr(self) -> crate::common::Reg<regs::Ccipr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "BDCR"]
    #[inline(always)]
    pub const fn bdcr(self) -> crate::common::Reg<regs::Bdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "CSR"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "Clock recovery RC register"]
    #[inline(always)]
    pub const fn crrcr(self) -> crate::common::Reg<regs::Crrcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "Peripherals independent clock configuration register"]
    #[inline(always)]
    pub const fn ccipr2(self) -> crate::common::Reg<regs::Ccipr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "RCC secure configuration register"]
    #[inline(always)]
    pub const fn seccfgr(self) -> crate::common::Reg<regs::Seccfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[doc = "RCC secure status register"]
    #[inline(always)]
    pub const fn secsr(self) -> crate::common::Reg<regs::Secsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[doc = "RCC AHB1 security status register"]
    #[inline(always)]
    pub const fn ahb1secsr(self) -> crate::common::Reg<regs::Ahb1secsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe8usize) as _) }
    }
    #[doc = "RCC AHB2 security status register"]
    #[inline(always)]
    pub const fn ahb2secsr(self) -> crate::common::Reg<regs::Ahb2secsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xecusize) as _) }
    }
    #[doc = "RCC AHB3 security status register"]
    #[inline(always)]
    pub const fn ahb3secsr(self) -> crate::common::Reg<regs::Ahb3secsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[doc = "RCC APB1 security status register 1"]
    #[inline(always)]
    pub const fn apb1secsr1(self) -> crate::common::Reg<regs::Apb1secsr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
    #[doc = "RCC APB1 security status register 2"]
    #[inline(always)]
    pub const fn apb1secsr2(self) -> crate::common::Reg<regs::Apb1secsr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
    #[doc = "RCC APB2 security status register"]
    #[inline(always)]
    pub const fn apb2secsr(self) -> crate::common::Reg<regs::Apb2secsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
}
pub mod regs {
    #[doc = "AHB1 peripheral clock enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1enr(pub u32);
    impl Ahb1enr {
        #[doc = "DMA1 clock enable"]
        #[inline(always)]
        pub const fn dma1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 clock enable"]
        #[inline(always)]
        pub fn set_dma1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2 clock enable"]
        #[inline(always)]
        pub const fn dma2en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 clock enable"]
        #[inline(always)]
        pub fn set_dma2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DMAMUX clock enable"]
        #[inline(always)]
        pub const fn dmamux1en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DMAMUX clock enable"]
        #[inline(always)]
        pub fn set_dmamux1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Flash memory interface clock enable"]
        #[inline(always)]
        pub const fn flashen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Flash memory interface clock enable"]
        #[inline(always)]
        pub fn set_flashen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CRC clock enable"]
        #[inline(always)]
        pub const fn crcen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC clock enable"]
        #[inline(always)]
        pub fn set_crcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Touch Sensing Controller clock enable"]
        #[inline(always)]
        pub const fn tscen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Touch Sensing Controller clock enable"]
        #[inline(always)]
        pub fn set_tscen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "GTZCEN"]
        #[inline(always)]
        pub const fn gtzcen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "GTZCEN"]
        #[inline(always)]
        pub fn set_gtzcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Ahb1enr {
        #[inline(always)]
        fn default() -> Ahb1enr {
            Ahb1enr(0)
        }
    }
    #[doc = "AHB1 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1rstr(pub u32);
    impl Ahb1rstr {
        #[doc = "DMA1 reset"]
        #[inline(always)]
        pub const fn dma1rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 reset"]
        #[inline(always)]
        pub fn set_dma1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2 reset"]
        #[inline(always)]
        pub const fn dma2rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 reset"]
        #[inline(always)]
        pub fn set_dma2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DMAMUX1RST"]
        #[inline(always)]
        pub const fn dmamux1rst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DMAMUX1RST"]
        #[inline(always)]
        pub fn set_dmamux1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Flash memory interface reset"]
        #[inline(always)]
        pub const fn flashrst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Flash memory interface reset"]
        #[inline(always)]
        pub fn set_flashrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CRC reset"]
        #[inline(always)]
        pub const fn crcrst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC reset"]
        #[inline(always)]
        pub fn set_crcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Touch Sensing Controller reset"]
        #[inline(always)]
        pub const fn tscrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Touch Sensing Controller reset"]
        #[inline(always)]
        pub fn set_tscrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "GTZC reset"]
        #[inline(always)]
        pub const fn gtzcrst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "GTZC reset"]
        #[inline(always)]
        pub fn set_gtzcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Ahb1rstr {
        #[inline(always)]
        fn default() -> Ahb1rstr {
            Ahb1rstr(0)
        }
    }
    #[doc = "RCC AHB1 security status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1secsr(pub u32);
    impl Ahb1secsr {
        #[doc = "DMA1SECF"]
        #[inline(always)]
        pub const fn dma1secf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1SECF"]
        #[inline(always)]
        pub fn set_dma1secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2SECF"]
        #[inline(always)]
        pub const fn dma2secf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2SECF"]
        #[inline(always)]
        pub fn set_dma2secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DMAMUX1SECF"]
        #[inline(always)]
        pub const fn dmamux1secf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DMAMUX1SECF"]
        #[inline(always)]
        pub fn set_dmamux1secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "FLASHSECF"]
        #[inline(always)]
        pub const fn flashsecf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FLASHSECF"]
        #[inline(always)]
        pub fn set_flashsecf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "SRAM1SECF"]
        #[inline(always)]
        pub const fn sram1secf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1SECF"]
        #[inline(always)]
        pub fn set_sram1secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "CRCSECF"]
        #[inline(always)]
        pub const fn crcsecf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRCSECF"]
        #[inline(always)]
        pub fn set_crcsecf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TSCSECF"]
        #[inline(always)]
        pub const fn tscsecf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TSCSECF"]
        #[inline(always)]
        pub fn set_tscsecf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "GTZCSECF"]
        #[inline(always)]
        pub const fn gtzcsecf(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "GTZCSECF"]
        #[inline(always)]
        pub fn set_gtzcsecf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "ICACHESECF"]
        #[inline(always)]
        pub const fn icachesecf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "ICACHESECF"]
        #[inline(always)]
        pub fn set_icachesecf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Ahb1secsr {
        #[inline(always)]
        fn default() -> Ahb1secsr {
            Ahb1secsr(0)
        }
    }
    #[doc = "AHB1 peripheral clocks enable in Sleep and Stop modes register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1smenr(pub u32);
    impl Ahb1smenr {
        #[doc = "DMA1 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn dma1smen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_dma1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn dma2smen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_dma2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DMAMUX clock enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn dmamux1smen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DMAMUX clock enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_dmamux1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Flash memory interface clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn flashsmen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Flash memory interface clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_flashsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "SRAM1 interface clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn sram1smen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 interface clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_sram1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "CRCSMEN"]
        #[inline(always)]
        pub const fn crcsmen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRCSMEN"]
        #[inline(always)]
        pub fn set_crcsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Touch Sensing Controller clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn tscsmen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Touch Sensing Controller clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_tscsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "GTZCSMEN"]
        #[inline(always)]
        pub const fn gtzcsmen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "GTZCSMEN"]
        #[inline(always)]
        pub fn set_gtzcsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "ICACHESMEN"]
        #[inline(always)]
        pub const fn icachesmen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "ICACHESMEN"]
        #[inline(always)]
        pub fn set_icachesmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Ahb1smenr {
        #[inline(always)]
        fn default() -> Ahb1smenr {
            Ahb1smenr(0)
        }
    }
    #[doc = "AHB2 peripheral clock enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2enr(pub u32);
    impl Ahb2enr {
        #[doc = "IO port A clock enable"]
        #[inline(always)]
        pub const fn gpioaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IO port A clock enable"]
        #[inline(always)]
        pub fn set_gpioaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IO port B clock enable"]
        #[inline(always)]
        pub const fn gpioben(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IO port B clock enable"]
        #[inline(always)]
        pub fn set_gpioben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IO port C clock enable"]
        #[inline(always)]
        pub const fn gpiocen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IO port C clock enable"]
        #[inline(always)]
        pub fn set_gpiocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "IO port D clock enable"]
        #[inline(always)]
        pub const fn gpioden(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IO port D clock enable"]
        #[inline(always)]
        pub fn set_gpioden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "IO port E clock enable"]
        #[inline(always)]
        pub const fn gpioeen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IO port E clock enable"]
        #[inline(always)]
        pub fn set_gpioeen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "IO port F clock enable"]
        #[inline(always)]
        pub const fn gpiofen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "IO port F clock enable"]
        #[inline(always)]
        pub fn set_gpiofen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "IO port G clock enable"]
        #[inline(always)]
        pub const fn gpiogen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "IO port G clock enable"]
        #[inline(always)]
        pub fn set_gpiogen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "IO port H clock enable"]
        #[inline(always)]
        pub const fn gpiohen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "IO port H clock enable"]
        #[inline(always)]
        pub fn set_gpiohen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ADC clock enable"]
        #[inline(always)]
        pub const fn adcen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "ADC clock enable"]
        #[inline(always)]
        pub fn set_adcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "AES accelerator clock enable"]
        #[inline(always)]
        pub const fn aesen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "AES accelerator clock enable"]
        #[inline(always)]
        pub fn set_aesen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "HASH clock enable"]
        #[inline(always)]
        pub const fn hashen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "HASH clock enable"]
        #[inline(always)]
        pub fn set_hashen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Random Number Generator clock enable"]
        #[inline(always)]
        pub const fn rngen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Random Number Generator clock enable"]
        #[inline(always)]
        pub fn set_rngen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PKAEN"]
        #[inline(always)]
        pub const fn pkaen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "PKAEN"]
        #[inline(always)]
        pub fn set_pkaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "OTFDEC1EN"]
        #[inline(always)]
        pub const fn otfdec1en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC1EN"]
        #[inline(always)]
        pub fn set_otfdec1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SDMMC1 clock enable"]
        #[inline(always)]
        pub const fn sdmmc1en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC1 clock enable"]
        #[inline(always)]
        pub fn set_sdmmc1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Ahb2enr {
        #[inline(always)]
        fn default() -> Ahb2enr {
            Ahb2enr(0)
        }
    }
    #[doc = "AHB2 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2rstr(pub u32);
    impl Ahb2rstr {
        #[doc = "IO port A reset"]
        #[inline(always)]
        pub const fn gpioarst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IO port A reset"]
        #[inline(always)]
        pub fn set_gpioarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IO port B reset"]
        #[inline(always)]
        pub const fn gpiobrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IO port B reset"]
        #[inline(always)]
        pub fn set_gpiobrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IO port C reset"]
        #[inline(always)]
        pub const fn gpiocrst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IO port C reset"]
        #[inline(always)]
        pub fn set_gpiocrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "IO port D reset"]
        #[inline(always)]
        pub const fn gpiodrst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IO port D reset"]
        #[inline(always)]
        pub fn set_gpiodrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "IO port E reset"]
        #[inline(always)]
        pub const fn gpioerst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IO port E reset"]
        #[inline(always)]
        pub fn set_gpioerst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "IO port F reset"]
        #[inline(always)]
        pub const fn gpiofrst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "IO port F reset"]
        #[inline(always)]
        pub fn set_gpiofrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "IO port G reset"]
        #[inline(always)]
        pub const fn gpiogrst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "IO port G reset"]
        #[inline(always)]
        pub fn set_gpiogrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "IO port H reset"]
        #[inline(always)]
        pub const fn gpiohrst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "IO port H reset"]
        #[inline(always)]
        pub fn set_gpiohrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ADC reset"]
        #[inline(always)]
        pub const fn adcrst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "ADC reset"]
        #[inline(always)]
        pub fn set_adcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "AES hardware accelerator reset"]
        #[inline(always)]
        pub const fn aesrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "AES hardware accelerator reset"]
        #[inline(always)]
        pub fn set_aesrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Hash reset"]
        #[inline(always)]
        pub const fn hashrst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Hash reset"]
        #[inline(always)]
        pub fn set_hashrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Random number generator reset"]
        #[inline(always)]
        pub const fn rngrst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Random number generator reset"]
        #[inline(always)]
        pub fn set_rngrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PKARST"]
        #[inline(always)]
        pub const fn pkarst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "PKARST"]
        #[inline(always)]
        pub fn set_pkarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "OTFDEC1RST"]
        #[inline(always)]
        pub const fn otfdec1rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC1RST"]
        #[inline(always)]
        pub fn set_otfdec1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SDMMC1 reset"]
        #[inline(always)]
        pub const fn sdmmc1rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC1 reset"]
        #[inline(always)]
        pub fn set_sdmmc1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Ahb2rstr {
        #[inline(always)]
        fn default() -> Ahb2rstr {
            Ahb2rstr(0)
        }
    }
    #[doc = "RCC AHB2 security status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2secsr(pub u32);
    impl Ahb2secsr {
        #[doc = "GPIOASECF"]
        #[inline(always)]
        pub const fn gpioasecf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOASECF"]
        #[inline(always)]
        pub fn set_gpioasecf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPIOBSECF"]
        #[inline(always)]
        pub const fn gpiobsecf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOBSECF"]
        #[inline(always)]
        pub fn set_gpiobsecf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "GPIOCSECF"]
        #[inline(always)]
        pub const fn gpiocsecf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOCSECF"]
        #[inline(always)]
        pub fn set_gpiocsecf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "GPIODSECF"]
        #[inline(always)]
        pub const fn gpiodsecf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "GPIODSECF"]
        #[inline(always)]
        pub fn set_gpiodsecf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "GPIOESECF"]
        #[inline(always)]
        pub const fn gpioesecf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOESECF"]
        #[inline(always)]
        pub fn set_gpioesecf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "GPIOFSECF"]
        #[inline(always)]
        pub const fn gpiofsecf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOFSECF"]
        #[inline(always)]
        pub fn set_gpiofsecf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "GPIOGSECF"]
        #[inline(always)]
        pub const fn gpiogsecf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOGSECF"]
        #[inline(always)]
        pub fn set_gpiogsecf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "GPIOHSECF"]
        #[inline(always)]
        pub const fn gpiohsecf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOHSECF"]
        #[inline(always)]
        pub fn set_gpiohsecf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "SRAM2SECF"]
        #[inline(always)]
        pub const fn sram2secf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2SECF"]
        #[inline(always)]
        pub fn set_sram2secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "OTFDEC1SECF"]
        #[inline(always)]
        pub const fn otfdec1secf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC1SECF"]
        #[inline(always)]
        pub fn set_otfdec1secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SDMMC1SECF"]
        #[inline(always)]
        pub const fn sdmmc1secf(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC1SECF"]
        #[inline(always)]
        pub fn set_sdmmc1secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Ahb2secsr {
        #[inline(always)]
        fn default() -> Ahb2secsr {
            Ahb2secsr(0)
        }
    }
    #[doc = "AHB2 peripheral clocks enable in Sleep and Stop modes register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2smenr(pub u32);
    impl Ahb2smenr {
        #[doc = "IO port A clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn gpioasmen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IO port A clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_gpioasmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IO port B clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn gpiobsmen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IO port B clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_gpiobsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IO port C clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn gpiocsmen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IO port C clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_gpiocsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "IO port D clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn gpiodsmen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IO port D clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_gpiodsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "IO port E clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn gpioesmen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IO port E clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_gpioesmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "IO port F clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn gpiofsmen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "IO port F clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_gpiofsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "IO port G clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn gpiogsmen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "IO port G clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_gpiogsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "IO port H clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn gpiohsmen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "IO port H clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_gpiohsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "SRAM2 interface clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn sram2smen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 interface clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_sram2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "ADC clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn adcfssmen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "ADC clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_adcfssmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "AES accelerator clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn aessmen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "AES accelerator clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_aessmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "HASH clock enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn hashsmen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "HASH clock enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_hashsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Random Number Generator clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn rngsmen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Random Number Generator clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_rngsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PKASMEN"]
        #[inline(always)]
        pub const fn pkasmen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "PKASMEN"]
        #[inline(always)]
        pub fn set_pkasmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "OTFDEC1SMEN"]
        #[inline(always)]
        pub const fn otfdec1smen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC1SMEN"]
        #[inline(always)]
        pub fn set_otfdec1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SDMMC1 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn sdmmc1smen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC1 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_sdmmc1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Ahb2smenr {
        #[inline(always)]
        fn default() -> Ahb2smenr {
            Ahb2smenr(0)
        }
    }
    #[doc = "AHB3 peripheral clock enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3enr(pub u32);
    impl Ahb3enr {
        #[doc = "Flexible memory controller clock enable"]
        #[inline(always)]
        pub const fn fmcen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Flexible memory controller clock enable"]
        #[inline(always)]
        pub fn set_fmcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "OCTOSPI1EN"]
        #[inline(always)]
        pub const fn octospi1en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI1EN"]
        #[inline(always)]
        pub fn set_octospi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Ahb3enr {
        #[inline(always)]
        fn default() -> Ahb3enr {
            Ahb3enr(0)
        }
    }
    #[doc = "AHB3 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3rstr(pub u32);
    impl Ahb3rstr {
        #[doc = "Flexible memory controller reset"]
        #[inline(always)]
        pub const fn fmcrst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Flexible memory controller reset"]
        #[inline(always)]
        pub fn set_fmcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "OCTOSPI1RST"]
        #[inline(always)]
        pub const fn octospi1rst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI1RST"]
        #[inline(always)]
        pub fn set_octospi1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Ahb3rstr {
        #[inline(always)]
        fn default() -> Ahb3rstr {
            Ahb3rstr(0)
        }
    }
    #[doc = "RCC AHB3 security status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3secsr(pub u32);
    impl Ahb3secsr {
        #[doc = "FSMCSECF"]
        #[inline(always)]
        pub const fn fsmcsecf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "FSMCSECF"]
        #[inline(always)]
        pub fn set_fsmcsecf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "OCTOSPI1SECF"]
        #[inline(always)]
        pub const fn octospi1secf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI1SECF"]
        #[inline(always)]
        pub fn set_octospi1secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Ahb3secsr {
        #[inline(always)]
        fn default() -> Ahb3secsr {
            Ahb3secsr(0)
        }
    }
    #[doc = "AHB3 peripheral clocks enable in Sleep and Stop modes register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3smenr(pub u32);
    impl Ahb3smenr {
        #[doc = "Flexible memory controller clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn fmcsmen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Flexible memory controller clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_fmcsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "OCTOSPI1SMEN"]
        #[inline(always)]
        pub const fn octospi1smen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI1SMEN"]
        #[inline(always)]
        pub fn set_octospi1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Ahb3smenr {
        #[inline(always)]
        fn default() -> Ahb3smenr {
            Ahb3smenr(0)
        }
    }
    #[doc = "APB1ENR1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1enr1(pub u32);
    impl Apb1enr1 {
        #[doc = "TIM2 timer clock enable"]
        #[inline(always)]
        pub const fn tim2en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 timer clock enable"]
        #[inline(always)]
        pub fn set_tim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 timer clock enable"]
        #[inline(always)]
        pub const fn tim3en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 timer clock enable"]
        #[inline(always)]
        pub fn set_tim3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 timer clock enable"]
        #[inline(always)]
        pub const fn tim4en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 timer clock enable"]
        #[inline(always)]
        pub fn set_tim4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5 timer clock enable"]
        #[inline(always)]
        pub const fn tim5en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 timer clock enable"]
        #[inline(always)]
        pub fn set_tim5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 timer clock enable"]
        #[inline(always)]
        pub const fn tim6en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 timer clock enable"]
        #[inline(always)]
        pub fn set_tim6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 timer clock enable"]
        #[inline(always)]
        pub const fn tim7en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 timer clock enable"]
        #[inline(always)]
        pub fn set_tim7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "RTC APB clock enable"]
        #[inline(always)]
        pub const fn rtcapben(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "RTC APB clock enable"]
        #[inline(always)]
        pub fn set_rtcapben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Window watchdog clock enable"]
        #[inline(always)]
        pub const fn wwdgen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog clock enable"]
        #[inline(always)]
        pub fn set_wwdgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI2 clock enable"]
        #[inline(always)]
        pub const fn spi2en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 clock enable"]
        #[inline(always)]
        pub fn set_spi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 clock enable"]
        #[inline(always)]
        pub const fn spi3en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 clock enable"]
        #[inline(always)]
        pub fn set_spi3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "USART2 clock enable"]
        #[inline(always)]
        pub const fn usart2en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 clock enable"]
        #[inline(always)]
        pub fn set_usart2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 clock enable"]
        #[inline(always)]
        pub const fn usart3en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 clock enable"]
        #[inline(always)]
        pub fn set_usart3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 clock enable"]
        #[inline(always)]
        pub const fn uart4en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 clock enable"]
        #[inline(always)]
        pub fn set_uart4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 clock enable"]
        #[inline(always)]
        pub const fn uart5en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 clock enable"]
        #[inline(always)]
        pub fn set_uart5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 clock enable"]
        #[inline(always)]
        pub const fn i2c1en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 clock enable"]
        #[inline(always)]
        pub fn set_i2c1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 clock enable"]
        #[inline(always)]
        pub const fn i2c2en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 clock enable"]
        #[inline(always)]
        pub fn set_i2c2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I2C3 clock enable"]
        #[inline(always)]
        pub const fn i2c3en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 clock enable"]
        #[inline(always)]
        pub fn set_i2c3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Clock Recovery System clock enable"]
        #[inline(always)]
        pub const fn crsen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Clock Recovery System clock enable"]
        #[inline(always)]
        pub fn set_crsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Power interface clock enable"]
        #[inline(always)]
        pub const fn pwren(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Power interface clock enable"]
        #[inline(always)]
        pub fn set_pwren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "DAC1 interface clock enable"]
        #[inline(always)]
        pub const fn dac1en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1 interface clock enable"]
        #[inline(always)]
        pub fn set_dac1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "OPAMP interface clock enable"]
        #[inline(always)]
        pub const fn opampen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP interface clock enable"]
        #[inline(always)]
        pub fn set_opampen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Low power timer 1 clock enable"]
        #[inline(always)]
        pub const fn lptim1en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Low power timer 1 clock enable"]
        #[inline(always)]
        pub fn set_lptim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Apb1enr1 {
        #[inline(always)]
        fn default() -> Apb1enr1 {
            Apb1enr1(0)
        }
    }
    #[doc = "APB1 peripheral clock enable register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1enr2(pub u32);
    impl Apb1enr2 {
        #[doc = "Low power UART 1 clock enable"]
        #[inline(always)]
        pub const fn lpuart1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Low power UART 1 clock enable"]
        #[inline(always)]
        pub fn set_lpuart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "I2C4 clock enable"]
        #[inline(always)]
        pub const fn i2c4en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 clock enable"]
        #[inline(always)]
        pub fn set_i2c4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPTIM2EN"]
        #[inline(always)]
        pub const fn lptim2en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2EN"]
        #[inline(always)]
        pub fn set_lptim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LPTIM3EN"]
        #[inline(always)]
        pub const fn lptim3en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3EN"]
        #[inline(always)]
        pub fn set_lptim3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "FDCAN1EN"]
        #[inline(always)]
        pub const fn fdcan1en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN1EN"]
        #[inline(always)]
        pub fn set_fdcan1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "USBEN"]
        #[inline(always)]
        pub const fn usben(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "USBEN"]
        #[inline(always)]
        pub fn set_usben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "UCPD1EN"]
        #[inline(always)]
        pub const fn ucpd1en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "UCPD1EN"]
        #[inline(always)]
        pub fn set_ucpd1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Apb1enr2 {
        #[inline(always)]
        fn default() -> Apb1enr2 {
            Apb1enr2(0)
        }
    }
    #[doc = "APB1 peripheral reset register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1rstr1(pub u32);
    impl Apb1rstr1 {
        #[doc = "TIM2 timer reset"]
        #[inline(always)]
        pub const fn tim2rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 timer reset"]
        #[inline(always)]
        pub fn set_tim2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 timer reset"]
        #[inline(always)]
        pub const fn tim3rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 timer reset"]
        #[inline(always)]
        pub fn set_tim3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM3 timer reset"]
        #[inline(always)]
        pub const fn tim4rst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 timer reset"]
        #[inline(always)]
        pub fn set_tim4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5 timer reset"]
        #[inline(always)]
        pub const fn tim5rst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 timer reset"]
        #[inline(always)]
        pub fn set_tim5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 timer reset"]
        #[inline(always)]
        pub const fn tim6rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 timer reset"]
        #[inline(always)]
        pub fn set_tim6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 timer reset"]
        #[inline(always)]
        pub const fn tim7rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 timer reset"]
        #[inline(always)]
        pub fn set_tim7rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SPI2 reset"]
        #[inline(always)]
        pub const fn spi2rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 reset"]
        #[inline(always)]
        pub fn set_spi2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 reset"]
        #[inline(always)]
        pub const fn spi3rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 reset"]
        #[inline(always)]
        pub fn set_spi3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "USART2 reset"]
        #[inline(always)]
        pub const fn usart2rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 reset"]
        #[inline(always)]
        pub fn set_usart2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 reset"]
        #[inline(always)]
        pub const fn usart3rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 reset"]
        #[inline(always)]
        pub fn set_usart3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 reset"]
        #[inline(always)]
        pub const fn uart4rst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 reset"]
        #[inline(always)]
        pub fn set_uart4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 reset"]
        #[inline(always)]
        pub const fn uart5rst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 reset"]
        #[inline(always)]
        pub fn set_uart5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 reset"]
        #[inline(always)]
        pub const fn i2c1rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 reset"]
        #[inline(always)]
        pub fn set_i2c1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 reset"]
        #[inline(always)]
        pub const fn i2c2rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 reset"]
        #[inline(always)]
        pub fn set_i2c2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I2C3 reset"]
        #[inline(always)]
        pub const fn i2c3rst(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 reset"]
        #[inline(always)]
        pub fn set_i2c3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "CRS reset"]
        #[inline(always)]
        pub const fn crsrst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "CRS reset"]
        #[inline(always)]
        pub fn set_crsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Power interface reset"]
        #[inline(always)]
        pub const fn pwrrst(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Power interface reset"]
        #[inline(always)]
        pub fn set_pwrrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "DAC1 interface reset"]
        #[inline(always)]
        pub const fn dac1rst(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1 interface reset"]
        #[inline(always)]
        pub fn set_dac1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "OPAMP interface reset"]
        #[inline(always)]
        pub const fn opamprst(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP interface reset"]
        #[inline(always)]
        pub fn set_opamprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Low Power Timer 1 reset"]
        #[inline(always)]
        pub const fn lptim1rst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Low Power Timer 1 reset"]
        #[inline(always)]
        pub fn set_lptim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Apb1rstr1 {
        #[inline(always)]
        fn default() -> Apb1rstr1 {
            Apb1rstr1(0)
        }
    }
    #[doc = "APB1 peripheral reset register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1rstr2(pub u32);
    impl Apb1rstr2 {
        #[doc = "Low-power UART 1 reset"]
        #[inline(always)]
        pub const fn lpuart1rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power UART 1 reset"]
        #[inline(always)]
        pub fn set_lpuart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "I2C4 reset"]
        #[inline(always)]
        pub const fn i2c4rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 reset"]
        #[inline(always)]
        pub fn set_i2c4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Low-power timer 2 reset"]
        #[inline(always)]
        pub const fn lptim2rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power timer 2 reset"]
        #[inline(always)]
        pub fn set_lptim2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LPTIM3RST"]
        #[inline(always)]
        pub const fn lptim3rst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3RST"]
        #[inline(always)]
        pub fn set_lptim3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "FDCAN1RST"]
        #[inline(always)]
        pub const fn fdcan1rst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN1RST"]
        #[inline(always)]
        pub fn set_fdcan1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "USBRST"]
        #[inline(always)]
        pub const fn usbrst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "USBRST"]
        #[inline(always)]
        pub fn set_usbrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "UCPD1RST"]
        #[inline(always)]
        pub const fn ucpd1rst(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "UCPD1RST"]
        #[inline(always)]
        pub fn set_ucpd1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Apb1rstr2 {
        #[inline(always)]
        fn default() -> Apb1rstr2 {
            Apb1rstr2(0)
        }
    }
    #[doc = "RCC APB1 security status register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1secsr1(pub u32);
    impl Apb1secsr1 {
        #[doc = "TIM2SECF"]
        #[inline(always)]
        pub const fn tim2secf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2SECF"]
        #[inline(always)]
        pub fn set_tim2secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3SECF"]
        #[inline(always)]
        pub const fn tim3secf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3SECF"]
        #[inline(always)]
        pub fn set_tim3secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4SECF"]
        #[inline(always)]
        pub const fn tim4secf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4SECF"]
        #[inline(always)]
        pub fn set_tim4secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5SECF"]
        #[inline(always)]
        pub const fn tim5secf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5SECF"]
        #[inline(always)]
        pub fn set_tim5secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6SECF"]
        #[inline(always)]
        pub const fn tim6secf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6SECF"]
        #[inline(always)]
        pub fn set_tim6secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7SECF"]
        #[inline(always)]
        pub const fn tim7secf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7SECF"]
        #[inline(always)]
        pub fn set_tim7secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "RTCAPBSECF"]
        #[inline(always)]
        pub const fn rtcapbsecf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "RTCAPBSECF"]
        #[inline(always)]
        pub fn set_rtcapbsecf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "WWDGSECF"]
        #[inline(always)]
        pub const fn wwdgsecf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "WWDGSECF"]
        #[inline(always)]
        pub fn set_wwdgsecf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI2SECF"]
        #[inline(always)]
        pub const fn spi2secf(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2SECF"]
        #[inline(always)]
        pub fn set_spi2secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3SECF"]
        #[inline(always)]
        pub const fn spi3secf(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3SECF"]
        #[inline(always)]
        pub fn set_spi3secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "UART2SECF"]
        #[inline(always)]
        pub const fn uart2secf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "UART2SECF"]
        #[inline(always)]
        pub fn set_uart2secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "UART3SECF"]
        #[inline(always)]
        pub const fn uart3secf(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "UART3SECF"]
        #[inline(always)]
        pub fn set_uart3secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4SECF"]
        #[inline(always)]
        pub const fn uart4secf(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4SECF"]
        #[inline(always)]
        pub fn set_uart4secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5SECF"]
        #[inline(always)]
        pub const fn uart5secf(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5SECF"]
        #[inline(always)]
        pub fn set_uart5secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1SECF"]
        #[inline(always)]
        pub const fn i2c1secf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1SECF"]
        #[inline(always)]
        pub fn set_i2c1secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2SECF"]
        #[inline(always)]
        pub const fn i2c2secf(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2SECF"]
        #[inline(always)]
        pub fn set_i2c2secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I2C3SECF"]
        #[inline(always)]
        pub const fn i2c3secf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3SECF"]
        #[inline(always)]
        pub fn set_i2c3secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "CRSSECF"]
        #[inline(always)]
        pub const fn crssecf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "CRSSECF"]
        #[inline(always)]
        pub fn set_crssecf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "PWRSECF"]
        #[inline(always)]
        pub const fn pwrsecf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "PWRSECF"]
        #[inline(always)]
        pub fn set_pwrsecf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "DACSECF"]
        #[inline(always)]
        pub const fn dacsecf(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DACSECF"]
        #[inline(always)]
        pub fn set_dacsecf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "OPAMPSECF"]
        #[inline(always)]
        pub const fn opampsecf(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMPSECF"]
        #[inline(always)]
        pub fn set_opampsecf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "LPTIM1SECF"]
        #[inline(always)]
        pub const fn lptim1secf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1SECF"]
        #[inline(always)]
        pub fn set_lptim1secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Apb1secsr1 {
        #[inline(always)]
        fn default() -> Apb1secsr1 {
            Apb1secsr1(0)
        }
    }
    #[doc = "RCC APB1 security status register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1secsr2(pub u32);
    impl Apb1secsr2 {
        #[doc = "LPUART1SECF"]
        #[inline(always)]
        pub const fn lpuart1secf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1SECF"]
        #[inline(always)]
        pub fn set_lpuart1secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "I2C4SECF"]
        #[inline(always)]
        pub const fn i2c4secf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4SECF"]
        #[inline(always)]
        pub fn set_i2c4secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPTIM2SECF"]
        #[inline(always)]
        pub const fn lptim2secf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2SECF"]
        #[inline(always)]
        pub fn set_lptim2secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LPTIM3SECF"]
        #[inline(always)]
        pub const fn lptim3secf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3SECF"]
        #[inline(always)]
        pub fn set_lptim3secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "FDCAN1SECF"]
        #[inline(always)]
        pub const fn fdcan1secf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN1SECF"]
        #[inline(always)]
        pub fn set_fdcan1secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "USBSECF"]
        #[inline(always)]
        pub const fn usbsecf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "USBSECF"]
        #[inline(always)]
        pub fn set_usbsecf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "UCPD1SECF"]
        #[inline(always)]
        pub const fn ucpd1secf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "UCPD1SECF"]
        #[inline(always)]
        pub fn set_ucpd1secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Apb1secsr2 {
        #[inline(always)]
        fn default() -> Apb1secsr2 {
            Apb1secsr2(0)
        }
    }
    #[doc = "APB1SMENR1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1smenr1(pub u32);
    impl Apb1smenr1 {
        #[doc = "TIM2 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn tim2smen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_tim2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn tim3smen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_tim3smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn tim4smen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_tim4smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn tim5smen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_tim5smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn tim6smen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_tim6smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn tim7smen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_tim7smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "RTC APB clock enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn rtcapbsmen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "RTC APB clock enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_rtcapbsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Window watchdog clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn wwdgsmen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_wwdgsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI2 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn spi2smen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_spi2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn sp3smen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_sp3smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "USART2 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn usart2smen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_usart2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn usart3smen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_usart3smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn uart4smen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_uart4smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn uart5smen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_uart5smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn i2c1smen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_i2c1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn i2c2smen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_i2c2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I2C3 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn i2c3smen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_i2c3smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "CRS clock enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn crssmen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "CRS clock enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_crssmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Power interface clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn pwrsmen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Power interface clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_pwrsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "DAC1 interface clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn dac1smen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1 interface clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_dac1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "OPAMP interface clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn opampsmen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP interface clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_opampsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Low power timer 1 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn lptim1smen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Low power timer 1 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_lptim1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Apb1smenr1 {
        #[inline(always)]
        fn default() -> Apb1smenr1 {
            Apb1smenr1(0)
        }
    }
    #[doc = "APB1 peripheral clocks enable in Sleep and Stop modes register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1smenr2(pub u32);
    impl Apb1smenr2 {
        #[doc = "Low power UART 1 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn lpuart1smen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Low power UART 1 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_lpuart1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "I2C4 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn i2c4smen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_i2c4smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPTIM2SMEN"]
        #[inline(always)]
        pub const fn lptim2smen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2SMEN"]
        #[inline(always)]
        pub fn set_lptim2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LPTIM3SMEN"]
        #[inline(always)]
        pub const fn lptim3smen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3SMEN"]
        #[inline(always)]
        pub fn set_lptim3smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "FDCAN1SMEN"]
        #[inline(always)]
        pub const fn fdcan1smen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN1SMEN"]
        #[inline(always)]
        pub fn set_fdcan1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "USBSMEN"]
        #[inline(always)]
        pub const fn usbsmen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "USBSMEN"]
        #[inline(always)]
        pub fn set_usbsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "UCPD1SMEN"]
        #[inline(always)]
        pub const fn ucpd1smen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "UCPD1SMEN"]
        #[inline(always)]
        pub fn set_ucpd1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Apb1smenr2 {
        #[inline(always)]
        fn default() -> Apb1smenr2 {
            Apb1smenr2(0)
        }
    }
    #[doc = "APB2ENR"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2enr(pub u32);
    impl Apb2enr {
        #[doc = "SYSCFG clock enable"]
        #[inline(always)]
        pub const fn syscfgen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG clock enable"]
        #[inline(always)]
        pub fn set_syscfgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM1 timer clock enable"]
        #[inline(always)]
        pub const fn tim1en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 timer clock enable"]
        #[inline(always)]
        pub fn set_tim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI1 clock enable"]
        #[inline(always)]
        pub const fn spi1en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 clock enable"]
        #[inline(always)]
        pub fn set_spi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TIM8 timer clock enable"]
        #[inline(always)]
        pub const fn tim8en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 timer clock enable"]
        #[inline(always)]
        pub fn set_tim8en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "USART1clock enable"]
        #[inline(always)]
        pub const fn usart1en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1clock enable"]
        #[inline(always)]
        pub fn set_usart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TIM15 timer clock enable"]
        #[inline(always)]
        pub const fn tim15en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 timer clock enable"]
        #[inline(always)]
        pub fn set_tim15en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 timer clock enable"]
        #[inline(always)]
        pub const fn tim16en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 timer clock enable"]
        #[inline(always)]
        pub fn set_tim16en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 timer clock enable"]
        #[inline(always)]
        pub const fn tim17en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 timer clock enable"]
        #[inline(always)]
        pub fn set_tim17en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SAI1 clock enable"]
        #[inline(always)]
        pub const fn sai1en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 clock enable"]
        #[inline(always)]
        pub fn set_sai1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SAI2 clock enable"]
        #[inline(always)]
        pub const fn sai2en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SAI2 clock enable"]
        #[inline(always)]
        pub fn set_sai2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "DFSDM timer clock enable"]
        #[inline(always)]
        pub const fn dfsdm1en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "DFSDM timer clock enable"]
        #[inline(always)]
        pub fn set_dfsdm1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Apb2enr {
        #[inline(always)]
        fn default() -> Apb2enr {
            Apb2enr(0)
        }
    }
    #[doc = "APB2 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2rstr(pub u32);
    impl Apb2rstr {
        #[doc = "System configuration (SYSCFG) reset"]
        #[inline(always)]
        pub const fn syscfgrst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "System configuration (SYSCFG) reset"]
        #[inline(always)]
        pub fn set_syscfgrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM1 timer reset"]
        #[inline(always)]
        pub const fn tim1rst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 timer reset"]
        #[inline(always)]
        pub fn set_tim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI1 reset"]
        #[inline(always)]
        pub const fn spi1rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 reset"]
        #[inline(always)]
        pub fn set_spi1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TIM8 timer reset"]
        #[inline(always)]
        pub const fn tim8rst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 timer reset"]
        #[inline(always)]
        pub fn set_tim8rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "USART1 reset"]
        #[inline(always)]
        pub const fn usart1rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 reset"]
        #[inline(always)]
        pub fn set_usart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TIM15 timer reset"]
        #[inline(always)]
        pub const fn tim15rst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 timer reset"]
        #[inline(always)]
        pub fn set_tim15rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 timer reset"]
        #[inline(always)]
        pub const fn tim16rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 timer reset"]
        #[inline(always)]
        pub fn set_tim16rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 timer reset"]
        #[inline(always)]
        pub const fn tim17rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 timer reset"]
        #[inline(always)]
        pub fn set_tim17rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Serial audio interface 1 (SAI1) reset"]
        #[inline(always)]
        pub const fn sai1rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Serial audio interface 1 (SAI1) reset"]
        #[inline(always)]
        pub fn set_sai1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Serial audio interface 2 (SAI2) reset"]
        #[inline(always)]
        pub const fn sai2rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Serial audio interface 2 (SAI2) reset"]
        #[inline(always)]
        pub fn set_sai2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Digital filters for sigma-delata modulators (DFSDM) reset"]
        #[inline(always)]
        pub const fn dfsdm1rst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Digital filters for sigma-delata modulators (DFSDM) reset"]
        #[inline(always)]
        pub fn set_dfsdm1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Apb2rstr {
        #[inline(always)]
        fn default() -> Apb2rstr {
            Apb2rstr(0)
        }
    }
    #[doc = "RCC APB2 security status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2secsr(pub u32);
    impl Apb2secsr {
        #[doc = "SYSCFGSECF"]
        #[inline(always)]
        pub const fn syscfgsecf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFGSECF"]
        #[inline(always)]
        pub fn set_syscfgsecf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM1SECF"]
        #[inline(always)]
        pub const fn tim1secf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1SECF"]
        #[inline(always)]
        pub fn set_tim1secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI1SECF"]
        #[inline(always)]
        pub const fn spi1secf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1SECF"]
        #[inline(always)]
        pub fn set_spi1secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TIM8SECF"]
        #[inline(always)]
        pub const fn tim8secf(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8SECF"]
        #[inline(always)]
        pub fn set_tim8secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "USART1SECF"]
        #[inline(always)]
        pub const fn usart1secf(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1SECF"]
        #[inline(always)]
        pub fn set_usart1secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TIM15SECF"]
        #[inline(always)]
        pub const fn tim15secf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15SECF"]
        #[inline(always)]
        pub fn set_tim15secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16SECF"]
        #[inline(always)]
        pub const fn tim16secf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16SECF"]
        #[inline(always)]
        pub fn set_tim16secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17SECF"]
        #[inline(always)]
        pub const fn tim17secf(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17SECF"]
        #[inline(always)]
        pub fn set_tim17secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SAI1SECF"]
        #[inline(always)]
        pub const fn sai1secf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1SECF"]
        #[inline(always)]
        pub fn set_sai1secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SAI2SECF"]
        #[inline(always)]
        pub const fn sai2secf(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SAI2SECF"]
        #[inline(always)]
        pub fn set_sai2secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "DFSDM1SECF"]
        #[inline(always)]
        pub const fn dfsdm1secf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "DFSDM1SECF"]
        #[inline(always)]
        pub fn set_dfsdm1secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Apb2secsr {
        #[inline(always)]
        fn default() -> Apb2secsr {
            Apb2secsr(0)
        }
    }
    #[doc = "APB2SMENR"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2smenr(pub u32);
    impl Apb2smenr {
        #[doc = "SYSCFG clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn syscfgsmen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_syscfgsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM1 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn tim1smen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_tim1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI1 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn spi1smen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_spi1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TIM8 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn tim8smen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_tim8smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "USART1clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn usart1smen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_usart1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TIM15 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn tim15smen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_tim15smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn tim16smen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_tim16smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn tim17smen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_tim17smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SAI1 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn sai1smen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_sai1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SAI2 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn sai2smen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SAI2 clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_sai2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "DFSDM timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub const fn dfsdm1smen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "DFSDM timer clocks enable during Sleep and Stop modes"]
        #[inline(always)]
        pub fn set_dfsdm1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Apb2smenr {
        #[inline(always)]
        fn default() -> Apb2smenr {
            Apb2smenr(0)
        }
    }
    #[doc = "BDCR"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdcr(pub u32);
    impl Bdcr {
        #[doc = "LSE oscillator enable"]
        #[inline(always)]
        pub const fn lseon(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSE oscillator enable"]
        #[inline(always)]
        pub fn set_lseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE oscillator ready"]
        #[inline(always)]
        pub const fn lserdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE oscillator ready"]
        #[inline(always)]
        pub fn set_lserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LSE oscillator bypass"]
        #[inline(always)]
        pub const fn lsebyp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "LSE oscillator bypass"]
        #[inline(always)]
        pub fn set_lsebyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "SE oscillator drive capability"]
        #[inline(always)]
        pub const fn lsedrv(&self) -> super::vals::Lsedrv {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Lsedrv::from_bits(val as u8)
        }
        #[doc = "SE oscillator drive capability"]
        #[inline(always)]
        pub fn set_lsedrv(&mut self, val: super::vals::Lsedrv) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "LSECSSON"]
        #[inline(always)]
        pub const fn lsecsson(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LSECSSON"]
        #[inline(always)]
        pub fn set_lsecsson(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LSECSSD"]
        #[inline(always)]
        pub const fn lsecssd(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LSECSSD"]
        #[inline(always)]
        pub fn set_lsecssd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LSESYSEN"]
        #[inline(always)]
        pub const fn lsesysen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LSESYSEN"]
        #[inline(always)]
        pub fn set_lsesysen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "RTC clock source selection"]
        #[inline(always)]
        pub const fn rtcsel(&self) -> super::vals::Rtcsel {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Rtcsel::from_bits(val as u8)
        }
        #[doc = "RTC clock source selection"]
        #[inline(always)]
        pub fn set_rtcsel(&mut self, val: super::vals::Rtcsel) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "LSESYSRDY"]
        #[inline(always)]
        pub const fn lsesysrdy(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LSESYSRDY"]
        #[inline(always)]
        pub fn set_lsesysrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "RTC clock enable"]
        #[inline(always)]
        pub const fn rtcen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "RTC clock enable"]
        #[inline(always)]
        pub fn set_rtcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Backup domain software reset"]
        #[inline(always)]
        pub const fn bdrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Backup domain software reset"]
        #[inline(always)]
        pub fn set_bdrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Low speed clock output enable"]
        #[inline(always)]
        pub const fn lscoen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Low speed clock output enable"]
        #[inline(always)]
        pub fn set_lscoen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Low speed clock output selection"]
        #[inline(always)]
        pub const fn lscosel(&self) -> super::vals::Lscosel {
            let val = (self.0 >> 25usize) & 0x01;
            super::vals::Lscosel::from_bits(val as u8)
        }
        #[doc = "Low speed clock output selection"]
        #[inline(always)]
        pub fn set_lscosel(&mut self, val: super::vals::Lscosel) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Bdcr {
        #[inline(always)]
        fn default() -> Bdcr {
            Bdcr(0)
        }
    }
    #[doc = "CCIPR"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccipr(pub u32);
    impl Ccipr {
        #[doc = "USART1 clock source selection"]
        #[inline(always)]
        pub const fn usart1sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "USART1 clock source selection"]
        #[inline(always)]
        pub fn set_usart1sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "USART2 clock source selection"]
        #[inline(always)]
        pub const fn usart2sel(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "USART2 clock source selection"]
        #[inline(always)]
        pub fn set_usart2sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "USART3 clock source selection"]
        #[inline(always)]
        pub const fn usart3sel(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "USART3 clock source selection"]
        #[inline(always)]
        pub fn set_usart3sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "UART4 clock source selection"]
        #[inline(always)]
        pub const fn uart4sel(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "UART4 clock source selection"]
        #[inline(always)]
        pub fn set_uart4sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "UART5 clock source selection"]
        #[inline(always)]
        pub const fn uart5sel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "UART5 clock source selection"]
        #[inline(always)]
        pub fn set_uart5sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "LPUART1 clock source selection"]
        #[inline(always)]
        pub const fn lpuart1sel(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "LPUART1 clock source selection"]
        #[inline(always)]
        pub fn set_lpuart1sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "I2C1 clock source selection"]
        #[inline(always)]
        pub const fn i2c1sel(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "I2C1 clock source selection"]
        #[inline(always)]
        pub fn set_i2c1sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "I2C2 clock source selection"]
        #[inline(always)]
        pub const fn i2c2sel(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "I2C2 clock source selection"]
        #[inline(always)]
        pub fn set_i2c2sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "I2C3 clock source selection"]
        #[inline(always)]
        pub const fn i2c3sel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "I2C3 clock source selection"]
        #[inline(always)]
        pub fn set_i2c3sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "Low power timer 1 clock source selection"]
        #[inline(always)]
        pub const fn lptim1sel(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x03;
            val as u8
        }
        #[doc = "Low power timer 1 clock source selection"]
        #[inline(always)]
        pub fn set_lptim1sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
        }
        #[doc = "Low power timer 2 clock source selection"]
        #[inline(always)]
        pub const fn lptim2sel(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Low power timer 2 clock source selection"]
        #[inline(always)]
        pub fn set_lptim2sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
        #[doc = "Low-power timer 3 clock source selection"]
        #[inline(always)]
        pub const fn lptim3sel(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "Low-power timer 3 clock source selection"]
        #[inline(always)]
        pub fn set_lptim3sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "FDCAN clock source selection"]
        #[inline(always)]
        pub const fn fdcansel(&self) -> super::vals::Fdcansel {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Fdcansel::from_bits(val as u8)
        }
        #[doc = "FDCAN clock source selection"]
        #[inline(always)]
        pub fn set_fdcansel(&mut self, val: super::vals::Fdcansel) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "48 MHz clock source selection"]
        #[inline(always)]
        pub const fn clk48sel(&self) -> super::vals::Clk48sel {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::Clk48sel::from_bits(val as u8)
        }
        #[doc = "48 MHz clock source selection"]
        #[inline(always)]
        pub fn set_clk48sel(&mut self, val: super::vals::Clk48sel) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "ADCs clock source selection"]
        #[inline(always)]
        pub const fn adcsel(&self) -> super::vals::Adcsel {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Adcsel::from_bits(val as u8)
        }
        #[doc = "ADCs clock source selection"]
        #[inline(always)]
        pub fn set_adcsel(&mut self, val: super::vals::Adcsel) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Ccipr {
        #[inline(always)]
        fn default() -> Ccipr {
            Ccipr(0)
        }
    }
    #[doc = "Peripherals independent clock configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccipr2(pub u32);
    impl Ccipr2 {
        #[doc = "I2C4 clock source selection"]
        #[inline(always)]
        pub const fn i2c4sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "I2C4 clock source selection"]
        #[inline(always)]
        pub fn set_i2c4sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Digital filter for sigma delta modulator kernel clock source selection"]
        #[inline(always)]
        pub const fn dfsdmsel(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Digital filter for sigma delta modulator kernel clock source selection"]
        #[inline(always)]
        pub fn set_dfsdmsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Digital filter for sigma delta modulator audio clock source selection"]
        #[inline(always)]
        pub const fn adfsdmsel(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "Digital filter for sigma delta modulator audio clock source selection"]
        #[inline(always)]
        pub fn set_adfsdmsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "SAI1 clock source selection"]
        #[inline(always)]
        pub const fn sai1sel(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x07;
            val as u8
        }
        #[doc = "SAI1 clock source selection"]
        #[inline(always)]
        pub fn set_sai1sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
        }
        #[doc = "SAI2 clock source selection"]
        #[inline(always)]
        pub const fn sai2sel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "SAI2 clock source selection"]
        #[inline(always)]
        pub fn set_sai2sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "SDMMC clock selection"]
        #[inline(always)]
        pub const fn sdmmcsel(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC clock selection"]
        #[inline(always)]
        pub fn set_sdmmcsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Octospi clock source selection"]
        #[inline(always)]
        pub const fn octospisel(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x03;
            val as u8
        }
        #[doc = "Octospi clock source selection"]
        #[inline(always)]
        pub fn set_octospisel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
        }
    }
    impl Default for Ccipr2 {
        #[inline(always)]
        fn default() -> Ccipr2 {
            Ccipr2(0)
        }
    }
    #[doc = "Clock configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "System clock switch"]
        #[inline(always)]
        pub const fn sw(&self) -> super::vals::Sw {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "System clock switch"]
        #[inline(always)]
        pub fn set_sw(&mut self, val: super::vals::Sw) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "System clock switch status"]
        #[inline(always)]
        pub const fn sws(&self) -> super::vals::Sw {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "System clock switch status"]
        #[inline(always)]
        pub fn set_sws(&mut self, val: super::vals::Sw) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "AHB prescaler"]
        #[inline(always)]
        pub const fn hpre(&self) -> super::vals::Hpre {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::Hpre::from_bits(val as u8)
        }
        #[doc = "AHB prescaler"]
        #[inline(always)]
        pub fn set_hpre(&mut self, val: super::vals::Hpre) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
        #[doc = "APB low-speed prescaler (APB1)"]
        #[inline(always)]
        pub const fn ppre1(&self) -> super::vals::Ppre {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB low-speed prescaler (APB1)"]
        #[inline(always)]
        pub fn set_ppre1(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "APB high-speed prescaler (APB2)"]
        #[inline(always)]
        pub const fn ppre2(&self) -> super::vals::Ppre {
            let val = (self.0 >> 11usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB high-speed prescaler (APB2)"]
        #[inline(always)]
        pub fn set_ppre2(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 11usize)) | (((val.to_bits() as u32) & 0x07) << 11usize);
        }
        #[doc = "Wakeup from Stop and CSS backup clock selection"]
        #[inline(always)]
        pub const fn stopwuck(&self) -> super::vals::Stopwuck {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Stopwuck::from_bits(val as u8)
        }
        #[doc = "Wakeup from Stop and CSS backup clock selection"]
        #[inline(always)]
        pub fn set_stopwuck(&mut self, val: super::vals::Stopwuck) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "Microcontroller clock output selection"]
        #[inline(always)]
        pub const fn mcosel(&self) -> super::vals::Mcosel {
            let val = (self.0 >> 24usize) & 0x0f;
            super::vals::Mcosel::from_bits(val as u8)
        }
        #[doc = "Microcontroller clock output selection"]
        #[inline(always)]
        pub fn set_mcosel(&mut self, val: super::vals::Mcosel) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
        }
        #[doc = "Microcontroller clock output prescaler"]
        #[inline(always)]
        pub const fn mcopre(&self) -> super::vals::Mcopre {
            let val = (self.0 >> 28usize) & 0x07;
            super::vals::Mcopre::from_bits(val as u8)
        }
        #[doc = "Microcontroller clock output prescaler"]
        #[inline(always)]
        pub fn set_mcopre(&mut self, val: super::vals::Mcopre) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Cfgr {
        #[inline(always)]
        fn default() -> Cfgr {
            Cfgr(0)
        }
    }
    #[doc = "Clock interrupt clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cicr(pub u32);
    impl Cicr {
        #[doc = "LSI ready interrupt clear"]
        #[inline(always)]
        pub const fn lsirdyc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt clear"]
        #[inline(always)]
        pub fn set_lsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt clear"]
        #[inline(always)]
        pub const fn lserdyc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt clear"]
        #[inline(always)]
        pub fn set_lserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MSI ready interrupt clear"]
        #[inline(always)]
        pub const fn msirdyc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "MSI ready interrupt clear"]
        #[inline(always)]
        pub fn set_msirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSI ready interrupt clear"]
        #[inline(always)]
        pub const fn hsirdyc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt clear"]
        #[inline(always)]
        pub fn set_hsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HSE ready interrupt clear"]
        #[inline(always)]
        pub const fn hserdyc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt clear"]
        #[inline(always)]
        pub fn set_hserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "PLL ready interrupt clear"]
        #[inline(always)]
        pub const fn pllrdyc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "PLL ready interrupt clear"]
        #[inline(always)]
        pub fn set_pllrdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PLLSAI1 ready interrupt clear"]
        #[inline(always)]
        pub const fn pllsai1rdyc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PLLSAI1 ready interrupt clear"]
        #[inline(always)]
        pub fn set_pllsai1rdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "PLLSAI2 ready interrupt clear"]
        #[inline(always)]
        pub const fn pllsai2rdyc(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "PLLSAI2 ready interrupt clear"]
        #[inline(always)]
        pub fn set_pllsai2rdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Clock security system interrupt clear"]
        #[inline(always)]
        pub const fn cssc(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Clock security system interrupt clear"]
        #[inline(always)]
        pub fn set_cssc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LSE Clock security system interrupt clear"]
        #[inline(always)]
        pub const fn lsecssc(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LSE Clock security system interrupt clear"]
        #[inline(always)]
        pub fn set_lsecssc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "HSI48 oscillator ready interrupt clear"]
        #[inline(always)]
        pub const fn hsi48rdyc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 oscillator ready interrupt clear"]
        #[inline(always)]
        pub fn set_hsi48rdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Cicr {
        #[inline(always)]
        fn default() -> Cicr {
            Cicr(0)
        }
    }
    #[doc = "Clock interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cier(pub u32);
    impl Cier {
        #[doc = "LSI ready interrupt enable"]
        #[inline(always)]
        pub const fn lsirdyie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt enable"]
        #[inline(always)]
        pub fn set_lsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt enable"]
        #[inline(always)]
        pub const fn lserdyie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt enable"]
        #[inline(always)]
        pub fn set_lserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MSI ready interrupt enable"]
        #[inline(always)]
        pub const fn msirdyie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "MSI ready interrupt enable"]
        #[inline(always)]
        pub fn set_msirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSI ready interrupt enable"]
        #[inline(always)]
        pub const fn hsirdyie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt enable"]
        #[inline(always)]
        pub fn set_hsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HSE ready interrupt enable"]
        #[inline(always)]
        pub const fn hserdyie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt enable"]
        #[inline(always)]
        pub fn set_hserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "PLL ready interrupt enable"]
        #[inline(always)]
        pub const fn pllrdyie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "PLL ready interrupt enable"]
        #[inline(always)]
        pub fn set_pllrdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PLLSAI1 ready interrupt enable"]
        #[inline(always)]
        pub const fn pllsai1rdyie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PLLSAI1 ready interrupt enable"]
        #[inline(always)]
        pub fn set_pllsai1rdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "PLLSAI2 ready interrupt enable"]
        #[inline(always)]
        pub const fn pllsai2rdyie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "PLLSAI2 ready interrupt enable"]
        #[inline(always)]
        pub fn set_pllsai2rdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LSE clock security system interrupt enable"]
        #[inline(always)]
        pub const fn lsecssie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock security system interrupt enable"]
        #[inline(always)]
        pub fn set_lsecssie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "HSI48 ready interrupt enable"]
        #[inline(always)]
        pub const fn hsi48rdyie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 ready interrupt enable"]
        #[inline(always)]
        pub fn set_hsi48rdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Cier {
        #[inline(always)]
        fn default() -> Cier {
            Cier(0)
        }
    }
    #[doc = "Clock interrupt flag register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cifr(pub u32);
    impl Cifr {
        #[doc = "LSI ready interrupt flag"]
        #[inline(always)]
        pub const fn lsirdyf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt flag"]
        #[inline(always)]
        pub fn set_lsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt flag"]
        #[inline(always)]
        pub const fn lserdyf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt flag"]
        #[inline(always)]
        pub fn set_lserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MSI ready interrupt flag"]
        #[inline(always)]
        pub const fn msirdyf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "MSI ready interrupt flag"]
        #[inline(always)]
        pub fn set_msirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSI ready interrupt flag"]
        #[inline(always)]
        pub const fn hsirdyf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt flag"]
        #[inline(always)]
        pub fn set_hsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HSE ready interrupt flag"]
        #[inline(always)]
        pub const fn hserdyf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt flag"]
        #[inline(always)]
        pub fn set_hserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "PLL ready interrupt flag"]
        #[inline(always)]
        pub const fn pllrdyf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "PLL ready interrupt flag"]
        #[inline(always)]
        pub fn set_pllrdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PLLSAI1 ready interrupt flag"]
        #[inline(always)]
        pub const fn pllsai1rdyf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PLLSAI1 ready interrupt flag"]
        #[inline(always)]
        pub fn set_pllsai1rdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "PLLSAI2 ready interrupt flag"]
        #[inline(always)]
        pub const fn pllsai2rdyf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "PLLSAI2 ready interrupt flag"]
        #[inline(always)]
        pub fn set_pllsai2rdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Clock security system interrupt flag"]
        #[inline(always)]
        pub const fn cssf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Clock security system interrupt flag"]
        #[inline(always)]
        pub fn set_cssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LSE Clock security system interrupt flag"]
        #[inline(always)]
        pub const fn lsecssf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LSE Clock security system interrupt flag"]
        #[inline(always)]
        pub fn set_lsecssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "HSI48 ready interrupt flag"]
        #[inline(always)]
        pub const fn hsi48rdyf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 ready interrupt flag"]
        #[inline(always)]
        pub fn set_hsi48rdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Cifr {
        #[inline(always)]
        fn default() -> Cifr {
            Cifr(0)
        }
    }
    #[doc = "Clock control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "MSI clock enable"]
        #[inline(always)]
        pub const fn msion(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MSI clock enable"]
        #[inline(always)]
        pub fn set_msion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "MSI clock ready flag"]
        #[inline(always)]
        pub const fn msirdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "MSI clock ready flag"]
        #[inline(always)]
        pub fn set_msirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MSI clock PLL enable"]
        #[inline(always)]
        pub const fn msipllen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "MSI clock PLL enable"]
        #[inline(always)]
        pub fn set_msipllen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "MSI clock range selection"]
        #[inline(always)]
        pub const fn msirgsel(&self) -> super::vals::Msirgsel {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Msirgsel::from_bits(val as u8)
        }
        #[doc = "MSI clock range selection"]
        #[inline(always)]
        pub fn set_msirgsel(&mut self, val: super::vals::Msirgsel) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "MSI clock ranges"]
        #[inline(always)]
        pub const fn msirange(&self) -> super::vals::Msirange {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::Msirange::from_bits(val as u8)
        }
        #[doc = "MSI clock ranges"]
        #[inline(always)]
        pub fn set_msirange(&mut self, val: super::vals::Msirange) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
        #[doc = "HSI clock enable"]
        #[inline(always)]
        pub const fn hsion(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "HSI clock enable"]
        #[inline(always)]
        pub fn set_hsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "HSI always enable for peripheral kernels"]
        #[inline(always)]
        pub const fn hsikeron(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "HSI always enable for peripheral kernels"]
        #[inline(always)]
        pub fn set_hsikeron(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "HSI clock ready flag"]
        #[inline(always)]
        pub const fn hsirdy(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HSI clock ready flag"]
        #[inline(always)]
        pub fn set_hsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "HSI automatic start from Stop"]
        #[inline(always)]
        pub const fn hsiasfs(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "HSI automatic start from Stop"]
        #[inline(always)]
        pub fn set_hsiasfs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "HSE clock enable"]
        #[inline(always)]
        pub const fn hseon(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock enable"]
        #[inline(always)]
        pub fn set_hseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "HSE clock ready flag"]
        #[inline(always)]
        pub const fn hserdy(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock ready flag"]
        #[inline(always)]
        pub fn set_hserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "HSE crystal oscillator bypass"]
        #[inline(always)]
        pub const fn hsebyp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "HSE crystal oscillator bypass"]
        #[inline(always)]
        pub fn set_hsebyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Clock security system enable"]
        #[inline(always)]
        pub const fn csson(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Clock security system enable"]
        #[inline(always)]
        pub fn set_csson(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Main PLL enable"]
        #[inline(always)]
        pub const fn pllon(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Main PLL enable"]
        #[inline(always)]
        pub fn set_pllon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Main PLL clock ready flag"]
        #[inline(always)]
        pub const fn pllrdy(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Main PLL clock ready flag"]
        #[inline(always)]
        pub fn set_pllrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "SAI1 PLL enable"]
        #[inline(always)]
        pub const fn pllsai1on(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 PLL enable"]
        #[inline(always)]
        pub fn set_pllsai1on(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "SAI1 PLL clock ready flag"]
        #[inline(always)]
        pub const fn pllsai1rdy(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 PLL clock ready flag"]
        #[inline(always)]
        pub fn set_pllsai1rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "SAI2 PLL enable"]
        #[inline(always)]
        pub const fn pllsai2on(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "SAI2 PLL enable"]
        #[inline(always)]
        pub fn set_pllsai2on(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "SAI2 PLL clock ready flag"]
        #[inline(always)]
        pub const fn pllsai2rdy(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "SAI2 PLL clock ready flag"]
        #[inline(always)]
        pub fn set_pllsai2rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "PRIV"]
        #[inline(always)]
        pub const fn priv_(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "PRIV"]
        #[inline(always)]
        pub fn set_priv_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    #[doc = "Clock recovery RC register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crrcr(pub u32);
    impl Crrcr {
        #[doc = "HSI48 clock enable"]
        #[inline(always)]
        pub const fn hsi48on(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 clock enable"]
        #[inline(always)]
        pub fn set_hsi48on(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "HSI48 clock ready flag"]
        #[inline(always)]
        pub const fn hsi48rdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 clock ready flag"]
        #[inline(always)]
        pub fn set_hsi48rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI48 clock calibration"]
        #[inline(always)]
        pub const fn hsi48cal(&self) -> u16 {
            let val = (self.0 >> 7usize) & 0x01ff;
            val as u16
        }
        #[doc = "HSI48 clock calibration"]
        #[inline(always)]
        pub fn set_hsi48cal(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 7usize)) | (((val as u32) & 0x01ff) << 7usize);
        }
    }
    impl Default for Crrcr {
        #[inline(always)]
        fn default() -> Crrcr {
            Crrcr(0)
        }
    }
    #[doc = "CSR"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "LSI oscillator enable"]
        #[inline(always)]
        pub const fn lsion(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI oscillator enable"]
        #[inline(always)]
        pub fn set_lsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSI oscillator ready"]
        #[inline(always)]
        pub const fn lsirdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSI oscillator ready"]
        #[inline(always)]
        pub fn set_lsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LSIPREDIV"]
        #[inline(always)]
        pub const fn lsiprediv(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "LSIPREDIV"]
        #[inline(always)]
        pub fn set_lsiprediv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SI range after Standby mode"]
        #[inline(always)]
        pub const fn msisrange(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "SI range after Standby mode"]
        #[inline(always)]
        pub fn set_msisrange(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Remove reset flag"]
        #[inline(always)]
        pub const fn rmvf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Remove reset flag"]
        #[inline(always)]
        pub fn set_rmvf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Option byte loader reset flag"]
        #[inline(always)]
        pub const fn oblrstf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte loader reset flag"]
        #[inline(always)]
        pub fn set_oblrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Pin reset flag"]
        #[inline(always)]
        pub const fn pinrstf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Pin reset flag"]
        #[inline(always)]
        pub fn set_pinrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "BOR flag"]
        #[inline(always)]
        pub const fn borrstf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "BOR flag"]
        #[inline(always)]
        pub fn set_borrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Software reset flag"]
        #[inline(always)]
        pub const fn sftrstf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Software reset flag"]
        #[inline(always)]
        pub fn set_sftrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Independent window watchdog reset flag"]
        #[inline(always)]
        pub const fn iwdgrstf(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Independent window watchdog reset flag"]
        #[inline(always)]
        pub fn set_iwdgrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Window watchdog reset flag"]
        #[inline(always)]
        pub const fn wwdgrstf(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog reset flag"]
        #[inline(always)]
        pub fn set_wwdgrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Low-power reset flag"]
        #[inline(always)]
        pub const fn lpwrrstf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power reset flag"]
        #[inline(always)]
        pub fn set_lpwrrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Csr {
        #[inline(always)]
        fn default() -> Csr {
            Csr(0)
        }
    }
    #[doc = "Internal clock sources calibration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icscr(pub u32);
    impl Icscr {
        #[doc = "MSI clock calibration"]
        #[inline(always)]
        pub const fn msical(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "MSI clock calibration"]
        #[inline(always)]
        pub fn set_msical(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "MSI clock trimming"]
        #[inline(always)]
        pub const fn msitrim(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "MSI clock trimming"]
        #[inline(always)]
        pub fn set_msitrim(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "HSI clock calibration"]
        #[inline(always)]
        pub const fn hsical(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "HSI clock calibration"]
        #[inline(always)]
        pub fn set_hsical(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "HSI clock trimming"]
        #[inline(always)]
        pub const fn hsitrim(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x7f;
            val as u8
        }
        #[doc = "HSI clock trimming"]
        #[inline(always)]
        pub fn set_hsitrim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val as u32) & 0x7f) << 24usize);
        }
    }
    impl Default for Icscr {
        #[inline(always)]
        fn default() -> Icscr {
            Icscr(0)
        }
    }
    #[doc = "PLL configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pllcfgr(pub u32);
    impl Pllcfgr {
        #[doc = "PLL clock source"]
        #[inline(always)]
        pub const fn pllsrc(&self) -> super::vals::Pllsrc {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Pllsrc::from_bits(val as u8)
        }
        #[doc = "PLL clock source"]
        #[inline(always)]
        pub fn set_pllsrc(&mut self, val: super::vals::Pllsrc) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Division factor for the PLL input clock"]
        #[inline(always)]
        pub const fn pllm(&self) -> super::vals::Pllm {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::Pllm::from_bits(val as u8)
        }
        #[doc = "Division factor for the PLL input clock"]
        #[inline(always)]
        pub fn set_pllm(&mut self, val: super::vals::Pllm) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
        #[doc = "PLL multiplication factor for VCO"]
        #[inline(always)]
        pub const fn plln(&self) -> super::vals::Plln {
            let val = (self.0 >> 8usize) & 0x7f;
            super::vals::Plln::from_bits(val as u8)
        }
        #[doc = "PLL multiplication factor for VCO"]
        #[inline(always)]
        pub fn set_plln(&mut self, val: super::vals::Plln) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val.to_bits() as u32) & 0x7f) << 8usize);
        }
        #[doc = "PLL PLLSAI3CLK output enable"]
        #[inline(always)]
        pub const fn pllpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PLL PLLSAI3CLK output enable"]
        #[inline(always)]
        pub fn set_pllpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)"]
        #[inline(always)]
        pub const fn pllpbit(&self) -> super::vals::Pllpbit {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Pllpbit::from_bits(val as u8)
        }
        #[doc = "PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)"]
        #[inline(always)]
        pub fn set_pllpbit(&mut self, val: super::vals::Pllpbit) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "PLL PLLUSB1CLK output enable"]
        #[inline(always)]
        pub const fn pllqen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "PLL PLLUSB1CLK output enable"]
        #[inline(always)]
        pub fn set_pllqen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "PLL division factor for PLLUSB1CLK(48 MHz clock)"]
        #[inline(always)]
        pub const fn pllq(&self) -> super::vals::Pllq {
            let val = (self.0 >> 21usize) & 0x03;
            super::vals::Pllq::from_bits(val as u8)
        }
        #[doc = "PLL division factor for PLLUSB1CLK(48 MHz clock)"]
        #[inline(always)]
        pub fn set_pllq(&mut self, val: super::vals::Pllq) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
        }
        #[doc = "PLL PLLCLK output enable"]
        #[inline(always)]
        pub const fn pllren(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "PLL PLLCLK output enable"]
        #[inline(always)]
        pub fn set_pllren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "PLL division factor for PLLCLK (system clock)"]
        #[inline(always)]
        pub const fn pllr(&self) -> super::vals::Pllr {
            let val = (self.0 >> 25usize) & 0x03;
            super::vals::Pllr::from_bits(val as u8)
        }
        #[doc = "PLL division factor for PLLCLK (system clock)"]
        #[inline(always)]
        pub fn set_pllr(&mut self, val: super::vals::Pllr) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
        }
        #[doc = "PLL division factor for PLLSAI2CLK"]
        #[inline(always)]
        pub const fn pllp(&self) -> super::vals::Pllp {
            let val = (self.0 >> 27usize) & 0x1f;
            super::vals::Pllp::from_bits(val as u8)
        }
        #[doc = "PLL division factor for PLLSAI2CLK"]
        #[inline(always)]
        pub fn set_pllp(&mut self, val: super::vals::Pllp) {
            self.0 = (self.0 & !(0x1f << 27usize)) | (((val.to_bits() as u32) & 0x1f) << 27usize);
        }
    }
    impl Default for Pllcfgr {
        #[inline(always)]
        fn default() -> Pllcfgr {
            Pllcfgr(0)
        }
    }
    #[doc = "RCC secure configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccfgr(pub u32);
    impl Seccfgr {
        #[doc = "HSISEC"]
        #[inline(always)]
        pub const fn hsisec(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "HSISEC"]
        #[inline(always)]
        pub fn set_hsisec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "HSESEC"]
        #[inline(always)]
        pub const fn hsesec(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "HSESEC"]
        #[inline(always)]
        pub fn set_hsesec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MSISEC"]
        #[inline(always)]
        pub const fn msisec(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "MSISEC"]
        #[inline(always)]
        pub fn set_msisec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "LSISEC"]
        #[inline(always)]
        pub const fn lsisec(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LSISEC"]
        #[inline(always)]
        pub fn set_lsisec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "LSESEC"]
        #[inline(always)]
        pub const fn lsesec(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "LSESEC"]
        #[inline(always)]
        pub fn set_lsesec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SYSCLKSEC"]
        #[inline(always)]
        pub const fn sysclksec(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCLKSEC"]
        #[inline(always)]
        pub fn set_sysclksec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PRESCSEC"]
        #[inline(always)]
        pub const fn prescsec(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PRESCSEC"]
        #[inline(always)]
        pub fn set_prescsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "PLLSEC"]
        #[inline(always)]
        pub const fn pllsec(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "PLLSEC"]
        #[inline(always)]
        pub fn set_pllsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "PLLSAI1SEC"]
        #[inline(always)]
        pub const fn pllsai1sec(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "PLLSAI1SEC"]
        #[inline(always)]
        pub fn set_pllsai1sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "PLLSAI2SEC"]
        #[inline(always)]
        pub const fn pllsai2sec(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "PLLSAI2SEC"]
        #[inline(always)]
        pub fn set_pllsai2sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "CLK48SEC"]
        #[inline(always)]
        pub const fn clk48sec(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "CLK48SEC"]
        #[inline(always)]
        pub fn set_clk48sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "HSI48SEC"]
        #[inline(always)]
        pub const fn hsi48sec(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48SEC"]
        #[inline(always)]
        pub fn set_hsi48sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "RMVFSEC"]
        #[inline(always)]
        pub const fn rmvfsec(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "RMVFSEC"]
        #[inline(always)]
        pub fn set_rmvfsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Seccfgr {
        #[inline(always)]
        fn default() -> Seccfgr {
            Seccfgr(0)
        }
    }
    #[doc = "RCC secure status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secsr(pub u32);
    impl Secsr {
        #[doc = "HSISECF"]
        #[inline(always)]
        pub const fn hsisecf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "HSISECF"]
        #[inline(always)]
        pub fn set_hsisecf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "HSESECF"]
        #[inline(always)]
        pub const fn hsesecf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "HSESECF"]
        #[inline(always)]
        pub fn set_hsesecf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MSISECF"]
        #[inline(always)]
        pub const fn msisecf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "MSISECF"]
        #[inline(always)]
        pub fn set_msisecf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "LSISECF"]
        #[inline(always)]
        pub const fn lsisecf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LSISECF"]
        #[inline(always)]
        pub fn set_lsisecf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "LSESECF"]
        #[inline(always)]
        pub const fn lsesecf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "LSESECF"]
        #[inline(always)]
        pub fn set_lsesecf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SYSCLKSECF"]
        #[inline(always)]
        pub const fn sysclksecf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCLKSECF"]
        #[inline(always)]
        pub fn set_sysclksecf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PRESCSECF"]
        #[inline(always)]
        pub const fn prescsecf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PRESCSECF"]
        #[inline(always)]
        pub fn set_prescsecf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "PLLSECF"]
        #[inline(always)]
        pub const fn pllsecf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "PLLSECF"]
        #[inline(always)]
        pub fn set_pllsecf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "PLLSAI1SECF"]
        #[inline(always)]
        pub const fn pllsai1secf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "PLLSAI1SECF"]
        #[inline(always)]
        pub fn set_pllsai1secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "PLLSAI2SECF"]
        #[inline(always)]
        pub const fn pllsai2secf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "PLLSAI2SECF"]
        #[inline(always)]
        pub fn set_pllsai2secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "CLK48SECF"]
        #[inline(always)]
        pub const fn clk48secf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "CLK48SECF"]
        #[inline(always)]
        pub fn set_clk48secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "HSI48SECF"]
        #[inline(always)]
        pub const fn hsi48secf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48SECF"]
        #[inline(always)]
        pub fn set_hsi48secf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "RMVFSECF"]
        #[inline(always)]
        pub const fn rmvfsecf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "RMVFSECF"]
        #[inline(always)]
        pub fn set_rmvfsecf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Secsr {
        #[inline(always)]
        fn default() -> Secsr {
            Secsr(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Adcsel {
        #[doc = "No clock selected"]
        DISABLE = 0x0,
        #[doc = "PLLADC1CLK clock selected"]
        PLLSAI1_R = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "SYSCLK clock selected"]
        SYS = 0x03,
    }
    impl Adcsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adcsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adcsel {
        #[inline(always)]
        fn from(val: u8) -> Adcsel {
            Adcsel::from_bits(val)
        }
    }
    impl From<Adcsel> for u8 {
        #[inline(always)]
        fn from(val: Adcsel) -> u8 {
            Adcsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Clk48sel {
        #[doc = "HSI48 clock selected"]
        HSI48 = 0x0,
        #[doc = "PLLSAI1_Q aka PLL48M1CLK clock selected"]
        PLLSAI1_Q = 0x01,
        #[doc = "PLL_Q aka PLL48M2CLK clock selected"]
        PLL1_Q = 0x02,
        #[doc = "MSI clock selected"]
        MSI = 0x03,
    }
    impl Clk48sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Clk48sel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Clk48sel {
        #[inline(always)]
        fn from(val: u8) -> Clk48sel {
            Clk48sel::from_bits(val)
        }
    }
    impl From<Clk48sel> for u8 {
        #[inline(always)]
        fn from(val: Clk48sel) -> u8 {
            Clk48sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Fdcansel {
        #[doc = "HSE clock selected"]
        HSE = 0x0,
        #[doc = "PLL \"Q\" clock selected"]
        PLL1_Q = 0x01,
        #[doc = "PLLSAI \"P\" clock selected"]
        PLLSAI1_P = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Fdcansel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fdcansel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fdcansel {
        #[inline(always)]
        fn from(val: u8) -> Fdcansel {
            Fdcansel::from_bits(val)
        }
    }
    impl From<Fdcansel> for u8 {
        #[inline(always)]
        fn from(val: Fdcansel) -> u8 {
            Fdcansel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Hpre {
        #[doc = "SYSCLK not divided"]
        DIV1 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "SYSCLK divided by 2"]
        DIV2 = 0x08,
        #[doc = "SYSCLK divided by 4"]
        DIV4 = 0x09,
        #[doc = "SYSCLK divided by 8"]
        DIV8 = 0x0a,
        #[doc = "SYSCLK divided by 16"]
        DIV16 = 0x0b,
        #[doc = "SYSCLK divided by 64"]
        DIV64 = 0x0c,
        #[doc = "SYSCLK divided by 128"]
        DIV128 = 0x0d,
        #[doc = "SYSCLK divided by 256"]
        DIV256 = 0x0e,
        #[doc = "SYSCLK divided by 512"]
        DIV512 = 0x0f,
    }
    impl Hpre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hpre {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hpre {
        #[inline(always)]
        fn from(val: u8) -> Hpre {
            Hpre::from_bits(val)
        }
    }
    impl From<Hpre> for u8 {
        #[inline(always)]
        fn from(val: Hpre) -> u8 {
            Hpre::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Lscosel {
        #[doc = "LSI clock selected\""]
        LSI = 0x0,
        #[doc = "LSE clock selected"]
        LSE = 0x01,
    }
    impl Lscosel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lscosel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lscosel {
        #[inline(always)]
        fn from(val: u8) -> Lscosel {
            Lscosel::from_bits(val)
        }
    }
    impl From<Lscosel> for u8 {
        #[inline(always)]
        fn from(val: Lscosel) -> u8 {
            Lscosel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Lsedrv {
        #[doc = "Low driving capability"]
        LOW = 0x0,
        #[doc = "Medium low driving capability"]
        MEDIUMLOW = 0x01,
        #[doc = "Medium high driving capability"]
        MEDIUMHIGH = 0x02,
        #[doc = "High driving capability"]
        HIGH = 0x03,
    }
    impl Lsedrv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lsedrv {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lsedrv {
        #[inline(always)]
        fn from(val: u8) -> Lsedrv {
            Lsedrv::from_bits(val)
        }
    }
    impl From<Lsedrv> for u8 {
        #[inline(always)]
        fn from(val: Lsedrv) -> u8 {
            Lsedrv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Mcopre {
        #[doc = "MCO divided by 1"]
        DIV1 = 0x0,
        #[doc = "MCO divided by 2"]
        DIV2 = 0x01,
        #[doc = "MCO divided by 4"]
        DIV4 = 0x02,
        #[doc = "MCO divided by 8"]
        DIV8 = 0x03,
        #[doc = "MCO divided by 16"]
        DIV16 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Mcopre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mcopre {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mcopre {
        #[inline(always)]
        fn from(val: u8) -> Mcopre {
            Mcopre::from_bits(val)
        }
    }
    impl From<Mcopre> for u8 {
        #[inline(always)]
        fn from(val: Mcopre) -> u8 {
            Mcopre::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Mcosel {
        #[doc = "MCO output disabled, no clock on MCO"]
        DISABLE = 0x0,
        #[doc = "SYSCLK system clock selected"]
        SYS = 0x01,
        #[doc = "MSI clock selected"]
        MSI = 0x02,
        #[doc = "HSI clock selected"]
        HSI = 0x03,
        #[doc = "HSE clock selected"]
        HSE = 0x04,
        #[doc = "Main PLL clock selected"]
        PLL = 0x05,
        #[doc = "LSI clock selected"]
        LSI = 0x06,
        #[doc = "LSE clock selected"]
        LSE = 0x07,
        #[doc = "Internal HSI48 clock selected"]
        HSI48 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Mcosel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mcosel {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mcosel {
        #[inline(always)]
        fn from(val: u8) -> Mcosel {
            Mcosel::from_bits(val)
        }
    }
    impl From<Mcosel> for u8 {
        #[inline(always)]
        fn from(val: Mcosel) -> u8 {
            Mcosel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Msirange {
        #[doc = "range 0 around 100 kHz"]
        RANGE100K = 0x0,
        #[doc = "range 1 around 200 kHz"]
        RANGE200K = 0x01,
        #[doc = "range 2 around 400 kHz"]
        RANGE400K = 0x02,
        #[doc = "range 3 around 800 kHz"]
        RANGE800K = 0x03,
        #[doc = "range 4 around 1 MHz"]
        RANGE1M = 0x04,
        #[doc = "range 5 around 2 MHz"]
        RANGE2M = 0x05,
        #[doc = "range 6 around 4 MHz"]
        RANGE4M = 0x06,
        #[doc = "range 7 around 8 MHz"]
        RANGE8M = 0x07,
        #[doc = "range 8 around 16 MHz"]
        RANGE16M = 0x08,
        #[doc = "range 9 around 24 MHz"]
        RANGE24M = 0x09,
        #[doc = "range 10 around 32 MHz"]
        RANGE32M = 0x0a,
        #[doc = "range 11 around 48 MHz"]
        RANGE48M = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Msirange {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Msirange {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Msirange {
        #[inline(always)]
        fn from(val: u8) -> Msirange {
            Msirange::from_bits(val)
        }
    }
    impl From<Msirange> for u8 {
        #[inline(always)]
        fn from(val: Msirange) -> u8 {
            Msirange::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Msirgsel {
        #[doc = "MSI Range is provided by MSISRANGE\\[3:0\\]
in RCC_CSR register"]
        CSR = 0x0,
        #[doc = "MSI Range is provided by MSIRANGE\\[3:0\\]
in the RCC_CR register"]
        CR = 0x01,
    }
    impl Msirgsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Msirgsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Msirgsel {
        #[inline(always)]
        fn from(val: u8) -> Msirgsel {
            Msirgsel::from_bits(val)
        }
    }
    impl From<Msirgsel> for u8 {
        #[inline(always)]
        fn from(val: Msirgsel) -> u8 {
            Msirgsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Pllm {
        DIV1 = 0x0,
        DIV2 = 0x01,
        DIV3 = 0x02,
        DIV4 = 0x03,
        DIV5 = 0x04,
        DIV6 = 0x05,
        DIV7 = 0x06,
        DIV8 = 0x07,
        DIV9 = 0x08,
        DIV10 = 0x09,
        DIV11 = 0x0a,
        DIV12 = 0x0b,
        DIV13 = 0x0c,
        DIV14 = 0x0d,
        DIV15 = 0x0e,
        DIV16 = 0x0f,
    }
    impl Pllm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllm {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllm {
        #[inline(always)]
        fn from(val: u8) -> Pllm {
            Pllm::from_bits(val)
        }
    }
    impl From<Pllm> for u8 {
        #[inline(always)]
        fn from(val: Pllm) -> u8 {
            Pllm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Plln {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        MUL8 = 0x08,
        MUL9 = 0x09,
        MUL10 = 0x0a,
        MUL11 = 0x0b,
        MUL12 = 0x0c,
        MUL13 = 0x0d,
        MUL14 = 0x0e,
        MUL15 = 0x0f,
        MUL16 = 0x10,
        MUL17 = 0x11,
        MUL18 = 0x12,
        MUL19 = 0x13,
        MUL20 = 0x14,
        MUL21 = 0x15,
        MUL22 = 0x16,
        MUL23 = 0x17,
        MUL24 = 0x18,
        MUL25 = 0x19,
        MUL26 = 0x1a,
        MUL27 = 0x1b,
        MUL28 = 0x1c,
        MUL29 = 0x1d,
        MUL30 = 0x1e,
        MUL31 = 0x1f,
        MUL32 = 0x20,
        MUL33 = 0x21,
        MUL34 = 0x22,
        MUL35 = 0x23,
        MUL36 = 0x24,
        MUL37 = 0x25,
        MUL38 = 0x26,
        MUL39 = 0x27,
        MUL40 = 0x28,
        MUL41 = 0x29,
        MUL42 = 0x2a,
        MUL43 = 0x2b,
        MUL44 = 0x2c,
        MUL45 = 0x2d,
        MUL46 = 0x2e,
        MUL47 = 0x2f,
        MUL48 = 0x30,
        MUL49 = 0x31,
        MUL50 = 0x32,
        MUL51 = 0x33,
        MUL52 = 0x34,
        MUL53 = 0x35,
        MUL54 = 0x36,
        MUL55 = 0x37,
        MUL56 = 0x38,
        MUL57 = 0x39,
        MUL58 = 0x3a,
        MUL59 = 0x3b,
        MUL60 = 0x3c,
        MUL61 = 0x3d,
        MUL62 = 0x3e,
        MUL63 = 0x3f,
        MUL64 = 0x40,
        MUL65 = 0x41,
        MUL66 = 0x42,
        MUL67 = 0x43,
        MUL68 = 0x44,
        MUL69 = 0x45,
        MUL70 = 0x46,
        MUL71 = 0x47,
        MUL72 = 0x48,
        MUL73 = 0x49,
        MUL74 = 0x4a,
        MUL75 = 0x4b,
        MUL76 = 0x4c,
        MUL77 = 0x4d,
        MUL78 = 0x4e,
        MUL79 = 0x4f,
        MUL80 = 0x50,
        MUL81 = 0x51,
        MUL82 = 0x52,
        MUL83 = 0x53,
        MUL84 = 0x54,
        MUL85 = 0x55,
        MUL86 = 0x56,
        MUL87 = 0x57,
        MUL88 = 0x58,
        MUL89 = 0x59,
        MUL90 = 0x5a,
        MUL91 = 0x5b,
        MUL92 = 0x5c,
        MUL93 = 0x5d,
        MUL94 = 0x5e,
        MUL95 = 0x5f,
        MUL96 = 0x60,
        MUL97 = 0x61,
        MUL98 = 0x62,
        MUL99 = 0x63,
        MUL100 = 0x64,
        MUL101 = 0x65,
        MUL102 = 0x66,
        MUL103 = 0x67,
        MUL104 = 0x68,
        MUL105 = 0x69,
        MUL106 = 0x6a,
        MUL107 = 0x6b,
        MUL108 = 0x6c,
        MUL109 = 0x6d,
        MUL110 = 0x6e,
        MUL111 = 0x6f,
        MUL112 = 0x70,
        MUL113 = 0x71,
        MUL114 = 0x72,
        MUL115 = 0x73,
        MUL116 = 0x74,
        MUL117 = 0x75,
        MUL118 = 0x76,
        MUL119 = 0x77,
        MUL120 = 0x78,
        MUL121 = 0x79,
        MUL122 = 0x7a,
        MUL123 = 0x7b,
        MUL124 = 0x7c,
        MUL125 = 0x7d,
        MUL126 = 0x7e,
        MUL127 = 0x7f,
    }
    impl Plln {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Plln {
            unsafe { core::mem::transmute(val & 0x7f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Plln {
        #[inline(always)]
        fn from(val: u8) -> Plln {
            Plln::from_bits(val)
        }
    }
    impl From<Plln> for u8 {
        #[inline(always)]
        fn from(val: Plln) -> u8 {
            Plln::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Pllp {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        DIV2 = 0x02,
        DIV3 = 0x03,
        DIV4 = 0x04,
        DIV5 = 0x05,
        DIV6 = 0x06,
        DIV7 = 0x07,
        DIV8 = 0x08,
        DIV9 = 0x09,
        DIV10 = 0x0a,
        DIV11 = 0x0b,
        DIV12 = 0x0c,
        DIV13 = 0x0d,
        DIV14 = 0x0e,
        DIV15 = 0x0f,
        DIV16 = 0x10,
        DIV17 = 0x11,
        DIV18 = 0x12,
        DIV19 = 0x13,
        DIV20 = 0x14,
        DIV21 = 0x15,
        DIV22 = 0x16,
        DIV23 = 0x17,
        DIV24 = 0x18,
        DIV25 = 0x19,
        DIV26 = 0x1a,
        DIV27 = 0x1b,
        DIV28 = 0x1c,
        DIV29 = 0x1d,
        DIV30 = 0x1e,
        DIV31 = 0x1f,
    }
    impl Pllp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllp {
            unsafe { core::mem::transmute(val & 0x1f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllp {
        #[inline(always)]
        fn from(val: u8) -> Pllp {
            Pllp::from_bits(val)
        }
    }
    impl From<Pllp> for u8 {
        #[inline(always)]
        fn from(val: Pllp) -> u8 {
            Pllp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Pllpbit {
        DIV7 = 0x0,
        DIV17 = 0x01,
    }
    impl Pllpbit {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllpbit {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllpbit {
        #[inline(always)]
        fn from(val: u8) -> Pllpbit {
            Pllpbit::from_bits(val)
        }
    }
    impl From<Pllpbit> for u8 {
        #[inline(always)]
        fn from(val: Pllpbit) -> u8 {
            Pllpbit::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Pllq {
        DIV2 = 0x0,
        DIV4 = 0x01,
        DIV6 = 0x02,
        DIV8 = 0x03,
    }
    impl Pllq {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllq {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllq {
        #[inline(always)]
        fn from(val: u8) -> Pllq {
            Pllq::from_bits(val)
        }
    }
    impl From<Pllq> for u8 {
        #[inline(always)]
        fn from(val: Pllq) -> u8 {
            Pllq::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Pllr {
        DIV2 = 0x0,
        DIV4 = 0x01,
        DIV6 = 0x02,
        DIV8 = 0x03,
    }
    impl Pllr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllr {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllr {
        #[inline(always)]
        fn from(val: u8) -> Pllr {
            Pllr::from_bits(val)
        }
    }
    impl From<Pllr> for u8 {
        #[inline(always)]
        fn from(val: Pllr) -> u8 {
            Pllr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Pllsrc {
        #[doc = "No clock sent to PLL"]
        DISABLE = 0x0,
        #[doc = "MSI selected as PLL input clock"]
        MSI = 0x01,
        #[doc = "HSI selected as PLL input clock"]
        HSI = 0x02,
        #[doc = "HSE selected as PLL input clock"]
        HSE = 0x03,
    }
    impl Pllsrc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllsrc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllsrc {
        #[inline(always)]
        fn from(val: u8) -> Pllsrc {
            Pllsrc::from_bits(val)
        }
    }
    impl From<Pllsrc> for u8 {
        #[inline(always)]
        fn from(val: Pllsrc) -> u8 {
            Pllsrc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ppre {
        #[doc = "HCLK not divided"]
        DIV1 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "HCLK divided by 2"]
        DIV2 = 0x04,
        #[doc = "HCLK divided by 4"]
        DIV4 = 0x05,
        #[doc = "HCLK divided by 8"]
        DIV8 = 0x06,
        #[doc = "HCLK divided by 16"]
        DIV16 = 0x07,
    }
    impl Ppre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ppre {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ppre {
        #[inline(always)]
        fn from(val: u8) -> Ppre {
            Ppre::from_bits(val)
        }
    }
    impl From<Ppre> for u8 {
        #[inline(always)]
        fn from(val: Ppre) -> u8 {
            Ppre::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Rtcsel {
        #[doc = "No clock"]
        DISABLE = 0x0,
        #[doc = "LSE oscillator clock used as RTC clock"]
        LSE = 0x01,
        #[doc = "LSI oscillator clock used as RTC clock"]
        LSI = 0x02,
        #[doc = "HSE oscillator clock divided by a prescaler used as RTC clock"]
        HSE = 0x03,
    }
    impl Rtcsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rtcsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rtcsel {
        #[inline(always)]
        fn from(val: u8) -> Rtcsel {
            Rtcsel::from_bits(val)
        }
    }
    impl From<Rtcsel> for u8 {
        #[inline(always)]
        fn from(val: Rtcsel) -> u8 {
            Rtcsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Stopwuck {
        #[doc = "MSI oscillator selected as wake-up from Stop clock and CSS backup clock"]
        MSI = 0x0,
        #[doc = "HSI oscillator selected as wake-up from stop clock and CSS backup clock"]
        HSI = 0x01,
    }
    impl Stopwuck {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Stopwuck {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Stopwuck {
        #[inline(always)]
        fn from(val: u8) -> Stopwuck {
            Stopwuck::from_bits(val)
        }
    }
    impl From<Stopwuck> for u8 {
        #[inline(always)]
        fn from(val: Stopwuck) -> u8 {
            Stopwuck::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Sw {
        #[doc = "MSI selected as system clock"]
        MSI = 0x0,
        #[doc = "HSI selected as system clock"]
        HSI = 0x01,
        #[doc = "HSE selected as system clock"]
        HSE = 0x02,
        #[doc = "PLL selected as system clock"]
        PLL1_R = 0x03,
    }
    impl Sw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sw {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sw {
        #[inline(always)]
        fn from(val: u8) -> Sw {
            Sw::from_bits(val)
        }
    }
    impl From<Sw> for u8 {
        #[inline(always)]
        fn from(val: Sw) -> u8 {
            Sw::to_bits(val)
        }
    }
}
