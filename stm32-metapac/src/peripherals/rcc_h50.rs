#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Reset and clock controller"]
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
    #[doc = "RCC HSI calibration register"]
    #[inline(always)]
    pub const fn hsicfgr(self) -> crate::common::Reg<regs::Hsicfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "RCC clock recovery RC register"]
    #[inline(always)]
    pub const fn crrcr(self) -> crate::common::Reg<regs::Crrcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "RCC CSI calibration register"]
    #[inline(always)]
    pub const fn csicfgr(self) -> crate::common::Reg<regs::Csicfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "RCC clock configuration register"]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "RCC CPU domain clock configuration register 2"]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "RCC PLL clock source selection register"]
    #[inline(always)]
    pub const fn pllcfgr(self, n: usize) -> crate::common::Reg<regs::Pllcfgr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize + n * 4usize) as _) }
    }
    #[doc = "RCC PLL1 dividers register"]
    #[inline(always)]
    pub const fn plldivr(self, n: usize) -> crate::common::Reg<regs::Plldivr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize + n * 8usize) as _) }
    }
    #[doc = "RCC PLL1 fractional divider register"]
    #[inline(always)]
    pub const fn pllfracr(self, n: usize) -> crate::common::Reg<regs::Pllfracr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize + n * 8usize) as _) }
    }
    #[doc = "RCC clock source interrupt enable register"]
    #[inline(always)]
    pub const fn cier(self) -> crate::common::Reg<regs::Cier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "RCC clock source interrupt flag register"]
    #[inline(always)]
    pub const fn cifr(self) -> crate::common::Reg<regs::Cifr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "RCC clock source interrupt clear register"]
    #[inline(always)]
    pub const fn cicr(self) -> crate::common::Reg<regs::Cicr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "RCC AHB1 reset register"]
    #[inline(always)]
    pub const fn ahb1rstr(self) -> crate::common::Reg<regs::Ahb1rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "RCC AHB2 peripheral reset register"]
    #[inline(always)]
    pub const fn ahb2rstr(self) -> crate::common::Reg<regs::Ahb2rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "RCC APB1 peripheral low reset register"]
    #[inline(always)]
    pub const fn apb1lrstr(self) -> crate::common::Reg<regs::Apb1lrstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "RCC APB1 peripheral high reset register"]
    #[inline(always)]
    pub const fn apb1hrstr(self) -> crate::common::Reg<regs::Apb1hrstr, crate::common::RW> {
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
    #[doc = "RCC AHB1 peripherals clock register"]
    #[inline(always)]
    pub const fn ahb1enr(self) -> crate::common::Reg<regs::Ahb1enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "RCC AHB2 peripheral clock register"]
    #[inline(always)]
    pub const fn ahb2enr(self) -> crate::common::Reg<regs::Ahb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "RCC APB1 peripheral clock register"]
    #[inline(always)]
    pub const fn apb1lenr(self) -> crate::common::Reg<regs::Apb1lenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "RCC APB1 peripheral clock register"]
    #[inline(always)]
    pub const fn apb1henr(self) -> crate::common::Reg<regs::Apb1henr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "RCC APB2 peripheral clock register"]
    #[inline(always)]
    pub const fn apb2enr(self) -> crate::common::Reg<regs::Apb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "RCC APB3 peripheral clock register"]
    #[inline(always)]
    pub const fn apb3enr(self) -> crate::common::Reg<regs::Apb3enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "RCC AHB1 sleep clock register"]
    #[inline(always)]
    pub const fn ahb1lpenr(self) -> crate::common::Reg<regs::Ahb1lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "RCC AHB2 sleep clock register"]
    #[inline(always)]
    pub const fn ahb2lpenr(self) -> crate::common::Reg<regs::Ahb2lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[doc = "RCC APB1 sleep clock register"]
    #[inline(always)]
    pub const fn apb1llpenr(self) -> crate::common::Reg<regs::Apb1llpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "RCC APB1 sleep clock register"]
    #[inline(always)]
    pub const fn apb1hlpenr(self) -> crate::common::Reg<regs::Apb1hlpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[doc = "RCC APB2 sleep clock register"]
    #[inline(always)]
    pub const fn apb2lpenr(self) -> crate::common::Reg<regs::Apb2lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[doc = "RCC APB3 sleep clock register"]
    #[inline(always)]
    pub const fn apb3lpenr(self) -> crate::common::Reg<regs::Apb3lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[doc = "RCC kernel clock configuration register"]
    #[inline(always)]
    pub const fn ccipr1(self) -> crate::common::Reg<regs::Ccipr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[doc = "RCC kernel clock configuration register"]
    #[inline(always)]
    pub const fn ccipr2(self) -> crate::common::Reg<regs::Ccipr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[doc = "RCC kernel clock configuration register"]
    #[inline(always)]
    pub const fn ccipr3(self) -> crate::common::Reg<regs::Ccipr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[doc = "RCC kernel clock configuration register"]
    #[inline(always)]
    pub const fn ccipr4(self) -> crate::common::Reg<regs::Ccipr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
    }
    #[doc = "RCC kernel clock configuration register"]
    #[inline(always)]
    pub const fn ccipr5(self) -> crate::common::Reg<regs::Ccipr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe8usize) as _) }
    }
    #[doc = "RCC Backup domain control register"]
    #[inline(always)]
    pub const fn bdcr(self) -> crate::common::Reg<regs::Bdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[doc = "RCC reset status register"]
    #[inline(always)]
    pub const fn rsr(self) -> crate::common::Reg<regs::Rsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
}
pub mod regs {
    #[doc = "RCC AHB1 peripherals clock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1enr(pub u32);
    impl Ahb1enr {
        #[doc = "GPDMA1 clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn gpdma1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1 clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_gpdma1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPDMA2 clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn gpdma2en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA2 clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_gpdma2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Flash interface clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn flitfen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Flash interface clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_flitfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CRC clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn crcen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_crcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "RAMCFG clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn ramcfgen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "RAMCFG clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_ramcfgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "BKPRAM clock enable Set and reset by software"]
        #[inline(always)]
        pub const fn bkpramen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "BKPRAM clock enable Set and reset by software"]
        #[inline(always)]
        pub fn set_bkpramen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
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
                .field("gpdma2en", &self.gpdma2en())
                .field("flitfen", &self.flitfen())
                .field("crcen", &self.crcen())
                .field("ramcfgen", &self.ramcfgen())
                .field("bkpramen", &self.bkpramen())
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
                gpdma2en: bool,
                flitfen: bool,
                crcen: bool,
                ramcfgen: bool,
                bkpramen: bool,
                sram1en: bool,
            }
            let proxy = Ahb1enr {
                gpdma1en: self.gpdma1en(),
                gpdma2en: self.gpdma2en(),
                flitfen: self.flitfen(),
                crcen: self.crcen(),
                ramcfgen: self.ramcfgen(),
                bkpramen: self.bkpramen(),
                sram1en: self.sram1en(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC AHB1 sleep clock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1lpenr(pub u32);
    impl Ahb1lpenr {
        #[doc = "GPDMA1 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn gpdma1lpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_gpdma1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPDMA2 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn gpdma2lpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA2 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_gpdma2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Flash interface (FLITF) clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn flitflpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Flash interface (FLITF) clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_flitflpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CRC clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn crclpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_crclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "RAMCFG clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn ramcfglpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "RAMCFG clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_ramcfglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "BKPRAM clock enable during sleep mode Set and reset by software"]
        #[inline(always)]
        pub const fn bkpramlpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "BKPRAM clock enable during sleep mode Set and reset by software"]
        #[inline(always)]
        pub fn set_bkpramlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "ICACHE clock enable during sleep mode Set and reset by software"]
        #[inline(always)]
        pub const fn icachelpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "ICACHE clock enable during sleep mode Set and reset by software"]
        #[inline(always)]
        pub fn set_icachelpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "SRAM1 clock enable during sleep mode Set and reset by software"]
        #[inline(always)]
        pub const fn sram1lpen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 clock enable during sleep mode Set and reset by software"]
        #[inline(always)]
        pub fn set_sram1lpen(&mut self, val: bool) {
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
                .field("flitflpen", &self.flitflpen())
                .field("crclpen", &self.crclpen())
                .field("ramcfglpen", &self.ramcfglpen())
                .field("bkpramlpen", &self.bkpramlpen())
                .field("icachelpen", &self.icachelpen())
                .field("sram1lpen", &self.sram1lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1lpenr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb1lpenr {
                gpdma1lpen: bool,
                gpdma2lpen: bool,
                flitflpen: bool,
                crclpen: bool,
                ramcfglpen: bool,
                bkpramlpen: bool,
                icachelpen: bool,
                sram1lpen: bool,
            }
            let proxy = Ahb1lpenr {
                gpdma1lpen: self.gpdma1lpen(),
                gpdma2lpen: self.gpdma2lpen(),
                flitflpen: self.flitflpen(),
                crclpen: self.crclpen(),
                ramcfglpen: self.ramcfglpen(),
                bkpramlpen: self.bkpramlpen(),
                icachelpen: self.icachelpen(),
                sram1lpen: self.sram1lpen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC AHB1 reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1rstr(pub u32);
    impl Ahb1rstr {
        #[doc = "GPDMA1 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn gpdma1rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_gpdma1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPDMA2 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn gpdma2rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA2 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_gpdma2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CRC block reset Set and reset by software."]
        #[inline(always)]
        pub const fn crcrst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_crcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "RAMCFG block reset Set and reset by software."]
        #[inline(always)]
        pub const fn ramcfgrst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "RAMCFG block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_ramcfgrst(&mut self, val: bool) {
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
                .field("ramcfgrst", &self.ramcfgrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1rstr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb1rstr {
                gpdma1rst: bool,
                gpdma2rst: bool,
                crcrst: bool,
                ramcfgrst: bool,
            }
            let proxy = Ahb1rstr {
                gpdma1rst: self.gpdma1rst(),
                gpdma2rst: self.gpdma2rst(),
                crcrst: self.crcrst(),
                ramcfgrst: self.ramcfgrst(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC AHB2 peripheral clock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2enr(pub u32);
    impl Ahb2enr {
        #[doc = "GPIOA clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn gpioaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOA clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_gpioaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPIOB clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn gpioben(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOB clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_gpioben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "GPIOC clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn gpiocen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOC clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_gpiocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "GPIOD clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn gpioden(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOD clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_gpioden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "GPIOH clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn gpiohen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOH clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_gpiohen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ADC1 peripherals clock enabled Set and reset by software."]
        #[inline(always)]
        pub const fn adc1en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1 peripherals clock enabled Set and reset by software."]
        #[inline(always)]
        pub fn set_adc1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "DAC clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn dac1en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "DAC clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_dac1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "HASH clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn hashen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "HASH clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_hashen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "RNG clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn rngen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "RNG clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_rngen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
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
                .field("gpioaen", &self.gpioaen())
                .field("gpioben", &self.gpioben())
                .field("gpiocen", &self.gpiocen())
                .field("gpioden", &self.gpioden())
                .field("gpiohen", &self.gpiohen())
                .field("adc1en", &self.adc1en())
                .field("dac1en", &self.dac1en())
                .field("hashen", &self.hashen())
                .field("rngen", &self.rngen())
                .field("sram2en", &self.sram2en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2enr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb2enr {
                gpioaen: bool,
                gpioben: bool,
                gpiocen: bool,
                gpioden: bool,
                gpiohen: bool,
                adc1en: bool,
                dac1en: bool,
                hashen: bool,
                rngen: bool,
                sram2en: bool,
            }
            let proxy = Ahb2enr {
                gpioaen: self.gpioaen(),
                gpioben: self.gpioben(),
                gpiocen: self.gpiocen(),
                gpioden: self.gpioden(),
                gpiohen: self.gpiohen(),
                adc1en: self.adc1en(),
                dac1en: self.dac1en(),
                hashen: self.hashen(),
                rngen: self.rngen(),
                sram2en: self.sram2en(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC AHB2 sleep clock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2lpenr(pub u32);
    impl Ahb2lpenr {
        #[doc = "GPIOA clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn gpioalpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOA clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_gpioalpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPIOB clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn gpioblpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOB clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_gpioblpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "GPIOC clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn gpioclpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOC clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_gpioclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "GPIOD clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn gpiodlpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOD clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_gpiodlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "GPIOH clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn gpiohlpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOH clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_gpiohlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ADC1 peripherals clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn adc1lpen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1 peripherals clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_adc1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "DAC clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn dac1lpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "DAC clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_dac1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "HASH clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn hashlpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "HASH clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_hashlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "RNG clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn rnglpen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "RNG clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_rnglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SRAM2 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn sram2lpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_sram2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
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
                .field("gpioalpen", &self.gpioalpen())
                .field("gpioblpen", &self.gpioblpen())
                .field("gpioclpen", &self.gpioclpen())
                .field("gpiodlpen", &self.gpiodlpen())
                .field("gpiohlpen", &self.gpiohlpen())
                .field("adc1lpen", &self.adc1lpen())
                .field("dac1lpen", &self.dac1lpen())
                .field("hashlpen", &self.hashlpen())
                .field("rnglpen", &self.rnglpen())
                .field("sram2lpen", &self.sram2lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2lpenr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb2lpenr {
                gpioalpen: bool,
                gpioblpen: bool,
                gpioclpen: bool,
                gpiodlpen: bool,
                gpiohlpen: bool,
                adc1lpen: bool,
                dac1lpen: bool,
                hashlpen: bool,
                rnglpen: bool,
                sram2lpen: bool,
            }
            let proxy = Ahb2lpenr {
                gpioalpen: self.gpioalpen(),
                gpioblpen: self.gpioblpen(),
                gpioclpen: self.gpioclpen(),
                gpiodlpen: self.gpiodlpen(),
                gpiohlpen: self.gpiohlpen(),
                adc1lpen: self.adc1lpen(),
                dac1lpen: self.dac1lpen(),
                hashlpen: self.hashlpen(),
                rnglpen: self.rnglpen(),
                sram2lpen: self.sram2lpen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC AHB2 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2rstr(pub u32);
    impl Ahb2rstr {
        #[doc = "GPIOA block reset Set and reset by software."]
        #[inline(always)]
        pub const fn gpioarst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOA block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_gpioarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPIOB block reset Set and reset by software."]
        #[inline(always)]
        pub const fn gpiobrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOB block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_gpiobrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "GPIOC block reset Set and reset by software."]
        #[inline(always)]
        pub const fn gpiocrst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOC block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_gpiocrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "GPIOD block reset Set and reset by software."]
        #[inline(always)]
        pub const fn gpiodrst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOD block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_gpiodrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "GPIOH block reset Set and reset by software."]
        #[inline(always)]
        pub const fn gpiohrst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOH block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_gpiohrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ADC1 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn adc1rst(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_adc1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "DAC block reset Set and reset by software."]
        #[inline(always)]
        pub const fn dac1rst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "DAC block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_dac1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "HASH block reset Set and reset by software."]
        #[inline(always)]
        pub const fn hashrst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "HASH block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_hashrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "RNG block reset Set and reset by software."]
        #[inline(always)]
        pub const fn rngrst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "RNG block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_rngrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
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
                .field("gpioarst", &self.gpioarst())
                .field("gpiobrst", &self.gpiobrst())
                .field("gpiocrst", &self.gpiocrst())
                .field("gpiodrst", &self.gpiodrst())
                .field("gpiohrst", &self.gpiohrst())
                .field("adc1rst", &self.adc1rst())
                .field("dac1rst", &self.dac1rst())
                .field("hashrst", &self.hashrst())
                .field("rngrst", &self.rngrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2rstr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahb2rstr {
                gpioarst: bool,
                gpiobrst: bool,
                gpiocrst: bool,
                gpiodrst: bool,
                gpiohrst: bool,
                adc1rst: bool,
                dac1rst: bool,
                hashrst: bool,
                rngrst: bool,
            }
            let proxy = Ahb2rstr {
                gpioarst: self.gpioarst(),
                gpiobrst: self.gpiobrst(),
                gpiocrst: self.gpiocrst(),
                gpiodrst: self.gpiodrst(),
                gpiohrst: self.gpiohrst(),
                adc1rst: self.adc1rst(),
                dac1rst: self.dac1rst(),
                hashrst: self.hashrst(),
                rngrst: self.rngrst(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB1 peripheral clock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1henr(pub u32);
    impl Apb1henr {
        #[doc = "DTS clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn dtsen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DTS clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_dtsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "LPTIM2 clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn lptim2en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_lptim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FDCAN1 peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn fdcan12en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN1 peripheral clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_fdcan12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
                .field("lptim2en", &self.lptim2en())
                .field("fdcan12en", &self.fdcan12en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1henr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb1henr {
                dtsen: bool,
                lptim2en: bool,
                fdcan12en: bool,
            }
            let proxy = Apb1henr {
                dtsen: self.dtsen(),
                lptim2en: self.lptim2en(),
                fdcan12en: self.fdcan12en(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB1 sleep clock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1hlpenr(pub u32);
    impl Apb1hlpenr {
        #[doc = "DTS clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn dtslpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DTS clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_dtslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "LPTIM2 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn lptim2lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_lptim2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FDCAN1 peripheral clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn fdcan12lpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN1 peripheral clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_fdcan12lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
                .field("lptim2lpen", &self.lptim2lpen())
                .field("fdcan12lpen", &self.fdcan12lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1hlpenr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb1hlpenr {
                dtslpen: bool,
                lptim2lpen: bool,
                fdcan12lpen: bool,
            }
            let proxy = Apb1hlpenr {
                dtslpen: self.dtslpen(),
                lptim2lpen: self.lptim2lpen(),
                fdcan12lpen: self.fdcan12lpen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB1 peripheral high reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1hrstr(pub u32);
    impl Apb1hrstr {
        #[doc = "DTS block reset Set and reset by software."]
        #[inline(always)]
        pub const fn dtsrst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DTS block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_dtsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "LPTIM2 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn lptim2rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_lptim2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "FDCAN1 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn fdcan12rst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN1 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_fdcan12rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
                .field("lptim2rst", &self.lptim2rst())
                .field("fdcan12rst", &self.fdcan12rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1hrstr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb1hrstr {
                dtsrst: bool,
                lptim2rst: bool,
                fdcan12rst: bool,
            }
            let proxy = Apb1hrstr {
                dtsrst: self.dtsrst(),
                lptim2rst: self.lptim2rst(),
                fdcan12rst: self.fdcan12rst(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB1 peripheral clock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1lenr(pub u32);
    impl Apb1lenr {
        #[doc = "TIM2 clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn tim2en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_tim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn tim3en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_tim3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM6 clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn tim6en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_tim6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn tim7en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_tim7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "WWDG clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn wwdgen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_wwdgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "OPAMP clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn opampen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_opampen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "SPI2 clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn spi2en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_spi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn spi3en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_spi3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "COMP clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn compen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "COMP clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_compen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "USART2 clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn usart2en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_usart2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn usart3en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_usart3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "I2C1 clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn i2c1en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_i2c1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn i2c2en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_i2c2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I3C1 clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn i3c1en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I3C1 clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_i3c1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "CRS clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn crsen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "CRS clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_crsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
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
                .field("wwdgen", &self.wwdgen())
                .field("opampen", &self.opampen())
                .field("spi2en", &self.spi2en())
                .field("spi3en", &self.spi3en())
                .field("compen", &self.compen())
                .field("usart2en", &self.usart2en())
                .field("usart3en", &self.usart3en())
                .field("i2c1en", &self.i2c1en())
                .field("i2c2en", &self.i2c2en())
                .field("i3c1en", &self.i3c1en())
                .field("crsen", &self.crsen())
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
                tim6en: bool,
                tim7en: bool,
                wwdgen: bool,
                opampen: bool,
                spi2en: bool,
                spi3en: bool,
                compen: bool,
                usart2en: bool,
                usart3en: bool,
                i2c1en: bool,
                i2c2en: bool,
                i3c1en: bool,
                crsen: bool,
            }
            let proxy = Apb1lenr {
                tim2en: self.tim2en(),
                tim3en: self.tim3en(),
                tim6en: self.tim6en(),
                tim7en: self.tim7en(),
                wwdgen: self.wwdgen(),
                opampen: self.opampen(),
                spi2en: self.spi2en(),
                spi3en: self.spi3en(),
                compen: self.compen(),
                usart2en: self.usart2en(),
                usart3en: self.usart3en(),
                i2c1en: self.i2c1en(),
                i2c2en: self.i2c2en(),
                i3c1en: self.i3c1en(),
                crsen: self.crsen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB1 sleep clock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1llpenr(pub u32);
    impl Apb1llpenr {
        #[doc = "TIM2 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn tim2lpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_tim2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn tim3lpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_tim3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM6 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn tim6lpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_tim6lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn tim7lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_tim7lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "WWDG clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn wwdglpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_wwdglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "OPAMP clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn opamplpen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_opamplpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "SPI2 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn spi2lpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_spi2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn spi3lpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_spi3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "COMP clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn complpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "COMP clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_complpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "USART2 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn usart2lpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_usart2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn usart3lpen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_usart3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "I2C1 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn i2c1lpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_i2c1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn i2c2lpen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_i2c2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I3C1 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn i3c1lpen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I3C1 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_i3c1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "CRS clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn crslpen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "CRS clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_crslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
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
                .field("wwdglpen", &self.wwdglpen())
                .field("opamplpen", &self.opamplpen())
                .field("spi2lpen", &self.spi2lpen())
                .field("spi3lpen", &self.spi3lpen())
                .field("complpen", &self.complpen())
                .field("usart2lpen", &self.usart2lpen())
                .field("usart3lpen", &self.usart3lpen())
                .field("i2c1lpen", &self.i2c1lpen())
                .field("i2c2lpen", &self.i2c2lpen())
                .field("i3c1lpen", &self.i3c1lpen())
                .field("crslpen", &self.crslpen())
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
                tim6lpen: bool,
                tim7lpen: bool,
                wwdglpen: bool,
                opamplpen: bool,
                spi2lpen: bool,
                spi3lpen: bool,
                complpen: bool,
                usart2lpen: bool,
                usart3lpen: bool,
                i2c1lpen: bool,
                i2c2lpen: bool,
                i3c1lpen: bool,
                crslpen: bool,
            }
            let proxy = Apb1llpenr {
                tim2lpen: self.tim2lpen(),
                tim3lpen: self.tim3lpen(),
                tim6lpen: self.tim6lpen(),
                tim7lpen: self.tim7lpen(),
                wwdglpen: self.wwdglpen(),
                opamplpen: self.opamplpen(),
                spi2lpen: self.spi2lpen(),
                spi3lpen: self.spi3lpen(),
                complpen: self.complpen(),
                usart2lpen: self.usart2lpen(),
                usart3lpen: self.usart3lpen(),
                i2c1lpen: self.i2c1lpen(),
                i2c2lpen: self.i2c2lpen(),
                i3c1lpen: self.i3c1lpen(),
                crslpen: self.crslpen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB1 peripheral low reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1lrstr(pub u32);
    impl Apb1lrstr {
        #[doc = "TIM2 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn tim2rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_tim2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn tim3rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_tim3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM6 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn tim6rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_tim6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn tim7rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_tim7rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "OPAMP block reset Set and reset by software."]
        #[inline(always)]
        pub const fn opamprst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_opamprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "SPI2 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn spi2rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_spi2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn spi3rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_spi3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "COMP block reset Set and reset by software."]
        #[inline(always)]
        pub const fn comprst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "COMP block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_comprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "USART2 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn usart2rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_usart2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn usart3rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_usart3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "I2C1 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn i2c1rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_i2c1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn i2c2rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_i2c2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I3C1 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn i3c1rst(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I3C1 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_i3c1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "CRS block reset Set and reset by software."]
        #[inline(always)]
        pub const fn crsrst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "CRS block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_crsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
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
                .field("opamprst", &self.opamprst())
                .field("spi2rst", &self.spi2rst())
                .field("spi3rst", &self.spi3rst())
                .field("comprst", &self.comprst())
                .field("usart2rst", &self.usart2rst())
                .field("usart3rst", &self.usart3rst())
                .field("i2c1rst", &self.i2c1rst())
                .field("i2c2rst", &self.i2c2rst())
                .field("i3c1rst", &self.i3c1rst())
                .field("crsrst", &self.crsrst())
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
                tim6rst: bool,
                tim7rst: bool,
                opamprst: bool,
                spi2rst: bool,
                spi3rst: bool,
                comprst: bool,
                usart2rst: bool,
                usart3rst: bool,
                i2c1rst: bool,
                i2c2rst: bool,
                i3c1rst: bool,
                crsrst: bool,
            }
            let proxy = Apb1lrstr {
                tim2rst: self.tim2rst(),
                tim3rst: self.tim3rst(),
                tim6rst: self.tim6rst(),
                tim7rst: self.tim7rst(),
                opamprst: self.opamprst(),
                spi2rst: self.spi2rst(),
                spi3rst: self.spi3rst(),
                comprst: self.comprst(),
                usart2rst: self.usart2rst(),
                usart3rst: self.usart3rst(),
                i2c1rst: self.i2c1rst(),
                i2c2rst: self.i2c2rst(),
                i3c1rst: self.i3c1rst(),
                crsrst: self.crsrst(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB2 peripheral clock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2enr(pub u32);
    impl Apb2enr {
        #[doc = "TIM1 clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn tim1en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_tim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI1 clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn spi1en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_spi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "USART1 clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn usart1en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_usart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "USB clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn usben(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "USB clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_usben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
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
                .field("usart1en", &self.usart1en())
                .field("usben", &self.usben())
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
                usart1en: bool,
                usben: bool,
            }
            let proxy = Apb2enr {
                tim1en: self.tim1en(),
                spi1en: self.spi1en(),
                usart1en: self.usart1en(),
                usben: self.usben(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB2 sleep clock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2lpenr(pub u32);
    impl Apb2lpenr {
        #[doc = "TIM1 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn tim1lpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_tim1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI1 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn spi1lpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_spi1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "USART1 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn usart1lpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_usart1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "USB clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn usblpen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "USB clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_usblpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
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
                .field("spi1lpen", &self.spi1lpen())
                .field("usart1lpen", &self.usart1lpen())
                .field("usblpen", &self.usblpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2lpenr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb2lpenr {
                tim1lpen: bool,
                spi1lpen: bool,
                usart1lpen: bool,
                usblpen: bool,
            }
            let proxy = Apb2lpenr {
                tim1lpen: self.tim1lpen(),
                spi1lpen: self.spi1lpen(),
                usart1lpen: self.usart1lpen(),
                usblpen: self.usblpen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB2 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2rstr(pub u32);
    impl Apb2rstr {
        #[doc = "TIM1 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn tim1rst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_tim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI1 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn spi1rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_spi1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "USART1 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn usart1rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_usart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "USB block reset Set and reset by software."]
        #[inline(always)]
        pub const fn usbrst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "USB block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_usbrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
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
                .field("usart1rst", &self.usart1rst())
                .field("usbrst", &self.usbrst())
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
                usart1rst: bool,
                usbrst: bool,
            }
            let proxy = Apb2rstr {
                tim1rst: self.tim1rst(),
                spi1rst: self.spi1rst(),
                usart1rst: self.usart1rst(),
                usbrst: self.usbrst(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB3 peripheral clock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb3enr(pub u32);
    impl Apb3enr {
        #[doc = "SBS clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn syscfgen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SBS clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_syscfgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPUART1 clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn lpuart1en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_lpuart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I3C2EN clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn i3c2en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "I3C2EN clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_i3c2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LPTIM1 clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn lptim1en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_lptim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "VREF clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn vrefen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "VREF clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_vrefen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "RTC APB interface clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn rtcapben(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "RTC APB interface clock enable Set and reset by software."]
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
                .field("lpuart1en", &self.lpuart1en())
                .field("i3c2en", &self.i3c2en())
                .field("lptim1en", &self.lptim1en())
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
                lpuart1en: bool,
                i3c2en: bool,
                lptim1en: bool,
                vrefen: bool,
                rtcapben: bool,
            }
            let proxy = Apb3enr {
                syscfgen: self.syscfgen(),
                lpuart1en: self.lpuart1en(),
                i3c2en: self.i3c2en(),
                lptim1en: self.lptim1en(),
                vrefen: self.vrefen(),
                rtcapben: self.rtcapben(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB3 sleep clock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb3lpenr(pub u32);
    impl Apb3lpenr {
        #[doc = "SBS clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn syscfglpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SBS clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_syscfglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPUART1 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn lpuart1lpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_lpuart1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I3C2 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn i3c2lpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "I3C2 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_i3c2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LPTIM1 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn lptim1lpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_lptim1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "VREF clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn vreflpen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "VREF clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_vreflpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "RTC APB interface clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub const fn rtcapblpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "RTC APB interface clock enable during sleep mode Set and reset by software."]
        #[inline(always)]
        pub fn set_rtcapblpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
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
                .field("syscfglpen", &self.syscfglpen())
                .field("lpuart1lpen", &self.lpuart1lpen())
                .field("i3c2lpen", &self.i3c2lpen())
                .field("lptim1lpen", &self.lptim1lpen())
                .field("vreflpen", &self.vreflpen())
                .field("rtcapblpen", &self.rtcapblpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb3lpenr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb3lpenr {
                syscfglpen: bool,
                lpuart1lpen: bool,
                i3c2lpen: bool,
                lptim1lpen: bool,
                vreflpen: bool,
                rtcapblpen: bool,
            }
            let proxy = Apb3lpenr {
                syscfglpen: self.syscfglpen(),
                lpuart1lpen: self.lpuart1lpen(),
                i3c2lpen: self.i3c2lpen(),
                lptim1lpen: self.lptim1lpen(),
                vreflpen: self.vreflpen(),
                rtcapblpen: self.rtcapblpen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC APB3 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb3rstr(pub u32);
    impl Apb3rstr {
        #[doc = "SBS block reset Set and reset by software."]
        #[inline(always)]
        pub const fn syscfgrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SBS block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_syscfgrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPUART1 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn lpuart1rst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_lpuart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I3C2RST block reset Set and reset by software."]
        #[inline(always)]
        pub const fn i3c2rst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "I3C2RST block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_i3c2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "LPTIM1 block reset Set and reset by software."]
        #[inline(always)]
        pub const fn lptim1rst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 block reset Set and reset by software."]
        #[inline(always)]
        pub fn set_lptim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "VREF block reset Set and reset by software."]
        #[inline(always)]
        pub const fn vrefrst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "VREF block reset Set and reset by software."]
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
                .field("lpuart1rst", &self.lpuart1rst())
                .field("i3c2rst", &self.i3c2rst())
                .field("lptim1rst", &self.lptim1rst())
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
                lpuart1rst: bool,
                i3c2rst: bool,
                lptim1rst: bool,
                vrefrst: bool,
            }
            let proxy = Apb3rstr {
                syscfgrst: self.syscfgrst(),
                lpuart1rst: self.lpuart1rst(),
                i3c2rst: self.i3c2rst(),
                lptim1rst: self.lptim1rst(),
                vrefrst: self.vrefrst(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC Backup domain control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdcr(pub u32);
    impl Bdcr {
        #[doc = "LSE oscillator enabled Set and reset by software."]
        #[inline(always)]
        pub const fn lseon(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSE oscillator enabled Set and reset by software."]
        #[inline(always)]
        pub fn set_lseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE oscillator ready Set and reset by hardware to indicate when the LSE is stable. This bit needs 6 cycles of lse_ck clock to fall down after LSEON has been set to 0."]
        #[inline(always)]
        pub const fn lserdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE oscillator ready Set and reset by hardware to indicate when the LSE is stable. This bit needs 6 cycles of lse_ck clock to fall down after LSEON has been set to 0."]
        #[inline(always)]
        pub fn set_lserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LSE oscillator bypass Set and reset by software to bypass oscillator in debug mode. This bit must not be written when the LSE is enabled (by LSEON) or ready (LSERDY = 1)"]
        #[inline(always)]
        pub const fn lsebyp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "LSE oscillator bypass Set and reset by software to bypass oscillator in debug mode. This bit must not be written when the LSE is enabled (by LSEON) or ready (LSERDY = 1)"]
        #[inline(always)]
        pub fn set_lsebyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "LSE oscillator driving capability Set by software to select the driving capability of the LSE oscillator. These bit can be written only if LSE oscillator is disabled (LSEON = 0 and LSERDY = 0)."]
        #[inline(always)]
        pub const fn lsedrv(&self) -> super::vals::Lsedrv {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Lsedrv::from_bits(val as u8)
        }
        #[doc = "LSE oscillator driving capability Set by software to select the driving capability of the LSE oscillator. These bit can be written only if LSE oscillator is disabled (LSEON = 0 and LSERDY = 0)."]
        #[inline(always)]
        pub fn set_lsedrv(&mut self, val: super::vals::Lsedrv) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "LSE clock security system enable Set by software to enable the clock security system on 32 kHz oscillator. LSECSSON must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware) and after RTCSEL is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD = 1). In that case the software must disable LSECSSON."]
        #[inline(always)]
        pub const fn lsecsson(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock security system enable Set by software to enable the clock security system on 32 kHz oscillator. LSECSSON must be enabled after LSE is enabled (LSEON enabled) and ready (LSERDY set by hardware) and after RTCSEL is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD = 1). In that case the software must disable LSECSSON."]
        #[inline(always)]
        pub fn set_lsecsson(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LSE clock security system failure detection Set by hardware to indicate when a failure has been detected by the clock security system on the external 32 kHz oscillator."]
        #[inline(always)]
        pub const fn lsecssd(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock security system failure detection Set by hardware to indicate when a failure has been detected by the clock security system on the external 32 kHz oscillator."]
        #[inline(always)]
        pub fn set_lsecssd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "low-speed external clock type in bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the LSEON bit, to be used by the device. The LSEEXT bit can be written only if the LSE oscillator is disabled."]
        #[inline(always)]
        pub const fn lseext(&self) -> super::vals::Lseext {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Lseext::from_bits(val as u8)
        }
        #[doc = "low-speed external clock type in bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the LSEON bit, to be used by the device. The LSEEXT bit can be written only if the LSE oscillator is disabled."]
        #[inline(always)]
        pub fn set_lseext(&mut self, val: super::vals::Lseext) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "RTC clock source selection Set by software to select the clock source for the RTC. These bits can be written only one time (except in case of failure detection on LSE). These bits must be written before LSECSSON is enabled. The VSWRST bit can be used to reset them, then it can be written one time again. If HSE is selected as RTC clock, this clock is lost when the system is in Stop mode or in case of a pin reset (NRST)."]
        #[inline(always)]
        pub const fn rtcsel(&self) -> super::vals::Rtcsel {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Rtcsel::from_bits(val as u8)
        }
        #[doc = "RTC clock source selection Set by software to select the clock source for the RTC. These bits can be written only one time (except in case of failure detection on LSE). These bits must be written before LSECSSON is enabled. The VSWRST bit can be used to reset them, then it can be written one time again. If HSE is selected as RTC clock, this clock is lost when the system is in Stop mode or in case of a pin reset (NRST)."]
        #[inline(always)]
        pub fn set_rtcsel(&mut self, val: super::vals::Rtcsel) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "RTC clock enable Set and reset by software."]
        #[inline(always)]
        pub const fn rtcen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "RTC clock enable Set and reset by software."]
        #[inline(always)]
        pub fn set_rtcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "VSwitch domain software reset Set and reset by software."]
        #[inline(always)]
        pub const fn vswrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "VSwitch domain software reset Set and reset by software."]
        #[inline(always)]
        pub fn set_vswrst(&mut self, val: bool) {
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
                .field("rtcen", &self.rtcen())
                .field("vswrst", &self.vswrst())
                .field("lscoen", &self.lscoen())
                .field("lscosel", &self.lscosel())
                .field("lsion", &self.lsion())
                .field("lsirdy", &self.lsirdy())
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
                lseext: super::vals::Lseext,
                rtcsel: super::vals::Rtcsel,
                rtcen: bool,
                vswrst: bool,
                lscoen: bool,
                lscosel: super::vals::Lscosel,
                lsion: bool,
                lsirdy: bool,
            }
            let proxy = Bdcr {
                lseon: self.lseon(),
                lserdy: self.lserdy(),
                lsebyp: self.lsebyp(),
                lsedrv: self.lsedrv(),
                lsecsson: self.lsecsson(),
                lsecssd: self.lsecssd(),
                lseext: self.lseext(),
                rtcsel: self.rtcsel(),
                rtcen: self.rtcen(),
                vswrst: self.vswrst(),
                lscoen: self.lscoen(),
                lscosel: self.lscosel(),
                lsion: self.lsion(),
                lsirdy: self.lsirdy(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC kernel clock configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccipr1(pub u32);
    impl Ccipr1 {
        #[doc = "USART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
        #[inline(always)]
        pub const fn usart1sel(&self) -> super::vals::Usart1sel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Usart1sel::from_bits(val as u8)
        }
        #[doc = "USART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
        #[inline(always)]
        pub fn set_usart1sel(&mut self, val: super::vals::Usart1sel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "USART2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
        #[inline(always)]
        pub const fn usart2sel(&self) -> super::vals::Usartsel {
            let val = (self.0 >> 3usize) & 0x07;
            super::vals::Usartsel::from_bits(val as u8)
        }
        #[doc = "USART2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
        #[inline(always)]
        pub fn set_usart2sel(&mut self, val: super::vals::Usartsel) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
        }
        #[doc = "USART3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
        #[inline(always)]
        pub const fn usart3sel(&self) -> super::vals::Usartsel {
            let val = (self.0 >> 6usize) & 0x07;
            super::vals::Usartsel::from_bits(val as u8)
        }
        #[doc = "USART3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
        #[inline(always)]
        pub fn set_usart3sel(&mut self, val: super::vals::Usartsel) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val.to_bits() as u32) & 0x07) << 6usize);
        }
        #[doc = "TIM2, TIM3 and LPTIM2 input capture source selection Set and reset by software."]
        #[inline(always)]
        pub const fn timicsel(&self) -> super::vals::Timicsel {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Timicsel::from_bits(val as u8)
        }
        #[doc = "TIM2, TIM3 and LPTIM2 input capture source selection Set and reset by software."]
        #[inline(always)]
        pub fn set_timicsel(&mut self, val: super::vals::Timicsel) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
                timicsel: super::vals::Timicsel,
            }
            let proxy = Ccipr1 {
                usart1sel: self.usart1sel(),
                usart2sel: self.usart2sel(),
                usart3sel: self.usart3sel(),
                timicsel: self.timicsel(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC kernel clock configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccipr2(pub u32);
    impl Ccipr2 {
        #[doc = "LPTIM1 kernel clock source selection others: reserved, the kernel clock is disabled"]
        #[inline(always)]
        pub const fn lptim1sel(&self) -> super::vals::Lptim1sel {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Lptim1sel::from_bits(val as u8)
        }
        #[doc = "LPTIM1 kernel clock source selection others: reserved, the kernel clock is disabled"]
        #[inline(always)]
        pub fn set_lptim1sel(&mut self, val: super::vals::Lptim1sel) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "LPTIM2 kernel clock source selection others: reserved, the kernel clock is disabled"]
        #[inline(always)]
        pub const fn lptim2sel(&self) -> super::vals::Lptim2sel {
            let val = (self.0 >> 12usize) & 0x07;
            super::vals::Lptim2sel::from_bits(val as u8)
        }
        #[doc = "LPTIM2 kernel clock source selection others: reserved, the kernel clock is disabled"]
        #[inline(always)]
        pub fn set_lptim2sel(&mut self, val: super::vals::Lptim2sel) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
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
                .field("lptim1sel", &self.lptim1sel())
                .field("lptim2sel", &self.lptim2sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccipr2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ccipr2 {
                lptim1sel: super::vals::Lptim1sel,
                lptim2sel: super::vals::Lptim2sel,
            }
            let proxy = Ccipr2 {
                lptim1sel: self.lptim1sel(),
                lptim2sel: self.lptim2sel(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC kernel clock configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccipr3(pub u32);
    impl Ccipr3 {
        #[doc = "SPI1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
        #[inline(always)]
        pub const fn spi1sel(&self) -> super::vals::Spisel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Spisel::from_bits(val as u8)
        }
        #[doc = "SPI1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
        #[inline(always)]
        pub fn set_spi1sel(&mut self, val: super::vals::Spisel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "SPI2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
        #[inline(always)]
        pub const fn spi2sel(&self) -> super::vals::Spisel {
            let val = (self.0 >> 3usize) & 0x07;
            super::vals::Spisel::from_bits(val as u8)
        }
        #[doc = "SPI2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
        #[inline(always)]
        pub fn set_spi2sel(&mut self, val: super::vals::Spisel) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
        }
        #[doc = "SPI3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
        #[inline(always)]
        pub const fn spi3sel(&self) -> super::vals::Spisel {
            let val = (self.0 >> 6usize) & 0x07;
            super::vals::Spisel::from_bits(val as u8)
        }
        #[doc = "SPI3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
        #[inline(always)]
        pub fn set_spi3sel(&mut self, val: super::vals::Spisel) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val.to_bits() as u32) & 0x07) << 6usize);
        }
        #[doc = "LPUART1 kernel clock source selection others: reserved, the kernel clock is disabled"]
        #[inline(always)]
        pub const fn lpuart1sel(&self) -> super::vals::Lpuartsel {
            let val = (self.0 >> 24usize) & 0x07;
            super::vals::Lpuartsel::from_bits(val as u8)
        }
        #[doc = "LPUART1 kernel clock source selection others: reserved, the kernel clock is disabled"]
        #[inline(always)]
        pub fn set_lpuart1sel(&mut self, val: super::vals::Lpuartsel) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
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
                .field("lpuart1sel", &self.lpuart1sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccipr3 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ccipr3 {
                spi1sel: super::vals::Spisel,
                spi2sel: super::vals::Spisel,
                spi3sel: super::vals::Spisel,
                lpuart1sel: super::vals::Lpuartsel,
            }
            let proxy = Ccipr3 {
                spi1sel: self.spi1sel(),
                spi2sel: self.spi2sel(),
                spi3sel: self.spi3sel(),
                lpuart1sel: self.lpuart1sel(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC kernel clock configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccipr4(pub u32);
    impl Ccipr4 {
        #[doc = "SYSTICK clock source selection Note: rcc_hclk frequency must be four times higher than lsi_ker_ck/lse_ck (period (LSI/LSE) ≥ 4 * period (HCLK)."]
        #[inline(always)]
        pub const fn systicksel(&self) -> super::vals::Systicksel {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Systicksel::from_bits(val as u8)
        }
        #[doc = "SYSTICK clock source selection Note: rcc_hclk frequency must be four times higher than lsi_ker_ck/lse_ck (period (LSI/LSE) ≥ 4 * period (HCLK)."]
        #[inline(always)]
        pub fn set_systicksel(&mut self, val: super::vals::Systicksel) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "USB kernel clock source selection"]
        #[inline(always)]
        pub const fn usbsel(&self) -> super::vals::Usbsel {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Usbsel::from_bits(val as u8)
        }
        #[doc = "USB kernel clock source selection"]
        #[inline(always)]
        pub fn set_usbsel(&mut self, val: super::vals::Usbsel) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "I2C1 kernel clock source selection"]
        #[inline(always)]
        pub const fn i2c1sel(&self) -> super::vals::I2csel {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::I2csel::from_bits(val as u8)
        }
        #[doc = "I2C1 kernel clock source selection"]
        #[inline(always)]
        pub fn set_i2c1sel(&mut self, val: super::vals::I2csel) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "I2C2 kernel clock source selection"]
        #[inline(always)]
        pub const fn i2c2sel(&self) -> super::vals::I2csel {
            let val = (self.0 >> 18usize) & 0x03;
            super::vals::I2csel::from_bits(val as u8)
        }
        #[doc = "I2C2 kernel clock source selection"]
        #[inline(always)]
        pub fn set_i2c2sel(&mut self, val: super::vals::I2csel) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
        }
        #[doc = "I3C1 kernel clock source selection"]
        #[inline(always)]
        pub const fn i3c1sel(&self) -> super::vals::I2csel {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::I2csel::from_bits(val as u8)
        }
        #[doc = "I3C1 kernel clock source selection"]
        #[inline(always)]
        pub fn set_i3c1sel(&mut self, val: super::vals::I2csel) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "I3C2 kernel clock source selection"]
        #[inline(always)]
        pub const fn i3c2sel(&self) -> super::vals::I3c2sel {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::I3c2sel::from_bits(val as u8)
        }
        #[doc = "I3C2 kernel clock source selection"]
        #[inline(always)]
        pub fn set_i3c2sel(&mut self, val: super::vals::I3c2sel) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
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
                .field("systicksel", &self.systicksel())
                .field("usbsel", &self.usbsel())
                .field("i2c1sel", &self.i2c1sel())
                .field("i2c2sel", &self.i2c2sel())
                .field("i3c1sel", &self.i3c1sel())
                .field("i3c2sel", &self.i3c2sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccipr4 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ccipr4 {
                systicksel: super::vals::Systicksel,
                usbsel: super::vals::Usbsel,
                i2c1sel: super::vals::I2csel,
                i2c2sel: super::vals::I2csel,
                i3c1sel: super::vals::I2csel,
                i3c2sel: super::vals::I3c2sel,
            }
            let proxy = Ccipr4 {
                systicksel: self.systicksel(),
                usbsel: self.usbsel(),
                i2c1sel: self.i2c1sel(),
                i2c2sel: self.i2c2sel(),
                i3c1sel: self.i3c1sel(),
                i3c2sel: self.i3c2sel(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC kernel clock configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccipr5(pub u32);
    impl Ccipr5 {
        #[doc = "ADC and DAC kernel clock source selection others: reserved, the kernel clock is disabled"]
        #[inline(always)]
        pub const fn adcdacsel(&self) -> super::vals::Adcdacsel {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Adcdacsel::from_bits(val as u8)
        }
        #[doc = "ADC and DAC kernel clock source selection others: reserved, the kernel clock is disabled"]
        #[inline(always)]
        pub fn set_adcdacsel(&mut self, val: super::vals::Adcdacsel) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "DAC hold clock"]
        #[inline(always)]
        pub const fn dacholdsel(&self) -> super::vals::Dacholdsel {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Dacholdsel::from_bits(val as u8)
        }
        #[doc = "DAC hold clock"]
        #[inline(always)]
        pub fn set_dacholdsel(&mut self, val: super::vals::Dacholdsel) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "RNG kernel clock source selection"]
        #[inline(always)]
        pub const fn rngsel(&self) -> super::vals::Rngsel {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Rngsel::from_bits(val as u8)
        }
        #[doc = "RNG kernel clock source selection"]
        #[inline(always)]
        pub fn set_rngsel(&mut self, val: super::vals::Rngsel) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "FDCAN1 kernel clock source selection"]
        #[inline(always)]
        pub const fn fdcan12sel(&self) -> super::vals::Fdcansel {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Fdcansel::from_bits(val as u8)
        }
        #[doc = "FDCAN1 kernel clock source selection"]
        #[inline(always)]
        pub fn set_fdcan12sel(&mut self, val: super::vals::Fdcansel) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "per_ck clock source selection"]
        #[inline(always)]
        pub const fn persel(&self) -> super::vals::Persel {
            let val = (self.0 >> 30usize) & 0x03;
            super::vals::Persel::from_bits(val as u8)
        }
        #[doc = "per_ck clock source selection"]
        #[inline(always)]
        pub fn set_persel(&mut self, val: super::vals::Persel) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
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
                .field("adcdacsel", &self.adcdacsel())
                .field("dacholdsel", &self.dacholdsel())
                .field("rngsel", &self.rngsel())
                .field("fdcan12sel", &self.fdcan12sel())
                .field("persel", &self.persel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccipr5 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ccipr5 {
                adcdacsel: super::vals::Adcdacsel,
                dacholdsel: super::vals::Dacholdsel,
                rngsel: super::vals::Rngsel,
                fdcan12sel: super::vals::Fdcansel,
                persel: super::vals::Persel,
            }
            let proxy = Ccipr5 {
                adcdacsel: self.adcdacsel(),
                dacholdsel: self.dacholdsel(),
                rngsel: self.rngsel(),
                fdcan12sel: self.fdcan12sel(),
                persel: self.persel(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC clock configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "system clock and trace clock switch Set and reset by software to select system clock and trace clock sources (sys_ck). Set by hardware in order to: - force the selection of the HSI or CSI (depending on STOPWUCK selection) when leaving a system Stop mode - force the selection of the HSI in case of failure of the HSE when used directly or indirectly as system clock others: reserved"]
        #[inline(always)]
        pub const fn sw(&self) -> super::vals::Sw {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "system clock and trace clock switch Set and reset by software to select system clock and trace clock sources (sys_ck). Set by hardware in order to: - force the selection of the HSI or CSI (depending on STOPWUCK selection) when leaving a system Stop mode - force the selection of the HSI in case of failure of the HSE when used directly or indirectly as system clock others: reserved"]
        #[inline(always)]
        pub fn set_sw(&mut self, val: super::vals::Sw) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "system clock switch status Set and reset by hardware to indicate which clock source is used as system clock. 000: HSI used as system clock (hsi_ck) (default after reset). others: reserved"]
        #[inline(always)]
        pub const fn sws(&self) -> super::vals::Sw {
            let val = (self.0 >> 3usize) & 0x07;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "system clock switch status Set and reset by hardware to indicate which clock source is used as system clock. 000: HSI used as system clock (hsi_ck) (default after reset). others: reserved"]
        #[inline(always)]
        pub fn set_sws(&mut self, val: super::vals::Sw) {
            self.0 = (self.0 & !(0x07 << 3usize)) | (((val.to_bits() as u32) & 0x07) << 3usize);
        }
        #[doc = "system clock selection after a wakeup from system Stop Set and reset by software to select the system wakeup clock from system Stop. The selected clock is also used as emergency clock for the clock security system (CSS) on HSE. 0: HSI selected as wakeup clock from system Stop (default after reset) STOPWUCK must not be modified when CSS is enabled (by HSECSSON bit) and the system clock is HSE (SWS = 10) or a switch on HSE is requested (SW =10)."]
        #[inline(always)]
        pub const fn stopwuck(&self) -> super::vals::Stopwuck {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Stopwuck::from_bits(val as u8)
        }
        #[doc = "system clock selection after a wakeup from system Stop Set and reset by software to select the system wakeup clock from system Stop. The selected clock is also used as emergency clock for the clock security system (CSS) on HSE. 0: HSI selected as wakeup clock from system Stop (default after reset) STOPWUCK must not be modified when CSS is enabled (by HSECSSON bit) and the system clock is HSE (SWS = 10) or a switch on HSE is requested (SW =10)."]
        #[inline(always)]
        pub fn set_stopwuck(&mut self, val: super::vals::Stopwuck) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "kernel clock selection after a wakeup from system Stop Set and reset by software to select the kernel wakeup clock from system Stop."]
        #[inline(always)]
        pub const fn stopkerwuck(&self) -> super::vals::Stopkerwuck {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Stopkerwuck::from_bits(val as u8)
        }
        #[doc = "kernel clock selection after a wakeup from system Stop Set and reset by software to select the kernel wakeup clock from system Stop."]
        #[inline(always)]
        pub fn set_stopkerwuck(&mut self, val: super::vals::Stopkerwuck) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "HSE division factor for RTC clock Set and cleared by software to divide the HSE to generate a clock for RTC. Caution: The software must set these bits correctly to ensure that the clock supplied to the RTC is lower than 1 MHz. These bits must be configured if needed before selecting the RTC clock source. ..."]
        #[inline(always)]
        pub const fn rtcpre(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "HSE division factor for RTC clock Set and cleared by software to divide the HSE to generate a clock for RTC. Caution: The software must set these bits correctly to ensure that the clock supplied to the RTC is lower than 1 MHz. These bits must be configured if needed before selecting the RTC clock source. ..."]
        #[inline(always)]
        pub fn set_rtcpre(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "timers clocks prescaler selection This bit is set and reset by software to control the clock frequency of all the timers connected to APB1 and APB2 domains."]
        #[inline(always)]
        pub const fn timpre(&self) -> super::vals::Timpre {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Timpre::from_bits(val as u8)
        }
        #[doc = "timers clocks prescaler selection This bit is set and reset by software to control the clock frequency of all the timers connected to APB1 and APB2 domains."]
        #[inline(always)]
        pub fn set_timpre(&mut self, val: super::vals::Timpre) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "MCO1 prescaler Set and cleared by software to configure the prescaler of the MCO1. Modification of this prescaler may generate glitches on MCO1. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ..."]
        #[inline(always)]
        pub const fn mco1pre(&self) -> super::vals::Mcopre {
            let val = (self.0 >> 18usize) & 0x0f;
            super::vals::Mcopre::from_bits(val as u8)
        }
        #[doc = "MCO1 prescaler Set and cleared by software to configure the prescaler of the MCO1. Modification of this prescaler may generate glitches on MCO1. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ..."]
        #[inline(always)]
        pub fn set_mco1pre(&mut self, val: super::vals::Mcopre) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val.to_bits() as u32) & 0x0f) << 18usize);
        }
        #[doc = "Microcontroller clock output 1 Set and cleared by software. Clock source selection may generate glitches on MCO1. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved"]
        #[inline(always)]
        pub const fn mco1sel(&self) -> super::vals::Mco1sel {
            let val = (self.0 >> 22usize) & 0x07;
            super::vals::Mco1sel::from_bits(val as u8)
        }
        #[doc = "Microcontroller clock output 1 Set and cleared by software. Clock source selection may generate glitches on MCO1. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved"]
        #[inline(always)]
        pub fn set_mco1sel(&mut self, val: super::vals::Mco1sel) {
            self.0 = (self.0 & !(0x07 << 22usize)) | (((val.to_bits() as u32) & 0x07) << 22usize);
        }
        #[doc = "MCO2 prescaler Set and cleared by software to configure the prescaler of the MCO2. Modification of this prescaler may generate glitches on MCO2. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ..."]
        #[inline(always)]
        pub const fn mco2pre(&self) -> super::vals::Mcopre {
            let val = (self.0 >> 25usize) & 0x0f;
            super::vals::Mcopre::from_bits(val as u8)
        }
        #[doc = "MCO2 prescaler Set and cleared by software to configure the prescaler of the MCO2. Modification of this prescaler may generate glitches on MCO2. It is highly recommended to change this prescaler only after reset, before enabling the external oscillators and the PLLs. ..."]
        #[inline(always)]
        pub fn set_mco2pre(&mut self, val: super::vals::Mcopre) {
            self.0 = (self.0 & !(0x0f << 25usize)) | (((val.to_bits() as u32) & 0x0f) << 25usize);
        }
        #[doc = "microcontroller clock output 2 Set and cleared by software. Clock source selection may generate glitches on MCO2. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved"]
        #[inline(always)]
        pub const fn mco2sel(&self) -> super::vals::Mco2sel {
            let val = (self.0 >> 29usize) & 0x07;
            super::vals::Mco2sel::from_bits(val as u8)
        }
        #[doc = "microcontroller clock output 2 Set and cleared by software. Clock source selection may generate glitches on MCO2. It is highly recommended to configure these bits only after reset, before enabling the external oscillators and the PLLs. others: reserved"]
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
                stopkerwuck: super::vals::Stopkerwuck,
                rtcpre: u8,
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
                timpre: self.timpre(),
                mco1pre: self.mco1pre(),
                mco1sel: self.mco1sel(),
                mco2pre: self.mco2pre(),
                mco2sel: self.mco2sel(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC CPU domain clock configuration register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "AHB prescaler Set and reset by software to control the division factor of rcc_hclk. Changing this division ratio has an impact on the frequency of all bus matrix clocks 0xxx: rcc_hclk = sys_ck (default after reset)"]
        #[inline(always)]
        pub const fn hpre(&self) -> super::vals::Hpre {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Hpre::from_bits(val as u8)
        }
        #[doc = "AHB prescaler Set and reset by software to control the division factor of rcc_hclk. Changing this division ratio has an impact on the frequency of all bus matrix clocks 0xxx: rcc_hclk = sys_ck (default after reset)"]
        #[inline(always)]
        pub fn set_hpre(&mut self, val: super::vals::Hpre) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "APB low-speed prescaler (APB1) Set and reset by software to control the division factor of rcc_pclk1. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk after PPRE write. 0xx: rcc_pclk1 = rcc_hclk1 (default after reset)"]
        #[inline(always)]
        pub const fn ppre1(&self) -> super::vals::Ppre {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB low-speed prescaler (APB1) Set and reset by software to control the division factor of rcc_pclk1. The clock is divided by the new prescaler factor from 1 to 16 cycles of rcc_hclk after PPRE write. 0xx: rcc_pclk1 = rcc_hclk1 (default after reset)"]
        #[inline(always)]
        pub fn set_ppre1(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "APB high-speed prescaler (APB2) Set and reset by software to control APB high-speed clocks division factor. The clocks are divided with the new prescaler factor from 1 to 16 APB cycles after PPRE2 write. 0xx: rcc_pclk2 = rcc_hclk1"]
        #[inline(always)]
        pub const fn ppre2(&self) -> super::vals::Ppre {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB high-speed prescaler (APB2) Set and reset by software to control APB high-speed clocks division factor. The clocks are divided with the new prescaler factor from 1 to 16 APB cycles after PPRE2 write. 0xx: rcc_pclk2 = rcc_hclk1"]
        #[inline(always)]
        pub fn set_ppre2(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "APB low-speed prescaler (APB3) Set and reset by software to control APB low-speed clocks division factor. The clocks are divided with the new prescaler factor from 1 to 16 APB cycles after PPRE3 write. 0xx: rcc_pclk3 = rcc_hclk1"]
        #[inline(always)]
        pub const fn ppre3(&self) -> super::vals::Ppre {
            let val = (self.0 >> 12usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB low-speed prescaler (APB3) Set and reset by software to control APB low-speed clocks division factor. The clocks are divided with the new prescaler factor from 1 to 16 APB cycles after PPRE3 write. 0xx: rcc_pclk3 = rcc_hclk1"]
        #[inline(always)]
        pub fn set_ppre3(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
        }
        #[doc = "AHB1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB1 peripherals from RCC_AHB1ENR are used and when their clocks are disabled in RCC_AHB1ENR. When this bit is set, all the AHB1 peripherals clocks from RCC_AHB1ENR are off. enable control bits"]
        #[inline(always)]
        pub const fn ahb1dis(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "AHB1 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB1 peripherals from RCC_AHB1ENR are used and when their clocks are disabled in RCC_AHB1ENR. When this bit is set, all the AHB1 peripherals clocks from RCC_AHB1ENR are off. enable control bits"]
        #[inline(always)]
        pub fn set_ahb1dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "AHB2 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR are used and when their clocks are disabled in RCC_AHB2ENR. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR are off. enable control bits"]
        #[inline(always)]
        pub const fn ahb2dis(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "AHB2 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB2 peripherals from RCC_AHB2ENR are used and when their clocks are disabled in RCC_AHB2ENR. When this bit is set, all the AHB2 peripherals clocks from RCC_AHB2ENR are off. enable control bits"]
        #[inline(always)]
        pub fn set_ahb2dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "AHB4 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB4 peripherals from RCC_AHB4ENR are used and when their clocks are disabled in RCC_AHB4ENR. When this bit is set, all the AHB4 peripherals clocks from RCC_AHB4ENR are off. enable control bits"]
        #[inline(always)]
        pub const fn ahb4dis(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "AHB4 clock disable This bit can be set in order to further reduce power consumption, when none of the AHB4 peripherals from RCC_AHB4ENR are used and when their clocks are disabled in RCC_AHB4ENR. When this bit is set, all the AHB4 peripherals clocks from RCC_AHB4ENR are off. enable control bits"]
        #[inline(always)]
        pub fn set_ahb4dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "APB1 clock disable value This bit can be set in order to further reduce power consumption, when none of the APB1 peripherals (except IWDG) are used and when their clocks are disabled in RCC_APB1ENR. When this bit is set, all the APB1 peripherals clocks are off, except for IWDG. control bits"]
        #[inline(always)]
        pub const fn apb1dis(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "APB1 clock disable value This bit can be set in order to further reduce power consumption, when none of the APB1 peripherals (except IWDG) are used and when their clocks are disabled in RCC_APB1ENR. When this bit is set, all the APB1 peripherals clocks are off, except for IWDG. control bits"]
        #[inline(always)]
        pub fn set_apb1dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "APB2 clock disable value This bit can be set in order to further reduce power consumption, when none of the APB2 peripherals are used and when their clocks are disabled in RCC_APB2ENR. When this bit is set, all the APB2 peripherals clocks are off. control bits"]
        #[inline(always)]
        pub const fn apb2dis(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "APB2 clock disable value This bit can be set in order to further reduce power consumption, when none of the APB2 peripherals are used and when their clocks are disabled in RCC_APB2ENR. When this bit is set, all the APB2 peripherals clocks are off. control bits"]
        #[inline(always)]
        pub fn set_apb2dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "APB3 clock disable value.Set and cleared by software This bit can be set in order to further reduce power consumption, when none of the APB3 peripherals are used and when their clocks are disabled in RCC_APB3ENR. When this bit is set, all the APB3 peripherals clocks are off. control bits"]
        #[inline(always)]
        pub const fn apb3dis(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "APB3 clock disable value.Set and cleared by software This bit can be set in order to further reduce power consumption, when none of the APB3 peripherals are used and when their clocks are disabled in RCC_APB3ENR. When this bit is set, all the APB3 peripherals clocks are off. control bits"]
        #[inline(always)]
        pub fn set_apb3dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
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
            #[derive(defmt :: Format)]
            struct Cfgr2 {
                hpre: super::vals::Hpre,
                ppre1: super::vals::Ppre,
                ppre2: super::vals::Ppre,
                ppre3: super::vals::Ppre,
                ahb1dis: bool,
                ahb2dis: bool,
                ahb4dis: bool,
                apb1dis: bool,
                apb2dis: bool,
                apb3dis: bool,
            }
            let proxy = Cfgr2 {
                hpre: self.hpre(),
                ppre1: self.ppre1(),
                ppre2: self.ppre2(),
                ppre3: self.ppre3(),
                ahb1dis: self.ahb1dis(),
                ahb2dis: self.ahb2dis(),
                ahb4dis: self.ahb4dis(),
                apb1dis: self.apb1dis(),
                apb2dis: self.apb2dis(),
                apb3dis: self.apb3dis(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC clock source interrupt clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cicr(pub u32);
    impl Cicr {
        #[doc = "LSI ready interrupt clear Set by software to clear LSIRDYF. Reset by hardware when clear done."]
        #[inline(always)]
        pub const fn lsirdyc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt clear Set by software to clear LSIRDYF. Reset by hardware when clear done."]
        #[inline(always)]
        pub fn set_lsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt clear Set by software to clear LSERDYF. Reset by hardware when clear done."]
        #[inline(always)]
        pub const fn lserdyc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt clear Set by software to clear LSERDYF. Reset by hardware when clear done."]
        #[inline(always)]
        pub fn set_lserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI ready interrupt clear Set by software to clear CSIRDYF. Reset by hardware when clear done."]
        #[inline(always)]
        pub const fn csirdyc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt clear Set by software to clear CSIRDYF. Reset by hardware when clear done."]
        #[inline(always)]
        pub fn set_csirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSI ready interrupt clear Set by software to clear HSIRDYF. Reset by hardware when clear done."]
        #[inline(always)]
        pub const fn hsirdyc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt clear Set by software to clear HSIRDYF. Reset by hardware when clear done."]
        #[inline(always)]
        pub fn set_hsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HSE ready interrupt clear Set by software to clear HSERDYF. Reset by hardware when clear done."]
        #[inline(always)]
        pub const fn hserdyc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt clear Set by software to clear HSERDYF. Reset by hardware when clear done."]
        #[inline(always)]
        pub fn set_hserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "HSI48 ready interrupt clear Set by software to clear HSI48RDYF. Reset by hardware when clear done."]
        #[inline(always)]
        pub const fn hsi48rdyc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 ready interrupt clear Set by software to clear HSI48RDYF. Reset by hardware when clear done."]
        #[inline(always)]
        pub fn set_hsi48rdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PLL1 ready interrupt clear Set by software to clear PLL1RDYF. Reset by hardware when clear done."]
        #[inline(always)]
        pub const fn pllrdyc(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 6usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 ready interrupt clear Set by software to clear PLL1RDYF. Reset by hardware when clear done."]
        #[inline(always)]
        pub fn set_pllrdyc(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 6usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "HSE clock security system interrupt clear Set by software to clear HSECSSF. Reset by hardware when clear done."]
        #[inline(always)]
        pub const fn hsecssc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock security system interrupt clear Set by software to clear HSECSSF. Reset by hardware when clear done."]
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
                .field("csirdyc", &self.csirdyc())
                .field("hsirdyc", &self.hsirdyc())
                .field("hserdyc", &self.hserdyc())
                .field("hsi48rdyc", &self.hsi48rdyc())
                .field("pllrdyc", &[self.pllrdyc(0usize), self.pllrdyc(1usize)])
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
                csirdyc: bool,
                hsirdyc: bool,
                hserdyc: bool,
                hsi48rdyc: bool,
                pllrdyc: [bool; 2usize],
                hsecssc: bool,
            }
            let proxy = Cicr {
                lsirdyc: self.lsirdyc(),
                lserdyc: self.lserdyc(),
                csirdyc: self.csirdyc(),
                hsirdyc: self.hsirdyc(),
                hserdyc: self.hserdyc(),
                hsi48rdyc: self.hsi48rdyc(),
                pllrdyc: [self.pllrdyc(0usize), self.pllrdyc(1usize)],
                hsecssc: self.hsecssc(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC clock source interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cier(pub u32);
    impl Cier {
        #[doc = "LSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSI oscillator stabilization."]
        #[inline(always)]
        pub const fn lsirdyie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSI oscillator stabilization."]
        #[inline(always)]
        pub fn set_lsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSE oscillator stabilization."]
        #[inline(always)]
        pub const fn lserdyie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the LSE oscillator stabilization."]
        #[inline(always)]
        pub fn set_lserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the CSI oscillator stabilization."]
        #[inline(always)]
        pub const fn csirdyie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "CSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the CSI oscillator stabilization."]
        #[inline(always)]
        pub fn set_csirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI oscillator stabilization."]
        #[inline(always)]
        pub const fn hsirdyie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI oscillator stabilization."]
        #[inline(always)]
        pub fn set_hsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSE oscillator stabilization."]
        #[inline(always)]
        pub const fn hserdyie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSE oscillator stabilization."]
        #[inline(always)]
        pub fn set_hserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "HSI48 ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI48 oscillator stabilization."]
        #[inline(always)]
        pub const fn hsi48rdyie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 ready interrupt enable Set and reset by software to enable/disable interrupt caused by the HSI48 oscillator stabilization."]
        #[inline(always)]
        pub fn set_hsi48rdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PLL1 ready interrupt enable Set and reset by software to enable/disable interrupt caused by PLL1 lock."]
        #[inline(always)]
        pub const fn pllrdyie(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 6usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 ready interrupt enable Set and reset by software to enable/disable interrupt caused by PLL1 lock."]
        #[inline(always)]
        pub fn set_pllrdyie(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 6usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
                .field("csirdyie", &self.csirdyie())
                .field("hsirdyie", &self.hsirdyie())
                .field("hserdyie", &self.hserdyie())
                .field("hsi48rdyie", &self.hsi48rdyie())
                .field("pllrdyie", &[self.pllrdyie(0usize), self.pllrdyie(1usize)])
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
                csirdyie: bool,
                hsirdyie: bool,
                hserdyie: bool,
                hsi48rdyie: bool,
                pllrdyie: [bool; 2usize],
            }
            let proxy = Cier {
                lsirdyie: self.lsirdyie(),
                lserdyie: self.lserdyie(),
                csirdyie: self.csirdyie(),
                hsirdyie: self.hsirdyie(),
                hserdyie: self.hserdyie(),
                hsi48rdyie: self.hsi48rdyie(),
                pllrdyie: [self.pllrdyie(0usize), self.pllrdyie(1usize)],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC clock source interrupt flag register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cifr(pub u32);
    impl Cifr {
        #[doc = "LSI ready interrupt flag Reset by software by writing LSIRDYC bit. Set by hardware when the LSI clock becomes stable and LSIRDYIE is set."]
        #[inline(always)]
        pub const fn lsirdyf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt flag Reset by software by writing LSIRDYC bit. Set by hardware when the LSI clock becomes stable and LSIRDYIE is set."]
        #[inline(always)]
        pub fn set_lsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt flag Reset by software by writing LSERDYC bit. Set by hardware when the LSE clock becomes stable and LSERDYIE is set."]
        #[inline(always)]
        pub const fn lserdyf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt flag Reset by software by writing LSERDYC bit. Set by hardware when the LSE clock becomes stable and LSERDYIE is set."]
        #[inline(always)]
        pub fn set_lserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "CSI ready interrupt flag Reset by software by writing CSIRDYC bit. Set by hardware when the CSI clock becomes stable and CSIRDYIE is set."]
        #[inline(always)]
        pub const fn csirdyf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "CSI ready interrupt flag Reset by software by writing CSIRDYC bit. Set by hardware when the CSI clock becomes stable and CSIRDYIE is set."]
        #[inline(always)]
        pub fn set_csirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSI ready interrupt flag Reset by software by writing HSIRDYC bit. Set by hardware when the HSI clock becomes stable and HSIRDYIE is set."]
        #[inline(always)]
        pub const fn hsirdyf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt flag Reset by software by writing HSIRDYC bit. Set by hardware when the HSI clock becomes stable and HSIRDYIE is set."]
        #[inline(always)]
        pub fn set_hsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HSE ready interrupt flag Reset by software by writing HSERDYC bit. Set by hardware when the HSE clock becomes stable and HSERDYIE is set."]
        #[inline(always)]
        pub const fn hserdyf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt flag Reset by software by writing HSERDYC bit. Set by hardware when the HSE clock becomes stable and HSERDYIE is set."]
        #[inline(always)]
        pub fn set_hserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "HSI48 ready interrupt flag Reset by software by writing HSI48RDYC bit. Set by hardware when the HSI48 clock becomes stable and HSI48RDYIE is set."]
        #[inline(always)]
        pub const fn hsi48rdyf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 ready interrupt flag Reset by software by writing HSI48RDYC bit. Set by hardware when the HSI48 clock becomes stable and HSI48RDYIE is set."]
        #[inline(always)]
        pub fn set_hsi48rdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PLL1 ready interrupt flag Reset by software by writing PLL1RDYC bit. Set by hardware when the PLL1 locks and PLL1RDYIE is set."]
        #[inline(always)]
        pub const fn pllrdyf(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 6usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 ready interrupt flag Reset by software by writing PLL1RDYC bit. Set by hardware when the PLL1 locks and PLL1RDYIE is set."]
        #[inline(always)]
        pub fn set_pllrdyf(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 6usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "HSE clock security system interrupt flag Reset by software by writing HSECSSC bit. Set by hardware in case of HSE clock failure."]
        #[inline(always)]
        pub const fn hsecssf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock security system interrupt flag Reset by software by writing HSECSSC bit. Set by hardware in case of HSE clock failure."]
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
                .field("csirdyf", &self.csirdyf())
                .field("hsirdyf", &self.hsirdyf())
                .field("hserdyf", &self.hserdyf())
                .field("hsi48rdyf", &self.hsi48rdyf())
                .field("pllrdyf", &[self.pllrdyf(0usize), self.pllrdyf(1usize)])
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
                csirdyf: bool,
                hsirdyf: bool,
                hserdyf: bool,
                hsi48rdyf: bool,
                pllrdyf: [bool; 2usize],
                hsecssf: bool,
            }
            let proxy = Cifr {
                lsirdyf: self.lsirdyf(),
                lserdyf: self.lserdyf(),
                csirdyf: self.csirdyf(),
                hsirdyf: self.hsirdyf(),
                hserdyf: self.hserdyf(),
                hsi48rdyf: self.hsi48rdyf(),
                pllrdyf: [self.pllrdyf(0usize), self.pllrdyf(1usize)],
                hsecssf: self.hsecssf(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC clock control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "HSI clock enable Set and cleared by software. Set by hardware to force the HSI to ON when the product leaves Stop mode, if STOPWUCK = 1 or STOPKERWUCK = 1. Set by hardware to force the HSI to ON when the product leaves Standby mode or in case of a failure of the HSE which is used as the system clock source. This bit cannot be cleared if the HSI is used directly (via SW mux) as system clock, or if the HSI is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1)."]
        #[inline(always)]
        pub const fn hsion(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "HSI clock enable Set and cleared by software. Set by hardware to force the HSI to ON when the product leaves Stop mode, if STOPWUCK = 1 or STOPKERWUCK = 1. Set by hardware to force the HSI to ON when the product leaves Standby mode or in case of a failure of the HSE which is used as the system clock source. This bit cannot be cleared if the HSI is used directly (via SW mux) as system clock, or if the HSI is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1)."]
        #[inline(always)]
        pub fn set_hsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "HSI clock ready flag Set by hardware to indicate that the HSI oscillator is stable."]
        #[inline(always)]
        pub const fn hsirdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "HSI clock ready flag Set by hardware to indicate that the HSI oscillator is stable."]
        #[inline(always)]
        pub fn set_hsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI clock enable in Stop mode Set and reset by software to force the HSI to ON, even in Stop mode, in order to be quickly available as kernel clock for peripherals. This bit has no effect on the value of HSION."]
        #[inline(always)]
        pub const fn hsikeron(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSI clock enable in Stop mode Set and reset by software to force the HSI to ON, even in Stop mode, in order to be quickly available as kernel clock for peripherals. This bit has no effect on the value of HSION."]
        #[inline(always)]
        pub fn set_hsikeron(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSI clock divider Set and reset by software. These bits allow selecting a division ratio in order to configure the wanted HSI clock frequency. The HSIDIV cannot be changed if the HSI is selected as reference clock for at least one enabled PLL (PLLxON bit set to 1). In that case, the new HSIDIV value is ignored."]
        #[inline(always)]
        pub const fn hsidiv(&self) -> super::vals::Hsidiv {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Hsidiv::from_bits(val as u8)
        }
        #[doc = "HSI clock divider Set and reset by software. These bits allow selecting a division ratio in order to configure the wanted HSI clock frequency. The HSIDIV cannot be changed if the HSI is selected as reference clock for at least one enabled PLL (PLLxON bit set to 1). In that case, the new HSIDIV value is ignored."]
        #[inline(always)]
        pub fn set_hsidiv(&mut self, val: super::vals::Hsidiv) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "HSI divider flag Set and reset by hardware. As a write operation to HSIDIV has not an immediate effect on the frequency, this flag indicates the current status of the HSI divider. HSIDIVF goes immediately to 0 when HSIDIV value is changed, and is set back to 1 when the output frequency matches the value programmed into HSIDIV."]
        #[inline(always)]
        pub const fn hsidivf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HSI divider flag Set and reset by hardware. As a write operation to HSIDIV has not an immediate effect on the frequency, this flag indicates the current status of the HSI divider. HSIDIVF goes immediately to 0 when HSIDIV value is changed, and is set back to 1 when the output frequency matches the value programmed into HSIDIV."]
        #[inline(always)]
        pub fn set_hsidivf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "CSI clock enable Set and reset by software to enable/disable CSI clock for system and/or peripheral. Set by hardware to force the CSI to ON when the system leaves Stop mode, if STOPWUCK = 1 or STOPKERWUCK = 1. This bit cannot be cleared if the CSI is used directly (via SW mux) as system clock, or if the CSI is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1)."]
        #[inline(always)]
        pub const fn csion(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "CSI clock enable Set and reset by software to enable/disable CSI clock for system and/or peripheral. Set by hardware to force the CSI to ON when the system leaves Stop mode, if STOPWUCK = 1 or STOPKERWUCK = 1. This bit cannot be cleared if the CSI is used directly (via SW mux) as system clock, or if the CSI is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1)."]
        #[inline(always)]
        pub fn set_csion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CSI clock ready flag Set by hardware to indicate that the CSI oscillator is stable. This bit is activated only if the RC is enabled by CSION (it is not activated if the CSI is enabled by CSIKERON or by a peripheral request)."]
        #[inline(always)]
        pub const fn csirdy(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "CSI clock ready flag Set by hardware to indicate that the CSI oscillator is stable. This bit is activated only if the RC is enabled by CSION (it is not activated if the CSI is enabled by CSIKERON or by a peripheral request)."]
        #[inline(always)]
        pub fn set_csirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "CSI clock enable in Stop mode Set and reset by software to force the CSI to ON, even in Stop mode, in order to be quickly available as kernel clock for some peripherals. This bit has no effect on the value of CSION."]
        #[inline(always)]
        pub const fn csikeron(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "CSI clock enable in Stop mode Set and reset by software to force the CSI to ON, even in Stop mode, in order to be quickly available as kernel clock for some peripherals. This bit has no effect on the value of CSION."]
        #[inline(always)]
        pub fn set_csikeron(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "HSI48 clock enable Set by software and cleared by software or by the hardware when the system enters to Stop or Standby mode."]
        #[inline(always)]
        pub const fn hsi48on(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 clock enable Set by software and cleared by software or by the hardware when the system enters to Stop or Standby mode."]
        #[inline(always)]
        pub fn set_hsi48on(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "HSI48 clock ready flag Set by hardware to indicate that the HSI48 oscillator is stable."]
        #[inline(always)]
        pub const fn hsi48rdy(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "HSI48 clock ready flag Set by hardware to indicate that the HSI48 oscillator is stable."]
        #[inline(always)]
        pub fn set_hsi48rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE when entering Stop or Standby mode. This bit cannot be cleared if the HSE is used directly (via SW mux) as system clock, or if the HSE is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1)."]
        #[inline(always)]
        pub const fn hseon(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE when entering Stop or Standby mode. This bit cannot be cleared if the HSE is used directly (via SW mux) as system clock, or if the HSE is selected as reference clock for PLL1 with PLL1 enabled (PLL1ON bit set to 1)."]
        #[inline(always)]
        pub fn set_hseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "HSE clock ready flag Set by hardware to indicate that the HSE oscillator is stable."]
        #[inline(always)]
        pub const fn hserdy(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock ready flag Set by hardware to indicate that the HSE oscillator is stable."]
        #[inline(always)]
        pub fn set_hserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "HSE clock bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled."]
        #[inline(always)]
        pub const fn hsebyp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock bypass Set and cleared by software to bypass the oscillator with an external clock. The external clock must be enabled with the HSEON bit to be used by the device. The HSEBYP bit can be written only if the HSE oscillator is disabled."]
        #[inline(always)]
        pub fn set_hsebyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "HSE clock security system enable Set by software to enable clock security system on HSE. This bit is “set only” (disabled by a system reset or when the system enters in Standby mode). When HSECSSON is set, the clock detector is enabled by hardware when the HSE is ready and disabled by hardware if an oscillator failure is detected."]
        #[inline(always)]
        pub const fn hsecsson(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock security system enable Set by software to enable clock security system on HSE. This bit is “set only” (disabled by a system reset or when the system enters in Standby mode). When HSECSSON is set, the clock detector is enabled by hardware when the HSE is ready and disabled by hardware if an oscillator failure is detected."]
        #[inline(always)]
        pub fn set_hsecsson(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "external high speed clock type in Bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the HSEON bit to be used by the device. The HSEEXT bit can be written only if the HSE oscillator is disabled."]
        #[inline(always)]
        pub const fn hseext(&self) -> super::vals::Hseext {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Hseext::from_bits(val as u8)
        }
        #[doc = "external high speed clock type in Bypass mode Set and reset by software to select the external clock type (analog or digital). The external clock must be enabled with the HSEON bit to be used by the device. The HSEEXT bit can be written only if the HSE oscillator is disabled."]
        #[inline(always)]
        pub fn set_hseext(&mut self, val: super::vals::Hseext) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "PLL1 enable Set and cleared by software to enable PLL1. Cleared by hardware when entering Stop or Standby mode. Note that the hardware prevents writing this bit to 0, if the PLL1 output is used as the system clock."]
        #[inline(always)]
        pub const fn pllon(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 24usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 enable Set and cleared by software to enable PLL1. Cleared by hardware when entering Stop or Standby mode. Note that the hardware prevents writing this bit to 0, if the PLL1 output is used as the system clock."]
        #[inline(always)]
        pub fn set_pllon(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 24usize + n * 2usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "PLL1 clock ready flag Set by hardware to indicate that the PLL1 is locked."]
        #[inline(always)]
        pub const fn pllrdy(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 25usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PLL1 clock ready flag Set by hardware to indicate that the PLL1 is locked."]
        #[inline(always)]
        pub fn set_pllrdy(&mut self, n: usize, val: bool) {
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
    impl core::fmt::Debug for Cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr")
                .field("hsion", &self.hsion())
                .field("hsirdy", &self.hsirdy())
                .field("hsikeron", &self.hsikeron())
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
                .field("hsecsson", &self.hsecsson())
                .field("hseext", &self.hseext())
                .field("pllon", &[self.pllon(0usize), self.pllon(1usize)])
                .field("pllrdy", &[self.pllrdy(0usize), self.pllrdy(1usize)])
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cr {
                hsion: bool,
                hsirdy: bool,
                hsikeron: bool,
                hsidiv: super::vals::Hsidiv,
                hsidivf: bool,
                csion: bool,
                csirdy: bool,
                csikeron: bool,
                hsi48on: bool,
                hsi48rdy: bool,
                hseon: bool,
                hserdy: bool,
                hsebyp: bool,
                hsecsson: bool,
                hseext: super::vals::Hseext,
                pllon: [bool; 2usize],
                pllrdy: [bool; 2usize],
            }
            let proxy = Cr {
                hsion: self.hsion(),
                hsirdy: self.hsirdy(),
                hsikeron: self.hsikeron(),
                hsidiv: self.hsidiv(),
                hsidivf: self.hsidivf(),
                csion: self.csion(),
                csirdy: self.csirdy(),
                csikeron: self.csikeron(),
                hsi48on: self.hsi48on(),
                hsi48rdy: self.hsi48rdy(),
                hseon: self.hseon(),
                hserdy: self.hserdy(),
                hsebyp: self.hsebyp(),
                hsecsson: self.hsecsson(),
                hseext: self.hseext(),
                pllon: [self.pllon(0usize), self.pllon(1usize)],
                pllrdy: [self.pllrdy(0usize), self.pllrdy(1usize)],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC clock recovery RC register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Crrcr(pub u32);
    impl Crrcr {
        #[doc = "Internal RC 48 MHz clock calibration Set by hardware by option-byte loading during system reset NRESET. Read-only."]
        #[inline(always)]
        pub const fn hsi48cal(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Internal RC 48 MHz clock calibration Set by hardware by option-byte loading during system reset NRESET. Read-only."]
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
    #[doc = "RCC CSI calibration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csicfgr(pub u32);
    impl Csicfgr {
        #[doc = "CSI clock calibration Set by hardware by option byte loading during system reset NRESET. Adjusted by software through trimming bits CSITRIM. This field represents the sum of engineering option byte calibration value and CSITRIM bits value."]
        #[inline(always)]
        pub const fn csical(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "CSI clock calibration Set by hardware by option byte loading during system reset NRESET. Adjusted by software through trimming bits CSITRIM. This field represents the sum of engineering option byte calibration value and CSITRIM bits value."]
        #[inline(always)]
        pub fn set_csical(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "CSI clock trimming Set by software to adjust calibration. CSITRIM field is added to the engineering option bytes loaded during reset phase (FLASH_CSI_OPT) in order to form the calibration trimming value. CSICAL = CSITRIM + FLASH_CSI_OPT. Note: The reset value of the field is 0x20."]
        #[inline(always)]
        pub const fn csitrim(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "CSI clock trimming Set by software to adjust calibration. CSITRIM field is added to the engineering option bytes loaded during reset phase (FLASH_CSI_OPT) in order to form the calibration trimming value. CSICAL = CSITRIM + FLASH_CSI_OPT. Note: The reset value of the field is 0x20."]
        #[inline(always)]
        pub fn set_csitrim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
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
                csical: u8,
                csitrim: u8,
            }
            let proxy = Csicfgr {
                csical: self.csical(),
                csitrim: self.csitrim(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "RCC HSI calibration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hsicfgr(pub u32);
    impl Hsicfgr {
        #[doc = "HSI clock calibration Set by hardware by option byte loading during system reset nreset. Adjusted by software through trimming bits HSITRIM. This field represents the sum of engineering option byte calibration value and HSITRIM bits value."]
        #[inline(always)]
        pub const fn hsical(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "HSI clock calibration Set by hardware by option byte loading during system reset nreset. Adjusted by software through trimming bits HSITRIM. This field represents the sum of engineering option byte calibration value and HSITRIM bits value."]
        #[inline(always)]
        pub fn set_hsical(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "HSI clock trimming Set by software to adjust calibration. HSITRIM field is added to the engineering option bytes loaded during reset phase (FLASH_HSI_OPT) in order to form the calibration trimming value. HSICAL = HSITRIM + FLASH_HSI_OPT. After a change of HSITRIM it takes one system clock cycle before the new HSITRIM value is updated Note: The reset value of the field is 0x40."]
        #[inline(always)]
        pub const fn hsitrim(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "HSI clock trimming Set by software to adjust calibration. HSITRIM field is added to the engineering option bytes loaded during reset phase (FLASH_HSI_OPT) in order to form the calibration trimming value. HSICAL = HSITRIM + FLASH_HSI_OPT. After a change of HSITRIM it takes one system clock cycle before the new HSITRIM value is updated Note: The reset value of the field is 0x40."]
        #[inline(always)]
        pub fn set_hsitrim(&mut self, val: u8) {
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
    #[doc = "RCC PLL clock source selection register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pllcfgr(pub u32);
    impl Pllcfgr {
        #[doc = "DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLL1SRC must be set to '00'. 00: no clock send to DIVMx divider and PLLs (default after reset)."]
        #[inline(always)]
        pub const fn pllsrc(&self) -> super::vals::Pllsrc {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Pllsrc::from_bits(val as u8)
        }
        #[doc = "DIVMx and PLLs clock source selection Set and reset by software to select the PLL clock source. These bits can be written only when all PLLs are disabled. In order to save power, when no PLL is used, the value of PLL1SRC must be set to '00'. 00: no clock send to DIVMx divider and PLLs (default after reset)."]
        #[inline(always)]
        pub fn set_pllsrc(&mut self, val: super::vals::Pllsrc) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1."]
        #[inline(always)]
        pub const fn pllrge(&self) -> super::vals::Pllrge {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Pllrge::from_bits(val as u8)
        }
        #[doc = "PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1."]
        #[inline(always)]
        pub fn set_pllrge(&mut self, val: super::vals::Pllrge) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "PLL1 fractional latch enable Set and reset by software to latch the content of FRACN1 into the sigma-delta modulator. In order to latch the FRACN1 value into the sigma-delta modulator, PLL1FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN1 into the modulator."]
        #[inline(always)]
        pub const fn pllfracen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PLL1 fractional latch enable Set and reset by software to latch the content of FRACN1 into the sigma-delta modulator. In order to latch the FRACN1 value into the sigma-delta modulator, PLL1FRACEN must be set to 0, then set to 1. The transition 0 to 1 transfers the content of FRACN1 into the modulator."]
        #[inline(always)]
        pub fn set_pllfracen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "PLL1 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL1. This bit must be written before enabling the PLL1."]
        #[inline(always)]
        pub const fn pllvcosel(&self) -> super::vals::Pllvcosel {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Pllvcosel::from_bits(val as u8)
        }
        #[doc = "PLL1 VCO selection Set and reset by software to select the proper VCO frequency range used for PLL1. This bit must be written before enabling the PLL1."]
        #[inline(always)]
        pub fn set_pllvcosel(&mut self, val: super::vals::Pllvcosel) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The hardware does not allow any modification of this prescaler when PLL1 is enabled (PLL1ON = 1 or PLL1RDY = 1). In order to save power when PLL1 is not used, the value of DIVM1 must be set to 0. ... ..."]
        #[inline(always)]
        pub const fn divm(&self) -> super::vals::Pllm {
            let val = (self.0 >> 8usize) & 0x3f;
            super::vals::Pllm::from_bits(val as u8)
        }
        #[doc = "prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The hardware does not allow any modification of this prescaler when PLL1 is enabled (PLL1ON = 1 or PLL1RDY = 1). In order to save power when PLL1 is not used, the value of DIVM1 must be set to 0. ... ..."]
        #[inline(always)]
        pub fn set_divm(&mut self, val: super::vals::Pllm) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val.to_bits() as u32) & 0x3f) << 8usize);
        }
        #[doc = "PLL1 DIVP divider output enable Set and reset by software to enable the pll1_p_ck output of the PLL1. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). In order to save power, when the pll1_p_ck output of the PLL1 is not used, the pll1_p_ck must be disabled."]
        #[inline(always)]
        pub const fn pllpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PLL1 DIVP divider output enable Set and reset by software to enable the pll1_p_ck output of the PLL1. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). In order to save power, when the pll1_p_ck output of the PLL1 is not used, the pll1_p_ck must be disabled."]
        #[inline(always)]
        pub fn set_pllpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "PLL1 DIVQ divider output enable Set and reset by software to enable the pll1_q_ck output of the PLL1. In order to save power, when the pll1_q_ck output of the PLL1 is not used, the pll1_q_ck must be disabled. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
        #[inline(always)]
        pub const fn pllqen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "PLL1 DIVQ divider output enable Set and reset by software to enable the pll1_q_ck output of the PLL1. In order to save power, when the pll1_q_ck output of the PLL1 is not used, the pll1_q_ck must be disabled. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
        #[inline(always)]
        pub fn set_pllqen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "PLL1 DIVR divider output enable Set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, DIVR1EN and DIVR1 bits must be set to 0 when the pll1_r_ck is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
        #[inline(always)]
        pub const fn pllren(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PLL1 DIVR divider output enable Set and reset by software to enable the pll1_r_ck output of the PLL1. To save power, DIVR1EN and DIVR1 bits must be set to 0 when the pll1_r_ck is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
        #[inline(always)]
        pub fn set_pllren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
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
                .field("pllsrc", &self.pllsrc())
                .field("pllrge", &self.pllrge())
                .field("pllfracen", &self.pllfracen())
                .field("pllvcosel", &self.pllvcosel())
                .field("divm", &self.divm())
                .field("pllpen", &self.pllpen())
                .field("pllqen", &self.pllqen())
                .field("pllren", &self.pllren())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pllcfgr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Pllcfgr {
                pllsrc: super::vals::Pllsrc,
                pllrge: super::vals::Pllrge,
                pllfracen: bool,
                pllvcosel: super::vals::Pllvcosel,
                divm: super::vals::Pllm,
                pllpen: bool,
                pllqen: bool,
                pllren: bool,
            }
            let proxy = Pllcfgr {
                pllsrc: self.pllsrc(),
                pllrge: self.pllrge(),
                pllfracen: self.pllfracen(),
                pllvcosel: self.pllvcosel(),
                divm: self.divm(),
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
        #[doc = "Multiplication factor for PLL1VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL1ON = 0 and PLL1RDY = 0). ... ... Others: reserved"]
        #[inline(always)]
        pub const fn plln(&self) -> super::vals::Plln {
            let val = (self.0 >> 0usize) & 0x01ff;
            super::vals::Plln::from_bits(val as u16)
        }
        #[doc = "Multiplication factor for PLL1VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled (PLL1ON = 0 and PLL1RDY = 0). ... ... Others: reserved"]
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
        #[doc = "fractional part of the multiplication factor for PLL1 VCO Set and reset by software to control the fractional part of the multiplication factor of the VCO. These bits can be written at any time, allowing dynamic fine-tuning of the PLL1 VCO. The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: * 128 to 560 MHz if PLL1VCOSEL = 0 * 150 to 420 MHz if PLL1VCOSEL = 1 VCO output frequency = Fref1_ck x (PLL1N + (PLL1FRACN / 213)), with * PLL1N between 8 and 420 * PLL1FRACN can be between 0 and 213- 1 * The input frequency Fref1_ck must be between 1 and 16 MHz. To change the PLL1FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: * Set the bit PLL1FRACEN to 0 * Write the new fractional value into PLL1FRACN * Set the bit PLL1FRACEN to 1"]
        #[inline(always)]
        pub const fn pllfracn(&self) -> u16 {
            let val = (self.0 >> 3usize) & 0x1fff;
            val as u16
        }
        #[doc = "fractional part of the multiplication factor for PLL1 VCO Set and reset by software to control the fractional part of the multiplication factor of the VCO. These bits can be written at any time, allowing dynamic fine-tuning of the PLL1 VCO. The software must set correctly these bits to insure that the VCO output frequency is between its valid frequency range, that is: * 128 to 560 MHz if PLL1VCOSEL = 0 * 150 to 420 MHz if PLL1VCOSEL = 1 VCO output frequency = Fref1_ck x (PLL1N + (PLL1FRACN / 213)), with * PLL1N between 8 and 420 * PLL1FRACN can be between 0 and 213- 1 * The input frequency Fref1_ck must be between 1 and 16 MHz. To change the PLL1FRACN value on-the-fly even if the PLL is enabled, the application must proceed as follows: * Set the bit PLL1FRACEN to 0 * Write the new fractional value into PLL1FRACN * Set the bit PLL1FRACEN to 1"]
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
    #[doc = "RCC reset status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rsr(pub u32);
    impl Rsr {
        #[doc = "remove reset flag Set and reset by software to reset the value of the reset flags."]
        #[inline(always)]
        pub const fn rmvf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "remove reset flag Set and reset by software to reset the value of the reset flags."]
        #[inline(always)]
        pub fn set_rmvf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "pin reset flag (NRST) Reset by software by writing the RMVF bit. Set by hardware when a reset from pin occurs."]
        #[inline(always)]
        pub const fn pinrstf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "pin reset flag (NRST) Reset by software by writing the RMVF bit. Set by hardware when a reset from pin occurs."]
        #[inline(always)]
        pub fn set_pinrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "BOR reset flag Reset by software by writing the RMVF bit. Set by hardware when a BOR reset occurs (pwr_bor_rst)."]
        #[inline(always)]
        pub const fn borrstf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "BOR reset flag Reset by software by writing the RMVF bit. Set by hardware when a BOR reset occurs (pwr_bor_rst)."]
        #[inline(always)]
        pub fn set_borrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "system reset from CPU reset flag Reset by software by writing the RMVF bit. Set by hardware when the system reset is due to CPU.The CPU can generate a system reset by writing SYSRESETREQ bit of AIRCR register of the core M33."]
        #[inline(always)]
        pub const fn sftrstf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "system reset from CPU reset flag Reset by software by writing the RMVF bit. Set by hardware when the system reset is due to CPU.The CPU can generate a system reset by writing SYSRESETREQ bit of AIRCR register of the core M33."]
        #[inline(always)]
        pub fn set_sftrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "independent watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when an independent watchdog reset occurs."]
        #[inline(always)]
        pub const fn iwdgrstf(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "independent watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when an independent watchdog reset occurs."]
        #[inline(always)]
        pub fn set_iwdgrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "window watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when a window watchdog reset occurs."]
        #[inline(always)]
        pub const fn wwdgrstf(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "window watchdog reset flag Reset by software by writing the RMVF bit. Set by hardware when a window watchdog reset occurs."]
        #[inline(always)]
        pub fn set_wwdgrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Low-power reset flag Set by hardware when a reset occurs due to Stop or Standby mode entry, whereas the corresponding nRST_STOP, nRST_STBY option bit is cleared. Cleared by writing to the RMVF bit."]
        #[inline(always)]
        pub const fn lpwrrstf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power reset flag Set by hardware when a reset occurs due to Stop or Standby mode entry, whereas the corresponding nRST_STOP, nRST_STBY option bit is cleared. Cleared by writing to the RMVF bit."]
        #[inline(always)]
        pub fn set_lpwrrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("borrstf", &self.borrstf())
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
            #[derive(defmt :: Format)]
            struct Rsr {
                rmvf: bool,
                pinrstf: bool,
                borrstf: bool,
                sftrstf: bool,
                iwdgrstf: bool,
                wwdgrstf: bool,
                lpwrrstf: bool,
            }
            let proxy = Rsr {
                rmvf: self.rmvf(),
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
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adcdacsel {
        #[doc = "rcc_hclk selected as kernel clock (default after reset)"]
        HCLK2 = 0x0,
        #[doc = "sys_ck selected as kernel clock"]
        SYS = 0x01,
        #[doc = "pll2_r_ck selected as kernel clock"]
        PLL2_R = 0x02,
        #[doc = "hse_ck selected as kernel clock"]
        HSE = 0x03,
        #[doc = "hsi_ker_ck selected as kernel clock"]
        HSI = 0x04,
        #[doc = "csi_ker_ck selected as kernel clock"]
        CSI = 0x05,
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
    pub enum Dacholdsel {
        #[doc = "dac_hold_ck selected as kernel clock (default after reset)"]
        DAC_HOLD = 0x0,
        #[doc = "dac_hold_ck selected as kernel clock"]
        DAC_HOLD_2 = 0x01,
    }
    impl Dacholdsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dacholdsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dacholdsel {
        #[inline(always)]
        fn from(val: u8) -> Dacholdsel {
            Dacholdsel::from_bits(val)
        }
    }
    impl From<Dacholdsel> for u8 {
        #[inline(always)]
        fn from(val: Dacholdsel) -> u8 {
            Dacholdsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fdcansel {
        #[doc = "hse_ck selected as kernel clock (default after reset)"]
        HSE = 0x0,
        #[doc = "pll1_q_ck selected as kernel clock"]
        PLL1_Q = 0x01,
        #[doc = "pll2_q_ck selected as kernel clock"]
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
    pub enum Hseext {
        #[doc = "HSE in analog mode (default after reset)"]
        ANALOG = 0x0,
        #[doc = "HSE in digital mode"]
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
    pub enum I2csel {
        #[doc = "rcc_pclk1 selected as peripheral clock"]
        PCLK1 = 0x0,
        #[doc = "pll3_r selected as peripheral clock"]
        PLL3_R = 0x01,
        #[doc = "hsi_ker selected as peripheral clock"]
        HSI = 0x02,
        #[doc = "csi_ker selected as peripheral clock"]
        CSI = 0x03,
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
    pub enum I3c2sel {
        #[doc = "rcc_pclk3 selected as peripheral clock"]
        PCLK3 = 0x0,
        #[doc = "pll3_r selected as peripheral clock"]
        PLL3_R = 0x01,
        #[doc = "hsi_ker selected as peripheral clock"]
        HSI = 0x02,
        #[doc = "csi_ker selected as peripheral clock"]
        CSI = 0x03,
    }
    impl I3c2sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I3c2sel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for I3c2sel {
        #[inline(always)]
        fn from(val: u8) -> I3c2sel {
            I3c2sel::from_bits(val)
        }
    }
    impl From<I3c2sel> for u8 {
        #[inline(always)]
        fn from(val: I3c2sel) -> u8 {
            I3c2sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lptim1sel {
        #[doc = "rcc_pclk3 selected as peripheral clock"]
        PCLK3 = 0x0,
        #[doc = "pll2_p selected as peripheral clock"]
        PLL2_P = 0x01,
        _RESERVED_2 = 0x02,
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
        #[doc = "rcc_pclk1 selected as peripheral clock"]
        PCLK1 = 0x0,
        #[doc = "pll2_p selected as peripheral clock"]
        PLL2_P = 0x01,
        _RESERVED_2 = 0x02,
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
        #[doc = "rcc_pclk3 selected as kernel clock (default after reset)"]
        PCLK3 = 0x0,
        #[doc = "pll2_q_ck selected as kernel clock"]
        PLL2_Q = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "hsi_ker_ck selected as kernel clock"]
        HSI = 0x03,
        #[doc = "csi_ker_ck selected as kernel clock"]
        CSI = 0x04,
        #[doc = "lse_ck selected as kernel clock"]
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
    pub enum Lseext {
        #[doc = "LSE in analog mode (default after Backup domain reset)"]
        ANALOG = 0x0,
        #[doc = "LSE in digital mode (do not use if RTC is active)."]
        DIGITAL = 0x01,
    }
    impl Lseext {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lseext {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lseext {
        #[inline(always)]
        fn from(val: u8) -> Lseext {
            Lseext::from_bits(val)
        }
    }
    impl From<Lseext> for u8 {
        #[inline(always)]
        fn from(val: Lseext) -> u8 {
            Lseext::to_bits(val)
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
        #[doc = "hsi_ker_ck selected as kernel clock (default after reset)"]
        HSI = 0x0,
        #[doc = "csi_ker_ck selected as kernel clock"]
        CSI = 0x01,
        #[doc = "hse_ck selected as kernel clock"]
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
        #[doc = "no clock send to DIVMx divider and PLLs (default after reset)"]
        DISABLE = 0x0,
        #[doc = "HSI selected as PLL clock (hsi_ck)"]
        HSI = 0x01,
        #[doc = "CSI selected as PLL clock (csi_ck)"]
        CSI = 0x02,
        #[doc = "HSE selected as PLL clock (hse_ck)"]
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
        #[doc = "rcc_pclk3 = rcc_hclk1 / 1"]
        DIV1 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "rcc_pclk3 = rcc_hclk1 / 2"]
        DIV2 = 0x04,
        #[doc = "rcc_pclk3 = rcc_hclk1 / 4"]
        DIV4 = 0x05,
        #[doc = "rcc_pclk3 = rcc_hclk1 / 8"]
        DIV8 = 0x06,
        #[doc = "rcc_pclk3 = rcc_hclk1 / 16"]
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
        #[doc = "hsi48_ker_ck selected as kernel clock (default after reset)"]
        HSI48 = 0x0,
        #[doc = "pll1_q_ck selected as kernel clock"]
        PLL1_Q = 0x01,
        #[doc = "lse_ck selected as kernel clock"]
        LSE = 0x02,
        #[doc = "lsi_ker_ck selected as kernel clock"]
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
        #[doc = "no clock (default after Backup domain reset)"]
        DISABLE = 0x0,
        #[doc = "LSE selected as RTC clock"]
        LSE = 0x01,
        #[doc = "LSI selected as RTC clock"]
        LSI = 0x02,
        #[doc = "HSE divided by RTCPRE value selected as RTC clock"]
        HSE_DIV_RTCPRE = 0x03,
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
    pub enum Spisel {
        #[doc = "pll1_q selected as peripheral clock"]
        PLL1_Q = 0x0,
        #[doc = "pll2_p selected as peripheral clock"]
        PLL2_P = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "hsi_ker selected as peripheral clock"]
        AUDIOCLK = 0x03,
        #[doc = "csi_ker selected as peripheral clock"]
        PER = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Spisel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Spisel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Spisel {
        #[inline(always)]
        fn from(val: u8) -> Spisel {
            Spisel::from_bits(val)
        }
    }
    impl From<Spisel> for u8 {
        #[inline(always)]
        fn from(val: Spisel) -> u8 {
            Spisel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Stopkerwuck {
        #[doc = "HSI selected as wakeup clock from system Stop (default after reset)"]
        HSI = 0x0,
        #[doc = "CSI selected as wakeup clock from system Stop"]
        CSI = 0x01,
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
        _RESERVED_0 = 0x0,
        #[doc = "CSI selected as wakeup clock from system Stop"]
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
    pub enum Systicksel {
        #[doc = "rcc_hclk/8 selected as clock source (default after reset)"]
        HCLK1_DIV_8 = 0x0,
        #[doc = "lsi_ker_ck\\[1\\]
selected as clock source"]
        LSI = 0x01,
        #[doc = "lse_ck\\[1\\]
selected as clock source"]
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
        #[doc = "No internal clock available for timers input capture (default after reset)"]
        B_0X0 = 0x0,
        #[doc = "hsi_ker_ck/1024, hsi_ker_ck/8 and csi_ker_ck/128 selected for timers input capture"]
        B_0X1 = 0x01,
    }
    impl Timicsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Timicsel {
            unsafe { core::mem::transmute(val & 0x01) }
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
    pub enum Timpre {
        #[doc = "The timers kernel clock is equal to rcc_hclk1 if PPRE1 or PPRE2 corresponds to a division by 1 or 2, else it is equal to 2 x Frcc_pclk1 or 2 x Frcc_pclk2 (default after reset)"]
        DEFAULT_X2 = 0x0,
        #[doc = "The timers kernel clock is equal to 2 x Frcc_pclk1 or 2 x Frcc_pclk2 if PPRE1 or PPRE2 corresponds to a division by 1, 2 or 4, else it is equal to 4 x Frcc_pclk1 or 4 x Frcc_pclk2"]
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
    pub enum Usart1sel {
        #[doc = "rcc_pclk2 selected as peripheral clock"]
        PCLK2 = 0x0,
        #[doc = "pll2_q selected as peripheral clock"]
        PLL2_Q = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "hsi_ker selected as peripheral clock"]
        HSI = 0x03,
        #[doc = "csi_ker selected as peripheral clock"]
        CSI = 0x04,
        #[doc = "LSE selected as peripheral clock"]
        LSE = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Usart1sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usart1sel {
            unsafe { core::mem::transmute(val & 0x07) }
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
        #[doc = "rcc_pclk2 selected as peripheral clock"]
        PCLK1 = 0x0,
        #[doc = "pll2_q selected as peripheral clock"]
        PLL2_Q = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "hsi_ker selected as peripheral clock"]
        HSI = 0x03,
        #[doc = "csi_ker selected as peripheral clock"]
        CSI = 0x04,
        #[doc = "LSE selected as peripheral clock"]
        LSE = 0x05,
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
        #[doc = "Disable the kernel clock"]
        DISABLE = 0x0,
        #[doc = "pll1_q selected as peripheral clock"]
        PLL1_Q = 0x01,
        _RESERVED_2 = 0x02,
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
