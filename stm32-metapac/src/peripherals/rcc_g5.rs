#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "RCC address block description."]
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
    #[doc = "RCC clock control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "RCC HSI calibration register."]
    #[inline(always)]
    pub const fn hsicfgr(self) -> crate::common::Reg<regs::Hsicfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "RCC clock recovery RC register."]
    #[inline(always)]
    pub const fn crrcr(self) -> crate::common::Reg<regs::Crrcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "RCC clock configuration register1."]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::Cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "RCC CPU domain clock configuration register 2."]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "RCC PLL clock source selection register."]
    #[inline(always)]
    pub const fn pllcfgr(self, n: usize) -> crate::common::Reg<regs::Pll1cfgr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize + n * 4usize) as _) }
    }
    #[doc = "RCC PLL1 divider register."]
    #[inline(always)]
    pub const fn plldivr(self, n: usize) -> crate::common::Reg<regs::Pll1divr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize + n * 8usize) as _) }
    }
    #[doc = "RCC PLL1 fractional divider register."]
    #[inline(always)]
    pub const fn pllfracr(self, n: usize) -> crate::common::Reg<regs::Pll1fracr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize + n * 8usize) as _) }
    }
    #[doc = "RCC interrupt enable register."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "RCC interrupt flag register."]
    #[inline(always)]
    pub const fn ifr(self) -> crate::common::Reg<regs::Ifr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "RCC Interrupt clear register."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "RCC AHB1 reset register."]
    #[inline(always)]
    pub const fn ahb1rstr(self) -> crate::common::Reg<regs::Ahb1rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "RCC AHB2 peripheral reset register."]
    #[inline(always)]
    pub const fn ahb2rstr(self) -> crate::common::Reg<regs::Ahb2rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "RCC AHB4 peripheral reset register."]
    #[inline(always)]
    pub const fn ahb4rstr(self) -> crate::common::Reg<regs::Ahb4rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "RCC APB1 peripheral low reset register."]
    #[inline(always)]
    pub const fn apb1lrstr(self) -> crate::common::Reg<regs::Apb1lrstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "RCC APB1 peripheral high reset register."]
    #[inline(always)]
    pub const fn apb1hrstr(self) -> crate::common::Reg<regs::Apb1hrstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "RCC APB2 peripheral reset register."]
    #[inline(always)]
    pub const fn apb2rstr(self) -> crate::common::Reg<regs::Apb2rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "RCC APB3 peripheral reset register."]
    #[inline(always)]
    pub const fn apb3rstr(self) -> crate::common::Reg<regs::Apb3rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "RCC AHB1 peripheral clock enable register."]
    #[inline(always)]
    pub const fn ahb1enr(self) -> crate::common::Reg<regs::Ahb1enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "RCC AHB2 peripheral clock enable register."]
    #[inline(always)]
    pub const fn ahb2enr(self) -> crate::common::Reg<regs::Ahb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "RCC AHB3 peripheral clock enable register."]
    #[inline(always)]
    pub const fn ahb3enr(self) -> crate::common::Reg<regs::Ahb3enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "RCC AHB4 peripheral clock enable register."]
    #[inline(always)]
    pub const fn ahb4enr(self) -> crate::common::Reg<regs::Ahb4enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "RCC APB1 peripheral clock enable register."]
    #[inline(always)]
    pub const fn apb1lenr(self) -> crate::common::Reg<regs::Apb1lenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "RCC APB1 peripheral clock enable register."]
    #[inline(always)]
    pub const fn apb1henr(self) -> crate::common::Reg<regs::Apb1henr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "RCC APB2 peripheral clock enable register."]
    #[inline(always)]
    pub const fn apb2enr(self) -> crate::common::Reg<regs::Apb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "RCC APB3 peripheral clock enable register."]
    #[inline(always)]
    pub const fn apb3enr(self) -> crate::common::Reg<regs::Apb3enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "RCC AHB1 peripheral low-power clock enable register."]
    #[inline(always)]
    pub const fn ahb1lpenr(self) -> crate::common::Reg<regs::Ahb1lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "RCC AHB2 peripheral low-power clock enable register."]
    #[inline(always)]
    pub const fn ahb2lpenr(self) -> crate::common::Reg<regs::Ahb2lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "RCC AHB3 peripheral low-power clock enable register."]
    #[inline(always)]
    pub const fn ahb3lpenr(self) -> crate::common::Reg<regs::Ahb3lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "RCC AHB4 peripheral low-power clock enable register."]
    #[inline(always)]
    pub const fn ahb4lpenr(self) -> crate::common::Reg<regs::Ahb4lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "RCC APB1 peripheral low-power clock enable register."]
    #[inline(always)]
    pub const fn apb1llpenr(self) -> crate::common::Reg<regs::Apb1llpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "RCC APB1 peripheral low-power clock enable register."]
    #[inline(always)]
    pub const fn apb1hlpenr(self) -> crate::common::Reg<regs::Apb1hlpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "RCC APB2 peripheral low-power clock enable register."]
    #[inline(always)]
    pub const fn apb2lpenr(self) -> crate::common::Reg<regs::Apb2lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "RCC APB3 peripheral low-power clock enable register."]
    #[inline(always)]
    pub const fn apb3lpenr(self) -> crate::common::Reg<regs::Apb3lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "RCC kernel clock configuration register."]
    #[inline(always)]
    pub const fn ccipr1(self) -> crate::common::Reg<regs::Ccipr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "RCC kernel clock configuration register."]
    #[inline(always)]
    pub const fn ccipr2(self) -> crate::common::Reg<regs::Ccipr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "RCC kernel clock configuration register."]
    #[inline(always)]
    pub const fn ccipr3(self) -> crate::common::Reg<regs::Ccipr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "RCC kernel clock configuration register."]
    #[inline(always)]
    pub const fn ccipr4(self) -> crate::common::Reg<regs::Ccipr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "RCC kernel clock configuration register."]
    #[inline(always)]
    pub const fn ccipr5(self) -> crate::common::Reg<regs::Ccipr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "RCC RTC domain control register."]
    #[inline(always)]
    pub const fn rtccr(self) -> crate::common::Reg<regs::Rtccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "RCC reset status register."]
    #[inline(always)]
    pub const fn rsr(self) -> crate::common::Reg<regs::Rsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "RCC secure configuration register."]
    #[inline(always)]
    pub const fn seccfgr(self) -> crate::common::Reg<regs::Seccfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "RCC privilege configuration register."]
    #[inline(always)]
    pub const fn privcfgr(self) -> crate::common::Reg<regs::Privcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
}
pub mod regs {
    #[doc = "RCC AHB1 peripheral clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1enr(pub u32);
    impl Ahb1enr {
        #[doc = "GPDMA1 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn gpdma1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1 clock enable."]
        #[inline(always)]
        pub const fn set_gpdma1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPDMA2 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn gpdma2en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA2 clock enable."]
        #[inline(always)]
        pub const fn set_gpdma2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Flash memory interface clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flashen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Flash memory interface clock enable."]
        #[inline(always)]
        pub const fn set_flashen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CRC clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn crcen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC clock enable."]
        #[inline(always)]
        pub const fn set_crcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "CORDIC1 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cordic1en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "CORDIC1 clock enable."]
        #[inline(always)]
        pub const fn set_cordic1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "FMAC1 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fmac1en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "FMAC1 clock enable."]
        #[inline(always)]
        pub const fn set_fmac1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "RAMCFG clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ramcfgen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "RAMCFG clock enable."]
        #[inline(always)]
        pub const fn set_ramcfgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TZSC1 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tzsc1en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "TZSC1 clock enable."]
        #[inline(always)]
        pub const fn set_tzsc1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "SRAM2 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sram2en(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 clock enable."]
        #[inline(always)]
        pub const fn set_sram2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "SRAM3 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sram3en(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM3 clock enable."]
        #[inline(always)]
        pub const fn set_sram3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "DCACHE clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dcacheen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "DCACHE clock enable."]
        #[inline(always)]
        pub const fn set_dcacheen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SRAM1 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sram1en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 clock enable."]
        #[inline(always)]
        pub const fn set_sram1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ahb1enr {
        #[inline(always)]
        fn default() -> Ahb1enr {
            Ahb1enr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Ahb1enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb1enr")
                .field("gpdma1en", &self.gpdma1en())
                .field("gpdma2en", &self.gpdma2en())
                .field("flashen", &self.flashen())
                .field("crcen", &self.crcen())
                .field("cordic1en", &self.cordic1en())
                .field("fmac1en", &self.fmac1en())
                .field("ramcfgen", &self.ramcfgen())
                .field("tzsc1en", &self.tzsc1en())
                .field("sram2en", &self.sram2en())
                .field("sram3en", &self.sram3en())
                .field("dcacheen", &self.dcacheen())
                .field("sram1en", &self.sram1en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1enr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb1enr {{ gpdma1en: {=bool:?}, gpdma2en: {=bool:?}, flashen: {=bool:?}, crcen: {=bool:?}, cordic1en: {=bool:?}, fmac1en: {=bool:?}, ramcfgen: {=bool:?}, tzsc1en: {=bool:?}, sram2en: {=bool:?}, sram3en: {=bool:?}, dcacheen: {=bool:?}, sram1en: {=bool:?} }}",
                self.gpdma1en(),
                self.gpdma2en(),
                self.flashen(),
                self.crcen(),
                self.cordic1en(),
                self.fmac1en(),
                self.ramcfgen(),
                self.tzsc1en(),
                self.sram2en(),
                self.sram3en(),
                self.dcacheen(),
                self.sram1en()
            )
        }
    }
    #[doc = "RCC AHB1 peripheral low-power clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1lpenr(pub u32);
    impl Ahb1lpenr {
        #[doc = "GPDMA1 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn gpdma1lpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_gpdma1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPDMA2 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn gpdma2lpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA2 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_gpdma2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Flash memory interface clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn flashlpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Flash memory interface clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_flashlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CRC clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn crclpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_crclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "CORDIC1 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn cordic1lpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "CORDIC1 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_cordic1lpen(&mut self, val: bool) {
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
        #[doc = "RAMCFG clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ramcfglpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "RAMCFG clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_ramcfglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TZSC1 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tzsc1lpen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "TZSC1 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tzsc1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "SRAM2 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn sram2lpen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_sram2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "SRAM3 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn sram3lpen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM3 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_sram3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "ICACHE clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn icachelpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "ICACHE clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_icachelpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "DCACHE clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dcachelpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "DCACHE clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_dcachelpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SRAM1 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn sram1lpen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_sram1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ahb1lpenr {
        #[inline(always)]
        fn default() -> Ahb1lpenr {
            Ahb1lpenr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Ahb1lpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb1lpenr")
                .field("gpdma1lpen", &self.gpdma1lpen())
                .field("gpdma2lpen", &self.gpdma2lpen())
                .field("flashlpen", &self.flashlpen())
                .field("crclpen", &self.crclpen())
                .field("cordic1lpen", &self.cordic1lpen())
                .field("fmac1lpen", &self.fmac1lpen())
                .field("ramcfglpen", &self.ramcfglpen())
                .field("tzsc1lpen", &self.tzsc1lpen())
                .field("sram2lpen", &self.sram2lpen())
                .field("sram3lpen", &self.sram3lpen())
                .field("icachelpen", &self.icachelpen())
                .field("dcachelpen", &self.dcachelpen())
                .field("sram1lpen", &self.sram1lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1lpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb1lpenr {{ gpdma1lpen: {=bool:?}, gpdma2lpen: {=bool:?}, flashlpen: {=bool:?}, crclpen: {=bool:?}, cordic1lpen: {=bool:?}, fmac1lpen: {=bool:?}, ramcfglpen: {=bool:?}, tzsc1lpen: {=bool:?}, sram2lpen: {=bool:?}, sram3lpen: {=bool:?}, icachelpen: {=bool:?}, dcachelpen: {=bool:?}, sram1lpen: {=bool:?} }}",
                self.gpdma1lpen(),
                self.gpdma2lpen(),
                self.flashlpen(),
                self.crclpen(),
                self.cordic1lpen(),
                self.fmac1lpen(),
                self.ramcfglpen(),
                self.tzsc1lpen(),
                self.sram2lpen(),
                self.sram3lpen(),
                self.icachelpen(),
                self.dcachelpen(),
                self.sram1lpen()
            )
        }
    }
    #[doc = "RCC AHB1 reset register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1rstr(pub u32);
    impl Ahb1rstr {
        #[doc = "GPDMA1 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn gpdma1rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1 block reset."]
        #[inline(always)]
        pub const fn set_gpdma1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPDMA2 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn gpdma2rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA2 block reset."]
        #[inline(always)]
        pub const fn set_gpdma2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CRC block reset Set and cleared by software."]
        #[must_use]
        #[inline(always)]
        pub const fn crcrst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC block reset Set and cleared by software."]
        #[inline(always)]
        pub const fn set_crcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "CORDIC1 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn cordic1rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "CORDIC1 block reset."]
        #[inline(always)]
        pub const fn set_cordic1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "FMAC1 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn fmac1rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "FMAC1 block reset."]
        #[inline(always)]
        pub const fn set_fmac1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "RAMCFG block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn ramcfgrst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "RAMCFG block reset."]
        #[inline(always)]
        pub const fn set_ramcfgrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Ahb1rstr {
        #[inline(always)]
        fn default() -> Ahb1rstr {
            Ahb1rstr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Ahb1rstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb1rstr")
                .field("gpdma1rst", &self.gpdma1rst())
                .field("gpdma2rst", &self.gpdma2rst())
                .field("crcrst", &self.crcrst())
                .field("cordic1rst", &self.cordic1rst())
                .field("fmac1rst", &self.fmac1rst())
                .field("ramcfgrst", &self.ramcfgrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb1rstr {{ gpdma1rst: {=bool:?}, gpdma2rst: {=bool:?}, crcrst: {=bool:?}, cordic1rst: {=bool:?}, fmac1rst: {=bool:?}, ramcfgrst: {=bool:?} }}",
                self.gpdma1rst(),
                self.gpdma2rst(),
                self.crcrst(),
                self.cordic1rst(),
                self.fmac1rst(),
                self.ramcfgrst()
            )
        }
    }
    #[doc = "RCC AHB2 peripheral clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2enr(pub u32);
    impl Ahb2enr {
        #[doc = "DAC1 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dac1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1 clock enable."]
        #[inline(always)]
        pub const fn set_dac1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DAC2 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dac2en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DAC2 clock enable."]
        #[inline(always)]
        pub const fn set_dac2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DAC5 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dac5en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DAC5 clock enable."]
        #[inline(always)]
        pub const fn set_dac5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "DAC6 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dac6en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "DAC6 clock enable."]
        #[inline(always)]
        pub const fn set_dac6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "DAC7 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dac7en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "DAC7 clock enable."]
        #[inline(always)]
        pub const fn set_dac7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "DAC8 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dac8en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "DAC8 clock enable."]
        #[inline(always)]
        pub const fn set_dac8en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "DAC9 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dac9en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "DAC9 clock enable."]
        #[inline(always)]
        pub const fn set_dac9en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "DAC10 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dac10en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "DAC10 clock enable."]
        #[inline(always)]
        pub const fn set_dac10en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "FMAC2 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fmac2en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "FMAC2 clock enable."]
        #[inline(always)]
        pub const fn set_fmac2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "ADC1 and ADC2 peripheral clocks enable."]
        #[must_use]
        #[inline(always)]
        pub const fn adc12en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1 and ADC2 peripheral clocks enable."]
        #[inline(always)]
        pub const fn set_adc12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "ADC3 peripheral clocks enable."]
        #[must_use]
        #[inline(always)]
        pub const fn adc3en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "ADC3 peripheral clocks enable."]
        #[inline(always)]
        pub const fn set_adc3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Ahb2enr {
        #[inline(always)]
        fn default() -> Ahb2enr {
            Ahb2enr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Ahb2enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb2enr")
                .field("dac1en", &self.dac1en())
                .field("dac2en", &self.dac2en())
                .field("dac5en", &self.dac5en())
                .field("dac6en", &self.dac6en())
                .field("dac7en", &self.dac7en())
                .field("dac8en", &self.dac8en())
                .field("dac9en", &self.dac9en())
                .field("dac10en", &self.dac10en())
                .field("fmac2en", &self.fmac2en())
                .field("adc12en", &self.adc12en())
                .field("adc3en", &self.adc3en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2enr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb2enr {{ dac1en: {=bool:?}, dac2en: {=bool:?}, dac5en: {=bool:?}, dac6en: {=bool:?}, dac7en: {=bool:?}, dac8en: {=bool:?}, dac9en: {=bool:?}, dac10en: {=bool:?}, fmac2en: {=bool:?}, adc12en: {=bool:?}, adc3en: {=bool:?} }}",
                self.dac1en(),
                self.dac2en(),
                self.dac5en(),
                self.dac6en(),
                self.dac7en(),
                self.dac8en(),
                self.dac9en(),
                self.dac10en(),
                self.fmac2en(),
                self.adc12en(),
                self.adc3en()
            )
        }
    }
    #[doc = "RCC AHB2 peripheral low-power clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2lpenr(pub u32);
    impl Ahb2lpenr {
        #[doc = "DAC1 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dac1lpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_dac1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DAC2 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dac2lpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DAC2 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_dac2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DAC5 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dac5lpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DAC5 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_dac5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "DAC6 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dac6lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "DAC6 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_dac6lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "DAC7 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dac7lpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "DAC7 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_dac7lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "DAC8 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dac8lpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "DAC8 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_dac8lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "DAC9 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dac9lpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "DAC9 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_dac9lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "DAC10 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dac10lpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "DAC10 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_dac10lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "FMAC2 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn fmac2lpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "FMAC2 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_fmac2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "ADC1 and 2 peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn adc12lpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1 and 2 peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_adc12lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "ADC3 peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn adc3lpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "ADC3 peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_adc3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Ahb2lpenr {
        #[inline(always)]
        fn default() -> Ahb2lpenr {
            Ahb2lpenr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Ahb2lpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb2lpenr")
                .field("dac1lpen", &self.dac1lpen())
                .field("dac2lpen", &self.dac2lpen())
                .field("dac5lpen", &self.dac5lpen())
                .field("dac6lpen", &self.dac6lpen())
                .field("dac7lpen", &self.dac7lpen())
                .field("dac8lpen", &self.dac8lpen())
                .field("dac9lpen", &self.dac9lpen())
                .field("dac10lpen", &self.dac10lpen())
                .field("fmac2lpen", &self.fmac2lpen())
                .field("adc12lpen", &self.adc12lpen())
                .field("adc3lpen", &self.adc3lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2lpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb2lpenr {{ dac1lpen: {=bool:?}, dac2lpen: {=bool:?}, dac5lpen: {=bool:?}, dac6lpen: {=bool:?}, dac7lpen: {=bool:?}, dac8lpen: {=bool:?}, dac9lpen: {=bool:?}, dac10lpen: {=bool:?}, fmac2lpen: {=bool:?}, adc12lpen: {=bool:?}, adc3lpen: {=bool:?} }}",
                self.dac1lpen(),
                self.dac2lpen(),
                self.dac5lpen(),
                self.dac6lpen(),
                self.dac7lpen(),
                self.dac8lpen(),
                self.dac9lpen(),
                self.dac10lpen(),
                self.fmac2lpen(),
                self.adc12lpen(),
                self.adc3lpen()
            )
        }
    }
    #[doc = "RCC AHB2 peripheral reset register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2rstr(pub u32);
    impl Ahb2rstr {
        #[doc = "DAC1 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn dac1rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DAC1 block reset."]
        #[inline(always)]
        pub const fn set_dac1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "DAC2 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn dac2rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DAC2 block reset."]
        #[inline(always)]
        pub const fn set_dac2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DAC5 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn dac5rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DAC5 block reset."]
        #[inline(always)]
        pub const fn set_dac5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "DAC6 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn dac6rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "DAC6 block reset."]
        #[inline(always)]
        pub const fn set_dac6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "DAC7 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn dac7rst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "DAC7 block reset."]
        #[inline(always)]
        pub const fn set_dac7rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "DAC8 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn dac8rst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "DAC8 block reset."]
        #[inline(always)]
        pub const fn set_dac8rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "DAC9 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn dac9rst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "DAC9 block reset."]
        #[inline(always)]
        pub const fn set_dac9rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "DAC10 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn dac10rst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "DAC10 block reset."]
        #[inline(always)]
        pub const fn set_dac10rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "FMAC2 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn fmac2rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "FMAC2 block reset."]
        #[inline(always)]
        pub const fn set_fmac2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "ADC1 and 2 blocks reset."]
        #[must_use]
        #[inline(always)]
        pub const fn adc12rst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1 and 2 blocks reset."]
        #[inline(always)]
        pub const fn set_adc12rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "ADC3 blocks reset."]
        #[must_use]
        #[inline(always)]
        pub const fn adc3rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "ADC3 blocks reset."]
        #[inline(always)]
        pub const fn set_adc3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Ahb2rstr {
        #[inline(always)]
        fn default() -> Ahb2rstr {
            Ahb2rstr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Ahb2rstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb2rstr")
                .field("dac1rst", &self.dac1rst())
                .field("dac2rst", &self.dac2rst())
                .field("dac5rst", &self.dac5rst())
                .field("dac6rst", &self.dac6rst())
                .field("dac7rst", &self.dac7rst())
                .field("dac8rst", &self.dac8rst())
                .field("dac9rst", &self.dac9rst())
                .field("dac10rst", &self.dac10rst())
                .field("fmac2rst", &self.fmac2rst())
                .field("adc12rst", &self.adc12rst())
                .field("adc3rst", &self.adc3rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb2rstr {{ dac1rst: {=bool:?}, dac2rst: {=bool:?}, dac5rst: {=bool:?}, dac6rst: {=bool:?}, dac7rst: {=bool:?}, dac8rst: {=bool:?}, dac9rst: {=bool:?}, dac10rst: {=bool:?}, fmac2rst: {=bool:?}, adc12rst: {=bool:?}, adc3rst: {=bool:?} }}",
                self.dac1rst(),
                self.dac2rst(),
                self.dac5rst(),
                self.dac6rst(),
                self.dac7rst(),
                self.dac8rst(),
                self.dac9rst(),
                self.dac10rst(),
                self.fmac2rst(),
                self.adc12rst(),
                self.adc3rst()
            )
        }
    }
    #[doc = "RCC AHB3 peripheral clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3enr(pub u32);
    impl Ahb3enr {
        #[doc = "DBG clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dbgcken(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "DBG clock enable."]
        #[inline(always)]
        pub const fn set_dbgcken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Ahb3enr {
        #[inline(always)]
        fn default() -> Ahb3enr {
            Ahb3enr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Ahb3enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb3enr").field("dbgcken", &self.dbgcken()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb3enr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ahb3enr {{ dbgcken: {=bool:?} }}", self.dbgcken())
        }
    }
    #[doc = "RCC AHB3 peripheral low-power clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3lpenr(pub u32);
    impl Ahb3lpenr {
        #[doc = "DBG clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dbgcklpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "DBG clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_dbgcklpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Ahb3lpenr {
        #[inline(always)]
        fn default() -> Ahb3lpenr {
            Ahb3lpenr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Ahb3lpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb3lpenr")
                .field("dbgcklpen", &self.dbgcklpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb3lpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ahb3lpenr {{ dbgcklpen: {=bool:?} }}", self.dbgcklpen())
        }
    }
    #[doc = "RCC AHB4 peripheral clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb4enr(pub u32);
    impl Ahb4enr {
        #[doc = "AES clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn aesen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "AES clock enable."]
        #[inline(always)]
        pub const fn set_aesen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "HASH clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hashen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "HASH clock enable."]
        #[inline(always)]
        pub const fn set_hashen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "RNG clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rngen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RNG clock enable."]
        #[inline(always)]
        pub const fn set_rngen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "PKA clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pkaen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PKA clock enable."]
        #[inline(always)]
        pub const fn set_pkaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SAES clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn saesen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "SAES clock enable."]
        #[inline(always)]
        pub const fn set_saesen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "CCB clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ccben(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "CCB clock enable."]
        #[inline(always)]
        pub const fn set_ccben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "OTFDEC1 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn otfdec1en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC1 clock enable."]
        #[inline(always)]
        pub const fn set_otfdec1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "GPIOA clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioaen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOA clock enable."]
        #[inline(always)]
        pub const fn set_gpioaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "GPIOB clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioben(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOB clock enable."]
        #[inline(always)]
        pub const fn set_gpioben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "GPIOC clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiocen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOC clock enable."]
        #[inline(always)]
        pub const fn set_gpiocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "GPIOD clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioden(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOD clock enable."]
        #[inline(always)]
        pub const fn set_gpioden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "GPIOE clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioeen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOE clock enable."]
        #[inline(always)]
        pub const fn set_gpioeen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "GPIOF clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiofen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOF clock enable."]
        #[inline(always)]
        pub const fn set_gpiofen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "GPIOG clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiogen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOG clock enable."]
        #[inline(always)]
        pub const fn set_gpiogen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "GPIOH clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiohen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOH clock enable."]
        #[inline(always)]
        pub const fn set_gpiohen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "XSPI1 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn xspi1en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "XSPI1 clock enable."]
        #[inline(always)]
        pub const fn set_xspi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Ahb4enr {
        #[inline(always)]
        fn default() -> Ahb4enr {
            Ahb4enr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Ahb4enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb4enr")
                .field("aesen", &self.aesen())
                .field("hashen", &self.hashen())
                .field("rngen", &self.rngen())
                .field("pkaen", &self.pkaen())
                .field("saesen", &self.saesen())
                .field("ccben", &self.ccben())
                .field("otfdec1en", &self.otfdec1en())
                .field("gpioaen", &self.gpioaen())
                .field("gpioben", &self.gpioben())
                .field("gpiocen", &self.gpiocen())
                .field("gpioden", &self.gpioden())
                .field("gpioeen", &self.gpioeen())
                .field("gpiofen", &self.gpiofen())
                .field("gpiogen", &self.gpiogen())
                .field("gpiohen", &self.gpiohen())
                .field("xspi1en", &self.xspi1en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb4enr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb4enr {{ aesen: {=bool:?}, hashen: {=bool:?}, rngen: {=bool:?}, pkaen: {=bool:?}, saesen: {=bool:?}, ccben: {=bool:?}, otfdec1en: {=bool:?}, gpioaen: {=bool:?}, gpioben: {=bool:?}, gpiocen: {=bool:?}, gpioden: {=bool:?}, gpioeen: {=bool:?}, gpiofen: {=bool:?}, gpiogen: {=bool:?}, gpiohen: {=bool:?}, xspi1en: {=bool:?} }}",
                self.aesen(),
                self.hashen(),
                self.rngen(),
                self.pkaen(),
                self.saesen(),
                self.ccben(),
                self.otfdec1en(),
                self.gpioaen(),
                self.gpioben(),
                self.gpiocen(),
                self.gpioden(),
                self.gpioeen(),
                self.gpiofen(),
                self.gpiogen(),
                self.gpiohen(),
                self.xspi1en()
            )
        }
    }
    #[doc = "RCC AHB4 peripheral low-power clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb4lpenr(pub u32);
    impl Ahb4lpenr {
        #[doc = "AES clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn aeslpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "AES clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_aeslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "HASH clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn hashlpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "HASH clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_hashlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "RNG clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn rnglpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RNG clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_rnglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "PKA clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn pkalpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PKA clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_pkalpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SAES clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn saeslpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "SAES clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_saeslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "CCB clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ccblpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "CCB clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_ccblpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "OTFDEC1 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn otfdec1lpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC1 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_otfdec1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "GPIOA clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioalpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOA clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_gpioalpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "GPIOB clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioblpen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOB clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_gpioblpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "GPIOC clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioclpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOC clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_gpioclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "GPIOD clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiodlpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOD clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_gpiodlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "GPIOE clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioelpen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOE clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_gpioelpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "GPIOF clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioflpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOF clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_gpioflpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "GPIOG clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioglpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOG clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_gpioglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "GPIOH clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiohlpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOH clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_gpiohlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "XSPI1 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn xspi1lpen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "XSPI1 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_xspi1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Ahb4lpenr {
        #[inline(always)]
        fn default() -> Ahb4lpenr {
            Ahb4lpenr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Ahb4lpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb4lpenr")
                .field("aeslpen", &self.aeslpen())
                .field("hashlpen", &self.hashlpen())
                .field("rnglpen", &self.rnglpen())
                .field("pkalpen", &self.pkalpen())
                .field("saeslpen", &self.saeslpen())
                .field("ccblpen", &self.ccblpen())
                .field("otfdec1lpen", &self.otfdec1lpen())
                .field("gpioalpen", &self.gpioalpen())
                .field("gpioblpen", &self.gpioblpen())
                .field("gpioclpen", &self.gpioclpen())
                .field("gpiodlpen", &self.gpiodlpen())
                .field("gpioelpen", &self.gpioelpen())
                .field("gpioflpen", &self.gpioflpen())
                .field("gpioglpen", &self.gpioglpen())
                .field("gpiohlpen", &self.gpiohlpen())
                .field("xspi1lpen", &self.xspi1lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb4lpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb4lpenr {{ aeslpen: {=bool:?}, hashlpen: {=bool:?}, rnglpen: {=bool:?}, pkalpen: {=bool:?}, saeslpen: {=bool:?}, ccblpen: {=bool:?}, otfdec1lpen: {=bool:?}, gpioalpen: {=bool:?}, gpioblpen: {=bool:?}, gpioclpen: {=bool:?}, gpiodlpen: {=bool:?}, gpioelpen: {=bool:?}, gpioflpen: {=bool:?}, gpioglpen: {=bool:?}, gpiohlpen: {=bool:?}, xspi1lpen: {=bool:?} }}",
                self.aeslpen(),
                self.hashlpen(),
                self.rnglpen(),
                self.pkalpen(),
                self.saeslpen(),
                self.ccblpen(),
                self.otfdec1lpen(),
                self.gpioalpen(),
                self.gpioblpen(),
                self.gpioclpen(),
                self.gpiodlpen(),
                self.gpioelpen(),
                self.gpioflpen(),
                self.gpioglpen(),
                self.gpiohlpen(),
                self.xspi1lpen()
            )
        }
    }
    #[doc = "RCC AHB4 peripheral reset register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb4rstr(pub u32);
    impl Ahb4rstr {
        #[doc = "AES block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn aesrst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "AES block reset."]
        #[inline(always)]
        pub const fn set_aesrst(&mut self, val: bool) {
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
        #[doc = "RNG block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn rngrst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RNG block reset."]
        #[inline(always)]
        pub const fn set_rngrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "PKA block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn pkarst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PKA block reset."]
        #[inline(always)]
        pub const fn set_pkarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SAES block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn saesrst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "SAES block reset."]
        #[inline(always)]
        pub const fn set_saesrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "CCB block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn ccbrst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "CCB block reset."]
        #[inline(always)]
        pub const fn set_ccbrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "OTFDEC1 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn otfdec1rst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "OTFDEC1 block reset."]
        #[inline(always)]
        pub const fn set_otfdec1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "GPIOA block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioarst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOA block reset."]
        #[inline(always)]
        pub const fn set_gpioarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "GPIOB block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiobrst(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOB block reset."]
        #[inline(always)]
        pub const fn set_gpiobrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "GPIOC block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiocrst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOC block reset."]
        #[inline(always)]
        pub const fn set_gpiocrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "GPIOD block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiodrst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOD block reset."]
        #[inline(always)]
        pub const fn set_gpiodrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "GPIOE block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioerst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOE block reset."]
        #[inline(always)]
        pub const fn set_gpioerst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "GPIOF block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiofrst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOF block reset."]
        #[inline(always)]
        pub const fn set_gpiofrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "GPIOG block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiogrst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOG block reset."]
        #[inline(always)]
        pub const fn set_gpiogrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "GPIOH block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiohrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOH block reset."]
        #[inline(always)]
        pub const fn set_gpiohrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "XSPI1 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn xspi1rst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "XSPI1 block reset."]
        #[inline(always)]
        pub const fn set_xspi1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Ahb4rstr {
        #[inline(always)]
        fn default() -> Ahb4rstr {
            Ahb4rstr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Ahb4rstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb4rstr")
                .field("aesrst", &self.aesrst())
                .field("hashrst", &self.hashrst())
                .field("rngrst", &self.rngrst())
                .field("pkarst", &self.pkarst())
                .field("saesrst", &self.saesrst())
                .field("ccbrst", &self.ccbrst())
                .field("otfdec1rst", &self.otfdec1rst())
                .field("gpioarst", &self.gpioarst())
                .field("gpiobrst", &self.gpiobrst())
                .field("gpiocrst", &self.gpiocrst())
                .field("gpiodrst", &self.gpiodrst())
                .field("gpioerst", &self.gpioerst())
                .field("gpiofrst", &self.gpiofrst())
                .field("gpiogrst", &self.gpiogrst())
                .field("gpiohrst", &self.gpiohrst())
                .field("xspi1rst", &self.xspi1rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb4rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb4rstr {{ aesrst: {=bool:?}, hashrst: {=bool:?}, rngrst: {=bool:?}, pkarst: {=bool:?}, saesrst: {=bool:?}, ccbrst: {=bool:?}, otfdec1rst: {=bool:?}, gpioarst: {=bool:?}, gpiobrst: {=bool:?}, gpiocrst: {=bool:?}, gpiodrst: {=bool:?}, gpioerst: {=bool:?}, gpiofrst: {=bool:?}, gpiogrst: {=bool:?}, gpiohrst: {=bool:?}, xspi1rst: {=bool:?} }}",
                self.aesrst(),
                self.hashrst(),
                self.rngrst(),
                self.pkarst(),
                self.saesrst(),
                self.ccbrst(),
                self.otfdec1rst(),
                self.gpioarst(),
                self.gpiobrst(),
                self.gpiocrst(),
                self.gpiodrst(),
                self.gpioerst(),
                self.gpiofrst(),
                self.gpiogrst(),
                self.gpiohrst(),
                self.xspi1rst()
            )
        }
    }
    #[doc = "RCC APB1 peripheral clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1henr(pub u32);
    impl Apb1henr {
        #[doc = "DTS clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dtsen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DTS clock enable."]
        #[inline(always)]
        pub const fn set_dtsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "FDCAN1, FDCAN2 and FDCAN3 peripheral clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fdcanen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN1, FDCAN2 and FDCAN3 peripheral clock enable."]
        #[inline(always)]
        pub const fn set_fdcanen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "UCPD1 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ucpd1en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "UCPD1 clock enable."]
        #[inline(always)]
        pub const fn set_ucpd1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Apb1henr {
        #[inline(always)]
        fn default() -> Apb1henr {
            Apb1henr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Apb1henr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1henr")
                .field("dtsen", &self.dtsen())
                .field("fdcanen", &self.fdcanen())
                .field("ucpd1en", &self.ucpd1en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1henr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb1henr {{ dtsen: {=bool:?}, fdcanen: {=bool:?}, ucpd1en: {=bool:?} }}",
                self.dtsen(),
                self.fdcanen(),
                self.ucpd1en()
            )
        }
    }
    #[doc = "RCC APB1 peripheral low-power clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1hlpenr(pub u32);
    impl Apb1hlpenr {
        #[doc = "DTS clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dtslpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DTS clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_dtslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "FDCAN1, FDCAN2 and FDCAN3 peripheral clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn fdcanlpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN1, FDCAN2 and FDCAN3 peripheral clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_fdcanlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "UCPD1 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ucpd1lpen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "UCPD1 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_ucpd1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Apb1hlpenr {
        #[inline(always)]
        fn default() -> Apb1hlpenr {
            Apb1hlpenr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Apb1hlpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1hlpenr")
                .field("dtslpen", &self.dtslpen())
                .field("fdcanlpen", &self.fdcanlpen())
                .field("ucpd1lpen", &self.ucpd1lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1hlpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb1hlpenr {{ dtslpen: {=bool:?}, fdcanlpen: {=bool:?}, ucpd1lpen: {=bool:?} }}",
                self.dtslpen(),
                self.fdcanlpen(),
                self.ucpd1lpen()
            )
        }
    }
    #[doc = "RCC APB1 peripheral high reset register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1hrstr(pub u32);
    impl Apb1hrstr {
        #[doc = "DTS block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn dtsrst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DTS block reset."]
        #[inline(always)]
        pub const fn set_dtsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "FDCAN1, FDCAN2 and FDCAN3 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn fdcanrst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN1, FDCAN2 and FDCAN3 block reset."]
        #[inline(always)]
        pub const fn set_fdcanrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "UCPD1 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn ucpd1rst(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "UCPD1 block reset."]
        #[inline(always)]
        pub const fn set_ucpd1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Apb1hrstr {
        #[inline(always)]
        fn default() -> Apb1hrstr {
            Apb1hrstr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Apb1hrstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1hrstr")
                .field("dtsrst", &self.dtsrst())
                .field("fdcanrst", &self.fdcanrst())
                .field("ucpd1rst", &self.ucpd1rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1hrstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb1hrstr {{ dtsrst: {=bool:?}, fdcanrst: {=bool:?}, ucpd1rst: {=bool:?} }}",
                self.dtsrst(),
                self.fdcanrst(),
                self.ucpd1rst()
            )
        }
    }
    #[doc = "RCC APB1 peripheral clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1lenr(pub u32);
    impl Apb1lenr {
        #[doc = "TIM2 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim2en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 clock enable."]
        #[inline(always)]
        pub const fn set_tim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim3en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 clock enable."]
        #[inline(always)]
        pub const fn set_tim3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM6 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim6en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 clock enable."]
        #[inline(always)]
        pub const fn set_tim6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim7en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 clock enable."]
        #[inline(always)]
        pub const fn set_tim7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM9 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim9en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM9 clock enable."]
        #[inline(always)]
        pub const fn set_tim9en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIM10 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim10en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIM10 clock enable."]
        #[inline(always)]
        pub const fn set_tim10en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIM11 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim11en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM11 clock enable."]
        #[inline(always)]
        pub const fn set_tim11en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "TIM12 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim12en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12 clock enable."]
        #[inline(always)]
        pub const fn set_tim12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "TIM13 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim13en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "TIM13 clock enable."]
        #[inline(always)]
        pub const fn set_tim13en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "TIM14 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim14en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM14 clock enable."]
        #[inline(always)]
        pub const fn set_tim14en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "WWDG clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn wwdgen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG clock enable."]
        #[inline(always)]
        pub const fn set_wwdgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SPI2 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn spi2en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 clock enable."]
        #[inline(always)]
        pub const fn set_spi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn spi3en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 clock enable."]
        #[inline(always)]
        pub const fn set_spi3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "USART2 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn usart2en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 clock enable."]
        #[inline(always)]
        pub const fn set_usart2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn usart3en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 clock enable."]
        #[inline(always)]
        pub const fn set_usart3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uart4en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 clock enable."]
        #[inline(always)]
        pub const fn set_uart4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uart5en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 clock enable."]
        #[inline(always)]
        pub const fn set_uart5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 clock enable."]
        #[inline(always)]
        pub const fn set_i2c1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c2en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 clock enable."]
        #[inline(always)]
        pub const fn set_i2c2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I3C1 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn i3c1en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I3C1 clock enable."]
        #[inline(always)]
        pub const fn set_i3c1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "CRS clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn crsen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "CRS clock enable."]
        #[inline(always)]
        pub const fn set_crsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "UART6 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uart6en(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "UART6 clock enable."]
        #[inline(always)]
        pub const fn set_uart6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Apb1lenr {
        #[inline(always)]
        fn default() -> Apb1lenr {
            Apb1lenr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Apb1lenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1lenr")
                .field("tim2en", &self.tim2en())
                .field("tim3en", &self.tim3en())
                .field("tim6en", &self.tim6en())
                .field("tim7en", &self.tim7en())
                .field("tim9en", &self.tim9en())
                .field("tim10en", &self.tim10en())
                .field("tim11en", &self.tim11en())
                .field("tim12en", &self.tim12en())
                .field("tim13en", &self.tim13en())
                .field("tim14en", &self.tim14en())
                .field("wwdgen", &self.wwdgen())
                .field("spi2en", &self.spi2en())
                .field("spi3en", &self.spi3en())
                .field("usart2en", &self.usart2en())
                .field("usart3en", &self.usart3en())
                .field("uart4en", &self.uart4en())
                .field("uart5en", &self.uart5en())
                .field("i2c1en", &self.i2c1en())
                .field("i2c2en", &self.i2c2en())
                .field("i3c1en", &self.i3c1en())
                .field("crsen", &self.crsen())
                .field("uart6en", &self.uart6en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1lenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb1lenr {{ tim2en: {=bool:?}, tim3en: {=bool:?}, tim6en: {=bool:?}, tim7en: {=bool:?}, tim9en: {=bool:?}, tim10en: {=bool:?}, tim11en: {=bool:?}, tim12en: {=bool:?}, tim13en: {=bool:?}, tim14en: {=bool:?}, wwdgen: {=bool:?}, spi2en: {=bool:?}, spi3en: {=bool:?}, usart2en: {=bool:?}, usart3en: {=bool:?}, uart4en: {=bool:?}, uart5en: {=bool:?}, i2c1en: {=bool:?}, i2c2en: {=bool:?}, i3c1en: {=bool:?}, crsen: {=bool:?}, uart6en: {=bool:?} }}",
                self.tim2en(),
                self.tim3en(),
                self.tim6en(),
                self.tim7en(),
                self.tim9en(),
                self.tim10en(),
                self.tim11en(),
                self.tim12en(),
                self.tim13en(),
                self.tim14en(),
                self.wwdgen(),
                self.spi2en(),
                self.spi3en(),
                self.usart2en(),
                self.usart3en(),
                self.uart4en(),
                self.uart5en(),
                self.i2c1en(),
                self.i2c2en(),
                self.i3c1en(),
                self.crsen(),
                self.uart6en()
            )
        }
    }
    #[doc = "RCC APB1 peripheral low-power clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1llpenr(pub u32);
    impl Apb1llpenr {
        #[doc = "TIM2 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim2lpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim3lpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM6 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim6lpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim6lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim7lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim7lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM9 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim9lpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM9 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim9lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIM10 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim10lpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIM10 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim10lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIM11 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim11lpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM11 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim11lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "TIM12 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim12lpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim12lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "TIM13 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim13lpen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "TIM13 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim13lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "TIM14 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim14lpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM14 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim14lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "WWDG clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn wwdglpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_wwdglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SPI2 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn spi2lpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_spi2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn spi3lpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_spi3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "USART2 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn usart2lpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_usart2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn usart3lpen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_usart3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn uart4lpen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_uart4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn uart5lpen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_uart5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1lpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_i2c1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c2lpen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_i2c2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I3C1 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn i3c1lpen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I3C1 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_i3c1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "CRS clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn crslpen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "CRS clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_crslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "UART6 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn uart6lpen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "UART6 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_uart6lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Apb1llpenr {
        #[inline(always)]
        fn default() -> Apb1llpenr {
            Apb1llpenr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Apb1llpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1llpenr")
                .field("tim2lpen", &self.tim2lpen())
                .field("tim3lpen", &self.tim3lpen())
                .field("tim6lpen", &self.tim6lpen())
                .field("tim7lpen", &self.tim7lpen())
                .field("tim9lpen", &self.tim9lpen())
                .field("tim10lpen", &self.tim10lpen())
                .field("tim11lpen", &self.tim11lpen())
                .field("tim12lpen", &self.tim12lpen())
                .field("tim13lpen", &self.tim13lpen())
                .field("tim14lpen", &self.tim14lpen())
                .field("wwdglpen", &self.wwdglpen())
                .field("spi2lpen", &self.spi2lpen())
                .field("spi3lpen", &self.spi3lpen())
                .field("usart2lpen", &self.usart2lpen())
                .field("usart3lpen", &self.usart3lpen())
                .field("uart4lpen", &self.uart4lpen())
                .field("uart5lpen", &self.uart5lpen())
                .field("i2c1lpen", &self.i2c1lpen())
                .field("i2c2lpen", &self.i2c2lpen())
                .field("i3c1lpen", &self.i3c1lpen())
                .field("crslpen", &self.crslpen())
                .field("uart6lpen", &self.uart6lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1llpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb1llpenr {{ tim2lpen: {=bool:?}, tim3lpen: {=bool:?}, tim6lpen: {=bool:?}, tim7lpen: {=bool:?}, tim9lpen: {=bool:?}, tim10lpen: {=bool:?}, tim11lpen: {=bool:?}, tim12lpen: {=bool:?}, tim13lpen: {=bool:?}, tim14lpen: {=bool:?}, wwdglpen: {=bool:?}, spi2lpen: {=bool:?}, spi3lpen: {=bool:?}, usart2lpen: {=bool:?}, usart3lpen: {=bool:?}, uart4lpen: {=bool:?}, uart5lpen: {=bool:?}, i2c1lpen: {=bool:?}, i2c2lpen: {=bool:?}, i3c1lpen: {=bool:?}, crslpen: {=bool:?}, uart6lpen: {=bool:?} }}",
                self.tim2lpen(),
                self.tim3lpen(),
                self.tim6lpen(),
                self.tim7lpen(),
                self.tim9lpen(),
                self.tim10lpen(),
                self.tim11lpen(),
                self.tim12lpen(),
                self.tim13lpen(),
                self.tim14lpen(),
                self.wwdglpen(),
                self.spi2lpen(),
                self.spi3lpen(),
                self.usart2lpen(),
                self.usart3lpen(),
                self.uart4lpen(),
                self.uart5lpen(),
                self.i2c1lpen(),
                self.i2c2lpen(),
                self.i3c1lpen(),
                self.crslpen(),
                self.uart6lpen()
            )
        }
    }
    #[doc = "RCC APB1 peripheral low reset register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1lrstr(pub u32);
    impl Apb1lrstr {
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
        pub const fn tim9rst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12 block reset."]
        #[inline(always)]
        pub const fn set_tim9rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIM12 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tim10rst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12 block reset."]
        #[inline(always)]
        pub const fn set_tim10rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIM12 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tim11rst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12 block reset."]
        #[inline(always)]
        pub const fn set_tim11rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "TIM12 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tim12rst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12 block reset."]
        #[inline(always)]
        pub const fn set_tim12rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "TIM13 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tim13rst(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "TIM13 block reset."]
        #[inline(always)]
        pub const fn set_tim13rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "TIM14 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tim14rst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM14 block reset."]
        #[inline(always)]
        pub const fn set_tim14rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI2 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn spi2rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 block reset."]
        #[inline(always)]
        pub const fn set_spi2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn spi3rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 block reset."]
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
        #[doc = "I2C1 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 block reset."]
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
        #[doc = "I3C1 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn i3c1rst(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I3C1 block reset."]
        #[inline(always)]
        pub const fn set_i3c1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "CRS block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn crsrst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "CRS block reset."]
        #[inline(always)]
        pub const fn set_crsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "UART6 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn uart6rst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "UART6 block reset."]
        #[inline(always)]
        pub const fn set_uart6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Apb1lrstr {
        #[inline(always)]
        fn default() -> Apb1lrstr {
            Apb1lrstr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Apb1lrstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1lrstr")
                .field("tim2rst", &self.tim2rst())
                .field("tim3rst", &self.tim3rst())
                .field("tim6rst", &self.tim6rst())
                .field("tim7rst", &self.tim7rst())
                .field("tim9rst", &self.tim9rst())
                .field("tim10rst", &self.tim10rst())
                .field("tim11rst", &self.tim11rst())
                .field("tim12rst", &self.tim12rst())
                .field("tim13rst", &self.tim13rst())
                .field("tim14rst", &self.tim14rst())
                .field("spi2rst", &self.spi2rst())
                .field("spi3rst", &self.spi3rst())
                .field("usart2rst", &self.usart2rst())
                .field("usart3rst", &self.usart3rst())
                .field("uart4rst", &self.uart4rst())
                .field("uart5rst", &self.uart5rst())
                .field("i2c1rst", &self.i2c1rst())
                .field("i2c2rst", &self.i2c2rst())
                .field("i3c1rst", &self.i3c1rst())
                .field("crsrst", &self.crsrst())
                .field("uart6rst", &self.uart6rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1lrstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb1lrstr {{ tim2rst: {=bool:?}, tim3rst: {=bool:?}, tim6rst: {=bool:?}, tim7rst: {=bool:?}, tim9rst: {=bool:?}, tim10rst: {=bool:?}, tim11rst: {=bool:?}, tim12rst: {=bool:?}, tim13rst: {=bool:?}, tim14rst: {=bool:?}, spi2rst: {=bool:?}, spi3rst: {=bool:?}, usart2rst: {=bool:?}, usart3rst: {=bool:?}, uart4rst: {=bool:?}, uart5rst: {=bool:?}, i2c1rst: {=bool:?}, i2c2rst: {=bool:?}, i3c1rst: {=bool:?}, crsrst: {=bool:?}, uart6rst: {=bool:?} }}",
                self.tim2rst(),
                self.tim3rst(),
                self.tim6rst(),
                self.tim7rst(),
                self.tim9rst(),
                self.tim10rst(),
                self.tim11rst(),
                self.tim12rst(),
                self.tim13rst(),
                self.tim14rst(),
                self.spi2rst(),
                self.spi3rst(),
                self.usart2rst(),
                self.usart3rst(),
                self.uart4rst(),
                self.uart5rst(),
                self.i2c1rst(),
                self.i2c2rst(),
                self.i3c1rst(),
                self.crsrst(),
                self.uart6rst()
            )
        }
    }
    #[doc = "RCC APB2 peripheral clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2enr(pub u32);
    impl Apb2enr {
        #[doc = "HRTIM1 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hrtim1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM1 clock enable."]
        #[inline(always)]
        pub const fn set_hrtim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "PLAY1 APB clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn play1apben(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PLAY1 APB clock enable."]
        #[inline(always)]
        pub const fn set_play1apben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "PLAY1 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn play1en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PLAY1 clock enable."]
        #[inline(always)]
        pub const fn set_play1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM1 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim1en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 clock enable."]
        #[inline(always)]
        pub const fn set_tim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI1 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn spi1en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 clock enable."]
        #[inline(always)]
        pub const fn set_spi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TIM8 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim8en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 clock enable."]
        #[inline(always)]
        pub const fn set_tim8en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "USART1 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn usart1en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 clock enable."]
        #[inline(always)]
        pub const fn set_usart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TIM15 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim15en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 clock enable."]
        #[inline(always)]
        pub const fn set_tim15en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim16en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 clock enable."]
        #[inline(always)]
        pub const fn set_tim16en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim17en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 clock enable."]
        #[inline(always)]
        pub const fn set_tim17en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "USB clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn usben(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "USB clock enable."]
        #[inline(always)]
        pub const fn set_usben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "TIM4 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim4en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 clock enable."]
        #[inline(always)]
        pub const fn set_tim4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "TIM5 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim5en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 clock enable."]
        #[inline(always)]
        pub const fn set_tim5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Apb2enr {
        #[inline(always)]
        fn default() -> Apb2enr {
            Apb2enr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Apb2enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2enr")
                .field("hrtim1en", &self.hrtim1en())
                .field("play1apben", &self.play1apben())
                .field("play1en", &self.play1en())
                .field("tim1en", &self.tim1en())
                .field("spi1en", &self.spi1en())
                .field("tim8en", &self.tim8en())
                .field("usart1en", &self.usart1en())
                .field("tim15en", &self.tim15en())
                .field("tim16en", &self.tim16en())
                .field("tim17en", &self.tim17en())
                .field("usben", &self.usben())
                .field("tim4en", &self.tim4en())
                .field("tim5en", &self.tim5en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2enr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2enr {{ hrtim1en: {=bool:?}, play1apben: {=bool:?}, play1en: {=bool:?}, tim1en: {=bool:?}, spi1en: {=bool:?}, tim8en: {=bool:?}, usart1en: {=bool:?}, tim15en: {=bool:?}, tim16en: {=bool:?}, tim17en: {=bool:?}, usben: {=bool:?}, tim4en: {=bool:?}, tim5en: {=bool:?} }}",
                self.hrtim1en(),
                self.play1apben(),
                self.play1en(),
                self.tim1en(),
                self.spi1en(),
                self.tim8en(),
                self.usart1en(),
                self.tim15en(),
                self.tim16en(),
                self.tim17en(),
                self.usben(),
                self.tim4en(),
                self.tim5en()
            )
        }
    }
    #[doc = "RCC APB2 peripheral low-power clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2lpenr(pub u32);
    impl Apb2lpenr {
        #[doc = "HRTIM1 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn hrtim1lpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM1 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_hrtim1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "PLAY1 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn play1lpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PLAY1 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_play1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM1 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim1lpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI1 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn spi1lpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_spi1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TIM8 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim8lpen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim8lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "USART1 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn usart1lpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_usart1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TIM15 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim15lpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim15lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim16lpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim16lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim17lpen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim17lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "USB clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn usblpen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "USB clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_usblpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "TIM4 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim4lpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "TIM5 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim5lpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Apb2lpenr {
        #[inline(always)]
        fn default() -> Apb2lpenr {
            Apb2lpenr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Apb2lpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2lpenr")
                .field("hrtim1lpen", &self.hrtim1lpen())
                .field("play1lpen", &self.play1lpen())
                .field("tim1lpen", &self.tim1lpen())
                .field("spi1lpen", &self.spi1lpen())
                .field("tim8lpen", &self.tim8lpen())
                .field("usart1lpen", &self.usart1lpen())
                .field("tim15lpen", &self.tim15lpen())
                .field("tim16lpen", &self.tim16lpen())
                .field("tim17lpen", &self.tim17lpen())
                .field("usblpen", &self.usblpen())
                .field("tim4lpen", &self.tim4lpen())
                .field("tim5lpen", &self.tim5lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2lpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2lpenr {{ hrtim1lpen: {=bool:?}, play1lpen: {=bool:?}, tim1lpen: {=bool:?}, spi1lpen: {=bool:?}, tim8lpen: {=bool:?}, usart1lpen: {=bool:?}, tim15lpen: {=bool:?}, tim16lpen: {=bool:?}, tim17lpen: {=bool:?}, usblpen: {=bool:?}, tim4lpen: {=bool:?}, tim5lpen: {=bool:?} }}",
                self.hrtim1lpen(),
                self.play1lpen(),
                self.tim1lpen(),
                self.spi1lpen(),
                self.tim8lpen(),
                self.usart1lpen(),
                self.tim15lpen(),
                self.tim16lpen(),
                self.tim17lpen(),
                self.usblpen(),
                self.tim4lpen(),
                self.tim5lpen()
            )
        }
    }
    #[doc = "RCC APB2 peripheral reset register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2rstr(pub u32);
    impl Apb2rstr {
        #[doc = "HRTIM1 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn hrtim1rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM1 block reset."]
        #[inline(always)]
        pub const fn set_hrtim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "PLAY1 global block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn play1rst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PLAY1 global block reset."]
        #[inline(always)]
        pub const fn set_play1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "PLAY1 application block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn play1apprst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PLAY1 application block reset."]
        #[inline(always)]
        pub const fn set_play1apprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM1 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tim1rst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 block reset."]
        #[inline(always)]
        pub const fn set_tim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI1 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn spi1rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 block reset."]
        #[inline(always)]
        pub const fn set_spi1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TIM8 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tim8rst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 block reset."]
        #[inline(always)]
        pub const fn set_tim8rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "USART1 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn usart1rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 block reset."]
        #[inline(always)]
        pub const fn set_usart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
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
        #[doc = "USB block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn usbrst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "USB block reset."]
        #[inline(always)]
        pub const fn set_usbrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "TIM4 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tim4rst(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 block reset."]
        #[inline(always)]
        pub const fn set_tim4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "TIM5 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tim5rst(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 block reset."]
        #[inline(always)]
        pub const fn set_tim5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Apb2rstr {
        #[inline(always)]
        fn default() -> Apb2rstr {
            Apb2rstr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Apb2rstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2rstr")
                .field("hrtim1rst", &self.hrtim1rst())
                .field("play1rst", &self.play1rst())
                .field("play1apprst", &self.play1apprst())
                .field("tim1rst", &self.tim1rst())
                .field("spi1rst", &self.spi1rst())
                .field("tim8rst", &self.tim8rst())
                .field("usart1rst", &self.usart1rst())
                .field("tim15rst", &self.tim15rst())
                .field("tim16rst", &self.tim16rst())
                .field("tim17rst", &self.tim17rst())
                .field("usbrst", &self.usbrst())
                .field("tim4rst", &self.tim4rst())
                .field("tim5rst", &self.tim5rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2rstr {{ hrtim1rst: {=bool:?}, play1rst: {=bool:?}, play1apprst: {=bool:?}, tim1rst: {=bool:?}, spi1rst: {=bool:?}, tim8rst: {=bool:?}, usart1rst: {=bool:?}, tim15rst: {=bool:?}, tim16rst: {=bool:?}, tim17rst: {=bool:?}, usbrst: {=bool:?}, tim4rst: {=bool:?}, tim5rst: {=bool:?} }}",
                self.hrtim1rst(),
                self.play1rst(),
                self.play1apprst(),
                self.tim1rst(),
                self.spi1rst(),
                self.tim8rst(),
                self.usart1rst(),
                self.tim15rst(),
                self.tim16rst(),
                self.tim17rst(),
                self.usbrst(),
                self.tim4rst(),
                self.tim5rst()
            )
        }
    }
    #[doc = "RCC APB3 peripheral clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb3enr(pub u32);
    impl Apb3enr {
        #[doc = "SBS clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sbsen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SBS clock enable."]
        #[inline(always)]
        pub const fn set_sbsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM20 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim20en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM20 clock enable."]
        #[inline(always)]
        pub const fn set_tim20en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM26 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim26en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM26 clock enable."]
        #[inline(always)]
        pub const fn set_tim26en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM27 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tim27en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM27 clock enable."]
        #[inline(always)]
        pub const fn set_tim27en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "LPUART1 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lpuart1en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 clock enable."]
        #[inline(always)]
        pub const fn set_lpuart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I2C3 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c3en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 clock enable."]
        #[inline(always)]
        pub const fn set_i2c3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "I2C4 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c4en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 clock enable."]
        #[inline(always)]
        pub const fn set_i2c4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LPTIM1 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim1en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 clock enable."]
        #[inline(always)]
        pub const fn set_lptim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "POSENC1 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn posenc1en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "POSENC1 clock enable."]
        #[inline(always)]
        pub const fn set_posenc1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "POSENC2 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn posenc2en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "POSENC2 clock enable."]
        #[inline(always)]
        pub const fn set_posenc2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "OPAMP1 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn opamp1en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP1 clock enable."]
        #[inline(always)]
        pub const fn set_opamp1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "OPAMP2 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn opamp2en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP2 clock enable."]
        #[inline(always)]
        pub const fn set_opamp2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "OPAMP3 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn opamp3en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP3 clock enable."]
        #[inline(always)]
        pub const fn set_opamp3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "VREFBUF clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn vrefen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "VREFBUF clock enable."]
        #[inline(always)]
        pub const fn set_vrefen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "RTC APB interface clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rtcapben(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "RTC APB interface clock enable."]
        #[inline(always)]
        pub const fn set_rtcapben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "COMP1 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn comp1en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "COMP1 clock enable."]
        #[inline(always)]
        pub const fn set_comp1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "COMP2 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn comp2en(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "COMP2 clock enable."]
        #[inline(always)]
        pub const fn set_comp2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "COMP3 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn comp3en(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "COMP3 clock enable."]
        #[inline(always)]
        pub const fn set_comp3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "COMP4 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn comp4en(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "COMP4 clock enable."]
        #[inline(always)]
        pub const fn set_comp4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "COMP5 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn comp5en(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "COMP5 clock enable."]
        #[inline(always)]
        pub const fn set_comp5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "COMP6 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn comp6en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "COMP6 clock enable."]
        #[inline(always)]
        pub const fn set_comp6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "COMP7 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn comp7en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "COMP7 clock enable."]
        #[inline(always)]
        pub const fn set_comp7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "COMP8 clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn comp8en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "COMP8 clock enable."]
        #[inline(always)]
        pub const fn set_comp8en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Apb3enr {
        #[inline(always)]
        fn default() -> Apb3enr {
            Apb3enr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Apb3enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb3enr")
                .field("sbsen", &self.sbsen())
                .field("tim20en", &self.tim20en())
                .field("tim26en", &self.tim26en())
                .field("tim27en", &self.tim27en())
                .field("lpuart1en", &self.lpuart1en())
                .field("i2c3en", &self.i2c3en())
                .field("i2c4en", &self.i2c4en())
                .field("lptim1en", &self.lptim1en())
                .field("posenc1en", &self.posenc1en())
                .field("posenc2en", &self.posenc2en())
                .field("opamp1en", &self.opamp1en())
                .field("opamp2en", &self.opamp2en())
                .field("opamp3en", &self.opamp3en())
                .field("vrefen", &self.vrefen())
                .field("rtcapben", &self.rtcapben())
                .field("comp1en", &self.comp1en())
                .field("comp2en", &self.comp2en())
                .field("comp3en", &self.comp3en())
                .field("comp4en", &self.comp4en())
                .field("comp5en", &self.comp5en())
                .field("comp6en", &self.comp6en())
                .field("comp7en", &self.comp7en())
                .field("comp8en", &self.comp8en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb3enr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb3enr {{ sbsen: {=bool:?}, tim20en: {=bool:?}, tim26en: {=bool:?}, tim27en: {=bool:?}, lpuart1en: {=bool:?}, i2c3en: {=bool:?}, i2c4en: {=bool:?}, lptim1en: {=bool:?}, posenc1en: {=bool:?}, posenc2en: {=bool:?}, opamp1en: {=bool:?}, opamp2en: {=bool:?}, opamp3en: {=bool:?}, vrefen: {=bool:?}, rtcapben: {=bool:?}, comp1en: {=bool:?}, comp2en: {=bool:?}, comp3en: {=bool:?}, comp4en: {=bool:?}, comp5en: {=bool:?}, comp6en: {=bool:?}, comp7en: {=bool:?}, comp8en: {=bool:?} }}",
                self.sbsen(),
                self.tim20en(),
                self.tim26en(),
                self.tim27en(),
                self.lpuart1en(),
                self.i2c3en(),
                self.i2c4en(),
                self.lptim1en(),
                self.posenc1en(),
                self.posenc2en(),
                self.opamp1en(),
                self.opamp2en(),
                self.opamp3en(),
                self.vrefen(),
                self.rtcapben(),
                self.comp1en(),
                self.comp2en(),
                self.comp3en(),
                self.comp4en(),
                self.comp5en(),
                self.comp6en(),
                self.comp7en(),
                self.comp8en()
            )
        }
    }
    #[doc = "RCC APB3 peripheral low-power clock enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb3lpenr(pub u32);
    impl Apb3lpenr {
        #[doc = "SBS clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn sbslpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SBS clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_sbslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM20 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim20lpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM20 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim20lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM26 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim26lpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM26 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim26lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM27 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn tim27lpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM27 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_tim27lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "LPUART1 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn lpuart1lpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_lpuart1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I2C3 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c3lpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_i2c3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "I2C4 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c4lpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_i2c4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LPTIM1 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim1lpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_lptim1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "POSENC1 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn posenc1lpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "POSENC1 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_posenc1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "POSENC2 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn posenc2lpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "POSENC2 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_posenc2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "OPAMP1 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn opamp1lpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP1 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_opamp1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "OPAMP2 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn opamp2lpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP2 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_opamp2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "OPAMP3 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn opamp3lpen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP3 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_opamp3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "VREFBUF clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn vreflpen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "VREFBUF clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_vreflpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "RTC APB interface clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn rtcapblpen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "RTC APB interface clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_rtcapblpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "COMP1 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn comp1lpen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "COMP1 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_comp1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "COMP2 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn comp2lpen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "COMP2 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_comp2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "COMP3 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn comp3lpen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "COMP3 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_comp3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "COMP4 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn comp4lpen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "COMP4 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_comp4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "COMP5 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn comp5lpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "COMP5 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_comp5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "COMP6 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn comp6lpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "COMP6 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_comp6lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "COMP7 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn comp7lpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "COMP7 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_comp7lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "COMP8 clock enable in low-power mode."]
        #[must_use]
        #[inline(always)]
        pub const fn comp8lpen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "COMP8 clock enable in low-power mode."]
        #[inline(always)]
        pub const fn set_comp8lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Apb3lpenr {
        #[inline(always)]
        fn default() -> Apb3lpenr {
            Apb3lpenr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Apb3lpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb3lpenr")
                .field("sbslpen", &self.sbslpen())
                .field("tim20lpen", &self.tim20lpen())
                .field("tim26lpen", &self.tim26lpen())
                .field("tim27lpen", &self.tim27lpen())
                .field("lpuart1lpen", &self.lpuart1lpen())
                .field("i2c3lpen", &self.i2c3lpen())
                .field("i2c4lpen", &self.i2c4lpen())
                .field("lptim1lpen", &self.lptim1lpen())
                .field("posenc1lpen", &self.posenc1lpen())
                .field("posenc2lpen", &self.posenc2lpen())
                .field("opamp1lpen", &self.opamp1lpen())
                .field("opamp2lpen", &self.opamp2lpen())
                .field("opamp3lpen", &self.opamp3lpen())
                .field("vreflpen", &self.vreflpen())
                .field("rtcapblpen", &self.rtcapblpen())
                .field("comp1lpen", &self.comp1lpen())
                .field("comp2lpen", &self.comp2lpen())
                .field("comp3lpen", &self.comp3lpen())
                .field("comp4lpen", &self.comp4lpen())
                .field("comp5lpen", &self.comp5lpen())
                .field("comp6lpen", &self.comp6lpen())
                .field("comp7lpen", &self.comp7lpen())
                .field("comp8lpen", &self.comp8lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb3lpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb3lpenr {{ sbslpen: {=bool:?}, tim20lpen: {=bool:?}, tim26lpen: {=bool:?}, tim27lpen: {=bool:?}, lpuart1lpen: {=bool:?}, i2c3lpen: {=bool:?}, i2c4lpen: {=bool:?}, lptim1lpen: {=bool:?}, posenc1lpen: {=bool:?}, posenc2lpen: {=bool:?}, opamp1lpen: {=bool:?}, opamp2lpen: {=bool:?}, opamp3lpen: {=bool:?}, vreflpen: {=bool:?}, rtcapblpen: {=bool:?}, comp1lpen: {=bool:?}, comp2lpen: {=bool:?}, comp3lpen: {=bool:?}, comp4lpen: {=bool:?}, comp5lpen: {=bool:?}, comp6lpen: {=bool:?}, comp7lpen: {=bool:?}, comp8lpen: {=bool:?} }}",
                self.sbslpen(),
                self.tim20lpen(),
                self.tim26lpen(),
                self.tim27lpen(),
                self.lpuart1lpen(),
                self.i2c3lpen(),
                self.i2c4lpen(),
                self.lptim1lpen(),
                self.posenc1lpen(),
                self.posenc2lpen(),
                self.opamp1lpen(),
                self.opamp2lpen(),
                self.opamp3lpen(),
                self.vreflpen(),
                self.rtcapblpen(),
                self.comp1lpen(),
                self.comp2lpen(),
                self.comp3lpen(),
                self.comp4lpen(),
                self.comp5lpen(),
                self.comp6lpen(),
                self.comp7lpen(),
                self.comp8lpen()
            )
        }
    }
    #[doc = "RCC APB3 peripheral reset register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb3rstr(pub u32);
    impl Apb3rstr {
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
        #[doc = "TIM26 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tim26rst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM26 block reset."]
        #[inline(always)]
        pub const fn set_tim26rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM27 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn tim27rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM27 block reset."]
        #[inline(always)]
        pub const fn set_tim27rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "LPUART1 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn lpuart1rst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 block reset."]
        #[inline(always)]
        pub const fn set_lpuart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I2C3 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c3rst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 block reset."]
        #[inline(always)]
        pub const fn set_i2c3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "I2C4 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c4rst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 block reset."]
        #[inline(always)]
        pub const fn set_i2c4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LPTIM1 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim1rst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 block reset."]
        #[inline(always)]
        pub const fn set_lptim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "POSENC1 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn posenc1rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "POSENC1 block reset."]
        #[inline(always)]
        pub const fn set_posenc1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "POSENC2 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn posenc2rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "POSENC2 block reset."]
        #[inline(always)]
        pub const fn set_posenc2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "OPAMP1 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn opamp1rst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP1 block reset."]
        #[inline(always)]
        pub const fn set_opamp1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "OPAMP2 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn opamp2rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP2 block reset."]
        #[inline(always)]
        pub const fn set_opamp2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "OPAMP3 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn opamp3rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP3 block reset."]
        #[inline(always)]
        pub const fn set_opamp3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "VREFBUF block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn vrefrst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "VREFBUF block reset."]
        #[inline(always)]
        pub const fn set_vrefrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "COMP1 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn comp1rst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "COMP1 block reset."]
        #[inline(always)]
        pub const fn set_comp1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "COMP2 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn comp2rst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "COMP2 block reset."]
        #[inline(always)]
        pub const fn set_comp2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "COMP3 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn comp3rst(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "COMP3 block reset."]
        #[inline(always)]
        pub const fn set_comp3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "COMP4 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn comp4rst(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "COMP4 block reset."]
        #[inline(always)]
        pub const fn set_comp4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "COMP5 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn comp5rst(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "COMP5 block reset."]
        #[inline(always)]
        pub const fn set_comp5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "COMP6 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn comp6rst(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "COMP6 block reset."]
        #[inline(always)]
        pub const fn set_comp6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "COMP7 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn comp7rst(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "COMP7 block reset."]
        #[inline(always)]
        pub const fn set_comp7rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "COMP8 block reset."]
        #[must_use]
        #[inline(always)]
        pub const fn comp8rst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "COMP8 block reset."]
        #[inline(always)]
        pub const fn set_comp8rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Apb3rstr {
        #[inline(always)]
        fn default() -> Apb3rstr {
            Apb3rstr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Apb3rstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb3rstr")
                .field("tim20rst", &self.tim20rst())
                .field("tim26rst", &self.tim26rst())
                .field("tim27rst", &self.tim27rst())
                .field("lpuart1rst", &self.lpuart1rst())
                .field("i2c3rst", &self.i2c3rst())
                .field("i2c4rst", &self.i2c4rst())
                .field("lptim1rst", &self.lptim1rst())
                .field("posenc1rst", &self.posenc1rst())
                .field("posenc2rst", &self.posenc2rst())
                .field("opamp1rst", &self.opamp1rst())
                .field("opamp2rst", &self.opamp2rst())
                .field("opamp3rst", &self.opamp3rst())
                .field("vrefrst", &self.vrefrst())
                .field("comp1rst", &self.comp1rst())
                .field("comp2rst", &self.comp2rst())
                .field("comp3rst", &self.comp3rst())
                .field("comp4rst", &self.comp4rst())
                .field("comp5rst", &self.comp5rst())
                .field("comp6rst", &self.comp6rst())
                .field("comp7rst", &self.comp7rst())
                .field("comp8rst", &self.comp8rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb3rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb3rstr {{ tim20rst: {=bool:?}, tim26rst: {=bool:?}, tim27rst: {=bool:?}, lpuart1rst: {=bool:?}, i2c3rst: {=bool:?}, i2c4rst: {=bool:?}, lptim1rst: {=bool:?}, posenc1rst: {=bool:?}, posenc2rst: {=bool:?}, opamp1rst: {=bool:?}, opamp2rst: {=bool:?}, opamp3rst: {=bool:?}, vrefrst: {=bool:?}, comp1rst: {=bool:?}, comp2rst: {=bool:?}, comp3rst: {=bool:?}, comp4rst: {=bool:?}, comp5rst: {=bool:?}, comp6rst: {=bool:?}, comp7rst: {=bool:?}, comp8rst: {=bool:?} }}",
                self.tim20rst(),
                self.tim26rst(),
                self.tim27rst(),
                self.lpuart1rst(),
                self.i2c3rst(),
                self.i2c4rst(),
                self.lptim1rst(),
                self.posenc1rst(),
                self.posenc2rst(),
                self.opamp1rst(),
                self.opamp2rst(),
                self.opamp3rst(),
                self.vrefrst(),
                self.comp1rst(),
                self.comp2rst(),
                self.comp3rst(),
                self.comp4rst(),
                self.comp5rst(),
                self.comp6rst(),
                self.comp7rst(),
                self.comp8rst()
            )
        }
    }
    #[doc = "RCC kernel clock configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccipr1(pub u32);
    impl Ccipr1 {
        #[doc = "USART1 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn usart1sel(&self) -> super::vals::Usartsel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Usartsel::from_bits(val as u8)
        }
        #[doc = "USART1 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_usart1sel(&mut self, val: super::vals::Usartsel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "USART2 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn usart2sel(&self) -> super::vals::Usart2sel {
            let val = (self.0 >> 3usize) & 0x07;
            super::vals::Usart2sel::from_bits(val as u8)
        }
        #[doc = "USART2 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_usart2sel(&mut self, val: super::vals::Usart2sel) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
        }
        #[doc = "USART3 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn usart3sel(&self) -> super::vals::Usart2sel {
            let val = (self.0 >> 6usize) & 0x07;
            super::vals::Usart2sel::from_bits(val as u8)
        }
        #[doc = "USART3 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_usart3sel(&mut self, val: super::vals::Usart2sel) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val.to_bits() as u32) & 0x07) << 6usize);
        }
        #[doc = "UART4 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn uart4sel(&self) -> super::vals::Uartsel {
            let val = (self.0 >> 9usize) & 0x07;
            super::vals::Uartsel::from_bits(val as u8)
        }
        #[doc = "UART4 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_uart4sel(&mut self, val: super::vals::Uartsel) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val.to_bits() as u32) & 0x07) << 9usize);
        }
        #[doc = "UART5 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn uart5sel(&self) -> super::vals::Uartsel {
            let val = (self.0 >> 12usize) & 0x07;
            super::vals::Uartsel::from_bits(val as u8)
        }
        #[doc = "UART5 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_uart5sel(&mut self, val: super::vals::Uartsel) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
        }
        #[doc = "UART6 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn uart6sel(&self) -> super::vals::Uartsel {
            let val = (self.0 >> 15usize) & 0x07;
            super::vals::Uartsel::from_bits(val as u8)
        }
        #[doc = "UART6 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_uart6sel(&mut self, val: super::vals::Uartsel) {
            self.0 = (self.0 & !(0x07 << 15usize)) | (((val.to_bits() as u32) & 0x07) << 15usize);
        }
        #[doc = "LPUART1 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn lpuart1sel(&self) -> super::vals::Lpuartsel {
            let val = (self.0 >> 24usize) & 0x07;
            super::vals::Lpuartsel::from_bits(val as u8)
        }
        #[doc = "LPUART1 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_lpuart1sel(&mut self, val: super::vals::Lpuartsel) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
        }
        #[doc = "Calibration clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn calcken(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Calibration clock enable."]
        #[inline(always)]
        pub const fn set_calcken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ccipr1 {
        #[inline(always)]
        fn default() -> Ccipr1 {
            Ccipr1(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Ccipr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccipr1")
                .field("usart1sel", &self.usart1sel())
                .field("usart2sel", &self.usart2sel())
                .field("usart3sel", &self.usart3sel())
                .field("uart4sel", &self.uart4sel())
                .field("uart5sel", &self.uart5sel())
                .field("uart6sel", &self.uart6sel())
                .field("lpuart1sel", &self.lpuart1sel())
                .field("calcken", &self.calcken())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccipr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccipr1 {{ usart1sel: {:?}, usart2sel: {:?}, usart3sel: {:?}, uart4sel: {:?}, uart5sel: {:?}, uart6sel: {:?}, lpuart1sel: {:?}, calcken: {=bool:?} }}",
                self.usart1sel(),
                self.usart2sel(),
                self.usart3sel(),
                self.uart4sel(),
                self.uart5sel(),
                self.uart6sel(),
                self.lpuart1sel(),
                self.calcken()
            )
        }
    }
    #[doc = "RCC kernel clock configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccipr2(pub u32);
    impl Ccipr2 {
        #[doc = "DAC1 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn dac1sel(&self) -> super::vals::Dacsel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Dacsel::from_bits(val as u8)
        }
        #[doc = "DAC1 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_dac1sel(&mut self, val: super::vals::Dacsel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "DAC1 sample and hold clock."]
        #[must_use]
        #[inline(always)]
        pub const fn dac1shsel(&self) -> super::vals::Dacshsel {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Dacshsel::from_bits(val as u8)
        }
        #[doc = "DAC1 sample and hold clock."]
        #[inline(always)]
        pub const fn set_dac1shsel(&mut self, val: super::vals::Dacshsel) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "DAC2 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn dac2sel(&self) -> super::vals::Dacsel {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Dacsel::from_bits(val as u8)
        }
        #[doc = "DAC2 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_dac2sel(&mut self, val: super::vals::Dacsel) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "DAC2 sample and hold clock."]
        #[must_use]
        #[inline(always)]
        pub const fn dac2shsel(&self) -> super::vals::Dacshsel {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Dacshsel::from_bits(val as u8)
        }
        #[doc = "DAC2 sample and hold clock."]
        #[inline(always)]
        pub const fn set_dac2shsel(&mut self, val: super::vals::Dacshsel) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "DAC5 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn dac5sel(&self) -> super::vals::Dacsel {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Dacsel::from_bits(val as u8)
        }
        #[doc = "DAC5 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_dac5sel(&mut self, val: super::vals::Dacsel) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "DAC6 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn dac6sel(&self) -> super::vals::Dacsel {
            let val = (self.0 >> 12usize) & 0x07;
            super::vals::Dacsel::from_bits(val as u8)
        }
        #[doc = "DAC6 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_dac6sel(&mut self, val: super::vals::Dacsel) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
        }
        #[doc = "DAC7 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn dac7sel(&self) -> super::vals::Dacsel {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::Dacsel::from_bits(val as u8)
        }
        #[doc = "DAC7 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_dac7sel(&mut self, val: super::vals::Dacsel) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
        #[doc = "DAC8 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn dac8sel(&self) -> super::vals::Dacsel {
            let val = (self.0 >> 20usize) & 0x07;
            super::vals::Dacsel::from_bits(val as u8)
        }
        #[doc = "DAC8 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_dac8sel(&mut self, val: super::vals::Dacsel) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
        }
        #[doc = "DAC9 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn dac9sel(&self) -> super::vals::Dacsel {
            let val = (self.0 >> 24usize) & 0x07;
            super::vals::Dacsel::from_bits(val as u8)
        }
        #[doc = "DAC9 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_dac9sel(&mut self, val: super::vals::Dacsel) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
        }
        #[doc = "DAC10 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn dac10sel(&self) -> super::vals::Dacsel {
            let val = (self.0 >> 28usize) & 0x07;
            super::vals::Dacsel::from_bits(val as u8)
        }
        #[doc = "DAC10 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_dac10sel(&mut self, val: super::vals::Dacsel) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Ccipr2 {
        #[inline(always)]
        fn default() -> Ccipr2 {
            Ccipr2(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Ccipr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccipr2")
                .field("dac1sel", &self.dac1sel())
                .field("dac1shsel", &self.dac1shsel())
                .field("dac2sel", &self.dac2sel())
                .field("dac2shsel", &self.dac2shsel())
                .field("dac5sel", &self.dac5sel())
                .field("dac6sel", &self.dac6sel())
                .field("dac7sel", &self.dac7sel())
                .field("dac8sel", &self.dac8sel())
                .field("dac9sel", &self.dac9sel())
                .field("dac10sel", &self.dac10sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccipr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccipr2 {{ dac1sel: {:?}, dac1shsel: {:?}, dac2sel: {:?}, dac2shsel: {:?}, dac5sel: {:?}, dac6sel: {:?}, dac7sel: {:?}, dac8sel: {:?}, dac9sel: {:?}, dac10sel: {:?} }}",
                self.dac1sel(),
                self.dac1shsel(),
                self.dac2sel(),
                self.dac2shsel(),
                self.dac5sel(),
                self.dac6sel(),
                self.dac7sel(),
                self.dac8sel(),
                self.dac9sel(),
                self.dac10sel()
            )
        }
    }
    #[doc = "RCC kernel clock configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccipr3(pub u32);
    impl Ccipr3 {
        #[doc = "SPI1 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn spi1sel(&self) -> super::vals::Spi1sel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Spi1sel::from_bits(val as u8)
        }
        #[doc = "SPI1 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_spi1sel(&mut self, val: super::vals::Spi1sel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "SPI2 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn spi2sel(&self) -> super::vals::Spi2sel {
            let val = (self.0 >> 3usize) & 0x07;
            super::vals::Spi2sel::from_bits(val as u8)
        }
        #[doc = "SPI2 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_spi2sel(&mut self, val: super::vals::Spi2sel) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
        }
        #[doc = "SPI3 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn spi3sel(&self) -> super::vals::Spi3sel {
            let val = (self.0 >> 6usize) & 0x07;
            super::vals::Spi3sel::from_bits(val as u8)
        }
        #[doc = "SPI3 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_spi3sel(&mut self, val: super::vals::Spi3sel) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val.to_bits() as u32) & 0x07) << 6usize);
        }
        #[doc = "PLAY1 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn play1sel(&self) -> super::vals::Playsel {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::Playsel::from_bits(val as u8)
        }
        #[doc = "PLAY1 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_play1sel(&mut self, val: super::vals::Playsel) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
        #[doc = "PLAY1 kernel clock divider."]
        #[must_use]
        #[inline(always)]
        pub const fn play1div(&self) -> super::vals::Playdiv {
            let val = (self.0 >> 20usize) & 0x0f;
            super::vals::Playdiv::from_bits(val as u8)
        }
        #[doc = "PLAY1 kernel clock divider."]
        #[inline(always)]
        pub const fn set_play1div(&mut self, val: super::vals::Playdiv) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val.to_bits() as u32) & 0x0f) << 20usize);
        }
        #[doc = "LPTIM1 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim1sel(&self) -> super::vals::Lptimsel {
            let val = (self.0 >> 24usize) & 0x07;
            super::vals::Lptimsel::from_bits(val as u8)
        }
        #[doc = "LPTIM1 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_lptim1sel(&mut self, val: super::vals::Lptimsel) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
        }
        #[doc = "SYSTICK clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn systicksel(&self) -> super::vals::Systicksel {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Systicksel::from_bits(val as u8)
        }
        #[doc = "SYSTICK clock source selection."]
        #[inline(always)]
        pub const fn set_systicksel(&mut self, val: super::vals::Systicksel) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Ccipr3 {
        #[inline(always)]
        fn default() -> Ccipr3 {
            Ccipr3(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Ccipr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccipr3")
                .field("spi1sel", &self.spi1sel())
                .field("spi2sel", &self.spi2sel())
                .field("spi3sel", &self.spi3sel())
                .field("play1sel", &self.play1sel())
                .field("play1div", &self.play1div())
                .field("lptim1sel", &self.lptim1sel())
                .field("systicksel", &self.systicksel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccipr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccipr3 {{ spi1sel: {:?}, spi2sel: {:?}, spi3sel: {:?}, play1sel: {:?}, play1div: {:?}, lptim1sel: {:?}, systicksel: {:?} }}",
                self.spi1sel(),
                self.spi2sel(),
                self.spi3sel(),
                self.play1sel(),
                self.play1div(),
                self.lptim1sel(),
                self.systicksel()
            )
        }
    }
    #[doc = "RCC kernel clock configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccipr4(pub u32);
    impl Ccipr4 {
        #[doc = "XSPI1 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn xspi1sel(&self) -> super::vals::Xspisel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Xspisel::from_bits(val as u8)
        }
        #[doc = "XSPI1 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_xspi1sel(&mut self, val: super::vals::Xspisel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "UCPD1 clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn ucpd1sel(&self) -> super::vals::Ucpdsel {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Ucpdsel::from_bits(val as u8)
        }
        #[doc = "UCPD1 clock source selection."]
        #[inline(always)]
        pub const fn set_ucpd1sel(&mut self, val: super::vals::Ucpdsel) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "USB kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn usbsel(&self) -> super::vals::Usbsel {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Usbsel::from_bits(val as u8)
        }
        #[doc = "USB kernel clock source selection."]
        #[inline(always)]
        pub const fn set_usbsel(&mut self, val: super::vals::Usbsel) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "I3C1 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn i3c1sel(&self) -> super::vals::I3c1sel {
            let val = (self.0 >> 12usize) & 0x07;
            super::vals::I3c1sel::from_bits(val as u8)
        }
        #[doc = "I3C1 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_i3c1sel(&mut self, val: super::vals::I3c1sel) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
        }
        #[doc = "I2C1 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1sel(&self) -> super::vals::I2c1sel {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::I2c1sel::from_bits(val as u8)
        }
        #[doc = "I2C1 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_i2c1sel(&mut self, val: super::vals::I2c1sel) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
        #[doc = "I2C2 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c2sel(&self) -> super::vals::I2c2sel {
            let val = (self.0 >> 20usize) & 0x07;
            super::vals::I2c2sel::from_bits(val as u8)
        }
        #[doc = "I2C2 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_i2c2sel(&mut self, val: super::vals::I2c2sel) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
        }
        #[doc = "I2C3 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c3sel(&self) -> super::vals::I2c3sel {
            let val = (self.0 >> 24usize) & 0x07;
            super::vals::I2c3sel::from_bits(val as u8)
        }
        #[doc = "I2C3 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_i2c3sel(&mut self, val: super::vals::I2c3sel) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
        }
        #[doc = "I2C4 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c4sel(&self) -> super::vals::I2c4sel {
            let val = (self.0 >> 28usize) & 0x07;
            super::vals::I2c4sel::from_bits(val as u8)
        }
        #[doc = "I2C4 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_i2c4sel(&mut self, val: super::vals::I2c4sel) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Ccipr4 {
        #[inline(always)]
        fn default() -> Ccipr4 {
            Ccipr4(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Ccipr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccipr4")
                .field("xspi1sel", &self.xspi1sel())
                .field("ucpd1sel", &self.ucpd1sel())
                .field("usbsel", &self.usbsel())
                .field("i3c1sel", &self.i3c1sel())
                .field("i2c1sel", &self.i2c1sel())
                .field("i2c2sel", &self.i2c2sel())
                .field("i2c3sel", &self.i2c3sel())
                .field("i2c4sel", &self.i2c4sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccipr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccipr4 {{ xspi1sel: {:?}, ucpd1sel: {:?}, usbsel: {:?}, i3c1sel: {:?}, i2c1sel: {:?}, i2c2sel: {:?}, i2c3sel: {:?}, i2c4sel: {:?} }}",
                self.xspi1sel(),
                self.ucpd1sel(),
                self.usbsel(),
                self.i3c1sel(),
                self.i2c1sel(),
                self.i2c2sel(),
                self.i2c3sel(),
                self.i2c4sel()
            )
        }
    }
    #[doc = "RCC kernel clock configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccipr5(pub u32);
    impl Ccipr5 {
        #[doc = "ADC12 and ADC3 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn adcsel(&self) -> super::vals::Adcsel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Adcsel::from_bits(val as u8)
        }
        #[doc = "ADC12 and ADC3 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_adcsel(&mut self, val: super::vals::Adcsel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "ADC clock prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn adcdiv(&self) -> super::vals::Adcdiv {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::Adcdiv::from_bits(val as u8)
        }
        #[doc = "ADC clock prescaler."]
        #[inline(always)]
        pub const fn set_adcdiv(&mut self, val: super::vals::Adcdiv) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
        #[doc = "FDCAN1, FDCAN2, and FDCAN3 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn fdcansel(&self) -> super::vals::Fdcansel {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Fdcansel::from_bits(val as u8)
        }
        #[doc = "FDCAN1, FDCAN2, and FDCAN3 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_fdcansel(&mut self, val: super::vals::Fdcansel) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "POSENC1 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn posenc1sel(&self) -> super::vals::Posencsel {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::Posencsel::from_bits(val as u8)
        }
        #[doc = "POSENC1 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_posenc1sel(&mut self, val: super::vals::Posencsel) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
        #[doc = "POSENC2 kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn posenc2sel(&self) -> super::vals::Posencsel {
            let val = (self.0 >> 20usize) & 0x07;
            super::vals::Posencsel::from_bits(val as u8)
        }
        #[doc = "POSENC2 kernel clock source selection."]
        #[inline(always)]
        pub const fn set_posenc2sel(&mut self, val: super::vals::Posencsel) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
        }
    }
    impl Default for Ccipr5 {
        #[inline(always)]
        fn default() -> Ccipr5 {
            Ccipr5(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Ccipr5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccipr5")
                .field("adcsel", &self.adcsel())
                .field("adcdiv", &self.adcdiv())
                .field("fdcansel", &self.fdcansel())
                .field("posenc1sel", &self.posenc1sel())
                .field("posenc2sel", &self.posenc2sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccipr5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccipr5 {{ adcsel: {:?}, adcdiv: {:?}, fdcansel: {:?}, posenc1sel: {:?}, posenc2sel: {:?} }}",
                self.adcsel(),
                self.adcdiv(),
                self.fdcansel(),
                self.posenc1sel(),
                self.posenc2sel()
            )
        }
    }
    #[doc = "RCC clock configuration register1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr1(pub u32);
    impl Cfgr1 {
        #[doc = "system clock and trace clock switch."]
        #[must_use]
        #[inline(always)]
        pub const fn syssw(&self) -> super::vals::Syssw {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Syssw::from_bits(val as u8)
        }
        #[doc = "system clock and trace clock switch."]
        #[inline(always)]
        pub const fn set_syssw(&mut self, val: super::vals::Syssw) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "system clock switch status."]
        #[must_use]
        #[inline(always)]
        pub const fn syssws(&self) -> super::vals::Syssw {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Syssw::from_bits(val as u8)
        }
        #[doc = "system clock switch status."]
        #[inline(always)]
        pub const fn set_syssws(&mut self, val: super::vals::Syssw) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "HSE division factor for RTC clock."]
        #[must_use]
        #[inline(always)]
        pub const fn rtcpre(&self) -> super::vals::Rtcpre {
            let val = (self.0 >> 8usize) & 0x3f;
            super::vals::Rtcpre::from_bits(val as u8)
        }
        #[doc = "HSE division factor for RTC clock."]
        #[inline(always)]
        pub const fn set_rtcpre(&mut self, val: super::vals::Rtcpre) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val.to_bits() as u32) & 0x3f) << 8usize);
        }
        #[doc = "MCO1 prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn mco1pre(&self) -> super::vals::Mcopre {
            let val = (self.0 >> 18usize) & 0x0f;
            super::vals::Mcopre::from_bits(val as u8)
        }
        #[doc = "MCO1 prescaler."]
        #[inline(always)]
        pub const fn set_mco1pre(&mut self, val: super::vals::Mcopre) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val.to_bits() as u32) & 0x0f) << 18usize);
        }
        #[doc = "Microcontroller clock output 1."]
        #[must_use]
        #[inline(always)]
        pub const fn mco1sel(&self) -> super::vals::Mcosel {
            let val = (self.0 >> 22usize) & 0x07;
            super::vals::Mcosel::from_bits(val as u8)
        }
        #[doc = "Microcontroller clock output 1."]
        #[inline(always)]
        pub const fn set_mco1sel(&mut self, val: super::vals::Mcosel) {
            self.0 = (self.0 & !(0x07 << 22usize)) | (((val.to_bits() as u32) & 0x07) << 22usize);
        }
        #[doc = "MCO2 prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn mco2pre(&self) -> super::vals::Mcopre {
            let val = (self.0 >> 25usize) & 0x0f;
            super::vals::Mcopre::from_bits(val as u8)
        }
        #[doc = "MCO2 prescaler."]
        #[inline(always)]
        pub const fn set_mco2pre(&mut self, val: super::vals::Mcopre) {
            self.0 = (self.0 & !(0x0f << 25usize)) | (((val.to_bits() as u32) & 0x0f) << 25usize);
        }
        #[doc = "microcontroller clock output 2."]
        #[must_use]
        #[inline(always)]
        pub const fn mco2sel(&self) -> super::vals::Mcosel {
            let val = (self.0 >> 29usize) & 0x07;
            super::vals::Mcosel::from_bits(val as u8)
        }
        #[doc = "microcontroller clock output 2."]
        #[inline(always)]
        pub const fn set_mco2sel(&mut self, val: super::vals::Mcosel) {
            self.0 = (self.0 & !(0x07 << 29usize)) | (((val.to_bits() as u32) & 0x07) << 29usize);
        }
    }
    impl Default for Cfgr1 {
        #[inline(always)]
        fn default() -> Cfgr1 {
            Cfgr1(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Cfgr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr1")
                .field("syssw", &self.syssw())
                .field("syssws", &self.syssws())
                .field("rtcpre", &self.rtcpre())
                .field("mco1pre", &self.mco1pre())
                .field("mco1sel", &self.mco1sel())
                .field("mco2pre", &self.mco2pre())
                .field("mco2sel", &self.mco2sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfgr1 {{ syssw: {:?}, syssws: {:?}, rtcpre: {:?}, mco1pre: {:?}, mco1sel: {:?}, mco2pre: {:?}, mco2sel: {:?} }}",
                self.syssw(),
                self.syssws(),
                self.rtcpre(),
                self.mco1pre(),
                self.mco1sel(),
                self.mco2pre(),
                self.mco2sel()
            )
        }
    }
    #[doc = "RCC CPU domain clock configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "AHB prescaler."]
        #[must_use]
        #[inline(always)]
        pub const fn hpre(&self) -> super::vals::Hpre {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Hpre::from_bits(val as u8)
        }
        #[doc = "AHB prescaler."]
        #[inline(always)]
        pub const fn set_hpre(&mut self, val: super::vals::Hpre) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "APB low-speed prescaler (APB1)."]
        #[must_use]
        #[inline(always)]
        pub const fn ppre1(&self) -> super::vals::Ppre {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB low-speed prescaler (APB1)."]
        #[inline(always)]
        pub const fn set_ppre1(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "APB high-speed prescaler (APB2)."]
        #[must_use]
        #[inline(always)]
        pub const fn ppre2(&self) -> super::vals::Ppre {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB high-speed prescaler (APB2)."]
        #[inline(always)]
        pub const fn set_ppre2(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "APB low-speed prescaler (APB3)."]
        #[must_use]
        #[inline(always)]
        pub const fn ppre3(&self) -> super::vals::Ppre {
            let val = (self.0 >> 12usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB low-speed prescaler (APB3)."]
        #[inline(always)]
        pub const fn set_ppre3(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
        }
        #[doc = "AHB1 clock disable."]
        #[must_use]
        #[inline(always)]
        pub const fn ahb1dis(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "AHB1 clock disable."]
        #[inline(always)]
        pub const fn set_ahb1dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "AHB2 clock disable."]
        #[must_use]
        #[inline(always)]
        pub const fn ahb2dis(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "AHB2 clock disable."]
        #[inline(always)]
        pub const fn set_ahb2dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "AHB3 clock disable."]
        #[must_use]
        #[inline(always)]
        pub const fn ahb3dis(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "AHB3 clock disable."]
        #[inline(always)]
        pub const fn set_ahb3dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "AHB4 clock disable."]
        #[must_use]
        #[inline(always)]
        pub const fn ahb4dis(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "AHB4 clock disable."]
        #[inline(always)]
        pub const fn set_ahb4dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "APB1 clock disable value."]
        #[must_use]
        #[inline(always)]
        pub const fn apb1dis(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "APB1 clock disable value."]
        #[inline(always)]
        pub const fn set_apb1dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "APB2 clock disable value."]
        #[must_use]
        #[inline(always)]
        pub const fn apb2dis(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "APB2 clock disable value."]
        #[inline(always)]
        pub const fn set_apb2dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "APB3 clock disable value."]
        #[must_use]
        #[inline(always)]
        pub const fn apb3dis(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "APB3 clock disable value."]
        #[inline(always)]
        pub const fn set_apb3dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Cfgr2 {
        #[inline(always)]
        fn default() -> Cfgr2 {
            Cfgr2(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Cfgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr2")
                .field("hpre", &self.hpre())
                .field("ppre1", &self.ppre1())
                .field("ppre2", &self.ppre2())
                .field("ppre3", &self.ppre3())
                .field("ahb1dis", &self.ahb1dis())
                .field("ahb2dis", &self.ahb2dis())
                .field("ahb3dis", &self.ahb3dis())
                .field("ahb4dis", &self.ahb4dis())
                .field("apb1dis", &self.apb1dis())
                .field("apb2dis", &self.apb2dis())
                .field("apb3dis", &self.apb3dis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfgr2 {{ hpre: {:?}, ppre1: {:?}, ppre2: {:?}, ppre3: {:?}, ahb1dis: {=bool:?}, ahb2dis: {=bool:?}, ahb3dis: {=bool:?}, ahb4dis: {=bool:?}, apb1dis: {=bool:?}, apb2dis: {=bool:?}, apb3dis: {=bool:?} }}",
                self.hpre(),
                self.ppre1(),
                self.ppre2(),
                self.ppre3(),
                self.ahb1dis(),
                self.ahb2dis(),
                self.ahb3dis(),
                self.ahb4dis(),
                self.apb1dis(),
                self.apb2dis(),
                self.apb3dis()
            )
        }
    }
    #[doc = "RCC clock control register."]
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
        #[doc = "HSI clock ready flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hsirdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "HSI clock ready flag."]
        #[inline(always)]
        pub const fn set_hsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI clock enable in Stop mode."]
        #[must_use]
        #[inline(always)]
        pub const fn hsikeron(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSI clock enable in Stop mode."]
        #[inline(always)]
        pub const fn set_hsikeron(&mut self, val: bool) {
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
        #[doc = "HSE clock security system enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hsecsson(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock security system enable."]
        #[inline(always)]
        pub const fn set_hsecsson(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "External high-speed clock type in bypass mode."]
        #[must_use]
        #[inline(always)]
        pub const fn hseext(&self) -> super::vals::Hseext {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Hseext::from_bits(val as u8)
        }
        #[doc = "External high-speed clock type in bypass mode."]
        #[inline(always)]
        pub const fn set_hseext(&mut self, val: super::vals::Hseext) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "PLL1 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pllon(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 24usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 enable."]
        #[inline(always)]
        pub const fn set_pllon(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 24usize + n * 2usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "PLL1 clock ready flag."]
        #[must_use]
        #[inline(always)]
        pub const fn pllrdy(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 25usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 clock ready flag."]
        #[inline(always)]
        pub const fn set_pllrdy(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
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
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr")
                .field("hsion", &self.hsion())
                .field("hsirdy", &self.hsirdy())
                .field("hsikeron", &self.hsikeron())
                .field("hsidiv", &self.hsidiv())
                .field("hsidivf", &self.hsidivf())
                .field("hsi48on", &self.hsi48on())
                .field("hsi48rdy", &self.hsi48rdy())
                .field("hseon", &self.hseon())
                .field("hserdy", &self.hserdy())
                .field("hsebyp", &self.hsebyp())
                .field("hsecsson", &self.hsecsson())
                .field("hseext", &self.hseext())
                .field("pllon[0]", &self.pllon(0usize))
                .field("pllon[1]", &self.pllon(1usize))
                .field("pllrdy[0]", &self.pllrdy(0usize))
                .field("pllrdy[1]", &self.pllrdy(1usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ hsion: {=bool:?}, hsirdy: {=bool:?}, hsikeron: {=bool:?}, hsidiv: {:?}, hsidivf: {=bool:?}, hsi48on: {=bool:?}, hsi48rdy: {=bool:?}, hseon: {=bool:?}, hserdy: {=bool:?}, hsebyp: {=bool:?}, hsecsson: {=bool:?}, hseext: {:?}, pllon[0]: {=bool:?}, pllon[1]: {=bool:?}, pllrdy[0]: {=bool:?}, pllrdy[1]: {=bool:?} }}",
                self.hsion(),
                self.hsirdy(),
                self.hsikeron(),
                self.hsidiv(),
                self.hsidivf(),
                self.hsi48on(),
                self.hsi48rdy(),
                self.hseon(),
                self.hserdy(),
                self.hsebyp(),
                self.hsecsson(),
                self.hseext(),
                self.pllon(0usize),
                self.pllon(1usize),
                self.pllrdy(0usize),
                self.pllrdy(1usize)
            )
        }
    }
    #[doc = "RCC clock recovery RC register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crrcr(pub u32);
    impl Crrcr {
        #[doc = "Internal RC 48MHz clock calibration."]
        #[must_use]
        #[inline(always)]
        pub const fn hsi48cal(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Internal RC 48MHz clock calibration."]
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
    #[cfg(feature = "debug")]
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
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "HSI clock trimming."]
        #[inline(always)]
        pub const fn set_hsitrim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for Hsicfgr {
        #[inline(always)]
        fn default() -> Hsicfgr {
            Hsicfgr(0)
        }
    }
    #[cfg(feature = "debug")]
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
    #[doc = "RCC Interrupt clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
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
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt clear."]
        #[inline(always)]
        pub const fn set_hsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HSE ready interrupt clear."]
        #[must_use]
        #[inline(always)]
        pub const fn hserdyc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt clear."]
        #[inline(always)]
        pub const fn set_hserdyc(&mut self, val: bool) {
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
        pub const fn pllrdyc(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 6usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 ready interrupt clear."]
        #[inline(always)]
        pub const fn set_pllrdyc(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 6usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
    impl Default for Icr {
        #[inline(always)]
        fn default() -> Icr {
            Icr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Icr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icr")
                .field("lsirdyc", &self.lsirdyc())
                .field("lserdyc", &self.lserdyc())
                .field("hsirdyc", &self.hsirdyc())
                .field("hserdyc", &self.hserdyc())
                .field("hsi48rdyc", &self.hsi48rdyc())
                .field("pllrdyc[0]", &self.pllrdyc(0usize))
                .field("pllrdyc[1]", &self.pllrdyc(1usize))
                .field("lsecssc", &self.lsecssc())
                .field("hsecssc", &self.hsecssc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icr {{ lsirdyc: {=bool:?}, lserdyc: {=bool:?}, hsirdyc: {=bool:?}, hserdyc: {=bool:?}, hsi48rdyc: {=bool:?}, pllrdyc[0]: {=bool:?}, pllrdyc[1]: {=bool:?}, lsecssc: {=bool:?}, hsecssc: {=bool:?} }}",
                self.lsirdyc(),
                self.lserdyc(),
                self.hsirdyc(),
                self.hserdyc(),
                self.hsi48rdyc(),
                self.pllrdyc(0usize),
                self.pllrdyc(1usize),
                self.lsecssc(),
                self.hsecssc()
            )
        }
    }
    #[doc = "RCC Interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
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
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt enable."]
        #[inline(always)]
        pub const fn set_hsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HSE ready interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hserdyie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt enable."]
        #[inline(always)]
        pub const fn set_hserdyie(&mut self, val: bool) {
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
        pub const fn pllrdyie(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 6usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 ready interrupt enable."]
        #[inline(always)]
        pub const fn set_pllrdyie(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 6usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
    impl Default for Ier {
        #[inline(always)]
        fn default() -> Ier {
            Ier(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Ier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ier")
                .field("lsirdyie", &self.lsirdyie())
                .field("lserdyie", &self.lserdyie())
                .field("hsirdyie", &self.hsirdyie())
                .field("hserdyie", &self.hserdyie())
                .field("hsi48rdyie", &self.hsi48rdyie())
                .field("pllrdyie[0]", &self.pllrdyie(0usize))
                .field("pllrdyie[1]", &self.pllrdyie(1usize))
                .field("lsecssie", &self.lsecssie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ier {{ lsirdyie: {=bool:?}, lserdyie: {=bool:?}, hsirdyie: {=bool:?}, hserdyie: {=bool:?}, hsi48rdyie: {=bool:?}, pllrdyie[0]: {=bool:?}, pllrdyie[1]: {=bool:?}, lsecssie: {=bool:?} }}",
                self.lsirdyie(),
                self.lserdyie(),
                self.hsirdyie(),
                self.hserdyie(),
                self.hsi48rdyie(),
                self.pllrdyie(0usize),
                self.pllrdyie(1usize),
                self.lsecssie()
            )
        }
    }
    #[doc = "RCC Interrupt flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ifr(pub u32);
    impl Ifr {
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
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt flag."]
        #[inline(always)]
        pub const fn set_hsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HSE ready interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hserdyf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt flag."]
        #[inline(always)]
        pub const fn set_hserdyf(&mut self, val: bool) {
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
        pub const fn pllrdyf(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 6usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 ready interrupt flag."]
        #[inline(always)]
        pub const fn set_pllrdyf(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 6usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
    impl Default for Ifr {
        #[inline(always)]
        fn default() -> Ifr {
            Ifr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Ifr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ifr")
                .field("lsirdyf", &self.lsirdyf())
                .field("lserdyf", &self.lserdyf())
                .field("hsirdyf", &self.hsirdyf())
                .field("hserdyf", &self.hserdyf())
                .field("hsi48rdyf", &self.hsi48rdyf())
                .field("pllrdyf[0]", &self.pllrdyf(0usize))
                .field("pllrdyf[1]", &self.pllrdyf(1usize))
                .field("lsecssf", &self.lsecssf())
                .field("hsecssf", &self.hsecssf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ifr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ifr {{ lsirdyf: {=bool:?}, lserdyf: {=bool:?}, hsirdyf: {=bool:?}, hserdyf: {=bool:?}, hsi48rdyf: {=bool:?}, pllrdyf[0]: {=bool:?}, pllrdyf[1]: {=bool:?}, lsecssf: {=bool:?}, hsecssf: {=bool:?} }}",
                self.lsirdyf(),
                self.lserdyf(),
                self.hsirdyf(),
                self.hserdyf(),
                self.hsi48rdyf(),
                self.pllrdyf(0usize),
                self.pllrdyf(1usize),
                self.lsecssf(),
                self.hsecssf()
            )
        }
    }
    #[doc = "RCC PLL clock source selection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pll1cfgr(pub u32);
    impl Pll1cfgr {
        #[doc = "PLL1M and PLLs clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn pllsrc(&self) -> super::vals::Pllsrc {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Pllsrc::from_bits(val as u8)
        }
        #[doc = "PLL1M and PLLs clock source selection."]
        #[inline(always)]
        pub const fn set_pllsrc(&mut self, val: super::vals::Pllsrc) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "PLL1 input frequency range."]
        #[must_use]
        #[inline(always)]
        pub const fn pllrge(&self) -> super::vals::Pllrge {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Pllrge::from_bits(val as u8)
        }
        #[doc = "PLL1 input frequency range."]
        #[inline(always)]
        pub const fn set_pllrge(&mut self, val: super::vals::Pllrge) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "PLL1 fractional latch enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pllfracen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PLL1 fractional latch enable."]
        #[inline(always)]
        pub const fn set_pllfracen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "PLL1 VCO selection."]
        #[must_use]
        #[inline(always)]
        pub const fn pllvcosel(&self) -> super::vals::Pllvcosel {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Pllvcosel::from_bits(val as u8)
        }
        #[doc = "PLL1 VCO selection."]
        #[inline(always)]
        pub const fn set_pllvcosel(&mut self, val: super::vals::Pllvcosel) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "Prescaler for PLL1."]
        #[must_use]
        #[inline(always)]
        pub const fn pllm(&self) -> super::vals::Pllm {
            let val = (self.0 >> 8usize) & 0x3f;
            super::vals::Pllm::from_bits(val as u8)
        }
        #[doc = "Prescaler for PLL1."]
        #[inline(always)]
        pub const fn set_pllm(&mut self, val: super::vals::Pllm) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val.to_bits() as u32) & 0x3f) << 8usize);
        }
        #[doc = "PLL1 DIVP divider output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pllpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PLL1 DIVP divider output enable."]
        #[inline(always)]
        pub const fn set_pllpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "PLL1 DIVQ divider output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pllqen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "PLL1 DIVQ divider output enable."]
        #[inline(always)]
        pub const fn set_pllqen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "PLL1 DIVR and ICLK1 divider output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pllren(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PLL1 DIVR and ICLK1 divider output enable."]
        #[inline(always)]
        pub const fn set_pllren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Intermediate clock 1 division."]
        #[must_use]
        #[inline(always)]
        pub const fn iclk1div(&self) -> super::vals::Iclkdiv {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Iclkdiv::from_bits(val as u8)
        }
        #[doc = "Intermediate clock 1 division."]
        #[inline(always)]
        pub const fn set_iclk1div(&mut self, val: super::vals::Iclkdiv) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Pll1cfgr {
        #[inline(always)]
        fn default() -> Pll1cfgr {
            Pll1cfgr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Pll1cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pll1cfgr")
                .field("pllsrc", &self.pllsrc())
                .field("pllrge", &self.pllrge())
                .field("pllfracen", &self.pllfracen())
                .field("pllvcosel", &self.pllvcosel())
                .field("pllm", &self.pllm())
                .field("pllpen", &self.pllpen())
                .field("pllqen", &self.pllqen())
                .field("pllren", &self.pllren())
                .field("iclk1div", &self.iclk1div())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pll1cfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pll1cfgr {{ pllsrc: {:?}, pllrge: {:?}, pllfracen: {=bool:?}, pllvcosel: {:?}, pllm: {:?}, pllpen: {=bool:?}, pllqen: {=bool:?}, pllren: {=bool:?}, iclk1div: {:?} }}",
                self.pllsrc(),
                self.pllrge(),
                self.pllfracen(),
                self.pllvcosel(),
                self.pllm(),
                self.pllpen(),
                self.pllqen(),
                self.pllren(),
                self.iclk1div()
            )
        }
    }
    #[doc = "RCC PLL1 dividers register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pll1divr(pub u32);
    impl Pll1divr {
        #[doc = "Multiplication factor for PLL1VCO."]
        #[must_use]
        #[inline(always)]
        pub const fn plln(&self) -> super::vals::Plln {
            let val = (self.0 >> 0usize) & 0x01ff;
            super::vals::Plln::from_bits(val as u16)
        }
        #[doc = "Multiplication factor for PLL1VCO."]
        #[inline(always)]
        pub const fn set_plln(&mut self, val: super::vals::Plln) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val.to_bits() as u32) & 0x01ff) << 0usize);
        }
        #[doc = "PLL1 DIVP division factor."]
        #[must_use]
        #[inline(always)]
        pub const fn pllp(&self) -> super::vals::Plldiv {
            let val = (self.0 >> 9usize) & 0x7f;
            super::vals::Plldiv::from_bits(val as u8)
        }
        #[doc = "PLL1 DIVP division factor."]
        #[inline(always)]
        pub const fn set_pllp(&mut self, val: super::vals::Plldiv) {
            self.0 = (self.0 & !(0x7f << 9usize)) | (((val.to_bits() as u32) & 0x7f) << 9usize);
        }
        #[doc = "PLL1 DIVQ division factor."]
        #[must_use]
        #[inline(always)]
        pub const fn pllq(&self) -> super::vals::Plldiv {
            let val = (self.0 >> 16usize) & 0x7f;
            super::vals::Plldiv::from_bits(val as u8)
        }
        #[doc = "PLL1 DIVQ division factor."]
        #[inline(always)]
        pub const fn set_pllq(&mut self, val: super::vals::Plldiv) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val.to_bits() as u32) & 0x7f) << 16usize);
        }
        #[doc = "PLL1 DIVR division factor."]
        #[must_use]
        #[inline(always)]
        pub const fn pllr(&self) -> super::vals::Plldiv {
            let val = (self.0 >> 24usize) & 0x7f;
            super::vals::Plldiv::from_bits(val as u8)
        }
        #[doc = "PLL1 DIVR division factor."]
        #[inline(always)]
        pub const fn set_pllr(&mut self, val: super::vals::Plldiv) {
            self.0 = (self.0 & !(0x7f << 24usize)) | (((val.to_bits() as u32) & 0x7f) << 24usize);
        }
    }
    impl Default for Pll1divr {
        #[inline(always)]
        fn default() -> Pll1divr {
            Pll1divr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Pll1divr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pll1divr")
                .field("plln", &self.plln())
                .field("pllp", &self.pllp())
                .field("pllq", &self.pllq())
                .field("pllr", &self.pllr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pll1divr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pll1divr {{ plln: {:?}, pllp: {:?}, pllq: {:?}, pllr: {:?} }}",
                self.plln(),
                self.pllp(),
                self.pllq(),
                self.pllr()
            )
        }
    }
    #[doc = "RCC PLL1 fractional divider register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pll1fracr(pub u32);
    impl Pll1fracr {
        #[doc = "Fractional part of the multiplication factor for PLL1 VCO."]
        #[must_use]
        #[inline(always)]
        pub const fn pllfracn(&self) -> u16 {
            let val = (self.0 >> 3usize) & 0x1fff;
            val as u16
        }
        #[doc = "Fractional part of the multiplication factor for PLL1 VCO."]
        #[inline(always)]
        pub const fn set_pllfracn(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 3usize)) | (((val as u32) & 0x1fff) << 3usize);
        }
    }
    impl Default for Pll1fracr {
        #[inline(always)]
        fn default() -> Pll1fracr {
            Pll1fracr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Pll1fracr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pll1fracr").field("pllfracn", &self.pllfracn()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pll1fracr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pll1fracr {{ pllfracn: {=u16:?} }}", self.pllfracn())
        }
    }
    #[doc = "RCC privilege configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcfgr(pub u32);
    impl Privcfgr {
        #[doc = "RCC secure functions privilege configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn spriv(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RCC secure functions privilege configuration."]
        #[inline(always)]
        pub const fn set_spriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RCC nonsecure functions privilege configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn nspriv(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RCC nonsecure functions privilege configuration."]
        #[inline(always)]
        pub const fn set_nspriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Privcfgr {
        #[inline(always)]
        fn default() -> Privcfgr {
            Privcfgr(0)
        }
    }
    #[cfg(feature = "debug")]
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
            defmt::write!(
                f,
                "Privcfgr {{ spriv: {=bool:?}, nspriv: {=bool:?} }}",
                self.spriv(),
                self.nspriv()
            )
        }
    }
    #[doc = "RCC reset status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rsr(pub u32);
    impl Rsr {
        #[doc = "remove reset flag."]
        #[must_use]
        #[inline(always)]
        pub const fn rmvf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "remove reset flag."]
        #[inline(always)]
        pub const fn set_rmvf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "pin reset flag (NRST)."]
        #[must_use]
        #[inline(always)]
        pub const fn pinrstf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "pin reset flag (NRST)."]
        #[inline(always)]
        pub const fn set_pinrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "POR reset flag."]
        #[must_use]
        #[inline(always)]
        pub const fn porrstf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "POR reset flag."]
        #[inline(always)]
        pub const fn set_porrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "system reset from CPU reset flag."]
        #[must_use]
        #[inline(always)]
        pub const fn sftrstf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "system reset from CPU reset flag."]
        #[inline(always)]
        pub const fn set_sftrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "independent watchdog reset flag."]
        #[must_use]
        #[inline(always)]
        pub const fn iwdgrstf(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "independent watchdog reset flag."]
        #[inline(always)]
        pub const fn set_iwdgrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "window watchdog reset flag."]
        #[must_use]
        #[inline(always)]
        pub const fn wwdgrstf(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "window watchdog reset flag."]
        #[inline(always)]
        pub const fn set_wwdgrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Low-power reset flag."]
        #[must_use]
        #[inline(always)]
        pub const fn lpwrrstf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power reset flag."]
        #[inline(always)]
        pub const fn set_lpwrrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Rsr {
        #[inline(always)]
        fn default() -> Rsr {
            Rsr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Rsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rsr")
                .field("rmvf", &self.rmvf())
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
                "Rsr {{ rmvf: {=bool:?}, pinrstf: {=bool:?}, porrstf: {=bool:?}, sftrstf: {=bool:?}, iwdgrstf: {=bool:?}, wwdgrstf: {=bool:?}, lpwrrstf: {=bool:?} }}",
                self.rmvf(),
                self.pinrstf(),
                self.porrstf(),
                self.sftrstf(),
                self.iwdgrstf(),
                self.wwdgrstf(),
                self.lpwrrstf()
            )
        }
    }
    #[doc = "RCC RTC domain control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rtccr(pub u32);
    impl Rtccr {
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
        #[doc = "RTC domain software reset."]
        #[must_use]
        #[inline(always)]
        pub const fn rtcdrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "RTC domain software reset."]
        #[inline(always)]
        pub const fn set_rtcdrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "LSI oscillator enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lsion(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "LSI oscillator enable."]
        #[inline(always)]
        pub const fn set_lsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "LSI oscillator ready."]
        #[must_use]
        #[inline(always)]
        pub const fn lsirdy(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "LSI oscillator ready."]
        #[inline(always)]
        pub const fn set_lsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Rtccr {
        #[inline(always)]
        fn default() -> Rtccr {
            Rtccr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Rtccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rtccr")
                .field("lseon", &self.lseon())
                .field("lserdy", &self.lserdy())
                .field("lsebyp", &self.lsebyp())
                .field("lsedrv", &self.lsedrv())
                .field("lsecsson", &self.lsecsson())
                .field("lsecssd", &self.lsecssd())
                .field("rtcsel", &self.rtcsel())
                .field("rtcen", &self.rtcen())
                .field("rtcdrst", &self.rtcdrst())
                .field("lsion", &self.lsion())
                .field("lsirdy", &self.lsirdy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rtccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rtccr {{ lseon: {=bool:?}, lserdy: {=bool:?}, lsebyp: {=bool:?}, lsedrv: {:?}, lsecsson: {=bool:?}, lsecssd: {=bool:?}, rtcsel: {:?}, rtcen: {=bool:?}, rtcdrst: {=bool:?}, lsion: {=bool:?}, lsirdy: {=bool:?} }}",
                self.lseon(),
                self.lserdy(),
                self.lsebyp(),
                self.lsedrv(),
                self.lsecsson(),
                self.lsecssd(),
                self.rtcsel(),
                self.rtcen(),
                self.rtcdrst(),
                self.lsion(),
                self.lsirdy()
            )
        }
    }
    #[doc = "RCC secure configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccfgr(pub u32);
    impl Seccfgr {
        #[doc = "HSI clock configuration and status bits security."]
        #[must_use]
        #[inline(always)]
        pub const fn hsisec(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "HSI clock configuration and status bits security."]
        #[inline(always)]
        pub const fn set_hsisec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "HSE clock configuration bits, status bits and HSE_CSS security."]
        #[must_use]
        #[inline(always)]
        pub const fn hsesec(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock configuration bits, status bits and HSE_CSS security."]
        #[inline(always)]
        pub const fn set_hsesec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LSI clock configuration and status bits security."]
        #[must_use]
        #[inline(always)]
        pub const fn lsisec(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LSI clock configuration and status bits security."]
        #[inline(always)]
        pub const fn set_lsisec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "LSE clock configuration and status bits security."]
        #[must_use]
        #[inline(always)]
        pub const fn lsesec(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock configuration and status bits security."]
        #[inline(always)]
        pub const fn set_lsesec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SYSCLK clock selection, clock output on MCO configuration security."]
        #[must_use]
        #[inline(always)]
        pub const fn sysclksec(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCLK clock selection, clock output on MCO configuration security."]
        #[inline(always)]
        pub const fn set_sysclksec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "AHBx/APBx prescaler configuration bits security."]
        #[must_use]
        #[inline(always)]
        pub const fn prescsec(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "AHBx/APBx prescaler configuration bits security."]
        #[inline(always)]
        pub const fn set_prescsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "PLL1 clock configuration and status bits security."]
        #[must_use]
        #[inline(always)]
        pub const fn pllsec(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 clock configuration and status bits security."]
        #[inline(always)]
        pub const fn set_pllsec(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "HSI48 clock configuration and status bits security."]
        #[must_use]
        #[inline(always)]
        pub const fn hsi48sec(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 clock configuration and status bits security."]
        #[inline(always)]
        pub const fn set_hsi48sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Remove reset flag security."]
        #[must_use]
        #[inline(always)]
        pub const fn rmvfsec(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Remove reset flag security."]
        #[inline(always)]
        pub const fn set_rmvfsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Seccfgr {
        #[inline(always)]
        fn default() -> Seccfgr {
            Seccfgr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Seccfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Seccfgr")
                .field("hsisec", &self.hsisec())
                .field("hsesec", &self.hsesec())
                .field("lsisec", &self.lsisec())
                .field("lsesec", &self.lsesec())
                .field("sysclksec", &self.sysclksec())
                .field("prescsec", &self.prescsec())
                .field("pllsec[0]", &self.pllsec(0usize))
                .field("pllsec[1]", &self.pllsec(1usize))
                .field("hsi48sec", &self.hsi48sec())
                .field("rmvfsec", &self.rmvfsec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Seccfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Seccfgr {{ hsisec: {=bool:?}, hsesec: {=bool:?}, lsisec: {=bool:?}, lsesec: {=bool:?}, sysclksec: {=bool:?}, prescsec: {=bool:?}, pllsec[0]: {=bool:?}, pllsec[1]: {=bool:?}, hsi48sec: {=bool:?}, rmvfsec: {=bool:?} }}",
                self.hsisec(),
                self.hsesec(),
                self.lsisec(),
                self.lsesec(),
                self.sysclksec(),
                self.prescsec(),
                self.pllsec(0usize),
                self.pllsec(1usize),
                self.hsi48sec(),
                self.rmvfsec()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adcdiv {
        #[doc = "ADC kernel clock not divided (default after reset)."]
        Div1 = 0x0,
        #[doc = "ADC kernel clock divided by 2."]
        Div2 = 0x01,
        #[doc = "ADC kernel clock divided by 4."]
        Div4 = 0x02,
        #[doc = "ADC kernel clock divided by 6."]
        Div6 = 0x03,
        #[doc = "ADC kernel clock divided by 8."]
        Div8 = 0x04,
        #[doc = "ADC kernel clock divided by 10."]
        Div10 = 0x05,
        #[doc = "ADC kernel clock divided by 12."]
        Div12 = 0x06,
        #[doc = "ADC kernel clock divided by 16."]
        Div16 = 0x07,
        #[doc = "ADC kernel clock divided by 32."]
        Div32 = 0x08,
        #[doc = "ADC kernel clock divided by 64."]
        Div64 = 0x09,
        #[doc = "ADC kernel clock divided by 128."]
        Div128 = 0x0a,
        #[doc = "ADC kernel clock divided by 256."]
        Div256 = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Adcdiv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adcdiv {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adcdiv {
        #[inline(always)]
        fn from(val: u8) -> Adcdiv {
            Adcdiv::from_bits(val)
        }
    }
    impl From<Adcdiv> for u8 {
        #[inline(always)]
        fn from(val: Adcdiv) -> u8 {
            Adcdiv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adcsel {
        #[doc = "HCLK2"]
        Hclk2 = 0x0,
        #[doc = "SYS"]
        Sys = 0x01,
        #[doc = "HSI"]
        Hsi = 0x02,
        #[doc = "HSE"]
        Hse = 0x03,
        #[doc = "PLL1_Q"]
        Pll1Q = 0x04,
        #[doc = "IC1"]
        Ic1 = 0x05,
        #[doc = "PLL2_P"]
        Pll2P = 0x06,
        #[doc = "IC2"]
        Ic2 = 0x07,
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dacsel {
        #[doc = "HCLK2"]
        Hclk2 = 0x0,
        #[doc = "SYS"]
        Sys = 0x01,
        #[doc = "ADC"]
        Adc = 0x02,
        #[doc = "HSE"]
        Hse = 0x03,
        #[doc = "HSI"]
        Hsi = 0x04,
        #[doc = "PLL1_Q"]
        Pll1Q = 0x05,
        #[doc = "PLL2_Q"]
        Pll2Q = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Dacsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dacsel {
            unsafe { core::mem::transmute(val & 0x07) }
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dacshsel {
        #[doc = "LSE"]
        Lse = 0x0,
        #[doc = "LSI"]
        Lsi = 0x01,
    }
    impl Dacshsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dacshsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dacshsel {
        #[inline(always)]
        fn from(val: u8) -> Dacshsel {
            Dacshsel::from_bits(val)
        }
    }
    impl From<Dacshsel> for u8 {
        #[inline(always)]
        fn from(val: Dacshsel) -> u8 {
            Dacshsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fdcansel {
        #[doc = "HSE"]
        Hse = 0x0,
        #[doc = "PLL1_Q"]
        Pll1Q = 0x01,
        #[doc = "PLL2_Q"]
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hpre {
        #[doc = "rcc_hclk=sys_ck (default after reset)."]
        Div1 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "rcc_hclk=sys_ck / 2."]
        Div2 = 0x08,
        #[doc = "rcc_hclk=sys_ck / 4."]
        Div4 = 0x09,
        #[doc = "rcc_hclk=sys_ck / 8."]
        Div8 = 0x0a,
        #[doc = "rcc_hclk=sys_ck / 16."]
        Div16 = 0x0b,
        #[doc = "rcc_hclk=sys_ck / 64."]
        Div64 = 0x0c,
        #[doc = "rcc_hclk=sys_ck / 128."]
        Div128 = 0x0d,
        #[doc = "rcc_hclk=sys_ck / 256."]
        Div256 = 0x0e,
        #[doc = "rcc_hclk=sys_ck / 512."]
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hseext {
        #[doc = "HSE in analog mode (default after reset)."]
        Analog = 0x0,
        #[doc = "HSE in digital mode."]
        Digital = 0x01,
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hsidiv {
        #[doc = "division by 1, hsi_ck, hsi_ker_ck=64MHz (default after reset)."]
        Div1 = 0x0,
        #[doc = "division by 2, hsi_ck, hsi_ker_ck=32MHz."]
        Div2 = 0x01,
        #[doc = "division by 4, hsi_ck, hsi_ker_ck=16MHz."]
        Div4 = 0x02,
        #[doc = "division by 8, hsi_ck, hsi_ker_ck=8MHz."]
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum I2c1sel {
        #[doc = "PCLK1"]
        Pclk1 = 0x0,
        #[doc = "IC1"]
        Ic1 = 0x01,
        #[doc = "HSI"]
        Hsi = 0x02,
        #[doc = "HSE"]
        Hse = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl I2c1sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2c1sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I2c1sel {
        #[inline(always)]
        fn from(val: u8) -> I2c1sel {
            I2c1sel::from_bits(val)
        }
    }
    impl From<I2c1sel> for u8 {
        #[inline(always)]
        fn from(val: I2c1sel) -> u8 {
            I2c1sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum I2c2sel {
        #[doc = "PCLK1"]
        Pclk1 = 0x0,
        #[doc = "IC1"]
        Ic1 = 0x01,
        #[doc = "HSI"]
        Hsi = 0x02,
        #[doc = "HSE"]
        Hse = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl I2c2sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2c2sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I2c2sel {
        #[inline(always)]
        fn from(val: u8) -> I2c2sel {
            I2c2sel::from_bits(val)
        }
    }
    impl From<I2c2sel> for u8 {
        #[inline(always)]
        fn from(val: I2c2sel) -> u8 {
            I2c2sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum I2c3sel {
        #[doc = "PCLK3"]
        Pclk3 = 0x0,
        #[doc = "IC1"]
        Ic1 = 0x01,
        #[doc = "HSI"]
        Hsi = 0x02,
        #[doc = "HSE"]
        Hse = 0x03,
        #[doc = "HSI48"]
        Hsi48 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl I2c3sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2c3sel {
            unsafe { core::mem::transmute(val & 0x07) }
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum I2c4sel {
        #[doc = "PCLK3"]
        Pclk3 = 0x0,
        #[doc = "IC1"]
        Ic1 = 0x01,
        #[doc = "HSI"]
        Hsi = 0x02,
        #[doc = "HSE"]
        Hse = 0x03,
        #[doc = "HSI48"]
        Hsi48 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl I2c4sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2c4sel {
            unsafe { core::mem::transmute(val & 0x07) }
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum I3c1sel {
        #[doc = "PCLK1"]
        Pclk1 = 0x0,
        #[doc = "IC1"]
        Ic1 = 0x01,
        #[doc = "HSI"]
        Hsi = 0x02,
        #[doc = "HSE"]
        Hse = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl I3c1sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I3c1sel {
            unsafe { core::mem::transmute(val & 0x07) }
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Iclkdiv {
        #[doc = "Intermediate clock 1 division ratio is 5 (default after reset)."]
        Div5 = 0x0,
        #[doc = "Intermediate clock 1 division ratio is 3."]
        Div3 = 0x01,
    }
    impl Iclkdiv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Iclkdiv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Iclkdiv {
        #[inline(always)]
        fn from(val: u8) -> Iclkdiv {
            Iclkdiv::from_bits(val)
        }
    }
    impl From<Iclkdiv> for u8 {
        #[inline(always)]
        fn from(val: Iclkdiv) -> u8 {
            Iclkdiv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lptimsel {
        #[doc = "PCLK3"]
        Pclk3 = 0x0,
        #[doc = "PLL1_Q"]
        Pll1Q = 0x01,
        #[doc = "PLL2_P"]
        Pll2P = 0x02,
        #[doc = "HSE"]
        Hse = 0x03,
        #[doc = "HSI"]
        Hsi = 0x04,
        #[doc = "LSI"]
        Lsi = 0x05,
        #[doc = "LSE"]
        Lse = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Lptimsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lptimsel {
            unsafe { core::mem::transmute(val & 0x07) }
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lpuartsel {
        #[doc = "PCLK3"]
        Pclk3 = 0x0,
        #[doc = "IC1"]
        Ic1 = 0x01,
        #[doc = "PLL2_Q"]
        Pll2Q = 0x02,
        #[doc = "HSE"]
        Hse = 0x03,
        #[doc = "HSI"]
        Hsi = 0x04,
        #[doc = "LSE"]
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lsedrv {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "medium-high drive."]
        MediumHigh = 0x02,
        #[doc = "highest drive."]
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mcopre {
        #[doc = "prescaler disabled, no clock output on MCO1 (default after reset)."]
        Disabled = 0x0,
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mcosel {
        #[doc = "HSI clock selected (hsi_ck) (default after reset)."]
        Hsi = 0x0,
        #[doc = "LSE oscillator clock selected (lse_ck)."]
        Lse = 0x01,
        #[doc = "HSE clock selected (hse_ck)."]
        Hse = 0x02,
        #[doc = "PLL1 clock selected (pll1_q_ck)."]
        Pll1Q = 0x03,
        #[doc = "HSI48 clock selected (hsi48_ker_ck)."]
        Hsi48 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Mcosel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mcosel {
            unsafe { core::mem::transmute(val & 0x07) }
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
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Playdiv {
        #[doc = "Selected kernel is not divided (default after reset)."]
        Div1 = 0x0,
        #[doc = "Selected kernel clock is divided by 2."]
        Div2 = 0x01,
        #[doc = "Selected kernel clock is divided by 4."]
        Div4 = 0x02,
        #[doc = "Selected kernel clock is divided by 8."]
        Div8 = 0x03,
        #[doc = "Selected kernel clock is divided by 16."]
        Div16 = 0x04,
        #[doc = "Selected kernel clock is divided by 32."]
        Div32 = 0x05,
        #[doc = "Selected kernel clock is divided by 64."]
        Div64 = 0x06,
        #[doc = "Selected kernel clock is divided by 128."]
        Div128 = 0x07,
        #[doc = "Selected kernel clock is divided by 256."]
        Div256 = 0x08,
        #[doc = "Selected kernel clock is divided by 512."]
        Div512 = 0x09,
        #[doc = "Selected kernel clock is divided by 1024."]
        Div1024 = 0x0a,
        #[doc = "Selected kernel clock is divided by 2048."]
        Div2048 = 0x0b,
        #[doc = "Selected kernel clock is divided by 4096."]
        Div4096 = 0x0c,
        #[doc = "Selected kernel clock is divided by 8192."]
        Div8192 = 0x0d,
        #[doc = "Selected kernel clock is divided by 16384."]
        Div16384 = 0x0e,
        #[doc = "Selected kernel clock is divided by 32768."]
        Div32768 = 0x0f,
    }
    impl Playdiv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Playdiv {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Playdiv {
        #[inline(always)]
        fn from(val: u8) -> Playdiv {
            Playdiv::from_bits(val)
        }
    }
    impl From<Playdiv> for u8 {
        #[inline(always)]
        fn from(val: Playdiv) -> u8 {
            Playdiv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Playsel {
        #[doc = "LSE"]
        Lse = 0x0,
        #[doc = "LSI"]
        Lsi = 0x01,
        #[doc = "PLL2_P"]
        Pll2P = 0x02,
        #[doc = "HSE"]
        Hse = 0x03,
        #[doc = "HSI"]
        Hsi = 0x04,
        #[doc = "HSI48"]
        Hsi48 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Playsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Playsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Playsel {
        #[inline(always)]
        fn from(val: u8) -> Playsel {
            Playsel::from_bits(val)
        }
    }
    impl From<Playsel> for u8 {
        #[inline(always)]
        fn from(val: Playsel) -> u8 {
            Playsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Plldiv {
        #[doc = "division by 1."]
        Div1 = 0x0,
        #[doc = "division by 2 (default after reset)."]
        Div2 = 0x01,
        #[doc = "division by 3."]
        Div3 = 0x02,
        #[doc = "division by 4."]
        Div4 = 0x03,
        #[doc = "division by 5."]
        Div5 = 0x04,
        #[doc = "division by 6."]
        Div6 = 0x05,
        #[doc = "division by 7."]
        Div7 = 0x06,
        #[doc = "division by 8."]
        Div8 = 0x07,
        #[doc = "division by 9."]
        Div9 = 0x08,
        #[doc = "division by 10."]
        Div10 = 0x09,
        #[doc = "division by 11."]
        Div11 = 0x0a,
        #[doc = "division by 12."]
        Div12 = 0x0b,
        #[doc = "division by 13."]
        Div13 = 0x0c,
        #[doc = "division by 14."]
        Div14 = 0x0d,
        #[doc = "division by 15."]
        Div15 = 0x0e,
        #[doc = "division by 16."]
        Div16 = 0x0f,
        #[doc = "division by 17."]
        Div17 = 0x10,
        #[doc = "division by 18."]
        Div18 = 0x11,
        #[doc = "division by 19."]
        Div19 = 0x12,
        #[doc = "division by 20."]
        Div20 = 0x13,
        #[doc = "division by 21."]
        Div21 = 0x14,
        #[doc = "division by 22."]
        Div22 = 0x15,
        #[doc = "division by 23."]
        Div23 = 0x16,
        #[doc = "division by 24."]
        Div24 = 0x17,
        #[doc = "division by 25."]
        Div25 = 0x18,
        #[doc = "division by 26."]
        Div26 = 0x19,
        #[doc = "division by 27."]
        Div27 = 0x1a,
        #[doc = "division by 28."]
        Div28 = 0x1b,
        #[doc = "division by 29."]
        Div29 = 0x1c,
        #[doc = "division by 30."]
        Div30 = 0x1d,
        #[doc = "division by 31."]
        Div31 = 0x1e,
        #[doc = "division by 32."]
        Div32 = 0x1f,
        #[doc = "division by 33."]
        Div33 = 0x20,
        #[doc = "division by 34."]
        Div34 = 0x21,
        #[doc = "division by 35."]
        Div35 = 0x22,
        #[doc = "division by 36."]
        Div36 = 0x23,
        #[doc = "division by 37."]
        Div37 = 0x24,
        #[doc = "division by 38."]
        Div38 = 0x25,
        #[doc = "division by 39."]
        Div39 = 0x26,
        #[doc = "division by 40."]
        Div40 = 0x27,
        #[doc = "division by 41."]
        Div41 = 0x28,
        #[doc = "division by 42."]
        Div42 = 0x29,
        #[doc = "division by 43."]
        Div43 = 0x2a,
        #[doc = "division by 44."]
        Div44 = 0x2b,
        #[doc = "division by 45."]
        Div45 = 0x2c,
        #[doc = "division by 46."]
        Div46 = 0x2d,
        #[doc = "division by 47."]
        Div47 = 0x2e,
        #[doc = "division by 48."]
        Div48 = 0x2f,
        #[doc = "division by 49."]
        Div49 = 0x30,
        #[doc = "division by 50."]
        Div50 = 0x31,
        #[doc = "division by 51."]
        Div51 = 0x32,
        #[doc = "division by 52."]
        Div52 = 0x33,
        #[doc = "division by 53."]
        Div53 = 0x34,
        #[doc = "division by 54."]
        Div54 = 0x35,
        #[doc = "division by 55."]
        Div55 = 0x36,
        #[doc = "division by 56."]
        Div56 = 0x37,
        #[doc = "division by 57."]
        Div57 = 0x38,
        #[doc = "division by 58."]
        Div58 = 0x39,
        #[doc = "division by 59."]
        Div59 = 0x3a,
        #[doc = "division by 60."]
        Div60 = 0x3b,
        #[doc = "division by 61."]
        Div61 = 0x3c,
        #[doc = "division by 62."]
        Div62 = 0x3d,
        #[doc = "division by 63."]
        Div63 = 0x3e,
        #[doc = "division by 64."]
        Div64 = 0x3f,
        #[doc = "division by 65."]
        Div65 = 0x40,
        #[doc = "division by 66."]
        Div66 = 0x41,
        #[doc = "division by 67."]
        Div67 = 0x42,
        #[doc = "division by 68."]
        Div68 = 0x43,
        #[doc = "division by 69."]
        Div69 = 0x44,
        #[doc = "division by 70."]
        Div70 = 0x45,
        #[doc = "division by 71."]
        Div71 = 0x46,
        #[doc = "division by 72."]
        Div72 = 0x47,
        #[doc = "division by 73."]
        Div73 = 0x48,
        #[doc = "division by 74."]
        Div74 = 0x49,
        #[doc = "division by 75."]
        Div75 = 0x4a,
        #[doc = "division by 76."]
        Div76 = 0x4b,
        #[doc = "division by 77."]
        Div77 = 0x4c,
        #[doc = "division by 78."]
        Div78 = 0x4d,
        #[doc = "division by 79."]
        Div79 = 0x4e,
        #[doc = "division by 80."]
        Div80 = 0x4f,
        #[doc = "division by 81."]
        Div81 = 0x50,
        #[doc = "division by 82."]
        Div82 = 0x51,
        #[doc = "division by 83."]
        Div83 = 0x52,
        #[doc = "division by 84."]
        Div84 = 0x53,
        #[doc = "division by 85."]
        Div85 = 0x54,
        #[doc = "division by 86."]
        Div86 = 0x55,
        #[doc = "division by 87."]
        Div87 = 0x56,
        #[doc = "division by 88."]
        Div88 = 0x57,
        #[doc = "division by 89."]
        Div89 = 0x58,
        #[doc = "division by 90."]
        Div90 = 0x59,
        #[doc = "division by 91."]
        Div91 = 0x5a,
        #[doc = "division by 92."]
        Div92 = 0x5b,
        #[doc = "division by 93."]
        Div93 = 0x5c,
        #[doc = "division by 94."]
        Div94 = 0x5d,
        #[doc = "division by 95."]
        Div95 = 0x5e,
        #[doc = "division by 96."]
        Div96 = 0x5f,
        #[doc = "division by 97."]
        Div97 = 0x60,
        #[doc = "division by 98."]
        Div98 = 0x61,
        #[doc = "division by 99."]
        Div99 = 0x62,
        #[doc = "division by 100."]
        Div100 = 0x63,
        #[doc = "division by 101."]
        Div101 = 0x64,
        #[doc = "division by 102."]
        Div102 = 0x65,
        #[doc = "division by 103."]
        Div103 = 0x66,
        #[doc = "division by 104."]
        Div104 = 0x67,
        #[doc = "division by 105."]
        Div105 = 0x68,
        #[doc = "division by 106."]
        Div106 = 0x69,
        #[doc = "division by 107."]
        Div107 = 0x6a,
        #[doc = "division by 108."]
        Div108 = 0x6b,
        #[doc = "division by 109."]
        Div109 = 0x6c,
        #[doc = "division by 110."]
        Div110 = 0x6d,
        #[doc = "division by 111."]
        Div111 = 0x6e,
        #[doc = "division by 112."]
        Div112 = 0x6f,
        #[doc = "division by 113."]
        Div113 = 0x70,
        #[doc = "division by 114."]
        Div114 = 0x71,
        #[doc = "division by 115."]
        Div115 = 0x72,
        #[doc = "division by 116."]
        Div116 = 0x73,
        #[doc = "division by 117."]
        Div117 = 0x74,
        #[doc = "division by 118."]
        Div118 = 0x75,
        #[doc = "division by 119."]
        Div119 = 0x76,
        #[doc = "division by 120."]
        Div120 = 0x77,
        #[doc = "division by 121."]
        Div121 = 0x78,
        #[doc = "division by 122."]
        Div122 = 0x79,
        #[doc = "division by 123."]
        Div123 = 0x7a,
        #[doc = "division by 124."]
        Div124 = 0x7b,
        #[doc = "division by 125."]
        Div125 = 0x7c,
        #[doc = "division by 126."]
        Div126 = 0x7d,
        #[doc = "division by 127."]
        Div127 = 0x7e,
        #[doc = "division by 128."]
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllm {
        #[doc = "prescaler disabled (default after reset)."]
        Disabled = 0x0,
        #[doc = "division by 1 (bypass)."]
        Div1 = 0x01,
        #[doc = "division by 2."]
        Div2 = 0x02,
        #[doc = "division by 3."]
        Div3 = 0x03,
        #[doc = "division by 4."]
        Div4 = 0x04,
        #[doc = "division by 5."]
        Div5 = 0x05,
        #[doc = "division by 6."]
        Div6 = 0x06,
        #[doc = "division by 7."]
        Div7 = 0x07,
        #[doc = "division by 8."]
        Div8 = 0x08,
        #[doc = "division by 9."]
        Div9 = 0x09,
        #[doc = "division by 10."]
        Div10 = 0x0a,
        #[doc = "division by 11."]
        Div11 = 0x0b,
        #[doc = "division by 12."]
        Div12 = 0x0c,
        #[doc = "division by 13."]
        Div13 = 0x0d,
        #[doc = "division by 14."]
        Div14 = 0x0e,
        #[doc = "division by 15."]
        Div15 = 0x0f,
        #[doc = "division by 16."]
        Div16 = 0x10,
        #[doc = "division by 17."]
        Div17 = 0x11,
        #[doc = "division by 18."]
        Div18 = 0x12,
        #[doc = "division by 19."]
        Div19 = 0x13,
        #[doc = "division by 20."]
        Div20 = 0x14,
        #[doc = "division by 21."]
        Div21 = 0x15,
        #[doc = "division by 22."]
        Div22 = 0x16,
        #[doc = "division by 23."]
        Div23 = 0x17,
        #[doc = "division by 24."]
        Div24 = 0x18,
        #[doc = "division by 25."]
        Div25 = 0x19,
        #[doc = "division by 26."]
        Div26 = 0x1a,
        #[doc = "division by 27."]
        Div27 = 0x1b,
        #[doc = "division by 28."]
        Div28 = 0x1c,
        #[doc = "division by 29."]
        Div29 = 0x1d,
        #[doc = "division by 30."]
        Div30 = 0x1e,
        #[doc = "division by 31."]
        Div31 = 0x1f,
        #[doc = "division by 32."]
        Div32 = 0x20,
        #[doc = "division by 33."]
        Div33 = 0x21,
        #[doc = "division by 34."]
        Div34 = 0x22,
        #[doc = "division by 35."]
        Div35 = 0x23,
        #[doc = "division by 36."]
        Div36 = 0x24,
        #[doc = "division by 37."]
        Div37 = 0x25,
        #[doc = "division by 38."]
        Div38 = 0x26,
        #[doc = "division by 39."]
        Div39 = 0x27,
        #[doc = "division by 40."]
        Div40 = 0x28,
        #[doc = "division by 41."]
        Div41 = 0x29,
        #[doc = "division by 42."]
        Div42 = 0x2a,
        #[doc = "division by 43."]
        Div43 = 0x2b,
        #[doc = "division by 44."]
        Div44 = 0x2c,
        #[doc = "division by 45."]
        Div45 = 0x2d,
        #[doc = "division by 46."]
        Div46 = 0x2e,
        #[doc = "division by 47."]
        Div47 = 0x2f,
        #[doc = "division by 48."]
        Div48 = 0x30,
        #[doc = "division by 49."]
        Div49 = 0x31,
        #[doc = "division by 50."]
        Div50 = 0x32,
        #[doc = "division by 51."]
        Div51 = 0x33,
        #[doc = "division by 52."]
        Div52 = 0x34,
        #[doc = "division by 53."]
        Div53 = 0x35,
        #[doc = "division by 54."]
        Div54 = 0x36,
        #[doc = "division by 55."]
        Div55 = 0x37,
        #[doc = "division by 56."]
        Div56 = 0x38,
        #[doc = "division by 57."]
        Div57 = 0x39,
        #[doc = "division by 58."]
        Div58 = 0x3a,
        #[doc = "division by 59."]
        Div59 = 0x3b,
        #[doc = "division by 60."]
        Div60 = 0x3c,
        #[doc = "division by 61."]
        Div61 = 0x3d,
        #[doc = "division by 62."]
        Div62 = 0x3e,
        #[doc = "division by 63."]
        Div63 = 0x3f,
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Plln {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "PLLN=4."]
        Mul4 = 0x03,
        #[doc = "PLLN=5."]
        Mul5 = 0x04,
        #[doc = "PLLN=6."]
        Mul6 = 0x05,
        #[doc = "PLLN=7."]
        Mul7 = 0x06,
        #[doc = "PLLN=8."]
        Mul8 = 0x07,
        #[doc = "PLLN=9."]
        Mul9 = 0x08,
        #[doc = "PLLN=10."]
        Mul10 = 0x09,
        #[doc = "PLLN=11."]
        Mul11 = 0x0a,
        #[doc = "PLLN=12."]
        Mul12 = 0x0b,
        #[doc = "PLLN=13."]
        Mul13 = 0x0c,
        #[doc = "PLLN=14."]
        Mul14 = 0x0d,
        #[doc = "PLLN=15."]
        Mul15 = 0x0e,
        #[doc = "PLLN=16."]
        Mul16 = 0x0f,
        #[doc = "PLLN=17."]
        Mul17 = 0x10,
        #[doc = "PLLN=18."]
        Mul18 = 0x11,
        #[doc = "PLLN=19."]
        Mul19 = 0x12,
        #[doc = "PLLN=20."]
        Mul20 = 0x13,
        #[doc = "PLLN=21."]
        Mul21 = 0x14,
        #[doc = "PLLN=22."]
        Mul22 = 0x15,
        #[doc = "PLLN=23."]
        Mul23 = 0x16,
        #[doc = "PLLN=24."]
        Mul24 = 0x17,
        #[doc = "PLLN=25."]
        Mul25 = 0x18,
        #[doc = "PLLN=26."]
        Mul26 = 0x19,
        #[doc = "PLLN=27."]
        Mul27 = 0x1a,
        #[doc = "PLLN=28."]
        Mul28 = 0x1b,
        #[doc = "PLLN=29."]
        Mul29 = 0x1c,
        #[doc = "PLLN=30."]
        Mul30 = 0x1d,
        #[doc = "PLLN=31."]
        Mul31 = 0x1e,
        #[doc = "PLLN=32."]
        Mul32 = 0x1f,
        #[doc = "PLLN=33."]
        Mul33 = 0x20,
        #[doc = "PLLN=34."]
        Mul34 = 0x21,
        #[doc = "PLLN=35."]
        Mul35 = 0x22,
        #[doc = "PLLN=36."]
        Mul36 = 0x23,
        #[doc = "PLLN=37."]
        Mul37 = 0x24,
        #[doc = "PLLN=38."]
        Mul38 = 0x25,
        #[doc = "PLLN=39."]
        Mul39 = 0x26,
        #[doc = "PLLN=40."]
        Mul40 = 0x27,
        #[doc = "PLLN=41."]
        Mul41 = 0x28,
        #[doc = "PLLN=42."]
        Mul42 = 0x29,
        #[doc = "PLLN=43."]
        Mul43 = 0x2a,
        #[doc = "PLLN=44."]
        Mul44 = 0x2b,
        #[doc = "PLLN=45."]
        Mul45 = 0x2c,
        #[doc = "PLLN=46."]
        Mul46 = 0x2d,
        #[doc = "PLLN=47."]
        Mul47 = 0x2e,
        #[doc = "PLLN=48."]
        Mul48 = 0x2f,
        #[doc = "PLLN=49."]
        Mul49 = 0x30,
        #[doc = "PLLN=50."]
        Mul50 = 0x31,
        #[doc = "PLLN=51."]
        Mul51 = 0x32,
        #[doc = "PLLN=52."]
        Mul52 = 0x33,
        #[doc = "PLLN=53."]
        Mul53 = 0x34,
        #[doc = "PLLN=54."]
        Mul54 = 0x35,
        #[doc = "PLLN=55."]
        Mul55 = 0x36,
        #[doc = "PLLN=56."]
        Mul56 = 0x37,
        #[doc = "PLLN=57."]
        Mul57 = 0x38,
        #[doc = "PLLN=58."]
        Mul58 = 0x39,
        #[doc = "PLLN=59."]
        Mul59 = 0x3a,
        #[doc = "PLLN=60."]
        Mul60 = 0x3b,
        #[doc = "PLLN=61."]
        Mul61 = 0x3c,
        #[doc = "PLLN=62."]
        Mul62 = 0x3d,
        #[doc = "PLLN=63."]
        Mul63 = 0x3e,
        #[doc = "PLLN=64."]
        Mul64 = 0x3f,
        #[doc = "PLLN=65."]
        Mul65 = 0x40,
        #[doc = "PLLN=66."]
        Mul66 = 0x41,
        #[doc = "PLLN=67."]
        Mul67 = 0x42,
        #[doc = "PLLN=68."]
        Mul68 = 0x43,
        #[doc = "PLLN=69."]
        Mul69 = 0x44,
        #[doc = "PLLN=70."]
        Mul70 = 0x45,
        #[doc = "PLLN=71."]
        Mul71 = 0x46,
        #[doc = "PLLN=72."]
        Mul72 = 0x47,
        #[doc = "PLLN=73."]
        Mul73 = 0x48,
        #[doc = "PLLN=74."]
        Mul74 = 0x49,
        #[doc = "PLLN=75."]
        Mul75 = 0x4a,
        #[doc = "PLLN=76."]
        Mul76 = 0x4b,
        #[doc = "PLLN=77."]
        Mul77 = 0x4c,
        #[doc = "PLLN=78."]
        Mul78 = 0x4d,
        #[doc = "PLLN=79."]
        Mul79 = 0x4e,
        #[doc = "PLLN=80."]
        Mul80 = 0x4f,
        #[doc = "PLLN=81."]
        Mul81 = 0x50,
        #[doc = "PLLN=82."]
        Mul82 = 0x51,
        #[doc = "PLLN=83."]
        Mul83 = 0x52,
        #[doc = "PLLN=84."]
        Mul84 = 0x53,
        #[doc = "PLLN=85."]
        Mul85 = 0x54,
        #[doc = "PLLN=86."]
        Mul86 = 0x55,
        #[doc = "PLLN=87."]
        Mul87 = 0x56,
        #[doc = "PLLN=88."]
        Mul88 = 0x57,
        #[doc = "PLLN=89."]
        Mul89 = 0x58,
        #[doc = "PLLN=90."]
        Mul90 = 0x59,
        #[doc = "PLLN=91."]
        Mul91 = 0x5a,
        #[doc = "PLLN=92."]
        Mul92 = 0x5b,
        #[doc = "PLLN=93."]
        Mul93 = 0x5c,
        #[doc = "PLLN=94."]
        Mul94 = 0x5d,
        #[doc = "PLLN=95."]
        Mul95 = 0x5e,
        #[doc = "PLLN=96."]
        Mul96 = 0x5f,
        #[doc = "PLLN=97."]
        Mul97 = 0x60,
        #[doc = "PLLN=98."]
        Mul98 = 0x61,
        #[doc = "PLLN=99."]
        Mul99 = 0x62,
        #[doc = "PLLN=100."]
        Mul100 = 0x63,
        #[doc = "PLLN=101."]
        Mul101 = 0x64,
        #[doc = "PLLN=102."]
        Mul102 = 0x65,
        #[doc = "PLLN=103."]
        Mul103 = 0x66,
        #[doc = "PLLN=104."]
        Mul104 = 0x67,
        #[doc = "PLLN=105."]
        Mul105 = 0x68,
        #[doc = "PLLN=106."]
        Mul106 = 0x69,
        #[doc = "PLLN=107."]
        Mul107 = 0x6a,
        #[doc = "PLLN=108."]
        Mul108 = 0x6b,
        #[doc = "PLLN=109."]
        Mul109 = 0x6c,
        #[doc = "PLLN=110."]
        Mul110 = 0x6d,
        #[doc = "PLLN=111."]
        Mul111 = 0x6e,
        #[doc = "PLLN=112."]
        Mul112 = 0x6f,
        #[doc = "PLLN=113."]
        Mul113 = 0x70,
        #[doc = "PLLN=114."]
        Mul114 = 0x71,
        #[doc = "PLLN=115."]
        Mul115 = 0x72,
        #[doc = "PLLN=116."]
        Mul116 = 0x73,
        #[doc = "PLLN=117."]
        Mul117 = 0x74,
        #[doc = "PLLN=118."]
        Mul118 = 0x75,
        #[doc = "PLLN=119."]
        Mul119 = 0x76,
        #[doc = "PLLN=120."]
        Mul120 = 0x77,
        #[doc = "PLLN=121."]
        Mul121 = 0x78,
        #[doc = "PLLN=122."]
        Mul122 = 0x79,
        #[doc = "PLLN=123."]
        Mul123 = 0x7a,
        #[doc = "PLLN=124."]
        Mul124 = 0x7b,
        #[doc = "PLLN=125."]
        Mul125 = 0x7c,
        #[doc = "PLLN=126."]
        Mul126 = 0x7d,
        #[doc = "PLLN=127."]
        Mul127 = 0x7e,
        #[doc = "PLLN=128."]
        Mul128 = 0x7f,
        #[doc = "PLLN=129 (default after reset)."]
        Mul129 = 0x80,
        #[doc = "PLLN=130."]
        Mul130 = 0x81,
        #[doc = "PLLN=131."]
        Mul131 = 0x82,
        #[doc = "PLLN=132."]
        Mul132 = 0x83,
        #[doc = "PLLN=133."]
        Mul133 = 0x84,
        #[doc = "PLLN=134."]
        Mul134 = 0x85,
        #[doc = "PLLN=135."]
        Mul135 = 0x86,
        #[doc = "PLLN=136."]
        Mul136 = 0x87,
        #[doc = "PLLN=137."]
        Mul137 = 0x88,
        #[doc = "PLLN=138."]
        Mul138 = 0x89,
        #[doc = "PLLN=139."]
        Mul139 = 0x8a,
        #[doc = "PLLN=140."]
        Mul140 = 0x8b,
        #[doc = "PLLN=141."]
        Mul141 = 0x8c,
        #[doc = "PLLN=142."]
        Mul142 = 0x8d,
        #[doc = "PLLN=143."]
        Mul143 = 0x8e,
        #[doc = "PLLN=144."]
        Mul144 = 0x8f,
        #[doc = "PLLN=145."]
        Mul145 = 0x90,
        #[doc = "PLLN=146."]
        Mul146 = 0x91,
        #[doc = "PLLN=147."]
        Mul147 = 0x92,
        #[doc = "PLLN=148."]
        Mul148 = 0x93,
        #[doc = "PLLN=149."]
        Mul149 = 0x94,
        #[doc = "PLLN=150."]
        Mul150 = 0x95,
        #[doc = "PLLN=151."]
        Mul151 = 0x96,
        #[doc = "PLLN=152."]
        Mul152 = 0x97,
        #[doc = "PLLN=153."]
        Mul153 = 0x98,
        #[doc = "PLLN=154."]
        Mul154 = 0x99,
        #[doc = "PLLN=155."]
        Mul155 = 0x9a,
        #[doc = "PLLN=156."]
        Mul156 = 0x9b,
        #[doc = "PLLN=157."]
        Mul157 = 0x9c,
        #[doc = "PLLN=158."]
        Mul158 = 0x9d,
        #[doc = "PLLN=159."]
        Mul159 = 0x9e,
        #[doc = "PLLN=160."]
        Mul160 = 0x9f,
        #[doc = "PLLN=161."]
        Mul161 = 0xa0,
        #[doc = "PLLN=162."]
        Mul162 = 0xa1,
        #[doc = "PLLN=163."]
        Mul163 = 0xa2,
        #[doc = "PLLN=164."]
        Mul164 = 0xa3,
        #[doc = "PLLN=165."]
        Mul165 = 0xa4,
        #[doc = "PLLN=166."]
        Mul166 = 0xa5,
        #[doc = "PLLN=167."]
        Mul167 = 0xa6,
        #[doc = "PLLN=168."]
        Mul168 = 0xa7,
        #[doc = "PLLN=169."]
        Mul169 = 0xa8,
        #[doc = "PLLN=170."]
        Mul170 = 0xa9,
        #[doc = "PLLN=171."]
        Mul171 = 0xaa,
        #[doc = "PLLN=172."]
        Mul172 = 0xab,
        #[doc = "PLLN=173."]
        Mul173 = 0xac,
        #[doc = "PLLN=174."]
        Mul174 = 0xad,
        #[doc = "PLLN=175."]
        Mul175 = 0xae,
        #[doc = "PLLN=176."]
        Mul176 = 0xaf,
        #[doc = "PLLN=177."]
        Mul177 = 0xb0,
        #[doc = "PLLN=178."]
        Mul178 = 0xb1,
        #[doc = "PLLN=179."]
        Mul179 = 0xb2,
        #[doc = "PLLN=180."]
        Mul180 = 0xb3,
        #[doc = "PLLN=181."]
        Mul181 = 0xb4,
        #[doc = "PLLN=182."]
        Mul182 = 0xb5,
        #[doc = "PLLN=183."]
        Mul183 = 0xb6,
        #[doc = "PLLN=184."]
        Mul184 = 0xb7,
        #[doc = "PLLN=185."]
        Mul185 = 0xb8,
        #[doc = "PLLN=186."]
        Mul186 = 0xb9,
        #[doc = "PLLN=187."]
        Mul187 = 0xba,
        #[doc = "PLLN=188."]
        Mul188 = 0xbb,
        #[doc = "PLLN=189."]
        Mul189 = 0xbc,
        #[doc = "PLLN=190."]
        Mul190 = 0xbd,
        #[doc = "PLLN=191."]
        Mul191 = 0xbe,
        #[doc = "PLLN=192."]
        Mul192 = 0xbf,
        #[doc = "PLLN=193."]
        Mul193 = 0xc0,
        #[doc = "PLLN=194."]
        Mul194 = 0xc1,
        #[doc = "PLLN=195."]
        Mul195 = 0xc2,
        #[doc = "PLLN=196."]
        Mul196 = 0xc3,
        #[doc = "PLLN=197."]
        Mul197 = 0xc4,
        #[doc = "PLLN=198."]
        Mul198 = 0xc5,
        #[doc = "PLLN=199."]
        Mul199 = 0xc6,
        #[doc = "PLLN=200."]
        Mul200 = 0xc7,
        #[doc = "PLLN=201."]
        Mul201 = 0xc8,
        #[doc = "PLLN=202."]
        Mul202 = 0xc9,
        #[doc = "PLLN=203."]
        Mul203 = 0xca,
        #[doc = "PLLN=204."]
        Mul204 = 0xcb,
        #[doc = "PLLN=205."]
        Mul205 = 0xcc,
        #[doc = "PLLN=206."]
        Mul206 = 0xcd,
        #[doc = "PLLN=207."]
        Mul207 = 0xce,
        #[doc = "PLLN=208."]
        Mul208 = 0xcf,
        #[doc = "PLLN=209."]
        Mul209 = 0xd0,
        #[doc = "PLLN=210."]
        Mul210 = 0xd1,
        #[doc = "PLLN=211."]
        Mul211 = 0xd2,
        #[doc = "PLLN=212."]
        Mul212 = 0xd3,
        #[doc = "PLLN=213."]
        Mul213 = 0xd4,
        #[doc = "PLLN=214."]
        Mul214 = 0xd5,
        #[doc = "PLLN=215."]
        Mul215 = 0xd6,
        #[doc = "PLLN=216."]
        Mul216 = 0xd7,
        #[doc = "PLLN=217."]
        Mul217 = 0xd8,
        #[doc = "PLLN=218."]
        Mul218 = 0xd9,
        #[doc = "PLLN=219."]
        Mul219 = 0xda,
        #[doc = "PLLN=220."]
        Mul220 = 0xdb,
        #[doc = "PLLN=221."]
        Mul221 = 0xdc,
        #[doc = "PLLN=222."]
        Mul222 = 0xdd,
        #[doc = "PLLN=223."]
        Mul223 = 0xde,
        #[doc = "PLLN=224."]
        Mul224 = 0xdf,
        #[doc = "PLLN=225."]
        Mul225 = 0xe0,
        #[doc = "PLLN=226."]
        Mul226 = 0xe1,
        #[doc = "PLLN=227."]
        Mul227 = 0xe2,
        #[doc = "PLLN=228."]
        Mul228 = 0xe3,
        #[doc = "PLLN=229."]
        Mul229 = 0xe4,
        #[doc = "PLLN=230."]
        Mul230 = 0xe5,
        #[doc = "PLLN=231."]
        Mul231 = 0xe6,
        #[doc = "PLLN=232."]
        Mul232 = 0xe7,
        #[doc = "PLLN=233."]
        Mul233 = 0xe8,
        #[doc = "PLLN=234."]
        Mul234 = 0xe9,
        #[doc = "PLLN=235."]
        Mul235 = 0xea,
        #[doc = "PLLN=236."]
        Mul236 = 0xeb,
        #[doc = "PLLN=237."]
        Mul237 = 0xec,
        #[doc = "PLLN=238."]
        Mul238 = 0xed,
        #[doc = "PLLN=239."]
        Mul239 = 0xee,
        #[doc = "PLLN=240."]
        Mul240 = 0xef,
        #[doc = "PLLN=241."]
        Mul241 = 0xf0,
        #[doc = "PLLN=242."]
        Mul242 = 0xf1,
        #[doc = "PLLN=243."]
        Mul243 = 0xf2,
        #[doc = "PLLN=244."]
        Mul244 = 0xf3,
        #[doc = "PLLN=245."]
        Mul245 = 0xf4,
        #[doc = "PLLN=246."]
        Mul246 = 0xf5,
        #[doc = "PLLN=247."]
        Mul247 = 0xf6,
        #[doc = "PLLN=248."]
        Mul248 = 0xf7,
        #[doc = "PLLN=249."]
        Mul249 = 0xf8,
        #[doc = "PLLN=250."]
        Mul250 = 0xf9,
        #[doc = "PLLN=251."]
        Mul251 = 0xfa,
        #[doc = "PLLN=252."]
        Mul252 = 0xfb,
        #[doc = "PLLN=253."]
        Mul253 = 0xfc,
        #[doc = "PLLN=254."]
        Mul254 = 0xfd,
        #[doc = "PLLN=255."]
        Mul255 = 0xfe,
        #[doc = "PLLN=256."]
        Mul256 = 0xff,
        #[doc = "PLLN=257."]
        Mul257 = 0x0100,
        #[doc = "PLLN=258."]
        Mul258 = 0x0101,
        #[doc = "PLLN=259."]
        Mul259 = 0x0102,
        #[doc = "PLLN=260."]
        Mul260 = 0x0103,
        #[doc = "PLLN=261."]
        Mul261 = 0x0104,
        #[doc = "PLLN=262."]
        Mul262 = 0x0105,
        #[doc = "PLLN=263."]
        Mul263 = 0x0106,
        #[doc = "PLLN=264."]
        Mul264 = 0x0107,
        #[doc = "PLLN=265."]
        Mul265 = 0x0108,
        #[doc = "PLLN=266."]
        Mul266 = 0x0109,
        #[doc = "PLLN=267."]
        Mul267 = 0x010a,
        #[doc = "PLLN=268."]
        Mul268 = 0x010b,
        #[doc = "PLLN=269."]
        Mul269 = 0x010c,
        #[doc = "PLLN=270."]
        Mul270 = 0x010d,
        #[doc = "PLLN=271."]
        Mul271 = 0x010e,
        #[doc = "PLLN=272."]
        Mul272 = 0x010f,
        #[doc = "PLLN=273."]
        Mul273 = 0x0110,
        #[doc = "PLLN=274."]
        Mul274 = 0x0111,
        #[doc = "PLLN=275."]
        Mul275 = 0x0112,
        #[doc = "PLLN=276."]
        Mul276 = 0x0113,
        #[doc = "PLLN=277."]
        Mul277 = 0x0114,
        #[doc = "PLLN=278."]
        Mul278 = 0x0115,
        #[doc = "PLLN=279."]
        Mul279 = 0x0116,
        #[doc = "PLLN=280."]
        Mul280 = 0x0117,
        #[doc = "PLLN=281."]
        Mul281 = 0x0118,
        #[doc = "PLLN=282."]
        Mul282 = 0x0119,
        #[doc = "PLLN=283."]
        Mul283 = 0x011a,
        #[doc = "PLLN=284."]
        Mul284 = 0x011b,
        #[doc = "PLLN=285."]
        Mul285 = 0x011c,
        #[doc = "PLLN=286."]
        Mul286 = 0x011d,
        #[doc = "PLLN=287."]
        Mul287 = 0x011e,
        #[doc = "PLLN=288."]
        Mul288 = 0x011f,
        #[doc = "PLLN=289."]
        Mul289 = 0x0120,
        #[doc = "PLLN=290."]
        Mul290 = 0x0121,
        #[doc = "PLLN=291."]
        Mul291 = 0x0122,
        #[doc = "PLLN=292."]
        Mul292 = 0x0123,
        #[doc = "PLLN=293."]
        Mul293 = 0x0124,
        #[doc = "PLLN=294."]
        Mul294 = 0x0125,
        #[doc = "PLLN=295."]
        Mul295 = 0x0126,
        #[doc = "PLLN=296."]
        Mul296 = 0x0127,
        #[doc = "PLLN=297."]
        Mul297 = 0x0128,
        #[doc = "PLLN=298."]
        Mul298 = 0x0129,
        #[doc = "PLLN=299."]
        Mul299 = 0x012a,
        #[doc = "PLLN=300."]
        Mul300 = 0x012b,
        #[doc = "PLLN=301."]
        Mul301 = 0x012c,
        #[doc = "PLLN=302."]
        Mul302 = 0x012d,
        #[doc = "PLLN=303."]
        Mul303 = 0x012e,
        #[doc = "PLLN=304."]
        Mul304 = 0x012f,
        #[doc = "PLLN=305."]
        Mul305 = 0x0130,
        #[doc = "PLLN=306."]
        Mul306 = 0x0131,
        #[doc = "PLLN=307."]
        Mul307 = 0x0132,
        #[doc = "PLLN=308."]
        Mul308 = 0x0133,
        #[doc = "PLLN=309."]
        Mul309 = 0x0134,
        #[doc = "PLLN=310."]
        Mul310 = 0x0135,
        #[doc = "PLLN=311."]
        Mul311 = 0x0136,
        #[doc = "PLLN=312."]
        Mul312 = 0x0137,
        #[doc = "PLLN=313."]
        Mul313 = 0x0138,
        #[doc = "PLLN=314."]
        Mul314 = 0x0139,
        #[doc = "PLLN=315."]
        Mul315 = 0x013a,
        #[doc = "PLLN=316."]
        Mul316 = 0x013b,
        #[doc = "PLLN=317."]
        Mul317 = 0x013c,
        #[doc = "PLLN=318."]
        Mul318 = 0x013d,
        #[doc = "PLLN=319."]
        Mul319 = 0x013e,
        #[doc = "PLLN=320."]
        Mul320 = 0x013f,
        #[doc = "PLLN=321."]
        Mul321 = 0x0140,
        #[doc = "PLLN=322."]
        Mul322 = 0x0141,
        #[doc = "PLLN=323."]
        Mul323 = 0x0142,
        #[doc = "PLLN=324."]
        Mul324 = 0x0143,
        #[doc = "PLLN=325."]
        Mul325 = 0x0144,
        #[doc = "PLLN=326."]
        Mul326 = 0x0145,
        #[doc = "PLLN=327."]
        Mul327 = 0x0146,
        #[doc = "PLLN=328."]
        Mul328 = 0x0147,
        #[doc = "PLLN=329."]
        Mul329 = 0x0148,
        #[doc = "PLLN=330."]
        Mul330 = 0x0149,
        #[doc = "PLLN=331."]
        Mul331 = 0x014a,
        #[doc = "PLLN=332."]
        Mul332 = 0x014b,
        #[doc = "PLLN=333."]
        Mul333 = 0x014c,
        #[doc = "PLLN=334."]
        Mul334 = 0x014d,
        #[doc = "PLLN=335."]
        Mul335 = 0x014e,
        #[doc = "PLLN=336."]
        Mul336 = 0x014f,
        #[doc = "PLLN=337."]
        Mul337 = 0x0150,
        #[doc = "PLLN=338."]
        Mul338 = 0x0151,
        #[doc = "PLLN=339."]
        Mul339 = 0x0152,
        #[doc = "PLLN=340."]
        Mul340 = 0x0153,
        #[doc = "PLLN=341."]
        Mul341 = 0x0154,
        #[doc = "PLLN=342."]
        Mul342 = 0x0155,
        #[doc = "PLLN=343."]
        Mul343 = 0x0156,
        #[doc = "PLLN=344."]
        Mul344 = 0x0157,
        #[doc = "PLLN=345."]
        Mul345 = 0x0158,
        #[doc = "PLLN=346."]
        Mul346 = 0x0159,
        #[doc = "PLLN=347."]
        Mul347 = 0x015a,
        #[doc = "PLLN=348."]
        Mul348 = 0x015b,
        #[doc = "PLLN=349."]
        Mul349 = 0x015c,
        #[doc = "PLLN=350."]
        Mul350 = 0x015d,
        #[doc = "PLLN=351."]
        Mul351 = 0x015e,
        #[doc = "PLLN=352."]
        Mul352 = 0x015f,
        #[doc = "PLLN=353."]
        Mul353 = 0x0160,
        #[doc = "PLLN=354."]
        Mul354 = 0x0161,
        #[doc = "PLLN=355."]
        Mul355 = 0x0162,
        #[doc = "PLLN=356."]
        Mul356 = 0x0163,
        #[doc = "PLLN=357."]
        Mul357 = 0x0164,
        #[doc = "PLLN=358."]
        Mul358 = 0x0165,
        #[doc = "PLLN=359."]
        Mul359 = 0x0166,
        #[doc = "PLLN=360."]
        Mul360 = 0x0167,
        #[doc = "PLLN=361."]
        Mul361 = 0x0168,
        #[doc = "PLLN=362."]
        Mul362 = 0x0169,
        #[doc = "PLLN=363."]
        Mul363 = 0x016a,
        #[doc = "PLLN=364."]
        Mul364 = 0x016b,
        #[doc = "PLLN=365."]
        Mul365 = 0x016c,
        #[doc = "PLLN=366."]
        Mul366 = 0x016d,
        #[doc = "PLLN=367."]
        Mul367 = 0x016e,
        #[doc = "PLLN=368."]
        Mul368 = 0x016f,
        #[doc = "PLLN=369."]
        Mul369 = 0x0170,
        #[doc = "PLLN=370."]
        Mul370 = 0x0171,
        #[doc = "PLLN=371."]
        Mul371 = 0x0172,
        #[doc = "PLLN=372."]
        Mul372 = 0x0173,
        #[doc = "PLLN=373."]
        Mul373 = 0x0174,
        #[doc = "PLLN=374."]
        Mul374 = 0x0175,
        #[doc = "PLLN=375."]
        Mul375 = 0x0176,
        #[doc = "PLLN=376."]
        Mul376 = 0x0177,
        #[doc = "PLLN=377."]
        Mul377 = 0x0178,
        #[doc = "PLLN=378."]
        Mul378 = 0x0179,
        #[doc = "PLLN=379."]
        Mul379 = 0x017a,
        #[doc = "PLLN=380."]
        Mul380 = 0x017b,
        #[doc = "PLLN=381."]
        Mul381 = 0x017c,
        #[doc = "PLLN=382."]
        Mul382 = 0x017d,
        #[doc = "PLLN=383."]
        Mul383 = 0x017e,
        #[doc = "PLLN=384."]
        Mul384 = 0x017f,
        #[doc = "PLLN=385."]
        Mul385 = 0x0180,
        #[doc = "PLLN=386."]
        Mul386 = 0x0181,
        #[doc = "PLLN=387."]
        Mul387 = 0x0182,
        #[doc = "PLLN=388."]
        Mul388 = 0x0183,
        #[doc = "PLLN=389."]
        Mul389 = 0x0184,
        #[doc = "PLLN=390."]
        Mul390 = 0x0185,
        #[doc = "PLLN=391."]
        Mul391 = 0x0186,
        #[doc = "PLLN=392."]
        Mul392 = 0x0187,
        #[doc = "PLLN=393."]
        Mul393 = 0x0188,
        #[doc = "PLLN=394."]
        Mul394 = 0x0189,
        #[doc = "PLLN=395."]
        Mul395 = 0x018a,
        #[doc = "PLLN=396."]
        Mul396 = 0x018b,
        #[doc = "PLLN=397."]
        Mul397 = 0x018c,
        #[doc = "PLLN=398."]
        Mul398 = 0x018d,
        #[doc = "PLLN=399."]
        Mul399 = 0x018e,
        #[doc = "PLLN=400."]
        Mul400 = 0x018f,
        #[doc = "PLLN=401."]
        Mul401 = 0x0190,
        #[doc = "PLLN=402."]
        Mul402 = 0x0191,
        #[doc = "PLLN=403."]
        Mul403 = 0x0192,
        #[doc = "PLLN=404."]
        Mul404 = 0x0193,
        #[doc = "PLLN=405."]
        Mul405 = 0x0194,
        #[doc = "PLLN=406."]
        Mul406 = 0x0195,
        #[doc = "PLLN=407."]
        Mul407 = 0x0196,
        #[doc = "PLLN=408."]
        Mul408 = 0x0197,
        #[doc = "PLLN=409."]
        Mul409 = 0x0198,
        #[doc = "PLLN=410."]
        Mul410 = 0x0199,
        #[doc = "PLLN=411."]
        Mul411 = 0x019a,
        #[doc = "PLLN=412."]
        Mul412 = 0x019b,
        #[doc = "PLLN=413."]
        Mul413 = 0x019c,
        #[doc = "PLLN=414."]
        Mul414 = 0x019d,
        #[doc = "PLLN=415."]
        Mul415 = 0x019e,
        #[doc = "PLLN=416."]
        Mul416 = 0x019f,
        #[doc = "PLLN=417."]
        Mul417 = 0x01a0,
        #[doc = "PLLN=418."]
        Mul418 = 0x01a1,
        #[doc = "PLLN=419."]
        Mul419 = 0x01a2,
        #[doc = "PLLN=420."]
        Mul420 = 0x01a3,
        #[doc = "PLLN=421."]
        Mul421 = 0x01a4,
        #[doc = "PLLN=422."]
        Mul422 = 0x01a5,
        #[doc = "PLLN=423."]
        Mul423 = 0x01a6,
        #[doc = "PLLN=424."]
        Mul424 = 0x01a7,
        #[doc = "PLLN=425."]
        Mul425 = 0x01a8,
        #[doc = "PLLN=426."]
        Mul426 = 0x01a9,
        #[doc = "PLLN=427."]
        Mul427 = 0x01aa,
        #[doc = "PLLN=428."]
        Mul428 = 0x01ab,
        #[doc = "PLLN=429."]
        Mul429 = 0x01ac,
        #[doc = "PLLN=430."]
        Mul430 = 0x01ad,
        #[doc = "PLLN=431."]
        Mul431 = 0x01ae,
        #[doc = "PLLN=432."]
        Mul432 = 0x01af,
        #[doc = "PLLN=433."]
        Mul433 = 0x01b0,
        #[doc = "PLLN=434."]
        Mul434 = 0x01b1,
        #[doc = "PLLN=435."]
        Mul435 = 0x01b2,
        #[doc = "PLLN=436."]
        Mul436 = 0x01b3,
        #[doc = "PLLN=437."]
        Mul437 = 0x01b4,
        #[doc = "PLLN=438."]
        Mul438 = 0x01b5,
        #[doc = "PLLN=439."]
        Mul439 = 0x01b6,
        #[doc = "PLLN=440."]
        Mul440 = 0x01b7,
        #[doc = "PLLN=441."]
        Mul441 = 0x01b8,
        #[doc = "PLLN=442."]
        Mul442 = 0x01b9,
        #[doc = "PLLN=443."]
        Mul443 = 0x01ba,
        #[doc = "PLLN=444."]
        Mul444 = 0x01bb,
        #[doc = "PLLN=445."]
        Mul445 = 0x01bc,
        #[doc = "PLLN=446."]
        Mul446 = 0x01bd,
        #[doc = "PLLN=447."]
        Mul447 = 0x01be,
        #[doc = "PLLN=448."]
        Mul448 = 0x01bf,
        #[doc = "PLLN=449."]
        Mul449 = 0x01c0,
        #[doc = "PLLN=450."]
        Mul450 = 0x01c1,
        #[doc = "PLLN=451."]
        Mul451 = 0x01c2,
        #[doc = "PLLN=452."]
        Mul452 = 0x01c3,
        #[doc = "PLLN=453."]
        Mul453 = 0x01c4,
        #[doc = "PLLN=454."]
        Mul454 = 0x01c5,
        #[doc = "PLLN=455."]
        Mul455 = 0x01c6,
        #[doc = "PLLN=456."]
        Mul456 = 0x01c7,
        #[doc = "PLLN=457."]
        Mul457 = 0x01c8,
        #[doc = "PLLN=458."]
        Mul458 = 0x01c9,
        #[doc = "PLLN=459."]
        Mul459 = 0x01ca,
        #[doc = "PLLN=460."]
        Mul460 = 0x01cb,
        #[doc = "PLLN=461."]
        Mul461 = 0x01cc,
        #[doc = "PLLN=462."]
        Mul462 = 0x01cd,
        #[doc = "PLLN=463."]
        Mul463 = 0x01ce,
        #[doc = "PLLN=464."]
        Mul464 = 0x01cf,
        #[doc = "PLLN=465."]
        Mul465 = 0x01d0,
        #[doc = "PLLN=466."]
        Mul466 = 0x01d1,
        #[doc = "PLLN=467."]
        Mul467 = 0x01d2,
        #[doc = "PLLN=468."]
        Mul468 = 0x01d3,
        #[doc = "PLLN=469."]
        Mul469 = 0x01d4,
        #[doc = "PLLN=470."]
        Mul470 = 0x01d5,
        #[doc = "PLLN=471."]
        Mul471 = 0x01d6,
        #[doc = "PLLN=472."]
        Mul472 = 0x01d7,
        #[doc = "PLLN=473."]
        Mul473 = 0x01d8,
        #[doc = "PLLN=474."]
        Mul474 = 0x01d9,
        #[doc = "PLLN=475."]
        Mul475 = 0x01da,
        #[doc = "PLLN=476."]
        Mul476 = 0x01db,
        #[doc = "PLLN=477."]
        Mul477 = 0x01dc,
        #[doc = "PLLN=478."]
        Mul478 = 0x01dd,
        #[doc = "PLLN=479."]
        Mul479 = 0x01de,
        #[doc = "PLLN=480."]
        Mul480 = 0x01df,
        #[doc = "PLLN=481."]
        Mul481 = 0x01e0,
        #[doc = "PLLN=482."]
        Mul482 = 0x01e1,
        #[doc = "PLLN=483."]
        Mul483 = 0x01e2,
        #[doc = "PLLN=484."]
        Mul484 = 0x01e3,
        #[doc = "PLLN=485."]
        Mul485 = 0x01e4,
        #[doc = "PLLN=486."]
        Mul486 = 0x01e5,
        #[doc = "PLLN=487."]
        Mul487 = 0x01e6,
        #[doc = "PLLN=488."]
        Mul488 = 0x01e7,
        #[doc = "PLLN=489."]
        Mul489 = 0x01e8,
        #[doc = "PLLN=490."]
        Mul490 = 0x01e9,
        #[doc = "PLLN=491."]
        Mul491 = 0x01ea,
        #[doc = "PLLN=492."]
        Mul492 = 0x01eb,
        #[doc = "PLLN=493."]
        Mul493 = 0x01ec,
        #[doc = "PLLN=494."]
        Mul494 = 0x01ed,
        #[doc = "PLLN=495."]
        Mul495 = 0x01ee,
        #[doc = "PLLN=496."]
        Mul496 = 0x01ef,
        #[doc = "PLLN=497."]
        Mul497 = 0x01f0,
        #[doc = "PLLN=498."]
        Mul498 = 0x01f1,
        #[doc = "PLLN=499."]
        Mul499 = 0x01f2,
        #[doc = "PLLN=500."]
        Mul500 = 0x01f3,
        #[doc = "PLLN=501."]
        Mul501 = 0x01f4,
        #[doc = "PLLN=502."]
        Mul502 = 0x01f5,
        #[doc = "PLLN=503."]
        Mul503 = 0x01f6,
        #[doc = "PLLN=504."]
        Mul504 = 0x01f7,
        #[doc = "PLLN=505."]
        Mul505 = 0x01f8,
        #[doc = "PLLN=506."]
        Mul506 = 0x01f9,
        #[doc = "PLLN=507."]
        Mul507 = 0x01fa,
        #[doc = "PLLN=508."]
        Mul508 = 0x01fb,
        #[doc = "PLLN=509."]
        Mul509 = 0x01fc,
        #[doc = "PLLN=510."]
        Mul510 = 0x01fd,
        #[doc = "PLLN=511."]
        Mul511 = 0x01fe,
        #[doc = "PLLN=512."]
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllrge {
        #[doc = "PLL1 input (ref1_ck) clock range frequency between 1 and 2MHz (default after reset)."]
        Range1 = 0x0,
        #[doc = "PLL1 input (ref1_ck) clock range frequency between 2 and 4MHz."]
        Range2 = 0x01,
        #[doc = "PLL1 input (ref1_ck) clock range frequency between 4 and 8MHz."]
        Range4 = 0x02,
        #[doc = "PLL1 input (ref1_ck) clock range frequency between 8 and 16MHz."]
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllsrc {
        #[doc = "no clock sent to PLL1M divider and PLL1 (default after reset)."]
        Disable = 0x0,
        #[doc = "HSI selected as PLL clock (hsi_ck)."]
        Hsi = 0x01,
        #[doc = "HSE selected as PLL clock (hse_ck)."]
        Hse = 0x02,
        _RESERVED_3 = 0x03,
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
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllvcosel {
        #[doc = "wide VCO range: 128 to 560MHz (default after reset)."]
        WideVco = 0x0,
        #[doc = "medium VCO range: 150 to 420MHz."]
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Posencsel {
        #[doc = "PLL1_P"]
        Pll1P = 0x0,
        #[doc = "PLL1_Q"]
        Pll1Q = 0x01,
        #[doc = "IC1"]
        Ic1 = 0x02,
        #[doc = "PLL2_P"]
        Pll2P = 0x03,
        #[doc = "PLL2_Q"]
        Pll2Q = 0x04,
        #[doc = "IC2"]
        Ic2 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Posencsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Posencsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Posencsel {
        #[inline(always)]
        fn from(val: u8) -> Posencsel {
            Posencsel::from_bits(val)
        }
    }
    impl From<Posencsel> for u8 {
        #[inline(always)]
        fn from(val: Posencsel) -> u8 {
            Posencsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ppre {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "rcc_pclk1=rcc_hclk1 / 2."]
        Div2 = 0x04,
        #[doc = "rcc_pclk1=rcc_hclk1 / 4."]
        Div4 = 0x05,
        #[doc = "rcc_pclk1=rcc_hclk1 / 8."]
        Div8 = 0x06,
        #[doc = "rcc_pclk1=rcc_hclk1 / 16."]
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rtcpre {
        #[doc = "no clock (default after reset)."]
        Disabled = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "HSE/2."]
        Div2 = 0x02,
        #[doc = "HSE/3."]
        Div3 = 0x03,
        #[doc = "HSE/4."]
        Div4 = 0x04,
        #[doc = "HSE/5."]
        Div5 = 0x05,
        #[doc = "HSE/6."]
        Div6 = 0x06,
        #[doc = "HSE/7."]
        Div7 = 0x07,
        #[doc = "HSE/8."]
        Div8 = 0x08,
        #[doc = "HSE/9."]
        Div9 = 0x09,
        #[doc = "HSE/10."]
        Div10 = 0x0a,
        #[doc = "HSE/11."]
        Div11 = 0x0b,
        #[doc = "HSE/12."]
        Div12 = 0x0c,
        #[doc = "HSE/13."]
        Div13 = 0x0d,
        #[doc = "HSE/14."]
        Div14 = 0x0e,
        #[doc = "HSE/15."]
        Div15 = 0x0f,
        #[doc = "HSE/16."]
        Div16 = 0x10,
        #[doc = "HSE/17."]
        Div17 = 0x11,
        #[doc = "HSE/18."]
        Div18 = 0x12,
        #[doc = "HSE/19."]
        Div19 = 0x13,
        #[doc = "HSE/20."]
        Div20 = 0x14,
        #[doc = "HSE/21."]
        Div21 = 0x15,
        #[doc = "HSE/22."]
        Div22 = 0x16,
        #[doc = "HSE/23."]
        Div23 = 0x17,
        #[doc = "HSE/24."]
        Div24 = 0x18,
        #[doc = "HSE/25."]
        Div25 = 0x19,
        #[doc = "HSE/26."]
        Div26 = 0x1a,
        #[doc = "HSE/27."]
        Div27 = 0x1b,
        #[doc = "HSE/28."]
        Div28 = 0x1c,
        #[doc = "HSE/29."]
        Div29 = 0x1d,
        #[doc = "HSE/30."]
        Div30 = 0x1e,
        #[doc = "HSE/31."]
        Div31 = 0x1f,
        #[doc = "HSE/32."]
        Div32 = 0x20,
        #[doc = "HSE/33."]
        Div33 = 0x21,
        #[doc = "HSE/34."]
        Div34 = 0x22,
        #[doc = "HSE/35."]
        Div35 = 0x23,
        #[doc = "HSE/36."]
        Div36 = 0x24,
        #[doc = "HSE/37."]
        Div37 = 0x25,
        #[doc = "HSE/38."]
        Div38 = 0x26,
        #[doc = "HSE/39."]
        Div39 = 0x27,
        #[doc = "HSE/40."]
        Div40 = 0x28,
        #[doc = "HSE/41."]
        Div41 = 0x29,
        #[doc = "HSE/42."]
        Div42 = 0x2a,
        #[doc = "HSE/43."]
        Div43 = 0x2b,
        #[doc = "HSE/44."]
        Div44 = 0x2c,
        #[doc = "HSE/45."]
        Div45 = 0x2d,
        #[doc = "HSE/46."]
        Div46 = 0x2e,
        #[doc = "HSE/47."]
        Div47 = 0x2f,
        #[doc = "HSE/48."]
        Div48 = 0x30,
        #[doc = "HSE/49."]
        Div49 = 0x31,
        #[doc = "HSE/50."]
        Div50 = 0x32,
        #[doc = "HSE/51."]
        Div51 = 0x33,
        #[doc = "HSE/52."]
        Div52 = 0x34,
        #[doc = "HSE/53."]
        Div53 = 0x35,
        #[doc = "HSE/54."]
        Div54 = 0x36,
        #[doc = "HSE/55."]
        Div55 = 0x37,
        #[doc = "HSE/56."]
        Div56 = 0x38,
        #[doc = "HSE/57."]
        Div57 = 0x39,
        #[doc = "HSE/58."]
        Div58 = 0x3a,
        #[doc = "HSE/59."]
        Div59 = 0x3b,
        #[doc = "HSE/60."]
        Div60 = 0x3c,
        #[doc = "HSE/61."]
        Div61 = 0x3d,
        #[doc = "HSE/62."]
        Div62 = 0x3e,
        #[doc = "HSE/63."]
        Div63 = 0x3f,
    }
    impl Rtcpre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rtcpre {
            unsafe { core::mem::transmute(val & 0x3f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rtcpre {
        #[inline(always)]
        fn from(val: u8) -> Rtcpre {
            Rtcpre::from_bits(val)
        }
    }
    impl From<Rtcpre> for u8 {
        #[inline(always)]
        fn from(val: Rtcpre) -> u8 {
            Rtcpre::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rtcsel {
        #[doc = "no clock (default after RTC domain reset)."]
        Disable = 0x0,
        #[doc = "LSE selected as RTC clock."]
        Lse = 0x01,
        #[doc = "LSI selected as RTC clock."]
        Lsi = 0x02,
        #[doc = "HSE divided by RTCPRE value selected as RTC clock."]
        HseDivRtcpre = 0x03,
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
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Spi1sel {
        #[doc = "PCLK2"]
        Pclk2 = 0x0,
        #[doc = "IC1"]
        Ic1 = 0x01,
        #[doc = "PLL2_P"]
        Pll2P = 0x02,
        #[doc = "HSE"]
        Hse = 0x03,
        #[doc = "HSI"]
        Hsi = 0x04,
        #[doc = "AUDIOCLK"]
        Audioclk = 0x05,
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Spi2sel {
        #[doc = "PCLK1"]
        Pclk1 = 0x0,
        #[doc = "IC1"]
        Ic1 = 0x01,
        #[doc = "PLL2_P"]
        Pll2P = 0x02,
        #[doc = "HSE"]
        Hse = 0x03,
        #[doc = "HSI"]
        Hsi = 0x04,
        #[doc = "AUDIOCLK"]
        Audioclk = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Spi2sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Spi2sel {
            unsafe { core::mem::transmute(val & 0x07) }
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Spi3sel {
        #[doc = "PCLK1"]
        Pclk1 = 0x0,
        #[doc = "IC1"]
        Ic1 = 0x01,
        #[doc = "PLL2_P"]
        Pll2P = 0x02,
        #[doc = "HSE"]
        Hse = 0x03,
        #[doc = "HSI"]
        Hsi = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Spi3sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Spi3sel {
            unsafe { core::mem::transmute(val & 0x07) }
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Syssw {
        #[doc = "HSI"]
        Hsi = 0x0,
        #[doc = "HSE"]
        Hse = 0x01,
        #[doc = "PLL1_P"]
        Pll1P = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Syssw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Syssw {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Syssw {
        #[inline(always)]
        fn from(val: u8) -> Syssw {
            Syssw::from_bits(val)
        }
    }
    impl From<Syssw> for u8 {
        #[inline(always)]
        fn from(val: Syssw) -> u8 {
            Syssw::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Systicksel {
        #[doc = "HCLK_DIV_8"]
        HclkDiv8 = 0x0,
        #[doc = "LSI"]
        Lsi = 0x01,
        #[doc = "LSE"]
        Lse = 0x02,
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Uartsel {
        #[doc = "PCLK1"]
        Pclk1 = 0x0,
        #[doc = "IC1"]
        Ic1 = 0x01,
        #[doc = "PLL2_Q"]
        Pll2Q = 0x02,
        #[doc = "HSE"]
        Hse = 0x03,
        #[doc = "HSI"]
        Hsi = 0x04,
        #[doc = "LSE"]
        Lse = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Uartsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Uartsel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Uartsel {
        #[inline(always)]
        fn from(val: u8) -> Uartsel {
            Uartsel::from_bits(val)
        }
    }
    impl From<Uartsel> for u8 {
        #[inline(always)]
        fn from(val: Uartsel) -> u8 {
            Uartsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ucpdsel {
        #[doc = "HSI"]
        Hsi = 0x0,
        #[doc = "HSE"]
        Hse = 0x01,
        #[doc = "HSI48"]
        Hsi48 = 0x02,
        #[doc = "PLL2_Q"]
        Pll2Q = 0x03,
    }
    impl Ucpdsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ucpdsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ucpdsel {
        #[inline(always)]
        fn from(val: u8) -> Ucpdsel {
            Ucpdsel::from_bits(val)
        }
    }
    impl From<Ucpdsel> for u8 {
        #[inline(always)]
        fn from(val: Ucpdsel) -> u8 {
            Ucpdsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Usart2sel {
        #[doc = "PCLK1"]
        Pclk1 = 0x0,
        #[doc = "IC1"]
        Ic1 = 0x01,
        #[doc = "PLL2_Q"]
        Pll2Q = 0x02,
        #[doc = "HSE"]
        Hse = 0x03,
        #[doc = "HSI"]
        Hsi = 0x04,
        #[doc = "LSE"]
        Lse = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Usart2sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usart2sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usart2sel {
        #[inline(always)]
        fn from(val: u8) -> Usart2sel {
            Usart2sel::from_bits(val)
        }
    }
    impl From<Usart2sel> for u8 {
        #[inline(always)]
        fn from(val: Usart2sel) -> u8 {
            Usart2sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Usartsel {
        #[doc = "PCLK2"]
        Pclk2 = 0x0,
        #[doc = "IC1"]
        Ic1 = 0x01,
        #[doc = "PLL2_Q"]
        Pll2Q = 0x02,
        #[doc = "HSE"]
        Hse = 0x03,
        #[doc = "HSI"]
        Hsi = 0x04,
        #[doc = "LSE"]
        Lse = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Usartsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usartsel {
            unsafe { core::mem::transmute(val & 0x07) }
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
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Usbsel {
        #[doc = "HSI48"]
        Hsi48 = 0x0,
        #[doc = "HSE"]
        Hse = 0x01,
        #[doc = "PLL1_Q"]
        Pll1Q = 0x02,
        #[doc = "PLL2_Q"]
        Pll2Q = 0x03,
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
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Xspisel {
        #[doc = "HCLK4"]
        Hclk4 = 0x0,
        #[doc = "PLL1_Q"]
        Pll1Q = 0x01,
        #[doc = "IC2"]
        Ic2 = 0x02,
        #[doc = "HSE"]
        Hse = 0x03,
        #[doc = "HSI"]
        Hsi = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Xspisel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Xspisel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Xspisel {
        #[inline(always)]
        fn from(val: u8) -> Xspisel {
            Xspisel::from_bits(val)
        }
    }
    impl From<Xspisel> for u8 {
        #[inline(always)]
        fn from(val: Xspisel) -> u8 {
            Xspisel::to_bits(val)
        }
    }
}
