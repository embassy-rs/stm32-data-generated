#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "RCC register block."]
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
    #[doc = "RCC source control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "RCC HSI calibration register."]
    #[inline(always)]
    pub const fn hsicfgr(self) -> crate::common::Reg<regs::Hsicfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "RCC clock recovery RC register."]
    #[inline(always)]
    pub const fn crrcr(self) -> crate::common::Reg<regs::Crrcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "RCC CSI calibration register."]
    #[inline(always)]
    pub const fn csicfgr(self) -> crate::common::Reg<regs::Csicfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "RCC clock configuration register."]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "RCC CPU domain clock configuration register."]
    #[inline(always)]
    pub const fn cdcfgr(self) -> crate::common::Reg<regs::Cdcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "RCC AHB clock configuration register."]
    #[inline(always)]
    pub const fn bmcfgr(self) -> crate::common::Reg<regs::Bmcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "RCC APB clocks configuration register."]
    #[inline(always)]
    pub const fn apbcfgr(self) -> crate::common::Reg<regs::Apbcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "RCC PLLs clock source selection register."]
    #[inline(always)]
    pub const fn pllckselr(self) -> crate::common::Reg<regs::Pllckselr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "RCC PLLs configuration register."]
    #[inline(always)]
    pub const fn pllcfgr(self) -> crate::common::Reg<regs::Pllcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "RCC PLL1 dividers configuration register 1."]
    #[inline(always)]
    pub const fn pll1divr1(self) -> crate::common::Reg<regs::Pll1divr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "RCC PLL1 fractional divider register."]
    #[inline(always)]
    pub const fn pll1fracr(self) -> crate::common::Reg<regs::Pll1fracr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "RCC PLL2 dividers configuration register 1."]
    #[inline(always)]
    pub const fn pll2divr1(self) -> crate::common::Reg<regs::Pll2divr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "RCC PLL2 fractional divider register."]
    #[inline(always)]
    pub const fn pll2fracr(self) -> crate::common::Reg<regs::Pll2fracr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "RCC PLL3 dividers configuration register 1."]
    #[inline(always)]
    pub const fn pll3divr1(self) -> crate::common::Reg<regs::Pll3divr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "RCC PLL3 fractional divider register."]
    #[inline(always)]
    pub const fn pll3fracr(self) -> crate::common::Reg<regs::Pll3fracr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "RCC peripheral kernel clock selection register 1."]
    #[inline(always)]
    pub const fn ccipr1(self) -> crate::common::Reg<regs::Ccipr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "RCC peripheral kernel clock selection register 2."]
    #[inline(always)]
    pub const fn ccipr2(self) -> crate::common::Reg<regs::Ccipr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "RCC peripheral kernel clock selection register 3."]
    #[inline(always)]
    pub const fn ccipr3(self) -> crate::common::Reg<regs::Ccipr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "RCC peripheral kernel clock selection register 4."]
    #[inline(always)]
    pub const fn ccipr4(self) -> crate::common::Reg<regs::Ccipr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "RCC clock source interrupt enable register."]
    #[inline(always)]
    pub const fn cier(self) -> crate::common::Reg<regs::Cier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "RCC clock source interrupt flag register."]
    #[inline(always)]
    pub const fn cifr(self) -> crate::common::Reg<regs::Cifr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "RCC clock source interrupt clear register."]
    #[inline(always)]
    pub const fn cicr(self) -> crate::common::Reg<regs::Cicr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "RCC Backup domain control register."]
    #[inline(always)]
    pub const fn bdcr(self) -> crate::common::Reg<regs::Bdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "RCC clock control and status register."]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "RCC AHB5 peripheral reset register."]
    #[inline(always)]
    pub const fn ahb5rstr(self) -> crate::common::Reg<regs::Ahb5rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "RCC AHB1 peripheral reset register."]
    #[inline(always)]
    pub const fn ahb1rstr(self) -> crate::common::Reg<regs::Ahb1rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "RCC AHB2 peripheral reset register."]
    #[inline(always)]
    pub const fn ahb2rstr(self) -> crate::common::Reg<regs::Ahb2rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "RCC AHB4 peripheral reset register."]
    #[inline(always)]
    pub const fn ahb4rstr(self) -> crate::common::Reg<regs::Ahb4rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "RCC APB1 peripheral reset register 1."]
    #[inline(always)]
    pub const fn apb1rstr1(self) -> crate::common::Reg<regs::Apb1rstr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "RCC APB1 peripheral reset register 2."]
    #[inline(always)]
    pub const fn apb1rstr2(self) -> crate::common::Reg<regs::Apb1rstr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "RCC APB2 peripheral reset register."]
    #[inline(always)]
    pub const fn apb2rstr(self) -> crate::common::Reg<regs::Apb2rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "RCC APB4 peripheral reset register."]
    #[inline(always)]
    pub const fn apb4rstr(self) -> crate::common::Reg<regs::Apb4rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "RCC AHB3 peripheral reset register."]
    #[inline(always)]
    pub const fn ahb3rstr(self) -> crate::common::Reg<regs::Ahb3rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "RCC AXI clocks gating disable register."]
    #[inline(always)]
    pub const fn ckgdisr(self) -> crate::common::Reg<regs::Ckgdisr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "RCC PLL1 dividers configuration register 2."]
    #[inline(always)]
    pub const fn pll1divr2(self) -> crate::common::Reg<regs::Pll1divr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "RCC PLL2 dividers configuration register 2."]
    #[inline(always)]
    pub const fn pll2divr2(self) -> crate::common::Reg<regs::Pll2divr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "RCC PLL3 dividers configuration register 2."]
    #[inline(always)]
    pub const fn pll3divr2(self) -> crate::common::Reg<regs::Pll3divr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "RCC PLL1 Spread Spectrum Clock Generator register."]
    #[inline(always)]
    pub const fn pll1sscgr(self) -> crate::common::Reg<regs::Pll1sscgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "RCC PLL2 Spread Spectrum Clock Generator register."]
    #[inline(always)]
    pub const fn pll2sscgr(self) -> crate::common::Reg<regs::Pll2sscgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "RCC PLL3 Spread Spectrum Clock Generator register."]
    #[inline(always)]
    pub const fn pll3sscgr(self) -> crate::common::Reg<regs::Pll3sscgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "RCC clock protection register."]
    #[inline(always)]
    pub const fn ckprotr(self) -> crate::common::Reg<regs::Ckprotr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "RCC Reset status register."]
    #[inline(always)]
    pub const fn rsr(self) -> crate::common::Reg<regs::Rsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "RCC AHB5 clock enable register."]
    #[inline(always)]
    pub const fn ahb5enr(self) -> crate::common::Reg<regs::Ahb5enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "RCC AHB1 clock enable register."]
    #[inline(always)]
    pub const fn ahb1enr(self) -> crate::common::Reg<regs::Ahb1enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "RCC AHB2 clock enable register."]
    #[inline(always)]
    pub const fn ahb2enr(self) -> crate::common::Reg<regs::Ahb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "RCC AHB4 clock enable register."]
    #[inline(always)]
    pub const fn ahb4enr(self) -> crate::common::Reg<regs::Ahb4enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "RCC APB1 clock enable register 1."]
    #[inline(always)]
    pub const fn apb1enr1(self) -> crate::common::Reg<regs::Apb1enr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "RCC APB1 clock enable register 2."]
    #[inline(always)]
    pub const fn apb1enr2(self) -> crate::common::Reg<regs::Apb1enr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "RCC APB2 clock enable register."]
    #[inline(always)]
    pub const fn apb2enr(self) -> crate::common::Reg<regs::Apb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "RCC APB4 clock enable register."]
    #[inline(always)]
    pub const fn apb4enr(self) -> crate::common::Reg<regs::Apb4enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "RCC AHB3 clock enable register."]
    #[inline(always)]
    pub const fn ahb3enr(self) -> crate::common::Reg<regs::Ahb3enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "RCC AHB5 low-power clock enable register."]
    #[inline(always)]
    pub const fn ahb5lpenr(self) -> crate::common::Reg<regs::Ahb5lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "RCC AHB1 low-power clock enable register."]
    #[inline(always)]
    pub const fn ahb1lpenr(self) -> crate::common::Reg<regs::Ahb1lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "RCC AHB2 low-power clock enable register."]
    #[inline(always)]
    pub const fn ahb2lpenr(self) -> crate::common::Reg<regs::Ahb2lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "RCC AHB4 low-power clock enable register."]
    #[inline(always)]
    pub const fn ahb4lpenr(self) -> crate::common::Reg<regs::Ahb4lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "RCC AHB3 low-power clock enable register."]
    #[inline(always)]
    pub const fn ahb3lpenr(self) -> crate::common::Reg<regs::Ahb3lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "RCC APB1 low-power clock enable register 1."]
    #[inline(always)]
    pub const fn apb1lpenr1(self) -> crate::common::Reg<regs::Apb1lpenr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "RCC APB1 low-power clock enable register 2."]
    #[inline(always)]
    pub const fn apb1lpenr2(self) -> crate::common::Reg<regs::Apb1lpenr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "RCC APB2 low-power clock enable register."]
    #[inline(always)]
    pub const fn apb2lpenr(self) -> crate::common::Reg<regs::Apb2lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "RCC APB4 low-power clock enable register."]
    #[inline(always)]
    pub const fn apb4lpenr(self) -> crate::common::Reg<regs::Apb4lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
}
pub mod regs {
    #[doc = "RCC AHB1 clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1enr(pub u32);
    impl Ahb1enr {
        #[doc = "HSP clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hspen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSP clock enable."]
        #[inline(always)]
        pub const fn set_hspen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "GPDMA1 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn gpdma1en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1 clock enable."]
        #[inline(always)]
        pub const fn set_gpdma1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "OTGFS peripheral clocks enable."]
        #[must_use]
        #[inline(always)]
        pub const fn otgfsen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "OTGFS peripheral clocks enable."]
        #[inline(always)]
        pub const fn set_otgfsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "ADF clocks enable."]
        #[must_use]
        #[inline(always)]
        pub const fn adf1en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "ADF clocks enable."]
        #[inline(always)]
        pub const fn set_adf1en(&mut self, val: bool) {
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
                .field("hspen", &self.hspen())
                .field("gpdma1en", &self.gpdma1en())
                .field("otgfsen", &self.otgfsen())
                .field("adf1en", &self.adf1en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1enr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb1enr {{ hspen: {=bool:?}, gpdma1en: {=bool:?}, otgfsen: {=bool:?}, adf1en: {=bool:?} }}",
                self.hspen(),
                self.gpdma1en(),
                self.otgfsen(),
                self.adf1en()
            )
        }
    }
    #[doc = "RCC AHB1 low-power clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1lpenr(pub u32);
    impl Ahb1lpenr {
        #[doc = "HSP clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn hsplpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSP clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_hsplpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "GPDMA1 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn gpdma1lpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_gpdma1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "OTGFS clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn otgfslpen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "OTGFS clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_otgfslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "ADF clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn adf1lpen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "ADF clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_adf1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("hsplpen", &self.hsplpen())
                .field("gpdma1lpen", &self.gpdma1lpen())
                .field("otgfslpen", &self.otgfslpen())
                .field("adf1lpen", &self.adf1lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1lpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb1lpenr {{ hsplpen: {=bool:?}, gpdma1lpen: {=bool:?}, otgfslpen: {=bool:?}, adf1lpen: {=bool:?} }}",
                self.hsplpen(),
                self.gpdma1lpen(),
                self.otgfslpen(),
                self.adf1lpen()
            )
        }
    }
    #[doc = "RCC AHB1 peripheral reset register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1rstr(pub u32);
    impl Ahb1rstr {
        #[doc = "HSP block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn hsprst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSP block reset."]
        #[inline(always)]
        pub const fn set_hsprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "GPDMA1 blocks reset."]
        #[must_use]
        #[inline(always)]
        pub const fn gpdma1rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1 blocks reset."]
        #[inline(always)]
        pub const fn set_gpdma1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "OTGFS block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn otgfsrst(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "OTGFS block reset."]
        #[inline(always)]
        pub const fn set_otgfsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "ADF block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn adf1rst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "ADF block reset."]
        #[inline(always)]
        pub const fn set_adf1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("hsprst", &self.hsprst())
                .field("gpdma1rst", &self.gpdma1rst())
                .field("otgfsrst", &self.otgfsrst())
                .field("adf1rst", &self.adf1rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb1rstr {{ hsprst: {=bool:?}, gpdma1rst: {=bool:?}, otgfsrst: {=bool:?}, adf1rst: {=bool:?} }}",
                self.hsprst(),
                self.gpdma1rst(),
                self.otgfsrst(),
                self.adf1rst()
            )
        }
    }
    #[doc = "RCC AHB2 clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2enr(pub u32);
    impl Ahb2enr {
        #[doc = "CORDIC clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cordicen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "CORDIC clock enable."]
        #[inline(always)]
        pub const fn set_cordicen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "FMAC clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fmac1en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "FMAC clock enable."]
        #[inline(always)]
        pub const fn set_fmac1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "AHBSRAM1 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ahbsram1en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "AHBSRAM1 clock enable."]
        #[inline(always)]
        pub const fn set_ahbsram1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "AHBSRAM2 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ahbsram2en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "AHBSRAM2 clock enable."]
        #[inline(always)]
        pub const fn set_ahbsram2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "AHBSRAM3 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ahbsram3en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "AHBSRAM3 clock enable."]
        #[inline(always)]
        pub const fn set_ahbsram3en(&mut self, val: bool) {
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
                .field("cordicen", &self.cordicen())
                .field("fmac1en", &self.fmac1en())
                .field("ahbsram1en", &self.ahbsram1en())
                .field("ahbsram2en", &self.ahbsram2en())
                .field("ahbsram3en", &self.ahbsram3en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2enr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb2enr {{ cordicen: {=bool:?}, fmac1en: {=bool:?}, ahbsram1en: {=bool:?}, ahbsram2en: {=bool:?}, ahbsram3en: {=bool:?} }}",
                self.cordicen(),
                self.fmac1en(),
                self.ahbsram1en(),
                self.ahbsram2en(),
                self.ahbsram3en()
            )
        }
    }
    #[doc = "RCC AHB2 low-power clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2lpenr(pub u32);
    impl Ahb2lpenr {
        #[doc = "CORDIC clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn cordiclpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "CORDIC clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_cordiclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "FMAC1 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn fmac1lpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "FMAC1 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_fmac1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "AHBSRAM1 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ahbsram1lpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "AHBSRAM1 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_ahbsram1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "AHBSRAM2 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ahbsram2lpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "AHBSRAM2 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_ahbsram2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "AHBSRAM3 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ahbsram3lpen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "AHBSRAM3 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_ahbsram3lpen(&mut self, val: bool) {
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
                .field("cordiclpen", &self.cordiclpen())
                .field("fmac1lpen", &self.fmac1lpen())
                .field("ahbsram1lpen", &self.ahbsram1lpen())
                .field("ahbsram2lpen", &self.ahbsram2lpen())
                .field("ahbsram3lpen", &self.ahbsram3lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2lpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb2lpenr {{ cordiclpen: {=bool:?}, fmac1lpen: {=bool:?}, ahbsram1lpen: {=bool:?}, ahbsram2lpen: {=bool:?}, ahbsram3lpen: {=bool:?} }}",
                self.cordiclpen(),
                self.fmac1lpen(),
                self.ahbsram1lpen(),
                self.ahbsram2lpen(),
                self.ahbsram3lpen()
            )
        }
    }
    #[doc = "RCC AHB2 peripheral reset register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2rstr(pub u32);
    impl Ahb2rstr {
        #[doc = "CORDIC reset."]
        #[must_use]
        #[inline(always)]
        pub const fn cordicrst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "CORDIC reset."]
        #[inline(always)]
        pub const fn set_cordicrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "FMAC reset."]
        #[must_use]
        #[inline(always)]
        pub const fn fmac1rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "FMAC reset."]
        #[inline(always)]
        pub const fn set_fmac1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
                .field("cordicrst", &self.cordicrst())
                .field("fmac1rst", &self.fmac1rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb2rstr {{ cordicrst: {=bool:?}, fmac1rst: {=bool:?} }}",
                self.cordicrst(),
                self.fmac1rst()
            )
        }
    }
    #[doc = "RCC AHB3 clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3enr(pub u32);
    impl Ahb3enr {
        #[doc = "RNG peripheral clocks enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rngen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RNG peripheral clocks enable."]
        #[inline(always)]
        pub const fn set_rngen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "HASH peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hashen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "HASH peripheral clock enable."]
        #[inline(always)]
        pub const fn set_hashen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ADC12 peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn adc12en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ADC12 peripheral clock enable."]
        #[inline(always)]
        pub const fn set_adc12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "ADC34 peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn adc34en(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "ADC34 peripheral clock enable."]
        #[inline(always)]
        pub const fn set_adc34en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "DAC1 peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dac1en(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1 peripheral clock enable."]
        #[inline(always)]
        pub const fn set_dac1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "DAC2 peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dac2en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC2 peripheral clock enable."]
        #[inline(always)]
        pub const fn set_dac2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "DAC3 peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dac3en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "DAC3 peripheral clock enable."]
        #[inline(always)]
        pub const fn set_dac3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "DAC4 peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dac4en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "DAC4 peripheral clock enable."]
        #[inline(always)]
        pub const fn set_dac4en(&mut self, val: bool) {
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
                .field("rngen", &self.rngen())
                .field("hashen", &self.hashen())
                .field("adc12en", &self.adc12en())
                .field("adc34en", &self.adc34en())
                .field("dac1en", &self.dac1en())
                .field("dac2en", &self.dac2en())
                .field("dac3en", &self.dac3en())
                .field("dac4en", &self.dac4en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb3enr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb3enr {{ rngen: {=bool:?}, hashen: {=bool:?}, adc12en: {=bool:?}, adc34en: {=bool:?}, dac1en: {=bool:?}, dac2en: {=bool:?}, dac3en: {=bool:?}, dac4en: {=bool:?} }}",
                self.rngen(),
                self.hashen(),
                self.adc12en(),
                self.adc34en(),
                self.dac1en(),
                self.dac2en(),
                self.dac3en(),
                self.dac4en()
            )
        }
    }
    #[doc = "RCC AHB3 low-power clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3lpenr(pub u32);
    impl Ahb3lpenr {
        #[doc = "RNG peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn rnglpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RNG peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_rnglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "HASH peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn hashlpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "HASH peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_hashlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ADC12 peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn adc12lpen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ADC12 peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_adc12lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "ADC34 peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn adc34lpen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "ADC34 peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_adc34lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "DAC1 peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dac1lpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1 peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_dac1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "DAC2 peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dac2lpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC2 peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_dac2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "DAC3 peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dac3lpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "DAC3 peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_dac3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "DAC4 peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dac4lpen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "DAC4 peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_dac4lpen(&mut self, val: bool) {
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
                .field("rnglpen", &self.rnglpen())
                .field("hashlpen", &self.hashlpen())
                .field("adc12lpen", &self.adc12lpen())
                .field("adc34lpen", &self.adc34lpen())
                .field("dac1lpen", &self.dac1lpen())
                .field("dac2lpen", &self.dac2lpen())
                .field("dac3lpen", &self.dac3lpen())
                .field("dac4lpen", &self.dac4lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb3lpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb3lpenr {{ rnglpen: {=bool:?}, hashlpen: {=bool:?}, adc12lpen: {=bool:?}, adc34lpen: {=bool:?}, dac1lpen: {=bool:?}, dac2lpen: {=bool:?}, dac3lpen: {=bool:?}, dac4lpen: {=bool:?} }}",
                self.rnglpen(),
                self.hashlpen(),
                self.adc12lpen(),
                self.adc34lpen(),
                self.dac1lpen(),
                self.dac2lpen(),
                self.dac3lpen(),
                self.dac4lpen()
            )
        }
    }
    #[doc = "RCC AHB3 peripheral reset register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3rstr(pub u32);
    impl Ahb3rstr {
        #[doc = "random number generator block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn rngrst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "random number generator block reset."]
        #[inline(always)]
        pub const fn set_rngrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "HASH block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn hashrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "HASH block reset."]
        #[inline(always)]
        pub const fn set_hashrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ADC12 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn adc12rst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ADC12 block reset."]
        #[inline(always)]
        pub const fn set_adc12rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "ADC34 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn adc34rst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "ADC34 block reset."]
        #[inline(always)]
        pub const fn set_adc34rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "DAC1 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn dac1rst(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1 block reset."]
        #[inline(always)]
        pub const fn set_dac1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "DAC2 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn dac2rst(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC2 block reset."]
        #[inline(always)]
        pub const fn set_dac2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "DAC3 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn dac3rst(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "DAC3 block reset."]
        #[inline(always)]
        pub const fn set_dac3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "DAC4block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn dac4rst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "DAC4block reset."]
        #[inline(always)]
        pub const fn set_dac4rst(&mut self, val: bool) {
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
                .field("rngrst", &self.rngrst())
                .field("hashrst", &self.hashrst())
                .field("adc12rst", &self.adc12rst())
                .field("adc34rst", &self.adc34rst())
                .field("dac1rst", &self.dac1rst())
                .field("dac2rst", &self.dac2rst())
                .field("dac3rst", &self.dac3rst())
                .field("dac4rst", &self.dac4rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb3rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb3rstr {{ rngrst: {=bool:?}, hashrst: {=bool:?}, adc12rst: {=bool:?}, adc34rst: {=bool:?}, dac1rst: {=bool:?}, dac2rst: {=bool:?}, dac3rst: {=bool:?}, dac4rst: {=bool:?} }}",
                self.rngrst(),
                self.hashrst(),
                self.adc12rst(),
                self.adc34rst(),
                self.dac1rst(),
                self.dac2rst(),
                self.dac3rst(),
                self.dac4rst()
            )
        }
    }
    #[doc = "RCC AHB4 clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb4enr(pub u32);
    impl Ahb4enr {
        #[doc = "GPIOA peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOA peripheral clock enable."]
        #[inline(always)]
        pub const fn set_gpioaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPIOB peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioben(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOB peripheral clock enable."]
        #[inline(always)]
        pub const fn set_gpioben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "GPIOC peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiocen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOC peripheral clock enable."]
        #[inline(always)]
        pub const fn set_gpiocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "GPIOD peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioden(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOD peripheral clock enable."]
        #[inline(always)]
        pub const fn set_gpioden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "GPIOE peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioeen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOE peripheral clock enable."]
        #[inline(always)]
        pub const fn set_gpioeen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "GPIOF peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiofen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOF peripheral clock enable."]
        #[inline(always)]
        pub const fn set_gpiofen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "GPIOG peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiogen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOG peripheral clock enable."]
        #[inline(always)]
        pub const fn set_gpiogen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "GPIOH peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiohen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOH peripheral clock enable."]
        #[inline(always)]
        pub const fn set_gpiohen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "GPIOI peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioien(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOI peripheral clock enable."]
        #[inline(always)]
        pub const fn set_gpioien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CRC clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn crcen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "CRC clock enable."]
        #[inline(always)]
        pub const fn set_crcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Backup RAM clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn bkpramen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Backup RAM clock enable."]
        #[inline(always)]
        pub const fn set_bkpramen(&mut self, val: bool) {
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
                .field("crcen", &self.crcen())
                .field("bkpramen", &self.bkpramen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb4enr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb4enr {{ gpioaen: {=bool:?}, gpioben: {=bool:?}, gpiocen: {=bool:?}, gpioden: {=bool:?}, gpioeen: {=bool:?}, gpiofen: {=bool:?}, gpiogen: {=bool:?}, gpiohen: {=bool:?}, gpioien: {=bool:?}, crcen: {=bool:?}, bkpramen: {=bool:?} }}",
                self.gpioaen(),
                self.gpioben(),
                self.gpiocen(),
                self.gpioden(),
                self.gpioeen(),
                self.gpiofen(),
                self.gpiogen(),
                self.gpiohen(),
                self.gpioien(),
                self.crcen(),
                self.bkpramen()
            )
        }
    }
    #[doc = "RCC AHB4 low-power clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb4lpenr(pub u32);
    impl Ahb4lpenr {
        #[doc = "GPIOA peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioalpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOA peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_gpioalpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPIOB peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioblpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOB peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_gpioblpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "GPIOC peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioclpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOC peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_gpioclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "GPIOD peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiodlpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOD peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_gpiodlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "GPIOE peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioelpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOE peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_gpioelpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "GPIOF peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioflpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOF peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_gpioflpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "GPIOG peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioglpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOG peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_gpioglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "GPIOH peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiohlpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOH peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_gpiohlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "GPIOI peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioilpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOI peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_gpioilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CRC clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn crclpen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "CRC clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_crclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Backup RAM clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn bkpramlpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Backup RAM clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_bkpramlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
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
                .field("crclpen", &self.crclpen())
                .field("bkpramlpen", &self.bkpramlpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb4lpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb4lpenr {{ gpioalpen: {=bool:?}, gpioblpen: {=bool:?}, gpioclpen: {=bool:?}, gpiodlpen: {=bool:?}, gpioelpen: {=bool:?}, gpioflpen: {=bool:?}, gpioglpen: {=bool:?}, gpiohlpen: {=bool:?}, gpioilpen: {=bool:?}, crclpen: {=bool:?}, bkpramlpen: {=bool:?} }}",
                self.gpioalpen(),
                self.gpioblpen(),
                self.gpioclpen(),
                self.gpiodlpen(),
                self.gpioelpen(),
                self.gpioflpen(),
                self.gpioglpen(),
                self.gpiohlpen(),
                self.gpioilpen(),
                self.crclpen(),
                self.bkpramlpen()
            )
        }
    }
    #[doc = "RCC AHB4 peripheral reset register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb4rstr(pub u32);
    impl Ahb4rstr {
        #[doc = "GPIOA block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioarst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOA block reset."]
        #[inline(always)]
        pub const fn set_gpioarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPIOB block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiobrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOB block reset."]
        #[inline(always)]
        pub const fn set_gpiobrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "GPIOC block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiocrst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOC block reset."]
        #[inline(always)]
        pub const fn set_gpiocrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "GPIOD block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiodrst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOD block reset."]
        #[inline(always)]
        pub const fn set_gpiodrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "GPIOE block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioerst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOE block reset."]
        #[inline(always)]
        pub const fn set_gpioerst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "GPIOF block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiofrst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOF block reset."]
        #[inline(always)]
        pub const fn set_gpiofrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "GPIOG block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiogrst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOG block reset."]
        #[inline(always)]
        pub const fn set_gpiogrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "GPIOH block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiohrst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOH block reset."]
        #[inline(always)]
        pub const fn set_gpiohrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "GPIOI block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioirst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOI block reset."]
        #[inline(always)]
        pub const fn set_gpioirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CRC block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn crcrst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "CRC block reset."]
        #[inline(always)]
        pub const fn set_crcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
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
                .field("crcrst", &self.crcrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb4rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb4rstr {{ gpioarst: {=bool:?}, gpiobrst: {=bool:?}, gpiocrst: {=bool:?}, gpiodrst: {=bool:?}, gpioerst: {=bool:?}, gpiofrst: {=bool:?}, gpiogrst: {=bool:?}, gpiohrst: {=bool:?}, gpioirst: {=bool:?}, crcrst: {=bool:?} }}",
                self.gpioarst(),
                self.gpiobrst(),
                self.gpiocrst(),
                self.gpiodrst(),
                self.gpioerst(),
                self.gpiofrst(),
                self.gpiogrst(),
                self.gpiohrst(),
                self.gpioirst(),
                self.crcrst()
            )
        }
    }
    #[doc = "RCC AHB5 clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb5enr(pub u32);
    impl Ahb5enr {
        #[doc = "HPDMA1 peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hpdma1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "HPDMA1 peripheral clock enable."]
        #[inline(always)]
        pub const fn set_hpdma1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "FMC peripheral clocks enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fmcen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "FMC peripheral clocks enable."]
        #[inline(always)]
        pub const fn set_fmcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "XSPI1 peripheral clocks enable."]
        #[must_use]
        #[inline(always)]
        pub const fn xspi1en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "XSPI1 peripheral clocks enable."]
        #[inline(always)]
        pub const fn set_xspi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SDMMC1 and DB_SDMMC1 peripheral clocks enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sdmmc1en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC1 and DB_SDMMC1 peripheral clocks enable."]
        #[inline(always)]
        pub const fn set_sdmmc1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "NPU clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn npuen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "NPU clock enable."]
        #[inline(always)]
        pub const fn set_npuen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ahb5enr {
        #[inline(always)]
        fn default() -> Ahb5enr {
            Ahb5enr(0)
        }
    }
    impl core::fmt::Debug for Ahb5enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb5enr")
                .field("hpdma1en", &self.hpdma1en())
                .field("fmcen", &self.fmcen())
                .field("xspi1en", &self.xspi1en())
                .field("sdmmc1en", &self.sdmmc1en())
                .field("npuen", &self.npuen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb5enr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb5enr {{ hpdma1en: {=bool:?}, fmcen: {=bool:?}, xspi1en: {=bool:?}, sdmmc1en: {=bool:?}, npuen: {=bool:?} }}",
                self.hpdma1en(),
                self.fmcen(),
                self.xspi1en(),
                self.sdmmc1en(),
                self.npuen()
            )
        }
    }
    #[doc = "RCC AHB5 low-power clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb5lpenr(pub u32);
    impl Ahb5lpenr {
        #[doc = "HPDMA1 low-power peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hpdma1lpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "HPDMA1 low-power peripheral clock enable."]
        #[inline(always)]
        pub const fn set_hpdma1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "FLASH low-power peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flashlpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "FLASH low-power peripheral clock enable."]
        #[inline(always)]
        pub const fn set_flashlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "FMC and MCE3 peripheral clocks enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn fmclpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "FMC and MCE3 peripheral clocks enable in low-power mode."]
        #[inline(always)]
        pub const fn set_fmclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "XSPI1 and MCE1 low-power peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn xspi1lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "XSPI1 and MCE1 low-power peripheral clock enable."]
        #[inline(always)]
        pub const fn set_xspi1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SDMMC1 and SDMMC1 delay low-power peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sdmmc1lpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC1 and SDMMC1 delay low-power peripheral clock enable."]
        #[inline(always)]
        pub const fn set_sdmmc1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "AXISRAM\\[4:1\\]
low-power peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn axisramlpen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "AXISRAM\\[4:1\\]
low-power peripheral clock enable."]
        #[inline(always)]
        pub const fn set_axisramlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "DTCM1 low-power peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dtcm1lpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "DTCM1 low-power peripheral clock enable."]
        #[inline(always)]
        pub const fn set_dtcm1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "DTCM2 low-power peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dtcm2lpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DTCM2 low-power peripheral clock enable."]
        #[inline(always)]
        pub const fn set_dtcm2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "ITCM low-power peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn itcmlpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "ITCM low-power peripheral clock enable."]
        #[inline(always)]
        pub const fn set_itcmlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "NPU low-power peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn npulpen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "NPU low-power peripheral clock enable."]
        #[inline(always)]
        pub const fn set_npulpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ahb5lpenr {
        #[inline(always)]
        fn default() -> Ahb5lpenr {
            Ahb5lpenr(0)
        }
    }
    impl core::fmt::Debug for Ahb5lpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb5lpenr")
                .field("hpdma1lpen", &self.hpdma1lpen())
                .field("flashlpen", &self.flashlpen())
                .field("fmclpen", &self.fmclpen())
                .field("xspi1lpen", &self.xspi1lpen())
                .field("sdmmc1lpen", &self.sdmmc1lpen())
                .field("axisramlpen", &self.axisramlpen())
                .field("dtcm1lpen", &self.dtcm1lpen())
                .field("dtcm2lpen", &self.dtcm2lpen())
                .field("itcmlpen", &self.itcmlpen())
                .field("npulpen", &self.npulpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb5lpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb5lpenr {{ hpdma1lpen: {=bool:?}, flashlpen: {=bool:?}, fmclpen: {=bool:?}, xspi1lpen: {=bool:?}, sdmmc1lpen: {=bool:?}, axisramlpen: {=bool:?}, dtcm1lpen: {=bool:?}, dtcm2lpen: {=bool:?}, itcmlpen: {=bool:?}, npulpen: {=bool:?} }}",
                self.hpdma1lpen(),
                self.flashlpen(),
                self.fmclpen(),
                self.xspi1lpen(),
                self.sdmmc1lpen(),
                self.axisramlpen(),
                self.dtcm1lpen(),
                self.dtcm2lpen(),
                self.itcmlpen(),
                self.npulpen()
            )
        }
    }
    #[doc = "RCC AHB5 peripheral reset register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb5rstr(pub u32);
    impl Ahb5rstr {
        #[doc = "HPDMA1 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn hpdma1rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "HPDMA1 block reset."]
        #[inline(always)]
        pub const fn set_hpdma1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "FMC and MCE3 blocks reset."]
        #[must_use]
        #[inline(always)]
        pub const fn fmcrst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "FMC and MCE3 blocks reset."]
        #[inline(always)]
        pub const fn set_fmcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "XSPI1 and MCE1 blocks reset."]
        #[must_use]
        #[inline(always)]
        pub const fn xspi1rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "XSPI1 and MCE1 blocks reset."]
        #[inline(always)]
        pub const fn set_xspi1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SDMMC1 and DB_SDMMC1 blocks reset."]
        #[must_use]
        #[inline(always)]
        pub const fn sdmmc1rst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC1 and DB_SDMMC1 blocks reset."]
        #[inline(always)]
        pub const fn set_sdmmc1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "NPU block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn npurst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "NPU block reset."]
        #[inline(always)]
        pub const fn set_npurst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ahb5rstr {
        #[inline(always)]
        fn default() -> Ahb5rstr {
            Ahb5rstr(0)
        }
    }
    impl core::fmt::Debug for Ahb5rstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb5rstr")
                .field("hpdma1rst", &self.hpdma1rst())
                .field("fmcrst", &self.fmcrst())
                .field("xspi1rst", &self.xspi1rst())
                .field("sdmmc1rst", &self.sdmmc1rst())
                .field("npurst", &self.npurst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb5rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb5rstr {{ hpdma1rst: {=bool:?}, fmcrst: {=bool:?}, xspi1rst: {=bool:?}, sdmmc1rst: {=bool:?}, npurst: {=bool:?} }}",
                self.hpdma1rst(),
                self.fmcrst(),
                self.xspi1rst(),
                self.sdmmc1rst(),
                self.npurst()
            )
        }
    }
    #[doc = "RCC APB1 clock enable register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1enr1(pub u32);
    impl Apb1enr1 {
        #[doc = "TIM2 peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim2en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 peripheral clock enable."]
        #[inline(always)]
        pub const fn set_tim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim3en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 peripheral clock enable."]
        #[inline(always)]
        pub const fn set_tim3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim4en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 peripheral clock enable."]
        #[inline(always)]
        pub const fn set_tim4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5 peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim5en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 peripheral clock enable."]
        #[inline(always)]
        pub const fn set_tim5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim6en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 peripheral clock enable."]
        #[inline(always)]
        pub const fn set_tim6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim7en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 peripheral clock enable."]
        #[inline(always)]
        pub const fn set_tim7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM12 peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim12en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12 peripheral clock enable."]
        #[inline(always)]
        pub const fn set_tim12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIM13 peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim13en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIM13 peripheral clock enable."]
        #[inline(always)]
        pub const fn set_tim13en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIM14 peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim14en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM14 peripheral clock enable."]
        #[inline(always)]
        pub const fn set_tim14en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LPTIM1 peripheral clocks enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim1en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 peripheral clocks enable."]
        #[inline(always)]
        pub const fn set_lptim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "WWDG clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn wwdgen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG clock enable."]
        #[inline(always)]
        pub const fn set_wwdgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "TIM10 peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim10en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "TIM10 peripheral clock enable."]
        #[inline(always)]
        pub const fn set_tim10en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TIM11 peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim11en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIM11 peripheral clock enable."]
        #[inline(always)]
        pub const fn set_tim11en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "SPI2 peripheral clocks enable."]
        #[must_use]
        #[inline(always)]
        pub const fn spi2en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 peripheral clocks enable."]
        #[inline(always)]
        pub const fn set_spi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 peripheral clocks enable."]
        #[must_use]
        #[inline(always)]
        pub const fn spi3en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 peripheral clocks enable."]
        #[inline(always)]
        pub const fn set_spi3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "USART2peripheral clocks enable."]
        #[must_use]
        #[inline(always)]
        pub const fn usart2en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2peripheral clocks enable."]
        #[inline(always)]
        pub const fn set_usart2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 peripheral clocks enable."]
        #[must_use]
        #[inline(always)]
        pub const fn usart3en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 peripheral clocks enable."]
        #[inline(always)]
        pub const fn set_usart3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 peripheral clocks enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uart4en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 peripheral clocks enable."]
        #[inline(always)]
        pub const fn set_uart4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 peripheral clocks enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uart5en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 peripheral clocks enable."]
        #[inline(always)]
        pub const fn set_uart5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 peripheral clocks enable."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 peripheral clocks enable."]
        #[inline(always)]
        pub const fn set_i2c1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 peripheral clocks enable."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c2en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 peripheral clocks enable."]
        #[inline(always)]
        pub const fn set_i2c2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I2C3 peripheral clocks enable."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c3en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 peripheral clocks enable."]
        #[inline(always)]
        pub const fn set_i2c3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "I3C1 peripheral clocks enable."]
        #[must_use]
        #[inline(always)]
        pub const fn i3c1en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "I3C1 peripheral clocks enable."]
        #[inline(always)]
        pub const fn set_i3c1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "UART7 peripheral clocks enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uart7en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "UART7 peripheral clocks enable."]
        #[inline(always)]
        pub const fn set_uart7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "UART8 peripheral clocks enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uart8en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "UART8 peripheral clocks enable."]
        #[inline(always)]
        pub const fn set_uart8en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("tim12en", &self.tim12en())
                .field("tim13en", &self.tim13en())
                .field("tim14en", &self.tim14en())
                .field("lptim1en", &self.lptim1en())
                .field("wwdgen", &self.wwdgen())
                .field("tim10en", &self.tim10en())
                .field("tim11en", &self.tim11en())
                .field("spi2en", &self.spi2en())
                .field("spi3en", &self.spi3en())
                .field("usart2en", &self.usart2en())
                .field("usart3en", &self.usart3en())
                .field("uart4en", &self.uart4en())
                .field("uart5en", &self.uart5en())
                .field("i2c1en", &self.i2c1en())
                .field("i2c2en", &self.i2c2en())
                .field("i2c3en", &self.i2c3en())
                .field("i3c1en", &self.i3c1en())
                .field("uart7en", &self.uart7en())
                .field("uart8en", &self.uart8en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1enr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb1enr1 {{ tim2en: {=bool:?}, tim3en: {=bool:?}, tim4en: {=bool:?}, tim5en: {=bool:?}, tim6en: {=bool:?}, tim7en: {=bool:?}, tim12en: {=bool:?}, tim13en: {=bool:?}, tim14en: {=bool:?}, lptim1en: {=bool:?}, wwdgen: {=bool:?}, tim10en: {=bool:?}, tim11en: {=bool:?}, spi2en: {=bool:?}, spi3en: {=bool:?}, usart2en: {=bool:?}, usart3en: {=bool:?}, uart4en: {=bool:?}, uart5en: {=bool:?}, i2c1en: {=bool:?}, i2c2en: {=bool:?}, i2c3en: {=bool:?}, i3c1en: {=bool:?}, uart7en: {=bool:?}, uart8en: {=bool:?} }}",
                self.tim2en(),
                self.tim3en(),
                self.tim4en(),
                self.tim5en(),
                self.tim6en(),
                self.tim7en(),
                self.tim12en(),
                self.tim13en(),
                self.tim14en(),
                self.lptim1en(),
                self.wwdgen(),
                self.tim10en(),
                self.tim11en(),
                self.spi2en(),
                self.spi3en(),
                self.usart2en(),
                self.usart3en(),
                self.uart4en(),
                self.uart5en(),
                self.i2c1en(),
                self.i2c2en(),
                self.i2c3en(),
                self.i3c1en(),
                self.uart7en(),
                self.uart8en()
            )
        }
    }
    #[doc = "RCC APB1 clock enable register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1enr2(pub u32);
    impl Apb1enr2 {
        #[doc = "clock recovery system peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn crsen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "clock recovery system peripheral clock enable."]
        #[inline(always)]
        pub const fn set_crsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "FDCAN peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fdcanen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN peripheral clock enable."]
        #[inline(always)]
        pub const fn set_fdcanen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "UCPD peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ucpd1en(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "UCPD peripheral clock enable."]
        #[inline(always)]
        pub const fn set_ucpd1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
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
                .field("crsen", &self.crsen())
                .field("fdcanen", &self.fdcanen())
                .field("ucpd1en", &self.ucpd1en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1enr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb1enr2 {{ crsen: {=bool:?}, fdcanen: {=bool:?}, ucpd1en: {=bool:?} }}",
                self.crsen(),
                self.fdcanen(),
                self.ucpd1en()
            )
        }
    }
    #[doc = "RCC APB1 low-power clock enable register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1lpenr1(pub u32);
    impl Apb1lpenr1 {
        #[doc = "TIM2 peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim2lpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim3lpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim4lpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5 peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim5lpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim6lpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim6lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim7lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim7lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM12 peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim12lpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12 peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim12lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIM13 peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim13lpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIM13 peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim13lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIM14 peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim14lpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM14 peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim14lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LPTIM1 peripheral clocks enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim1lpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 peripheral clocks enable in low-power mode."]
        #[inline(always)]
        pub const fn set_lptim1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "WWDG clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn wwdglpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_wwdglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "TIM10 peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim10lpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "TIM10 peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim10lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TIM11 peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim11lpen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIM11 peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim11lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "SPI2 peripheral clocks enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn spi2lpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 peripheral clocks enable in low-power mode."]
        #[inline(always)]
        pub const fn set_spi2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 peripheral clocks enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn spi3lpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 peripheral clocks enable in low-power mode."]
        #[inline(always)]
        pub const fn set_spi3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "USART2 peripheral clocks enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn usart2lpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 peripheral clocks enable in low-power mode."]
        #[inline(always)]
        pub const fn set_usart2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 peripheral clocks enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn usart3lpen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 peripheral clocks enable in low-power mode."]
        #[inline(always)]
        pub const fn set_usart3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 peripheral clocks enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn uart4lpen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 peripheral clocks enable in low-power mode."]
        #[inline(always)]
        pub const fn set_uart4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 peripheral clocks enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn uart5lpen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 peripheral clocks enable in low-power mode."]
        #[inline(always)]
        pub const fn set_uart5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 peripheral clocks enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1lpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 peripheral clocks enable in low-power mode."]
        #[inline(always)]
        pub const fn set_i2c1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 peripheral clocks enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c2lpen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 peripheral clocks enable in low-power mode."]
        #[inline(always)]
        pub const fn set_i2c2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I2C3 peripheral clocks enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c3lpen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 peripheral clocks enable in low-power mode."]
        #[inline(always)]
        pub const fn set_i2c3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "I3C1 peripheral clocks enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn i3c1lpen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "I3C1 peripheral clocks enable in low-power mode."]
        #[inline(always)]
        pub const fn set_i3c1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "UART7 peripheral clocks enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn uart7lpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "UART7 peripheral clocks enable in low-power mode."]
        #[inline(always)]
        pub const fn set_uart7lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "UART8 peripheral clocks enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn uart8lpen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "UART8 peripheral clocks enable in low-power mode."]
        #[inline(always)]
        pub const fn set_uart8lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Apb1lpenr1 {
        #[inline(always)]
        fn default() -> Apb1lpenr1 {
            Apb1lpenr1(0)
        }
    }
    impl core::fmt::Debug for Apb1lpenr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1lpenr1")
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
                .field("wwdglpen", &self.wwdglpen())
                .field("tim10lpen", &self.tim10lpen())
                .field("tim11lpen", &self.tim11lpen())
                .field("spi2lpen", &self.spi2lpen())
                .field("spi3lpen", &self.spi3lpen())
                .field("usart2lpen", &self.usart2lpen())
                .field("usart3lpen", &self.usart3lpen())
                .field("uart4lpen", &self.uart4lpen())
                .field("uart5lpen", &self.uart5lpen())
                .field("i2c1lpen", &self.i2c1lpen())
                .field("i2c2lpen", &self.i2c2lpen())
                .field("i2c3lpen", &self.i2c3lpen())
                .field("i3c1lpen", &self.i3c1lpen())
                .field("uart7lpen", &self.uart7lpen())
                .field("uart8lpen", &self.uart8lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1lpenr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb1lpenr1 {{ tim2lpen: {=bool:?}, tim3lpen: {=bool:?}, tim4lpen: {=bool:?}, tim5lpen: {=bool:?}, tim6lpen: {=bool:?}, tim7lpen: {=bool:?}, tim12lpen: {=bool:?}, tim13lpen: {=bool:?}, tim14lpen: {=bool:?}, lptim1lpen: {=bool:?}, wwdglpen: {=bool:?}, tim10lpen: {=bool:?}, tim11lpen: {=bool:?}, spi2lpen: {=bool:?}, spi3lpen: {=bool:?}, usart2lpen: {=bool:?}, usart3lpen: {=bool:?}, uart4lpen: {=bool:?}, uart5lpen: {=bool:?}, i2c1lpen: {=bool:?}, i2c2lpen: {=bool:?}, i2c3lpen: {=bool:?}, i3c1lpen: {=bool:?}, uart7lpen: {=bool:?}, uart8lpen: {=bool:?} }}",
                self.tim2lpen(),
                self.tim3lpen(),
                self.tim4lpen(),
                self.tim5lpen(),
                self.tim6lpen(),
                self.tim7lpen(),
                self.tim12lpen(),
                self.tim13lpen(),
                self.tim14lpen(),
                self.lptim1lpen(),
                self.wwdglpen(),
                self.tim10lpen(),
                self.tim11lpen(),
                self.spi2lpen(),
                self.spi3lpen(),
                self.usart2lpen(),
                self.usart3lpen(),
                self.uart4lpen(),
                self.uart5lpen(),
                self.i2c1lpen(),
                self.i2c2lpen(),
                self.i2c3lpen(),
                self.i3c1lpen(),
                self.uart7lpen(),
                self.uart8lpen()
            )
        }
    }
    #[doc = "RCC APB1 low-power clock enable register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1lpenr2(pub u32);
    impl Apb1lpenr2 {
        #[doc = "clock recovery system peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn crslpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "clock recovery system peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_crslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "FDCAN peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn fdcanlpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_fdcanlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "UCPD peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ucpd1lpen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "UCPD peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_ucpd1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Apb1lpenr2 {
        #[inline(always)]
        fn default() -> Apb1lpenr2 {
            Apb1lpenr2(0)
        }
    }
    impl core::fmt::Debug for Apb1lpenr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1lpenr2")
                .field("crslpen", &self.crslpen())
                .field("fdcanlpen", &self.fdcanlpen())
                .field("ucpd1lpen", &self.ucpd1lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1lpenr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb1lpenr2 {{ crslpen: {=bool:?}, fdcanlpen: {=bool:?}, ucpd1lpen: {=bool:?} }}",
                self.crslpen(),
                self.fdcanlpen(),
                self.ucpd1lpen()
            )
        }
    }
    #[doc = "RCC APB1 peripheral reset register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1rstr1(pub u32);
    impl Apb1rstr1 {
        #[doc = "TIM2 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tim2rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 block reset."]
        #[inline(always)]
        pub const fn set_tim2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tim3rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 block reset."]
        #[inline(always)]
        pub const fn set_tim3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tim4rst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 block reset."]
        #[inline(always)]
        pub const fn set_tim4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tim5rst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 block reset."]
        #[inline(always)]
        pub const fn set_tim5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tim6rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 block reset."]
        #[inline(always)]
        pub const fn set_tim6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tim7rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 block reset."]
        #[inline(always)]
        pub const fn set_tim7rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM12 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tim12rst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12 block reset."]
        #[inline(always)]
        pub const fn set_tim12rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIM13 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tim13rst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIM13 block reset."]
        #[inline(always)]
        pub const fn set_tim13rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIM14 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tim14rst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM14 block reset."]
        #[inline(always)]
        pub const fn set_tim14rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LPTIM1 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim1rst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 block reset."]
        #[inline(always)]
        pub const fn set_lptim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "TIM10 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tim10rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "TIM10 block reset."]
        #[inline(always)]
        pub const fn set_tim10rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TIM11 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tim11rst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIM11 block reset."]
        #[inline(always)]
        pub const fn set_tim11rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "SPI2S2 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn spi2rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2S2 block reset."]
        #[inline(always)]
        pub const fn set_spi2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI2S3 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn spi3rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2S3 block reset."]
        #[inline(always)]
        pub const fn set_spi3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "USART2 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn usart2rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 block reset."]
        #[inline(always)]
        pub const fn set_usart2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn usart3rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 block reset."]
        #[inline(always)]
        pub const fn set_usart3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn uart4rst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 block reset."]
        #[inline(always)]
        pub const fn set_uart4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn uart5rst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 block reset."]
        #[inline(always)]
        pub const fn set_uart5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1/I3C1 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1/I3C1 block reset."]
        #[inline(always)]
        pub const fn set_i2c1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c2rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 block reset."]
        #[inline(always)]
        pub const fn set_i2c2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I2C3 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c3rst(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 block reset."]
        #[inline(always)]
        pub const fn set_i2c3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "I3C1 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn i3c1rst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "I3C1 block reset."]
        #[inline(always)]
        pub const fn set_i3c1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "UART7 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn uart7rst(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "UART7 block reset."]
        #[inline(always)]
        pub const fn set_uart7rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "UART8 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn uart8rst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "UART8 block reset."]
        #[inline(always)]
        pub const fn set_uart8rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("tim12rst", &self.tim12rst())
                .field("tim13rst", &self.tim13rst())
                .field("tim14rst", &self.tim14rst())
                .field("lptim1rst", &self.lptim1rst())
                .field("tim10rst", &self.tim10rst())
                .field("tim11rst", &self.tim11rst())
                .field("spi2rst", &self.spi2rst())
                .field("spi3rst", &self.spi3rst())
                .field("usart2rst", &self.usart2rst())
                .field("usart3rst", &self.usart3rst())
                .field("uart4rst", &self.uart4rst())
                .field("uart5rst", &self.uart5rst())
                .field("i2c1rst", &self.i2c1rst())
                .field("i2c2rst", &self.i2c2rst())
                .field("i2c3rst", &self.i2c3rst())
                .field("i3c1rst", &self.i3c1rst())
                .field("uart7rst", &self.uart7rst())
                .field("uart8rst", &self.uart8rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1rstr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb1rstr1 {{ tim2rst: {=bool:?}, tim3rst: {=bool:?}, tim4rst: {=bool:?}, tim5rst: {=bool:?}, tim6rst: {=bool:?}, tim7rst: {=bool:?}, tim12rst: {=bool:?}, tim13rst: {=bool:?}, tim14rst: {=bool:?}, lptim1rst: {=bool:?}, tim10rst: {=bool:?}, tim11rst: {=bool:?}, spi2rst: {=bool:?}, spi3rst: {=bool:?}, usart2rst: {=bool:?}, usart3rst: {=bool:?}, uart4rst: {=bool:?}, uart5rst: {=bool:?}, i2c1rst: {=bool:?}, i2c2rst: {=bool:?}, i2c3rst: {=bool:?}, i3c1rst: {=bool:?}, uart7rst: {=bool:?}, uart8rst: {=bool:?} }}",
                self.tim2rst(),
                self.tim3rst(),
                self.tim4rst(),
                self.tim5rst(),
                self.tim6rst(),
                self.tim7rst(),
                self.tim12rst(),
                self.tim13rst(),
                self.tim14rst(),
                self.lptim1rst(),
                self.tim10rst(),
                self.tim11rst(),
                self.spi2rst(),
                self.spi3rst(),
                self.usart2rst(),
                self.usart3rst(),
                self.uart4rst(),
                self.uart5rst(),
                self.i2c1rst(),
                self.i2c2rst(),
                self.i2c3rst(),
                self.i3c1rst(),
                self.uart7rst(),
                self.uart8rst()
            )
        }
    }
    #[doc = "RCC APB1 peripheral reset register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1rstr2(pub u32);
    impl Apb1rstr2 {
        #[doc = "clock recovery system reset."]
        #[must_use]
        #[inline(always)]
        pub const fn crsrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "clock recovery system reset."]
        #[inline(always)]
        pub const fn set_crsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "FDCAN block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn fdcanrst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN block reset."]
        #[inline(always)]
        pub const fn set_fdcanrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "UCPD block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn ucpd1rst(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "UCPD block reset."]
        #[inline(always)]
        pub const fn set_ucpd1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
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
                .field("crsrst", &self.crsrst())
                .field("fdcanrst", &self.fdcanrst())
                .field("ucpd1rst", &self.ucpd1rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1rstr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb1rstr2 {{ crsrst: {=bool:?}, fdcanrst: {=bool:?}, ucpd1rst: {=bool:?} }}",
                self.crsrst(),
                self.fdcanrst(),
                self.ucpd1rst()
            )
        }
    }
    #[doc = "RCC APB2 clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2enr(pub u32);
    impl Apb2enr {
        #[doc = "TIM1 peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 peripheral clock enable."]
        #[inline(always)]
        pub const fn set_tim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM8 peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim8en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 peripheral clock enable."]
        #[inline(always)]
        pub const fn set_tim8en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM20 peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim20en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM20 peripheral clock enable."]
        #[inline(always)]
        pub const fn set_tim20en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "USART1 peripheral clocks enable."]
        #[must_use]
        #[inline(always)]
        pub const fn usart1en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 peripheral clocks enable."]
        #[inline(always)]
        pub const fn set_usart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "USART6 peripheral clocks enable."]
        #[must_use]
        #[inline(always)]
        pub const fn usart6en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USART6 peripheral clocks enable."]
        #[inline(always)]
        pub const fn set_usart6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "USART9 peripheral clocks enable."]
        #[must_use]
        #[inline(always)]
        pub const fn usart9en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "USART9 peripheral clocks enable."]
        #[inline(always)]
        pub const fn set_usart9en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "USART10 peripheral clocks enable."]
        #[must_use]
        #[inline(always)]
        pub const fn usart10en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "USART10 peripheral clocks enable."]
        #[inline(always)]
        pub const fn set_usart10en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "SPI2S1 Peripheral Clocks Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn spi1en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2S1 Peripheral Clocks Enable."]
        #[inline(always)]
        pub const fn set_spi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SPI4 Peripheral Clocks Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn spi4en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "SPI4 Peripheral Clocks Enable."]
        #[inline(always)]
        pub const fn set_spi4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "TIM15 peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim15en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 peripheral clock enable."]
        #[inline(always)]
        pub const fn set_tim15en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim16en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 peripheral clock enable."]
        #[inline(always)]
        pub const fn set_tim16en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim17en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 peripheral clock enable."]
        #[inline(always)]
        pub const fn set_tim17en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "TIM9 peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim9en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "TIM9 peripheral clock enable."]
        #[inline(always)]
        pub const fn set_tim9en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "SPI5 peripheral clocks enable."]
        #[must_use]
        #[inline(always)]
        pub const fn spi5en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SPI5 peripheral clocks enable."]
        #[inline(always)]
        pub const fn set_spi5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "HRTIM1 peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hrtim1en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM1 peripheral clock enable."]
        #[inline(always)]
        pub const fn set_hrtim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "HRTIM2 peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hrtim2en(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM2 peripheral clock enable."]
        #[inline(always)]
        pub const fn set_hrtim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
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
                .field("tim20en", &self.tim20en())
                .field("usart1en", &self.usart1en())
                .field("usart6en", &self.usart6en())
                .field("usart9en", &self.usart9en())
                .field("usart10en", &self.usart10en())
                .field("spi1en", &self.spi1en())
                .field("spi4en", &self.spi4en())
                .field("tim15en", &self.tim15en())
                .field("tim16en", &self.tim16en())
                .field("tim17en", &self.tim17en())
                .field("tim9en", &self.tim9en())
                .field("spi5en", &self.spi5en())
                .field("hrtim1en", &self.hrtim1en())
                .field("hrtim2en", &self.hrtim2en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2enr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2enr {{ tim1en: {=bool:?}, tim8en: {=bool:?}, tim20en: {=bool:?}, usart1en: {=bool:?}, usart6en: {=bool:?}, usart9en: {=bool:?}, usart10en: {=bool:?}, spi1en: {=bool:?}, spi4en: {=bool:?}, tim15en: {=bool:?}, tim16en: {=bool:?}, tim17en: {=bool:?}, tim9en: {=bool:?}, spi5en: {=bool:?}, hrtim1en: {=bool:?}, hrtim2en: {=bool:?} }}",
                self.tim1en(),
                self.tim8en(),
                self.tim20en(),
                self.usart1en(),
                self.usart6en(),
                self.usart9en(),
                self.usart10en(),
                self.spi1en(),
                self.spi4en(),
                self.tim15en(),
                self.tim16en(),
                self.tim17en(),
                self.tim9en(),
                self.spi5en(),
                self.hrtim1en(),
                self.hrtim2en()
            )
        }
    }
    #[doc = "RCC APB2 low-power clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2lpenr(pub u32);
    impl Apb2lpenr {
        #[doc = "TIM1 peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim1lpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM8 peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim8lpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim8lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM20 peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim20lpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM20 peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim20lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "USART1 peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn usart1lpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_usart1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "USART6 peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn usart6lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USART6 peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_usart6lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "USART9 peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn usart9lpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "USART9 peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_usart9lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "USART10 peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn usart10lpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "USART10 peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_usart10lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "SPI2S1 peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn spi1lpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2S1 peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_spi1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SPI4 peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn spi4lpen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "SPI4 peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_spi4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "TIM15 peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim15lpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim15lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim16lpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim16lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim17lpen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim17lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "TIM9 peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim9lpen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "TIM9 peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim9lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "SPI5 peripheral clocks enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn spi5lpen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SPI5 peripheral clocks enable in low-power mode."]
        #[inline(always)]
        pub const fn set_spi5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "HRTIM1 peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn hrtim1lpen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM1 peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_hrtim1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "HRTIM2 peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn hrtim2lpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM2 peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_hrtim2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
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
                .field("tim20lpen", &self.tim20lpen())
                .field("usart1lpen", &self.usart1lpen())
                .field("usart6lpen", &self.usart6lpen())
                .field("usart9lpen", &self.usart9lpen())
                .field("usart10lpen", &self.usart10lpen())
                .field("spi1lpen", &self.spi1lpen())
                .field("spi4lpen", &self.spi4lpen())
                .field("tim15lpen", &self.tim15lpen())
                .field("tim16lpen", &self.tim16lpen())
                .field("tim17lpen", &self.tim17lpen())
                .field("tim9lpen", &self.tim9lpen())
                .field("spi5lpen", &self.spi5lpen())
                .field("hrtim1lpen", &self.hrtim1lpen())
                .field("hrtim2lpen", &self.hrtim2lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2lpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2lpenr {{ tim1lpen: {=bool:?}, tim8lpen: {=bool:?}, tim20lpen: {=bool:?}, usart1lpen: {=bool:?}, usart6lpen: {=bool:?}, usart9lpen: {=bool:?}, usart10lpen: {=bool:?}, spi1lpen: {=bool:?}, spi4lpen: {=bool:?}, tim15lpen: {=bool:?}, tim16lpen: {=bool:?}, tim17lpen: {=bool:?}, tim9lpen: {=bool:?}, spi5lpen: {=bool:?}, hrtim1lpen: {=bool:?}, hrtim2lpen: {=bool:?} }}",
                self.tim1lpen(),
                self.tim8lpen(),
                self.tim20lpen(),
                self.usart1lpen(),
                self.usart6lpen(),
                self.usart9lpen(),
                self.usart10lpen(),
                self.spi1lpen(),
                self.spi4lpen(),
                self.tim15lpen(),
                self.tim16lpen(),
                self.tim17lpen(),
                self.tim9lpen(),
                self.spi5lpen(),
                self.hrtim1lpen(),
                self.hrtim2lpen()
            )
        }
    }
    #[doc = "RCC APB2 peripheral reset register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2rstr(pub u32);
    impl Apb2rstr {
        #[doc = "TIM1 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tim1rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 block reset."]
        #[inline(always)]
        pub const fn set_tim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM8 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tim8rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 block reset."]
        #[inline(always)]
        pub const fn set_tim8rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM20 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tim20rst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM20 block reset."]
        #[inline(always)]
        pub const fn set_tim20rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "USART1 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn usart1rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 block reset."]
        #[inline(always)]
        pub const fn set_usart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "USART6 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn usart6rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USART6 block reset."]
        #[inline(always)]
        pub const fn set_usart6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "USART9 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn usart9rst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "USART9 block reset."]
        #[inline(always)]
        pub const fn set_usart9rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "USART10 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn usart10rst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "USART10 block reset."]
        #[inline(always)]
        pub const fn set_usart10rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "SPI2S1 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn spi1rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2S1 block reset."]
        #[inline(always)]
        pub const fn set_spi1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SPI4 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn spi4rst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "SPI4 block reset."]
        #[inline(always)]
        pub const fn set_spi4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "TIM15 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tim15rst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 block reset."]
        #[inline(always)]
        pub const fn set_tim15rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tim16rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 block reset."]
        #[inline(always)]
        pub const fn set_tim16rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tim17rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 block reset."]
        #[inline(always)]
        pub const fn set_tim17rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "TIM9 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tim9rst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "TIM9 block reset."]
        #[inline(always)]
        pub const fn set_tim9rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "SPI5 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn spi5rst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SPI5 block reset."]
        #[inline(always)]
        pub const fn set_spi5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "HRTIM1 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn hrtim1rst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM1 block reset."]
        #[inline(always)]
        pub const fn set_hrtim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "HRTIM2 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn hrtim2rst(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM2 block reset."]
        #[inline(always)]
        pub const fn set_hrtim2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
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
                .field("tim20rst", &self.tim20rst())
                .field("usart1rst", &self.usart1rst())
                .field("usart6rst", &self.usart6rst())
                .field("usart9rst", &self.usart9rst())
                .field("usart10rst", &self.usart10rst())
                .field("spi1rst", &self.spi1rst())
                .field("spi4rst", &self.spi4rst())
                .field("tim15rst", &self.tim15rst())
                .field("tim16rst", &self.tim16rst())
                .field("tim17rst", &self.tim17rst())
                .field("tim9rst", &self.tim9rst())
                .field("spi5rst", &self.spi5rst())
                .field("hrtim1rst", &self.hrtim1rst())
                .field("hrtim2rst", &self.hrtim2rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2rstr {{ tim1rst: {=bool:?}, tim8rst: {=bool:?}, tim20rst: {=bool:?}, usart1rst: {=bool:?}, usart6rst: {=bool:?}, usart9rst: {=bool:?}, usart10rst: {=bool:?}, spi1rst: {=bool:?}, spi4rst: {=bool:?}, tim15rst: {=bool:?}, tim16rst: {=bool:?}, tim17rst: {=bool:?}, tim9rst: {=bool:?}, spi5rst: {=bool:?}, hrtim1rst: {=bool:?}, hrtim2rst: {=bool:?} }}",
                self.tim1rst(),
                self.tim8rst(),
                self.tim20rst(),
                self.usart1rst(),
                self.usart6rst(),
                self.usart9rst(),
                self.usart10rst(),
                self.spi1rst(),
                self.spi4rst(),
                self.tim15rst(),
                self.tim16rst(),
                self.tim17rst(),
                self.tim9rst(),
                self.spi5rst(),
                self.hrtim1rst(),
                self.hrtim2rst()
            )
        }
    }
    #[doc = "RCC APB4 clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb4enr(pub u32);
    impl Apb4enr {
        #[doc = "SBS peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sbsen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SBS peripheral clock enable."]
        #[inline(always)]
        pub const fn set_sbsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPUART1 peripheral clocks enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lpuart1en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 peripheral clocks enable."]
        #[inline(always)]
        pub const fn set_lpuart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SPI/I2S6 peripheral clocks enable."]
        #[must_use]
        #[inline(always)]
        pub const fn spi6en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI/I2S6 peripheral clocks enable."]
        #[inline(always)]
        pub const fn set_spi6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LPTIM2 peripheral clocks enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim2en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 peripheral clocks enable."]
        #[inline(always)]
        pub const fn set_lptim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LPTIM3 peripheral clocks enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim3en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3 peripheral clocks enable."]
        #[inline(always)]
        pub const fn set_lptim3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LPTIM4 peripheral clocks enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim4en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM4 peripheral clocks enable."]
        #[inline(always)]
        pub const fn set_lptim4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIM5 peripheral clocks enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim5en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM5 peripheral clocks enable."]
        #[inline(always)]
        pub const fn set_lptim5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "VREF peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn vrefen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "VREF peripheral clock enable."]
        #[inline(always)]
        pub const fn set_vrefen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "RTC APB clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rtcapben(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "RTC APB clock enable."]
        #[inline(always)]
        pub const fn set_rtcapben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Temperature Sensor peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dtsen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Temperature Sensor peripheral clock enable."]
        #[inline(always)]
        pub const fn set_dtsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "COMP12 peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn comp12en(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "COMP12 peripheral clock enable."]
        #[inline(always)]
        pub const fn set_comp12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "COMP34 peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn comp34en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "COMP34 peripheral clock enable."]
        #[inline(always)]
        pub const fn set_comp34en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "COMP56 peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn comp56en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "COMP56 peripheral clock enable."]
        #[inline(always)]
        pub const fn set_comp56en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "COMP78 peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn comp78en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "COMP78 peripheral clock enable."]
        #[inline(always)]
        pub const fn set_comp78en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("sbsen", &self.sbsen())
                .field("lpuart1en", &self.lpuart1en())
                .field("spi6en", &self.spi6en())
                .field("lptim2en", &self.lptim2en())
                .field("lptim3en", &self.lptim3en())
                .field("lptim4en", &self.lptim4en())
                .field("lptim5en", &self.lptim5en())
                .field("vrefen", &self.vrefen())
                .field("rtcapben", &self.rtcapben())
                .field("dtsen", &self.dtsen())
                .field("comp12en", &self.comp12en())
                .field("comp34en", &self.comp34en())
                .field("comp56en", &self.comp56en())
                .field("comp78en", &self.comp78en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb4enr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb4enr {{ sbsen: {=bool:?}, lpuart1en: {=bool:?}, spi6en: {=bool:?}, lptim2en: {=bool:?}, lptim3en: {=bool:?}, lptim4en: {=bool:?}, lptim5en: {=bool:?}, vrefen: {=bool:?}, rtcapben: {=bool:?}, dtsen: {=bool:?}, comp12en: {=bool:?}, comp34en: {=bool:?}, comp56en: {=bool:?}, comp78en: {=bool:?} }}",
                self.sbsen(),
                self.lpuart1en(),
                self.spi6en(),
                self.lptim2en(),
                self.lptim3en(),
                self.lptim4en(),
                self.lptim5en(),
                self.vrefen(),
                self.rtcapben(),
                self.dtsen(),
                self.comp12en(),
                self.comp34en(),
                self.comp56en(),
                self.comp78en()
            )
        }
    }
    #[doc = "RCC APB4 low-power clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb4lpenr(pub u32);
    impl Apb4lpenr {
        #[doc = "SBS peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn sbslpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SBS peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_sbslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPUART1 peripheral clocks enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn lpuart1lpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 peripheral clocks enable in low-power mode."]
        #[inline(always)]
        pub const fn set_lpuart1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SPI/I2S6 peripheral clocks enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn spi6lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI/I2S6 peripheral clocks enable in low-power mode."]
        #[inline(always)]
        pub const fn set_spi6lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LPTIM2 peripheral clocks enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim2lpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 peripheral clocks enable in low-power mode."]
        #[inline(always)]
        pub const fn set_lptim2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LPTIM3 peripheral clocks enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim3lpen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3 peripheral clocks enable in low-power mode."]
        #[inline(always)]
        pub const fn set_lptim3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LPTIM4 peripheral clocks enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim4lpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM4 peripheral clocks enable in low-power mode."]
        #[inline(always)]
        pub const fn set_lptim4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIM5 peripheral clocks enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim5lpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM5 peripheral clocks enable in low-power mode."]
        #[inline(always)]
        pub const fn set_lptim5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "VREF peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn vreflpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "VREF peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_vreflpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "RTC APB clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn rtcapblpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "RTC APB clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_rtcapblpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "temperature sensor peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dtslpen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "temperature sensor peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_dtslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "temperature sensor peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn comp12lpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "temperature sensor peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_comp12lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "temperature sensor peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn comp34lpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "temperature sensor peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_comp34lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "temperature sensor peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn comp56lpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "temperature sensor peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_comp56lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "temperature sensor peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn comp78lpen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "temperature sensor peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_comp78lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("sbslpen", &self.sbslpen())
                .field("lpuart1lpen", &self.lpuart1lpen())
                .field("spi6lpen", &self.spi6lpen())
                .field("lptim2lpen", &self.lptim2lpen())
                .field("lptim3lpen", &self.lptim3lpen())
                .field("lptim4lpen", &self.lptim4lpen())
                .field("lptim5lpen", &self.lptim5lpen())
                .field("vreflpen", &self.vreflpen())
                .field("rtcapblpen", &self.rtcapblpen())
                .field("dtslpen", &self.dtslpen())
                .field("comp12lpen", &self.comp12lpen())
                .field("comp34lpen", &self.comp34lpen())
                .field("comp56lpen", &self.comp56lpen())
                .field("comp78lpen", &self.comp78lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb4lpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb4lpenr {{ sbslpen: {=bool:?}, lpuart1lpen: {=bool:?}, spi6lpen: {=bool:?}, lptim2lpen: {=bool:?}, lptim3lpen: {=bool:?}, lptim4lpen: {=bool:?}, lptim5lpen: {=bool:?}, vreflpen: {=bool:?}, rtcapblpen: {=bool:?}, dtslpen: {=bool:?}, comp12lpen: {=bool:?}, comp34lpen: {=bool:?}, comp56lpen: {=bool:?}, comp78lpen: {=bool:?} }}",
                self.sbslpen(),
                self.lpuart1lpen(),
                self.spi6lpen(),
                self.lptim2lpen(),
                self.lptim3lpen(),
                self.lptim4lpen(),
                self.lptim5lpen(),
                self.vreflpen(),
                self.rtcapblpen(),
                self.dtslpen(),
                self.comp12lpen(),
                self.comp34lpen(),
                self.comp56lpen(),
                self.comp78lpen()
            )
        }
    }
    #[doc = "RCC APB4 peripheral reset register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb4rstr(pub u32);
    impl Apb4rstr {
        #[doc = "SBS block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn sbsrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SBS block reset."]
        #[inline(always)]
        pub const fn set_sbsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPUART1 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn lpuart1rst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 block reset."]
        #[inline(always)]
        pub const fn set_lpuart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SPI/I2S6 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn spi6rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI/I2S6 block reset."]
        #[inline(always)]
        pub const fn set_spi6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LPTIM2 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim2rst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 block reset."]
        #[inline(always)]
        pub const fn set_lptim2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LPTIM3 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim3rst(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM3 block reset."]
        #[inline(always)]
        pub const fn set_lptim3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LPTIM4 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim4rst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM4 block reset."]
        #[inline(always)]
        pub const fn set_lptim4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIM5 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim5rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM5 block reset."]
        #[inline(always)]
        pub const fn set_lptim5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "VREF block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn vrefrst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "VREF block reset."]
        #[inline(always)]
        pub const fn set_vrefrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "DTS block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn dtsrst(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "DTS block reset."]
        #[inline(always)]
        pub const fn set_dtsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "COMP512 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn comp12rst(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "COMP512 block reset."]
        #[inline(always)]
        pub const fn set_comp12rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "COMP34 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn comp34rst(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "COMP34 block reset."]
        #[inline(always)]
        pub const fn set_comp34rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "COMP56 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn comp56rst(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "COMP56 block reset."]
        #[inline(always)]
        pub const fn set_comp56rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "COMP78 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn comp78rst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "COMP78 block reset."]
        #[inline(always)]
        pub const fn set_comp78rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("sbsrst", &self.sbsrst())
                .field("lpuart1rst", &self.lpuart1rst())
                .field("spi6rst", &self.spi6rst())
                .field("lptim2rst", &self.lptim2rst())
                .field("lptim3rst", &self.lptim3rst())
                .field("lptim4rst", &self.lptim4rst())
                .field("lptim5rst", &self.lptim5rst())
                .field("vrefrst", &self.vrefrst())
                .field("dtsrst", &self.dtsrst())
                .field("comp12rst", &self.comp12rst())
                .field("comp34rst", &self.comp34rst())
                .field("comp56rst", &self.comp56rst())
                .field("comp78rst", &self.comp78rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb4rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb4rstr {{ sbsrst: {=bool:?}, lpuart1rst: {=bool:?}, spi6rst: {=bool:?}, lptim2rst: {=bool:?}, lptim3rst: {=bool:?}, lptim4rst: {=bool:?}, lptim5rst: {=bool:?}, vrefrst: {=bool:?}, dtsrst: {=bool:?}, comp12rst: {=bool:?}, comp34rst: {=bool:?}, comp56rst: {=bool:?}, comp78rst: {=bool:?} }}",
                self.sbsrst(),
                self.lpuart1rst(),
                self.spi6rst(),
                self.lptim2rst(),
                self.lptim3rst(),
                self.lptim4rst(),
                self.lptim5rst(),
                self.vrefrst(),
                self.dtsrst(),
                self.comp12rst(),
                self.comp34rst(),
                self.comp56rst(),
                self.comp78rst()
            )
        }
    }
    #[doc = "RCC APB clocks configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apbcfgr(pub u32);
    impl Apbcfgr {
        #[doc = "CPU domain APB1 prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn ppre1(&self) -> super::vals::Ppre1 {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Ppre1::from_bits(val as u8)
        }
        #[doc = "CPU domain APB1 prescaler."]
        #[inline(always)]
        pub const fn set_ppre1(&mut self, val: super::vals::Ppre1) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "CPU domain APB2 prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn ppre2(&self) -> super::vals::Ppre2 {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Ppre2::from_bits(val as u8)
        }
        #[doc = "CPU domain APB2 prescaler."]
        #[inline(always)]
        pub const fn set_ppre2(&mut self, val: super::vals::Ppre2) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "CPU domain APB4 prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn ppre4(&self) -> super::vals::Ppre4 {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Ppre4::from_bits(val as u8)
        }
        #[doc = "CPU domain APB4 prescaler."]
        #[inline(always)]
        pub const fn set_ppre4(&mut self, val: super::vals::Ppre4) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
    }
    impl Default for Apbcfgr {
        #[inline(always)]
        fn default() -> Apbcfgr {
            Apbcfgr(0)
        }
    }
    impl core::fmt::Debug for Apbcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apbcfgr")
                .field("ppre1", &self.ppre1())
                .field("ppre2", &self.ppre2())
                .field("ppre4", &self.ppre4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apbcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apbcfgr {{ ppre1: {:?}, ppre2: {:?}, ppre4: {:?} }}",
                self.ppre1(),
                self.ppre2(),
                self.ppre4()
            )
        }
    }
    #[doc = "RCC Backup domain control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdcr(pub u32);
    impl Bdcr {
        #[doc = "LSE oscillator enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn lseon(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSE oscillator enabled."]
        #[inline(always)]
        pub const fn set_lseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE oscillator ready."]
        #[must_use]
        #[inline(always)]
        pub const fn lserdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE oscillator ready."]
        #[inline(always)]
        pub const fn set_lserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LSE oscillator bypass."]
        #[must_use]
        #[inline(always)]
        pub const fn lsebyp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "LSE oscillator bypass."]
        #[inline(always)]
        pub const fn set_lsebyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "LSE oscillator driving capability."]
        #[must_use]
        #[inline(always)]
        pub const fn lsedrv(&self) -> super::vals::Lsedrv {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Lsedrv::from_bits(val as u8)
        }
        #[doc = "LSE oscillator driving capability."]
        #[inline(always)]
        pub const fn set_lsedrv(&mut self, val: super::vals::Lsedrv) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "LSE clock security system enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lsecsson(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock security system enable."]
        #[inline(always)]
        pub const fn set_lsecsson(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LSE clock security system failure detection."]
        #[must_use]
        #[inline(always)]
        pub const fn lsecssd(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock security system failure detection."]
        #[inline(always)]
        pub const fn set_lsecssd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "low-speed external clock type in Bypass mode."]
        #[must_use]
        #[inline(always)]
        pub const fn lseext(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "low-speed external clock type in Bypass mode."]
        #[inline(always)]
        pub const fn set_lseext(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "RTC clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn rtcsel(&self) -> super::vals::Rtcsel {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Rtcsel::from_bits(val as u8)
        }
        #[doc = "RTC clock source selection."]
        #[inline(always)]
        pub const fn set_rtcsel(&mut self, val: super::vals::Rtcsel) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "Re-Arm the LSECSS function."]
        #[must_use]
        #[inline(always)]
        pub const fn lsecssra(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Re-Arm the LSECSS function."]
        #[inline(always)]
        pub const fn set_lsecssra(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "RTC clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rtcen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "RTC clock enable."]
        #[inline(always)]
        pub const fn set_rtcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "VSwitch domain software reset."]
        #[must_use]
        #[inline(always)]
        pub const fn vswrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "VSwitch domain software reset."]
        #[inline(always)]
        pub const fn set_vswrst(&mut self, val: bool) {
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
                .field("lseext", &self.lseext())
                .field("rtcsel", &self.rtcsel())
                .field("lsecssra", &self.lsecssra())
                .field("rtcen", &self.rtcen())
                .field("vswrst", &self.vswrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bdcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bdcr {{ lseon: {=bool:?}, lserdy: {=bool:?}, lsebyp: {=bool:?}, lsedrv: {:?}, lsecsson: {=bool:?}, lsecssd: {=bool:?}, lseext: {=bool:?}, rtcsel: {:?}, lsecssra: {=bool:?}, rtcen: {=bool:?}, vswrst: {=bool:?} }}",
                self.lseon(),
                self.lserdy(),
                self.lsebyp(),
                self.lsedrv(),
                self.lsecsson(),
                self.lsecssd(),
                self.lseext(),
                self.rtcsel(),
                self.lsecssra(),
                self.rtcen(),
                self.vswrst()
            )
        }
    }
    #[doc = "RCC AHB clock configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bmcfgr(pub u32);
    impl Bmcfgr {
        #[doc = "Bus matrix clock prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn bmpre(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Bus matrix clock prescaler."]
        #[inline(always)]
        pub const fn set_bmpre(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Bmcfgr {
        #[inline(always)]
        fn default() -> Bmcfgr {
            Bmcfgr(0)
        }
    }
    impl core::fmt::Debug for Bmcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bmcfgr").field("bmpre", &self.bmpre()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bmcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bmcfgr {{ bmpre: {=u8:?} }}", self.bmpre())
        }
    }
    #[doc = "RCC peripheral kernel clock selection register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccipr1(pub u32);
    impl Ccipr1 {
        #[doc = "FMC kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn fmcsel(&self) -> super::vals::Fmcsel {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Fmcsel::from_bits(val as u8)
        }
        #[doc = "FMC kernel clock source selection."]
        #[inline(always)]
        pub const fn set_fmcsel(&mut self, val: super::vals::Fmcsel) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "SDMMC1 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn sdmmc1sel(&self) -> super::vals::Sdmmc1sel {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Sdmmc1sel::from_bits(val as u8)
        }
        #[doc = "SDMMC1 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_sdmmc1sel(&mut self, val: super::vals::Sdmmc1sel) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "XSPI1 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn xspi1sel(&self) -> super::vals::Xspi1sel {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Xspi1sel::from_bits(val as u8)
        }
        #[doc = "XSPI1 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_xspi1sel(&mut self, val: super::vals::Xspi1sel) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "OTGFS kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn otgfssel(&self) -> super::vals::Otgfssel {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::Otgfssel::from_bits(val as u8)
        }
        #[doc = "OTGFS kernel clock source selection."]
        #[inline(always)]
        pub const fn set_otgfssel(&mut self, val: super::vals::Otgfssel) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
        #[doc = "ADC kernel clock division factor."]
        #[must_use]
        #[inline(always)]
        pub const fn adcdiv(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "ADC kernel clock division factor."]
        #[inline(always)]
        pub const fn set_adcdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "ADF kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn adf1sel(&self) -> super::vals::Adf1sel {
            let val = (self.0 >> 20usize) & 0x07;
            super::vals::Adf1sel::from_bits(val as u8)
        }
        #[doc = "ADF kernel clock source selection."]
        #[inline(always)]
        pub const fn set_adf1sel(&mut self, val: super::vals::Adf1sel) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
        }
        #[doc = "SAR ADC kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn adcsel(&self) -> super::vals::Adcsel {
            let val = (self.0 >> 24usize) & 0x07;
            super::vals::Adcsel::from_bits(val as u8)
        }
        #[doc = "SAR ADC kernel clock source selection."]
        #[inline(always)]
        pub const fn set_adcsel(&mut self, val: super::vals::Adcsel) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
        }
        #[doc = "per_ck clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn ckpersel(&self) -> super::vals::Ckpersel {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Ckpersel::from_bits(val as u8)
        }
        #[doc = "per_ck clock source selection."]
        #[inline(always)]
        pub const fn set_ckpersel(&mut self, val: super::vals::Ckpersel) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
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
                .field("fmcsel", &self.fmcsel())
                .field("sdmmc1sel", &self.sdmmc1sel())
                .field("xspi1sel", &self.xspi1sel())
                .field("otgfssel", &self.otgfssel())
                .field("adcdiv", &self.adcdiv())
                .field("adf1sel", &self.adf1sel())
                .field("adcsel", &self.adcsel())
                .field("ckpersel", &self.ckpersel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccipr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccipr1 {{ fmcsel: {:?}, sdmmc1sel: {:?}, xspi1sel: {:?}, otgfssel: {:?}, adcdiv: {=u8:?}, adf1sel: {:?}, adcsel: {:?}, ckpersel: {:?} }}",
                self.fmcsel(),
                self.sdmmc1sel(),
                self.xspi1sel(),
                self.otgfssel(),
                self.adcdiv(),
                self.adf1sel(),
                self.adcsel(),
                self.ckpersel()
            )
        }
    }
    #[doc = "RCC peripheral kernel clock selection register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccipr2(pub u32);
    impl Ccipr2 {
        #[doc = "USART2,3, UART4,5,7,8 (APB1) kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn uart234578sel(&self) -> super::vals::Uart234578sel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Uart234578sel::from_bits(val as u8)
        }
        #[doc = "USART2,3, UART4,5,7,8 (APB1) kernel clock source selection."]
        #[inline(always)]
        pub const fn set_uart234578sel(&mut self, val: super::vals::Uart234578sel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "SPI/I2S2 and SPI/I2S3 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn spi23sel(&self) -> super::vals::Spi23sel {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Spi23sel::from_bits(val as u8)
        }
        #[doc = "SPI/I2S2 and SPI/I2S3 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_spi23sel(&mut self, val: super::vals::Spi23sel) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "I2C1, I2C2, I2C3 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c123sel(&self) -> super::vals::I2c123sel {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::I2c123sel::from_bits(val as u8)
        }
        #[doc = "I2C1, I2C2, I2C3 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_i2c123sel(&mut self, val: super::vals::I2c123sel) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "I3C1 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn i3c1sel(&self) -> super::vals::I3c1sel {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::I3c1sel::from_bits(val as u8)
        }
        #[doc = "I3C1 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_i3c1sel(&mut self, val: super::vals::I3c1sel) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "LPTIM1 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim1sel(&self) -> super::vals::Lptim1sel {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::Lptim1sel::from_bits(val as u8)
        }
        #[doc = "LPTIM1 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_lptim1sel(&mut self, val: super::vals::Lptim1sel) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
        #[doc = "FDCAN kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn fdcansel(&self) -> super::vals::Fdcansel {
            let val = (self.0 >> 22usize) & 0x03;
            super::vals::Fdcansel::from_bits(val as u8)
        }
        #[doc = "FDCAN kernel clock source selection."]
        #[inline(always)]
        pub const fn set_fdcansel(&mut self, val: super::vals::Fdcansel) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
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
                .field("uart234578sel", &self.uart234578sel())
                .field("spi23sel", &self.spi23sel())
                .field("i2c123sel", &self.i2c123sel())
                .field("i3c1sel", &self.i3c1sel())
                .field("lptim1sel", &self.lptim1sel())
                .field("fdcansel", &self.fdcansel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccipr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccipr2 {{ uart234578sel: {:?}, spi23sel: {:?}, i2c123sel: {:?}, i3c1sel: {:?}, lptim1sel: {:?}, fdcansel: {:?} }}",
                self.uart234578sel(),
                self.spi23sel(),
                self.i2c123sel(),
                self.i3c1sel(),
                self.lptim1sel(),
                self.fdcansel()
            )
        }
    }
    #[doc = "RCC peripheral kernel clock selection register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccipr3(pub u32);
    impl Ccipr3 {
        #[doc = "USART1, UARTS6, USART9 and USART10 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn usart16910sel(&self) -> super::vals::Usart16910sel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Usart16910sel::from_bits(val as u8)
        }
        #[doc = "USART1, UARTS6, USART9 and USART10 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_usart16910sel(&mut self, val: super::vals::Usart16910sel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "SPI4 and 5 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn spi45sel(&self) -> super::vals::Spi45sel {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Spi45sel::from_bits(val as u8)
        }
        #[doc = "SPI4 and 5 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_spi45sel(&mut self, val: super::vals::Spi45sel) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "SPI/I2S1 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn spi1sel(&self) -> super::vals::Spi1sel {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Spi1sel::from_bits(val as u8)
        }
        #[doc = "SPI/I2S1 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_spi1sel(&mut self, val: super::vals::Spi1sel) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
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
                .field("usart16910sel", &self.usart16910sel())
                .field("spi45sel", &self.spi45sel())
                .field("spi1sel", &self.spi1sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccipr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccipr3 {{ usart16910sel: {:?}, spi45sel: {:?}, spi1sel: {:?} }}",
                self.usart16910sel(),
                self.spi45sel(),
                self.spi1sel()
            )
        }
    }
    #[doc = "RCC peripheral kernel clock selection register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccipr4(pub u32);
    impl Ccipr4 {
        #[doc = "LPUART1 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn lpuart1sel(&self) -> super::vals::Lpuart1sel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Lpuart1sel::from_bits(val as u8)
        }
        #[doc = "LPUART1 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_lpuart1sel(&mut self, val: super::vals::Lpuart1sel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "SPI/I2S6 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn spi6sel(&self) -> super::vals::Spi6sel {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Spi6sel::from_bits(val as u8)
        }
        #[doc = "SPI/I2S6 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_spi6sel(&mut self, val: super::vals::Spi6sel) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "LPTIM2 and LPTIM3 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim23sel(&self) -> super::vals::Lptim23sel {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Lptim23sel::from_bits(val as u8)
        }
        #[doc = "LPTIM2 and LPTIM3 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_lptim23sel(&mut self, val: super::vals::Lptim23sel) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "LPTIM4, and LPTIM5 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim45sel(&self) -> super::vals::Lptim45sel {
            let val = (self.0 >> 12usize) & 0x07;
            super::vals::Lptim45sel::from_bits(val as u8)
        }
        #[doc = "LPTIM4, and LPTIM5 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_lptim45sel(&mut self, val: super::vals::Lptim45sel) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
        }
        #[doc = "DAC1 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn dac1sel(&self) -> super::vals::Dac1sel {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::Dac1sel::from_bits(val as u8)
        }
        #[doc = "DAC1 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_dac1sel(&mut self, val: super::vals::Dac1sel) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
        #[doc = "DAC1 hold clock selection."]
        #[must_use]
        #[inline(always)]
        pub const fn dac1shsel(&self) -> super::vals::Dac1shsel {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Dac1shsel::from_bits(val as u8)
        }
        #[doc = "DAC1 hold clock selection."]
        #[inline(always)]
        pub const fn set_dac1shsel(&mut self, val: super::vals::Dac1shsel) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "DAC2 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn dac2sel(&self) -> super::vals::Dac2sel {
            let val = (self.0 >> 20usize) & 0x07;
            super::vals::Dac2sel::from_bits(val as u8)
        }
        #[doc = "DAC2 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_dac2sel(&mut self, val: super::vals::Dac2sel) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
        }
        #[doc = "DAC3 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn dac3sel(&self) -> super::vals::Dac3sel {
            let val = (self.0 >> 24usize) & 0x07;
            super::vals::Dac3sel::from_bits(val as u8)
        }
        #[doc = "DAC3 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_dac3sel(&mut self, val: super::vals::Dac3sel) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
        }
        #[doc = "DAC4 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn dac4sel(&self) -> super::vals::Dac4sel {
            let val = (self.0 >> 28usize) & 0x07;
            super::vals::Dac4sel::from_bits(val as u8)
        }
        #[doc = "DAC4 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_dac4sel(&mut self, val: super::vals::Dac4sel) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Ccipr4 {
        #[inline(always)]
        fn default() -> Ccipr4 {
            Ccipr4(0)
        }
    }
    impl core::fmt::Debug for Ccipr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccipr4")
                .field("lpuart1sel", &self.lpuart1sel())
                .field("spi6sel", &self.spi6sel())
                .field("lptim23sel", &self.lptim23sel())
                .field("lptim45sel", &self.lptim45sel())
                .field("dac1sel", &self.dac1sel())
                .field("dac1shsel", &self.dac1shsel())
                .field("dac2sel", &self.dac2sel())
                .field("dac3sel", &self.dac3sel())
                .field("dac4sel", &self.dac4sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccipr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccipr4 {{ lpuart1sel: {:?}, spi6sel: {:?}, lptim23sel: {:?}, lptim45sel: {:?}, dac1sel: {:?}, dac1shsel: {:?}, dac2sel: {:?}, dac3sel: {:?}, dac4sel: {:?} }}",
                self.lpuart1sel(),
                self.spi6sel(),
                self.lptim23sel(),
                self.lptim45sel(),
                self.dac1sel(),
                self.dac1shsel(),
                self.dac2sel(),
                self.dac3sel(),
                self.dac4sel()
            )
        }
    }
    #[doc = "RCC CPU domain clock configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cdcfgr(pub u32);
    impl Cdcfgr {
        #[doc = "CPU domain core prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn cpre(&self) -> super::vals::Cpre {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Cpre::from_bits(val as u8)
        }
        #[doc = "CPU domain core prescaler."]
        #[inline(always)]
        pub const fn set_cpre(&mut self, val: super::vals::Cpre) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Cdcfgr {
        #[inline(always)]
        fn default() -> Cdcfgr {
            Cdcfgr(0)
        }
    }
    impl core::fmt::Debug for Cdcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cdcfgr").field("cpre", &self.cpre()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cdcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cdcfgr {{ cpre: {:?} }}", self.cpre())
        }
    }
    #[doc = "RCC clock configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "system clock switch."]
        #[must_use]
        #[inline(always)]
        pub const fn sw(&self) -> super::vals::Sw {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "system clock switch."]
        #[inline(always)]
        pub const fn set_sw(&mut self, val: super::vals::Sw) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "system clock switch status."]
        #[must_use]
        #[inline(always)]
        pub const fn sws(&self) -> super::vals::Sws {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Sws::from_bits(val as u8)
        }
        #[doc = "system clock switch status."]
        #[inline(always)]
        pub const fn set_sws(&mut self, val: super::vals::Sws) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "system clock selection after a wake up from system Stop."]
        #[must_use]
        #[inline(always)]
        pub const fn stopwuck(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "system clock selection after a wake up from system Stop."]
        #[inline(always)]
        pub const fn set_stopwuck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "kernel clock selection after a wake up from system Stop."]
        #[must_use]
        #[inline(always)]
        pub const fn stopkerwuck(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "kernel clock selection after a wake up from system Stop."]
        #[inline(always)]
        pub const fn set_stopkerwuck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "HSE division factor for RTC clock."]
        #[must_use]
        #[inline(always)]
        pub const fn rtcpre(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "HSE division factor for RTC clock."]
        #[inline(always)]
        pub const fn set_rtcpre(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "MCO1 prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn mco1pre(&self) -> super::vals::Mco1pre {
            let val = (self.0 >> 18usize) & 0x0f;
            super::vals::Mco1pre::from_bits(val as u8)
        }
        #[doc = "MCO1 prescaler."]
        #[inline(always)]
        pub const fn set_mco1pre(&mut self, val: super::vals::Mco1pre) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val.to_bits() as u32) & 0x0f) << 18usize);
        }
        #[doc = "Microcontroller clock output 1."]
        #[must_use]
        #[inline(always)]
        pub const fn mco1sel(&self) -> super::vals::Mco1sel {
            let val = (self.0 >> 22usize) & 0x07;
            super::vals::Mco1sel::from_bits(val as u8)
        }
        #[doc = "Microcontroller clock output 1."]
        #[inline(always)]
        pub const fn set_mco1sel(&mut self, val: super::vals::Mco1sel) {
            self.0 = (self.0 & !(0x07 << 22usize)) | (((val.to_bits() as u32) & 0x07) << 22usize);
        }
        #[doc = "MCO2 prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn mco2pre(&self) -> super::vals::Mco2pre {
            let val = (self.0 >> 25usize) & 0x0f;
            super::vals::Mco2pre::from_bits(val as u8)
        }
        #[doc = "MCO2 prescaler."]
        #[inline(always)]
        pub const fn set_mco2pre(&mut self, val: super::vals::Mco2pre) {
            self.0 = (self.0 & !(0x0f << 25usize)) | (((val.to_bits() as u32) & 0x0f) << 25usize);
        }
        #[doc = "microcontroller clock output 2."]
        #[must_use]
        #[inline(always)]
        pub const fn mco2sel(&self) -> super::vals::Mco2sel {
            let val = (self.0 >> 29usize) & 0x07;
            super::vals::Mco2sel::from_bits(val as u8)
        }
        #[doc = "microcontroller clock output 2."]
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
            defmt::write!(
                f,
                "Cfgr {{ sw: {:?}, sws: {:?}, stopwuck: {=bool:?}, stopkerwuck: {=bool:?}, rtcpre: {=u8:?}, mco1pre: {:?}, mco1sel: {:?}, mco2pre: {:?}, mco2sel: {:?} }}",
                self.sw(),
                self.sws(),
                self.stopwuck(),
                self.stopkerwuck(),
                self.rtcpre(),
                self.mco1pre(),
                self.mco1sel(),
                self.mco2pre(),
                self.mco2sel()
            )
        }
    }
    #[doc = "RCC clock source interrupt clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cicr(pub u32);
    impl Cicr {
        #[doc = "LSI ready interrupt clear."]
        #[must_use]
        #[inline(always)]
        pub const fn lsirdyc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt clear."]
        #[inline(always)]
        pub const fn set_lsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt clear."]
        #[must_use]
        #[inline(always)]
        pub const fn lserdyc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt clear."]
        #[inline(always)]
        pub const fn set_lserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI ready interrupt clear."]
        #[must_use]
        #[inline(always)]
        pub const fn hsirdyc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt clear."]
        #[inline(always)]
        pub const fn set_hsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSE ready interrupt clear."]
        #[must_use]
        #[inline(always)]
        pub const fn hserdyc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt clear."]
        #[inline(always)]
        pub const fn set_hserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CSI ready interrupt clear."]
        #[must_use]
        #[inline(always)]
        pub const fn csirdyc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CSI ready interrupt clear."]
        #[inline(always)]
        pub const fn set_csirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "HSI48 ready interrupt clear."]
        #[must_use]
        #[inline(always)]
        pub const fn hsi48rdyc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 ready interrupt clear."]
        #[inline(always)]
        pub const fn set_hsi48rdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PLL1 ready interrupt clear."]
        #[must_use]
        #[inline(always)]
        pub const fn pll1rdyc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PLL1 ready interrupt clear."]
        #[inline(always)]
        pub const fn set_pll1rdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "PLL2 ready interrupt clear."]
        #[must_use]
        #[inline(always)]
        pub const fn pll2rdyc(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "PLL2 ready interrupt clear."]
        #[inline(always)]
        pub const fn set_pll2rdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "PLL3 ready interrupt clear."]
        #[must_use]
        #[inline(always)]
        pub const fn pll3rdyc(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "PLL3 ready interrupt clear."]
        #[inline(always)]
        pub const fn set_pll3rdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LSE clock security system interrupt clear."]
        #[must_use]
        #[inline(always)]
        pub const fn lsecssc(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock security system interrupt clear."]
        #[inline(always)]
        pub const fn set_lsecssc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "HSE clock security system interrupt clear."]
        #[must_use]
        #[inline(always)]
        pub const fn hsecssc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock security system interrupt clear."]
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
                .field("csirdyc", &self.csirdyc())
                .field("hsi48rdyc", &self.hsi48rdyc())
                .field("pll1rdyc", &self.pll1rdyc())
                .field("pll2rdyc", &self.pll2rdyc())
                .field("pll3rdyc", &self.pll3rdyc())
                .field("lsecssc", &self.lsecssc())
                .field("hsecssc", &self.hsecssc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cicr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cicr {{ lsirdyc: {=bool:?}, lserdyc: {=bool:?}, hsirdyc: {=bool:?}, hserdyc: {=bool:?}, csirdyc: {=bool:?}, hsi48rdyc: {=bool:?}, pll1rdyc: {=bool:?}, pll2rdyc: {=bool:?}, pll3rdyc: {=bool:?}, lsecssc: {=bool:?}, hsecssc: {=bool:?} }}",
                self.lsirdyc(),
                self.lserdyc(),
                self.hsirdyc(),
                self.hserdyc(),
                self.csirdyc(),
                self.hsi48rdyc(),
                self.pll1rdyc(),
                self.pll2rdyc(),
                self.pll3rdyc(),
                self.lsecssc(),
                self.hsecssc()
            )
        }
    }
    #[doc = "RCC clock source interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cier(pub u32);
    impl Cier {
        #[doc = "LSI ready interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lsirdyie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt enable."]
        #[inline(always)]
        pub const fn set_lsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lserdyie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt enable."]
        #[inline(always)]
        pub const fn set_lserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI ready interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hsirdyie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt enable."]
        #[inline(always)]
        pub const fn set_hsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSE ready interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hserdyie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt enable."]
        #[inline(always)]
        pub const fn set_hserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CSI ready interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn csirdyie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CSI ready interrupt enable."]
        #[inline(always)]
        pub const fn set_csirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "HSI48 ready interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hsi48rdyie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 ready interrupt enable."]
        #[inline(always)]
        pub const fn set_hsi48rdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PLL1 ready interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pll1rdyie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PLL1 ready interrupt enable."]
        #[inline(always)]
        pub const fn set_pll1rdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "PLL2 ready interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pll2rdyie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "PLL2 ready interrupt enable."]
        #[inline(always)]
        pub const fn set_pll2rdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "PLL3 ready interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pll3rdyie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "PLL3 ready interrupt enable."]
        #[inline(always)]
        pub const fn set_pll3rdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LSE clock security system interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lsecssie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock security system interrupt enable."]
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
                .field("pll1rdyie", &self.pll1rdyie())
                .field("pll2rdyie", &self.pll2rdyie())
                .field("pll3rdyie", &self.pll3rdyie())
                .field("lsecssie", &self.lsecssie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cier {{ lsirdyie: {=bool:?}, lserdyie: {=bool:?}, hsirdyie: {=bool:?}, hserdyie: {=bool:?}, csirdyie: {=bool:?}, hsi48rdyie: {=bool:?}, pll1rdyie: {=bool:?}, pll2rdyie: {=bool:?}, pll3rdyie: {=bool:?}, lsecssie: {=bool:?} }}",
                self.lsirdyie(),
                self.lserdyie(),
                self.hsirdyie(),
                self.hserdyie(),
                self.csirdyie(),
                self.hsi48rdyie(),
                self.pll1rdyie(),
                self.pll2rdyie(),
                self.pll3rdyie(),
                self.lsecssie()
            )
        }
    }
    #[doc = "RCC clock source interrupt flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cifr(pub u32);
    impl Cifr {
        #[doc = "LSI ready interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn lsirdyf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt flag."]
        #[inline(always)]
        pub const fn set_lsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn lserdyf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt flag."]
        #[inline(always)]
        pub const fn set_lserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI ready interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hsirdyf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt flag."]
        #[inline(always)]
        pub const fn set_hsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSE ready interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hserdyf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt flag."]
        #[inline(always)]
        pub const fn set_hserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "CSI ready interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn csirdyf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "CSI ready interrupt flag."]
        #[inline(always)]
        pub const fn set_csirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "HSI48 ready interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hsi48rdyf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 ready interrupt flag."]
        #[inline(always)]
        pub const fn set_hsi48rdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PLL1 ready interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn pll1rdyf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PLL1 ready interrupt flag."]
        #[inline(always)]
        pub const fn set_pll1rdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "PLL2 ready interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn pll2rdyf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "PLL2 ready interrupt flag."]
        #[inline(always)]
        pub const fn set_pll2rdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "PLL3 ready interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn pll3rdyf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "PLL3 ready interrupt flag."]
        #[inline(always)]
        pub const fn set_pll3rdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LSE clock security system interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn lsecssf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock security system interrupt flag."]
        #[inline(always)]
        pub const fn set_lsecssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "HSE clock security system interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hsecssf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock security system interrupt flag."]
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
                .field("csirdyf", &self.csirdyf())
                .field("hsi48rdyf", &self.hsi48rdyf())
                .field("pll1rdyf", &self.pll1rdyf())
                .field("pll2rdyf", &self.pll2rdyf())
                .field("pll3rdyf", &self.pll3rdyf())
                .field("lsecssf", &self.lsecssf())
                .field("hsecssf", &self.hsecssf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cifr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cifr {{ lsirdyf: {=bool:?}, lserdyf: {=bool:?}, hsirdyf: {=bool:?}, hserdyf: {=bool:?}, csirdyf: {=bool:?}, hsi48rdyf: {=bool:?}, pll1rdyf: {=bool:?}, pll2rdyf: {=bool:?}, pll3rdyf: {=bool:?}, lsecssf: {=bool:?}, hsecssf: {=bool:?} }}",
                self.lsirdyf(),
                self.lserdyf(),
                self.hsirdyf(),
                self.hserdyf(),
                self.csirdyf(),
                self.hsi48rdyf(),
                self.pll1rdyf(),
                self.pll2rdyf(),
                self.pll3rdyf(),
                self.lsecssf(),
                self.hsecssf()
            )
        }
    }
    #[doc = "RCC AXI clocks gating disable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ckgdisr(pub u32);
    impl Ckgdisr {
        #[doc = "AXI interconnect matrix clock gating disable."]
        #[must_use]
        #[inline(always)]
        pub const fn axickg(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "AXI interconnect matrix clock gating disable."]
        #[inline(always)]
        pub const fn set_axickg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "AXI AHB matrix port clock gating disable."]
        #[must_use]
        #[inline(always)]
        pub const fn ahbmckg(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "AXI AHB matrix port clock gating disable."]
        #[inline(always)]
        pub const fn set_ahbmckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "AXI SDMMC1 port clock gating disable."]
        #[must_use]
        #[inline(always)]
        pub const fn sdmmc1ckg(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "AXI SDMMC1 port clock gating disable."]
        #[inline(always)]
        pub const fn set_sdmmc1ckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "AXI HPDMA1 port clock gating disable."]
        #[must_use]
        #[inline(always)]
        pub const fn hpdma1ckg(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "AXI HPDMA1 port clock gating disable."]
        #[inline(always)]
        pub const fn set_hpdma1ckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "AXI CPU port clock gating disable."]
        #[must_use]
        #[inline(always)]
        pub const fn cpuckg(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "AXI CPU port clock gating disable."]
        #[inline(always)]
        pub const fn set_cpuckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "AXI NPU port 0 clock gating disable."]
        #[must_use]
        #[inline(always)]
        pub const fn npu0ckg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "AXI NPU port 0 clock gating disable."]
        #[inline(always)]
        pub const fn set_npu0ckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "AXI NPU port 1 clock gating disable."]
        #[must_use]
        #[inline(always)]
        pub const fn npu1ckg(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "AXI NPU port 1 clock gating disable."]
        #[inline(always)]
        pub const fn set_npu1ckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "AHBS AXI manager port clock gating disable."]
        #[must_use]
        #[inline(always)]
        pub const fn ahbsckg(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "AHBS AXI manager port clock gating disable."]
        #[inline(always)]
        pub const fn set_ahbsckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "AHB interconnect AXI manager port clock gating disable."]
        #[must_use]
        #[inline(always)]
        pub const fn ahbicckg(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "AHB interconnect AXI manager port clock gating disable."]
        #[inline(always)]
        pub const fn set_ahbicckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "FMC AXI manager port clock gating disable."]
        #[must_use]
        #[inline(always)]
        pub const fn fmcckg(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "FMC AXI manager port clock gating disable."]
        #[inline(always)]
        pub const fn set_fmcckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "XSPI1 AXI manager port clock gating disable."]
        #[must_use]
        #[inline(always)]
        pub const fn xspi1ckg(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "XSPI1 AXI manager port clock gating disable."]
        #[inline(always)]
        pub const fn set_xspi1ckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "AXISRAM4 AXI manager port clock gating disable."]
        #[must_use]
        #[inline(always)]
        pub const fn axisram4ckg(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "AXISRAM4 AXI manager port clock gating disable."]
        #[inline(always)]
        pub const fn set_axisram4ckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "AXISRAM3 AXI manager port clock gating disable."]
        #[must_use]
        #[inline(always)]
        pub const fn axisram3ckg(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "AXISRAM3 AXI manager port clock gating disable."]
        #[inline(always)]
        pub const fn set_axisram3ckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "AXISRAM2 AXI manager port clock gating disable."]
        #[must_use]
        #[inline(always)]
        pub const fn axisram2ckg(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "AXISRAM2 AXI manager port clock gating disable."]
        #[inline(always)]
        pub const fn set_axisram2ckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "AXISRAM1 AXI manager port clock gating disable."]
        #[must_use]
        #[inline(always)]
        pub const fn axisram1ckg(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "AXISRAM1 AXI manager port clock gating disable."]
        #[inline(always)]
        pub const fn set_axisram1ckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Flash interface (FLIFT) AXI manager port clock gating disable."]
        #[must_use]
        #[inline(always)]
        pub const fn flashckg(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Flash interface (FLIFT) AXI manager port clock gating disable."]
        #[inline(always)]
        pub const fn set_flashckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "EXTI clock gating disable."]
        #[must_use]
        #[inline(always)]
        pub const fn extickg(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI clock gating disable."]
        #[inline(always)]
        pub const fn set_extickg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "JTAG automatic clock gating disabling."]
        #[must_use]
        #[inline(always)]
        pub const fn jtagckg(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "JTAG automatic clock gating disabling."]
        #[inline(always)]
        pub const fn set_jtagckg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ckgdisr {
        #[inline(always)]
        fn default() -> Ckgdisr {
            Ckgdisr(0)
        }
    }
    impl core::fmt::Debug for Ckgdisr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ckgdisr")
                .field("axickg", &self.axickg())
                .field("ahbmckg", &self.ahbmckg())
                .field("sdmmc1ckg", &self.sdmmc1ckg())
                .field("hpdma1ckg", &self.hpdma1ckg())
                .field("cpuckg", &self.cpuckg())
                .field("npu0ckg", &self.npu0ckg())
                .field("npu1ckg", &self.npu1ckg())
                .field("ahbsckg", &self.ahbsckg())
                .field("ahbicckg", &self.ahbicckg())
                .field("fmcckg", &self.fmcckg())
                .field("xspi1ckg", &self.xspi1ckg())
                .field("axisram4ckg", &self.axisram4ckg())
                .field("axisram3ckg", &self.axisram3ckg())
                .field("axisram2ckg", &self.axisram2ckg())
                .field("axisram1ckg", &self.axisram1ckg())
                .field("flashckg", &self.flashckg())
                .field("extickg", &self.extickg())
                .field("jtagckg", &self.jtagckg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ckgdisr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ckgdisr {{ axickg: {=bool:?}, ahbmckg: {=bool:?}, sdmmc1ckg: {=bool:?}, hpdma1ckg: {=bool:?}, cpuckg: {=bool:?}, npu0ckg: {=bool:?}, npu1ckg: {=bool:?}, ahbsckg: {=bool:?}, ahbicckg: {=bool:?}, fmcckg: {=bool:?}, xspi1ckg: {=bool:?}, axisram4ckg: {=bool:?}, axisram3ckg: {=bool:?}, axisram2ckg: {=bool:?}, axisram1ckg: {=bool:?}, flashckg: {=bool:?}, extickg: {=bool:?}, jtagckg: {=bool:?} }}",
                self.axickg(),
                self.ahbmckg(),
                self.sdmmc1ckg(),
                self.hpdma1ckg(),
                self.cpuckg(),
                self.npu0ckg(),
                self.npu1ckg(),
                self.ahbsckg(),
                self.ahbicckg(),
                self.fmcckg(),
                self.xspi1ckg(),
                self.axisram4ckg(),
                self.axisram3ckg(),
                self.axisram2ckg(),
                self.axisram1ckg(),
                self.flashckg(),
                self.extickg(),
                self.jtagckg()
            )
        }
    }
    #[doc = "RCC clock protection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ckprotr(pub u32);
    impl Ckprotr {
        #[doc = "XSPI clock protection."]
        #[must_use]
        #[inline(always)]
        pub const fn xspickp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "XSPI clock protection."]
        #[inline(always)]
        pub const fn set_xspickp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "FMC clock protection."]
        #[must_use]
        #[inline(always)]
        pub const fn fmcckp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "FMC clock protection."]
        #[inline(always)]
        pub const fn set_fmcckp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "XSPI kernel clock switch position."]
        #[must_use]
        #[inline(always)]
        pub const fn xspiswp(&self) -> super::vals::Xspiswp {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Xspiswp::from_bits(val as u8)
        }
        #[doc = "XSPI kernel clock switch position."]
        #[inline(always)]
        pub const fn set_xspiswp(&mut self, val: super::vals::Xspiswp) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "FMC kernel clock switch position."]
        #[must_use]
        #[inline(always)]
        pub const fn fmcswp(&self) -> super::vals::Fmcswp {
            let val = (self.0 >> 12usize) & 0x07;
            super::vals::Fmcswp::from_bits(val as u8)
        }
        #[doc = "FMC kernel clock switch position."]
        #[inline(always)]
        pub const fn set_fmcswp(&mut self, val: super::vals::Fmcswp) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
        }
    }
    impl Default for Ckprotr {
        #[inline(always)]
        fn default() -> Ckprotr {
            Ckprotr(0)
        }
    }
    impl core::fmt::Debug for Ckprotr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ckprotr")
                .field("xspickp", &self.xspickp())
                .field("fmcckp", &self.fmcckp())
                .field("xspiswp", &self.xspiswp())
                .field("fmcswp", &self.fmcswp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ckprotr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ckprotr {{ xspickp: {=bool:?}, fmcckp: {=bool:?}, xspiswp: {:?}, fmcswp: {:?} }}",
                self.xspickp(),
                self.fmcckp(),
                self.xspiswp(),
                self.fmcswp()
            )
        }
    }
    #[doc = "RCC source control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "HSI clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hsion(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "HSI clock enable."]
        #[inline(always)]
        pub const fn set_hsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "HSI clock enable in Stop mode."]
        #[must_use]
        #[inline(always)]
        pub const fn hsikeron(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "HSI clock enable in Stop mode."]
        #[inline(always)]
        pub const fn set_hsikeron(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI clock ready flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hsirdy(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSI clock ready flag."]
        #[inline(always)]
        pub const fn set_hsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSI clock divider."]
        #[must_use]
        #[inline(always)]
        pub const fn hsidiv(&self) -> super::vals::Hsidiv {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Hsidiv::from_bits(val as u8)
        }
        #[doc = "HSI clock divider."]
        #[inline(always)]
        pub const fn set_hsidiv(&mut self, val: super::vals::Hsidiv) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "HSI divider flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hsidivf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HSI divider flag."]
        #[inline(always)]
        pub const fn set_hsidivf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "CSI clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn csion(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "CSI clock enable."]
        #[inline(always)]
        pub const fn set_csion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "CSI clock ready flag."]
        #[must_use]
        #[inline(always)]
        pub const fn csirdy(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "CSI clock ready flag."]
        #[inline(always)]
        pub const fn set_csirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CSI clock enable in Stop mode."]
        #[must_use]
        #[inline(always)]
        pub const fn csikeron(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "CSI clock enable in Stop mode."]
        #[inline(always)]
        pub const fn set_csikeron(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "HSI48 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hsi48on(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 clock enable."]
        #[inline(always)]
        pub const fn set_hsi48on(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "HSI48 clock ready flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hsi48rdy(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 clock ready flag."]
        #[inline(always)]
        pub const fn set_hsi48rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "HSE clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hseon(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock enable."]
        #[inline(always)]
        pub const fn set_hseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "HSE clock ready flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hserdy(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock ready flag."]
        #[inline(always)]
        pub const fn set_hserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "HSE clock bypass."]
        #[must_use]
        #[inline(always)]
        pub const fn hsebyp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock bypass."]
        #[inline(always)]
        pub const fn set_hsebyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "external high speed clock type in Bypass mode."]
        #[must_use]
        #[inline(always)]
        pub const fn hseext(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "external high speed clock type in Bypass mode."]
        #[inline(always)]
        pub const fn set_hseext(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "HSE clock security system enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hsecsson(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock security system enable."]
        #[inline(always)]
        pub const fn set_hsecsson(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "PLL1 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pll1on(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "PLL1 enable."]
        #[inline(always)]
        pub const fn set_pll1on(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "PLL1 clock ready flag."]
        #[must_use]
        #[inline(always)]
        pub const fn pll1rdy(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "PLL1 clock ready flag."]
        #[inline(always)]
        pub const fn set_pll1rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "PLL2 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pll2on(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "PLL2 enable."]
        #[inline(always)]
        pub const fn set_pll2on(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "PLL2 clock ready flag."]
        #[must_use]
        #[inline(always)]
        pub const fn pll2rdy(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "PLL2 clock ready flag."]
        #[inline(always)]
        pub const fn set_pll2rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "PLL3 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pll3on(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "PLL3 enable."]
        #[inline(always)]
        pub const fn set_pll3on(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "PLL3 clock ready flag."]
        #[must_use]
        #[inline(always)]
        pub const fn pll3rdy(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "PLL3 clock ready flag."]
        #[inline(always)]
        pub const fn set_pll3rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
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
                .field("hseon", &self.hseon())
                .field("hserdy", &self.hserdy())
                .field("hsebyp", &self.hsebyp())
                .field("hseext", &self.hseext())
                .field("hsecsson", &self.hsecsson())
                .field("pll1on", &self.pll1on())
                .field("pll1rdy", &self.pll1rdy())
                .field("pll2on", &self.pll2on())
                .field("pll2rdy", &self.pll2rdy())
                .field("pll3on", &self.pll3on())
                .field("pll3rdy", &self.pll3rdy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ hsion: {=bool:?}, hsikeron: {=bool:?}, hsirdy: {=bool:?}, hsidiv: {:?}, hsidivf: {=bool:?}, csion: {=bool:?}, csirdy: {=bool:?}, csikeron: {=bool:?}, hsi48on: {=bool:?}, hsi48rdy: {=bool:?}, hseon: {=bool:?}, hserdy: {=bool:?}, hsebyp: {=bool:?}, hseext: {=bool:?}, hsecsson: {=bool:?}, pll1on: {=bool:?}, pll1rdy: {=bool:?}, pll2on: {=bool:?}, pll2rdy: {=bool:?}, pll3on: {=bool:?}, pll3rdy: {=bool:?} }}",
                self.hsion(),
                self.hsikeron(),
                self.hsirdy(),
                self.hsidiv(),
                self.hsidivf(),
                self.csion(),
                self.csirdy(),
                self.csikeron(),
                self.hsi48on(),
                self.hsi48rdy(),
                self.hseon(),
                self.hserdy(),
                self.hsebyp(),
                self.hseext(),
                self.hsecsson(),
                self.pll1on(),
                self.pll1rdy(),
                self.pll2on(),
                self.pll2rdy(),
                self.pll3on(),
                self.pll3rdy()
            )
        }
    }
    #[doc = "RCC clock recovery RC register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crrcr(pub u32);
    impl Crrcr {
        #[doc = "Internal RC 48 MHz clock calibration."]
        #[must_use]
        #[inline(always)]
        pub const fn hsi48cal(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Internal RC 48 MHz clock calibration."]
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
    #[doc = "RCC CSI calibration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csicfgr(pub u32);
    impl Csicfgr {
        #[doc = "CSI clock calibration."]
        #[must_use]
        #[inline(always)]
        pub const fn csical(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "CSI clock calibration."]
        #[inline(always)]
        pub const fn set_csical(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "CSI clock trimming."]
        #[must_use]
        #[inline(always)]
        pub const fn csitrim(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "CSI clock trimming."]
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
                "Csicfgr {{ csical: {=u8:?}, csitrim: {=u8:?} }}",
                self.csical(),
                self.csitrim()
            )
        }
    }
    #[doc = "RCC clock control and status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "LSI oscillator enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lsion(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI oscillator enable."]
        #[inline(always)]
        pub const fn set_lsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSI oscillator ready."]
        #[must_use]
        #[inline(always)]
        pub const fn lsirdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSI oscillator ready."]
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
    #[doc = "RCC HSI calibration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hsicfgr(pub u32);
    impl Hsicfgr {
        #[doc = "HSI clock calibration."]
        #[must_use]
        #[inline(always)]
        pub const fn hsical(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "HSI clock calibration."]
        #[inline(always)]
        pub const fn set_hsical(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "HSI clock trimming."]
        #[must_use]
        #[inline(always)]
        pub const fn hsitrim(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x7f;
            val as u8
        }
        #[doc = "HSI clock trimming."]
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
    #[doc = "RCC PLL1 dividers configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pll1divr1(pub u32);
    impl Pll1divr1 {
        #[doc = "multiplication factor for PLL1 VCO."]
        #[must_use]
        #[inline(always)]
        pub const fn divn(&self) -> super::vals::Pll1divr1Divn {
            let val = (self.0 >> 0usize) & 0x01ff;
            super::vals::Pll1divr1Divn::from_bits(val as u16)
        }
        #[doc = "multiplication factor for PLL1 VCO."]
        #[inline(always)]
        pub const fn set_divn(&mut self, val: super::vals::Pll1divr1Divn) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val.to_bits() as u32) & 0x01ff) << 0usize);
        }
        #[doc = "PLL1 DIVP division factor."]
        #[must_use]
        #[inline(always)]
        pub const fn divp(&self) -> super::vals::Pll1divr1Divp {
            let val = (self.0 >> 9usize) & 0x7f;
            super::vals::Pll1divr1Divp::from_bits(val as u8)
        }
        #[doc = "PLL1 DIVP division factor."]
        #[inline(always)]
        pub const fn set_divp(&mut self, val: super::vals::Pll1divr1Divp) {
            self.0 = (self.0 & !(0x7f << 9usize)) | (((val.to_bits() as u32) & 0x7f) << 9usize);
        }
        #[doc = "PLL1 DIVQ division factor."]
        #[must_use]
        #[inline(always)]
        pub const fn divq(&self) -> super::vals::Pll1divr1Divq {
            let val = (self.0 >> 16usize) & 0x7f;
            super::vals::Pll1divr1Divq::from_bits(val as u8)
        }
        #[doc = "PLL1 DIVQ division factor."]
        #[inline(always)]
        pub const fn set_divq(&mut self, val: super::vals::Pll1divr1Divq) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val.to_bits() as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for Pll1divr1 {
        #[inline(always)]
        fn default() -> Pll1divr1 {
            Pll1divr1(0)
        }
    }
    impl core::fmt::Debug for Pll1divr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pll1divr1")
                .field("divn", &self.divn())
                .field("divp", &self.divp())
                .field("divq", &self.divq())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pll1divr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pll1divr1 {{ divn: {:?}, divp: {:?}, divq: {:?} }}",
                self.divn(),
                self.divp(),
                self.divq()
            )
        }
    }
    #[doc = "RCC PLL1 dividers configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pll1divr2(pub u32);
    impl Pll1divr2 {
        #[doc = "PLL1 DIVS division factor."]
        #[must_use]
        #[inline(always)]
        pub const fn divs(&self) -> super::vals::Pll1divr2Divs {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Pll1divr2Divs::from_bits(val as u8)
        }
        #[doc = "PLL1 DIVS division factor."]
        #[inline(always)]
        pub const fn set_divs(&mut self, val: super::vals::Pll1divr2Divs) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Pll1divr2 {
        #[inline(always)]
        fn default() -> Pll1divr2 {
            Pll1divr2(0)
        }
    }
    impl core::fmt::Debug for Pll1divr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pll1divr2").field("divs", &self.divs()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pll1divr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pll1divr2 {{ divs: {:?} }}", self.divs())
        }
    }
    #[doc = "RCC PLL1 fractional divider register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pll1fracr(pub u32);
    impl Pll1fracr {
        #[doc = "fractional part of the multiplication factor for PLL1 VCO."]
        #[must_use]
        #[inline(always)]
        pub const fn fracn(&self) -> u16 {
            let val = (self.0 >> 3usize) & 0x1fff;
            val as u16
        }
        #[doc = "fractional part of the multiplication factor for PLL1 VCO."]
        #[inline(always)]
        pub const fn set_fracn(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 3usize)) | (((val as u32) & 0x1fff) << 3usize);
        }
    }
    impl Default for Pll1fracr {
        #[inline(always)]
        fn default() -> Pll1fracr {
            Pll1fracr(0)
        }
    }
    impl core::fmt::Debug for Pll1fracr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pll1fracr").field("fracn", &self.fracn()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pll1fracr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pll1fracr {{ fracn: {=u16:?} }}", self.fracn())
        }
    }
    #[doc = "RCC PLL1 Spread Spectrum Clock Generator register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pll1sscgr(pub u32);
    impl Pll1sscgr {
        #[doc = "Modulation Period Adjustment for PLL1."]
        #[must_use]
        #[inline(always)]
        pub const fn modper(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Modulation Period Adjustment for PLL1."]
        #[inline(always)]
        pub const fn set_modper(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "Dithering TPDF noise control for PLL1."]
        #[must_use]
        #[inline(always)]
        pub const fn tpdfndis(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Dithering TPDF noise control for PLL1."]
        #[inline(always)]
        pub const fn set_tpdfndis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Dithering RPDF noise control for PLL1."]
        #[must_use]
        #[inline(always)]
        pub const fn rpdfndis(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Dithering RPDF noise control for PLL1."]
        #[inline(always)]
        pub const fn set_rpdfndis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Spread spectrum clock generator mode for PLL1."]
        #[must_use]
        #[inline(always)]
        pub const fn spreadsel(&self) -> super::vals::Pll1sscgrSpreadsel {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Pll1sscgrSpreadsel::from_bits(val as u8)
        }
        #[doc = "Spread spectrum clock generator mode for PLL1."]
        #[inline(always)]
        pub const fn set_spreadsel(&mut self, val: super::vals::Pll1sscgrSpreadsel) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "Modulation Depth Adjustment for PLL1."]
        #[must_use]
        #[inline(always)]
        pub const fn incstep(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x7fff;
            val as u16
        }
        #[doc = "Modulation Depth Adjustment for PLL1."]
        #[inline(always)]
        pub const fn set_incstep(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 16usize)) | (((val as u32) & 0x7fff) << 16usize);
        }
    }
    impl Default for Pll1sscgr {
        #[inline(always)]
        fn default() -> Pll1sscgr {
            Pll1sscgr(0)
        }
    }
    impl core::fmt::Debug for Pll1sscgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pll1sscgr")
                .field("modper", &self.modper())
                .field("tpdfndis", &self.tpdfndis())
                .field("rpdfndis", &self.rpdfndis())
                .field("spreadsel", &self.spreadsel())
                .field("incstep", &self.incstep())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pll1sscgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pll1sscgr {{ modper: {=u16:?}, tpdfndis: {=bool:?}, rpdfndis: {=bool:?}, spreadsel: {:?}, incstep: {=u16:?} }}",
                self.modper(),
                self.tpdfndis(),
                self.rpdfndis(),
                self.spreadsel(),
                self.incstep()
            )
        }
    }
    #[doc = "RCC PLL2 dividers configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pll2divr1(pub u32);
    impl Pll2divr1 {
        #[doc = "multiplication factor for PLL2 VCO."]
        #[must_use]
        #[inline(always)]
        pub const fn divn(&self) -> super::vals::Pll2divr1Divn {
            let val = (self.0 >> 0usize) & 0x01ff;
            super::vals::Pll2divr1Divn::from_bits(val as u16)
        }
        #[doc = "multiplication factor for PLL2 VCO."]
        #[inline(always)]
        pub const fn set_divn(&mut self, val: super::vals::Pll2divr1Divn) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val.to_bits() as u32) & 0x01ff) << 0usize);
        }
        #[doc = "PLL2 DIVP division factor."]
        #[must_use]
        #[inline(always)]
        pub const fn divp(&self) -> super::vals::Pll2divr1Divp {
            let val = (self.0 >> 9usize) & 0x7f;
            super::vals::Pll2divr1Divp::from_bits(val as u8)
        }
        #[doc = "PLL2 DIVP division factor."]
        #[inline(always)]
        pub const fn set_divp(&mut self, val: super::vals::Pll2divr1Divp) {
            self.0 = (self.0 & !(0x7f << 9usize)) | (((val.to_bits() as u32) & 0x7f) << 9usize);
        }
        #[doc = "PLL2 DIVQ division factor."]
        #[must_use]
        #[inline(always)]
        pub const fn divq(&self) -> super::vals::Pll2divr1Divq {
            let val = (self.0 >> 16usize) & 0x7f;
            super::vals::Pll2divr1Divq::from_bits(val as u8)
        }
        #[doc = "PLL2 DIVQ division factor."]
        #[inline(always)]
        pub const fn set_divq(&mut self, val: super::vals::Pll2divr1Divq) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val.to_bits() as u32) & 0x7f) << 16usize);
        }
        #[doc = "PLL2 DIVR division factor."]
        #[must_use]
        #[inline(always)]
        pub const fn divr(&self) -> super::vals::Pll2divr1Divr {
            let val = (self.0 >> 24usize) & 0x7f;
            super::vals::Pll2divr1Divr::from_bits(val as u8)
        }
        #[doc = "PLL2 DIVR division factor."]
        #[inline(always)]
        pub const fn set_divr(&mut self, val: super::vals::Pll2divr1Divr) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val.to_bits() as u32) & 0x7f) << 24usize);
        }
    }
    impl Default for Pll2divr1 {
        #[inline(always)]
        fn default() -> Pll2divr1 {
            Pll2divr1(0)
        }
    }
    impl core::fmt::Debug for Pll2divr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pll2divr1")
                .field("divn", &self.divn())
                .field("divp", &self.divp())
                .field("divq", &self.divq())
                .field("divr", &self.divr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pll2divr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pll2divr1 {{ divn: {:?}, divp: {:?}, divq: {:?}, divr: {:?} }}",
                self.divn(),
                self.divp(),
                self.divq(),
                self.divr()
            )
        }
    }
    #[doc = "RCC PLL2 dividers configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pll2divr2(pub u32);
    impl Pll2divr2 {
        #[doc = "PLL2 DIVS division factor."]
        #[must_use]
        #[inline(always)]
        pub const fn divs(&self) -> super::vals::Pll2divr2Divs {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Pll2divr2Divs::from_bits(val as u8)
        }
        #[doc = "PLL2 DIVS division factor."]
        #[inline(always)]
        pub const fn set_divs(&mut self, val: super::vals::Pll2divr2Divs) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "PLL2 DIVT division factor."]
        #[must_use]
        #[inline(always)]
        pub const fn divt(&self) -> super::vals::Divt {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Divt::from_bits(val as u8)
        }
        #[doc = "PLL2 DIVT division factor."]
        #[inline(always)]
        pub const fn set_divt(&mut self, val: super::vals::Divt) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
    }
    impl Default for Pll2divr2 {
        #[inline(always)]
        fn default() -> Pll2divr2 {
            Pll2divr2(0)
        }
    }
    impl core::fmt::Debug for Pll2divr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pll2divr2")
                .field("divs", &self.divs())
                .field("divt", &self.divt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pll2divr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pll2divr2 {{ divs: {:?}, divt: {:?} }}", self.divs(), self.divt())
        }
    }
    #[doc = "RCC PLL2 fractional divider register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pll2fracr(pub u32);
    impl Pll2fracr {
        #[doc = "fractional part of the multiplication factor for PLL2 VCO."]
        #[must_use]
        #[inline(always)]
        pub const fn fracn(&self) -> u16 {
            let val = (self.0 >> 3usize) & 0x1fff;
            val as u16
        }
        #[doc = "fractional part of the multiplication factor for PLL2 VCO."]
        #[inline(always)]
        pub const fn set_fracn(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 3usize)) | (((val as u32) & 0x1fff) << 3usize);
        }
    }
    impl Default for Pll2fracr {
        #[inline(always)]
        fn default() -> Pll2fracr {
            Pll2fracr(0)
        }
    }
    impl core::fmt::Debug for Pll2fracr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pll2fracr").field("fracn", &self.fracn()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pll2fracr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pll2fracr {{ fracn: {=u16:?} }}", self.fracn())
        }
    }
    #[doc = "RCC PLL2 Spread Spectrum Clock Generator register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pll2sscgr(pub u32);
    impl Pll2sscgr {
        #[doc = "Modulation Period Adjustment for PLL2."]
        #[must_use]
        #[inline(always)]
        pub const fn modper(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Modulation Period Adjustment for PLL2."]
        #[inline(always)]
        pub const fn set_modper(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "Dithering TPDF noise control for PLL2."]
        #[must_use]
        #[inline(always)]
        pub const fn tpdfndis(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Dithering TPDF noise control for PLL2."]
        #[inline(always)]
        pub const fn set_tpdfndis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Dithering RPDF noise control for PLL2."]
        #[must_use]
        #[inline(always)]
        pub const fn rpdfndis(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Dithering RPDF noise control for PLL2."]
        #[inline(always)]
        pub const fn set_rpdfndis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Spread spectrum clock generator mode for PLL2."]
        #[must_use]
        #[inline(always)]
        pub const fn spreadsel(&self) -> super::vals::Pll2sscgrSpreadsel {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Pll2sscgrSpreadsel::from_bits(val as u8)
        }
        #[doc = "Spread spectrum clock generator mode for PLL2."]
        #[inline(always)]
        pub const fn set_spreadsel(&mut self, val: super::vals::Pll2sscgrSpreadsel) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "Modulation Depth Adjustment for PLL2."]
        #[must_use]
        #[inline(always)]
        pub const fn incstep(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x7fff;
            val as u16
        }
        #[doc = "Modulation Depth Adjustment for PLL2."]
        #[inline(always)]
        pub const fn set_incstep(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 16usize)) | (((val as u32) & 0x7fff) << 16usize);
        }
    }
    impl Default for Pll2sscgr {
        #[inline(always)]
        fn default() -> Pll2sscgr {
            Pll2sscgr(0)
        }
    }
    impl core::fmt::Debug for Pll2sscgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pll2sscgr")
                .field("modper", &self.modper())
                .field("tpdfndis", &self.tpdfndis())
                .field("rpdfndis", &self.rpdfndis())
                .field("spreadsel", &self.spreadsel())
                .field("incstep", &self.incstep())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pll2sscgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pll2sscgr {{ modper: {=u16:?}, tpdfndis: {=bool:?}, rpdfndis: {=bool:?}, spreadsel: {:?}, incstep: {=u16:?} }}",
                self.modper(),
                self.tpdfndis(),
                self.rpdfndis(),
                self.spreadsel(),
                self.incstep()
            )
        }
    }
    #[doc = "RCC PLL3 dividers configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pll3divr1(pub u32);
    impl Pll3divr1 {
        #[doc = "Multiplication factor for PLL3 VCO."]
        #[must_use]
        #[inline(always)]
        pub const fn divn(&self) -> super::vals::Pll3divr1Divn {
            let val = (self.0 >> 0usize) & 0x01ff;
            super::vals::Pll3divr1Divn::from_bits(val as u16)
        }
        #[doc = "Multiplication factor for PLL3 VCO."]
        #[inline(always)]
        pub const fn set_divn(&mut self, val: super::vals::Pll3divr1Divn) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val.to_bits() as u32) & 0x01ff) << 0usize);
        }
        #[doc = "PLL3 DIVP division factor."]
        #[must_use]
        #[inline(always)]
        pub const fn divp(&self) -> super::vals::Pll3divr1Divp {
            let val = (self.0 >> 9usize) & 0x7f;
            super::vals::Pll3divr1Divp::from_bits(val as u8)
        }
        #[doc = "PLL3 DIVP division factor."]
        #[inline(always)]
        pub const fn set_divp(&mut self, val: super::vals::Pll3divr1Divp) {
            self.0 = (self.0 & !(0x7f << 9usize)) | (((val.to_bits() as u32) & 0x7f) << 9usize);
        }
        #[doc = "PLL3 DIVQ division factor."]
        #[must_use]
        #[inline(always)]
        pub const fn divq(&self) -> super::vals::Pll3divr1Divq {
            let val = (self.0 >> 16usize) & 0x7f;
            super::vals::Pll3divr1Divq::from_bits(val as u8)
        }
        #[doc = "PLL3 DIVQ division factor."]
        #[inline(always)]
        pub const fn set_divq(&mut self, val: super::vals::Pll3divr1Divq) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val.to_bits() as u32) & 0x7f) << 16usize);
        }
        #[doc = "PLL3 DIVR division factor."]
        #[must_use]
        #[inline(always)]
        pub const fn divr(&self) -> super::vals::Pll3divr1Divr {
            let val = (self.0 >> 24usize) & 0x7f;
            super::vals::Pll3divr1Divr::from_bits(val as u8)
        }
        #[doc = "PLL3 DIVR division factor."]
        #[inline(always)]
        pub const fn set_divr(&mut self, val: super::vals::Pll3divr1Divr) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val.to_bits() as u32) & 0x7f) << 24usize);
        }
    }
    impl Default for Pll3divr1 {
        #[inline(always)]
        fn default() -> Pll3divr1 {
            Pll3divr1(0)
        }
    }
    impl core::fmt::Debug for Pll3divr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pll3divr1")
                .field("divn", &self.divn())
                .field("divp", &self.divp())
                .field("divq", &self.divq())
                .field("divr", &self.divr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pll3divr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pll3divr1 {{ divn: {:?}, divp: {:?}, divq: {:?}, divr: {:?} }}",
                self.divn(),
                self.divp(),
                self.divq(),
                self.divr()
            )
        }
    }
    #[doc = "RCC PLL3 dividers configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pll3divr2(pub u32);
    impl Pll3divr2 {
        #[doc = "PLL3 DIVS division factor."]
        #[must_use]
        #[inline(always)]
        pub const fn divs(&self) -> super::vals::Pll3divr2Divs {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Pll3divr2Divs::from_bits(val as u8)
        }
        #[doc = "PLL3 DIVS division factor."]
        #[inline(always)]
        pub const fn set_divs(&mut self, val: super::vals::Pll3divr2Divs) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Pll3divr2 {
        #[inline(always)]
        fn default() -> Pll3divr2 {
            Pll3divr2(0)
        }
    }
    impl core::fmt::Debug for Pll3divr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pll3divr2").field("divs", &self.divs()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pll3divr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pll3divr2 {{ divs: {:?} }}", self.divs())
        }
    }
    #[doc = "RCC PLL3 fractional divider register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pll3fracr(pub u32);
    impl Pll3fracr {
        #[doc = "fractional part of the multiplication factor for PLL3 VCO."]
        #[must_use]
        #[inline(always)]
        pub const fn fracn(&self) -> u16 {
            let val = (self.0 >> 3usize) & 0x1fff;
            val as u16
        }
        #[doc = "fractional part of the multiplication factor for PLL3 VCO."]
        #[inline(always)]
        pub const fn set_fracn(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 3usize)) | (((val as u32) & 0x1fff) << 3usize);
        }
    }
    impl Default for Pll3fracr {
        #[inline(always)]
        fn default() -> Pll3fracr {
            Pll3fracr(0)
        }
    }
    impl core::fmt::Debug for Pll3fracr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pll3fracr").field("fracn", &self.fracn()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pll3fracr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pll3fracr {{ fracn: {=u16:?} }}", self.fracn())
        }
    }
    #[doc = "RCC PLL3 Spread Spectrum Clock Generator register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pll3sscgr(pub u32);
    impl Pll3sscgr {
        #[doc = "Modulation Period Adjustment for PLL3."]
        #[must_use]
        #[inline(always)]
        pub const fn modper(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Modulation Period Adjustment for PLL3."]
        #[inline(always)]
        pub const fn set_modper(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "Dithering TPDF noise control for PLL3."]
        #[must_use]
        #[inline(always)]
        pub const fn tpdfndis(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Dithering TPDF noise control for PLL3."]
        #[inline(always)]
        pub const fn set_tpdfndis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Dithering RPDF noise control for PLL3."]
        #[must_use]
        #[inline(always)]
        pub const fn rpdfndis(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Dithering RPDF noise control for PLL3."]
        #[inline(always)]
        pub const fn set_rpdfndis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Spread spectrum clock generator mode for PLL3."]
        #[must_use]
        #[inline(always)]
        pub const fn spreadsel(&self) -> super::vals::Pll3sscgrSpreadsel {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Pll3sscgrSpreadsel::from_bits(val as u8)
        }
        #[doc = "Spread spectrum clock generator mode for PLL3."]
        #[inline(always)]
        pub const fn set_spreadsel(&mut self, val: super::vals::Pll3sscgrSpreadsel) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "Modulation Depth Adjustment for PLL3."]
        #[must_use]
        #[inline(always)]
        pub const fn incstep(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x7fff;
            val as u16
        }
        #[doc = "Modulation Depth Adjustment for PLL3."]
        #[inline(always)]
        pub const fn set_incstep(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 16usize)) | (((val as u32) & 0x7fff) << 16usize);
        }
    }
    impl Default for Pll3sscgr {
        #[inline(always)]
        fn default() -> Pll3sscgr {
            Pll3sscgr(0)
        }
    }
    impl core::fmt::Debug for Pll3sscgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pll3sscgr")
                .field("modper", &self.modper())
                .field("tpdfndis", &self.tpdfndis())
                .field("rpdfndis", &self.rpdfndis())
                .field("spreadsel", &self.spreadsel())
                .field("incstep", &self.incstep())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pll3sscgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pll3sscgr {{ modper: {=u16:?}, tpdfndis: {=bool:?}, rpdfndis: {=bool:?}, spreadsel: {:?}, incstep: {=u16:?} }}",
                self.modper(),
                self.tpdfndis(),
                self.rpdfndis(),
                self.spreadsel(),
                self.incstep()
            )
        }
    }
    #[doc = "RCC PLLs configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pllcfgr(pub u32);
    impl Pllcfgr {
        #[doc = "PLL1 fractional latch enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pll1fracen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PLL1 fractional latch enable."]
        #[inline(always)]
        pub const fn set_pll1fracen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "PLL1 VCO selection."]
        #[must_use]
        #[inline(always)]
        pub const fn pll1vcosel(&self) -> super::vals::Pll1vcosel {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Pll1vcosel::from_bits(val as u8)
        }
        #[doc = "PLL1 VCO selection."]
        #[inline(always)]
        pub const fn set_pll1vcosel(&mut self, val: super::vals::Pll1vcosel) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "PLL1 SSCG enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pll1sscgen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PLL1 SSCG enable."]
        #[inline(always)]
        pub const fn set_pll1sscgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "PLL1 input frequency range."]
        #[must_use]
        #[inline(always)]
        pub const fn pll1rge(&self) -> super::vals::Pll1rge {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Pll1rge::from_bits(val as u8)
        }
        #[doc = "PLL1 input frequency range."]
        #[inline(always)]
        pub const fn set_pll1rge(&mut self, val: super::vals::Pll1rge) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "PLL1 DIVP divider output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pll1pen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "PLL1 DIVP divider output enable."]
        #[inline(always)]
        pub const fn set_pll1pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PLL1 DIVQ divider output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pll1qen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PLL1 DIVQ divider output enable."]
        #[inline(always)]
        pub const fn set_pll1qen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "PLL1 DIVS divider output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pll1sen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "PLL1 DIVS divider output enable."]
        #[inline(always)]
        pub const fn set_pll1sen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "PLL2 fractional latch enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pll2fraclen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "PLL2 fractional latch enable."]
        #[inline(always)]
        pub const fn set_pll2fraclen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "PLL2 VCO selection."]
        #[must_use]
        #[inline(always)]
        pub const fn pll2vcosel(&self) -> super::vals::Pll2vcosel {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Pll2vcosel::from_bits(val as u8)
        }
        #[doc = "PLL2 VCO selection."]
        #[inline(always)]
        pub const fn set_pll2vcosel(&mut self, val: super::vals::Pll2vcosel) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "PLL2 SSCG enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pll2sscgen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "PLL2 SSCG enable."]
        #[inline(always)]
        pub const fn set_pll2sscgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "PLL2 input frequency range."]
        #[must_use]
        #[inline(always)]
        pub const fn pll2rge(&self) -> super::vals::Pll2rge {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::Pll2rge::from_bits(val as u8)
        }
        #[doc = "PLL2 input frequency range."]
        #[inline(always)]
        pub const fn set_pll2rge(&mut self, val: super::vals::Pll2rge) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
        #[doc = "PLL2 DIVP divider output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pll2pen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PLL2 DIVP divider output enable."]
        #[inline(always)]
        pub const fn set_pll2pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "PLL2 DIVQ divider output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pll2qen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "PLL2 DIVQ divider output enable."]
        #[inline(always)]
        pub const fn set_pll2qen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "PLL2 DIVR divider output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pll2ren(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PLL2 DIVR divider output enable."]
        #[inline(always)]
        pub const fn set_pll2ren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PLL2 DIVS divider output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pll2sen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "PLL2 DIVS divider output enable."]
        #[inline(always)]
        pub const fn set_pll2sen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "PLL2 DIVT divider output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pll2ten(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "PLL2 DIVT divider output enable."]
        #[inline(always)]
        pub const fn set_pll2ten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "PLL3 fractional latch enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pll3fracen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "PLL3 fractional latch enable."]
        #[inline(always)]
        pub const fn set_pll3fracen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "PLL3 VCO selection."]
        #[must_use]
        #[inline(always)]
        pub const fn pll3vcosel(&self) -> super::vals::Pll3vcosel {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Pll3vcosel::from_bits(val as u8)
        }
        #[doc = "PLL3 VCO selection."]
        #[inline(always)]
        pub const fn set_pll3vcosel(&mut self, val: super::vals::Pll3vcosel) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "PLL3 SSCG enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pll3sscgen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "PLL3 SSCG enable."]
        #[inline(always)]
        pub const fn set_pll3sscgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "PLL3 input frequency range."]
        #[must_use]
        #[inline(always)]
        pub const fn pll3rge(&self) -> super::vals::Pll3rge {
            let val = (self.0 >> 25usize) & 0x03;
            super::vals::Pll3rge::from_bits(val as u8)
        }
        #[doc = "PLL3 input frequency range."]
        #[inline(always)]
        pub const fn set_pll3rge(&mut self, val: super::vals::Pll3rge) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
        }
        #[doc = "PLL3 DIVP divider output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pll3pen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "PLL3 DIVP divider output enable."]
        #[inline(always)]
        pub const fn set_pll3pen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "PLL3 DIVQ divider output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pll3qen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "PLL3 DIVQ divider output enable."]
        #[inline(always)]
        pub const fn set_pll3qen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "PLL3 DIVR divider output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pll3ren(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "PLL3 DIVR divider output enable."]
        #[inline(always)]
        pub const fn set_pll3ren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "PLL3 DIVS divider output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pll3sen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "PLL3 DIVS divider output enable."]
        #[inline(always)]
        pub const fn set_pll3sen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
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
                .field("pll1fracen", &self.pll1fracen())
                .field("pll1vcosel", &self.pll1vcosel())
                .field("pll1sscgen", &self.pll1sscgen())
                .field("pll1rge", &self.pll1rge())
                .field("pll1pen", &self.pll1pen())
                .field("pll1qen", &self.pll1qen())
                .field("pll1sen", &self.pll1sen())
                .field("pll2fraclen", &self.pll2fraclen())
                .field("pll2vcosel", &self.pll2vcosel())
                .field("pll2sscgen", &self.pll2sscgen())
                .field("pll2rge", &self.pll2rge())
                .field("pll2pen", &self.pll2pen())
                .field("pll2qen", &self.pll2qen())
                .field("pll2ren", &self.pll2ren())
                .field("pll2sen", &self.pll2sen())
                .field("pll2ten", &self.pll2ten())
                .field("pll3fracen", &self.pll3fracen())
                .field("pll3vcosel", &self.pll3vcosel())
                .field("pll3sscgen", &self.pll3sscgen())
                .field("pll3rge", &self.pll3rge())
                .field("pll3pen", &self.pll3pen())
                .field("pll3qen", &self.pll3qen())
                .field("pll3ren", &self.pll3ren())
                .field("pll3sen", &self.pll3sen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pllcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pllcfgr {{ pll1fracen: {=bool:?}, pll1vcosel: {:?}, pll1sscgen: {=bool:?}, pll1rge: {:?}, pll1pen: {=bool:?}, pll1qen: {=bool:?}, pll1sen: {=bool:?}, pll2fraclen: {=bool:?}, pll2vcosel: {:?}, pll2sscgen: {=bool:?}, pll2rge: {:?}, pll2pen: {=bool:?}, pll2qen: {=bool:?}, pll2ren: {=bool:?}, pll2sen: {=bool:?}, pll2ten: {=bool:?}, pll3fracen: {=bool:?}, pll3vcosel: {:?}, pll3sscgen: {=bool:?}, pll3rge: {:?}, pll3pen: {=bool:?}, pll3qen: {=bool:?}, pll3ren: {=bool:?}, pll3sen: {=bool:?} }}",
                self.pll1fracen(),
                self.pll1vcosel(),
                self.pll1sscgen(),
                self.pll1rge(),
                self.pll1pen(),
                self.pll1qen(),
                self.pll1sen(),
                self.pll2fraclen(),
                self.pll2vcosel(),
                self.pll2sscgen(),
                self.pll2rge(),
                self.pll2pen(),
                self.pll2qen(),
                self.pll2ren(),
                self.pll2sen(),
                self.pll2ten(),
                self.pll3fracen(),
                self.pll3vcosel(),
                self.pll3sscgen(),
                self.pll3rge(),
                self.pll3pen(),
                self.pll3qen(),
                self.pll3ren(),
                self.pll3sen()
            )
        }
    }
    #[doc = "RCC PLLs clock source selection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pllckselr(pub u32);
    impl Pllckselr {
        #[doc = "DIVMx and PLLs clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn pllsrc(&self) -> super::vals::Pllsrc {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Pllsrc::from_bits(val as u8)
        }
        #[doc = "DIVMx and PLLs clock source selection."]
        #[inline(always)]
        pub const fn set_pllsrc(&mut self, val: super::vals::Pllsrc) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "prescaler for PLL1."]
        #[must_use]
        #[inline(always)]
        pub const fn divm1(&self) -> super::vals::Divm1 {
            let val = (self.0 >> 4usize) & 0x3f;
            super::vals::Divm1::from_bits(val as u8)
        }
        #[doc = "prescaler for PLL1."]
        #[inline(always)]
        pub const fn set_divm1(&mut self, val: super::vals::Divm1) {
            self.0 = (self.0 & !(0x3f << 4usize)) | (((val.to_bits() as u32) & 0x3f) << 4usize);
        }
        #[doc = "prescaler for PLL2."]
        #[must_use]
        #[inline(always)]
        pub const fn divm2(&self) -> super::vals::Divm2 {
            let val = (self.0 >> 12usize) & 0x3f;
            super::vals::Divm2::from_bits(val as u8)
        }
        #[doc = "prescaler for PLL2."]
        #[inline(always)]
        pub const fn set_divm2(&mut self, val: super::vals::Divm2) {
            self.0 = (self.0 & !(0x3f << 12usize)) | (((val.to_bits() as u32) & 0x3f) << 12usize);
        }
        #[doc = "prescaler for PLL3."]
        #[must_use]
        #[inline(always)]
        pub const fn divm3(&self) -> super::vals::Divm3 {
            let val = (self.0 >> 20usize) & 0x3f;
            super::vals::Divm3::from_bits(val as u8)
        }
        #[doc = "prescaler for PLL3."]
        #[inline(always)]
        pub const fn set_divm3(&mut self, val: super::vals::Divm3) {
            self.0 = (self.0 & !(0x3f << 20usize)) | (((val.to_bits() as u32) & 0x3f) << 20usize);
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
                .field("divm1", &self.divm1())
                .field("divm2", &self.divm2())
                .field("divm3", &self.divm3())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pllckselr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pllckselr {{ pllsrc: {:?}, divm1: {:?}, divm2: {:?}, divm3: {:?} }}",
                self.pllsrc(),
                self.divm1(),
                self.divm2(),
                self.divm3()
            )
        }
    }
    #[doc = "RCC Reset status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rsr(pub u32);
    impl Rsr {
        #[doc = "remove reset flags."]
        #[must_use]
        #[inline(always)]
        pub const fn rmvf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "remove reset flags."]
        #[inline(always)]
        pub const fn set_rmvf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Option byte loading reset flag less thansup>(1)less than/sup>."]
        #[must_use]
        #[inline(always)]
        pub const fn oblrstf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte loading reset flag less thansup>(1)less than/sup>."]
        #[inline(always)]
        pub const fn set_oblrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "BOR reset flag less thansup>(1)less than/sup>."]
        #[must_use]
        #[inline(always)]
        pub const fn borrstf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "BOR reset flag less thansup>(1)less than/sup>."]
        #[inline(always)]
        pub const fn set_borrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "pin reset flag (NRST) less thansup>(1)less than/sup>."]
        #[must_use]
        #[inline(always)]
        pub const fn pinrstf(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "pin reset flag (NRST) less thansup>(1)less than/sup>."]
        #[inline(always)]
        pub const fn set_pinrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "POR/PDR reset flag less thansup>(1)less than/sup>."]
        #[must_use]
        #[inline(always)]
        pub const fn porrstf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "POR/PDR reset flag less thansup>(1)less than/sup>."]
        #[inline(always)]
        pub const fn set_porrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "system reset from CPU reset flag less thansup>(1)less than/sup>."]
        #[must_use]
        #[inline(always)]
        pub const fn sftrstf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "system reset from CPU reset flag less thansup>(1)less than/sup>."]
        #[inline(always)]
        pub const fn set_sftrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "independent watchdog reset flag less thansup>(1)less than/sup>."]
        #[must_use]
        #[inline(always)]
        pub const fn iwdgrstf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "independent watchdog reset flag less thansup>(1)less than/sup>."]
        #[inline(always)]
        pub const fn set_iwdgrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "window watchdog reset flag less thansup>(1)less than/sup>."]
        #[must_use]
        #[inline(always)]
        pub const fn wwdgrstf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "window watchdog reset flag less thansup>(1)less than/sup>."]
        #[inline(always)]
        pub const fn set_wwdgrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "reset due to illegal Stop or Standby flag."]
        #[must_use]
        #[inline(always)]
        pub const fn lpwrrstf(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "reset due to illegal Stop or Standby flag."]
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
                .field("oblrstf", &self.oblrstf())
                .field("borrstf", &self.borrstf())
                .field("pinrstf", &self.pinrstf())
                .field("porrstf", &self.porrstf())
                .field("sftrstf", &self.sftrstf())
                .field("iwdgrstf", &self.iwdgrstf())
                .field("wwdgrstf", &self.wwdgrstf())
                .field("lpwrrstf", &self.lpwrrstf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rsr {{ rmvf: {=bool:?}, oblrstf: {=bool:?}, borrstf: {=bool:?}, pinrstf: {=bool:?}, porrstf: {=bool:?}, sftrstf: {=bool:?}, iwdgrstf: {=bool:?}, wwdgrstf: {=bool:?}, lpwrrstf: {=bool:?} }}",
                self.rmvf(),
                self.oblrstf(),
                self.borrstf(),
                self.pinrstf(),
                self.porrstf(),
                self.sftrstf(),
                self.iwdgrstf(),
                self.wwdgrstf(),
                self.lpwrrstf()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adcsel {
        #[doc = "hclk3 selected as kernel peripheral clock (default after reset)."]
        Hclk3 = 0x0,
        #[doc = "rcc_timx_ker_ck selected as kernel peripheral clock."]
        TimxKer = 0x01,
        #[doc = "hsi_ker_ck selected as kernel peripheral clock."]
        Hsi = 0x02,
        #[doc = "hse_ker_ck selected as kernel peripheral clock."]
        Hse = 0x03,
        #[doc = "pll2_p_ck selected as kernel peripheral clock."]
        Pll2P = 0x04,
        #[doc = "pll3_r_ck selected as kernel peripheral clock."]
        Pll3R = 0x05,
        #[doc = "per_ck selected as kernel peripheral clock."]
        Per = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Adcsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adcsel {
            unsafe { core::mem::transmute(val & 0x07) }
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
    pub enum Adf1sel {
        #[doc = "hclk1 selected as ADF kernel clock (default after reset)."]
        Hclk1 = 0x0,
        #[doc = "pll2_p_ck selected as ADF kernel clock."]
        Pll2P = 0x01,
        #[doc = "pll3_p_ck selected as ADF kernel clock."]
        Pll3P = 0x02,
        #[doc = "AUDIOCLK selected as ADF kernel clock."]
        Audioclk = 0x03,
        #[doc = "csi_ker_ck selected as ADF kernel clock."]
        Csi = 0x04,
        #[doc = "hsi_ker_ck selected as ADF kernel clock."]
        Hsi = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Adf1sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adf1sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adf1sel {
        #[inline(always)]
        fn from(val: u8) -> Adf1sel {
            Adf1sel::from_bits(val)
        }
    }
    impl From<Adf1sel> for u8 {
        #[inline(always)]
        fn from(val: Adf1sel) -> u8 {
            Adf1sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ckpersel {
        #[doc = "hsi_ker_ck selected as per_ck clock (default after reset)."]
        Hsi = 0x0,
        #[doc = "csi_ker_ck selected as per_ck clock."]
        Csi = 0x01,
        #[doc = "hse_ker_ck selected as per_ck clock."]
        Hse = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ckpersel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ckpersel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ckpersel {
        #[inline(always)]
        fn from(val: u8) -> Ckpersel {
            Ckpersel::from_bits(val)
        }
    }
    impl From<Ckpersel> for u8 {
        #[inline(always)]
        fn from(val: Ckpersel) -> u8 {
            Ckpersel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cpre {
        #[doc = "sys_ck not divided (default after reset)."]
        Div1 = 0x0,
        #[doc = "sys_ck not divided (default after reset)."]
        Reserved1 = 0x01,
        #[doc = "sys_ck not divided (default after reset)."]
        Reserved2 = 0x02,
        #[doc = "sys_ck not divided (default after reset)."]
        Reserved3 = 0x03,
        #[doc = "sys_ck not divided (default after reset)."]
        Reserved4 = 0x04,
        #[doc = "sys_ck not divided (default after reset)."]
        Reserved5 = 0x05,
        #[doc = "sys_ck not divided (default after reset)."]
        Reserved6 = 0x06,
        #[doc = "sys_ck not divided (default after reset)."]
        Reserved7 = 0x07,
        #[doc = "sys_ck divided by 2."]
        Div2 = 0x08,
        #[doc = "sys_ck divided by 4."]
        Div4 = 0x09,
        #[doc = "sys_ck divided by 8."]
        Div8 = 0x0a,
        #[doc = "sys_ck divided by 16."]
        Div16 = 0x0b,
        #[doc = "sys_ck divided by 64."]
        Div64 = 0x0c,
        #[doc = "sys_ck divided by 128."]
        Div128 = 0x0d,
        #[doc = "sys_ck divided by 256."]
        Div256 = 0x0e,
        #[doc = "sys_ck divided by 512."]
        Div512 = 0x0f,
    }
    impl Cpre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cpre {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cpre {
        #[inline(always)]
        fn from(val: u8) -> Cpre {
            Cpre::from_bits(val)
        }
    }
    impl From<Cpre> for u8 {
        #[inline(always)]
        fn from(val: Cpre) -> u8 {
            Cpre::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dac1sel {
        #[doc = "hclk3 selected as kernel peripheral clock - synchronous mode (default after reset)."]
        Hclk3 = 0x0,
        #[doc = "rcc_tim_ker_ck selected as kernel peripheral clock."]
        TimxKer = 0x01,
        #[doc = "rcc_adc_ker_ck selected as kernel peripheral clock."]
        Adc = 0x02,
        #[doc = "hsi_ker_ck selected as kernel peripheral clock."]
        Hsi = 0x03,
        #[doc = "hse_ker_ck selected as kernel peripheral clock."]
        Hse = 0x04,
        #[doc = "pll2_p_ck selected as kernel peripheral clock."]
        Pll2P = 0x05,
        #[doc = "pll3_r_ck selected as kernel peripheral clock."]
        Pll3R = 0x06,
        #[doc = "per_ck selected as kernel peripheral clock."]
        Per = 0x07,
    }
    impl Dac1sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dac1sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dac1sel {
        #[inline(always)]
        fn from(val: u8) -> Dac1sel {
            Dac1sel::from_bits(val)
        }
    }
    impl From<Dac1sel> for u8 {
        #[inline(always)]
        fn from(val: Dac1sel) -> u8 {
            Dac1sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dac1shsel {
        #[doc = "lse_ck selected as rcc_dac_hold_ck."]
        Lse = 0x0,
        #[doc = "lsi_ck selected as rcc_dac_hold_ck."]
        Lsi = 0x01,
    }
    impl Dac1shsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dac1shsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dac1shsel {
        #[inline(always)]
        fn from(val: u8) -> Dac1shsel {
            Dac1shsel::from_bits(val)
        }
    }
    impl From<Dac1shsel> for u8 {
        #[inline(always)]
        fn from(val: Dac1shsel) -> u8 {
            Dac1shsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dac2sel {
        #[doc = "hclk3 selected as kernel peripheral clock - synchronous mode (default after reset)."]
        Hclk3 = 0x0,
        #[doc = "rcc_tim_ker_ck selected as kernel peripheral clock."]
        TimxKer = 0x01,
        #[doc = "rcc_adc_ker_ck selected as kernel peripheral clock."]
        Adc = 0x02,
        #[doc = "hsi_ker_ck selected as kernel peripheral clock."]
        Hsi = 0x03,
        #[doc = "hse_ker_ck selected as kernel peripheral clock."]
        Hse = 0x04,
        #[doc = "pll2_p_ck selected as kernel peripheral clock."]
        Pll2P = 0x05,
        #[doc = "pll3_r_ck selected as kernel peripheral clock."]
        Pll3R = 0x06,
        #[doc = "per_ck selected as kernel peripheral clock."]
        Per = 0x07,
    }
    impl Dac2sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dac2sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dac2sel {
        #[inline(always)]
        fn from(val: u8) -> Dac2sel {
            Dac2sel::from_bits(val)
        }
    }
    impl From<Dac2sel> for u8 {
        #[inline(always)]
        fn from(val: Dac2sel) -> u8 {
            Dac2sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dac3sel {
        #[doc = "hclk3 selected as kernel peripheral clock - synchronous mode (default after reset)."]
        Hclk3 = 0x0,
        #[doc = "rcc_tim_ker_ck selected as kernel peripheral clock."]
        TimxKer = 0x01,
        #[doc = "rcc_adc_ker_ck selected as kernel peripheral clock."]
        Adc = 0x02,
        #[doc = "hsi_ker_ck selected as kernel peripheral clock."]
        Hsi = 0x03,
        #[doc = "hse_ker_ck selected as kernel peripheral clock."]
        Hse = 0x04,
        #[doc = "pll2_p_ck selected as kernel peripheral clock."]
        Pll2P = 0x05,
        #[doc = "pll3_r_ck selected as kernel peripheral clock."]
        Pll3R = 0x06,
        #[doc = "per_ck selected as kernel peripheral clock."]
        Per = 0x07,
    }
    impl Dac3sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dac3sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dac3sel {
        #[inline(always)]
        fn from(val: u8) -> Dac3sel {
            Dac3sel::from_bits(val)
        }
    }
    impl From<Dac3sel> for u8 {
        #[inline(always)]
        fn from(val: Dac3sel) -> u8 {
            Dac3sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dac4sel {
        #[doc = "hclk3 selected as kernel peripheral clock - synchronous mode (default after reset)."]
        Hclk3 = 0x0,
        #[doc = "rcc_tim_ker_ck selected as kernel peripheral clock."]
        TimxKer = 0x01,
        #[doc = "rcc_adc_ker_ck selected as kernel peripheral clock."]
        Adc = 0x02,
        #[doc = "hsi_ker_ck selected as kernel peripheral clock."]
        Hsi = 0x03,
        #[doc = "hse_ker_ck selected as kernel peripheral clock."]
        Hse = 0x04,
        #[doc = "pll2_p_ck selected as kernel peripheral clock."]
        Pll2P = 0x05,
        #[doc = "pll3_r_ck selected as kernel peripheral clock."]
        Pll3R = 0x06,
        #[doc = "per_ck selected as kernel peripheral clock."]
        Per = 0x07,
    }
    impl Dac4sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dac4sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dac4sel {
        #[inline(always)]
        fn from(val: u8) -> Dac4sel {
            Dac4sel::from_bits(val)
        }
    }
    impl From<Dac4sel> for u8 {
        #[inline(always)]
        fn from(val: Dac4sel) -> u8 {
            Dac4sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Divm1 {
        #[doc = "prescaler disabled."]
        Disable = 0x0,
        #[doc = "division by 1 (bypass)."]
        Div1 = 0x01,
        #[doc = "division by 2."]
        Div2 = 0x02,
        #[doc = "division by 3."]
        Div3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
        _RESERVED_10 = 0x10,
        _RESERVED_11 = 0x11,
        _RESERVED_12 = 0x12,
        _RESERVED_13 = 0x13,
        _RESERVED_14 = 0x14,
        _RESERVED_15 = 0x15,
        _RESERVED_16 = 0x16,
        _RESERVED_17 = 0x17,
        _RESERVED_18 = 0x18,
        _RESERVED_19 = 0x19,
        _RESERVED_1a = 0x1a,
        _RESERVED_1b = 0x1b,
        _RESERVED_1c = 0x1c,
        _RESERVED_1d = 0x1d,
        _RESERVED_1e = 0x1e,
        _RESERVED_1f = 0x1f,
        #[doc = "division by 32 (default after reset)."]
        Div32 = 0x20,
        _RESERVED_21 = 0x21,
        _RESERVED_22 = 0x22,
        _RESERVED_23 = 0x23,
        _RESERVED_24 = 0x24,
        _RESERVED_25 = 0x25,
        _RESERVED_26 = 0x26,
        _RESERVED_27 = 0x27,
        _RESERVED_28 = 0x28,
        _RESERVED_29 = 0x29,
        _RESERVED_2a = 0x2a,
        _RESERVED_2b = 0x2b,
        _RESERVED_2c = 0x2c,
        _RESERVED_2d = 0x2d,
        _RESERVED_2e = 0x2e,
        _RESERVED_2f = 0x2f,
        _RESERVED_30 = 0x30,
        _RESERVED_31 = 0x31,
        _RESERVED_32 = 0x32,
        _RESERVED_33 = 0x33,
        _RESERVED_34 = 0x34,
        _RESERVED_35 = 0x35,
        _RESERVED_36 = 0x36,
        _RESERVED_37 = 0x37,
        _RESERVED_38 = 0x38,
        _RESERVED_39 = 0x39,
        _RESERVED_3a = 0x3a,
        _RESERVED_3b = 0x3b,
        _RESERVED_3c = 0x3c,
        _RESERVED_3d = 0x3d,
        _RESERVED_3e = 0x3e,
        #[doc = "division by 63."]
        Div63 = 0x3f,
    }
    impl Divm1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Divm1 {
            unsafe { core::mem::transmute(val & 0x3f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Divm1 {
        #[inline(always)]
        fn from(val: u8) -> Divm1 {
            Divm1::from_bits(val)
        }
    }
    impl From<Divm1> for u8 {
        #[inline(always)]
        fn from(val: Divm1) -> u8 {
            Divm1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Divm2 {
        #[doc = "prescaler disabled."]
        Disable = 0x0,
        #[doc = "division by 1 (bypass)."]
        Div1 = 0x01,
        #[doc = "division by 2."]
        Div2 = 0x02,
        #[doc = "division by 3."]
        Div3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
        _RESERVED_10 = 0x10,
        _RESERVED_11 = 0x11,
        _RESERVED_12 = 0x12,
        _RESERVED_13 = 0x13,
        _RESERVED_14 = 0x14,
        _RESERVED_15 = 0x15,
        _RESERVED_16 = 0x16,
        _RESERVED_17 = 0x17,
        _RESERVED_18 = 0x18,
        _RESERVED_19 = 0x19,
        _RESERVED_1a = 0x1a,
        _RESERVED_1b = 0x1b,
        _RESERVED_1c = 0x1c,
        _RESERVED_1d = 0x1d,
        _RESERVED_1e = 0x1e,
        _RESERVED_1f = 0x1f,
        #[doc = "division by 32 (default after reset)."]
        Div32 = 0x20,
        _RESERVED_21 = 0x21,
        _RESERVED_22 = 0x22,
        _RESERVED_23 = 0x23,
        _RESERVED_24 = 0x24,
        _RESERVED_25 = 0x25,
        _RESERVED_26 = 0x26,
        _RESERVED_27 = 0x27,
        _RESERVED_28 = 0x28,
        _RESERVED_29 = 0x29,
        _RESERVED_2a = 0x2a,
        _RESERVED_2b = 0x2b,
        _RESERVED_2c = 0x2c,
        _RESERVED_2d = 0x2d,
        _RESERVED_2e = 0x2e,
        _RESERVED_2f = 0x2f,
        _RESERVED_30 = 0x30,
        _RESERVED_31 = 0x31,
        _RESERVED_32 = 0x32,
        _RESERVED_33 = 0x33,
        _RESERVED_34 = 0x34,
        _RESERVED_35 = 0x35,
        _RESERVED_36 = 0x36,
        _RESERVED_37 = 0x37,
        _RESERVED_38 = 0x38,
        _RESERVED_39 = 0x39,
        _RESERVED_3a = 0x3a,
        _RESERVED_3b = 0x3b,
        _RESERVED_3c = 0x3c,
        _RESERVED_3d = 0x3d,
        _RESERVED_3e = 0x3e,
        #[doc = "division by 63."]
        Div63 = 0x3f,
    }
    impl Divm2 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Divm2 {
            unsafe { core::mem::transmute(val & 0x3f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Divm2 {
        #[inline(always)]
        fn from(val: u8) -> Divm2 {
            Divm2::from_bits(val)
        }
    }
    impl From<Divm2> for u8 {
        #[inline(always)]
        fn from(val: Divm2) -> u8 {
            Divm2::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Divm3 {
        #[doc = "prescaler disabled."]
        Disable = 0x0,
        #[doc = "division by 1 (bypass)."]
        Div1 = 0x01,
        #[doc = "division by 2."]
        Div2 = 0x02,
        #[doc = "division by 3."]
        Div3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
        _RESERVED_10 = 0x10,
        _RESERVED_11 = 0x11,
        _RESERVED_12 = 0x12,
        _RESERVED_13 = 0x13,
        _RESERVED_14 = 0x14,
        _RESERVED_15 = 0x15,
        _RESERVED_16 = 0x16,
        _RESERVED_17 = 0x17,
        _RESERVED_18 = 0x18,
        _RESERVED_19 = 0x19,
        _RESERVED_1a = 0x1a,
        _RESERVED_1b = 0x1b,
        _RESERVED_1c = 0x1c,
        _RESERVED_1d = 0x1d,
        _RESERVED_1e = 0x1e,
        _RESERVED_1f = 0x1f,
        #[doc = "division by 32 (default after reset)."]
        Div32 = 0x20,
        _RESERVED_21 = 0x21,
        _RESERVED_22 = 0x22,
        _RESERVED_23 = 0x23,
        _RESERVED_24 = 0x24,
        _RESERVED_25 = 0x25,
        _RESERVED_26 = 0x26,
        _RESERVED_27 = 0x27,
        _RESERVED_28 = 0x28,
        _RESERVED_29 = 0x29,
        _RESERVED_2a = 0x2a,
        _RESERVED_2b = 0x2b,
        _RESERVED_2c = 0x2c,
        _RESERVED_2d = 0x2d,
        _RESERVED_2e = 0x2e,
        _RESERVED_2f = 0x2f,
        _RESERVED_30 = 0x30,
        _RESERVED_31 = 0x31,
        _RESERVED_32 = 0x32,
        _RESERVED_33 = 0x33,
        _RESERVED_34 = 0x34,
        _RESERVED_35 = 0x35,
        _RESERVED_36 = 0x36,
        _RESERVED_37 = 0x37,
        _RESERVED_38 = 0x38,
        _RESERVED_39 = 0x39,
        _RESERVED_3a = 0x3a,
        _RESERVED_3b = 0x3b,
        _RESERVED_3c = 0x3c,
        _RESERVED_3d = 0x3d,
        _RESERVED_3e = 0x3e,
        #[doc = "division by 63."]
        Div63 = 0x3f,
    }
    impl Divm3 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Divm3 {
            unsafe { core::mem::transmute(val & 0x3f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Divm3 {
        #[inline(always)]
        fn from(val: u8) -> Divm3 {
            Divm3::from_bits(val)
        }
    }
    impl From<Divm3> for u8 {
        #[inline(always)]
        fn from(val: Divm3) -> u8 {
            Divm3::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Divt {
        #[doc = "pll2_t_ck = vco2_ck."]
        Div1 = 0x0,
        #[doc = "pll2_t_ck = vco2_ck / 2 (default after reset)."]
        Div2 = 0x01,
        #[doc = "pll2_t_ck = vco2_ck / 3."]
        Div3 = 0x02,
        #[doc = "pll2_t_ck = vco2_ck / 4."]
        Div4 = 0x03,
        #[doc = "pll2_t_ck = vco2_ck / 5."]
        Div5 = 0x04,
        #[doc = "pll2_t_ck = vco2_ck / 6."]
        Div6 = 0x05,
        #[doc = "pll2_t_ck = vco2_ck / 7."]
        Div7 = 0x06,
        #[doc = "pll2_t_ck = vco2_ck / 8."]
        Div8 = 0x07,
    }
    impl Divt {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Divt {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Divt {
        #[inline(always)]
        fn from(val: u8) -> Divt {
            Divt::from_bits(val)
        }
    }
    impl From<Divt> for u8 {
        #[inline(always)]
        fn from(val: Divt) -> u8 {
            Divt::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fdcansel {
        #[doc = "hse_ker_ck selected as FDCAN kernel clock (default after reset)."]
        Hse = 0x0,
        #[doc = "pll1_q_ck selected as FDCAN kernel clock."]
        Pll1Q = 0x01,
        #[doc = "pll2_p_ck selected as FDCAN kernel clock."]
        Pll2P = 0x02,
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
        #[doc = "hclk5 selected as kernel peripheral clock (default after reset)."]
        Hclk5 = 0x0,
        #[doc = "pll1_q_ck selected as kernel peripheral clock."]
        Pll1Q = 0x01,
        #[doc = "pll2_r_ck selected as kernel peripheral clock."]
        Pll2R = 0x02,
        #[doc = "hsi_ker_ck selected as kernel peripheral clock."]
        Hsi = 0x03,
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
    pub enum Fmcswp {
        #[doc = "The switch is in neutral mode and output clock is gated (default after reset)."]
        Neutral = 0x0,
        #[doc = "The switch is selecting hclk5."]
        Hclk5 = 0x01,
        #[doc = "The switch is selecting pll1_q_ck."]
        Pll1Q = 0x02,
        #[doc = "The switch is selecting pll2_r_ck."]
        Pll2R = 0x03,
        #[doc = "The switch is selecting hsi_ker_ck."]
        Hsi = 0x04,
        #[doc = "The switch is in recovery position (hclk5/4)."]
        Recovery = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Fmcswp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fmcswp {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fmcswp {
        #[inline(always)]
        fn from(val: u8) -> Fmcswp {
            Fmcswp::from_bits(val)
        }
    }
    impl From<Fmcswp> for u8 {
        #[inline(always)]
        fn from(val: Fmcswp) -> u8 {
            Fmcswp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hsidiv {
        #[doc = "division by 1, hsi(_ker)_ck = 64 MHz (default after reset)."]
        Div1 = 0x0,
        #[doc = "division by 2, hsi(_ker)_ck = 32 MHz."]
        Div2 = 0x01,
        #[doc = "division by 4, hsi(_ker)_ck = 16 MHz."]
        Div4 = 0x02,
        #[doc = "division by 8, hsi(_ker)_ck = 8 MHz."]
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
    pub enum I2c123sel {
        #[doc = "pclk1 selected as kernel clock (default after reset)."]
        Pclk1 = 0x0,
        #[doc = "pll3_r_ck selected as kernel clock."]
        Pll3R = 0x01,
        #[doc = "hsi_ker_ck selected as kernel clock."]
        Hsi = 0x02,
        #[doc = "csi_ker_ck selected as kernel clock."]
        Csi = 0x03,
    }
    impl I2c123sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2c123sel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I2c123sel {
        #[inline(always)]
        fn from(val: u8) -> I2c123sel {
            I2c123sel::from_bits(val)
        }
    }
    impl From<I2c123sel> for u8 {
        #[inline(always)]
        fn from(val: I2c123sel) -> u8 {
            I2c123sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum I3c1sel {
        #[doc = "pclk1 selected as kernel peripheral clock (default after reset)."]
        Pclk1 = 0x0,
        #[doc = "pll3_r_ck selected as kernel peripheral clock."]
        Pll3R = 0x01,
        #[doc = "hsi_ker_ck selected as kernel peripheral clock."]
        Hsi = 0x02,
        #[doc = "csi_ker_ck selected as kernel peripheral clock."]
        Csi = 0x03,
    }
    impl I3c1sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I3c1sel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I3c1sel {
        #[inline(always)]
        fn from(val: u8) -> I3c1sel {
            I3c1sel::from_bits(val)
        }
    }
    impl From<I3c1sel> for u8 {
        #[inline(always)]
        fn from(val: I3c1sel) -> u8 {
            I3c1sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lptim1sel {
        #[doc = "pclk1 selected as kernel peripheral clock (default after reset)."]
        Pclk1 = 0x0,
        #[doc = "pll2_p_ck selected as kernel peripheral clock."]
        Pll2P = 0x01,
        #[doc = "pll3_r_ck selected as kernel peripheral clock."]
        Pll3R = 0x02,
        #[doc = "lse_ck selected as kernel peripheral clock."]
        Lse = 0x03,
        #[doc = "lsi_ck selected as kernel peripheral clock."]
        Lsi = 0x04,
        #[doc = "per_ck selected as kernel peripheral clock."]
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
    pub enum Lptim23sel {
        #[doc = "pclk4 selected as kernel peripheral clock (default after reset)."]
        Pclk4 = 0x0,
        #[doc = "pll2_p_ck selected as kernel peripheral clock."]
        Pll2P = 0x01,
        #[doc = "pll3_r_ck selected as kernel peripheral clock."]
        Pll3R = 0x02,
        #[doc = "lse_ck selected as kernel peripheral clock."]
        Lse = 0x03,
        #[doc = "lsi_ck selected as kernel peripheral clock."]
        Lsi = 0x04,
        #[doc = "per_ck selected as kernel peripheral clock."]
        Per = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Lptim23sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lptim23sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lptim23sel {
        #[inline(always)]
        fn from(val: u8) -> Lptim23sel {
            Lptim23sel::from_bits(val)
        }
    }
    impl From<Lptim23sel> for u8 {
        #[inline(always)]
        fn from(val: Lptim23sel) -> u8 {
            Lptim23sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lptim45sel {
        #[doc = "pclk4 selected as kernel peripheral clock (default after reset)."]
        Pclk4 = 0x0,
        #[doc = "pll2_p_ck selected as kernel peripheral clock."]
        Pll2P = 0x01,
        #[doc = "pll3_r_ck selected as kernel peripheral clock."]
        Pll3R = 0x02,
        #[doc = "lse_ck selected as kernel peripheral clock."]
        Lse = 0x03,
        #[doc = "lsi_ck selected as kernel peripheral clock."]
        Lsi = 0x04,
        #[doc = "per_ck selected as kernel peripheral clock."]
        Per = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Lptim45sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lptim45sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lptim45sel {
        #[inline(always)]
        fn from(val: u8) -> Lptim45sel {
            Lptim45sel::from_bits(val)
        }
    }
    impl From<Lptim45sel> for u8 {
        #[inline(always)]
        fn from(val: Lptim45sel) -> u8 {
            Lptim45sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lpuart1sel {
        #[doc = "pclk4 selected as kernel peripheral clock (default after reset)."]
        Pclk4 = 0x0,
        #[doc = "pll2_q_ck selected as kernel peripheral clock."]
        Pll2Q = 0x01,
        #[doc = "pll3_q_ck selected as kernel peripheral clock."]
        Pll3Q = 0x02,
        #[doc = "hsi_ker_ck selected as kernel peripheral clock."]
        Hsi = 0x03,
        #[doc = "csi_ker_ck selected as kernel peripheral clock."]
        Csi = 0x04,
        #[doc = "lse_ck selected as kernel peripheral clock."]
        Lse = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Lpuart1sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lpuart1sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lpuart1sel {
        #[inline(always)]
        fn from(val: u8) -> Lpuart1sel {
            Lpuart1sel::from_bits(val)
        }
    }
    impl From<Lpuart1sel> for u8 {
        #[inline(always)]
        fn from(val: Lpuart1sel) -> u8 {
            Lpuart1sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lsedrv {
        #[doc = "lowest drive."]
        Lowest = 0x0,
        #[doc = "medium-low drive."]
        MediumLow = 0x01,
        #[doc = "medium-high drive (default after backup domain reset)."]
        MediumHigh = 0x02,
        #[doc = "highest drive."]
        Highest = 0x03,
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
    pub enum Mco1pre {
        #[doc = "prescaler disabled (default after reset)."]
        Disable = 0x0,
        #[doc = "division by 1 (bypass)."]
        Div1 = 0x01,
        #[doc = "division by 2."]
        Div2 = 0x02,
        #[doc = "division by 3."]
        Div3 = 0x03,
        #[doc = "division by 4."]
        Div4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        #[doc = "division by 15."]
        Div15 = 0x0f,
    }
    impl Mco1pre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mco1pre {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mco1pre {
        #[inline(always)]
        fn from(val: u8) -> Mco1pre {
            Mco1pre::from_bits(val)
        }
    }
    impl From<Mco1pre> for u8 {
        #[inline(always)]
        fn from(val: Mco1pre) -> u8 {
            Mco1pre::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mco1sel {
        #[doc = "HSI clock selected (hsi_ck) (default after reset)."]
        Hsi = 0x0,
        #[doc = "LSE oscillator clock selected (lse_ck)."]
        Lse = 0x01,
        #[doc = "HSE clock selected (hse_ck)."]
        Hse = 0x02,
        #[doc = "PLL1 clock selected (pll1_q_ck)."]
        Pll1Q = 0x03,
        #[doc = "HSI48 clock selected (hsi48_ck)."]
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
    pub enum Mco2pre {
        #[doc = "prescaler disabled (default after reset)."]
        Disable = 0x0,
        #[doc = "division by 1 (bypass)."]
        Div1 = 0x01,
        #[doc = "division by 2."]
        Div2 = 0x02,
        #[doc = "division by 3."]
        Div3 = 0x03,
        #[doc = "division by 4."]
        Div4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        #[doc = "division by 15."]
        Div15 = 0x0f,
    }
    impl Mco2pre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mco2pre {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mco2pre {
        #[inline(always)]
        fn from(val: u8) -> Mco2pre {
            Mco2pre::from_bits(val)
        }
    }
    impl From<Mco2pre> for u8 {
        #[inline(always)]
        fn from(val: Mco2pre) -> u8 {
            Mco2pre::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mco2sel {
        #[doc = "system clock selected (sys_ck) (default after reset)."]
        Sys = 0x0,
        #[doc = "PLL2 oscillator clock selected (pll2_p_ck)."]
        Pll2P = 0x01,
        #[doc = "HSE clock selected (hse_ck)."]
        Hse = 0x02,
        #[doc = "PLL1 clock selected (pll1_p_ck)."]
        Pll1P = 0x03,
        #[doc = "CSI clock selected (csi_ck)."]
        Csi = 0x04,
        #[doc = "LSI clock selected (lsi_ck)."]
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
    pub enum Otgfssel {
        #[doc = "hsi48_ker_ck (default after reset)."]
        Hsi48 = 0x0,
        #[doc = "pll3_q_ck."]
        Pll3Q = 0x01,
        #[doc = "hse_ker_ck."]
        Hse = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Otgfssel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Otgfssel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Otgfssel {
        #[inline(always)]
        fn from(val: u8) -> Otgfssel {
            Otgfssel::from_bits(val)
        }
    }
    impl From<Otgfssel> for u8 {
        #[inline(always)]
        fn from(val: Otgfssel) -> u8 {
            Otgfssel::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pll1divr1Divn(u16);
    impl Pll1divr1Divn {
        #[doc = "wrong configuration."]
        pub const Invalid: Self = Self(0x06);
        #[doc = "DIVN = 8."]
        pub const Div8: Self = Self(0x07);
        #[doc = "DIVN = 129 (default after reset)."]
        pub const Div129: Self = Self(0x80);
        #[doc = "DIVN = 420."]
        pub const Div420: Self = Self(0x01a3);
    }
    impl Pll1divr1Divn {
        pub const fn from_bits(val: u16) -> Pll1divr1Divn {
            Self(val & 0x01ff)
        }
        pub const fn to_bits(self) -> u16 {
            self.0
        }
    }
    impl core::fmt::Debug for Pll1divr1Divn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x06 => f.write_str("Invalid"),
                0x07 => f.write_str("Div8"),
                0x80 => f.write_str("Div129"),
                0x01a3 => f.write_str("Div420"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pll1divr1Divn {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x06 => defmt::write!(f, "Invalid"),
                0x07 => defmt::write!(f, "Div8"),
                0x80 => defmt::write!(f, "Div129"),
                0x01a3 => defmt::write!(f, "Div420"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u16> for Pll1divr1Divn {
        #[inline(always)]
        fn from(val: u16) -> Pll1divr1Divn {
            Pll1divr1Divn::from_bits(val)
        }
    }
    impl From<Pll1divr1Divn> for u16 {
        #[inline(always)]
        fn from(val: Pll1divr1Divn) -> u16 {
            Pll1divr1Divn::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pll1divr1Divp(u8);
    impl Pll1divr1Divp {
        #[doc = "pll1_p_ck = vco1_ck."]
        pub const Div1: Self = Self(0x0);
        #[doc = "pll1_p_ck = vco1_ck / 2 (default after reset)."]
        pub const Div2: Self = Self(0x01);
        #[doc = "not allowed."]
        pub const Invalid: Self = Self(0x02);
        #[doc = "pll1_p_ck = vco1_ck / 4."]
        pub const Div4: Self = Self(0x03);
        #[doc = "pll1_p_ck = vco1_ck / 128."]
        pub const Div128: Self = Self(0x7f);
    }
    impl Pll1divr1Divp {
        pub const fn from_bits(val: u8) -> Pll1divr1Divp {
            Self(val & 0x7f)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for Pll1divr1Divp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("Div1"),
                0x01 => f.write_str("Div2"),
                0x02 => f.write_str("Invalid"),
                0x03 => f.write_str("Div4"),
                0x7f => f.write_str("Div128"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pll1divr1Divp {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "Div1"),
                0x01 => defmt::write!(f, "Div2"),
                0x02 => defmt::write!(f, "Invalid"),
                0x03 => defmt::write!(f, "Div4"),
                0x7f => defmt::write!(f, "Div128"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for Pll1divr1Divp {
        #[inline(always)]
        fn from(val: u8) -> Pll1divr1Divp {
            Pll1divr1Divp::from_bits(val)
        }
    }
    impl From<Pll1divr1Divp> for u8 {
        #[inline(always)]
        fn from(val: Pll1divr1Divp) -> u8 {
            Pll1divr1Divp::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pll1divr1Divq(u8);
    impl Pll1divr1Divq {
        #[doc = "pll1_q_ck = vco1_ck."]
        pub const Div1: Self = Self(0x0);
        #[doc = "pll1_q_ck = vco1_ck / 2 (default after reset)."]
        pub const Div2: Self = Self(0x01);
        #[doc = "pll1_q_ck = vco1_ck / 3."]
        pub const Div3: Self = Self(0x02);
        #[doc = "pll1_q_ck = vco1_ck / 4."]
        pub const Div4: Self = Self(0x03);
        #[doc = "pll1_q_ck = vco1_ck / 128."]
        pub const Div128: Self = Self(0x7f);
    }
    impl Pll1divr1Divq {
        pub const fn from_bits(val: u8) -> Pll1divr1Divq {
            Self(val & 0x7f)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for Pll1divr1Divq {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("Div1"),
                0x01 => f.write_str("Div2"),
                0x02 => f.write_str("Div3"),
                0x03 => f.write_str("Div4"),
                0x7f => f.write_str("Div128"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pll1divr1Divq {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "Div1"),
                0x01 => defmt::write!(f, "Div2"),
                0x02 => defmt::write!(f, "Div3"),
                0x03 => defmt::write!(f, "Div4"),
                0x7f => defmt::write!(f, "Div128"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for Pll1divr1Divq {
        #[inline(always)]
        fn from(val: u8) -> Pll1divr1Divq {
            Pll1divr1Divq::from_bits(val)
        }
    }
    impl From<Pll1divr1Divq> for u8 {
        #[inline(always)]
        fn from(val: Pll1divr1Divq) -> u8 {
            Pll1divr1Divq::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pll1divr2Divs {
        #[doc = "pll1_s_ck = vco1_ck."]
        Div1 = 0x0,
        #[doc = "pll1_s_ck = vco1_ck / 2 (default after reset)."]
        Div2 = 0x01,
        #[doc = "pll1_s_ck = vco1_ck / 3."]
        Div3 = 0x02,
        #[doc = "pll1_s_ck = vco1_ck / 4."]
        Div4 = 0x03,
        #[doc = "pll1_s_ck = vco1_ck / 5."]
        Div5 = 0x04,
        #[doc = "pll1_s_ck = vco1_ck / 6."]
        Div6 = 0x05,
        #[doc = "pll1_s_ck = vco1_ck / 7."]
        Div7 = 0x06,
        #[doc = "pll1_s_ck = vco1_ck / 8."]
        Div8 = 0x07,
    }
    impl Pll1divr2Divs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pll1divr2Divs {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pll1divr2Divs {
        #[inline(always)]
        fn from(val: u8) -> Pll1divr2Divs {
            Pll1divr2Divs::from_bits(val)
        }
    }
    impl From<Pll1divr2Divs> for u8 {
        #[inline(always)]
        fn from(val: Pll1divr2Divs) -> u8 {
            Pll1divr2Divs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pll1rge {
        #[doc = "PLL1 input (ref1_ck) clock range frequency between 1 and 2 MHz (default after reset)."]
        Range12Mhz = 0x0,
        #[doc = "PLL1 input (ref1_ck) clock range frequency between 2 and 4 MHz."]
        Range24Mhz = 0x01,
        #[doc = "PLL1 input (ref1_ck) clock range frequency between 4 and 8 MHz."]
        Range48Mhz = 0x02,
        #[doc = "PLL1 input (ref1_ck) clock range frequency between 8 and 16 MHz."]
        Range816Mhz = 0x03,
    }
    impl Pll1rge {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pll1rge {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pll1rge {
        #[inline(always)]
        fn from(val: u8) -> Pll1rge {
            Pll1rge::from_bits(val)
        }
    }
    impl From<Pll1rge> for u8 {
        #[inline(always)]
        fn from(val: Pll1rge) -> u8 {
            Pll1rge::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pll1sscgrSpreadsel {
        #[doc = "Center-spread modulation selected (default after reset)."]
        Center = 0x0,
        #[doc = "Down-spread modulation selected."]
        Down = 0x01,
    }
    impl Pll1sscgrSpreadsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pll1sscgrSpreadsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pll1sscgrSpreadsel {
        #[inline(always)]
        fn from(val: u8) -> Pll1sscgrSpreadsel {
            Pll1sscgrSpreadsel::from_bits(val)
        }
    }
    impl From<Pll1sscgrSpreadsel> for u8 {
        #[inline(always)]
        fn from(val: Pll1sscgrSpreadsel) -> u8 {
            Pll1sscgrSpreadsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pll1vcosel {
        #[doc = "VCOH selected (default after reset)."]
        Vcoh = 0x0,
        #[doc = "VCOL selected."]
        Vcol = 0x01,
    }
    impl Pll1vcosel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pll1vcosel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pll1vcosel {
        #[inline(always)]
        fn from(val: u8) -> Pll1vcosel {
            Pll1vcosel::from_bits(val)
        }
    }
    impl From<Pll1vcosel> for u8 {
        #[inline(always)]
        fn from(val: Pll1vcosel) -> u8 {
            Pll1vcosel::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pll2divr1Divn(u16);
    impl Pll2divr1Divn {
        #[doc = "wrong configuration."]
        pub const Invalid: Self = Self(0x06);
        #[doc = "DIVN = 8."]
        pub const Div8: Self = Self(0x07);
        #[doc = "DIVN = 129 (default after reset)."]
        pub const Div129: Self = Self(0x80);
        #[doc = "DIVN = 420."]
        pub const Div420: Self = Self(0x01a3);
    }
    impl Pll2divr1Divn {
        pub const fn from_bits(val: u16) -> Pll2divr1Divn {
            Self(val & 0x01ff)
        }
        pub const fn to_bits(self) -> u16 {
            self.0
        }
    }
    impl core::fmt::Debug for Pll2divr1Divn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x06 => f.write_str("Invalid"),
                0x07 => f.write_str("Div8"),
                0x80 => f.write_str("Div129"),
                0x01a3 => f.write_str("Div420"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pll2divr1Divn {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x06 => defmt::write!(f, "Invalid"),
                0x07 => defmt::write!(f, "Div8"),
                0x80 => defmt::write!(f, "Div129"),
                0x01a3 => defmt::write!(f, "Div420"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u16> for Pll2divr1Divn {
        #[inline(always)]
        fn from(val: u16) -> Pll2divr1Divn {
            Pll2divr1Divn::from_bits(val)
        }
    }
    impl From<Pll2divr1Divn> for u16 {
        #[inline(always)]
        fn from(val: Pll2divr1Divn) -> u16 {
            Pll2divr1Divn::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pll2divr1Divp(u8);
    impl Pll2divr1Divp {
        #[doc = "pll2_p_ck = vco2_ck."]
        pub const Div1: Self = Self(0x0);
        #[doc = "pll2_p_ck = vco2_ck / 2 (default after reset)."]
        pub const Div2: Self = Self(0x01);
        #[doc = "pll2_p_ck = vco2_ck / 3."]
        pub const Invalid: Self = Self(0x02);
        #[doc = "pll2_p_ck = vco2_ck / 4."]
        pub const Div4: Self = Self(0x03);
        #[doc = "pll2_p_ck = vco2_ck / 128."]
        pub const Div128: Self = Self(0x7f);
    }
    impl Pll2divr1Divp {
        pub const fn from_bits(val: u8) -> Pll2divr1Divp {
            Self(val & 0x7f)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for Pll2divr1Divp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("Div1"),
                0x01 => f.write_str("Div2"),
                0x02 => f.write_str("Invalid"),
                0x03 => f.write_str("Div4"),
                0x7f => f.write_str("Div128"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pll2divr1Divp {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "Div1"),
                0x01 => defmt::write!(f, "Div2"),
                0x02 => defmt::write!(f, "Invalid"),
                0x03 => defmt::write!(f, "Div4"),
                0x7f => defmt::write!(f, "Div128"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for Pll2divr1Divp {
        #[inline(always)]
        fn from(val: u8) -> Pll2divr1Divp {
            Pll2divr1Divp::from_bits(val)
        }
    }
    impl From<Pll2divr1Divp> for u8 {
        #[inline(always)]
        fn from(val: Pll2divr1Divp) -> u8 {
            Pll2divr1Divp::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pll2divr1Divq(u8);
    impl Pll2divr1Divq {
        #[doc = "pll2_q_ck = vco2_ck."]
        pub const Div1: Self = Self(0x0);
        #[doc = "pll2_q_ck = vco2_ck / 2 (default after reset)."]
        pub const Div2: Self = Self(0x01);
        #[doc = "pll2_q_ck = vco2_ck / 3."]
        pub const Div3: Self = Self(0x02);
        #[doc = "pll2_q_ck = vco2_ck / 4."]
        pub const Div4: Self = Self(0x03);
        #[doc = "pll2_q_ck = vco2_ck / 128."]
        pub const Div128: Self = Self(0x7f);
    }
    impl Pll2divr1Divq {
        pub const fn from_bits(val: u8) -> Pll2divr1Divq {
            Self(val & 0x7f)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for Pll2divr1Divq {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("Div1"),
                0x01 => f.write_str("Div2"),
                0x02 => f.write_str("Div3"),
                0x03 => f.write_str("Div4"),
                0x7f => f.write_str("Div128"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pll2divr1Divq {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "Div1"),
                0x01 => defmt::write!(f, "Div2"),
                0x02 => defmt::write!(f, "Div3"),
                0x03 => defmt::write!(f, "Div4"),
                0x7f => defmt::write!(f, "Div128"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for Pll2divr1Divq {
        #[inline(always)]
        fn from(val: u8) -> Pll2divr1Divq {
            Pll2divr1Divq::from_bits(val)
        }
    }
    impl From<Pll2divr1Divq> for u8 {
        #[inline(always)]
        fn from(val: Pll2divr1Divq) -> u8 {
            Pll2divr1Divq::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pll2divr1Divr(u8);
    impl Pll2divr1Divr {
        #[doc = "pll2_r_ck = vco2_ck."]
        pub const Div1: Self = Self(0x0);
        #[doc = "pll2_r_ck = vco2_ck / 2 (default after reset)."]
        pub const Div2: Self = Self(0x01);
        #[doc = "pll2_r_ck = vco2_ck / 3."]
        pub const Div3: Self = Self(0x02);
        #[doc = "pll2_r_ck = vco2_ck / 4."]
        pub const Div4: Self = Self(0x03);
        #[doc = "pll2_r_ck = vco2_ck / 128."]
        pub const Div128: Self = Self(0x7f);
    }
    impl Pll2divr1Divr {
        pub const fn from_bits(val: u8) -> Pll2divr1Divr {
            Self(val & 0x7f)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for Pll2divr1Divr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("Div1"),
                0x01 => f.write_str("Div2"),
                0x02 => f.write_str("Div3"),
                0x03 => f.write_str("Div4"),
                0x7f => f.write_str("Div128"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pll2divr1Divr {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "Div1"),
                0x01 => defmt::write!(f, "Div2"),
                0x02 => defmt::write!(f, "Div3"),
                0x03 => defmt::write!(f, "Div4"),
                0x7f => defmt::write!(f, "Div128"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for Pll2divr1Divr {
        #[inline(always)]
        fn from(val: u8) -> Pll2divr1Divr {
            Pll2divr1Divr::from_bits(val)
        }
    }
    impl From<Pll2divr1Divr> for u8 {
        #[inline(always)]
        fn from(val: Pll2divr1Divr) -> u8 {
            Pll2divr1Divr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pll2divr2Divs {
        #[doc = "pll2_s_ck = vco2_ck."]
        Div1 = 0x0,
        #[doc = "pll2_s_ck = vco2_ck / 2 (default after reset)."]
        Div2 = 0x01,
        #[doc = "pll2_s_ck = vco2_ck / 3."]
        Div3 = 0x02,
        #[doc = "pll2_s_ck = vco2_ck / 4."]
        Div4 = 0x03,
        #[doc = "pll2_s_ck = vco2_ck / 5."]
        Div5 = 0x04,
        #[doc = "pll2_s_ck = vco2_ck / 6."]
        Div6 = 0x05,
        #[doc = "pll2_s_ck = vco2_ck / 7."]
        Div7 = 0x06,
        #[doc = "pll2_s_ck = vco2_ck / 8."]
        Div8 = 0x07,
    }
    impl Pll2divr2Divs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pll2divr2Divs {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pll2divr2Divs {
        #[inline(always)]
        fn from(val: u8) -> Pll2divr2Divs {
            Pll2divr2Divs::from_bits(val)
        }
    }
    impl From<Pll2divr2Divs> for u8 {
        #[inline(always)]
        fn from(val: Pll2divr2Divs) -> u8 {
            Pll2divr2Divs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pll2rge {
        #[doc = "PLL3 input (ref2_ck) clock range frequency between 1 and 2 MHz (default after reset)."]
        Range12Mhz = 0x0,
        #[doc = "PLL3 input (ref2_ck) clock range frequency between 2 and 4 MHz."]
        Range24Mhz = 0x01,
        #[doc = "PLL3 input (ref2_ck) clock range frequency between 4 and 8 MHz."]
        Range48Mhz = 0x02,
        #[doc = "PLL3 input (ref2_ck) clock range frequency between 8 and 16 MHz."]
        Range816Mhz = 0x03,
    }
    impl Pll2rge {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pll2rge {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pll2rge {
        #[inline(always)]
        fn from(val: u8) -> Pll2rge {
            Pll2rge::from_bits(val)
        }
    }
    impl From<Pll2rge> for u8 {
        #[inline(always)]
        fn from(val: Pll2rge) -> u8 {
            Pll2rge::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pll2sscgrSpreadsel {
        #[doc = "Center-spread modulation selected (default after reset)."]
        Center = 0x0,
        #[doc = "Down-spread modulation selected."]
        Down = 0x01,
    }
    impl Pll2sscgrSpreadsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pll2sscgrSpreadsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pll2sscgrSpreadsel {
        #[inline(always)]
        fn from(val: u8) -> Pll2sscgrSpreadsel {
            Pll2sscgrSpreadsel::from_bits(val)
        }
    }
    impl From<Pll2sscgrSpreadsel> for u8 {
        #[inline(always)]
        fn from(val: Pll2sscgrSpreadsel) -> u8 {
            Pll2sscgrSpreadsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pll2vcosel {
        #[doc = "VCOH selected (default after reset)."]
        Vcoh = 0x0,
        #[doc = "VCOL selected."]
        Vcol = 0x01,
    }
    impl Pll2vcosel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pll2vcosel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pll2vcosel {
        #[inline(always)]
        fn from(val: u8) -> Pll2vcosel {
            Pll2vcosel::from_bits(val)
        }
    }
    impl From<Pll2vcosel> for u8 {
        #[inline(always)]
        fn from(val: Pll2vcosel) -> u8 {
            Pll2vcosel::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pll3divr1Divn(u16);
    impl Pll3divr1Divn {
        #[doc = "wrong configuration."]
        pub const Invalid: Self = Self(0x06);
        #[doc = "DIVN = 8."]
        pub const Div8: Self = Self(0x07);
        #[doc = "DIVN = 129 (default after reset)."]
        pub const Div129: Self = Self(0x80);
        #[doc = "DIVN = 420."]
        pub const Div420: Self = Self(0x01a3);
    }
    impl Pll3divr1Divn {
        pub const fn from_bits(val: u16) -> Pll3divr1Divn {
            Self(val & 0x01ff)
        }
        pub const fn to_bits(self) -> u16 {
            self.0
        }
    }
    impl core::fmt::Debug for Pll3divr1Divn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x06 => f.write_str("Invalid"),
                0x07 => f.write_str("Div8"),
                0x80 => f.write_str("Div129"),
                0x01a3 => f.write_str("Div420"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pll3divr1Divn {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x06 => defmt::write!(f, "Invalid"),
                0x07 => defmt::write!(f, "Div8"),
                0x80 => defmt::write!(f, "Div129"),
                0x01a3 => defmt::write!(f, "Div420"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u16> for Pll3divr1Divn {
        #[inline(always)]
        fn from(val: u16) -> Pll3divr1Divn {
            Pll3divr1Divn::from_bits(val)
        }
    }
    impl From<Pll3divr1Divn> for u16 {
        #[inline(always)]
        fn from(val: Pll3divr1Divn) -> u16 {
            Pll3divr1Divn::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pll3divr1Divp(u8);
    impl Pll3divr1Divp {
        #[doc = "pll3_p_ck = vco3_ck."]
        pub const Div1: Self = Self(0x0);
        #[doc = "pll3_p_ck = vco3_ck / 2 (default after reset)."]
        pub const Div2: Self = Self(0x01);
        #[doc = "pll3_p_ck = vco3_ck / 3."]
        pub const Invalid: Self = Self(0x02);
        #[doc = "pll3_p_ck = vco3_ck / 4."]
        pub const Div4: Self = Self(0x03);
        #[doc = "pll3_p_ck = vco3_ck / 128."]
        pub const Div128: Self = Self(0x7f);
    }
    impl Pll3divr1Divp {
        pub const fn from_bits(val: u8) -> Pll3divr1Divp {
            Self(val & 0x7f)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for Pll3divr1Divp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("Div1"),
                0x01 => f.write_str("Div2"),
                0x02 => f.write_str("Invalid"),
                0x03 => f.write_str("Div4"),
                0x7f => f.write_str("Div128"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pll3divr1Divp {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "Div1"),
                0x01 => defmt::write!(f, "Div2"),
                0x02 => defmt::write!(f, "Invalid"),
                0x03 => defmt::write!(f, "Div4"),
                0x7f => defmt::write!(f, "Div128"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for Pll3divr1Divp {
        #[inline(always)]
        fn from(val: u8) -> Pll3divr1Divp {
            Pll3divr1Divp::from_bits(val)
        }
    }
    impl From<Pll3divr1Divp> for u8 {
        #[inline(always)]
        fn from(val: Pll3divr1Divp) -> u8 {
            Pll3divr1Divp::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pll3divr1Divq(u8);
    impl Pll3divr1Divq {
        #[doc = "pll3_q_ck = vco3_ck."]
        pub const Div1: Self = Self(0x0);
        #[doc = "pll3_q_ck = vco3_ck / 2 (default after reset)."]
        pub const Div2: Self = Self(0x01);
        #[doc = "pll3_q_ck = vco3_ck / 3."]
        pub const Div3: Self = Self(0x02);
        #[doc = "pll3_q_ck = vco3_ck / 4."]
        pub const Div4: Self = Self(0x03);
        #[doc = "pll3_q_ck = vco3_ck / 128."]
        pub const Div128: Self = Self(0x7f);
    }
    impl Pll3divr1Divq {
        pub const fn from_bits(val: u8) -> Pll3divr1Divq {
            Self(val & 0x7f)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for Pll3divr1Divq {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("Div1"),
                0x01 => f.write_str("Div2"),
                0x02 => f.write_str("Div3"),
                0x03 => f.write_str("Div4"),
                0x7f => f.write_str("Div128"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pll3divr1Divq {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "Div1"),
                0x01 => defmt::write!(f, "Div2"),
                0x02 => defmt::write!(f, "Div3"),
                0x03 => defmt::write!(f, "Div4"),
                0x7f => defmt::write!(f, "Div128"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for Pll3divr1Divq {
        #[inline(always)]
        fn from(val: u8) -> Pll3divr1Divq {
            Pll3divr1Divq::from_bits(val)
        }
    }
    impl From<Pll3divr1Divq> for u8 {
        #[inline(always)]
        fn from(val: Pll3divr1Divq) -> u8 {
            Pll3divr1Divq::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pll3divr1Divr(u8);
    impl Pll3divr1Divr {
        #[doc = "pll3_r_ck = vco3_ck."]
        pub const Div1: Self = Self(0x0);
        #[doc = "pll3_r_ck = vco3_ck / 2 (default after reset)."]
        pub const Div2: Self = Self(0x01);
        #[doc = "pll3_r_ck = vco3_ck / 3."]
        pub const Div3: Self = Self(0x02);
        #[doc = "pll3_r_ck = vco3_ck / 4."]
        pub const Div4: Self = Self(0x03);
        #[doc = "pll3_r_ck = vco3_ck / 128."]
        pub const Div128: Self = Self(0x7f);
    }
    impl Pll3divr1Divr {
        pub const fn from_bits(val: u8) -> Pll3divr1Divr {
            Self(val & 0x7f)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for Pll3divr1Divr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("Div1"),
                0x01 => f.write_str("Div2"),
                0x02 => f.write_str("Div3"),
                0x03 => f.write_str("Div4"),
                0x7f => f.write_str("Div128"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pll3divr1Divr {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "Div1"),
                0x01 => defmt::write!(f, "Div2"),
                0x02 => defmt::write!(f, "Div3"),
                0x03 => defmt::write!(f, "Div4"),
                0x7f => defmt::write!(f, "Div128"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for Pll3divr1Divr {
        #[inline(always)]
        fn from(val: u8) -> Pll3divr1Divr {
            Pll3divr1Divr::from_bits(val)
        }
    }
    impl From<Pll3divr1Divr> for u8 {
        #[inline(always)]
        fn from(val: Pll3divr1Divr) -> u8 {
            Pll3divr1Divr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pll3divr2Divs {
        #[doc = "pll3_s_ck = vco3_ck."]
        Div1 = 0x0,
        #[doc = "pll3_s_ck = vco3_ck / 2 (default after reset)."]
        Div2 = 0x01,
        #[doc = "pll3_s_ck = vco3_ck / 3."]
        Div3 = 0x02,
        #[doc = "pll3_s_ck = vco3_ck / 4."]
        Div4 = 0x03,
        #[doc = "pll3_s_ck = vco3_ck / 5."]
        Div5 = 0x04,
        #[doc = "pll3_s_ck = vco3_ck / 6."]
        Div6 = 0x05,
        #[doc = "pll3_s_ck = vco3_ck / 7."]
        Div7 = 0x06,
        #[doc = "pll3_s_ck = vco3_ck / 8."]
        Div8 = 0x07,
    }
    impl Pll3divr2Divs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pll3divr2Divs {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pll3divr2Divs {
        #[inline(always)]
        fn from(val: u8) -> Pll3divr2Divs {
            Pll3divr2Divs::from_bits(val)
        }
    }
    impl From<Pll3divr2Divs> for u8 {
        #[inline(always)]
        fn from(val: Pll3divr2Divs) -> u8 {
            Pll3divr2Divs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pll3rge {
        #[doc = "PLL3 input (ref3_ck) clock range frequency between 1 and 2 MHz (default after reset)."]
        Range12Mhz = 0x0,
        #[doc = "PLL3 input (ref3_ck) clock range frequency between 2 and 4 MHz."]
        Range24Mhz = 0x01,
        #[doc = "PLL3 input (ref3_ck) clock range frequency between 4 and 8 MHz."]
        Range48Mhz = 0x02,
        #[doc = "PLL3 input (ref3_ck) clock range frequency between 8 and 16 MHz."]
        Range816Mhz = 0x03,
    }
    impl Pll3rge {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pll3rge {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pll3rge {
        #[inline(always)]
        fn from(val: u8) -> Pll3rge {
            Pll3rge::from_bits(val)
        }
    }
    impl From<Pll3rge> for u8 {
        #[inline(always)]
        fn from(val: Pll3rge) -> u8 {
            Pll3rge::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pll3sscgrSpreadsel {
        #[doc = "Center-spread modulation selected (default after reset)."]
        Center = 0x0,
        #[doc = "Down-spread modulation selected."]
        Down = 0x01,
    }
    impl Pll3sscgrSpreadsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pll3sscgrSpreadsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pll3sscgrSpreadsel {
        #[inline(always)]
        fn from(val: u8) -> Pll3sscgrSpreadsel {
            Pll3sscgrSpreadsel::from_bits(val)
        }
    }
    impl From<Pll3sscgrSpreadsel> for u8 {
        #[inline(always)]
        fn from(val: Pll3sscgrSpreadsel) -> u8 {
            Pll3sscgrSpreadsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pll3vcosel {
        #[doc = "VCOH selected (default after reset)."]
        Vcoh = 0x0,
        #[doc = "VCOL selected."]
        Vcol = 0x01,
    }
    impl Pll3vcosel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pll3vcosel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pll3vcosel {
        #[inline(always)]
        fn from(val: u8) -> Pll3vcosel {
            Pll3vcosel::from_bits(val)
        }
    }
    impl From<Pll3vcosel> for u8 {
        #[inline(always)]
        fn from(val: Pll3vcosel) -> u8 {
            Pll3vcosel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllsrc {
        #[doc = "HSI selected as PLL clock (hsi_ck) (default after reset)."]
        Hsi = 0x0,
        #[doc = "CSI selected as PLL clock (csi_ck)."]
        Csi = 0x01,
        #[doc = "HSE selected as PLL clock (hse_ck)."]
        Hse = 0x02,
        #[doc = "no clock send to DIVMx divider and PLLs."]
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
    pub enum Ppre1 {
        #[doc = "rcc_pclk1 = sys_bus_ck (default after reset)."]
        Div1 = 0x0,
        #[doc = "rcc_pclk1 = sys_bus_ck (default after reset)."]
        Reserved1 = 0x01,
        #[doc = "rcc_pclk1 = sys_bus_ck (default after reset)."]
        Reserved2 = 0x02,
        #[doc = "rcc_pclk1 = sys_bus_ck (default after reset)."]
        Reserved3 = 0x03,
        #[doc = "rcc_pclk1 = sys_bus_ck / 2."]
        Div2 = 0x04,
        #[doc = "rcc_pclk1 = sys_bus_ck / 4."]
        Div4 = 0x05,
        #[doc = "rcc_pclk1 = sys_bus_ck / 8."]
        Div8 = 0x06,
        #[doc = "rcc_pclk1 = sys_bus_ck / 16."]
        Div16 = 0x07,
    }
    impl Ppre1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ppre1 {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ppre1 {
        #[inline(always)]
        fn from(val: u8) -> Ppre1 {
            Ppre1::from_bits(val)
        }
    }
    impl From<Ppre1> for u8 {
        #[inline(always)]
        fn from(val: Ppre1) -> u8 {
            Ppre1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ppre2 {
        #[doc = "rcc_pclk2 = sys_bus_ck (default after reset)."]
        Div1 = 0x0,
        #[doc = "rcc_pclk2 = sys_bus_ck (default after reset)."]
        Reserved1 = 0x01,
        #[doc = "rcc_pclk2 = sys_bus_ck (default after reset)."]
        Reserved2 = 0x02,
        #[doc = "rcc_pclk2 = sys_bus_ck (default after reset)."]
        Reserved3 = 0x03,
        #[doc = "rcc_pclk2 = sys_bus_ck / 2."]
        Div2 = 0x04,
        #[doc = "rcc_pclk2 = sys_bus_ck / 4."]
        Div4 = 0x05,
        #[doc = "rcc_pclk2 = sys_bus_ck / 8."]
        Div8 = 0x06,
        #[doc = "rcc_pclk2 = sys_bus_ck / 16."]
        Div16 = 0x07,
    }
    impl Ppre2 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ppre2 {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ppre2 {
        #[inline(always)]
        fn from(val: u8) -> Ppre2 {
            Ppre2::from_bits(val)
        }
    }
    impl From<Ppre2> for u8 {
        #[inline(always)]
        fn from(val: Ppre2) -> u8 {
            Ppre2::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ppre4 {
        #[doc = "rcc_pclk4 = sys_bus_ck (default after reset)."]
        Div1 = 0x0,
        #[doc = "rcc_pclk4 = sys_bus_ck (default after reset)."]
        Reserved1 = 0x01,
        #[doc = "rcc_pclk4 = sys_bus_ck (default after reset)."]
        Reserved2 = 0x02,
        #[doc = "rcc_pclk4 = sys_bus_ck (default after reset)."]
        Reserved3 = 0x03,
        #[doc = "rcc_pclk4 = sys_bus_ck / 2."]
        Div2 = 0x04,
        #[doc = "rcc_pclk4 = sys_bus_ck / 4."]
        Div4 = 0x05,
        #[doc = "rcc_pclk4 = sys_bus_ck / 8."]
        Div8 = 0x06,
        #[doc = "rcc_pclk4 = sys_bus_ck / 16."]
        Div16 = 0x07,
    }
    impl Ppre4 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ppre4 {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ppre4 {
        #[inline(always)]
        fn from(val: u8) -> Ppre4 {
            Ppre4::from_bits(val)
        }
    }
    impl From<Ppre4> for u8 {
        #[inline(always)]
        fn from(val: Ppre4) -> u8 {
            Ppre4::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rtcsel {
        #[doc = "no clock (default after Backup domain reset)."]
        Disable = 0x0,
        #[doc = "LSE selected as RTC clock."]
        Lse = 0x01,
        #[doc = "LSI selected as RTC clock."]
        Lsi = 0x02,
        #[doc = "HSE divided by RTCPRE value selected as RTC clock."]
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
    pub enum Sdmmc1sel {
        #[doc = "pll2_s_ck selected as kernel peripheral clock (default after reset)."]
        Pll2S = 0x0,
        #[doc = "pll2_t_ck selected as kernel peripheral clock."]
        Pll2T = 0x01,
    }
    impl Sdmmc1sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sdmmc1sel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sdmmc1sel {
        #[inline(always)]
        fn from(val: u8) -> Sdmmc1sel {
            Sdmmc1sel::from_bits(val)
        }
    }
    impl From<Sdmmc1sel> for u8 {
        #[inline(always)]
        fn from(val: Sdmmc1sel) -> u8 {
            Sdmmc1sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Spi1sel {
        #[doc = "pll1_q_ck selected as SPI/I2S1 kernel clock (default after reset)."]
        Pll1Q = 0x0,
        #[doc = "pll2_p_ck selected as SPI/I2S1 kernel clock."]
        Pll2P = 0x01,
        #[doc = "pll3_p_ck selected as SPI/I2S1 kernel clock."]
        Pll3P = 0x02,
        #[doc = "AUDIOCLK selected as SPI/I2S1 kernel clock."]
        Audioclk = 0x03,
        #[doc = "per_ck selected as SPI/I2S1 kernel clock."]
        Per = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Spi1sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Spi1sel {
            unsafe { core::mem::transmute(val & 0x07) }
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
    pub enum Spi23sel {
        #[doc = "pll1_q_ck selected as kernel clock (default after reset)."]
        Pll1Q = 0x0,
        #[doc = "pll2_p_ck selected as kernel clock."]
        Pll2P = 0x01,
        #[doc = "pll3_p_ck selected as kernel clock."]
        Pll3P = 0x02,
        #[doc = "AUDIOCLK selected as kernel clock."]
        Audioclk = 0x03,
        #[doc = "per_ck selected as kernel clock."]
        Per = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Spi23sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Spi23sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Spi23sel {
        #[inline(always)]
        fn from(val: u8) -> Spi23sel {
            Spi23sel::from_bits(val)
        }
    }
    impl From<Spi23sel> for u8 {
        #[inline(always)]
        fn from(val: Spi23sel) -> u8 {
            Spi23sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Spi45sel {
        #[doc = "pclk2 selected as kernel clock (default after reset)."]
        Pclk2 = 0x0,
        #[doc = "pll2_q_ck is selected as kernel clock."]
        Pll2Q = 0x01,
        #[doc = "pll3_q_ck is selected as kernel clock."]
        Pll3Q = 0x02,
        #[doc = "hsi_ker_ck is selected as kernel clock."]
        Hsi = 0x03,
        #[doc = "csi_ker_ck is selected as kernel clock."]
        Csi = 0x04,
        #[doc = "hse_ker_ck is selected as kernel clock."]
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
        #[doc = "pclk4 selected as kernel peripheral clock (default after reset)."]
        Pclk4 = 0x0,
        #[doc = "pll2_q_ck selected as kernel peripheral clock."]
        Pll2Q = 0x01,
        #[doc = "pll3_q_ck selected as kernel peripheral clock."]
        Pll3Q = 0x02,
        #[doc = "hsi_ker_ck selected as kernel peripheral clock."]
        Hsi = 0x03,
        #[doc = "csi_ker_ck selected as kernel peripheral clock."]
        Csi = 0x04,
        #[doc = "hse_ker_ck selected as kernel peripheral clock."]
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
    pub enum Sw {
        #[doc = "HSI selected as system clock (hsi_ck) (default after reset)."]
        Hsi = 0x0,
        #[doc = "CSI selected as system clock (csi_ck)."]
        Csi = 0x01,
        #[doc = "HSE selected as system clock (hse_ck)."]
        Hse = 0x02,
        #[doc = "PLL1 selected as system clock (pll1_p_ck)."]
        Pll1 = 0x03,
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
    pub enum Sws {
        #[doc = "HSI used as system clock (hsi_ck) (default after reset)."]
        Hsi = 0x0,
        #[doc = "CSI used as system clock (csi_ck)."]
        Csi = 0x01,
        #[doc = "HSE used as system clock (hse_ck)."]
        Hse = 0x02,
        #[doc = "PLL1 used as system clock (pll1_p_ck)."]
        Pll1 = 0x03,
    }
    impl Sws {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sws {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sws {
        #[inline(always)]
        fn from(val: u8) -> Sws {
            Sws::from_bits(val)
        }
    }
    impl From<Sws> for u8 {
        #[inline(always)]
        fn from(val: Sws) -> u8 {
            Sws::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Uart234578sel {
        #[doc = "pclk1 selected as kernel clock (default after reset)."]
        Pclk1 = 0x0,
        #[doc = "pll2_q_ck selected as kernel clock."]
        Pll2Q = 0x01,
        #[doc = "pll3_q_ck selected as kernel clock."]
        Pll3Q = 0x02,
        #[doc = "hsi_ker_ck selected as kernel clock."]
        Hsi = 0x03,
        #[doc = "csi_ker_ck selected as kernel clock."]
        Csi = 0x04,
        #[doc = "lse_ck selected as kernel clock."]
        Lse = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Uart234578sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Uart234578sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Uart234578sel {
        #[inline(always)]
        fn from(val: u8) -> Uart234578sel {
            Uart234578sel::from_bits(val)
        }
    }
    impl From<Uart234578sel> for u8 {
        #[inline(always)]
        fn from(val: Uart234578sel) -> u8 {
            Uart234578sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Usart16910sel {
        #[doc = "pclk2 selected as kernel clock (default after reset)."]
        Pclk2 = 0x0,
        #[doc = "pll2_q_ck selected as kernel clock."]
        Pll2Q = 0x01,
        #[doc = "pll3_q_ck selected as kernel clock."]
        Pll3Q = 0x02,
        #[doc = "hsi_ker_ck selected as kernel clock."]
        Hsi = 0x03,
        #[doc = "csi_ker_ck selected as kernel clock."]
        Csi = 0x04,
        #[doc = "lse_ck selected as kernel clock."]
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
    pub enum Xspi1sel {
        #[doc = "hclk5 selected as kernel peripheral clock (default after reset)."]
        Hclk5 = 0x0,
        #[doc = "pll2_s_ck selected as kernel peripheral clock."]
        Pll2S = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Xspi1sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Xspi1sel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Xspi1sel {
        #[inline(always)]
        fn from(val: u8) -> Xspi1sel {
            Xspi1sel::from_bits(val)
        }
    }
    impl From<Xspi1sel> for u8 {
        #[inline(always)]
        fn from(val: Xspi1sel) -> u8 {
            Xspi1sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Xspiswp {
        #[doc = "The switch is in neutral mode and output clock is gated (default after reset)."]
        Neutral = 0x0,
        #[doc = "The switch is selecting hclk5."]
        Hclk5 = 0x01,
        #[doc = "The switch is selecting pll2_s_ck."]
        Pll2S = 0x02,
        #[doc = "The switch is selecting pll2_t_ck."]
        Pll2T = 0x03,
        #[doc = "The switch is in recovery position (hclk5/4)."]
        Recovery = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Xspiswp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Xspiswp {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Xspiswp {
        #[inline(always)]
        fn from(val: u8) -> Xspiswp {
            Xspiswp::from_bits(val)
        }
    }
    impl From<Xspiswp> for u8 {
        #[inline(always)]
        fn from(val: Xspiswp) -> u8 {
            Xspiswp::to_bits(val)
        }
    }
}
