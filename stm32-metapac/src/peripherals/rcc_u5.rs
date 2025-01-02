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
    #[doc = "RCC clock control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "RCC internal clock sources calibration register 1"]
    #[inline(always)]
    pub const fn icscr1(self) -> crate::common::Reg<regs::Icscr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "RCC internal clock sources calibration register 2"]
    #[inline(always)]
    pub const fn icscr2(self) -> crate::common::Reg<regs::Icscr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "RCC internal clock sources calibration register 3"]
    #[inline(always)]
    pub const fn icscr3(self) -> crate::common::Reg<regs::Icscr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "RCC clock recovery RC register"]
    #[inline(always)]
    pub const fn crrcr(self) -> crate::common::Reg<regs::Crrcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "RCC clock configuration register 1"]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::Cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "RCC clock configuration register 2"]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "RCC clock configuration register 3"]
    #[inline(always)]
    pub const fn cfgr3(self) -> crate::common::Reg<regs::Cfgr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "RCC PLL1 configuration register"]
    #[inline(always)]
    pub const fn pll1cfgr(self) -> crate::common::Reg<regs::Pll1cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "RCC PLL2 configuration register"]
    #[inline(always)]
    pub const fn pll2cfgr(self) -> crate::common::Reg<regs::Pll23cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "RCC PLL3 configuration register"]
    #[inline(always)]
    pub const fn pll3cfgr(self) -> crate::common::Reg<regs::Pll23cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "RCC PLL1 dividers register"]
    #[inline(always)]
    pub const fn pll1divr(self) -> crate::common::Reg<regs::Plldivr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "RCC PLL1 fractional divider register"]
    #[inline(always)]
    pub const fn pll1fracr(self) -> crate::common::Reg<regs::Pllfracr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "RCC PLL2 dividers configuration register"]
    #[inline(always)]
    pub const fn pll2divr(self) -> crate::common::Reg<regs::Plldivr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "RCC PLL2 fractional divider register"]
    #[inline(always)]
    pub const fn pll2fracr(self) -> crate::common::Reg<regs::Pllfracr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "RCC PLL3 dividers configuration register"]
    #[inline(always)]
    pub const fn pll3divr(self) -> crate::common::Reg<regs::Plldivr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "RCC PLL3 fractional divider register"]
    #[inline(always)]
    pub const fn pll3fracr(self) -> crate::common::Reg<regs::Pllfracr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "RCC clock interrupt enable register"]
    #[inline(always)]
    pub const fn cier(self) -> crate::common::Reg<regs::Cier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "RCC clock interrupt flag register"]
    #[inline(always)]
    pub const fn cifr(self) -> crate::common::Reg<regs::Cifr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "RCC clock interrupt clear register"]
    #[inline(always)]
    pub const fn cicr(self) -> crate::common::Reg<regs::Cicr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "RCC AHB1 peripheral reset register"]
    #[inline(always)]
    pub const fn ahb1rstr(self) -> crate::common::Reg<regs::Ahb1rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "RCC AHB2 peripheral reset register 1"]
    #[inline(always)]
    pub const fn ahb2rstr1(self) -> crate::common::Reg<regs::Ahb2rstr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "RCC AHB2 peripheral reset register 2"]
    #[inline(always)]
    pub const fn ahb2rstr2(self) -> crate::common::Reg<regs::Ahb2rstr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "RCC AHB3 peripheral reset register"]
    #[inline(always)]
    pub const fn ahb3rstr(self) -> crate::common::Reg<regs::Ahb3rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "RCC APB1 peripheral reset register 1"]
    #[inline(always)]
    pub const fn apb1rstr1(self) -> crate::common::Reg<regs::Apb1rstr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "RCC APB1 peripheral reset register 2"]
    #[inline(always)]
    pub const fn apb1rstr2(self) -> crate::common::Reg<regs::Apb1rstr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "RCC APB2 peripheral reset register"]
    #[inline(always)]
    pub const fn apb2rstr(self) -> crate::common::Reg<regs::Apb2rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "RCC APB3 peripheral reset register"]
    #[inline(always)]
    pub const fn apb3rstr(self) -> crate::common::Reg<regs::Apb3rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "RCC AHB1 peripheral clock enable register"]
    #[inline(always)]
    pub const fn ahb1enr(self) -> crate::common::Reg<regs::Ahb1enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "RCC AHB2 peripheral clock enable register 1"]
    #[inline(always)]
    pub const fn ahb2enr1(self) -> crate::common::Reg<regs::Ahb2enr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "RCC AHB2 peripheral clock enable register 2"]
    #[inline(always)]
    pub const fn ahb2enr2(self) -> crate::common::Reg<regs::Ahb2enr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "RCC AHB3 peripheral clock enable register"]
    #[inline(always)]
    pub const fn ahb3enr(self) -> crate::common::Reg<regs::Ahb3enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "RCC APB1 peripheral clock enable register 1"]
    #[inline(always)]
    pub const fn apb1enr1(self) -> crate::common::Reg<regs::Apb1enr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "RCC APB1 peripheral clock enable register 2"]
    #[inline(always)]
    pub const fn apb1enr2(self) -> crate::common::Reg<regs::Apb1enr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "RCC APB2 peripheral clock enable register"]
    #[inline(always)]
    pub const fn apb2enr(self) -> crate::common::Reg<regs::Apb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "RCC APB3 peripheral clock enable register"]
    #[inline(always)]
    pub const fn apb3enr(self) -> crate::common::Reg<regs::Apb3enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "RCC AHB1 peripheral clocks enable in Sleep and Stop modes register"]
    #[inline(always)]
    pub const fn ahb1smenr(self) -> crate::common::Reg<regs::Ahb1smenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "RCC AHB2 peripheral clocks enable in Sleep and Stop modes register 1"]
    #[inline(always)]
    pub const fn ahb2smenr1(self) -> crate::common::Reg<regs::Ahb2smenr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[doc = "RCC AHB2 peripheral clocks enable in Sleep and Stop modes register 2"]
    #[inline(always)]
    pub const fn ahb2smenr2(self) -> crate::common::Reg<regs::Ahb2smenr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[doc = "RCC AHB3 peripheral clocks enable in Sleep and Stop modes register"]
    #[inline(always)]
    pub const fn ahb3smenr(self) -> crate::common::Reg<regs::Ahb3smenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[doc = "RCC APB1 peripheral clocks enable in Sleep and Stop modes register 1"]
    #[inline(always)]
    pub const fn apb1smenr1(self) -> crate::common::Reg<regs::Apb1smenr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "RCC APB1 peripheral clocks enable in Sleep and Stop modes register 2"]
    #[inline(always)]
    pub const fn apb1smenr2(self) -> crate::common::Reg<regs::Apb1smenr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[doc = "RCC APB2 peripheral clocks enable in Sleep and Stop modes register"]
    #[inline(always)]
    pub const fn apb2smenr(self) -> crate::common::Reg<regs::Apb2smenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[doc = "RCC APB3 peripheral clock enable in Sleep and Stop modes register"]
    #[inline(always)]
    pub const fn apb3smenr(self) -> crate::common::Reg<regs::Apb3smenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[doc = "RCC SmartRun domain peripheral autonomous mode register"]
    #[inline(always)]
    pub const fn srdamr(self) -> crate::common::Reg<regs::Srdamr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[doc = "RCC peripherals independent clock configuration register 1"]
    #[inline(always)]
    pub const fn ccipr1(self) -> crate::common::Reg<regs::Ccipr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[doc = "RCC peripherals independent clock configuration register 2"]
    #[inline(always)]
    pub const fn ccipr2(self) -> crate::common::Reg<regs::Ccipr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
    }
    #[doc = "RCC peripherals independent clock configuration register 3"]
    #[inline(always)]
    pub const fn ccipr3(self) -> crate::common::Reg<regs::Ccipr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe8usize) as _) }
    }
    #[doc = "RCC Backup domain control register"]
    #[inline(always)]
    pub const fn bdcr(self) -> crate::common::Reg<regs::Bdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[doc = "RCC control/status register"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[doc = "RCC secure configuration register"]
    #[inline(always)]
    pub const fn seccfgr(self) -> crate::common::Reg<regs::Seccfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "RCC privilege configuration register"]
    #[inline(always)]
    pub const fn privcfgr(self) -> crate::common::Reg<regs::Privcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
}
pub mod regs {
    #[doc = "RCC AHB1 peripheral clock enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1enr(pub u32);
    impl Ahb1enr {
        #[doc = "GPDMA1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn gpdma1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpdma1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CORDIC clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn cordicen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CORDIC clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_cordicen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "FMAC clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn fmacen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "FMAC clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_fmacen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "MDF1 clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn mdf1en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "MDF1 clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_mdf1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "FLASH clock enable Set and cleared by software. This bit can be disabled only when the Flash memory is in power down mode."]
        #[inline(always)]
        pub const fn flashen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FLASH clock enable Set and cleared by software. This bit can be disabled only when the Flash memory is in power down mode."]
        #[inline(always)]
        pub fn set_flashen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CRC clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn crcen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_crcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "JPEG clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn jpegen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "JPEG clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_jpegen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Touch sensing controller clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn tscen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Touch sensing controller clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_tscen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "RAMCFG clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn ramcfgen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "RAMCFG clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_ramcfgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "DMA2D clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn dma2den(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2D clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_dma2den(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "GFXMMU clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn gfxmmuen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "GFXMMU clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_gfxmmuen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "GPU2D clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn gpu2den(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "GPU2D clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_gpu2den(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "DCACHE2 clock enable This bit is set and reset by software. Note: DCACHE2 clock must be enabled to access memories, even if the DCACHE2 is bypassed. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn dcache2en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "DCACHE2 clock enable This bit is set and reset by software. Note: DCACHE2 clock must be enabled to access memories, even if the DCACHE2 is bypassed. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_dcache2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "GTZC1 clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn gtzc1en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "GTZC1 clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_gtzc1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "BKPSRAM clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn bkpsramen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "BKPSRAM clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_bkpsramen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "DCACHE1 clock enable Set and reset by software. Note: DCACHE1 clock must be enabled when external memories are accessed through OCTOSPI1, OCTOSPI2 or FSMC, even if the DCACHE1 is bypassed."]
        #[inline(always)]
        pub const fn dcache1en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "DCACHE1 clock enable Set and reset by software. Note: DCACHE1 clock must be enabled when external memories are accessed through OCTOSPI1, OCTOSPI2 or FSMC, even if the DCACHE1 is bypassed."]
        #[inline(always)]
        pub fn set_dcache1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SRAM1 clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn sram1en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_sram1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("gpdma1en", &self.gpdma1en())
                .field("cordicen", &self.cordicen())
                .field("fmacen", &self.fmacen())
                .field("mdf1en", &self.mdf1en())
                .field("flashen", &self.flashen())
                .field("crcen", &self.crcen())
                .field("jpegen", &self.jpegen())
                .field("tscen", &self.tscen())
                .field("ramcfgen", &self.ramcfgen())
                .field("dma2den", &self.dma2den())
                .field("gfxmmuen", &self.gfxmmuen())
                .field("gpu2den", &self.gpu2den())
                .field("dcache2en", &self.dcache2en())
                .field("gtzc1en", &self.gtzc1en())
                .field("bkpsramen", &self.bkpsramen())
                .field("dcache1en", &self.dcache1en())
                .field("sram1en", &self.sram1en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1enr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb1enr {
                gpdma1en: bool,
                cordicen: bool,
                fmacen: bool,
                mdf1en: bool,
                flashen: bool,
                crcen: bool,
                jpegen: bool,
                tscen: bool,
                ramcfgen: bool,
                dma2den: bool,
                gfxmmuen: bool,
                gpu2den: bool,
                dcache2en: bool,
                gtzc1en: bool,
                bkpsramen: bool,
                dcache1en: bool,
                sram1en: bool,
            }
            let proxy = Ahb1enr {
                gpdma1en: self.gpdma1en(),
                cordicen: self.cordicen(),
                fmacen: self.fmacen(),
                mdf1en: self.mdf1en(),
                flashen: self.flashen(),
                crcen: self.crcen(),
                jpegen: self.jpegen(),
                tscen: self.tscen(),
                ramcfgen: self.ramcfgen(),
                dma2den: self.dma2den(),
                gfxmmuen: self.gfxmmuen(),
                gpu2den: self.gpu2den(),
                dcache2en: self.dcache2en(),
                gtzc1en: self.gtzc1en(),
                bkpsramen: self.bkpsramen(),
                dcache1en: self.dcache1en(),
                sram1en: self.sram1en(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC AHB1 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1rstr(pub u32);
    impl Ahb1rstr {
        #[doc = "GPDMA1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn gpdma1rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpdma1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CORDIC reset Set and cleared by software."]
        #[inline(always)]
        pub const fn cordicrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CORDIC reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_cordicrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "FMAC reset Set and cleared by software."]
        #[inline(always)]
        pub const fn fmacrst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "FMAC reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_fmacrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "MDF1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn mdf1rst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "MDF1 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_mdf1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CRC reset Set and cleared by software."]
        #[inline(always)]
        pub const fn crcrst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_crcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "JPEG reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn jpegrst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "JPEG reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_jpegrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "TSC reset Set and cleared by software."]
        #[inline(always)]
        pub const fn tscrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TSC reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_tscrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "RAMCFG reset Set and cleared by software."]
        #[inline(always)]
        pub const fn ramcfgrst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "RAMCFG reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_ramcfgrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "DMA2D reset Set and cleared by software."]
        #[inline(always)]
        pub const fn dma2drst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2D reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_dma2drst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "GFXMMU reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn gfxmmurst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "GFXMMU reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_gfxmmurst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "GPU2D reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn gpu2drst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "GPU2D reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_gpu2drst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
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
                .field("gpdma1rst", &self.gpdma1rst())
                .field("cordicrst", &self.cordicrst())
                .field("fmacrst", &self.fmacrst())
                .field("mdf1rst", &self.mdf1rst())
                .field("crcrst", &self.crcrst())
                .field("jpegrst", &self.jpegrst())
                .field("tscrst", &self.tscrst())
                .field("ramcfgrst", &self.ramcfgrst())
                .field("dma2drst", &self.dma2drst())
                .field("gfxmmurst", &self.gfxmmurst())
                .field("gpu2drst", &self.gpu2drst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1rstr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb1rstr {
                gpdma1rst: bool,
                cordicrst: bool,
                fmacrst: bool,
                mdf1rst: bool,
                crcrst: bool,
                jpegrst: bool,
                tscrst: bool,
                ramcfgrst: bool,
                dma2drst: bool,
                gfxmmurst: bool,
                gpu2drst: bool,
            }
            let proxy = Ahb1rstr {
                gpdma1rst: self.gpdma1rst(),
                cordicrst: self.cordicrst(),
                fmacrst: self.fmacrst(),
                mdf1rst: self.mdf1rst(),
                crcrst: self.crcrst(),
                jpegrst: self.jpegrst(),
                tscrst: self.tscrst(),
                ramcfgrst: self.ramcfgrst(),
                dma2drst: self.dma2drst(),
                gfxmmurst: self.gfxmmurst(),
                gpu2drst: self.gpu2drst(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC AHB1 peripheral clocks enable in Sleep and Stop modes register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1smenr(pub u32);
    impl Ahb1smenr {
        #[doc = "GPDMA1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn gpdma1smen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_gpdma1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CORDIC clocks enable during Sleep and Stop modes Set and cleared by software during Sleep mode."]
        #[inline(always)]
        pub const fn cordicsmen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CORDIC clocks enable during Sleep and Stop modes Set and cleared by software during Sleep mode."]
        #[inline(always)]
        pub fn set_cordicsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "FMAC clocks enable during Sleep and Stop modes. Set and cleared by software."]
        #[inline(always)]
        pub const fn fmacsmen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "FMAC clocks enable during Sleep and Stop modes. Set and cleared by software."]
        #[inline(always)]
        pub fn set_fmacsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "MDF1 clocks enable during Sleep and Stop modes. Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn mdf1smen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "MDF1 clocks enable during Sleep and Stop modes. Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_mdf1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "FLASH clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn flashsmen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FLASH clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_flashsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CRC clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn crcsmen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_crcsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "JPEG clocks enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn jpegsmen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "JPEG clocks enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_jpegsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "TSC clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn tscsmen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TSC clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_tscsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "RAMCFG clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn ramcfgsmen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "RAMCFG clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_ramcfgsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "DMA2D clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn dma2dsmen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2D clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_dma2dsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "GFXMMU clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn gfxmmusmen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "GFXMMU clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_gfxmmusmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "GPU2D clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn gpu2dsmen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "GPU2D clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_gpu2dsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "DCACHE2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn dcache2smen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "DCACHE2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_dcache2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "GTZC1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn gtzc1smen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "GTZC1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_gtzc1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "BKPSRAM clocks enable during Sleep and Stop modes Set and cleared by software"]
        #[inline(always)]
        pub const fn bkpsramsmen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "BKPSRAM clocks enable during Sleep and Stop modes Set and cleared by software"]
        #[inline(always)]
        pub fn set_bkpsramsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "ICACHE clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn icachesmen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "ICACHE clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_icachesmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "DCACHE1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn dcache1smen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "DCACHE1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_dcache1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SRAM1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn sram1smen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_sram1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ahb1smenr {
        #[inline(always)]
        fn default() -> Ahb1smenr {
            Ahb1smenr(0)
        }
    }
    impl core::fmt::Debug for Ahb1smenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb1smenr")
                .field("gpdma1smen", &self.gpdma1smen())
                .field("cordicsmen", &self.cordicsmen())
                .field("fmacsmen", &self.fmacsmen())
                .field("mdf1smen", &self.mdf1smen())
                .field("flashsmen", &self.flashsmen())
                .field("crcsmen", &self.crcsmen())
                .field("jpegsmen", &self.jpegsmen())
                .field("tscsmen", &self.tscsmen())
                .field("ramcfgsmen", &self.ramcfgsmen())
                .field("dma2dsmen", &self.dma2dsmen())
                .field("gfxmmusmen", &self.gfxmmusmen())
                .field("gpu2dsmen", &self.gpu2dsmen())
                .field("dcache2smen", &self.dcache2smen())
                .field("gtzc1smen", &self.gtzc1smen())
                .field("bkpsramsmen", &self.bkpsramsmen())
                .field("icachesmen", &self.icachesmen())
                .field("dcache1smen", &self.dcache1smen())
                .field("sram1smen", &self.sram1smen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1smenr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb1smenr {
                gpdma1smen: bool,
                cordicsmen: bool,
                fmacsmen: bool,
                mdf1smen: bool,
                flashsmen: bool,
                crcsmen: bool,
                jpegsmen: bool,
                tscsmen: bool,
                ramcfgsmen: bool,
                dma2dsmen: bool,
                gfxmmusmen: bool,
                gpu2dsmen: bool,
                dcache2smen: bool,
                gtzc1smen: bool,
                bkpsramsmen: bool,
                icachesmen: bool,
                dcache1smen: bool,
                sram1smen: bool,
            }
            let proxy = Ahb1smenr {
                gpdma1smen: self.gpdma1smen(),
                cordicsmen: self.cordicsmen(),
                fmacsmen: self.fmacsmen(),
                mdf1smen: self.mdf1smen(),
                flashsmen: self.flashsmen(),
                crcsmen: self.crcsmen(),
                jpegsmen: self.jpegsmen(),
                tscsmen: self.tscsmen(),
                ramcfgsmen: self.ramcfgsmen(),
                dma2dsmen: self.dma2dsmen(),
                gfxmmusmen: self.gfxmmusmen(),
                gpu2dsmen: self.gpu2dsmen(),
                dcache2smen: self.dcache2smen(),
                gtzc1smen: self.gtzc1smen(),
                bkpsramsmen: self.bkpsramsmen(),
                icachesmen: self.icachesmen(),
                dcache1smen: self.dcache1smen(),
                sram1smen: self.sram1smen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC AHB2 peripheral clock enable register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2enr1(pub u32);
    impl Ahb2enr1 {
        #[doc = "IO port A clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn gpioaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IO port A clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpioaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IO port B clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn gpioben(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IO port B clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpioben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IO port C clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn gpiocen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IO port C clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpiocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "IO port D clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn gpioden(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IO port D clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpioden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "IO port E clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn gpioeen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IO port E clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpioeen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "IO port F clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn gpiofen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "IO port F clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpiofen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "IO port G clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn gpiogen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "IO port G clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpiogen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "IO port H clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn gpiohen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "IO port H clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpiohen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "IO port I clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn gpioien(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "IO port I clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpioien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "I/O port J clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn gpiojen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port J clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_gpiojen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "ADC1 and ADC2 clock enable This bit is set and cleared by software. Note: This bit impacts ADC1 in STM32U535/545/575/585, and ADC1/ADC2 in�STM32U59x/5Ax/5Fx/5Gx."]
        #[inline(always)]
        pub const fn adc12en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1 and ADC2 clock enable This bit is set and cleared by software. Note: This bit impacts ADC1 in STM32U535/545/575/585, and ADC1/ADC2 in�STM32U59x/5Ax/5Fx/5Gx."]
        #[inline(always)]
        pub fn set_adc12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "DCMI and PSSI clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn dcmien(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "DCMI and PSSI clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_dcmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "OTG_FS clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn usb_otg_fsen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "OTG_FS clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_usb_otg_fsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "OTG_HS clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn usb_otg_hsen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "OTG_HS clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_usb_otg_hsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "OTG_HS PHY clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn usb_otg_hs_phyen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "OTG_HS PHY clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_usb_otg_hs_phyen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "AES clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn aesen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "AES clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_aesen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "HASH clock enable Set and cleared by software"]
        #[inline(always)]
        pub const fn hashen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "HASH clock enable Set and cleared by software"]
        #[inline(always)]
        pub fn set_hashen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "RNG clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn rngen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "RNG clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_rngen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PKA clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn pkaen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "PKA clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_pkaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "SAES clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn saesen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SAES clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_saesen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "OCTOSPIM clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn octospimen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPIM clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_octospimen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "OTFDEC1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn otfdec1en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_otfdec1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "OTFDEC2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn otfdec2en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_otfdec2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "SDMMC1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn sdmmc1en(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_sdmmc1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "SDMMC2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn sdmmc2en(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_sdmmc2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "SRAM2 clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn sram2en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_sram2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SRAM3 clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn sram3en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM3 clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_sram3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ahb2enr1 {
        #[inline(always)]
        fn default() -> Ahb2enr1 {
            Ahb2enr1(0)
        }
    }
    impl core::fmt::Debug for Ahb2enr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb2enr1")
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
                .field("adc12en", &self.adc12en())
                .field("dcmien", &self.dcmien())
                .field("usb_otg_fsen", &self.usb_otg_fsen())
                .field("usb_otg_hsen", &self.usb_otg_hsen())
                .field("usb_otg_hs_phyen", &self.usb_otg_hs_phyen())
                .field("aesen", &self.aesen())
                .field("hashen", &self.hashen())
                .field("rngen", &self.rngen())
                .field("pkaen", &self.pkaen())
                .field("saesen", &self.saesen())
                .field("octospimen", &self.octospimen())
                .field("otfdec1en", &self.otfdec1en())
                .field("otfdec2en", &self.otfdec2en())
                .field("sdmmc1en", &self.sdmmc1en())
                .field("sdmmc2en", &self.sdmmc2en())
                .field("sram2en", &self.sram2en())
                .field("sram3en", &self.sram3en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2enr1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb2enr1 {
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
                adc12en: bool,
                dcmien: bool,
                usb_otg_fsen: bool,
                usb_otg_hsen: bool,
                usb_otg_hs_phyen: bool,
                aesen: bool,
                hashen: bool,
                rngen: bool,
                pkaen: bool,
                saesen: bool,
                octospimen: bool,
                otfdec1en: bool,
                otfdec2en: bool,
                sdmmc1en: bool,
                sdmmc2en: bool,
                sram2en: bool,
                sram3en: bool,
            }
            let proxy = Ahb2enr1 {
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
                adc12en: self.adc12en(),
                dcmien: self.dcmien(),
                usb_otg_fsen: self.usb_otg_fsen(),
                usb_otg_hsen: self.usb_otg_hsen(),
                usb_otg_hs_phyen: self.usb_otg_hs_phyen(),
                aesen: self.aesen(),
                hashen: self.hashen(),
                rngen: self.rngen(),
                pkaen: self.pkaen(),
                saesen: self.saesen(),
                octospimen: self.octospimen(),
                otfdec1en: self.otfdec1en(),
                otfdec2en: self.otfdec2en(),
                sdmmc1en: self.sdmmc1en(),
                sdmmc2en: self.sdmmc2en(),
                sram2en: self.sram2en(),
                sram3en: self.sram3en(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC AHB2 peripheral clock enable register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2enr2(pub u32);
    impl Ahb2enr2 {
        #[doc = "FSMC clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn fsmcen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "FSMC clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_fsmcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "OCTOSPI1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn octospi1en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_octospi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "OCTOSPI2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn octospi2en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_octospi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "HSPI1 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn hspi1en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "HSPI1 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_hspi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SRAM6 clock enable This bit is set and reset by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn sram6en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM6 clock enable This bit is set and reset by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_sram6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SRAM5 clock enable This bit is set and reset by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn sram5en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM5 clock enable This bit is set and reset by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_sram5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ahb2enr2 {
        #[inline(always)]
        fn default() -> Ahb2enr2 {
            Ahb2enr2(0)
        }
    }
    impl core::fmt::Debug for Ahb2enr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb2enr2")
                .field("fsmcen", &self.fsmcen())
                .field("octospi1en", &self.octospi1en())
                .field("octospi2en", &self.octospi2en())
                .field("hspi1en", &self.hspi1en())
                .field("sram6en", &self.sram6en())
                .field("sram5en", &self.sram5en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2enr2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb2enr2 {
                fsmcen: bool,
                octospi1en: bool,
                octospi2en: bool,
                hspi1en: bool,
                sram6en: bool,
                sram5en: bool,
            }
            let proxy = Ahb2enr2 {
                fsmcen: self.fsmcen(),
                octospi1en: self.octospi1en(),
                octospi2en: self.octospi2en(),
                hspi1en: self.hspi1en(),
                sram6en: self.sram6en(),
                sram5en: self.sram5en(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC AHB2 peripheral reset register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2rstr1(pub u32);
    impl Ahb2rstr1 {
        #[doc = "IO port A reset Set and cleared by software."]
        #[inline(always)]
        pub const fn gpioarst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IO port A reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpioarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IO port B reset Set and cleared by software."]
        #[inline(always)]
        pub const fn gpiobrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IO port B reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpiobrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IO port C reset Set and cleared by software."]
        #[inline(always)]
        pub const fn gpiocrst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IO port C reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpiocrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "IO port D reset Set and cleared by software."]
        #[inline(always)]
        pub const fn gpiodrst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IO port D reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpiodrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "IO port E reset Set and cleared by software."]
        #[inline(always)]
        pub const fn gpioerst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IO port E reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpioerst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "IO port F reset Set and cleared by software."]
        #[inline(always)]
        pub const fn gpiofrst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "IO port F reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpiofrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "IO port G reset Set and cleared by software."]
        #[inline(always)]
        pub const fn gpiogrst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "IO port G reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpiogrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "IO port H reset Set and cleared by software."]
        #[inline(always)]
        pub const fn gpiohrst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "IO port H reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpiohrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "IO port I reset Set and cleared by software."]
        #[inline(always)]
        pub const fn gpioirst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "IO port I reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpioirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "I/O port J reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn gpiojrst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port J reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_gpiojrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "ADC1 and ADC2 reset This bit is set and cleared by software. Note: This bit impacts ADC1 in STM32U535/545/575/585, and ADC1/ADC2 in�STM32U59x/5Ax/5Fx/5Gx."]
        #[inline(always)]
        pub const fn adc12rst(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1 and ADC2 reset This bit is set and cleared by software. Note: This bit impacts ADC1 in STM32U535/545/575/585, and ADC1/ADC2 in�STM32U59x/5Ax/5Fx/5Gx."]
        #[inline(always)]
        pub fn set_adc12rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "DCMI and PSSI reset Set and cleared by software."]
        #[inline(always)]
        pub const fn dcmirst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "DCMI and PSSI reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_dcmirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "OTG_FS reset Set and cleared by software."]
        #[inline(always)]
        pub const fn usb_otg_fsrst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "OTG_FS reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_usb_otg_fsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "OTG_HS reset Set and cleared by software."]
        #[inline(always)]
        pub const fn usb_otg_hsrst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "OTG_HS reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_usb_otg_hsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "AES hardware accelerator reset Set and cleared by software."]
        #[inline(always)]
        pub const fn aesrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "AES hardware accelerator reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_aesrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Hash reset Set and cleared by software."]
        #[inline(always)]
        pub const fn hashrst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Hash reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_hashrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Random number generator reset Set and cleared by software."]
        #[inline(always)]
        pub const fn rngrst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Random number generator reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_rngrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PKA reset Set and cleared by software."]
        #[inline(always)]
        pub const fn pkarst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "PKA reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_pkarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "SAES hardware accelerator reset Set and cleared by software."]
        #[inline(always)]
        pub const fn saesrst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SAES hardware accelerator reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_saesrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "OCTOSPIM reset Set and cleared by software."]
        #[inline(always)]
        pub const fn octospimrst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPIM reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_octospimrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "OTFDEC1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn otfdec1rst(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC1 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_otfdec1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "OTFDEC2 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn otfdec2rst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC2 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_otfdec2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "SDMMC1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn sdmmc1rst(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC1 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_sdmmc1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "SDMMC2 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn sdmmc2rst(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC2 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_sdmmc2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
    }
    impl Default for Ahb2rstr1 {
        #[inline(always)]
        fn default() -> Ahb2rstr1 {
            Ahb2rstr1(0)
        }
    }
    impl core::fmt::Debug for Ahb2rstr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb2rstr1")
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
                .field("adc12rst", &self.adc12rst())
                .field("dcmirst", &self.dcmirst())
                .field("usb_otg_fsrst", &self.usb_otg_fsrst())
                .field("usb_otg_hsrst", &self.usb_otg_hsrst())
                .field("aesrst", &self.aesrst())
                .field("hashrst", &self.hashrst())
                .field("rngrst", &self.rngrst())
                .field("pkarst", &self.pkarst())
                .field("saesrst", &self.saesrst())
                .field("octospimrst", &self.octospimrst())
                .field("otfdec1rst", &self.otfdec1rst())
                .field("otfdec2rst", &self.otfdec2rst())
                .field("sdmmc1rst", &self.sdmmc1rst())
                .field("sdmmc2rst", &self.sdmmc2rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2rstr1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb2rstr1 {
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
                adc12rst: bool,
                dcmirst: bool,
                usb_otg_fsrst: bool,
                usb_otg_hsrst: bool,
                aesrst: bool,
                hashrst: bool,
                rngrst: bool,
                pkarst: bool,
                saesrst: bool,
                octospimrst: bool,
                otfdec1rst: bool,
                otfdec2rst: bool,
                sdmmc1rst: bool,
                sdmmc2rst: bool,
            }
            let proxy = Ahb2rstr1 {
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
                adc12rst: self.adc12rst(),
                dcmirst: self.dcmirst(),
                usb_otg_fsrst: self.usb_otg_fsrst(),
                usb_otg_hsrst: self.usb_otg_hsrst(),
                aesrst: self.aesrst(),
                hashrst: self.hashrst(),
                rngrst: self.rngrst(),
                pkarst: self.pkarst(),
                saesrst: self.saesrst(),
                octospimrst: self.octospimrst(),
                otfdec1rst: self.otfdec1rst(),
                otfdec2rst: self.otfdec2rst(),
                sdmmc1rst: self.sdmmc1rst(),
                sdmmc2rst: self.sdmmc2rst(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC AHB2 peripheral reset register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2rstr2(pub u32);
    impl Ahb2rstr2 {
        #[doc = "Flexible memory controller reset Set and cleared by software."]
        #[inline(always)]
        pub const fn fsmcrst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Flexible memory controller reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_fsmcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "OCTOSPI1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn octospi1rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI1 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_octospi1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "OCTOSPI2 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn octospi2rst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI2 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_octospi2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "HSPI1 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn hspi1rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "HSPI1 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_hspi1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Ahb2rstr2 {
        #[inline(always)]
        fn default() -> Ahb2rstr2 {
            Ahb2rstr2(0)
        }
    }
    impl core::fmt::Debug for Ahb2rstr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb2rstr2")
                .field("fsmcrst", &self.fsmcrst())
                .field("octospi1rst", &self.octospi1rst())
                .field("octospi2rst", &self.octospi2rst())
                .field("hspi1rst", &self.hspi1rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2rstr2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb2rstr2 {
                fsmcrst: bool,
                octospi1rst: bool,
                octospi2rst: bool,
                hspi1rst: bool,
            }
            let proxy = Ahb2rstr2 {
                fsmcrst: self.fsmcrst(),
                octospi1rst: self.octospi1rst(),
                octospi2rst: self.octospi2rst(),
                hspi1rst: self.hspi1rst(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC AHB2 peripheral clocks enable in Sleep and Stop modes register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2smenr1(pub u32);
    impl Ahb2smenr1 {
        #[doc = "IO port A clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn gpioasmen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IO port A clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpioasmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IO port B clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn gpiobsmen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IO port B clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpiobsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IO port C clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn gpiocsmen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IO port C clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpiocsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "IO port D clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn gpiodsmen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IO port D clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpiodsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "IO port E clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn gpioesmen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IO port E clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpioesmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "IO port F clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn gpiofsmen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "IO port F clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpiofsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "IO port G clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn gpiogsmen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "IO port G clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpiogsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "IO port H clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn gpiohsmen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "IO port H clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpiohsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "IO port I clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn gpioismen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "IO port I clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_gpioismen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "I/O port J clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn gpiojsmen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port J clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_gpiojsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "ADC1 and ADC2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit impacts ADC1 in STM32U535/545/575/585 and ADC1/ADC2 in�STM32U59x/5Ax/5Fx/5Gx."]
        #[inline(always)]
        pub const fn adc12smen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1 and ADC2 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit impacts ADC1 in STM32U535/545/575/585 and ADC1/ADC2 in�STM32U59x/5Ax/5Fx/5Gx."]
        #[inline(always)]
        pub fn set_adc12smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "DCMI and PSSI clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn dcmismen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "DCMI and PSSI clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_dcmismen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "OTG_FS clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn usb_otg_fssmen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "OTG_FS clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_usb_otg_fssmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "OTG_HS clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn usb_otg_hssmen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "OTG_HS clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_usb_otg_hssmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "OTG_HS PHY clock enable during Sleep and Stop modes This bit is set and cleared by software Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn usb_otg_hs_physmen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "OTG_HS PHY clock enable during Sleep and Stop modes This bit is set and cleared by software Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_usb_otg_hs_physmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "AES clock enable during Sleep and Stop modes Set and cleared by software"]
        #[inline(always)]
        pub const fn aessmen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "AES clock enable during Sleep and Stop modes Set and cleared by software"]
        #[inline(always)]
        pub fn set_aessmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "HASH clock enable during Sleep and Stop modes Set and cleared by software"]
        #[inline(always)]
        pub const fn hashsmen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "HASH clock enable during Sleep and Stop modes Set and cleared by software"]
        #[inline(always)]
        pub fn set_hashsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Random number generator (RNG) clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn rngsmen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Random number generator (RNG) clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_rngsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PKA clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn pkasmen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "PKA clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_pkasmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "SAES accelerator clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn saessmen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SAES accelerator clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_saessmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "OCTOSPIM clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn octospimsmen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPIM clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_octospimsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "OTFDEC1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn otfdec1smen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_otfdec1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "OTFDEC2 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn otfdec2smen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC2 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_otfdec2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "SDMMC1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn sdmmc1smen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_sdmmc1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "SDMMC2 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn sdmmc2smen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC2 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_sdmmc2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "SRAM2 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn sram2smen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_sram2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SRAM3 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn sram3smen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM3 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_sram3smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ahb2smenr1 {
        #[inline(always)]
        fn default() -> Ahb2smenr1 {
            Ahb2smenr1(0)
        }
    }
    impl core::fmt::Debug for Ahb2smenr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb2smenr1")
                .field("gpioasmen", &self.gpioasmen())
                .field("gpiobsmen", &self.gpiobsmen())
                .field("gpiocsmen", &self.gpiocsmen())
                .field("gpiodsmen", &self.gpiodsmen())
                .field("gpioesmen", &self.gpioesmen())
                .field("gpiofsmen", &self.gpiofsmen())
                .field("gpiogsmen", &self.gpiogsmen())
                .field("gpiohsmen", &self.gpiohsmen())
                .field("gpioismen", &self.gpioismen())
                .field("gpiojsmen", &self.gpiojsmen())
                .field("adc12smen", &self.adc12smen())
                .field("dcmismen", &self.dcmismen())
                .field("usb_otg_fssmen", &self.usb_otg_fssmen())
                .field("usb_otg_hssmen", &self.usb_otg_hssmen())
                .field("usb_otg_hs_physmen", &self.usb_otg_hs_physmen())
                .field("aessmen", &self.aessmen())
                .field("hashsmen", &self.hashsmen())
                .field("rngsmen", &self.rngsmen())
                .field("pkasmen", &self.pkasmen())
                .field("saessmen", &self.saessmen())
                .field("octospimsmen", &self.octospimsmen())
                .field("otfdec1smen", &self.otfdec1smen())
                .field("otfdec2smen", &self.otfdec2smen())
                .field("sdmmc1smen", &self.sdmmc1smen())
                .field("sdmmc2smen", &self.sdmmc2smen())
                .field("sram2smen", &self.sram2smen())
                .field("sram3smen", &self.sram3smen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2smenr1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb2smenr1 {
                gpioasmen: bool,
                gpiobsmen: bool,
                gpiocsmen: bool,
                gpiodsmen: bool,
                gpioesmen: bool,
                gpiofsmen: bool,
                gpiogsmen: bool,
                gpiohsmen: bool,
                gpioismen: bool,
                gpiojsmen: bool,
                adc12smen: bool,
                dcmismen: bool,
                usb_otg_fssmen: bool,
                usb_otg_hssmen: bool,
                usb_otg_hs_physmen: bool,
                aessmen: bool,
                hashsmen: bool,
                rngsmen: bool,
                pkasmen: bool,
                saessmen: bool,
                octospimsmen: bool,
                otfdec1smen: bool,
                otfdec2smen: bool,
                sdmmc1smen: bool,
                sdmmc2smen: bool,
                sram2smen: bool,
                sram3smen: bool,
            }
            let proxy = Ahb2smenr1 {
                gpioasmen: self.gpioasmen(),
                gpiobsmen: self.gpiobsmen(),
                gpiocsmen: self.gpiocsmen(),
                gpiodsmen: self.gpiodsmen(),
                gpioesmen: self.gpioesmen(),
                gpiofsmen: self.gpiofsmen(),
                gpiogsmen: self.gpiogsmen(),
                gpiohsmen: self.gpiohsmen(),
                gpioismen: self.gpioismen(),
                gpiojsmen: self.gpiojsmen(),
                adc12smen: self.adc12smen(),
                dcmismen: self.dcmismen(),
                usb_otg_fssmen: self.usb_otg_fssmen(),
                usb_otg_hssmen: self.usb_otg_hssmen(),
                usb_otg_hs_physmen: self.usb_otg_hs_physmen(),
                aessmen: self.aessmen(),
                hashsmen: self.hashsmen(),
                rngsmen: self.rngsmen(),
                pkasmen: self.pkasmen(),
                saessmen: self.saessmen(),
                octospimsmen: self.octospimsmen(),
                otfdec1smen: self.otfdec1smen(),
                otfdec2smen: self.otfdec2smen(),
                sdmmc1smen: self.sdmmc1smen(),
                sdmmc2smen: self.sdmmc2smen(),
                sram2smen: self.sram2smen(),
                sram3smen: self.sram3smen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC AHB2 peripheral clocks enable in Sleep and Stop modes register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2smenr2(pub u32);
    impl Ahb2smenr2 {
        #[doc = "FSMC clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn fsmcsmen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "FSMC clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_fsmcsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "OCTOSPI1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn octospi1smen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_octospi1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "OCTOSPI2 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn octospi2smen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "OCTOSPI2 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_octospi2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "HSPI1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn hspi1smen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "HSPI1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_hspi1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SRAM6 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn sram6smen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM6 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_sram6smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SRAM5 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn sram5smen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM5 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_sram5smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ahb2smenr2 {
        #[inline(always)]
        fn default() -> Ahb2smenr2 {
            Ahb2smenr2(0)
        }
    }
    impl core::fmt::Debug for Ahb2smenr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb2smenr2")
                .field("fsmcsmen", &self.fsmcsmen())
                .field("octospi1smen", &self.octospi1smen())
                .field("octospi2smen", &self.octospi2smen())
                .field("hspi1smen", &self.hspi1smen())
                .field("sram6smen", &self.sram6smen())
                .field("sram5smen", &self.sram5smen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2smenr2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb2smenr2 {
                fsmcsmen: bool,
                octospi1smen: bool,
                octospi2smen: bool,
                hspi1smen: bool,
                sram6smen: bool,
                sram5smen: bool,
            }
            let proxy = Ahb2smenr2 {
                fsmcsmen: self.fsmcsmen(),
                octospi1smen: self.octospi1smen(),
                octospi2smen: self.octospi2smen(),
                hspi1smen: self.hspi1smen(),
                sram6smen: self.sram6smen(),
                sram5smen: self.sram5smen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC AHB3 peripheral clock enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3enr(pub u32);
    impl Ahb3enr {
        #[doc = "LPGPIO1 enable Set and cleared by software."]
        #[inline(always)]
        pub const fn lpgpio1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LPGPIO1 enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_lpgpio1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "PWR clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn pwren(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PWR clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_pwren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ADC4 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn adc4en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ADC4 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_adc4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "DAC1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn dac1en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_dac1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LPDMA1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn lpdma1en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPDMA1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_lpdma1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "ADF1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn adf1en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADF1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_adf1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "GTZC2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn gtzc2en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "GTZC2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_gtzc2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SRAM4 clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn sram4en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM4 clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_sram4en(&mut self, val: bool) {
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
                .field("lpgpio1en", &self.lpgpio1en())
                .field("pwren", &self.pwren())
                .field("adc4en", &self.adc4en())
                .field("dac1en", &self.dac1en())
                .field("lpdma1en", &self.lpdma1en())
                .field("adf1en", &self.adf1en())
                .field("gtzc2en", &self.gtzc2en())
                .field("sram4en", &self.sram4en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb3enr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb3enr {
                lpgpio1en: bool,
                pwren: bool,
                adc4en: bool,
                dac1en: bool,
                lpdma1en: bool,
                adf1en: bool,
                gtzc2en: bool,
                sram4en: bool,
            }
            let proxy = Ahb3enr {
                lpgpio1en: self.lpgpio1en(),
                pwren: self.pwren(),
                adc4en: self.adc4en(),
                dac1en: self.dac1en(),
                lpdma1en: self.lpdma1en(),
                adf1en: self.adf1en(),
                gtzc2en: self.gtzc2en(),
                sram4en: self.sram4en(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC AHB3 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3rstr(pub u32);
    impl Ahb3rstr {
        #[doc = "LPGPIO1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn lpgpio1rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LPGPIO1 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_lpgpio1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ADC4 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn adc4rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ADC4 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_adc4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "DAC1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn dac1rst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_dac1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LPDMA1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn lpdma1rst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPDMA1 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_lpdma1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "ADF1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn adf1rst(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADF1 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_adf1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
                .field("lpgpio1rst", &self.lpgpio1rst())
                .field("adc4rst", &self.adc4rst())
                .field("dac1rst", &self.dac1rst())
                .field("lpdma1rst", &self.lpdma1rst())
                .field("adf1rst", &self.adf1rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb3rstr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb3rstr {
                lpgpio1rst: bool,
                adc4rst: bool,
                dac1rst: bool,
                lpdma1rst: bool,
                adf1rst: bool,
            }
            let proxy = Ahb3rstr {
                lpgpio1rst: self.lpgpio1rst(),
                adc4rst: self.adc4rst(),
                dac1rst: self.dac1rst(),
                lpdma1rst: self.lpdma1rst(),
                adf1rst: self.adf1rst(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC AHB3 peripheral clocks enable in Sleep and Stop modes register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3smenr(pub u32);
    impl Ahb3smenr {
        #[doc = "LPGPIO1 enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn lpgpio1smen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LPGPIO1 enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_lpgpio1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "PWR clock enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn pwrsmen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PWR clock enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_pwrsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ADC4 clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn adc4smen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ADC4 clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_adc4smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "DAC1 clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn dac1smen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1 clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_dac1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LPDMA1 clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn lpdma1smen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPDMA1 clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_lpdma1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "ADF1 clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn adf1smen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADF1 clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_adf1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "GTZC2 clock enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn gtzc2smen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "GTZC2 clock enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_gtzc2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SRAM4 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn sram4smen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM4 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_sram4smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ahb3smenr {
        #[inline(always)]
        fn default() -> Ahb3smenr {
            Ahb3smenr(0)
        }
    }
    impl core::fmt::Debug for Ahb3smenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb3smenr")
                .field("lpgpio1smen", &self.lpgpio1smen())
                .field("pwrsmen", &self.pwrsmen())
                .field("adc4smen", &self.adc4smen())
                .field("dac1smen", &self.dac1smen())
                .field("lpdma1smen", &self.lpdma1smen())
                .field("adf1smen", &self.adf1smen())
                .field("gtzc2smen", &self.gtzc2smen())
                .field("sram4smen", &self.sram4smen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb3smenr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb3smenr {
                lpgpio1smen: bool,
                pwrsmen: bool,
                adc4smen: bool,
                dac1smen: bool,
                lpdma1smen: bool,
                adf1smen: bool,
                gtzc2smen: bool,
                sram4smen: bool,
            }
            let proxy = Ahb3smenr {
                lpgpio1smen: self.lpgpio1smen(),
                pwrsmen: self.pwrsmen(),
                adc4smen: self.adc4smen(),
                dac1smen: self.dac1smen(),
                lpdma1smen: self.lpdma1smen(),
                adf1smen: self.adf1smen(),
                gtzc2smen: self.gtzc2smen(),
                sram4smen: self.sram4smen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB1 peripheral clock enable register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1enr1(pub u32);
    impl Apb1enr1 {
        #[doc = "TIM2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn tim2en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn tim3en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn tim4en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn tim5en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn tim6en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn tim7en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "WWDG clock enable Set by software to enable the window watchdog clock. Reset by hardware system reset. This bit can also be set by hardware if the WWDG_SW option bit is reset."]
        #[inline(always)]
        pub const fn wwdgen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG clock enable Set by software to enable the window watchdog clock. Reset by hardware system reset. This bit can also be set by hardware if the WWDG_SW option bit is reset."]
        #[inline(always)]
        pub fn set_wwdgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn spi2en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_spi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "USART2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn usart2en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_usart2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn usart3en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_usart3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn uart4en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_uart4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn uart5en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_uart5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn i2c1en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_i2c1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn i2c2en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_i2c2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "CRS clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn crsen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "CRS clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_crsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "USART6 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn usart6en(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "USART6 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_usart6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Apb1enr1 {
        #[inline(always)]
        fn default() -> Apb1enr1 {
            Apb1enr1(0)
        }
    }
    impl core::fmt::Debug for Apb1enr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1enr1")
                .field("tim2en", &self.tim2en())
                .field("tim3en", &self.tim3en())
                .field("tim4en", &self.tim4en())
                .field("tim5en", &self.tim5en())
                .field("tim6en", &self.tim6en())
                .field("tim7en", &self.tim7en())
                .field("wwdgen", &self.wwdgen())
                .field("spi2en", &self.spi2en())
                .field("usart2en", &self.usart2en())
                .field("usart3en", &self.usart3en())
                .field("uart4en", &self.uart4en())
                .field("uart5en", &self.uart5en())
                .field("i2c1en", &self.i2c1en())
                .field("i2c2en", &self.i2c2en())
                .field("crsen", &self.crsen())
                .field("usart6en", &self.usart6en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1enr1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb1enr1 {
                tim2en: bool,
                tim3en: bool,
                tim4en: bool,
                tim5en: bool,
                tim6en: bool,
                tim7en: bool,
                wwdgen: bool,
                spi2en: bool,
                usart2en: bool,
                usart3en: bool,
                uart4en: bool,
                uart5en: bool,
                i2c1en: bool,
                i2c2en: bool,
                crsen: bool,
                usart6en: bool,
            }
            let proxy = Apb1enr1 {
                tim2en: self.tim2en(),
                tim3en: self.tim3en(),
                tim4en: self.tim4en(),
                tim5en: self.tim5en(),
                tim6en: self.tim6en(),
                tim7en: self.tim7en(),
                wwdgen: self.wwdgen(),
                spi2en: self.spi2en(),
                usart2en: self.usart2en(),
                usart3en: self.usart3en(),
                uart4en: self.uart4en(),
                uart5en: self.uart5en(),
                i2c1en: self.i2c1en(),
                i2c2en: self.i2c2en(),
                crsen: self.crsen(),
                usart6en: self.usart6en(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB1 peripheral clock enable register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1enr2(pub u32);
    impl Apb1enr2 {
        #[doc = "I2C4 clock enable Set and cleared by software"]
        #[inline(always)]
        pub const fn i2c4en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 clock enable Set and cleared by software"]
        #[inline(always)]
        pub fn set_i2c4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPTIM2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn lptim2en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_lptim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "I2C5 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn i2c5en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "I2C5 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_i2c5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I2C6 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn i2c6en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C6 clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_i2c6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "FDCAN1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn fdcan1en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_fdcan1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "UCPD1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn ucpd1en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "UCPD1 clock enable Set and cleared by software."]
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
    impl core::fmt::Debug for Apb1enr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1enr2")
                .field("i2c4en", &self.i2c4en())
                .field("lptim2en", &self.lptim2en())
                .field("i2c5en", &self.i2c5en())
                .field("i2c6en", &self.i2c6en())
                .field("fdcan1en", &self.fdcan1en())
                .field("ucpd1en", &self.ucpd1en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1enr2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb1enr2 {
                i2c4en: bool,
                lptim2en: bool,
                i2c5en: bool,
                i2c6en: bool,
                fdcan1en: bool,
                ucpd1en: bool,
            }
            let proxy = Apb1enr2 {
                i2c4en: self.i2c4en(),
                lptim2en: self.lptim2en(),
                i2c5en: self.i2c5en(),
                i2c6en: self.i2c6en(),
                fdcan1en: self.fdcan1en(),
                ucpd1en: self.ucpd1en(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB1 peripheral reset register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1rstr1(pub u32);
    impl Apb1rstr1 {
        #[doc = "TIM2 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn tim2rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn tim3rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn tim4rst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn tim5rst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn tim6rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn tim7rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim7rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SPI2 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn spi2rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_spi2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "USART2 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn usart2rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_usart2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn usart3rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_usart3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn uart4rst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_uart4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn uart5rst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_uart5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn i2c1rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_i2c1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn i2c2rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_i2c2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "CRS reset Set and cleared by software."]
        #[inline(always)]
        pub const fn crsrst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "CRS reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_crsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "USART6 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn usart6rst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "USART6 reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_usart6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Apb1rstr1 {
        #[inline(always)]
        fn default() -> Apb1rstr1 {
            Apb1rstr1(0)
        }
    }
    impl core::fmt::Debug for Apb1rstr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1rstr1")
                .field("tim2rst", &self.tim2rst())
                .field("tim3rst", &self.tim3rst())
                .field("tim4rst", &self.tim4rst())
                .field("tim5rst", &self.tim5rst())
                .field("tim6rst", &self.tim6rst())
                .field("tim7rst", &self.tim7rst())
                .field("spi2rst", &self.spi2rst())
                .field("usart2rst", &self.usart2rst())
                .field("usart3rst", &self.usart3rst())
                .field("uart4rst", &self.uart4rst())
                .field("uart5rst", &self.uart5rst())
                .field("i2c1rst", &self.i2c1rst())
                .field("i2c2rst", &self.i2c2rst())
                .field("crsrst", &self.crsrst())
                .field("usart6rst", &self.usart6rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1rstr1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb1rstr1 {
                tim2rst: bool,
                tim3rst: bool,
                tim4rst: bool,
                tim5rst: bool,
                tim6rst: bool,
                tim7rst: bool,
                spi2rst: bool,
                usart2rst: bool,
                usart3rst: bool,
                uart4rst: bool,
                uart5rst: bool,
                i2c1rst: bool,
                i2c2rst: bool,
                crsrst: bool,
                usart6rst: bool,
            }
            let proxy = Apb1rstr1 {
                tim2rst: self.tim2rst(),
                tim3rst: self.tim3rst(),
                tim4rst: self.tim4rst(),
                tim5rst: self.tim5rst(),
                tim6rst: self.tim6rst(),
                tim7rst: self.tim7rst(),
                spi2rst: self.spi2rst(),
                usart2rst: self.usart2rst(),
                usart3rst: self.usart3rst(),
                uart4rst: self.uart4rst(),
                uart5rst: self.uart5rst(),
                i2c1rst: self.i2c1rst(),
                i2c2rst: self.i2c2rst(),
                crsrst: self.crsrst(),
                usart6rst: self.usart6rst(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB1 peripheral reset register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1rstr2(pub u32);
    impl Apb1rstr2 {
        #[doc = "I2C4 reset Set and cleared by software"]
        #[inline(always)]
        pub const fn i2c4rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 reset Set and cleared by software"]
        #[inline(always)]
        pub fn set_i2c4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPTIM2 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn lptim2rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_lptim2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "I2C5 reset This bit is set and cleared by software Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn i2c5rst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "I2C5 reset This bit is set and cleared by software Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_i2c5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I2C6 reset This bit is set and cleared by software Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn i2c6rst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C6 reset This bit is set and cleared by software Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_i2c6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "FDCAN1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn fdcan1rst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN1 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_fdcan1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "UCPD1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn ucpd1rst(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "UCPD1 reset Set and cleared by software."]
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
    impl core::fmt::Debug for Apb1rstr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1rstr2")
                .field("i2c4rst", &self.i2c4rst())
                .field("lptim2rst", &self.lptim2rst())
                .field("i2c5rst", &self.i2c5rst())
                .field("i2c6rst", &self.i2c6rst())
                .field("fdcan1rst", &self.fdcan1rst())
                .field("ucpd1rst", &self.ucpd1rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1rstr2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb1rstr2 {
                i2c4rst: bool,
                lptim2rst: bool,
                i2c5rst: bool,
                i2c6rst: bool,
                fdcan1rst: bool,
                ucpd1rst: bool,
            }
            let proxy = Apb1rstr2 {
                i2c4rst: self.i2c4rst(),
                lptim2rst: self.lptim2rst(),
                i2c5rst: self.i2c5rst(),
                i2c6rst: self.i2c6rst(),
                fdcan1rst: self.fdcan1rst(),
                ucpd1rst: self.ucpd1rst(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB1 peripheral clocks enable in Sleep and Stop modes register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1smenr1(pub u32);
    impl Apb1smenr1 {
        #[doc = "TIM2 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn tim2smen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn tim3smen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim3smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn tim4smen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim4smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn tim5smen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim5smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn tim6smen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim6smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn tim7smen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim7smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Window watchdog clocks enable during Sleep and Stop modes Set and cleared by software. This bit is forced to 1 by hardware when the hardware WWDG option is activated."]
        #[inline(always)]
        pub const fn wwdgsmen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog clocks enable during Sleep and Stop modes Set and cleared by software. This bit is forced to 1 by hardware when the hardware WWDG option is activated."]
        #[inline(always)]
        pub fn set_wwdgsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI2 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn spi2smen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_spi2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "USART2 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn usart2smen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_usart2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn usart3smen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_usart3smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn uart4smen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_uart4smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn uart5smen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_uart5smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn i2c1smen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_i2c1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn i2c2smen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_i2c2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "CRS clock enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn crssmen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "CRS clock enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_crssmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "USART6 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn usart6smen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "USART6 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_usart6smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Apb1smenr1 {
        #[inline(always)]
        fn default() -> Apb1smenr1 {
            Apb1smenr1(0)
        }
    }
    impl core::fmt::Debug for Apb1smenr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1smenr1")
                .field("tim2smen", &self.tim2smen())
                .field("tim3smen", &self.tim3smen())
                .field("tim4smen", &self.tim4smen())
                .field("tim5smen", &self.tim5smen())
                .field("tim6smen", &self.tim6smen())
                .field("tim7smen", &self.tim7smen())
                .field("wwdgsmen", &self.wwdgsmen())
                .field("spi2smen", &self.spi2smen())
                .field("usart2smen", &self.usart2smen())
                .field("usart3smen", &self.usart3smen())
                .field("uart4smen", &self.uart4smen())
                .field("uart5smen", &self.uart5smen())
                .field("i2c1smen", &self.i2c1smen())
                .field("i2c2smen", &self.i2c2smen())
                .field("crssmen", &self.crssmen())
                .field("usart6smen", &self.usart6smen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1smenr1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb1smenr1 {
                tim2smen: bool,
                tim3smen: bool,
                tim4smen: bool,
                tim5smen: bool,
                tim6smen: bool,
                tim7smen: bool,
                wwdgsmen: bool,
                spi2smen: bool,
                usart2smen: bool,
                usart3smen: bool,
                uart4smen: bool,
                uart5smen: bool,
                i2c1smen: bool,
                i2c2smen: bool,
                crssmen: bool,
                usart6smen: bool,
            }
            let proxy = Apb1smenr1 {
                tim2smen: self.tim2smen(),
                tim3smen: self.tim3smen(),
                tim4smen: self.tim4smen(),
                tim5smen: self.tim5smen(),
                tim6smen: self.tim6smen(),
                tim7smen: self.tim7smen(),
                wwdgsmen: self.wwdgsmen(),
                spi2smen: self.spi2smen(),
                usart2smen: self.usart2smen(),
                usart3smen: self.usart3smen(),
                uart4smen: self.uart4smen(),
                uart5smen: self.uart5smen(),
                i2c1smen: self.i2c1smen(),
                i2c2smen: self.i2c2smen(),
                crssmen: self.crssmen(),
                usart6smen: self.usart6smen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB1 peripheral clocks enable in Sleep and Stop modes register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1smenr2(pub u32);
    impl Apb1smenr2 {
        #[doc = "I2C4 clocks enable during Sleep and Stop modes Set and cleared by software Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn i2c4smen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 clocks enable during Sleep and Stop modes Set and cleared by software Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_i2c4smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPTIM2 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn lptim2smen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_lptim2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "I2C5 clock enable during Sleep and Stop modes This bit is set and cleared by software Note: This bit must be set to allow the peripheral to wake up from Stop modes. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn i2c5smen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "I2C5 clock enable during Sleep and Stop modes This bit is set and cleared by software Note: This bit must be set to allow the peripheral to wake up from Stop modes. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_i2c5smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I2C6 clock enable during Sleep and Stop modes This bit is set and cleared by software Note: This bit must be set to allow the peripheral to wake up from Stop modes. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn i2c6smen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C6 clock enable during Sleep and Stop modes This bit is set and cleared by software Note: This bit must be set to allow the peripheral to wake up from Stop modes. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_i2c6smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "FDCAN1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn fdcan1smen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_fdcan1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "UCPD1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn ucpd1smen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "UCPD1 clocks enable during Sleep and Stop modes Set and cleared by software."]
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
    impl core::fmt::Debug for Apb1smenr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1smenr2")
                .field("i2c4smen", &self.i2c4smen())
                .field("lptim2smen", &self.lptim2smen())
                .field("i2c5smen", &self.i2c5smen())
                .field("i2c6smen", &self.i2c6smen())
                .field("fdcan1smen", &self.fdcan1smen())
                .field("ucpd1smen", &self.ucpd1smen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1smenr2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb1smenr2 {
                i2c4smen: bool,
                lptim2smen: bool,
                i2c5smen: bool,
                i2c6smen: bool,
                fdcan1smen: bool,
                ucpd1smen: bool,
            }
            let proxy = Apb1smenr2 {
                i2c4smen: self.i2c4smen(),
                lptim2smen: self.lptim2smen(),
                i2c5smen: self.i2c5smen(),
                i2c6smen: self.i2c6smen(),
                fdcan1smen: self.fdcan1smen(),
                ucpd1smen: self.ucpd1smen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB2 peripheral clock enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2enr(pub u32);
    impl Apb2enr {
        #[doc = "TIM1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn tim1en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn spi1en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_spi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TIM8 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn tim8en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim8en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "USART1clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn usart1en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_usart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TIM15 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn tim15en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim15en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn tim16en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim16en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn tim17en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim17en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SAI1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn sai1en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_sai1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SAI2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn sai2en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SAI2 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_sai2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "USB clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn usben(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "USB clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_usben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "GFXTIM clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn gfxtimen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "GFXTIM clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_gfxtimen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "LTDC clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn ltdcen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_ltdcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "DSI clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn dsien(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "DSI clock enable This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_dsien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
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
                .field("spi1en", &self.spi1en())
                .field("tim8en", &self.tim8en())
                .field("usart1en", &self.usart1en())
                .field("tim15en", &self.tim15en())
                .field("tim16en", &self.tim16en())
                .field("tim17en", &self.tim17en())
                .field("sai1en", &self.sai1en())
                .field("sai2en", &self.sai2en())
                .field("usben", &self.usben())
                .field("gfxtimen", &self.gfxtimen())
                .field("ltdcen", &self.ltdcen())
                .field("dsien", &self.dsien())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2enr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb2enr {
                tim1en: bool,
                spi1en: bool,
                tim8en: bool,
                usart1en: bool,
                tim15en: bool,
                tim16en: bool,
                tim17en: bool,
                sai1en: bool,
                sai2en: bool,
                usben: bool,
                gfxtimen: bool,
                ltdcen: bool,
                dsien: bool,
            }
            let proxy = Apb2enr {
                tim1en: self.tim1en(),
                spi1en: self.spi1en(),
                tim8en: self.tim8en(),
                usart1en: self.usart1en(),
                tim15en: self.tim15en(),
                tim16en: self.tim16en(),
                tim17en: self.tim17en(),
                sai1en: self.sai1en(),
                sai2en: self.sai2en(),
                usben: self.usben(),
                gfxtimen: self.gfxtimen(),
                ltdcen: self.ltdcen(),
                dsien: self.dsien(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB2 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2rstr(pub u32);
    impl Apb2rstr {
        #[doc = "TIM1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn tim1rst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn spi1rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_spi1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TIM8 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn tim8rst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim8rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "USART1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn usart1rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_usart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TIM15 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn tim15rst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim15rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn tim16rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim16rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn tim17rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim17rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SAI1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn sai1rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_sai1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SAI2 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn sai2rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SAI2 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_sai2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "USB reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn usbrst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "USB reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_usbrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "GFXTIM reset This bit is set and cleared by software. Note: .This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn gfxtimrst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "GFXTIM reset This bit is set and cleared by software. Note: .This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_gfxtimrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "LTDC reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn ltdcrst(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_ltdcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "DSI reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn dsirst(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "DSI reset This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_dsirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
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
                .field("spi1rst", &self.spi1rst())
                .field("tim8rst", &self.tim8rst())
                .field("usart1rst", &self.usart1rst())
                .field("tim15rst", &self.tim15rst())
                .field("tim16rst", &self.tim16rst())
                .field("tim17rst", &self.tim17rst())
                .field("sai1rst", &self.sai1rst())
                .field("sai2rst", &self.sai2rst())
                .field("usbrst", &self.usbrst())
                .field("gfxtimrst", &self.gfxtimrst())
                .field("ltdcrst", &self.ltdcrst())
                .field("dsirst", &self.dsirst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2rstr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb2rstr {
                tim1rst: bool,
                spi1rst: bool,
                tim8rst: bool,
                usart1rst: bool,
                tim15rst: bool,
                tim16rst: bool,
                tim17rst: bool,
                sai1rst: bool,
                sai2rst: bool,
                usbrst: bool,
                gfxtimrst: bool,
                ltdcrst: bool,
                dsirst: bool,
            }
            let proxy = Apb2rstr {
                tim1rst: self.tim1rst(),
                spi1rst: self.spi1rst(),
                tim8rst: self.tim8rst(),
                usart1rst: self.usart1rst(),
                tim15rst: self.tim15rst(),
                tim16rst: self.tim16rst(),
                tim17rst: self.tim17rst(),
                sai1rst: self.sai1rst(),
                sai2rst: self.sai2rst(),
                usbrst: self.usbrst(),
                gfxtimrst: self.gfxtimrst(),
                ltdcrst: self.ltdcrst(),
                dsirst: self.dsirst(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB2 peripheral clocks enable in Sleep and Stop modes register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2smenr(pub u32);
    impl Apb2smenr {
        #[doc = "TIM1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn tim1smen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn spi1smen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_spi1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TIM8 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn tim8smen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim8smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "USART1clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn usart1smen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_usart1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TIM15 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn tim15smen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim15smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn tim16smen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim16smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn tim17smen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_tim17smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SAI1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn sai1smen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_sai1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SAI2 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn sai2smen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SAI2 clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_sai2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "USB clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn usbsmen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "USB clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_usbsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "GFXTIM clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn gfxtimsmen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "GFXTIM clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_gfxtimsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "LTDC clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn ltdcsmen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_ltdcsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "DSI clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn dsismen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "DSI clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_dsismen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Apb2smenr {
        #[inline(always)]
        fn default() -> Apb2smenr {
            Apb2smenr(0)
        }
    }
    impl core::fmt::Debug for Apb2smenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2smenr")
                .field("tim1smen", &self.tim1smen())
                .field("spi1smen", &self.spi1smen())
                .field("tim8smen", &self.tim8smen())
                .field("usart1smen", &self.usart1smen())
                .field("tim15smen", &self.tim15smen())
                .field("tim16smen", &self.tim16smen())
                .field("tim17smen", &self.tim17smen())
                .field("sai1smen", &self.sai1smen())
                .field("sai2smen", &self.sai2smen())
                .field("usbsmen", &self.usbsmen())
                .field("gfxtimsmen", &self.gfxtimsmen())
                .field("ltdcsmen", &self.ltdcsmen())
                .field("dsismen", &self.dsismen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2smenr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb2smenr {
                tim1smen: bool,
                spi1smen: bool,
                tim8smen: bool,
                usart1smen: bool,
                tim15smen: bool,
                tim16smen: bool,
                tim17smen: bool,
                sai1smen: bool,
                sai2smen: bool,
                usbsmen: bool,
                gfxtimsmen: bool,
                ltdcsmen: bool,
                dsismen: bool,
            }
            let proxy = Apb2smenr {
                tim1smen: self.tim1smen(),
                spi1smen: self.spi1smen(),
                tim8smen: self.tim8smen(),
                usart1smen: self.usart1smen(),
                tim15smen: self.tim15smen(),
                tim16smen: self.tim16smen(),
                tim17smen: self.tim17smen(),
                sai1smen: self.sai1smen(),
                sai2smen: self.sai2smen(),
                usbsmen: self.usbsmen(),
                gfxtimsmen: self.gfxtimsmen(),
                ltdcsmen: self.ltdcsmen(),
                dsismen: self.dsismen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB3 peripheral clock enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb3enr(pub u32);
    impl Apb3enr {
        #[doc = "SYSCFG clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn syscfgen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_syscfgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SPI3 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn spi3en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_spi3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LPUART1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn lpuart1en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_lpuart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I2C3 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn i2c3en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_i2c3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LPTIM1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn lptim1en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_lptim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIM3 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn lptim3en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_lptim3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "LPTIM4 clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn lptim4en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM4 clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_lptim4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "OPAMP clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn opampen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_opampen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "COMP clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn compen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "COMP clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_compen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "VREFBUF clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn vrefen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "VREFBUF clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_vrefen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "RTC and TAMP APB clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn rtcapben(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "RTC and TAMP APB clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_rtcapben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
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
                .field("syscfgen", &self.syscfgen())
                .field("spi3en", &self.spi3en())
                .field("lpuart1en", &self.lpuart1en())
                .field("i2c3en", &self.i2c3en())
                .field("lptim1en", &self.lptim1en())
                .field("lptim3en", &self.lptim3en())
                .field("lptim4en", &self.lptim4en())
                .field("opampen", &self.opampen())
                .field("compen", &self.compen())
                .field("vrefen", &self.vrefen())
                .field("rtcapben", &self.rtcapben())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb3enr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb3enr {
                syscfgen: bool,
                spi3en: bool,
                lpuart1en: bool,
                i2c3en: bool,
                lptim1en: bool,
                lptim3en: bool,
                lptim4en: bool,
                opampen: bool,
                compen: bool,
                vrefen: bool,
                rtcapben: bool,
            }
            let proxy = Apb3enr {
                syscfgen: self.syscfgen(),
                spi3en: self.spi3en(),
                lpuart1en: self.lpuart1en(),
                i2c3en: self.i2c3en(),
                lptim1en: self.lptim1en(),
                lptim3en: self.lptim3en(),
                lptim4en: self.lptim4en(),
                opampen: self.opampen(),
                compen: self.compen(),
                vrefen: self.vrefen(),
                rtcapben: self.rtcapben(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB3 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb3rstr(pub u32);
    impl Apb3rstr {
        #[doc = "SYSCFG reset Set and cleared by software."]
        #[inline(always)]
        pub const fn syscfgrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_syscfgrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SPI3 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn spi3rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_spi3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LPUART1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn lpuart1rst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_lpuart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I2C3 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn i2c3rst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_i2c3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LPTIM1 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn lptim1rst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_lptim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIM3 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn lptim3rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_lptim3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "LPTIM4 reset Set and cleared by software."]
        #[inline(always)]
        pub const fn lptim4rst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM4 reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_lptim4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "OPAMP reset Set and cleared by software."]
        #[inline(always)]
        pub const fn opamprst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_opamprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "COMP reset Set and cleared by software."]
        #[inline(always)]
        pub const fn comprst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "COMP reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_comprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "VREFBUF reset Set and cleared by software."]
        #[inline(always)]
        pub const fn vrefrst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "VREFBUF reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_vrefrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
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
                .field("syscfgrst", &self.syscfgrst())
                .field("spi3rst", &self.spi3rst())
                .field("lpuart1rst", &self.lpuart1rst())
                .field("i2c3rst", &self.i2c3rst())
                .field("lptim1rst", &self.lptim1rst())
                .field("lptim3rst", &self.lptim3rst())
                .field("lptim4rst", &self.lptim4rst())
                .field("opamprst", &self.opamprst())
                .field("comprst", &self.comprst())
                .field("vrefrst", &self.vrefrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb3rstr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb3rstr {
                syscfgrst: bool,
                spi3rst: bool,
                lpuart1rst: bool,
                i2c3rst: bool,
                lptim1rst: bool,
                lptim3rst: bool,
                lptim4rst: bool,
                opamprst: bool,
                comprst: bool,
                vrefrst: bool,
            }
            let proxy = Apb3rstr {
                syscfgrst: self.syscfgrst(),
                spi3rst: self.spi3rst(),
                lpuart1rst: self.lpuart1rst(),
                i2c3rst: self.i2c3rst(),
                lptim1rst: self.lptim1rst(),
                lptim3rst: self.lptim3rst(),
                lptim4rst: self.lptim4rst(),
                opamprst: self.opamprst(),
                comprst: self.comprst(),
                vrefrst: self.vrefrst(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB3 peripheral clock enable in Sleep and Stop modes register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb3smenr(pub u32);
    impl Apb3smenr {
        #[doc = "SYSCFG clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn syscfgsmen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_syscfgsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SPI3 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn spi3smen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_spi3smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LPUART1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn lpuart1smen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_lpuart1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I2C3 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn i2c3smen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_i2c3smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LPTIM1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn lptim1smen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_lptim1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIM3 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn lptim3smen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_lptim3smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "LPTIM4 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn lptim4smen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM4 clocks enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_lptim4smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "OPAMP clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn opampsmen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_opampsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "COMP clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn compsmen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "COMP clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_compsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "VREFBUF clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub const fn vrefsmen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "VREFBUF clocks enable during Sleep and Stop modes Set and cleared by software."]
        #[inline(always)]
        pub fn set_vrefsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "RTC and TAMP APB clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn rtcapbsmen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "RTC and TAMP APB clock enable during Sleep and Stop modes Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_rtcapbsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Apb3smenr {
        #[inline(always)]
        fn default() -> Apb3smenr {
            Apb3smenr(0)
        }
    }
    impl core::fmt::Debug for Apb3smenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb3smenr")
                .field("syscfgsmen", &self.syscfgsmen())
                .field("spi3smen", &self.spi3smen())
                .field("lpuart1smen", &self.lpuart1smen())
                .field("i2c3smen", &self.i2c3smen())
                .field("lptim1smen", &self.lptim1smen())
                .field("lptim3smen", &self.lptim3smen())
                .field("lptim4smen", &self.lptim4smen())
                .field("opampsmen", &self.opampsmen())
                .field("compsmen", &self.compsmen())
                .field("vrefsmen", &self.vrefsmen())
                .field("rtcapbsmen", &self.rtcapbsmen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb3smenr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb3smenr {
                syscfgsmen: bool,
                spi3smen: bool,
                lpuart1smen: bool,
                i2c3smen: bool,
                lptim1smen: bool,
                lptim3smen: bool,
                lptim4smen: bool,
                opampsmen: bool,
                compsmen: bool,
                vrefsmen: bool,
                rtcapbsmen: bool,
            }
            let proxy = Apb3smenr {
                syscfgsmen: self.syscfgsmen(),
                spi3smen: self.spi3smen(),
                lpuart1smen: self.lpuart1smen(),
                i2c3smen: self.i2c3smen(),
                lptim1smen: self.lptim1smen(),
                lptim3smen: self.lptim3smen(),
                lptim4smen: self.lptim4smen(),
                opampsmen: self.opampsmen(),
                compsmen: self.compsmen(),
                vrefsmen: self.vrefsmen(),
                rtcapbsmen: self.rtcapbsmen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC Backup domain control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdcr(pub u32);
    impl Bdcr {
        #[doc = "LSE oscillator enable Set and cleared by software."]
        #[inline(always)]
        pub const fn lseon(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSE oscillator enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_lseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE oscillator ready Set and cleared by hardware to indicate when the external 32 kHz oscillator is stable. After the LSEON bit is cleared, LSERDY goes low after six external low-speed oscillator clock cycles."]
        #[inline(always)]
        pub const fn lserdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE oscillator ready Set and cleared by hardware to indicate when the external 32 kHz oscillator is stable. After the LSEON bit is cleared, LSERDY goes low after six external low-speed oscillator clock cycles."]
        #[inline(always)]
        pub fn set_lserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LSE oscillator bypass Set and cleared by software to bypass oscillator in debug mode. This bit can be written only when the external 32 kHz oscillator is disabled (LSEON = 0 and LSERDY = 0)."]
        #[inline(always)]
        pub const fn lsebyp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "LSE oscillator bypass Set and cleared by software to bypass oscillator in debug mode. This bit can be written only when the external 32 kHz oscillator is disabled (LSEON = 0 and LSERDY = 0)."]
        #[inline(always)]
        pub fn set_lsebyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "LSE oscillator drive capability Set by software to modulate the drive capability of the LSE oscillator. This field can be written only when the external 32 kHz oscillator is disabled (LSEON = 0 and LSERDY = 0). Note: The oscillator is in 'Xtal mode when it is not in bypass mode."]
        #[inline(always)]
        pub const fn lsedrv(&self) -> super::vals::Lsedrv {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Lsedrv::from_bits(val as u8)
        }
        #[doc = "LSE oscillator drive capability Set by software to modulate the drive capability of the LSE oscillator. This field can be written only when the external 32 kHz oscillator is disabled (LSEON = 0 and LSERDY = 0). Note: The oscillator is in 'Xtal mode when it is not in bypass mode."]
        #[inline(always)]
        pub fn set_lsedrv(&mut self, val: super::vals::Lsedrv) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "CSS on LSE enable Set by software to enable the CSS on LSE. LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD = 1). In that case, the software must disable the LSECSSON bit."]
        #[inline(always)]
        pub const fn lsecsson(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "CSS on LSE enable Set by software to enable the CSS on LSE. LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware), and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD = 1). In that case, the software must disable the LSECSSON bit."]
        #[inline(always)]
        pub fn set_lsecsson(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "CSS on LSE failure Detection Set by hardware to indicate when a failure is detected by the CCS on the external 32 kHz oscillator (LSE)."]
        #[inline(always)]
        pub const fn lsecssd(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "CSS on LSE failure Detection Set by hardware to indicate when a failure is detected by the CCS on the external 32 kHz oscillator (LSE)."]
        #[inline(always)]
        pub fn set_lsecssd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LSE system clock (LSESYS) enable Set by software to enable always the LSE system clock generated by RCC. This clock can be used by any peripheral when its source clock is the LSE or at system level in case of one of the LSCOSEL, MCO, MSI PLL mode or CSS on LSE is needed. The LSESYS clock can be generated even if LSESYSEN= 0 if the LSE clock is requested by the CSS on LSE, by a peripheral or any other source clock using LSE."]
        #[inline(always)]
        pub const fn lsesysen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LSE system clock (LSESYS) enable Set by software to enable always the LSE system clock generated by RCC. This clock can be used by any peripheral when its source clock is the LSE or at system level in case of one of the LSCOSEL, MCO, MSI PLL mode or CSS on LSE is needed. The LSESYS clock can be generated even if LSESYSEN= 0 if the LSE clock is requested by the CSS on LSE, by a peripheral or any other source clock using LSE."]
        #[inline(always)]
        pub fn set_lsesysen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "RTC and TAMP clock source selection Set by software to select the clock source for the RTC and TAMP . Once the RTC and TAMP clock source has been selected, it cannot be changed anymore unless the Backup domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The BDRST bit can be used to reset them."]
        #[inline(always)]
        pub const fn rtcsel(&self) -> super::vals::Rtcsel {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Rtcsel::from_bits(val as u8)
        }
        #[doc = "RTC and TAMP clock source selection Set by software to select the clock source for the RTC and TAMP . Once the RTC and TAMP clock source has been selected, it cannot be changed anymore unless the Backup domain is reset, or unless a failure is detected on LSE (LSECSSD is set). The BDRST bit can be used to reset them."]
        #[inline(always)]
        pub fn set_rtcsel(&mut self, val: super::vals::Rtcsel) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "LSE system clock (LSESYS) ready Set and cleared by hardware to indicate when the LSE system clock is stable.When the LSESYSEN bit is set, the LSESYSRDY flag is set after two LSE clock cycles. The LSE clock must be already enabled and stable (LSEON and LSERDY are set). When the LSEON bit is cleared, LSERDY goes low after six external low-speed oscillator clock cycles."]
        #[inline(always)]
        pub const fn lsesysrdy(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LSE system clock (LSESYS) ready Set and cleared by hardware to indicate when the LSE system clock is stable.When the LSESYSEN bit is set, the LSESYSRDY flag is set after two LSE clock cycles. The LSE clock must be already enabled and stable (LSEON and LSERDY are set). When the LSEON bit is cleared, LSERDY goes low after six external low-speed oscillator clock cycles."]
        #[inline(always)]
        pub fn set_lsesysrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LSE clock glitch filter enable Set and cleared by hardware to enable the LSE glitch filter. This bit can be written only when the LSE is disabled (LSEON = 0 and LSERDY = 0)"]
        #[inline(always)]
        pub const fn lsegfon(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock glitch filter enable Set and cleared by hardware to enable the LSE glitch filter. This bit can be written only when the LSE is disabled (LSEON = 0 and LSERDY = 0)"]
        #[inline(always)]
        pub fn set_lsegfon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "RTC and TAMP clock enable Set and cleared by software."]
        #[inline(always)]
        pub const fn rtcen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "RTC and TAMP clock enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_rtcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Backup domain software reset Set and cleared by software."]
        #[inline(always)]
        pub const fn bdrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Backup domain software reset Set and cleared by software."]
        #[inline(always)]
        pub fn set_bdrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Low-speed clock output (LSCO) enable Set and cleared by software."]
        #[inline(always)]
        pub const fn lscoen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Low-speed clock output (LSCO) enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_lscoen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Low-speed clock output selection Set and cleared by software."]
        #[inline(always)]
        pub const fn lscosel(&self) -> super::vals::Lscosel {
            let val = (self.0 >> 25usize) & 0x01;
            super::vals::Lscosel::from_bits(val as u8)
        }
        #[doc = "Low-speed clock output selection Set and cleared by software."]
        #[inline(always)]
        pub fn set_lscosel(&mut self, val: super::vals::Lscosel) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
        }
        #[doc = "LSI oscillator enable Set and cleared by software."]
        #[inline(always)]
        pub const fn lsion(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "LSI oscillator enable Set and cleared by software."]
        #[inline(always)]
        pub fn set_lsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is stable. After the LSION bit is cleared, LSIRDY goes low after three internal low-speed oscillator clock cycles. This bit is set when the LSI is used by IWDG or RTC, even if LSION = 0."]
        #[inline(always)]
        pub const fn lsirdy(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "LSI oscillator ready Set and cleared by hardware to indicate when the LSI oscillator is stable. After the LSION bit is cleared, LSIRDY goes low after three internal low-speed oscillator clock cycles. This bit is set when the LSI is used by IWDG or RTC, even if LSION = 0."]
        #[inline(always)]
        pub fn set_lsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Low-speed clock divider configuration Set and cleared by software to enable the LSI division. This bit can be written only when the LSI is disabled (LSION = 0 and LSIRDY = 0). If the LSI was previously enabled, it is necessary to wait for at least 60 Î¼s after clearing LSION bit (synchronization time for LSI to be really disabled), before writing LSIPREDIV. The LSIPREDIV cannot be changed if the LSI is used by the IWDG or by the RTC."]
        #[inline(always)]
        pub const fn lsiprediv(&self) -> super::vals::Lsiprediv {
            let val = (self.0 >> 28usize) & 0x01;
            super::vals::Lsiprediv::from_bits(val as u8)
        }
        #[doc = "Low-speed clock divider configuration Set and cleared by software to enable the LSI division. This bit can be written only when the LSI is disabled (LSION = 0 and LSIRDY = 0). If the LSI was previously enabled, it is necessary to wait for at least 60 Î¼s after clearing LSION bit (synchronization time for LSI to be really disabled), before writing LSIPREDIV. The LSIPREDIV cannot be changed if the LSI is used by the IWDG or by the RTC."]
        #[inline(always)]
        pub fn set_lsiprediv(&mut self, val: super::vals::Lsiprediv) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
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
                .field("lsesysen", &self.lsesysen())
                .field("rtcsel", &self.rtcsel())
                .field("lsesysrdy", &self.lsesysrdy())
                .field("lsegfon", &self.lsegfon())
                .field("rtcen", &self.rtcen())
                .field("bdrst", &self.bdrst())
                .field("lscoen", &self.lscoen())
                .field("lscosel", &self.lscosel())
                .field("lsion", &self.lsion())
                .field("lsirdy", &self.lsirdy())
                .field("lsiprediv", &self.lsiprediv())
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
                lsesysen: bool,
                rtcsel: super::vals::Rtcsel,
                lsesysrdy: bool,
                lsegfon: bool,
                rtcen: bool,
                bdrst: bool,
                lscoen: bool,
                lscosel: super::vals::Lscosel,
                lsion: bool,
                lsirdy: bool,
                lsiprediv: super::vals::Lsiprediv,
            }
            let proxy = Bdcr {
                lseon: self.lseon(),
                lserdy: self.lserdy(),
                lsebyp: self.lsebyp(),
                lsedrv: self.lsedrv(),
                lsecsson: self.lsecsson(),
                lsecssd: self.lsecssd(),
                lsesysen: self.lsesysen(),
                rtcsel: self.rtcsel(),
                lsesysrdy: self.lsesysrdy(),
                lsegfon: self.lsegfon(),
                rtcen: self.rtcen(),
                bdrst: self.bdrst(),
                lscoen: self.lscoen(),
                lscosel: self.lscosel(),
                lsion: self.lsion(),
                lsirdy: self.lsirdy(),
                lsiprediv: self.lsiprediv(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC peripherals independent clock configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccipr1(pub u32);
    impl Ccipr1 {
        #[doc = "USART1 kernel clock source selection This bits are used to select the USART1 kernel clock source. Note: The USART1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or LSE."]
        #[inline(always)]
        pub const fn usart1sel(&self) -> super::vals::Usart1sel {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Usart1sel::from_bits(val as u8)
        }
        #[doc = "USART1 kernel clock source selection This bits are used to select the USART1 kernel clock source. Note: The USART1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or LSE."]
        #[inline(always)]
        pub fn set_usart1sel(&mut self, val: super::vals::Usart1sel) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "USART2 kernel clock source selection This bits are used to select the USART2 kernel clock source. Note: The USART2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or LSE."]
        #[inline(always)]
        pub const fn usart2sel(&self) -> super::vals::Usartsel {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Usartsel::from_bits(val as u8)
        }
        #[doc = "USART2 kernel clock source selection This bits are used to select the USART2 kernel clock source. Note: The USART2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or LSE."]
        #[inline(always)]
        pub fn set_usart2sel(&mut self, val: super::vals::Usartsel) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "USART3 kernel clock source selection This bits are used to select the USART3 kernel clock source. Note: The USART3 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or LSE."]
        #[inline(always)]
        pub const fn usart3sel(&self) -> super::vals::Usartsel {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Usartsel::from_bits(val as u8)
        }
        #[doc = "USART3 kernel clock source selection This bits are used to select the USART3 kernel clock source. Note: The USART3 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or LSE."]
        #[inline(always)]
        pub fn set_usart3sel(&mut self, val: super::vals::Usartsel) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "UART4 kernel clock source selection This bits are used to select the UART4 kernel clock source. Note: The UART4 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or LSE."]
        #[inline(always)]
        pub const fn uart4sel(&self) -> super::vals::Usartsel {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::Usartsel::from_bits(val as u8)
        }
        #[doc = "UART4 kernel clock source selection This bits are used to select the UART4 kernel clock source. Note: The UART4 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or LSE."]
        #[inline(always)]
        pub fn set_uart4sel(&mut self, val: super::vals::Usartsel) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "UART5 kernel clock source selection These bits are used to select the UART5 kernel clock source. Note: The UART5 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or LSE."]
        #[inline(always)]
        pub const fn uart5sel(&self) -> super::vals::Usartsel {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Usartsel::from_bits(val as u8)
        }
        #[doc = "UART5 kernel clock source selection These bits are used to select the UART5 kernel clock source. Note: The UART5 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or LSE."]
        #[inline(always)]
        pub fn set_uart5sel(&mut self, val: super::vals::Usartsel) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "I2C1 kernel clock source selection These bits are used to select the I2C1 kernel clock source. Note: The I2C1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or MSIK."]
        #[inline(always)]
        pub const fn i2c1sel(&self) -> super::vals::I2csel {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::I2csel::from_bits(val as u8)
        }
        #[doc = "I2C1 kernel clock source selection These bits are used to select the I2C1 kernel clock source. Note: The I2C1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or MSIK."]
        #[inline(always)]
        pub fn set_i2c1sel(&mut self, val: super::vals::I2csel) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
        #[doc = "I2C2 kernel clock source selection These bits are used to select the I2C2 kernel clock source. Note: The I2C2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or MSIK."]
        #[inline(always)]
        pub const fn i2c2sel(&self) -> super::vals::I2csel {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::I2csel::from_bits(val as u8)
        }
        #[doc = "I2C2 kernel clock source selection These bits are used to select the I2C2 kernel clock source. Note: The I2C2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or MSIK."]
        #[inline(always)]
        pub fn set_i2c2sel(&mut self, val: super::vals::I2csel) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "I2C4 kernel clock source selection These bits are used to select the I2C4 kernel clock source. Note: The I2C4 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or MSIK."]
        #[inline(always)]
        pub const fn i2c4sel(&self) -> super::vals::I2csel {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::I2csel::from_bits(val as u8)
        }
        #[doc = "I2C4 kernel clock source selection These bits are used to select the I2C4 kernel clock source. Note: The I2C4 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or MSIK."]
        #[inline(always)]
        pub fn set_i2c4sel(&mut self, val: super::vals::I2csel) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
        #[doc = "SPI2 kernel clock source selection These bits are used to select the SPI2 kernel clock source. Note: The SPI2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or MSIK."]
        #[inline(always)]
        pub const fn spi2sel(&self) -> super::vals::Spi2sel {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Spi2sel::from_bits(val as u8)
        }
        #[doc = "SPI2 kernel clock source selection These bits are used to select the SPI2 kernel clock source. Note: The SPI2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or MSIK."]
        #[inline(always)]
        pub fn set_spi2sel(&mut self, val: super::vals::Spi2sel) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Low-power timer 2 kernel clock source selection These bits are used to select the LPTIM2 kernel clock source. Note: The LPTIM2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is LSI, LSE or HSI if HSIKERON = 1."]
        #[inline(always)]
        pub const fn lptim2sel(&self) -> super::vals::Lptim2sel {
            let val = (self.0 >> 18usize) & 0x03;
            super::vals::Lptim2sel::from_bits(val as u8)
        }
        #[doc = "Low-power timer 2 kernel clock source selection These bits are used to select the LPTIM2 kernel clock source. Note: The LPTIM2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is LSI, LSE or HSI if HSIKERON = 1."]
        #[inline(always)]
        pub fn set_lptim2sel(&mut self, val: super::vals::Lptim2sel) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
        }
        #[doc = "SPI1 kernel clock source selection These bits are used to select the SPI1 kernel clock source. Note: The SPI1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or MSIK."]
        #[inline(always)]
        pub const fn spi1sel(&self) -> super::vals::Spi1sel {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::Spi1sel::from_bits(val as u8)
        }
        #[doc = "SPI1 kernel clock source selection These bits are used to select the SPI1 kernel clock source. Note: The SPI1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or MSIK."]
        #[inline(always)]
        pub fn set_spi1sel(&mut self, val: super::vals::Spi1sel) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "SysTick clock source selection These bits are used to select the SysTick clock source. Note: When LSE or LSI is selected, the AHB frequency must be at least four times higher than the LSI or LSE frequency. In addition, a jitter up to one HCLK cycle is introduced, due to the LSE or LSI sampling with HCLK in the SysTick circuitry."]
        #[inline(always)]
        pub const fn systicksel(&self) -> super::vals::Systicksel {
            let val = (self.0 >> 22usize) & 0x03;
            super::vals::Systicksel::from_bits(val as u8)
        }
        #[doc = "SysTick clock source selection These bits are used to select the SysTick clock source. Note: When LSE or LSI is selected, the AHB frequency must be at least four times higher than the LSI or LSE frequency. In addition, a jitter up to one HCLK cycle is introduced, due to the LSE or LSI sampling with HCLK in the SysTick circuitry."]
        #[inline(always)]
        pub fn set_systicksel(&mut self, val: super::vals::Systicksel) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
        }
        #[doc = "FDCAN1 kernel clock source selection These bits are used to select the FDCAN1 kernel clock source."]
        #[inline(always)]
        pub const fn fdcan1sel(&self) -> super::vals::Fdcansel {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Fdcansel::from_bits(val as u8)
        }
        #[doc = "FDCAN1 kernel clock source selection These bits are used to select the FDCAN1 kernel clock source."]
        #[inline(always)]
        pub fn set_fdcan1sel(&mut self, val: super::vals::Fdcansel) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "intermediate clock source selection These bits are used to select the clock source used by OTG_FS and SDMMC."]
        #[inline(always)]
        pub const fn iclksel(&self) -> super::vals::Iclksel {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::Iclksel::from_bits(val as u8)
        }
        #[doc = "intermediate clock source selection These bits are used to select the clock source used by OTG_FS and SDMMC."]
        #[inline(always)]
        pub fn set_iclksel(&mut self, val: super::vals::Iclksel) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "Clocks sources for TIM16,TIM17 and LPTIM2 internal input capture When the TIMICSEL2 bit is set, the TIM16, TIM17 and LPTIM2 internal input capture can be connected either to HSI/256, MSI/4 or MSI/1024. Depending on TIMICSEL\\[1:0\\]
value, MSI is either MSIK or MSIS. When TIMICSEL2 is cleared, the HSI, MSIK and MSIS clock sources cannot be selected as TIM16, TIM17 or LPTIM2 internal input capture. 0xx: HSI, MSIK and MSIS dividers disabled Note: The clock division must be disabled (TIMICSEL configured to 0xx) before selecting or changing a clock sources division."]
        #[inline(always)]
        pub const fn timicsel(&self) -> super::vals::Timicsel {
            let val = (self.0 >> 29usize) & 0x07;
            super::vals::Timicsel::from_bits(val as u8)
        }
        #[doc = "Clocks sources for TIM16,TIM17 and LPTIM2 internal input capture When the TIMICSEL2 bit is set, the TIM16, TIM17 and LPTIM2 internal input capture can be connected either to HSI/256, MSI/4 or MSI/1024. Depending on TIMICSEL\\[1:0\\]
value, MSI is either MSIK or MSIS. When TIMICSEL2 is cleared, the HSI, MSIK and MSIS clock sources cannot be selected as TIM16, TIM17 or LPTIM2 internal input capture. 0xx: HSI, MSIK and MSIS dividers disabled Note: The clock division must be disabled (TIMICSEL configured to 0xx) before selecting or changing a clock sources division."]
        #[inline(always)]
        pub fn set_timicsel(&mut self, val: super::vals::Timicsel) {
            self.0 = (self.0 & !(0x07 << 29usize)) | (((val.to_bits() as u32) & 0x07) << 29usize);
        }
    }
    impl Default for Ccipr1 {
        #[inline(always)]
        fn default() -> Ccipr1 {
            Ccipr1(0)
        }
    }
    impl core::fmt::Debug for Ccipr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccipr1")
                .field("usart1sel", &self.usart1sel())
                .field("usart2sel", &self.usart2sel())
                .field("usart3sel", &self.usart3sel())
                .field("uart4sel", &self.uart4sel())
                .field("uart5sel", &self.uart5sel())
                .field("i2c1sel", &self.i2c1sel())
                .field("i2c2sel", &self.i2c2sel())
                .field("i2c4sel", &self.i2c4sel())
                .field("spi2sel", &self.spi2sel())
                .field("lptim2sel", &self.lptim2sel())
                .field("spi1sel", &self.spi1sel())
                .field("systicksel", &self.systicksel())
                .field("fdcan1sel", &self.fdcan1sel())
                .field("iclksel", &self.iclksel())
                .field("timicsel", &self.timicsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccipr1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ccipr1 {
                usart1sel: super::vals::Usart1sel,
                usart2sel: super::vals::Usartsel,
                usart3sel: super::vals::Usartsel,
                uart4sel: super::vals::Usartsel,
                uart5sel: super::vals::Usartsel,
                i2c1sel: super::vals::I2csel,
                i2c2sel: super::vals::I2csel,
                i2c4sel: super::vals::I2csel,
                spi2sel: super::vals::Spi2sel,
                lptim2sel: super::vals::Lptim2sel,
                spi1sel: super::vals::Spi1sel,
                systicksel: super::vals::Systicksel,
                fdcan1sel: super::vals::Fdcansel,
                iclksel: super::vals::Iclksel,
                timicsel: super::vals::Timicsel,
            }
            let proxy = Ccipr1 {
                usart1sel: self.usart1sel(),
                usart2sel: self.usart2sel(),
                usart3sel: self.usart3sel(),
                uart4sel: self.uart4sel(),
                uart5sel: self.uart5sel(),
                i2c1sel: self.i2c1sel(),
                i2c2sel: self.i2c2sel(),
                i2c4sel: self.i2c4sel(),
                spi2sel: self.spi2sel(),
                lptim2sel: self.lptim2sel(),
                spi1sel: self.spi1sel(),
                systicksel: self.systicksel(),
                fdcan1sel: self.fdcan1sel(),
                iclksel: self.iclksel(),
                timicsel: self.timicsel(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC peripherals independent clock configuration register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccipr2(pub u32);
    impl Ccipr2 {
        #[doc = "MDF1 kernel clock source selection These bits are used to select the MDF1 kernel clock source. others: reserved"]
        #[inline(always)]
        pub const fn mdf1sel(&self) -> super::vals::Mdfsel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Mdfsel::from_bits(val as u8)
        }
        #[doc = "MDF1 kernel clock source selection These bits are used to select the MDF1 kernel clock source. others: reserved"]
        #[inline(always)]
        pub fn set_mdf1sel(&mut self, val: super::vals::Mdfsel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "SAI1 kernel clock source selection These bits are used to select the SAI1 kernel clock source. others: reserved Note: If the selected clock is the external clock and this clock is stopped, a switch to another clock is impossible."]
        #[inline(always)]
        pub const fn sai1sel(&self) -> super::vals::Saisel {
            let val = (self.0 >> 5usize) & 0x07;
            super::vals::Saisel::from_bits(val as u8)
        }
        #[doc = "SAI1 kernel clock source selection These bits are used to select the SAI1 kernel clock source. others: reserved Note: If the selected clock is the external clock and this clock is stopped, a switch to another clock is impossible."]
        #[inline(always)]
        pub fn set_sai1sel(&mut self, val: super::vals::Saisel) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
        }
        #[doc = "SAI2 kernel clock source selection These bits are used to select the SAI2 kernel clock source. others: reserved Note: If the selected clock is the external clock and this clock is stopped, a switch to another clock is impossible."]
        #[inline(always)]
        pub const fn sai2sel(&self) -> super::vals::Saisel {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Saisel::from_bits(val as u8)
        }
        #[doc = "SAI2 kernel clock source selection These bits are used to select the SAI2 kernel clock source. others: reserved Note: If the selected clock is the external clock and this clock is stopped, a switch to another clock is impossible."]
        #[inline(always)]
        pub fn set_sai2sel(&mut self, val: super::vals::Saisel) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "SAES kernel clock source selection This bit is used to select the SAES kernel clock source."]
        #[inline(always)]
        pub const fn saessel(&self) -> super::vals::Saessel {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Saessel::from_bits(val as u8)
        }
        #[doc = "SAES kernel clock source selection This bit is used to select the SAES kernel clock source."]
        #[inline(always)]
        pub fn set_saessel(&mut self, val: super::vals::Saessel) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "RNGSEL kernel clock source selection These bits are used to select the RNG kernel clock source."]
        #[inline(always)]
        pub const fn rngsel(&self) -> super::vals::Rngsel {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Rngsel::from_bits(val as u8)
        }
        #[doc = "RNGSEL kernel clock source selection These bits are used to select the RNG kernel clock source."]
        #[inline(always)]
        pub fn set_rngsel(&mut self, val: super::vals::Rngsel) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "SDMMC1 and SDMMC2 kernel clock source selection This bit is used to select the SDMMC kernel clock source. It is recommended to change this bit only after reset and before enabling the SDMMC."]
        #[inline(always)]
        pub const fn sdmmcsel(&self) -> super::vals::Sdmmcsel {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Sdmmcsel::from_bits(val as u8)
        }
        #[doc = "SDMMC1 and SDMMC2 kernel clock source selection This bit is used to select the SDMMC kernel clock source. It is recommended to change this bit only after reset and before enabling the SDMMC."]
        #[inline(always)]
        pub fn set_sdmmcsel(&mut self, val: super::vals::Sdmmcsel) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "DSI kernel clock source selection This bit is used to select the DSI kernel clock source. This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. Note: If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn dsisel(&self) -> super::vals::Dsisel {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Dsisel::from_bits(val as u8)
        }
        #[doc = "DSI kernel clock source selection This bit is used to select the DSI kernel clock source. This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. Note: If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_dsisel(&mut self, val: super::vals::Dsisel) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "USART6 kernel clock source selection These bits are used to select the USART6 kernel clock source. The USART6 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI or LSE. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn usart6sel(&self) -> super::vals::Usartsel {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Usartsel::from_bits(val as u8)
        }
        #[doc = "USART6 kernel clock source selection These bits are used to select the USART6 kernel clock source. The USART6 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI or LSE. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_usart6sel(&mut self, val: super::vals::Usartsel) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "LTDC kernel clock source selection This bit is used to select the LTDC kernel clock source. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn ltdcsel(&self) -> super::vals::Ltdcsel {
            let val = (self.0 >> 18usize) & 0x01;
            super::vals::Ltdcsel::from_bits(val as u8)
        }
        #[doc = "LTDC kernel clock source selection This bit is used to select the LTDC kernel clock source. Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bit as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_ltdcsel(&mut self, val: super::vals::Ltdcsel) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
        }
        #[doc = "OCTOSPI1 and OCTOSPI2 kernel clock source selection These bits are used to select the OCTOSPI1 and OCTOSPI2 kernel clock source."]
        #[inline(always)]
        pub const fn octospisel(&self) -> super::vals::Octospisel {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::Octospisel::from_bits(val as u8)
        }
        #[doc = "OCTOSPI1 and OCTOSPI2 kernel clock source selection These bits are used to select the OCTOSPI1 and OCTOSPI2 kernel clock source."]
        #[inline(always)]
        pub fn set_octospisel(&mut self, val: super::vals::Octospisel) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "HSPI1 kernel clock source selection These bits are used to select the HSPI1 kernel clock source. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn hspi1sel(&self) -> super::vals::Hspisel {
            let val = (self.0 >> 22usize) & 0x03;
            super::vals::Hspisel::from_bits(val as u8)
        }
        #[doc = "HSPI1 kernel clock source selection These bits are used to select the HSPI1 kernel clock source. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_hspi1sel(&mut self, val: super::vals::Hspisel) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
        }
        #[doc = "I2C5 kernel clock source selection These bits are used to select the I2C5 kernel clock source. The I2C5 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI�or MSIK. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn i2c5sel(&self) -> super::vals::I2csel {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::I2csel::from_bits(val as u8)
        }
        #[doc = "I2C5 kernel clock source selection These bits are used to select the I2C5 kernel clock source. The I2C5 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI�or MSIK. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_i2c5sel(&mut self, val: super::vals::I2csel) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "I2C6 kernel clock source selection These bits are used to select the I2C6 kernel clock source. The I2C6 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI�or MSIK. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn i2c6sel(&self) -> super::vals::I2csel {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::I2csel::from_bits(val as u8)
        }
        #[doc = "I2C6 kernel clock source selection These bits are used to select the I2C6 kernel clock source. The I2C6 is functional in Stop 0 and Stop 1 modes only when the kernel clock is HSI�or MSIK. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_i2c6sel(&mut self, val: super::vals::I2csel) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "OTG_HS PHY kernel clock source selection These bits are used to select the OTG_HS PHY kernel clock source. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn otghssel(&self) -> super::vals::Otghssel {
            let val = (self.0 >> 30usize) & 0x03;
            super::vals::Otghssel::from_bits(val as u8)
        }
        #[doc = "OTG_HS PHY kernel clock source selection These bits are used to select the OTG_HS PHY kernel clock source. Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_otghssel(&mut self, val: super::vals::Otghssel) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
        }
    }
    impl Default for Ccipr2 {
        #[inline(always)]
        fn default() -> Ccipr2 {
            Ccipr2(0)
        }
    }
    impl core::fmt::Debug for Ccipr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccipr2")
                .field("mdf1sel", &self.mdf1sel())
                .field("sai1sel", &self.sai1sel())
                .field("sai2sel", &self.sai2sel())
                .field("saessel", &self.saessel())
                .field("rngsel", &self.rngsel())
                .field("sdmmcsel", &self.sdmmcsel())
                .field("dsisel", &self.dsisel())
                .field("usart6sel", &self.usart6sel())
                .field("ltdcsel", &self.ltdcsel())
                .field("octospisel", &self.octospisel())
                .field("hspi1sel", &self.hspi1sel())
                .field("i2c5sel", &self.i2c5sel())
                .field("i2c6sel", &self.i2c6sel())
                .field("otghssel", &self.otghssel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccipr2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ccipr2 {
                mdf1sel: super::vals::Mdfsel,
                sai1sel: super::vals::Saisel,
                sai2sel: super::vals::Saisel,
                saessel: super::vals::Saessel,
                rngsel: super::vals::Rngsel,
                sdmmcsel: super::vals::Sdmmcsel,
                dsisel: super::vals::Dsisel,
                usart6sel: super::vals::Usartsel,
                ltdcsel: super::vals::Ltdcsel,
                octospisel: super::vals::Octospisel,
                hspi1sel: super::vals::Hspisel,
                i2c5sel: super::vals::I2csel,
                i2c6sel: super::vals::I2csel,
                otghssel: super::vals::Otghssel,
            }
            let proxy = Ccipr2 {
                mdf1sel: self.mdf1sel(),
                sai1sel: self.sai1sel(),
                sai2sel: self.sai2sel(),
                saessel: self.saessel(),
                rngsel: self.rngsel(),
                sdmmcsel: self.sdmmcsel(),
                dsisel: self.dsisel(),
                usart6sel: self.usart6sel(),
                ltdcsel: self.ltdcsel(),
                octospisel: self.octospisel(),
                hspi1sel: self.hspi1sel(),
                i2c5sel: self.i2c5sel(),
                i2c6sel: self.i2c6sel(),
                otghssel: self.otghssel(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC peripherals independent clock configuration register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccipr3(pub u32);
    impl Ccipr3 {
        #[doc = "LPUART1 kernel clock source selection These bits are used to select the LPUART1 kernel clock source. others: reserved Note: The LPUART1 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI, LSE or MSIK."]
        #[inline(always)]
        pub const fn lpuart1sel(&self) -> super::vals::Lpusartsel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Lpusartsel::from_bits(val as u8)
        }
        #[doc = "LPUART1 kernel clock source selection These bits are used to select the LPUART1 kernel clock source. others: reserved Note: The LPUART1 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI, LSE or MSIK."]
        #[inline(always)]
        pub fn set_lpuart1sel(&mut self, val: super::vals::Lpusartsel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "SPI3 kernel clock source selection These bits are used to select the SPI3 kernel clock source. Note: The SPI3 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI or MSIK."]
        #[inline(always)]
        pub const fn spi3sel(&self) -> super::vals::Spi3sel {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Spi3sel::from_bits(val as u8)
        }
        #[doc = "SPI3 kernel clock source selection These bits are used to select the SPI3 kernel clock source. Note: The SPI3 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI or MSIK."]
        #[inline(always)]
        pub fn set_spi3sel(&mut self, val: super::vals::Spi3sel) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "I2C3 kernel clock source selection These bits are used to select the I2C3 kernel clock source. Note: The I2C3 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI or MSIK."]
        #[inline(always)]
        pub const fn i2c3sel(&self) -> super::vals::I2c3sel {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::I2c3sel::from_bits(val as u8)
        }
        #[doc = "I2C3 kernel clock source selection These bits are used to select the I2C3 kernel clock source. Note: The I2C3 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI or MSIK."]
        #[inline(always)]
        pub fn set_i2c3sel(&mut self, val: super::vals::I2c3sel) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "LPTIM3 and LPTIM4 kernel clock source selection These bits are used to select the LPTIM3 and LPTIM4 kernel clock source. Note: The LPTIM3 and LPTIM4 are functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is LSI, LSE, HSI with HSIKERON = 1 or MSIK with MSIKERON = 1."]
        #[inline(always)]
        pub const fn lptim34sel(&self) -> super::vals::Lptimsel {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Lptimsel::from_bits(val as u8)
        }
        #[doc = "LPTIM3 and LPTIM4 kernel clock source selection These bits are used to select the LPTIM3 and LPTIM4 kernel clock source. Note: The LPTIM3 and LPTIM4 are functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is LSI, LSE, HSI with HSIKERON = 1 or MSIK with MSIKERON = 1."]
        #[inline(always)]
        pub fn set_lptim34sel(&mut self, val: super::vals::Lptimsel) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "LPTIM1 kernel clock source selection These bits are used to select the LPTIM1 kernel clock source. Note: The LPTIM1 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is LSI, LSE, HSI with HSIKERON = 1 or MSIK with MSIKERON = 1."]
        #[inline(always)]
        pub const fn lptim1sel(&self) -> super::vals::Lptimsel {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::Lptimsel::from_bits(val as u8)
        }
        #[doc = "LPTIM1 kernel clock source selection These bits are used to select the LPTIM1 kernel clock source. Note: The LPTIM1 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is LSI, LSE, HSI with HSIKERON = 1 or MSIK with MSIKERON = 1."]
        #[inline(always)]
        pub fn set_lptim1sel(&mut self, val: super::vals::Lptimsel) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
        #[doc = "ADC1, ADC4 and DAC1 kernel clock source selection These bits are used to select the ADC1, ADC4 and DAC1 kernel clock source. others: reserved Note: The ADC1, ADC4 and DAC1 are functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI or MSIK (only ADC4 and DAC1 are functional in Stop 2 mode)."]
        #[inline(always)]
        pub const fn adcdacsel(&self) -> super::vals::Adcdacsel {
            let val = (self.0 >> 12usize) & 0x07;
            super::vals::Adcdacsel::from_bits(val as u8)
        }
        #[doc = "ADC1, ADC4 and DAC1 kernel clock source selection These bits are used to select the ADC1, ADC4 and DAC1 kernel clock source. others: reserved Note: The ADC1, ADC4 and DAC1 are functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is HSI or MSIK (only ADC4 and DAC1 are functional in Stop 2 mode)."]
        #[inline(always)]
        pub fn set_adcdacsel(&mut self, val: super::vals::Adcdacsel) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
        }
        #[doc = "DAC1 sample and hold clock source selection This bit is used to select the DAC1 sample and hold clock source."]
        #[inline(always)]
        pub const fn dac1sel(&self) -> super::vals::Dacsel {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Dacsel::from_bits(val as u8)
        }
        #[doc = "DAC1 sample and hold clock source selection This bit is used to select the DAC1 sample and hold clock source."]
        #[inline(always)]
        pub fn set_dac1sel(&mut self, val: super::vals::Dacsel) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "ADF1 kernel clock source selection These bits are used to select the ADF1 kernel clock source. others: reserved Note: The ADF1 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is AUDIOCLK or MSIK."]
        #[inline(always)]
        pub const fn adf1sel(&self) -> super::vals::Adfsel {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::Adfsel::from_bits(val as u8)
        }
        #[doc = "ADF1 kernel clock source selection These bits are used to select the ADF1 kernel clock source. others: reserved Note: The ADF1 is functional in Stop 0, Stop 1 and Stop 2 modes only when the kernel clock is AUDIOCLK or MSIK."]
        #[inline(always)]
        pub fn set_adf1sel(&mut self, val: super::vals::Adfsel) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
    }
    impl Default for Ccipr3 {
        #[inline(always)]
        fn default() -> Ccipr3 {
            Ccipr3(0)
        }
    }
    impl core::fmt::Debug for Ccipr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccipr3")
                .field("lpuart1sel", &self.lpuart1sel())
                .field("spi3sel", &self.spi3sel())
                .field("i2c3sel", &self.i2c3sel())
                .field("lptim34sel", &self.lptim34sel())
                .field("lptim1sel", &self.lptim1sel())
                .field("adcdacsel", &self.adcdacsel())
                .field("dac1sel", &self.dac1sel())
                .field("adf1sel", &self.adf1sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccipr3 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ccipr3 {
                lpuart1sel: super::vals::Lpusartsel,
                spi3sel: super::vals::Spi3sel,
                i2c3sel: super::vals::I2c3sel,
                lptim34sel: super::vals::Lptimsel,
                lptim1sel: super::vals::Lptimsel,
                adcdacsel: super::vals::Adcdacsel,
                dac1sel: super::vals::Dacsel,
                adf1sel: super::vals::Adfsel,
            }
            let proxy = Ccipr3 {
                lpuart1sel: self.lpuart1sel(),
                spi3sel: self.spi3sel(),
                i2c3sel: self.i2c3sel(),
                lptim34sel: self.lptim34sel(),
                lptim1sel: self.lptim1sel(),
                adcdacsel: self.adcdacsel(),
                dac1sel: self.dac1sel(),
                adf1sel: self.adf1sel(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC clock configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr1(pub u32);
    impl Cfgr1 {
        #[doc = "system clock switch Set and cleared by software to select system clock source (SYSCLK). Configured by hardware to force MSIS oscillator selection when exiting Standby or Shutdown mode. Configured by hardware to force MSIS or HSI oscillator selection when exiting Stop mode or in case of HSE oscillator failure, depending on STOPWUCK value."]
        #[inline(always)]
        pub const fn sw(&self) -> super::vals::Sw {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "system clock switch Set and cleared by software to select system clock source (SYSCLK). Configured by hardware to force MSIS oscillator selection when exiting Standby or Shutdown mode. Configured by hardware to force MSIS or HSI oscillator selection when exiting Stop mode or in case of HSE oscillator failure, depending on STOPWUCK value."]
        #[inline(always)]
        pub fn set_sw(&mut self, val: super::vals::Sw) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "system clock switch status Set and cleared by hardware to indicate which clock source is used as system clock."]
        #[inline(always)]
        pub const fn sws(&self) -> super::vals::Sw {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "system clock switch status Set and cleared by hardware to indicate which clock source is used as system clock."]
        #[inline(always)]
        pub fn set_sws(&mut self, val: super::vals::Sw) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "wakeup from Stop and CSS backup clock selection Set and cleared by software to select the system clock used when exiting Stop mode. The selected clock is also used as emergency clock for the clock security system on HSE. Warning: STOPWUCK must not be modified when the CSS is enabled by HSECSSON bit in RCC_CR and the system clock is HSE (SWS = 10) or a switch on HSE is requested (SW = 10)."]
        #[inline(always)]
        pub const fn stopwuck(&self) -> super::vals::Stopwuck {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Stopwuck::from_bits(val as u8)
        }
        #[doc = "wakeup from Stop and CSS backup clock selection Set and cleared by software to select the system clock used when exiting Stop mode. The selected clock is also used as emergency clock for the clock security system on HSE. Warning: STOPWUCK must not be modified when the CSS is enabled by HSECSSON bit in RCC_CR and the system clock is HSE (SWS = 10) or a switch on HSE is requested (SW = 10)."]
        #[inline(always)]
        pub fn set_stopwuck(&mut self, val: super::vals::Stopwuck) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "wakeup from Stop kernel clock automatic enable selection Set and cleared by software to enable automatically another oscillator when exiting Stop mode. This oscillator can be used as independent kernel clock by peripherals."]
        #[inline(always)]
        pub const fn stopkerwuck(&self) -> super::vals::Stopkerwuck {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Stopkerwuck::from_bits(val as u8)
        }
        #[doc = "wakeup from Stop kernel clock automatic enable selection Set and cleared by software to enable automatically another oscillator when exiting Stop mode. This oscillator can be used as independent kernel clock by peripherals."]
        #[inline(always)]
        pub fn set_stopkerwuck(&mut self, val: super::vals::Stopkerwuck) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "microcontroller clock output Set and cleared by software. Others: reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching."]
        #[inline(always)]
        pub const fn mcosel(&self) -> super::vals::Mcosel {
            let val = (self.0 >> 24usize) & 0x0f;
            super::vals::Mcosel::from_bits(val as u8)
        }
        #[doc = "microcontroller clock output Set and cleared by software. Others: reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching."]
        #[inline(always)]
        pub fn set_mcosel(&mut self, val: super::vals::Mcosel) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
        }
        #[doc = "microcontroller clock output prescaler Set and cleared by software. It is highly recommended to change this prescaler before MCO output is enabled. Others: not allowed"]
        #[inline(always)]
        pub const fn mcopre(&self) -> super::vals::Mcopre {
            let val = (self.0 >> 28usize) & 0x07;
            super::vals::Mcopre::from_bits(val as u8)
        }
        #[doc = "microcontroller clock output prescaler Set and cleared by software. It is highly recommended to change this prescaler before MCO output is enabled. Others: not allowed"]
        #[inline(always)]
        pub fn set_mcopre(&mut self, val: super::vals::Mcopre) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Cfgr1 {
        #[inline(always)]
        fn default() -> Cfgr1 {
            Cfgr1(0)
        }
    }
    impl core::fmt::Debug for Cfgr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr1")
                .field("sw", &self.sw())
                .field("sws", &self.sws())
                .field("stopwuck", &self.stopwuck())
                .field("stopkerwuck", &self.stopkerwuck())
                .field("mcosel", &self.mcosel())
                .field("mcopre", &self.mcopre())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cfgr1 {
                sw: super::vals::Sw,
                sws: super::vals::Sw,
                stopwuck: super::vals::Stopwuck,
                stopkerwuck: super::vals::Stopkerwuck,
                mcosel: super::vals::Mcosel,
                mcopre: super::vals::Mcopre,
            }
            let proxy = Cfgr1 {
                sw: self.sw(),
                sws: self.sws(),
                stopwuck: self.stopwuck(),
                stopkerwuck: self.stopkerwuck(),
                mcosel: self.mcosel(),
                mcopre: self.mcopre(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC clock configuration register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "AHB prescaler Set and cleared by software to control the division factor of the AHB clock (HCLK). Depending on the device voltage range, the software must set these bits correctly to ensure that the system frequency does not exceed the maximum allowed frequency (for more details, refer to ). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account. 0xxx: SYSCLK not divided"]
        #[inline(always)]
        pub const fn hpre(&self) -> super::vals::Hpre {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Hpre::from_bits(val as u8)
        }
        #[doc = "AHB prescaler Set and cleared by software to control the division factor of the AHB clock (HCLK). Depending on the device voltage range, the software must set these bits correctly to ensure that the system frequency does not exceed the maximum allowed frequency (for more details, refer to ). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account. 0xxx: SYSCLK not divided"]
        #[inline(always)]
        pub fn set_hpre(&mut self, val: super::vals::Hpre) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "APB1 prescaler Set and cleared by software to control the division factor of the APB1 clock (PCLK1). 0xx: HCLK not divided"]
        #[inline(always)]
        pub const fn ppre1(&self) -> super::vals::Ppre {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB1 prescaler Set and cleared by software to control the division factor of the APB1 clock (PCLK1). 0xx: HCLK not divided"]
        #[inline(always)]
        pub fn set_ppre1(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "APB2 prescaler Set and cleared by software to control the division factor of the APB2 clock (PCLK2). 0xx: HCLK not divided"]
        #[inline(always)]
        pub const fn ppre2(&self) -> super::vals::Ppre {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB2 prescaler Set and cleared by software to control the division factor of the APB2 clock (PCLK2). 0xx: HCLK not divided"]
        #[inline(always)]
        pub fn set_ppre2(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "DSI PHY prescaler This bitfiled is set and cleared by software to control the division factor of DSI PHY bus clock (DCLK). 0xx: DCLK not divided Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value."]
        #[inline(always)]
        pub const fn dpre(&self) -> super::vals::Dpre {
            let val = (self.0 >> 12usize) & 0x07;
            super::vals::Dpre::from_bits(val as u8)
        }
        #[doc = "DSI PHY prescaler This bitfiled is set and cleared by software to control the division factor of DSI PHY bus clock (DCLK). 0xx: DCLK not divided Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the device datasheet for availability of its associated peripheral. If not present, consider this bitfield as reserved and keep it at reset value."]
        #[inline(always)]
        pub fn set_dpre(&mut self, val: super::vals::Dpre) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
        }
        #[doc = "AHB1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB1 peripherals (except those listed hereafter) are used and when their clocks are disabled in RCC_AHB1ENR. When this bit is set, all the AHB1 peripherals clocks are off, except for FLASH, BKPSRAM, ICACHE, DCACHE1 and SRAM1."]
        #[inline(always)]
        pub const fn ahb1dis(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "AHB1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB1 peripherals (except those listed hereafter) are used and when their clocks are disabled in RCC_AHB1ENR. When this bit is set, all the AHB1 peripherals clocks are off, except for FLASH, BKPSRAM, ICACHE, DCACHE1 and SRAM1."]
        #[inline(always)]
        pub fn set_ahb1dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "AHB2_1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR1 (except SRAM2 and SRAM3) are used and when their clocks are disabled in RCC_AHB2ENR1. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR1 are off, except for SRAM2 and SRAM3."]
        #[inline(always)]
        pub const fn ahb2dis1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "AHB2_1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR1 (except SRAM2 and SRAM3) are used and when their clocks are disabled in RCC_AHB2ENR1. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR1 are off, except for SRAM2 and SRAM3."]
        #[inline(always)]
        pub fn set_ahb2dis1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "AHB2_2 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR2 are used and when their clocks are disabled in RCC_AHB2ENR2. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2EBNR2 are off."]
        #[inline(always)]
        pub const fn ahb2dis2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "AHB2_2 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR2 are used and when their clocks are disabled in RCC_AHB2ENR2. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2EBNR2 are off."]
        #[inline(always)]
        pub fn set_ahb2dis2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "APB1 clock disable This bit can be set in order to further reduce power consumption, when none of the APB1 peripherals (except IWDG) are used and when their clocks are disabled in RCC_APB1ENR. When this bit is set, all the APB1 peripherals clocks are off, except for IWDG."]
        #[inline(always)]
        pub const fn apb1dis(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "APB1 clock disable This bit can be set in order to further reduce power consumption, when none of the APB1 peripherals (except IWDG) are used and when their clocks are disabled in RCC_APB1ENR. When this bit is set, all the APB1 peripherals clocks are off, except for IWDG."]
        #[inline(always)]
        pub fn set_apb1dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "APB2 clock disable This bit can be set in order to further reduce power consumption, when none of the APB2 peripherals are used and when their clocks are disabled in RCC_APB2ENR. When this bit is set, all the APB2 peripherals clocks are off."]
        #[inline(always)]
        pub const fn apb2dis(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "APB2 clock disable This bit can be set in order to further reduce power consumption, when none of the APB2 peripherals are used and when their clocks are disabled in RCC_APB2ENR. When this bit is set, all the APB2 peripherals clocks are off."]
        #[inline(always)]
        pub fn set_apb2dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Cfgr2 {
        #[inline(always)]
        fn default() -> Cfgr2 {
            Cfgr2(0)
        }
    }
    impl core::fmt::Debug for Cfgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr2")
                .field("hpre", &self.hpre())
                .field("ppre1", &self.ppre1())
                .field("ppre2", &self.ppre2())
                .field("dpre", &self.dpre())
                .field("ahb1dis", &self.ahb1dis())
                .field("ahb2dis1", &self.ahb2dis1())
                .field("ahb2dis2", &self.ahb2dis2())
                .field("apb1dis", &self.apb1dis())
                .field("apb2dis", &self.apb2dis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cfgr2 {
                hpre: super::vals::Hpre,
                ppre1: super::vals::Ppre,
                ppre2: super::vals::Ppre,
                dpre: super::vals::Dpre,
                ahb1dis: bool,
                ahb2dis1: bool,
                ahb2dis2: bool,
                apb1dis: bool,
                apb2dis: bool,
            }
            let proxy = Cfgr2 {
                hpre: self.hpre(),
                ppre1: self.ppre1(),
                ppre2: self.ppre2(),
                dpre: self.dpre(),
                ahb1dis: self.ahb1dis(),
                ahb2dis1: self.ahb2dis1(),
                ahb2dis2: self.ahb2dis2(),
                apb1dis: self.apb1dis(),
                apb2dis: self.apb2dis(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC clock configuration register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr3(pub u32);
    impl Cfgr3 {
        #[doc = "APB3 prescaler Set and cleared by software to control the division factor of the APB3 clock (PCLK3). 0xx: HCLK not divided"]
        #[inline(always)]
        pub const fn ppre3(&self) -> super::vals::Ppre {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB3 prescaler Set and cleared by software to control the division factor of the APB3 clock (PCLK3). 0xx: HCLK not divided"]
        #[inline(always)]
        pub fn set_ppre3(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "AHB3 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB3 peripherals (except SRAM4) are used and when their clocks are disabled in RCC_AHB3ENR. When this bit is set, all the AHB3 peripherals clocks are off, except for SRAM4."]
        #[inline(always)]
        pub const fn ahb3dis(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "AHB3 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB3 peripherals (except SRAM4) are used and when their clocks are disabled in RCC_AHB3ENR. When this bit is set, all the AHB3 peripherals clocks are off, except for SRAM4."]
        #[inline(always)]
        pub fn set_ahb3dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "APB3 clock disable This bit can be set in order to further reduce power consumption, when none of the APB3 peripherals from RCC_APB3ENR are used and when their clocks are disabled in RCC_APB3ENR. When this bit is set, all the APB3 peripherals clocks are off."]
        #[inline(always)]
        pub const fn apb3dis(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "APB3 clock disable This bit can be set in order to further reduce power consumption, when none of the APB3 peripherals from RCC_APB3ENR are used and when their clocks are disabled in RCC_APB3ENR. When this bit is set, all the APB3 peripherals clocks are off."]
        #[inline(always)]
        pub fn set_apb3dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Cfgr3 {
        #[inline(always)]
        fn default() -> Cfgr3 {
            Cfgr3(0)
        }
    }
    impl core::fmt::Debug for Cfgr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr3")
                .field("ppre3", &self.ppre3())
                .field("ahb3dis", &self.ahb3dis())
                .field("apb3dis", &self.apb3dis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr3 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cfgr3 {
                ppre3: super::vals::Ppre,
                ahb3dis: bool,
                apb3dis: bool,
            }
            let proxy = Cfgr3 {
                ppre3: self.ppre3(),
                ahb3dis: self.ahb3dis(),
                apb3dis: self.apb3dis(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC clock interrupt clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cicr(pub u32);
    impl Cicr {
        #[doc = "LSI ready interrupt clear Writing this bit to 1 clears the LSIRDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub const fn lsirdyc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt clear Writing this bit to 1 clears the LSIRDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub fn set_lsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt clear Writing this bit to 1 clears the LSERDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub const fn lserdyc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt clear Writing this bit to 1 clears the LSERDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub fn set_lserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MSIS ready interrupt clear Writing this bit to 1 clears the MSISRDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub const fn msisrdyc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "MSIS ready interrupt clear Writing this bit to 1 clears the MSISRDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub fn set_msisrdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSI ready interrupt clear Writing this bit to 1 clears the HSIRDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub const fn hsirdyc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt clear Writing this bit to 1 clears the HSIRDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub fn set_hsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HSE ready interrupt clear Writing this bit to 1 clears the HSERDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub const fn hserdyc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt clear Writing this bit to 1 clears the HSERDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub fn set_hserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "HSI48 ready interrupt clear Writing this bit to 1 clears the HSI48RDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub const fn hsi48rdyc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 ready interrupt clear Writing this bit to 1 clears the HSI48RDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub fn set_hsi48rdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PLL1 ready interrupt clear Writing this bit to 1 clears the PLL1RDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub const fn pllrdyc(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 ready interrupt clear Writing this bit to 1 clears the PLL1RDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub fn set_pllrdyc(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Clock security system interrupt clear Writing this bit to 1 clears the CSSF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub const fn cssc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Clock security system interrupt clear Writing this bit to 1 clears the CSSF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub fn set_cssc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "MSIK oscillator ready interrupt clear Writing this bit to 1 clears the MSIKRDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub const fn msikrdyc(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "MSIK oscillator ready interrupt clear Writing this bit to 1 clears the MSIKRDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub fn set_msikrdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SHSI oscillator ready interrupt clear Writing this bit to 1 clears the SHSIRDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub const fn shsirdyc(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SHSI oscillator ready interrupt clear Writing this bit to 1 clears the SHSIRDYF flag. Writing 0 has no effect."]
        #[inline(always)]
        pub fn set_shsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
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
                .field("msisrdyc", &self.msisrdyc())
                .field("hsirdyc", &self.hsirdyc())
                .field("hserdyc", &self.hserdyc())
                .field("hsi48rdyc", &self.hsi48rdyc())
                .field(
                    "pllrdyc",
                    &[self.pllrdyc(0usize), self.pllrdyc(1usize), self.pllrdyc(2usize)],
                )
                .field("cssc", &self.cssc())
                .field("msikrdyc", &self.msikrdyc())
                .field("shsirdyc", &self.shsirdyc())
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
                msisrdyc: bool,
                hsirdyc: bool,
                hserdyc: bool,
                hsi48rdyc: bool,
                pllrdyc: [bool; 3usize],
                cssc: bool,
                msikrdyc: bool,
                shsirdyc: bool,
            }
            let proxy = Cicr {
                lsirdyc: self.lsirdyc(),
                lserdyc: self.lserdyc(),
                msisrdyc: self.msisrdyc(),
                hsirdyc: self.hsirdyc(),
                hserdyc: self.hserdyc(),
                hsi48rdyc: self.hsi48rdyc(),
                pllrdyc: [self.pllrdyc(0usize), self.pllrdyc(1usize), self.pllrdyc(2usize)],
                cssc: self.cssc(),
                msikrdyc: self.msikrdyc(),
                shsirdyc: self.shsirdyc(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC clock interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cier(pub u32);
    impl Cier {
        #[doc = "LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization."]
        #[inline(always)]
        pub const fn lsirdyie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI oscillator stabilization."]
        #[inline(always)]
        pub fn set_lsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization."]
        #[inline(always)]
        pub const fn lserdyie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization."]
        #[inline(always)]
        pub fn set_lserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MSIS ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the MSIS oscillator stabilization."]
        #[inline(always)]
        pub const fn msisrdyie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "MSIS ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the MSIS oscillator stabilization."]
        #[inline(always)]
        pub fn set_msisrdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI oscillator stabilization."]
        #[inline(always)]
        pub const fn hsirdyie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI oscillator stabilization."]
        #[inline(always)]
        pub fn set_hsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization."]
        #[inline(always)]
        pub const fn hserdyie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization."]
        #[inline(always)]
        pub fn set_hserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "HSI48 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI48 oscillator stabilization."]
        #[inline(always)]
        pub const fn hsi48rdyie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI48 oscillator stabilization."]
        #[inline(always)]
        pub fn set_hsi48rdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PLL ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL1 lock."]
        #[inline(always)]
        pub const fn pllrdyie(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL1 lock."]
        #[inline(always)]
        pub fn set_pllrdyie(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "MSIK ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the MSIK oscillator stabilization."]
        #[inline(always)]
        pub const fn msikrdyie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "MSIK ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the MSIK oscillator stabilization."]
        #[inline(always)]
        pub fn set_msikrdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SHSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the SHSI oscillator stabilization."]
        #[inline(always)]
        pub const fn shsirdyie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SHSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the SHSI oscillator stabilization."]
        #[inline(always)]
        pub fn set_shsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
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
                .field("msisrdyie", &self.msisrdyie())
                .field("hsirdyie", &self.hsirdyie())
                .field("hserdyie", &self.hserdyie())
                .field("hsi48rdyie", &self.hsi48rdyie())
                .field(
                    "pllrdyie",
                    &[self.pllrdyie(0usize), self.pllrdyie(1usize), self.pllrdyie(2usize)],
                )
                .field("msikrdyie", &self.msikrdyie())
                .field("shsirdyie", &self.shsirdyie())
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
                msisrdyie: bool,
                hsirdyie: bool,
                hserdyie: bool,
                hsi48rdyie: bool,
                pllrdyie: [bool; 3usize],
                msikrdyie: bool,
                shsirdyie: bool,
            }
            let proxy = Cier {
                lsirdyie: self.lsirdyie(),
                lserdyie: self.lserdyie(),
                msisrdyie: self.msisrdyie(),
                hsirdyie: self.hsirdyie(),
                hserdyie: self.hserdyie(),
                hsi48rdyie: self.hsi48rdyie(),
                pllrdyie: [self.pllrdyie(0usize), self.pllrdyie(1usize), self.pllrdyie(2usize)],
                msikrdyie: self.msikrdyie(),
                shsirdyie: self.shsirdyie(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC clock interrupt flag register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cifr(pub u32);
    impl Cifr {
        #[doc = "LSI ready interrupt flag Set by hardware when the LSI clock becomes stable and LSIRDYIE is set. Cleared by software setting the LSIRDYC bit."]
        #[inline(always)]
        pub const fn lsirdyf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt flag Set by hardware when the LSI clock becomes stable and LSIRDYIE is set. Cleared by software setting the LSIRDYC bit."]
        #[inline(always)]
        pub fn set_lsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt flag Set by hardware when the LSE clock becomes stable and LSERDYIE is set. Cleared by software setting the LSERDYC bit."]
        #[inline(always)]
        pub const fn lserdyf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt flag Set by hardware when the LSE clock becomes stable and LSERDYIE is set. Cleared by software setting the LSERDYC bit."]
        #[inline(always)]
        pub fn set_lserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MSIS ready interrupt flag Set by hardware when the MSIS clock becomes stable and MSISRDYIE is set. Cleared by software setting the MSISRDYC bit."]
        #[inline(always)]
        pub const fn msisrdyf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "MSIS ready interrupt flag Set by hardware when the MSIS clock becomes stable and MSISRDYIE is set. Cleared by software setting the MSISRDYC bit."]
        #[inline(always)]
        pub fn set_msisrdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSI ready interrupt flag Set by hardware when the HSI clock becomes stable and HSIRDYIE is set in a response to setting the HSION (see RCC_CR). When HSION is not set but the HSI oscillator is enabled by the peripheral through a clock request, this bit is not set and no interrupt is generated. Cleared by software setting the HSIRDYC bit."]
        #[inline(always)]
        pub const fn hsirdyf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt flag Set by hardware when the HSI clock becomes stable and HSIRDYIE is set in a response to setting the HSION (see RCC_CR). When HSION is not set but the HSI oscillator is enabled by the peripheral through a clock request, this bit is not set and no interrupt is generated. Cleared by software setting the HSIRDYC bit."]
        #[inline(always)]
        pub fn set_hsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HSE ready interrupt flag Set by hardware when the HSE clock becomes stable and HSERDYIE is set. Cleared by software setting the HSERDYC bit."]
        #[inline(always)]
        pub const fn hserdyf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt flag Set by hardware when the HSE clock becomes stable and HSERDYIE is set. Cleared by software setting the HSERDYC bit."]
        #[inline(always)]
        pub fn set_hserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "HSI48 ready interrupt flag Set by hardware when the HSI48 clock becomes stable and HSI48RDYIE is set. Cleared by software setting the HSI48RDYC bit."]
        #[inline(always)]
        pub const fn hsi48rdyf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 ready interrupt flag Set by hardware when the HSI48 clock becomes stable and HSI48RDYIE is set. Cleared by software setting the HSI48RDYC bit."]
        #[inline(always)]
        pub fn set_hsi48rdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PLL1 ready interrupt flag Set by hardware when the PLL1 locks and PLL1RDYIE is set. Cleared by software setting the PLL1RDYC bit."]
        #[inline(always)]
        pub const fn pllrdyf(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 ready interrupt flag Set by hardware when the PLL1 locks and PLL1RDYIE is set. Cleared by software setting the PLL1RDYC bit."]
        #[inline(always)]
        pub fn set_pllrdyf(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 6usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Clock security system interrupt flag Set by hardware when a failure is detected in the HSE oscillator. Cleared by software setting the CSSC bit."]
        #[inline(always)]
        pub const fn cssf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Clock security system interrupt flag Set by hardware when a failure is detected in the HSE oscillator. Cleared by software setting the CSSC bit."]
        #[inline(always)]
        pub fn set_cssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "MSIK ready interrupt flag Set by hardware when the MSIK clock becomes stable and MSIKRDYIE is set. Cleared by software setting the MSIKRDYC bit."]
        #[inline(always)]
        pub const fn msikrdyf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "MSIK ready interrupt flag Set by hardware when the MSIK clock becomes stable and MSIKRDYIE is set. Cleared by software setting the MSIKRDYC bit."]
        #[inline(always)]
        pub fn set_msikrdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SHSI ready interrupt flag Set by hardware when the SHSI clock becomes stable and SHSIRDYIE is set. Cleared by software setting the SHSIRDYC bit."]
        #[inline(always)]
        pub const fn shsirdyf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SHSI ready interrupt flag Set by hardware when the SHSI clock becomes stable and SHSIRDYIE is set. Cleared by software setting the SHSIRDYC bit."]
        #[inline(always)]
        pub fn set_shsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
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
                .field("msisrdyf", &self.msisrdyf())
                .field("hsirdyf", &self.hsirdyf())
                .field("hserdyf", &self.hserdyf())
                .field("hsi48rdyf", &self.hsi48rdyf())
                .field(
                    "pllrdyf",
                    &[self.pllrdyf(0usize), self.pllrdyf(1usize), self.pllrdyf(2usize)],
                )
                .field("cssf", &self.cssf())
                .field("msikrdyf", &self.msikrdyf())
                .field("shsirdyf", &self.shsirdyf())
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
                msisrdyf: bool,
                hsirdyf: bool,
                hserdyf: bool,
                hsi48rdyf: bool,
                pllrdyf: [bool; 3usize],
                cssf: bool,
                msikrdyf: bool,
                shsirdyf: bool,
            }
            let proxy = Cifr {
                lsirdyf: self.lsirdyf(),
                lserdyf: self.lserdyf(),
                msisrdyf: self.msisrdyf(),
                hsirdyf: self.hsirdyf(),
                hserdyf: self.hserdyf(),
                hsi48rdyf: self.hsi48rdyf(),
                pllrdyf: [self.pllrdyf(0usize), self.pllrdyf(1usize), self.pllrdyf(2usize)],
                cssf: self.cssf(),
                msikrdyf: self.msikrdyf(),
                shsirdyf: self.shsirdyf(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC clock control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "MSIS clock enable Set and cleared by software. Cleared by hardware to stop the MSIS oscillator when entering Stop, Standby or Shutdown mode. Set by hardware to force the MSIS oscillator ON when exiting Standby or Shutdown mode. Set by hardware to force the MSIS oscillator ON when STOPWUCK = 0 when exiting Stop modes or in case of a failure of the HSE oscillator. Set by hardware when used directly or indirectly as system clock."]
        #[inline(always)]
        pub const fn msison(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MSIS clock enable Set and cleared by software. Cleared by hardware to stop the MSIS oscillator when entering Stop, Standby or Shutdown mode. Set by hardware to force the MSIS oscillator ON when exiting Standby or Shutdown mode. Set by hardware to force the MSIS oscillator ON when STOPWUCK = 0 when exiting Stop modes or in case of a failure of the HSE oscillator. Set by hardware when used directly or indirectly as system clock."]
        #[inline(always)]
        pub fn set_msison(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "MSI enable for some peripheral kernels Set and cleared by software to force MSI ON even in Stop modes. Keeping the MSI ON in Stop mode allows the communication speed not to be reduced by the MSI startup time. This bit has no effect on MSISON and MSIKON values (see autonomous mode for more details). The MSIKERON must be configured at 0 before entering Stop 3 mode."]
        #[inline(always)]
        pub const fn msikeron(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "MSI enable for some peripheral kernels Set and cleared by software to force MSI ON even in Stop modes. Keeping the MSI ON in Stop mode allows the communication speed not to be reduced by the MSI startup time. This bit has no effect on MSISON and MSIKON values (see autonomous mode for more details). The MSIKERON must be configured at 0 before entering Stop 3 mode."]
        #[inline(always)]
        pub fn set_msikeron(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MSIS clock ready flag Set by hardware to indicate that the MSIS oscillator is stable. This bit is set only when MSIS is enabled by software by setting MSISON. Note: Once the MSISON bit is cleared, MSISRDY goes low after six MSIS clock cycles."]
        #[inline(always)]
        pub const fn msisrdy(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "MSIS clock ready flag Set by hardware to indicate that the MSIS oscillator is stable. This bit is set only when MSIS is enabled by software by setting MSISON. Note: Once the MSISON bit is cleared, MSISRDY goes low after six MSIS clock cycles."]
        #[inline(always)]
        pub fn set_msisrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "MSI clock PLL-mode enable Set and cleared by software to enable/disable the PLL part of the MSI clock source. MSIPLLEN must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware). A hardware protection prevents from enabling MSIPLLEN if LSE is not ready. This bit is cleared by hardware when LSE is disabled (LSEON = 0) or when the CSS on LSE detects a LSE failure (see RCC_CSR)."]
        #[inline(always)]
        pub const fn msipllen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "MSI clock PLL-mode enable Set and cleared by software to enable/disable the PLL part of the MSI clock source. MSIPLLEN must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware). A hardware protection prevents from enabling MSIPLLEN if LSE is not ready. This bit is cleared by hardware when LSE is disabled (LSEON = 0) or when the CSS on LSE detects a LSE failure (see RCC_CSR)."]
        #[inline(always)]
        pub fn set_msipllen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "MSIK clock enable Set and cleared by software. Cleared by hardware to stop the MSIK when entering Stop, Standby or Shutdown mode. Set by hardware to force the MSIK oscillator ON when exiting Standby or Shutdown mode. Set by hardware to force the MSIK oscillator ON when STOPWUCK = 0 or STOPKERWUCK = 0 when exiting Stop modes or in case of a failure of the HSE oscillator."]
        #[inline(always)]
        pub const fn msikon(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "MSIK clock enable Set and cleared by software. Cleared by hardware to stop the MSIK when entering Stop, Standby or Shutdown mode. Set by hardware to force the MSIK oscillator ON when exiting Standby or Shutdown mode. Set by hardware to force the MSIK oscillator ON when STOPWUCK = 0 or STOPKERWUCK = 0 when exiting Stop modes or in case of a failure of the HSE oscillator."]
        #[inline(always)]
        pub fn set_msikon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MSIK clock ready flag Set by hardware to indicate that the MSIK is stable. This bit is set only when MSI kernel oscillator is enabled by software by setting MSIKON. Note: Once the MSIKON bit is cleared, MSIKRDY goes low after six MSIK oscillator clock cycles."]
        #[inline(always)]
        pub const fn msikrdy(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MSIK clock ready flag Set by hardware to indicate that the MSIK is stable. This bit is set only when MSI kernel oscillator is enabled by software by setting MSIKON. Note: Once the MSIKON bit is cleared, MSIKRDY goes low after six MSIK oscillator clock cycles."]
        #[inline(always)]
        pub fn set_msikrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "MSI clock with PLL mode selection Set and cleared by software to select which MSI output clock uses the PLL mode. This bit can be written only when the MSI PLL mode is disabled (MSIPLLEN = 0). Note: If the MSI kernel clock output uses the same oscillator source than the MSI system clock output, then the PLL mode is applied to the both clocks outputs."]
        #[inline(always)]
        pub const fn msipllsel(&self) -> super::vals::Msipllsel {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Msipllsel::from_bits(val as u8)
        }
        #[doc = "MSI clock with PLL mode selection Set and cleared by software to select which MSI output clock uses the PLL mode. This bit can be written only when the MSI PLL mode is disabled (MSIPLLEN = 0). Note: If the MSI kernel clock output uses the same oscillator source than the MSI system clock output, then the PLL mode is applied to the both clocks outputs."]
        #[inline(always)]
        pub fn set_msipllsel(&mut self, val: super::vals::Msipllsel) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "MSI PLL mode fast startup Set and reset by software to enable/disable the fast PLL mode start-up of the MSI clock source. This bit is used only if PLL mode is selected (MSIPLLEN = 1). The fast start-up feature is not active the first time the PLL mode is selected. The fast start-up is active when the MSI in PLL mode returns from switch off."]
        #[inline(always)]
        pub const fn msipllfast(&self) -> super::vals::Msipllfast {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Msipllfast::from_bits(val as u8)
        }
        #[doc = "MSI PLL mode fast startup Set and reset by software to enable/disable the fast PLL mode start-up of the MSI clock source. This bit is used only if PLL mode is selected (MSIPLLEN = 1). The fast start-up feature is not active the first time the PLL mode is selected. The fast start-up is active when the MSI in PLL mode returns from switch off."]
        #[inline(always)]
        pub fn set_msipllfast(&mut self, val: super::vals::Msipllfast) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "HSI clock enable Set and cleared by software. Cleared by hardware to stop the HSI oscillator when entering Stop, Standby or Shutdown mode. Set by hardware to force the HSI oscillator ON when STOPWUCK = 1 when leaving Stop modes, or in case of failure of the HSE crystal oscillator. This bit is set by hardware if the HSI is used directly or indirectly as system clock."]
        #[inline(always)]
        pub const fn hsion(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "HSI clock enable Set and cleared by software. Cleared by hardware to stop the HSI oscillator when entering Stop, Standby or Shutdown mode. Set by hardware to force the HSI oscillator ON when STOPWUCK = 1 when leaving Stop modes, or in case of failure of the HSE crystal oscillator. This bit is set by hardware if the HSI is used directly or indirectly as system clock."]
        #[inline(always)]
        pub fn set_hsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "HSI enable for some peripheral kernels Set and cleared by software to force HSI ON even in Stop modes. Keeping the HSI ON in Stop mode allows the communication speed not to be reduced by the HSI startup time. This bit has no effect on HSION value. Refer to for more details. The HSIKERON must be configured at 0 before entering Stop 3 mode."]
        #[inline(always)]
        pub const fn hsikeron(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "HSI enable for some peripheral kernels Set and cleared by software to force HSI ON even in Stop modes. Keeping the HSI ON in Stop mode allows the communication speed not to be reduced by the HSI startup time. This bit has no effect on HSION value. Refer to for more details. The HSIKERON must be configured at 0 before entering Stop 3 mode."]
        #[inline(always)]
        pub fn set_hsikeron(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "HSI clock ready flag Set by hardware to indicate that HSI oscillator is stable. This bit is set only when HSI is enabled by software by setting HSION. Note: Once the HSION bit is cleared, HSIRDY goes low after six HSI clock cycles."]
        #[inline(always)]
        pub const fn hsirdy(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HSI clock ready flag Set by hardware to indicate that HSI oscillator is stable. This bit is set only when HSI is enabled by software by setting HSION. Note: Once the HSION bit is cleared, HSIRDY goes low after six HSI clock cycles."]
        #[inline(always)]
        pub fn set_hsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "HSI48 clock enable Set and cleared by software. Cleared by hardware to stop the HSI48 when entering in Stop, Standby or Shutdown modes."]
        #[inline(always)]
        pub const fn hsi48on(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 clock enable Set and cleared by software. Cleared by hardware to stop the HSI48 when entering in Stop, Standby or Shutdown modes."]
        #[inline(always)]
        pub fn set_hsi48on(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "HSI48 clock ready flag Set by hardware to indicate that HSI48 oscillator is stable. This bit is set only when HSI48 is enabled by software by setting HSI48ON."]
        #[inline(always)]
        pub const fn hsi48rdy(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 clock ready flag Set by hardware to indicate that HSI48 oscillator is stable. This bit is set only when HSI48 is enabled by software by setting HSI48ON."]
        #[inline(always)]
        pub fn set_hsi48rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "SHSI clock enable Set and cleared by software. Cleared by hardware to stop the SHSI when entering in Stop, Standby or Shutdown modes."]
        #[inline(always)]
        pub const fn shsion(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SHSI clock enable Set and cleared by software. Cleared by hardware to stop the SHSI when entering in Stop, Standby or Shutdown modes."]
        #[inline(always)]
        pub fn set_shsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SHSI clock ready flag Set by hardware to indicate that the SHSI oscillator is stable. This bit is set only when SHSI is enabled by software by setting SHSION. Note: Once the SHSION bit is cleared, SHSIRDY goes low after six SHSI clock cycles."]
        #[inline(always)]
        pub const fn shsirdy(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SHSI clock ready flag Set by hardware to indicate that the SHSI oscillator is stable. This bit is set only when SHSI is enabled by software by setting SHSION. Note: Once the SHSION bit is cleared, SHSIRDY goes low after six SHSI clock cycles."]
        #[inline(always)]
        pub fn set_shsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE oscillator when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock."]
        #[inline(always)]
        pub const fn hseon(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE oscillator when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock."]
        #[inline(always)]
        pub fn set_hseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "HSE clock ready flag Set by hardware to indicate that the HSE oscillator is stable. Note: Once the HSEON bit is cleared, HSERDY goes low after six HSE clock cycles."]
        #[inline(always)]
        pub const fn hserdy(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock ready flag Set by hardware to indicate that the HSE oscillator is stable. Note: Once the HSEON bit is cleared, HSERDY goes low after six HSE clock cycles."]
        #[inline(always)]
        pub fn set_hserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "HSE crystal oscillator bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit set, to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled."]
        #[inline(always)]
        pub const fn hsebyp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "HSE crystal oscillator bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit set, to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled."]
        #[inline(always)]
        pub fn set_hsebyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Clock security system enable Set by software to enable the clock security system. When CSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset."]
        #[inline(always)]
        pub const fn csson(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Clock security system enable Set by software to enable the clock security system. When CSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready, and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset."]
        #[inline(always)]
        pub fn set_csson(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "HSE external clock bypass mode Set and reset by software to select the external clock mode in bypass mode. External clock mode must be configured with HSEON bit to be used by the device. This bit can be written only if the HSE oscillator is disabled. This bit is active only if the HSE bypass mode is enabled."]
        #[inline(always)]
        pub const fn hseext(&self) -> super::vals::Hseext {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Hseext::from_bits(val as u8)
        }
        #[doc = "HSE external clock bypass mode Set and reset by software to select the external clock mode in bypass mode. External clock mode must be configured with HSEON bit to be used by the device. This bit can be written only if the HSE oscillator is disabled. This bit is active only if the HSE bypass mode is enabled."]
        #[inline(always)]
        pub fn set_hseext(&mut self, val: super::vals::Hseext) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "PLL1 enable Set and cleared by software to enable the main PLL. Cleared by hardware when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the PLL1 clock is used as the system clock."]
        #[inline(always)]
        pub const fn pllon(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 24usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 enable Set and cleared by software to enable the main PLL. Cleared by hardware when entering Stop, Standby or Shutdown mode. This bit cannot be reset if the PLL1 clock is used as the system clock."]
        #[inline(always)]
        pub fn set_pllon(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 24usize + n * 2usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "PLL1 clock ready flag Set by hardware to indicate that the PLL1 is locked."]
        #[inline(always)]
        pub const fn pllrdy(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 25usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 clock ready flag Set by hardware to indicate that the PLL1 is locked."]
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
                .field("msison", &self.msison())
                .field("msikeron", &self.msikeron())
                .field("msisrdy", &self.msisrdy())
                .field("msipllen", &self.msipllen())
                .field("msikon", &self.msikon())
                .field("msikrdy", &self.msikrdy())
                .field("msipllsel", &self.msipllsel())
                .field("msipllfast", &self.msipllfast())
                .field("hsion", &self.hsion())
                .field("hsikeron", &self.hsikeron())
                .field("hsirdy", &self.hsirdy())
                .field("hsi48on", &self.hsi48on())
                .field("hsi48rdy", &self.hsi48rdy())
                .field("shsion", &self.shsion())
                .field("shsirdy", &self.shsirdy())
                .field("hseon", &self.hseon())
                .field("hserdy", &self.hserdy())
                .field("hsebyp", &self.hsebyp())
                .field("csson", &self.csson())
                .field("hseext", &self.hseext())
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
                msison: bool,
                msikeron: bool,
                msisrdy: bool,
                msipllen: bool,
                msikon: bool,
                msikrdy: bool,
                msipllsel: super::vals::Msipllsel,
                msipllfast: super::vals::Msipllfast,
                hsion: bool,
                hsikeron: bool,
                hsirdy: bool,
                hsi48on: bool,
                hsi48rdy: bool,
                shsion: bool,
                shsirdy: bool,
                hseon: bool,
                hserdy: bool,
                hsebyp: bool,
                csson: bool,
                hseext: super::vals::Hseext,
                pllon: [bool; 3usize],
                pllrdy: [bool; 3usize],
            }
            let proxy = Cr {
                msison: self.msison(),
                msikeron: self.msikeron(),
                msisrdy: self.msisrdy(),
                msipllen: self.msipllen(),
                msikon: self.msikon(),
                msikrdy: self.msikrdy(),
                msipllsel: self.msipllsel(),
                msipllfast: self.msipllfast(),
                hsion: self.hsion(),
                hsikeron: self.hsikeron(),
                hsirdy: self.hsirdy(),
                hsi48on: self.hsi48on(),
                hsi48rdy: self.hsi48rdy(),
                shsion: self.shsion(),
                shsirdy: self.shsirdy(),
                hseon: self.hseon(),
                hserdy: self.hserdy(),
                hsebyp: self.hsebyp(),
                csson: self.csson(),
                hseext: self.hseext(),
                pllon: [self.pllon(0usize), self.pllon(1usize), self.pllon(2usize)],
                pllrdy: [self.pllrdy(0usize), self.pllrdy(1usize), self.pllrdy(2usize)],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC clock recovery RC register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crrcr(pub u32);
    impl Crrcr {
        #[doc = "HSI48 clock calibration These bits are initialized at startup with the factory-programmed HSI48 calibration trim value."]
        #[inline(always)]
        pub const fn hsi48cal(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HSI48 clock calibration These bits are initialized at startup with the factory-programmed HSI48 calibration trim value."]
        #[inline(always)]
        pub fn set_hsi48cal(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
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
    #[doc = "RCC control/status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "MSIK range after Standby mode Set by software to chose the MSIK frequency at startup. This range is used after exiting Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting Shutdown mode, the range is always 4 MHz. MSIKSRANGE can be written only when MSIRGSEL = 1. others: reserved Note: Changing the MSIKSRANGE does not change the current MSIK frequency."]
        #[inline(always)]
        pub const fn msiksrange(&self) -> super::vals::Msixsrange {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::Msixsrange::from_bits(val as u8)
        }
        #[doc = "MSIK range after Standby mode Set by software to chose the MSIK frequency at startup. This range is used after exiting Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting Shutdown mode, the range is always 4 MHz. MSIKSRANGE can be written only when MSIRGSEL = 1. others: reserved Note: Changing the MSIKSRANGE does not change the current MSIK frequency."]
        #[inline(always)]
        pub fn set_msiksrange(&mut self, val: super::vals::Msixsrange) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
        }
        #[doc = "MSIS range after Standby mode Set by software to chose the MSIS frequency at startup. This range is used after exiting Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting Shutdown mode, the range is always 4 MHz. MSISSRANGE can be written only when MSIRGSEL = 1. others: reserved Note: Changing the MSISSRANGE does not change the current MSIS frequency."]
        #[inline(always)]
        pub const fn msissrange(&self) -> super::vals::Msixsrange {
            let val = (self.0 >> 12usize) & 0x0f;
            super::vals::Msixsrange::from_bits(val as u8)
        }
        #[doc = "MSIS range after Standby mode Set by software to chose the MSIS frequency at startup. This range is used after exiting Standby mode until MSIRGSEL is set. After a NRST pin or a power-on reset or when exiting Shutdown mode, the range is always 4 MHz. MSISSRANGE can be written only when MSIRGSEL = 1. others: reserved Note: Changing the MSISSRANGE does not change the current MSIS frequency."]
        #[inline(always)]
        pub fn set_msissrange(&mut self, val: super::vals::Msixsrange) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
        }
        #[doc = "Remove reset flag Set by software to clear the reset flags."]
        #[inline(always)]
        pub const fn rmvf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Remove reset flag Set by software to clear the reset flags."]
        #[inline(always)]
        pub fn set_rmvf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Option byte loader reset flag Set by hardware when a reset from the option byte loading occurs. Cleared by writing to the RMVF bit."]
        #[inline(always)]
        pub const fn oblrstf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte loader reset flag Set by hardware when a reset from the option byte loading occurs. Cleared by writing to the RMVF bit."]
        #[inline(always)]
        pub fn set_oblrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "NRST pin reset flag Set by hardware when a reset from the NRST pin occurs. Cleared by writing to the RMVF bit."]
        #[inline(always)]
        pub const fn pinrstf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "NRST pin reset flag Set by hardware when a reset from the NRST pin occurs. Cleared by writing to the RMVF bit."]
        #[inline(always)]
        pub fn set_pinrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "BOR flag Set by hardware when a BOR occurs. Cleared by writing to the RMVF bit."]
        #[inline(always)]
        pub const fn borrstf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "BOR flag Set by hardware when a BOR occurs. Cleared by writing to the RMVF bit."]
        #[inline(always)]
        pub fn set_borrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Software reset flag Set by hardware when a software reset occurs. Cleared by writing to the RMVF bit."]
        #[inline(always)]
        pub const fn sftrstf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Software reset flag Set by hardware when a software reset occurs. Cleared by writing to the RMVF bit."]
        #[inline(always)]
        pub fn set_sftrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Independent watchdog reset flag Set by hardware when an independent watchdog reset domain occurs. Cleared by writing to the RMVF bit."]
        #[inline(always)]
        pub const fn iwdgrstf(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog reset flag Set by hardware when an independent watchdog reset domain occurs. Cleared by writing to the RMVF bit."]
        #[inline(always)]
        pub fn set_iwdgrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Window watchdog reset flag Set by hardware when a window watchdog reset occurs. Cleared by writing to the RMVF bit."]
        #[inline(always)]
        pub const fn wwdgrstf(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog reset flag Set by hardware when a window watchdog reset occurs. Cleared by writing to the RMVF bit."]
        #[inline(always)]
        pub fn set_wwdgrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Low-power reset flag Set by hardware when a reset occurs due to Stop, Standby or Shutdown mode entry, whereas the corresponding nRST_STOP, nRST_STBY or nRST_SHDW option bit is cleared. Cleared by writing to the RMVF bit."]
        #[inline(always)]
        pub const fn lpwrrstf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power reset flag Set by hardware when a reset occurs due to Stop, Standby or Shutdown mode entry, whereas the corresponding nRST_STOP, nRST_STBY or nRST_SHDW option bit is cleared. Cleared by writing to the RMVF bit."]
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
    impl core::fmt::Debug for Csr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csr")
                .field("msiksrange", &self.msiksrange())
                .field("msissrange", &self.msissrange())
                .field("rmvf", &self.rmvf())
                .field("oblrstf", &self.oblrstf())
                .field("pinrstf", &self.pinrstf())
                .field("borrstf", &self.borrstf())
                .field("sftrstf", &self.sftrstf())
                .field("iwdgrstf", &self.iwdgrstf())
                .field("wwdgrstf", &self.wwdgrstf())
                .field("lpwrrstf", &self.lpwrrstf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Csr {
                msiksrange: super::vals::Msixsrange,
                msissrange: super::vals::Msixsrange,
                rmvf: bool,
                oblrstf: bool,
                pinrstf: bool,
                borrstf: bool,
                sftrstf: bool,
                iwdgrstf: bool,
                wwdgrstf: bool,
                lpwrrstf: bool,
            }
            let proxy = Csr {
                msiksrange: self.msiksrange(),
                msissrange: self.msissrange(),
                rmvf: self.rmvf(),
                oblrstf: self.oblrstf(),
                pinrstf: self.pinrstf(),
                borrstf: self.borrstf(),
                sftrstf: self.sftrstf(),
                iwdgrstf: self.iwdgrstf(),
                wwdgrstf: self.wwdgrstf(),
                lpwrrstf: self.lpwrrstf(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC internal clock sources calibration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icscr1(pub u32);
    impl Icscr1 {
        #[doc = "MSIRC3 clock calibration for MSI ranges 12 to 15 These bits are initialized at startup with the factory-programmed MSIRC3 calibration trim value for ranges 12 to 15. When MSITRIM3 is written, MSICAL3 is updated with the sum of MSITRIM3\\[4:0\\]
and the factory calibration trim value MSIRC2\\[4:0\\]. There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level."]
        #[inline(always)]
        pub const fn msical3(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "MSIRC3 clock calibration for MSI ranges 12 to 15 These bits are initialized at startup with the factory-programmed MSIRC3 calibration trim value for ranges 12 to 15. When MSITRIM3 is written, MSICAL3 is updated with the sum of MSITRIM3\\[4:0\\]
and the factory calibration trim value MSIRC2\\[4:0\\]. There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level."]
        #[inline(always)]
        pub fn set_msical3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "MSIRC2 clock calibration for MSI ranges 8 to 11 These bits are initialized at startup with the factory-programmed MSIRC2 calibration trim value for ranges 8 to 11. When MSITRIM2 is written, MSICAL2 is updated with the sum of MSITRIM2\\[4:0\\]
and the factory calibration trim value MSIRC2\\[4:0\\]. There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level."]
        #[inline(always)]
        pub const fn msical2(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x1f;
            val as u8
        }
        #[doc = "MSIRC2 clock calibration for MSI ranges 8 to 11 These bits are initialized at startup with the factory-programmed MSIRC2 calibration trim value for ranges 8 to 11. When MSITRIM2 is written, MSICAL2 is updated with the sum of MSITRIM2\\[4:0\\]
and the factory calibration trim value MSIRC2\\[4:0\\]. There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level."]
        #[inline(always)]
        pub fn set_msical2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
        }
        #[doc = "MSIRC1 clock calibration for MSI ranges 4 to 7 These bits are initialized at startup with the factory-programmed MSIRC1 calibration trim value for ranges 4 to 7. When MSITRIM1 is written, MSICAL1 is updated with the sum of MSITRIM1\\[4:0\\]
and the factory calibration trim value MSIRC1\\[4:0\\]. There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level."]
        #[inline(always)]
        pub const fn msical1(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x1f;
            val as u8
        }
        #[doc = "MSIRC1 clock calibration for MSI ranges 4 to 7 These bits are initialized at startup with the factory-programmed MSIRC1 calibration trim value for ranges 4 to 7. When MSITRIM1 is written, MSICAL1 is updated with the sum of MSITRIM1\\[4:0\\]
and the factory calibration trim value MSIRC1\\[4:0\\]. There is no hardware protection to limit a potential overflow due to the addition of MSITRIM bitfield and factory program bitfield for this calibration value. Control must be managed by software at user level."]
        #[inline(always)]
        pub fn set_msical1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
        }
        #[doc = "MSIRC0 clock calibration for MSI ranges 0 to 3 These bits are initialized at startup with the factory-programmed MSIRC0 calibration trim value for ranges 0 to 3. When MSITRIM0 is written, MSICAL0 is updated with the sum of MSITRIM0\\[4:0\\]
and the factory-programmed calibration trim value MSIRC0\\[4:0\\]."]
        #[inline(always)]
        pub const fn msical0(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x1f;
            val as u8
        }
        #[doc = "MSIRC0 clock calibration for MSI ranges 0 to 3 These bits are initialized at startup with the factory-programmed MSIRC0 calibration trim value for ranges 0 to 3. When MSITRIM0 is written, MSICAL0 is updated with the sum of MSITRIM0\\[4:0\\]
and the factory-programmed calibration trim value MSIRC0\\[4:0\\]."]
        #[inline(always)]
        pub fn set_msical0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
        }
        #[doc = "MSI bias mode selection Set by software to select the MSI bias mode. By default, the MSI bias is in continuous mode in order to maintain the output clocks accuracy. Setting this bit reduces the MSI consumption under range 4 but decrease its accuracy."]
        #[inline(always)]
        pub const fn msibias(&self) -> super::vals::Msibias {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Msibias::from_bits(val as u8)
        }
        #[doc = "MSI bias mode selection Set by software to select the MSI bias mode. By default, the MSI bias is in continuous mode in order to maintain the output clocks accuracy. Setting this bit reduces the MSI consumption under range 4 but decrease its accuracy."]
        #[inline(always)]
        pub fn set_msibias(&mut self, val: super::vals::Msibias) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "MSI clock range selection Set by software to select the MSIS and MSIK clocks range with MSISRANGE\\[3:0\\]
and MSIKRANGE\\[3:0\\]. Write 0 has no effect. After exiting Standby or Shutdown mode, or after a reset, this bit is at 0 and the MSIS and MSIK ranges are provided by MSISSRANGE\\[3:0\\]
and MSIKSRANGE\\[3:0\\]
in RCC_CSR."]
        #[inline(always)]
        pub const fn msirgsel(&self) -> super::vals::Msirgsel {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Msirgsel::from_bits(val as u8)
        }
        #[doc = "MSI clock range selection Set by software to select the MSIS and MSIK clocks range with MSISRANGE\\[3:0\\]
and MSIKRANGE\\[3:0\\]. Write 0 has no effect. After exiting Standby or Shutdown mode, or after a reset, this bit is at 0 and the MSIS and MSIK ranges are provided by MSISSRANGE\\[3:0\\]
and MSIKSRANGE\\[3:0\\]
in RCC_CSR."]
        #[inline(always)]
        pub fn set_msirgsel(&mut self, val: super::vals::Msirgsel) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "MSIK clock ranges These bits are configured by software to choose the frequency range of MSIK oscillator when MSIRGSEL is set. 16 frequency ranges are available: Note: MSIKRANGE can be modified when MSIK is OFF (MSISON = 0) or when MSIK is ready (MSIKRDY = 1). MSIKRANGE must NOT be modified when MSIK is ON and NOT ready (MSIKON = 1 and MSIKRDY = 0) MSIKRANGE is kept when the device wakes up from Stop mode, except when the MSIK range is above 24 MHz. In this case MSIKRANGE is changed by hardware into Range 2 (24 MHz)."]
        #[inline(always)]
        pub const fn msikrange(&self) -> super::vals::Msirange {
            let val = (self.0 >> 24usize) & 0x0f;
            super::vals::Msirange::from_bits(val as u8)
        }
        #[doc = "MSIK clock ranges These bits are configured by software to choose the frequency range of MSIK oscillator when MSIRGSEL is set. 16 frequency ranges are available: Note: MSIKRANGE can be modified when MSIK is OFF (MSISON = 0) or when MSIK is ready (MSIKRDY = 1). MSIKRANGE must NOT be modified when MSIK is ON and NOT ready (MSIKON = 1 and MSIKRDY = 0) MSIKRANGE is kept when the device wakes up from Stop mode, except when the MSIK range is above 24 MHz. In this case MSIKRANGE is changed by hardware into Range 2 (24 MHz)."]
        #[inline(always)]
        pub fn set_msikrange(&mut self, val: super::vals::Msirange) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
        }
        #[doc = "MSIS clock ranges These bits are configured by software to choose the frequency range of MSIS oscillator when MSIRGSEL is set. 16 frequency ranges are available: Note: MSISRANGE can be modified when MSIS is OFF (MSISON = 0) or when MSIS is ready (MSISRDY = 1). MSISRANGE must NOT be modified when MSIS is ON and NOT ready (MSISON = 1 and MSISRDY = 0) MSISRANGE is kept when the device wakes up from Stop mode, except when the MSIS range is above 24 MHz. In this case MSISRANGE is changed by hardware into Range 2 (24 MHz)."]
        #[inline(always)]
        pub const fn msisrange(&self) -> super::vals::Msirange {
            let val = (self.0 >> 28usize) & 0x0f;
            super::vals::Msirange::from_bits(val as u8)
        }
        #[doc = "MSIS clock ranges These bits are configured by software to choose the frequency range of MSIS oscillator when MSIRGSEL is set. 16 frequency ranges are available: Note: MSISRANGE can be modified when MSIS is OFF (MSISON = 0) or when MSIS is ready (MSISRDY = 1). MSISRANGE must NOT be modified when MSIS is ON and NOT ready (MSISON = 1 and MSISRDY = 0) MSISRANGE is kept when the device wakes up from Stop mode, except when the MSIS range is above 24 MHz. In this case MSISRANGE is changed by hardware into Range 2 (24 MHz)."]
        #[inline(always)]
        pub fn set_msisrange(&mut self, val: super::vals::Msirange) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Icscr1 {
        #[inline(always)]
        fn default() -> Icscr1 {
            Icscr1(0)
        }
    }
    impl core::fmt::Debug for Icscr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icscr1")
                .field("msical3", &self.msical3())
                .field("msical2", &self.msical2())
                .field("msical1", &self.msical1())
                .field("msical0", &self.msical0())
                .field("msibias", &self.msibias())
                .field("msirgsel", &self.msirgsel())
                .field("msikrange", &self.msikrange())
                .field("msisrange", &self.msisrange())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icscr1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Icscr1 {
                msical3: u8,
                msical2: u8,
                msical1: u8,
                msical0: u8,
                msibias: super::vals::Msibias,
                msirgsel: super::vals::Msirgsel,
                msikrange: super::vals::Msirange,
                msisrange: super::vals::Msirange,
            }
            let proxy = Icscr1 {
                msical3: self.msical3(),
                msical2: self.msical2(),
                msical1: self.msical1(),
                msical0: self.msical0(),
                msibias: self.msibias(),
                msirgsel: self.msirgsel(),
                msikrange: self.msikrange(),
                msisrange: self.msisrange(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC internal clock sources calibration register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icscr2(pub u32);
    impl Icscr2 {
        #[doc = "MSI clock trimming for ranges 12 to 15 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC3\\[4:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI."]
        #[inline(always)]
        pub const fn msitrim3(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "MSI clock trimming for ranges 12 to 15 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC3\\[4:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI."]
        #[inline(always)]
        pub fn set_msitrim3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "MSI clock trimming for ranges 8 to 11 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC2\\[4:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI."]
        #[inline(always)]
        pub const fn msitrim2(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x1f;
            val as u8
        }
        #[doc = "MSI clock trimming for ranges 8 to 11 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC2\\[4:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI."]
        #[inline(always)]
        pub fn set_msitrim2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
        }
        #[doc = "MSI clock trimming for ranges 4 to 7 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC1\\[4:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI."]
        #[inline(always)]
        pub const fn msitrim1(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x1f;
            val as u8
        }
        #[doc = "MSI clock trimming for ranges 4 to 7 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC1\\[4:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI."]
        #[inline(always)]
        pub fn set_msitrim1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
        }
        #[doc = "MSI clock trimming for ranges 0 to 3 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC0\\[4:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI."]
        #[inline(always)]
        pub const fn msitrim0(&self) -> u8 {
            let val = (self.0 >> 15usize) & 0x1f;
            val as u8
        }
        #[doc = "MSI clock trimming for ranges 0 to 3 These bits provide an additional user-programmable trimming value that is added to the factory-programmed calibration trim value MSIRC0\\[4:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the MSI."]
        #[inline(always)]
        pub fn set_msitrim0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
        }
    }
    impl Default for Icscr2 {
        #[inline(always)]
        fn default() -> Icscr2 {
            Icscr2(0)
        }
    }
    impl core::fmt::Debug for Icscr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icscr2")
                .field("msitrim3", &self.msitrim3())
                .field("msitrim2", &self.msitrim2())
                .field("msitrim1", &self.msitrim1())
                .field("msitrim0", &self.msitrim0())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icscr2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Icscr2 {
                msitrim3: u8,
                msitrim2: u8,
                msitrim1: u8,
                msitrim0: u8,
            }
            let proxy = Icscr2 {
                msitrim3: self.msitrim3(),
                msitrim2: self.msitrim2(),
                msitrim1: self.msitrim1(),
                msitrim0: self.msitrim0(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC internal clock sources calibration register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icscr3(pub u32);
    impl Icscr3 {
        #[doc = "HSI clock calibration These bits are initialized at startup with the factory-programmed HSI calibration trim value. When HSITRIM is written, HSICAL is updated with the sum of HSITRIM and the factory trim value."]
        #[inline(always)]
        pub const fn hsical(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "HSI clock calibration These bits are initialized at startup with the factory-programmed HSI calibration trim value. When HSITRIM is written, HSICAL is updated with the sum of HSITRIM and the factory trim value."]
        #[inline(always)]
        pub fn set_hsical(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "HSI clock trimming These bits provide an additional user-programmable trimming value that is added to the HSICAL\\[11:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the HSI."]
        #[inline(always)]
        pub const fn hsitrim(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "HSI clock trimming These bits provide an additional user-programmable trimming value that is added to the HSICAL\\[11:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the HSI."]
        #[inline(always)]
        pub fn set_hsitrim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
    }
    impl Default for Icscr3 {
        #[inline(always)]
        fn default() -> Icscr3 {
            Icscr3(0)
        }
    }
    impl core::fmt::Debug for Icscr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icscr3")
                .field("hsical", &self.hsical())
                .field("hsitrim", &self.hsitrim())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icscr3 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Icscr3 {
                hsical: u16,
                hsitrim: u8,
            }
            let proxy = Icscr3 {
                hsical: self.hsical(),
                hsitrim: self.hsitrim(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC PLL configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pll1cfgr(pub u32);
    impl Pll1cfgr {
        #[doc = "PLL entry clock source Set and cleared by software to select PLL clock source. These bits can be written only when the PLL is disabled. In order to save power, when no PLL is used, the value of PLLSRC must be 0."]
        #[inline(always)]
        pub const fn pllsrc(&self) -> super::vals::Pllsrc {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Pllsrc::from_bits(val as u8)
        }
        #[doc = "PLL entry clock source Set and cleared by software to select PLL clock source. These bits can be written only when the PLL is disabled. In order to save power, when no PLL is used, the value of PLLSRC must be 0."]
        #[inline(always)]
        pub fn set_pllsrc(&mut self, val: super::vals::Pllsrc) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "PLL input frequency range Set and reset by software to select the proper reference frequency range used for PLL. This bit must be written before enabling the PLL. 00-01-10: PLL input (ref1_ck) clock range frequency between 4 and 8 MHz"]
        #[inline(always)]
        pub const fn pllrge(&self) -> super::vals::Pllrge {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Pllrge::from_bits(val as u8)
        }
        #[doc = "PLL input frequency range Set and reset by software to select the proper reference frequency range used for PLL. This bit must be written before enabling the PLL. 00-01-10: PLL input (ref1_ck) clock range frequency between 4 and 8 MHz"]
        #[inline(always)]
        pub fn set_pllrge(&mut self, val: super::vals::Pllrge) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "PLL fractional latch enable Set and reset by software to latch the content of PLLFRACN into the Î£Î modulator. In order to latch the PLLFRACN value into the Î£Î modulator, PLLFRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLLFRACN into the modulator (see for details)."]
        #[inline(always)]
        pub const fn pllfracen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PLL fractional latch enable Set and reset by software to latch the content of PLLFRACN into the Î£Î modulator. In order to latch the PLLFRACN value into the Î£Î modulator, PLLFRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLLFRACN into the modulator (see for details)."]
        #[inline(always)]
        pub fn set_pllfracen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Prescaler for PLL Set and cleared by software to configure the prescaler of the PLL. The VCO1 input frequency is PLL input clock frequency/PLLM. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0). ..."]
        #[inline(always)]
        pub const fn pllm(&self) -> super::vals::Pllm {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::Pllm::from_bits(val as u8)
        }
        #[doc = "Prescaler for PLL Set and cleared by software to configure the prescaler of the PLL. The VCO1 input frequency is PLL input clock frequency/PLLM. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0). ..."]
        #[inline(always)]
        pub fn set_pllm(&mut self, val: super::vals::Pllm) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
        }
        #[doc = "Prescaler for EPOD booster input clock Set and cleared by software to configure the prescaler of the PLL, used for the EPOD booster. The EPOD booster input frequency is PLL input clock frequency/PLLMBOOST. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0) and EPOD Boost mode is disabled (see ). others: reserved"]
        #[inline(always)]
        pub const fn pllmboost(&self) -> super::vals::Pllmboost {
            let val = (self.0 >> 12usize) & 0x0f;
            super::vals::Pllmboost::from_bits(val as u8)
        }
        #[doc = "Prescaler for EPOD booster input clock Set and cleared by software to configure the prescaler of the PLL, used for the EPOD booster. The EPOD booster input frequency is PLL input clock frequency/PLLMBOOST. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0) and EPOD Boost mode is disabled (see ). others: reserved"]
        #[inline(always)]
        pub fn set_pllmboost(&mut self, val: super::vals::Pllmboost) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
        }
        #[doc = "PLL DIVP divider output enable Set and reset by software to enable the PLL_p_ck output of the PLL. To save power, PLLPEN and PLLP bits must be set to 0 when the PLL_p_ck is not used. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0)."]
        #[inline(always)]
        pub const fn pllpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PLL DIVP divider output enable Set and reset by software to enable the PLL_p_ck output of the PLL. To save power, PLLPEN and PLLP bits must be set to 0 when the PLL_p_ck is not used. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0)."]
        #[inline(always)]
        pub fn set_pllpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "PLL DIVQ divider output enable Set and reset by software to enable the PLL_q_ck output of the PLL. To save power, PLLQEN and PLLQ bits must be set to 0 when the PLL_q_ck is not used. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0)."]
        #[inline(always)]
        pub const fn pllqen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "PLL DIVQ divider output enable Set and reset by software to enable the PLL_q_ck output of the PLL. To save power, PLLQEN and PLLQ bits must be set to 0 when the PLL_q_ck is not used. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0)."]
        #[inline(always)]
        pub fn set_pllqen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "PLL DIVR divider output enable Set and reset by software to enable the PLL_r_ck output of the PLL. To save power, PLLRENPLL2REN and PLLR bits must be set to 0 when the PLL_r_ck is not used. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0)."]
        #[inline(always)]
        pub const fn pllren(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PLL DIVR divider output enable Set and reset by software to enable the PLL_r_ck output of the PLL. To save power, PLLRENPLL2REN and PLLR bits must be set to 0 when the PLL_r_ck is not used. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0)."]
        #[inline(always)]
        pub fn set_pllren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for Pll1cfgr {
        #[inline(always)]
        fn default() -> Pll1cfgr {
            Pll1cfgr(0)
        }
    }
    impl core::fmt::Debug for Pll1cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pll1cfgr")
                .field("pllsrc", &self.pllsrc())
                .field("pllrge", &self.pllrge())
                .field("pllfracen", &self.pllfracen())
                .field("pllm", &self.pllm())
                .field("pllmboost", &self.pllmboost())
                .field("pllpen", &self.pllpen())
                .field("pllqen", &self.pllqen())
                .field("pllren", &self.pllren())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pll1cfgr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Pll1cfgr {
                pllsrc: super::vals::Pllsrc,
                pllrge: super::vals::Pllrge,
                pllfracen: bool,
                pllm: super::vals::Pllm,
                pllmboost: super::vals::Pllmboost,
                pllpen: bool,
                pllqen: bool,
                pllren: bool,
            }
            let proxy = Pll1cfgr {
                pllsrc: self.pllsrc(),
                pllrge: self.pllrge(),
                pllfracen: self.pllfracen(),
                pllm: self.pllm(),
                pllmboost: self.pllmboost(),
                pllpen: self.pllpen(),
                pllqen: self.pllqen(),
                pllren: self.pllren(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC PLL configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pll23cfgr(pub u32);
    impl Pll23cfgr {
        #[doc = "PLL entry clock source Set and cleared by software to select PLL clock source. These bits can be written only when the PLL is disabled. In order to save power, when no PLL is used, the value of PLLSRC must be 0."]
        #[inline(always)]
        pub const fn pllsrc(&self) -> super::vals::Pllsrc {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Pllsrc::from_bits(val as u8)
        }
        #[doc = "PLL entry clock source Set and cleared by software to select PLL clock source. These bits can be written only when the PLL is disabled. In order to save power, when no PLL is used, the value of PLLSRC must be 0."]
        #[inline(always)]
        pub fn set_pllsrc(&mut self, val: super::vals::Pllsrc) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "PLL input frequency range Set and reset by software to select the proper reference frequency range used for PLL. This bit must be written before enabling the PLL. 00-01-10: PLL input (ref1_ck) clock range frequency between 4 and 8 MHz"]
        #[inline(always)]
        pub const fn pllrge(&self) -> super::vals::Pllrge {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Pllrge::from_bits(val as u8)
        }
        #[doc = "PLL input frequency range Set and reset by software to select the proper reference frequency range used for PLL. This bit must be written before enabling the PLL. 00-01-10: PLL input (ref1_ck) clock range frequency between 4 and 8 MHz"]
        #[inline(always)]
        pub fn set_pllrge(&mut self, val: super::vals::Pllrge) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "PLL fractional latch enable Set and reset by software to latch the content of PLLFRACN into the Î£Î modulator. In order to latch the PLLFRACN value into the Î£Î modulator, PLLFRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLLFRACN into the modulator (see for details)."]
        #[inline(always)]
        pub const fn pllfracen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PLL fractional latch enable Set and reset by software to latch the content of PLLFRACN into the Î£Î modulator. In order to latch the PLLFRACN value into the Î£Î modulator, PLLFRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLLFRACN into the modulator (see for details)."]
        #[inline(always)]
        pub fn set_pllfracen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Prescaler for PLL Set and cleared by software to configure the prescaler of the PLL. The VCO1 input frequency is PLL input clock frequency/PLLM. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0). ..."]
        #[inline(always)]
        pub const fn pllm(&self) -> super::vals::Pllm {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::Pllm::from_bits(val as u8)
        }
        #[doc = "Prescaler for PLL Set and cleared by software to configure the prescaler of the PLL. The VCO1 input frequency is PLL input clock frequency/PLLM. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0). ..."]
        #[inline(always)]
        pub fn set_pllm(&mut self, val: super::vals::Pllm) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
        }
        #[doc = "PLL DIVP divider output enable Set and reset by software to enable the PLL_p_ck output of the PLL. To save power, PLLPEN and PLLP bits must be set to 0 when the PLL_p_ck is not used. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0)."]
        #[inline(always)]
        pub const fn pllpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PLL DIVP divider output enable Set and reset by software to enable the PLL_p_ck output of the PLL. To save power, PLLPEN and PLLP bits must be set to 0 when the PLL_p_ck is not used. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0)."]
        #[inline(always)]
        pub fn set_pllpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "PLL DIVQ divider output enable Set and reset by software to enable the PLL_q_ck output of the PLL. To save power, PLLQEN and PLLQ bits must be set to 0 when the PLL_q_ck is not used. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0)."]
        #[inline(always)]
        pub const fn pllqen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "PLL DIVQ divider output enable Set and reset by software to enable the PLL_q_ck output of the PLL. To save power, PLLQEN and PLLQ bits must be set to 0 when the PLL_q_ck is not used. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0)."]
        #[inline(always)]
        pub fn set_pllqen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "PLL DIVR divider output enable Set and reset by software to enable the PLL_r_ck output of the PLL. To save power, PLLRENPLL2REN and PLLR bits must be set to 0 when the PLL_r_ck is not used. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0)."]
        #[inline(always)]
        pub const fn pllren(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PLL DIVR divider output enable Set and reset by software to enable the PLL_r_ck output of the PLL. To save power, PLLRENPLL2REN and PLLR bits must be set to 0 when the PLL_r_ck is not used. This bit can be written only when the PLL is disabled (PLLON = 0 and PLLRDY = 0)."]
        #[inline(always)]
        pub fn set_pllren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for Pll23cfgr {
        #[inline(always)]
        fn default() -> Pll23cfgr {
            Pll23cfgr(0)
        }
    }
    impl core::fmt::Debug for Pll23cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pll23cfgr")
                .field("pllsrc", &self.pllsrc())
                .field("pllrge", &self.pllrge())
                .field("pllfracen", &self.pllfracen())
                .field("pllm", &self.pllm())
                .field("pllpen", &self.pllpen())
                .field("pllqen", &self.pllqen())
                .field("pllren", &self.pllren())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pll23cfgr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Pll23cfgr {
                pllsrc: super::vals::Pllsrc,
                pllrge: super::vals::Pllrge,
                pllfracen: bool,
                pllm: super::vals::Pllm,
                pllpen: bool,
                pllqen: bool,
                pllren: bool,
            }
            let proxy = Pll23cfgr {
                pllsrc: self.pllsrc(),
                pllrge: self.pllrge(),
                pllfracen: self.pllfracen(),
                pllm: self.pllm(),
                pllpen: self.pllpen(),
                pllqen: self.pllqen(),
                pllren: self.pllren(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC PLL1 dividers register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Plldivr(pub u32);
    impl Plldivr {
        #[doc = "Multiplication factor for PLL1 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL1ON = 0 and PLL1RDY = 0). ... ... Others: reserved VCO output frequency = Fref1_ck x PLL1N, when fractional value 0 has been loaded into PLL1FRACN, with: PLL1N between 4 and 512 input frequency Fref1_ck between 4 and 16 MHz"]
        #[inline(always)]
        pub const fn plln(&self) -> super::vals::Plln {
            let val = (self.0 >> 0usize) & 0x01ff;
            super::vals::Plln::from_bits(val as u16)
        }
        #[doc = "Multiplication factor for PLL1 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL1ON = 0 and PLL1RDY = 0). ... ... Others: reserved VCO output frequency = Fref1_ck x PLL1N, when fractional value 0 has been loaded into PLL1FRACN, with: PLL1N between 4 and 512 input frequency Fref1_ck between 4 and 16 MHz"]
        #[inline(always)]
        pub fn set_plln(&mut self, val: super::vals::Plln) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val.to_bits() as u32) & 0x01ff) << 0usize);
        }
        #[doc = "PLL1 DIVP division factor Set and reset by software to control the frequency of the pll1_p_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). Note that odd division factors are not allowed. ..."]
        #[inline(always)]
        pub const fn pllp(&self) -> super::vals::Plldiv {
            let val = (self.0 >> 9usize) & 0x7f;
            super::vals::Plldiv::from_bits(val as u8)
        }
        #[doc = "PLL1 DIVP division factor Set and reset by software to control the frequency of the pll1_p_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). Note that odd division factors are not allowed. ..."]
        #[inline(always)]
        pub fn set_pllp(&mut self, val: super::vals::Plldiv) {
            self.0 = (self.0 & !(0x7f << 9usize)) | (((val.to_bits() as u32) & 0x7f) << 9usize);
        }
        #[doc = "PLL1 DIVQ division factor Set and reset by software to control the frequency of the pll1_q_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
        #[inline(always)]
        pub const fn pllq(&self) -> super::vals::Plldiv {
            let val = (self.0 >> 16usize) & 0x7f;
            super::vals::Plldiv::from_bits(val as u8)
        }
        #[doc = "PLL1 DIVQ division factor Set and reset by software to control the frequency of the pll1_q_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
        #[inline(always)]
        pub fn set_pllq(&mut self, val: super::vals::Plldiv) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val.to_bits() as u32) & 0x7f) << 16usize);
        }
        #[doc = "PLL1 DIVR division factor Set and reset by software to control the frequency of the pll1_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
        #[inline(always)]
        pub const fn pllr(&self) -> super::vals::Plldiv {
            let val = (self.0 >> 24usize) & 0x7f;
            super::vals::Plldiv::from_bits(val as u8)
        }
        #[doc = "PLL1 DIVR division factor Set and reset by software to control the frequency of the pll1_r_ck clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
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
    #[doc = "RCC PLL1 fractional divider register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pllfracr(pub u32);
    impl Pllfracr {
        #[doc = "Fractional part of the multiplication factor for PLL1 VCO Set and reset by software to control the fractional part of the multiplication factor of the VCO. These bits can be written at any time, allowing dynamic fine-tuning of the PLL1 VCO. VCO output frequency = Fref1_ck x (PLL1N + (PLL1FRACN / 213)), with: PLL1N must be between 4 and 512. PLL1FRACN can be between 0 and 213- 1. The input frequency Fref1_ck must be between 4 and 16 MHz. To change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: Set the bit PLL1FRACEN to 0. Write the new fractional value into PLL1FRACN. Set the bit PLL1FRACEN to 1."]
        #[inline(always)]
        pub const fn pllfracn(&self) -> u16 {
            let val = (self.0 >> 3usize) & 0x1fff;
            val as u16
        }
        #[doc = "Fractional part of the multiplication factor for PLL1 VCO Set and reset by software to control the fractional part of the multiplication factor of the VCO. These bits can be written at any time, allowing dynamic fine-tuning of the PLL1 VCO. VCO output frequency = Fref1_ck x (PLL1N + (PLL1FRACN / 213)), with: PLL1N must be between 4 and 512. PLL1FRACN can be between 0 and 213- 1. The input frequency Fref1_ck must be between 4 and 16 MHz. To change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: Set the bit PLL1FRACEN to 0. Write the new fractional value into PLL1FRACN. Set the bit PLL1FRACEN to 1."]
        #[inline(always)]
        pub fn set_pllfracn(&mut self, val: u16) {
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
            f.debug_struct("Pllfracr").field("pllfracn", &self.pllfracn()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pllfracr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Pllfracr {
                pllfracn: u16,
            }
            let proxy = Pllfracr {
                pllfracn: self.pllfracn(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC privilege configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcfgr(pub u32);
    impl Privcfgr {
        #[doc = "RCC secure functions privilege configuration Set and reset by software. This bit can be written only by a secure privileged access."]
        #[inline(always)]
        pub const fn spriv(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RCC secure functions privilege configuration Set and reset by software. This bit can be written only by a secure privileged access."]
        #[inline(always)]
        pub fn set_spriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RCC non-secure functions privilege configuration Set and reset by software. This bit can be written only by privileged access, secure or non-secure."]
        #[inline(always)]
        pub const fn nspriv(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RCC non-secure functions privilege configuration Set and reset by software. This bit can be written only by privileged access, secure or non-secure."]
        #[inline(always)]
        pub fn set_nspriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Privcfgr {
        #[inline(always)]
        fn default() -> Privcfgr {
            Privcfgr(0)
        }
    }
    impl core::fmt::Debug for Privcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Privcfgr")
                .field("spriv", &self.spriv())
                .field("nspriv", &self.nspriv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privcfgr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Privcfgr {
                spriv: bool,
                nspriv: bool,
            }
            let proxy = Privcfgr {
                spriv: self.spriv(),
                nspriv: self.nspriv(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC secure configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccfgr(pub u32);
    impl Seccfgr {
        #[doc = "HSI clock configuration and status bits security Set and reset by software."]
        #[inline(always)]
        pub const fn hsisec(&self) -> super::vals::Security {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Security::from_bits(val as u8)
        }
        #[doc = "HSI clock configuration and status bits security Set and reset by software."]
        #[inline(always)]
        pub fn set_hsisec(&mut self, val: super::vals::Security) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "HSE clock configuration bits, status bits and HSE_CSS security Set and reset by software."]
        #[inline(always)]
        pub const fn hsesec(&self) -> super::vals::Security {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Security::from_bits(val as u8)
        }
        #[doc = "HSE clock configuration bits, status bits and HSE_CSS security Set and reset by software."]
        #[inline(always)]
        pub fn set_hsesec(&mut self, val: super::vals::Security) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "MSI clock configuration and status bits security Set and reset by software."]
        #[inline(always)]
        pub const fn msisec(&self) -> super::vals::Security {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Security::from_bits(val as u8)
        }
        #[doc = "MSI clock configuration and status bits security Set and reset by software."]
        #[inline(always)]
        pub fn set_msisec(&mut self, val: super::vals::Security) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "LSI clock configuration and status bits security Set and reset by software."]
        #[inline(always)]
        pub const fn lsisec(&self) -> super::vals::Security {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Security::from_bits(val as u8)
        }
        #[doc = "LSI clock configuration and status bits security Set and reset by software."]
        #[inline(always)]
        pub fn set_lsisec(&mut self, val: super::vals::Security) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "LSE clock configuration and status bits security Set and reset by software."]
        #[inline(always)]
        pub const fn lsesec(&self) -> super::vals::Security {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Security::from_bits(val as u8)
        }
        #[doc = "LSE clock configuration and status bits security Set and reset by software."]
        #[inline(always)]
        pub fn set_lsesec(&mut self, val: super::vals::Security) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "SYSCLK clock selection, STOPWUCK bit, clock output on MCO configuration security Set and reset by software."]
        #[inline(always)]
        pub const fn sysclksec(&self) -> super::vals::Security {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Security::from_bits(val as u8)
        }
        #[doc = "SYSCLK clock selection, STOPWUCK bit, clock output on MCO configuration security Set and reset by software."]
        #[inline(always)]
        pub fn set_sysclksec(&mut self, val: super::vals::Security) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "AHBx/APBx prescaler configuration bits security Set and reset by software."]
        #[inline(always)]
        pub const fn prescsec(&self) -> super::vals::Security {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Security::from_bits(val as u8)
        }
        #[doc = "AHBx/APBx prescaler configuration bits security Set and reset by software."]
        #[inline(always)]
        pub fn set_prescsec(&mut self, val: super::vals::Security) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "PLL1 clock configuration and status bits security Set and reset by software."]
        #[inline(always)]
        pub const fn pllsec(&self, n: usize) -> super::vals::Security {
            assert!(n < 3usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Security::from_bits(val as u8)
        }
        #[doc = "PLL1 clock configuration and status bits security Set and reset by software."]
        #[inline(always)]
        pub fn set_pllsec(&mut self, n: usize, val: super::vals::Security) {
            assert!(n < 3usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "intermediate clock source selection security Set and reset by software."]
        #[inline(always)]
        pub const fn iclksec(&self) -> super::vals::Security {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Security::from_bits(val as u8)
        }
        #[doc = "intermediate clock source selection security Set and reset by software."]
        #[inline(always)]
        pub fn set_iclksec(&mut self, val: super::vals::Security) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "HSI48 clock configuration and status bits security Set and reset by software."]
        #[inline(always)]
        pub const fn hsi48sec(&self) -> super::vals::Security {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Security::from_bits(val as u8)
        }
        #[doc = "HSI48 clock configuration and status bits security Set and reset by software."]
        #[inline(always)]
        pub fn set_hsi48sec(&mut self, val: super::vals::Security) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Remove reset flag security Set and reset by software."]
        #[inline(always)]
        pub const fn rmvfsec(&self) -> super::vals::Security {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Security::from_bits(val as u8)
        }
        #[doc = "Remove reset flag security Set and reset by software."]
        #[inline(always)]
        pub fn set_rmvfsec(&mut self, val: super::vals::Security) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Seccfgr {
        #[inline(always)]
        fn default() -> Seccfgr {
            Seccfgr(0)
        }
    }
    impl core::fmt::Debug for Seccfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Seccfgr")
                .field("hsisec", &self.hsisec())
                .field("hsesec", &self.hsesec())
                .field("msisec", &self.msisec())
                .field("lsisec", &self.lsisec())
                .field("lsesec", &self.lsesec())
                .field("sysclksec", &self.sysclksec())
                .field("prescsec", &self.prescsec())
                .field(
                    "pllsec",
                    &[self.pllsec(0usize), self.pllsec(1usize), self.pllsec(2usize)],
                )
                .field("iclksec", &self.iclksec())
                .field("hsi48sec", &self.hsi48sec())
                .field("rmvfsec", &self.rmvfsec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Seccfgr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Seccfgr {
                hsisec: super::vals::Security,
                hsesec: super::vals::Security,
                msisec: super::vals::Security,
                lsisec: super::vals::Security,
                lsesec: super::vals::Security,
                sysclksec: super::vals::Security,
                prescsec: super::vals::Security,
                pllsec: [super::vals::Security; 3usize],
                iclksec: super::vals::Security,
                hsi48sec: super::vals::Security,
                rmvfsec: super::vals::Security,
            }
            let proxy = Seccfgr {
                hsisec: self.hsisec(),
                hsesec: self.hsesec(),
                msisec: self.msisec(),
                lsisec: self.lsisec(),
                lsesec: self.lsesec(),
                sysclksec: self.sysclksec(),
                prescsec: self.prescsec(),
                pllsec: [self.pllsec(0usize), self.pllsec(1usize), self.pllsec(2usize)],
                iclksec: self.iclksec(),
                hsi48sec: self.hsi48sec(),
                rmvfsec: self.rmvfsec(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC SmartRun domain peripheral autonomous mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Srdamr(pub u32);
    impl Srdamr {
        #[doc = "SPI3 autonomous mode enable in Stop 0,1, 2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn spi3amen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 autonomous mode enable in Stop 0,1, 2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_spi3amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LPUART1 autonomous mode enable in Stop 0,1, 2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn lpuart1amen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 autonomous mode enable in Stop 0,1, 2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_lpuart1amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I2C3 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn i2c3amen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_i2c3amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LPTIM1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn lptim1amen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_lptim1amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIM3 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn lptim3amen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_lptim3amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "LPTIM4 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn lptim4amen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM4 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_lptim4amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "OPAMP autonomous mode enable in Stop 0,1,2 mode Set and cleared by software."]
        #[inline(always)]
        pub const fn opampamen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP autonomous mode enable in Stop 0,1,2 mode Set and cleared by software."]
        #[inline(always)]
        pub fn set_opampamen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "COMP autonomous mode enable in Stop 0,1,2 mode Set and cleared by software."]
        #[inline(always)]
        pub const fn compamen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "COMP autonomous mode enable in Stop 0,1,2 mode Set and cleared by software."]
        #[inline(always)]
        pub fn set_compamen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "VREFBUF autonomous mode enable in Stop 0,1,2 mode Set and cleared by software."]
        #[inline(always)]
        pub const fn vrefamen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "VREFBUF autonomous mode enable in Stop 0,1,2 mode Set and cleared by software."]
        #[inline(always)]
        pub fn set_vrefamen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "RTC and TAMP autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn rtcapbamen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "RTC and TAMP autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_rtcapbamen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ADC4 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn adc4amen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "ADC4 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_adc4amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "LPGPIO1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software."]
        #[inline(always)]
        pub const fn lpgpio1amen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "LPGPIO1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software."]
        #[inline(always)]
        pub fn set_lpgpio1amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "DAC1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn dac1amen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_dac1amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "LPDMA1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn lpdma1amen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "LPDMA1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_lpdma1amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "ADF1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn adf1amen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "ADF1 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub fn set_adf1amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "SRAM4 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software."]
        #[inline(always)]
        pub const fn sram4amen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM4 autonomous mode enable in Stop 0,1,2 mode Set and cleared by software."]
        #[inline(always)]
        pub fn set_sram4amen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Srdamr {
        #[inline(always)]
        fn default() -> Srdamr {
            Srdamr(0)
        }
    }
    impl core::fmt::Debug for Srdamr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Srdamr")
                .field("spi3amen", &self.spi3amen())
                .field("lpuart1amen", &self.lpuart1amen())
                .field("i2c3amen", &self.i2c3amen())
                .field("lptim1amen", &self.lptim1amen())
                .field("lptim3amen", &self.lptim3amen())
                .field("lptim4amen", &self.lptim4amen())
                .field("opampamen", &self.opampamen())
                .field("compamen", &self.compamen())
                .field("vrefamen", &self.vrefamen())
                .field("rtcapbamen", &self.rtcapbamen())
                .field("adc4amen", &self.adc4amen())
                .field("lpgpio1amen", &self.lpgpio1amen())
                .field("dac1amen", &self.dac1amen())
                .field("lpdma1amen", &self.lpdma1amen())
                .field("adf1amen", &self.adf1amen())
                .field("sram4amen", &self.sram4amen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Srdamr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Srdamr {
                spi3amen: bool,
                lpuart1amen: bool,
                i2c3amen: bool,
                lptim1amen: bool,
                lptim3amen: bool,
                lptim4amen: bool,
                opampamen: bool,
                compamen: bool,
                vrefamen: bool,
                rtcapbamen: bool,
                adc4amen: bool,
                lpgpio1amen: bool,
                dac1amen: bool,
                lpdma1amen: bool,
                adf1amen: bool,
                sram4amen: bool,
            }
            let proxy = Srdamr {
                spi3amen: self.spi3amen(),
                lpuart1amen: self.lpuart1amen(),
                i2c3amen: self.i2c3amen(),
                lptim1amen: self.lptim1amen(),
                lptim3amen: self.lptim3amen(),
                lptim4amen: self.lptim4amen(),
                opampamen: self.opampamen(),
                compamen: self.compamen(),
                vrefamen: self.vrefamen(),
                rtcapbamen: self.rtcapbamen(),
                adc4amen: self.adc4amen(),
                lpgpio1amen: self.lpgpio1amen(),
                dac1amen: self.dac1amen(),
                lpdma1amen: self.lpdma1amen(),
                adf1amen: self.adf1amen(),
                sram4amen: self.sram4amen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adcdacsel {
        #[doc = "HCLK clock selected"]
        HCLK1 = 0x0,
        #[doc = "SYSCLK selected"]
        SYS = 0x01,
        #[doc = "PLL2 R (pll2_r_ck) selected"]
        PLL2_R = 0x02,
        #[doc = "HSE clock selected"]
        HSE = 0x03,
        #[doc = "HSI clock selected"]
        HSI = 0x04,
        #[doc = "MSIK clock selected"]
        MSIK = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Adcdacsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adcdacsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adcdacsel {
        #[inline(always)]
        fn from(val: u8) -> Adcdacsel {
            Adcdacsel::from_bits(val)
        }
    }
    impl From<Adcdacsel> for u8 {
        #[inline(always)]
        fn from(val: Adcdacsel) -> u8 {
            Adcdacsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adfsel {
        #[doc = "HCLK selected"]
        HCLK3 = 0x0,
        #[doc = "PLL1 P (pll1_p_ck) selected"]
        PLL1_P = 0x01,
        #[doc = "PLL3 Q (pll3_q_ck) selected"]
        PLL3_Q = 0x02,
        #[doc = "input pin AUDIOCLK selected"]
        AUDIOCLK = 0x03,
        #[doc = "MSIK clock selected"]
        MSIK = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Adfsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adfsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adfsel {
        #[inline(always)]
        fn from(val: u8) -> Adfsel {
            Adfsel::from_bits(val)
        }
    }
    impl From<Adfsel> for u8 {
        #[inline(always)]
        fn from(val: Adfsel) -> u8 {
            Adfsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dacsel {
        #[doc = "LSE selected"]
        LSE = 0x0,
        #[doc = "LSI selected"]
        LSI = 0x01,
    }
    impl Dacsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dacsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dacsel {
        #[inline(always)]
        fn from(val: u8) -> Dacsel {
            Dacsel::from_bits(val)
        }
    }
    impl From<Dacsel> for u8 {
        #[inline(always)]
        fn from(val: Dacsel) -> u8 {
            Dacsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dpre {
        #[doc = "DCLK not divided"]
        DIV1 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "DCLK divided by 2"]
        DIV2 = 0x04,
        #[doc = "DCLK divided by 4"]
        DIV4 = 0x05,
        #[doc = "DCLK divided by 8"]
        DIV8 = 0x06,
        #[doc = "DCLK divided by 16"]
        DIV16 = 0x07,
    }
    impl Dpre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dpre {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dpre {
        #[inline(always)]
        fn from(val: u8) -> Dpre {
            Dpre::from_bits(val)
        }
    }
    impl From<Dpre> for u8 {
        #[inline(always)]
        fn from(val: Dpre) -> u8 {
            Dpre::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dsisel {
        #[doc = "PLL3 “P” (pll3_p_ck) selected"]
        PLL3_P = 0x0,
        #[doc = "DSI PHY PLL output selected (formerly called DCLK, renamed to DSI_PHY to match other chip families)"]
        DSI_PHY = 0x01,
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
        #[doc = "HSE clock selected"]
        HSE = 0x0,
        #[doc = "PLL1 Q (pll1_q_ck) selected"]
        PLL1_Q = 0x01,
        #[doc = "PLL2 P (pll2_p_ck) selected"]
        PLL2_P = 0x02,
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hseext {
        #[doc = "external HSE clock analog mode"]
        ANALOG = 0x0,
        #[doc = "external HSE clock digital mode (through I/O Schmitt trigger)"]
        DIGITAL = 0x01,
    }
    impl Hseext {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hseext {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hseext {
        #[inline(always)]
        fn from(val: u8) -> Hseext {
            Hseext::from_bits(val)
        }
    }
    impl From<Hseext> for u8 {
        #[inline(always)]
        fn from(val: Hseext) -> u8 {
            Hseext::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hspisel {
        #[doc = "SYSCLK selected"]
        SYS = 0x0,
        #[doc = "PLL1 “Q” (pll1_q_ck) selected, can be up to 200 MHz"]
        PLL1_Q = 0x01,
        #[doc = "PLL2 “Q” (pll2_q_ck) selected, can be up to 200 MHz"]
        PLL2_Q = 0x02,
        #[doc = "PLL3 “R” (pll3_r_ck) selected, can be up to 200 MHz"]
        PLL3_R = 0x03,
    }
    impl Hspisel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hspisel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hspisel {
        #[inline(always)]
        fn from(val: u8) -> Hspisel {
            Hspisel::from_bits(val)
        }
    }
    impl From<Hspisel> for u8 {
        #[inline(always)]
        fn from(val: Hspisel) -> u8 {
            Hspisel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum I2c3sel {
        #[doc = "PCLK3 selected"]
        PCLK3 = 0x0,
        #[doc = "SYSCLK selected"]
        SYS = 0x01,
        #[doc = "HSI selected"]
        HSI = 0x02,
        #[doc = "MSIK selected"]
        MSIK = 0x03,
    }
    impl I2c3sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2c3sel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I2c3sel {
        #[inline(always)]
        fn from(val: u8) -> I2c3sel {
            I2c3sel::from_bits(val)
        }
    }
    impl From<I2c3sel> for u8 {
        #[inline(always)]
        fn from(val: I2c3sel) -> u8 {
            I2c3sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum I2csel {
        #[doc = "PCLK1 selected"]
        PCLK1 = 0x0,
        #[doc = "SYSCLK selected"]
        SYS = 0x01,
        #[doc = "HSI selected"]
        HSI = 0x02,
        #[doc = "MSIK selected"]
        MSIK = 0x03,
    }
    impl I2csel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2csel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I2csel {
        #[inline(always)]
        fn from(val: u8) -> I2csel {
            I2csel::from_bits(val)
        }
    }
    impl From<I2csel> for u8 {
        #[inline(always)]
        fn from(val: I2csel) -> u8 {
            I2csel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Iclksel {
        #[doc = "HSI48 clock selected"]
        HSI48 = 0x0,
        #[doc = "PLL2 Q (pll2_q_ck) selected"]
        PLL2_Q = 0x01,
        #[doc = "PLL1 Q (pll1_q_ck) selected"]
        PLL1_Q = 0x02,
        #[doc = "MSIK clock selected"]
        MSIK = 0x03,
    }
    impl Iclksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Iclksel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Iclksel {
        #[inline(always)]
        fn from(val: u8) -> Iclksel {
            Iclksel::from_bits(val)
        }
    }
    impl From<Iclksel> for u8 {
        #[inline(always)]
        fn from(val: Iclksel) -> u8 {
            Iclksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lptim2sel {
        #[doc = "PCLK1 selected"]
        PCLK1 = 0x0,
        #[doc = "LSI selected"]
        LSI = 0x01,
        #[doc = "HSI selected"]
        HSI = 0x02,
        #[doc = "LSE selected"]
        LSE = 0x03,
    }
    impl Lptim2sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lptim2sel {
            unsafe { core::mem::transmute(val & 0x03) }
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
    pub enum Lptimsel {
        #[doc = "MSIK selected"]
        MSIK = 0x0,
        #[doc = "LSI selected"]
        LSI = 0x01,
        #[doc = "HSI selected"]
        HSI = 0x02,
        #[doc = "LSE selected"]
        LSE = 0x03,
    }
    impl Lptimsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lptimsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lptimsel {
        #[inline(always)]
        fn from(val: u8) -> Lptimsel {
            Lptimsel::from_bits(val)
        }
    }
    impl From<Lptimsel> for u8 {
        #[inline(always)]
        fn from(val: Lptimsel) -> u8 {
            Lptimsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lpusartsel {
        #[doc = "PCLK3 selected"]
        PCLK3 = 0x0,
        #[doc = "SYSCLK selected"]
        SYS = 0x01,
        #[doc = "HSI selected"]
        HSI = 0x02,
        #[doc = "LSE selected"]
        LSE = 0x03,
        #[doc = "MSIK selected"]
        MSIK = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Lpusartsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lpusartsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lpusartsel {
        #[inline(always)]
        fn from(val: u8) -> Lpusartsel {
            Lpusartsel::from_bits(val)
        }
    }
    impl From<Lpusartsel> for u8 {
        #[inline(always)]
        fn from(val: Lpusartsel) -> u8 {
            Lpusartsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lscosel {
        #[doc = "LSI clock selected"]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lsedrv {
        #[doc = "Low driving capability"]
        LOW = 0x0,
        #[doc = "Medium low driving capability"]
        MEDIUM_LOW = 0x01,
        #[doc = "Medium high driving capability"]
        MEDIUM_HIGH = 0x02,
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
    pub enum Lsiprediv {
        #[doc = "LSI not divided"]
        DIV1 = 0x0,
        #[doc = "LSI divided by 128"]
        DIV128 = 0x01,
    }
    impl Lsiprediv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lsiprediv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lsiprediv {
        #[inline(always)]
        fn from(val: u8) -> Lsiprediv {
            Lsiprediv::from_bits(val)
        }
    }
    impl From<Lsiprediv> for u8 {
        #[inline(always)]
        fn from(val: Lsiprediv) -> u8 {
            Lsiprediv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ltdcsel {
        #[doc = "PLL3 “R” (pll3_r_ck) selected"]
        PLL3_R = 0x0,
        #[doc = "PLL2 “R” (pll2_r_ck) selected"]
        PLL2_R = 0x01,
    }
    impl Ltdcsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ltdcsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ltdcsel {
        #[inline(always)]
        fn from(val: u8) -> Ltdcsel {
            Ltdcsel::from_bits(val)
        }
    }
    impl From<Ltdcsel> for u8 {
        #[inline(always)]
        fn from(val: Ltdcsel) -> u8 {
            Ltdcsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mcosel {
        #[doc = "MCO output disabled, no clock on MCO"]
        DISABLE = 0x0,
        #[doc = "SYSCLK system clock selected"]
        SYS = 0x01,
        #[doc = "MSIS clock selected"]
        MSIS = 0x02,
        #[doc = "HSI clock selected"]
        HSI = 0x03,
        #[doc = "HSE clock selected"]
        HSE = 0x04,
        #[doc = "Main PLL clock pll1_r_ck selected"]
        PLL1_R = 0x05,
        #[doc = "LSI clock selected"]
        LSI = 0x06,
        #[doc = "LSE clock selected"]
        LSE = 0x07,
        #[doc = "Internal HSI48 clock selected"]
        HSI48 = 0x08,
        #[doc = "MSIK clock selected"]
        MSIK = 0x09,
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mdfsel {
        #[doc = "HCLK selected"]
        HCLK1 = 0x0,
        #[doc = "PLL1 P (pll1_p_ck) selected"]
        PLL1_P = 0x01,
        #[doc = "PLL3 Q (pll3_q_ck) selected"]
        PLL3_Q = 0x02,
        #[doc = "input pin AUDIOCLK selected"]
        AUDIOCLK = 0x03,
        #[doc = "MSIK clock selected"]
        MSIK = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Mdfsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mdfsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mdfsel {
        #[inline(always)]
        fn from(val: u8) -> Mdfsel {
            Mdfsel::from_bits(val)
        }
    }
    impl From<Mdfsel> for u8 {
        #[inline(always)]
        fn from(val: Mdfsel) -> u8 {
            Mdfsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Msibias {
        #[doc = "MSI bias continuous mode (clock accuracy fast settling time)"]
        CONTINUOUS = 0x0,
        #[doc = "MSI bias sampling mode (ultra-low-power mode)"]
        SAMPLING = 0x01,
    }
    impl Msibias {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Msibias {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Msibias {
        #[inline(always)]
        fn from(val: u8) -> Msibias {
            Msibias::from_bits(val)
        }
    }
    impl From<Msibias> for u8 {
        #[inline(always)]
        fn from(val: Msibias) -> u8 {
            Msibias::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Msipllfast {
        #[doc = "MSI PLL normal start-up"]
        NORMAL = 0x0,
        #[doc = "MSI PLL fast start-up"]
        FAST = 0x01,
    }
    impl Msipllfast {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Msipllfast {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Msipllfast {
        #[inline(always)]
        fn from(val: u8) -> Msipllfast {
            Msipllfast::from_bits(val)
        }
    }
    impl From<Msipllfast> for u8 {
        #[inline(always)]
        fn from(val: Msipllfast) -> u8 {
            Msipllfast::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Msipllsel {
        #[doc = "PLL mode applied to MSIK (MSI kernel) clock output"]
        MSIK = 0x0,
        #[doc = "PLL mode applied to MSIS (MSI system) clock output"]
        MSIS = 0x01,
    }
    impl Msipllsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Msipllsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Msipllsel {
        #[inline(always)]
        fn from(val: u8) -> Msipllsel {
            Msipllsel::from_bits(val)
        }
    }
    impl From<Msipllsel> for u8 {
        #[inline(always)]
        fn from(val: Msipllsel) -> u8 {
            Msipllsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Msirange {
        #[doc = "range 0 around 48 MHz"]
        RANGE_48MHZ = 0x0,
        #[doc = "range 1 around 24 MHz"]
        RANGE_24MHZ = 0x01,
        #[doc = "range 2 around 16 MHz"]
        RANGE_16MHZ = 0x02,
        #[doc = "range 3 around 12 MHz"]
        RANGE_12MHZ = 0x03,
        #[doc = "range 4 around 4 MHz (reset value)"]
        RANGE_4MHZ = 0x04,
        #[doc = "range 5 around 2 MHz"]
        RANGE_2MHZ = 0x05,
        #[doc = "range 6 around 1.33 MHz"]
        RANGE_1_33MHZ = 0x06,
        #[doc = "range 7 around 1 MHz"]
        RANGE_1MHZ = 0x07,
        #[doc = "range 8 around 3.072 MHz"]
        RANGE_3_072MHZ = 0x08,
        #[doc = "range 9 around 1.536 MHz"]
        RANGE_1_536MHZ = 0x09,
        #[doc = "range 10 around 1.024 MHz"]
        RANGE_1_024MHZ = 0x0a,
        #[doc = "range 11 around 768 kHz"]
        RANGE_768KHZ = 0x0b,
        #[doc = "range 12 around 400 kHz"]
        RANGE_400KHZ = 0x0c,
        #[doc = "range 13 around 200 kHz"]
        RANGE_200KHZ = 0x0d,
        #[doc = "range 14 around 133 kHz"]
        RANGE_133KHZ = 0x0e,
        #[doc = "range 15 around 100 kHz"]
        RANGE_100KHZ = 0x0f,
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Msirgsel {
        #[doc = "MSIS/MSIK ranges provided by MSISSRANGE\\[3:0\\]
and MSIKSRANGE\\[3:0\\]
in RCC_CSR"]
        CSR = 0x0,
        #[doc = "MSIS/MSIK ranges provided by MSISRANGE\\[3:0\\]
and MSIKRANGE\\[3:0\\]
in RCC_ICSCR1"]
        ICSCR1 = 0x01,
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Msixsrange {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "range 4 around 4M Hz (reset value)"]
        RANGE_4MHZ = 0x04,
        #[doc = "range 5 around 2 MHz"]
        RANGE_2MHZ = 0x05,
        #[doc = "range 6 around 1.5 MHz"]
        RANGE_1_5MHZ = 0x06,
        #[doc = "range 7 around 1 MHz"]
        RANGE_1MHZ = 0x07,
        #[doc = "range 8 around 3.072 MHz"]
        RANGE_3_072MHZ = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Msixsrange {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Msixsrange {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Msixsrange {
        #[inline(always)]
        fn from(val: u8) -> Msixsrange {
            Msixsrange::from_bits(val)
        }
    }
    impl From<Msixsrange> for u8 {
        #[inline(always)]
        fn from(val: Msixsrange) -> u8 {
            Msixsrange::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Octospisel {
        #[doc = "SYSCLK selected"]
        SYS = 0x0,
        #[doc = "MSIK selected"]
        MSIK = 0x01,
        #[doc = "PLL1 Q (pll1_q_ck) selected, can be up to 200 MHz"]
        PLL1_Q = 0x02,
        #[doc = "PLL2 Q (pll2_q_ck) selected, can be up to 200 MHz"]
        PLL2_Q = 0x03,
    }
    impl Octospisel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Octospisel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Octospisel {
        #[inline(always)]
        fn from(val: u8) -> Octospisel {
            Octospisel::from_bits(val)
        }
    }
    impl From<Octospisel> for u8 {
        #[inline(always)]
        fn from(val: Octospisel) -> u8 {
            Octospisel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Otghssel {
        #[doc = "HSE selected"]
        HSE = 0x0,
        #[doc = "PLL1 “P” (pll1_q_ck) selected,"]
        PLL1_P = 0x01,
        #[doc = "HSE/2 selected"]
        HSE_DIV_2 = 0x02,
        #[doc = "PLL1 “P” divided by 2 (pll1_p_ck/2) selected"]
        PLL1_P_DIV_2 = 0x03,
    }
    impl Otghssel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Otghssel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Otghssel {
        #[inline(always)]
        fn from(val: u8) -> Otghssel {
            Otghssel::from_bits(val)
        }
    }
    impl From<Otghssel> for u8 {
        #[inline(always)]
        fn from(val: Otghssel) -> u8 {
            Otghssel::to_bits(val)
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllmboost {
        #[doc = "division by 1 (bypass)"]
        DIV1 = 0x0,
        #[doc = "division by 2"]
        DIV2 = 0x01,
        #[doc = "division by 4"]
        DIV4 = 0x02,
        #[doc = "division by 6"]
        DIV6 = 0x03,
        #[doc = "division by 8"]
        DIV8 = 0x04,
        #[doc = "division by 10"]
        DIV10 = 0x05,
        #[doc = "division by 12"]
        DIV12 = 0x06,
        #[doc = "division by 14"]
        DIV14 = 0x07,
        #[doc = "division by 16"]
        DIV16 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Pllmboost {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllmboost {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllmboost {
        #[inline(always)]
        fn from(val: u8) -> Pllmboost {
            Pllmboost::from_bits(val)
        }
    }
    impl From<Pllmboost> for u8 {
        #[inline(always)]
        fn from(val: Pllmboost) -> u8 {
            Pllmboost::to_bits(val)
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
        #[doc = "PLL2 input (ref2_ck) clock range frequency between 4 and 8 MHz"]
        FREQ_4TO8MHZ = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "PLL2 input (ref2_ck) clock range frequency between 8 and 16 MHz"]
        FREQ_8TO16MHZ = 0x03,
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
        #[doc = "No clock sent to PLL3"]
        DISABLE = 0x0,
        #[doc = "MSIS clock selected as PLL3 clock entry"]
        MSIS = 0x01,
        #[doc = "HSI clock selected as PLL3 clock entry"]
        HSI = 0x02,
        #[doc = "HSE clock selected as PLL3 clock entry"]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rngsel {
        #[doc = "HSI48 selected"]
        HSI48 = 0x0,
        #[doc = "HSI48 / 2 selected, can be used in Range 4"]
        HSI48_DIV_2 = 0x01,
        #[doc = "HSI selected"]
        HSI = 0x02,
        _RESERVED_3 = 0x03,
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
        #[doc = "No clock selected"]
        DISABLE = 0x0,
        #[doc = "LSE oscillator clock selected"]
        LSE = 0x01,
        #[doc = "LSI oscillator clock selected"]
        LSI = 0x02,
        #[doc = "HSE oscillator clock divided by 32 selected"]
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
    pub enum Saessel {
        #[doc = "SHSI selected"]
        SHSI = 0x0,
        #[doc = "SHSI / 2 selected, can be used in Range 4"]
        SHSI_DIV_2 = 0x01,
    }
    impl Saessel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Saessel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Saessel {
        #[inline(always)]
        fn from(val: u8) -> Saessel {
            Saessel::from_bits(val)
        }
    }
    impl From<Saessel> for u8 {
        #[inline(always)]
        fn from(val: Saessel) -> u8 {
            Saessel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Saisel {
        #[doc = "PLL2 P (pll2_p_ck) selected"]
        PLL2_P = 0x0,
        #[doc = "PLL3 P (pll3_p_ck) selected"]
        PLL3_P = 0x01,
        #[doc = "PLL1 P (pll1_p_ck) selected"]
        PLL1_P = 0x02,
        #[doc = "input pin AUDIOCLK selected"]
        AUDIOCLK = 0x03,
        #[doc = "HSI clock selected"]
        HSI = 0x04,
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
        #[doc = "ICLK clock selected"]
        ICLK = 0x0,
        #[doc = "PLL1 P (pll1_p_ck) selected, in case higher than 48 MHz is needed (for SDR50 mode)"]
        PLL1_P = 0x01,
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
    pub enum Security {
        #[doc = "non secure"]
        NON_SECURE = 0x0,
        #[doc = "secure"]
        SECURE = 0x01,
    }
    impl Security {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Security {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Security {
        #[inline(always)]
        fn from(val: u8) -> Security {
            Security::from_bits(val)
        }
    }
    impl From<Security> for u8 {
        #[inline(always)]
        fn from(val: Security) -> u8 {
            Security::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Spi1sel {
        #[doc = "PCLK2 selected"]
        PCLK2 = 0x0,
        #[doc = "SYSCLK selected"]
        SYS = 0x01,
        #[doc = "HSI selected"]
        HSI = 0x02,
        #[doc = "MSIK selected"]
        MSIK = 0x03,
    }
    impl Spi1sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Spi1sel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Spi1sel {
        #[inline(always)]
        fn from(val: u8) -> Spi1sel {
            Spi1sel::from_bits(val)
        }
    }
    impl From<Spi1sel> for u8 {
        #[inline(always)]
        fn from(val: Spi1sel) -> u8 {
            Spi1sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Spi2sel {
        #[doc = "PCLK2 selected"]
        PCLK1 = 0x0,
        #[doc = "SYSCLK selected"]
        SYS = 0x01,
        #[doc = "HSI selected"]
        HSI = 0x02,
        #[doc = "MSIK selected"]
        MSIK = 0x03,
    }
    impl Spi2sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Spi2sel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Spi2sel {
        #[inline(always)]
        fn from(val: u8) -> Spi2sel {
            Spi2sel::from_bits(val)
        }
    }
    impl From<Spi2sel> for u8 {
        #[inline(always)]
        fn from(val: Spi2sel) -> u8 {
            Spi2sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Spi3sel {
        #[doc = "PCLK2 selected"]
        PCLK3 = 0x0,
        #[doc = "SYSCLK selected"]
        SYS = 0x01,
        #[doc = "HSI selected"]
        HSI = 0x02,
        #[doc = "MSIK selected"]
        MSIK = 0x03,
    }
    impl Spi3sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Spi3sel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Spi3sel {
        #[inline(always)]
        fn from(val: u8) -> Spi3sel {
            Spi3sel::from_bits(val)
        }
    }
    impl From<Spi3sel> for u8 {
        #[inline(always)]
        fn from(val: Spi3sel) -> u8 {
            Spi3sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Stopkerwuck {
        #[doc = "MSIK oscillator automatically enabled when exiting Stop mode"]
        MSIK = 0x0,
        #[doc = "HSI oscillator automatically enabled when exiting Stop mode"]
        HSI = 0x01,
    }
    impl Stopkerwuck {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Stopkerwuck {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Stopkerwuck {
        #[inline(always)]
        fn from(val: u8) -> Stopkerwuck {
            Stopkerwuck::from_bits(val)
        }
    }
    impl From<Stopkerwuck> for u8 {
        #[inline(always)]
        fn from(val: Stopkerwuck) -> u8 {
            Stopkerwuck::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Stopwuck {
        #[doc = "MSIS oscillator selected as wakeup from stop clock and CSS backup clock"]
        MSIS = 0x0,
        #[doc = "HSI oscillator selected as wakeup from stop clock and CSS backup clock"]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sw {
        #[doc = "MSIS selected as system clock"]
        MSIS = 0x0,
        #[doc = "HSI selected as system clock"]
        HSI = 0x01,
        #[doc = "HSE selected as system clock"]
        HSE = 0x02,
        #[doc = "PLL pll1_r_ck selected as system clock"]
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
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Systicksel {
        #[doc = "HCLK/8 selected"]
        HCLK1_DIV_8 = 0x0,
        #[doc = "LSI selected"]
        LSI = 0x01,
        #[doc = "LSE selected"]
        LSE = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Systicksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Systicksel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Systicksel {
        #[inline(always)]
        fn from(val: u8) -> Systicksel {
            Systicksel::from_bits(val)
        }
    }
    impl From<Systicksel> for u8 {
        #[inline(always)]
        fn from(val: Systicksel) -> u8 {
            Systicksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Timicsel {
        #[doc = "No sources can be selected by TIM16, TIM17 and LPTIM2 as internal input capture"]
        DISABLE = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "HSI/256, MSIS/1024 and MSIS/4 generated and can be selected by TIM16, TIM17 and LPTIM2 as internal input capture"]
        HSI256_MSIS1024_MSIS4 = 0x04,
        #[doc = "HSI/256, MSIS/1024 and MSIK/4 generated and can be selected by TIM16, TIM17 and LPTIM2 as internal input capture"]
        HSI256_MSIS1024_MSIK4 = 0x05,
        #[doc = "HSI/256, MSIK/1024 and MSIS/4 generated and can be selected by TIM16, TIM17 and LPTIM2 as internal input capture"]
        HSI256_MSIK1024_MSIS4 = 0x06,
        #[doc = "HSI/256, MSIK/1024 and MSIK/4 generated and can be selected by TIM16, TIM17 and LPTIM2 as internal input capture"]
        HSI256_MSIK1024_MSIK4 = 0x07,
    }
    impl Timicsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Timicsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Timicsel {
        #[inline(always)]
        fn from(val: u8) -> Timicsel {
            Timicsel::from_bits(val)
        }
    }
    impl From<Timicsel> for u8 {
        #[inline(always)]
        fn from(val: Timicsel) -> u8 {
            Timicsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Usart1sel {
        #[doc = "PCLK2 selected"]
        PCLK2 = 0x0,
        #[doc = "SYSCLK selected"]
        SYS = 0x01,
        #[doc = "HSI selected"]
        HSI = 0x02,
        #[doc = "LSE selected"]
        LSE = 0x03,
    }
    impl Usart1sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usart1sel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usart1sel {
        #[inline(always)]
        fn from(val: u8) -> Usart1sel {
            Usart1sel::from_bits(val)
        }
    }
    impl From<Usart1sel> for u8 {
        #[inline(always)]
        fn from(val: Usart1sel) -> u8 {
            Usart1sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Usartsel {
        #[doc = "PCLK1 selected"]
        PCLK1 = 0x0,
        #[doc = "SYSCLK selected"]
        SYS = 0x01,
        #[doc = "HSI selected"]
        HSI = 0x02,
        #[doc = "LSE selected"]
        LSE = 0x03,
    }
    impl Usartsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usartsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usartsel {
        #[inline(always)]
        fn from(val: u8) -> Usartsel {
            Usartsel::from_bits(val)
        }
    }
    impl From<Usartsel> for u8 {
        #[inline(always)]
        fn from(val: Usartsel) -> u8 {
            Usartsel::to_bits(val)
        }
    }
}
