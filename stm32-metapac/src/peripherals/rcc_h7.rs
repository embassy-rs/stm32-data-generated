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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "RCC HSI configuration register"]
    #[inline(always)]
    pub const fn hsicfgr(self) -> crate::common::Reg<regs::Hsicfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "RCC Internal Clock Source Calibration Register"]
    #[inline(always)]
    pub const fn icscr(self) -> crate::common::Reg<regs::Icscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "RCC Clock Recovery RC Register"]
    #[inline(always)]
    pub const fn crrcr(self) -> crate::common::Reg<regs::Crrcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "RCC CSI configuration register"]
    #[inline(always)]
    pub const fn csicfgr(self) -> crate::common::Reg<regs::Csicfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "RCC Clock Configuration Register"]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "RCC Domain 1 Clock Configuration Register"]
    #[inline(always)]
    pub const fn d1cfgr(self) -> crate::common::Reg<regs::D1cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "RCC Domain 2 Clock Configuration Register"]
    #[inline(always)]
    pub const fn d2cfgr(self) -> crate::common::Reg<regs::D2cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "RCC Domain 3 Clock Configuration Register"]
    #[inline(always)]
    pub const fn d3cfgr(self) -> crate::common::Reg<regs::D3cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "RCC PLLs Clock Source Selection Register"]
    #[inline(always)]
    pub const fn pllckselr(self) -> crate::common::Reg<regs::Pllckselr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "RCC PLLs Configuration Register"]
    #[inline(always)]
    pub const fn pllcfgr(self) -> crate::common::Reg<regs::Pllcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "RCC PLL1 Dividers Configuration Register"]
    #[inline(always)]
    pub const fn plldivr(self, n: usize) -> crate::common::Reg<regs::Plldivr, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize + n * 8usize) as _) }
    }
    #[doc = "RCC PLL1 Fractional Divider Register"]
    #[inline(always)]
    pub const fn pllfracr(self, n: usize) -> crate::common::Reg<regs::Pllfracr, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize + n * 8usize) as _) }
    }
    #[doc = "RCC Domain 1 Kernel Clock Configuration Register"]
    #[inline(always)]
    pub const fn d1ccipr(self) -> crate::common::Reg<regs::D1ccipr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "RCC Domain 2 Kernel Clock Configuration Register"]
    #[inline(always)]
    pub const fn d2ccip1r(self) -> crate::common::Reg<regs::D2ccip1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "RCC Domain 2 Kernel Clock Configuration Register"]
    #[inline(always)]
    pub const fn d2ccip2r(self) -> crate::common::Reg<regs::D2ccip2r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "RCC Domain 3 Kernel Clock Configuration Register"]
    #[inline(always)]
    pub const fn d3ccipr(self) -> crate::common::Reg<regs::D3ccipr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "RCC Clock Source Interrupt Enable Register"]
    #[inline(always)]
    pub const fn cier(self) -> crate::common::Reg<regs::Cier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "RCC Clock Source Interrupt Flag Register"]
    #[inline(always)]
    pub const fn cifr(self) -> crate::common::Reg<regs::Cifr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "RCC Clock Source Interrupt Clear Register"]
    #[inline(always)]
    pub const fn cicr(self) -> crate::common::Reg<regs::Cicr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "RCC Backup Domain Control Register"]
    #[inline(always)]
    pub const fn bdcr(self) -> crate::common::Reg<regs::Bdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "RCC Clock Control and Status Register"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "RCC AHB3 Reset Register"]
    #[inline(always)]
    pub const fn ahb3rstr(self) -> crate::common::Reg<regs::Ahb3rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "RCC AHB1 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn ahb1rstr(self) -> crate::common::Reg<regs::Ahb1rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "RCC AHB2 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn ahb2rstr(self) -> crate::common::Reg<regs::Ahb2rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "RCC AHB4 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn ahb4rstr(self) -> crate::common::Reg<regs::Ahb4rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "RCC APB3 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn apb3rstr(self) -> crate::common::Reg<regs::Apb3rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "RCC APB1 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn apb1lrstr(self) -> crate::common::Reg<regs::Apb1lrstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "RCC APB1 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn apb1hrstr(self) -> crate::common::Reg<regs::Apb1hrstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "RCC APB2 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn apb2rstr(self) -> crate::common::Reg<regs::Apb2rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "RCC APB4 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn apb4rstr(self) -> crate::common::Reg<regs::Apb4rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "Global Control Register"]
    #[inline(always)]
    pub const fn gcr(self) -> crate::common::Reg<regs::Gcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "RCC D3 Autonomous mode Register"]
    #[inline(always)]
    pub const fn d3amr(self) -> crate::common::Reg<regs::D3amr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "RCC Reset Status Register"]
    #[inline(always)]
    pub const fn rsr(self) -> crate::common::Reg<regs::Rsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "RCC AHB3 Clock Register"]
    #[inline(always)]
    pub const fn ahb3enr(self) -> crate::common::Reg<regs::Ahb3enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "RCC AHB1 Clock Register"]
    #[inline(always)]
    pub const fn ahb1enr(self) -> crate::common::Reg<regs::Ahb1enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "RCC AHB2 Clock Register"]
    #[inline(always)]
    pub const fn ahb2enr(self) -> crate::common::Reg<regs::Ahb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "RCC AHB4 Clock Register"]
    #[inline(always)]
    pub const fn ahb4enr(self) -> crate::common::Reg<regs::Ahb4enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "RCC APB3 Clock Register"]
    #[inline(always)]
    pub const fn apb3enr(self) -> crate::common::Reg<regs::Apb3enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "RCC APB1 Clock Register"]
    #[inline(always)]
    pub const fn apb1lenr(self) -> crate::common::Reg<regs::Apb1lenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "RCC APB1 Clock Register"]
    #[inline(always)]
    pub const fn apb1henr(self) -> crate::common::Reg<regs::Apb1henr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "RCC APB2 Clock Register"]
    #[inline(always)]
    pub const fn apb2enr(self) -> crate::common::Reg<regs::Apb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "RCC APB4 Clock Register"]
    #[inline(always)]
    pub const fn apb4enr(self) -> crate::common::Reg<regs::Apb4enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "RCC AHB3 Sleep Clock Register"]
    #[inline(always)]
    pub const fn ahb3lpenr(self) -> crate::common::Reg<regs::Ahb3lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "RCC AHB1 Sleep Clock Register"]
    #[inline(always)]
    pub const fn ahb1lpenr(self) -> crate::common::Reg<regs::Ahb1lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "RCC AHB2 Sleep Clock Register"]
    #[inline(always)]
    pub const fn ahb2lpenr(self) -> crate::common::Reg<regs::Ahb2lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "RCC AHB4 Sleep Clock Register"]
    #[inline(always)]
    pub const fn ahb4lpenr(self) -> crate::common::Reg<regs::Ahb4lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "RCC APB3 Sleep Clock Register"]
    #[inline(always)]
    pub const fn apb3lpenr(self) -> crate::common::Reg<regs::Apb3lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "RCC APB1 Low Sleep Clock Register"]
    #[inline(always)]
    pub const fn apb1llpenr(self) -> crate::common::Reg<regs::Apb1llpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "RCC APB1 High Sleep Clock Register"]
    #[inline(always)]
    pub const fn apb1hlpenr(self) -> crate::common::Reg<regs::Apb1hlpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "RCC APB2 Sleep Clock Register"]
    #[inline(always)]
    pub const fn apb2lpenr(self) -> crate::common::Reg<regs::Apb2lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "RCC APB4 Sleep Clock Register"]
    #[inline(always)]
    pub const fn apb4lpenr(self) -> crate::common::Reg<regs::Apb4lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "RCC Reset Status Register"]
    #[inline(always)]
    pub const fn c1_rsr(self) -> crate::common::Reg<regs::C1Rsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "RCC AHB3 Clock Register"]
    #[inline(always)]
    pub const fn c1_ahb3enr(self) -> crate::common::Reg<regs::C1Ahb3enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "RCC AHB1 Clock Register"]
    #[inline(always)]
    pub const fn c1_ahb1enr(self) -> crate::common::Reg<regs::C1Ahb1enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "RCC AHB2 Clock Register"]
    #[inline(always)]
    pub const fn c1_ahb2enr(self) -> crate::common::Reg<regs::C1Ahb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "RCC AHB4 Clock Register"]
    #[inline(always)]
    pub const fn c1_ahb4enr(self) -> crate::common::Reg<regs::C1Ahb4enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "RCC APB3 Clock Register"]
    #[inline(always)]
    pub const fn c1_apb3enr(self) -> crate::common::Reg<regs::C1Apb3enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "RCC APB1 Clock Register"]
    #[inline(always)]
    pub const fn c1_apb1lenr(self) -> crate::common::Reg<regs::C1Apb1lenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "RCC APB1 Clock Register"]
    #[inline(always)]
    pub const fn c1_apb1henr(self) -> crate::common::Reg<regs::C1Apb1henr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "RCC APB2 Clock Register"]
    #[inline(always)]
    pub const fn c1_apb2enr(self) -> crate::common::Reg<regs::C1Apb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "RCC APB4 Clock Register"]
    #[inline(always)]
    pub const fn c1_apb4enr(self) -> crate::common::Reg<regs::C1Apb4enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "RCC AHB3 Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_ahb3lpenr(self) -> crate::common::Reg<regs::C1Ahb3lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "RCC AHB1 Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_ahb1lpenr(self) -> crate::common::Reg<regs::C1Ahb1lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "RCC AHB2 Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_ahb2lpenr(self) -> crate::common::Reg<regs::C1Ahb2lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "RCC AHB4 Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_ahb4lpenr(self) -> crate::common::Reg<regs::C1Ahb4lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "RCC APB3 Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_apb3lpenr(self) -> crate::common::Reg<regs::C1Apb3lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "RCC APB1 Low Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_apb1llpenr(self) -> crate::common::Reg<regs::C1Apb1llpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "RCC APB1 High Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_apb1hlpenr(self) -> crate::common::Reg<regs::C1Apb1hlpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "RCC APB2 Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_apb2lpenr(self) -> crate::common::Reg<regs::C1Apb2lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "RCC APB4 Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_apb4lpenr(self) -> crate::common::Reg<regs::C1Apb4lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
}
pub mod regs {
    #[doc = "RCC AHB1 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1enr(pub u32);
    impl Ahb1enr {
        #[doc = "DMA1 Clock Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn dma1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 Clock Enable"]
        #[inline(always)]
        pub const fn set_dma1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2 Clock Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn dma2en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 Clock Enable"]
        #[inline(always)]
        pub const fn set_dma2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ADC1/2 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn adc12en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1/2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_adc12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ART Clock Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn arten(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "ART Clock Enable"]
        #[inline(always)]
        pub const fn set_arten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Ethernet MAC bus interface Clock Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ethen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet MAC bus interface Clock Enable"]
        #[inline(always)]
        pub const fn set_ethen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Ethernet Transmission Clock Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ethtxen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet Transmission Clock Enable"]
        #[inline(always)]
        pub const fn set_ethtxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Ethernet Reception Clock Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ethrxen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet Reception Clock Enable"]
        #[inline(always)]
        pub const fn set_ethrxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USB_OTG_HS Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn usb_otg_hsen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "USB_OTG_HS Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_usb_otg_hsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "USB_OTG_HS ULPI clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn usb_otg_hs_ulpien(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "USB_OTG_HS ULPI clock enable"]
        #[inline(always)]
        pub const fn set_usb_otg_hs_ulpien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "USB_OTG_FS Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn usb_otg_fsen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "USB_OTG_FS Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_usb_otg_fsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "USB_OTG_FS ULPI clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn usb_otg_fs_ulpien(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "USB_OTG_FS ULPI clock enable"]
        #[inline(always)]
        pub const fn set_usb_otg_fs_ulpien(&mut self, val: bool) {
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
            defmt :: write ! (f , "Ahb1enr {{ dma1en: {=bool:?}, dma2en: {=bool:?}, adc12en: {=bool:?}, arten: {=bool:?}, ethen: {=bool:?}, ethtxen: {=bool:?}, ethrxen: {=bool:?}, usb_otg_hsen: {=bool:?}, usb_otg_hs_ulpien: {=bool:?}, usb_otg_fsen: {=bool:?}, usb_otg_fs_ulpien: {=bool:?} }}" , self . dma1en () , self . dma2en () , self . adc12en () , self . arten () , self . ethen () , self . ethtxen () , self . ethrxen () , self . usb_otg_hsen () , self . usb_otg_hs_ulpien () , self . usb_otg_fsen () , self . usb_otg_fs_ulpien ())
        }
    }
    #[doc = "RCC AHB1 Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1lpenr(pub u32);
    impl Ahb1lpenr {
        #[doc = "DMA1 Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn dma1lpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_dma1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2 Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn dma2lpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_dma2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ADC1/2 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn adc12lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1/2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_adc12lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ART Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn artlpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "ART Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_artlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Ethernet MAC bus interface Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn ethlpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet MAC bus interface Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_ethlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Ethernet Transmission Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn ethtxlpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet Transmission Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_ethtxlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Ethernet Reception Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn ethrxlpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet Reception Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_ethrxlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USB_OTG_HS peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn usb_otg_hslpen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "USB_OTG_HS peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_usb_otg_hslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "USB_PHY1 clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn usb_otg_hs_ulpilpen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "USB_PHY1 clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_usb_otg_hs_ulpilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "USB_OTG_FS peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn usb_otg_fslpen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "USB_OTG_FS peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_usb_otg_fslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "USB_PHY2 clocks enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn usb_otg_fs_ulpilpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "USB_PHY2 clocks enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_usb_otg_fs_ulpilpen(&mut self, val: bool) {
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
            defmt :: write ! (f , "Ahb1lpenr {{ dma1lpen: {=bool:?}, dma2lpen: {=bool:?}, adc12lpen: {=bool:?}, artlpen: {=bool:?}, ethlpen: {=bool:?}, ethtxlpen: {=bool:?}, ethrxlpen: {=bool:?}, usb_otg_hslpen: {=bool:?}, usb_otg_hs_ulpilpen: {=bool:?}, usb_otg_fslpen: {=bool:?}, usb_otg_fs_ulpilpen: {=bool:?} }}" , self . dma1lpen () , self . dma2lpen () , self . adc12lpen () , self . artlpen () , self . ethlpen () , self . ethtxlpen () , self . ethrxlpen () , self . usb_otg_hslpen () , self . usb_otg_hs_ulpilpen () , self . usb_otg_fslpen () , self . usb_otg_fs_ulpilpen ())
        }
    }
    #[doc = "RCC AHB1 Peripheral Reset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1rstr(pub u32);
    impl Ahb1rstr {
        #[doc = "DMA1 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn dma1rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 block reset"]
        #[inline(always)]
        pub const fn set_dma1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn dma2rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 block reset"]
        #[inline(always)]
        pub const fn set_dma2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ADC1&2 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn adc12rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1&2 block reset"]
        #[inline(always)]
        pub const fn set_adc12rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ART block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn artrst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "ART block reset"]
        #[inline(always)]
        pub const fn set_artrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "ETH block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn ethrst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "ETH block reset"]
        #[inline(always)]
        pub const fn set_ethrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "USB_OTG_HS block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn usb_otg_hsrst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "USB_OTG_HS block reset"]
        #[inline(always)]
        pub const fn set_usb_otg_hsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "USB_OTG_FS block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn usb_otg_fsrst(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "USB_OTG_FS block reset"]
        #[inline(always)]
        pub const fn set_usb_otg_fsrst(&mut self, val: bool) {
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
            defmt :: write ! (f , "Ahb1rstr {{ dma1rst: {=bool:?}, dma2rst: {=bool:?}, adc12rst: {=bool:?}, artrst: {=bool:?}, ethrst: {=bool:?}, usb_otg_hsrst: {=bool:?}, usb_otg_fsrst: {=bool:?} }}" , self . dma1rst () , self . dma2rst () , self . adc12rst () , self . artrst () , self . ethrst () , self . usb_otg_hsrst () , self . usb_otg_fsrst ())
        }
    }
    #[doc = "RCC AHB2 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2enr(pub u32);
    impl Ahb2enr {
        #[doc = "DCMI peripheral clock"]
        #[must_use]
        #[inline(always)]
        pub const fn dcmien(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DCMI peripheral clock"]
        #[inline(always)]
        pub const fn set_dcmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CRYP peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn crypen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CRYP peripheral clock enable"]
        #[inline(always)]
        pub const fn set_crypen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "HASH peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn hashen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HASH peripheral clock enable"]
        #[inline(always)]
        pub const fn set_hashen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "RNG peripheral clocks enable"]
        #[must_use]
        #[inline(always)]
        pub const fn rngen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RNG peripheral clocks enable"]
        #[inline(always)]
        pub const fn set_rngen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "SDMMC2 and SDMMC2 delay clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn sdmmc2en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC2 and SDMMC2 delay clock enable"]
        #[inline(always)]
        pub const fn set_sdmmc2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "FMAC enable"]
        #[must_use]
        #[inline(always)]
        pub const fn fmacen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "FMAC enable"]
        #[inline(always)]
        pub const fn set_fmacen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "CORDIC enable"]
        #[must_use]
        #[inline(always)]
        pub const fn cordicen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "CORDIC enable"]
        #[inline(always)]
        pub const fn set_cordicen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "SRAM1 block enable"]
        #[must_use]
        #[inline(always)]
        pub const fn sram1en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 block enable"]
        #[inline(always)]
        pub const fn set_sram1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "SRAM2 block enable"]
        #[must_use]
        #[inline(always)]
        pub const fn sram2en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 block enable"]
        #[inline(always)]
        pub const fn set_sram2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SRAM3 block enable"]
        #[must_use]
        #[inline(always)]
        pub const fn sram3en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM3 block enable"]
        #[inline(always)]
        pub const fn set_sram3en(&mut self, val: bool) {
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
            defmt :: write ! (f , "Ahb2enr {{ dcmien: {=bool:?}, crypen: {=bool:?}, hashen: {=bool:?}, rngen: {=bool:?}, sdmmc2en: {=bool:?}, fmacen: {=bool:?}, cordicen: {=bool:?}, sram1en: {=bool:?}, sram2en: {=bool:?}, sram3en: {=bool:?} }}" , self . dcmien () , self . crypen () , self . hashen () , self . rngen () , self . sdmmc2en () , self . fmacen () , self . cordicen () , self . sram1en () , self . sram2en () , self . sram3en ())
        }
    }
    #[doc = "RCC AHB2 Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2lpenr(pub u32);
    impl Ahb2lpenr {
        #[doc = "DCMI peripheral clock enable during csleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn dcmilpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DCMI peripheral clock enable during csleep mode"]
        #[inline(always)]
        pub const fn set_dcmilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CRYP peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn cryplpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CRYP peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_cryplpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "HASH peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn hashlpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HASH peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_hashlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "RNG peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn rnglpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RNG peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_rnglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn sdmmc2lpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_sdmmc2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "FMAC enable during CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn fmaclpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "FMAC enable during CSleep Mode"]
        #[inline(always)]
        pub const fn set_fmaclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "CORDIC enable during CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn cordiclpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "CORDIC enable during CSleep Mode"]
        #[inline(always)]
        pub const fn set_cordiclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "SRAM1 Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn sram1lpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_sram1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "SRAM2 Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn sram2lpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_sram2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SRAM3 Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn sram3lpen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM3 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_sram3lpen(&mut self, val: bool) {
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
            defmt :: write ! (f , "Ahb2lpenr {{ dcmilpen: {=bool:?}, cryplpen: {=bool:?}, hashlpen: {=bool:?}, rnglpen: {=bool:?}, sdmmc2lpen: {=bool:?}, fmaclpen: {=bool:?}, cordiclpen: {=bool:?}, sram1lpen: {=bool:?}, sram2lpen: {=bool:?}, sram3lpen: {=bool:?} }}" , self . dcmilpen () , self . cryplpen () , self . hashlpen () , self . rnglpen () , self . sdmmc2lpen () , self . fmaclpen () , self . cordiclpen () , self . sram1lpen () , self . sram2lpen () , self . sram3lpen ())
        }
    }
    #[doc = "RCC AHB2 Peripheral Reset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2rstr(pub u32);
    impl Ahb2rstr {
        #[doc = "DCMI block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn dcmirst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DCMI block reset"]
        #[inline(always)]
        pub const fn set_dcmirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CRYPography block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn cryprst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CRYPography block reset"]
        #[inline(always)]
        pub const fn set_cryprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Hash block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn hashrst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Hash block reset"]
        #[inline(always)]
        pub const fn set_hashrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Random Number Generator block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn rngrst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Random Number Generator block reset"]
        #[inline(always)]
        pub const fn set_rngrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "SDMMC2 and SDMMC2 Delay block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn sdmmc2rst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC2 and SDMMC2 Delay block reset"]
        #[inline(always)]
        pub const fn set_sdmmc2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "FMAC reset"]
        #[must_use]
        #[inline(always)]
        pub const fn fmacrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "FMAC reset"]
        #[inline(always)]
        pub const fn set_fmacrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "CORDIC reset"]
        #[must_use]
        #[inline(always)]
        pub const fn cordicrst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "CORDIC reset"]
        #[inline(always)]
        pub const fn set_cordicrst(&mut self, val: bool) {
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
            defmt :: write ! (f , "Ahb2rstr {{ dcmirst: {=bool:?}, cryprst: {=bool:?}, hashrst: {=bool:?}, rngrst: {=bool:?}, sdmmc2rst: {=bool:?}, fmacrst: {=bool:?}, cordicrst: {=bool:?} }}" , self . dcmirst () , self . cryprst () , self . hashrst () , self . rngrst () , self . sdmmc2rst () , self . fmacrst () , self . cordicrst ())
        }
    }
    #[doc = "RCC AHB3 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3enr(pub u32);
    impl Ahb3enr {
        #[doc = "MDMA Peripheral Clock Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn mdmaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MDMA Peripheral Clock Enable"]
        #[inline(always)]
        pub const fn set_mdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2D Peripheral Clock Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn dma2den(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2D Peripheral Clock Enable"]
        #[inline(always)]
        pub const fn set_dma2den(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "JPGDEC Peripheral Clock Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn jpgdecen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "JPGDEC Peripheral Clock Enable"]
        #[inline(always)]
        pub const fn set_jpgdecen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FMC Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn fmcen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "FMC Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_fmcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "OCTOSPI2 and OCTOSPI2 delay block enable"]
        #[must_use]
        #[inline(always)]
        pub const fn octospi1en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI2 and OCTOSPI2 delay block enable"]
        #[inline(always)]
        pub const fn set_octospi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "QUADSPI and QUADSPI Delay Clock Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn quadspien(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "QUADSPI and QUADSPI Delay Clock Enable"]
        #[inline(always)]
        pub const fn set_quadspien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SDMMC1 and SDMMC1 Delay Clock Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn sdmmc1en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC1 and SDMMC1 Delay Clock Enable"]
        #[inline(always)]
        pub const fn set_sdmmc1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "OCTOSPI2 and OCTOSPI2 delay block enable"]
        #[must_use]
        #[inline(always)]
        pub const fn octospi2en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI2 and OCTOSPI2 delay block enable"]
        #[inline(always)]
        pub const fn set_octospi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "OCTOSPI IO manager enable"]
        #[must_use]
        #[inline(always)]
        pub const fn iomngren(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI IO manager enable"]
        #[inline(always)]
        pub const fn set_iomngren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "OTFDEC1 enable"]
        #[must_use]
        #[inline(always)]
        pub const fn otfd1en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC1 enable"]
        #[inline(always)]
        pub const fn set_otfd1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "OTFDEC2 enable"]
        #[must_use]
        #[inline(always)]
        pub const fn otfd2en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC2 enable"]
        #[inline(always)]
        pub const fn set_otfd2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "D1 DTCM1 block enable"]
        #[must_use]
        #[inline(always)]
        pub const fn dtcm1en(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "D1 DTCM1 block enable"]
        #[inline(always)]
        pub const fn set_dtcm1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "D1 DTCM2 block enable"]
        #[must_use]
        #[inline(always)]
        pub const fn dtcm2en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "D1 DTCM2 block enable"]
        #[inline(always)]
        pub const fn set_dtcm2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "D1 ITCM block enable"]
        #[must_use]
        #[inline(always)]
        pub const fn itcm1en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "D1 ITCM block enable"]
        #[inline(always)]
        pub const fn set_itcm1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "AXISRAM block enable"]
        #[must_use]
        #[inline(always)]
        pub const fn axisramen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "AXISRAM block enable"]
        #[inline(always)]
        pub const fn set_axisramen(&mut self, val: bool) {
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
                .field("octospi1en", &self.octospi1en())
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
            defmt :: write ! (f , "Ahb3enr {{ mdmaen: {=bool:?}, dma2den: {=bool:?}, jpgdecen: {=bool:?}, fmcen: {=bool:?}, octospi1en: {=bool:?}, quadspien: {=bool:?}, sdmmc1en: {=bool:?}, octospi2en: {=bool:?}, iomngren: {=bool:?}, otfd1en: {=bool:?}, otfd2en: {=bool:?}, dtcm1en: {=bool:?}, dtcm2en: {=bool:?}, itcm1en: {=bool:?}, axisramen: {=bool:?} }}" , self . mdmaen () , self . dma2den () , self . jpgdecen () , self . fmcen () , self . octospi1en () , self . quadspien () , self . sdmmc1en () , self . octospi2en () , self . iomngren () , self . otfd1en () , self . otfd2en () , self . dtcm1en () , self . dtcm2en () , self . itcm1en () , self . axisramen ())
        }
    }
    #[doc = "RCC AHB3 Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3lpenr(pub u32);
    impl Ahb3lpenr {
        #[doc = "MDMA Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn mdmalpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MDMA Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_mdmalpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2D Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn dma2dlpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2D Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_dma2dlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "JPGDEC Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn jpgdeclpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "JPGDEC Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_jpgdeclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FLASH Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn flashlpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FLASH Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_flashlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "FMC Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn fmclpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "FMC Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_fmclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "OCTOSPI1 and OCTOSPI1 delay block enable during CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn octospi1lpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI1 and OCTOSPI1 delay block enable during CSleep Mode"]
        #[inline(always)]
        pub const fn set_octospi1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "QUADSPI and QUADSPI Delay Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn quadspilpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "QUADSPI and QUADSPI Delay Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_quadspilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn sdmmc1lpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_sdmmc1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "OCTOSPI2 and OCTOSPI2 delay block enable during CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn octospi2lpen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI2 and OCTOSPI2 delay block enable during CSleep Mode"]
        #[inline(always)]
        pub const fn set_octospi2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "OCTOSPI IO manager enable during CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn iomngrlpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI IO manager enable during CSleep Mode"]
        #[inline(always)]
        pub const fn set_iomngrlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "OTFDEC1 enable during CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn otfd1lpen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC1 enable during CSleep Mode"]
        #[inline(always)]
        pub const fn set_otfd1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "OTFDEC2 enable during CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn otfd2lpen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC2 enable during CSleep Mode"]
        #[inline(always)]
        pub const fn set_otfd2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "D1DTCM1 Block Clock Enable During CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn d1dtcm1lpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "D1DTCM1 Block Clock Enable During CSleep mode"]
        #[inline(always)]
        pub const fn set_d1dtcm1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "D1 DTCM2 Block Clock Enable During CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn dtcm2lpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "D1 DTCM2 Block Clock Enable During CSleep mode"]
        #[inline(always)]
        pub const fn set_dtcm2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "D1ITCM Block Clock Enable During CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn itcmlpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "D1ITCM Block Clock Enable During CSleep mode"]
        #[inline(always)]
        pub const fn set_itcmlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "AXISRAM Block Clock Enable During CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn axisramlpen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "AXISRAM Block Clock Enable During CSleep mode"]
        #[inline(always)]
        pub const fn set_axisramlpen(&mut self, val: bool) {
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
                .field("octospi1lpen", &self.octospi1lpen())
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
            defmt :: write ! (f , "Ahb3lpenr {{ mdmalpen: {=bool:?}, dma2dlpen: {=bool:?}, jpgdeclpen: {=bool:?}, flashlpen: {=bool:?}, fmclpen: {=bool:?}, octospi1lpen: {=bool:?}, quadspilpen: {=bool:?}, sdmmc1lpen: {=bool:?}, octospi2lpen: {=bool:?}, iomngrlpen: {=bool:?}, otfd1lpen: {=bool:?}, otfd2lpen: {=bool:?}, d1dtcm1lpen: {=bool:?}, dtcm2lpen: {=bool:?}, itcmlpen: {=bool:?}, axisramlpen: {=bool:?} }}" , self . mdmalpen () , self . dma2dlpen () , self . jpgdeclpen () , self . flashlpen () , self . fmclpen () , self . octospi1lpen () , self . quadspilpen () , self . sdmmc1lpen () , self . octospi2lpen () , self . iomngrlpen () , self . otfd1lpen () , self . otfd2lpen () , self . d1dtcm1lpen () , self . dtcm2lpen () , self . itcmlpen () , self . axisramlpen ())
        }
    }
    #[doc = "RCC AHB3 Reset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3rstr(pub u32);
    impl Ahb3rstr {
        #[doc = "MDMA block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn mdmarst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MDMA block reset"]
        #[inline(always)]
        pub const fn set_mdmarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2D block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn dma2drst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2D block reset"]
        #[inline(always)]
        pub const fn set_dma2drst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "JPGDEC block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn jpgdecrst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "JPGDEC block reset"]
        #[inline(always)]
        pub const fn set_jpgdecrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FMC block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn fmcrst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "FMC block reset"]
        #[inline(always)]
        pub const fn set_fmcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "OCTOSPI1 and OCTOSPI1 delay block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn octospi1rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI1 and OCTOSPI1 delay block reset"]
        #[inline(always)]
        pub const fn set_octospi1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "QUADSPI and QUADSPI delay block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn quadspirst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "QUADSPI and QUADSPI delay block reset"]
        #[inline(always)]
        pub const fn set_quadspirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SDMMC1 and SDMMC1 delay block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn sdmmc1rst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC1 and SDMMC1 delay block reset"]
        #[inline(always)]
        pub const fn set_sdmmc1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "OCTOSPI2 and OCTOSPI2 delay block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn octospi2rst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI2 and OCTOSPI2 delay block reset"]
        #[inline(always)]
        pub const fn set_octospi2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "OCTOSPI IO manager reset"]
        #[must_use]
        #[inline(always)]
        pub const fn iomngrrst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI IO manager reset"]
        #[inline(always)]
        pub const fn set_iomngrrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "OTFDEC1 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn otfd1rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC1 reset"]
        #[inline(always)]
        pub const fn set_otfd1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "OTFDEC2 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn otfd2rst(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC2 reset"]
        #[inline(always)]
        pub const fn set_otfd2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "CPU reset"]
        #[must_use]
        #[inline(always)]
        pub const fn cpurst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "CPU reset"]
        #[inline(always)]
        pub const fn set_cpurst(&mut self, val: bool) {
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
                .field("octospi1rst", &self.octospi1rst())
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
            defmt :: write ! (f , "Ahb3rstr {{ mdmarst: {=bool:?}, dma2drst: {=bool:?}, jpgdecrst: {=bool:?}, fmcrst: {=bool:?}, octospi1rst: {=bool:?}, quadspirst: {=bool:?}, sdmmc1rst: {=bool:?}, octospi2rst: {=bool:?}, iomngrrst: {=bool:?}, otfd1rst: {=bool:?}, otfd2rst: {=bool:?}, cpurst: {=bool:?} }}" , self . mdmarst () , self . dma2drst () , self . jpgdecrst () , self . fmcrst () , self . octospi1rst () , self . quadspirst () , self . sdmmc1rst () , self . octospi2rst () , self . iomngrrst () , self . otfd1rst () , self . otfd2rst () , self . cpurst ())
        }
    }
    #[doc = "RCC AHB4 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb4enr(pub u32);
    impl Ahb4enr {
        #[doc = "0GPIO peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn set_gpioaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioben(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn set_gpioben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiocen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn set_gpiocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioden(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn set_gpioden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioeen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn set_gpioeen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiofen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn set_gpiofen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiogen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn set_gpiogen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiohen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn set_gpiohen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioien(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn set_gpioien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiojen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn set_gpiojen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioken(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn set_gpioken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "CRC peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn crcen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "CRC peripheral clock enable"]
        #[inline(always)]
        pub const fn set_crcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "BDMA and DMAMUX2 Clock Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn bdmaen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "BDMA and DMAMUX2 Clock Enable"]
        #[inline(always)]
        pub const fn set_bdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ADC3 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn adc3en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ADC3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_adc3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "HSEM peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn hsemen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "HSEM peripheral clock enable"]
        #[inline(always)]
        pub const fn set_hsemen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Backup RAM Clock Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn bkpsramen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Backup RAM Clock Enable"]
        #[inline(always)]
        pub const fn set_bkpsramen(&mut self, val: bool) {
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
            defmt :: write ! (f , "Ahb4enr {{ gpioaen: {=bool:?}, gpioben: {=bool:?}, gpiocen: {=bool:?}, gpioden: {=bool:?}, gpioeen: {=bool:?}, gpiofen: {=bool:?}, gpiogen: {=bool:?}, gpiohen: {=bool:?}, gpioien: {=bool:?}, gpiojen: {=bool:?}, gpioken: {=bool:?}, crcen: {=bool:?}, bdmaen: {=bool:?}, adc3en: {=bool:?}, hsemen: {=bool:?}, bkpsramen: {=bool:?} }}" , self . gpioaen () , self . gpioben () , self . gpiocen () , self . gpioden () , self . gpioeen () , self . gpiofen () , self . gpiogen () , self . gpiohen () , self . gpioien () , self . gpiojen () , self . gpioken () , self . crcen () , self . bdmaen () , self . adc3en () , self . hsemen () , self . bkpsramen ())
        }
    }
    #[doc = "RCC AHB4 Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb4lpenr(pub u32);
    impl Ahb4lpenr {
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioalpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_gpioalpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioblpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_gpioblpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioclpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_gpioclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiodlpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_gpiodlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioelpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_gpioelpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioflpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_gpioflpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioglpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_gpioglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiohlpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_gpiohlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioilpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_gpioilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiojlpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_gpiojlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioklpen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_gpioklpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "CRC peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn crclpen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "CRC peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_crclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "BDMA Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn bdmalpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "BDMA Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_bdmalpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ADC3 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn adc3lpen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ADC3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_adc3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Backup RAM Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn bkpsramlpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Backup RAM Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_bkpsramlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "SRAM4 Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn sram4lpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM4 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_sram4lpen(&mut self, val: bool) {
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
            defmt :: write ! (f , "Ahb4lpenr {{ gpioalpen: {=bool:?}, gpioblpen: {=bool:?}, gpioclpen: {=bool:?}, gpiodlpen: {=bool:?}, gpioelpen: {=bool:?}, gpioflpen: {=bool:?}, gpioglpen: {=bool:?}, gpiohlpen: {=bool:?}, gpioilpen: {=bool:?}, gpiojlpen: {=bool:?}, gpioklpen: {=bool:?}, crclpen: {=bool:?}, bdmalpen: {=bool:?}, adc3lpen: {=bool:?}, bkpsramlpen: {=bool:?}, sram4lpen: {=bool:?} }}" , self . gpioalpen () , self . gpioblpen () , self . gpioclpen () , self . gpiodlpen () , self . gpioelpen () , self . gpioflpen () , self . gpioglpen () , self . gpiohlpen () , self . gpioilpen () , self . gpiojlpen () , self . gpioklpen () , self . crclpen () , self . bdmalpen () , self . adc3lpen () , self . bkpsramlpen () , self . sram4lpen ())
        }
    }
    #[doc = "RCC AHB4 Peripheral Reset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb4rstr(pub u32);
    impl Ahb4rstr {
        #[doc = "GPIO block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioarst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub const fn set_gpioarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPIO block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiobrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub const fn set_gpiobrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "GPIO block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiocrst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub const fn set_gpiocrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "GPIO block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiodrst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub const fn set_gpiodrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "GPIO block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioerst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub const fn set_gpioerst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "GPIO block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiofrst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub const fn set_gpiofrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "GPIO block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiogrst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub const fn set_gpiogrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "GPIO block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiohrst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub const fn set_gpiohrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "GPIO block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioirst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub const fn set_gpioirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "GPIO block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiojrst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub const fn set_gpiojrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "GPIO block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiokrst(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO block reset"]
        #[inline(always)]
        pub const fn set_gpiokrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "CRC block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn crcrst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "CRC block reset"]
        #[inline(always)]
        pub const fn set_crcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "BDMA block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn bdmarst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "BDMA block reset"]
        #[inline(always)]
        pub const fn set_bdmarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ADC3 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn adc3rst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ADC3 block reset"]
        #[inline(always)]
        pub const fn set_adc3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "HSEM block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn hsemrst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "HSEM block reset"]
        #[inline(always)]
        pub const fn set_hsemrst(&mut self, val: bool) {
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
            defmt :: write ! (f , "Ahb4rstr {{ gpioarst: {=bool:?}, gpiobrst: {=bool:?}, gpiocrst: {=bool:?}, gpiodrst: {=bool:?}, gpioerst: {=bool:?}, gpiofrst: {=bool:?}, gpiogrst: {=bool:?}, gpiohrst: {=bool:?}, gpioirst: {=bool:?}, gpiojrst: {=bool:?}, gpiokrst: {=bool:?}, crcrst: {=bool:?}, bdmarst: {=bool:?}, adc3rst: {=bool:?}, hsemrst: {=bool:?} }}" , self . gpioarst () , self . gpiobrst () , self . gpiocrst () , self . gpiodrst () , self . gpioerst () , self . gpiofrst () , self . gpiogrst () , self . gpiohrst () , self . gpioirst () , self . gpiojrst () , self . gpiokrst () , self . crcrst () , self . bdmarst () , self . adc3rst () , self . hsemrst ())
        }
    }
    #[doc = "RCC APB1 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1henr(pub u32);
    impl Apb1henr {
        #[doc = "Clock Recovery System peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn crsen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clock Recovery System peripheral clock enable"]
        #[inline(always)]
        pub const fn set_crsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SWPMI Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn swpmien(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SWPMI Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_swpmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "OPAMP peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn opampen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP peripheral clock enable"]
        #[inline(always)]
        pub const fn set_opampen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MDIOS peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn mdiosen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MDIOS peripheral clock enable"]
        #[inline(always)]
        pub const fn set_mdiosen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FDCAN Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn fdcanen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_fdcanen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "TIM23 block enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim23en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "TIM23 block enable"]
        #[inline(always)]
        pub const fn set_tim23en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "TIM24 block enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim24en(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "TIM24 block enable"]
        #[inline(always)]
        pub const fn set_tim24en(&mut self, val: bool) {
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
            defmt :: write ! (f , "Apb1henr {{ crsen: {=bool:?}, swpmien: {=bool:?}, opampen: {=bool:?}, mdiosen: {=bool:?}, fdcanen: {=bool:?}, tim23en: {=bool:?}, tim24en: {=bool:?} }}" , self . crsen () , self . swpmien () , self . opampen () , self . mdiosen () , self . fdcanen () , self . tim23en () , self . tim24en ())
        }
    }
    #[doc = "RCC APB1 High Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1hlpenr(pub u32);
    impl Apb1hlpenr {
        #[doc = "Clock Recovery System peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn crslpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clock Recovery System peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_crslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SWPMI Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn swpmilpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SWPMI Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_swpmilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "OPAMP peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn opamplpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_opamplpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MDIOS peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn mdioslpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MDIOS peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_mdioslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FDCAN Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn fdcanlpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_fdcanlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "TIM23 block enable during CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim23lpen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "TIM23 block enable during CSleep Mode"]
        #[inline(always)]
        pub const fn set_tim23lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "TIM24 block enable during CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim24lpen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "TIM24 block enable during CSleep Mode"]
        #[inline(always)]
        pub const fn set_tim24lpen(&mut self, val: bool) {
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
            defmt :: write ! (f , "Apb1hlpenr {{ crslpen: {=bool:?}, swpmilpen: {=bool:?}, opamplpen: {=bool:?}, mdioslpen: {=bool:?}, fdcanlpen: {=bool:?}, tim23lpen: {=bool:?}, tim24lpen: {=bool:?} }}" , self . crslpen () , self . swpmilpen () , self . opamplpen () , self . mdioslpen () , self . fdcanlpen () , self . tim23lpen () , self . tim24lpen ())
        }
    }
    #[doc = "RCC APB1 Peripheral Reset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1hrstr(pub u32);
    impl Apb1hrstr {
        #[doc = "Clock Recovery System reset"]
        #[must_use]
        #[inline(always)]
        pub const fn crsrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clock Recovery System reset"]
        #[inline(always)]
        pub const fn set_crsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SWPMI block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn swpmirst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SWPMI block reset"]
        #[inline(always)]
        pub const fn set_swpmirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "OPAMP block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn opamprst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP block reset"]
        #[inline(always)]
        pub const fn set_opamprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MDIOS block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn mdiosrst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MDIOS block reset"]
        #[inline(always)]
        pub const fn set_mdiosrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FDCAN block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn fdcanrst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN block reset"]
        #[inline(always)]
        pub const fn set_fdcanrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "TIM23 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim23rst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "TIM23 block reset"]
        #[inline(always)]
        pub const fn set_tim23rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "TIM24 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim24rst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "TIM24 block reset"]
        #[inline(always)]
        pub const fn set_tim24rst(&mut self, val: bool) {
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
            defmt :: write ! (f , "Apb1hrstr {{ crsrst: {=bool:?}, swpmirst: {=bool:?}, opamprst: {=bool:?}, mdiosrst: {=bool:?}, fdcanrst: {=bool:?}, tim23rst: {=bool:?}, tim24rst: {=bool:?} }}" , self . crsrst () , self . swpmirst () , self . opamprst () , self . mdiosrst () , self . fdcanrst () , self . tim23rst () , self . tim24rst ())
        }
    }
    #[doc = "RCC APB1 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1lenr(pub u32);
    impl Apb1lenr {
        #[doc = "TIM peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim2en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn set_tim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim3en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn set_tim3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim4en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn set_tim4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim5en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn set_tim5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim6en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn set_tim6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim7en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn set_tim7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim12en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn set_tim12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim13en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn set_tim13en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim14en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn set_tim14en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LPTIM1 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim1en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_lptim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "WWDG2 peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn wwdg2en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG2 peripheral clock enable"]
        #[inline(always)]
        pub const fn set_wwdg2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI2 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn spi2en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_spi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn spi3en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_spi3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "SPDIFRX Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn spdifrxen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SPDIFRX Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_spdifrxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "USART2 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn usart2en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_usart2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn usart3en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_usart3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn uart4en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_uart4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn uart5en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_uart5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_i2c1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c2en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_i2c2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I2C3 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c3en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_i2c3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "I2C5 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c5en(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "I2C5 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_i2c5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "HDMI-CEC peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn cecen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "HDMI-CEC peripheral clock enable"]
        #[inline(always)]
        pub const fn set_cecen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "DAC1&2 peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn dac12en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1&2 peripheral clock enable"]
        #[inline(always)]
        pub const fn set_dac12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "UART7 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn uart7en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "UART7 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_uart7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "UART8 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn uart8en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "UART8 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_uart8en(&mut self, val: bool) {
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
            defmt :: write ! (f , "Apb1lenr {{ tim2en: {=bool:?}, tim3en: {=bool:?}, tim4en: {=bool:?}, tim5en: {=bool:?}, tim6en: {=bool:?}, tim7en: {=bool:?}, tim12en: {=bool:?}, tim13en: {=bool:?}, tim14en: {=bool:?}, lptim1en: {=bool:?}, wwdg2en: {=bool:?}, spi2en: {=bool:?}, spi3en: {=bool:?}, spdifrxen: {=bool:?}, usart2en: {=bool:?}, usart3en: {=bool:?}, uart4en: {=bool:?}, uart5en: {=bool:?}, i2c1en: {=bool:?}, i2c2en: {=bool:?}, i2c3en: {=bool:?}, i2c5en: {=bool:?}, cecen: {=bool:?}, dac12en: {=bool:?}, uart7en: {=bool:?}, uart8en: {=bool:?} }}" , self . tim2en () , self . tim3en () , self . tim4en () , self . tim5en () , self . tim6en () , self . tim7en () , self . tim12en () , self . tim13en () , self . tim14en () , self . lptim1en () , self . wwdg2en () , self . spi2en () , self . spi3en () , self . spdifrxen () , self . usart2en () , self . usart3en () , self . uart4en () , self . uart5en () , self . i2c1en () , self . i2c2en () , self . i2c3en () , self . i2c5en () , self . cecen () , self . dac12en () , self . uart7en () , self . uart8en ())
        }
    }
    #[doc = "RCC APB1 Low Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1llpenr(pub u32);
    impl Apb1llpenr {
        #[doc = "TIM2 peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim2lpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_tim2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim3lpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_tim3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim4lpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_tim4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5 peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim5lpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_tim5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim6lpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_tim6lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim7lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_tim7lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM12 peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim12lpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_tim12lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIM13 peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim13lpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIM13 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_tim13lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIM14 peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim14lpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM14 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_tim14lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LPTIM1 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim1lpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_lptim1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "WWDG2 peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn wwdg2lpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG2 peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_wwdg2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI2 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn spi2lpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_spi2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn spi3lpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_spi3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "SPDIFRX Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn spdifrxlpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SPDIFRX Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_spdifrxlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "USART2 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn usart2lpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_usart2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn usart3lpen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_usart3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn uart4lpen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_uart4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn uart5lpen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_uart5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1lpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_i2c1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c2lpen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_i2c2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I2C3 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c3lpen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_i2c3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "I2C5 block enable during CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c5lpen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "I2C5 block enable during CSleep Mode"]
        #[inline(always)]
        pub const fn set_i2c5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "HDMI-CEC Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn ceclpen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "HDMI-CEC Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_ceclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "DAC1/2 peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn dac12lpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1/2 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_dac12lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "UART7 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn uart7lpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "UART7 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_uart7lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "UART8 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn uart8lpen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "UART8 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_uart8lpen(&mut self, val: bool) {
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
            defmt :: write ! (f , "Apb1llpenr {{ tim2lpen: {=bool:?}, tim3lpen: {=bool:?}, tim4lpen: {=bool:?}, tim5lpen: {=bool:?}, tim6lpen: {=bool:?}, tim7lpen: {=bool:?}, tim12lpen: {=bool:?}, tim13lpen: {=bool:?}, tim14lpen: {=bool:?}, lptim1lpen: {=bool:?}, wwdg2lpen: {=bool:?}, spi2lpen: {=bool:?}, spi3lpen: {=bool:?}, spdifrxlpen: {=bool:?}, usart2lpen: {=bool:?}, usart3lpen: {=bool:?}, uart4lpen: {=bool:?}, uart5lpen: {=bool:?}, i2c1lpen: {=bool:?}, i2c2lpen: {=bool:?}, i2c3lpen: {=bool:?}, i2c5lpen: {=bool:?}, ceclpen: {=bool:?}, dac12lpen: {=bool:?}, uart7lpen: {=bool:?}, uart8lpen: {=bool:?} }}" , self . tim2lpen () , self . tim3lpen () , self . tim4lpen () , self . tim5lpen () , self . tim6lpen () , self . tim7lpen () , self . tim12lpen () , self . tim13lpen () , self . tim14lpen () , self . lptim1lpen () , self . wwdg2lpen () , self . spi2lpen () , self . spi3lpen () , self . spdifrxlpen () , self . usart2lpen () , self . usart3lpen () , self . uart4lpen () , self . uart5lpen () , self . i2c1lpen () , self . i2c2lpen () , self . i2c3lpen () , self . i2c5lpen () , self . ceclpen () , self . dac12lpen () , self . uart7lpen () , self . uart8lpen ())
        }
    }
    #[doc = "RCC APB1 Peripheral Reset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1lrstr(pub u32);
    impl Apb1lrstr {
        #[doc = "TIM block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim2rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub const fn set_tim2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim3rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub const fn set_tim3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim4rst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub const fn set_tim4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim5rst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub const fn set_tim5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim6rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub const fn set_tim6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim7rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub const fn set_tim7rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim12rst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub const fn set_tim12rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIM block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim13rst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub const fn set_tim13rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIM block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim14rst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub const fn set_tim14rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "TIM block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim1rst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "TIM block reset"]
        #[inline(always)]
        pub const fn set_lptim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "SPI2 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn spi2rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 block reset"]
        #[inline(always)]
        pub const fn set_spi2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn spi3rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 block reset"]
        #[inline(always)]
        pub const fn set_spi3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "SPDIFRX block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn spdifrxrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SPDIFRX block reset"]
        #[inline(always)]
        pub const fn set_spdifrxrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "USART2 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn usart2rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 block reset"]
        #[inline(always)]
        pub const fn set_usart2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn usart3rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 block reset"]
        #[inline(always)]
        pub const fn set_usart3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn uart4rst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 block reset"]
        #[inline(always)]
        pub const fn set_uart4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn uart5rst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 block reset"]
        #[inline(always)]
        pub const fn set_uart5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 block reset"]
        #[inline(always)]
        pub const fn set_i2c1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c2rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 block reset"]
        #[inline(always)]
        pub const fn set_i2c2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I2C3 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c3rst(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 block reset"]
        #[inline(always)]
        pub const fn set_i2c3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "I2C5 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c5rst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "I2C5 block reset"]
        #[inline(always)]
        pub const fn set_i2c5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "HDMI-CEC block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn cecrst(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "HDMI-CEC block reset"]
        #[inline(always)]
        pub const fn set_cecrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "DAC1 and 2 Blocks Reset"]
        #[must_use]
        #[inline(always)]
        pub const fn dac12rst(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1 and 2 Blocks Reset"]
        #[inline(always)]
        pub const fn set_dac12rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "UART7 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn uart7rst(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "UART7 block reset"]
        #[inline(always)]
        pub const fn set_uart7rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "UART8 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn uart8rst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "UART8 block reset"]
        #[inline(always)]
        pub const fn set_uart8rst(&mut self, val: bool) {
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
            defmt :: write ! (f , "Apb1lrstr {{ tim2rst: {=bool:?}, tim3rst: {=bool:?}, tim4rst: {=bool:?}, tim5rst: {=bool:?}, tim6rst: {=bool:?}, tim7rst: {=bool:?}, tim12rst: {=bool:?}, tim13rst: {=bool:?}, tim14rst: {=bool:?}, lptim1rst: {=bool:?}, spi2rst: {=bool:?}, spi3rst: {=bool:?}, spdifrxrst: {=bool:?}, usart2rst: {=bool:?}, usart3rst: {=bool:?}, uart4rst: {=bool:?}, uart5rst: {=bool:?}, i2c1rst: {=bool:?}, i2c2rst: {=bool:?}, i2c3rst: {=bool:?}, i2c5rst: {=bool:?}, cecrst: {=bool:?}, dac12rst: {=bool:?}, uart7rst: {=bool:?}, uart8rst: {=bool:?} }}" , self . tim2rst () , self . tim3rst () , self . tim4rst () , self . tim5rst () , self . tim6rst () , self . tim7rst () , self . tim12rst () , self . tim13rst () , self . tim14rst () , self . lptim1rst () , self . spi2rst () , self . spi3rst () , self . spdifrxrst () , self . usart2rst () , self . usart3rst () , self . uart4rst () , self . uart5rst () , self . i2c1rst () , self . i2c2rst () , self . i2c3rst () , self . i2c5rst () , self . cecrst () , self . dac12rst () , self . uart7rst () , self . uart8rst ())
        }
    }
    #[doc = "RCC APB2 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2enr(pub u32);
    impl Apb2enr {
        #[doc = "TIM1 peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 peripheral clock enable"]
        #[inline(always)]
        pub const fn set_tim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM8 peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim8en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 peripheral clock enable"]
        #[inline(always)]
        pub const fn set_tim8en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "USART1 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn usart1en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_usart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "USART6 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn usart6en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USART6 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_usart6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "UART9 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn uart9en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "UART9 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_uart9en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "USART10 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn usart10en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "USART10 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_usart10en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "SPI1 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn spi1en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_spi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SPI4 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn spi4en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "SPI4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_spi4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "TIM15 peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim15en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 peripheral clock enable"]
        #[inline(always)]
        pub const fn set_tim15en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim16en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 peripheral clock enable"]
        #[inline(always)]
        pub const fn set_tim16en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim17en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 peripheral clock enable"]
        #[inline(always)]
        pub const fn set_tim17en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SPI5 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn spi5en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SPI5 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_spi5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "SAI1 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn sai1en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_sai1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "SAI2 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn sai2en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "SAI2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_sai2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "SAI3 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn sai3en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "SAI3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_sai3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "DFSDM1 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn dfsdm1en(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "DFSDM1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_dfsdm1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "HRTIM peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn hrtimen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM peripheral clock enable"]
        #[inline(always)]
        pub const fn set_hrtimen(&mut self, val: bool) {
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
            defmt :: write ! (f , "Apb2enr {{ tim1en: {=bool:?}, tim8en: {=bool:?}, usart1en: {=bool:?}, usart6en: {=bool:?}, uart9en: {=bool:?}, usart10en: {=bool:?}, spi1en: {=bool:?}, spi4en: {=bool:?}, tim15en: {=bool:?}, tim16en: {=bool:?}, tim17en: {=bool:?}, spi5en: {=bool:?}, sai1en: {=bool:?}, sai2en: {=bool:?}, sai3en: {=bool:?}, dfsdm1en: {=bool:?}, hrtimen: {=bool:?} }}" , self . tim1en () , self . tim8en () , self . usart1en () , self . usart6en () , self . uart9en () , self . usart10en () , self . spi1en () , self . spi4en () , self . tim15en () , self . tim16en () , self . tim17en () , self . spi5en () , self . sai1en () , self . sai2en () , self . sai3en () , self . dfsdm1en () , self . hrtimen ())
        }
    }
    #[doc = "RCC APB2 Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2lpenr(pub u32);
    impl Apb2lpenr {
        #[doc = "TIM1 peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim1lpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_tim1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM8 peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim8lpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_tim8lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "USART1 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn usart1lpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_usart1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "USART6 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn usart6lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USART6 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_usart6lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SPI1 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn spi1lpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_spi1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SPI4 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn spi4lpen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "SPI4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_spi4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "TIM15 peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim15lpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_tim15lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim16lpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_tim16lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim17lpen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_tim17lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SPI5 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn spi5lpen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SPI5 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_spi5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "SAI1 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn sai1lpen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_sai1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "SAI2 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn sai2lpen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "SAI2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_sai2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "SAI3 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn sai3lpen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "SAI3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_sai3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "DFSDM1 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn dfsdm1lpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "DFSDM1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_dfsdm1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "HRTIM peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn hrtimlpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_hrtimlpen(&mut self, val: bool) {
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
            defmt :: write ! (f , "Apb2lpenr {{ tim1lpen: {=bool:?}, tim8lpen: {=bool:?}, usart1lpen: {=bool:?}, usart6lpen: {=bool:?}, spi1lpen: {=bool:?}, spi4lpen: {=bool:?}, tim15lpen: {=bool:?}, tim16lpen: {=bool:?}, tim17lpen: {=bool:?}, spi5lpen: {=bool:?}, sai1lpen: {=bool:?}, sai2lpen: {=bool:?}, sai3lpen: {=bool:?}, dfsdm1lpen: {=bool:?}, hrtimlpen: {=bool:?} }}" , self . tim1lpen () , self . tim8lpen () , self . usart1lpen () , self . usart6lpen () , self . spi1lpen () , self . spi4lpen () , self . tim15lpen () , self . tim16lpen () , self . tim17lpen () , self . spi5lpen () , self . sai1lpen () , self . sai2lpen () , self . sai3lpen () , self . dfsdm1lpen () , self . hrtimlpen ())
        }
    }
    #[doc = "RCC APB2 Peripheral Reset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2rstr(pub u32);
    impl Apb2rstr {
        #[doc = "TIM1 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim1rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 block reset"]
        #[inline(always)]
        pub const fn set_tim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM8 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim8rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 block reset"]
        #[inline(always)]
        pub const fn set_tim8rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "USART1 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn usart1rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 block reset"]
        #[inline(always)]
        pub const fn set_usart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "USART6 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn usart6rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USART6 block reset"]
        #[inline(always)]
        pub const fn set_usart6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "UART9 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn uart9rst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "UART9 block reset"]
        #[inline(always)]
        pub const fn set_uart9rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "USART10 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn usart10rst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "USART10 block reset"]
        #[inline(always)]
        pub const fn set_usart10rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "SPI1 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn spi1rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 block reset"]
        #[inline(always)]
        pub const fn set_spi1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SPI4 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn spi4rst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "SPI4 block reset"]
        #[inline(always)]
        pub const fn set_spi4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "TIM15 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim15rst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 block reset"]
        #[inline(always)]
        pub const fn set_tim15rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim16rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 block reset"]
        #[inline(always)]
        pub const fn set_tim16rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim17rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 block reset"]
        #[inline(always)]
        pub const fn set_tim17rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SPI5 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn spi5rst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SPI5 block reset"]
        #[inline(always)]
        pub const fn set_spi5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "SAI1 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn sai1rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 block reset"]
        #[inline(always)]
        pub const fn set_sai1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "SAI2 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn sai2rst(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "SAI2 block reset"]
        #[inline(always)]
        pub const fn set_sai2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "SAI3 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn sai3rst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "SAI3 block reset"]
        #[inline(always)]
        pub const fn set_sai3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "DFSDM1 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn dfsdm1rst(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "DFSDM1 block reset"]
        #[inline(always)]
        pub const fn set_dfsdm1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "HRTIM block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn hrtimrst(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM block reset"]
        #[inline(always)]
        pub const fn set_hrtimrst(&mut self, val: bool) {
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
            defmt :: write ! (f , "Apb2rstr {{ tim1rst: {=bool:?}, tim8rst: {=bool:?}, usart1rst: {=bool:?}, usart6rst: {=bool:?}, uart9rst: {=bool:?}, usart10rst: {=bool:?}, spi1rst: {=bool:?}, spi4rst: {=bool:?}, tim15rst: {=bool:?}, tim16rst: {=bool:?}, tim17rst: {=bool:?}, spi5rst: {=bool:?}, sai1rst: {=bool:?}, sai2rst: {=bool:?}, sai3rst: {=bool:?}, dfsdm1rst: {=bool:?}, hrtimrst: {=bool:?} }}" , self . tim1rst () , self . tim8rst () , self . usart1rst () , self . usart6rst () , self . uart9rst () , self . usart10rst () , self . spi1rst () , self . spi4rst () , self . tim15rst () , self . tim16rst () , self . tim17rst () , self . spi5rst () , self . sai1rst () , self . sai2rst () , self . sai3rst () , self . dfsdm1rst () , self . hrtimrst ())
        }
    }
    #[doc = "RCC APB3 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb3enr(pub u32);
    impl Apb3enr {
        #[doc = "LTDC peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ltdcen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC peripheral clock enable"]
        #[inline(always)]
        pub const fn set_ltdcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DSI Peripheral clocks enable"]
        #[must_use]
        #[inline(always)]
        pub const fn dsien(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DSI Peripheral clocks enable"]
        #[inline(always)]
        pub const fn set_dsien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "WWDG1 Clock Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn wwdg1en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG1 Clock Enable"]
        #[inline(always)]
        pub const fn set_wwdg1en(&mut self, val: bool) {
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
            defmt::write!(
                f,
                "Apb3enr {{ ltdcen: {=bool:?}, dsien: {=bool:?}, wwdg1en: {=bool:?} }}",
                self.ltdcen(),
                self.dsien(),
                self.wwdg1en()
            )
        }
    }
    #[doc = "RCC APB3 Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb3lpenr(pub u32);
    impl Apb3lpenr {
        #[doc = "LTDC peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn ltdclpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_ltdclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DSI Peripheral Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn dsilpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DSI Peripheral Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_dsilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "WWDG1 Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn wwdg1lpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG1 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_wwdg1lpen(&mut self, val: bool) {
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
            defmt::write!(
                f,
                "Apb3lpenr {{ ltdclpen: {=bool:?}, dsilpen: {=bool:?}, wwdg1lpen: {=bool:?} }}",
                self.ltdclpen(),
                self.dsilpen(),
                self.wwdg1lpen()
            )
        }
    }
    #[doc = "RCC APB3 Peripheral Reset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb3rstr(pub u32);
    impl Apb3rstr {
        #[doc = "LTDC block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn ltdcrst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC block reset"]
        #[inline(always)]
        pub const fn set_ltdcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DSI block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn dsirst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DSI block reset"]
        #[inline(always)]
        pub const fn set_dsirst(&mut self, val: bool) {
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
            defmt::write!(
                f,
                "Apb3rstr {{ ltdcrst: {=bool:?}, dsirst: {=bool:?} }}",
                self.ltdcrst(),
                self.dsirst()
            )
        }
    }
    #[doc = "RCC APB4 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb4enr(pub u32);
    impl Apb4enr {
        #[doc = "SYSCFG peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn syscfgen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG peripheral clock enable"]
        #[inline(always)]
        pub const fn set_syscfgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPUART1 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lpuart1en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_lpuart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SPI6 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn spi6en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI6 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_spi6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "I2C4 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c4en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_i2c4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LPTIM2 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim2en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_lptim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LPTIM3 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim3en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_lptim3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LPTIM4 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim4en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_lptim4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIM5 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim5en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM5 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_lptim5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "DAC2 (containing one converter) peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn dac2en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "DAC2 (containing one converter) peripheral clock enable"]
        #[inline(always)]
        pub const fn set_dac2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "COMP1/2 peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn comp12en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "COMP1/2 peripheral clock enable"]
        #[inline(always)]
        pub const fn set_comp12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "VREF peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn vrefen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "VREF peripheral clock enable"]
        #[inline(always)]
        pub const fn set_vrefen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "RTC APB Clock Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn rtcapben(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "RTC APB Clock Enable"]
        #[inline(always)]
        pub const fn set_rtcapben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "SAI4 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn sai4en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SAI4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_sai4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Digital temperature sensor block enable"]
        #[must_use]
        #[inline(always)]
        pub const fn dtsen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Digital temperature sensor block enable"]
        #[inline(always)]
        pub const fn set_dtsen(&mut self, val: bool) {
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
            defmt :: write ! (f , "Apb4enr {{ syscfgen: {=bool:?}, lpuart1en: {=bool:?}, spi6en: {=bool:?}, i2c4en: {=bool:?}, lptim2en: {=bool:?}, lptim3en: {=bool:?}, lptim4en: {=bool:?}, lptim5en: {=bool:?}, dac2en: {=bool:?}, comp12en: {=bool:?}, vrefen: {=bool:?}, rtcapben: {=bool:?}, sai4en: {=bool:?}, dtsen: {=bool:?} }}" , self . syscfgen () , self . lpuart1en () , self . spi6en () , self . i2c4en () , self . lptim2en () , self . lptim3en () , self . lptim4en () , self . lptim5en () , self . dac2en () , self . comp12en () , self . vrefen () , self . rtcapben () , self . sai4en () , self . dtsen ())
        }
    }
    #[doc = "RCC APB4 Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb4lpenr(pub u32);
    impl Apb4lpenr {
        #[doc = "SYSCFG peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn syscfglpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_syscfglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPUART1 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn lpuart1lpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_lpuart1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SPI6 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn spi6lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI6 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_spi6lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "I2C4 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c4lpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_i2c4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LPTIM2 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim2lpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_lptim2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LPTIM3 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim3lpen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_lptim3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LPTIM4 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim4lpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_lptim4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIM5 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim5lpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM5 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_lptim5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "DAC2 (containing one converter) peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn dac2lpen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "DAC2 (containing one converter) peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_dac2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "COMP1/2 peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn comp12lpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "COMP1/2 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_comp12lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "VREF peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn vreflpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "VREF peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_vreflpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "RTC APB Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn rtcapblpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "RTC APB Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_rtcapblpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "SAI4 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn sai4lpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SAI4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_sai4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Digital temperature sensor block enable during CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn dtslpen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Digital temperature sensor block enable during CSleep Mode"]
        #[inline(always)]
        pub const fn set_dtslpen(&mut self, val: bool) {
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
            defmt :: write ! (f , "Apb4lpenr {{ syscfglpen: {=bool:?}, lpuart1lpen: {=bool:?}, spi6lpen: {=bool:?}, i2c4lpen: {=bool:?}, lptim2lpen: {=bool:?}, lptim3lpen: {=bool:?}, lptim4lpen: {=bool:?}, lptim5lpen: {=bool:?}, dac2lpen: {=bool:?}, comp12lpen: {=bool:?}, vreflpen: {=bool:?}, rtcapblpen: {=bool:?}, sai4lpen: {=bool:?}, dtslpen: {=bool:?} }}" , self . syscfglpen () , self . lpuart1lpen () , self . spi6lpen () , self . i2c4lpen () , self . lptim2lpen () , self . lptim3lpen () , self . lptim4lpen () , self . lptim5lpen () , self . dac2lpen () , self . comp12lpen () , self . vreflpen () , self . rtcapblpen () , self . sai4lpen () , self . dtslpen ())
        }
    }
    #[doc = "RCC APB4 Peripheral Reset Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb4rstr(pub u32);
    impl Apb4rstr {
        #[doc = "SYSCFG block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn syscfgrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG block reset"]
        #[inline(always)]
        pub const fn set_syscfgrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPUART1 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn lpuart1rst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 block reset"]
        #[inline(always)]
        pub const fn set_lpuart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SPI6 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn spi6rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI6 block reset"]
        #[inline(always)]
        pub const fn set_spi6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "I2C4 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c4rst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 block reset"]
        #[inline(always)]
        pub const fn set_i2c4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LPTIM2 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim2rst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 block reset"]
        #[inline(always)]
        pub const fn set_lptim2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LPTIM3 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim3rst(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3 block reset"]
        #[inline(always)]
        pub const fn set_lptim3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LPTIM4 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim4rst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM4 block reset"]
        #[inline(always)]
        pub const fn set_lptim4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIM5 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim5rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM5 block reset"]
        #[inline(always)]
        pub const fn set_lptim5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "DAC2 (containing one converter) reset"]
        #[must_use]
        #[inline(always)]
        pub const fn dac2rst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "DAC2 (containing one converter) reset"]
        #[inline(always)]
        pub const fn set_dac2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "COMP12 Blocks Reset"]
        #[must_use]
        #[inline(always)]
        pub const fn comp12rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "COMP12 Blocks Reset"]
        #[inline(always)]
        pub const fn set_comp12rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "VREF block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn vrefrst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "VREF block reset"]
        #[inline(always)]
        pub const fn set_vrefrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "SAI4 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn sai4rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SAI4 block reset"]
        #[inline(always)]
        pub const fn set_sai4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Digital temperature sensor block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn dtsrst(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Digital temperature sensor block reset"]
        #[inline(always)]
        pub const fn set_dtsrst(&mut self, val: bool) {
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
            defmt :: write ! (f , "Apb4rstr {{ syscfgrst: {=bool:?}, lpuart1rst: {=bool:?}, spi6rst: {=bool:?}, i2c4rst: {=bool:?}, lptim2rst: {=bool:?}, lptim3rst: {=bool:?}, lptim4rst: {=bool:?}, lptim5rst: {=bool:?}, dac2rst: {=bool:?}, comp12rst: {=bool:?}, vrefrst: {=bool:?}, sai4rst: {=bool:?}, dtsrst: {=bool:?} }}" , self . syscfgrst () , self . lpuart1rst () , self . spi6rst () , self . i2c4rst () , self . lptim2rst () , self . lptim3rst () , self . lptim4rst () , self . lptim5rst () , self . dac2rst () , self . comp12rst () , self . vrefrst () , self . sai4rst () , self . dtsrst ())
        }
    }
    #[doc = "RCC Backup Domain Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdcr(pub u32);
    impl Bdcr {
        #[doc = "LSE oscillator enabled"]
        #[must_use]
        #[inline(always)]
        pub const fn lseon(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSE oscillator enabled"]
        #[inline(always)]
        pub const fn set_lseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE oscillator ready"]
        #[must_use]
        #[inline(always)]
        pub const fn lserdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE oscillator ready"]
        #[inline(always)]
        pub const fn set_lserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LSE oscillator bypass"]
        #[must_use]
        #[inline(always)]
        pub const fn lsebyp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "LSE oscillator bypass"]
        #[inline(always)]
        pub const fn set_lsebyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "LSE oscillator driving capability"]
        #[must_use]
        #[inline(always)]
        pub const fn lsedrv(&self) -> super::vals::Lsedrv {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Lsedrv::from_bits(val as u8)
        }
        #[doc = "LSE oscillator driving capability"]
        #[inline(always)]
        pub const fn set_lsedrv(&mut self, val: super::vals::Lsedrv) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "LSE clock security system enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lsecsson(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock security system enable"]
        #[inline(always)]
        pub const fn set_lsecsson(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LSE clock security system failure detection"]
        #[must_use]
        #[inline(always)]
        pub const fn lsecssd(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock security system failure detection"]
        #[inline(always)]
        pub const fn set_lsecssd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "RTC clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn rtcsel(&self) -> super::vals::Rtcsel {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Rtcsel::from_bits(val as u8)
        }
        #[doc = "RTC clock source selection"]
        #[inline(always)]
        pub const fn set_rtcsel(&mut self, val: super::vals::Rtcsel) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "RTC clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn rtcen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "RTC clock enable"]
        #[inline(always)]
        pub const fn set_rtcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "VSwitch domain software reset"]
        #[must_use]
        #[inline(always)]
        pub const fn bdrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "VSwitch domain software reset"]
        #[inline(always)]
        pub const fn set_bdrst(&mut self, val: bool) {
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
            defmt :: write ! (f , "Bdcr {{ lseon: {=bool:?}, lserdy: {=bool:?}, lsebyp: {=bool:?}, lsedrv: {:?}, lsecsson: {=bool:?}, lsecssd: {=bool:?}, rtcsel: {:?}, rtcen: {=bool:?}, bdrst: {=bool:?} }}" , self . lseon () , self . lserdy () , self . lsebyp () , self . lsedrv () , self . lsecsson () , self . lsecssd () , self . rtcsel () , self . rtcen () , self . bdrst ())
        }
    }
    #[doc = "RCC AHB1 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Ahb1enr(pub u32);
    impl C1Ahb1enr {
        #[doc = "DMA1 Clock Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn dma1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 Clock Enable"]
        #[inline(always)]
        pub const fn set_dma1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2 Clock Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn dma2en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 Clock Enable"]
        #[inline(always)]
        pub const fn set_dma2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ADC1/2 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn adc12en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1/2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_adc12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ART Clock Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn arten(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "ART Clock Enable"]
        #[inline(always)]
        pub const fn set_arten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Ethernet MAC bus interface Clock Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ethen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet MAC bus interface Clock Enable"]
        #[inline(always)]
        pub const fn set_ethen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Ethernet Transmission Clock Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ethtxen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet Transmission Clock Enable"]
        #[inline(always)]
        pub const fn set_ethtxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Ethernet Reception Clock Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ethrxen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet Reception Clock Enable"]
        #[inline(always)]
        pub const fn set_ethrxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USB_OTG_HS Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn usb_otg_hsen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "USB_OTG_HS Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_usb_otg_hsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "USB_PHY1 Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn usb_otg_hs_ulpien(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "USB_PHY1 Clocks Enable"]
        #[inline(always)]
        pub const fn set_usb_otg_hs_ulpien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "USB_OTG_FS Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn usb_otg_fsen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "USB_OTG_FS Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_usb_otg_fsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "USB_PHY2 Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn usb_otg_fs_ulpien(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "USB_PHY2 Clocks Enable"]
        #[inline(always)]
        pub const fn set_usb_otg_fs_ulpien(&mut self, val: bool) {
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
            defmt :: write ! (f , "C1Ahb1enr {{ dma1en: {=bool:?}, dma2en: {=bool:?}, adc12en: {=bool:?}, arten: {=bool:?}, ethen: {=bool:?}, ethtxen: {=bool:?}, ethrxen: {=bool:?}, usb_otg_hsen: {=bool:?}, usb_otg_hs_ulpien: {=bool:?}, usb_otg_fsen: {=bool:?}, usb_otg_fs_ulpien: {=bool:?} }}" , self . dma1en () , self . dma2en () , self . adc12en () , self . arten () , self . ethen () , self . ethtxen () , self . ethrxen () , self . usb_otg_hsen () , self . usb_otg_hs_ulpien () , self . usb_otg_fsen () , self . usb_otg_fs_ulpien ())
        }
    }
    #[doc = "RCC AHB1 Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Ahb1lpenr(pub u32);
    impl C1Ahb1lpenr {
        #[doc = "DMA1 Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn dma1lpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_dma1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2 Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn dma2lpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_dma2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ADC1/2 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn adc12lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1/2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_adc12lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ART Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn artlpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "ART Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_artlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Ethernet MAC bus interface Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn ethlpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet MAC bus interface Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_ethlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Ethernet Transmission Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn ethtxlpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet Transmission Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_ethtxlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Ethernet Reception Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn ethrxlpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet Reception Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_ethrxlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USB_OTG_HS peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn usb_otg_hslpen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "USB_OTG_HS peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_usb_otg_hslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "USB_PHY1 clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn usb_otg_hs_ulpilpen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "USB_PHY1 clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_usb_otg_hs_ulpilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "USB_OTG_FS peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn usb_otg_fslpen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "USB_OTG_FS peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_usb_otg_fslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "USB_PHY2 clocks enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn usb_otg_fs_ulpilpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "USB_PHY2 clocks enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_usb_otg_fs_ulpilpen(&mut self, val: bool) {
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
            defmt :: write ! (f , "C1Ahb1lpenr {{ dma1lpen: {=bool:?}, dma2lpen: {=bool:?}, adc12lpen: {=bool:?}, artlpen: {=bool:?}, ethlpen: {=bool:?}, ethtxlpen: {=bool:?}, ethrxlpen: {=bool:?}, usb_otg_hslpen: {=bool:?}, usb_otg_hs_ulpilpen: {=bool:?}, usb_otg_fslpen: {=bool:?}, usb_otg_fs_ulpilpen: {=bool:?} }}" , self . dma1lpen () , self . dma2lpen () , self . adc12lpen () , self . artlpen () , self . ethlpen () , self . ethtxlpen () , self . ethrxlpen () , self . usb_otg_hslpen () , self . usb_otg_hs_ulpilpen () , self . usb_otg_fslpen () , self . usb_otg_fs_ulpilpen ())
        }
    }
    #[doc = "RCC AHB2 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Ahb2enr(pub u32);
    impl C1Ahb2enr {
        #[doc = "DCMI peripheral clock"]
        #[must_use]
        #[inline(always)]
        pub const fn dcmien(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DCMI peripheral clock"]
        #[inline(always)]
        pub const fn set_dcmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CRYP peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn crypen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CRYP peripheral clock enable"]
        #[inline(always)]
        pub const fn set_crypen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "HASH peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn hashen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HASH peripheral clock enable"]
        #[inline(always)]
        pub const fn set_hashen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "RNG peripheral clocks enable"]
        #[must_use]
        #[inline(always)]
        pub const fn rngen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RNG peripheral clocks enable"]
        #[inline(always)]
        pub const fn set_rngen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "SDMMC2 and SDMMC2 delay clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn sdmmc2en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC2 and SDMMC2 delay clock enable"]
        #[inline(always)]
        pub const fn set_sdmmc2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "SRAM1 block enable"]
        #[must_use]
        #[inline(always)]
        pub const fn sram1en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 block enable"]
        #[inline(always)]
        pub const fn set_sram1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "SRAM2 block enable"]
        #[must_use]
        #[inline(always)]
        pub const fn sram2en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 block enable"]
        #[inline(always)]
        pub const fn set_sram2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SRAM3 block enable"]
        #[must_use]
        #[inline(always)]
        pub const fn sram3en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM3 block enable"]
        #[inline(always)]
        pub const fn set_sram3en(&mut self, val: bool) {
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
            defmt :: write ! (f , "C1Ahb2enr {{ dcmien: {=bool:?}, crypen: {=bool:?}, hashen: {=bool:?}, rngen: {=bool:?}, sdmmc2en: {=bool:?}, sram1en: {=bool:?}, sram2en: {=bool:?}, sram3en: {=bool:?} }}" , self . dcmien () , self . crypen () , self . hashen () , self . rngen () , self . sdmmc2en () , self . sram1en () , self . sram2en () , self . sram3en ())
        }
    }
    #[doc = "RCC AHB2 Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Ahb2lpenr(pub u32);
    impl C1Ahb2lpenr {
        #[doc = "DCMI peripheral clock enable during csleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn dcmilpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DCMI peripheral clock enable during csleep mode"]
        #[inline(always)]
        pub const fn set_dcmilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CRYP peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn cryplpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CRYP peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_cryplpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "HASH peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn hashlpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HASH peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_hashlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "RNG peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn rnglpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RNG peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_rnglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn sdmmc2lpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_sdmmc2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "FMAC enable during CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn fmaclpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "FMAC enable during CSleep Mode"]
        #[inline(always)]
        pub const fn set_fmaclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "CORDIC enable during CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn cordiclpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "CORDIC enable during CSleep Mode"]
        #[inline(always)]
        pub const fn set_cordiclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "SRAM1 Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn sram1lpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_sram1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "SRAM2 Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn sram2lpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_sram2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SRAM3 Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn sram3lpen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM3 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_sram3lpen(&mut self, val: bool) {
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
            defmt :: write ! (f , "C1Ahb2lpenr {{ dcmilpen: {=bool:?}, cryplpen: {=bool:?}, hashlpen: {=bool:?}, rnglpen: {=bool:?}, sdmmc2lpen: {=bool:?}, fmaclpen: {=bool:?}, cordiclpen: {=bool:?}, sram1lpen: {=bool:?}, sram2lpen: {=bool:?}, sram3lpen: {=bool:?} }}" , self . dcmilpen () , self . cryplpen () , self . hashlpen () , self . rnglpen () , self . sdmmc2lpen () , self . fmaclpen () , self . cordiclpen () , self . sram1lpen () , self . sram2lpen () , self . sram3lpen ())
        }
    }
    #[doc = "RCC AHB3 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Ahb3enr(pub u32);
    impl C1Ahb3enr {
        #[doc = "MDMA Peripheral Clock Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn mdmaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MDMA Peripheral Clock Enable"]
        #[inline(always)]
        pub const fn set_mdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2D Peripheral Clock Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn dma2den(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2D Peripheral Clock Enable"]
        #[inline(always)]
        pub const fn set_dma2den(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "JPGDEC Peripheral Clock Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn jpgdecen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "JPGDEC Peripheral Clock Enable"]
        #[inline(always)]
        pub const fn set_jpgdecen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FMC Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn fmcen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "FMC Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_fmcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "QUADSPI and QUADSPI Delay Clock Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn quadspien(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "QUADSPI and QUADSPI Delay Clock Enable"]
        #[inline(always)]
        pub const fn set_quadspien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SDMMC1 and SDMMC1 Delay Clock Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn sdmmc1en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC1 and SDMMC1 Delay Clock Enable"]
        #[inline(always)]
        pub const fn set_sdmmc1en(&mut self, val: bool) {
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
            defmt :: write ! (f , "C1Ahb3enr {{ mdmaen: {=bool:?}, dma2den: {=bool:?}, jpgdecen: {=bool:?}, fmcen: {=bool:?}, quadspien: {=bool:?}, sdmmc1en: {=bool:?} }}" , self . mdmaen () , self . dma2den () , self . jpgdecen () , self . fmcen () , self . quadspien () , self . sdmmc1en ())
        }
    }
    #[doc = "RCC AHB3 Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Ahb3lpenr(pub u32);
    impl C1Ahb3lpenr {
        #[doc = "MDMA Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn mdmalpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MDMA Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_mdmalpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DMA2D Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn dma2dlpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2D Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_dma2dlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "JPGDEC Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn jpgdeclpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "JPGDEC Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_jpgdeclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Flash interface clock enable during csleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn flashpren(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Flash interface clock enable during csleep mode"]
        #[inline(always)]
        pub const fn set_flashpren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "FMC Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn fmclpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "FMC Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_fmclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "QUADSPI and QUADSPI Delay Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn quadspilpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "QUADSPI and QUADSPI Delay Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_quadspilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn sdmmc1lpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_sdmmc1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "OCTOSPI2 and OCTOSPI2 delay block enable during CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn octospi2lpen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI2 and OCTOSPI2 delay block enable during CSleep Mode"]
        #[inline(always)]
        pub const fn set_octospi2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "OCTOSPI IO manager enable during CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn iomngrlpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI IO manager enable during CSleep Mode"]
        #[inline(always)]
        pub const fn set_iomngrlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "OTFDEC1 enable during CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn otfd1lpen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC1 enable during CSleep Mode"]
        #[inline(always)]
        pub const fn set_otfd1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "OTFDEC2 enable during CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn otfd2lpen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC2 enable during CSleep Mode"]
        #[inline(always)]
        pub const fn set_otfd2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "D1DTCM1 Block Clock Enable During CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn d1dtcm1lpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "D1DTCM1 Block Clock Enable During CSleep mode"]
        #[inline(always)]
        pub const fn set_d1dtcm1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "D1 DTCM2 Block Clock Enable During CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn dtcm2lpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "D1 DTCM2 Block Clock Enable During CSleep mode"]
        #[inline(always)]
        pub const fn set_dtcm2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "D1ITCM Block Clock Enable During CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn itcmlpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "D1ITCM Block Clock Enable During CSleep mode"]
        #[inline(always)]
        pub const fn set_itcmlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "AXISRAM Block Clock Enable During CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn axisramlpen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "AXISRAM Block Clock Enable During CSleep mode"]
        #[inline(always)]
        pub const fn set_axisramlpen(&mut self, val: bool) {
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
            defmt :: write ! (f , "C1Ahb3lpenr {{ mdmalpen: {=bool:?}, dma2dlpen: {=bool:?}, jpgdeclpen: {=bool:?}, flashpren: {=bool:?}, fmclpen: {=bool:?}, quadspilpen: {=bool:?}, sdmmc1lpen: {=bool:?}, octospi2lpen: {=bool:?}, iomngrlpen: {=bool:?}, otfd1lpen: {=bool:?}, otfd2lpen: {=bool:?}, d1dtcm1lpen: {=bool:?}, dtcm2lpen: {=bool:?}, itcmlpen: {=bool:?}, axisramlpen: {=bool:?} }}" , self . mdmalpen () , self . dma2dlpen () , self . jpgdeclpen () , self . flashpren () , self . fmclpen () , self . quadspilpen () , self . sdmmc1lpen () , self . octospi2lpen () , self . iomngrlpen () , self . otfd1lpen () , self . otfd2lpen () , self . d1dtcm1lpen () , self . dtcm2lpen () , self . itcmlpen () , self . axisramlpen ())
        }
    }
    #[doc = "RCC AHB4 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Ahb4enr(pub u32);
    impl C1Ahb4enr {
        #[doc = "0GPIO peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn set_gpioaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioben(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn set_gpioben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiocen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn set_gpiocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioden(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn set_gpioden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioeen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn set_gpioeen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiofen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn set_gpiofen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiogen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn set_gpiogen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiohen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn set_gpiohen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioien(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn set_gpioien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiojen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn set_gpiojen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioken(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "0GPIO peripheral clock enable"]
        #[inline(always)]
        pub const fn set_gpioken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "CRC peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn crcen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "CRC peripheral clock enable"]
        #[inline(always)]
        pub const fn set_crcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "BDMA and DMAMUX2 Clock Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn bdmaen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "BDMA and DMAMUX2 Clock Enable"]
        #[inline(always)]
        pub const fn set_bdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ADC3 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn adc3en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ADC3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_adc3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "HSEM peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn hsemen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "HSEM peripheral clock enable"]
        #[inline(always)]
        pub const fn set_hsemen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Backup RAM Clock Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn bkpsramen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Backup RAM Clock Enable"]
        #[inline(always)]
        pub const fn set_bkpsramen(&mut self, val: bool) {
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
            defmt :: write ! (f , "C1Ahb4enr {{ gpioaen: {=bool:?}, gpioben: {=bool:?}, gpiocen: {=bool:?}, gpioden: {=bool:?}, gpioeen: {=bool:?}, gpiofen: {=bool:?}, gpiogen: {=bool:?}, gpiohen: {=bool:?}, gpioien: {=bool:?}, gpiojen: {=bool:?}, gpioken: {=bool:?}, crcen: {=bool:?}, bdmaen: {=bool:?}, adc3en: {=bool:?}, hsemen: {=bool:?}, bkpsramen: {=bool:?} }}" , self . gpioaen () , self . gpioben () , self . gpiocen () , self . gpioden () , self . gpioeen () , self . gpiofen () , self . gpiogen () , self . gpiohen () , self . gpioien () , self . gpiojen () , self . gpioken () , self . crcen () , self . bdmaen () , self . adc3en () , self . hsemen () , self . bkpsramen ())
        }
    }
    #[doc = "RCC AHB4 Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Ahb4lpenr(pub u32);
    impl C1Ahb4lpenr {
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioalpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_gpioalpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioblpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_gpioblpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioclpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_gpioclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiodlpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_gpiodlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioelpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_gpioelpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioflpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_gpioflpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioglpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_gpioglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiohlpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_gpiohlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioilpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_gpioilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiojlpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_gpiojlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioklpen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_gpioklpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "CRC peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn crclpen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "CRC peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_crclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "BDMA Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn bdmalpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "BDMA Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_bdmalpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ADC3 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn adc3lpen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ADC3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_adc3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Backup RAM Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn bkpsramlpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Backup RAM Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_bkpsramlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "SRAM4 Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn sram4lpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM4 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_sram4lpen(&mut self, val: bool) {
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
            defmt :: write ! (f , "C1Ahb4lpenr {{ gpioalpen: {=bool:?}, gpioblpen: {=bool:?}, gpioclpen: {=bool:?}, gpiodlpen: {=bool:?}, gpioelpen: {=bool:?}, gpioflpen: {=bool:?}, gpioglpen: {=bool:?}, gpiohlpen: {=bool:?}, gpioilpen: {=bool:?}, gpiojlpen: {=bool:?}, gpioklpen: {=bool:?}, crclpen: {=bool:?}, bdmalpen: {=bool:?}, adc3lpen: {=bool:?}, bkpsramlpen: {=bool:?}, sram4lpen: {=bool:?} }}" , self . gpioalpen () , self . gpioblpen () , self . gpioclpen () , self . gpiodlpen () , self . gpioelpen () , self . gpioflpen () , self . gpioglpen () , self . gpiohlpen () , self . gpioilpen () , self . gpiojlpen () , self . gpioklpen () , self . crclpen () , self . bdmalpen () , self . adc3lpen () , self . bkpsramlpen () , self . sram4lpen ())
        }
    }
    #[doc = "RCC APB1 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Apb1henr(pub u32);
    impl C1Apb1henr {
        #[doc = "Clock Recovery System peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn crsen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clock Recovery System peripheral clock enable"]
        #[inline(always)]
        pub const fn set_crsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SWPMI Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn swpmien(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SWPMI Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_swpmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "OPAMP peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn opampen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP peripheral clock enable"]
        #[inline(always)]
        pub const fn set_opampen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MDIOS peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn mdiosen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MDIOS peripheral clock enable"]
        #[inline(always)]
        pub const fn set_mdiosen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FDCAN Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn fdcanen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_fdcanen(&mut self, val: bool) {
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
            defmt :: write ! (f , "C1Apb1henr {{ crsen: {=bool:?}, swpmien: {=bool:?}, opampen: {=bool:?}, mdiosen: {=bool:?}, fdcanen: {=bool:?} }}" , self . crsen () , self . swpmien () , self . opampen () , self . mdiosen () , self . fdcanen ())
        }
    }
    #[doc = "RCC APB1 High Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Apb1hlpenr(pub u32);
    impl C1Apb1hlpenr {
        #[doc = "Clock Recovery System peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn crslpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clock Recovery System peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_crslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SWPMI Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn swpmilpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SWPMI Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_swpmilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "OPAMP peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn opamplpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_opamplpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MDIOS peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn mdioslpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MDIOS peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_mdioslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FDCAN Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn fdcanlpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_fdcanlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "TIM23 block enable during CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim23lpen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "TIM23 block enable during CSleep Mode"]
        #[inline(always)]
        pub const fn set_tim23lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "TIM24 block enable during CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim24lpen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "TIM24 block enable during CSleep Mode"]
        #[inline(always)]
        pub const fn set_tim24lpen(&mut self, val: bool) {
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
            defmt :: write ! (f , "C1Apb1hlpenr {{ crslpen: {=bool:?}, swpmilpen: {=bool:?}, opamplpen: {=bool:?}, mdioslpen: {=bool:?}, fdcanlpen: {=bool:?}, tim23lpen: {=bool:?}, tim24lpen: {=bool:?} }}" , self . crslpen () , self . swpmilpen () , self . opamplpen () , self . mdioslpen () , self . fdcanlpen () , self . tim23lpen () , self . tim24lpen ())
        }
    }
    #[doc = "RCC APB1 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Apb1lenr(pub u32);
    impl C1Apb1lenr {
        #[doc = "TIM peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim2en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn set_tim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim3en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn set_tim3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim4en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn set_tim4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim5en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn set_tim5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim6en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn set_tim6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim7en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn set_tim7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim12en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn set_tim12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim13en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn set_tim13en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIM peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim14en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM peripheral clock enable"]
        #[inline(always)]
        pub const fn set_tim14en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LPTIM1 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim1en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_lptim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "WWDG2 peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn wwdg2en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG2 peripheral clock enable"]
        #[inline(always)]
        pub const fn set_wwdg2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI2 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn spi2en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_spi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn spi3en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_spi3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "SPDIFRX Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn spdifrxen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SPDIFRX Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_spdifrxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "USART2 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn usart2en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_usart2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn usart3en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_usart3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn uart4en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_uart4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn uart5en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_uart5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_i2c1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c2en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_i2c2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I2C3 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c3en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_i2c3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "I2C5 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c5en(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "I2C5 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_i2c5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "HDMI-CEC peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn cecen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "HDMI-CEC peripheral clock enable"]
        #[inline(always)]
        pub const fn set_cecen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "DAC1&2 peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn dac12en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1&2 peripheral clock enable"]
        #[inline(always)]
        pub const fn set_dac12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "UART7 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn uart7en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "UART7 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_uart7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "UART8 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn uart8en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "UART8 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_uart8en(&mut self, val: bool) {
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
            defmt :: write ! (f , "C1Apb1lenr {{ tim2en: {=bool:?}, tim3en: {=bool:?}, tim4en: {=bool:?}, tim5en: {=bool:?}, tim6en: {=bool:?}, tim7en: {=bool:?}, tim12en: {=bool:?}, tim13en: {=bool:?}, tim14en: {=bool:?}, lptim1en: {=bool:?}, wwdg2en: {=bool:?}, spi2en: {=bool:?}, spi3en: {=bool:?}, spdifrxen: {=bool:?}, usart2en: {=bool:?}, usart3en: {=bool:?}, uart4en: {=bool:?}, uart5en: {=bool:?}, i2c1en: {=bool:?}, i2c2en: {=bool:?}, i2c3en: {=bool:?}, i2c5en: {=bool:?}, cecen: {=bool:?}, dac12en: {=bool:?}, uart7en: {=bool:?}, uart8en: {=bool:?} }}" , self . tim2en () , self . tim3en () , self . tim4en () , self . tim5en () , self . tim6en () , self . tim7en () , self . tim12en () , self . tim13en () , self . tim14en () , self . lptim1en () , self . wwdg2en () , self . spi2en () , self . spi3en () , self . spdifrxen () , self . usart2en () , self . usart3en () , self . uart4en () , self . uart5en () , self . i2c1en () , self . i2c2en () , self . i2c3en () , self . i2c5en () , self . cecen () , self . dac12en () , self . uart7en () , self . uart8en ())
        }
    }
    #[doc = "RCC APB1 Low Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Apb1llpenr(pub u32);
    impl C1Apb1llpenr {
        #[doc = "TIM2 peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim2lpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_tim2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim3lpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_tim3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim4lpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_tim4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5 peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim5lpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_tim5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim6lpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_tim6lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim7lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_tim7lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM12 peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim12lpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_tim12lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIM13 peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim13lpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIM13 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_tim13lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIM14 peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim14lpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM14 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_tim14lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LPTIM1 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim1lpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_lptim1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "WWDG2 peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn wwdg2lpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG2 peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_wwdg2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI2 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn spi2lpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_spi2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn spi3lpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_spi3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "SPDIFRX Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn spdifrxlpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SPDIFRX Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_spdifrxlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "USART2 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn usart2lpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_usart2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn usart3lpen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_usart3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn uart4lpen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_uart4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn uart5lpen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_uart5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1lpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_i2c1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c2lpen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_i2c2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I2C3 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c3lpen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_i2c3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "I2C5 block enable during CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c5lpen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "I2C5 block enable during CSleep Mode"]
        #[inline(always)]
        pub const fn set_i2c5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "HDMI-CEC Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn ceclpen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "HDMI-CEC Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_ceclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "DAC1/2 peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn dac12lpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1/2 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_dac12lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "UART7 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn uart7lpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "UART7 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_uart7lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "UART8 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn uart8lpen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "UART8 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_uart8lpen(&mut self, val: bool) {
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
            defmt :: write ! (f , "C1Apb1llpenr {{ tim2lpen: {=bool:?}, tim3lpen: {=bool:?}, tim4lpen: {=bool:?}, tim5lpen: {=bool:?}, tim6lpen: {=bool:?}, tim7lpen: {=bool:?}, tim12lpen: {=bool:?}, tim13lpen: {=bool:?}, tim14lpen: {=bool:?}, lptim1lpen: {=bool:?}, wwdg2lpen: {=bool:?}, spi2lpen: {=bool:?}, spi3lpen: {=bool:?}, spdifrxlpen: {=bool:?}, usart2lpen: {=bool:?}, usart3lpen: {=bool:?}, uart4lpen: {=bool:?}, uart5lpen: {=bool:?}, i2c1lpen: {=bool:?}, i2c2lpen: {=bool:?}, i2c3lpen: {=bool:?}, i2c5lpen: {=bool:?}, ceclpen: {=bool:?}, dac12lpen: {=bool:?}, uart7lpen: {=bool:?}, uart8lpen: {=bool:?} }}" , self . tim2lpen () , self . tim3lpen () , self . tim4lpen () , self . tim5lpen () , self . tim6lpen () , self . tim7lpen () , self . tim12lpen () , self . tim13lpen () , self . tim14lpen () , self . lptim1lpen () , self . wwdg2lpen () , self . spi2lpen () , self . spi3lpen () , self . spdifrxlpen () , self . usart2lpen () , self . usart3lpen () , self . uart4lpen () , self . uart5lpen () , self . i2c1lpen () , self . i2c2lpen () , self . i2c3lpen () , self . i2c5lpen () , self . ceclpen () , self . dac12lpen () , self . uart7lpen () , self . uart8lpen ())
        }
    }
    #[doc = "RCC APB2 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Apb2enr(pub u32);
    impl C1Apb2enr {
        #[doc = "TIM1 peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 peripheral clock enable"]
        #[inline(always)]
        pub const fn set_tim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM8 peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim8en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 peripheral clock enable"]
        #[inline(always)]
        pub const fn set_tim8en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "USART1 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn usart1en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_usart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "USART6 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn usart6en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USART6 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_usart6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "UART9 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn uart9en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "UART9 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_uart9en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "USART10 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn usart10en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "USART10 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_usart10en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "SPI1 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn spi1en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_spi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SPI4 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn spi4en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "SPI4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_spi4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "TIM15 peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim15en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 peripheral clock enable"]
        #[inline(always)]
        pub const fn set_tim15en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim16en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 peripheral clock enable"]
        #[inline(always)]
        pub const fn set_tim16en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim17en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 peripheral clock enable"]
        #[inline(always)]
        pub const fn set_tim17en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SPI5 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn spi5en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SPI5 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_spi5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "SAI1 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn sai1en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_sai1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "SAI2 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn sai2en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "SAI2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_sai2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "SAI3 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn sai3en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "SAI3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_sai3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "DFSDM1 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn dfsdm1en(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "DFSDM1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_dfsdm1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "HRTIM peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn hrtimen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM peripheral clock enable"]
        #[inline(always)]
        pub const fn set_hrtimen(&mut self, val: bool) {
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
            defmt :: write ! (f , "C1Apb2enr {{ tim1en: {=bool:?}, tim8en: {=bool:?}, usart1en: {=bool:?}, usart6en: {=bool:?}, uart9en: {=bool:?}, usart10en: {=bool:?}, spi1en: {=bool:?}, spi4en: {=bool:?}, tim15en: {=bool:?}, tim16en: {=bool:?}, tim17en: {=bool:?}, spi5en: {=bool:?}, sai1en: {=bool:?}, sai2en: {=bool:?}, sai3en: {=bool:?}, dfsdm1en: {=bool:?}, hrtimen: {=bool:?} }}" , self . tim1en () , self . tim8en () , self . usart1en () , self . usart6en () , self . uart9en () , self . usart10en () , self . spi1en () , self . spi4en () , self . tim15en () , self . tim16en () , self . tim17en () , self . spi5en () , self . sai1en () , self . sai2en () , self . sai3en () , self . dfsdm1en () , self . hrtimen ())
        }
    }
    #[doc = "RCC APB2 Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Apb2lpenr(pub u32);
    impl C1Apb2lpenr {
        #[doc = "TIM1 peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim1lpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_tim1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM8 peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim8lpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_tim8lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "USART1 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn usart1lpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_usart1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "USART6 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn usart6lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USART6 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_usart6lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SPI1 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn spi1lpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_spi1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SPI4 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn spi4lpen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "SPI4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_spi4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "TIM15 peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim15lpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_tim15lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim16lpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_tim16lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim17lpen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_tim17lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SPI5 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn spi5lpen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SPI5 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_spi5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "SAI1 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn sai1lpen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_sai1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "SAI2 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn sai2lpen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "SAI2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_sai2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "SAI3 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn sai3lpen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "SAI3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_sai3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "DFSDM1 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn dfsdm1lpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "DFSDM1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_dfsdm1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "HRTIM peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn hrtimlpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_hrtimlpen(&mut self, val: bool) {
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
            defmt :: write ! (f , "C1Apb2lpenr {{ tim1lpen: {=bool:?}, tim8lpen: {=bool:?}, usart1lpen: {=bool:?}, usart6lpen: {=bool:?}, spi1lpen: {=bool:?}, spi4lpen: {=bool:?}, tim15lpen: {=bool:?}, tim16lpen: {=bool:?}, tim17lpen: {=bool:?}, spi5lpen: {=bool:?}, sai1lpen: {=bool:?}, sai2lpen: {=bool:?}, sai3lpen: {=bool:?}, dfsdm1lpen: {=bool:?}, hrtimlpen: {=bool:?} }}" , self . tim1lpen () , self . tim8lpen () , self . usart1lpen () , self . usart6lpen () , self . spi1lpen () , self . spi4lpen () , self . tim15lpen () , self . tim16lpen () , self . tim17lpen () , self . spi5lpen () , self . sai1lpen () , self . sai2lpen () , self . sai3lpen () , self . dfsdm1lpen () , self . hrtimlpen ())
        }
    }
    #[doc = "RCC APB3 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Apb3enr(pub u32);
    impl C1Apb3enr {
        #[doc = "LTDC peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ltdcen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC peripheral clock enable"]
        #[inline(always)]
        pub const fn set_ltdcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DSI Peripheral clocks enable"]
        #[must_use]
        #[inline(always)]
        pub const fn dsien(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DSI Peripheral clocks enable"]
        #[inline(always)]
        pub const fn set_dsien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "WWDG1 Clock Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn wwdg1en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG1 Clock Enable"]
        #[inline(always)]
        pub const fn set_wwdg1en(&mut self, val: bool) {
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
            defmt::write!(
                f,
                "C1Apb3enr {{ ltdcen: {=bool:?}, dsien: {=bool:?}, wwdg1en: {=bool:?} }}",
                self.ltdcen(),
                self.dsien(),
                self.wwdg1en()
            )
        }
    }
    #[doc = "RCC APB3 Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Apb3lpenr(pub u32);
    impl C1Apb3lpenr {
        #[doc = "LTDC peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn ltdclpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_ltdclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DSI Peripheral Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn dsilpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DSI Peripheral Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_dsilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "WWDG1 Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn wwdg1lpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG1 Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_wwdg1lpen(&mut self, val: bool) {
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
            defmt::write!(
                f,
                "C1Apb3lpenr {{ ltdclpen: {=bool:?}, dsilpen: {=bool:?}, wwdg1lpen: {=bool:?} }}",
                self.ltdclpen(),
                self.dsilpen(),
                self.wwdg1lpen()
            )
        }
    }
    #[doc = "RCC APB4 Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Apb4enr(pub u32);
    impl C1Apb4enr {
        #[doc = "SYSCFG peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn syscfgen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG peripheral clock enable"]
        #[inline(always)]
        pub const fn set_syscfgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPUART1 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lpuart1en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_lpuart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SPI6 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn spi6en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI6 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_spi6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "I2C4 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c4en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_i2c4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LPTIM2 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim2en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_lptim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LPTIM3 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim3en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_lptim3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LPTIM4 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim4en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_lptim4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIM5 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim5en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM5 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_lptim5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "COMP1/2 peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn comp12en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "COMP1/2 peripheral clock enable"]
        #[inline(always)]
        pub const fn set_comp12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "VREF peripheral clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn vrefen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "VREF peripheral clock enable"]
        #[inline(always)]
        pub const fn set_vrefen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "RTC APB Clock Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn rtcapben(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "RTC APB Clock Enable"]
        #[inline(always)]
        pub const fn set_rtcapben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "SAI4 Peripheral Clocks Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn sai4en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SAI4 Peripheral Clocks Enable"]
        #[inline(always)]
        pub const fn set_sai4en(&mut self, val: bool) {
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
            defmt :: write ! (f , "C1Apb4enr {{ syscfgen: {=bool:?}, lpuart1en: {=bool:?}, spi6en: {=bool:?}, i2c4en: {=bool:?}, lptim2en: {=bool:?}, lptim3en: {=bool:?}, lptim4en: {=bool:?}, lptim5en: {=bool:?}, comp12en: {=bool:?}, vrefen: {=bool:?}, rtcapben: {=bool:?}, sai4en: {=bool:?} }}" , self . syscfgen () , self . lpuart1en () , self . spi6en () , self . i2c4en () , self . lptim2en () , self . lptim3en () , self . lptim4en () , self . lptim5en () , self . comp12en () , self . vrefen () , self . rtcapben () , self . sai4en ())
        }
    }
    #[doc = "RCC APB4 Sleep Clock Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Apb4lpenr(pub u32);
    impl C1Apb4lpenr {
        #[doc = "SYSCFG peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn syscfglpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_syscfglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPUART1 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn lpuart1lpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_lpuart1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SPI6 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn spi6lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI6 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_spi6lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "I2C4 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c4lpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_i2c4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LPTIM2 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim2lpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_lptim2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LPTIM3 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim3lpen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_lptim3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LPTIM4 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim4lpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_lptim4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIM5 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim5lpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM5 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_lptim5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "COMP1/2 peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn comp12lpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "COMP1/2 peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_comp12lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "VREF peripheral clock enable during CSleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn vreflpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "VREF peripheral clock enable during CSleep mode"]
        #[inline(always)]
        pub const fn set_vreflpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "RTC APB Clock Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn rtcapblpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "RTC APB Clock Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_rtcapblpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "SAI4 Peripheral Clocks Enable During CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn sai4lpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SAI4 Peripheral Clocks Enable During CSleep Mode"]
        #[inline(always)]
        pub const fn set_sai4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Digital temperature sensor block enable during CSleep Mode"]
        #[must_use]
        #[inline(always)]
        pub const fn dtslpen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Digital temperature sensor block enable during CSleep Mode"]
        #[inline(always)]
        pub const fn set_dtslpen(&mut self, val: bool) {
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
            defmt :: write ! (f , "C1Apb4lpenr {{ syscfglpen: {=bool:?}, lpuart1lpen: {=bool:?}, spi6lpen: {=bool:?}, i2c4lpen: {=bool:?}, lptim2lpen: {=bool:?}, lptim3lpen: {=bool:?}, lptim4lpen: {=bool:?}, lptim5lpen: {=bool:?}, comp12lpen: {=bool:?}, vreflpen: {=bool:?}, rtcapblpen: {=bool:?}, sai4lpen: {=bool:?}, dtslpen: {=bool:?} }}" , self . syscfglpen () , self . lpuart1lpen () , self . spi6lpen () , self . i2c4lpen () , self . lptim2lpen () , self . lptim3lpen () , self . lptim4lpen () , self . lptim5lpen () , self . comp12lpen () , self . vreflpen () , self . rtcapblpen () , self . sai4lpen () , self . dtslpen ())
        }
    }
    #[doc = "RCC Reset Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C1Rsr(pub u32);
    impl C1Rsr {
        #[doc = "Remove reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn rmvf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Remove reset flag"]
        #[inline(always)]
        pub const fn set_rmvf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "CPU reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn cpurstf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "CPU reset flag"]
        #[inline(always)]
        pub const fn set_cpurstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "D1 domain power switch reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn d1rstf(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "D1 domain power switch reset flag"]
        #[inline(always)]
        pub const fn set_d1rstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "D2 domain power switch reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn d2rstf(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "D2 domain power switch reset flag"]
        #[inline(always)]
        pub const fn set_d2rstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "BOR reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn borrstf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "BOR reset flag"]
        #[inline(always)]
        pub const fn set_borrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Pin reset flag (NRST)"]
        #[must_use]
        #[inline(always)]
        pub const fn pinrstf(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Pin reset flag (NRST)"]
        #[inline(always)]
        pub const fn set_pinrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "POR/PDR reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn porrstf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "POR/PDR reset flag"]
        #[inline(always)]
        pub const fn set_porrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "System reset from CPU reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn sftrstf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "System reset from CPU reset flag"]
        #[inline(always)]
        pub const fn set_sftrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Independent Watchdog reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn iwdg1rstf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Independent Watchdog reset flag"]
        #[inline(always)]
        pub const fn set_iwdg1rstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Window Watchdog reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn wwdg1rstf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Window Watchdog reset flag"]
        #[inline(always)]
        pub const fn set_wwdg1rstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Reset due to illegal D1 DStandby or CPU CStop flag"]
        #[must_use]
        #[inline(always)]
        pub const fn lpwrrstf(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Reset due to illegal D1 DStandby or CPU CStop flag"]
        #[inline(always)]
        pub const fn set_lpwrrstf(&mut self, val: bool) {
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
            defmt :: write ! (f , "C1Rsr {{ rmvf: {=bool:?}, cpurstf: {=bool:?}, d1rstf: {=bool:?}, d2rstf: {=bool:?}, borrstf: {=bool:?}, pinrstf: {=bool:?}, porrstf: {=bool:?}, sftrstf: {=bool:?}, iwdg1rstf: {=bool:?}, wwdg1rstf: {=bool:?}, lpwrrstf: {=bool:?} }}" , self . rmvf () , self . cpurstf () , self . d1rstf () , self . d2rstf () , self . borrstf () , self . pinrstf () , self . porrstf () , self . sftrstf () , self . iwdg1rstf () , self . wwdg1rstf () , self . lpwrrstf ())
        }
    }
    #[doc = "RCC Clock Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "System clock switch"]
        #[must_use]
        #[inline(always)]
        pub const fn sw(&self) -> super::vals::Sw {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "System clock switch"]
        #[inline(always)]
        pub const fn set_sw(&mut self, val: super::vals::Sw) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "System clock switch status"]
        #[must_use]
        #[inline(always)]
        pub const fn sws(&self) -> super::vals::Sw {
            let val = (self.0 >> 3usize) & 0x07;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "System clock switch status"]
        #[inline(always)]
        pub const fn set_sws(&mut self, val: super::vals::Sw) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
        }
        #[doc = "System clock selection after a wake up from system Stop"]
        #[must_use]
        #[inline(always)]
        pub const fn stopwuck(&self) -> super::vals::Stopwuck {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Stopwuck::from_bits(val as u8)
        }
        #[doc = "System clock selection after a wake up from system Stop"]
        #[inline(always)]
        pub const fn set_stopwuck(&mut self, val: super::vals::Stopwuck) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "Kernel clock selection after a wake up from system Stop"]
        #[must_use]
        #[inline(always)]
        pub const fn stopkerwuck(&self) -> super::vals::Stopwuck {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Stopwuck::from_bits(val as u8)
        }
        #[doc = "Kernel clock selection after a wake up from system Stop"]
        #[inline(always)]
        pub const fn set_stopkerwuck(&mut self, val: super::vals::Stopwuck) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "HSE division factor for RTC clock"]
        #[must_use]
        #[inline(always)]
        pub const fn rtcpre(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "HSE division factor for RTC clock"]
        #[inline(always)]
        pub const fn set_rtcpre(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "High Resolution Timer clock prescaler selection"]
        #[must_use]
        #[inline(always)]
        pub const fn hrtimsel(&self) -> super::vals::Hrtimsel {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Hrtimsel::from_bits(val as u8)
        }
        #[doc = "High Resolution Timer clock prescaler selection"]
        #[inline(always)]
        pub const fn set_hrtimsel(&mut self, val: super::vals::Hrtimsel) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "Timers clocks prescaler selection"]
        #[must_use]
        #[inline(always)]
        pub const fn timpre(&self) -> super::vals::Timpre {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Timpre::from_bits(val as u8)
        }
        #[doc = "Timers clocks prescaler selection"]
        #[inline(always)]
        pub const fn set_timpre(&mut self, val: super::vals::Timpre) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "MCO1 prescaler"]
        #[must_use]
        #[inline(always)]
        pub const fn mco1pre(&self) -> super::vals::Mcopre {
            let val = (self.0 >> 18usize) & 0x0f;
            super::vals::Mcopre::from_bits(val as u8)
        }
        #[doc = "MCO1 prescaler"]
        #[inline(always)]
        pub const fn set_mco1pre(&mut self, val: super::vals::Mcopre) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val.to_bits() as u32) & 0x0f) << 18usize);
        }
        #[doc = "Micro-controller clock output 1"]
        #[must_use]
        #[inline(always)]
        pub const fn mco1sel(&self) -> super::vals::Mco1sel {
            let val = (self.0 >> 22usize) & 0x07;
            super::vals::Mco1sel::from_bits(val as u8)
        }
        #[doc = "Micro-controller clock output 1"]
        #[inline(always)]
        pub const fn set_mco1sel(&mut self, val: super::vals::Mco1sel) {
            self.0 = (self.0 & !(0x07 << 22usize)) | (((val.to_bits() as u32) & 0x07) << 22usize);
        }
        #[doc = "MCO2 prescaler"]
        #[must_use]
        #[inline(always)]
        pub const fn mco2pre(&self) -> super::vals::Mcopre {
            let val = (self.0 >> 25usize) & 0x0f;
            super::vals::Mcopre::from_bits(val as u8)
        }
        #[doc = "MCO2 prescaler"]
        #[inline(always)]
        pub const fn set_mco2pre(&mut self, val: super::vals::Mcopre) {
            self.0 = (self.0 & !(0x0f << 25usize)) | (((val.to_bits() as u32) & 0x0f) << 25usize);
        }
        #[doc = "Micro-controller clock output 2"]
        #[must_use]
        #[inline(always)]
        pub const fn mco2sel(&self) -> super::vals::Mco2sel {
            let val = (self.0 >> 29usize) & 0x07;
            super::vals::Mco2sel::from_bits(val as u8)
        }
        #[doc = "Micro-controller clock output 2"]
        #[inline(always)]
        pub const fn set_mco2sel(&mut self, val: super::vals::Mco2sel) {
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
            defmt :: write ! (f , "Cfgr {{ sw: {:?}, sws: {:?}, stopwuck: {:?}, stopkerwuck: {:?}, rtcpre: {=u8:?}, hrtimsel: {:?}, timpre: {:?}, mco1pre: {:?}, mco1sel: {:?}, mco2pre: {:?}, mco2sel: {:?} }}" , self . sw () , self . sws () , self . stopwuck () , self . stopkerwuck () , self . rtcpre () , self . hrtimsel () , self . timpre () , self . mco1pre () , self . mco1sel () , self . mco2pre () , self . mco2sel ())
        }
    }
    #[doc = "RCC Clock Source Interrupt Clear Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cicr(pub u32);
    impl Cicr {
        #[doc = "LSI ready Interrupt Clear"]
        #[must_use]
        #[inline(always)]
        pub const fn lsirdyc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready Interrupt Clear"]
        #[inline(always)]
        pub const fn set_lsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready Interrupt Clear"]
        #[must_use]
        #[inline(always)]
        pub const fn lserdyc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready Interrupt Clear"]
        #[inline(always)]
        pub const fn set_lserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI ready Interrupt Clear"]
        #[must_use]
        #[inline(always)]
        pub const fn hsirdyc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready Interrupt Clear"]
        #[inline(always)]
        pub const fn set_hsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSE ready Interrupt Clear"]
        #[must_use]
        #[inline(always)]
        pub const fn hserdyc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready Interrupt Clear"]
        #[inline(always)]
        pub const fn set_hserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CSI ready Interrupt Clear"]
        #[must_use]
        #[inline(always)]
        pub const fn hse_ready_interrupt_clear(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CSI ready Interrupt Clear"]
        #[inline(always)]
        pub const fn set_hse_ready_interrupt_clear(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "RC48 ready Interrupt Clear"]
        #[must_use]
        #[inline(always)]
        pub const fn hsi48rdyc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RC48 ready Interrupt Clear"]
        #[inline(always)]
        pub const fn set_hsi48rdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PLL1 ready Interrupt Clear"]
        #[must_use]
        #[inline(always)]
        pub const fn pllrdyc(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 ready Interrupt Clear"]
        #[inline(always)]
        pub const fn set_pllrdyc(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "LSE clock security system Interrupt Clear"]
        #[must_use]
        #[inline(always)]
        pub const fn lsecssc(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock security system Interrupt Clear"]
        #[inline(always)]
        pub const fn set_lsecssc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "HSE clock security system Interrupt Clear"]
        #[must_use]
        #[inline(always)]
        pub const fn hsecssc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock security system Interrupt Clear"]
        #[inline(always)]
        pub const fn set_hsecssc(&mut self, val: bool) {
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
                .field("pllrdyc[0]", &self.pllrdyc(0usize))
                .field("pllrdyc[1]", &self.pllrdyc(1usize))
                .field("pllrdyc[2]", &self.pllrdyc(2usize))
                .field("lsecssc", &self.lsecssc())
                .field("hsecssc", &self.hsecssc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cicr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cicr {{ lsirdyc: {=bool:?}, lserdyc: {=bool:?}, hsirdyc: {=bool:?}, hserdyc: {=bool:?}, hse_ready_interrupt_clear: {=bool:?}, hsi48rdyc: {=bool:?}, pllrdyc[0]: {=bool:?}, pllrdyc[1]: {=bool:?}, pllrdyc[2]: {=bool:?}, lsecssc: {=bool:?}, hsecssc: {=bool:?} }}" , self . lsirdyc () , self . lserdyc () , self . hsirdyc () , self . hserdyc () , self . hse_ready_interrupt_clear () , self . hsi48rdyc () , self . pllrdyc (0usize) , self . pllrdyc (1usize) , self . pllrdyc (2usize) , self . lsecssc () , self . hsecssc ())
        }
    }
    #[doc = "RCC Clock Source Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cier(pub u32);
    impl Cier {
        #[doc = "LSI ready Interrupt Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lsirdyie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready Interrupt Enable"]
        #[inline(always)]
        pub const fn set_lsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready Interrupt Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lserdyie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready Interrupt Enable"]
        #[inline(always)]
        pub const fn set_lserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI ready Interrupt Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn hsirdyie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready Interrupt Enable"]
        #[inline(always)]
        pub const fn set_hsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSE ready Interrupt Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn hserdyie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready Interrupt Enable"]
        #[inline(always)]
        pub const fn set_hserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CSI ready Interrupt Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn csirdyie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CSI ready Interrupt Enable"]
        #[inline(always)]
        pub const fn set_csirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "RC48 ready Interrupt Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn hsi48rdyie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RC48 ready Interrupt Enable"]
        #[inline(always)]
        pub const fn set_hsi48rdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PLL1 ready Interrupt Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn pllrdyie(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 ready Interrupt Enable"]
        #[inline(always)]
        pub const fn set_pllrdyie(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "LSE clock security system Interrupt Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lsecssie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock security system Interrupt Enable"]
        #[inline(always)]
        pub const fn set_lsecssie(&mut self, val: bool) {
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
                .field("pllrdyie[0]", &self.pllrdyie(0usize))
                .field("pllrdyie[1]", &self.pllrdyie(1usize))
                .field("pllrdyie[2]", &self.pllrdyie(2usize))
                .field("lsecssie", &self.lsecssie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cier {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cier {{ lsirdyie: {=bool:?}, lserdyie: {=bool:?}, hsirdyie: {=bool:?}, hserdyie: {=bool:?}, csirdyie: {=bool:?}, hsi48rdyie: {=bool:?}, pllrdyie[0]: {=bool:?}, pllrdyie[1]: {=bool:?}, pllrdyie[2]: {=bool:?}, lsecssie: {=bool:?} }}" , self . lsirdyie () , self . lserdyie () , self . hsirdyie () , self . hserdyie () , self . csirdyie () , self . hsi48rdyie () , self . pllrdyie (0usize) , self . pllrdyie (1usize) , self . pllrdyie (2usize) , self . lsecssie ())
        }
    }
    #[doc = "RCC Clock Source Interrupt Flag Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cifr(pub u32);
    impl Cifr {
        #[doc = "LSI ready Interrupt Flag"]
        #[must_use]
        #[inline(always)]
        pub const fn lsirdyf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready Interrupt Flag"]
        #[inline(always)]
        pub const fn set_lsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready Interrupt Flag"]
        #[must_use]
        #[inline(always)]
        pub const fn lserdyf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready Interrupt Flag"]
        #[inline(always)]
        pub const fn set_lserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI ready Interrupt Flag"]
        #[must_use]
        #[inline(always)]
        pub const fn hsirdyf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready Interrupt Flag"]
        #[inline(always)]
        pub const fn set_hsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSE ready Interrupt Flag"]
        #[must_use]
        #[inline(always)]
        pub const fn hserdyf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready Interrupt Flag"]
        #[inline(always)]
        pub const fn set_hserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CSI ready Interrupt Flag"]
        #[must_use]
        #[inline(always)]
        pub const fn csirdy(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CSI ready Interrupt Flag"]
        #[inline(always)]
        pub const fn set_csirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "RC48 ready Interrupt Flag"]
        #[must_use]
        #[inline(always)]
        pub const fn hsi48rdyf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RC48 ready Interrupt Flag"]
        #[inline(always)]
        pub const fn set_hsi48rdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PLL1 ready Interrupt Flag"]
        #[must_use]
        #[inline(always)]
        pub const fn pllrdyf(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 ready Interrupt Flag"]
        #[inline(always)]
        pub const fn set_pllrdyf(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "LSE clock security system Interrupt Flag"]
        #[must_use]
        #[inline(always)]
        pub const fn lsecssf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock security system Interrupt Flag"]
        #[inline(always)]
        pub const fn set_lsecssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "HSE clock security system Interrupt Flag"]
        #[must_use]
        #[inline(always)]
        pub const fn hsecssf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock security system Interrupt Flag"]
        #[inline(always)]
        pub const fn set_hsecssf(&mut self, val: bool) {
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
                .field("pllrdyf[0]", &self.pllrdyf(0usize))
                .field("pllrdyf[1]", &self.pllrdyf(1usize))
                .field("pllrdyf[2]", &self.pllrdyf(2usize))
                .field("lsecssf", &self.lsecssf())
                .field("hsecssf", &self.hsecssf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cifr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cifr {{ lsirdyf: {=bool:?}, lserdyf: {=bool:?}, hsirdyf: {=bool:?}, hserdyf: {=bool:?}, csirdy: {=bool:?}, hsi48rdyf: {=bool:?}, pllrdyf[0]: {=bool:?}, pllrdyf[1]: {=bool:?}, pllrdyf[2]: {=bool:?}, lsecssf: {=bool:?}, hsecssf: {=bool:?} }}" , self . lsirdyf () , self . lserdyf () , self . hsirdyf () , self . hserdyf () , self . csirdy () , self . hsi48rdyf () , self . pllrdyf (0usize) , self . pllrdyf (1usize) , self . pllrdyf (2usize) , self . lsecssf () , self . hsecssf ())
        }
    }
    #[doc = "clock control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Internal high-speed clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn hsion(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Internal high-speed clock enable"]
        #[inline(always)]
        pub const fn set_hsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "High Speed Internal clock enable in Stop mode"]
        #[must_use]
        #[inline(always)]
        pub const fn hsikeron(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "High Speed Internal clock enable in Stop mode"]
        #[inline(always)]
        pub const fn set_hsikeron(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI clock ready flag"]
        #[must_use]
        #[inline(always)]
        pub const fn hsirdy(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSI clock ready flag"]
        #[inline(always)]
        pub const fn set_hsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSI clock divider"]
        #[must_use]
        #[inline(always)]
        pub const fn hsidiv(&self) -> super::vals::Hsidiv {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Hsidiv::from_bits(val as u8)
        }
        #[doc = "HSI clock divider"]
        #[inline(always)]
        pub const fn set_hsidiv(&mut self, val: super::vals::Hsidiv) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "HSI divider flag"]
        #[must_use]
        #[inline(always)]
        pub const fn hsidivf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HSI divider flag"]
        #[inline(always)]
        pub const fn set_hsidivf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "CSI clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn csion(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CSI clock enable"]
        #[inline(always)]
        pub const fn set_csion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "CSI clock ready flag"]
        #[must_use]
        #[inline(always)]
        pub const fn csirdy(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "CSI clock ready flag"]
        #[inline(always)]
        pub const fn set_csirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CSI clock enable in Stop mode"]
        #[must_use]
        #[inline(always)]
        pub const fn csikeron(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "CSI clock enable in Stop mode"]
        #[inline(always)]
        pub const fn set_csikeron(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "RC48 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn hsi48on(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "RC48 clock enable"]
        #[inline(always)]
        pub const fn set_hsi48on(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "RC48 clock ready flag"]
        #[must_use]
        #[inline(always)]
        pub const fn hsi48rdy(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "RC48 clock ready flag"]
        #[inline(always)]
        pub const fn set_hsi48rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "D1 domain clocks ready flag"]
        #[must_use]
        #[inline(always)]
        pub const fn d1ckrdy(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "D1 domain clocks ready flag"]
        #[inline(always)]
        pub const fn set_d1ckrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "D2 domain clocks ready flag"]
        #[must_use]
        #[inline(always)]
        pub const fn d2ckrdy(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "D2 domain clocks ready flag"]
        #[inline(always)]
        pub const fn set_d2ckrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "HSE clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn hseon(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock enable"]
        #[inline(always)]
        pub const fn set_hseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "HSE clock ready flag"]
        #[must_use]
        #[inline(always)]
        pub const fn hserdy(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock ready flag"]
        #[inline(always)]
        pub const fn set_hserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "HSE clock bypass"]
        #[must_use]
        #[inline(always)]
        pub const fn hsebyp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock bypass"]
        #[inline(always)]
        pub const fn set_hsebyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "HSE Clock Security System enable"]
        #[must_use]
        #[inline(always)]
        pub const fn hsecsson(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "HSE Clock Security System enable"]
        #[inline(always)]
        pub const fn set_hsecsson(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "PLL1 enable"]
        #[must_use]
        #[inline(always)]
        pub const fn pllon(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 24usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 enable"]
        #[inline(always)]
        pub const fn set_pllon(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 24usize + n * 2usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "PLL1 clock ready flag"]
        #[must_use]
        #[inline(always)]
        pub const fn pllrdy(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 25usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 clock ready flag"]
        #[inline(always)]
        pub const fn set_pllrdy(&mut self, n: usize, val: bool) {
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
                .field("pllon[0]", &self.pllon(0usize))
                .field("pllon[1]", &self.pllon(1usize))
                .field("pllon[2]", &self.pllon(2usize))
                .field("pllrdy[0]", &self.pllrdy(0usize))
                .field("pllrdy[1]", &self.pllrdy(1usize))
                .field("pllrdy[2]", &self.pllrdy(2usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ hsion: {=bool:?}, hsikeron: {=bool:?}, hsirdy: {=bool:?}, hsidiv: {:?}, hsidivf: {=bool:?}, csion: {=bool:?}, csirdy: {=bool:?}, csikeron: {=bool:?}, hsi48on: {=bool:?}, hsi48rdy: {=bool:?}, d1ckrdy: {=bool:?}, d2ckrdy: {=bool:?}, hseon: {=bool:?}, hserdy: {=bool:?}, hsebyp: {=bool:?}, hsecsson: {=bool:?}, pllon[0]: {=bool:?}, pllon[1]: {=bool:?}, pllon[2]: {=bool:?}, pllrdy[0]: {=bool:?}, pllrdy[1]: {=bool:?}, pllrdy[2]: {=bool:?} }}" , self . hsion () , self . hsikeron () , self . hsirdy () , self . hsidiv () , self . hsidivf () , self . csion () , self . csirdy () , self . csikeron () , self . hsi48on () , self . hsi48rdy () , self . d1ckrdy () , self . d2ckrdy () , self . hseon () , self . hserdy () , self . hsebyp () , self . hsecsson () , self . pllon (0usize) , self . pllon (1usize) , self . pllon (2usize) , self . pllrdy (0usize) , self . pllrdy (1usize) , self . pllrdy (2usize))
        }
    }
    #[doc = "RCC Clock Recovery RC Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crrcr(pub u32);
    impl Crrcr {
        #[doc = "Internal RC 48 MHz clock calibration"]
        #[must_use]
        #[inline(always)]
        pub const fn hsi48cal(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Internal RC 48 MHz clock calibration"]
        #[inline(always)]
        pub const fn set_hsi48cal(&mut self, val: u16) {
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
            defmt::write!(f, "Crrcr {{ hsi48cal: {=u16:?} }}", self.hsi48cal())
        }
    }
    #[doc = "RCC CSI configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csicfgr(pub u32);
    impl Csicfgr {
        #[doc = "CSI clock calibration"]
        #[must_use]
        #[inline(always)]
        pub const fn csical(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "CSI clock calibration"]
        #[inline(always)]
        pub const fn set_csical(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "CSI clock trimming"]
        #[must_use]
        #[inline(always)]
        pub const fn csitrim(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "CSI clock trimming"]
        #[inline(always)]
        pub const fn set_csitrim(&mut self, val: u8) {
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
            defmt::write!(
                f,
                "Csicfgr {{ csical: {=u16:?}, csitrim: {=u8:?} }}",
                self.csical(),
                self.csitrim()
            )
        }
    }
    #[doc = "RCC Clock Control and Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "LSI oscillator enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lsion(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI oscillator enable"]
        #[inline(always)]
        pub const fn set_lsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSI oscillator ready"]
        #[must_use]
        #[inline(always)]
        pub const fn lsirdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSI oscillator ready"]
        #[inline(always)]
        pub const fn set_lsirdy(&mut self, val: bool) {
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
            defmt::write!(
                f,
                "Csr {{ lsion: {=bool:?}, lsirdy: {=bool:?} }}",
                self.lsion(),
                self.lsirdy()
            )
        }
    }
    #[doc = "RCC Domain 1 Kernel Clock Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct D1ccipr(pub u32);
    impl D1ccipr {
        #[doc = "FMC kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn fmcsel(&self) -> super::vals::Fmcsel {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Fmcsel::from_bits(val as u8)
        }
        #[doc = "FMC kernel clock source selection"]
        #[inline(always)]
        pub const fn set_fmcsel(&mut self, val: super::vals::Fmcsel) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "OCTOSPI kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn octospisel(&self) -> super::vals::Fmcsel {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Fmcsel::from_bits(val as u8)
        }
        #[doc = "OCTOSPI kernel clock source selection"]
        #[inline(always)]
        pub const fn set_octospisel(&mut self, val: super::vals::Fmcsel) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "QUADSPI kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn quadspisel(&self) -> super::vals::Fmcsel {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Fmcsel::from_bits(val as u8)
        }
        #[doc = "QUADSPI kernel clock source selection"]
        #[inline(always)]
        pub const fn set_quadspisel(&mut self, val: super::vals::Fmcsel) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "DSI clock source selection (not available on all chips)"]
        #[must_use]
        #[inline(always)]
        pub const fn dsisel(&self) -> super::vals::Dsisel {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Dsisel::from_bits(val as u8)
        }
        #[doc = "DSI clock source selection (not available on all chips)"]
        #[inline(always)]
        pub const fn set_dsisel(&mut self, val: super::vals::Dsisel) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "SDMMC kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn sdmmcsel(&self) -> super::vals::Sdmmcsel {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Sdmmcsel::from_bits(val as u8)
        }
        #[doc = "SDMMC kernel clock source selection"]
        #[inline(always)]
        pub const fn set_sdmmcsel(&mut self, val: super::vals::Sdmmcsel) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "per_ck clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn persel(&self) -> super::vals::Persel {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Persel::from_bits(val as u8)
        }
        #[doc = "per_ck clock source selection"]
        #[inline(always)]
        pub const fn set_persel(&mut self, val: super::vals::Persel) {
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
                .field("octospisel", &self.octospisel())
                .field("quadspisel", &self.quadspisel())
                .field("dsisel", &self.dsisel())
                .field("sdmmcsel", &self.sdmmcsel())
                .field("persel", &self.persel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for D1ccipr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "D1ccipr {{ fmcsel: {:?}, octospisel: {:?}, quadspisel: {:?}, dsisel: {:?}, sdmmcsel: {:?}, persel: {:?} }}" , self . fmcsel () , self . octospisel () , self . quadspisel () , self . dsisel () , self . sdmmcsel () , self . persel ())
        }
    }
    #[doc = "RCC Domain 1 Clock Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct D1cfgr(pub u32);
    impl D1cfgr {
        #[doc = "D1 domain AHB prescaler"]
        #[must_use]
        #[inline(always)]
        pub const fn hpre(&self) -> super::vals::Hpre {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Hpre::from_bits(val as u8)
        }
        #[doc = "D1 domain AHB prescaler"]
        #[inline(always)]
        pub const fn set_hpre(&mut self, val: super::vals::Hpre) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "D1 domain APB3 prescaler"]
        #[must_use]
        #[inline(always)]
        pub const fn d1ppre(&self) -> super::vals::Ppre {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "D1 domain APB3 prescaler"]
        #[inline(always)]
        pub const fn set_d1ppre(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "D1 domain Core prescaler"]
        #[must_use]
        #[inline(always)]
        pub const fn d1cpre(&self) -> super::vals::Hpre {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::Hpre::from_bits(val as u8)
        }
        #[doc = "D1 domain Core prescaler"]
        #[inline(always)]
        pub const fn set_d1cpre(&mut self, val: super::vals::Hpre) {
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
            defmt::write!(
                f,
                "D1cfgr {{ hpre: {:?}, d1ppre: {:?}, d1cpre: {:?} }}",
                self.hpre(),
                self.d1ppre(),
                self.d1cpre()
            )
        }
    }
    #[doc = "RCC Domain 2 Kernel Clock Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct D2ccip1r(pub u32);
    impl D2ccip1r {
        #[doc = "SAI1 and DFSDM1 kernel Aclk clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn sai1sel(&self) -> super::vals::Saisel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Saisel::from_bits(val as u8)
        }
        #[doc = "SAI1 and DFSDM1 kernel Aclk clock source selection"]
        #[inline(always)]
        pub const fn set_sai1sel(&mut self, val: super::vals::Saisel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "SAI2 and SAI3 kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn sai23sel(&self) -> super::vals::Saisel {
            let val = (self.0 >> 6usize) & 0x07;
            super::vals::Saisel::from_bits(val as u8)
        }
        #[doc = "SAI2 and SAI3 kernel clock source selection"]
        #[inline(always)]
        pub const fn set_sai23sel(&mut self, val: super::vals::Saisel) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val.to_bits() as u32) & 0x07) << 6usize);
        }
        #[doc = "SPI/I2S1,2 and 3 kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn spi123sel(&self) -> super::vals::Saisel {
            let val = (self.0 >> 12usize) & 0x07;
            super::vals::Saisel::from_bits(val as u8)
        }
        #[doc = "SPI/I2S1,2 and 3 kernel clock source selection"]
        #[inline(always)]
        pub const fn set_spi123sel(&mut self, val: super::vals::Saisel) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
        }
        #[doc = "SPI4 and 5 kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn spi45sel(&self) -> super::vals::Spi45sel {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::Spi45sel::from_bits(val as u8)
        }
        #[doc = "SPI4 and 5 kernel clock source selection"]
        #[inline(always)]
        pub const fn set_spi45sel(&mut self, val: super::vals::Spi45sel) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
        #[doc = "SPDIFRX kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn spdifrxsel(&self) -> super::vals::Spdifrxsel {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::Spdifrxsel::from_bits(val as u8)
        }
        #[doc = "SPDIFRX kernel clock source selection"]
        #[inline(always)]
        pub const fn set_spdifrxsel(&mut self, val: super::vals::Spdifrxsel) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "DFSDM1 kernel Clk clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn dfsdm1sel(&self) -> super::vals::Dfsdmsel {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Dfsdmsel::from_bits(val as u8)
        }
        #[doc = "DFSDM1 kernel Clk clock source selection"]
        #[inline(always)]
        pub const fn set_dfsdm1sel(&mut self, val: super::vals::Dfsdmsel) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
        #[doc = "FDCAN kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn fdcansel(&self) -> super::vals::Fdcansel {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Fdcansel::from_bits(val as u8)
        }
        #[doc = "FDCAN kernel clock source selection"]
        #[inline(always)]
        pub const fn set_fdcansel(&mut self, val: super::vals::Fdcansel) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "SWPMI kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn swpmisel(&self) -> super::vals::Swpmisel {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Swpmisel::from_bits(val as u8)
        }
        #[doc = "SWPMI kernel clock source selection"]
        #[inline(always)]
        pub const fn set_swpmisel(&mut self, val: super::vals::Swpmisel) {
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
            defmt :: write ! (f , "D2ccip1r {{ sai1sel: {:?}, sai23sel: {:?}, spi123sel: {:?}, spi45sel: {:?}, spdifrxsel: {:?}, dfsdm1sel: {:?}, fdcansel: {:?}, swpmisel: {:?} }}" , self . sai1sel () , self . sai23sel () , self . spi123sel () , self . spi45sel () , self . spdifrxsel () , self . dfsdm1sel () , self . fdcansel () , self . swpmisel ())
        }
    }
    #[doc = "RCC Domain 2 Kernel Clock Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct D2ccip2r(pub u32);
    impl D2ccip2r {
        #[doc = "USART2/3, UART4,5, 7/8 (APB1) kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn usart234578sel(&self) -> super::vals::Usart234578sel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Usart234578sel::from_bits(val as u8)
        }
        #[doc = "USART2/3, UART4,5, 7/8 (APB1) kernel clock source selection"]
        #[inline(always)]
        pub const fn set_usart234578sel(&mut self, val: super::vals::Usart234578sel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "USART1, 6, 9 and 10 kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn usart16910sel(&self) -> super::vals::Usart16910sel {
            let val = (self.0 >> 3usize) & 0x07;
            super::vals::Usart16910sel::from_bits(val as u8)
        }
        #[doc = "USART1, 6, 9 and 10 kernel clock source selection"]
        #[inline(always)]
        pub const fn set_usart16910sel(&mut self, val: super::vals::Usart16910sel) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
        }
        #[doc = "RNG kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn rngsel(&self) -> super::vals::Rngsel {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Rngsel::from_bits(val as u8)
        }
        #[doc = "RNG kernel clock source selection"]
        #[inline(always)]
        pub const fn set_rngsel(&mut self, val: super::vals::Rngsel) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "I2C1,2,3 kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1235sel(&self) -> super::vals::I2c1235sel {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::I2c1235sel::from_bits(val as u8)
        }
        #[doc = "I2C1,2,3 kernel clock source selection"]
        #[inline(always)]
        pub const fn set_i2c1235sel(&mut self, val: super::vals::I2c1235sel) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "USBOTG 1 and 2 kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn usbsel(&self) -> super::vals::Usbsel {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::Usbsel::from_bits(val as u8)
        }
        #[doc = "USBOTG 1 and 2 kernel clock source selection"]
        #[inline(always)]
        pub const fn set_usbsel(&mut self, val: super::vals::Usbsel) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "HDMI-CEC kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn cecsel(&self) -> super::vals::Cecsel {
            let val = (self.0 >> 22usize) & 0x03;
            super::vals::Cecsel::from_bits(val as u8)
        }
        #[doc = "HDMI-CEC kernel clock source selection"]
        #[inline(always)]
        pub const fn set_cecsel(&mut self, val: super::vals::Cecsel) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
        }
        #[doc = "LPTIM1 kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim1sel(&self) -> super::vals::Lptim1sel {
            let val = (self.0 >> 28usize) & 0x07;
            super::vals::Lptim1sel::from_bits(val as u8)
        }
        #[doc = "LPTIM1 kernel clock source selection"]
        #[inline(always)]
        pub const fn set_lptim1sel(&mut self, val: super::vals::Lptim1sel) {
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
            defmt :: write ! (f , "D2ccip2r {{ usart234578sel: {:?}, usart16910sel: {:?}, rngsel: {:?}, i2c1235sel: {:?}, usbsel: {:?}, cecsel: {:?}, lptim1sel: {:?} }}" , self . usart234578sel () , self . usart16910sel () , self . rngsel () , self . i2c1235sel () , self . usbsel () , self . cecsel () , self . lptim1sel ())
        }
    }
    #[doc = "RCC Domain 2 Clock Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct D2cfgr(pub u32);
    impl D2cfgr {
        #[doc = "D2 domain APB1 prescaler"]
        #[must_use]
        #[inline(always)]
        pub const fn d2ppre1(&self) -> super::vals::Ppre {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "D2 domain APB1 prescaler"]
        #[inline(always)]
        pub const fn set_d2ppre1(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "D2 domain APB2 prescaler"]
        #[must_use]
        #[inline(always)]
        pub const fn d2ppre2(&self) -> super::vals::Ppre {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "D2 domain APB2 prescaler"]
        #[inline(always)]
        pub const fn set_d2ppre2(&mut self, val: super::vals::Ppre) {
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
            defmt::write!(
                f,
                "D2cfgr {{ d2ppre1: {:?}, d2ppre2: {:?} }}",
                self.d2ppre1(),
                self.d2ppre2()
            )
        }
    }
    #[doc = "RCC D3 Autonomous mode Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct D3amr(pub u32);
    impl D3amr {
        #[doc = "BDMA and DMAMUX Autonomous mode enable"]
        #[must_use]
        #[inline(always)]
        pub const fn bdmaamen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "BDMA and DMAMUX Autonomous mode enable"]
        #[inline(always)]
        pub const fn set_bdmaamen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LPUART1 Autonomous mode enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lpuart1amen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 Autonomous mode enable"]
        #[inline(always)]
        pub const fn set_lpuart1amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SPI6 Autonomous mode enable"]
        #[must_use]
        #[inline(always)]
        pub const fn spi6amen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI6 Autonomous mode enable"]
        #[inline(always)]
        pub const fn set_spi6amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "I2C4 Autonomous mode enable"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c4amen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 Autonomous mode enable"]
        #[inline(always)]
        pub const fn set_i2c4amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LPTIM2 Autonomous mode enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim2amen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 Autonomous mode enable"]
        #[inline(always)]
        pub const fn set_lptim2amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LPTIM3 Autonomous mode enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim3amen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3 Autonomous mode enable"]
        #[inline(always)]
        pub const fn set_lptim3amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LPTIM4 Autonomous mode enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim4amen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM4 Autonomous mode enable"]
        #[inline(always)]
        pub const fn set_lptim4amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIM5 Autonomous mode enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim5amen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM5 Autonomous mode enable"]
        #[inline(always)]
        pub const fn set_lptim5amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "DAC2 (containing one converter) Autonomous mode enable"]
        #[must_use]
        #[inline(always)]
        pub const fn dac2amen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "DAC2 (containing one converter) Autonomous mode enable"]
        #[inline(always)]
        pub const fn set_dac2amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "COMP12 Autonomous mode enable"]
        #[must_use]
        #[inline(always)]
        pub const fn comp12amen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "COMP12 Autonomous mode enable"]
        #[inline(always)]
        pub const fn set_comp12amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "VREF Autonomous mode enable"]
        #[must_use]
        #[inline(always)]
        pub const fn vrefamen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "VREF Autonomous mode enable"]
        #[inline(always)]
        pub const fn set_vrefamen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "RTC Autonomous mode enable"]
        #[must_use]
        #[inline(always)]
        pub const fn rtcamen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "RTC Autonomous mode enable"]
        #[inline(always)]
        pub const fn set_rtcamen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "CRC Autonomous mode enable"]
        #[must_use]
        #[inline(always)]
        pub const fn crcamen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "CRC Autonomous mode enable"]
        #[inline(always)]
        pub const fn set_crcamen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "SAI4 Autonomous mode enable"]
        #[must_use]
        #[inline(always)]
        pub const fn sai4amen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SAI4 Autonomous mode enable"]
        #[inline(always)]
        pub const fn set_sai4amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ADC3 Autonomous mode enable"]
        #[must_use]
        #[inline(always)]
        pub const fn adc3amen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ADC3 Autonomous mode enable"]
        #[inline(always)]
        pub const fn set_adc3amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Digital temperature sensor Autonomous mode enable"]
        #[must_use]
        #[inline(always)]
        pub const fn dtsamen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Digital temperature sensor Autonomous mode enable"]
        #[inline(always)]
        pub const fn set_dtsamen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Backup RAM Autonomous mode enable"]
        #[must_use]
        #[inline(always)]
        pub const fn bkpsramamen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Backup RAM Autonomous mode enable"]
        #[inline(always)]
        pub const fn set_bkpsramamen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "SRAM4 Autonomous mode enable"]
        #[must_use]
        #[inline(always)]
        pub const fn sram4amen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM4 Autonomous mode enable"]
        #[inline(always)]
        pub const fn set_sram4amen(&mut self, val: bool) {
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
            defmt :: write ! (f , "D3amr {{ bdmaamen: {=bool:?}, lpuart1amen: {=bool:?}, spi6amen: {=bool:?}, i2c4amen: {=bool:?}, lptim2amen: {=bool:?}, lptim3amen: {=bool:?}, lptim4amen: {=bool:?}, lptim5amen: {=bool:?}, dac2amen: {=bool:?}, comp12amen: {=bool:?}, vrefamen: {=bool:?}, rtcamen: {=bool:?}, crcamen: {=bool:?}, sai4amen: {=bool:?}, adc3amen: {=bool:?}, dtsamen: {=bool:?}, bkpsramamen: {=bool:?}, sram4amen: {=bool:?} }}" , self . bdmaamen () , self . lpuart1amen () , self . spi6amen () , self . i2c4amen () , self . lptim2amen () , self . lptim3amen () , self . lptim4amen () , self . lptim5amen () , self . dac2amen () , self . comp12amen () , self . vrefamen () , self . rtcamen () , self . crcamen () , self . sai4amen () , self . adc3amen () , self . dtsamen () , self . bkpsramamen () , self . sram4amen ())
        }
    }
    #[doc = "RCC Domain 3 Kernel Clock Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct D3ccipr(pub u32);
    impl D3ccipr {
        #[doc = "LPUART1 kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn lpuart1sel(&self) -> super::vals::Lpuartsel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Lpuartsel::from_bits(val as u8)
        }
        #[doc = "LPUART1 kernel clock source selection"]
        #[inline(always)]
        pub const fn set_lpuart1sel(&mut self, val: super::vals::Lpuartsel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "I2C4 kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c4sel(&self) -> super::vals::I2c4sel {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::I2c4sel::from_bits(val as u8)
        }
        #[doc = "I2C4 kernel clock source selection"]
        #[inline(always)]
        pub const fn set_i2c4sel(&mut self, val: super::vals::I2c4sel) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "LPTIM2 kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim2sel(&self) -> super::vals::Lptim2sel {
            let val = (self.0 >> 10usize) & 0x07;
            super::vals::Lptim2sel::from_bits(val as u8)
        }
        #[doc = "LPTIM2 kernel clock source selection"]
        #[inline(always)]
        pub const fn set_lptim2sel(&mut self, val: super::vals::Lptim2sel) {
            self.0 = (self.0 & !(0x07 << 10usize)) | (((val.to_bits() as u32) & 0x07) << 10usize);
        }
        #[doc = "LPTIM3,4,5 kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim345sel(&self) -> super::vals::Lptim2sel {
            let val = (self.0 >> 13usize) & 0x07;
            super::vals::Lptim2sel::from_bits(val as u8)
        }
        #[doc = "LPTIM3,4,5 kernel clock source selection"]
        #[inline(always)]
        pub const fn set_lptim345sel(&mut self, val: super::vals::Lptim2sel) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val.to_bits() as u32) & 0x07) << 13usize);
        }
        #[doc = "SAR ADC kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn adcsel(&self) -> super::vals::Adcsel {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Adcsel::from_bits(val as u8)
        }
        #[doc = "SAR ADC kernel clock source selection"]
        #[inline(always)]
        pub const fn set_adcsel(&mut self, val: super::vals::Adcsel) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Sub-Block A of SAI4 kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn sai4asel(&self) -> super::vals::Saiasel {
            let val = (self.0 >> 21usize) & 0x07;
            super::vals::Saiasel::from_bits(val as u8)
        }
        #[doc = "Sub-Block A of SAI4 kernel clock source selection"]
        #[inline(always)]
        pub const fn set_sai4asel(&mut self, val: super::vals::Saiasel) {
            self.0 = (self.0 & !(0x07 << 21usize)) | (((val.to_bits() as u32) & 0x07) << 21usize);
        }
        #[doc = "Sub-Block B of SAI4 kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn sai4bsel(&self) -> super::vals::Saiasel {
            let val = (self.0 >> 24usize) & 0x07;
            super::vals::Saiasel::from_bits(val as u8)
        }
        #[doc = "Sub-Block B of SAI4 kernel clock source selection"]
        #[inline(always)]
        pub const fn set_sai4bsel(&mut self, val: super::vals::Saiasel) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
        }
        #[doc = "DFSDM2 kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn dfsdm2sel(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "DFSDM2 kernel clock source selection"]
        #[inline(always)]
        pub const fn set_dfsdm2sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "SPI6 kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn spi6sel(&self) -> super::vals::Spi6sel {
            let val = (self.0 >> 28usize) & 0x07;
            super::vals::Spi6sel::from_bits(val as u8)
        }
        #[doc = "SPI6 kernel clock source selection"]
        #[inline(always)]
        pub const fn set_spi6sel(&mut self, val: super::vals::Spi6sel) {
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
            defmt :: write ! (f , "D3ccipr {{ lpuart1sel: {:?}, i2c4sel: {:?}, lptim2sel: {:?}, lptim345sel: {:?}, adcsel: {:?}, sai4asel: {:?}, sai4bsel: {:?}, dfsdm2sel: {=bool:?}, spi6sel: {:?} }}" , self . lpuart1sel () , self . i2c4sel () , self . lptim2sel () , self . lptim345sel () , self . adcsel () , self . sai4asel () , self . sai4bsel () , self . dfsdm2sel () , self . spi6sel ())
        }
    }
    #[doc = "RCC Domain 3 Clock Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct D3cfgr(pub u32);
    impl D3cfgr {
        #[doc = "D3 domain APB4 prescaler"]
        #[must_use]
        #[inline(always)]
        pub const fn d3ppre(&self) -> super::vals::Ppre {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "D3 domain APB4 prescaler"]
        #[inline(always)]
        pub const fn set_d3ppre(&mut self, val: super::vals::Ppre) {
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
            defmt::write!(f, "D3cfgr {{ d3ppre: {:?} }}", self.d3ppre())
        }
    }
    #[doc = "Global Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gcr(pub u32);
    impl Gcr {
        #[doc = "WWDG1 reset scope control"]
        #[must_use]
        #[inline(always)]
        pub const fn ww1rsc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG1 reset scope control"]
        #[inline(always)]
        pub const fn set_ww1rsc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "WWDG2 reset scope control"]
        #[must_use]
        #[inline(always)]
        pub const fn ww2rsc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG2 reset scope control"]
        #[inline(always)]
        pub const fn set_ww2rsc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Force allow CPU1 to boot"]
        #[must_use]
        #[inline(always)]
        pub const fn boot_c1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Force allow CPU1 to boot"]
        #[inline(always)]
        pub const fn set_boot_c1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Force allow CPU2 to boot"]
        #[must_use]
        #[inline(always)]
        pub const fn boot_c2(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Force allow CPU2 to boot"]
        #[inline(always)]
        pub const fn set_boot_c2(&mut self, val: bool) {
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
            defmt::write!(
                f,
                "Gcr {{ ww1rsc: {=bool:?}, ww2rsc: {=bool:?}, boot_c1: {=bool:?}, boot_c2: {=bool:?} }}",
                self.ww1rsc(),
                self.ww2rsc(),
                self.boot_c1(),
                self.boot_c2()
            )
        }
    }
    #[doc = "RCC HSI configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hsicfgr(pub u32);
    impl Hsicfgr {
        #[doc = "HSI clock calibration"]
        #[must_use]
        #[inline(always)]
        pub const fn hsical(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "HSI clock calibration"]
        #[inline(always)]
        pub const fn set_hsical(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "HSI clock trimming"]
        #[must_use]
        #[inline(always)]
        pub const fn hsitrim(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x7f;
            val as u8
        }
        #[doc = "HSI clock trimming"]
        #[inline(always)]
        pub const fn set_hsitrim(&mut self, val: u8) {
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
            defmt::write!(
                f,
                "Hsicfgr {{ hsical: {=u16:?}, hsitrim: {=u8:?} }}",
                self.hsical(),
                self.hsitrim()
            )
        }
    }
    #[doc = "RCC Internal Clock Source Calibration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icscr(pub u32);
    impl Icscr {
        #[doc = "HSI clock calibration"]
        #[must_use]
        #[inline(always)]
        pub const fn hsical(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "HSI clock calibration"]
        #[inline(always)]
        pub const fn set_hsical(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "HSI clock trimming"]
        #[must_use]
        #[inline(always)]
        pub const fn hsitrim(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x3f;
            val as u8
        }
        #[doc = "HSI clock trimming"]
        #[inline(always)]
        pub const fn set_hsitrim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 12usize)) | (((val as u32) & 0x3f) << 12usize);
        }
        #[doc = "CSI clock calibration"]
        #[must_use]
        #[inline(always)]
        pub const fn csical(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0xff;
            val as u8
        }
        #[doc = "CSI clock calibration"]
        #[inline(always)]
        pub const fn set_csical(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 18usize)) | (((val as u32) & 0xff) << 18usize);
        }
        #[doc = "CSI clock trimming"]
        #[must_use]
        #[inline(always)]
        pub const fn csitrim(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x1f;
            val as u8
        }
        #[doc = "CSI clock trimming"]
        #[inline(always)]
        pub const fn set_csitrim(&mut self, val: u8) {
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
            defmt::write!(
                f,
                "Icscr {{ hsical: {=u16:?}, hsitrim: {=u8:?}, csical: {=u8:?}, csitrim: {=u8:?} }}",
                self.hsical(),
                self.hsitrim(),
                self.csical(),
                self.csitrim()
            )
        }
    }
    #[doc = "RCC PLLs Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pllcfgr(pub u32);
    impl Pllcfgr {
        #[doc = "PLL1 fractional latch enable"]
        #[must_use]
        #[inline(always)]
        pub const fn pllfracen(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 fractional latch enable"]
        #[inline(always)]
        pub const fn set_pllfracen(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "PLL1 VCO selection"]
        #[must_use]
        #[inline(always)]
        pub const fn pllvcosel(&self, n: usize) -> super::vals::Pllvcosel {
            assert!(n < 3usize);
            let offs = 1usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Pllvcosel::from_bits(val as u8)
        }
        #[doc = "PLL1 VCO selection"]
        #[inline(always)]
        pub const fn set_pllvcosel(&mut self, n: usize, val: super::vals::Pllvcosel) {
            assert!(n < 3usize);
            let offs = 1usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "PLL1 input frequency range"]
        #[must_use]
        #[inline(always)]
        pub const fn pllrge(&self, n: usize) -> super::vals::Pllrge {
            assert!(n < 3usize);
            let offs = 2usize + n * 4usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::Pllrge::from_bits(val as u8)
        }
        #[doc = "PLL1 input frequency range"]
        #[inline(always)]
        pub const fn set_pllrge(&mut self, n: usize, val: super::vals::Pllrge) {
            assert!(n < 3usize);
            let offs = 2usize + n * 4usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "PLL1 DIVP divider output enable"]
        #[must_use]
        #[inline(always)]
        pub const fn divpen(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 16usize + n * 3usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 DIVP divider output enable"]
        #[inline(always)]
        pub const fn set_divpen(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 16usize + n * 3usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "PLL1 DIVQ divider output enable"]
        #[must_use]
        #[inline(always)]
        pub const fn divqen(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 17usize + n * 3usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 DIVQ divider output enable"]
        #[inline(always)]
        pub const fn set_divqen(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 17usize + n * 3usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "PLL1 DIVR divider output enable"]
        #[must_use]
        #[inline(always)]
        pub const fn divren(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 18usize + n * 3usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 DIVR divider output enable"]
        #[inline(always)]
        pub const fn set_divren(&mut self, n: usize, val: bool) {
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
                .field("pllfracen[0]", &self.pllfracen(0usize))
                .field("pllfracen[1]", &self.pllfracen(1usize))
                .field("pllfracen[2]", &self.pllfracen(2usize))
                .field("pllvcosel[0]", &self.pllvcosel(0usize))
                .field("pllvcosel[1]", &self.pllvcosel(1usize))
                .field("pllvcosel[2]", &self.pllvcosel(2usize))
                .field("pllrge[0]", &self.pllrge(0usize))
                .field("pllrge[1]", &self.pllrge(1usize))
                .field("pllrge[2]", &self.pllrge(2usize))
                .field("divpen[0]", &self.divpen(0usize))
                .field("divpen[1]", &self.divpen(1usize))
                .field("divpen[2]", &self.divpen(2usize))
                .field("divqen[0]", &self.divqen(0usize))
                .field("divqen[1]", &self.divqen(1usize))
                .field("divqen[2]", &self.divqen(2usize))
                .field("divren[0]", &self.divren(0usize))
                .field("divren[1]", &self.divren(1usize))
                .field("divren[2]", &self.divren(2usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pllcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Pllcfgr {{ pllfracen[0]: {=bool:?}, pllfracen[1]: {=bool:?}, pllfracen[2]: {=bool:?}, pllvcosel[0]: {:?}, pllvcosel[1]: {:?}, pllvcosel[2]: {:?}, pllrge[0]: {:?}, pllrge[1]: {:?}, pllrge[2]: {:?}, divpen[0]: {=bool:?}, divpen[1]: {=bool:?}, divpen[2]: {=bool:?}, divqen[0]: {=bool:?}, divqen[1]: {=bool:?}, divqen[2]: {=bool:?}, divren[0]: {=bool:?}, divren[1]: {=bool:?}, divren[2]: {=bool:?} }}" , self . pllfracen (0usize) , self . pllfracen (1usize) , self . pllfracen (2usize) , self . pllvcosel (0usize) , self . pllvcosel (1usize) , self . pllvcosel (2usize) , self . pllrge (0usize) , self . pllrge (1usize) , self . pllrge (2usize) , self . divpen (0usize) , self . divpen (1usize) , self . divpen (2usize) , self . divqen (0usize) , self . divqen (1usize) , self . divqen (2usize) , self . divren (0usize) , self . divren (1usize) , self . divren (2usize))
        }
    }
    #[doc = "RCC PLLs Clock Source Selection Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pllckselr(pub u32);
    impl Pllckselr {
        #[doc = "DIVMx and PLLs clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn pllsrc(&self) -> super::vals::Pllsrc {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Pllsrc::from_bits(val as u8)
        }
        #[doc = "DIVMx and PLLs clock source selection"]
        #[inline(always)]
        pub const fn set_pllsrc(&mut self, val: super::vals::Pllsrc) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Prescaler for PLL1"]
        #[must_use]
        #[inline(always)]
        pub const fn divm(&self, n: usize) -> super::vals::Pllm {
            assert!(n < 3usize);
            let offs = 4usize + n * 8usize;
            let val = (self.0 >> offs) & 0x3f;
            super::vals::Pllm::from_bits(val as u8)
        }
        #[doc = "Prescaler for PLL1"]
        #[inline(always)]
        pub const fn set_divm(&mut self, n: usize, val: super::vals::Pllm) {
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
                .field("divm[0]", &self.divm(0usize))
                .field("divm[1]", &self.divm(1usize))
                .field("divm[2]", &self.divm(2usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pllckselr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pllckselr {{ pllsrc: {:?}, divm[0]: {:?}, divm[1]: {:?}, divm[2]: {:?} }}",
                self.pllsrc(),
                self.divm(0usize),
                self.divm(1usize),
                self.divm(2usize)
            )
        }
    }
    #[doc = "RCC PLL1 Dividers Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Plldivr(pub u32);
    impl Plldivr {
        #[doc = "Multiplication factor for PLL1 VCO"]
        #[must_use]
        #[inline(always)]
        pub const fn plln(&self) -> super::vals::Plln {
            let val = (self.0 >> 0usize) & 0x01ff;
            super::vals::Plln::from_bits(val as u16)
        }
        #[doc = "Multiplication factor for PLL1 VCO"]
        #[inline(always)]
        pub const fn set_plln(&mut self, val: super::vals::Plln) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val.to_bits() as u32) & 0x01ff) << 0usize);
        }
        #[doc = "PLL DIVP division factor"]
        #[must_use]
        #[inline(always)]
        pub const fn pllp(&self) -> super::vals::Plldiv {
            let val = (self.0 >> 9usize) & 0x7f;
            super::vals::Plldiv::from_bits(val as u8)
        }
        #[doc = "PLL DIVP division factor"]
        #[inline(always)]
        pub const fn set_pllp(&mut self, val: super::vals::Plldiv) {
            self.0 = (self.0 & !(0x7f << 9usize)) | (((val.to_bits() as u32) & 0x7f) << 9usize);
        }
        #[doc = "PLL DIVQ division factor"]
        #[must_use]
        #[inline(always)]
        pub const fn pllq(&self) -> super::vals::Plldiv {
            let val = (self.0 >> 16usize) & 0x7f;
            super::vals::Plldiv::from_bits(val as u8)
        }
        #[doc = "PLL DIVQ division factor"]
        #[inline(always)]
        pub const fn set_pllq(&mut self, val: super::vals::Plldiv) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val.to_bits() as u32) & 0x7f) << 16usize);
        }
        #[doc = "PLL DIVR division factor"]
        #[must_use]
        #[inline(always)]
        pub const fn pllr(&self) -> super::vals::Plldiv {
            let val = (self.0 >> 24usize) & 0x7f;
            super::vals::Plldiv::from_bits(val as u8)
        }
        #[doc = "PLL DIVR division factor"]
        #[inline(always)]
        pub const fn set_pllr(&mut self, val: super::vals::Plldiv) {
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
            defmt::write!(
                f,
                "Plldivr {{ plln: {:?}, pllp: {:?}, pllq: {:?}, pllr: {:?} }}",
                self.plln(),
                self.pllp(),
                self.pllq(),
                self.pllr()
            )
        }
    }
    #[doc = "RCC PLL Fractional Divider Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pllfracr(pub u32);
    impl Pllfracr {
        #[doc = "Fractional part of the multiplication factor for PLL VCO"]
        #[must_use]
        #[inline(always)]
        pub const fn fracn(&self) -> u16 {
            let val = (self.0 >> 3usize) & 0x1fff;
            val as u16
        }
        #[doc = "Fractional part of the multiplication factor for PLL VCO"]
        #[inline(always)]
        pub const fn set_fracn(&mut self, val: u16) {
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
            defmt::write!(f, "Pllfracr {{ fracn: {=u16:?} }}", self.fracn())
        }
    }
    #[doc = "RCC Reset Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rsr(pub u32);
    impl Rsr {
        #[doc = "Remove reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn rmvf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Remove reset flag"]
        #[inline(always)]
        pub const fn set_rmvf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "CPU reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn cpurstf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "CPU reset flag"]
        #[inline(always)]
        pub const fn set_cpurstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "D1 domain power switch reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn d1rstf(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "D1 domain power switch reset flag"]
        #[inline(always)]
        pub const fn set_d1rstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "D2 domain power switch reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn d2rstf(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "D2 domain power switch reset flag"]
        #[inline(always)]
        pub const fn set_d2rstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "BOR reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn borrstf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "BOR reset flag"]
        #[inline(always)]
        pub const fn set_borrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Pin reset flag (NRST)"]
        #[must_use]
        #[inline(always)]
        pub const fn pinrstf(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Pin reset flag (NRST)"]
        #[inline(always)]
        pub const fn set_pinrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "POR/PDR reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn porrstf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "POR/PDR reset flag"]
        #[inline(always)]
        pub const fn set_porrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "System reset from CPU reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn sftrstf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "System reset from CPU reset flag"]
        #[inline(always)]
        pub const fn set_sftrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Independent Watchdog reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn iwdg1rstf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Independent Watchdog reset flag"]
        #[inline(always)]
        pub const fn set_iwdg1rstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Window Watchdog reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn wwdg1rstf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Window Watchdog reset flag"]
        #[inline(always)]
        pub const fn set_wwdg1rstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Reset due to illegal D1 DStandby or CPU CStop flag"]
        #[must_use]
        #[inline(always)]
        pub const fn lpwrrstf(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Reset due to illegal D1 DStandby or CPU CStop flag"]
        #[inline(always)]
        pub const fn set_lpwrrstf(&mut self, val: bool) {
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
            defmt :: write ! (f , "Rsr {{ rmvf: {=bool:?}, cpurstf: {=bool:?}, d1rstf: {=bool:?}, d2rstf: {=bool:?}, borrstf: {=bool:?}, pinrstf: {=bool:?}, porrstf: {=bool:?}, sftrstf: {=bool:?}, iwdg1rstf: {=bool:?}, wwdg1rstf: {=bool:?}, lpwrrstf: {=bool:?} }}" , self . rmvf () , self . cpurstf () , self . d1rstf () , self . d2rstf () , self . borrstf () , self . pinrstf () , self . porrstf () , self . sftrstf () , self . iwdg1rstf () , self . wwdg1rstf () , self . lpwrrstf ())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adcsel {
        #[doc = "pll2_p selected as peripheral clock"]
        Pll2P = 0x0,
        #[doc = "pll3_r selected as peripheral clock"]
        Pll3R = 0x01,
        #[doc = "PER selected as peripheral clock"]
        Per = 0x02,
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
        Lse = 0x0,
        #[doc = "LSI selected as peripheral clock"]
        Lsi = 0x01,
        #[doc = "csi_ker selected as peripheral clock"]
        Csi = 0x02,
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
        Pclk2 = 0x0,
        #[doc = "System clock selected as peripheral clock"]
        Sys = 0x01,
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
    pub enum Dsisel {
        #[doc = "DSI-PHY used as DSI byte lane clock source (usual case)"]
        DsiPhyDiv8 = 0x0,
        #[doc = "PLL2_Q used as DSI byte lane clock source, used in case DSI PLL and DSI-PHY are off (low power mode)"]
        Pll2Q = 0x01,
    }
    impl Dsisel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dsisel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dsisel {
        #[inline(always)]
        fn from(val: u8) -> Dsisel {
            Dsisel::from_bits(val)
        }
    }
    impl From<Dsisel> for u8 {
        #[inline(always)]
        fn from(val: Dsisel) -> u8 {
            Dsisel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fdcansel {
        #[doc = "HSE selected as peripheral clock"]
        Hse = 0x0,
        #[doc = "pll1_q selected as peripheral clock"]
        Pll1Q = 0x01,
        #[doc = "pll2_q selected as peripheral clock"]
        Pll2Q = 0x02,
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
        #[doc = "AHB3 selected as peripheral clock"]
        Hclk3 = 0x0,
        #[doc = "pll1_q selected as peripheral clock"]
        Pll1Q = 0x01,
        #[doc = "pll2_r selected as peripheral clock"]
        Pll2R = 0x02,
        #[doc = "PER selected as peripheral clock"]
        Per = 0x03,
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
        Div1 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "sys_ck divided by 2"]
        Div2 = 0x08,
        #[doc = "sys_ck divided by 4"]
        Div4 = 0x09,
        #[doc = "sys_ck divided by 8"]
        Div8 = 0x0a,
        #[doc = "sys_ck divided by 16"]
        Div16 = 0x0b,
        #[doc = "sys_ck divided by 64"]
        Div64 = 0x0c,
        #[doc = "sys_ck divided by 128"]
        Div128 = 0x0d,
        #[doc = "sys_ck divided by 256"]
        Div256 = 0x0e,
        #[doc = "sys_ck divided by 512"]
        Div512 = 0x0f,
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
        TimyKer = 0x0,
        #[doc = "The HRTIM prescaler clock source is the CPU clock (c_ck)"]
        CCk = 0x01,
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
        Div1 = 0x0,
        #[doc = "Division by 2"]
        Div2 = 0x01,
        #[doc = "Division by 4"]
        Div4 = 0x02,
        #[doc = "Division by 8"]
        Div8 = 0x03,
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
        Pclk1 = 0x0,
        #[doc = "pll3_r selected as peripheral clock"]
        Pll3R = 0x01,
        #[doc = "hsi_ker selected as peripheral clock"]
        Hsi = 0x02,
        #[doc = "csi_ker selected as peripheral clock"]
        Csi = 0x03,
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
        Pclk4 = 0x0,
        #[doc = "pll3_r selected as peripheral clock"]
        Pll3R = 0x01,
        #[doc = "hsi_ker selected as peripheral clock"]
        Hsi = 0x02,
        #[doc = "csi_ker selected as peripheral clock"]
        Csi = 0x03,
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
        Pclk1 = 0x0,
        #[doc = "pll2_p selected as peripheral clock"]
        Pll2P = 0x01,
        #[doc = "pll3_r selected as peripheral clock"]
        Pll3R = 0x02,
        #[doc = "LSE selected as peripheral clock"]
        Lse = 0x03,
        #[doc = "LSI selected as peripheral clock"]
        Lsi = 0x04,
        #[doc = "PER selected as peripheral clock"]
        Per = 0x05,
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
        Pclk4 = 0x0,
        #[doc = "pll2_p selected as peripheral clock"]
        Pll2P = 0x01,
        #[doc = "pll3_r selected as peripheral clock"]
        Pll3R = 0x02,
        #[doc = "LSE selected as peripheral clock"]
        Lse = 0x03,
        #[doc = "LSI selected as peripheral clock"]
        Lsi = 0x04,
        #[doc = "PER selected as peripheral clock"]
        Per = 0x05,
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
        Pclk4 = 0x0,
        #[doc = "pll2_q selected as peripheral clock"]
        Pll2Q = 0x01,
        #[doc = "pll3_q selected as peripheral clock"]
        Pll3Q = 0x02,
        #[doc = "hsi_ker selected as peripheral clock"]
        Hsi = 0x03,
        #[doc = "csi_ker selected as peripheral clock"]
        Csi = 0x04,
        #[doc = "LSE selected as peripheral clock"]
        Lse = 0x05,
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
        Low = 0x0,
        #[doc = "Medium low driving capability"]
        MediumLow = 0x01,
        #[doc = "Medium high driving capability"]
        MediumHigh = 0x02,
        #[doc = "High driving capability"]
        High = 0x03,
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
        Hsi = 0x0,
        #[doc = "LSE selected for micro-controller clock output"]
        Lse = 0x01,
        #[doc = "HSE selected for micro-controller clock output"]
        Hse = 0x02,
        #[doc = "pll1_q selected for micro-controller clock output"]
        Pll1Q = 0x03,
        #[doc = "HSI48 selected for micro-controller clock output"]
        Hsi48 = 0x04,
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
        Sys = 0x0,
        #[doc = "pll2_p selected for micro-controller clock output"]
        Pll2P = 0x01,
        #[doc = "HSE selected for micro-controller clock output"]
        Hse = 0x02,
        #[doc = "pll1_p selected for micro-controller clock output"]
        Pll1P = 0x03,
        #[doc = "CSI selected for micro-controller clock output"]
        Csi = 0x04,
        #[doc = "LSI selected for micro-controller clock output"]
        Lsi = 0x05,
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
        Div1 = 0x01,
        #[doc = "Divide by 2"]
        Div2 = 0x02,
        #[doc = "Divide by 3"]
        Div3 = 0x03,
        #[doc = "Divide by 4"]
        Div4 = 0x04,
        #[doc = "Divide by 5"]
        Div5 = 0x05,
        #[doc = "Divide by 6"]
        Div6 = 0x06,
        #[doc = "Divide by 7"]
        Div7 = 0x07,
        #[doc = "Divide by 8"]
        Div8 = 0x08,
        #[doc = "Divide by 9"]
        Div9 = 0x09,
        #[doc = "Divide by 10"]
        Div10 = 0x0a,
        #[doc = "Divide by 11"]
        Div11 = 0x0b,
        #[doc = "Divide by 12"]
        Div12 = 0x0c,
        #[doc = "Divide by 13"]
        Div13 = 0x0d,
        #[doc = "Divide by 14"]
        Div14 = 0x0e,
        #[doc = "Divide by 15"]
        Div15 = 0x0f,
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
        Hsi = 0x0,
        #[doc = "CSI selected as peripheral clock"]
        Csi = 0x01,
        #[doc = "HSE selected as peripheral clock"]
        Hse = 0x02,
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
        Div1 = 0x0,
        Div2 = 0x01,
        Div3 = 0x02,
        Div4 = 0x03,
        Div5 = 0x04,
        Div6 = 0x05,
        Div7 = 0x06,
        Div8 = 0x07,
        Div9 = 0x08,
        Div10 = 0x09,
        Div11 = 0x0a,
        Div12 = 0x0b,
        Div13 = 0x0c,
        Div14 = 0x0d,
        Div15 = 0x0e,
        Div16 = 0x0f,
        Div17 = 0x10,
        Div18 = 0x11,
        Div19 = 0x12,
        Div20 = 0x13,
        Div21 = 0x14,
        Div22 = 0x15,
        Div23 = 0x16,
        Div24 = 0x17,
        Div25 = 0x18,
        Div26 = 0x19,
        Div27 = 0x1a,
        Div28 = 0x1b,
        Div29 = 0x1c,
        Div30 = 0x1d,
        Div31 = 0x1e,
        Div32 = 0x1f,
        Div33 = 0x20,
        Div34 = 0x21,
        Div35 = 0x22,
        Div36 = 0x23,
        Div37 = 0x24,
        Div38 = 0x25,
        Div39 = 0x26,
        Div40 = 0x27,
        Div41 = 0x28,
        Div42 = 0x29,
        Div43 = 0x2a,
        Div44 = 0x2b,
        Div45 = 0x2c,
        Div46 = 0x2d,
        Div47 = 0x2e,
        Div48 = 0x2f,
        Div49 = 0x30,
        Div50 = 0x31,
        Div51 = 0x32,
        Div52 = 0x33,
        Div53 = 0x34,
        Div54 = 0x35,
        Div55 = 0x36,
        Div56 = 0x37,
        Div57 = 0x38,
        Div58 = 0x39,
        Div59 = 0x3a,
        Div60 = 0x3b,
        Div61 = 0x3c,
        Div62 = 0x3d,
        Div63 = 0x3e,
        Div64 = 0x3f,
        Div65 = 0x40,
        Div66 = 0x41,
        Div67 = 0x42,
        Div68 = 0x43,
        Div69 = 0x44,
        Div70 = 0x45,
        Div71 = 0x46,
        Div72 = 0x47,
        Div73 = 0x48,
        Div74 = 0x49,
        Div75 = 0x4a,
        Div76 = 0x4b,
        Div77 = 0x4c,
        Div78 = 0x4d,
        Div79 = 0x4e,
        Div80 = 0x4f,
        Div81 = 0x50,
        Div82 = 0x51,
        Div83 = 0x52,
        Div84 = 0x53,
        Div85 = 0x54,
        Div86 = 0x55,
        Div87 = 0x56,
        Div88 = 0x57,
        Div89 = 0x58,
        Div90 = 0x59,
        Div91 = 0x5a,
        Div92 = 0x5b,
        Div93 = 0x5c,
        Div94 = 0x5d,
        Div95 = 0x5e,
        Div96 = 0x5f,
        Div97 = 0x60,
        Div98 = 0x61,
        Div99 = 0x62,
        Div100 = 0x63,
        Div101 = 0x64,
        Div102 = 0x65,
        Div103 = 0x66,
        Div104 = 0x67,
        Div105 = 0x68,
        Div106 = 0x69,
        Div107 = 0x6a,
        Div108 = 0x6b,
        Div109 = 0x6c,
        Div110 = 0x6d,
        Div111 = 0x6e,
        Div112 = 0x6f,
        Div113 = 0x70,
        Div114 = 0x71,
        Div115 = 0x72,
        Div116 = 0x73,
        Div117 = 0x74,
        Div118 = 0x75,
        Div119 = 0x76,
        Div120 = 0x77,
        Div121 = 0x78,
        Div122 = 0x79,
        Div123 = 0x7a,
        Div124 = 0x7b,
        Div125 = 0x7c,
        Div126 = 0x7d,
        Div127 = 0x7e,
        Div128 = 0x7f,
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
        Div1 = 0x01,
        Div2 = 0x02,
        Div3 = 0x03,
        Div4 = 0x04,
        Div5 = 0x05,
        Div6 = 0x06,
        Div7 = 0x07,
        Div8 = 0x08,
        Div9 = 0x09,
        Div10 = 0x0a,
        Div11 = 0x0b,
        Div12 = 0x0c,
        Div13 = 0x0d,
        Div14 = 0x0e,
        Div15 = 0x0f,
        Div16 = 0x10,
        Div17 = 0x11,
        Div18 = 0x12,
        Div19 = 0x13,
        Div20 = 0x14,
        Div21 = 0x15,
        Div22 = 0x16,
        Div23 = 0x17,
        Div24 = 0x18,
        Div25 = 0x19,
        Div26 = 0x1a,
        Div27 = 0x1b,
        Div28 = 0x1c,
        Div29 = 0x1d,
        Div30 = 0x1e,
        Div31 = 0x1f,
        Div32 = 0x20,
        Div33 = 0x21,
        Div34 = 0x22,
        Div35 = 0x23,
        Div36 = 0x24,
        Div37 = 0x25,
        Div38 = 0x26,
        Div39 = 0x27,
        Div40 = 0x28,
        Div41 = 0x29,
        Div42 = 0x2a,
        Div43 = 0x2b,
        Div44 = 0x2c,
        Div45 = 0x2d,
        Div46 = 0x2e,
        Div47 = 0x2f,
        Div48 = 0x30,
        Div49 = 0x31,
        Div50 = 0x32,
        Div51 = 0x33,
        Div52 = 0x34,
        Div53 = 0x35,
        Div54 = 0x36,
        Div55 = 0x37,
        Div56 = 0x38,
        Div57 = 0x39,
        Div58 = 0x3a,
        Div59 = 0x3b,
        Div60 = 0x3c,
        Div61 = 0x3d,
        Div62 = 0x3e,
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
    #[repr(u16)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Plln {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        Mul4 = 0x03,
        Mul5 = 0x04,
        Mul6 = 0x05,
        Mul7 = 0x06,
        Mul8 = 0x07,
        Mul9 = 0x08,
        Mul10 = 0x09,
        Mul11 = 0x0a,
        Mul12 = 0x0b,
        Mul13 = 0x0c,
        Mul14 = 0x0d,
        Mul15 = 0x0e,
        Mul16 = 0x0f,
        Mul17 = 0x10,
        Mul18 = 0x11,
        Mul19 = 0x12,
        Mul20 = 0x13,
        Mul21 = 0x14,
        Mul22 = 0x15,
        Mul23 = 0x16,
        Mul24 = 0x17,
        Mul25 = 0x18,
        Mul26 = 0x19,
        Mul27 = 0x1a,
        Mul28 = 0x1b,
        Mul29 = 0x1c,
        Mul30 = 0x1d,
        Mul31 = 0x1e,
        Mul32 = 0x1f,
        Mul33 = 0x20,
        Mul34 = 0x21,
        Mul35 = 0x22,
        Mul36 = 0x23,
        Mul37 = 0x24,
        Mul38 = 0x25,
        Mul39 = 0x26,
        Mul40 = 0x27,
        Mul41 = 0x28,
        Mul42 = 0x29,
        Mul43 = 0x2a,
        Mul44 = 0x2b,
        Mul45 = 0x2c,
        Mul46 = 0x2d,
        Mul47 = 0x2e,
        Mul48 = 0x2f,
        Mul49 = 0x30,
        Mul50 = 0x31,
        Mul51 = 0x32,
        Mul52 = 0x33,
        Mul53 = 0x34,
        Mul54 = 0x35,
        Mul55 = 0x36,
        Mul56 = 0x37,
        Mul57 = 0x38,
        Mul58 = 0x39,
        Mul59 = 0x3a,
        Mul60 = 0x3b,
        Mul61 = 0x3c,
        Mul62 = 0x3d,
        Mul63 = 0x3e,
        Mul64 = 0x3f,
        Mul65 = 0x40,
        Mul66 = 0x41,
        Mul67 = 0x42,
        Mul68 = 0x43,
        Mul69 = 0x44,
        Mul70 = 0x45,
        Mul71 = 0x46,
        Mul72 = 0x47,
        Mul73 = 0x48,
        Mul74 = 0x49,
        Mul75 = 0x4a,
        Mul76 = 0x4b,
        Mul77 = 0x4c,
        Mul78 = 0x4d,
        Mul79 = 0x4e,
        Mul80 = 0x4f,
        Mul81 = 0x50,
        Mul82 = 0x51,
        Mul83 = 0x52,
        Mul84 = 0x53,
        Mul85 = 0x54,
        Mul86 = 0x55,
        Mul87 = 0x56,
        Mul88 = 0x57,
        Mul89 = 0x58,
        Mul90 = 0x59,
        Mul91 = 0x5a,
        Mul92 = 0x5b,
        Mul93 = 0x5c,
        Mul94 = 0x5d,
        Mul95 = 0x5e,
        Mul96 = 0x5f,
        Mul97 = 0x60,
        Mul98 = 0x61,
        Mul99 = 0x62,
        Mul100 = 0x63,
        Mul101 = 0x64,
        Mul102 = 0x65,
        Mul103 = 0x66,
        Mul104 = 0x67,
        Mul105 = 0x68,
        Mul106 = 0x69,
        Mul107 = 0x6a,
        Mul108 = 0x6b,
        Mul109 = 0x6c,
        Mul110 = 0x6d,
        Mul111 = 0x6e,
        Mul112 = 0x6f,
        Mul113 = 0x70,
        Mul114 = 0x71,
        Mul115 = 0x72,
        Mul116 = 0x73,
        Mul117 = 0x74,
        Mul118 = 0x75,
        Mul119 = 0x76,
        Mul120 = 0x77,
        Mul121 = 0x78,
        Mul122 = 0x79,
        Mul123 = 0x7a,
        Mul124 = 0x7b,
        Mul125 = 0x7c,
        Mul126 = 0x7d,
        Mul127 = 0x7e,
        Mul128 = 0x7f,
        Mul129 = 0x80,
        Mul130 = 0x81,
        Mul131 = 0x82,
        Mul132 = 0x83,
        Mul133 = 0x84,
        Mul134 = 0x85,
        Mul135 = 0x86,
        Mul136 = 0x87,
        Mul137 = 0x88,
        Mul138 = 0x89,
        Mul139 = 0x8a,
        Mul140 = 0x8b,
        Mul141 = 0x8c,
        Mul142 = 0x8d,
        Mul143 = 0x8e,
        Mul144 = 0x8f,
        Mul145 = 0x90,
        Mul146 = 0x91,
        Mul147 = 0x92,
        Mul148 = 0x93,
        Mul149 = 0x94,
        Mul150 = 0x95,
        Mul151 = 0x96,
        Mul152 = 0x97,
        Mul153 = 0x98,
        Mul154 = 0x99,
        Mul155 = 0x9a,
        Mul156 = 0x9b,
        Mul157 = 0x9c,
        Mul158 = 0x9d,
        Mul159 = 0x9e,
        Mul160 = 0x9f,
        Mul161 = 0xa0,
        Mul162 = 0xa1,
        Mul163 = 0xa2,
        Mul164 = 0xa3,
        Mul165 = 0xa4,
        Mul166 = 0xa5,
        Mul167 = 0xa6,
        Mul168 = 0xa7,
        Mul169 = 0xa8,
        Mul170 = 0xa9,
        Mul171 = 0xaa,
        Mul172 = 0xab,
        Mul173 = 0xac,
        Mul174 = 0xad,
        Mul175 = 0xae,
        Mul176 = 0xaf,
        Mul177 = 0xb0,
        Mul178 = 0xb1,
        Mul179 = 0xb2,
        Mul180 = 0xb3,
        Mul181 = 0xb4,
        Mul182 = 0xb5,
        Mul183 = 0xb6,
        Mul184 = 0xb7,
        Mul185 = 0xb8,
        Mul186 = 0xb9,
        Mul187 = 0xba,
        Mul188 = 0xbb,
        Mul189 = 0xbc,
        Mul190 = 0xbd,
        Mul191 = 0xbe,
        Mul192 = 0xbf,
        Mul193 = 0xc0,
        Mul194 = 0xc1,
        Mul195 = 0xc2,
        Mul196 = 0xc3,
        Mul197 = 0xc4,
        Mul198 = 0xc5,
        Mul199 = 0xc6,
        Mul200 = 0xc7,
        Mul201 = 0xc8,
        Mul202 = 0xc9,
        Mul203 = 0xca,
        Mul204 = 0xcb,
        Mul205 = 0xcc,
        Mul206 = 0xcd,
        Mul207 = 0xce,
        Mul208 = 0xcf,
        Mul209 = 0xd0,
        Mul210 = 0xd1,
        Mul211 = 0xd2,
        Mul212 = 0xd3,
        Mul213 = 0xd4,
        Mul214 = 0xd5,
        Mul215 = 0xd6,
        Mul216 = 0xd7,
        Mul217 = 0xd8,
        Mul218 = 0xd9,
        Mul219 = 0xda,
        Mul220 = 0xdb,
        Mul221 = 0xdc,
        Mul222 = 0xdd,
        Mul223 = 0xde,
        Mul224 = 0xdf,
        Mul225 = 0xe0,
        Mul226 = 0xe1,
        Mul227 = 0xe2,
        Mul228 = 0xe3,
        Mul229 = 0xe4,
        Mul230 = 0xe5,
        Mul231 = 0xe6,
        Mul232 = 0xe7,
        Mul233 = 0xe8,
        Mul234 = 0xe9,
        Mul235 = 0xea,
        Mul236 = 0xeb,
        Mul237 = 0xec,
        Mul238 = 0xed,
        Mul239 = 0xee,
        Mul240 = 0xef,
        Mul241 = 0xf0,
        Mul242 = 0xf1,
        Mul243 = 0xf2,
        Mul244 = 0xf3,
        Mul245 = 0xf4,
        Mul246 = 0xf5,
        Mul247 = 0xf6,
        Mul248 = 0xf7,
        Mul249 = 0xf8,
        Mul250 = 0xf9,
        Mul251 = 0xfa,
        Mul252 = 0xfb,
        Mul253 = 0xfc,
        Mul254 = 0xfd,
        Mul255 = 0xfe,
        Mul256 = 0xff,
        Mul257 = 0x0100,
        Mul258 = 0x0101,
        Mul259 = 0x0102,
        Mul260 = 0x0103,
        Mul261 = 0x0104,
        Mul262 = 0x0105,
        Mul263 = 0x0106,
        Mul264 = 0x0107,
        Mul265 = 0x0108,
        Mul266 = 0x0109,
        Mul267 = 0x010a,
        Mul268 = 0x010b,
        Mul269 = 0x010c,
        Mul270 = 0x010d,
        Mul271 = 0x010e,
        Mul272 = 0x010f,
        Mul273 = 0x0110,
        Mul274 = 0x0111,
        Mul275 = 0x0112,
        Mul276 = 0x0113,
        Mul277 = 0x0114,
        Mul278 = 0x0115,
        Mul279 = 0x0116,
        Mul280 = 0x0117,
        Mul281 = 0x0118,
        Mul282 = 0x0119,
        Mul283 = 0x011a,
        Mul284 = 0x011b,
        Mul285 = 0x011c,
        Mul286 = 0x011d,
        Mul287 = 0x011e,
        Mul288 = 0x011f,
        Mul289 = 0x0120,
        Mul290 = 0x0121,
        Mul291 = 0x0122,
        Mul292 = 0x0123,
        Mul293 = 0x0124,
        Mul294 = 0x0125,
        Mul295 = 0x0126,
        Mul296 = 0x0127,
        Mul297 = 0x0128,
        Mul298 = 0x0129,
        Mul299 = 0x012a,
        Mul300 = 0x012b,
        Mul301 = 0x012c,
        Mul302 = 0x012d,
        Mul303 = 0x012e,
        Mul304 = 0x012f,
        Mul305 = 0x0130,
        Mul306 = 0x0131,
        Mul307 = 0x0132,
        Mul308 = 0x0133,
        Mul309 = 0x0134,
        Mul310 = 0x0135,
        Mul311 = 0x0136,
        Mul312 = 0x0137,
        Mul313 = 0x0138,
        Mul314 = 0x0139,
        Mul315 = 0x013a,
        Mul316 = 0x013b,
        Mul317 = 0x013c,
        Mul318 = 0x013d,
        Mul319 = 0x013e,
        Mul320 = 0x013f,
        Mul321 = 0x0140,
        Mul322 = 0x0141,
        Mul323 = 0x0142,
        Mul324 = 0x0143,
        Mul325 = 0x0144,
        Mul326 = 0x0145,
        Mul327 = 0x0146,
        Mul328 = 0x0147,
        Mul329 = 0x0148,
        Mul330 = 0x0149,
        Mul331 = 0x014a,
        Mul332 = 0x014b,
        Mul333 = 0x014c,
        Mul334 = 0x014d,
        Mul335 = 0x014e,
        Mul336 = 0x014f,
        Mul337 = 0x0150,
        Mul338 = 0x0151,
        Mul339 = 0x0152,
        Mul340 = 0x0153,
        Mul341 = 0x0154,
        Mul342 = 0x0155,
        Mul343 = 0x0156,
        Mul344 = 0x0157,
        Mul345 = 0x0158,
        Mul346 = 0x0159,
        Mul347 = 0x015a,
        Mul348 = 0x015b,
        Mul349 = 0x015c,
        Mul350 = 0x015d,
        Mul351 = 0x015e,
        Mul352 = 0x015f,
        Mul353 = 0x0160,
        Mul354 = 0x0161,
        Mul355 = 0x0162,
        Mul356 = 0x0163,
        Mul357 = 0x0164,
        Mul358 = 0x0165,
        Mul359 = 0x0166,
        Mul360 = 0x0167,
        Mul361 = 0x0168,
        Mul362 = 0x0169,
        Mul363 = 0x016a,
        Mul364 = 0x016b,
        Mul365 = 0x016c,
        Mul366 = 0x016d,
        Mul367 = 0x016e,
        Mul368 = 0x016f,
        Mul369 = 0x0170,
        Mul370 = 0x0171,
        Mul371 = 0x0172,
        Mul372 = 0x0173,
        Mul373 = 0x0174,
        Mul374 = 0x0175,
        Mul375 = 0x0176,
        Mul376 = 0x0177,
        Mul377 = 0x0178,
        Mul378 = 0x0179,
        Mul379 = 0x017a,
        Mul380 = 0x017b,
        Mul381 = 0x017c,
        Mul382 = 0x017d,
        Mul383 = 0x017e,
        Mul384 = 0x017f,
        Mul385 = 0x0180,
        Mul386 = 0x0181,
        Mul387 = 0x0182,
        Mul388 = 0x0183,
        Mul389 = 0x0184,
        Mul390 = 0x0185,
        Mul391 = 0x0186,
        Mul392 = 0x0187,
        Mul393 = 0x0188,
        Mul394 = 0x0189,
        Mul395 = 0x018a,
        Mul396 = 0x018b,
        Mul397 = 0x018c,
        Mul398 = 0x018d,
        Mul399 = 0x018e,
        Mul400 = 0x018f,
        Mul401 = 0x0190,
        Mul402 = 0x0191,
        Mul403 = 0x0192,
        Mul404 = 0x0193,
        Mul405 = 0x0194,
        Mul406 = 0x0195,
        Mul407 = 0x0196,
        Mul408 = 0x0197,
        Mul409 = 0x0198,
        Mul410 = 0x0199,
        Mul411 = 0x019a,
        Mul412 = 0x019b,
        Mul413 = 0x019c,
        Mul414 = 0x019d,
        Mul415 = 0x019e,
        Mul416 = 0x019f,
        Mul417 = 0x01a0,
        Mul418 = 0x01a1,
        Mul419 = 0x01a2,
        Mul420 = 0x01a3,
        Mul421 = 0x01a4,
        Mul422 = 0x01a5,
        Mul423 = 0x01a6,
        Mul424 = 0x01a7,
        Mul425 = 0x01a8,
        Mul426 = 0x01a9,
        Mul427 = 0x01aa,
        Mul428 = 0x01ab,
        Mul429 = 0x01ac,
        Mul430 = 0x01ad,
        Mul431 = 0x01ae,
        Mul432 = 0x01af,
        Mul433 = 0x01b0,
        Mul434 = 0x01b1,
        Mul435 = 0x01b2,
        Mul436 = 0x01b3,
        Mul437 = 0x01b4,
        Mul438 = 0x01b5,
        Mul439 = 0x01b6,
        Mul440 = 0x01b7,
        Mul441 = 0x01b8,
        Mul442 = 0x01b9,
        Mul443 = 0x01ba,
        Mul444 = 0x01bb,
        Mul445 = 0x01bc,
        Mul446 = 0x01bd,
        Mul447 = 0x01be,
        Mul448 = 0x01bf,
        Mul449 = 0x01c0,
        Mul450 = 0x01c1,
        Mul451 = 0x01c2,
        Mul452 = 0x01c3,
        Mul453 = 0x01c4,
        Mul454 = 0x01c5,
        Mul455 = 0x01c6,
        Mul456 = 0x01c7,
        Mul457 = 0x01c8,
        Mul458 = 0x01c9,
        Mul459 = 0x01ca,
        Mul460 = 0x01cb,
        Mul461 = 0x01cc,
        Mul462 = 0x01cd,
        Mul463 = 0x01ce,
        Mul464 = 0x01cf,
        Mul465 = 0x01d0,
        Mul466 = 0x01d1,
        Mul467 = 0x01d2,
        Mul468 = 0x01d3,
        Mul469 = 0x01d4,
        Mul470 = 0x01d5,
        Mul471 = 0x01d6,
        Mul472 = 0x01d7,
        Mul473 = 0x01d8,
        Mul474 = 0x01d9,
        Mul475 = 0x01da,
        Mul476 = 0x01db,
        Mul477 = 0x01dc,
        Mul478 = 0x01dd,
        Mul479 = 0x01de,
        Mul480 = 0x01df,
        Mul481 = 0x01e0,
        Mul482 = 0x01e1,
        Mul483 = 0x01e2,
        Mul484 = 0x01e3,
        Mul485 = 0x01e4,
        Mul486 = 0x01e5,
        Mul487 = 0x01e6,
        Mul488 = 0x01e7,
        Mul489 = 0x01e8,
        Mul490 = 0x01e9,
        Mul491 = 0x01ea,
        Mul492 = 0x01eb,
        Mul493 = 0x01ec,
        Mul494 = 0x01ed,
        Mul495 = 0x01ee,
        Mul496 = 0x01ef,
        Mul497 = 0x01f0,
        Mul498 = 0x01f1,
        Mul499 = 0x01f2,
        Mul500 = 0x01f3,
        Mul501 = 0x01f4,
        Mul502 = 0x01f5,
        Mul503 = 0x01f6,
        Mul504 = 0x01f7,
        Mul505 = 0x01f8,
        Mul506 = 0x01f9,
        Mul507 = 0x01fa,
        Mul508 = 0x01fb,
        Mul509 = 0x01fc,
        Mul510 = 0x01fd,
        Mul511 = 0x01fe,
        Mul512 = 0x01ff,
    }
    impl Plln {
        #[inline(always)]
        pub const fn from_bits(val: u16) -> Plln {
            unsafe { core::mem::transmute(val & 0x01ff) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u16 {
            unsafe { core::mem::transmute(self) }
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
        Range1 = 0x0,
        #[doc = "Frequency is between 2 and 4 MHz"]
        Range2 = 0x01,
        #[doc = "Frequency is between 4 and 8 MHz"]
        Range4 = 0x02,
        #[doc = "Frequency is between 8 and 16 MHz"]
        Range8 = 0x03,
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
        Hsi = 0x0,
        #[doc = "CSI selected as PLL clock"]
        Csi = 0x01,
        #[doc = "HSE selected as PLL clock"]
        Hse = 0x02,
        #[doc = "No clock sent to DIVMx dividers and PLLs"]
        Disable = 0x03,
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
        WideVco = 0x0,
        #[doc = "VCO frequency range 150 to 420 MHz"]
        MediumVco = 0x01,
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
        Div1 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "rcc_hclk divided by 2"]
        Div2 = 0x04,
        #[doc = "rcc_hclk divided by 4"]
        Div4 = 0x05,
        #[doc = "rcc_hclk divided by 8"]
        Div8 = 0x06,
        #[doc = "rcc_hclk divided by 16"]
        Div16 = 0x07,
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
        Hsi48 = 0x0,
        #[doc = "pll1_q selected as peripheral clock"]
        Pll1Q = 0x01,
        #[doc = "LSE selected as peripheral clock"]
        Lse = 0x02,
        #[doc = "LSI selected as peripheral clock"]
        Lsi = 0x03,
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
        Disable = 0x0,
        #[doc = "LSE oscillator clock used as RTC clock"]
        Lse = 0x01,
        #[doc = "LSI oscillator clock used as RTC clock"]
        Lsi = 0x02,
        #[doc = "HSE oscillator clock divided by a prescaler used as RTC clock"]
        Hse = 0x03,
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
        Pll1Q = 0x0,
        #[doc = "pll2_p selected as peripheral clock"]
        Pll2P = 0x01,
        #[doc = "pll3_p selected as peripheral clock"]
        Pll3P = 0x02,
        #[doc = "i2s_ckin selected as peripheral clock"]
        I2sCkin = 0x03,
        #[doc = "PER selected as peripheral clock"]
        Per = 0x04,
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
        Pll1Q = 0x0,
        #[doc = "pll2_p selected as peripheral clock"]
        Pll2P = 0x01,
        #[doc = "pll3_p selected as peripheral clock"]
        Pll3P = 0x02,
        #[doc = "I2S_CKIN selected as peripheral clock"]
        I2sCkin = 0x03,
        #[doc = "PER selected as peripheral clock"]
        Per = 0x04,
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
        Pll1Q = 0x0,
        #[doc = "pll2_r selected as peripheral clock"]
        Pll2R = 0x01,
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
        Pll1Q = 0x0,
        #[doc = "pll2_r selected as peripheral clock"]
        Pll2R = 0x01,
        #[doc = "pll3_r selected as peripheral clock"]
        Pll3R = 0x02,
        #[doc = "hsi_ker selected as peripheral clock"]
        Hsi = 0x03,
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
        Pclk2 = 0x0,
        #[doc = "pll2_q selected as peripheral clock"]
        Pll2Q = 0x01,
        #[doc = "pll3_q selected as peripheral clock"]
        Pll3Q = 0x02,
        #[doc = "hsi_ker selected as peripheral clock"]
        Hsi = 0x03,
        #[doc = "csi_ker selected as peripheral clock"]
        Csi = 0x04,
        #[doc = "HSE selected as peripheral clock"]
        Hse = 0x05,
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
        Pclk4 = 0x0,
        #[doc = "pll2_q selected as peripheral clock"]
        Pll2Q = 0x01,
        #[doc = "pll3_q selected as peripheral clock"]
        Pll3Q = 0x02,
        #[doc = "hsi_ker selected as peripheral clock"]
        Hsi = 0x03,
        #[doc = "csi_ker selected as peripheral clock"]
        Csi = 0x04,
        #[doc = "HSE selected as peripheral clock"]
        Hse = 0x05,
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
        Hsi = 0x0,
        #[doc = "CSI selected as wake up clock from system Stop"]
        Csi = 0x01,
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
        Hsi = 0x0,
        #[doc = "CSI selected as system clock"]
        Csi = 0x01,
        #[doc = "HSE selected as system clock"]
        Hse = 0x02,
        #[doc = "PLL1 selected as system clock"]
        Pll1P = 0x03,
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
        Pclk1 = 0x0,
        #[doc = "hsi_ker selected as peripheral clock"]
        Hsi = 0x01,
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
        DefaultX2 = 0x0,
        #[doc = "Timer kernel clock equal to 4x pclk by default"]
        DefaultX4 = 0x01,
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
        Pclk2 = 0x0,
        #[doc = "pll2_q selected as peripheral clock"]
        Pll2Q = 0x01,
        #[doc = "pll3_q selected as peripheral clock"]
        Pll3Q = 0x02,
        #[doc = "hsi_ker selected as peripheral clock"]
        Hsi = 0x03,
        #[doc = "csi_ker selected as peripheral clock"]
        Csi = 0x04,
        #[doc = "LSE selected as peripheral clock"]
        Lse = 0x05,
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
        Pclk1 = 0x0,
        #[doc = "pll2_q selected as peripheral clock"]
        Pll2Q = 0x01,
        #[doc = "pll3_q selected as peripheral clock"]
        Pll3Q = 0x02,
        #[doc = "hsi_ker selected as peripheral clock"]
        Hsi = 0x03,
        #[doc = "csi_ker selected as peripheral clock"]
        Csi = 0x04,
        #[doc = "LSE selected as peripheral clock"]
        Lse = 0x05,
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
        Disable = 0x0,
        #[doc = "pll1_q selected as peripheral clock"]
        Pll1Q = 0x01,
        #[doc = "pll3_q selected as peripheral clock"]
        Pll3Q = 0x02,
        #[doc = "HSI48 selected as peripheral clock"]
        Hsi48 = 0x03,
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
