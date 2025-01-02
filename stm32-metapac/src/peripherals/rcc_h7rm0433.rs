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
    #[doc = "clock control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "RCC HSI configuration register"]
    #[inline(always)]
    pub const fn hsicfgr(self) -> crate::common::Reg<regs::Hsicfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "RCC Internal Clock Source Calibration Register"]
    #[inline(always)]
    pub const fn icscr(self) -> crate::common::Reg<regs::Icscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "RCC Clock Recovery RC Register"]
    #[inline(always)]
    pub const fn crrcr(self) -> crate::common::Reg<regs::Crrcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "RCC CSI configuration register"]
    #[inline(always)]
    pub const fn csicfgr(self) -> crate::common::Reg<regs::Csicfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "RCC Clock Configuration Register"]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "RCC Domain 1 Clock Configuration Register"]
    #[inline(always)]
    pub const fn d1cfgr(self) -> crate::common::Reg<regs::D1cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "RCC Domain 2 Clock Configuration Register"]
    #[inline(always)]
    pub const fn d2cfgr(self) -> crate::common::Reg<regs::D2cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "RCC Domain 3 Clock Configuration Register"]
    #[inline(always)]
    pub const fn d3cfgr(self) -> crate::common::Reg<regs::D3cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "RCC PLLs Clock Source Selection Register"]
    #[inline(always)]
    pub const fn pllckselr(self) -> crate::common::Reg<regs::Pllckselr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "RCC PLLs Configuration Register"]
    #[inline(always)]
    pub const fn pllcfgr(self) -> crate::common::Reg<regs::Pllcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "RCC PLL1 Dividers Configuration Register"]
    #[inline(always)]
    pub const fn plldivr(self, n: usize) -> crate::common::Reg<regs::Plldivr, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize + n * 8usize) as _) }
    }
    #[doc = "RCC PLL1 Fractional Divider Register"]
    #[inline(always)]
    pub const fn pllfracr(self, n: usize) -> crate::common::Reg<regs::Pllfracr, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize + n * 8usize) as _) }
    }
    #[doc = "RCC Domain 1 Kernel Clock Configuration Register"]
    #[inline(always)]
    pub const fn d1ccipr(self) -> crate::common::Reg<regs::D1ccipr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "RCC Domain 2 Kernel Clock Configuration Register"]
    #[inline(always)]
    pub const fn d2ccip1r(self) -> crate::common::Reg<regs::D2ccip1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "RCC Domain 2 Kernel Clock Configuration Register"]
    #[inline(always)]
    pub const fn d2ccip2r(self) -> crate::common::Reg<regs::D2ccip2r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "RCC Domain 3 Kernel Clock Configuration Register"]
    #[inline(always)]
    pub const fn d3ccipr(self) -> crate::common::Reg<regs::D3ccipr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "RCC Clock Source Interrupt Enable Register"]
    #[inline(always)]
    pub const fn cier(self) -> crate::common::Reg<regs::Cier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "RCC Clock Source Interrupt Flag Register"]
    #[inline(always)]
    pub const fn cifr(self) -> crate::common::Reg<regs::Cifr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "RCC Clock Source Interrupt Clear Register"]
    #[inline(always)]
    pub const fn cicr(self) -> crate::common::Reg<regs::Cicr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "RCC Backup Domain Control Register"]
    #[inline(always)]
    pub const fn bdcr(self) -> crate::common::Reg<regs::Bdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "RCC Clock Control and Status Register"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "RCC AHB3 Reset Register"]
    #[inline(always)]
    pub const fn ahb3rstr(self) -> crate::common::Reg<regs::Ahb3rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "RCC AHB1 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn ahb1rstr(self) -> crate::common::Reg<regs::Ahb1rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "RCC AHB2 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn ahb2rstr(self) -> crate::common::Reg<regs::Ahb2rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "RCC AHB4 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn ahb4rstr(self) -> crate::common::Reg<regs::Ahb4rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "RCC APB3 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn apb3rstr(self) -> crate::common::Reg<regs::Apb3rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "RCC APB1 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn apb1lrstr(self) -> crate::common::Reg<regs::Apb1lrstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "RCC APB1 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn apb1hrstr(self) -> crate::common::Reg<regs::Apb1hrstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "RCC APB2 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn apb2rstr(self) -> crate::common::Reg<regs::Apb2rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "RCC APB4 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn apb4rstr(self) -> crate::common::Reg<regs::Apb4rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "Global Control Register"]
    #[inline(always)]
    pub const fn gcr(self) -> crate::common::Reg<regs::Gcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "RCC D3 Autonomous mode Register"]
    #[inline(always)]
    pub const fn d3amr(self) -> crate::common::Reg<regs::D3amr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "RCC Reset Status Register"]
    #[inline(always)]
    pub const fn rsr(self) -> crate::common::Reg<regs::Rsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[doc = "RCC AHB3 Clock Register"]
    #[inline(always)]
    pub const fn ahb3enr(self) -> crate::common::Reg<regs::Ahb3enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[doc = "RCC AHB1 Clock Register"]
    #[inline(always)]
    pub const fn ahb1enr(self) -> crate::common::Reg<regs::Ahb1enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[doc = "RCC AHB2 Clock Register"]
    #[inline(always)]
    pub const fn ahb2enr(self) -> crate::common::Reg<regs::Ahb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[doc = "RCC AHB4 Clock Register"]
    #[inline(always)]
    pub const fn ahb4enr(self) -> crate::common::Reg<regs::Ahb4enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[doc = "RCC APB3 Clock Register"]
    #[inline(always)]
    pub const fn apb3enr(self) -> crate::common::Reg<regs::Apb3enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
    }
    #[doc = "RCC APB1 Clock Register"]
    #[inline(always)]
    pub const fn apb1lenr(self) -> crate::common::Reg<regs::Apb1lenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe8usize) as _) }
    }
    #[doc = "RCC APB1 Clock Register"]
    #[inline(always)]
    pub const fn apb1henr(self) -> crate::common::Reg<regs::Apb1henr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xecusize) as _) }
    }
    #[doc = "RCC APB2 Clock Register"]
    #[inline(always)]
    pub const fn apb2enr(self) -> crate::common::Reg<regs::Apb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[doc = "RCC APB4 Clock Register"]
    #[inline(always)]
    pub const fn apb4enr(self) -> crate::common::Reg<regs::Apb4enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[doc = "RCC AHB3 Sleep Clock Register"]
    #[inline(always)]
    pub const fn ahb3lpenr(self) -> crate::common::Reg<regs::Ahb3lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
    #[doc = "RCC AHB1 Sleep Clock Register"]
    #[inline(always)]
    pub const fn ahb1lpenr(self) -> crate::common::Reg<regs::Ahb1lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "RCC AHB2 Sleep Clock Register"]
    #[inline(always)]
    pub const fn ahb2lpenr(self) -> crate::common::Reg<regs::Ahb2lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "RCC AHB4 Sleep Clock Register"]
    #[inline(always)]
    pub const fn ahb4lpenr(self) -> crate::common::Reg<regs::Ahb4lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "RCC APB3 Sleep Clock Register"]
    #[inline(always)]
    pub const fn apb3lpenr(self) -> crate::common::Reg<regs::Apb3lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "RCC APB1 Low Sleep Clock Register"]
    #[inline(always)]
    pub const fn apb1llpenr(self) -> crate::common::Reg<regs::Apb1llpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "RCC APB1 High Sleep Clock Register"]
    #[inline(always)]
    pub const fn apb1hlpenr(self) -> crate::common::Reg<regs::Apb1hlpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "RCC APB2 Sleep Clock Register"]
    #[inline(always)]
    pub const fn apb2lpenr(self) -> crate::common::Reg<regs::Apb2lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[doc = "RCC APB4 Sleep Clock Register"]
    #[inline(always)]
    pub const fn apb4lpenr(self) -> crate::common::Reg<regs::Apb4lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
    }
    #[doc = "RCC Reset Status Register"]
    #[inline(always)]
    pub const fn c1_rsr(self) -> crate::common::Reg<regs::C1Rsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[doc = "RCC AHB3 Clock Register"]
    #[inline(always)]
    pub const fn c1_ahb3enr(self) -> crate::common::Reg<regs::C1Ahb3enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
    }
    #[doc = "RCC AHB1 Clock Register"]
    #[inline(always)]
    pub const fn c1_ahb1enr(self) -> crate::common::Reg<regs::C1Ahb1enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
    }
    #[doc = "RCC AHB2 Clock Register"]
    #[inline(always)]
    pub const fn c1_ahb2enr(self) -> crate::common::Reg<regs::C1Ahb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x013cusize) as _) }
    }
    #[doc = "RCC AHB4 Clock Register"]
    #[inline(always)]
    pub const fn c1_ahb4enr(self) -> crate::common::Reg<regs::C1Ahb4enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "RCC APB3 Clock Register"]
    #[inline(always)]
    pub const fn c1_apb3enr(self) -> crate::common::Reg<regs::C1Apb3enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
    #[doc = "RCC APB1 Clock Register"]
    #[inline(always)]
    pub const fn c1_apb1lenr(self) -> crate::common::Reg<regs::C1Apb1lenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
    }
    #[doc = "RCC APB1 Clock Register"]
    #[inline(always)]
    pub const fn c1_apb1henr(self) -> crate::common::Reg<regs::C1Apb1henr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
    }
    #[doc = "RCC APB2 Clock Register"]
    #[inline(always)]
    pub const fn c1_apb2enr(self) -> crate::common::Reg<regs::C1Apb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
    }
    #[doc = "RCC APB4 Clock Register"]
    #[inline(always)]
    pub const fn c1_apb4enr(self) -> crate::common::Reg<regs::C1Apb4enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0154usize) as _) }
    }
    #[doc = "RCC AHB3 Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_ahb3lpenr(self) -> crate::common::Reg<regs::C1Ahb3lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x015cusize) as _) }
    }
    #[doc = "RCC AHB1 Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_ahb1lpenr(self) -> crate::common::Reg<regs::C1Ahb1lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0160usize) as _) }
    }
    #[doc = "RCC AHB2 Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_ahb2lpenr(self) -> crate::common::Reg<regs::C1Ahb2lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0164usize) as _) }
    }
    #[doc = "RCC AHB4 Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_ahb4lpenr(self) -> crate::common::Reg<regs::C1Ahb4lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0168usize) as _) }
    }
    #[doc = "RCC APB3 Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_apb3lpenr(self) -> crate::common::Reg<regs::C1Apb3lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x016cusize) as _) }
    }
    #[doc = "RCC APB1 Low Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_apb1llpenr(self) -> crate::common::Reg<regs::C1Apb1llpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0170usize) as _) }
    }
    #[doc = "RCC APB1 High Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_apb1hlpenr(self) -> crate::common::Reg<regs::C1Apb1hlpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0174usize) as _) }
    }
    #[doc = "RCC APB2 Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_apb2lpenr(self) -> crate::common::Reg<regs::C1Apb2lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0178usize) as _) }
    }
    #[doc = "RCC APB4 Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_apb4lpenr(self) -> crate::common::Reg<regs::C1Apb4lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x017cusize) as _) }
    }
}
pub mod regs {
    #[doc = "RCC AHB1 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1enr(pub u32);
    impl Ahb1enr {
        #[doc = "DMA1 Clock Enable"]
        #[inline(always)]
        pub const fn dma1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 Clock Enable"]
        #[inline(always)]
        pub fn set_dma1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2 Clock Enable"]
        #[inline(always)]
        pub const fn dma2en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 Clock Enable"]
        #[inline(always)]
        pub fn set_dma2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ADC1/2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn adc12en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1/2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_adc12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ART Clock Enable"]
        #[inline(always)]
        pub const fn arten(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "ART Clock Enable"]
        #[inline(always)]
        pub fn set_arten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Ethernet MAC bus interface Clock Enable"]
        #[inline(always)]
        pub const fn ethen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet MAC bus interface Clock Enable"]
        #[inline(always)]
        pub fn set_ethen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Ethernet Transmission Clock Enable"]
        #[inline(always)]
        pub const fn ethtxen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet Transmission Clock Enable"]
        #[inline(always)]
        pub fn set_ethtxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Ethernet Reception Clock Enable"]
        #[inline(always)]
        pub const fn ethrxen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet Reception Clock Enable"]
        #[inline(always)]
        pub fn set_ethrxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USB_OTG_HS Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn usb_otg_hsen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "USB_OTG_HS Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_usb_otg_hsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "USB_OTG_HS ULPI clock enable"]
        #[inline(always)]
        pub const fn usb_otg_hs_ulpien(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "USB_OTG_HS ULPI clock enable"]
        #[inline(always)]
        pub fn set_usb_otg_hs_ulpien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "USB_OTG_FS Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn usb_otg_fsen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "USB_OTG_FS Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_usb_otg_fsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "USB_OTG_FS ULPI clock enable"]
        #[inline(always)]
        pub const fn usb_otg_fs_ulpien(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "USB_OTG_FS ULPI clock enable"]
        #[inline(always)]
        pub fn set_usb_otg_fs_ulpien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Ahb1enr {
        #[inline(always)]
        fn default() -> Ahb1enr {
            Ahb1enr(0)
        }
    }
    impl core::fmt::Debug for Ahb1enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb1enr")
                .field("dma1en", &self.dma1en())
                .field("dma2en", &self.dma2en())
                .field("adc12en", &self.adc12en())
                .field("arten", &self.arten())
                .field("ethen", &self.ethen())
                .field("ethtxen", &self.ethtxen())
                .field("ethrxen", &self.ethrxen())
                .field("usb_otg_hsen", &self.usb_otg_hsen())
                .field("usb_otg_hs_ulpien", &self.usb_otg_hs_ulpien())
                .field("usb_otg_fsen", &self.usb_otg_fsen())
                .field("usb_otg_fs_ulpien", &self.usb_otg_fs_ulpien())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1enr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb1enr {
                dma1en: bool,
                dma2en: bool,
                adc12en: bool,
                arten: bool,
                ethen: bool,
                ethtxen: bool,
                ethrxen: bool,
                usb_otg_hsen: bool,
                usb_otg_hs_ulpien: bool,
                usb_otg_fsen: bool,
                usb_otg_fs_ulpien: bool,
            }
            let proxy = Ahb1enr {
                dma1en: self.dma1en(),
                dma2en: self.dma2en(),
                adc12en: self.adc12en(),
                arten: self.arten(),
                ethen: self.ethen(),
                ethtxen: self.ethtxen(),
                ethrxen: self.ethrxen(),
                usb_otg_hsen: self.usb_otg_hsen(),
                usb_otg_hs_ulpien: self.usb_otg_hs_ulpien(),
                usb_otg_fsen: self.usb_otg_fsen(),
                usb_otg_fs_ulpien: self.usb_otg_fs_ulpien(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC AHB1 Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1lpenr(pub u32);
    impl Ahb1lpenr {
        #[doc = "DMA1 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn dma1lpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_dma1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn dma2lpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_dma2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ADC1/2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn adc12lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1/2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_adc12lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ART Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn artlpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "ART Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_artlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Ethernet MAC bus interface Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn ethlpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet MAC bus interface Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_ethlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Ethernet Transmission Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn ethtxlpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet Transmission Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_ethtxlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Ethernet Reception Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn ethrxlpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet Reception Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_ethrxlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USB_OTG_HS peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn usb_otg_hslpen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "USB_OTG_HS peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_usb_otg_hslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "USB_PHY1 clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn usb_otg_hs_ulpilpen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "USB_PHY1 clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_usb_otg_hs_ulpilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "USB_OTG_FS peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn usb_otg_fslpen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "USB_OTG_FS peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_usb_otg_fslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "USB_PHY2 clocks enable during CSleep mode"]
        #[inline(always)]
        pub const fn usb_otg_fs_ulpilpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "USB_PHY2 clocks enable during CSleep mode"]
        #[inline(always)]
        pub fn set_usb_otg_fs_ulpilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Ahb1lpenr {
        #[inline(always)]
        fn default() -> Ahb1lpenr {
            Ahb1lpenr(0)
        }
    }
    impl core::fmt::Debug for Ahb1lpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb1lpenr")
                .field("dma1lpen", &self.dma1lpen())
                .field("dma2lpen", &self.dma2lpen())
                .field("adc12lpen", &self.adc12lpen())
                .field("artlpen", &self.artlpen())
                .field("ethlpen", &self.ethlpen())
                .field("ethtxlpen", &self.ethtxlpen())
                .field("ethrxlpen", &self.ethrxlpen())
                .field("usb_otg_hslpen", &self.usb_otg_hslpen())
                .field("usb_otg_hs_ulpilpen", &self.usb_otg_hs_ulpilpen())
                .field("usb_otg_fslpen", &self.usb_otg_fslpen())
                .field("usb_otg_fs_ulpilpen", &self.usb_otg_fs_ulpilpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1lpenr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb1lpenr {
                dma1lpen: bool,
                dma2lpen: bool,
                adc12lpen: bool,
                artlpen: bool,
                ethlpen: bool,
                ethtxlpen: bool,
                ethrxlpen: bool,
                usb_otg_hslpen: bool,
                usb_otg_hs_ulpilpen: bool,
                usb_otg_fslpen: bool,
                usb_otg_fs_ulpilpen: bool,
            }
            let proxy = Ahb1lpenr {
                dma1lpen: self.dma1lpen(),
                dma2lpen: self.dma2lpen(),
                adc12lpen: self.adc12lpen(),
                artlpen: self.artlpen(),
                ethlpen: self.ethlpen(),
                ethtxlpen: self.ethtxlpen(),
                ethrxlpen: self.ethrxlpen(),
                usb_otg_hslpen: self.usb_otg_hslpen(),
                usb_otg_hs_ulpilpen: self.usb_otg_hs_ulpilpen(),
                usb_otg_fslpen: self.usb_otg_fslpen(),
                usb_otg_fs_ulpilpen: self.usb_otg_fs_ulpilpen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC AHB1 Peripheral Reset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1rstr(pub u32);
    impl Ahb1rstr {
        #[doc = "DMA1 block reset"]
        #[inline(always)]
        pub const fn dma1rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 block reset"]
        #[inline(always)]
        pub fn set_dma1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2 block reset"]
        #[inline(always)]
        pub const fn dma2rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 block reset"]
        #[inline(always)]
        pub fn set_dma2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ADC1&2 block reset"]
        #[inline(always)]
        pub const fn adc12rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1&2 block reset"]
        #[inline(always)]
        pub fn set_adc12rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ART block reset"]
        #[inline(always)]
        pub const fn artrst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "ART block reset"]
        #[inline(always)]
        pub fn set_artrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "ETH block reset"]
        #[inline(always)]
        pub const fn ethrst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "ETH block reset"]
        #[inline(always)]
        pub fn set_ethrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "USB_OTG_HS block reset"]
        #[inline(always)]
        pub const fn usb_otg_hsrst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "USB_OTG_HS block reset"]
        #[inline(always)]
        pub fn set_usb_otg_hsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "USB_OTG_FS block reset"]
        #[inline(always)]
        pub const fn usb_otg_fsrst(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "USB_OTG_FS block reset"]
        #[inline(always)]
        pub fn set_usb_otg_fsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Ahb1rstr {
        #[inline(always)]
        fn default() -> Ahb1rstr {
            Ahb1rstr(0)
        }
    }
    impl core::fmt::Debug for Ahb1rstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb1rstr")
                .field("dma1rst", &self.dma1rst())
                .field("dma2rst", &self.dma2rst())
                .field("adc12rst", &self.adc12rst())
                .field("artrst", &self.artrst())
                .field("ethrst", &self.ethrst())
                .field("usb_otg_hsrst", &self.usb_otg_hsrst())
                .field("usb_otg_fsrst", &self.usb_otg_fsrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1rstr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb1rstr {
                dma1rst: bool,
                dma2rst: bool,
                adc12rst: bool,
                artrst: bool,
                ethrst: bool,
                usb_otg_hsrst: bool,
                usb_otg_fsrst: bool,
            }
            let proxy = Ahb1rstr {
                dma1rst: self.dma1rst(),
                dma2rst: self.dma2rst(),
                adc12rst: self.adc12rst(),
                artrst: self.artrst(),
                ethrst: self.ethrst(),
                usb_otg_hsrst: self.usb_otg_hsrst(),
                usb_otg_fsrst: self.usb_otg_fsrst(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC AHB2 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2enr(pub u32);
    impl Ahb2enr {
        #[doc = "DCMI peripheral clock"]
        #[inline(always)]
        pub const fn dcmien(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DCMI peripheral clock"]
        #[inline(always)]
        pub fn set_dcmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CRYP peripheral clock enable"]
        #[inline(always)]
        pub const fn crypen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CRYP peripheral clock enable"]
        #[inline(always)]
        pub fn set_crypen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "HASH peripheral clock enable"]
        #[inline(always)]
        pub const fn hashen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HASH peripheral clock enable"]
        #[inline(always)]
        pub fn set_hashen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "RNG peripheral clocks enable"]
        #[inline(always)]
        pub const fn rngen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RNG peripheral clocks enable"]
        #[inline(always)]
        pub fn set_rngen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "SDMMC2 and SDMMC2 delay clock enable"]
        #[inline(always)]
        pub const fn sdmmc2en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC2 and SDMMC2 delay clock enable"]
        #[inline(always)]
        pub fn set_sdmmc2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "FMAC enable"]
        #[inline(always)]
        pub const fn fmacen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "FMAC enable"]
        #[inline(always)]
        pub fn set_fmacen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "CORDIC enable"]
        #[inline(always)]
        pub const fn cordicen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "CORDIC enable"]
        #[inline(always)]
        pub fn set_cordicen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "SRAM1 block enable"]
        #[inline(always)]
        pub const fn sram1en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 block enable"]
        #[inline(always)]
        pub fn set_sram1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "SRAM2 block enable"]
        #[inline(always)]
        pub const fn sram2en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 block enable"]
        #[inline(always)]
        pub fn set_sram2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SRAM3 block enable"]
        #[inline(always)]
        pub const fn sram3en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM3 block enable"]
        #[inline(always)]
        pub fn set_sram3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ahb2enr {
        #[inline(always)]
        fn default() -> Ahb2enr {
            Ahb2enr(0)
        }
    }
    impl core::fmt::Debug for Ahb2enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb2enr")
                .field("dcmien", &self.dcmien())
                .field("crypen", &self.crypen())
                .field("hashen", &self.hashen())
                .field("rngen", &self.rngen())
                .field("sdmmc2en", &self.sdmmc2en())
                .field("fmacen", &self.fmacen())
                .field("cordicen", &self.cordicen())
                .field("sram1en", &self.sram1en())
                .field("sram2en", &self.sram2en())
                .field("sram3en", &self.sram3en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2enr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb2enr {
                dcmien: bool,
                crypen: bool,
                hashen: bool,
                rngen: bool,
                sdmmc2en: bool,
                fmacen: bool,
                cordicen: bool,
                sram1en: bool,
                sram2en: bool,
                sram3en: bool,
            }
            let proxy = Ahb2enr {
                dcmien: self.dcmien(),
                crypen: self.crypen(),
                hashen: self.hashen(),
                rngen: self.rngen(),
                sdmmc2en: self.sdmmc2en(),
                fmacen: self.fmacen(),
                cordicen: self.cordicen(),
                sram1en: self.sram1en(),
                sram2en: self.sram2en(),
                sram3en: self.sram3en(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC AHB2 Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2lpenr(pub u32);
    impl Ahb2lpenr {
        #[doc = "DCMI peripheral clock enable during csleep mode"]
        #[inline(always)]
        pub const fn dcmilpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DCMI peripheral clock enable during csleep mode"]
        #[inline(always)]
        pub fn set_dcmilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CRYP peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn cryplpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CRYP peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_cryplpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "HASH peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn hashlpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HASH peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_hashlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "RNG peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn rnglpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RNG peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_rnglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn sdmmc2lpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_sdmmc2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "FMAC enable during CSleep Mode"]
        #[inline(always)]
        pub const fn fmaclpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "FMAC enable during CSleep Mode"]
        #[inline(always)]
        pub fn set_fmaclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "CORDIC enable during CSleep Mode"]
        #[inline(always)]
        pub const fn cordiclpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "CORDIC enable during CSleep Mode"]
        #[inline(always)]
        pub fn set_cordiclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "SRAM1 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn sram1lpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_sram1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "SRAM2 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn sram2lpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_sram2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SRAM3 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn sram3lpen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM3 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_sram3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ahb2lpenr {
        #[inline(always)]
        fn default() -> Ahb2lpenr {
            Ahb2lpenr(0)
        }
    }
    impl core::fmt::Debug for Ahb2lpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb2lpenr")
                .field("dcmilpen", &self.dcmilpen())
                .field("cryplpen", &self.cryplpen())
                .field("hashlpen", &self.hashlpen())
                .field("rnglpen", &self.rnglpen())
                .field("sdmmc2lpen", &self.sdmmc2lpen())
                .field("fmaclpen", &self.fmaclpen())
                .field("cordiclpen", &self.cordiclpen())
                .field("sram1lpen", &self.sram1lpen())
                .field("sram2lpen", &self.sram2lpen())
                .field("sram3lpen", &self.sram3lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2lpenr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb2lpenr {
                dcmilpen: bool,
                cryplpen: bool,
                hashlpen: bool,
                rnglpen: bool,
                sdmmc2lpen: bool,
                fmaclpen: bool,
                cordiclpen: bool,
                sram1lpen: bool,
                sram2lpen: bool,
                sram3lpen: bool,
            }
            let proxy = Ahb2lpenr {
                dcmilpen: self.dcmilpen(),
                cryplpen: self.cryplpen(),
                hashlpen: self.hashlpen(),
                rnglpen: self.rnglpen(),
                sdmmc2lpen: self.sdmmc2lpen(),
                fmaclpen: self.fmaclpen(),
                cordiclpen: self.cordiclpen(),
                sram1lpen: self.sram1lpen(),
                sram2lpen: self.sram2lpen(),
                sram3lpen: self.sram3lpen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC AHB2 Peripheral Reset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2rstr(pub u32);
    impl Ahb2rstr {
        #[doc = "DCMI block reset"]
        #[inline(always)]
        pub const fn dcmirst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DCMI block reset"]
        #[inline(always)]
        pub fn set_dcmirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CRYPography block reset"]
        #[inline(always)]
        pub const fn cryprst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CRYPography block reset"]
        #[inline(always)]
        pub fn set_cryprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Hash block reset"]
        #[inline(always)]
        pub const fn hashrst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Hash block reset"]
        #[inline(always)]
        pub fn set_hashrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Random Number Generator block reset"]
        #[inline(always)]
        pub const fn rngrst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Random Number Generator block reset"]
        #[inline(always)]
        pub fn set_rngrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "SDMMC2 and SDMMC2 Delay block reset"]
        #[inline(always)]
        pub const fn sdmmc2rst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC2 and SDMMC2 Delay block reset"]
        #[inline(always)]
        pub fn set_sdmmc2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "FMAC reset"]
        #[inline(always)]
        pub const fn fmacrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "FMAC reset"]
        #[inline(always)]
        pub fn set_fmacrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "CORDIC reset"]
        #[inline(always)]
        pub const fn cordicrst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "CORDIC reset"]
        #[inline(always)]
        pub fn set_cordicrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Ahb2rstr {
        #[inline(always)]
        fn default() -> Ahb2rstr {
            Ahb2rstr(0)
        }
    }
    impl core::fmt::Debug for Ahb2rstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb2rstr")
                .field("dcmirst", &self.dcmirst())
                .field("cryprst", &self.cryprst())
                .field("hashrst", &self.hashrst())
                .field("rngrst", &self.rngrst())
                .field("sdmmc2rst", &self.sdmmc2rst())
                .field("fmacrst", &self.fmacrst())
                .field("cordicrst", &self.cordicrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2rstr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb2rstr {
                dcmirst: bool,
                cryprst: bool,
                hashrst: bool,
                rngrst: bool,
                sdmmc2rst: bool,
                fmacrst: bool,
                cordicrst: bool,
            }
            let proxy = Ahb2rstr {
                dcmirst: self.dcmirst(),
                cryprst: self.cryprst(),
                hashrst: self.hashrst(),
                rngrst: self.rngrst(),
                sdmmc2rst: self.sdmmc2rst(),
                fmacrst: self.fmacrst(),
                cordicrst: self.cordicrst(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC AHB3 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3enr(pub u32);
    impl Ahb3enr {
        #[doc = "MDMA Peripheral Clock Enable"]
        #[inline(always)]
        pub const fn mdmaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MDMA Peripheral Clock Enable"]
        #[inline(always)]
        pub fn set_mdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2D Peripheral Clock Enable"]
        #[inline(always)]
        pub const fn dma2den(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2D Peripheral Clock Enable"]
        #[inline(always)]
        pub fn set_dma2den(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "JPGDEC Peripheral Clock Enable"]
        #[inline(always)]
        pub const fn jpgdecen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "JPGDEC Peripheral Clock Enable"]
        #[inline(always)]
        pub fn set_jpgdecen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FMC Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn fmcen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "FMC Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_fmcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "QUADSPI and QUADSPI Delay Clock Enable"]
        #[inline(always)]
        pub const fn quadspien(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "QUADSPI and QUADSPI Delay Clock Enable"]
        #[inline(always)]
        pub fn set_quadspien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SDMMC1 and SDMMC1 Delay Clock Enable"]
        #[inline(always)]
        pub const fn sdmmc1en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC1 and SDMMC1 Delay Clock Enable"]
        #[inline(always)]
        pub fn set_sdmmc1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "OCTOSPI2 and OCTOSPI2 delay block enable"]
        #[inline(always)]
        pub const fn octospi2en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI2 and OCTOSPI2 delay block enable"]
        #[inline(always)]
        pub fn set_octospi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "OCTOSPI IO manager enable"]
        #[inline(always)]
        pub const fn iomngren(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI IO manager enable"]
        #[inline(always)]
        pub fn set_iomngren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "OTFDEC1 enable"]
        #[inline(always)]
        pub const fn otfd1en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC1 enable"]
        #[inline(always)]
        pub fn set_otfd1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "OTFDEC2 enable"]
        #[inline(always)]
        pub const fn otfd2en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC2 enable"]
        #[inline(always)]
        pub fn set_otfd2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "D1 DTCM1 block enable"]
        #[inline(always)]
        pub const fn dtcm1en(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "D1 DTCM1 block enable"]
        #[inline(always)]
        pub fn set_dtcm1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "D1 DTCM2 block enable"]
        #[inline(always)]
        pub const fn dtcm2en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "D1 DTCM2 block enable"]
        #[inline(always)]
        pub fn set_dtcm2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "D1 ITCM block enable"]
        #[inline(always)]
        pub const fn itcm1en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "D1 ITCM block enable"]
        #[inline(always)]
        pub fn set_itcm1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "AXISRAM block enable"]
        #[inline(always)]
        pub const fn axisramen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "AXISRAM block enable"]
        #[inline(always)]
        pub fn set_axisramen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ahb3enr {
        #[inline(always)]
        fn default() -> Ahb3enr {
            Ahb3enr(0)
        }
    }
    impl core::fmt::Debug for Ahb3enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb3enr")
                .field("mdmaen", &self.mdmaen())
                .field("dma2den", &self.dma2den())
                .field("jpgdecen", &self.jpgdecen())
                .field("fmcen", &self.fmcen())
                .field("quadspien", &self.quadspien())
                .field("sdmmc1en", &self.sdmmc1en())
                .field("octospi2en", &self.octospi2en())
                .field("iomngren", &self.iomngren())
                .field("otfd1en", &self.otfd1en())
                .field("otfd2en", &self.otfd2en())
                .field("dtcm1en", &self.dtcm1en())
                .field("dtcm2en", &self.dtcm2en())
                .field("itcm1en", &self.itcm1en())
                .field("axisramen", &self.axisramen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb3enr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb3enr {
                mdmaen: bool,
                dma2den: bool,
                jpgdecen: bool,
                fmcen: bool,
                quadspien: bool,
                sdmmc1en: bool,
                octospi2en: bool,
                iomngren: bool,
                otfd1en: bool,
                otfd2en: bool,
                dtcm1en: bool,
                dtcm2en: bool,
                itcm1en: bool,
                axisramen: bool,
            }
            let proxy = Ahb3enr {
                mdmaen: self.mdmaen(),
                dma2den: self.dma2den(),
                jpgdecen: self.jpgdecen(),
                fmcen: self.fmcen(),
                quadspien: self.quadspien(),
                sdmmc1en: self.sdmmc1en(),
                octospi2en: self.octospi2en(),
                iomngren: self.iomngren(),
                otfd1en: self.otfd1en(),
                otfd2en: self.otfd2en(),
                dtcm1en: self.dtcm1en(),
                dtcm2en: self.dtcm2en(),
                itcm1en: self.itcm1en(),
                axisramen: self.axisramen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC AHB3 Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3lpenr(pub u32);
    impl Ahb3lpenr {
        #[doc = "MDMA Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn mdmalpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MDMA Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_mdmalpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2D Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn dma2dlpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2D Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_dma2dlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "JPGDEC Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn jpgdeclpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "JPGDEC Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_jpgdeclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FLASH Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn flashlpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FLASH Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_flashlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "FMC Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn fmclpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "FMC Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_fmclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "QUADSPI and QUADSPI Delay Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn quadspilpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "QUADSPI and QUADSPI Delay Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_quadspilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn sdmmc1lpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_sdmmc1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "OCTOSPI2 and OCTOSPI2 delay block enable during CSleep Mode"]
        #[inline(always)]
        pub const fn octospi2lpen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI2 and OCTOSPI2 delay block enable during CSleep Mode"]
        #[inline(always)]
        pub fn set_octospi2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "OCTOSPI IO manager enable during CSleep Mode"]
        #[inline(always)]
        pub const fn iomngrlpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI IO manager enable during CSleep Mode"]
        #[inline(always)]
        pub fn set_iomngrlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "OTFDEC1 enable during CSleep Mode"]
        #[inline(always)]
        pub const fn otfd1lpen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC1 enable during CSleep Mode"]
        #[inline(always)]
        pub fn set_otfd1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "OTFDEC2 enable during CSleep Mode"]
        #[inline(always)]
        pub const fn otfd2lpen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC2 enable during CSleep Mode"]
        #[inline(always)]
        pub fn set_otfd2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "D1DTCM1 Block Clock Enable During CSleep mode"]
        #[inline(always)]
        pub const fn d1dtcm1lpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "D1DTCM1 Block Clock Enable During CSleep mode"]
        #[inline(always)]
        pub fn set_d1dtcm1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "D1 DTCM2 Block Clock Enable During CSleep mode"]
        #[inline(always)]
        pub const fn dtcm2lpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "D1 DTCM2 Block Clock Enable During CSleep mode"]
        #[inline(always)]
        pub fn set_dtcm2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "D1ITCM Block Clock Enable During CSleep mode"]
        #[inline(always)]
        pub const fn itcmlpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "D1ITCM Block Clock Enable During CSleep mode"]
        #[inline(always)]
        pub fn set_itcmlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "AXISRAM Block Clock Enable During CSleep mode"]
        #[inline(always)]
        pub const fn axisramlpen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "AXISRAM Block Clock Enable During CSleep mode"]
        #[inline(always)]
        pub fn set_axisramlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ahb3lpenr {
        #[inline(always)]
        fn default() -> Ahb3lpenr {
            Ahb3lpenr(0)
        }
    }
    impl core::fmt::Debug for Ahb3lpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb3lpenr")
                .field("mdmalpen", &self.mdmalpen())
                .field("dma2dlpen", &self.dma2dlpen())
                .field("jpgdeclpen", &self.jpgdeclpen())
                .field("flashlpen", &self.flashlpen())
                .field("fmclpen", &self.fmclpen())
                .field("quadspilpen", &self.quadspilpen())
                .field("sdmmc1lpen", &self.sdmmc1lpen())
                .field("octospi2lpen", &self.octospi2lpen())
                .field("iomngrlpen", &self.iomngrlpen())
                .field("otfd1lpen", &self.otfd1lpen())
                .field("otfd2lpen", &self.otfd2lpen())
                .field("d1dtcm1lpen", &self.d1dtcm1lpen())
                .field("dtcm2lpen", &self.dtcm2lpen())
                .field("itcmlpen", &self.itcmlpen())
                .field("axisramlpen", &self.axisramlpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb3lpenr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb3lpenr {
                mdmalpen: bool,
                dma2dlpen: bool,
                jpgdeclpen: bool,
                flashlpen: bool,
                fmclpen: bool,
                quadspilpen: bool,
                sdmmc1lpen: bool,
                octospi2lpen: bool,
                iomngrlpen: bool,
                otfd1lpen: bool,
                otfd2lpen: bool,
                d1dtcm1lpen: bool,
                dtcm2lpen: bool,
                itcmlpen: bool,
                axisramlpen: bool,
            }
            let proxy = Ahb3lpenr {
                mdmalpen: self.mdmalpen(),
                dma2dlpen: self.dma2dlpen(),
                jpgdeclpen: self.jpgdeclpen(),
                flashlpen: self.flashlpen(),
                fmclpen: self.fmclpen(),
                quadspilpen: self.quadspilpen(),
                sdmmc1lpen: self.sdmmc1lpen(),
                octospi2lpen: self.octospi2lpen(),
                iomngrlpen: self.iomngrlpen(),
                otfd1lpen: self.otfd1lpen(),
                otfd2lpen: self.otfd2lpen(),
                d1dtcm1lpen: self.d1dtcm1lpen(),
                dtcm2lpen: self.dtcm2lpen(),
                itcmlpen: self.itcmlpen(),
                axisramlpen: self.axisramlpen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC AHB3 Reset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3rstr(pub u32);
    impl Ahb3rstr {
        #[doc = "MDMA block reset"]
        #[inline(always)]
        pub const fn mdmarst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MDMA block reset"]
        #[inline(always)]
        pub fn set_mdmarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2D block reset"]
        #[inline(always)]
        pub const fn dma2drst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2D block reset"]
        #[inline(always)]
        pub fn set_dma2drst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "JPGDEC block reset"]
        #[inline(always)]
        pub const fn jpgdecrst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "JPGDEC block reset"]
        #[inline(always)]
        pub fn set_jpgdecrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FMC block reset"]
        #[inline(always)]
        pub const fn fmcrst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "FMC block reset"]
        #[inline(always)]
        pub fn set_fmcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "QUADSPI and QUADSPI delay block reset"]
        #[inline(always)]
        pub const fn quadspirst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "QUADSPI and QUADSPI delay block reset"]
        #[inline(always)]
        pub fn set_quadspirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SDMMC1 and SDMMC1 delay block reset"]
        #[inline(always)]
        pub const fn sdmmc1rst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC1 and SDMMC1 delay block reset"]
        #[inline(always)]
        pub fn set_sdmmc1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "OCTOSPI2 and OCTOSPI2 delay block reset"]
        #[inline(always)]
        pub const fn octospi2rst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI2 and OCTOSPI2 delay block reset"]
        #[inline(always)]
        pub fn set_octospi2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "OCTOSPI IO manager reset"]
        #[inline(always)]
        pub const fn iomngrrst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI IO manager reset"]
        #[inline(always)]
        pub fn set_iomngrrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "OTFDEC1 reset"]
        #[inline(always)]
        pub const fn otfd1rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC1 reset"]
        #[inline(always)]
        pub fn set_otfd1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "OTFDEC2 reset"]
        #[inline(always)]
        pub const fn otfd2rst(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC2 reset"]
        #[inline(always)]
        pub fn set_otfd2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "CPU reset"]
        #[inline(always)]
        pub const fn cpurst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "CPU reset"]
        #[inline(always)]
        pub fn set_cpurst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ahb3rstr {
        #[inline(always)]
        fn default() -> Ahb3rstr {
            Ahb3rstr(0)
        }
    }
    impl core::fmt::Debug for Ahb3rstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb3rstr")
                .field("mdmarst", &self.mdmarst())
                .field("dma2drst", &self.dma2drst())
                .field("jpgdecrst", &self.jpgdecrst())
                .field("fmcrst", &self.fmcrst())
                .field("quadspirst", &self.quadspirst())
                .field("sdmmc1rst", &self.sdmmc1rst())
                .field("octospi2rst", &self.octospi2rst())
                .field("iomngrrst", &self.iomngrrst())
                .field("otfd1rst", &self.otfd1rst())
                .field("otfd2rst", &self.otfd2rst())
                .field("cpurst", &self.cpurst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb3rstr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb3rstr {
                mdmarst: bool,
                dma2drst: bool,
                jpgdecrst: bool,
                fmcrst: bool,
                quadspirst: bool,
                sdmmc1rst: bool,
                octospi2rst: bool,
                iomngrrst: bool,
                otfd1rst: bool,
                otfd2rst: bool,
                cpurst: bool,
            }
            let proxy = Ahb3rstr {
                mdmarst: self.mdmarst(),
                dma2drst: self.dma2drst(),
                jpgdecrst: self.jpgdecrst(),
                fmcrst: self.fmcrst(),
                quadspirst: self.quadspirst(),
                sdmmc1rst: self.sdmmc1rst(),
                octospi2rst: self.octospi2rst(),
                iomngrrst: self.iomngrrst(),
                otfd1rst: self.otfd1rst(),
                otfd2rst: self.otfd2rst(),
                cpurst: self.cpurst(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC AHB4 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb4enr(pub u32);
    impl Ahb4enr {
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn gpioaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub fn set_gpioaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn gpioben(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub fn set_gpioben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn gpiocen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub fn set_gpiocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn gpioden(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub fn set_gpioden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn gpioeen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub fn set_gpioeen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn gpiofen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub fn set_gpiofen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn gpiogen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub fn set_gpiogen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn gpiohen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub fn set_gpiohen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn gpioien(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub fn set_gpioien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn gpiojen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub fn set_gpiojen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn gpioken(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub fn set_gpioken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "CRC peripheral clock enable"]
        #[inline(always)]
        pub const fn crcen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "CRC peripheral clock enable"]
        #[inline(always)]
        pub fn set_crcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "BDMA and DMAMUX2 Clock Enable"]
        #[inline(always)]
        pub const fn bdmaen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "BDMA and DMAMUX2 Clock Enable"]
        #[inline(always)]
        pub fn set_bdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ADC3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn adc3en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ADC3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_adc3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "HSEM peripheral clock enable"]
        #[inline(always)]
        pub const fn hsemen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "HSEM peripheral clock enable"]
        #[inline(always)]
        pub fn set_hsemen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Backup RAM Clock Enable"]
        #[inline(always)]
        pub const fn bkpsramen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Backup RAM Clock Enable"]
        #[inline(always)]
        pub fn set_bkpsramen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Ahb4enr {
        #[inline(always)]
        fn default() -> Ahb4enr {
            Ahb4enr(0)
        }
    }
    impl core::fmt::Debug for Ahb4enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb4enr")
                .field("gpioaen", &self.gpioaen())
                .field("gpioben", &self.gpioben())
                .field("gpiocen", &self.gpiocen())
                .field("gpioden", &self.gpioden())
                .field("gpioeen", &self.gpioeen())
                .field("gpiofen", &self.gpiofen())
                .field("gpiogen", &self.gpiogen())
                .field("gpiohen", &self.gpiohen())
                .field("gpioien", &self.gpioien())
                .field("gpiojen", &self.gpiojen())
                .field("gpioken", &self.gpioken())
                .field("crcen", &self.crcen())
                .field("bdmaen", &self.bdmaen())
                .field("adc3en", &self.adc3en())
                .field("hsemen", &self.hsemen())
                .field("bkpsramen", &self.bkpsramen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb4enr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb4enr {
                gpioaen: bool,
                gpioben: bool,
                gpiocen: bool,
                gpioden: bool,
                gpioeen: bool,
                gpiofen: bool,
                gpiogen: bool,
                gpiohen: bool,
                gpioien: bool,
                gpiojen: bool,
                gpioken: bool,
                crcen: bool,
                bdmaen: bool,
                adc3en: bool,
                hsemen: bool,
                bkpsramen: bool,
            }
            let proxy = Ahb4enr {
                gpioaen: self.gpioaen(),
                gpioben: self.gpioben(),
                gpiocen: self.gpiocen(),
                gpioden: self.gpioden(),
                gpioeen: self.gpioeen(),
                gpiofen: self.gpiofen(),
                gpiogen: self.gpiogen(),
                gpiohen: self.gpiohen(),
                gpioien: self.gpioien(),
                gpiojen: self.gpiojen(),
                gpioken: self.gpioken(),
                crcen: self.crcen(),
                bdmaen: self.bdmaen(),
                adc3en: self.adc3en(),
                hsemen: self.hsemen(),
                bkpsramen: self.bkpsramen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC AHB4 Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb4lpenr(pub u32);
    impl Ahb4lpenr {
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn gpioalpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_gpioalpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn gpioblpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_gpioblpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn gpioclpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_gpioclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn gpiodlpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_gpiodlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn gpioelpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_gpioelpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn gpioflpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_gpioflpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn gpioglpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_gpioglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn gpiohlpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_gpiohlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn gpioilpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_gpioilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn gpiojlpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_gpiojlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn gpioklpen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_gpioklpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "CRC peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn crclpen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "CRC peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_crclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "BDMA Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn bdmalpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "BDMA Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_bdmalpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ADC3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn adc3lpen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ADC3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_adc3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Backup RAM Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn bkpsramlpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Backup RAM Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_bkpsramlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "SRAM4 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn sram4lpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM4 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_sram4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Ahb4lpenr {
        #[inline(always)]
        fn default() -> Ahb4lpenr {
            Ahb4lpenr(0)
        }
    }
    impl core::fmt::Debug for Ahb4lpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb4lpenr")
                .field("gpioalpen", &self.gpioalpen())
                .field("gpioblpen", &self.gpioblpen())
                .field("gpioclpen", &self.gpioclpen())
                .field("gpiodlpen", &self.gpiodlpen())
                .field("gpioelpen", &self.gpioelpen())
                .field("gpioflpen", &self.gpioflpen())
                .field("gpioglpen", &self.gpioglpen())
                .field("gpiohlpen", &self.gpiohlpen())
                .field("gpioilpen", &self.gpioilpen())
                .field("gpiojlpen", &self.gpiojlpen())
                .field("gpioklpen", &self.gpioklpen())
                .field("crclpen", &self.crclpen())
                .field("bdmalpen", &self.bdmalpen())
                .field("adc3lpen", &self.adc3lpen())
                .field("bkpsramlpen", &self.bkpsramlpen())
                .field("sram4lpen", &self.sram4lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb4lpenr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb4lpenr {
                gpioalpen: bool,
                gpioblpen: bool,
                gpioclpen: bool,
                gpiodlpen: bool,
                gpioelpen: bool,
                gpioflpen: bool,
                gpioglpen: bool,
                gpiohlpen: bool,
                gpioilpen: bool,
                gpiojlpen: bool,
                gpioklpen: bool,
                crclpen: bool,
                bdmalpen: bool,
                adc3lpen: bool,
                bkpsramlpen: bool,
                sram4lpen: bool,
            }
            let proxy = Ahb4lpenr {
                gpioalpen: self.gpioalpen(),
                gpioblpen: self.gpioblpen(),
                gpioclpen: self.gpioclpen(),
                gpiodlpen: self.gpiodlpen(),
                gpioelpen: self.gpioelpen(),
                gpioflpen: self.gpioflpen(),
                gpioglpen: self.gpioglpen(),
                gpiohlpen: self.gpiohlpen(),
                gpioilpen: self.gpioilpen(),
                gpiojlpen: self.gpiojlpen(),
                gpioklpen: self.gpioklpen(),
                crclpen: self.crclpen(),
                bdmalpen: self.bdmalpen(),
                adc3lpen: self.adc3lpen(),
                bkpsramlpen: self.bkpsramlpen(),
                sram4lpen: self.sram4lpen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC AHB4 Peripheral Reset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb4rstr(pub u32);
    impl Ahb4rstr {
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub const fn gpioarst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub fn set_gpioarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub const fn gpiobrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub fn set_gpiobrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub const fn gpiocrst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub fn set_gpiocrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub const fn gpiodrst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub fn set_gpiodrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub const fn gpioerst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub fn set_gpioerst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub const fn gpiofrst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub fn set_gpiofrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub const fn gpiogrst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub fn set_gpiogrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub const fn gpiohrst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub fn set_gpiohrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub const fn gpioirst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub fn set_gpioirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub const fn gpiojrst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub fn set_gpiojrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub const fn gpiokrst(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub fn set_gpiokrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "CRC block reset"]
        #[inline(always)]
        pub const fn crcrst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "CRC block reset"]
        #[inline(always)]
        pub fn set_crcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "BDMA block reset"]
        #[inline(always)]
        pub const fn bdmarst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "BDMA block reset"]
        #[inline(always)]
        pub fn set_bdmarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ADC3 block reset"]
        #[inline(always)]
        pub const fn adc3rst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ADC3 block reset"]
        #[inline(always)]
        pub fn set_adc3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "HSEM block reset"]
        #[inline(always)]
        pub const fn hsemrst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "HSEM block reset"]
        #[inline(always)]
        pub fn set_hsemrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Ahb4rstr {
        #[inline(always)]
        fn default() -> Ahb4rstr {
            Ahb4rstr(0)
        }
    }
    impl core::fmt::Debug for Ahb4rstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb4rstr")
                .field("gpioarst", &self.gpioarst())
                .field("gpiobrst", &self.gpiobrst())
                .field("gpiocrst", &self.gpiocrst())
                .field("gpiodrst", &self.gpiodrst())
                .field("gpioerst", &self.gpioerst())
                .field("gpiofrst", &self.gpiofrst())
                .field("gpiogrst", &self.gpiogrst())
                .field("gpiohrst", &self.gpiohrst())
                .field("gpioirst", &self.gpioirst())
                .field("gpiojrst", &self.gpiojrst())
                .field("gpiokrst", &self.gpiokrst())
                .field("crcrst", &self.crcrst())
                .field("bdmarst", &self.bdmarst())
                .field("adc3rst", &self.adc3rst())
                .field("hsemrst", &self.hsemrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb4rstr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb4rstr {
                gpioarst: bool,
                gpiobrst: bool,
                gpiocrst: bool,
                gpiodrst: bool,
                gpioerst: bool,
                gpiofrst: bool,
                gpiogrst: bool,
                gpiohrst: bool,
                gpioirst: bool,
                gpiojrst: bool,
                gpiokrst: bool,
                crcrst: bool,
                bdmarst: bool,
                adc3rst: bool,
                hsemrst: bool,
            }
            let proxy = Ahb4rstr {
                gpioarst: self.gpioarst(),
                gpiobrst: self.gpiobrst(),
                gpiocrst: self.gpiocrst(),
                gpiodrst: self.gpiodrst(),
                gpioerst: self.gpioerst(),
                gpiofrst: self.gpiofrst(),
                gpiogrst: self.gpiogrst(),
                gpiohrst: self.gpiohrst(),
                gpioirst: self.gpioirst(),
                gpiojrst: self.gpiojrst(),
                gpiokrst: self.gpiokrst(),
                crcrst: self.crcrst(),
                bdmarst: self.bdmarst(),
                adc3rst: self.adc3rst(),
                hsemrst: self.hsemrst(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB1 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1henr(pub u32);
    impl Apb1henr {
        #[doc = "Clock Recovery System peripheral clock enable"]
        #[inline(always)]
        pub const fn crsen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clock Recovery System peripheral clock enable"]
        #[inline(always)]
        pub fn set_crsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SWPMI Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn swpmien(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SWPMI Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_swpmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "OPAMP peripheral clock enable"]
        #[inline(always)]
        pub const fn opampen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP peripheral clock enable"]
        #[inline(always)]
        pub fn set_opampen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MDIOS peripheral clock enable"]
        #[inline(always)]
        pub const fn mdiosen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MDIOS peripheral clock enable"]
        #[inline(always)]
        pub fn set_mdiosen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FDCAN Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn fdcanen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_fdcanen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "TIM23 block enable"]
        #[inline(always)]
        pub const fn tim23en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "TIM23 block enable"]
        #[inline(always)]
        pub fn set_tim23en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "TIM24 block enable"]
        #[inline(always)]
        pub const fn tim24en(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "TIM24 block enable"]
        #[inline(always)]
        pub fn set_tim24en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Apb1henr {
        #[inline(always)]
        fn default() -> Apb1henr {
            Apb1henr(0)
        }
    }
    impl core::fmt::Debug for Apb1henr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1henr")
                .field("crsen", &self.crsen())
                .field("swpmien", &self.swpmien())
                .field("opampen", &self.opampen())
                .field("mdiosen", &self.mdiosen())
                .field("fdcanen", &self.fdcanen())
                .field("tim23en", &self.tim23en())
                .field("tim24en", &self.tim24en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1henr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb1henr {
                crsen: bool,
                swpmien: bool,
                opampen: bool,
                mdiosen: bool,
                fdcanen: bool,
                tim23en: bool,
                tim24en: bool,
            }
            let proxy = Apb1henr {
                crsen: self.crsen(),
                swpmien: self.swpmien(),
                opampen: self.opampen(),
                mdiosen: self.mdiosen(),
                fdcanen: self.fdcanen(),
                tim23en: self.tim23en(),
                tim24en: self.tim24en(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB1 High Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1hlpenr(pub u32);
    impl Apb1hlpenr {
        #[doc = "Clock Recovery System peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn crslpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clock Recovery System peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_crslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SWPMI Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn swpmilpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SWPMI Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_swpmilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "OPAMP peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn opamplpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_opamplpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MDIOS peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn mdioslpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MDIOS peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_mdioslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FDCAN Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn fdcanlpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_fdcanlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "TIM23 block enable during CSleep Mode"]
        #[inline(always)]
        pub const fn tim23lpen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "TIM23 block enable during CSleep Mode"]
        #[inline(always)]
        pub fn set_tim23lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "TIM24 block enable during CSleep Mode"]
        #[inline(always)]
        pub const fn tim24lpen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "TIM24 block enable during CSleep Mode"]
        #[inline(always)]
        pub fn set_tim24lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Apb1hlpenr {
        #[inline(always)]
        fn default() -> Apb1hlpenr {
            Apb1hlpenr(0)
        }
    }
    impl core::fmt::Debug for Apb1hlpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1hlpenr")
                .field("crslpen", &self.crslpen())
                .field("swpmilpen", &self.swpmilpen())
                .field("opamplpen", &self.opamplpen())
                .field("mdioslpen", &self.mdioslpen())
                .field("fdcanlpen", &self.fdcanlpen())
                .field("tim23lpen", &self.tim23lpen())
                .field("tim24lpen", &self.tim24lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1hlpenr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb1hlpenr {
                crslpen: bool,
                swpmilpen: bool,
                opamplpen: bool,
                mdioslpen: bool,
                fdcanlpen: bool,
                tim23lpen: bool,
                tim24lpen: bool,
            }
            let proxy = Apb1hlpenr {
                crslpen: self.crslpen(),
                swpmilpen: self.swpmilpen(),
                opamplpen: self.opamplpen(),
                mdioslpen: self.mdioslpen(),
                fdcanlpen: self.fdcanlpen(),
                tim23lpen: self.tim23lpen(),
                tim24lpen: self.tim24lpen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB1 Peripheral Reset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1hrstr(pub u32);
    impl Apb1hrstr {
        #[doc = "Clock Recovery System reset"]
        #[inline(always)]
        pub const fn crsrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clock Recovery System reset"]
        #[inline(always)]
        pub fn set_crsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SWPMI block reset"]
        #[inline(always)]
        pub const fn swpmirst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SWPMI block reset"]
        #[inline(always)]
        pub fn set_swpmirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "OPAMP block reset"]
        #[inline(always)]
        pub const fn opamprst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP block reset"]
        #[inline(always)]
        pub fn set_opamprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MDIOS block reset"]
        #[inline(always)]
        pub const fn mdiosrst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MDIOS block reset"]
        #[inline(always)]
        pub fn set_mdiosrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FDCAN block reset"]
        #[inline(always)]
        pub const fn fdcanrst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN block reset"]
        #[inline(always)]
        pub fn set_fdcanrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "TIM23 block reset"]
        #[inline(always)]
        pub const fn tim23rst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "TIM23 block reset"]
        #[inline(always)]
        pub fn set_tim23rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "TIM24 block reset"]
        #[inline(always)]
        pub const fn tim24rst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "TIM24 block reset"]
        #[inline(always)]
        pub fn set_tim24rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Apb1hrstr {
        #[inline(always)]
        fn default() -> Apb1hrstr {
            Apb1hrstr(0)
        }
    }
    impl core::fmt::Debug for Apb1hrstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1hrstr")
                .field("crsrst", &self.crsrst())
                .field("swpmirst", &self.swpmirst())
                .field("opamprst", &self.opamprst())
                .field("mdiosrst", &self.mdiosrst())
                .field("fdcanrst", &self.fdcanrst())
                .field("tim23rst", &self.tim23rst())
                .field("tim24rst", &self.tim24rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1hrstr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb1hrstr {
                crsrst: bool,
                swpmirst: bool,
                opamprst: bool,
                mdiosrst: bool,
                fdcanrst: bool,
                tim23rst: bool,
                tim24rst: bool,
            }
            let proxy = Apb1hrstr {
                crsrst: self.crsrst(),
                swpmirst: self.swpmirst(),
                opamprst: self.opamprst(),
                mdiosrst: self.mdiosrst(),
                fdcanrst: self.fdcanrst(),
                tim23rst: self.tim23rst(),
                tim24rst: self.tim24rst(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB1 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1lenr(pub u32);
    impl Apb1lenr {
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn tim2en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn tim3en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn tim4en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn tim5en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn tim6en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn tim7en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn tim12en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn tim13en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim13en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn tim14en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim14en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LPTIM1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn lptim1en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_lptim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "WWDG2 peripheral clock enable"]
        #[inline(always)]
        pub const fn wwdg2en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG2 peripheral clock enable"]
        #[inline(always)]
        pub fn set_wwdg2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn spi2en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_spi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn spi3en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_spi3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "SPDIFRX Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn spdifrxen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SPDIFRX Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_spdifrxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "USART2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn usart2en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_usart2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn usart3en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_usart3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn uart4en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_uart4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn uart5en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_uart5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn i2c1en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_i2c1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn i2c2en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_i2c2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I2C3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn i2c3en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_i2c3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "I2C5 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn i2c5en(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "I2C5 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_i2c5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "HDMI-CEC peripheral clock enable"]
        #[inline(always)]
        pub const fn cecen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "HDMI-CEC peripheral clock enable"]
        #[inline(always)]
        pub fn set_cecen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "DAC1&2 peripheral clock enable"]
        #[inline(always)]
        pub const fn dac12en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1&2 peripheral clock enable"]
        #[inline(always)]
        pub fn set_dac12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "UART7 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn uart7en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "UART7 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_uart7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "UART8 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn uart8en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "UART8 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_uart8en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Apb1lenr {
        #[inline(always)]
        fn default() -> Apb1lenr {
            Apb1lenr(0)
        }
    }
    impl core::fmt::Debug for Apb1lenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1lenr")
                .field("tim2en", &self.tim2en())
                .field("tim3en", &self.tim3en())
                .field("tim4en", &self.tim4en())
                .field("tim5en", &self.tim5en())
                .field("tim6en", &self.tim6en())
                .field("tim7en", &self.tim7en())
                .field("tim12en", &self.tim12en())
                .field("tim13en", &self.tim13en())
                .field("tim14en", &self.tim14en())
                .field("lptim1en", &self.lptim1en())
                .field("wwdg2en", &self.wwdg2en())
                .field("spi2en", &self.spi2en())
                .field("spi3en", &self.spi3en())
                .field("spdifrxen", &self.spdifrxen())
                .field("usart2en", &self.usart2en())
                .field("usart3en", &self.usart3en())
                .field("uart4en", &self.uart4en())
                .field("uart5en", &self.uart5en())
                .field("i2c1en", &self.i2c1en())
                .field("i2c2en", &self.i2c2en())
                .field("i2c3en", &self.i2c3en())
                .field("i2c5en", &self.i2c5en())
                .field("cecen", &self.cecen())
                .field("dac12en", &self.dac12en())
                .field("uart7en", &self.uart7en())
                .field("uart8en", &self.uart8en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1lenr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb1lenr {
                tim2en: bool,
                tim3en: bool,
                tim4en: bool,
                tim5en: bool,
                tim6en: bool,
                tim7en: bool,
                tim12en: bool,
                tim13en: bool,
                tim14en: bool,
                lptim1en: bool,
                wwdg2en: bool,
                spi2en: bool,
                spi3en: bool,
                spdifrxen: bool,
                usart2en: bool,
                usart3en: bool,
                uart4en: bool,
                uart5en: bool,
                i2c1en: bool,
                i2c2en: bool,
                i2c3en: bool,
                i2c5en: bool,
                cecen: bool,
                dac12en: bool,
                uart7en: bool,
                uart8en: bool,
            }
            let proxy = Apb1lenr {
                tim2en: self.tim2en(),
                tim3en: self.tim3en(),
                tim4en: self.tim4en(),
                tim5en: self.tim5en(),
                tim6en: self.tim6en(),
                tim7en: self.tim7en(),
                tim12en: self.tim12en(),
                tim13en: self.tim13en(),
                tim14en: self.tim14en(),
                lptim1en: self.lptim1en(),
                wwdg2en: self.wwdg2en(),
                spi2en: self.spi2en(),
                spi3en: self.spi3en(),
                spdifrxen: self.spdifrxen(),
                usart2en: self.usart2en(),
                usart3en: self.usart3en(),
                uart4en: self.uart4en(),
                uart5en: self.uart5en(),
                i2c1en: self.i2c1en(),
                i2c2en: self.i2c2en(),
                i2c3en: self.i2c3en(),
                i2c5en: self.i2c5en(),
                cecen: self.cecen(),
                dac12en: self.dac12en(),
                uart7en: self.uart7en(),
                uart8en: self.uart8en(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB1 Low Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1llpenr(pub u32);
    impl Apb1llpenr {
        #[doc = "TIM2 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim2lpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim3lpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim4lpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim5lpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim6lpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim6lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim7lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim7lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM12 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim12lpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim12lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIM13 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim13lpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIM13 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim13lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIM14 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim14lpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM14 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim14lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LPTIM1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn lptim1lpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_lptim1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "WWDG2 peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn wwdg2lpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG2 peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_wwdg2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn spi2lpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_spi2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn spi3lpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_spi3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "SPDIFRX Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn spdifrxlpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SPDIFRX Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_spdifrxlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "USART2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn usart2lpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_usart2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn usart3lpen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_usart3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn uart4lpen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_uart4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn uart5lpen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_uart5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn i2c1lpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_i2c1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn i2c2lpen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_i2c2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I2C3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn i2c3lpen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_i2c3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "I2C5 block enable during CSleep Mode"]
        #[inline(always)]
        pub const fn i2c5lpen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "I2C5 block enable during CSleep Mode"]
        #[inline(always)]
        pub fn set_i2c5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "HDMI-CEC Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn ceclpen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "HDMI-CEC Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_ceclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "DAC1/2 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn dac12lpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1/2 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_dac12lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "UART7 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn uart7lpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "UART7 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_uart7lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "UART8 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn uart8lpen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "UART8 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_uart8lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Apb1llpenr {
        #[inline(always)]
        fn default() -> Apb1llpenr {
            Apb1llpenr(0)
        }
    }
    impl core::fmt::Debug for Apb1llpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1llpenr")
                .field("tim2lpen", &self.tim2lpen())
                .field("tim3lpen", &self.tim3lpen())
                .field("tim4lpen", &self.tim4lpen())
                .field("tim5lpen", &self.tim5lpen())
                .field("tim6lpen", &self.tim6lpen())
                .field("tim7lpen", &self.tim7lpen())
                .field("tim12lpen", &self.tim12lpen())
                .field("tim13lpen", &self.tim13lpen())
                .field("tim14lpen", &self.tim14lpen())
                .field("lptim1lpen", &self.lptim1lpen())
                .field("wwdg2lpen", &self.wwdg2lpen())
                .field("spi2lpen", &self.spi2lpen())
                .field("spi3lpen", &self.spi3lpen())
                .field("spdifrxlpen", &self.spdifrxlpen())
                .field("usart2lpen", &self.usart2lpen())
                .field("usart3lpen", &self.usart3lpen())
                .field("uart4lpen", &self.uart4lpen())
                .field("uart5lpen", &self.uart5lpen())
                .field("i2c1lpen", &self.i2c1lpen())
                .field("i2c2lpen", &self.i2c2lpen())
                .field("i2c3lpen", &self.i2c3lpen())
                .field("i2c5lpen", &self.i2c5lpen())
                .field("ceclpen", &self.ceclpen())
                .field("dac12lpen", &self.dac12lpen())
                .field("uart7lpen", &self.uart7lpen())
                .field("uart8lpen", &self.uart8lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1llpenr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb1llpenr {
                tim2lpen: bool,
                tim3lpen: bool,
                tim4lpen: bool,
                tim5lpen: bool,
                tim6lpen: bool,
                tim7lpen: bool,
                tim12lpen: bool,
                tim13lpen: bool,
                tim14lpen: bool,
                lptim1lpen: bool,
                wwdg2lpen: bool,
                spi2lpen: bool,
                spi3lpen: bool,
                spdifrxlpen: bool,
                usart2lpen: bool,
                usart3lpen: bool,
                uart4lpen: bool,
                uart5lpen: bool,
                i2c1lpen: bool,
                i2c2lpen: bool,
                i2c3lpen: bool,
                i2c5lpen: bool,
                ceclpen: bool,
                dac12lpen: bool,
                uart7lpen: bool,
                uart8lpen: bool,
            }
            let proxy = Apb1llpenr {
                tim2lpen: self.tim2lpen(),
                tim3lpen: self.tim3lpen(),
                tim4lpen: self.tim4lpen(),
                tim5lpen: self.tim5lpen(),
                tim6lpen: self.tim6lpen(),
                tim7lpen: self.tim7lpen(),
                tim12lpen: self.tim12lpen(),
                tim13lpen: self.tim13lpen(),
                tim14lpen: self.tim14lpen(),
                lptim1lpen: self.lptim1lpen(),
                wwdg2lpen: self.wwdg2lpen(),
                spi2lpen: self.spi2lpen(),
                spi3lpen: self.spi3lpen(),
                spdifrxlpen: self.spdifrxlpen(),
                usart2lpen: self.usart2lpen(),
                usart3lpen: self.usart3lpen(),
                uart4lpen: self.uart4lpen(),
                uart5lpen: self.uart5lpen(),
                i2c1lpen: self.i2c1lpen(),
                i2c2lpen: self.i2c2lpen(),
                i2c3lpen: self.i2c3lpen(),
                i2c5lpen: self.i2c5lpen(),
                ceclpen: self.ceclpen(),
                dac12lpen: self.dac12lpen(),
                uart7lpen: self.uart7lpen(),
                uart8lpen: self.uart8lpen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB1 Peripheral Reset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1lrstr(pub u32);
    impl Apb1lrstr {
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub const fn tim2rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub fn set_tim2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub const fn tim3rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub fn set_tim3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub const fn tim4rst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub fn set_tim4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub const fn tim5rst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub fn set_tim5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub const fn tim6rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub fn set_tim6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub const fn tim7rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub fn set_tim7rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub const fn tim12rst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub fn set_tim12rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub const fn tim13rst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub fn set_tim13rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub const fn tim14rst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub fn set_tim14rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub const fn lptim1rst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub fn set_lptim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "SPI2 block reset"]
        #[inline(always)]
        pub const fn spi2rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 block reset"]
        #[inline(always)]
        pub fn set_spi2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 block reset"]
        #[inline(always)]
        pub const fn spi3rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 block reset"]
        #[inline(always)]
        pub fn set_spi3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "SPDIFRX block reset"]
        #[inline(always)]
        pub const fn spdifrxrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SPDIFRX block reset"]
        #[inline(always)]
        pub fn set_spdifrxrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "USART2 block reset"]
        #[inline(always)]
        pub const fn usart2rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 block reset"]
        #[inline(always)]
        pub fn set_usart2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 block reset"]
        #[inline(always)]
        pub const fn usart3rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 block reset"]
        #[inline(always)]
        pub fn set_usart3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 block reset"]
        #[inline(always)]
        pub const fn uart4rst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 block reset"]
        #[inline(always)]
        pub fn set_uart4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 block reset"]
        #[inline(always)]
        pub const fn uart5rst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 block reset"]
        #[inline(always)]
        pub fn set_uart5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 block reset"]
        #[inline(always)]
        pub const fn i2c1rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 block reset"]
        #[inline(always)]
        pub fn set_i2c1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 block reset"]
        #[inline(always)]
        pub const fn i2c2rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 block reset"]
        #[inline(always)]
        pub fn set_i2c2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I2C3 block reset"]
        #[inline(always)]
        pub const fn i2c3rst(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 block reset"]
        #[inline(always)]
        pub fn set_i2c3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "I2C5 block reset"]
        #[inline(always)]
        pub const fn i2c5rst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "I2C5 block reset"]
        #[inline(always)]
        pub fn set_i2c5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "HDMI-CEC block reset"]
        #[inline(always)]
        pub const fn cecrst(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "HDMI-CEC block reset"]
        #[inline(always)]
        pub fn set_cecrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "DAC1 and 2 Blocks Reset"]
        #[inline(always)]
        pub const fn dac12rst(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1 and 2 Blocks Reset"]
        #[inline(always)]
        pub fn set_dac12rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "UART7 block reset"]
        #[inline(always)]
        pub const fn uart7rst(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "UART7 block reset"]
        #[inline(always)]
        pub fn set_uart7rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "UART8 block reset"]
        #[inline(always)]
        pub const fn uart8rst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "UART8 block reset"]
        #[inline(always)]
        pub fn set_uart8rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Apb1lrstr {
        #[inline(always)]
        fn default() -> Apb1lrstr {
            Apb1lrstr(0)
        }
    }
    impl core::fmt::Debug for Apb1lrstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1lrstr")
                .field("tim2rst", &self.tim2rst())
                .field("tim3rst", &self.tim3rst())
                .field("tim4rst", &self.tim4rst())
                .field("tim5rst", &self.tim5rst())
                .field("tim6rst", &self.tim6rst())
                .field("tim7rst", &self.tim7rst())
                .field("tim12rst", &self.tim12rst())
                .field("tim13rst", &self.tim13rst())
                .field("tim14rst", &self.tim14rst())
                .field("lptim1rst", &self.lptim1rst())
                .field("spi2rst", &self.spi2rst())
                .field("spi3rst", &self.spi3rst())
                .field("spdifrxrst", &self.spdifrxrst())
                .field("usart2rst", &self.usart2rst())
                .field("usart3rst", &self.usart3rst())
                .field("uart4rst", &self.uart4rst())
                .field("uart5rst", &self.uart5rst())
                .field("i2c1rst", &self.i2c1rst())
                .field("i2c2rst", &self.i2c2rst())
                .field("i2c3rst", &self.i2c3rst())
                .field("i2c5rst", &self.i2c5rst())
                .field("cecrst", &self.cecrst())
                .field("dac12rst", &self.dac12rst())
                .field("uart7rst", &self.uart7rst())
                .field("uart8rst", &self.uart8rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1lrstr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb1lrstr {
                tim2rst: bool,
                tim3rst: bool,
                tim4rst: bool,
                tim5rst: bool,
                tim6rst: bool,
                tim7rst: bool,
                tim12rst: bool,
                tim13rst: bool,
                tim14rst: bool,
                lptim1rst: bool,
                spi2rst: bool,
                spi3rst: bool,
                spdifrxrst: bool,
                usart2rst: bool,
                usart3rst: bool,
                uart4rst: bool,
                uart5rst: bool,
                i2c1rst: bool,
                i2c2rst: bool,
                i2c3rst: bool,
                i2c5rst: bool,
                cecrst: bool,
                dac12rst: bool,
                uart7rst: bool,
                uart8rst: bool,
            }
            let proxy = Apb1lrstr {
                tim2rst: self.tim2rst(),
                tim3rst: self.tim3rst(),
                tim4rst: self.tim4rst(),
                tim5rst: self.tim5rst(),
                tim6rst: self.tim6rst(),
                tim7rst: self.tim7rst(),
                tim12rst: self.tim12rst(),
                tim13rst: self.tim13rst(),
                tim14rst: self.tim14rst(),
                lptim1rst: self.lptim1rst(),
                spi2rst: self.spi2rst(),
                spi3rst: self.spi3rst(),
                spdifrxrst: self.spdifrxrst(),
                usart2rst: self.usart2rst(),
                usart3rst: self.usart3rst(),
                uart4rst: self.uart4rst(),
                uart5rst: self.uart5rst(),
                i2c1rst: self.i2c1rst(),
                i2c2rst: self.i2c2rst(),
                i2c3rst: self.i2c3rst(),
                i2c5rst: self.i2c5rst(),
                cecrst: self.cecrst(),
                dac12rst: self.dac12rst(),
                uart7rst: self.uart7rst(),
                uart8rst: self.uart8rst(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB2 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2enr(pub u32);
    impl Apb2enr {
        #[doc = "TIM1 peripheral clock enable"]
        #[inline(always)]
        pub const fn tim1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM8 peripheral clock enable"]
        #[inline(always)]
        pub const fn tim8en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim8en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "USART1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn usart1en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_usart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "USART6 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn usart6en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USART6 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_usart6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "UART9 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn uart9en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "UART9 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_uart9en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "USART10 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn usart10en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "USART10 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_usart10en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "SPI1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn spi1en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_spi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SPI4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn spi4en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "SPI4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_spi4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "TIM15 peripheral clock enable"]
        #[inline(always)]
        pub const fn tim15en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim15en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 peripheral clock enable"]
        #[inline(always)]
        pub const fn tim16en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim16en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 peripheral clock enable"]
        #[inline(always)]
        pub const fn tim17en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim17en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SPI5 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn spi5en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SPI5 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_spi5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "SAI1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn sai1en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_sai1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "SAI2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn sai2en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "SAI2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_sai2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "SAI3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn sai3en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "SAI3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_sai3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "DFSDM1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn dfsdm1en(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "DFSDM1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_dfsdm1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "HRTIM peripheral clock enable"]
        #[inline(always)]
        pub const fn hrtimen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM peripheral clock enable"]
        #[inline(always)]
        pub fn set_hrtimen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Apb2enr {
        #[inline(always)]
        fn default() -> Apb2enr {
            Apb2enr(0)
        }
    }
    impl core::fmt::Debug for Apb2enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2enr")
                .field("tim1en", &self.tim1en())
                .field("tim8en", &self.tim8en())
                .field("usart1en", &self.usart1en())
                .field("usart6en", &self.usart6en())
                .field("uart9en", &self.uart9en())
                .field("usart10en", &self.usart10en())
                .field("spi1en", &self.spi1en())
                .field("spi4en", &self.spi4en())
                .field("tim15en", &self.tim15en())
                .field("tim16en", &self.tim16en())
                .field("tim17en", &self.tim17en())
                .field("spi5en", &self.spi5en())
                .field("sai1en", &self.sai1en())
                .field("sai2en", &self.sai2en())
                .field("sai3en", &self.sai3en())
                .field("dfsdm1en", &self.dfsdm1en())
                .field("hrtimen", &self.hrtimen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2enr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb2enr {
                tim1en: bool,
                tim8en: bool,
                usart1en: bool,
                usart6en: bool,
                uart9en: bool,
                usart10en: bool,
                spi1en: bool,
                spi4en: bool,
                tim15en: bool,
                tim16en: bool,
                tim17en: bool,
                spi5en: bool,
                sai1en: bool,
                sai2en: bool,
                sai3en: bool,
                dfsdm1en: bool,
                hrtimen: bool,
            }
            let proxy = Apb2enr {
                tim1en: self.tim1en(),
                tim8en: self.tim8en(),
                usart1en: self.usart1en(),
                usart6en: self.usart6en(),
                uart9en: self.uart9en(),
                usart10en: self.usart10en(),
                spi1en: self.spi1en(),
                spi4en: self.spi4en(),
                tim15en: self.tim15en(),
                tim16en: self.tim16en(),
                tim17en: self.tim17en(),
                spi5en: self.spi5en(),
                sai1en: self.sai1en(),
                sai2en: self.sai2en(),
                sai3en: self.sai3en(),
                dfsdm1en: self.dfsdm1en(),
                hrtimen: self.hrtimen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB2 Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2lpenr(pub u32);
    impl Apb2lpenr {
        #[doc = "TIM1 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim1lpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM8 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim8lpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim8lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "USART1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn usart1lpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_usart1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "USART6 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn usart6lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USART6 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_usart6lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SPI1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn spi1lpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_spi1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SPI4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn spi4lpen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "SPI4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_spi4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "TIM15 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim15lpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim15lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim16lpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim16lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim17lpen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim17lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SPI5 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn spi5lpen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SPI5 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_spi5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "SAI1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn sai1lpen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_sai1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "SAI2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn sai2lpen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "SAI2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_sai2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "SAI3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn sai3lpen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "SAI3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_sai3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "DFSDM1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn dfsdm1lpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "DFSDM1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_dfsdm1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "HRTIM peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn hrtimlpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_hrtimlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Apb2lpenr {
        #[inline(always)]
        fn default() -> Apb2lpenr {
            Apb2lpenr(0)
        }
    }
    impl core::fmt::Debug for Apb2lpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2lpenr")
                .field("tim1lpen", &self.tim1lpen())
                .field("tim8lpen", &self.tim8lpen())
                .field("usart1lpen", &self.usart1lpen())
                .field("usart6lpen", &self.usart6lpen())
                .field("spi1lpen", &self.spi1lpen())
                .field("spi4lpen", &self.spi4lpen())
                .field("tim15lpen", &self.tim15lpen())
                .field("tim16lpen", &self.tim16lpen())
                .field("tim17lpen", &self.tim17lpen())
                .field("spi5lpen", &self.spi5lpen())
                .field("sai1lpen", &self.sai1lpen())
                .field("sai2lpen", &self.sai2lpen())
                .field("sai3lpen", &self.sai3lpen())
                .field("dfsdm1lpen", &self.dfsdm1lpen())
                .field("hrtimlpen", &self.hrtimlpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2lpenr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb2lpenr {
                tim1lpen: bool,
                tim8lpen: bool,
                usart1lpen: bool,
                usart6lpen: bool,
                spi1lpen: bool,
                spi4lpen: bool,
                tim15lpen: bool,
                tim16lpen: bool,
                tim17lpen: bool,
                spi5lpen: bool,
                sai1lpen: bool,
                sai2lpen: bool,
                sai3lpen: bool,
                dfsdm1lpen: bool,
                hrtimlpen: bool,
            }
            let proxy = Apb2lpenr {
                tim1lpen: self.tim1lpen(),
                tim8lpen: self.tim8lpen(),
                usart1lpen: self.usart1lpen(),
                usart6lpen: self.usart6lpen(),
                spi1lpen: self.spi1lpen(),
                spi4lpen: self.spi4lpen(),
                tim15lpen: self.tim15lpen(),
                tim16lpen: self.tim16lpen(),
                tim17lpen: self.tim17lpen(),
                spi5lpen: self.spi5lpen(),
                sai1lpen: self.sai1lpen(),
                sai2lpen: self.sai2lpen(),
                sai3lpen: self.sai3lpen(),
                dfsdm1lpen: self.dfsdm1lpen(),
                hrtimlpen: self.hrtimlpen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB2 Peripheral Reset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2rstr(pub u32);
    impl Apb2rstr {
        #[doc = "TIM1 block reset"]
        #[inline(always)]
        pub const fn tim1rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 block reset"]
        #[inline(always)]
        pub fn set_tim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM8 block reset"]
        #[inline(always)]
        pub const fn tim8rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 block reset"]
        #[inline(always)]
        pub fn set_tim8rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "USART1 block reset"]
        #[inline(always)]
        pub const fn usart1rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 block reset"]
        #[inline(always)]
        pub fn set_usart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "USART6 block reset"]
        #[inline(always)]
        pub const fn usart6rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USART6 block reset"]
        #[inline(always)]
        pub fn set_usart6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "UART9 block reset"]
        #[inline(always)]
        pub const fn uart9rst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "UART9 block reset"]
        #[inline(always)]
        pub fn set_uart9rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "USART10 block reset"]
        #[inline(always)]
        pub const fn usart10rst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "USART10 block reset"]
        #[inline(always)]
        pub fn set_usart10rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "SPI1 block reset"]
        #[inline(always)]
        pub const fn spi1rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 block reset"]
        #[inline(always)]
        pub fn set_spi1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SPI4 block reset"]
        #[inline(always)]
        pub const fn spi4rst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "SPI4 block reset"]
        #[inline(always)]
        pub fn set_spi4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "TIM15 block reset"]
        #[inline(always)]
        pub const fn tim15rst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 block reset"]
        #[inline(always)]
        pub fn set_tim15rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 block reset"]
        #[inline(always)]
        pub const fn tim16rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 block reset"]
        #[inline(always)]
        pub fn set_tim16rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 block reset"]
        #[inline(always)]
        pub const fn tim17rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 block reset"]
        #[inline(always)]
        pub fn set_tim17rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SPI5 block reset"]
        #[inline(always)]
        pub const fn spi5rst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SPI5 block reset"]
        #[inline(always)]
        pub fn set_spi5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "SAI1 block reset"]
        #[inline(always)]
        pub const fn sai1rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 block reset"]
        #[inline(always)]
        pub fn set_sai1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "SAI2 block reset"]
        #[inline(always)]
        pub const fn sai2rst(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "SAI2 block reset"]
        #[inline(always)]
        pub fn set_sai2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "SAI3 block reset"]
        #[inline(always)]
        pub const fn sai3rst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "SAI3 block reset"]
        #[inline(always)]
        pub fn set_sai3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "DFSDM1 block reset"]
        #[inline(always)]
        pub const fn dfsdm1rst(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "DFSDM1 block reset"]
        #[inline(always)]
        pub fn set_dfsdm1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "HRTIM block reset"]
        #[inline(always)]
        pub const fn hrtimrst(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM block reset"]
        #[inline(always)]
        pub fn set_hrtimrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Apb2rstr {
        #[inline(always)]
        fn default() -> Apb2rstr {
            Apb2rstr(0)
        }
    }
    impl core::fmt::Debug for Apb2rstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2rstr")
                .field("tim1rst", &self.tim1rst())
                .field("tim8rst", &self.tim8rst())
                .field("usart1rst", &self.usart1rst())
                .field("usart6rst", &self.usart6rst())
                .field("uart9rst", &self.uart9rst())
                .field("usart10rst", &self.usart10rst())
                .field("spi1rst", &self.spi1rst())
                .field("spi4rst", &self.spi4rst())
                .field("tim15rst", &self.tim15rst())
                .field("tim16rst", &self.tim16rst())
                .field("tim17rst", &self.tim17rst())
                .field("spi5rst", &self.spi5rst())
                .field("sai1rst", &self.sai1rst())
                .field("sai2rst", &self.sai2rst())
                .field("sai3rst", &self.sai3rst())
                .field("dfsdm1rst", &self.dfsdm1rst())
                .field("hrtimrst", &self.hrtimrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2rstr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb2rstr {
                tim1rst: bool,
                tim8rst: bool,
                usart1rst: bool,
                usart6rst: bool,
                uart9rst: bool,
                usart10rst: bool,
                spi1rst: bool,
                spi4rst: bool,
                tim15rst: bool,
                tim16rst: bool,
                tim17rst: bool,
                spi5rst: bool,
                sai1rst: bool,
                sai2rst: bool,
                sai3rst: bool,
                dfsdm1rst: bool,
                hrtimrst: bool,
            }
            let proxy = Apb2rstr {
                tim1rst: self.tim1rst(),
                tim8rst: self.tim8rst(),
                usart1rst: self.usart1rst(),
                usart6rst: self.usart6rst(),
                uart9rst: self.uart9rst(),
                usart10rst: self.usart10rst(),
                spi1rst: self.spi1rst(),
                spi4rst: self.spi4rst(),
                tim15rst: self.tim15rst(),
                tim16rst: self.tim16rst(),
                tim17rst: self.tim17rst(),
                spi5rst: self.spi5rst(),
                sai1rst: self.sai1rst(),
                sai2rst: self.sai2rst(),
                sai3rst: self.sai3rst(),
                dfsdm1rst: self.dfsdm1rst(),
                hrtimrst: self.hrtimrst(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB3 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb3enr(pub u32);
    impl Apb3enr {
        #[doc = "LTDC peripheral clock enable"]
        #[inline(always)]
        pub const fn ltdcen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC peripheral clock enable"]
        #[inline(always)]
        pub fn set_ltdcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DSI Peripheral clocks enable"]
        #[inline(always)]
        pub const fn dsien(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DSI Peripheral clocks enable"]
        #[inline(always)]
        pub fn set_dsien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "WWDG1 Clock Enable"]
        #[inline(always)]
        pub const fn wwdg1en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG1 Clock Enable"]
        #[inline(always)]
        pub fn set_wwdg1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Apb3enr {
        #[inline(always)]
        fn default() -> Apb3enr {
            Apb3enr(0)
        }
    }
    impl core::fmt::Debug for Apb3enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb3enr")
                .field("ltdcen", &self.ltdcen())
                .field("dsien", &self.dsien())
                .field("wwdg1en", &self.wwdg1en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb3enr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb3enr {
                ltdcen: bool,
                dsien: bool,
                wwdg1en: bool,
            }
            let proxy = Apb3enr {
                ltdcen: self.ltdcen(),
                dsien: self.dsien(),
                wwdg1en: self.wwdg1en(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB3 Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb3lpenr(pub u32);
    impl Apb3lpenr {
        #[doc = "LTDC peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn ltdclpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_ltdclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DSI Peripheral Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn dsilpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DSI Peripheral Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_dsilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "WWDG1 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn wwdg1lpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG1 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_wwdg1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Apb3lpenr {
        #[inline(always)]
        fn default() -> Apb3lpenr {
            Apb3lpenr(0)
        }
    }
    impl core::fmt::Debug for Apb3lpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb3lpenr")
                .field("ltdclpen", &self.ltdclpen())
                .field("dsilpen", &self.dsilpen())
                .field("wwdg1lpen", &self.wwdg1lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb3lpenr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb3lpenr {
                ltdclpen: bool,
                dsilpen: bool,
                wwdg1lpen: bool,
            }
            let proxy = Apb3lpenr {
                ltdclpen: self.ltdclpen(),
                dsilpen: self.dsilpen(),
                wwdg1lpen: self.wwdg1lpen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB3 Peripheral Reset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb3rstr(pub u32);
    impl Apb3rstr {
        #[doc = "LTDC block reset"]
        #[inline(always)]
        pub const fn ltdcrst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC block reset"]
        #[inline(always)]
        pub fn set_ltdcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DSI block reset"]
        #[inline(always)]
        pub const fn dsirst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DSI block reset"]
        #[inline(always)]
        pub fn set_dsirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Apb3rstr {
        #[inline(always)]
        fn default() -> Apb3rstr {
            Apb3rstr(0)
        }
    }
    impl core::fmt::Debug for Apb3rstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb3rstr")
                .field("ltdcrst", &self.ltdcrst())
                .field("dsirst", &self.dsirst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb3rstr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb3rstr {
                ltdcrst: bool,
                dsirst: bool,
            }
            let proxy = Apb3rstr {
                ltdcrst: self.ltdcrst(),
                dsirst: self.dsirst(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB4 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb4enr(pub u32);
    impl Apb4enr {
        #[doc = "SYSCFG peripheral clock enable"]
        #[inline(always)]
        pub const fn syscfgen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG peripheral clock enable"]
        #[inline(always)]
        pub fn set_syscfgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPUART1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn lpuart1en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_lpuart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SPI6 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn spi6en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI6 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_spi6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "I2C4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn i2c4en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_i2c4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LPTIM2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn lptim2en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_lptim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LPTIM3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn lptim3en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_lptim3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LPTIM4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn lptim4en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_lptim4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIM5 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn lptim5en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM5 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_lptim5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "DAC2 (containing one converter) peripheral clock enable"]
        #[inline(always)]
        pub const fn dac2en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "DAC2 (containing one converter) peripheral clock enable"]
        #[inline(always)]
        pub fn set_dac2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "COMP1/2 peripheral clock enable"]
        #[inline(always)]
        pub const fn comp12en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "COMP1/2 peripheral clock enable"]
        #[inline(always)]
        pub fn set_comp12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "VREF peripheral clock enable"]
        #[inline(always)]
        pub const fn vrefen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "VREF peripheral clock enable"]
        #[inline(always)]
        pub fn set_vrefen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "RTC APB Clock Enable"]
        #[inline(always)]
        pub const fn rtcapben(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "RTC APB Clock Enable"]
        #[inline(always)]
        pub fn set_rtcapben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "SAI4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn sai4en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SAI4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_sai4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Digital temperature sensor block enable"]
        #[inline(always)]
        pub const fn dtsen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Digital temperature sensor block enable"]
        #[inline(always)]
        pub fn set_dtsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Apb4enr {
        #[inline(always)]
        fn default() -> Apb4enr {
            Apb4enr(0)
        }
    }
    impl core::fmt::Debug for Apb4enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb4enr")
                .field("syscfgen", &self.syscfgen())
                .field("lpuart1en", &self.lpuart1en())
                .field("spi6en", &self.spi6en())
                .field("i2c4en", &self.i2c4en())
                .field("lptim2en", &self.lptim2en())
                .field("lptim3en", &self.lptim3en())
                .field("lptim4en", &self.lptim4en())
                .field("lptim5en", &self.lptim5en())
                .field("dac2en", &self.dac2en())
                .field("comp12en", &self.comp12en())
                .field("vrefen", &self.vrefen())
                .field("rtcapben", &self.rtcapben())
                .field("sai4en", &self.sai4en())
                .field("dtsen", &self.dtsen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb4enr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb4enr {
                syscfgen: bool,
                lpuart1en: bool,
                spi6en: bool,
                i2c4en: bool,
                lptim2en: bool,
                lptim3en: bool,
                lptim4en: bool,
                lptim5en: bool,
                dac2en: bool,
                comp12en: bool,
                vrefen: bool,
                rtcapben: bool,
                sai4en: bool,
                dtsen: bool,
            }
            let proxy = Apb4enr {
                syscfgen: self.syscfgen(),
                lpuart1en: self.lpuart1en(),
                spi6en: self.spi6en(),
                i2c4en: self.i2c4en(),
                lptim2en: self.lptim2en(),
                lptim3en: self.lptim3en(),
                lptim4en: self.lptim4en(),
                lptim5en: self.lptim5en(),
                dac2en: self.dac2en(),
                comp12en: self.comp12en(),
                vrefen: self.vrefen(),
                rtcapben: self.rtcapben(),
                sai4en: self.sai4en(),
                dtsen: self.dtsen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB4 Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb4lpenr(pub u32);
    impl Apb4lpenr {
        #[doc = "SYSCFG peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn syscfglpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_syscfglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPUART1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn lpuart1lpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_lpuart1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SPI6 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn spi6lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI6 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_spi6lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "I2C4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn i2c4lpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_i2c4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LPTIM2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn lptim2lpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_lptim2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LPTIM3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn lptim3lpen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_lptim3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LPTIM4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn lptim4lpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_lptim4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIM5 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn lptim5lpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM5 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_lptim5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "DAC2 (containing one converter) peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn dac2lpen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "DAC2 (containing one converter) peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_dac2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "COMP1/2 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn comp12lpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "COMP1/2 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_comp12lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "VREF peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn vreflpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "VREF peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_vreflpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "RTC APB Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn rtcapblpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "RTC APB Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_rtcapblpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "SAI4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn sai4lpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SAI4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_sai4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Digital temperature sensor block enable during CSleep Mode"]
        #[inline(always)]
        pub const fn dtslpen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Digital temperature sensor block enable during CSleep Mode"]
        #[inline(always)]
        pub fn set_dtslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Apb4lpenr {
        #[inline(always)]
        fn default() -> Apb4lpenr {
            Apb4lpenr(0)
        }
    }
    impl core::fmt::Debug for Apb4lpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb4lpenr")
                .field("syscfglpen", &self.syscfglpen())
                .field("lpuart1lpen", &self.lpuart1lpen())
                .field("spi6lpen", &self.spi6lpen())
                .field("i2c4lpen", &self.i2c4lpen())
                .field("lptim2lpen", &self.lptim2lpen())
                .field("lptim3lpen", &self.lptim3lpen())
                .field("lptim4lpen", &self.lptim4lpen())
                .field("lptim5lpen", &self.lptim5lpen())
                .field("dac2lpen", &self.dac2lpen())
                .field("comp12lpen", &self.comp12lpen())
                .field("vreflpen", &self.vreflpen())
                .field("rtcapblpen", &self.rtcapblpen())
                .field("sai4lpen", &self.sai4lpen())
                .field("dtslpen", &self.dtslpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb4lpenr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb4lpenr {
                syscfglpen: bool,
                lpuart1lpen: bool,
                spi6lpen: bool,
                i2c4lpen: bool,
                lptim2lpen: bool,
                lptim3lpen: bool,
                lptim4lpen: bool,
                lptim5lpen: bool,
                dac2lpen: bool,
                comp12lpen: bool,
                vreflpen: bool,
                rtcapblpen: bool,
                sai4lpen: bool,
                dtslpen: bool,
            }
            let proxy = Apb4lpenr {
                syscfglpen: self.syscfglpen(),
                lpuart1lpen: self.lpuart1lpen(),
                spi6lpen: self.spi6lpen(),
                i2c4lpen: self.i2c4lpen(),
                lptim2lpen: self.lptim2lpen(),
                lptim3lpen: self.lptim3lpen(),
                lptim4lpen: self.lptim4lpen(),
                lptim5lpen: self.lptim5lpen(),
                dac2lpen: self.dac2lpen(),
                comp12lpen: self.comp12lpen(),
                vreflpen: self.vreflpen(),
                rtcapblpen: self.rtcapblpen(),
                sai4lpen: self.sai4lpen(),
                dtslpen: self.dtslpen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB4 Peripheral Reset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb4rstr(pub u32);
    impl Apb4rstr {
        #[doc = "SYSCFG block reset"]
        #[inline(always)]
        pub const fn syscfgrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG block reset"]
        #[inline(always)]
        pub fn set_syscfgrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPUART1 block reset"]
        #[inline(always)]
        pub const fn lpuart1rst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 block reset"]
        #[inline(always)]
        pub fn set_lpuart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SPI6 block reset"]
        #[inline(always)]
        pub const fn spi6rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI6 block reset"]
        #[inline(always)]
        pub fn set_spi6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "I2C4 block reset"]
        #[inline(always)]
        pub const fn i2c4rst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 block reset"]
        #[inline(always)]
        pub fn set_i2c4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LPTIM2 block reset"]
        #[inline(always)]
        pub const fn lptim2rst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 block reset"]
        #[inline(always)]
        pub fn set_lptim2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LPTIM3 block reset"]
        #[inline(always)]
        pub const fn lptim3rst(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3 block reset"]
        #[inline(always)]
        pub fn set_lptim3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LPTIM4 block reset"]
        #[inline(always)]
        pub const fn lptim4rst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM4 block reset"]
        #[inline(always)]
        pub fn set_lptim4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIM5 block reset"]
        #[inline(always)]
        pub const fn lptim5rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM5 block reset"]
        #[inline(always)]
        pub fn set_lptim5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "DAC2 (containing one converter) reset"]
        #[inline(always)]
        pub const fn dac2rst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "DAC2 (containing one converter) reset"]
        #[inline(always)]
        pub fn set_dac2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "COMP12 Blocks Reset"]
        #[inline(always)]
        pub const fn comp12rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "COMP12 Blocks Reset"]
        #[inline(always)]
        pub fn set_comp12rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "VREF block reset"]
        #[inline(always)]
        pub const fn vrefrst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "VREF block reset"]
        #[inline(always)]
        pub fn set_vrefrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "SAI4 block reset"]
        #[inline(always)]
        pub const fn sai4rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SAI4 block reset"]
        #[inline(always)]
        pub fn set_sai4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Digital temperature sensor block reset"]
        #[inline(always)]
        pub const fn dtsrst(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Digital temperature sensor block reset"]
        #[inline(always)]
        pub fn set_dtsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Apb4rstr {
        #[inline(always)]
        fn default() -> Apb4rstr {
            Apb4rstr(0)
        }
    }
    impl core::fmt::Debug for Apb4rstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb4rstr")
                .field("syscfgrst", &self.syscfgrst())
                .field("lpuart1rst", &self.lpuart1rst())
                .field("spi6rst", &self.spi6rst())
                .field("i2c4rst", &self.i2c4rst())
                .field("lptim2rst", &self.lptim2rst())
                .field("lptim3rst", &self.lptim3rst())
                .field("lptim4rst", &self.lptim4rst())
                .field("lptim5rst", &self.lptim5rst())
                .field("dac2rst", &self.dac2rst())
                .field("comp12rst", &self.comp12rst())
                .field("vrefrst", &self.vrefrst())
                .field("sai4rst", &self.sai4rst())
                .field("dtsrst", &self.dtsrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb4rstr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb4rstr {
                syscfgrst: bool,
                lpuart1rst: bool,
                spi6rst: bool,
                i2c4rst: bool,
                lptim2rst: bool,
                lptim3rst: bool,
                lptim4rst: bool,
                lptim5rst: bool,
                dac2rst: bool,
                comp12rst: bool,
                vrefrst: bool,
                sai4rst: bool,
                dtsrst: bool,
            }
            let proxy = Apb4rstr {
                syscfgrst: self.syscfgrst(),
                lpuart1rst: self.lpuart1rst(),
                spi6rst: self.spi6rst(),
                i2c4rst: self.i2c4rst(),
                lptim2rst: self.lptim2rst(),
                lptim3rst: self.lptim3rst(),
                lptim4rst: self.lptim4rst(),
                lptim5rst: self.lptim5rst(),
                dac2rst: self.dac2rst(),
                comp12rst: self.comp12rst(),
                vrefrst: self.vrefrst(),
                sai4rst: self.sai4rst(),
                dtsrst: self.dtsrst(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC Backup Domain Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdcr(pub u32);
    impl Bdcr {
        #[doc = "LSE oscillator enabled"]
        #[inline(always)]
        pub const fn lseon(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSE oscillator enabled"]
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
        #[doc = "LSE oscillator driving capability"]
        #[inline(always)]
        pub const fn lsedrv(&self) -> super::vals::Lsedrv {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Lsedrv::from_bits(val as u8)
        }
        #[doc = "LSE oscillator driving capability"]
        #[inline(always)]
        pub fn set_lsedrv(&mut self, val: super::vals::Lsedrv) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "LSE clock security system enable"]
        #[inline(always)]
        pub const fn lsecsson(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock security system enable"]
        #[inline(always)]
        pub fn set_lsecsson(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LSE clock security system failure detection"]
        #[inline(always)]
        pub const fn lsecssd(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock security system failure detection"]
        #[inline(always)]
        pub fn set_lsecssd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
        #[doc = "VSwitch domain software reset"]
        #[inline(always)]
        pub const fn bdrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "VSwitch domain software reset"]
        #[inline(always)]
        pub fn set_bdrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Bdcr {
        #[inline(always)]
        fn default() -> Bdcr {
            Bdcr(0)
        }
    }
    impl core::fmt::Debug for Bdcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bdcr")
                .field("lseon", &self.lseon())
                .field("lserdy", &self.lserdy())
                .field("lsebyp", &self.lsebyp())
                .field("lsedrv", &self.lsedrv())
                .field("lsecsson", &self.lsecsson())
                .field("lsecssd", &self.lsecssd())
                .field("rtcsel", &self.rtcsel())
                .field("rtcen", &self.rtcen())
                .field("bdrst", &self.bdrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bdcr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Bdcr {
                lseon: bool,
                lserdy: bool,
                lsebyp: bool,
                lsedrv: super::vals::Lsedrv,
                lsecsson: bool,
                lsecssd: bool,
                rtcsel: super::vals::Rtcsel,
                rtcen: bool,
                bdrst: bool,
            }
            let proxy = Bdcr {
                lseon: self.lseon(),
                lserdy: self.lserdy(),
                lsebyp: self.lsebyp(),
                lsedrv: self.lsedrv(),
                lsecsson: self.lsecsson(),
                lsecssd: self.lsecssd(),
                rtcsel: self.rtcsel(),
                rtcen: self.rtcen(),
                bdrst: self.bdrst(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC AHB1 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Ahb1enr(pub u32);
    impl C1Ahb1enr {
        #[doc = "DMA1 Clock Enable"]
        #[inline(always)]
        pub const fn dma1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 Clock Enable"]
        #[inline(always)]
        pub fn set_dma1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2 Clock Enable"]
        #[inline(always)]
        pub const fn dma2en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 Clock Enable"]
        #[inline(always)]
        pub fn set_dma2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ADC1/2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn adc12en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1/2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_adc12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ART Clock Enable"]
        #[inline(always)]
        pub const fn arten(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "ART Clock Enable"]
        #[inline(always)]
        pub fn set_arten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Ethernet MAC bus interface Clock Enable"]
        #[inline(always)]
        pub const fn ethen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet MAC bus interface Clock Enable"]
        #[inline(always)]
        pub fn set_ethen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Ethernet Transmission Clock Enable"]
        #[inline(always)]
        pub const fn ethtxen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet Transmission Clock Enable"]
        #[inline(always)]
        pub fn set_ethtxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Ethernet Reception Clock Enable"]
        #[inline(always)]
        pub const fn ethrxen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet Reception Clock Enable"]
        #[inline(always)]
        pub fn set_ethrxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USB_OTG_HS Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn usb_otg_hsen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "USB_OTG_HS Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_usb_otg_hsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "USB_PHY1 Clocks Enable"]
        #[inline(always)]
        pub const fn usb_otg_hs_ulpien(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "USB_PHY1 Clocks Enable"]
        #[inline(always)]
        pub fn set_usb_otg_hs_ulpien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "USB_OTG_FS Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn usb_otg_fsen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "USB_OTG_FS Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_usb_otg_fsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "USB_PHY2 Clocks Enable"]
        #[inline(always)]
        pub const fn usb_otg_fs_ulpien(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "USB_PHY2 Clocks Enable"]
        #[inline(always)]
        pub fn set_usb_otg_fs_ulpien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for C1Ahb1enr {
        #[inline(always)]
        fn default() -> C1Ahb1enr {
            C1Ahb1enr(0)
        }
    }
    impl core::fmt::Debug for C1Ahb1enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("C1Ahb1enr")
                .field("dma1en", &self.dma1en())
                .field("dma2en", &self.dma2en())
                .field("adc12en", &self.adc12en())
                .field("arten", &self.arten())
                .field("ethen", &self.ethen())
                .field("ethtxen", &self.ethtxen())
                .field("ethrxen", &self.ethrxen())
                .field("usb_otg_hsen", &self.usb_otg_hsen())
                .field("usb_otg_hs_ulpien", &self.usb_otg_hs_ulpien())
                .field("usb_otg_fsen", &self.usb_otg_fsen())
                .field("usb_otg_fs_ulpien", &self.usb_otg_fs_ulpien())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for C1Ahb1enr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct C1Ahb1enr {
                dma1en: bool,
                dma2en: bool,
                adc12en: bool,
                arten: bool,
                ethen: bool,
                ethtxen: bool,
                ethrxen: bool,
                usb_otg_hsen: bool,
                usb_otg_hs_ulpien: bool,
                usb_otg_fsen: bool,
                usb_otg_fs_ulpien: bool,
            }
            let proxy = C1Ahb1enr {
                dma1en: self.dma1en(),
                dma2en: self.dma2en(),
                adc12en: self.adc12en(),
                arten: self.arten(),
                ethen: self.ethen(),
                ethtxen: self.ethtxen(),
                ethrxen: self.ethrxen(),
                usb_otg_hsen: self.usb_otg_hsen(),
                usb_otg_hs_ulpien: self.usb_otg_hs_ulpien(),
                usb_otg_fsen: self.usb_otg_fsen(),
                usb_otg_fs_ulpien: self.usb_otg_fs_ulpien(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC AHB1 Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Ahb1lpenr(pub u32);
    impl C1Ahb1lpenr {
        #[doc = "DMA1 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn dma1lpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_dma1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn dma2lpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_dma2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ADC1/2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn adc12lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1/2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_adc12lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ART Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn artlpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "ART Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_artlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Ethernet MAC bus interface Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn ethlpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet MAC bus interface Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_ethlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Ethernet Transmission Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn ethtxlpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet Transmission Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_ethtxlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Ethernet Reception Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn ethrxlpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet Reception Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_ethrxlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USB_OTG_HS peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn usb_otg_hslpen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "USB_OTG_HS peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_usb_otg_hslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "USB_PHY1 clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn usb_otg_hs_ulpilpen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "USB_PHY1 clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_usb_otg_hs_ulpilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "USB_OTG_FS peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn usb_otg_fslpen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "USB_OTG_FS peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_usb_otg_fslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "USB_PHY2 clocks enable during CSleep mode"]
        #[inline(always)]
        pub const fn usb_otg_fs_ulpilpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "USB_PHY2 clocks enable during CSleep mode"]
        #[inline(always)]
        pub fn set_usb_otg_fs_ulpilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for C1Ahb1lpenr {
        #[inline(always)]
        fn default() -> C1Ahb1lpenr {
            C1Ahb1lpenr(0)
        }
    }
    impl core::fmt::Debug for C1Ahb1lpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("C1Ahb1lpenr")
                .field("dma1lpen", &self.dma1lpen())
                .field("dma2lpen", &self.dma2lpen())
                .field("adc12lpen", &self.adc12lpen())
                .field("artlpen", &self.artlpen())
                .field("ethlpen", &self.ethlpen())
                .field("ethtxlpen", &self.ethtxlpen())
                .field("ethrxlpen", &self.ethrxlpen())
                .field("usb_otg_hslpen", &self.usb_otg_hslpen())
                .field("usb_otg_hs_ulpilpen", &self.usb_otg_hs_ulpilpen())
                .field("usb_otg_fslpen", &self.usb_otg_fslpen())
                .field("usb_otg_fs_ulpilpen", &self.usb_otg_fs_ulpilpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for C1Ahb1lpenr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct C1Ahb1lpenr {
                dma1lpen: bool,
                dma2lpen: bool,
                adc12lpen: bool,
                artlpen: bool,
                ethlpen: bool,
                ethtxlpen: bool,
                ethrxlpen: bool,
                usb_otg_hslpen: bool,
                usb_otg_hs_ulpilpen: bool,
                usb_otg_fslpen: bool,
                usb_otg_fs_ulpilpen: bool,
            }
            let proxy = C1Ahb1lpenr {
                dma1lpen: self.dma1lpen(),
                dma2lpen: self.dma2lpen(),
                adc12lpen: self.adc12lpen(),
                artlpen: self.artlpen(),
                ethlpen: self.ethlpen(),
                ethtxlpen: self.ethtxlpen(),
                ethrxlpen: self.ethrxlpen(),
                usb_otg_hslpen: self.usb_otg_hslpen(),
                usb_otg_hs_ulpilpen: self.usb_otg_hs_ulpilpen(),
                usb_otg_fslpen: self.usb_otg_fslpen(),
                usb_otg_fs_ulpilpen: self.usb_otg_fs_ulpilpen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC AHB2 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Ahb2enr(pub u32);
    impl C1Ahb2enr {
        #[doc = "DCMI peripheral clock"]
        #[inline(always)]
        pub const fn dcmien(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DCMI peripheral clock"]
        #[inline(always)]
        pub fn set_dcmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CRYP peripheral clock enable"]
        #[inline(always)]
        pub const fn crypen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CRYP peripheral clock enable"]
        #[inline(always)]
        pub fn set_crypen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "HASH peripheral clock enable"]
        #[inline(always)]
        pub const fn hashen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HASH peripheral clock enable"]
        #[inline(always)]
        pub fn set_hashen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "RNG peripheral clocks enable"]
        #[inline(always)]
        pub const fn rngen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RNG peripheral clocks enable"]
        #[inline(always)]
        pub fn set_rngen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "SDMMC2 and SDMMC2 delay clock enable"]
        #[inline(always)]
        pub const fn sdmmc2en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC2 and SDMMC2 delay clock enable"]
        #[inline(always)]
        pub fn set_sdmmc2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "SRAM1 block enable"]
        #[inline(always)]
        pub const fn sram1en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 block enable"]
        #[inline(always)]
        pub fn set_sram1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "SRAM2 block enable"]
        #[inline(always)]
        pub const fn sram2en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 block enable"]
        #[inline(always)]
        pub fn set_sram2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SRAM3 block enable"]
        #[inline(always)]
        pub const fn sram3en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM3 block enable"]
        #[inline(always)]
        pub fn set_sram3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for C1Ahb2enr {
        #[inline(always)]
        fn default() -> C1Ahb2enr {
            C1Ahb2enr(0)
        }
    }
    impl core::fmt::Debug for C1Ahb2enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("C1Ahb2enr")
                .field("dcmien", &self.dcmien())
                .field("crypen", &self.crypen())
                .field("hashen", &self.hashen())
                .field("rngen", &self.rngen())
                .field("sdmmc2en", &self.sdmmc2en())
                .field("sram1en", &self.sram1en())
                .field("sram2en", &self.sram2en())
                .field("sram3en", &self.sram3en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for C1Ahb2enr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct C1Ahb2enr {
                dcmien: bool,
                crypen: bool,
                hashen: bool,
                rngen: bool,
                sdmmc2en: bool,
                sram1en: bool,
                sram2en: bool,
                sram3en: bool,
            }
            let proxy = C1Ahb2enr {
                dcmien: self.dcmien(),
                crypen: self.crypen(),
                hashen: self.hashen(),
                rngen: self.rngen(),
                sdmmc2en: self.sdmmc2en(),
                sram1en: self.sram1en(),
                sram2en: self.sram2en(),
                sram3en: self.sram3en(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC AHB2 Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Ahb2lpenr(pub u32);
    impl C1Ahb2lpenr {
        #[doc = "DCMI peripheral clock enable during csleep mode"]
        #[inline(always)]
        pub const fn dcmilpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DCMI peripheral clock enable during csleep mode"]
        #[inline(always)]
        pub fn set_dcmilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CRYP peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn cryplpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CRYP peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_cryplpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "HASH peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn hashlpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HASH peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_hashlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "RNG peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn rnglpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RNG peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_rnglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn sdmmc2lpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_sdmmc2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "FMAC enable during CSleep Mode"]
        #[inline(always)]
        pub const fn fmaclpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "FMAC enable during CSleep Mode"]
        #[inline(always)]
        pub fn set_fmaclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "CORDIC enable during CSleep Mode"]
        #[inline(always)]
        pub const fn cordiclpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "CORDIC enable during CSleep Mode"]
        #[inline(always)]
        pub fn set_cordiclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "SRAM1 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn sram1lpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_sram1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "SRAM2 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn sram2lpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_sram2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SRAM3 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn sram3lpen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM3 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_sram3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for C1Ahb2lpenr {
        #[inline(always)]
        fn default() -> C1Ahb2lpenr {
            C1Ahb2lpenr(0)
        }
    }
    impl core::fmt::Debug for C1Ahb2lpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("C1Ahb2lpenr")
                .field("dcmilpen", &self.dcmilpen())
                .field("cryplpen", &self.cryplpen())
                .field("hashlpen", &self.hashlpen())
                .field("rnglpen", &self.rnglpen())
                .field("sdmmc2lpen", &self.sdmmc2lpen())
                .field("fmaclpen", &self.fmaclpen())
                .field("cordiclpen", &self.cordiclpen())
                .field("sram1lpen", &self.sram1lpen())
                .field("sram2lpen", &self.sram2lpen())
                .field("sram3lpen", &self.sram3lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for C1Ahb2lpenr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct C1Ahb2lpenr {
                dcmilpen: bool,
                cryplpen: bool,
                hashlpen: bool,
                rnglpen: bool,
                sdmmc2lpen: bool,
                fmaclpen: bool,
                cordiclpen: bool,
                sram1lpen: bool,
                sram2lpen: bool,
                sram3lpen: bool,
            }
            let proxy = C1Ahb2lpenr {
                dcmilpen: self.dcmilpen(),
                cryplpen: self.cryplpen(),
                hashlpen: self.hashlpen(),
                rnglpen: self.rnglpen(),
                sdmmc2lpen: self.sdmmc2lpen(),
                fmaclpen: self.fmaclpen(),
                cordiclpen: self.cordiclpen(),
                sram1lpen: self.sram1lpen(),
                sram2lpen: self.sram2lpen(),
                sram3lpen: self.sram3lpen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC AHB3 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Ahb3enr(pub u32);
    impl C1Ahb3enr {
        #[doc = "MDMA Peripheral Clock Enable"]
        #[inline(always)]
        pub const fn mdmaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MDMA Peripheral Clock Enable"]
        #[inline(always)]
        pub fn set_mdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2D Peripheral Clock Enable"]
        #[inline(always)]
        pub const fn dma2den(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2D Peripheral Clock Enable"]
        #[inline(always)]
        pub fn set_dma2den(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "JPGDEC Peripheral Clock Enable"]
        #[inline(always)]
        pub const fn jpgdecen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "JPGDEC Peripheral Clock Enable"]
        #[inline(always)]
        pub fn set_jpgdecen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FMC Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn fmcen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "FMC Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_fmcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "QUADSPI and QUADSPI Delay Clock Enable"]
        #[inline(always)]
        pub const fn quadspien(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "QUADSPI and QUADSPI Delay Clock Enable"]
        #[inline(always)]
        pub fn set_quadspien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SDMMC1 and SDMMC1 Delay Clock Enable"]
        #[inline(always)]
        pub const fn sdmmc1en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC1 and SDMMC1 Delay Clock Enable"]
        #[inline(always)]
        pub fn set_sdmmc1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for C1Ahb3enr {
        #[inline(always)]
        fn default() -> C1Ahb3enr {
            C1Ahb3enr(0)
        }
    }
    impl core::fmt::Debug for C1Ahb3enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("C1Ahb3enr")
                .field("mdmaen", &self.mdmaen())
                .field("dma2den", &self.dma2den())
                .field("jpgdecen", &self.jpgdecen())
                .field("fmcen", &self.fmcen())
                .field("quadspien", &self.quadspien())
                .field("sdmmc1en", &self.sdmmc1en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for C1Ahb3enr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct C1Ahb3enr {
                mdmaen: bool,
                dma2den: bool,
                jpgdecen: bool,
                fmcen: bool,
                quadspien: bool,
                sdmmc1en: bool,
            }
            let proxy = C1Ahb3enr {
                mdmaen: self.mdmaen(),
                dma2den: self.dma2den(),
                jpgdecen: self.jpgdecen(),
                fmcen: self.fmcen(),
                quadspien: self.quadspien(),
                sdmmc1en: self.sdmmc1en(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC AHB3 Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Ahb3lpenr(pub u32);
    impl C1Ahb3lpenr {
        #[doc = "MDMA Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn mdmalpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MDMA Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_mdmalpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2D Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn dma2dlpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2D Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_dma2dlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "JPGDEC Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn jpgdeclpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "JPGDEC Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_jpgdeclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Flash interface clock enable during csleep mode"]
        #[inline(always)]
        pub const fn flashpren(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Flash interface clock enable during csleep mode"]
        #[inline(always)]
        pub fn set_flashpren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "FMC Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn fmclpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "FMC Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_fmclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "QUADSPI and QUADSPI Delay Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn quadspilpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "QUADSPI and QUADSPI Delay Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_quadspilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn sdmmc1lpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_sdmmc1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "OCTOSPI2 and OCTOSPI2 delay block enable during CSleep Mode"]
        #[inline(always)]
        pub const fn octospi2lpen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI2 and OCTOSPI2 delay block enable during CSleep Mode"]
        #[inline(always)]
        pub fn set_octospi2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "OCTOSPI IO manager enable during CSleep Mode"]
        #[inline(always)]
        pub const fn iomngrlpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI IO manager enable during CSleep Mode"]
        #[inline(always)]
        pub fn set_iomngrlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "OTFDEC1 enable during CSleep Mode"]
        #[inline(always)]
        pub const fn otfd1lpen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC1 enable during CSleep Mode"]
        #[inline(always)]
        pub fn set_otfd1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "OTFDEC2 enable during CSleep Mode"]
        #[inline(always)]
        pub const fn otfd2lpen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC2 enable during CSleep Mode"]
        #[inline(always)]
        pub fn set_otfd2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "D1DTCM1 Block Clock Enable During CSleep mode"]
        #[inline(always)]
        pub const fn d1dtcm1lpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "D1DTCM1 Block Clock Enable During CSleep mode"]
        #[inline(always)]
        pub fn set_d1dtcm1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "D1 DTCM2 Block Clock Enable During CSleep mode"]
        #[inline(always)]
        pub const fn dtcm2lpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "D1 DTCM2 Block Clock Enable During CSleep mode"]
        #[inline(always)]
        pub fn set_dtcm2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "D1ITCM Block Clock Enable During CSleep mode"]
        #[inline(always)]
        pub const fn itcmlpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "D1ITCM Block Clock Enable During CSleep mode"]
        #[inline(always)]
        pub fn set_itcmlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "AXISRAM Block Clock Enable During CSleep mode"]
        #[inline(always)]
        pub const fn axisramlpen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "AXISRAM Block Clock Enable During CSleep mode"]
        #[inline(always)]
        pub fn set_axisramlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for C1Ahb3lpenr {
        #[inline(always)]
        fn default() -> C1Ahb3lpenr {
            C1Ahb3lpenr(0)
        }
    }
    impl core::fmt::Debug for C1Ahb3lpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("C1Ahb3lpenr")
                .field("mdmalpen", &self.mdmalpen())
                .field("dma2dlpen", &self.dma2dlpen())
                .field("jpgdeclpen", &self.jpgdeclpen())
                .field("flashpren", &self.flashpren())
                .field("fmclpen", &self.fmclpen())
                .field("quadspilpen", &self.quadspilpen())
                .field("sdmmc1lpen", &self.sdmmc1lpen())
                .field("octospi2lpen", &self.octospi2lpen())
                .field("iomngrlpen", &self.iomngrlpen())
                .field("otfd1lpen", &self.otfd1lpen())
                .field("otfd2lpen", &self.otfd2lpen())
                .field("d1dtcm1lpen", &self.d1dtcm1lpen())
                .field("dtcm2lpen", &self.dtcm2lpen())
                .field("itcmlpen", &self.itcmlpen())
                .field("axisramlpen", &self.axisramlpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for C1Ahb3lpenr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct C1Ahb3lpenr {
                mdmalpen: bool,
                dma2dlpen: bool,
                jpgdeclpen: bool,
                flashpren: bool,
                fmclpen: bool,
                quadspilpen: bool,
                sdmmc1lpen: bool,
                octospi2lpen: bool,
                iomngrlpen: bool,
                otfd1lpen: bool,
                otfd2lpen: bool,
                d1dtcm1lpen: bool,
                dtcm2lpen: bool,
                itcmlpen: bool,
                axisramlpen: bool,
            }
            let proxy = C1Ahb3lpenr {
                mdmalpen: self.mdmalpen(),
                dma2dlpen: self.dma2dlpen(),
                jpgdeclpen: self.jpgdeclpen(),
                flashpren: self.flashpren(),
                fmclpen: self.fmclpen(),
                quadspilpen: self.quadspilpen(),
                sdmmc1lpen: self.sdmmc1lpen(),
                octospi2lpen: self.octospi2lpen(),
                iomngrlpen: self.iomngrlpen(),
                otfd1lpen: self.otfd1lpen(),
                otfd2lpen: self.otfd2lpen(),
                d1dtcm1lpen: self.d1dtcm1lpen(),
                dtcm2lpen: self.dtcm2lpen(),
                itcmlpen: self.itcmlpen(),
                axisramlpen: self.axisramlpen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC AHB4 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Ahb4enr(pub u32);
    impl C1Ahb4enr {
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn gpioaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub fn set_gpioaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn gpioben(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub fn set_gpioben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn gpiocen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub fn set_gpiocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn gpioden(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub fn set_gpioden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn gpioeen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub fn set_gpioeen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn gpiofen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub fn set_gpiofen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn gpiogen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub fn set_gpiogen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn gpiohen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub fn set_gpiohen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn gpioien(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub fn set_gpioien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn gpiojen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub fn set_gpiojen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn gpioken(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub fn set_gpioken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "CRC peripheral clock enable"]
        #[inline(always)]
        pub const fn crcen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "CRC peripheral clock enable"]
        #[inline(always)]
        pub fn set_crcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "BDMA and DMAMUX2 Clock Enable"]
        #[inline(always)]
        pub const fn bdmaen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "BDMA and DMAMUX2 Clock Enable"]
        #[inline(always)]
        pub fn set_bdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ADC3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn adc3en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ADC3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_adc3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "HSEM peripheral clock enable"]
        #[inline(always)]
        pub const fn hsemen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "HSEM peripheral clock enable"]
        #[inline(always)]
        pub fn set_hsemen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Backup RAM Clock Enable"]
        #[inline(always)]
        pub const fn bkpsramen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Backup RAM Clock Enable"]
        #[inline(always)]
        pub fn set_bkpsramen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for C1Ahb4enr {
        #[inline(always)]
        fn default() -> C1Ahb4enr {
            C1Ahb4enr(0)
        }
    }
    impl core::fmt::Debug for C1Ahb4enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("C1Ahb4enr")
                .field("gpioaen", &self.gpioaen())
                .field("gpioben", &self.gpioben())
                .field("gpiocen", &self.gpiocen())
                .field("gpioden", &self.gpioden())
                .field("gpioeen", &self.gpioeen())
                .field("gpiofen", &self.gpiofen())
                .field("gpiogen", &self.gpiogen())
                .field("gpiohen", &self.gpiohen())
                .field("gpioien", &self.gpioien())
                .field("gpiojen", &self.gpiojen())
                .field("gpioken", &self.gpioken())
                .field("crcen", &self.crcen())
                .field("bdmaen", &self.bdmaen())
                .field("adc3en", &self.adc3en())
                .field("hsemen", &self.hsemen())
                .field("bkpsramen", &self.bkpsramen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for C1Ahb4enr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct C1Ahb4enr {
                gpioaen: bool,
                gpioben: bool,
                gpiocen: bool,
                gpioden: bool,
                gpioeen: bool,
                gpiofen: bool,
                gpiogen: bool,
                gpiohen: bool,
                gpioien: bool,
                gpiojen: bool,
                gpioken: bool,
                crcen: bool,
                bdmaen: bool,
                adc3en: bool,
                hsemen: bool,
                bkpsramen: bool,
            }
            let proxy = C1Ahb4enr {
                gpioaen: self.gpioaen(),
                gpioben: self.gpioben(),
                gpiocen: self.gpiocen(),
                gpioden: self.gpioden(),
                gpioeen: self.gpioeen(),
                gpiofen: self.gpiofen(),
                gpiogen: self.gpiogen(),
                gpiohen: self.gpiohen(),
                gpioien: self.gpioien(),
                gpiojen: self.gpiojen(),
                gpioken: self.gpioken(),
                crcen: self.crcen(),
                bdmaen: self.bdmaen(),
                adc3en: self.adc3en(),
                hsemen: self.hsemen(),
                bkpsramen: self.bkpsramen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC AHB4 Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Ahb4lpenr(pub u32);
    impl C1Ahb4lpenr {
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn gpioalpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_gpioalpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn gpioblpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_gpioblpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn gpioclpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_gpioclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn gpiodlpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_gpiodlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn gpioelpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_gpioelpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn gpioflpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_gpioflpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn gpioglpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_gpioglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn gpiohlpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_gpiohlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn gpioilpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_gpioilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn gpiojlpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_gpiojlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn gpioklpen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_gpioklpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "CRC peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn crclpen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "CRC peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_crclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "BDMA Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn bdmalpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "BDMA Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_bdmalpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ADC3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn adc3lpen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ADC3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_adc3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Backup RAM Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn bkpsramlpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Backup RAM Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_bkpsramlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "SRAM4 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn sram4lpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM4 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_sram4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for C1Ahb4lpenr {
        #[inline(always)]
        fn default() -> C1Ahb4lpenr {
            C1Ahb4lpenr(0)
        }
    }
    impl core::fmt::Debug for C1Ahb4lpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("C1Ahb4lpenr")
                .field("gpioalpen", &self.gpioalpen())
                .field("gpioblpen", &self.gpioblpen())
                .field("gpioclpen", &self.gpioclpen())
                .field("gpiodlpen", &self.gpiodlpen())
                .field("gpioelpen", &self.gpioelpen())
                .field("gpioflpen", &self.gpioflpen())
                .field("gpioglpen", &self.gpioglpen())
                .field("gpiohlpen", &self.gpiohlpen())
                .field("gpioilpen", &self.gpioilpen())
                .field("gpiojlpen", &self.gpiojlpen())
                .field("gpioklpen", &self.gpioklpen())
                .field("crclpen", &self.crclpen())
                .field("bdmalpen", &self.bdmalpen())
                .field("adc3lpen", &self.adc3lpen())
                .field("bkpsramlpen", &self.bkpsramlpen())
                .field("sram4lpen", &self.sram4lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for C1Ahb4lpenr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct C1Ahb4lpenr {
                gpioalpen: bool,
                gpioblpen: bool,
                gpioclpen: bool,
                gpiodlpen: bool,
                gpioelpen: bool,
                gpioflpen: bool,
                gpioglpen: bool,
                gpiohlpen: bool,
                gpioilpen: bool,
                gpiojlpen: bool,
                gpioklpen: bool,
                crclpen: bool,
                bdmalpen: bool,
                adc3lpen: bool,
                bkpsramlpen: bool,
                sram4lpen: bool,
            }
            let proxy = C1Ahb4lpenr {
                gpioalpen: self.gpioalpen(),
                gpioblpen: self.gpioblpen(),
                gpioclpen: self.gpioclpen(),
                gpiodlpen: self.gpiodlpen(),
                gpioelpen: self.gpioelpen(),
                gpioflpen: self.gpioflpen(),
                gpioglpen: self.gpioglpen(),
                gpiohlpen: self.gpiohlpen(),
                gpioilpen: self.gpioilpen(),
                gpiojlpen: self.gpiojlpen(),
                gpioklpen: self.gpioklpen(),
                crclpen: self.crclpen(),
                bdmalpen: self.bdmalpen(),
                adc3lpen: self.adc3lpen(),
                bkpsramlpen: self.bkpsramlpen(),
                sram4lpen: self.sram4lpen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB1 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Apb1henr(pub u32);
    impl C1Apb1henr {
        #[doc = "Clock Recovery System peripheral clock enable"]
        #[inline(always)]
        pub const fn crsen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clock Recovery System peripheral clock enable"]
        #[inline(always)]
        pub fn set_crsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SWPMI Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn swpmien(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SWPMI Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_swpmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "OPAMP peripheral clock enable"]
        #[inline(always)]
        pub const fn opampen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP peripheral clock enable"]
        #[inline(always)]
        pub fn set_opampen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MDIOS peripheral clock enable"]
        #[inline(always)]
        pub const fn mdiosen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MDIOS peripheral clock enable"]
        #[inline(always)]
        pub fn set_mdiosen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FDCAN Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn fdcanen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_fdcanen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for C1Apb1henr {
        #[inline(always)]
        fn default() -> C1Apb1henr {
            C1Apb1henr(0)
        }
    }
    impl core::fmt::Debug for C1Apb1henr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("C1Apb1henr")
                .field("crsen", &self.crsen())
                .field("swpmien", &self.swpmien())
                .field("opampen", &self.opampen())
                .field("mdiosen", &self.mdiosen())
                .field("fdcanen", &self.fdcanen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for C1Apb1henr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct C1Apb1henr {
                crsen: bool,
                swpmien: bool,
                opampen: bool,
                mdiosen: bool,
                fdcanen: bool,
            }
            let proxy = C1Apb1henr {
                crsen: self.crsen(),
                swpmien: self.swpmien(),
                opampen: self.opampen(),
                mdiosen: self.mdiosen(),
                fdcanen: self.fdcanen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB1 High Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Apb1hlpenr(pub u32);
    impl C1Apb1hlpenr {
        #[doc = "Clock Recovery System peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn crslpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clock Recovery System peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_crslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SWPMI Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn swpmilpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SWPMI Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_swpmilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "OPAMP peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn opamplpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_opamplpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MDIOS peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn mdioslpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MDIOS peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_mdioslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FDCAN Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn fdcanlpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_fdcanlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "TIM23 block enable during CSleep Mode"]
        #[inline(always)]
        pub const fn tim23lpen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "TIM23 block enable during CSleep Mode"]
        #[inline(always)]
        pub fn set_tim23lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "TIM24 block enable during CSleep Mode"]
        #[inline(always)]
        pub const fn tim24lpen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "TIM24 block enable during CSleep Mode"]
        #[inline(always)]
        pub fn set_tim24lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for C1Apb1hlpenr {
        #[inline(always)]
        fn default() -> C1Apb1hlpenr {
            C1Apb1hlpenr(0)
        }
    }
    impl core::fmt::Debug for C1Apb1hlpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("C1Apb1hlpenr")
                .field("crslpen", &self.crslpen())
                .field("swpmilpen", &self.swpmilpen())
                .field("opamplpen", &self.opamplpen())
                .field("mdioslpen", &self.mdioslpen())
                .field("fdcanlpen", &self.fdcanlpen())
                .field("tim23lpen", &self.tim23lpen())
                .field("tim24lpen", &self.tim24lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for C1Apb1hlpenr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct C1Apb1hlpenr {
                crslpen: bool,
                swpmilpen: bool,
                opamplpen: bool,
                mdioslpen: bool,
                fdcanlpen: bool,
                tim23lpen: bool,
                tim24lpen: bool,
            }
            let proxy = C1Apb1hlpenr {
                crslpen: self.crslpen(),
                swpmilpen: self.swpmilpen(),
                opamplpen: self.opamplpen(),
                mdioslpen: self.mdioslpen(),
                fdcanlpen: self.fdcanlpen(),
                tim23lpen: self.tim23lpen(),
                tim24lpen: self.tim24lpen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB1 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Apb1lenr(pub u32);
    impl C1Apb1lenr {
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn tim2en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn tim3en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn tim4en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn tim5en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn tim6en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn tim7en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn tim12en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn tim13en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim13en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn tim14en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim14en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LPTIM1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn lptim1en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_lptim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "WWDG2 peripheral clock enable"]
        #[inline(always)]
        pub const fn wwdg2en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG2 peripheral clock enable"]
        #[inline(always)]
        pub fn set_wwdg2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn spi2en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_spi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn spi3en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_spi3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "SPDIFRX Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn spdifrxen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SPDIFRX Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_spdifrxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "USART2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn usart2en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_usart2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn usart3en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_usart3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn uart4en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_uart4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn uart5en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_uart5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn i2c1en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_i2c1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn i2c2en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_i2c2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I2C3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn i2c3en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_i2c3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "I2C5 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn i2c5en(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "I2C5 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_i2c5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "HDMI-CEC peripheral clock enable"]
        #[inline(always)]
        pub const fn cecen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "HDMI-CEC peripheral clock enable"]
        #[inline(always)]
        pub fn set_cecen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "DAC1&2 peripheral clock enable"]
        #[inline(always)]
        pub const fn dac12en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1&2 peripheral clock enable"]
        #[inline(always)]
        pub fn set_dac12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "UART7 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn uart7en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "UART7 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_uart7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "UART8 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn uart8en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "UART8 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_uart8en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for C1Apb1lenr {
        #[inline(always)]
        fn default() -> C1Apb1lenr {
            C1Apb1lenr(0)
        }
    }
    impl core::fmt::Debug for C1Apb1lenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("C1Apb1lenr")
                .field("tim2en", &self.tim2en())
                .field("tim3en", &self.tim3en())
                .field("tim4en", &self.tim4en())
                .field("tim5en", &self.tim5en())
                .field("tim6en", &self.tim6en())
                .field("tim7en", &self.tim7en())
                .field("tim12en", &self.tim12en())
                .field("tim13en", &self.tim13en())
                .field("tim14en", &self.tim14en())
                .field("lptim1en", &self.lptim1en())
                .field("wwdg2en", &self.wwdg2en())
                .field("spi2en", &self.spi2en())
                .field("spi3en", &self.spi3en())
                .field("spdifrxen", &self.spdifrxen())
                .field("usart2en", &self.usart2en())
                .field("usart3en", &self.usart3en())
                .field("uart4en", &self.uart4en())
                .field("uart5en", &self.uart5en())
                .field("i2c1en", &self.i2c1en())
                .field("i2c2en", &self.i2c2en())
                .field("i2c3en", &self.i2c3en())
                .field("i2c5en", &self.i2c5en())
                .field("cecen", &self.cecen())
                .field("dac12en", &self.dac12en())
                .field("uart7en", &self.uart7en())
                .field("uart8en", &self.uart8en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for C1Apb1lenr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct C1Apb1lenr {
                tim2en: bool,
                tim3en: bool,
                tim4en: bool,
                tim5en: bool,
                tim6en: bool,
                tim7en: bool,
                tim12en: bool,
                tim13en: bool,
                tim14en: bool,
                lptim1en: bool,
                wwdg2en: bool,
                spi2en: bool,
                spi3en: bool,
                spdifrxen: bool,
                usart2en: bool,
                usart3en: bool,
                uart4en: bool,
                uart5en: bool,
                i2c1en: bool,
                i2c2en: bool,
                i2c3en: bool,
                i2c5en: bool,
                cecen: bool,
                dac12en: bool,
                uart7en: bool,
                uart8en: bool,
            }
            let proxy = C1Apb1lenr {
                tim2en: self.tim2en(),
                tim3en: self.tim3en(),
                tim4en: self.tim4en(),
                tim5en: self.tim5en(),
                tim6en: self.tim6en(),
                tim7en: self.tim7en(),
                tim12en: self.tim12en(),
                tim13en: self.tim13en(),
                tim14en: self.tim14en(),
                lptim1en: self.lptim1en(),
                wwdg2en: self.wwdg2en(),
                spi2en: self.spi2en(),
                spi3en: self.spi3en(),
                spdifrxen: self.spdifrxen(),
                usart2en: self.usart2en(),
                usart3en: self.usart3en(),
                uart4en: self.uart4en(),
                uart5en: self.uart5en(),
                i2c1en: self.i2c1en(),
                i2c2en: self.i2c2en(),
                i2c3en: self.i2c3en(),
                i2c5en: self.i2c5en(),
                cecen: self.cecen(),
                dac12en: self.dac12en(),
                uart7en: self.uart7en(),
                uart8en: self.uart8en(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB1 Low Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Apb1llpenr(pub u32);
    impl C1Apb1llpenr {
        #[doc = "TIM2 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim2lpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim3lpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim4lpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim5lpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim6lpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim6lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim7lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim7lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM12 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim12lpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim12lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIM13 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim13lpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIM13 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim13lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIM14 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim14lpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM14 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim14lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LPTIM1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn lptim1lpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_lptim1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "WWDG2 peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn wwdg2lpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG2 peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_wwdg2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn spi2lpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_spi2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn spi3lpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_spi3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "SPDIFRX Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn spdifrxlpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SPDIFRX Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_spdifrxlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "USART2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn usart2lpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_usart2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn usart3lpen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_usart3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn uart4lpen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_uart4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn uart5lpen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_uart5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn i2c1lpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_i2c1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn i2c2lpen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_i2c2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I2C3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn i2c3lpen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_i2c3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "I2C5 block enable during CSleep Mode"]
        #[inline(always)]
        pub const fn i2c5lpen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "I2C5 block enable during CSleep Mode"]
        #[inline(always)]
        pub fn set_i2c5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "HDMI-CEC Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn ceclpen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "HDMI-CEC Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_ceclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "DAC1/2 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn dac12lpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1/2 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_dac12lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "UART7 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn uart7lpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "UART7 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_uart7lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "UART8 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn uart8lpen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "UART8 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_uart8lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for C1Apb1llpenr {
        #[inline(always)]
        fn default() -> C1Apb1llpenr {
            C1Apb1llpenr(0)
        }
    }
    impl core::fmt::Debug for C1Apb1llpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("C1Apb1llpenr")
                .field("tim2lpen", &self.tim2lpen())
                .field("tim3lpen", &self.tim3lpen())
                .field("tim4lpen", &self.tim4lpen())
                .field("tim5lpen", &self.tim5lpen())
                .field("tim6lpen", &self.tim6lpen())
                .field("tim7lpen", &self.tim7lpen())
                .field("tim12lpen", &self.tim12lpen())
                .field("tim13lpen", &self.tim13lpen())
                .field("tim14lpen", &self.tim14lpen())
                .field("lptim1lpen", &self.lptim1lpen())
                .field("wwdg2lpen", &self.wwdg2lpen())
                .field("spi2lpen", &self.spi2lpen())
                .field("spi3lpen", &self.spi3lpen())
                .field("spdifrxlpen", &self.spdifrxlpen())
                .field("usart2lpen", &self.usart2lpen())
                .field("usart3lpen", &self.usart3lpen())
                .field("uart4lpen", &self.uart4lpen())
                .field("uart5lpen", &self.uart5lpen())
                .field("i2c1lpen", &self.i2c1lpen())
                .field("i2c2lpen", &self.i2c2lpen())
                .field("i2c3lpen", &self.i2c3lpen())
                .field("i2c5lpen", &self.i2c5lpen())
                .field("ceclpen", &self.ceclpen())
                .field("dac12lpen", &self.dac12lpen())
                .field("uart7lpen", &self.uart7lpen())
                .field("uart8lpen", &self.uart8lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for C1Apb1llpenr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct C1Apb1llpenr {
                tim2lpen: bool,
                tim3lpen: bool,
                tim4lpen: bool,
                tim5lpen: bool,
                tim6lpen: bool,
                tim7lpen: bool,
                tim12lpen: bool,
                tim13lpen: bool,
                tim14lpen: bool,
                lptim1lpen: bool,
                wwdg2lpen: bool,
                spi2lpen: bool,
                spi3lpen: bool,
                spdifrxlpen: bool,
                usart2lpen: bool,
                usart3lpen: bool,
                uart4lpen: bool,
                uart5lpen: bool,
                i2c1lpen: bool,
                i2c2lpen: bool,
                i2c3lpen: bool,
                i2c5lpen: bool,
                ceclpen: bool,
                dac12lpen: bool,
                uart7lpen: bool,
                uart8lpen: bool,
            }
            let proxy = C1Apb1llpenr {
                tim2lpen: self.tim2lpen(),
                tim3lpen: self.tim3lpen(),
                tim4lpen: self.tim4lpen(),
                tim5lpen: self.tim5lpen(),
                tim6lpen: self.tim6lpen(),
                tim7lpen: self.tim7lpen(),
                tim12lpen: self.tim12lpen(),
                tim13lpen: self.tim13lpen(),
                tim14lpen: self.tim14lpen(),
                lptim1lpen: self.lptim1lpen(),
                wwdg2lpen: self.wwdg2lpen(),
                spi2lpen: self.spi2lpen(),
                spi3lpen: self.spi3lpen(),
                spdifrxlpen: self.spdifrxlpen(),
                usart2lpen: self.usart2lpen(),
                usart3lpen: self.usart3lpen(),
                uart4lpen: self.uart4lpen(),
                uart5lpen: self.uart5lpen(),
                i2c1lpen: self.i2c1lpen(),
                i2c2lpen: self.i2c2lpen(),
                i2c3lpen: self.i2c3lpen(),
                i2c5lpen: self.i2c5lpen(),
                ceclpen: self.ceclpen(),
                dac12lpen: self.dac12lpen(),
                uart7lpen: self.uart7lpen(),
                uart8lpen: self.uart8lpen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB2 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Apb2enr(pub u32);
    impl C1Apb2enr {
        #[doc = "TIM1 peripheral clock enable"]
        #[inline(always)]
        pub const fn tim1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM8 peripheral clock enable"]
        #[inline(always)]
        pub const fn tim8en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim8en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "USART1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn usart1en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_usart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "USART6 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn usart6en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USART6 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_usart6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "UART9 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn uart9en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "UART9 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_uart9en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "USART10 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn usart10en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "USART10 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_usart10en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "SPI1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn spi1en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_spi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SPI4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn spi4en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "SPI4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_spi4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "TIM15 peripheral clock enable"]
        #[inline(always)]
        pub const fn tim15en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim15en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 peripheral clock enable"]
        #[inline(always)]
        pub const fn tim16en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim16en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 peripheral clock enable"]
        #[inline(always)]
        pub const fn tim17en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 peripheral clock enable"]
        #[inline(always)]
        pub fn set_tim17en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SPI5 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn spi5en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SPI5 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_spi5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "SAI1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn sai1en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_sai1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "SAI2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn sai2en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "SAI2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_sai2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "SAI3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn sai3en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "SAI3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_sai3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "DFSDM1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn dfsdm1en(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "DFSDM1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_dfsdm1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "HRTIM peripheral clock enable"]
        #[inline(always)]
        pub const fn hrtimen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM peripheral clock enable"]
        #[inline(always)]
        pub fn set_hrtimen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for C1Apb2enr {
        #[inline(always)]
        fn default() -> C1Apb2enr {
            C1Apb2enr(0)
        }
    }
    impl core::fmt::Debug for C1Apb2enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("C1Apb2enr")
                .field("tim1en", &self.tim1en())
                .field("tim8en", &self.tim8en())
                .field("usart1en", &self.usart1en())
                .field("usart6en", &self.usart6en())
                .field("uart9en", &self.uart9en())
                .field("usart10en", &self.usart10en())
                .field("spi1en", &self.spi1en())
                .field("spi4en", &self.spi4en())
                .field("tim15en", &self.tim15en())
                .field("tim16en", &self.tim16en())
                .field("tim17en", &self.tim17en())
                .field("spi5en", &self.spi5en())
                .field("sai1en", &self.sai1en())
                .field("sai2en", &self.sai2en())
                .field("sai3en", &self.sai3en())
                .field("dfsdm1en", &self.dfsdm1en())
                .field("hrtimen", &self.hrtimen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for C1Apb2enr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct C1Apb2enr {
                tim1en: bool,
                tim8en: bool,
                usart1en: bool,
                usart6en: bool,
                uart9en: bool,
                usart10en: bool,
                spi1en: bool,
                spi4en: bool,
                tim15en: bool,
                tim16en: bool,
                tim17en: bool,
                spi5en: bool,
                sai1en: bool,
                sai2en: bool,
                sai3en: bool,
                dfsdm1en: bool,
                hrtimen: bool,
            }
            let proxy = C1Apb2enr {
                tim1en: self.tim1en(),
                tim8en: self.tim8en(),
                usart1en: self.usart1en(),
                usart6en: self.usart6en(),
                uart9en: self.uart9en(),
                usart10en: self.usart10en(),
                spi1en: self.spi1en(),
                spi4en: self.spi4en(),
                tim15en: self.tim15en(),
                tim16en: self.tim16en(),
                tim17en: self.tim17en(),
                spi5en: self.spi5en(),
                sai1en: self.sai1en(),
                sai2en: self.sai2en(),
                sai3en: self.sai3en(),
                dfsdm1en: self.dfsdm1en(),
                hrtimen: self.hrtimen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB2 Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Apb2lpenr(pub u32);
    impl C1Apb2lpenr {
        #[doc = "TIM1 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim1lpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM8 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim8lpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim8lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "USART1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn usart1lpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_usart1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "USART6 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn usart6lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USART6 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_usart6lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SPI1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn spi1lpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_spi1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SPI4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn spi4lpen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "SPI4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_spi4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "TIM15 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim15lpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim15lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim16lpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim16lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn tim17lpen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_tim17lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SPI5 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn spi5lpen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SPI5 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_spi5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "SAI1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn sai1lpen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_sai1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "SAI2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn sai2lpen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "SAI2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_sai2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "SAI3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn sai3lpen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "SAI3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_sai3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "DFSDM1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn dfsdm1lpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "DFSDM1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_dfsdm1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "HRTIM peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn hrtimlpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_hrtimlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for C1Apb2lpenr {
        #[inline(always)]
        fn default() -> C1Apb2lpenr {
            C1Apb2lpenr(0)
        }
    }
    impl core::fmt::Debug for C1Apb2lpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("C1Apb2lpenr")
                .field("tim1lpen", &self.tim1lpen())
                .field("tim8lpen", &self.tim8lpen())
                .field("usart1lpen", &self.usart1lpen())
                .field("usart6lpen", &self.usart6lpen())
                .field("spi1lpen", &self.spi1lpen())
                .field("spi4lpen", &self.spi4lpen())
                .field("tim15lpen", &self.tim15lpen())
                .field("tim16lpen", &self.tim16lpen())
                .field("tim17lpen", &self.tim17lpen())
                .field("spi5lpen", &self.spi5lpen())
                .field("sai1lpen", &self.sai1lpen())
                .field("sai2lpen", &self.sai2lpen())
                .field("sai3lpen", &self.sai3lpen())
                .field("dfsdm1lpen", &self.dfsdm1lpen())
                .field("hrtimlpen", &self.hrtimlpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for C1Apb2lpenr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct C1Apb2lpenr {
                tim1lpen: bool,
                tim8lpen: bool,
                usart1lpen: bool,
                usart6lpen: bool,
                spi1lpen: bool,
                spi4lpen: bool,
                tim15lpen: bool,
                tim16lpen: bool,
                tim17lpen: bool,
                spi5lpen: bool,
                sai1lpen: bool,
                sai2lpen: bool,
                sai3lpen: bool,
                dfsdm1lpen: bool,
                hrtimlpen: bool,
            }
            let proxy = C1Apb2lpenr {
                tim1lpen: self.tim1lpen(),
                tim8lpen: self.tim8lpen(),
                usart1lpen: self.usart1lpen(),
                usart6lpen: self.usart6lpen(),
                spi1lpen: self.spi1lpen(),
                spi4lpen: self.spi4lpen(),
                tim15lpen: self.tim15lpen(),
                tim16lpen: self.tim16lpen(),
                tim17lpen: self.tim17lpen(),
                spi5lpen: self.spi5lpen(),
                sai1lpen: self.sai1lpen(),
                sai2lpen: self.sai2lpen(),
                sai3lpen: self.sai3lpen(),
                dfsdm1lpen: self.dfsdm1lpen(),
                hrtimlpen: self.hrtimlpen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB3 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Apb3enr(pub u32);
    impl C1Apb3enr {
        #[doc = "LTDC peripheral clock enable"]
        #[inline(always)]
        pub const fn ltdcen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC peripheral clock enable"]
        #[inline(always)]
        pub fn set_ltdcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DSI Peripheral clocks enable"]
        #[inline(always)]
        pub const fn dsien(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DSI Peripheral clocks enable"]
        #[inline(always)]
        pub fn set_dsien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "WWDG1 Clock Enable"]
        #[inline(always)]
        pub const fn wwdg1en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG1 Clock Enable"]
        #[inline(always)]
        pub fn set_wwdg1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for C1Apb3enr {
        #[inline(always)]
        fn default() -> C1Apb3enr {
            C1Apb3enr(0)
        }
    }
    impl core::fmt::Debug for C1Apb3enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("C1Apb3enr")
                .field("ltdcen", &self.ltdcen())
                .field("dsien", &self.dsien())
                .field("wwdg1en", &self.wwdg1en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for C1Apb3enr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct C1Apb3enr {
                ltdcen: bool,
                dsien: bool,
                wwdg1en: bool,
            }
            let proxy = C1Apb3enr {
                ltdcen: self.ltdcen(),
                dsien: self.dsien(),
                wwdg1en: self.wwdg1en(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB3 Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Apb3lpenr(pub u32);
    impl C1Apb3lpenr {
        #[doc = "LTDC peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn ltdclpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_ltdclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DSI Peripheral Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn dsilpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DSI Peripheral Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_dsilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "WWDG1 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn wwdg1lpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG1 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_wwdg1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for C1Apb3lpenr {
        #[inline(always)]
        fn default() -> C1Apb3lpenr {
            C1Apb3lpenr(0)
        }
    }
    impl core::fmt::Debug for C1Apb3lpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("C1Apb3lpenr")
                .field("ltdclpen", &self.ltdclpen())
                .field("dsilpen", &self.dsilpen())
                .field("wwdg1lpen", &self.wwdg1lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for C1Apb3lpenr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct C1Apb3lpenr {
                ltdclpen: bool,
                dsilpen: bool,
                wwdg1lpen: bool,
            }
            let proxy = C1Apb3lpenr {
                ltdclpen: self.ltdclpen(),
                dsilpen: self.dsilpen(),
                wwdg1lpen: self.wwdg1lpen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB4 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Apb4enr(pub u32);
    impl C1Apb4enr {
        #[doc = "SYSCFG peripheral clock enable"]
        #[inline(always)]
        pub const fn syscfgen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG peripheral clock enable"]
        #[inline(always)]
        pub fn set_syscfgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPUART1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn lpuart1en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_lpuart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SPI6 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn spi6en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI6 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_spi6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "I2C4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn i2c4en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_i2c4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LPTIM2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn lptim2en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_lptim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LPTIM3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn lptim3en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_lptim3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LPTIM4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn lptim4en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_lptim4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIM5 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn lptim5en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM5 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_lptim5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "COMP1/2 peripheral clock enable"]
        #[inline(always)]
        pub const fn comp12en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "COMP1/2 peripheral clock enable"]
        #[inline(always)]
        pub fn set_comp12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "VREF peripheral clock enable"]
        #[inline(always)]
        pub const fn vrefen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "VREF peripheral clock enable"]
        #[inline(always)]
        pub fn set_vrefen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "RTC APB Clock Enable"]
        #[inline(always)]
        pub const fn rtcapben(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "RTC APB Clock Enable"]
        #[inline(always)]
        pub fn set_rtcapben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "SAI4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn sai4en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SAI4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub fn set_sai4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for C1Apb4enr {
        #[inline(always)]
        fn default() -> C1Apb4enr {
            C1Apb4enr(0)
        }
    }
    impl core::fmt::Debug for C1Apb4enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("C1Apb4enr")
                .field("syscfgen", &self.syscfgen())
                .field("lpuart1en", &self.lpuart1en())
                .field("spi6en", &self.spi6en())
                .field("i2c4en", &self.i2c4en())
                .field("lptim2en", &self.lptim2en())
                .field("lptim3en", &self.lptim3en())
                .field("lptim4en", &self.lptim4en())
                .field("lptim5en", &self.lptim5en())
                .field("comp12en", &self.comp12en())
                .field("vrefen", &self.vrefen())
                .field("rtcapben", &self.rtcapben())
                .field("sai4en", &self.sai4en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for C1Apb4enr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct C1Apb4enr {
                syscfgen: bool,
                lpuart1en: bool,
                spi6en: bool,
                i2c4en: bool,
                lptim2en: bool,
                lptim3en: bool,
                lptim4en: bool,
                lptim5en: bool,
                comp12en: bool,
                vrefen: bool,
                rtcapben: bool,
                sai4en: bool,
            }
            let proxy = C1Apb4enr {
                syscfgen: self.syscfgen(),
                lpuart1en: self.lpuart1en(),
                spi6en: self.spi6en(),
                i2c4en: self.i2c4en(),
                lptim2en: self.lptim2en(),
                lptim3en: self.lptim3en(),
                lptim4en: self.lptim4en(),
                lptim5en: self.lptim5en(),
                comp12en: self.comp12en(),
                vrefen: self.vrefen(),
                rtcapben: self.rtcapben(),
                sai4en: self.sai4en(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB4 Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Apb4lpenr(pub u32);
    impl C1Apb4lpenr {
        #[doc = "SYSCFG peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn syscfglpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_syscfglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPUART1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn lpuart1lpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_lpuart1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SPI6 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn spi6lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI6 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_spi6lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "I2C4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn i2c4lpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_i2c4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LPTIM2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn lptim2lpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_lptim2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LPTIM3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn lptim3lpen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_lptim3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LPTIM4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn lptim4lpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_lptim4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIM5 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn lptim5lpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM5 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_lptim5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "COMP1/2 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn comp12lpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "COMP1/2 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_comp12lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "VREF peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn vreflpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "VREF peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub fn set_vreflpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "RTC APB Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn rtcapblpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "RTC APB Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_rtcapblpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "SAI4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn sai4lpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SAI4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub fn set_sai4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Digital temperature sensor block enable during CSleep Mode"]
        #[inline(always)]
        pub const fn dtslpen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Digital temperature sensor block enable during CSleep Mode"]
        #[inline(always)]
        pub fn set_dtslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for C1Apb4lpenr {
        #[inline(always)]
        fn default() -> C1Apb4lpenr {
            C1Apb4lpenr(0)
        }
    }
    impl core::fmt::Debug for C1Apb4lpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("C1Apb4lpenr")
                .field("syscfglpen", &self.syscfglpen())
                .field("lpuart1lpen", &self.lpuart1lpen())
                .field("spi6lpen", &self.spi6lpen())
                .field("i2c4lpen", &self.i2c4lpen())
                .field("lptim2lpen", &self.lptim2lpen())
                .field("lptim3lpen", &self.lptim3lpen())
                .field("lptim4lpen", &self.lptim4lpen())
                .field("lptim5lpen", &self.lptim5lpen())
                .field("comp12lpen", &self.comp12lpen())
                .field("vreflpen", &self.vreflpen())
                .field("rtcapblpen", &self.rtcapblpen())
                .field("sai4lpen", &self.sai4lpen())
                .field("dtslpen", &self.dtslpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for C1Apb4lpenr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct C1Apb4lpenr {
                syscfglpen: bool,
                lpuart1lpen: bool,
                spi6lpen: bool,
                i2c4lpen: bool,
                lptim2lpen: bool,
                lptim3lpen: bool,
                lptim4lpen: bool,
                lptim5lpen: bool,
                comp12lpen: bool,
                vreflpen: bool,
                rtcapblpen: bool,
                sai4lpen: bool,
                dtslpen: bool,
            }
            let proxy = C1Apb4lpenr {
                syscfglpen: self.syscfglpen(),
                lpuart1lpen: self.lpuart1lpen(),
                spi6lpen: self.spi6lpen(),
                i2c4lpen: self.i2c4lpen(),
                lptim2lpen: self.lptim2lpen(),
                lptim3lpen: self.lptim3lpen(),
                lptim4lpen: self.lptim4lpen(),
                lptim5lpen: self.lptim5lpen(),
                comp12lpen: self.comp12lpen(),
                vreflpen: self.vreflpen(),
                rtcapblpen: self.rtcapblpen(),
                sai4lpen: self.sai4lpen(),
                dtslpen: self.dtslpen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC Reset Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Rsr(pub u32);
    impl C1Rsr {
        #[doc = "Remove reset flag"]
        #[inline(always)]
        pub const fn rmvf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Remove reset flag"]
        #[inline(always)]
        pub fn set_rmvf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "CPU reset flag"]
        #[inline(always)]
        pub const fn cpurstf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "CPU reset flag"]
        #[inline(always)]
        pub fn set_cpurstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "D1 domain power switch reset flag"]
        #[inline(always)]
        pub const fn d1rstf(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "D1 domain power switch reset flag"]
        #[inline(always)]
        pub fn set_d1rstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "D2 domain power switch reset flag"]
        #[inline(always)]
        pub const fn d2rstf(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "D2 domain power switch reset flag"]
        #[inline(always)]
        pub fn set_d2rstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "BOR reset flag"]
        #[inline(always)]
        pub const fn borrstf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "BOR reset flag"]
        #[inline(always)]
        pub fn set_borrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Pin reset flag (NRST)"]
        #[inline(always)]
        pub const fn pinrstf(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Pin reset flag (NRST)"]
        #[inline(always)]
        pub fn set_pinrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "POR/PDR reset flag"]
        #[inline(always)]
        pub const fn porrstf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "POR/PDR reset flag"]
        #[inline(always)]
        pub fn set_porrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "System reset from CPU reset flag"]
        #[inline(always)]
        pub const fn sftrstf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "System reset from CPU reset flag"]
        #[inline(always)]
        pub fn set_sftrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Independent Watchdog reset flag"]
        #[inline(always)]
        pub const fn iwdg1rstf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Independent Watchdog reset flag"]
        #[inline(always)]
        pub fn set_iwdg1rstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Window Watchdog reset flag"]
        #[inline(always)]
        pub const fn wwdg1rstf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Window Watchdog reset flag"]
        #[inline(always)]
        pub fn set_wwdg1rstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Reset due to illegal D1 DStandby or CPU CStop flag"]
        #[inline(always)]
        pub const fn lpwrrstf(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Reset due to illegal D1 DStandby or CPU CStop flag"]
        #[inline(always)]
        pub fn set_lpwrrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for C1Rsr {
        #[inline(always)]
        fn default() -> C1Rsr {
            C1Rsr(0)
        }
    }
    impl core::fmt::Debug for C1Rsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("C1Rsr")
                .field("rmvf", &self.rmvf())
                .field("cpurstf", &self.cpurstf())
                .field("d1rstf", &self.d1rstf())
                .field("d2rstf", &self.d2rstf())
                .field("borrstf", &self.borrstf())
                .field("pinrstf", &self.pinrstf())
                .field("porrstf", &self.porrstf())
                .field("sftrstf", &self.sftrstf())
                .field("iwdg1rstf", &self.iwdg1rstf())
                .field("wwdg1rstf", &self.wwdg1rstf())
                .field("lpwrrstf", &self.lpwrrstf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for C1Rsr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct C1Rsr {
                rmvf: bool,
                cpurstf: bool,
                d1rstf: bool,
                d2rstf: bool,
                borrstf: bool,
                pinrstf: bool,
                porrstf: bool,
                sftrstf: bool,
                iwdg1rstf: bool,
                wwdg1rstf: bool,
                lpwrrstf: bool,
            }
            let proxy = C1Rsr {
                rmvf: self.rmvf(),
                cpurstf: self.cpurstf(),
                d1rstf: self.d1rstf(),
                d2rstf: self.d2rstf(),
                borrstf: self.borrstf(),
                pinrstf: self.pinrstf(),
                porrstf: self.porrstf(),
                sftrstf: self.sftrstf(),
                iwdg1rstf: self.iwdg1rstf(),
                wwdg1rstf: self.wwdg1rstf(),
                lpwrrstf: self.lpwrrstf(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC Clock Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "System clock switch"]
        #[inline(always)]
        pub const fn sw(&self) -> super::vals::Sw {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "System clock switch"]
        #[inline(always)]
        pub fn set_sw(&mut self, val: super::vals::Sw) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "System clock switch status"]
        #[inline(always)]
        pub const fn sws(&self) -> super::vals::Sw {
            let val = (self.0 >> 3usize) & 0x07;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "System clock switch status"]
        #[inline(always)]
        pub fn set_sws(&mut self, val: super::vals::Sw) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
        }
        #[doc = "System clock selection after a wake up from system Stop"]
        #[inline(always)]
        pub const fn stopwuck(&self) -> super::vals::Stopwuck {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Stopwuck::from_bits(val as u8)
        }
        #[doc = "System clock selection after a wake up from system Stop"]
        #[inline(always)]
        pub fn set_stopwuck(&mut self, val: super::vals::Stopwuck) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "Kernel clock selection after a wake up from system Stop"]
        #[inline(always)]
        pub const fn stopkerwuck(&self) -> super::vals::Stopwuck {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Stopwuck::from_bits(val as u8)
        }
        #[doc = "Kernel clock selection after a wake up from system Stop"]
        #[inline(always)]
        pub fn set_stopkerwuck(&mut self, val: super::vals::Stopwuck) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "HSE division factor for RTC clock"]
        #[inline(always)]
        pub const fn rtcpre(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "HSE division factor for RTC clock"]
        #[inline(always)]
        pub fn set_rtcpre(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "High Resolution Timer clock prescaler selection"]
        #[inline(always)]
        pub const fn hrtimsel(&self) -> super::vals::Hrtimsel {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Hrtimsel::from_bits(val as u8)
        }
        #[doc = "High Resolution Timer clock prescaler selection"]
        #[inline(always)]
        pub fn set_hrtimsel(&mut self, val: super::vals::Hrtimsel) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "Timers clocks prescaler selection"]
        #[inline(always)]
        pub const fn timpre(&self) -> super::vals::Timpre {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Timpre::from_bits(val as u8)
        }
        #[doc = "Timers clocks prescaler selection"]
        #[inline(always)]
        pub fn set_timpre(&mut self, val: super::vals::Timpre) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "MCO1 prescaler"]
        #[inline(always)]
        pub const fn mco1pre(&self) -> super::vals::Mcopre {
            let val = (self.0 >> 18usize) & 0x0f;
            super::vals::Mcopre::from_bits(val as u8)
        }
        #[doc = "MCO1 prescaler"]
        #[inline(always)]
        pub fn set_mco1pre(&mut self, val: super::vals::Mcopre) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val.to_bits() as u32) & 0x0f) << 18usize);
        }
        #[doc = "Micro-controller clock output 1"]
        #[inline(always)]
        pub const fn mco1sel(&self) -> super::vals::Mco1sel {
            let val = (self.0 >> 22usize) & 0x07;
            super::vals::Mco1sel::from_bits(val as u8)
        }
        #[doc = "Micro-controller clock output 1"]
        #[inline(always)]
        pub fn set_mco1sel(&mut self, val: super::vals::Mco1sel) {
            self.0 = (self.0 & !(0x07 << 22usize)) | (((val.to_bits() as u32) & 0x07) << 22usize);
        }
        #[doc = "MCO2 prescaler"]
        #[inline(always)]
        pub const fn mco2pre(&self) -> super::vals::Mcopre {
            let val = (self.0 >> 25usize) & 0x0f;
            super::vals::Mcopre::from_bits(val as u8)
        }
        #[doc = "MCO2 prescaler"]
        #[inline(always)]
        pub fn set_mco2pre(&mut self, val: super::vals::Mcopre) {
            self.0 = (self.0 & !(0x0f << 25usize)) | (((val.to_bits() as u32) & 0x0f) << 25usize);
        }
        #[doc = "Micro-controller clock output 2"]
        #[inline(always)]
        pub const fn mco2sel(&self) -> super::vals::Mco2sel {
            let val = (self.0 >> 29usize) & 0x07;
            super::vals::Mco2sel::from_bits(val as u8)
        }
        #[doc = "Micro-controller clock output 2"]
        #[inline(always)]
        pub fn set_mco2sel(&mut self, val: super::vals::Mco2sel) {
            self.0 = (self.0 & !(0x07 << 29usize)) | (((val.to_bits() as u32) & 0x07) << 29usize);
        }
    }
    impl Default for Cfgr {
        #[inline(always)]
        fn default() -> Cfgr {
            Cfgr(0)
        }
    }
    impl core::fmt::Debug for Cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr")
                .field("sw", &self.sw())
                .field("sws", &self.sws())
                .field("stopwuck", &self.stopwuck())
                .field("stopkerwuck", &self.stopkerwuck())
                .field("rtcpre", &self.rtcpre())
                .field("hrtimsel", &self.hrtimsel())
                .field("timpre", &self.timpre())
                .field("mco1pre", &self.mco1pre())
                .field("mco1sel", &self.mco1sel())
                .field("mco2pre", &self.mco2pre())
                .field("mco2sel", &self.mco2sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cfgr {
                sw: super::vals::Sw,
                sws: super::vals::Sw,
                stopwuck: super::vals::Stopwuck,
                stopkerwuck: super::vals::Stopwuck,
                rtcpre: u8,
                hrtimsel: super::vals::Hrtimsel,
                timpre: super::vals::Timpre,
                mco1pre: super::vals::Mcopre,
                mco1sel: super::vals::Mco1sel,
                mco2pre: super::vals::Mcopre,
                mco2sel: super::vals::Mco2sel,
            }
            let proxy = Cfgr {
                sw: self.sw(),
                sws: self.sws(),
                stopwuck: self.stopwuck(),
                stopkerwuck: self.stopkerwuck(),
                rtcpre: self.rtcpre(),
                hrtimsel: self.hrtimsel(),
                timpre: self.timpre(),
                mco1pre: self.mco1pre(),
                mco1sel: self.mco1sel(),
                mco2pre: self.mco2pre(),
                mco2sel: self.mco2sel(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC Clock Source Interrupt Clear Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cicr(pub u32);
    impl Cicr {
        #[doc = "LSI ready Interrupt Clear"]
        #[inline(always)]
        pub const fn lsirdyc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready Interrupt Clear"]
        #[inline(always)]
        pub fn set_lsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready Interrupt Clear"]
        #[inline(always)]
        pub const fn lserdyc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready Interrupt Clear"]
        #[inline(always)]
        pub fn set_lserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI ready Interrupt Clear"]
        #[inline(always)]
        pub const fn hsirdyc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready Interrupt Clear"]
        #[inline(always)]
        pub fn set_hsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSE ready Interrupt Clear"]
        #[inline(always)]
        pub const fn hserdyc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready Interrupt Clear"]
        #[inline(always)]
        pub fn set_hserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CSI ready Interrupt Clear"]
        #[inline(always)]
        pub const fn hse_ready_interrupt_clear(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CSI ready Interrupt Clear"]
        #[inline(always)]
        pub fn set_hse_ready_interrupt_clear(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "RC48 ready Interrupt Clear"]
        #[inline(always)]
        pub const fn hsi48rdyc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RC48 ready Interrupt Clear"]
        #[inline(always)]
        pub fn set_hsi48rdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PLL1 ready Interrupt Clear"]
        #[inline(always)]
        pub const fn pllrdyc(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 ready Interrupt Clear"]
        #[inline(always)]
        pub fn set_pllrdyc(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "LSE clock security system Interrupt Clear"]
        #[inline(always)]
        pub const fn lsecssc(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock security system Interrupt Clear"]
        #[inline(always)]
        pub fn set_lsecssc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "HSE clock security system Interrupt Clear"]
        #[inline(always)]
        pub const fn hsecssc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock security system Interrupt Clear"]
        #[inline(always)]
        pub fn set_hsecssc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Cicr {
        #[inline(always)]
        fn default() -> Cicr {
            Cicr(0)
        }
    }
    impl core::fmt::Debug for Cicr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cicr")
                .field("lsirdyc", &self.lsirdyc())
                .field("lserdyc", &self.lserdyc())
                .field("hsirdyc", &self.hsirdyc())
                .field("hserdyc", &self.hserdyc())
                .field("hse_ready_interrupt_clear", &self.hse_ready_interrupt_clear())
                .field("hsi48rdyc", &self.hsi48rdyc())
                .field(
                    "pllrdyc",
                    &[self.pllrdyc(0usize), self.pllrdyc(1usize), self.pllrdyc(2usize)],
                )
                .field("lsecssc", &self.lsecssc())
                .field("hsecssc", &self.hsecssc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cicr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cicr {
                lsirdyc: bool,
                lserdyc: bool,
                hsirdyc: bool,
                hserdyc: bool,
                hse_ready_interrupt_clear: bool,
                hsi48rdyc: bool,
                pllrdyc: [bool; 3usize],
                lsecssc: bool,
                hsecssc: bool,
            }
            let proxy = Cicr {
                lsirdyc: self.lsirdyc(),
                lserdyc: self.lserdyc(),
                hsirdyc: self.hsirdyc(),
                hserdyc: self.hserdyc(),
                hse_ready_interrupt_clear: self.hse_ready_interrupt_clear(),
                hsi48rdyc: self.hsi48rdyc(),
                pllrdyc: [self.pllrdyc(0usize), self.pllrdyc(1usize), self.pllrdyc(2usize)],
                lsecssc: self.lsecssc(),
                hsecssc: self.hsecssc(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC Clock Source Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cier(pub u32);
    impl Cier {
        #[doc = "LSI ready Interrupt Enable"]
        #[inline(always)]
        pub const fn lsirdyie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready Interrupt Enable"]
        #[inline(always)]
        pub fn set_lsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready Interrupt Enable"]
        #[inline(always)]
        pub const fn lserdyie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready Interrupt Enable"]
        #[inline(always)]
        pub fn set_lserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI ready Interrupt Enable"]
        #[inline(always)]
        pub const fn hsirdyie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready Interrupt Enable"]
        #[inline(always)]
        pub fn set_hsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSE ready Interrupt Enable"]
        #[inline(always)]
        pub const fn hserdyie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready Interrupt Enable"]
        #[inline(always)]
        pub fn set_hserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CSI ready Interrupt Enable"]
        #[inline(always)]
        pub const fn csirdyie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CSI ready Interrupt Enable"]
        #[inline(always)]
        pub fn set_csirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "RC48 ready Interrupt Enable"]
        #[inline(always)]
        pub const fn hsi48rdyie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RC48 ready Interrupt Enable"]
        #[inline(always)]
        pub fn set_hsi48rdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PLL1 ready Interrupt Enable"]
        #[inline(always)]
        pub const fn pllrdyie(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 ready Interrupt Enable"]
        #[inline(always)]
        pub fn set_pllrdyie(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "LSE clock security system Interrupt Enable"]
        #[inline(always)]
        pub const fn lsecssie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock security system Interrupt Enable"]
        #[inline(always)]
        pub fn set_lsecssie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Cier {
        #[inline(always)]
        fn default() -> Cier {
            Cier(0)
        }
    }
    impl core::fmt::Debug for Cier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cier")
                .field("lsirdyie", &self.lsirdyie())
                .field("lserdyie", &self.lserdyie())
                .field("hsirdyie", &self.hsirdyie())
                .field("hserdyie", &self.hserdyie())
                .field("csirdyie", &self.csirdyie())
                .field("hsi48rdyie", &self.hsi48rdyie())
                .field(
                    "pllrdyie",
                    &[self.pllrdyie(0usize), self.pllrdyie(1usize), self.pllrdyie(2usize)],
                )
                .field("lsecssie", &self.lsecssie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cier {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cier {
                lsirdyie: bool,
                lserdyie: bool,
                hsirdyie: bool,
                hserdyie: bool,
                csirdyie: bool,
                hsi48rdyie: bool,
                pllrdyie: [bool; 3usize],
                lsecssie: bool,
            }
            let proxy = Cier {
                lsirdyie: self.lsirdyie(),
                lserdyie: self.lserdyie(),
                hsirdyie: self.hsirdyie(),
                hserdyie: self.hserdyie(),
                csirdyie: self.csirdyie(),
                hsi48rdyie: self.hsi48rdyie(),
                pllrdyie: [self.pllrdyie(0usize), self.pllrdyie(1usize), self.pllrdyie(2usize)],
                lsecssie: self.lsecssie(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC Clock Source Interrupt Flag Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cifr(pub u32);
    impl Cifr {
        #[doc = "LSI ready Interrupt Flag"]
        #[inline(always)]
        pub const fn lsirdyf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready Interrupt Flag"]
        #[inline(always)]
        pub fn set_lsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready Interrupt Flag"]
        #[inline(always)]
        pub const fn lserdyf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready Interrupt Flag"]
        #[inline(always)]
        pub fn set_lserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI ready Interrupt Flag"]
        #[inline(always)]
        pub const fn hsirdyf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready Interrupt Flag"]
        #[inline(always)]
        pub fn set_hsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSE ready Interrupt Flag"]
        #[inline(always)]
        pub const fn hserdyf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready Interrupt Flag"]
        #[inline(always)]
        pub fn set_hserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CSI ready Interrupt Flag"]
        #[inline(always)]
        pub const fn csirdy(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CSI ready Interrupt Flag"]
        #[inline(always)]
        pub fn set_csirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "RC48 ready Interrupt Flag"]
        #[inline(always)]
        pub const fn hsi48rdyf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RC48 ready Interrupt Flag"]
        #[inline(always)]
        pub fn set_hsi48rdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PLL1 ready Interrupt Flag"]
        #[inline(always)]
        pub const fn pllrdyf(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 ready Interrupt Flag"]
        #[inline(always)]
        pub fn set_pllrdyf(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "LSE clock security system Interrupt Flag"]
        #[inline(always)]
        pub const fn lsecssf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock security system Interrupt Flag"]
        #[inline(always)]
        pub fn set_lsecssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "HSE clock security system Interrupt Flag"]
        #[inline(always)]
        pub const fn hsecssf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock security system Interrupt Flag"]
        #[inline(always)]
        pub fn set_hsecssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Cifr {
        #[inline(always)]
        fn default() -> Cifr {
            Cifr(0)
        }
    }
    impl core::fmt::Debug for Cifr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cifr")
                .field("lsirdyf", &self.lsirdyf())
                .field("lserdyf", &self.lserdyf())
                .field("hsirdyf", &self.hsirdyf())
                .field("hserdyf", &self.hserdyf())
                .field("csirdy", &self.csirdy())
                .field("hsi48rdyf", &self.hsi48rdyf())
                .field(
                    "pllrdyf",
                    &[self.pllrdyf(0usize), self.pllrdyf(1usize), self.pllrdyf(2usize)],
                )
                .field("lsecssf", &self.lsecssf())
                .field("hsecssf", &self.hsecssf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cifr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cifr {
                lsirdyf: bool,
                lserdyf: bool,
                hsirdyf: bool,
                hserdyf: bool,
                csirdy: bool,
                hsi48rdyf: bool,
                pllrdyf: [bool; 3usize],
                lsecssf: bool,
                hsecssf: bool,
            }
            let proxy = Cifr {
                lsirdyf: self.lsirdyf(),
                lserdyf: self.lserdyf(),
                hsirdyf: self.hsirdyf(),
                hserdyf: self.hserdyf(),
                csirdy: self.csirdy(),
                hsi48rdyf: self.hsi48rdyf(),
                pllrdyf: [self.pllrdyf(0usize), self.pllrdyf(1usize), self.pllrdyf(2usize)],
                lsecssf: self.lsecssf(),
                hsecssf: self.hsecssf(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "clock control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Internal high-speed clock enable"]
        #[inline(always)]
        pub const fn hsion(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Internal high-speed clock enable"]
        #[inline(always)]
        pub fn set_hsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "High Speed Internal clock enable in Stop mode"]
        #[inline(always)]
        pub const fn hsikeron(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "High Speed Internal clock enable in Stop mode"]
        #[inline(always)]
        pub fn set_hsikeron(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI clock ready flag"]
        #[inline(always)]
        pub const fn hsirdy(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSI clock ready flag"]
        #[inline(always)]
        pub fn set_hsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSI clock divider"]
        #[inline(always)]
        pub const fn hsidiv(&self) -> super::vals::Hsidiv {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Hsidiv::from_bits(val as u8)
        }
        #[doc = "HSI clock divider"]
        #[inline(always)]
        pub fn set_hsidiv(&mut self, val: super::vals::Hsidiv) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "HSI divider flag"]
        #[inline(always)]
        pub const fn hsidivf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HSI divider flag"]
        #[inline(always)]
        pub fn set_hsidivf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "CSI clock enable"]
        #[inline(always)]
        pub const fn csion(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CSI clock enable"]
        #[inline(always)]
        pub fn set_csion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "CSI clock ready flag"]
        #[inline(always)]
        pub const fn csirdy(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "CSI clock ready flag"]
        #[inline(always)]
        pub fn set_csirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CSI clock enable in Stop mode"]
        #[inline(always)]
        pub const fn csikeron(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "CSI clock enable in Stop mode"]
        #[inline(always)]
        pub fn set_csikeron(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "RC48 clock enable"]
        #[inline(always)]
        pub const fn hsi48on(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "RC48 clock enable"]
        #[inline(always)]
        pub fn set_hsi48on(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "RC48 clock ready flag"]
        #[inline(always)]
        pub const fn hsi48rdy(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "RC48 clock ready flag"]
        #[inline(always)]
        pub fn set_hsi48rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "D1 domain clocks ready flag"]
        #[inline(always)]
        pub const fn d1ckrdy(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "D1 domain clocks ready flag"]
        #[inline(always)]
        pub fn set_d1ckrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "D2 domain clocks ready flag"]
        #[inline(always)]
        pub const fn d2ckrdy(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "D2 domain clocks ready flag"]
        #[inline(always)]
        pub fn set_d2ckrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
        #[doc = "HSE clock bypass"]
        #[inline(always)]
        pub const fn hsebyp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock bypass"]
        #[inline(always)]
        pub fn set_hsebyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "HSE Clock Security System enable"]
        #[inline(always)]
        pub const fn hsecsson(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "HSE Clock Security System enable"]
        #[inline(always)]
        pub fn set_hsecsson(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "PLL1 enable"]
        #[inline(always)]
        pub const fn pllon(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 24usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 enable"]
        #[inline(always)]
        pub fn set_pllon(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 24usize + n * 2usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "PLL1 clock ready flag"]
        #[inline(always)]
        pub const fn pllrdy(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 25usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 clock ready flag"]
        #[inline(always)]
        pub fn set_pllrdy(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 25usize + n * 2usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
                .field("hsion", &self.hsion())
                .field("hsikeron", &self.hsikeron())
                .field("hsirdy", &self.hsirdy())
                .field("hsidiv", &self.hsidiv())
                .field("hsidivf", &self.hsidivf())
                .field("csion", &self.csion())
                .field("csirdy", &self.csirdy())
                .field("csikeron", &self.csikeron())
                .field("hsi48on", &self.hsi48on())
                .field("hsi48rdy", &self.hsi48rdy())
                .field("d1ckrdy", &self.d1ckrdy())
                .field("d2ckrdy", &self.d2ckrdy())
                .field("hseon", &self.hseon())
                .field("hserdy", &self.hserdy())
                .field("hsebyp", &self.hsebyp())
                .field("hsecsson", &self.hsecsson())
                .field("pllon", &[self.pllon(0usize), self.pllon(1usize), self.pllon(2usize)])
                .field(
                    "pllrdy",
                    &[self.pllrdy(0usize), self.pllrdy(1usize), self.pllrdy(2usize)],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cr {
                hsion: bool,
                hsikeron: bool,
                hsirdy: bool,
                hsidiv: super::vals::Hsidiv,
                hsidivf: bool,
                csion: bool,
                csirdy: bool,
                csikeron: bool,
                hsi48on: bool,
                hsi48rdy: bool,
                d1ckrdy: bool,
                d2ckrdy: bool,
                hseon: bool,
                hserdy: bool,
                hsebyp: bool,
                hsecsson: bool,
                pllon: [bool; 3usize],
                pllrdy: [bool; 3usize],
            }
            let proxy = Cr {
                hsion: self.hsion(),
                hsikeron: self.hsikeron(),
                hsirdy: self.hsirdy(),
                hsidiv: self.hsidiv(),
                hsidivf: self.hsidivf(),
                csion: self.csion(),
                csirdy: self.csirdy(),
                csikeron: self.csikeron(),
                hsi48on: self.hsi48on(),
                hsi48rdy: self.hsi48rdy(),
                d1ckrdy: self.d1ckrdy(),
                d2ckrdy: self.d2ckrdy(),
                hseon: self.hseon(),
                hserdy: self.hserdy(),
                hsebyp: self.hsebyp(),
                hsecsson: self.hsecsson(),
                pllon: [self.pllon(0usize), self.pllon(1usize), self.pllon(2usize)],
                pllrdy: [self.pllrdy(0usize), self.pllrdy(1usize), self.pllrdy(2usize)],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC Clock Recovery RC Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crrcr(pub u32);
    impl Crrcr {
        #[doc = "Internal RC 48 MHz clock calibration"]
        #[inline(always)]
        pub const fn hsi48cal(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Internal RC 48 MHz clock calibration"]
        #[inline(always)]
        pub fn set_hsi48cal(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for Crrcr {
        #[inline(always)]
        fn default() -> Crrcr {
            Crrcr(0)
        }
    }
    impl core::fmt::Debug for Crrcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Crrcr").field("hsi48cal", &self.hsi48cal()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Crrcr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Crrcr {
                hsi48cal: u16,
            }
            let proxy = Crrcr {
                hsi48cal: self.hsi48cal(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC CSI configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csicfgr(pub u32);
    impl Csicfgr {
        #[doc = "CSI clock calibration"]
        #[inline(always)]
        pub const fn csical(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "CSI clock calibration"]
        #[inline(always)]
        pub fn set_csical(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "CSI clock trimming"]
        #[inline(always)]
        pub const fn csitrim(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "CSI clock trimming"]
        #[inline(always)]
        pub fn set_csitrim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
    }
    impl Default for Csicfgr {
        #[inline(always)]
        fn default() -> Csicfgr {
            Csicfgr(0)
        }
    }
    impl core::fmt::Debug for Csicfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csicfgr")
                .field("csical", &self.csical())
                .field("csitrim", &self.csitrim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csicfgr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Csicfgr {
                csical: u16,
                csitrim: u8,
            }
            let proxy = Csicfgr {
                csical: self.csical(),
                csitrim: self.csitrim(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC Clock Control and Status Register"]
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
    }
    impl Default for Csr {
        #[inline(always)]
        fn default() -> Csr {
            Csr(0)
        }
    }
    impl core::fmt::Debug for Csr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csr")
                .field("lsion", &self.lsion())
                .field("lsirdy", &self.lsirdy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Csr {
                lsion: bool,
                lsirdy: bool,
            }
            let proxy = Csr {
                lsion: self.lsion(),
                lsirdy: self.lsirdy(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC Domain 1 Kernel Clock Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct D1ccipr(pub u32);
    impl D1ccipr {
        #[doc = "FMC kernel clock source selection"]
        #[inline(always)]
        pub const fn fmcsel(&self) -> super::vals::Fmcsel {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Fmcsel::from_bits(val as u8)
        }
        #[doc = "FMC kernel clock source selection"]
        #[inline(always)]
        pub fn set_fmcsel(&mut self, val: super::vals::Fmcsel) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "QUADSPI kernel clock source selection"]
        #[inline(always)]
        pub const fn quadspisel(&self) -> super::vals::Fmcsel {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Fmcsel::from_bits(val as u8)
        }
        #[doc = "QUADSPI kernel clock source selection"]
        #[inline(always)]
        pub fn set_quadspisel(&mut self, val: super::vals::Fmcsel) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "SDMMC kernel clock source selection"]
        #[inline(always)]
        pub const fn sdmmcsel(&self) -> super::vals::Sdmmcsel {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Sdmmcsel::from_bits(val as u8)
        }
        #[doc = "SDMMC kernel clock source selection"]
        #[inline(always)]
        pub fn set_sdmmcsel(&mut self, val: super::vals::Sdmmcsel) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "per_ck clock source selection"]
        #[inline(always)]
        pub const fn persel(&self) -> super::vals::Persel {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Persel::from_bits(val as u8)
        }
        #[doc = "per_ck clock source selection"]
        #[inline(always)]
        pub fn set_persel(&mut self, val: super::vals::Persel) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
    }
    impl Default for D1ccipr {
        #[inline(always)]
        fn default() -> D1ccipr {
            D1ccipr(0)
        }
    }
    impl core::fmt::Debug for D1ccipr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("D1ccipr")
                .field("fmcsel", &self.fmcsel())
                .field("quadspisel", &self.quadspisel())
                .field("sdmmcsel", &self.sdmmcsel())
                .field("persel", &self.persel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for D1ccipr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct D1ccipr {
                fmcsel: super::vals::Fmcsel,
                quadspisel: super::vals::Fmcsel,
                sdmmcsel: super::vals::Sdmmcsel,
                persel: super::vals::Persel,
            }
            let proxy = D1ccipr {
                fmcsel: self.fmcsel(),
                quadspisel: self.quadspisel(),
                sdmmcsel: self.sdmmcsel(),
                persel: self.persel(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC Domain 1 Clock Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct D1cfgr(pub u32);
    impl D1cfgr {
        #[doc = "D1 domain AHB prescaler"]
        #[inline(always)]
        pub const fn hpre(&self) -> super::vals::Hpre {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Hpre::from_bits(val as u8)
        }
        #[doc = "D1 domain AHB prescaler"]
        #[inline(always)]
        pub fn set_hpre(&mut self, val: super::vals::Hpre) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "D1 domain APB3 prescaler"]
        #[inline(always)]
        pub const fn d1ppre(&self) -> super::vals::Ppre {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "D1 domain APB3 prescaler"]
        #[inline(always)]
        pub fn set_d1ppre(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "D1 domain Core prescaler"]
        #[inline(always)]
        pub const fn d1cpre(&self) -> super::vals::Hpre {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::Hpre::from_bits(val as u8)
        }
        #[doc = "D1 domain Core prescaler"]
        #[inline(always)]
        pub fn set_d1cpre(&mut self, val: super::vals::Hpre) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
        }
    }
    impl Default for D1cfgr {
        #[inline(always)]
        fn default() -> D1cfgr {
            D1cfgr(0)
        }
    }
    impl core::fmt::Debug for D1cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("D1cfgr")
                .field("hpre", &self.hpre())
                .field("d1ppre", &self.d1ppre())
                .field("d1cpre", &self.d1cpre())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for D1cfgr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct D1cfgr {
                hpre: super::vals::Hpre,
                d1ppre: super::vals::Ppre,
                d1cpre: super::vals::Hpre,
            }
            let proxy = D1cfgr {
                hpre: self.hpre(),
                d1ppre: self.d1ppre(),
                d1cpre: self.d1cpre(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC Domain 2 Kernel Clock Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct D2ccip1r(pub u32);
    impl D2ccip1r {
        #[doc = "SAI1 and DFSDM1 kernel Aclk clock source selection"]
        #[inline(always)]
        pub const fn sai1sel(&self) -> super::vals::Saisel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Saisel::from_bits(val as u8)
        }
        #[doc = "SAI1 and DFSDM1 kernel Aclk clock source selection"]
        #[inline(always)]
        pub fn set_sai1sel(&mut self, val: super::vals::Saisel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "SAI2 and SAI3 kernel clock source selection"]
        #[inline(always)]
        pub const fn sai23sel(&self) -> super::vals::Saisel {
            let val = (self.0 >> 6usize) & 0x07;
            super::vals::Saisel::from_bits(val as u8)
        }
        #[doc = "SAI2 and SAI3 kernel clock source selection"]
        #[inline(always)]
        pub fn set_sai23sel(&mut self, val: super::vals::Saisel) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val.to_bits() as u32) & 0x07) << 6usize);
        }
        #[doc = "SPI/I2S1,2 and 3 kernel clock source selection"]
        #[inline(always)]
        pub const fn spi123sel(&self) -> super::vals::Saisel {
            let val = (self.0 >> 12usize) & 0x07;
            super::vals::Saisel::from_bits(val as u8)
        }
        #[doc = "SPI/I2S1,2 and 3 kernel clock source selection"]
        #[inline(always)]
        pub fn set_spi123sel(&mut self, val: super::vals::Saisel) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
        }
        #[doc = "SPI4 and 5 kernel clock source selection"]
        #[inline(always)]
        pub const fn spi45sel(&self) -> super::vals::Spi45sel {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::Spi45sel::from_bits(val as u8)
        }
        #[doc = "SPI4 and 5 kernel clock source selection"]
        #[inline(always)]
        pub fn set_spi45sel(&mut self, val: super::vals::Spi45sel) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
        #[doc = "SPDIFRX kernel clock source selection"]
        #[inline(always)]
        pub const fn spdifrxsel(&self) -> super::vals::Spdifrxsel {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::Spdifrxsel::from_bits(val as u8)
        }
        #[doc = "SPDIFRX kernel clock source selection"]
        #[inline(always)]
        pub fn set_spdifrxsel(&mut self, val: super::vals::Spdifrxsel) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "DFSDM1 kernel Clk clock source selection"]
        #[inline(always)]
        pub const fn dfsdm1sel(&self) -> super::vals::Dfsdmsel {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Dfsdmsel::from_bits(val as u8)
        }
        #[doc = "DFSDM1 kernel Clk clock source selection"]
        #[inline(always)]
        pub fn set_dfsdm1sel(&mut self, val: super::vals::Dfsdmsel) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
        #[doc = "FDCAN kernel clock source selection"]
        #[inline(always)]
        pub const fn fdcansel(&self) -> super::vals::Fdcansel {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Fdcansel::from_bits(val as u8)
        }
        #[doc = "FDCAN kernel clock source selection"]
        #[inline(always)]
        pub fn set_fdcansel(&mut self, val: super::vals::Fdcansel) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "SWPMI kernel clock source selection"]
        #[inline(always)]
        pub const fn swpmisel(&self) -> super::vals::Swpmisel {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Swpmisel::from_bits(val as u8)
        }
        #[doc = "SWPMI kernel clock source selection"]
        #[inline(always)]
        pub fn set_swpmisel(&mut self, val: super::vals::Swpmisel) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for D2ccip1r {
        #[inline(always)]
        fn default() -> D2ccip1r {
            D2ccip1r(0)
        }
    }
    impl core::fmt::Debug for D2ccip1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("D2ccip1r")
                .field("sai1sel", &self.sai1sel())
                .field("sai23sel", &self.sai23sel())
                .field("spi123sel", &self.spi123sel())
                .field("spi45sel", &self.spi45sel())
                .field("spdifrxsel", &self.spdifrxsel())
                .field("dfsdm1sel", &self.dfsdm1sel())
                .field("fdcansel", &self.fdcansel())
                .field("swpmisel", &self.swpmisel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for D2ccip1r {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct D2ccip1r {
                sai1sel: super::vals::Saisel,
                sai23sel: super::vals::Saisel,
                spi123sel: super::vals::Saisel,
                spi45sel: super::vals::Spi45sel,
                spdifrxsel: super::vals::Spdifrxsel,
                dfsdm1sel: super::vals::Dfsdmsel,
                fdcansel: super::vals::Fdcansel,
                swpmisel: super::vals::Swpmisel,
            }
            let proxy = D2ccip1r {
                sai1sel: self.sai1sel(),
                sai23sel: self.sai23sel(),
                spi123sel: self.spi123sel(),
                spi45sel: self.spi45sel(),
                spdifrxsel: self.spdifrxsel(),
                dfsdm1sel: self.dfsdm1sel(),
                fdcansel: self.fdcansel(),
                swpmisel: self.swpmisel(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC Domain 2 Kernel Clock Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct D2ccip2r(pub u32);
    impl D2ccip2r {
        #[doc = "USART2/3, UART4,5, 7/8 (APB1) kernel clock source selection"]
        #[inline(always)]
        pub const fn usart234578sel(&self) -> super::vals::Usart234578sel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Usart234578sel::from_bits(val as u8)
        }
        #[doc = "USART2/3, UART4,5, 7/8 (APB1) kernel clock source selection"]
        #[inline(always)]
        pub fn set_usart234578sel(&mut self, val: super::vals::Usart234578sel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "USART1, 6, 9 and 10 kernel clock source selection"]
        #[inline(always)]
        pub const fn usart16910sel(&self) -> super::vals::Usart16910sel {
            let val = (self.0 >> 3usize) & 0x07;
            super::vals::Usart16910sel::from_bits(val as u8)
        }
        #[doc = "USART1, 6, 9 and 10 kernel clock source selection"]
        #[inline(always)]
        pub fn set_usart16910sel(&mut self, val: super::vals::Usart16910sel) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
        }
        #[doc = "RNG kernel clock source selection"]
        #[inline(always)]
        pub const fn rngsel(&self) -> super::vals::Rngsel {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Rngsel::from_bits(val as u8)
        }
        #[doc = "RNG kernel clock source selection"]
        #[inline(always)]
        pub fn set_rngsel(&mut self, val: super::vals::Rngsel) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "I2C1,2,3 kernel clock source selection"]
        #[inline(always)]
        pub const fn i2c1235sel(&self) -> super::vals::I2c1235sel {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::I2c1235sel::from_bits(val as u8)
        }
        #[doc = "I2C1,2,3 kernel clock source selection"]
        #[inline(always)]
        pub fn set_i2c1235sel(&mut self, val: super::vals::I2c1235sel) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "USBOTG 1 and 2 kernel clock source selection"]
        #[inline(always)]
        pub const fn usbsel(&self) -> super::vals::Usbsel {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::Usbsel::from_bits(val as u8)
        }
        #[doc = "USBOTG 1 and 2 kernel clock source selection"]
        #[inline(always)]
        pub fn set_usbsel(&mut self, val: super::vals::Usbsel) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "HDMI-CEC kernel clock source selection"]
        #[inline(always)]
        pub const fn cecsel(&self) -> super::vals::Cecsel {
            let val = (self.0 >> 22usize) & 0x03;
            super::vals::Cecsel::from_bits(val as u8)
        }
        #[doc = "HDMI-CEC kernel clock source selection"]
        #[inline(always)]
        pub fn set_cecsel(&mut self, val: super::vals::Cecsel) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
        }
        #[doc = "LPTIM1 kernel clock source selection"]
        #[inline(always)]
        pub const fn lptim1sel(&self) -> super::vals::Lptim1sel {
            let val = (self.0 >> 28usize) & 0x07;
            super::vals::Lptim1sel::from_bits(val as u8)
        }
        #[doc = "LPTIM1 kernel clock source selection"]
        #[inline(always)]
        pub fn set_lptim1sel(&mut self, val: super::vals::Lptim1sel) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
        }
    }
    impl Default for D2ccip2r {
        #[inline(always)]
        fn default() -> D2ccip2r {
            D2ccip2r(0)
        }
    }
    impl core::fmt::Debug for D2ccip2r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("D2ccip2r")
                .field("usart234578sel", &self.usart234578sel())
                .field("usart16910sel", &self.usart16910sel())
                .field("rngsel", &self.rngsel())
                .field("i2c1235sel", &self.i2c1235sel())
                .field("usbsel", &self.usbsel())
                .field("cecsel", &self.cecsel())
                .field("lptim1sel", &self.lptim1sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for D2ccip2r {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct D2ccip2r {
                usart234578sel: super::vals::Usart234578sel,
                usart16910sel: super::vals::Usart16910sel,
                rngsel: super::vals::Rngsel,
                i2c1235sel: super::vals::I2c1235sel,
                usbsel: super::vals::Usbsel,
                cecsel: super::vals::Cecsel,
                lptim1sel: super::vals::Lptim1sel,
            }
            let proxy = D2ccip2r {
                usart234578sel: self.usart234578sel(),
                usart16910sel: self.usart16910sel(),
                rngsel: self.rngsel(),
                i2c1235sel: self.i2c1235sel(),
                usbsel: self.usbsel(),
                cecsel: self.cecsel(),
                lptim1sel: self.lptim1sel(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC Domain 2 Clock Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct D2cfgr(pub u32);
    impl D2cfgr {
        #[doc = "D2 domain APB1 prescaler"]
        #[inline(always)]
        pub const fn d2ppre1(&self) -> super::vals::Ppre {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "D2 domain APB1 prescaler"]
        #[inline(always)]
        pub fn set_d2ppre1(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "D2 domain APB2 prescaler"]
        #[inline(always)]
        pub const fn d2ppre2(&self) -> super::vals::Ppre {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "D2 domain APB2 prescaler"]
        #[inline(always)]
        pub fn set_d2ppre2(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
    }
    impl Default for D2cfgr {
        #[inline(always)]
        fn default() -> D2cfgr {
            D2cfgr(0)
        }
    }
    impl core::fmt::Debug for D2cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("D2cfgr")
                .field("d2ppre1", &self.d2ppre1())
                .field("d2ppre2", &self.d2ppre2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for D2cfgr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct D2cfgr {
                d2ppre1: super::vals::Ppre,
                d2ppre2: super::vals::Ppre,
            }
            let proxy = D2cfgr {
                d2ppre1: self.d2ppre1(),
                d2ppre2: self.d2ppre2(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC D3 Autonomous mode Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct D3amr(pub u32);
    impl D3amr {
        #[doc = "BDMA and DMAMUX Autonomous mode enable"]
        #[inline(always)]
        pub const fn bdmaamen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "BDMA and DMAMUX Autonomous mode enable"]
        #[inline(always)]
        pub fn set_bdmaamen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LPUART1 Autonomous mode enable"]
        #[inline(always)]
        pub const fn lpuart1amen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 Autonomous mode enable"]
        #[inline(always)]
        pub fn set_lpuart1amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SPI6 Autonomous mode enable"]
        #[inline(always)]
        pub const fn spi6amen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI6 Autonomous mode enable"]
        #[inline(always)]
        pub fn set_spi6amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "I2C4 Autonomous mode enable"]
        #[inline(always)]
        pub const fn i2c4amen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 Autonomous mode enable"]
        #[inline(always)]
        pub fn set_i2c4amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LPTIM2 Autonomous mode enable"]
        #[inline(always)]
        pub const fn lptim2amen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 Autonomous mode enable"]
        #[inline(always)]
        pub fn set_lptim2amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LPTIM3 Autonomous mode enable"]
        #[inline(always)]
        pub const fn lptim3amen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3 Autonomous mode enable"]
        #[inline(always)]
        pub fn set_lptim3amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LPTIM4 Autonomous mode enable"]
        #[inline(always)]
        pub const fn lptim4amen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM4 Autonomous mode enable"]
        #[inline(always)]
        pub fn set_lptim4amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIM5 Autonomous mode enable"]
        #[inline(always)]
        pub const fn lptim5amen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM5 Autonomous mode enable"]
        #[inline(always)]
        pub fn set_lptim5amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "DAC2 (containing one converter) Autonomous mode enable"]
        #[inline(always)]
        pub const fn dac2amen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "DAC2 (containing one converter) Autonomous mode enable"]
        #[inline(always)]
        pub fn set_dac2amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "COMP12 Autonomous mode enable"]
        #[inline(always)]
        pub const fn comp12amen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "COMP12 Autonomous mode enable"]
        #[inline(always)]
        pub fn set_comp12amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "VREF Autonomous mode enable"]
        #[inline(always)]
        pub const fn vrefamen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "VREF Autonomous mode enable"]
        #[inline(always)]
        pub fn set_vrefamen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "RTC Autonomous mode enable"]
        #[inline(always)]
        pub const fn rtcamen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "RTC Autonomous mode enable"]
        #[inline(always)]
        pub fn set_rtcamen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "CRC Autonomous mode enable"]
        #[inline(always)]
        pub const fn crcamen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "CRC Autonomous mode enable"]
        #[inline(always)]
        pub fn set_crcamen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "SAI4 Autonomous mode enable"]
        #[inline(always)]
        pub const fn sai4amen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SAI4 Autonomous mode enable"]
        #[inline(always)]
        pub fn set_sai4amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ADC3 Autonomous mode enable"]
        #[inline(always)]
        pub const fn adc3amen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ADC3 Autonomous mode enable"]
        #[inline(always)]
        pub fn set_adc3amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Digital temperature sensor Autonomous mode enable"]
        #[inline(always)]
        pub const fn dtsamen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Digital temperature sensor Autonomous mode enable"]
        #[inline(always)]
        pub fn set_dtsamen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Backup RAM Autonomous mode enable"]
        #[inline(always)]
        pub const fn bkpsramamen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Backup RAM Autonomous mode enable"]
        #[inline(always)]
        pub fn set_bkpsramamen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "SRAM4 Autonomous mode enable"]
        #[inline(always)]
        pub const fn sram4amen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM4 Autonomous mode enable"]
        #[inline(always)]
        pub fn set_sram4amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for D3amr {
        #[inline(always)]
        fn default() -> D3amr {
            D3amr(0)
        }
    }
    impl core::fmt::Debug for D3amr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("D3amr")
                .field("bdmaamen", &self.bdmaamen())
                .field("lpuart1amen", &self.lpuart1amen())
                .field("spi6amen", &self.spi6amen())
                .field("i2c4amen", &self.i2c4amen())
                .field("lptim2amen", &self.lptim2amen())
                .field("lptim3amen", &self.lptim3amen())
                .field("lptim4amen", &self.lptim4amen())
                .field("lptim5amen", &self.lptim5amen())
                .field("dac2amen", &self.dac2amen())
                .field("comp12amen", &self.comp12amen())
                .field("vrefamen", &self.vrefamen())
                .field("rtcamen", &self.rtcamen())
                .field("crcamen", &self.crcamen())
                .field("sai4amen", &self.sai4amen())
                .field("adc3amen", &self.adc3amen())
                .field("dtsamen", &self.dtsamen())
                .field("bkpsramamen", &self.bkpsramamen())
                .field("sram4amen", &self.sram4amen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for D3amr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct D3amr {
                bdmaamen: bool,
                lpuart1amen: bool,
                spi6amen: bool,
                i2c4amen: bool,
                lptim2amen: bool,
                lptim3amen: bool,
                lptim4amen: bool,
                lptim5amen: bool,
                dac2amen: bool,
                comp12amen: bool,
                vrefamen: bool,
                rtcamen: bool,
                crcamen: bool,
                sai4amen: bool,
                adc3amen: bool,
                dtsamen: bool,
                bkpsramamen: bool,
                sram4amen: bool,
            }
            let proxy = D3amr {
                bdmaamen: self.bdmaamen(),
                lpuart1amen: self.lpuart1amen(),
                spi6amen: self.spi6amen(),
                i2c4amen: self.i2c4amen(),
                lptim2amen: self.lptim2amen(),
                lptim3amen: self.lptim3amen(),
                lptim4amen: self.lptim4amen(),
                lptim5amen: self.lptim5amen(),
                dac2amen: self.dac2amen(),
                comp12amen: self.comp12amen(),
                vrefamen: self.vrefamen(),
                rtcamen: self.rtcamen(),
                crcamen: self.crcamen(),
                sai4amen: self.sai4amen(),
                adc3amen: self.adc3amen(),
                dtsamen: self.dtsamen(),
                bkpsramamen: self.bkpsramamen(),
                sram4amen: self.sram4amen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC Domain 3 Kernel Clock Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct D3ccipr(pub u32);
    impl D3ccipr {
        #[doc = "LPUART1 kernel clock source selection"]
        #[inline(always)]
        pub const fn lpuart1sel(&self) -> super::vals::Lpuartsel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Lpuartsel::from_bits(val as u8)
        }
        #[doc = "LPUART1 kernel clock source selection"]
        #[inline(always)]
        pub fn set_lpuart1sel(&mut self, val: super::vals::Lpuartsel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "I2C4 kernel clock source selection"]
        #[inline(always)]
        pub const fn i2c4sel(&self) -> super::vals::I2c4sel {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::I2c4sel::from_bits(val as u8)
        }
        #[doc = "I2C4 kernel clock source selection"]
        #[inline(always)]
        pub fn set_i2c4sel(&mut self, val: super::vals::I2c4sel) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "LPTIM2 kernel clock source selection"]
        #[inline(always)]
        pub const fn lptim2sel(&self) -> super::vals::Lptim2sel {
            let val = (self.0 >> 10usize) & 0x07;
            super::vals::Lptim2sel::from_bits(val as u8)
        }
        #[doc = "LPTIM2 kernel clock source selection"]
        #[inline(always)]
        pub fn set_lptim2sel(&mut self, val: super::vals::Lptim2sel) {
            self.0 = (self.0 & !(0x07 << 10usize)) | (((val.to_bits() as u32) & 0x07) << 10usize);
        }
        #[doc = "LPTIM3,4,5 kernel clock source selection"]
        #[inline(always)]
        pub const fn lptim345sel(&self) -> super::vals::Lptim2sel {
            let val = (self.0 >> 13usize) & 0x07;
            super::vals::Lptim2sel::from_bits(val as u8)
        }
        #[doc = "LPTIM3,4,5 kernel clock source selection"]
        #[inline(always)]
        pub fn set_lptim345sel(&mut self, val: super::vals::Lptim2sel) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val.to_bits() as u32) & 0x07) << 13usize);
        }
        #[doc = "SAR ADC kernel clock source selection"]
        #[inline(always)]
        pub const fn adcsel(&self) -> super::vals::Adcsel {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Adcsel::from_bits(val as u8)
        }
        #[doc = "SAR ADC kernel clock source selection"]
        #[inline(always)]
        pub fn set_adcsel(&mut self, val: super::vals::Adcsel) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Sub-Block A of SAI4 kernel clock source selection"]
        #[inline(always)]
        pub const fn sai4asel(&self) -> super::vals::Saiasel {
            let val = (self.0 >> 21usize) & 0x07;
            super::vals::Saiasel::from_bits(val as u8)
        }
        #[doc = "Sub-Block A of SAI4 kernel clock source selection"]
        #[inline(always)]
        pub fn set_sai4asel(&mut self, val: super::vals::Saiasel) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
        }
        #[doc = "Sub-Block B of SAI4 kernel clock source selection"]
        #[inline(always)]
        pub const fn sai4bsel(&self) -> super::vals::Saiasel {
            let val = (self.0 >> 24usize) & 0x07;
            super::vals::Saiasel::from_bits(val as u8)
        }
        #[doc = "Sub-Block B of SAI4 kernel clock source selection"]
        #[inline(always)]
        pub fn set_sai4bsel(&mut self, val: super::vals::Saiasel) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
        }
        #[doc = "DFSDM2 kernel clock source selection"]
        #[inline(always)]
        pub const fn dfsdm2sel(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "DFSDM2 kernel clock source selection"]
        #[inline(always)]
        pub fn set_dfsdm2sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "SPI6 kernel clock source selection"]
        #[inline(always)]
        pub const fn spi6sel(&self) -> super::vals::Spi6sel {
            let val = (self.0 >> 28usize) & 0x07;
            super::vals::Spi6sel::from_bits(val as u8)
        }
        #[doc = "SPI6 kernel clock source selection"]
        #[inline(always)]
        pub fn set_spi6sel(&mut self, val: super::vals::Spi6sel) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
        }
    }
    impl Default for D3ccipr {
        #[inline(always)]
        fn default() -> D3ccipr {
            D3ccipr(0)
        }
    }
    impl core::fmt::Debug for D3ccipr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("D3ccipr")
                .field("lpuart1sel", &self.lpuart1sel())
                .field("i2c4sel", &self.i2c4sel())
                .field("lptim2sel", &self.lptim2sel())
                .field("lptim345sel", &self.lptim345sel())
                .field("adcsel", &self.adcsel())
                .field("sai4asel", &self.sai4asel())
                .field("sai4bsel", &self.sai4bsel())
                .field("dfsdm2sel", &self.dfsdm2sel())
                .field("spi6sel", &self.spi6sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for D3ccipr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct D3ccipr {
                lpuart1sel: super::vals::Lpuartsel,
                i2c4sel: super::vals::I2c4sel,
                lptim2sel: super::vals::Lptim2sel,
                lptim345sel: super::vals::Lptim2sel,
                adcsel: super::vals::Adcsel,
                sai4asel: super::vals::Saiasel,
                sai4bsel: super::vals::Saiasel,
                dfsdm2sel: bool,
                spi6sel: super::vals::Spi6sel,
            }
            let proxy = D3ccipr {
                lpuart1sel: self.lpuart1sel(),
                i2c4sel: self.i2c4sel(),
                lptim2sel: self.lptim2sel(),
                lptim345sel: self.lptim345sel(),
                adcsel: self.adcsel(),
                sai4asel: self.sai4asel(),
                sai4bsel: self.sai4bsel(),
                dfsdm2sel: self.dfsdm2sel(),
                spi6sel: self.spi6sel(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC Domain 3 Clock Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct D3cfgr(pub u32);
    impl D3cfgr {
        #[doc = "D3 domain APB4 prescaler"]
        #[inline(always)]
        pub const fn d3ppre(&self) -> super::vals::Ppre {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "D3 domain APB4 prescaler"]
        #[inline(always)]
        pub fn set_d3ppre(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
    }
    impl Default for D3cfgr {
        #[inline(always)]
        fn default() -> D3cfgr {
            D3cfgr(0)
        }
    }
    impl core::fmt::Debug for D3cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("D3cfgr").field("d3ppre", &self.d3ppre()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for D3cfgr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct D3cfgr {
                d3ppre: super::vals::Ppre,
            }
            let proxy = D3cfgr { d3ppre: self.d3ppre() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Global Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gcr(pub u32);
    impl Gcr {
        #[doc = "WWDG1 reset scope control"]
        #[inline(always)]
        pub const fn ww1rsc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG1 reset scope control"]
        #[inline(always)]
        pub fn set_ww1rsc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "WWDG2 reset scope control"]
        #[inline(always)]
        pub const fn ww2rsc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG2 reset scope control"]
        #[inline(always)]
        pub fn set_ww2rsc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Force allow CPU1 to boot"]
        #[inline(always)]
        pub const fn boot_c1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Force allow CPU1 to boot"]
        #[inline(always)]
        pub fn set_boot_c1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Force allow CPU2 to boot"]
        #[inline(always)]
        pub const fn boot_c2(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Force allow CPU2 to boot"]
        #[inline(always)]
        pub fn set_boot_c2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Gcr {
        #[inline(always)]
        fn default() -> Gcr {
            Gcr(0)
        }
    }
    impl core::fmt::Debug for Gcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gcr")
                .field("ww1rsc", &self.ww1rsc())
                .field("ww2rsc", &self.ww2rsc())
                .field("boot_c1", &self.boot_c1())
                .field("boot_c2", &self.boot_c2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gcr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Gcr {
                ww1rsc: bool,
                ww2rsc: bool,
                boot_c1: bool,
                boot_c2: bool,
            }
            let proxy = Gcr {
                ww1rsc: self.ww1rsc(),
                ww2rsc: self.ww2rsc(),
                boot_c1: self.boot_c1(),
                boot_c2: self.boot_c2(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC HSI configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hsicfgr(pub u32);
    impl Hsicfgr {
        #[doc = "HSI clock calibration"]
        #[inline(always)]
        pub const fn hsical(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "HSI clock calibration"]
        #[inline(always)]
        pub fn set_hsical(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
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
    impl Default for Hsicfgr {
        #[inline(always)]
        fn default() -> Hsicfgr {
            Hsicfgr(0)
        }
    }
    impl core::fmt::Debug for Hsicfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hsicfgr")
                .field("hsical", &self.hsical())
                .field("hsitrim", &self.hsitrim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hsicfgr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Hsicfgr {
                hsical: u16,
                hsitrim: u8,
            }
            let proxy = Hsicfgr {
                hsical: self.hsical(),
                hsitrim: self.hsitrim(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC Internal Clock Source Calibration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icscr(pub u32);
    impl Icscr {
        #[doc = "HSI clock calibration"]
        #[inline(always)]
        pub const fn hsical(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "HSI clock calibration"]
        #[inline(always)]
        pub fn set_hsical(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "HSI clock trimming"]
        #[inline(always)]
        pub const fn hsitrim(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x3f;
            val as u8
        }
        #[doc = "HSI clock trimming"]
        #[inline(always)]
        pub fn set_hsitrim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 12usize)) | (((val as u32) & 0x3f) << 12usize);
        }
        #[doc = "CSI clock calibration"]
        #[inline(always)]
        pub const fn csical(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0xff;
            val as u8
        }
        #[doc = "CSI clock calibration"]
        #[inline(always)]
        pub fn set_csical(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 18usize)) | (((val as u32) & 0xff) << 18usize);
        }
        #[doc = "CSI clock trimming"]
        #[inline(always)]
        pub const fn csitrim(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x1f;
            val as u8
        }
        #[doc = "CSI clock trimming"]
        #[inline(always)]
        pub fn set_csitrim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 26usize)) | (((val as u32) & 0x1f) << 26usize);
        }
    }
    impl Default for Icscr {
        #[inline(always)]
        fn default() -> Icscr {
            Icscr(0)
        }
    }
    impl core::fmt::Debug for Icscr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icscr")
                .field("hsical", &self.hsical())
                .field("hsitrim", &self.hsitrim())
                .field("csical", &self.csical())
                .field("csitrim", &self.csitrim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icscr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Icscr {
                hsical: u16,
                hsitrim: u8,
                csical: u8,
                csitrim: u8,
            }
            let proxy = Icscr {
                hsical: self.hsical(),
                hsitrim: self.hsitrim(),
                csical: self.csical(),
                csitrim: self.csitrim(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC PLLs Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pllcfgr(pub u32);
    impl Pllcfgr {
        #[doc = "PLL1 fractional latch enable"]
        #[inline(always)]
        pub const fn pllfracen(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 fractional latch enable"]
        #[inline(always)]
        pub fn set_pllfracen(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "PLL1 VCO selection"]
        #[inline(always)]
        pub const fn pllvcosel(&self, n: usize) -> super::vals::Pllvcosel {
            assert!(n < 3usize);
            let offs = 1usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Pllvcosel::from_bits(val as u8)
        }
        #[doc = "PLL1 VCO selection"]
        #[inline(always)]
        pub fn set_pllvcosel(&mut self, n: usize, val: super::vals::Pllvcosel) {
            assert!(n < 3usize);
            let offs = 1usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "PLL1 input frequency range"]
        #[inline(always)]
        pub const fn pllrge(&self, n: usize) -> super::vals::Pllrge {
            assert!(n < 3usize);
            let offs = 2usize + n * 4usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::Pllrge::from_bits(val as u8)
        }
        #[doc = "PLL1 input frequency range"]
        #[inline(always)]
        pub fn set_pllrge(&mut self, n: usize, val: super::vals::Pllrge) {
            assert!(n < 3usize);
            let offs = 2usize + n * 4usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "PLL1 DIVP divider output enable"]
        #[inline(always)]
        pub const fn divpen(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 16usize + n * 3usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 DIVP divider output enable"]
        #[inline(always)]
        pub fn set_divpen(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 16usize + n * 3usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "PLL1 DIVQ divider output enable"]
        #[inline(always)]
        pub const fn divqen(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 17usize + n * 3usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 DIVQ divider output enable"]
        #[inline(always)]
        pub fn set_divqen(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 17usize + n * 3usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "PLL1 DIVR divider output enable"]
        #[inline(always)]
        pub const fn divren(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 18usize + n * 3usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 DIVR divider output enable"]
        #[inline(always)]
        pub fn set_divren(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 18usize + n * 3usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Pllcfgr {
        #[inline(always)]
        fn default() -> Pllcfgr {
            Pllcfgr(0)
        }
    }
    impl core::fmt::Debug for Pllcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pllcfgr")
                .field(
                    "pllfracen",
                    &[self.pllfracen(0usize), self.pllfracen(1usize), self.pllfracen(2usize)],
                )
                .field(
                    "pllvcosel",
                    &[self.pllvcosel(0usize), self.pllvcosel(1usize), self.pllvcosel(2usize)],
                )
                .field(
                    "pllrge",
                    &[self.pllrge(0usize), self.pllrge(1usize), self.pllrge(2usize)],
                )
                .field(
                    "divpen",
                    &[self.divpen(0usize), self.divpen(1usize), self.divpen(2usize)],
                )
                .field(
                    "divqen",
                    &[self.divqen(0usize), self.divqen(1usize), self.divqen(2usize)],
                )
                .field(
                    "divren",
                    &[self.divren(0usize), self.divren(1usize), self.divren(2usize)],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pllcfgr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Pllcfgr {
                pllfracen: [bool; 3usize],
                pllvcosel: [super::vals::Pllvcosel; 3usize],
                pllrge: [super::vals::Pllrge; 3usize],
                divpen: [bool; 3usize],
                divqen: [bool; 3usize],
                divren: [bool; 3usize],
            }
            let proxy = Pllcfgr {
                pllfracen: [self.pllfracen(0usize), self.pllfracen(1usize), self.pllfracen(2usize)],
                pllvcosel: [self.pllvcosel(0usize), self.pllvcosel(1usize), self.pllvcosel(2usize)],
                pllrge: [self.pllrge(0usize), self.pllrge(1usize), self.pllrge(2usize)],
                divpen: [self.divpen(0usize), self.divpen(1usize), self.divpen(2usize)],
                divqen: [self.divqen(0usize), self.divqen(1usize), self.divqen(2usize)],
                divren: [self.divren(0usize), self.divren(1usize), self.divren(2usize)],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC PLLs Clock Source Selection Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pllckselr(pub u32);
    impl Pllckselr {
        #[doc = "DIVMx and PLLs clock source selection"]
        #[inline(always)]
        pub const fn pllsrc(&self) -> super::vals::Pllsrc {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Pllsrc::from_bits(val as u8)
        }
        #[doc = "DIVMx and PLLs clock source selection"]
        #[inline(always)]
        pub fn set_pllsrc(&mut self, val: super::vals::Pllsrc) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Prescaler for PLLx"]
        #[inline(always)]
        pub const fn divm(&self, n: usize) -> super::vals::Pllm {
            assert!(n < 3usize);
            let offs = 4usize + n * 8usize;
            let val = (self.0 >> offs) & 0x3f;
            super::vals::Pllm::from_bits(val as u8)
        }
        #[doc = "Prescaler for PLLx"]
        #[inline(always)]
        pub fn set_divm(&mut self, n: usize, val: super::vals::Pllm) {
            assert!(n < 3usize);
            let offs = 4usize + n * 8usize;
            self.0 = (self.0 & !(0x3f << offs)) | (((val.to_bits() as u32) & 0x3f) << offs);
        }
    }
    impl Default for Pllckselr {
        #[inline(always)]
        fn default() -> Pllckselr {
            Pllckselr(0)
        }
    }
    impl core::fmt::Debug for Pllckselr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pllckselr")
                .field("pllsrc", &self.pllsrc())
                .field("divm", &[self.divm(0usize), self.divm(1usize), self.divm(2usize)])
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pllckselr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Pllckselr {
                pllsrc: super::vals::Pllsrc,
                divm: [super::vals::Pllm; 3usize],
            }
            let proxy = Pllckselr {
                pllsrc: self.pllsrc(),
                divm: [self.divm(0usize), self.divm(1usize), self.divm(2usize)],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC PLL1 Dividers Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Plldivr(pub u32);
    impl Plldivr {
        #[doc = "Multiplication factor for PLL1 VCO"]
        #[inline(always)]
        pub const fn plln(&self) -> super::vals::Plln {
            let val = (self.0 >> 0usize) & 0x01ff;
            super::vals::Plln::from_bits(val as u16)
        }
        #[doc = "Multiplication factor for PLL1 VCO"]
        #[inline(always)]
        pub fn set_plln(&mut self, val: super::vals::Plln) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val.to_bits() as u32) & 0x01ff) << 0usize);
        }
        #[doc = "PLL DIVP division factor"]
        #[inline(always)]
        pub const fn pllp(&self) -> super::vals::Plldiv {
            let val = (self.0 >> 9usize) & 0x7f;
            super::vals::Plldiv::from_bits(val as u8)
        }
        #[doc = "PLL DIVP division factor"]
        #[inline(always)]
        pub fn set_pllp(&mut self, val: super::vals::Plldiv) {
            self.0 = (self.0 & !(0x7f << 9usize)) | (((val.to_bits() as u32) & 0x7f) << 9usize);
        }
        #[doc = "PLL DIVQ division factor"]
        #[inline(always)]
        pub const fn pllq(&self) -> super::vals::Plldiv {
            let val = (self.0 >> 16usize) & 0x7f;
            super::vals::Plldiv::from_bits(val as u8)
        }
        #[doc = "PLL DIVQ division factor"]
        #[inline(always)]
        pub fn set_pllq(&mut self, val: super::vals::Plldiv) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val.to_bits() as u32) & 0x7f) << 16usize);
        }
        #[doc = "PLL DIVR division factor"]
        #[inline(always)]
        pub const fn pllr(&self) -> super::vals::Plldiv {
            let val = (self.0 >> 24usize) & 0x7f;
            super::vals::Plldiv::from_bits(val as u8)
        }
        #[doc = "PLL DIVR division factor"]
        #[inline(always)]
        pub fn set_pllr(&mut self, val: super::vals::Plldiv) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val.to_bits() as u32) & 0x7f) << 24usize);
        }
    }
    impl Default for Plldivr {
        #[inline(always)]
        fn default() -> Plldivr {
            Plldivr(0)
        }
    }
    impl core::fmt::Debug for Plldivr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Plldivr")
                .field("plln", &self.plln())
                .field("pllp", &self.pllp())
                .field("pllq", &self.pllq())
                .field("pllr", &self.pllr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Plldivr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Plldivr {
                plln: super::vals::Plln,
                pllp: super::vals::Plldiv,
                pllq: super::vals::Plldiv,
                pllr: super::vals::Plldiv,
            }
            let proxy = Plldivr {
                plln: self.plln(),
                pllp: self.pllp(),
                pllq: self.pllq(),
                pllr: self.pllr(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC PLL Fractional Divider Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pllfracr(pub u32);
    impl Pllfracr {
        #[doc = "Fractional part of the multiplication factor for PLL VCO"]
        #[inline(always)]
        pub const fn fracn(&self) -> u16 {
            let val = (self.0 >> 3usize) & 0x1fff;
            val as u16
        }
        #[doc = "Fractional part of the multiplication factor for PLL VCO"]
        #[inline(always)]
        pub fn set_fracn(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 3usize)) | (((val as u32) & 0x1fff) << 3usize);
        }
    }
    impl Default for Pllfracr {
        #[inline(always)]
        fn default() -> Pllfracr {
            Pllfracr(0)
        }
    }
    impl core::fmt::Debug for Pllfracr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pllfracr").field("fracn", &self.fracn()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pllfracr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Pllfracr {
                fracn: u16,
            }
            let proxy = Pllfracr { fracn: self.fracn() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC Reset Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rsr(pub u32);
    impl Rsr {
        #[doc = "Remove reset flag"]
        #[inline(always)]
        pub const fn rmvf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Remove reset flag"]
        #[inline(always)]
        pub fn set_rmvf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "CPU reset flag"]
        #[inline(always)]
        pub const fn cpurstf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "CPU reset flag"]
        #[inline(always)]
        pub fn set_cpurstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "D1 domain power switch reset flag"]
        #[inline(always)]
        pub const fn d1rstf(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "D1 domain power switch reset flag"]
        #[inline(always)]
        pub fn set_d1rstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "D2 domain power switch reset flag"]
        #[inline(always)]
        pub const fn d2rstf(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "D2 domain power switch reset flag"]
        #[inline(always)]
        pub fn set_d2rstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "BOR reset flag"]
        #[inline(always)]
        pub const fn borrstf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "BOR reset flag"]
        #[inline(always)]
        pub fn set_borrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Pin reset flag (NRST)"]
        #[inline(always)]
        pub const fn pinrstf(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Pin reset flag (NRST)"]
        #[inline(always)]
        pub fn set_pinrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "POR/PDR reset flag"]
        #[inline(always)]
        pub const fn porrstf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "POR/PDR reset flag"]
        #[inline(always)]
        pub fn set_porrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "System reset from CPU reset flag"]
        #[inline(always)]
        pub const fn sftrstf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "System reset from CPU reset flag"]
        #[inline(always)]
        pub fn set_sftrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Independent Watchdog reset flag"]
        #[inline(always)]
        pub const fn iwdg1rstf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Independent Watchdog reset flag"]
        #[inline(always)]
        pub fn set_iwdg1rstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Window Watchdog reset flag"]
        #[inline(always)]
        pub const fn wwdg1rstf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Window Watchdog reset flag"]
        #[inline(always)]
        pub fn set_wwdg1rstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Reset due to illegal D1 DStandby or CPU CStop flag"]
        #[inline(always)]
        pub const fn lpwrrstf(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Reset due to illegal D1 DStandby or CPU CStop flag"]
        #[inline(always)]
        pub fn set_lpwrrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Rsr {
        #[inline(always)]
        fn default() -> Rsr {
            Rsr(0)
        }
    }
    impl core::fmt::Debug for Rsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rsr")
                .field("rmvf", &self.rmvf())
                .field("cpurstf", &self.cpurstf())
                .field("d1rstf", &self.d1rstf())
                .field("d2rstf", &self.d2rstf())
                .field("borrstf", &self.borrstf())
                .field("pinrstf", &self.pinrstf())
                .field("porrstf", &self.porrstf())
                .field("sftrstf", &self.sftrstf())
                .field("iwdg1rstf", &self.iwdg1rstf())
                .field("wwdg1rstf", &self.wwdg1rstf())
                .field("lpwrrstf", &self.lpwrrstf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rsr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Rsr {
                rmvf: bool,
                cpurstf: bool,
                d1rstf: bool,
                d2rstf: bool,
                borrstf: bool,
                pinrstf: bool,
                porrstf: bool,
                sftrstf: bool,
                iwdg1rstf: bool,
                wwdg1rstf: bool,
                lpwrrstf: bool,
            }
            let proxy = Rsr {
                rmvf: self.rmvf(),
                cpurstf: self.cpurstf(),
                d1rstf: self.d1rstf(),
                d2rstf: self.d2rstf(),
                borrstf: self.borrstf(),
                pinrstf: self.pinrstf(),
                porrstf: self.porrstf(),
                sftrstf: self.sftrstf(),
                iwdg1rstf: self.iwdg1rstf(),
                wwdg1rstf: self.wwdg1rstf(),
                lpwrrstf: self.lpwrrstf(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adcsel {
        #[doc = "pll2_p selected as peripheral clock"]
        PLL2_P = 0x0,
        #[doc = "pll3_r selected as peripheral clock"]
        PLL3_R = 0x01,
        #[doc = "PER selected as peripheral clock"]
        PER = 0x02,
        _RESERVED_3 = 0x03,
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cecsel {
        #[doc = "LSE selected as peripheral clock"]
        LSE = 0x0,
        #[doc = "LSI selected as peripheral clock"]
        LSI = 0x01,
        #[doc = "csi_ker selected as peripheral clock"]
        CSI = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Cecsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cecsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cecsel {
        #[inline(always)]
        fn from(val: u8) -> Cecsel {
            Cecsel::from_bits(val)
        }
    }
    impl From<Cecsel> for u8 {
        #[inline(always)]
        fn from(val: Cecsel) -> u8 {
            Cecsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dfsdmsel {
        #[doc = "rcc_pclk2 selected as peripheral clock"]
        PCLK2 = 0x0,
        #[doc = "System clock selected as peripheral clock"]
        SYS = 0x01,
    }
    impl Dfsdmsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dfsdmsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dfsdmsel {
        #[inline(always)]
        fn from(val: u8) -> Dfsdmsel {
            Dfsdmsel::from_bits(val)
        }
    }
    impl From<Dfsdmsel> for u8 {
        #[inline(always)]
        fn from(val: Dfsdmsel) -> u8 {
            Dfsdmsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fdcansel {
        #[doc = "HSE selected as peripheral clock"]
        HSE = 0x0,
        #[doc = "pll1_q selected as peripheral clock"]
        PLL1_Q = 0x01,
        #[doc = "pll2_q selected as peripheral clock"]
        PLL2_Q = 0x02,
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fmcsel {
        #[doc = "rcc_hclk3 selected as peripheral clock"]
        HCLK3 = 0x0,
        #[doc = "pll1_q selected as peripheral clock"]
        PLL1_Q = 0x01,
        #[doc = "pll2_r selected as peripheral clock"]
        PLL2_R = 0x02,
        #[doc = "PER selected as peripheral clock"]
        PER = 0x03,
    }
    impl Fmcsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fmcsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fmcsel {
        #[inline(always)]
        fn from(val: u8) -> Fmcsel {
            Fmcsel::from_bits(val)
        }
    }
    impl From<Fmcsel> for u8 {
        #[inline(always)]
        fn from(val: Fmcsel) -> u8 {
            Fmcsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hpre {
        #[doc = "sys_ck not divided"]
        DIV1 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "sys_ck divided by 2"]
        DIV2 = 0x08,
        #[doc = "sys_ck divided by 4"]
        DIV4 = 0x09,
        #[doc = "sys_ck divided by 8"]
        DIV8 = 0x0a,
        #[doc = "sys_ck divided by 16"]
        DIV16 = 0x0b,
        #[doc = "sys_ck divided by 64"]
        DIV64 = 0x0c,
        #[doc = "sys_ck divided by 128"]
        DIV128 = 0x0d,
        #[doc = "sys_ck divided by 256"]
        DIV256 = 0x0e,
        #[doc = "sys_ck divided by 512"]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hrtimsel {
        #[doc = "The HRTIM prescaler clock source is the same as other timers (rcc_timy_ker_ck)"]
        TIMY_KER = 0x0,
        #[doc = "The HRTIM prescaler clock source is the CPU clock (c_ck)"]
        C_CK = 0x01,
    }
    impl Hrtimsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hrtimsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hrtimsel {
        #[inline(always)]
        fn from(val: u8) -> Hrtimsel {
            Hrtimsel::from_bits(val)
        }
    }
    impl From<Hrtimsel> for u8 {
        #[inline(always)]
        fn from(val: Hrtimsel) -> u8 {
            Hrtimsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hsidiv {
        #[doc = "No division"]
        DIV1 = 0x0,
        #[doc = "Division by 2"]
        DIV2 = 0x01,
        #[doc = "Division by 4"]
        DIV4 = 0x02,
        #[doc = "Division by 8"]
        DIV8 = 0x03,
    }
    impl Hsidiv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hsidiv {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hsidiv {
        #[inline(always)]
        fn from(val: u8) -> Hsidiv {
            Hsidiv::from_bits(val)
        }
    }
    impl From<Hsidiv> for u8 {
        #[inline(always)]
        fn from(val: Hsidiv) -> u8 {
            Hsidiv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum I2c1235sel {
        #[doc = "rcc_pclk1 selected as peripheral clock"]
        PCLK1 = 0x0,
        #[doc = "pll3_r selected as peripheral clock"]
        PLL3_R = 0x01,
        #[doc = "hsi_ker selected as peripheral clock"]
        HSI = 0x02,
        #[doc = "csi_ker selected as peripheral clock"]
        CSI = 0x03,
    }
    impl I2c1235sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2c1235sel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I2c1235sel {
        #[inline(always)]
        fn from(val: u8) -> I2c1235sel {
            I2c1235sel::from_bits(val)
        }
    }
    impl From<I2c1235sel> for u8 {
        #[inline(always)]
        fn from(val: I2c1235sel) -> u8 {
            I2c1235sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum I2c4sel {
        #[doc = "rcc_pclk4 selected as peripheral clock"]
        PCLK4 = 0x0,
        #[doc = "pll3_r selected as peripheral clock"]
        PLL3_R = 0x01,
        #[doc = "hsi_ker selected as peripheral clock"]
        HSI = 0x02,
        #[doc = "csi_ker selected as peripheral clock"]
        CSI = 0x03,
    }
    impl I2c4sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2c4sel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I2c4sel {
        #[inline(always)]
        fn from(val: u8) -> I2c4sel {
            I2c4sel::from_bits(val)
        }
    }
    impl From<I2c4sel> for u8 {
        #[inline(always)]
        fn from(val: I2c4sel) -> u8 {
            I2c4sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lptim1sel {
        #[doc = "rcc_pclk1 selected as peripheral clock"]
        PCLK1 = 0x0,
        #[doc = "pll2_p selected as peripheral clock"]
        PLL2_P = 0x01,
        #[doc = "pll3_r selected as peripheral clock"]
        PLL3_R = 0x02,
        #[doc = "LSE selected as peripheral clock"]
        LSE = 0x03,
        #[doc = "LSI selected as peripheral clock"]
        LSI = 0x04,
        #[doc = "PER selected as peripheral clock"]
        PER = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Lptim1sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lptim1sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lptim1sel {
        #[inline(always)]
        fn from(val: u8) -> Lptim1sel {
            Lptim1sel::from_bits(val)
        }
    }
    impl From<Lptim1sel> for u8 {
        #[inline(always)]
        fn from(val: Lptim1sel) -> u8 {
            Lptim1sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lptim2sel {
        #[doc = "rcc_pclk4 selected as peripheral clock"]
        PCLK4 = 0x0,
        #[doc = "pll2_p selected as peripheral clock"]
        PLL2_P = 0x01,
        #[doc = "pll3_r selected as peripheral clock"]
        PLL3_R = 0x02,
        #[doc = "LSE selected as peripheral clock"]
        LSE = 0x03,
        #[doc = "LSI selected as peripheral clock"]
        LSI = 0x04,
        #[doc = "PER selected as peripheral clock"]
        PER = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Lptim2sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lptim2sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lptim2sel {
        #[inline(always)]
        fn from(val: u8) -> Lptim2sel {
            Lptim2sel::from_bits(val)
        }
    }
    impl From<Lptim2sel> for u8 {
        #[inline(always)]
        fn from(val: Lptim2sel) -> u8 {
            Lptim2sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lpuartsel {
        #[doc = "rcc_pclk_d4 selected as peripheral clock"]
        PCLK4 = 0x0,
        #[doc = "pll2_q selected as peripheral clock"]
        PLL2_Q = 0x01,
        #[doc = "pll3_q selected as peripheral clock"]
        PLL3_Q = 0x02,
        #[doc = "hsi_ker selected as peripheral clock"]
        HSI = 0x03,
        #[doc = "csi_ker selected as peripheral clock"]
        CSI = 0x04,
        #[doc = "LSE selected as peripheral clock"]
        LSE = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Lpuartsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lpuartsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lpuartsel {
        #[inline(always)]
        fn from(val: u8) -> Lpuartsel {
            Lpuartsel::from_bits(val)
        }
    }
    impl From<Lpuartsel> for u8 {
        #[inline(always)]
        fn from(val: Lpuartsel) -> u8 {
            Lpuartsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lsedrv {
        #[doc = "Low driving capability"]
        LOW = 0x0,
        #[doc = "Medium high driving capability"]
        MEDIUM_HIGH = 0x01,
        #[doc = "Medium low driving capability"]
        MEDIUM_LOW = 0x02,
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mco1sel {
        #[doc = "HSI selected for micro-controller clock output"]
        HSI = 0x0,
        #[doc = "LSE selected for micro-controller clock output"]
        LSE = 0x01,
        #[doc = "HSE selected for micro-controller clock output"]
        HSE = 0x02,
        #[doc = "pll1_q selected for micro-controller clock output"]
        PLL1_Q = 0x03,
        #[doc = "HSI48 selected for micro-controller clock output"]
        HSI48 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Mco1sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mco1sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mco1sel {
        #[inline(always)]
        fn from(val: u8) -> Mco1sel {
            Mco1sel::from_bits(val)
        }
    }
    impl From<Mco1sel> for u8 {
        #[inline(always)]
        fn from(val: Mco1sel) -> u8 {
            Mco1sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mco2sel {
        #[doc = "System clock selected for micro-controller clock output"]
        SYS = 0x0,
        #[doc = "pll2_p selected for micro-controller clock output"]
        PLL2_P = 0x01,
        #[doc = "HSE selected for micro-controller clock output"]
        HSE = 0x02,
        #[doc = "pll1_p selected for micro-controller clock output"]
        PLL1_P = 0x03,
        #[doc = "CSI selected for micro-controller clock output"]
        CSI = 0x04,
        #[doc = "LSI selected for micro-controller clock output"]
        LSI = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Mco2sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mco2sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mco2sel {
        #[inline(always)]
        fn from(val: u8) -> Mco2sel {
            Mco2sel::from_bits(val)
        }
    }
    impl From<Mco2sel> for u8 {
        #[inline(always)]
        fn from(val: Mco2sel) -> u8 {
            Mco2sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mcopre {
        _RESERVED_0 = 0x0,
        #[doc = "Divide by 1"]
        DIV1 = 0x01,
        #[doc = "Divide by 2"]
        DIV2 = 0x02,
        #[doc = "Divide by 3"]
        DIV3 = 0x03,
        #[doc = "Divide by 4"]
        DIV4 = 0x04,
        #[doc = "Divide by 5"]
        DIV5 = 0x05,
        #[doc = "Divide by 6"]
        DIV6 = 0x06,
        #[doc = "Divide by 7"]
        DIV7 = 0x07,
        #[doc = "Divide by 8"]
        DIV8 = 0x08,
        #[doc = "Divide by 9"]
        DIV9 = 0x09,
        #[doc = "Divide by 10"]
        DIV10 = 0x0a,
        #[doc = "Divide by 11"]
        DIV11 = 0x0b,
        #[doc = "Divide by 12"]
        DIV12 = 0x0c,
        #[doc = "Divide by 13"]
        DIV13 = 0x0d,
        #[doc = "Divide by 14"]
        DIV14 = 0x0e,
        #[doc = "Divide by 15"]
        DIV15 = 0x0f,
    }
    impl Mcopre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mcopre {
            unsafe { core::mem::transmute(val & 0x0f) }
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Persel {
        #[doc = "HSI selected as peripheral clock"]
        HSI = 0x0,
        #[doc = "CSI selected as peripheral clock"]
        CSI = 0x01,
        #[doc = "HSE selected as peripheral clock"]
        HSE = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Persel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Persel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Persel {
        #[inline(always)]
        fn from(val: u8) -> Persel {
            Persel::from_bits(val)
        }
    }
    impl From<Persel> for u8 {
        #[inline(always)]
        fn from(val: Persel) -> u8 {
            Persel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Plldiv {
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
        DIV17 = 0x10,
        DIV18 = 0x11,
        DIV19 = 0x12,
        DIV20 = 0x13,
        DIV21 = 0x14,
        DIV22 = 0x15,
        DIV23 = 0x16,
        DIV24 = 0x17,
        DIV25 = 0x18,
        DIV26 = 0x19,
        DIV27 = 0x1a,
        DIV28 = 0x1b,
        DIV29 = 0x1c,
        DIV30 = 0x1d,
        DIV31 = 0x1e,
        DIV32 = 0x1f,
        DIV33 = 0x20,
        DIV34 = 0x21,
        DIV35 = 0x22,
        DIV36 = 0x23,
        DIV37 = 0x24,
        DIV38 = 0x25,
        DIV39 = 0x26,
        DIV40 = 0x27,
        DIV41 = 0x28,
        DIV42 = 0x29,
        DIV43 = 0x2a,
        DIV44 = 0x2b,
        DIV45 = 0x2c,
        DIV46 = 0x2d,
        DIV47 = 0x2e,
        DIV48 = 0x2f,
        DIV49 = 0x30,
        DIV50 = 0x31,
        DIV51 = 0x32,
        DIV52 = 0x33,
        DIV53 = 0x34,
        DIV54 = 0x35,
        DIV55 = 0x36,
        DIV56 = 0x37,
        DIV57 = 0x38,
        DIV58 = 0x39,
        DIV59 = 0x3a,
        DIV60 = 0x3b,
        DIV61 = 0x3c,
        DIV62 = 0x3d,
        DIV63 = 0x3e,
        DIV64 = 0x3f,
        DIV65 = 0x40,
        DIV66 = 0x41,
        DIV67 = 0x42,
        DIV68 = 0x43,
        DIV69 = 0x44,
        DIV70 = 0x45,
        DIV71 = 0x46,
        DIV72 = 0x47,
        DIV73 = 0x48,
        DIV74 = 0x49,
        DIV75 = 0x4a,
        DIV76 = 0x4b,
        DIV77 = 0x4c,
        DIV78 = 0x4d,
        DIV79 = 0x4e,
        DIV80 = 0x4f,
        DIV81 = 0x50,
        DIV82 = 0x51,
        DIV83 = 0x52,
        DIV84 = 0x53,
        DIV85 = 0x54,
        DIV86 = 0x55,
        DIV87 = 0x56,
        DIV88 = 0x57,
        DIV89 = 0x58,
        DIV90 = 0x59,
        DIV91 = 0x5a,
        DIV92 = 0x5b,
        DIV93 = 0x5c,
        DIV94 = 0x5d,
        DIV95 = 0x5e,
        DIV96 = 0x5f,
        DIV97 = 0x60,
        DIV98 = 0x61,
        DIV99 = 0x62,
        DIV100 = 0x63,
        DIV101 = 0x64,
        DIV102 = 0x65,
        DIV103 = 0x66,
        DIV104 = 0x67,
        DIV105 = 0x68,
        DIV106 = 0x69,
        DIV107 = 0x6a,
        DIV108 = 0x6b,
        DIV109 = 0x6c,
        DIV110 = 0x6d,
        DIV111 = 0x6e,
        DIV112 = 0x6f,
        DIV113 = 0x70,
        DIV114 = 0x71,
        DIV115 = 0x72,
        DIV116 = 0x73,
        DIV117 = 0x74,
        DIV118 = 0x75,
        DIV119 = 0x76,
        DIV120 = 0x77,
        DIV121 = 0x78,
        DIV122 = 0x79,
        DIV123 = 0x7a,
        DIV124 = 0x7b,
        DIV125 = 0x7c,
        DIV126 = 0x7d,
        DIV127 = 0x7e,
        DIV128 = 0x7f,
    }
    impl Plldiv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Plldiv {
            unsafe { core::mem::transmute(val & 0x7f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Plldiv {
        #[inline(always)]
        fn from(val: u8) -> Plldiv {
            Plldiv::from_bits(val)
        }
    }
    impl From<Plldiv> for u8 {
        #[inline(always)]
        fn from(val: Plldiv) -> u8 {
            Plldiv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllm {
        _RESERVED_0 = 0x0,
        DIV1 = 0x01,
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
        DIV32 = 0x20,
        DIV33 = 0x21,
        DIV34 = 0x22,
        DIV35 = 0x23,
        DIV36 = 0x24,
        DIV37 = 0x25,
        DIV38 = 0x26,
        DIV39 = 0x27,
        DIV40 = 0x28,
        DIV41 = 0x29,
        DIV42 = 0x2a,
        DIV43 = 0x2b,
        DIV44 = 0x2c,
        DIV45 = 0x2d,
        DIV46 = 0x2e,
        DIV47 = 0x2f,
        DIV48 = 0x30,
        DIV49 = 0x31,
        DIV50 = 0x32,
        DIV51 = 0x33,
        DIV52 = 0x34,
        DIV53 = 0x35,
        DIV54 = 0x36,
        DIV55 = 0x37,
        DIV56 = 0x38,
        DIV57 = 0x39,
        DIV58 = 0x3a,
        DIV59 = 0x3b,
        DIV60 = 0x3c,
        DIV61 = 0x3d,
        DIV62 = 0x3e,
        _RESERVED_3f = 0x3f,
    }
    impl Pllm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllm {
            unsafe { core::mem::transmute(val & 0x3f) }
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
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Plln(pub u16);
    impl Plln {
        pub const MUL4: Self = Self(0x03);
        pub const MUL5: Self = Self(0x04);
        pub const MUL6: Self = Self(0x05);
        pub const MUL7: Self = Self(0x06);
        pub const MUL8: Self = Self(0x07);
        pub const MUL9: Self = Self(0x08);
        pub const MUL10: Self = Self(0x09);
        pub const MUL11: Self = Self(0x0a);
        pub const MUL12: Self = Self(0x0b);
        pub const MUL13: Self = Self(0x0c);
        pub const MUL14: Self = Self(0x0d);
        pub const MUL15: Self = Self(0x0e);
        pub const MUL16: Self = Self(0x0f);
        pub const MUL17: Self = Self(0x10);
        pub const MUL18: Self = Self(0x11);
        pub const MUL19: Self = Self(0x12);
        pub const MUL20: Self = Self(0x13);
        pub const MUL21: Self = Self(0x14);
        pub const MUL22: Self = Self(0x15);
        pub const MUL23: Self = Self(0x16);
        pub const MUL24: Self = Self(0x17);
        pub const MUL25: Self = Self(0x18);
        pub const MUL26: Self = Self(0x19);
        pub const MUL27: Self = Self(0x1a);
        pub const MUL28: Self = Self(0x1b);
        pub const MUL29: Self = Self(0x1c);
        pub const MUL30: Self = Self(0x1d);
        pub const MUL31: Self = Self(0x1e);
        pub const MUL32: Self = Self(0x1f);
        pub const MUL33: Self = Self(0x20);
        pub const MUL34: Self = Self(0x21);
        pub const MUL35: Self = Self(0x22);
        pub const MUL36: Self = Self(0x23);
        pub const MUL37: Self = Self(0x24);
        pub const MUL38: Self = Self(0x25);
        pub const MUL39: Self = Self(0x26);
        pub const MUL40: Self = Self(0x27);
        pub const MUL41: Self = Self(0x28);
        pub const MUL42: Self = Self(0x29);
        pub const MUL43: Self = Self(0x2a);
        pub const MUL44: Self = Self(0x2b);
        pub const MUL45: Self = Self(0x2c);
        pub const MUL46: Self = Self(0x2d);
        pub const MUL47: Self = Self(0x2e);
        pub const MUL48: Self = Self(0x2f);
        pub const MUL49: Self = Self(0x30);
        pub const MUL50: Self = Self(0x31);
        pub const MUL51: Self = Self(0x32);
        pub const MUL52: Self = Self(0x33);
        pub const MUL53: Self = Self(0x34);
        pub const MUL54: Self = Self(0x35);
        pub const MUL55: Self = Self(0x36);
        pub const MUL56: Self = Self(0x37);
        pub const MUL57: Self = Self(0x38);
        pub const MUL58: Self = Self(0x39);
        pub const MUL59: Self = Self(0x3a);
        pub const MUL60: Self = Self(0x3b);
        pub const MUL61: Self = Self(0x3c);
        pub const MUL62: Self = Self(0x3d);
        pub const MUL63: Self = Self(0x3e);
        pub const MUL64: Self = Self(0x3f);
        pub const MUL65: Self = Self(0x40);
        pub const MUL66: Self = Self(0x41);
        pub const MUL67: Self = Self(0x42);
        pub const MUL68: Self = Self(0x43);
        pub const MUL69: Self = Self(0x44);
        pub const MUL70: Self = Self(0x45);
        pub const MUL71: Self = Self(0x46);
        pub const MUL72: Self = Self(0x47);
        pub const MUL73: Self = Self(0x48);
        pub const MUL74: Self = Self(0x49);
        pub const MUL75: Self = Self(0x4a);
        pub const MUL76: Self = Self(0x4b);
        pub const MUL77: Self = Self(0x4c);
        pub const MUL78: Self = Self(0x4d);
        pub const MUL79: Self = Self(0x4e);
        pub const MUL80: Self = Self(0x4f);
        pub const MUL81: Self = Self(0x50);
        pub const MUL82: Self = Self(0x51);
        pub const MUL83: Self = Self(0x52);
        pub const MUL84: Self = Self(0x53);
        pub const MUL85: Self = Self(0x54);
        pub const MUL86: Self = Self(0x55);
        pub const MUL87: Self = Self(0x56);
        pub const MUL88: Self = Self(0x57);
        pub const MUL89: Self = Self(0x58);
        pub const MUL90: Self = Self(0x59);
        pub const MUL91: Self = Self(0x5a);
        pub const MUL92: Self = Self(0x5b);
        pub const MUL93: Self = Self(0x5c);
        pub const MUL94: Self = Self(0x5d);
        pub const MUL95: Self = Self(0x5e);
        pub const MUL96: Self = Self(0x5f);
        pub const MUL97: Self = Self(0x60);
        pub const MUL98: Self = Self(0x61);
        pub const MUL99: Self = Self(0x62);
        pub const MUL100: Self = Self(0x63);
        pub const MUL101: Self = Self(0x64);
        pub const MUL102: Self = Self(0x65);
        pub const MUL103: Self = Self(0x66);
        pub const MUL104: Self = Self(0x67);
        pub const MUL105: Self = Self(0x68);
        pub const MUL106: Self = Self(0x69);
        pub const MUL107: Self = Self(0x6a);
        pub const MUL108: Self = Self(0x6b);
        pub const MUL109: Self = Self(0x6c);
        pub const MUL110: Self = Self(0x6d);
        pub const MUL111: Self = Self(0x6e);
        pub const MUL112: Self = Self(0x6f);
        pub const MUL113: Self = Self(0x70);
        pub const MUL114: Self = Self(0x71);
        pub const MUL115: Self = Self(0x72);
        pub const MUL116: Self = Self(0x73);
        pub const MUL117: Self = Self(0x74);
        pub const MUL118: Self = Self(0x75);
        pub const MUL119: Self = Self(0x76);
        pub const MUL120: Self = Self(0x77);
        pub const MUL121: Self = Self(0x78);
        pub const MUL122: Self = Self(0x79);
        pub const MUL123: Self = Self(0x7a);
        pub const MUL124: Self = Self(0x7b);
        pub const MUL125: Self = Self(0x7c);
        pub const MUL126: Self = Self(0x7d);
        pub const MUL127: Self = Self(0x7e);
        pub const MUL128: Self = Self(0x7f);
        pub const MUL129: Self = Self(0x80);
        pub const MUL130: Self = Self(0x81);
        pub const MUL131: Self = Self(0x82);
        pub const MUL132: Self = Self(0x83);
        pub const MUL133: Self = Self(0x84);
        pub const MUL134: Self = Self(0x85);
        pub const MUL135: Self = Self(0x86);
        pub const MUL136: Self = Self(0x87);
        pub const MUL137: Self = Self(0x88);
        pub const MUL138: Self = Self(0x89);
        pub const MUL139: Self = Self(0x8a);
        pub const MUL140: Self = Self(0x8b);
        pub const MUL141: Self = Self(0x8c);
        pub const MUL142: Self = Self(0x8d);
        pub const MUL143: Self = Self(0x8e);
        pub const MUL144: Self = Self(0x8f);
        pub const MUL145: Self = Self(0x90);
        pub const MUL146: Self = Self(0x91);
        pub const MUL147: Self = Self(0x92);
        pub const MUL148: Self = Self(0x93);
        pub const MUL149: Self = Self(0x94);
        pub const MUL150: Self = Self(0x95);
        pub const MUL151: Self = Self(0x96);
        pub const MUL152: Self = Self(0x97);
        pub const MUL153: Self = Self(0x98);
        pub const MUL154: Self = Self(0x99);
        pub const MUL155: Self = Self(0x9a);
        pub const MUL156: Self = Self(0x9b);
        pub const MUL157: Self = Self(0x9c);
        pub const MUL158: Self = Self(0x9d);
        pub const MUL159: Self = Self(0x9e);
        pub const MUL160: Self = Self(0x9f);
        pub const MUL161: Self = Self(0xa0);
        pub const MUL162: Self = Self(0xa1);
        pub const MUL163: Self = Self(0xa2);
        pub const MUL164: Self = Self(0xa3);
        pub const MUL165: Self = Self(0xa4);
        pub const MUL166: Self = Self(0xa5);
        pub const MUL167: Self = Self(0xa6);
        pub const MUL168: Self = Self(0xa7);
        pub const MUL169: Self = Self(0xa8);
        pub const MUL170: Self = Self(0xa9);
        pub const MUL171: Self = Self(0xaa);
        pub const MUL172: Self = Self(0xab);
        pub const MUL173: Self = Self(0xac);
        pub const MUL174: Self = Self(0xad);
        pub const MUL175: Self = Self(0xae);
        pub const MUL176: Self = Self(0xaf);
        pub const MUL177: Self = Self(0xb0);
        pub const MUL178: Self = Self(0xb1);
        pub const MUL179: Self = Self(0xb2);
        pub const MUL180: Self = Self(0xb3);
        pub const MUL181: Self = Self(0xb4);
        pub const MUL182: Self = Self(0xb5);
        pub const MUL183: Self = Self(0xb6);
        pub const MUL184: Self = Self(0xb7);
        pub const MUL185: Self = Self(0xb8);
        pub const MUL186: Self = Self(0xb9);
        pub const MUL187: Self = Self(0xba);
        pub const MUL188: Self = Self(0xbb);
        pub const MUL189: Self = Self(0xbc);
        pub const MUL190: Self = Self(0xbd);
        pub const MUL191: Self = Self(0xbe);
        pub const MUL192: Self = Self(0xbf);
        pub const MUL193: Self = Self(0xc0);
        pub const MUL194: Self = Self(0xc1);
        pub const MUL195: Self = Self(0xc2);
        pub const MUL196: Self = Self(0xc3);
        pub const MUL197: Self = Self(0xc4);
        pub const MUL198: Self = Self(0xc5);
        pub const MUL199: Self = Self(0xc6);
        pub const MUL200: Self = Self(0xc7);
        pub const MUL201: Self = Self(0xc8);
        pub const MUL202: Self = Self(0xc9);
        pub const MUL203: Self = Self(0xca);
        pub const MUL204: Self = Self(0xcb);
        pub const MUL205: Self = Self(0xcc);
        pub const MUL206: Self = Self(0xcd);
        pub const MUL207: Self = Self(0xce);
        pub const MUL208: Self = Self(0xcf);
        pub const MUL209: Self = Self(0xd0);
        pub const MUL210: Self = Self(0xd1);
        pub const MUL211: Self = Self(0xd2);
        pub const MUL212: Self = Self(0xd3);
        pub const MUL213: Self = Self(0xd4);
        pub const MUL214: Self = Self(0xd5);
        pub const MUL215: Self = Self(0xd6);
        pub const MUL216: Self = Self(0xd7);
        pub const MUL217: Self = Self(0xd8);
        pub const MUL218: Self = Self(0xd9);
        pub const MUL219: Self = Self(0xda);
        pub const MUL220: Self = Self(0xdb);
        pub const MUL221: Self = Self(0xdc);
        pub const MUL222: Self = Self(0xdd);
        pub const MUL223: Self = Self(0xde);
        pub const MUL224: Self = Self(0xdf);
        pub const MUL225: Self = Self(0xe0);
        pub const MUL226: Self = Self(0xe1);
        pub const MUL227: Self = Self(0xe2);
        pub const MUL228: Self = Self(0xe3);
        pub const MUL229: Self = Self(0xe4);
        pub const MUL230: Self = Self(0xe5);
        pub const MUL231: Self = Self(0xe6);
        pub const MUL232: Self = Self(0xe7);
        pub const MUL233: Self = Self(0xe8);
        pub const MUL234: Self = Self(0xe9);
        pub const MUL235: Self = Self(0xea);
        pub const MUL236: Self = Self(0xeb);
        pub const MUL237: Self = Self(0xec);
        pub const MUL238: Self = Self(0xed);
        pub const MUL239: Self = Self(0xee);
        pub const MUL240: Self = Self(0xef);
        pub const MUL241: Self = Self(0xf0);
        pub const MUL242: Self = Self(0xf1);
        pub const MUL243: Self = Self(0xf2);
        pub const MUL244: Self = Self(0xf3);
        pub const MUL245: Self = Self(0xf4);
        pub const MUL246: Self = Self(0xf5);
        pub const MUL247: Self = Self(0xf6);
        pub const MUL248: Self = Self(0xf7);
        pub const MUL249: Self = Self(0xf8);
        pub const MUL250: Self = Self(0xf9);
        pub const MUL251: Self = Self(0xfa);
        pub const MUL252: Self = Self(0xfb);
        pub const MUL253: Self = Self(0xfc);
        pub const MUL254: Self = Self(0xfd);
        pub const MUL255: Self = Self(0xfe);
        pub const MUL256: Self = Self(0xff);
        pub const MUL257: Self = Self(0x0100);
        pub const MUL258: Self = Self(0x0101);
        pub const MUL259: Self = Self(0x0102);
        pub const MUL260: Self = Self(0x0103);
        pub const MUL261: Self = Self(0x0104);
        pub const MUL262: Self = Self(0x0105);
        pub const MUL263: Self = Self(0x0106);
        pub const MUL264: Self = Self(0x0107);
        pub const MUL265: Self = Self(0x0108);
        pub const MUL266: Self = Self(0x0109);
        pub const MUL267: Self = Self(0x010a);
        pub const MUL268: Self = Self(0x010b);
        pub const MUL269: Self = Self(0x010c);
        pub const MUL270: Self = Self(0x010d);
        pub const MUL271: Self = Self(0x010e);
        pub const MUL272: Self = Self(0x010f);
        pub const MUL273: Self = Self(0x0110);
        pub const MUL274: Self = Self(0x0111);
        pub const MUL275: Self = Self(0x0112);
        pub const MUL276: Self = Self(0x0113);
        pub const MUL277: Self = Self(0x0114);
        pub const MUL278: Self = Self(0x0115);
        pub const MUL279: Self = Self(0x0116);
        pub const MUL280: Self = Self(0x0117);
        pub const MUL281: Self = Self(0x0118);
        pub const MUL282: Self = Self(0x0119);
        pub const MUL283: Self = Self(0x011a);
        pub const MUL284: Self = Self(0x011b);
        pub const MUL285: Self = Self(0x011c);
        pub const MUL286: Self = Self(0x011d);
        pub const MUL287: Self = Self(0x011e);
        pub const MUL288: Self = Self(0x011f);
        pub const MUL289: Self = Self(0x0120);
        pub const MUL290: Self = Self(0x0121);
        pub const MUL291: Self = Self(0x0122);
        pub const MUL292: Self = Self(0x0123);
        pub const MUL293: Self = Self(0x0124);
        pub const MUL294: Self = Self(0x0125);
        pub const MUL295: Self = Self(0x0126);
        pub const MUL296: Self = Self(0x0127);
        pub const MUL297: Self = Self(0x0128);
        pub const MUL298: Self = Self(0x0129);
        pub const MUL299: Self = Self(0x012a);
        pub const MUL300: Self = Self(0x012b);
        pub const MUL301: Self = Self(0x012c);
        pub const MUL302: Self = Self(0x012d);
        pub const MUL303: Self = Self(0x012e);
        pub const MUL304: Self = Self(0x012f);
        pub const MUL305: Self = Self(0x0130);
        pub const MUL306: Self = Self(0x0131);
        pub const MUL307: Self = Self(0x0132);
        pub const MUL308: Self = Self(0x0133);
        pub const MUL309: Self = Self(0x0134);
        pub const MUL310: Self = Self(0x0135);
        pub const MUL311: Self = Self(0x0136);
        pub const MUL312: Self = Self(0x0137);
        pub const MUL313: Self = Self(0x0138);
        pub const MUL314: Self = Self(0x0139);
        pub const MUL315: Self = Self(0x013a);
        pub const MUL316: Self = Self(0x013b);
        pub const MUL317: Self = Self(0x013c);
        pub const MUL318: Self = Self(0x013d);
        pub const MUL319: Self = Self(0x013e);
        pub const MUL320: Self = Self(0x013f);
        pub const MUL321: Self = Self(0x0140);
        pub const MUL322: Self = Self(0x0141);
        pub const MUL323: Self = Self(0x0142);
        pub const MUL324: Self = Self(0x0143);
        pub const MUL325: Self = Self(0x0144);
        pub const MUL326: Self = Self(0x0145);
        pub const MUL327: Self = Self(0x0146);
        pub const MUL328: Self = Self(0x0147);
        pub const MUL329: Self = Self(0x0148);
        pub const MUL330: Self = Self(0x0149);
        pub const MUL331: Self = Self(0x014a);
        pub const MUL332: Self = Self(0x014b);
        pub const MUL333: Self = Self(0x014c);
        pub const MUL334: Self = Self(0x014d);
        pub const MUL335: Self = Self(0x014e);
        pub const MUL336: Self = Self(0x014f);
        pub const MUL337: Self = Self(0x0150);
        pub const MUL338: Self = Self(0x0151);
        pub const MUL339: Self = Self(0x0152);
        pub const MUL340: Self = Self(0x0153);
        pub const MUL341: Self = Self(0x0154);
        pub const MUL342: Self = Self(0x0155);
        pub const MUL343: Self = Self(0x0156);
        pub const MUL344: Self = Self(0x0157);
        pub const MUL345: Self = Self(0x0158);
        pub const MUL346: Self = Self(0x0159);
        pub const MUL347: Self = Self(0x015a);
        pub const MUL348: Self = Self(0x015b);
        pub const MUL349: Self = Self(0x015c);
        pub const MUL350: Self = Self(0x015d);
        pub const MUL351: Self = Self(0x015e);
        pub const MUL352: Self = Self(0x015f);
        pub const MUL353: Self = Self(0x0160);
        pub const MUL354: Self = Self(0x0161);
        pub const MUL355: Self = Self(0x0162);
        pub const MUL356: Self = Self(0x0163);
        pub const MUL357: Self = Self(0x0164);
        pub const MUL358: Self = Self(0x0165);
        pub const MUL359: Self = Self(0x0166);
        pub const MUL360: Self = Self(0x0167);
        pub const MUL361: Self = Self(0x0168);
        pub const MUL362: Self = Self(0x0169);
        pub const MUL363: Self = Self(0x016a);
        pub const MUL364: Self = Self(0x016b);
        pub const MUL365: Self = Self(0x016c);
        pub const MUL366: Self = Self(0x016d);
        pub const MUL367: Self = Self(0x016e);
        pub const MUL368: Self = Self(0x016f);
        pub const MUL369: Self = Self(0x0170);
        pub const MUL370: Self = Self(0x0171);
        pub const MUL371: Self = Self(0x0172);
        pub const MUL372: Self = Self(0x0173);
        pub const MUL373: Self = Self(0x0174);
        pub const MUL374: Self = Self(0x0175);
        pub const MUL375: Self = Self(0x0176);
        pub const MUL376: Self = Self(0x0177);
        pub const MUL377: Self = Self(0x0178);
        pub const MUL378: Self = Self(0x0179);
        pub const MUL379: Self = Self(0x017a);
        pub const MUL380: Self = Self(0x017b);
        pub const MUL381: Self = Self(0x017c);
        pub const MUL382: Self = Self(0x017d);
        pub const MUL383: Self = Self(0x017e);
        pub const MUL384: Self = Self(0x017f);
        pub const MUL385: Self = Self(0x0180);
        pub const MUL386: Self = Self(0x0181);
        pub const MUL387: Self = Self(0x0182);
        pub const MUL388: Self = Self(0x0183);
        pub const MUL389: Self = Self(0x0184);
        pub const MUL390: Self = Self(0x0185);
        pub const MUL391: Self = Self(0x0186);
        pub const MUL392: Self = Self(0x0187);
        pub const MUL393: Self = Self(0x0188);
        pub const MUL394: Self = Self(0x0189);
        pub const MUL395: Self = Self(0x018a);
        pub const MUL396: Self = Self(0x018b);
        pub const MUL397: Self = Self(0x018c);
        pub const MUL398: Self = Self(0x018d);
        pub const MUL399: Self = Self(0x018e);
        pub const MUL400: Self = Self(0x018f);
        pub const MUL401: Self = Self(0x0190);
        pub const MUL402: Self = Self(0x0191);
        pub const MUL403: Self = Self(0x0192);
        pub const MUL404: Self = Self(0x0193);
        pub const MUL405: Self = Self(0x0194);
        pub const MUL406: Self = Self(0x0195);
        pub const MUL407: Self = Self(0x0196);
        pub const MUL408: Self = Self(0x0197);
        pub const MUL409: Self = Self(0x0198);
        pub const MUL410: Self = Self(0x0199);
        pub const MUL411: Self = Self(0x019a);
        pub const MUL412: Self = Self(0x019b);
        pub const MUL413: Self = Self(0x019c);
        pub const MUL414: Self = Self(0x019d);
        pub const MUL415: Self = Self(0x019e);
        pub const MUL416: Self = Self(0x019f);
        pub const MUL417: Self = Self(0x01a0);
        pub const MUL418: Self = Self(0x01a1);
        pub const MUL419: Self = Self(0x01a2);
        pub const MUL420: Self = Self(0x01a3);
        pub const MUL421: Self = Self(0x01a4);
        pub const MUL422: Self = Self(0x01a5);
        pub const MUL423: Self = Self(0x01a6);
        pub const MUL424: Self = Self(0x01a7);
        pub const MUL425: Self = Self(0x01a8);
        pub const MUL426: Self = Self(0x01a9);
        pub const MUL427: Self = Self(0x01aa);
        pub const MUL428: Self = Self(0x01ab);
        pub const MUL429: Self = Self(0x01ac);
        pub const MUL430: Self = Self(0x01ad);
        pub const MUL431: Self = Self(0x01ae);
        pub const MUL432: Self = Self(0x01af);
        pub const MUL433: Self = Self(0x01b0);
        pub const MUL434: Self = Self(0x01b1);
        pub const MUL435: Self = Self(0x01b2);
        pub const MUL436: Self = Self(0x01b3);
        pub const MUL437: Self = Self(0x01b4);
        pub const MUL438: Self = Self(0x01b5);
        pub const MUL439: Self = Self(0x01b6);
        pub const MUL440: Self = Self(0x01b7);
        pub const MUL441: Self = Self(0x01b8);
        pub const MUL442: Self = Self(0x01b9);
        pub const MUL443: Self = Self(0x01ba);
        pub const MUL444: Self = Self(0x01bb);
        pub const MUL445: Self = Self(0x01bc);
        pub const MUL446: Self = Self(0x01bd);
        pub const MUL447: Self = Self(0x01be);
        pub const MUL448: Self = Self(0x01bf);
        pub const MUL449: Self = Self(0x01c0);
        pub const MUL450: Self = Self(0x01c1);
        pub const MUL451: Self = Self(0x01c2);
        pub const MUL452: Self = Self(0x01c3);
        pub const MUL453: Self = Self(0x01c4);
        pub const MUL454: Self = Self(0x01c5);
        pub const MUL455: Self = Self(0x01c6);
        pub const MUL456: Self = Self(0x01c7);
        pub const MUL457: Self = Self(0x01c8);
        pub const MUL458: Self = Self(0x01c9);
        pub const MUL459: Self = Self(0x01ca);
        pub const MUL460: Self = Self(0x01cb);
        pub const MUL461: Self = Self(0x01cc);
        pub const MUL462: Self = Self(0x01cd);
        pub const MUL463: Self = Self(0x01ce);
        pub const MUL464: Self = Self(0x01cf);
        pub const MUL465: Self = Self(0x01d0);
        pub const MUL466: Self = Self(0x01d1);
        pub const MUL467: Self = Self(0x01d2);
        pub const MUL468: Self = Self(0x01d3);
        pub const MUL469: Self = Self(0x01d4);
        pub const MUL470: Self = Self(0x01d5);
        pub const MUL471: Self = Self(0x01d6);
        pub const MUL472: Self = Self(0x01d7);
        pub const MUL473: Self = Self(0x01d8);
        pub const MUL474: Self = Self(0x01d9);
        pub const MUL475: Self = Self(0x01da);
        pub const MUL476: Self = Self(0x01db);
        pub const MUL477: Self = Self(0x01dc);
        pub const MUL478: Self = Self(0x01dd);
        pub const MUL479: Self = Self(0x01de);
        pub const MUL480: Self = Self(0x01df);
        pub const MUL481: Self = Self(0x01e0);
        pub const MUL482: Self = Self(0x01e1);
        pub const MUL483: Self = Self(0x01e2);
        pub const MUL484: Self = Self(0x01e3);
        pub const MUL485: Self = Self(0x01e4);
        pub const MUL486: Self = Self(0x01e5);
        pub const MUL487: Self = Self(0x01e6);
        pub const MUL488: Self = Self(0x01e7);
        pub const MUL489: Self = Self(0x01e8);
        pub const MUL490: Self = Self(0x01e9);
        pub const MUL491: Self = Self(0x01ea);
        pub const MUL492: Self = Self(0x01eb);
        pub const MUL493: Self = Self(0x01ec);
        pub const MUL494: Self = Self(0x01ed);
        pub const MUL495: Self = Self(0x01ee);
        pub const MUL496: Self = Self(0x01ef);
        pub const MUL497: Self = Self(0x01f0);
        pub const MUL498: Self = Self(0x01f1);
        pub const MUL499: Self = Self(0x01f2);
        pub const MUL500: Self = Self(0x01f3);
        pub const MUL501: Self = Self(0x01f4);
        pub const MUL502: Self = Self(0x01f5);
        pub const MUL503: Self = Self(0x01f6);
        pub const MUL504: Self = Self(0x01f7);
        pub const MUL505: Self = Self(0x01f8);
        pub const MUL506: Self = Self(0x01f9);
        pub const MUL507: Self = Self(0x01fa);
        pub const MUL508: Self = Self(0x01fb);
        pub const MUL509: Self = Self(0x01fc);
        pub const MUL510: Self = Self(0x01fd);
        pub const MUL511: Self = Self(0x01fe);
        pub const MUL512: Self = Self(0x01ff);
    }
    impl Plln {
        pub const fn from_bits(val: u16) -> Plln {
            Self(val & 0x01ff)
        }
        pub const fn to_bits(self) -> u16 {
            self.0
        }
    }
    impl core::fmt::Debug for Plln {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x03 => f.write_str("MUL4"),
                0x04 => f.write_str("MUL5"),
                0x05 => f.write_str("MUL6"),
                0x06 => f.write_str("MUL7"),
                0x07 => f.write_str("MUL8"),
                0x08 => f.write_str("MUL9"),
                0x09 => f.write_str("MUL10"),
                0x0a => f.write_str("MUL11"),
                0x0b => f.write_str("MUL12"),
                0x0c => f.write_str("MUL13"),
                0x0d => f.write_str("MUL14"),
                0x0e => f.write_str("MUL15"),
                0x0f => f.write_str("MUL16"),
                0x10 => f.write_str("MUL17"),
                0x11 => f.write_str("MUL18"),
                0x12 => f.write_str("MUL19"),
                0x13 => f.write_str("MUL20"),
                0x14 => f.write_str("MUL21"),
                0x15 => f.write_str("MUL22"),
                0x16 => f.write_str("MUL23"),
                0x17 => f.write_str("MUL24"),
                0x18 => f.write_str("MUL25"),
                0x19 => f.write_str("MUL26"),
                0x1a => f.write_str("MUL27"),
                0x1b => f.write_str("MUL28"),
                0x1c => f.write_str("MUL29"),
                0x1d => f.write_str("MUL30"),
                0x1e => f.write_str("MUL31"),
                0x1f => f.write_str("MUL32"),
                0x20 => f.write_str("MUL33"),
                0x21 => f.write_str("MUL34"),
                0x22 => f.write_str("MUL35"),
                0x23 => f.write_str("MUL36"),
                0x24 => f.write_str("MUL37"),
                0x25 => f.write_str("MUL38"),
                0x26 => f.write_str("MUL39"),
                0x27 => f.write_str("MUL40"),
                0x28 => f.write_str("MUL41"),
                0x29 => f.write_str("MUL42"),
                0x2a => f.write_str("MUL43"),
                0x2b => f.write_str("MUL44"),
                0x2c => f.write_str("MUL45"),
                0x2d => f.write_str("MUL46"),
                0x2e => f.write_str("MUL47"),
                0x2f => f.write_str("MUL48"),
                0x30 => f.write_str("MUL49"),
                0x31 => f.write_str("MUL50"),
                0x32 => f.write_str("MUL51"),
                0x33 => f.write_str("MUL52"),
                0x34 => f.write_str("MUL53"),
                0x35 => f.write_str("MUL54"),
                0x36 => f.write_str("MUL55"),
                0x37 => f.write_str("MUL56"),
                0x38 => f.write_str("MUL57"),
                0x39 => f.write_str("MUL58"),
                0x3a => f.write_str("MUL59"),
                0x3b => f.write_str("MUL60"),
                0x3c => f.write_str("MUL61"),
                0x3d => f.write_str("MUL62"),
                0x3e => f.write_str("MUL63"),
                0x3f => f.write_str("MUL64"),
                0x40 => f.write_str("MUL65"),
                0x41 => f.write_str("MUL66"),
                0x42 => f.write_str("MUL67"),
                0x43 => f.write_str("MUL68"),
                0x44 => f.write_str("MUL69"),
                0x45 => f.write_str("MUL70"),
                0x46 => f.write_str("MUL71"),
                0x47 => f.write_str("MUL72"),
                0x48 => f.write_str("MUL73"),
                0x49 => f.write_str("MUL74"),
                0x4a => f.write_str("MUL75"),
                0x4b => f.write_str("MUL76"),
                0x4c => f.write_str("MUL77"),
                0x4d => f.write_str("MUL78"),
                0x4e => f.write_str("MUL79"),
                0x4f => f.write_str("MUL80"),
                0x50 => f.write_str("MUL81"),
                0x51 => f.write_str("MUL82"),
                0x52 => f.write_str("MUL83"),
                0x53 => f.write_str("MUL84"),
                0x54 => f.write_str("MUL85"),
                0x55 => f.write_str("MUL86"),
                0x56 => f.write_str("MUL87"),
                0x57 => f.write_str("MUL88"),
                0x58 => f.write_str("MUL89"),
                0x59 => f.write_str("MUL90"),
                0x5a => f.write_str("MUL91"),
                0x5b => f.write_str("MUL92"),
                0x5c => f.write_str("MUL93"),
                0x5d => f.write_str("MUL94"),
                0x5e => f.write_str("MUL95"),
                0x5f => f.write_str("MUL96"),
                0x60 => f.write_str("MUL97"),
                0x61 => f.write_str("MUL98"),
                0x62 => f.write_str("MUL99"),
                0x63 => f.write_str("MUL100"),
                0x64 => f.write_str("MUL101"),
                0x65 => f.write_str("MUL102"),
                0x66 => f.write_str("MUL103"),
                0x67 => f.write_str("MUL104"),
                0x68 => f.write_str("MUL105"),
                0x69 => f.write_str("MUL106"),
                0x6a => f.write_str("MUL107"),
                0x6b => f.write_str("MUL108"),
                0x6c => f.write_str("MUL109"),
                0x6d => f.write_str("MUL110"),
                0x6e => f.write_str("MUL111"),
                0x6f => f.write_str("MUL112"),
                0x70 => f.write_str("MUL113"),
                0x71 => f.write_str("MUL114"),
                0x72 => f.write_str("MUL115"),
                0x73 => f.write_str("MUL116"),
                0x74 => f.write_str("MUL117"),
                0x75 => f.write_str("MUL118"),
                0x76 => f.write_str("MUL119"),
                0x77 => f.write_str("MUL120"),
                0x78 => f.write_str("MUL121"),
                0x79 => f.write_str("MUL122"),
                0x7a => f.write_str("MUL123"),
                0x7b => f.write_str("MUL124"),
                0x7c => f.write_str("MUL125"),
                0x7d => f.write_str("MUL126"),
                0x7e => f.write_str("MUL127"),
                0x7f => f.write_str("MUL128"),
                0x80 => f.write_str("MUL129"),
                0x81 => f.write_str("MUL130"),
                0x82 => f.write_str("MUL131"),
                0x83 => f.write_str("MUL132"),
                0x84 => f.write_str("MUL133"),
                0x85 => f.write_str("MUL134"),
                0x86 => f.write_str("MUL135"),
                0x87 => f.write_str("MUL136"),
                0x88 => f.write_str("MUL137"),
                0x89 => f.write_str("MUL138"),
                0x8a => f.write_str("MUL139"),
                0x8b => f.write_str("MUL140"),
                0x8c => f.write_str("MUL141"),
                0x8d => f.write_str("MUL142"),
                0x8e => f.write_str("MUL143"),
                0x8f => f.write_str("MUL144"),
                0x90 => f.write_str("MUL145"),
                0x91 => f.write_str("MUL146"),
                0x92 => f.write_str("MUL147"),
                0x93 => f.write_str("MUL148"),
                0x94 => f.write_str("MUL149"),
                0x95 => f.write_str("MUL150"),
                0x96 => f.write_str("MUL151"),
                0x97 => f.write_str("MUL152"),
                0x98 => f.write_str("MUL153"),
                0x99 => f.write_str("MUL154"),
                0x9a => f.write_str("MUL155"),
                0x9b => f.write_str("MUL156"),
                0x9c => f.write_str("MUL157"),
                0x9d => f.write_str("MUL158"),
                0x9e => f.write_str("MUL159"),
                0x9f => f.write_str("MUL160"),
                0xa0 => f.write_str("MUL161"),
                0xa1 => f.write_str("MUL162"),
                0xa2 => f.write_str("MUL163"),
                0xa3 => f.write_str("MUL164"),
                0xa4 => f.write_str("MUL165"),
                0xa5 => f.write_str("MUL166"),
                0xa6 => f.write_str("MUL167"),
                0xa7 => f.write_str("MUL168"),
                0xa8 => f.write_str("MUL169"),
                0xa9 => f.write_str("MUL170"),
                0xaa => f.write_str("MUL171"),
                0xab => f.write_str("MUL172"),
                0xac => f.write_str("MUL173"),
                0xad => f.write_str("MUL174"),
                0xae => f.write_str("MUL175"),
                0xaf => f.write_str("MUL176"),
                0xb0 => f.write_str("MUL177"),
                0xb1 => f.write_str("MUL178"),
                0xb2 => f.write_str("MUL179"),
                0xb3 => f.write_str("MUL180"),
                0xb4 => f.write_str("MUL181"),
                0xb5 => f.write_str("MUL182"),
                0xb6 => f.write_str("MUL183"),
                0xb7 => f.write_str("MUL184"),
                0xb8 => f.write_str("MUL185"),
                0xb9 => f.write_str("MUL186"),
                0xba => f.write_str("MUL187"),
                0xbb => f.write_str("MUL188"),
                0xbc => f.write_str("MUL189"),
                0xbd => f.write_str("MUL190"),
                0xbe => f.write_str("MUL191"),
                0xbf => f.write_str("MUL192"),
                0xc0 => f.write_str("MUL193"),
                0xc1 => f.write_str("MUL194"),
                0xc2 => f.write_str("MUL195"),
                0xc3 => f.write_str("MUL196"),
                0xc4 => f.write_str("MUL197"),
                0xc5 => f.write_str("MUL198"),
                0xc6 => f.write_str("MUL199"),
                0xc7 => f.write_str("MUL200"),
                0xc8 => f.write_str("MUL201"),
                0xc9 => f.write_str("MUL202"),
                0xca => f.write_str("MUL203"),
                0xcb => f.write_str("MUL204"),
                0xcc => f.write_str("MUL205"),
                0xcd => f.write_str("MUL206"),
                0xce => f.write_str("MUL207"),
                0xcf => f.write_str("MUL208"),
                0xd0 => f.write_str("MUL209"),
                0xd1 => f.write_str("MUL210"),
                0xd2 => f.write_str("MUL211"),
                0xd3 => f.write_str("MUL212"),
                0xd4 => f.write_str("MUL213"),
                0xd5 => f.write_str("MUL214"),
                0xd6 => f.write_str("MUL215"),
                0xd7 => f.write_str("MUL216"),
                0xd8 => f.write_str("MUL217"),
                0xd9 => f.write_str("MUL218"),
                0xda => f.write_str("MUL219"),
                0xdb => f.write_str("MUL220"),
                0xdc => f.write_str("MUL221"),
                0xdd => f.write_str("MUL222"),
                0xde => f.write_str("MUL223"),
                0xdf => f.write_str("MUL224"),
                0xe0 => f.write_str("MUL225"),
                0xe1 => f.write_str("MUL226"),
                0xe2 => f.write_str("MUL227"),
                0xe3 => f.write_str("MUL228"),
                0xe4 => f.write_str("MUL229"),
                0xe5 => f.write_str("MUL230"),
                0xe6 => f.write_str("MUL231"),
                0xe7 => f.write_str("MUL232"),
                0xe8 => f.write_str("MUL233"),
                0xe9 => f.write_str("MUL234"),
                0xea => f.write_str("MUL235"),
                0xeb => f.write_str("MUL236"),
                0xec => f.write_str("MUL237"),
                0xed => f.write_str("MUL238"),
                0xee => f.write_str("MUL239"),
                0xef => f.write_str("MUL240"),
                0xf0 => f.write_str("MUL241"),
                0xf1 => f.write_str("MUL242"),
                0xf2 => f.write_str("MUL243"),
                0xf3 => f.write_str("MUL244"),
                0xf4 => f.write_str("MUL245"),
                0xf5 => f.write_str("MUL246"),
                0xf6 => f.write_str("MUL247"),
                0xf7 => f.write_str("MUL248"),
                0xf8 => f.write_str("MUL249"),
                0xf9 => f.write_str("MUL250"),
                0xfa => f.write_str("MUL251"),
                0xfb => f.write_str("MUL252"),
                0xfc => f.write_str("MUL253"),
                0xfd => f.write_str("MUL254"),
                0xfe => f.write_str("MUL255"),
                0xff => f.write_str("MUL256"),
                0x0100 => f.write_str("MUL257"),
                0x0101 => f.write_str("MUL258"),
                0x0102 => f.write_str("MUL259"),
                0x0103 => f.write_str("MUL260"),
                0x0104 => f.write_str("MUL261"),
                0x0105 => f.write_str("MUL262"),
                0x0106 => f.write_str("MUL263"),
                0x0107 => f.write_str("MUL264"),
                0x0108 => f.write_str("MUL265"),
                0x0109 => f.write_str("MUL266"),
                0x010a => f.write_str("MUL267"),
                0x010b => f.write_str("MUL268"),
                0x010c => f.write_str("MUL269"),
                0x010d => f.write_str("MUL270"),
                0x010e => f.write_str("MUL271"),
                0x010f => f.write_str("MUL272"),
                0x0110 => f.write_str("MUL273"),
                0x0111 => f.write_str("MUL274"),
                0x0112 => f.write_str("MUL275"),
                0x0113 => f.write_str("MUL276"),
                0x0114 => f.write_str("MUL277"),
                0x0115 => f.write_str("MUL278"),
                0x0116 => f.write_str("MUL279"),
                0x0117 => f.write_str("MUL280"),
                0x0118 => f.write_str("MUL281"),
                0x0119 => f.write_str("MUL282"),
                0x011a => f.write_str("MUL283"),
                0x011b => f.write_str("MUL284"),
                0x011c => f.write_str("MUL285"),
                0x011d => f.write_str("MUL286"),
                0x011e => f.write_str("MUL287"),
                0x011f => f.write_str("MUL288"),
                0x0120 => f.write_str("MUL289"),
                0x0121 => f.write_str("MUL290"),
                0x0122 => f.write_str("MUL291"),
                0x0123 => f.write_str("MUL292"),
                0x0124 => f.write_str("MUL293"),
                0x0125 => f.write_str("MUL294"),
                0x0126 => f.write_str("MUL295"),
                0x0127 => f.write_str("MUL296"),
                0x0128 => f.write_str("MUL297"),
                0x0129 => f.write_str("MUL298"),
                0x012a => f.write_str("MUL299"),
                0x012b => f.write_str("MUL300"),
                0x012c => f.write_str("MUL301"),
                0x012d => f.write_str("MUL302"),
                0x012e => f.write_str("MUL303"),
                0x012f => f.write_str("MUL304"),
                0x0130 => f.write_str("MUL305"),
                0x0131 => f.write_str("MUL306"),
                0x0132 => f.write_str("MUL307"),
                0x0133 => f.write_str("MUL308"),
                0x0134 => f.write_str("MUL309"),
                0x0135 => f.write_str("MUL310"),
                0x0136 => f.write_str("MUL311"),
                0x0137 => f.write_str("MUL312"),
                0x0138 => f.write_str("MUL313"),
                0x0139 => f.write_str("MUL314"),
                0x013a => f.write_str("MUL315"),
                0x013b => f.write_str("MUL316"),
                0x013c => f.write_str("MUL317"),
                0x013d => f.write_str("MUL318"),
                0x013e => f.write_str("MUL319"),
                0x013f => f.write_str("MUL320"),
                0x0140 => f.write_str("MUL321"),
                0x0141 => f.write_str("MUL322"),
                0x0142 => f.write_str("MUL323"),
                0x0143 => f.write_str("MUL324"),
                0x0144 => f.write_str("MUL325"),
                0x0145 => f.write_str("MUL326"),
                0x0146 => f.write_str("MUL327"),
                0x0147 => f.write_str("MUL328"),
                0x0148 => f.write_str("MUL329"),
                0x0149 => f.write_str("MUL330"),
                0x014a => f.write_str("MUL331"),
                0x014b => f.write_str("MUL332"),
                0x014c => f.write_str("MUL333"),
                0x014d => f.write_str("MUL334"),
                0x014e => f.write_str("MUL335"),
                0x014f => f.write_str("MUL336"),
                0x0150 => f.write_str("MUL337"),
                0x0151 => f.write_str("MUL338"),
                0x0152 => f.write_str("MUL339"),
                0x0153 => f.write_str("MUL340"),
                0x0154 => f.write_str("MUL341"),
                0x0155 => f.write_str("MUL342"),
                0x0156 => f.write_str("MUL343"),
                0x0157 => f.write_str("MUL344"),
                0x0158 => f.write_str("MUL345"),
                0x0159 => f.write_str("MUL346"),
                0x015a => f.write_str("MUL347"),
                0x015b => f.write_str("MUL348"),
                0x015c => f.write_str("MUL349"),
                0x015d => f.write_str("MUL350"),
                0x015e => f.write_str("MUL351"),
                0x015f => f.write_str("MUL352"),
                0x0160 => f.write_str("MUL353"),
                0x0161 => f.write_str("MUL354"),
                0x0162 => f.write_str("MUL355"),
                0x0163 => f.write_str("MUL356"),
                0x0164 => f.write_str("MUL357"),
                0x0165 => f.write_str("MUL358"),
                0x0166 => f.write_str("MUL359"),
                0x0167 => f.write_str("MUL360"),
                0x0168 => f.write_str("MUL361"),
                0x0169 => f.write_str("MUL362"),
                0x016a => f.write_str("MUL363"),
                0x016b => f.write_str("MUL364"),
                0x016c => f.write_str("MUL365"),
                0x016d => f.write_str("MUL366"),
                0x016e => f.write_str("MUL367"),
                0x016f => f.write_str("MUL368"),
                0x0170 => f.write_str("MUL369"),
                0x0171 => f.write_str("MUL370"),
                0x0172 => f.write_str("MUL371"),
                0x0173 => f.write_str("MUL372"),
                0x0174 => f.write_str("MUL373"),
                0x0175 => f.write_str("MUL374"),
                0x0176 => f.write_str("MUL375"),
                0x0177 => f.write_str("MUL376"),
                0x0178 => f.write_str("MUL377"),
                0x0179 => f.write_str("MUL378"),
                0x017a => f.write_str("MUL379"),
                0x017b => f.write_str("MUL380"),
                0x017c => f.write_str("MUL381"),
                0x017d => f.write_str("MUL382"),
                0x017e => f.write_str("MUL383"),
                0x017f => f.write_str("MUL384"),
                0x0180 => f.write_str("MUL385"),
                0x0181 => f.write_str("MUL386"),
                0x0182 => f.write_str("MUL387"),
                0x0183 => f.write_str("MUL388"),
                0x0184 => f.write_str("MUL389"),
                0x0185 => f.write_str("MUL390"),
                0x0186 => f.write_str("MUL391"),
                0x0187 => f.write_str("MUL392"),
                0x0188 => f.write_str("MUL393"),
                0x0189 => f.write_str("MUL394"),
                0x018a => f.write_str("MUL395"),
                0x018b => f.write_str("MUL396"),
                0x018c => f.write_str("MUL397"),
                0x018d => f.write_str("MUL398"),
                0x018e => f.write_str("MUL399"),
                0x018f => f.write_str("MUL400"),
                0x0190 => f.write_str("MUL401"),
                0x0191 => f.write_str("MUL402"),
                0x0192 => f.write_str("MUL403"),
                0x0193 => f.write_str("MUL404"),
                0x0194 => f.write_str("MUL405"),
                0x0195 => f.write_str("MUL406"),
                0x0196 => f.write_str("MUL407"),
                0x0197 => f.write_str("MUL408"),
                0x0198 => f.write_str("MUL409"),
                0x0199 => f.write_str("MUL410"),
                0x019a => f.write_str("MUL411"),
                0x019b => f.write_str("MUL412"),
                0x019c => f.write_str("MUL413"),
                0x019d => f.write_str("MUL414"),
                0x019e => f.write_str("MUL415"),
                0x019f => f.write_str("MUL416"),
                0x01a0 => f.write_str("MUL417"),
                0x01a1 => f.write_str("MUL418"),
                0x01a2 => f.write_str("MUL419"),
                0x01a3 => f.write_str("MUL420"),
                0x01a4 => f.write_str("MUL421"),
                0x01a5 => f.write_str("MUL422"),
                0x01a6 => f.write_str("MUL423"),
                0x01a7 => f.write_str("MUL424"),
                0x01a8 => f.write_str("MUL425"),
                0x01a9 => f.write_str("MUL426"),
                0x01aa => f.write_str("MUL427"),
                0x01ab => f.write_str("MUL428"),
                0x01ac => f.write_str("MUL429"),
                0x01ad => f.write_str("MUL430"),
                0x01ae => f.write_str("MUL431"),
                0x01af => f.write_str("MUL432"),
                0x01b0 => f.write_str("MUL433"),
                0x01b1 => f.write_str("MUL434"),
                0x01b2 => f.write_str("MUL435"),
                0x01b3 => f.write_str("MUL436"),
                0x01b4 => f.write_str("MUL437"),
                0x01b5 => f.write_str("MUL438"),
                0x01b6 => f.write_str("MUL439"),
                0x01b7 => f.write_str("MUL440"),
                0x01b8 => f.write_str("MUL441"),
                0x01b9 => f.write_str("MUL442"),
                0x01ba => f.write_str("MUL443"),
                0x01bb => f.write_str("MUL444"),
                0x01bc => f.write_str("MUL445"),
                0x01bd => f.write_str("MUL446"),
                0x01be => f.write_str("MUL447"),
                0x01bf => f.write_str("MUL448"),
                0x01c0 => f.write_str("MUL449"),
                0x01c1 => f.write_str("MUL450"),
                0x01c2 => f.write_str("MUL451"),
                0x01c3 => f.write_str("MUL452"),
                0x01c4 => f.write_str("MUL453"),
                0x01c5 => f.write_str("MUL454"),
                0x01c6 => f.write_str("MUL455"),
                0x01c7 => f.write_str("MUL456"),
                0x01c8 => f.write_str("MUL457"),
                0x01c9 => f.write_str("MUL458"),
                0x01ca => f.write_str("MUL459"),
                0x01cb => f.write_str("MUL460"),
                0x01cc => f.write_str("MUL461"),
                0x01cd => f.write_str("MUL462"),
                0x01ce => f.write_str("MUL463"),
                0x01cf => f.write_str("MUL464"),
                0x01d0 => f.write_str("MUL465"),
                0x01d1 => f.write_str("MUL466"),
                0x01d2 => f.write_str("MUL467"),
                0x01d3 => f.write_str("MUL468"),
                0x01d4 => f.write_str("MUL469"),
                0x01d5 => f.write_str("MUL470"),
                0x01d6 => f.write_str("MUL471"),
                0x01d7 => f.write_str("MUL472"),
                0x01d8 => f.write_str("MUL473"),
                0x01d9 => f.write_str("MUL474"),
                0x01da => f.write_str("MUL475"),
                0x01db => f.write_str("MUL476"),
                0x01dc => f.write_str("MUL477"),
                0x01dd => f.write_str("MUL478"),
                0x01de => f.write_str("MUL479"),
                0x01df => f.write_str("MUL480"),
                0x01e0 => f.write_str("MUL481"),
                0x01e1 => f.write_str("MUL482"),
                0x01e2 => f.write_str("MUL483"),
                0x01e3 => f.write_str("MUL484"),
                0x01e4 => f.write_str("MUL485"),
                0x01e5 => f.write_str("MUL486"),
                0x01e6 => f.write_str("MUL487"),
                0x01e7 => f.write_str("MUL488"),
                0x01e8 => f.write_str("MUL489"),
                0x01e9 => f.write_str("MUL490"),
                0x01ea => f.write_str("MUL491"),
                0x01eb => f.write_str("MUL492"),
                0x01ec => f.write_str("MUL493"),
                0x01ed => f.write_str("MUL494"),
                0x01ee => f.write_str("MUL495"),
                0x01ef => f.write_str("MUL496"),
                0x01f0 => f.write_str("MUL497"),
                0x01f1 => f.write_str("MUL498"),
                0x01f2 => f.write_str("MUL499"),
                0x01f3 => f.write_str("MUL500"),
                0x01f4 => f.write_str("MUL501"),
                0x01f5 => f.write_str("MUL502"),
                0x01f6 => f.write_str("MUL503"),
                0x01f7 => f.write_str("MUL504"),
                0x01f8 => f.write_str("MUL505"),
                0x01f9 => f.write_str("MUL506"),
                0x01fa => f.write_str("MUL507"),
                0x01fb => f.write_str("MUL508"),
                0x01fc => f.write_str("MUL509"),
                0x01fd => f.write_str("MUL510"),
                0x01fe => f.write_str("MUL511"),
                0x01ff => f.write_str("MUL512"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Plln {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x03 => defmt::write!(f, "MUL4"),
                0x04 => defmt::write!(f, "MUL5"),
                0x05 => defmt::write!(f, "MUL6"),
                0x06 => defmt::write!(f, "MUL7"),
                0x07 => defmt::write!(f, "MUL8"),
                0x08 => defmt::write!(f, "MUL9"),
                0x09 => defmt::write!(f, "MUL10"),
                0x0a => defmt::write!(f, "MUL11"),
                0x0b => defmt::write!(f, "MUL12"),
                0x0c => defmt::write!(f, "MUL13"),
                0x0d => defmt::write!(f, "MUL14"),
                0x0e => defmt::write!(f, "MUL15"),
                0x0f => defmt::write!(f, "MUL16"),
                0x10 => defmt::write!(f, "MUL17"),
                0x11 => defmt::write!(f, "MUL18"),
                0x12 => defmt::write!(f, "MUL19"),
                0x13 => defmt::write!(f, "MUL20"),
                0x14 => defmt::write!(f, "MUL21"),
                0x15 => defmt::write!(f, "MUL22"),
                0x16 => defmt::write!(f, "MUL23"),
                0x17 => defmt::write!(f, "MUL24"),
                0x18 => defmt::write!(f, "MUL25"),
                0x19 => defmt::write!(f, "MUL26"),
                0x1a => defmt::write!(f, "MUL27"),
                0x1b => defmt::write!(f, "MUL28"),
                0x1c => defmt::write!(f, "MUL29"),
                0x1d => defmt::write!(f, "MUL30"),
                0x1e => defmt::write!(f, "MUL31"),
                0x1f => defmt::write!(f, "MUL32"),
                0x20 => defmt::write!(f, "MUL33"),
                0x21 => defmt::write!(f, "MUL34"),
                0x22 => defmt::write!(f, "MUL35"),
                0x23 => defmt::write!(f, "MUL36"),
                0x24 => defmt::write!(f, "MUL37"),
                0x25 => defmt::write!(f, "MUL38"),
                0x26 => defmt::write!(f, "MUL39"),
                0x27 => defmt::write!(f, "MUL40"),
                0x28 => defmt::write!(f, "MUL41"),
                0x29 => defmt::write!(f, "MUL42"),
                0x2a => defmt::write!(f, "MUL43"),
                0x2b => defmt::write!(f, "MUL44"),
                0x2c => defmt::write!(f, "MUL45"),
                0x2d => defmt::write!(f, "MUL46"),
                0x2e => defmt::write!(f, "MUL47"),
                0x2f => defmt::write!(f, "MUL48"),
                0x30 => defmt::write!(f, "MUL49"),
                0x31 => defmt::write!(f, "MUL50"),
                0x32 => defmt::write!(f, "MUL51"),
                0x33 => defmt::write!(f, "MUL52"),
                0x34 => defmt::write!(f, "MUL53"),
                0x35 => defmt::write!(f, "MUL54"),
                0x36 => defmt::write!(f, "MUL55"),
                0x37 => defmt::write!(f, "MUL56"),
                0x38 => defmt::write!(f, "MUL57"),
                0x39 => defmt::write!(f, "MUL58"),
                0x3a => defmt::write!(f, "MUL59"),
                0x3b => defmt::write!(f, "MUL60"),
                0x3c => defmt::write!(f, "MUL61"),
                0x3d => defmt::write!(f, "MUL62"),
                0x3e => defmt::write!(f, "MUL63"),
                0x3f => defmt::write!(f, "MUL64"),
                0x40 => defmt::write!(f, "MUL65"),
                0x41 => defmt::write!(f, "MUL66"),
                0x42 => defmt::write!(f, "MUL67"),
                0x43 => defmt::write!(f, "MUL68"),
                0x44 => defmt::write!(f, "MUL69"),
                0x45 => defmt::write!(f, "MUL70"),
                0x46 => defmt::write!(f, "MUL71"),
                0x47 => defmt::write!(f, "MUL72"),
                0x48 => defmt::write!(f, "MUL73"),
                0x49 => defmt::write!(f, "MUL74"),
                0x4a => defmt::write!(f, "MUL75"),
                0x4b => defmt::write!(f, "MUL76"),
                0x4c => defmt::write!(f, "MUL77"),
                0x4d => defmt::write!(f, "MUL78"),
                0x4e => defmt::write!(f, "MUL79"),
                0x4f => defmt::write!(f, "MUL80"),
                0x50 => defmt::write!(f, "MUL81"),
                0x51 => defmt::write!(f, "MUL82"),
                0x52 => defmt::write!(f, "MUL83"),
                0x53 => defmt::write!(f, "MUL84"),
                0x54 => defmt::write!(f, "MUL85"),
                0x55 => defmt::write!(f, "MUL86"),
                0x56 => defmt::write!(f, "MUL87"),
                0x57 => defmt::write!(f, "MUL88"),
                0x58 => defmt::write!(f, "MUL89"),
                0x59 => defmt::write!(f, "MUL90"),
                0x5a => defmt::write!(f, "MUL91"),
                0x5b => defmt::write!(f, "MUL92"),
                0x5c => defmt::write!(f, "MUL93"),
                0x5d => defmt::write!(f, "MUL94"),
                0x5e => defmt::write!(f, "MUL95"),
                0x5f => defmt::write!(f, "MUL96"),
                0x60 => defmt::write!(f, "MUL97"),
                0x61 => defmt::write!(f, "MUL98"),
                0x62 => defmt::write!(f, "MUL99"),
                0x63 => defmt::write!(f, "MUL100"),
                0x64 => defmt::write!(f, "MUL101"),
                0x65 => defmt::write!(f, "MUL102"),
                0x66 => defmt::write!(f, "MUL103"),
                0x67 => defmt::write!(f, "MUL104"),
                0x68 => defmt::write!(f, "MUL105"),
                0x69 => defmt::write!(f, "MUL106"),
                0x6a => defmt::write!(f, "MUL107"),
                0x6b => defmt::write!(f, "MUL108"),
                0x6c => defmt::write!(f, "MUL109"),
                0x6d => defmt::write!(f, "MUL110"),
                0x6e => defmt::write!(f, "MUL111"),
                0x6f => defmt::write!(f, "MUL112"),
                0x70 => defmt::write!(f, "MUL113"),
                0x71 => defmt::write!(f, "MUL114"),
                0x72 => defmt::write!(f, "MUL115"),
                0x73 => defmt::write!(f, "MUL116"),
                0x74 => defmt::write!(f, "MUL117"),
                0x75 => defmt::write!(f, "MUL118"),
                0x76 => defmt::write!(f, "MUL119"),
                0x77 => defmt::write!(f, "MUL120"),
                0x78 => defmt::write!(f, "MUL121"),
                0x79 => defmt::write!(f, "MUL122"),
                0x7a => defmt::write!(f, "MUL123"),
                0x7b => defmt::write!(f, "MUL124"),
                0x7c => defmt::write!(f, "MUL125"),
                0x7d => defmt::write!(f, "MUL126"),
                0x7e => defmt::write!(f, "MUL127"),
                0x7f => defmt::write!(f, "MUL128"),
                0x80 => defmt::write!(f, "MUL129"),
                0x81 => defmt::write!(f, "MUL130"),
                0x82 => defmt::write!(f, "MUL131"),
                0x83 => defmt::write!(f, "MUL132"),
                0x84 => defmt::write!(f, "MUL133"),
                0x85 => defmt::write!(f, "MUL134"),
                0x86 => defmt::write!(f, "MUL135"),
                0x87 => defmt::write!(f, "MUL136"),
                0x88 => defmt::write!(f, "MUL137"),
                0x89 => defmt::write!(f, "MUL138"),
                0x8a => defmt::write!(f, "MUL139"),
                0x8b => defmt::write!(f, "MUL140"),
                0x8c => defmt::write!(f, "MUL141"),
                0x8d => defmt::write!(f, "MUL142"),
                0x8e => defmt::write!(f, "MUL143"),
                0x8f => defmt::write!(f, "MUL144"),
                0x90 => defmt::write!(f, "MUL145"),
                0x91 => defmt::write!(f, "MUL146"),
                0x92 => defmt::write!(f, "MUL147"),
                0x93 => defmt::write!(f, "MUL148"),
                0x94 => defmt::write!(f, "MUL149"),
                0x95 => defmt::write!(f, "MUL150"),
                0x96 => defmt::write!(f, "MUL151"),
                0x97 => defmt::write!(f, "MUL152"),
                0x98 => defmt::write!(f, "MUL153"),
                0x99 => defmt::write!(f, "MUL154"),
                0x9a => defmt::write!(f, "MUL155"),
                0x9b => defmt::write!(f, "MUL156"),
                0x9c => defmt::write!(f, "MUL157"),
                0x9d => defmt::write!(f, "MUL158"),
                0x9e => defmt::write!(f, "MUL159"),
                0x9f => defmt::write!(f, "MUL160"),
                0xa0 => defmt::write!(f, "MUL161"),
                0xa1 => defmt::write!(f, "MUL162"),
                0xa2 => defmt::write!(f, "MUL163"),
                0xa3 => defmt::write!(f, "MUL164"),
                0xa4 => defmt::write!(f, "MUL165"),
                0xa5 => defmt::write!(f, "MUL166"),
                0xa6 => defmt::write!(f, "MUL167"),
                0xa7 => defmt::write!(f, "MUL168"),
                0xa8 => defmt::write!(f, "MUL169"),
                0xa9 => defmt::write!(f, "MUL170"),
                0xaa => defmt::write!(f, "MUL171"),
                0xab => defmt::write!(f, "MUL172"),
                0xac => defmt::write!(f, "MUL173"),
                0xad => defmt::write!(f, "MUL174"),
                0xae => defmt::write!(f, "MUL175"),
                0xaf => defmt::write!(f, "MUL176"),
                0xb0 => defmt::write!(f, "MUL177"),
                0xb1 => defmt::write!(f, "MUL178"),
                0xb2 => defmt::write!(f, "MUL179"),
                0xb3 => defmt::write!(f, "MUL180"),
                0xb4 => defmt::write!(f, "MUL181"),
                0xb5 => defmt::write!(f, "MUL182"),
                0xb6 => defmt::write!(f, "MUL183"),
                0xb7 => defmt::write!(f, "MUL184"),
                0xb8 => defmt::write!(f, "MUL185"),
                0xb9 => defmt::write!(f, "MUL186"),
                0xba => defmt::write!(f, "MUL187"),
                0xbb => defmt::write!(f, "MUL188"),
                0xbc => defmt::write!(f, "MUL189"),
                0xbd => defmt::write!(f, "MUL190"),
                0xbe => defmt::write!(f, "MUL191"),
                0xbf => defmt::write!(f, "MUL192"),
                0xc0 => defmt::write!(f, "MUL193"),
                0xc1 => defmt::write!(f, "MUL194"),
                0xc2 => defmt::write!(f, "MUL195"),
                0xc3 => defmt::write!(f, "MUL196"),
                0xc4 => defmt::write!(f, "MUL197"),
                0xc5 => defmt::write!(f, "MUL198"),
                0xc6 => defmt::write!(f, "MUL199"),
                0xc7 => defmt::write!(f, "MUL200"),
                0xc8 => defmt::write!(f, "MUL201"),
                0xc9 => defmt::write!(f, "MUL202"),
                0xca => defmt::write!(f, "MUL203"),
                0xcb => defmt::write!(f, "MUL204"),
                0xcc => defmt::write!(f, "MUL205"),
                0xcd => defmt::write!(f, "MUL206"),
                0xce => defmt::write!(f, "MUL207"),
                0xcf => defmt::write!(f, "MUL208"),
                0xd0 => defmt::write!(f, "MUL209"),
                0xd1 => defmt::write!(f, "MUL210"),
                0xd2 => defmt::write!(f, "MUL211"),
                0xd3 => defmt::write!(f, "MUL212"),
                0xd4 => defmt::write!(f, "MUL213"),
                0xd5 => defmt::write!(f, "MUL214"),
                0xd6 => defmt::write!(f, "MUL215"),
                0xd7 => defmt::write!(f, "MUL216"),
                0xd8 => defmt::write!(f, "MUL217"),
                0xd9 => defmt::write!(f, "MUL218"),
                0xda => defmt::write!(f, "MUL219"),
                0xdb => defmt::write!(f, "MUL220"),
                0xdc => defmt::write!(f, "MUL221"),
                0xdd => defmt::write!(f, "MUL222"),
                0xde => defmt::write!(f, "MUL223"),
                0xdf => defmt::write!(f, "MUL224"),
                0xe0 => defmt::write!(f, "MUL225"),
                0xe1 => defmt::write!(f, "MUL226"),
                0xe2 => defmt::write!(f, "MUL227"),
                0xe3 => defmt::write!(f, "MUL228"),
                0xe4 => defmt::write!(f, "MUL229"),
                0xe5 => defmt::write!(f, "MUL230"),
                0xe6 => defmt::write!(f, "MUL231"),
                0xe7 => defmt::write!(f, "MUL232"),
                0xe8 => defmt::write!(f, "MUL233"),
                0xe9 => defmt::write!(f, "MUL234"),
                0xea => defmt::write!(f, "MUL235"),
                0xeb => defmt::write!(f, "MUL236"),
                0xec => defmt::write!(f, "MUL237"),
                0xed => defmt::write!(f, "MUL238"),
                0xee => defmt::write!(f, "MUL239"),
                0xef => defmt::write!(f, "MUL240"),
                0xf0 => defmt::write!(f, "MUL241"),
                0xf1 => defmt::write!(f, "MUL242"),
                0xf2 => defmt::write!(f, "MUL243"),
                0xf3 => defmt::write!(f, "MUL244"),
                0xf4 => defmt::write!(f, "MUL245"),
                0xf5 => defmt::write!(f, "MUL246"),
                0xf6 => defmt::write!(f, "MUL247"),
                0xf7 => defmt::write!(f, "MUL248"),
                0xf8 => defmt::write!(f, "MUL249"),
                0xf9 => defmt::write!(f, "MUL250"),
                0xfa => defmt::write!(f, "MUL251"),
                0xfb => defmt::write!(f, "MUL252"),
                0xfc => defmt::write!(f, "MUL253"),
                0xfd => defmt::write!(f, "MUL254"),
                0xfe => defmt::write!(f, "MUL255"),
                0xff => defmt::write!(f, "MUL256"),
                0x0100 => defmt::write!(f, "MUL257"),
                0x0101 => defmt::write!(f, "MUL258"),
                0x0102 => defmt::write!(f, "MUL259"),
                0x0103 => defmt::write!(f, "MUL260"),
                0x0104 => defmt::write!(f, "MUL261"),
                0x0105 => defmt::write!(f, "MUL262"),
                0x0106 => defmt::write!(f, "MUL263"),
                0x0107 => defmt::write!(f, "MUL264"),
                0x0108 => defmt::write!(f, "MUL265"),
                0x0109 => defmt::write!(f, "MUL266"),
                0x010a => defmt::write!(f, "MUL267"),
                0x010b => defmt::write!(f, "MUL268"),
                0x010c => defmt::write!(f, "MUL269"),
                0x010d => defmt::write!(f, "MUL270"),
                0x010e => defmt::write!(f, "MUL271"),
                0x010f => defmt::write!(f, "MUL272"),
                0x0110 => defmt::write!(f, "MUL273"),
                0x0111 => defmt::write!(f, "MUL274"),
                0x0112 => defmt::write!(f, "MUL275"),
                0x0113 => defmt::write!(f, "MUL276"),
                0x0114 => defmt::write!(f, "MUL277"),
                0x0115 => defmt::write!(f, "MUL278"),
                0x0116 => defmt::write!(f, "MUL279"),
                0x0117 => defmt::write!(f, "MUL280"),
                0x0118 => defmt::write!(f, "MUL281"),
                0x0119 => defmt::write!(f, "MUL282"),
                0x011a => defmt::write!(f, "MUL283"),
                0x011b => defmt::write!(f, "MUL284"),
                0x011c => defmt::write!(f, "MUL285"),
                0x011d => defmt::write!(f, "MUL286"),
                0x011e => defmt::write!(f, "MUL287"),
                0x011f => defmt::write!(f, "MUL288"),
                0x0120 => defmt::write!(f, "MUL289"),
                0x0121 => defmt::write!(f, "MUL290"),
                0x0122 => defmt::write!(f, "MUL291"),
                0x0123 => defmt::write!(f, "MUL292"),
                0x0124 => defmt::write!(f, "MUL293"),
                0x0125 => defmt::write!(f, "MUL294"),
                0x0126 => defmt::write!(f, "MUL295"),
                0x0127 => defmt::write!(f, "MUL296"),
                0x0128 => defmt::write!(f, "MUL297"),
                0x0129 => defmt::write!(f, "MUL298"),
                0x012a => defmt::write!(f, "MUL299"),
                0x012b => defmt::write!(f, "MUL300"),
                0x012c => defmt::write!(f, "MUL301"),
                0x012d => defmt::write!(f, "MUL302"),
                0x012e => defmt::write!(f, "MUL303"),
                0x012f => defmt::write!(f, "MUL304"),
                0x0130 => defmt::write!(f, "MUL305"),
                0x0131 => defmt::write!(f, "MUL306"),
                0x0132 => defmt::write!(f, "MUL307"),
                0x0133 => defmt::write!(f, "MUL308"),
                0x0134 => defmt::write!(f, "MUL309"),
                0x0135 => defmt::write!(f, "MUL310"),
                0x0136 => defmt::write!(f, "MUL311"),
                0x0137 => defmt::write!(f, "MUL312"),
                0x0138 => defmt::write!(f, "MUL313"),
                0x0139 => defmt::write!(f, "MUL314"),
                0x013a => defmt::write!(f, "MUL315"),
                0x013b => defmt::write!(f, "MUL316"),
                0x013c => defmt::write!(f, "MUL317"),
                0x013d => defmt::write!(f, "MUL318"),
                0x013e => defmt::write!(f, "MUL319"),
                0x013f => defmt::write!(f, "MUL320"),
                0x0140 => defmt::write!(f, "MUL321"),
                0x0141 => defmt::write!(f, "MUL322"),
                0x0142 => defmt::write!(f, "MUL323"),
                0x0143 => defmt::write!(f, "MUL324"),
                0x0144 => defmt::write!(f, "MUL325"),
                0x0145 => defmt::write!(f, "MUL326"),
                0x0146 => defmt::write!(f, "MUL327"),
                0x0147 => defmt::write!(f, "MUL328"),
                0x0148 => defmt::write!(f, "MUL329"),
                0x0149 => defmt::write!(f, "MUL330"),
                0x014a => defmt::write!(f, "MUL331"),
                0x014b => defmt::write!(f, "MUL332"),
                0x014c => defmt::write!(f, "MUL333"),
                0x014d => defmt::write!(f, "MUL334"),
                0x014e => defmt::write!(f, "MUL335"),
                0x014f => defmt::write!(f, "MUL336"),
                0x0150 => defmt::write!(f, "MUL337"),
                0x0151 => defmt::write!(f, "MUL338"),
                0x0152 => defmt::write!(f, "MUL339"),
                0x0153 => defmt::write!(f, "MUL340"),
                0x0154 => defmt::write!(f, "MUL341"),
                0x0155 => defmt::write!(f, "MUL342"),
                0x0156 => defmt::write!(f, "MUL343"),
                0x0157 => defmt::write!(f, "MUL344"),
                0x0158 => defmt::write!(f, "MUL345"),
                0x0159 => defmt::write!(f, "MUL346"),
                0x015a => defmt::write!(f, "MUL347"),
                0x015b => defmt::write!(f, "MUL348"),
                0x015c => defmt::write!(f, "MUL349"),
                0x015d => defmt::write!(f, "MUL350"),
                0x015e => defmt::write!(f, "MUL351"),
                0x015f => defmt::write!(f, "MUL352"),
                0x0160 => defmt::write!(f, "MUL353"),
                0x0161 => defmt::write!(f, "MUL354"),
                0x0162 => defmt::write!(f, "MUL355"),
                0x0163 => defmt::write!(f, "MUL356"),
                0x0164 => defmt::write!(f, "MUL357"),
                0x0165 => defmt::write!(f, "MUL358"),
                0x0166 => defmt::write!(f, "MUL359"),
                0x0167 => defmt::write!(f, "MUL360"),
                0x0168 => defmt::write!(f, "MUL361"),
                0x0169 => defmt::write!(f, "MUL362"),
                0x016a => defmt::write!(f, "MUL363"),
                0x016b => defmt::write!(f, "MUL364"),
                0x016c => defmt::write!(f, "MUL365"),
                0x016d => defmt::write!(f, "MUL366"),
                0x016e => defmt::write!(f, "MUL367"),
                0x016f => defmt::write!(f, "MUL368"),
                0x0170 => defmt::write!(f, "MUL369"),
                0x0171 => defmt::write!(f, "MUL370"),
                0x0172 => defmt::write!(f, "MUL371"),
                0x0173 => defmt::write!(f, "MUL372"),
                0x0174 => defmt::write!(f, "MUL373"),
                0x0175 => defmt::write!(f, "MUL374"),
                0x0176 => defmt::write!(f, "MUL375"),
                0x0177 => defmt::write!(f, "MUL376"),
                0x0178 => defmt::write!(f, "MUL377"),
                0x0179 => defmt::write!(f, "MUL378"),
                0x017a => defmt::write!(f, "MUL379"),
                0x017b => defmt::write!(f, "MUL380"),
                0x017c => defmt::write!(f, "MUL381"),
                0x017d => defmt::write!(f, "MUL382"),
                0x017e => defmt::write!(f, "MUL383"),
                0x017f => defmt::write!(f, "MUL384"),
                0x0180 => defmt::write!(f, "MUL385"),
                0x0181 => defmt::write!(f, "MUL386"),
                0x0182 => defmt::write!(f, "MUL387"),
                0x0183 => defmt::write!(f, "MUL388"),
                0x0184 => defmt::write!(f, "MUL389"),
                0x0185 => defmt::write!(f, "MUL390"),
                0x0186 => defmt::write!(f, "MUL391"),
                0x0187 => defmt::write!(f, "MUL392"),
                0x0188 => defmt::write!(f, "MUL393"),
                0x0189 => defmt::write!(f, "MUL394"),
                0x018a => defmt::write!(f, "MUL395"),
                0x018b => defmt::write!(f, "MUL396"),
                0x018c => defmt::write!(f, "MUL397"),
                0x018d => defmt::write!(f, "MUL398"),
                0x018e => defmt::write!(f, "MUL399"),
                0x018f => defmt::write!(f, "MUL400"),
                0x0190 => defmt::write!(f, "MUL401"),
                0x0191 => defmt::write!(f, "MUL402"),
                0x0192 => defmt::write!(f, "MUL403"),
                0x0193 => defmt::write!(f, "MUL404"),
                0x0194 => defmt::write!(f, "MUL405"),
                0x0195 => defmt::write!(f, "MUL406"),
                0x0196 => defmt::write!(f, "MUL407"),
                0x0197 => defmt::write!(f, "MUL408"),
                0x0198 => defmt::write!(f, "MUL409"),
                0x0199 => defmt::write!(f, "MUL410"),
                0x019a => defmt::write!(f, "MUL411"),
                0x019b => defmt::write!(f, "MUL412"),
                0x019c => defmt::write!(f, "MUL413"),
                0x019d => defmt::write!(f, "MUL414"),
                0x019e => defmt::write!(f, "MUL415"),
                0x019f => defmt::write!(f, "MUL416"),
                0x01a0 => defmt::write!(f, "MUL417"),
                0x01a1 => defmt::write!(f, "MUL418"),
                0x01a2 => defmt::write!(f, "MUL419"),
                0x01a3 => defmt::write!(f, "MUL420"),
                0x01a4 => defmt::write!(f, "MUL421"),
                0x01a5 => defmt::write!(f, "MUL422"),
                0x01a6 => defmt::write!(f, "MUL423"),
                0x01a7 => defmt::write!(f, "MUL424"),
                0x01a8 => defmt::write!(f, "MUL425"),
                0x01a9 => defmt::write!(f, "MUL426"),
                0x01aa => defmt::write!(f, "MUL427"),
                0x01ab => defmt::write!(f, "MUL428"),
                0x01ac => defmt::write!(f, "MUL429"),
                0x01ad => defmt::write!(f, "MUL430"),
                0x01ae => defmt::write!(f, "MUL431"),
                0x01af => defmt::write!(f, "MUL432"),
                0x01b0 => defmt::write!(f, "MUL433"),
                0x01b1 => defmt::write!(f, "MUL434"),
                0x01b2 => defmt::write!(f, "MUL435"),
                0x01b3 => defmt::write!(f, "MUL436"),
                0x01b4 => defmt::write!(f, "MUL437"),
                0x01b5 => defmt::write!(f, "MUL438"),
                0x01b6 => defmt::write!(f, "MUL439"),
                0x01b7 => defmt::write!(f, "MUL440"),
                0x01b8 => defmt::write!(f, "MUL441"),
                0x01b9 => defmt::write!(f, "MUL442"),
                0x01ba => defmt::write!(f, "MUL443"),
                0x01bb => defmt::write!(f, "MUL444"),
                0x01bc => defmt::write!(f, "MUL445"),
                0x01bd => defmt::write!(f, "MUL446"),
                0x01be => defmt::write!(f, "MUL447"),
                0x01bf => defmt::write!(f, "MUL448"),
                0x01c0 => defmt::write!(f, "MUL449"),
                0x01c1 => defmt::write!(f, "MUL450"),
                0x01c2 => defmt::write!(f, "MUL451"),
                0x01c3 => defmt::write!(f, "MUL452"),
                0x01c4 => defmt::write!(f, "MUL453"),
                0x01c5 => defmt::write!(f, "MUL454"),
                0x01c6 => defmt::write!(f, "MUL455"),
                0x01c7 => defmt::write!(f, "MUL456"),
                0x01c8 => defmt::write!(f, "MUL457"),
                0x01c9 => defmt::write!(f, "MUL458"),
                0x01ca => defmt::write!(f, "MUL459"),
                0x01cb => defmt::write!(f, "MUL460"),
                0x01cc => defmt::write!(f, "MUL461"),
                0x01cd => defmt::write!(f, "MUL462"),
                0x01ce => defmt::write!(f, "MUL463"),
                0x01cf => defmt::write!(f, "MUL464"),
                0x01d0 => defmt::write!(f, "MUL465"),
                0x01d1 => defmt::write!(f, "MUL466"),
                0x01d2 => defmt::write!(f, "MUL467"),
                0x01d3 => defmt::write!(f, "MUL468"),
                0x01d4 => defmt::write!(f, "MUL469"),
                0x01d5 => defmt::write!(f, "MUL470"),
                0x01d6 => defmt::write!(f, "MUL471"),
                0x01d7 => defmt::write!(f, "MUL472"),
                0x01d8 => defmt::write!(f, "MUL473"),
                0x01d9 => defmt::write!(f, "MUL474"),
                0x01da => defmt::write!(f, "MUL475"),
                0x01db => defmt::write!(f, "MUL476"),
                0x01dc => defmt::write!(f, "MUL477"),
                0x01dd => defmt::write!(f, "MUL478"),
                0x01de => defmt::write!(f, "MUL479"),
                0x01df => defmt::write!(f, "MUL480"),
                0x01e0 => defmt::write!(f, "MUL481"),
                0x01e1 => defmt::write!(f, "MUL482"),
                0x01e2 => defmt::write!(f, "MUL483"),
                0x01e3 => defmt::write!(f, "MUL484"),
                0x01e4 => defmt::write!(f, "MUL485"),
                0x01e5 => defmt::write!(f, "MUL486"),
                0x01e6 => defmt::write!(f, "MUL487"),
                0x01e7 => defmt::write!(f, "MUL488"),
                0x01e8 => defmt::write!(f, "MUL489"),
                0x01e9 => defmt::write!(f, "MUL490"),
                0x01ea => defmt::write!(f, "MUL491"),
                0x01eb => defmt::write!(f, "MUL492"),
                0x01ec => defmt::write!(f, "MUL493"),
                0x01ed => defmt::write!(f, "MUL494"),
                0x01ee => defmt::write!(f, "MUL495"),
                0x01ef => defmt::write!(f, "MUL496"),
                0x01f0 => defmt::write!(f, "MUL497"),
                0x01f1 => defmt::write!(f, "MUL498"),
                0x01f2 => defmt::write!(f, "MUL499"),
                0x01f3 => defmt::write!(f, "MUL500"),
                0x01f4 => defmt::write!(f, "MUL501"),
                0x01f5 => defmt::write!(f, "MUL502"),
                0x01f6 => defmt::write!(f, "MUL503"),
                0x01f7 => defmt::write!(f, "MUL504"),
                0x01f8 => defmt::write!(f, "MUL505"),
                0x01f9 => defmt::write!(f, "MUL506"),
                0x01fa => defmt::write!(f, "MUL507"),
                0x01fb => defmt::write!(f, "MUL508"),
                0x01fc => defmt::write!(f, "MUL509"),
                0x01fd => defmt::write!(f, "MUL510"),
                0x01fe => defmt::write!(f, "MUL511"),
                0x01ff => defmt::write!(f, "MUL512"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u16> for Plln {
        #[inline(always)]
        fn from(val: u16) -> Plln {
            Plln::from_bits(val)
        }
    }
    impl From<Plln> for u16 {
        #[inline(always)]
        fn from(val: Plln) -> u16 {
            Plln::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllrge {
        #[doc = "Frequency is between 1 and 2 MHz"]
        RANGE1 = 0x0,
        #[doc = "Frequency is between 2 and 4 MHz"]
        RANGE2 = 0x01,
        #[doc = "Frequency is between 4 and 8 MHz"]
        RANGE4 = 0x02,
        #[doc = "Frequency is between 8 and 16 MHz"]
        RANGE8 = 0x03,
    }
    impl Pllrge {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllrge {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllrge {
        #[inline(always)]
        fn from(val: u8) -> Pllrge {
            Pllrge::from_bits(val)
        }
    }
    impl From<Pllrge> for u8 {
        #[inline(always)]
        fn from(val: Pllrge) -> u8 {
            Pllrge::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllsrc {
        #[doc = "HSI selected as PLL clock"]
        HSI = 0x0,
        #[doc = "CSI selected as PLL clock"]
        CSI = 0x01,
        #[doc = "HSE selected as PLL clock"]
        HSE = 0x02,
        #[doc = "No clock sent to DIVMx dividers and PLLs"]
        DISABLE = 0x03,
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllvcosel {
        #[doc = "VCO frequency range 192 to 836 MHz"]
        WIDE_VCO = 0x0,
        #[doc = "VCO frequency range 150 to 420 MHz"]
        MEDIUM_VCO = 0x01,
    }
    impl Pllvcosel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllvcosel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllvcosel {
        #[inline(always)]
        fn from(val: u8) -> Pllvcosel {
            Pllvcosel::from_bits(val)
        }
    }
    impl From<Pllvcosel> for u8 {
        #[inline(always)]
        fn from(val: Pllvcosel) -> u8 {
            Pllvcosel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ppre {
        #[doc = "rcc_hclk not divided"]
        DIV1 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "rcc_hclk divided by 2"]
        DIV2 = 0x04,
        #[doc = "rcc_hclk divided by 4"]
        DIV4 = 0x05,
        #[doc = "rcc_hclk divided by 8"]
        DIV8 = 0x06,
        #[doc = "rcc_hclk divided by 16"]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rngsel {
        #[doc = "HSI48 selected as peripheral clock"]
        HSI48 = 0x0,
        #[doc = "pll1_q selected as peripheral clock"]
        PLL1_Q = 0x01,
        #[doc = "LSE selected as peripheral clock"]
        LSE = 0x02,
        #[doc = "LSI selected as peripheral clock"]
        LSI = 0x03,
    }
    impl Rngsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rngsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rngsel {
        #[inline(always)]
        fn from(val: u8) -> Rngsel {
            Rngsel::from_bits(val)
        }
    }
    impl From<Rngsel> for u8 {
        #[inline(always)]
        fn from(val: Rngsel) -> u8 {
            Rngsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Saiasel {
        #[doc = "pll1_q selected as peripheral clock"]
        PLL1_Q = 0x0,
        #[doc = "pll2_p selected as peripheral clock"]
        PLL2_P = 0x01,
        #[doc = "pll3_p selected as peripheral clock"]
        PLL3_P = 0x02,
        #[doc = "i2s_ckin selected as peripheral clock"]
        I2S_CKIN = 0x03,
        #[doc = "PER selected as peripheral clock"]
        PER = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Saiasel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Saiasel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Saiasel {
        #[inline(always)]
        fn from(val: u8) -> Saiasel {
            Saiasel::from_bits(val)
        }
    }
    impl From<Saiasel> for u8 {
        #[inline(always)]
        fn from(val: Saiasel) -> u8 {
            Saiasel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Saisel {
        #[doc = "pll1_q selected as peripheral clock"]
        PLL1_Q = 0x0,
        #[doc = "pll2_p selected as peripheral clock"]
        PLL2_P = 0x01,
        #[doc = "pll3_p selected as peripheral clock"]
        PLL3_P = 0x02,
        #[doc = "I2S_CKIN selected as peripheral clock"]
        I2S_CKIN = 0x03,
        #[doc = "PER selected as peripheral clock"]
        PER = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Saisel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Saisel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Saisel {
        #[inline(always)]
        fn from(val: u8) -> Saisel {
            Saisel::from_bits(val)
        }
    }
    impl From<Saisel> for u8 {
        #[inline(always)]
        fn from(val: Saisel) -> u8 {
            Saisel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sdmmcsel {
        #[doc = "pll1_q selected as peripheral clock"]
        PLL1_Q = 0x0,
        #[doc = "pll2_r selected as peripheral clock"]
        PLL2_R = 0x01,
    }
    impl Sdmmcsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sdmmcsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sdmmcsel {
        #[inline(always)]
        fn from(val: u8) -> Sdmmcsel {
            Sdmmcsel::from_bits(val)
        }
    }
    impl From<Sdmmcsel> for u8 {
        #[inline(always)]
        fn from(val: Sdmmcsel) -> u8 {
            Sdmmcsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Spdifrxsel {
        #[doc = "pll1_q selected as peripheral clock"]
        PLL1_Q = 0x0,
        #[doc = "pll2_r selected as peripheral clock"]
        PLL2_R = 0x01,
        #[doc = "pll3_r selected as peripheral clock"]
        PLL3_R = 0x02,
        #[doc = "hsi_ker selected as peripheral clock"]
        HSI = 0x03,
    }
    impl Spdifrxsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Spdifrxsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Spdifrxsel {
        #[inline(always)]
        fn from(val: u8) -> Spdifrxsel {
            Spdifrxsel::from_bits(val)
        }
    }
    impl From<Spdifrxsel> for u8 {
        #[inline(always)]
        fn from(val: Spdifrxsel) -> u8 {
            Spdifrxsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Spi45sel {
        #[doc = "APB2 clock selected as peripheral clock"]
        PCLK2 = 0x0,
        #[doc = "pll2_q selected as peripheral clock"]
        PLL2_Q = 0x01,
        #[doc = "pll3_q selected as peripheral clock"]
        PLL3_Q = 0x02,
        #[doc = "hsi_ker selected as peripheral clock"]
        HSI = 0x03,
        #[doc = "csi_ker selected as peripheral clock"]
        CSI = 0x04,
        #[doc = "HSE selected as peripheral clock"]
        HSE = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Spi45sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Spi45sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Spi45sel {
        #[inline(always)]
        fn from(val: u8) -> Spi45sel {
            Spi45sel::from_bits(val)
        }
    }
    impl From<Spi45sel> for u8 {
        #[inline(always)]
        fn from(val: Spi45sel) -> u8 {
            Spi45sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Spi6sel {
        #[doc = "rcc_pclk4 selected as peripheral clock"]
        PCLK4 = 0x0,
        #[doc = "pll2_q selected as peripheral clock"]
        PLL2_Q = 0x01,
        #[doc = "pll3_q selected as peripheral clock"]
        PLL3_Q = 0x02,
        #[doc = "hsi_ker selected as peripheral clock"]
        HSI = 0x03,
        #[doc = "csi_ker selected as peripheral clock"]
        CSI = 0x04,
        #[doc = "HSE selected as peripheral clock"]
        HSE = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Spi6sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Spi6sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Spi6sel {
        #[inline(always)]
        fn from(val: u8) -> Spi6sel {
            Spi6sel::from_bits(val)
        }
    }
    impl From<Spi6sel> for u8 {
        #[inline(always)]
        fn from(val: Spi6sel) -> u8 {
            Spi6sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Stopwuck {
        #[doc = "HSI selected as wake up clock from system Stop"]
        HSI = 0x0,
        #[doc = "CSI selected as wake up clock from system Stop"]
        CSI = 0x01,
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sw {
        #[doc = "HSI selected as system clock"]
        HSI = 0x0,
        #[doc = "CSI selected as system clock"]
        CSI = 0x01,
        #[doc = "HSE selected as system clock"]
        HSE = 0x02,
        #[doc = "PLL1 selected as system clock"]
        PLL1_P = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Sw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sw {
            unsafe { core::mem::transmute(val & 0x07) }
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
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Swpmisel {
        #[doc = "pclk selected as peripheral clock"]
        PCLK1 = 0x0,
        #[doc = "hsi_ker selected as peripheral clock"]
        HSI = 0x01,
    }
    impl Swpmisel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Swpmisel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Swpmisel {
        #[inline(always)]
        fn from(val: u8) -> Swpmisel {
            Swpmisel::from_bits(val)
        }
    }
    impl From<Swpmisel> for u8 {
        #[inline(always)]
        fn from(val: Swpmisel) -> u8 {
            Swpmisel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Timpre {
        #[doc = "Timer kernel clock equal to 2x pclk by default"]
        DEFAULT_X2 = 0x0,
        #[doc = "Timer kernel clock equal to 4x pclk by default"]
        DEFAULT_X4 = 0x01,
    }
    impl Timpre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Timpre {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Timpre {
        #[inline(always)]
        fn from(val: u8) -> Timpre {
            Timpre::from_bits(val)
        }
    }
    impl From<Timpre> for u8 {
        #[inline(always)]
        fn from(val: Timpre) -> u8 {
            Timpre::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Usart16910sel {
        #[doc = "rcc_pclk2 selected as peripheral clock"]
        PCLK2 = 0x0,
        #[doc = "pll2_q selected as peripheral clock"]
        PLL2_Q = 0x01,
        #[doc = "pll3_q selected as peripheral clock"]
        PLL3_Q = 0x02,
        #[doc = "hsi_ker selected as peripheral clock"]
        HSI = 0x03,
        #[doc = "csi_ker selected as peripheral clock"]
        CSI = 0x04,
        #[doc = "LSE selected as peripheral clock"]
        LSE = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Usart16910sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usart16910sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usart16910sel {
        #[inline(always)]
        fn from(val: u8) -> Usart16910sel {
            Usart16910sel::from_bits(val)
        }
    }
    impl From<Usart16910sel> for u8 {
        #[inline(always)]
        fn from(val: Usart16910sel) -> u8 {
            Usart16910sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Usart234578sel {
        #[doc = "rcc_pclk1 selected as peripheral clock"]
        PCLK1 = 0x0,
        #[doc = "pll2_q selected as peripheral clock"]
        PLL2_Q = 0x01,
        #[doc = "pll3_q selected as peripheral clock"]
        PLL3_Q = 0x02,
        #[doc = "hsi_ker selected as peripheral clock"]
        HSI = 0x03,
        #[doc = "csi_ker selected as peripheral clock"]
        CSI = 0x04,
        #[doc = "LSE selected as peripheral clock"]
        LSE = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Usart234578sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usart234578sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usart234578sel {
        #[inline(always)]
        fn from(val: u8) -> Usart234578sel {
            Usart234578sel::from_bits(val)
        }
    }
    impl From<Usart234578sel> for u8 {
        #[inline(always)]
        fn from(val: Usart234578sel) -> u8 {
            Usart234578sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Usbsel {
        #[doc = "Disable the kernel clock"]
        DISABLE = 0x0,
        #[doc = "pll1_q selected as peripheral clock"]
        PLL1_Q = 0x01,
        #[doc = "pll3_q selected as peripheral clock"]
        PLL3_Q = 0x02,
        #[doc = "HSI48 selected as peripheral clock"]
        HSI48 = 0x03,
    }
    impl Usbsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usbsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usbsel {
        #[inline(always)]
        fn from(val: u8) -> Usbsel {
            Usbsel::from_bits(val)
        }
    }
    impl From<Usbsel> for u8 {
        #[inline(always)]
        fn from(val: Usbsel) -> u8 {
            Usbsel::to_bits(val)
        }
    }
}
