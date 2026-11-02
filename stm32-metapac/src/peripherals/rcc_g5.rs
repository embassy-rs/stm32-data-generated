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
        pub const fn syssws(&self) -> super::vals::Syssws {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Syssws::from_bits(val as u8)
        }
        #[doc = "system clock switch status."]
        #[inline(always)]
        pub const fn set_syssws(&mut self, val: super::vals::Syssws) {
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
        pub const fn ahb1dis(&self) -> super::vals::Ahbdis {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Ahbdis::from_bits(val as u8)
        }
        #[doc = "AHB1 clock disable."]
        #[inline(always)]
        pub const fn set_ahb1dis(&mut self, val: super::vals::Ahbdis) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "AHB2 clock disable."]
        #[must_use]
        #[inline(always)]
        pub const fn ahb2dis(&self) -> super::vals::Ahbdis {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Ahbdis::from_bits(val as u8)
        }
        #[doc = "AHB2 clock disable."]
        #[inline(always)]
        pub const fn set_ahb2dis(&mut self, val: super::vals::Ahbdis) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "AHB3 clock disable."]
        #[must_use]
        #[inline(always)]
        pub const fn ahb3dis(&self) -> super::vals::Ahbdis {
            let val = (self.0 >> 18usize) & 0x01;
            super::vals::Ahbdis::from_bits(val as u8)
        }
        #[doc = "AHB3 clock disable."]
        #[inline(always)]
        pub const fn set_ahb3dis(&mut self, val: super::vals::Ahbdis) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
        }
        #[doc = "AHB4 clock disable."]
        #[must_use]
        #[inline(always)]
        pub const fn ahb4dis(&self) -> super::vals::Ahbdis {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Ahbdis::from_bits(val as u8)
        }
        #[doc = "AHB4 clock disable."]
        #[inline(always)]
        pub const fn set_ahb4dis(&mut self, val: super::vals::Ahbdis) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "APB1 clock disable value."]
        #[must_use]
        #[inline(always)]
        pub const fn apb1dis(&self) -> super::vals::Apbdis {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Apbdis::from_bits(val as u8)
        }
        #[doc = "APB1 clock disable value."]
        #[inline(always)]
        pub const fn set_apb1dis(&mut self, val: super::vals::Apbdis) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "APB2 clock disable value."]
        #[must_use]
        #[inline(always)]
        pub const fn apb2dis(&self) -> super::vals::Apbdis {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Apbdis::from_bits(val as u8)
        }
        #[doc = "APB2 clock disable value."]
        #[inline(always)]
        pub const fn set_apb2dis(&mut self, val: super::vals::Apbdis) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "APB3 clock disable value."]
        #[must_use]
        #[inline(always)]
        pub const fn apb3dis(&self) -> super::vals::Apbdis {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Apbdis::from_bits(val as u8)
        }
        #[doc = "APB3 clock disable value."]
        #[inline(always)]
        pub const fn set_apb3dis(&mut self, val: super::vals::Apbdis) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
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
                "Cfgr2 {{ hpre: {:?}, ppre1: {:?}, ppre2: {:?}, ppre3: {:?}, ahb1dis: {:?}, ahb2dis: {:?}, ahb3dis: {:?}, ahb4dis: {:?}, apb1dis: {:?}, apb2dis: {:?}, apb3dis: {:?} }}",
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
        pub const fn hsirdy(&self) -> super::vals::Hsirdy {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Hsirdy::from_bits(val as u8)
        }
        #[doc = "HSI clock ready flag."]
        #[inline(always)]
        pub const fn set_hsirdy(&mut self, val: super::vals::Hsirdy) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
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
        pub const fn hsidivf(&self) -> super::vals::Hsidivf {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Hsidivf::from_bits(val as u8)
        }
        #[doc = "HSI divider flag."]
        #[inline(always)]
        pub const fn set_hsidivf(&mut self, val: super::vals::Hsidivf) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
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
        pub const fn hsi48rdy(&self) -> super::vals::Hsirdy {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::Hsirdy::from_bits(val as u8)
        }
        #[doc = "HSI48 clock ready flag."]
        #[inline(always)]
        pub const fn set_hsi48rdy(&mut self, val: super::vals::Hsirdy) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
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
        pub const fn hserdy(&self) -> super::vals::Hserdy {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Hserdy::from_bits(val as u8)
        }
        #[doc = "HSE clock ready flag."]
        #[inline(always)]
        pub const fn set_hserdy(&mut self, val: super::vals::Hserdy) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "HSE clock bypass."]
        #[must_use]
        #[inline(always)]
        pub const fn hsebyp(&self) -> super::vals::Hsebyp {
            let val = (self.0 >> 18usize) & 0x01;
            super::vals::Hsebyp::from_bits(val as u8)
        }
        #[doc = "HSE clock bypass."]
        #[inline(always)]
        pub const fn set_hsebyp(&mut self, val: super::vals::Hsebyp) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
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
        pub const fn pllrdy(&self, n: usize) -> super::vals::Pllrdy {
            assert!(n < 2usize);
            let offs = 25usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Pllrdy::from_bits(val as u8)
        }
        #[doc = "PLL1 clock ready flag."]
        #[inline(always)]
        pub const fn set_pllrdy(&mut self, n: usize, val: super::vals::Pllrdy) {
            assert!(n < 2usize);
            let offs = 25usize + n * 2usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
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
                "Cr {{ hsion: {=bool:?}, hsirdy: {:?}, hsikeron: {=bool:?}, hsidiv: {:?}, hsidivf: {:?}, hsi48on: {=bool:?}, hsi48rdy: {:?}, hseon: {=bool:?}, hserdy: {:?}, hsebyp: {:?}, hsecsson: {=bool:?}, hseext: {:?}, pllon[0]: {=bool:?}, pllon[1]: {=bool:?}, pllrdy[0]: {:?}, pllrdy[1]: {:?} }}",
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
        pub const fn lsirdyc(&self) -> super::vals::Lsirdyc {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Lsirdyc::from_bits(val as u8)
        }
        #[doc = "LSI ready interrupt clear."]
        #[inline(always)]
        pub const fn set_lsirdyc(&mut self, val: super::vals::Lsirdyc) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt clear."]
        #[must_use]
        #[inline(always)]
        pub const fn lserdyc(&self) -> super::vals::Lserdyc {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Lserdyc::from_bits(val as u8)
        }
        #[doc = "LSE ready interrupt clear."]
        #[inline(always)]
        pub const fn set_lserdyc(&mut self, val: super::vals::Lserdyc) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI ready interrupt clear."]
        #[must_use]
        #[inline(always)]
        pub const fn hsirdyc(&self) -> super::vals::Hsirdyc {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Hsirdyc::from_bits(val as u8)
        }
        #[doc = "HSI ready interrupt clear."]
        #[inline(always)]
        pub const fn set_hsirdyc(&mut self, val: super::vals::Hsirdyc) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "HSE ready interrupt clear."]
        #[must_use]
        #[inline(always)]
        pub const fn hserdyc(&self) -> super::vals::Hserdyc {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Hserdyc::from_bits(val as u8)
        }
        #[doc = "HSE ready interrupt clear."]
        #[inline(always)]
        pub const fn set_hserdyc(&mut self, val: super::vals::Hserdyc) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "HSI48 ready interrupt clear."]
        #[must_use]
        #[inline(always)]
        pub const fn hsi48rdyc(&self) -> super::vals::Hsirdyc {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Hsirdyc::from_bits(val as u8)
        }
        #[doc = "HSI48 ready interrupt clear."]
        #[inline(always)]
        pub const fn set_hsi48rdyc(&mut self, val: super::vals::Hsirdyc) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "PLL1 ready interrupt clear."]
        #[must_use]
        #[inline(always)]
        pub const fn pllrdyc(&self, n: usize) -> super::vals::Pllrdyc {
            assert!(n < 2usize);
            let offs = 6usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Pllrdyc::from_bits(val as u8)
        }
        #[doc = "PLL1 ready interrupt clear."]
        #[inline(always)]
        pub const fn set_pllrdyc(&mut self, n: usize, val: super::vals::Pllrdyc) {
            assert!(n < 2usize);
            let offs = 6usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "LSE clock security system interrupt clear."]
        #[must_use]
        #[inline(always)]
        pub const fn lsecssc(&self) -> super::vals::Lsecssc {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Lsecssc::from_bits(val as u8)
        }
        #[doc = "LSE clock security system interrupt clear."]
        #[inline(always)]
        pub const fn set_lsecssc(&mut self, val: super::vals::Lsecssc) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "HSE clock security system interrupt clear."]
        #[must_use]
        #[inline(always)]
        pub const fn hsecssc(&self) -> super::vals::Hsecssc {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Hsecssc::from_bits(val as u8)
        }
        #[doc = "HSE clock security system interrupt clear."]
        #[inline(always)]
        pub const fn set_hsecssc(&mut self, val: super::vals::Hsecssc) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Icr {
        #[inline(always)]
        fn default() -> Icr {
            Icr(0)
        }
    }
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
                "Icr {{ lsirdyc: {:?}, lserdyc: {:?}, hsirdyc: {:?}, hserdyc: {:?}, hsi48rdyc: {:?}, pllrdyc[0]: {:?}, pllrdyc[1]: {:?}, lsecssc: {:?}, hsecssc: {:?} }}",
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
        pub const fn lsirdyie(&self) -> super::vals::Lsirdyie {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Lsirdyie::from_bits(val as u8)
        }
        #[doc = "LSI ready interrupt enable."]
        #[inline(always)]
        pub const fn set_lsirdyie(&mut self, val: super::vals::Lsirdyie) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lserdyie(&self) -> super::vals::Lserdyie {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Lserdyie::from_bits(val as u8)
        }
        #[doc = "LSE ready interrupt enable."]
        #[inline(always)]
        pub const fn set_lserdyie(&mut self, val: super::vals::Lserdyie) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI ready interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hsirdyie(&self) -> super::vals::Hsirdyie {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Hsirdyie::from_bits(val as u8)
        }
        #[doc = "HSI ready interrupt enable."]
        #[inline(always)]
        pub const fn set_hsirdyie(&mut self, val: super::vals::Hsirdyie) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "HSE ready interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hserdyie(&self) -> super::vals::Hserdyie {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Hserdyie::from_bits(val as u8)
        }
        #[doc = "HSE ready interrupt enable."]
        #[inline(always)]
        pub const fn set_hserdyie(&mut self, val: super::vals::Hserdyie) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "HSI48 ready interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hsi48rdyie(&self) -> super::vals::Hsirdyie {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Hsirdyie::from_bits(val as u8)
        }
        #[doc = "HSI48 ready interrupt enable."]
        #[inline(always)]
        pub const fn set_hsi48rdyie(&mut self, val: super::vals::Hsirdyie) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "PLL1 ready interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pllrdyie(&self, n: usize) -> super::vals::Pllrdyie {
            assert!(n < 2usize);
            let offs = 6usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Pllrdyie::from_bits(val as u8)
        }
        #[doc = "PLL1 ready interrupt enable."]
        #[inline(always)]
        pub const fn set_pllrdyie(&mut self, n: usize, val: super::vals::Pllrdyie) {
            assert!(n < 2usize);
            let offs = 6usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "LSE clock security system interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lsecssie(&self) -> super::vals::Lsecssie {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Lsecssie::from_bits(val as u8)
        }
        #[doc = "LSE clock security system interrupt enable."]
        #[inline(always)]
        pub const fn set_lsecssie(&mut self, val: super::vals::Lsecssie) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Ier {
        #[inline(always)]
        fn default() -> Ier {
            Ier(0)
        }
    }
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
                "Ier {{ lsirdyie: {:?}, lserdyie: {:?}, hsirdyie: {:?}, hserdyie: {:?}, hsi48rdyie: {:?}, pllrdyie[0]: {:?}, pllrdyie[1]: {:?}, lsecssie: {:?} }}",
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
        pub const fn lsirdyf(&self) -> super::vals::Lsirdyf {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Lsirdyf::from_bits(val as u8)
        }
        #[doc = "LSI ready interrupt flag."]
        #[inline(always)]
        pub const fn set_lsirdyf(&mut self, val: super::vals::Lsirdyf) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn lserdyf(&self) -> super::vals::Lserdyf {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Lserdyf::from_bits(val as u8)
        }
        #[doc = "LSE ready interrupt flag."]
        #[inline(always)]
        pub const fn set_lserdyf(&mut self, val: super::vals::Lserdyf) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI ready interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hsirdyf(&self) -> super::vals::Hsirdyf {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Hsirdyf::from_bits(val as u8)
        }
        #[doc = "HSI ready interrupt flag."]
        #[inline(always)]
        pub const fn set_hsirdyf(&mut self, val: super::vals::Hsirdyf) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "HSE ready interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hserdyf(&self) -> super::vals::Hserdyf {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Hserdyf::from_bits(val as u8)
        }
        #[doc = "HSE ready interrupt flag."]
        #[inline(always)]
        pub const fn set_hserdyf(&mut self, val: super::vals::Hserdyf) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "HSI48 ready interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hsi48rdyf(&self) -> super::vals::Hsirdyf {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Hsirdyf::from_bits(val as u8)
        }
        #[doc = "HSI48 ready interrupt flag."]
        #[inline(always)]
        pub const fn set_hsi48rdyf(&mut self, val: super::vals::Hsirdyf) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "PLL1 ready interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn pllrdyf(&self, n: usize) -> super::vals::Pllrdyf {
            assert!(n < 2usize);
            let offs = 6usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Pllrdyf::from_bits(val as u8)
        }
        #[doc = "PLL1 ready interrupt flag."]
        #[inline(always)]
        pub const fn set_pllrdyf(&mut self, n: usize, val: super::vals::Pllrdyf) {
            assert!(n < 2usize);
            let offs = 6usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "LSE clock security system interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn lsecssf(&self) -> super::vals::Lsecssf {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Lsecssf::from_bits(val as u8)
        }
        #[doc = "LSE clock security system interrupt flag."]
        #[inline(always)]
        pub const fn set_lsecssf(&mut self, val: super::vals::Lsecssf) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "HSE clock security system interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hsecssf(&self) -> super::vals::Hsecssf {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Hsecssf::from_bits(val as u8)
        }
        #[doc = "HSE clock security system interrupt flag."]
        #[inline(always)]
        pub const fn set_hsecssf(&mut self, val: super::vals::Hsecssf) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Ifr {
        #[inline(always)]
        fn default() -> Ifr {
            Ifr(0)
        }
    }
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
                "Ifr {{ lsirdyf: {:?}, lserdyf: {:?}, hsirdyf: {:?}, hserdyf: {:?}, hsi48rdyf: {:?}, pllrdyf[0]: {:?}, pllrdyf[1]: {:?}, lsecssf: {:?}, hsecssf: {:?} }}",
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
        pub const fn pllsrc(&self, n: usize) -> super::vals::Pllsrc {
            assert!(n < 1usize);
            let offs = 0usize + n * 0usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::Pllsrc::from_bits(val as u8)
        }
        #[doc = "PLL1M and PLLs clock source selection."]
        #[inline(always)]
        pub const fn set_pllsrc(&mut self, n: usize, val: super::vals::Pllsrc) {
            assert!(n < 1usize);
            let offs = 0usize + n * 0usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "PLL1 input frequency range."]
        #[must_use]
        #[inline(always)]
        pub const fn pllrge(&self, n: usize) -> super::vals::Pll1rge {
            assert!(n < 1usize);
            let offs = 2usize + n * 0usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::Pll1rge::from_bits(val as u8)
        }
        #[doc = "PLL1 input frequency range."]
        #[inline(always)]
        pub const fn set_pllrge(&mut self, n: usize, val: super::vals::Pll1rge) {
            assert!(n < 1usize);
            let offs = 2usize + n * 0usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "PLL1 fractional latch enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pllfracen(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 4usize + n * 0usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 fractional latch enable."]
        #[inline(always)]
        pub const fn set_pllfracen(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 4usize + n * 0usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "PLL1 VCO selection."]
        #[must_use]
        #[inline(always)]
        pub const fn pllvcosel(&self, n: usize) -> super::vals::Pllvcosel {
            assert!(n < 1usize);
            let offs = 5usize + n * 0usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Pllvcosel::from_bits(val as u8)
        }
        #[doc = "PLL1 VCO selection."]
        #[inline(always)]
        pub const fn set_pllvcosel(&mut self, n: usize, val: super::vals::Pllvcosel) {
            assert!(n < 1usize);
            let offs = 5usize + n * 0usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Prescaler for PLL1."]
        #[must_use]
        #[inline(always)]
        pub const fn pllm(&self, n: usize) -> super::vals::Pllm {
            assert!(n < 1usize);
            let offs = 8usize + n * 0usize;
            let val = (self.0 >> offs) & 0x3f;
            super::vals::Pllm::from_bits(val as u8)
        }
        #[doc = "Prescaler for PLL1."]
        #[inline(always)]
        pub const fn set_pllm(&mut self, n: usize, val: super::vals::Pllm) {
            assert!(n < 1usize);
            let offs = 8usize + n * 0usize;
            self.0 = (self.0 & !(0x3f << offs)) | (((val.to_bits() as u32) & 0x3f) << offs);
        }
        #[doc = "PLL1 DIVP divider output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pllpen(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 16usize + n * 0usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 DIVP divider output enable."]
        #[inline(always)]
        pub const fn set_pllpen(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 16usize + n * 0usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "PLL1 DIVQ divider output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pllqen(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 17usize + n * 0usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 DIVQ divider output enable."]
        #[inline(always)]
        pub const fn set_pllqen(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 17usize + n * 0usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "PLL1 DIVR and ICLK1 divider output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pllren(&self, n: usize) -> bool {
            assert!(n < 1usize);
            let offs = 18usize + n * 0usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 DIVR and ICLK1 divider output enable."]
        #[inline(always)]
        pub const fn set_pllren(&mut self, n: usize, val: bool) {
            assert!(n < 1usize);
            let offs = 18usize + n * 0usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
    impl core::fmt::Debug for Pll1cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pll1cfgr")
                .field("pllsrc[0]", &self.pllsrc(0usize))
                .field("pllrge[0]", &self.pllrge(0usize))
                .field("pllfracen[0]", &self.pllfracen(0usize))
                .field("pllvcosel[0]", &self.pllvcosel(0usize))
                .field("pllm[0]", &self.pllm(0usize))
                .field("pllpen[0]", &self.pllpen(0usize))
                .field("pllqen[0]", &self.pllqen(0usize))
                .field("pllren[0]", &self.pllren(0usize))
                .field("iclk1div", &self.iclk1div())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pll1cfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pll1cfgr {{ pllsrc[0]: {:?}, pllrge[0]: {:?}, pllfracen[0]: {=bool:?}, pllvcosel[0]: {:?}, pllm[0]: {:?}, pllpen[0]: {=bool:?}, pllqen[0]: {=bool:?}, pllren[0]: {=bool:?}, iclk1div: {:?} }}",
                self.pllsrc(0usize),
                self.pllrge(0usize),
                self.pllfracen(0usize),
                self.pllvcosel(0usize),
                self.pllm(0usize),
                self.pllpen(0usize),
                self.pllqen(0usize),
                self.pllren(0usize),
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
        pub const fn plln(&self, n: usize) -> super::vals::Plln {
            assert!(n < 1usize);
            let offs = 0usize + n * 0usize;
            let val = (self.0 >> offs) & 0x01ff;
            super::vals::Plln::from_bits(val as u16)
        }
        #[doc = "Multiplication factor for PLL1VCO."]
        #[inline(always)]
        pub const fn set_plln(&mut self, n: usize, val: super::vals::Plln) {
            assert!(n < 1usize);
            let offs = 0usize + n * 0usize;
            self.0 = (self.0 & !(0x01ff << offs)) | (((val.to_bits() as u32) & 0x01ff) << offs);
        }
        #[doc = "PLL1 DIVP division factor."]
        #[must_use]
        #[inline(always)]
        pub const fn pllp(&self, n: usize) -> super::vals::Pllp {
            assert!(n < 1usize);
            let offs = 9usize + n * 0usize;
            let val = (self.0 >> offs) & 0x7f;
            super::vals::Pllp::from_bits(val as u8)
        }
        #[doc = "PLL1 DIVP division factor."]
        #[inline(always)]
        pub const fn set_pllp(&mut self, n: usize, val: super::vals::Pllp) {
            assert!(n < 1usize);
            let offs = 9usize + n * 0usize;
            self.0 = (self.0 & !(0x7f << offs)) | (((val.to_bits() as u32) & 0x7f) << offs);
        }
        #[doc = "PLL1 DIVQ division factor."]
        #[must_use]
        #[inline(always)]
        pub const fn pllq(&self, n: usize) -> super::vals::Pllq {
            assert!(n < 1usize);
            let offs = 16usize + n * 0usize;
            let val = (self.0 >> offs) & 0x7f;
            super::vals::Pllq::from_bits(val as u8)
        }
        #[doc = "PLL1 DIVQ division factor."]
        #[inline(always)]
        pub const fn set_pllq(&mut self, n: usize, val: super::vals::Pllq) {
            assert!(n < 1usize);
            let offs = 16usize + n * 0usize;
            self.0 = (self.0 & !(0x7f << offs)) | (((val.to_bits() as u32) & 0x7f) << offs);
        }
        #[doc = "PLL1 DIVR division factor."]
        #[must_use]
        #[inline(always)]
        pub const fn pllr(&self, n: usize) -> super::vals::Pllr {
            assert!(n < 1usize);
            let offs = 24usize + n * 0usize;
            let val = (self.0 >> offs) & 0x7f;
            super::vals::Pllr::from_bits(val as u8)
        }
        #[doc = "PLL1 DIVR division factor."]
        #[inline(always)]
        pub const fn set_pllr(&mut self, n: usize, val: super::vals::Pllr) {
            assert!(n < 1usize);
            let offs = 24usize + n * 0usize;
            self.0 = (self.0 & !(0x7f << offs)) | (((val.to_bits() as u32) & 0x7f) << offs);
        }
    }
    impl Default for Pll1divr {
        #[inline(always)]
        fn default() -> Pll1divr {
            Pll1divr(0)
        }
    }
    impl core::fmt::Debug for Pll1divr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pll1divr")
                .field("plln[0]", &self.plln(0usize))
                .field("pllp[0]", &self.pllp(0usize))
                .field("pllq[0]", &self.pllq(0usize))
                .field("pllr[0]", &self.pllr(0usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pll1divr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pll1divr {{ plln[0]: {:?}, pllp[0]: {:?}, pllq[0]: {:?}, pllr[0]: {:?} }}",
                self.plln(0usize),
                self.pllp(0usize),
                self.pllq(0usize),
                self.pllr(0usize)
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
        pub const fn pllfracn(&self, n: usize) -> u16 {
            assert!(n < 1usize);
            let offs = 3usize + n * 0usize;
            let val = (self.0 >> offs) & 0x1fff;
            val as u16
        }
        #[doc = "Fractional part of the multiplication factor for PLL1 VCO."]
        #[inline(always)]
        pub const fn set_pllfracn(&mut self, n: usize, val: u16) {
            assert!(n < 1usize);
            let offs = 3usize + n * 0usize;
            self.0 = (self.0 & !(0x1fff << offs)) | (((val as u32) & 0x1fff) << offs);
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
            f.debug_struct("Pll1fracr")
                .field("pllfracn[0]", &self.pllfracn(0usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pll1fracr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pll1fracr {{ pllfracn[0]: {=u16:?} }}", self.pllfracn(0usize))
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
        pub const fn spriv(&self) -> super::vals::Spriv {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Spriv::from_bits(val as u8)
        }
        #[doc = "RCC secure functions privilege configuration."]
        #[inline(always)]
        pub const fn set_spriv(&mut self, val: super::vals::Spriv) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "RCC nonsecure functions privilege configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn nspriv(&self) -> super::vals::Nspriv {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Nspriv::from_bits(val as u8)
        }
        #[doc = "RCC nonsecure functions privilege configuration."]
        #[inline(always)]
        pub const fn set_nspriv(&mut self, val: super::vals::Nspriv) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
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
            defmt::write!(
                f,
                "Privcfgr {{ spriv: {:?}, nspriv: {:?} }}",
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
        pub const fn rmvf(&self) -> super::vals::Rmvf {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Rmvf::from_bits(val as u8)
        }
        #[doc = "remove reset flag."]
        #[inline(always)]
        pub const fn set_rmvf(&mut self, val: super::vals::Rmvf) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "pin reset flag (NRST)."]
        #[must_use]
        #[inline(always)]
        pub const fn pinrstf(&self) -> super::vals::Pinrstf {
            let val = (self.0 >> 26usize) & 0x01;
            super::vals::Pinrstf::from_bits(val as u8)
        }
        #[doc = "pin reset flag (NRST)."]
        #[inline(always)]
        pub const fn set_pinrstf(&mut self, val: super::vals::Pinrstf) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
        }
        #[doc = "POR reset flag."]
        #[must_use]
        #[inline(always)]
        pub const fn porrstf(&self) -> super::vals::Porrstf {
            let val = (self.0 >> 27usize) & 0x01;
            super::vals::Porrstf::from_bits(val as u8)
        }
        #[doc = "POR reset flag."]
        #[inline(always)]
        pub const fn set_porrstf(&mut self, val: super::vals::Porrstf) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
        }
        #[doc = "system reset from CPU reset flag."]
        #[must_use]
        #[inline(always)]
        pub const fn sftrstf(&self) -> super::vals::Sftrstf {
            let val = (self.0 >> 28usize) & 0x01;
            super::vals::Sftrstf::from_bits(val as u8)
        }
        #[doc = "system reset from CPU reset flag."]
        #[inline(always)]
        pub const fn set_sftrstf(&mut self, val: super::vals::Sftrstf) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
        }
        #[doc = "independent watchdog reset flag."]
        #[must_use]
        #[inline(always)]
        pub const fn iwdgrstf(&self) -> super::vals::Iwdgrstf {
            let val = (self.0 >> 29usize) & 0x01;
            super::vals::Iwdgrstf::from_bits(val as u8)
        }
        #[doc = "independent watchdog reset flag."]
        #[inline(always)]
        pub const fn set_iwdgrstf(&mut self, val: super::vals::Iwdgrstf) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
        }
        #[doc = "window watchdog reset flag."]
        #[must_use]
        #[inline(always)]
        pub const fn wwdgrstf(&self) -> super::vals::Wwdgrstf {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Wwdgrstf::from_bits(val as u8)
        }
        #[doc = "window watchdog reset flag."]
        #[inline(always)]
        pub const fn set_wwdgrstf(&mut self, val: super::vals::Wwdgrstf) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Low-power reset flag."]
        #[must_use]
        #[inline(always)]
        pub const fn lpwrrstf(&self) -> super::vals::Lpwrrstf {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Lpwrrstf::from_bits(val as u8)
        }
        #[doc = "Low-power reset flag."]
        #[inline(always)]
        pub const fn set_lpwrrstf(&mut self, val: super::vals::Lpwrrstf) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
                "Rsr {{ rmvf: {:?}, pinrstf: {:?}, porrstf: {:?}, sftrstf: {:?}, iwdgrstf: {:?}, wwdgrstf: {:?}, lpwrrstf: {:?} }}",
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
        pub const fn lserdy(&self) -> super::vals::Lserdy {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Lserdy::from_bits(val as u8)
        }
        #[doc = "LSE oscillator ready."]
        #[inline(always)]
        pub const fn set_lserdy(&mut self, val: super::vals::Lserdy) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "LSE oscillator bypass."]
        #[must_use]
        #[inline(always)]
        pub const fn lsebyp(&self) -> super::vals::Lsebyp {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Lsebyp::from_bits(val as u8)
        }
        #[doc = "LSE oscillator bypass."]
        #[inline(always)]
        pub const fn set_lsebyp(&mut self, val: super::vals::Lsebyp) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
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
        pub const fn lsecssd(&self) -> super::vals::Lsecssd {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Lsecssd::from_bits(val as u8)
        }
        #[doc = "LSE clock security system failure detection."]
        #[inline(always)]
        pub const fn set_lsecssd(&mut self, val: super::vals::Lsecssd) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
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
        pub const fn lsirdy(&self) -> super::vals::Lsirdy {
            let val = (self.0 >> 27usize) & 0x01;
            super::vals::Lsirdy::from_bits(val as u8)
        }
        #[doc = "LSI oscillator ready."]
        #[inline(always)]
        pub const fn set_lsirdy(&mut self, val: super::vals::Lsirdy) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
        }
    }
    impl Default for Rtccr {
        #[inline(always)]
        fn default() -> Rtccr {
            Rtccr(0)
        }
    }
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
                "Rtccr {{ lseon: {=bool:?}, lserdy: {:?}, lsebyp: {:?}, lsedrv: {:?}, lsecsson: {=bool:?}, lsecssd: {:?}, rtcsel: {:?}, rtcen: {=bool:?}, rtcdrst: {=bool:?}, lsion: {=bool:?}, lsirdy: {:?} }}",
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
        pub const fn hsisec(&self) -> super::vals::Hsisec {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Hsisec::from_bits(val as u8)
        }
        #[doc = "HSI clock configuration and status bits security."]
        #[inline(always)]
        pub const fn set_hsisec(&mut self, val: super::vals::Hsisec) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "HSE clock configuration bits, status bits and HSE_CSS security."]
        #[must_use]
        #[inline(always)]
        pub const fn hsesec(&self) -> super::vals::Hsesec {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Hsesec::from_bits(val as u8)
        }
        #[doc = "HSE clock configuration bits, status bits and HSE_CSS security."]
        #[inline(always)]
        pub const fn set_hsesec(&mut self, val: super::vals::Hsesec) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "LSI clock configuration and status bits security."]
        #[must_use]
        #[inline(always)]
        pub const fn lsisec(&self) -> super::vals::Lsisec {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Lsisec::from_bits(val as u8)
        }
        #[doc = "LSI clock configuration and status bits security."]
        #[inline(always)]
        pub const fn set_lsisec(&mut self, val: super::vals::Lsisec) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "LSE clock configuration and status bits security."]
        #[must_use]
        #[inline(always)]
        pub const fn lsesec(&self) -> super::vals::Lsesec {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Lsesec::from_bits(val as u8)
        }
        #[doc = "LSE clock configuration and status bits security."]
        #[inline(always)]
        pub const fn set_lsesec(&mut self, val: super::vals::Lsesec) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "SYSCLK clock selection, clock output on MCO configuration security."]
        #[must_use]
        #[inline(always)]
        pub const fn sysclksec(&self) -> super::vals::Sysclksec {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Sysclksec::from_bits(val as u8)
        }
        #[doc = "SYSCLK clock selection, clock output on MCO configuration security."]
        #[inline(always)]
        pub const fn set_sysclksec(&mut self, val: super::vals::Sysclksec) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "AHBx/APBx prescaler configuration bits security."]
        #[must_use]
        #[inline(always)]
        pub const fn prescsec(&self) -> super::vals::Prescsec {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Prescsec::from_bits(val as u8)
        }
        #[doc = "AHBx/APBx prescaler configuration bits security."]
        #[inline(always)]
        pub const fn set_prescsec(&mut self, val: super::vals::Prescsec) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "PLL1 clock configuration and status bits security."]
        #[must_use]
        #[inline(always)]
        pub const fn pllsec(&self, n: usize) -> super::vals::Pllsec {
            assert!(n < 2usize);
            let offs = 7usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Pllsec::from_bits(val as u8)
        }
        #[doc = "PLL1 clock configuration and status bits security."]
        #[inline(always)]
        pub const fn set_pllsec(&mut self, n: usize, val: super::vals::Pllsec) {
            assert!(n < 2usize);
            let offs = 7usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "HSI48 clock configuration and status bits security."]
        #[must_use]
        #[inline(always)]
        pub const fn hsi48sec(&self) -> super::vals::Hsisec {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Hsisec::from_bits(val as u8)
        }
        #[doc = "HSI48 clock configuration and status bits security."]
        #[inline(always)]
        pub const fn set_hsi48sec(&mut self, val: super::vals::Hsisec) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Remove reset flag security."]
        #[must_use]
        #[inline(always)]
        pub const fn rmvfsec(&self) -> super::vals::Rmvfsec {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Rmvfsec::from_bits(val as u8)
        }
        #[doc = "Remove reset flag security."]
        #[inline(always)]
        pub const fn set_rmvfsec(&mut self, val: super::vals::Rmvfsec) {
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
                "Seccfgr {{ hsisec: {:?}, hsesec: {:?}, lsisec: {:?}, lsesec: {:?}, sysclksec: {:?}, prescsec: {:?}, pllsec[0]: {:?}, pllsec[1]: {:?}, hsi48sec: {:?}, rmvfsec: {:?} }}",
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adcdiv {
        #[doc = "ADC kernel clock not divided (default after reset)."]
        B0x0 = 0x0,
        #[doc = "ADC kernel clock divided by 2."]
        B0x1 = 0x01,
        #[doc = "ADC kernel clock divided by 4."]
        B0x2 = 0x02,
        #[doc = "ADC kernel clock divided by 6."]
        B0x3 = 0x03,
        #[doc = "ADC kernel clock divided by 8."]
        B0x4 = 0x04,
        #[doc = "ADC kernel clock divided by 10."]
        B0x5 = 0x05,
        #[doc = "ADC kernel clock divided by 12."]
        B0x6 = 0x06,
        #[doc = "ADC kernel clock divided by 16."]
        B0x7 = 0x07,
        #[doc = "ADC kernel clock divided by 32."]
        B0x8 = 0x08,
        #[doc = "ADC kernel clock divided by 64."]
        B0x9 = 0x09,
        #[doc = "ADC kernel clock divided by 128."]
        B0xA = 0x0a,
        #[doc = "ADC kernel clock divided by 256."]
        B0xB = 0x0b,
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ahbdis {
        #[doc = "AHB1 clock enabled, distributed to peripherals according to their dedicated clock enable control bits."]
        B0x0 = 0x0,
        #[doc = "AHB1 clock disabled."]
        B0x1 = 0x01,
    }
    impl Ahbdis {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ahbdis {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ahbdis {
        #[inline(always)]
        fn from(val: u8) -> Ahbdis {
            Ahbdis::from_bits(val)
        }
    }
    impl From<Ahbdis> for u8 {
        #[inline(always)]
        fn from(val: Ahbdis) -> u8 {
            Ahbdis::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Apbdis {
        #[doc = "APB1 clock enabled, distributed to peripherals according to their dedicated clock enable control bits."]
        B0x0 = 0x0,
        #[doc = "APB1 clock disabled."]
        B0x1 = 0x01,
    }
    impl Apbdis {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Apbdis {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Apbdis {
        #[inline(always)]
        fn from(val: u8) -> Apbdis {
            Apbdis::from_bits(val)
        }
    }
    impl From<Apbdis> for u8 {
        #[inline(always)]
        fn from(val: Apbdis) -> u8 {
            Apbdis::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hpre {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "rcc_hclk=sys_ck / 2."]
        B0x8 = 0x08,
        #[doc = "rcc_hclk=sys_ck / 4."]
        B0x9 = 0x09,
        #[doc = "rcc_hclk=sys_ck / 8."]
        B0xA = 0x0a,
        #[doc = "rcc_hclk=sys_ck / 16."]
        B0xB = 0x0b,
        #[doc = "rcc_hclk=sys_ck / 64."]
        B0xC = 0x0c,
        #[doc = "rcc_hclk=sys_ck / 128."]
        B0xD = 0x0d,
        #[doc = "rcc_hclk=sys_ck / 256."]
        B0xE = 0x0e,
        #[doc = "rcc_hclk=sys_ck / 512."]
        B0xF = 0x0f,
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
    pub enum Hsebyp {
        #[doc = "HSE oscillator not bypassed (default after reset)."]
        B0x0 = 0x0,
        #[doc = "HSE oscillator bypassed with an external clock."]
        B0x1 = 0x01,
    }
    impl Hsebyp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hsebyp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hsebyp {
        #[inline(always)]
        fn from(val: u8) -> Hsebyp {
            Hsebyp::from_bits(val)
        }
    }
    impl From<Hsebyp> for u8 {
        #[inline(always)]
        fn from(val: Hsebyp) -> u8 {
            Hsebyp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hsecssc {
        #[doc = "HSECSSF no effect (default after reset)."]
        B0x0 = 0x0,
        #[doc = "HSECSSF cleared."]
        B0x1 = 0x01,
    }
    impl Hsecssc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hsecssc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hsecssc {
        #[inline(always)]
        fn from(val: u8) -> Hsecssc {
            Hsecssc::from_bits(val)
        }
    }
    impl From<Hsecssc> for u8 {
        #[inline(always)]
        fn from(val: Hsecssc) -> u8 {
            Hsecssc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hsecssf {
        #[doc = "no clock security interrupt caused by HSE clock failure (default after reset)."]
        B0x0 = 0x0,
        #[doc = "clock security interrupt caused by HSE clock failure."]
        B0x1 = 0x01,
    }
    impl Hsecssf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hsecssf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hsecssf {
        #[inline(always)]
        fn from(val: u8) -> Hsecssf {
            Hsecssf::from_bits(val)
        }
    }
    impl From<Hsecssf> for u8 {
        #[inline(always)]
        fn from(val: Hsecssf) -> u8 {
            Hsecssf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hseext {
        #[doc = "HSE in analog mode (default after reset)."]
        B0x0 = 0x0,
        #[doc = "HSE in digital mode."]
        B0x1 = 0x01,
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
    pub enum Hserdy {
        #[doc = "HSE clock is not ready (default after reset)."]
        B0x0 = 0x0,
        #[doc = "HSE clock is ready."]
        B0x1 = 0x01,
    }
    impl Hserdy {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hserdy {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hserdy {
        #[inline(always)]
        fn from(val: u8) -> Hserdy {
            Hserdy::from_bits(val)
        }
    }
    impl From<Hserdy> for u8 {
        #[inline(always)]
        fn from(val: Hserdy) -> u8 {
            Hserdy::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hserdyc {
        #[doc = "HSERDYF no effect (default after reset)."]
        B0x0 = 0x0,
        #[doc = "HSERDYF cleared."]
        B0x1 = 0x01,
    }
    impl Hserdyc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hserdyc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hserdyc {
        #[inline(always)]
        fn from(val: u8) -> Hserdyc {
            Hserdyc::from_bits(val)
        }
    }
    impl From<Hserdyc> for u8 {
        #[inline(always)]
        fn from(val: Hserdyc) -> u8 {
            Hserdyc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hserdyf {
        #[doc = "no clock ready interrupt caused by the HSE (default after reset)."]
        B0x0 = 0x0,
        #[doc = "clock ready interrupt caused by the HSE."]
        B0x1 = 0x01,
    }
    impl Hserdyf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hserdyf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hserdyf {
        #[inline(always)]
        fn from(val: u8) -> Hserdyf {
            Hserdyf::from_bits(val)
        }
    }
    impl From<Hserdyf> for u8 {
        #[inline(always)]
        fn from(val: Hserdyf) -> u8 {
            Hserdyf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hserdyie {
        #[doc = "HSE ready interrupt disabled (default after reset)."]
        B0x0 = 0x0,
        #[doc = "HSE ready interrupt enabled."]
        B0x1 = 0x01,
    }
    impl Hserdyie {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hserdyie {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hserdyie {
        #[inline(always)]
        fn from(val: u8) -> Hserdyie {
            Hserdyie::from_bits(val)
        }
    }
    impl From<Hserdyie> for u8 {
        #[inline(always)]
        fn from(val: Hserdyie) -> u8 {
            Hserdyie::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hsesec {
        #[doc = "nonsecure."]
        B0x0 = 0x0,
        #[doc = "secure."]
        B0x1 = 0x01,
    }
    impl Hsesec {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hsesec {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hsesec {
        #[inline(always)]
        fn from(val: u8) -> Hsesec {
            Hsesec::from_bits(val)
        }
    }
    impl From<Hsesec> for u8 {
        #[inline(always)]
        fn from(val: Hsesec) -> u8 {
            Hsesec::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hsidiv {
        #[doc = "division by 1, hsi_ck, hsi_ker_ck=64MHz (default after reset)."]
        B0x0 = 0x0,
        #[doc = "division by 2, hsi_ck, hsi_ker_ck=32MHz."]
        B0x1 = 0x01,
        #[doc = "division by 4, hsi_ck, hsi_ker_ck=16MHz."]
        B0x2 = 0x02,
        #[doc = "division by 8, hsi_ck, hsi_ker_ck=8MHz."]
        B0x3 = 0x03,
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
    pub enum Hsidivf {
        #[doc = "new division ratio not yet propagated to hsi_ck, hsi_ker_ck (default after reset)."]
        B0x0 = 0x0,
        #[doc = "hsi_ck, hsi_ker_ck clock frequency reflects the new HSIDIV value (default register value when the clock setting is completed)."]
        B0x1 = 0x01,
    }
    impl Hsidivf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hsidivf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hsidivf {
        #[inline(always)]
        fn from(val: u8) -> Hsidivf {
            Hsidivf::from_bits(val)
        }
    }
    impl From<Hsidivf> for u8 {
        #[inline(always)]
        fn from(val: Hsidivf) -> u8 {
            Hsidivf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hsirdy {
        #[doc = "HSI48 clock is not ready (default after reset)."]
        B0x0 = 0x0,
        #[doc = "HSI48 clock is ready."]
        B0x1 = 0x01,
    }
    impl Hsirdy {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hsirdy {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hsirdy {
        #[inline(always)]
        fn from(val: u8) -> Hsirdy {
            Hsirdy::from_bits(val)
        }
    }
    impl From<Hsirdy> for u8 {
        #[inline(always)]
        fn from(val: Hsirdy) -> u8 {
            Hsirdy::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hsirdyc {
        #[doc = "HSI48RDYF no effect (default after reset)."]
        B0x0 = 0x0,
        #[doc = "HSI48RDYF cleared."]
        B0x1 = 0x01,
    }
    impl Hsirdyc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hsirdyc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hsirdyc {
        #[inline(always)]
        fn from(val: u8) -> Hsirdyc {
            Hsirdyc::from_bits(val)
        }
    }
    impl From<Hsirdyc> for u8 {
        #[inline(always)]
        fn from(val: Hsirdyc) -> u8 {
            Hsirdyc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hsirdyf {
        #[doc = "no clock ready interrupt caused by the HSI48 oscillator (default after reset)."]
        B0x0 = 0x0,
        #[doc = "clock ready interrupt caused by the HSI48 oscillator."]
        B0x1 = 0x01,
    }
    impl Hsirdyf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hsirdyf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hsirdyf {
        #[inline(always)]
        fn from(val: u8) -> Hsirdyf {
            Hsirdyf::from_bits(val)
        }
    }
    impl From<Hsirdyf> for u8 {
        #[inline(always)]
        fn from(val: Hsirdyf) -> u8 {
            Hsirdyf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hsirdyie {
        #[doc = "HSI48 ready interrupt disabled (default after reset)."]
        B0x0 = 0x0,
        #[doc = "HSI48 ready interrupt enabled."]
        B0x1 = 0x01,
    }
    impl Hsirdyie {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hsirdyie {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hsirdyie {
        #[inline(always)]
        fn from(val: u8) -> Hsirdyie {
            Hsirdyie::from_bits(val)
        }
    }
    impl From<Hsirdyie> for u8 {
        #[inline(always)]
        fn from(val: Hsirdyie) -> u8 {
            Hsirdyie::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hsisec {
        #[doc = "nonsecure."]
        B0x0 = 0x0,
        #[doc = "secure."]
        B0x1 = 0x01,
    }
    impl Hsisec {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hsisec {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hsisec {
        #[inline(always)]
        fn from(val: u8) -> Hsisec {
            Hsisec::from_bits(val)
        }
    }
    impl From<Hsisec> for u8 {
        #[inline(always)]
        fn from(val: Hsisec) -> u8 {
            Hsisec::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Iclkdiv {
        #[doc = "Intermediate clock 1 division ratio is 5 (default after reset)."]
        B0x0 = 0x0,
        #[doc = "Intermediate clock 1 division ratio is 3."]
        B0x1 = 0x01,
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Iwdgrstf {
        #[doc = "no independent watchdog reset occurred (default after power-on reset)."]
        B0x0 = 0x0,
        #[doc = "independent watchdog reset occurred."]
        B0x1 = 0x01,
    }
    impl Iwdgrstf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Iwdgrstf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Iwdgrstf {
        #[inline(always)]
        fn from(val: u8) -> Iwdgrstf {
            Iwdgrstf::from_bits(val)
        }
    }
    impl From<Iwdgrstf> for u8 {
        #[inline(always)]
        fn from(val: Iwdgrstf) -> u8 {
            Iwdgrstf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lpwrrstf {
        #[doc = "No illegal low-power mode reset occurred."]
        B0x0 = 0x0,
        #[doc = "Illegal low-power mode reset occurred."]
        B0x1 = 0x01,
    }
    impl Lpwrrstf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lpwrrstf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lpwrrstf {
        #[inline(always)]
        fn from(val: u8) -> Lpwrrstf {
            Lpwrrstf::from_bits(val)
        }
    }
    impl From<Lpwrrstf> for u8 {
        #[inline(always)]
        fn from(val: Lpwrrstf) -> u8 {
            Lpwrrstf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lsebyp {
        #[doc = "LSE oscillator not bypassed (default after VDD domain reset)."]
        B0x0 = 0x0,
        #[doc = "LSE oscillator bypassed."]
        B0x1 = 0x01,
    }
    impl Lsebyp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lsebyp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lsebyp {
        #[inline(always)]
        fn from(val: u8) -> Lsebyp {
            Lsebyp::from_bits(val)
        }
    }
    impl From<Lsebyp> for u8 {
        #[inline(always)]
        fn from(val: Lsebyp) -> u8 {
            Lsebyp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lsecssc {
        #[doc = "LSECSSF no effect (default after reset)."]
        B0x0 = 0x0,
        #[doc = "LSECSSF cleared."]
        B0x1 = 0x01,
    }
    impl Lsecssc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lsecssc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lsecssc {
        #[inline(always)]
        fn from(val: u8) -> Lsecssc {
            Lsecssc::from_bits(val)
        }
    }
    impl From<Lsecssc> for u8 {
        #[inline(always)]
        fn from(val: Lsecssc) -> u8 {
            Lsecssc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lsecssd {
        #[doc = "no failure detected on 32kHz oscillator (default after VDD domain reset)."]
        B0x0 = 0x0,
        #[doc = "failure detected on 32kHz oscillator."]
        B0x1 = 0x01,
    }
    impl Lsecssd {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lsecssd {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lsecssd {
        #[inline(always)]
        fn from(val: u8) -> Lsecssd {
            Lsecssd::from_bits(val)
        }
    }
    impl From<Lsecssd> for u8 {
        #[inline(always)]
        fn from(val: Lsecssd) -> u8 {
            Lsecssd::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lsecssf {
        #[doc = "no clock security interrupt caused by HSE clock failure (default after reset)."]
        B0x0 = 0x0,
        #[doc = "clock security interrupt caused by HSE clock failure."]
        B0x1 = 0x01,
    }
    impl Lsecssf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lsecssf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lsecssf {
        #[inline(always)]
        fn from(val: u8) -> Lsecssf {
            Lsecssf::from_bits(val)
        }
    }
    impl From<Lsecssf> for u8 {
        #[inline(always)]
        fn from(val: Lsecssf) -> u8 {
            Lsecssf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lsecssie {
        #[doc = "LSE CSS interrupt disabled (default after reset)."]
        B0x0 = 0x0,
        #[doc = "LSE CSS interrupt enabled."]
        B0x1 = 0x01,
    }
    impl Lsecssie {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lsecssie {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lsecssie {
        #[inline(always)]
        fn from(val: u8) -> Lsecssie {
            Lsecssie::from_bits(val)
        }
    }
    impl From<Lsecssie> for u8 {
        #[inline(always)]
        fn from(val: Lsecssie) -> u8 {
            Lsecssie::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lsedrv {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "medium-high drive."]
        B0x2 = 0x02,
        #[doc = "highest drive."]
        B0x3 = 0x03,
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
    pub enum Lserdy {
        #[doc = "LSE oscillator not ready (default after VDD domain reset)."]
        B0x0 = 0x0,
        #[doc = "LSE oscillator ready."]
        B0x1 = 0x01,
    }
    impl Lserdy {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lserdy {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lserdy {
        #[inline(always)]
        fn from(val: u8) -> Lserdy {
            Lserdy::from_bits(val)
        }
    }
    impl From<Lserdy> for u8 {
        #[inline(always)]
        fn from(val: Lserdy) -> u8 {
            Lserdy::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lserdyc {
        #[doc = "LSERDYF no effect (default after reset)."]
        B0x0 = 0x0,
        #[doc = "LSERDYF cleared."]
        B0x1 = 0x01,
    }
    impl Lserdyc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lserdyc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lserdyc {
        #[inline(always)]
        fn from(val: u8) -> Lserdyc {
            Lserdyc::from_bits(val)
        }
    }
    impl From<Lserdyc> for u8 {
        #[inline(always)]
        fn from(val: Lserdyc) -> u8 {
            Lserdyc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lserdyf {
        #[doc = "no clock ready interrupt caused by the LSE (default after reset)."]
        B0x0 = 0x0,
        #[doc = "clock ready interrupt caused by the LSE."]
        B0x1 = 0x01,
    }
    impl Lserdyf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lserdyf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lserdyf {
        #[inline(always)]
        fn from(val: u8) -> Lserdyf {
            Lserdyf::from_bits(val)
        }
    }
    impl From<Lserdyf> for u8 {
        #[inline(always)]
        fn from(val: Lserdyf) -> u8 {
            Lserdyf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lserdyie {
        #[doc = "LSE ready interrupt disabled (default after reset)."]
        B0x0 = 0x0,
        #[doc = "LSE ready interrupt enabled."]
        B0x1 = 0x01,
    }
    impl Lserdyie {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lserdyie {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lserdyie {
        #[inline(always)]
        fn from(val: u8) -> Lserdyie {
            Lserdyie::from_bits(val)
        }
    }
    impl From<Lserdyie> for u8 {
        #[inline(always)]
        fn from(val: Lserdyie) -> u8 {
            Lserdyie::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lsesec {
        #[doc = "nonsecure."]
        B0x0 = 0x0,
        #[doc = "secure."]
        B0x1 = 0x01,
    }
    impl Lsesec {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lsesec {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lsesec {
        #[inline(always)]
        fn from(val: u8) -> Lsesec {
            Lsesec::from_bits(val)
        }
    }
    impl From<Lsesec> for u8 {
        #[inline(always)]
        fn from(val: Lsesec) -> u8 {
            Lsesec::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lsirdy {
        #[doc = "LSI oscillator not ready (default after power-on reset)."]
        B0x0 = 0x0,
        #[doc = "LSI oscillator ready."]
        B0x1 = 0x01,
    }
    impl Lsirdy {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lsirdy {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lsirdy {
        #[inline(always)]
        fn from(val: u8) -> Lsirdy {
            Lsirdy::from_bits(val)
        }
    }
    impl From<Lsirdy> for u8 {
        #[inline(always)]
        fn from(val: Lsirdy) -> u8 {
            Lsirdy::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lsirdyc {
        #[doc = "LSIRDYF no effect (default after reset)."]
        B0x0 = 0x0,
        #[doc = "LSIRDYF cleared."]
        B0x1 = 0x01,
    }
    impl Lsirdyc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lsirdyc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lsirdyc {
        #[inline(always)]
        fn from(val: u8) -> Lsirdyc {
            Lsirdyc::from_bits(val)
        }
    }
    impl From<Lsirdyc> for u8 {
        #[inline(always)]
        fn from(val: Lsirdyc) -> u8 {
            Lsirdyc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lsirdyf {
        #[doc = "no clock ready interrupt caused by the LSI (default after reset)."]
        B0x0 = 0x0,
        #[doc = "clock ready interrupt caused by the LSI."]
        B0x1 = 0x01,
    }
    impl Lsirdyf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lsirdyf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lsirdyf {
        #[inline(always)]
        fn from(val: u8) -> Lsirdyf {
            Lsirdyf::from_bits(val)
        }
    }
    impl From<Lsirdyf> for u8 {
        #[inline(always)]
        fn from(val: Lsirdyf) -> u8 {
            Lsirdyf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lsirdyie {
        #[doc = "LSI ready interrupt disabled (default after reset)."]
        B0x0 = 0x0,
        #[doc = "LSI ready interrupt enabled."]
        B0x1 = 0x01,
    }
    impl Lsirdyie {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lsirdyie {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lsirdyie {
        #[inline(always)]
        fn from(val: u8) -> Lsirdyie {
            Lsirdyie::from_bits(val)
        }
    }
    impl From<Lsirdyie> for u8 {
        #[inline(always)]
        fn from(val: Lsirdyie) -> u8 {
            Lsirdyie::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lsisec {
        #[doc = "nonsecure."]
        B0x0 = 0x0,
        #[doc = "secure."]
        B0x1 = 0x01,
    }
    impl Lsisec {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lsisec {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lsisec {
        #[inline(always)]
        fn from(val: u8) -> Lsisec {
            Lsisec::from_bits(val)
        }
    }
    impl From<Lsisec> for u8 {
        #[inline(always)]
        fn from(val: Lsisec) -> u8 {
            Lsisec::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mcopre {
        #[doc = "prescaler disabled, no clock output on MCO1 (default after reset)."]
        B0x0 = 0x0,
        #[doc = "division by 1 (bypass)."]
        B0x1 = 0x01,
        #[doc = "division by 2."]
        B0x2 = 0x02,
        #[doc = "division by 3."]
        B0x3 = 0x03,
        #[doc = "division by 4."]
        B0x4 = 0x04,
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
        B0xF = 0x0f,
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
    pub enum Mcosel {
        #[doc = "HSI clock selected (hsi_ck) (default after reset)."]
        B0x0 = 0x0,
        #[doc = "LSE oscillator clock selected (lse_ck)."]
        B0x1 = 0x01,
        #[doc = "HSE clock selected (hse_ck)."]
        B0x2 = 0x02,
        #[doc = "PLL1 clock selected (pll1_q_ck)."]
        B0x3 = 0x03,
        #[doc = "HSI48 clock selected (hsi48_ker_ck)."]
        B0x4 = 0x04,
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Nspriv {
        #[doc = "Read and write to RCC nonsecure functions can be done by privileged or unprivileged access."]
        B0x0 = 0x0,
        #[doc = "Read and write to RCC nonsecure functions can be done by privileged access only."]
        B0x1 = 0x01,
    }
    impl Nspriv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Nspriv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Nspriv {
        #[inline(always)]
        fn from(val: u8) -> Nspriv {
            Nspriv::from_bits(val)
        }
    }
    impl From<Nspriv> for u8 {
        #[inline(always)]
        fn from(val: Nspriv) -> u8 {
            Nspriv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pinrstf {
        #[doc = "no reset from pin occurred."]
        B0x0 = 0x0,
        #[doc = "reset from pin occurred (default after power-on reset)."]
        B0x1 = 0x01,
    }
    impl Pinrstf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pinrstf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pinrstf {
        #[inline(always)]
        fn from(val: u8) -> Pinrstf {
            Pinrstf::from_bits(val)
        }
    }
    impl From<Pinrstf> for u8 {
        #[inline(always)]
        fn from(val: Pinrstf) -> u8 {
            Pinrstf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Playdiv {
        #[doc = "Selected kernel is not divided (default after reset)."]
        B0x0 = 0x0,
        #[doc = "Selected kernel clock is divided by 2."]
        B0x1 = 0x01,
        #[doc = "Selected kernel clock is divided by 4."]
        B0x2 = 0x02,
        #[doc = "Selected kernel clock is divided by 8."]
        B0x3 = 0x03,
        #[doc = "Selected kernel clock is divided by 16."]
        B0x4 = 0x04,
        #[doc = "Selected kernel clock is divided by 32."]
        B0x5 = 0x05,
        #[doc = "Selected kernel clock is divided by 64."]
        B0x6 = 0x06,
        #[doc = "Selected kernel clock is divided by 128."]
        B0x7 = 0x07,
        #[doc = "Selected kernel clock is divided by 256."]
        B0x8 = 0x08,
        #[doc = "Selected kernel clock is divided by 512."]
        B0x9 = 0x09,
        #[doc = "Selected kernel clock is divided by 1024."]
        B0xA = 0x0a,
        #[doc = "Selected kernel clock is divided by 2048."]
        B0xB = 0x0b,
        #[doc = "Selected kernel clock is divided by 4096."]
        B0xC = 0x0c,
        #[doc = "Selected kernel clock is divided by 8192."]
        B0xD = 0x0d,
        #[doc = "Selected kernel clock is divided by 16384."]
        B0xE = 0x0e,
        #[doc = "Selected kernel clock is divided by 32768."]
        B0xF = 0x0f,
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pll1rge {
        #[doc = "PLL1 input (ref1_ck) clock range frequency between 1 and 2MHz (default after reset)."]
        B0x0 = 0x0,
        #[doc = "PLL1 input (ref1_ck) clock range frequency between 2 and 4MHz."]
        B0x1 = 0x01,
        #[doc = "PLL1 input (ref1_ck) clock range frequency between 4 and 8MHz."]
        B0x2 = 0x02,
        #[doc = "PLL1 input (ref1_ck) clock range frequency between 8 and 16MHz."]
        B0x3 = 0x03,
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
    pub enum Pllm {
        #[doc = "prescaler disabled (default after reset)."]
        B0x0 = 0x0,
        #[doc = "division by 1 (bypass)."]
        B0x1 = 0x01,
        #[doc = "division by 2."]
        B0x2 = 0x02,
        #[doc = "division by 3."]
        B0x3 = 0x03,
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
        #[doc = "division by 32."]
        B0x20 = 0x20,
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
        B0x3f = 0x3f,
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
    pub struct Plln(u16);
    impl Plln {
        #[doc = "PLL1N=4."]
        pub const B0x003: Self = Self(0x03);
        #[doc = "PLL1N=5."]
        pub const B0x004: Self = Self(0x04);
        #[doc = "PLL1N=6."]
        pub const B0x005: Self = Self(0x05);
        #[doc = "PLL1N=129 (default after reset)."]
        pub const B0x080: Self = Self(0x80);
        #[doc = "PLL1N=512."]
        pub const B0x1ff: Self = Self(0x01ff);
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
                0x03 => f.write_str("B0x003"),
                0x04 => f.write_str("B0x004"),
                0x05 => f.write_str("B0x005"),
                0x80 => f.write_str("B0x080"),
                0x01ff => f.write_str("B0x1ff"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Plln {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x03 => defmt::write!(f, "B0x003"),
                0x04 => defmt::write!(f, "B0x004"),
                0x05 => defmt::write!(f, "B0x005"),
                0x80 => defmt::write!(f, "B0x080"),
                0x01ff => defmt::write!(f, "B0x1ff"),
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
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pllp(u8);
    impl Pllp {
        #[doc = "Not allowed."]
        pub const B0x0: Self = Self(0x0);
        #[doc = "pll1_p_ck=vco1_ck / 2 (default after reset)."]
        pub const B0x1: Self = Self(0x01);
        #[doc = "Not allowed."]
        pub const B0x2: Self = Self(0x02);
        #[doc = "pll1_p_ck=vco1_ck / 4."]
        pub const B0x3: Self = Self(0x03);
        #[doc = "pll1_p_ck=vco1_ck / 128."]
        pub const B0x7f: Self = Self(0x7f);
    }
    impl Pllp {
        pub const fn from_bits(val: u8) -> Pllp {
            Self(val & 0x7f)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for Pllp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("B0x0"),
                0x01 => f.write_str("B0x1"),
                0x02 => f.write_str("B0x2"),
                0x03 => f.write_str("B0x3"),
                0x7f => f.write_str("B0x7f"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pllp {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "B0x0"),
                0x01 => defmt::write!(f, "B0x1"),
                0x02 => defmt::write!(f, "B0x2"),
                0x03 => defmt::write!(f, "B0x3"),
                0x7f => defmt::write!(f, "B0x7f"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
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
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pllq(u8);
    impl Pllq {
        #[doc = "pll1_q_ck=vco1_ck."]
        pub const B0x0: Self = Self(0x0);
        #[doc = "pll1_q_ck=vco1_ck / 2 (default after reset)."]
        pub const B0x1: Self = Self(0x01);
        #[doc = "pll1_q_ck=vco1_ck / 3."]
        pub const B0x2: Self = Self(0x02);
        #[doc = "pll1_q_ck=vco1_ck / 4."]
        pub const B0x3: Self = Self(0x03);
        #[doc = "pll1_q_ck=vco1_ck / 128."]
        pub const B0x7f: Self = Self(0x7f);
    }
    impl Pllq {
        pub const fn from_bits(val: u8) -> Pllq {
            Self(val & 0x7f)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for Pllq {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("B0x0"),
                0x01 => f.write_str("B0x1"),
                0x02 => f.write_str("B0x2"),
                0x03 => f.write_str("B0x3"),
                0x7f => f.write_str("B0x7f"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pllq {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "B0x0"),
                0x01 => defmt::write!(f, "B0x1"),
                0x02 => defmt::write!(f, "B0x2"),
                0x03 => defmt::write!(f, "B0x3"),
                0x7f => defmt::write!(f, "B0x7f"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
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
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Pllr(u8);
    impl Pllr {
        #[doc = "pll1_r_ck=vco1_ck / 1."]
        pub const B0x0: Self = Self(0x0);
        #[doc = "pll1_r_ck=vco1_ck / 2 (default after reset)."]
        pub const B0x1: Self = Self(0x01);
        #[doc = "pll1_r_ck=vco1_ck / 3."]
        pub const B0x2: Self = Self(0x02);
        #[doc = "pll1_r_ck=vco1_ck / 4."]
        pub const B0x3: Self = Self(0x03);
        #[doc = "pll1_r_ck=vco1_ck / 128."]
        pub const B0x7f: Self = Self(0x7f);
    }
    impl Pllr {
        pub const fn from_bits(val: u8) -> Pllr {
            Self(val & 0x7f)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for Pllr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("B0x0"),
                0x01 => f.write_str("B0x1"),
                0x02 => f.write_str("B0x2"),
                0x03 => f.write_str("B0x3"),
                0x7f => f.write_str("B0x7f"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pllr {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "B0x0"),
                0x01 => defmt::write!(f, "B0x1"),
                0x02 => defmt::write!(f, "B0x2"),
                0x03 => defmt::write!(f, "B0x3"),
                0x7f => defmt::write!(f, "B0x7f"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllrdy {
        #[doc = "PLL1 unlocked (default after reset)."]
        B0x0 = 0x0,
        #[doc = "PLL1 locked."]
        B0x1 = 0x01,
    }
    impl Pllrdy {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllrdy {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllrdy {
        #[inline(always)]
        fn from(val: u8) -> Pllrdy {
            Pllrdy::from_bits(val)
        }
    }
    impl From<Pllrdy> for u8 {
        #[inline(always)]
        fn from(val: Pllrdy) -> u8 {
            Pllrdy::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllrdyc {
        #[doc = "PLL1RDYF no effect (default after reset)."]
        B0x0 = 0x0,
        #[doc = "PLL1RDYF cleared."]
        B0x1 = 0x01,
    }
    impl Pllrdyc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllrdyc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllrdyc {
        #[inline(always)]
        fn from(val: u8) -> Pllrdyc {
            Pllrdyc::from_bits(val)
        }
    }
    impl From<Pllrdyc> for u8 {
        #[inline(always)]
        fn from(val: Pllrdyc) -> u8 {
            Pllrdyc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllrdyf {
        #[doc = "no clock ready interrupt caused by PLL1 lock (default after reset)."]
        B0x0 = 0x0,
        #[doc = "clock ready interrupt caused by PLL1 lock."]
        B0x1 = 0x01,
    }
    impl Pllrdyf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllrdyf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllrdyf {
        #[inline(always)]
        fn from(val: u8) -> Pllrdyf {
            Pllrdyf::from_bits(val)
        }
    }
    impl From<Pllrdyf> for u8 {
        #[inline(always)]
        fn from(val: Pllrdyf) -> u8 {
            Pllrdyf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllrdyie {
        #[doc = "PLL1 lock interrupt disabled (default after reset)."]
        B0x0 = 0x0,
        #[doc = "PLL1 lock interrupt enabled."]
        B0x1 = 0x01,
    }
    impl Pllrdyie {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllrdyie {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllrdyie {
        #[inline(always)]
        fn from(val: u8) -> Pllrdyie {
            Pllrdyie::from_bits(val)
        }
    }
    impl From<Pllrdyie> for u8 {
        #[inline(always)]
        fn from(val: Pllrdyie) -> u8 {
            Pllrdyie::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllsec {
        #[doc = "nonsecure."]
        B0x0 = 0x0,
        #[doc = "secure."]
        B0x1 = 0x01,
    }
    impl Pllsec {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllsec {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllsec {
        #[inline(always)]
        fn from(val: u8) -> Pllsec {
            Pllsec::from_bits(val)
        }
    }
    impl From<Pllsec> for u8 {
        #[inline(always)]
        fn from(val: Pllsec) -> u8 {
            Pllsec::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllsrc {
        #[doc = "no clock sent to PLL1M divider and PLL1 (default after reset)."]
        B0x0 = 0x0,
        #[doc = "HSI selected as PLL clock (hsi_ck)."]
        B0x1 = 0x01,
        #[doc = "HSE selected as PLL clock (hse_ck)."]
        B0x2 = 0x02,
        #[doc = "no clock sent to PLL1M divider and PLL1."]
        B0x3 = 0x03,
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
        #[doc = "wide VCO range: 128 to 560MHz (default after reset)."]
        B0x0 = 0x0,
        #[doc = "medium VCO range: 150 to 420MHz."]
        B0x1 = 0x01,
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
    pub enum Porrstf {
        #[doc = "no POR reset occurred."]
        B0x0 = 0x0,
        #[doc = "POR reset occurred (default after power-on reset)."]
        B0x1 = 0x01,
    }
    impl Porrstf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Porrstf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Porrstf {
        #[inline(always)]
        fn from(val: u8) -> Porrstf {
            Porrstf::from_bits(val)
        }
    }
    impl From<Porrstf> for u8 {
        #[inline(always)]
        fn from(val: Porrstf) -> u8 {
            Porrstf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ppre {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "rcc_pclk1=rcc_hclk1 / 2."]
        B0x4 = 0x04,
        #[doc = "rcc_pclk1=rcc_hclk1 / 4."]
        B0x5 = 0x05,
        #[doc = "rcc_pclk1=rcc_hclk1 / 8."]
        B0x6 = 0x06,
        #[doc = "rcc_pclk1=rcc_hclk1 / 16."]
        B0x7 = 0x07,
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
    pub enum Prescsec {
        #[doc = "nonsecure."]
        B0x0 = 0x0,
        #[doc = "secure."]
        B0x1 = 0x01,
    }
    impl Prescsec {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Prescsec {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Prescsec {
        #[inline(always)]
        fn from(val: u8) -> Prescsec {
            Prescsec::from_bits(val)
        }
    }
    impl From<Prescsec> for u8 {
        #[inline(always)]
        fn from(val: Prescsec) -> u8 {
            Prescsec::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rmvf {
        #[doc = "reset of the reset flags not activated (default after power-on reset)."]
        B0x0 = 0x0,
        #[doc = "resets the value of the reset flags."]
        B0x1 = 0x01,
    }
    impl Rmvf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rmvf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rmvf {
        #[inline(always)]
        fn from(val: u8) -> Rmvf {
            Rmvf::from_bits(val)
        }
    }
    impl From<Rmvf> for u8 {
        #[inline(always)]
        fn from(val: Rmvf) -> u8 {
            Rmvf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rmvfsec {
        #[doc = "nonsecure."]
        B0x0 = 0x0,
        #[doc = "secure."]
        B0x1 = 0x01,
    }
    impl Rmvfsec {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rmvfsec {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rmvfsec {
        #[inline(always)]
        fn from(val: u8) -> Rmvfsec {
            Rmvfsec::from_bits(val)
        }
    }
    impl From<Rmvfsec> for u8 {
        #[inline(always)]
        fn from(val: Rmvfsec) -> u8 {
            Rmvfsec::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rtcpre {
        #[doc = "no clock (default after reset)."]
        B0x0 = 0x0,
        #[doc = "no clock."]
        B0x1 = 0x01,
        #[doc = "HSE/2."]
        B0x2 = 0x02,
        #[doc = "HSE/3."]
        B0x3 = 0x03,
        #[doc = "HSE/4."]
        B0x4 = 0x04,
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
        _RESERVED_20 = 0x20,
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
        #[doc = "HSE/62."]
        B0x3e = 0x3e,
        #[doc = "HSE/63."]
        B0x3f = 0x3f,
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rtcsel {
        #[doc = "no clock (default after RTC domain reset)."]
        B0x0 = 0x0,
        #[doc = "LSE selected as RTC clock."]
        B0x1 = 0x01,
        #[doc = "LSI selected as RTC clock."]
        B0x2 = 0x02,
        #[doc = "HSE divided by RTCPRE value selected as RTC clock."]
        B0x3 = 0x03,
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
    pub enum Sftrstf {
        #[doc = "no CPU software reset occurred (default after power-on reset)."]
        B0x0 = 0x0,
        #[doc = "a system reset has been generated by the CPU."]
        B0x1 = 0x01,
    }
    impl Sftrstf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sftrstf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sftrstf {
        #[inline(always)]
        fn from(val: u8) -> Sftrstf {
            Sftrstf::from_bits(val)
        }
    }
    impl From<Sftrstf> for u8 {
        #[inline(always)]
        fn from(val: Sftrstf) -> u8 {
            Sftrstf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Spriv {
        #[doc = "Read and write to RCC secure functions can be done by privileged or unprivileged access."]
        B0x0 = 0x0,
        #[doc = "Read and write to RCC secure functions can be done by privileged access only."]
        B0x1 = 0x01,
    }
    impl Spriv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Spriv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Spriv {
        #[inline(always)]
        fn from(val: u8) -> Spriv {
            Spriv::from_bits(val)
        }
    }
    impl From<Spriv> for u8 {
        #[inline(always)]
        fn from(val: Spriv) -> u8 {
            Spriv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sysclksec {
        #[doc = "nonsecure."]
        B0x0 = 0x0,
        #[doc = "secure."]
        B0x1 = 0x01,
    }
    impl Sysclksec {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sysclksec {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sysclksec {
        #[inline(always)]
        fn from(val: u8) -> Sysclksec {
            Sysclksec::from_bits(val)
        }
    }
    impl From<Sysclksec> for u8 {
        #[inline(always)]
        fn from(val: Sysclksec) -> u8 {
            Sysclksec::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Syssws {
        #[doc = "HSI used as system clock (hsi_ck) (default after reset)."]
        B0x0 = 0x0,
        #[doc = "HSE used as system clock (hse_ck)."]
        B0x1 = 0x01,
        #[doc = "PLL1 used as system clock (pll1_p_ck)."]
        B0x2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Syssws {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Syssws {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Syssws {
        #[inline(always)]
        fn from(val: u8) -> Syssws {
            Syssws::from_bits(val)
        }
    }
    impl From<Syssws> for u8 {
        #[inline(always)]
        fn from(val: Syssws) -> u8 {
            Syssws::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wwdgrstf {
        #[doc = "no window watchdog reset occurred from WWDG (default after power-on reset)."]
        B0x0 = 0x0,
        #[doc = "window watchdog reset occurred from WWDG."]
        B0x1 = 0x01,
    }
    impl Wwdgrstf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wwdgrstf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wwdgrstf {
        #[inline(always)]
        fn from(val: u8) -> Wwdgrstf {
            Wwdgrstf::from_bits(val)
        }
    }
    impl From<Wwdgrstf> for u8 {
        #[inline(always)]
        fn from(val: Wwdgrstf) -> u8 {
            Wwdgrstf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
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
