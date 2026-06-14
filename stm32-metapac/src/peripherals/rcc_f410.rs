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
    #[doc = "PLL configuration register"]
    #[inline(always)]
    pub const fn pllcfgr(self) -> crate::common::Reg<regs::Pllcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "clock configuration register"]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "clock interrupt register"]
    #[inline(always)]
    pub const fn cir(self) -> crate::common::Reg<regs::Cir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "AHB1 peripheral reset register"]
    #[inline(always)]
    pub const fn ahb1rstr(self) -> crate::common::Reg<regs::Ahb1rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "APB1 peripheral reset register"]
    #[inline(always)]
    pub const fn apb1rstr(self) -> crate::common::Reg<regs::Apb1rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "APB2 peripheral reset register"]
    #[inline(always)]
    pub const fn apb2rstr(self) -> crate::common::Reg<regs::Apb2rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "AHB1 peripheral clock register"]
    #[inline(always)]
    pub const fn ahb1enr(self) -> crate::common::Reg<regs::Ahb1enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "APB1 peripheral clock enable register"]
    #[inline(always)]
    pub const fn apb1enr(self) -> crate::common::Reg<regs::Apb1enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "APB2 peripheral clock enable register"]
    #[inline(always)]
    pub const fn apb2enr(self) -> crate::common::Reg<regs::Apb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "AHB1 peripheral clock enable in low power mode register"]
    #[inline(always)]
    pub const fn ahb1lpenr(self) -> crate::common::Reg<regs::Ahb1lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "APB1 peripheral clock enable in low power mode register"]
    #[inline(always)]
    pub const fn apb1lpenr(self) -> crate::common::Reg<regs::Apb1lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "APB2 peripheral clock enabled in low power mode register"]
    #[inline(always)]
    pub const fn apb2lpenr(self) -> crate::common::Reg<regs::Apb2lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Backup domain control register"]
    #[inline(always)]
    pub const fn bdcr(self) -> crate::common::Reg<regs::Bdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "clock control & status register"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "spread spectrum clock generation register"]
    #[inline(always)]
    pub const fn sscgr(self) -> crate::common::Reg<regs::Sscgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "DCKCFGR register"]
    #[inline(always)]
    pub const fn dckcfgr(self) -> crate::common::Reg<regs::Dckcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "DCKCFGR2 register"]
    #[inline(always)]
    pub const fn dckcfgr2(self) -> crate::common::Reg<regs::Dckcfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
}
pub mod regs {
    #[doc = "AHB1 peripheral clock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1enr(pub u32);
    impl Ahb1enr {
        #[doc = "IO port A clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IO port A clock enable"]
        #[inline(always)]
        pub const fn set_gpioaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IO port B clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioben(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IO port B clock enable"]
        #[inline(always)]
        pub const fn set_gpioben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IO port C clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiocen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IO port C clock enable"]
        #[inline(always)]
        pub const fn set_gpiocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "IO port H clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiohen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "IO port H clock enable"]
        #[inline(always)]
        pub const fn set_gpiohen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "CRC clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn crcen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC clock enable"]
        #[inline(always)]
        pub const fn set_crcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "DMA1 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn dma1en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 clock enable"]
        #[inline(always)]
        pub const fn set_dma1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "DMA2 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn dma2en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 clock enable"]
        #[inline(always)]
        pub const fn set_dma2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "RNG clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn rngen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "RNG clock enable"]
        #[inline(always)]
        pub const fn set_rngen(&mut self, val: bool) {
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
                .field("gpioaen", &self.gpioaen())
                .field("gpioben", &self.gpioben())
                .field("gpiocen", &self.gpiocen())
                .field("gpiohen", &self.gpiohen())
                .field("crcen", &self.crcen())
                .field("dma1en", &self.dma1en())
                .field("dma2en", &self.dma2en())
                .field("rngen", &self.rngen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1enr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb1enr {{ gpioaen: {=bool:?}, gpioben: {=bool:?}, gpiocen: {=bool:?}, gpiohen: {=bool:?}, crcen: {=bool:?}, dma1en: {=bool:?}, dma2en: {=bool:?}, rngen: {=bool:?} }}",
                self.gpioaen(),
                self.gpioben(),
                self.gpiocen(),
                self.gpiohen(),
                self.crcen(),
                self.dma1en(),
                self.dma2en(),
                self.rngen()
            )
        }
    }
    #[doc = "AHB1 peripheral clock enable in low power mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1lpenr(pub u32);
    impl Ahb1lpenr {
        #[doc = "IO port A clock enable during sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioalpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IO port A clock enable during sleep mode"]
        #[inline(always)]
        pub const fn set_gpioalpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IO port B clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioblpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IO port B clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_gpioblpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IO port C clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioclpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IO port C clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_gpioclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "IO port H clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiohlpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "IO port H clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_gpiohlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "CRC clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn crclpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_crclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Flash interface clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn flashlpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Flash interface clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_flashlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "SRAM 1interface clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn sram1lpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM 1interface clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_sram1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "DMA1 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn dma1lpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_dma1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "DMA2 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn dma2lpen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_dma2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "RNG clock enable during sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn rnglpen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "RNG clock enable during sleep mode"]
        #[inline(always)]
        pub const fn set_rnglpen(&mut self, val: bool) {
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
                .field("gpioalpen", &self.gpioalpen())
                .field("gpioblpen", &self.gpioblpen())
                .field("gpioclpen", &self.gpioclpen())
                .field("gpiohlpen", &self.gpiohlpen())
                .field("crclpen", &self.crclpen())
                .field("flashlpen", &self.flashlpen())
                .field("sram1lpen", &self.sram1lpen())
                .field("dma1lpen", &self.dma1lpen())
                .field("dma2lpen", &self.dma2lpen())
                .field("rnglpen", &self.rnglpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1lpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb1lpenr {{ gpioalpen: {=bool:?}, gpioblpen: {=bool:?}, gpioclpen: {=bool:?}, gpiohlpen: {=bool:?}, crclpen: {=bool:?}, flashlpen: {=bool:?}, sram1lpen: {=bool:?}, dma1lpen: {=bool:?}, dma2lpen: {=bool:?}, rnglpen: {=bool:?} }}",
                self.gpioalpen(),
                self.gpioblpen(),
                self.gpioclpen(),
                self.gpiohlpen(),
                self.crclpen(),
                self.flashlpen(),
                self.sram1lpen(),
                self.dma1lpen(),
                self.dma2lpen(),
                self.rnglpen()
            )
        }
    }
    #[doc = "AHB1 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1rstr(pub u32);
    impl Ahb1rstr {
        #[doc = "IO port A reset"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioarst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IO port A reset"]
        #[inline(always)]
        pub const fn set_gpioarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IO port B reset"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiobrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IO port B reset"]
        #[inline(always)]
        pub const fn set_gpiobrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IO port C reset"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiocrst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IO port C reset"]
        #[inline(always)]
        pub const fn set_gpiocrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "IO port H reset"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiohrst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "IO port H reset"]
        #[inline(always)]
        pub const fn set_gpiohrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "CRC reset"]
        #[must_use]
        #[inline(always)]
        pub const fn crcrst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC reset"]
        #[inline(always)]
        pub const fn set_crcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "DMA2 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn dma1rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 reset"]
        #[inline(always)]
        pub const fn set_dma1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "DMA2 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn dma2rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 reset"]
        #[inline(always)]
        pub const fn set_dma2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "RNGRST"]
        #[must_use]
        #[inline(always)]
        pub const fn rngrst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "RNGRST"]
        #[inline(always)]
        pub const fn set_rngrst(&mut self, val: bool) {
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
                .field("gpioarst", &self.gpioarst())
                .field("gpiobrst", &self.gpiobrst())
                .field("gpiocrst", &self.gpiocrst())
                .field("gpiohrst", &self.gpiohrst())
                .field("crcrst", &self.crcrst())
                .field("dma1rst", &self.dma1rst())
                .field("dma2rst", &self.dma2rst())
                .field("rngrst", &self.rngrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb1rstr {{ gpioarst: {=bool:?}, gpiobrst: {=bool:?}, gpiocrst: {=bool:?}, gpiohrst: {=bool:?}, crcrst: {=bool:?}, dma1rst: {=bool:?}, dma2rst: {=bool:?}, rngrst: {=bool:?} }}",
                self.gpioarst(),
                self.gpiobrst(),
                self.gpiocrst(),
                self.gpiohrst(),
                self.crcrst(),
                self.dma1rst(),
                self.dma2rst(),
                self.rngrst()
            )
        }
    }
    #[doc = "APB1 peripheral clock enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1enr(pub u32);
    impl Apb1enr {
        #[doc = "TIM5 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim5en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 clock enable"]
        #[inline(always)]
        pub const fn set_tim5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim6en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 clock enable"]
        #[inline(always)]
        pub const fn set_tim6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "LPTIM1 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim1en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 clock enable"]
        #[inline(always)]
        pub const fn set_lptim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "RTC APB clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn rtcapben(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "RTC APB clock enable"]
        #[inline(always)]
        pub const fn set_rtcapben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Window watchdog clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn wwdgen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog clock enable"]
        #[inline(always)]
        pub const fn set_wwdgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI2 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn spi2en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 clock enable"]
        #[inline(always)]
        pub const fn set_spi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "USART 2 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn usart2en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART 2 clock enable"]
        #[inline(always)]
        pub const fn set_usart2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "I2C1 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 clock enable"]
        #[inline(always)]
        pub const fn set_i2c1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c2en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 clock enable"]
        #[inline(always)]
        pub const fn set_i2c2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "FMPI2C1 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn fmpi2c1en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "FMPI2C1 clock enable"]
        #[inline(always)]
        pub const fn set_fmpi2c1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Power interface clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn pwren(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Power interface clock enable"]
        #[inline(always)]
        pub const fn set_pwren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "DAC interface clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn dacen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC interface clock enable"]
        #[inline(always)]
        pub const fn set_dacen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Apb1enr {
        #[inline(always)]
        fn default() -> Apb1enr {
            Apb1enr(0)
        }
    }
    impl core::fmt::Debug for Apb1enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1enr")
                .field("tim5en", &self.tim5en())
                .field("tim6en", &self.tim6en())
                .field("lptim1en", &self.lptim1en())
                .field("rtcapben", &self.rtcapben())
                .field("wwdgen", &self.wwdgen())
                .field("spi2en", &self.spi2en())
                .field("usart2en", &self.usart2en())
                .field("i2c1en", &self.i2c1en())
                .field("i2c2en", &self.i2c2en())
                .field("fmpi2c1en", &self.fmpi2c1en())
                .field("pwren", &self.pwren())
                .field("dacen", &self.dacen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1enr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb1enr {{ tim5en: {=bool:?}, tim6en: {=bool:?}, lptim1en: {=bool:?}, rtcapben: {=bool:?}, wwdgen: {=bool:?}, spi2en: {=bool:?}, usart2en: {=bool:?}, i2c1en: {=bool:?}, i2c2en: {=bool:?}, fmpi2c1en: {=bool:?}, pwren: {=bool:?}, dacen: {=bool:?} }}",
                self.tim5en(),
                self.tim6en(),
                self.lptim1en(),
                self.rtcapben(),
                self.wwdgen(),
                self.spi2en(),
                self.usart2en(),
                self.i2c1en(),
                self.i2c2en(),
                self.fmpi2c1en(),
                self.pwren(),
                self.dacen()
            )
        }
    }
    #[doc = "APB1 peripheral clock enable in low power mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1lpenr(pub u32);
    impl Apb1lpenr {
        #[doc = "TIM5 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim5lpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_tim5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim6lpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_tim6lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "LPTIM1 clock enable during sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim1lpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 clock enable during sleep mode"]
        #[inline(always)]
        pub const fn set_lptim1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "RTC APB clock enable during sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn rtcapblpen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "RTC APB clock enable during sleep mode"]
        #[inline(always)]
        pub const fn set_rtcapblpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Window watchdog clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn wwdglpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_wwdglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI2 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn spi2lpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_spi2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "USART2 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn usart2lpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_usart2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "I2C1 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1lpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_i2c1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c2lpen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_i2c2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "FMPI2C1 clock enable during Sleep"]
        #[must_use]
        #[inline(always)]
        pub const fn fmpi2c1lpen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "FMPI2C1 clock enable during Sleep"]
        #[inline(always)]
        pub const fn set_fmpi2c1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Power interface clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn pwrlpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Power interface clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_pwrlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "DAC interface clock enable during sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn daclpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC interface clock enable during sleep mode"]
        #[inline(always)]
        pub const fn set_daclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Apb1lpenr {
        #[inline(always)]
        fn default() -> Apb1lpenr {
            Apb1lpenr(0)
        }
    }
    impl core::fmt::Debug for Apb1lpenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1lpenr")
                .field("tim5lpen", &self.tim5lpen())
                .field("tim6lpen", &self.tim6lpen())
                .field("lptim1lpen", &self.lptim1lpen())
                .field("rtcapblpen", &self.rtcapblpen())
                .field("wwdglpen", &self.wwdglpen())
                .field("spi2lpen", &self.spi2lpen())
                .field("usart2lpen", &self.usart2lpen())
                .field("i2c1lpen", &self.i2c1lpen())
                .field("i2c2lpen", &self.i2c2lpen())
                .field("fmpi2c1lpen", &self.fmpi2c1lpen())
                .field("pwrlpen", &self.pwrlpen())
                .field("daclpen", &self.daclpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1lpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb1lpenr {{ tim5lpen: {=bool:?}, tim6lpen: {=bool:?}, lptim1lpen: {=bool:?}, rtcapblpen: {=bool:?}, wwdglpen: {=bool:?}, spi2lpen: {=bool:?}, usart2lpen: {=bool:?}, i2c1lpen: {=bool:?}, i2c2lpen: {=bool:?}, fmpi2c1lpen: {=bool:?}, pwrlpen: {=bool:?}, daclpen: {=bool:?} }}",
                self.tim5lpen(),
                self.tim6lpen(),
                self.lptim1lpen(),
                self.rtcapblpen(),
                self.wwdglpen(),
                self.spi2lpen(),
                self.usart2lpen(),
                self.i2c1lpen(),
                self.i2c2lpen(),
                self.fmpi2c1lpen(),
                self.pwrlpen(),
                self.daclpen()
            )
        }
    }
    #[doc = "APB1 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1rstr(pub u32);
    impl Apb1rstr {
        #[doc = "TIM5 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim5rst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 reset"]
        #[inline(always)]
        pub const fn set_tim5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim6rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 reset"]
        #[inline(always)]
        pub const fn set_tim6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "LPTIM1 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim1rst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 reset"]
        #[inline(always)]
        pub const fn set_lptim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Window watchdog reset"]
        #[must_use]
        #[inline(always)]
        pub const fn wwdgrst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog reset"]
        #[inline(always)]
        pub const fn set_wwdgrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI 2 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn spi2rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI 2 reset"]
        #[inline(always)]
        pub const fn set_spi2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "USART 2 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn usart2rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART 2 reset"]
        #[inline(always)]
        pub const fn set_usart2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "I2C 1 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C 1 reset"]
        #[inline(always)]
        pub const fn set_i2c1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C 2 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c2rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C 2 reset"]
        #[inline(always)]
        pub const fn set_i2c2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "FMPI2C1 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn fmpi2c1rst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "FMPI2C1 reset"]
        #[inline(always)]
        pub const fn set_fmpi2c1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Power interface reset"]
        #[must_use]
        #[inline(always)]
        pub const fn pwrrst(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Power interface reset"]
        #[inline(always)]
        pub const fn set_pwrrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "DAC reset"]
        #[must_use]
        #[inline(always)]
        pub const fn dacrst(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC reset"]
        #[inline(always)]
        pub const fn set_dacrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Apb1rstr {
        #[inline(always)]
        fn default() -> Apb1rstr {
            Apb1rstr(0)
        }
    }
    impl core::fmt::Debug for Apb1rstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1rstr")
                .field("tim5rst", &self.tim5rst())
                .field("tim6rst", &self.tim6rst())
                .field("lptim1rst", &self.lptim1rst())
                .field("wwdgrst", &self.wwdgrst())
                .field("spi2rst", &self.spi2rst())
                .field("usart2rst", &self.usart2rst())
                .field("i2c1rst", &self.i2c1rst())
                .field("i2c2rst", &self.i2c2rst())
                .field("fmpi2c1rst", &self.fmpi2c1rst())
                .field("pwrrst", &self.pwrrst())
                .field("dacrst", &self.dacrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb1rstr {{ tim5rst: {=bool:?}, tim6rst: {=bool:?}, lptim1rst: {=bool:?}, wwdgrst: {=bool:?}, spi2rst: {=bool:?}, usart2rst: {=bool:?}, i2c1rst: {=bool:?}, i2c2rst: {=bool:?}, fmpi2c1rst: {=bool:?}, pwrrst: {=bool:?}, dacrst: {=bool:?} }}",
                self.tim5rst(),
                self.tim6rst(),
                self.lptim1rst(),
                self.wwdgrst(),
                self.spi2rst(),
                self.usart2rst(),
                self.i2c1rst(),
                self.i2c2rst(),
                self.fmpi2c1rst(),
                self.pwrrst(),
                self.dacrst()
            )
        }
    }
    #[doc = "APB2 peripheral clock enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2enr(pub u32);
    impl Apb2enr {
        #[doc = "TIM1 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 clock enable"]
        #[inline(always)]
        pub const fn set_tim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "USART1 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn usart1en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 clock enable"]
        #[inline(always)]
        pub const fn set_usart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "USART6 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn usart6en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USART6 clock enable"]
        #[inline(always)]
        pub const fn set_usart6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ADC1 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn adc1en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1 clock enable"]
        #[inline(always)]
        pub const fn set_adc1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "SPI1 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn spi1en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 clock enable"]
        #[inline(always)]
        pub const fn set_spi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "System configuration controller clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn syscfgen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "System configuration controller clock enable"]
        #[inline(always)]
        pub const fn set_syscfgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "EXTI ans external IT clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn extiten(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI ans external IT clock enable"]
        #[inline(always)]
        pub const fn set_extiten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "TIM9 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim9en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM9 clock enable"]
        #[inline(always)]
        pub const fn set_tim9en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM11 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim11en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM11 clock enable"]
        #[inline(always)]
        pub const fn set_tim11en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SPI5 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn spi5en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SPI5 clock enable"]
        #[inline(always)]
        pub const fn set_spi5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
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
                .field("usart1en", &self.usart1en())
                .field("usart6en", &self.usart6en())
                .field("adc1en", &self.adc1en())
                .field("spi1en", &self.spi1en())
                .field("syscfgen", &self.syscfgen())
                .field("extiten", &self.extiten())
                .field("tim9en", &self.tim9en())
                .field("tim11en", &self.tim11en())
                .field("spi5en", &self.spi5en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2enr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2enr {{ tim1en: {=bool:?}, usart1en: {=bool:?}, usart6en: {=bool:?}, adc1en: {=bool:?}, spi1en: {=bool:?}, syscfgen: {=bool:?}, extiten: {=bool:?}, tim9en: {=bool:?}, tim11en: {=bool:?}, spi5en: {=bool:?} }}",
                self.tim1en(),
                self.usart1en(),
                self.usart6en(),
                self.adc1en(),
                self.spi1en(),
                self.syscfgen(),
                self.extiten(),
                self.tim9en(),
                self.tim11en(),
                self.spi5en()
            )
        }
    }
    #[doc = "APB2 peripheral clock enabled in low power mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2lpenr(pub u32);
    impl Apb2lpenr {
        #[doc = "TIM1 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim1lpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_tim1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "USART1 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn usart1lpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_usart1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "USART6 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn usart6lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USART6 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_usart6lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ADC1 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn adc1lpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_adc1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "SDIO clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn sdiolpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "SDIO clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_sdiolpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI 1 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn spi1lpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI 1 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_spi1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "System configuration controller clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn syscfglpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "System configuration controller clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_syscfglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "EXTI and External IT clock enable during sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn extitlpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "EXTI and External IT clock enable during sleep mode"]
        #[inline(always)]
        pub const fn set_extitlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "TIM9 clock enable during sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim9lpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM9 clock enable during sleep mode"]
        #[inline(always)]
        pub const fn set_tim9lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM11 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim11lpen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM11 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_tim11lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SPI5 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn spi5lpen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SPI5 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_spi5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
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
                .field("usart1lpen", &self.usart1lpen())
                .field("usart6lpen", &self.usart6lpen())
                .field("adc1lpen", &self.adc1lpen())
                .field("sdiolpen", &self.sdiolpen())
                .field("spi1lpen", &self.spi1lpen())
                .field("syscfglpen", &self.syscfglpen())
                .field("extitlpen", &self.extitlpen())
                .field("tim9lpen", &self.tim9lpen())
                .field("tim11lpen", &self.tim11lpen())
                .field("spi5lpen", &self.spi5lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2lpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2lpenr {{ tim1lpen: {=bool:?}, usart1lpen: {=bool:?}, usart6lpen: {=bool:?}, adc1lpen: {=bool:?}, sdiolpen: {=bool:?}, spi1lpen: {=bool:?}, syscfglpen: {=bool:?}, extitlpen: {=bool:?}, tim9lpen: {=bool:?}, tim11lpen: {=bool:?}, spi5lpen: {=bool:?} }}",
                self.tim1lpen(),
                self.usart1lpen(),
                self.usart6lpen(),
                self.adc1lpen(),
                self.sdiolpen(),
                self.spi1lpen(),
                self.syscfglpen(),
                self.extitlpen(),
                self.tim9lpen(),
                self.tim11lpen(),
                self.spi5lpen()
            )
        }
    }
    #[doc = "APB2 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2rstr(pub u32);
    impl Apb2rstr {
        #[doc = "TIM1 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim1rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 reset"]
        #[inline(always)]
        pub const fn set_tim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "USART1 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn usart1rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 reset"]
        #[inline(always)]
        pub const fn set_usart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "USART6 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn usart6rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USART6 reset"]
        #[inline(always)]
        pub const fn set_usart6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ADC interface reset (common to all ADCs)"]
        #[must_use]
        #[inline(always)]
        pub const fn adcrst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "ADC interface reset (common to all ADCs)"]
        #[inline(always)]
        pub const fn set_adcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "SPI 1 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn spi1rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI 1 reset"]
        #[inline(always)]
        pub const fn set_spi1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "System configuration controller reset"]
        #[must_use]
        #[inline(always)]
        pub const fn syscfgrst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "System configuration controller reset"]
        #[inline(always)]
        pub const fn set_syscfgrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TIM9 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim9rst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM9 reset"]
        #[inline(always)]
        pub const fn set_tim9rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM11 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim11rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM11 reset"]
        #[inline(always)]
        pub const fn set_tim11rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SPI5 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn spi5rst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SPI5 reset"]
        #[inline(always)]
        pub const fn set_spi5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
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
                .field("usart1rst", &self.usart1rst())
                .field("usart6rst", &self.usart6rst())
                .field("adcrst", &self.adcrst())
                .field("spi1rst", &self.spi1rst())
                .field("syscfgrst", &self.syscfgrst())
                .field("tim9rst", &self.tim9rst())
                .field("tim11rst", &self.tim11rst())
                .field("spi5rst", &self.spi5rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2rstr {{ tim1rst: {=bool:?}, usart1rst: {=bool:?}, usart6rst: {=bool:?}, adcrst: {=bool:?}, spi1rst: {=bool:?}, syscfgrst: {=bool:?}, tim9rst: {=bool:?}, tim11rst: {=bool:?}, spi5rst: {=bool:?} }}",
                self.tim1rst(),
                self.usart1rst(),
                self.usart6rst(),
                self.adcrst(),
                self.spi1rst(),
                self.syscfgrst(),
                self.tim9rst(),
                self.tim11rst(),
                self.spi5rst()
            )
        }
    }
    #[doc = "Backup domain control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdcr(pub u32);
    impl Bdcr {
        #[doc = "External low-speed oscillator enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lseon(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "External low-speed oscillator enable"]
        #[inline(always)]
        pub const fn set_lseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "External low-speed oscillator ready"]
        #[must_use]
        #[inline(always)]
        pub const fn lserdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "External low-speed oscillator ready"]
        #[inline(always)]
        pub const fn set_lserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "External low-speed oscillator bypass"]
        #[must_use]
        #[inline(always)]
        pub const fn lsebyp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "External low-speed oscillator bypass"]
        #[inline(always)]
        pub const fn set_lsebyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
        #[doc = "Backup domain software reset"]
        #[must_use]
        #[inline(always)]
        pub const fn bdrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Backup domain software reset"]
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
                .field("rtcsel", &self.rtcsel())
                .field("rtcen", &self.rtcen())
                .field("bdrst", &self.bdrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bdcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bdcr {{ lseon: {=bool:?}, lserdy: {=bool:?}, lsebyp: {=bool:?}, rtcsel: {:?}, rtcen: {=bool:?}, bdrst: {=bool:?} }}",
                self.lseon(),
                self.lserdy(),
                self.lsebyp(),
                self.rtcsel(),
                self.rtcen(),
                self.bdrst()
            )
        }
    }
    #[doc = "clock configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "System clock switch"]
        #[must_use]
        #[inline(always)]
        pub const fn sw(&self) -> super::vals::Sw {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "System clock switch"]
        #[inline(always)]
        pub const fn set_sw(&mut self, val: super::vals::Sw) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "System clock switch status"]
        #[must_use]
        #[inline(always)]
        pub const fn sws(&self) -> super::vals::Sw {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "System clock switch status"]
        #[inline(always)]
        pub const fn set_sws(&mut self, val: super::vals::Sw) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "AHB prescaler"]
        #[must_use]
        #[inline(always)]
        pub const fn hpre(&self) -> super::vals::Hpre {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::Hpre::from_bits(val as u8)
        }
        #[doc = "AHB prescaler"]
        #[inline(always)]
        pub const fn set_hpre(&mut self, val: super::vals::Hpre) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
        #[doc = "MCO output enable"]
        #[must_use]
        #[inline(always)]
        pub const fn mco1en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "MCO output enable"]
        #[inline(always)]
        pub const fn set_mco1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "MCO output enable"]
        #[must_use]
        #[inline(always)]
        pub const fn mco2en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "MCO output enable"]
        #[inline(always)]
        pub const fn set_mco2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "APB Low speed prescaler (APB1)"]
        #[must_use]
        #[inline(always)]
        pub const fn ppre1(&self) -> super::vals::Ppre {
            let val = (self.0 >> 10usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB Low speed prescaler (APB1)"]
        #[inline(always)]
        pub const fn set_ppre1(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 10usize)) | (((val.to_bits() as u32) & 0x07) << 10usize);
        }
        #[doc = "APB high-speed prescaler (APB2)"]
        #[must_use]
        #[inline(always)]
        pub const fn ppre2(&self) -> super::vals::Ppre {
            let val = (self.0 >> 13usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB high-speed prescaler (APB2)"]
        #[inline(always)]
        pub const fn set_ppre2(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val.to_bits() as u32) & 0x07) << 13usize);
        }
        #[doc = "HSE division factor for RTC clock"]
        #[must_use]
        #[inline(always)]
        pub const fn rtcpre(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "HSE division factor for RTC clock"]
        #[inline(always)]
        pub const fn set_rtcpre(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Microcontroller clock output 1"]
        #[must_use]
        #[inline(always)]
        pub const fn mco1sel(&self) -> super::vals::Mco1sel {
            let val = (self.0 >> 21usize) & 0x03;
            super::vals::Mco1sel::from_bits(val as u8)
        }
        #[doc = "Microcontroller clock output 1"]
        #[inline(always)]
        pub const fn set_mco1sel(&mut self, val: super::vals::Mco1sel) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
        }
        #[doc = "MCO1 prescaler"]
        #[must_use]
        #[inline(always)]
        pub const fn mco1pre(&self) -> super::vals::Mcopre {
            let val = (self.0 >> 24usize) & 0x07;
            super::vals::Mcopre::from_bits(val as u8)
        }
        #[doc = "MCO1 prescaler"]
        #[inline(always)]
        pub const fn set_mco1pre(&mut self, val: super::vals::Mcopre) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
        }
        #[doc = "MCO2 prescaler"]
        #[must_use]
        #[inline(always)]
        pub const fn mco2pre(&self) -> super::vals::Mcopre {
            let val = (self.0 >> 27usize) & 0x07;
            super::vals::Mcopre::from_bits(val as u8)
        }
        #[doc = "MCO2 prescaler"]
        #[inline(always)]
        pub const fn set_mco2pre(&mut self, val: super::vals::Mcopre) {
            self.0 = (self.0 & !(0x07 << 27usize)) | (((val.to_bits() as u32) & 0x07) << 27usize);
        }
        #[doc = "Microcontroller clock output 2"]
        #[must_use]
        #[inline(always)]
        pub const fn mco2sel(&self) -> super::vals::Mco2sel {
            let val = (self.0 >> 30usize) & 0x03;
            super::vals::Mco2sel::from_bits(val as u8)
        }
        #[doc = "Microcontroller clock output 2"]
        #[inline(always)]
        pub const fn set_mco2sel(&mut self, val: super::vals::Mco2sel) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
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
                .field("hpre", &self.hpre())
                .field("mco1en", &self.mco1en())
                .field("mco2en", &self.mco2en())
                .field("ppre1", &self.ppre1())
                .field("ppre2", &self.ppre2())
                .field("rtcpre", &self.rtcpre())
                .field("mco1sel", &self.mco1sel())
                .field("mco1pre", &self.mco1pre())
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
                "Cfgr {{ sw: {:?}, sws: {:?}, hpre: {:?}, mco1en: {=bool:?}, mco2en: {=bool:?}, ppre1: {:?}, ppre2: {:?}, rtcpre: {=u8:?}, mco1sel: {:?}, mco1pre: {:?}, mco2pre: {:?}, mco2sel: {:?} }}",
                self.sw(),
                self.sws(),
                self.hpre(),
                self.mco1en(),
                self.mco2en(),
                self.ppre1(),
                self.ppre2(),
                self.rtcpre(),
                self.mco1sel(),
                self.mco1pre(),
                self.mco2pre(),
                self.mco2sel()
            )
        }
    }
    #[doc = "clock interrupt register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cir(pub u32);
    impl Cir {
        #[doc = "LSI ready interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn lsirdyf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt flag"]
        #[inline(always)]
        pub const fn set_lsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn lserdyf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt flag"]
        #[inline(always)]
        pub const fn set_lserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI ready interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn hsirdyf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt flag"]
        #[inline(always)]
        pub const fn set_hsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSE ready interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn hserdyf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt flag"]
        #[inline(always)]
        pub const fn set_hserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Main PLL (PLL) ready interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn pllrdyf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Main PLL (PLL) ready interrupt flag"]
        #[inline(always)]
        pub const fn set_pllrdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Clock security system interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn cssf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Clock security system interrupt flag"]
        #[inline(always)]
        pub const fn set_cssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LSI ready interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lsirdyie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt enable"]
        #[inline(always)]
        pub const fn set_lsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LSE ready interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lserdyie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt enable"]
        #[inline(always)]
        pub const fn set_lserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "HSI ready interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn hsirdyie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt enable"]
        #[inline(always)]
        pub const fn set_hsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "HSE ready interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn hserdyie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt enable"]
        #[inline(always)]
        pub const fn set_hserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Main PLL (PLL) ready interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn pllrdyie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Main PLL (PLL) ready interrupt enable"]
        #[inline(always)]
        pub const fn set_pllrdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "LSI ready interrupt clear"]
        #[must_use]
        #[inline(always)]
        pub const fn lsirdyc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt clear"]
        #[inline(always)]
        pub const fn set_lsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "LSE ready interrupt clear"]
        #[must_use]
        #[inline(always)]
        pub const fn lserdyc(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt clear"]
        #[inline(always)]
        pub const fn set_lserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "HSI ready interrupt clear"]
        #[must_use]
        #[inline(always)]
        pub const fn hsirdyc(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt clear"]
        #[inline(always)]
        pub const fn set_hsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "HSE ready interrupt clear"]
        #[must_use]
        #[inline(always)]
        pub const fn hserdyc(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt clear"]
        #[inline(always)]
        pub const fn set_hserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Main PLL(PLL) ready interrupt clear"]
        #[must_use]
        #[inline(always)]
        pub const fn pllrdyc(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Main PLL(PLL) ready interrupt clear"]
        #[inline(always)]
        pub const fn set_pllrdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "PLLI2S ready interrupt clear"]
        #[must_use]
        #[inline(always)]
        pub const fn plli2srdyc(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "PLLI2S ready interrupt clear"]
        #[inline(always)]
        pub const fn set_plli2srdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Clock security system interrupt clear"]
        #[must_use]
        #[inline(always)]
        pub const fn cssc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Clock security system interrupt clear"]
        #[inline(always)]
        pub const fn set_cssc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Cir {
        #[inline(always)]
        fn default() -> Cir {
            Cir(0)
        }
    }
    impl core::fmt::Debug for Cir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cir")
                .field("lsirdyf", &self.lsirdyf())
                .field("lserdyf", &self.lserdyf())
                .field("hsirdyf", &self.hsirdyf())
                .field("hserdyf", &self.hserdyf())
                .field("pllrdyf", &self.pllrdyf())
                .field("cssf", &self.cssf())
                .field("lsirdyie", &self.lsirdyie())
                .field("lserdyie", &self.lserdyie())
                .field("hsirdyie", &self.hsirdyie())
                .field("hserdyie", &self.hserdyie())
                .field("pllrdyie", &self.pllrdyie())
                .field("lsirdyc", &self.lsirdyc())
                .field("lserdyc", &self.lserdyc())
                .field("hsirdyc", &self.hsirdyc())
                .field("hserdyc", &self.hserdyc())
                .field("pllrdyc", &self.pllrdyc())
                .field("plli2srdyc", &self.plli2srdyc())
                .field("cssc", &self.cssc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cir {{ lsirdyf: {=bool:?}, lserdyf: {=bool:?}, hsirdyf: {=bool:?}, hserdyf: {=bool:?}, pllrdyf: {=bool:?}, cssf: {=bool:?}, lsirdyie: {=bool:?}, lserdyie: {=bool:?}, hsirdyie: {=bool:?}, hserdyie: {=bool:?}, pllrdyie: {=bool:?}, lsirdyc: {=bool:?}, lserdyc: {=bool:?}, hsirdyc: {=bool:?}, hserdyc: {=bool:?}, pllrdyc: {=bool:?}, plli2srdyc: {=bool:?}, cssc: {=bool:?} }}",
                self.lsirdyf(),
                self.lserdyf(),
                self.hsirdyf(),
                self.hserdyf(),
                self.pllrdyf(),
                self.cssf(),
                self.lsirdyie(),
                self.lserdyie(),
                self.hsirdyie(),
                self.hserdyie(),
                self.pllrdyie(),
                self.lsirdyc(),
                self.lserdyc(),
                self.hsirdyc(),
                self.hserdyc(),
                self.pllrdyc(),
                self.plli2srdyc(),
                self.cssc()
            )
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
        #[doc = "Internal high-speed clock ready flag"]
        #[must_use]
        #[inline(always)]
        pub const fn hsirdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Internal high-speed clock ready flag"]
        #[inline(always)]
        pub const fn set_hsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Internal high-speed clock trimming"]
        #[must_use]
        #[inline(always)]
        pub const fn hsitrim(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x1f;
            val as u8
        }
        #[doc = "Internal high-speed clock trimming"]
        #[inline(always)]
        pub const fn set_hsitrim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
        }
        #[doc = "Internal high-speed clock calibration"]
        #[must_use]
        #[inline(always)]
        pub const fn hsical(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Internal high-speed clock calibration"]
        #[inline(always)]
        pub const fn set_hsical(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
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
        #[doc = "Clock security system enable"]
        #[must_use]
        #[inline(always)]
        pub const fn csson(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Clock security system enable"]
        #[inline(always)]
        pub const fn set_csson(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Main PLL (PLL) enable"]
        #[must_use]
        #[inline(always)]
        pub const fn pllon(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Main PLL (PLL) enable"]
        #[inline(always)]
        pub const fn set_pllon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Main PLL (PLL) clock ready flag"]
        #[must_use]
        #[inline(always)]
        pub const fn pllrdy(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Main PLL (PLL) clock ready flag"]
        #[inline(always)]
        pub const fn set_pllrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
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
                .field("hsitrim", &self.hsitrim())
                .field("hsical", &self.hsical())
                .field("hseon", &self.hseon())
                .field("hserdy", &self.hserdy())
                .field("hsebyp", &self.hsebyp())
                .field("csson", &self.csson())
                .field("pllon", &self.pllon())
                .field("pllrdy", &self.pllrdy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ hsion: {=bool:?}, hsirdy: {=bool:?}, hsitrim: {=u8:?}, hsical: {=u8:?}, hseon: {=bool:?}, hserdy: {=bool:?}, hsebyp: {=bool:?}, csson: {=bool:?}, pllon: {=bool:?}, pllrdy: {=bool:?} }}",
                self.hsion(),
                self.hsirdy(),
                self.hsitrim(),
                self.hsical(),
                self.hseon(),
                self.hserdy(),
                self.hsebyp(),
                self.csson(),
                self.pllon(),
                self.pllrdy()
            )
        }
    }
    #[doc = "clock control & status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "Internal low-speed oscillator enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lsion(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Internal low-speed oscillator enable"]
        #[inline(always)]
        pub const fn set_lsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Internal low-speed oscillator ready"]
        #[must_use]
        #[inline(always)]
        pub const fn lsirdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Internal low-speed oscillator ready"]
        #[inline(always)]
        pub const fn set_lsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Remove reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn rmvf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Remove reset flag"]
        #[inline(always)]
        pub const fn set_rmvf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "BOR reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn borrstf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "BOR reset flag"]
        #[inline(always)]
        pub const fn set_borrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "PIN reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn padrstf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "PIN reset flag"]
        #[inline(always)]
        pub const fn set_padrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "POR/PDR reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn porrstf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "POR/PDR reset flag"]
        #[inline(always)]
        pub const fn set_porrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Software reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn sftrstf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Software reset flag"]
        #[inline(always)]
        pub const fn set_sftrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Independent watchdog reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn wdgrstf(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog reset flag"]
        #[inline(always)]
        pub const fn set_wdgrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Window watchdog reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn wwdgrstf(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog reset flag"]
        #[inline(always)]
        pub const fn set_wwdgrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Low-power reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn lpwrrstf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power reset flag"]
        #[inline(always)]
        pub const fn set_lpwrrstf(&mut self, val: bool) {
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
                .field("lsion", &self.lsion())
                .field("lsirdy", &self.lsirdy())
                .field("rmvf", &self.rmvf())
                .field("borrstf", &self.borrstf())
                .field("padrstf", &self.padrstf())
                .field("porrstf", &self.porrstf())
                .field("sftrstf", &self.sftrstf())
                .field("wdgrstf", &self.wdgrstf())
                .field("wwdgrstf", &self.wwdgrstf())
                .field("lpwrrstf", &self.lpwrrstf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csr {{ lsion: {=bool:?}, lsirdy: {=bool:?}, rmvf: {=bool:?}, borrstf: {=bool:?}, padrstf: {=bool:?}, porrstf: {=bool:?}, sftrstf: {=bool:?}, wdgrstf: {=bool:?}, wwdgrstf: {=bool:?}, lpwrrstf: {=bool:?} }}",
                self.lsion(),
                self.lsirdy(),
                self.rmvf(),
                self.borrstf(),
                self.padrstf(),
                self.porrstf(),
                self.sftrstf(),
                self.wdgrstf(),
                self.wwdgrstf(),
                self.lpwrrstf()
            )
        }
    }
    #[doc = "DCKCFGR register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dckcfgr(pub u32);
    impl Dckcfgr {
        #[doc = "TIMPRE"]
        #[must_use]
        #[inline(always)]
        pub const fn timpre(&self) -> super::vals::Timpre {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Timpre::from_bits(val as u8)
        }
        #[doc = "TIMPRE"]
        #[inline(always)]
        pub const fn set_timpre(&mut self, val: super::vals::Timpre) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
        #[doc = "I2SSRC"]
        #[must_use]
        #[inline(always)]
        pub const fn i2ssrc(&self) -> super::vals::Issrc {
            let val = (self.0 >> 25usize) & 0x03;
            super::vals::Issrc::from_bits(val as u8)
        }
        #[doc = "I2SSRC"]
        #[inline(always)]
        pub const fn set_i2ssrc(&mut self, val: super::vals::Issrc) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val.to_bits() as u32) & 0x03) << 25usize);
        }
    }
    impl Default for Dckcfgr {
        #[inline(always)]
        fn default() -> Dckcfgr {
            Dckcfgr(0)
        }
    }
    impl core::fmt::Debug for Dckcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dckcfgr")
                .field("timpre", &self.timpre())
                .field("i2ssrc", &self.i2ssrc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dckcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dckcfgr {{ timpre: {:?}, i2ssrc: {:?} }}",
                self.timpre(),
                self.i2ssrc()
            )
        }
    }
    #[doc = "DCKCFGR2 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dckcfgr2(pub u32);
    impl Dckcfgr2 {
        #[doc = "FMPI2C1 kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn fmpi2c1sel(&self) -> super::vals::Fmpi2csel {
            let val = (self.0 >> 22usize) & 0x03;
            super::vals::Fmpi2csel::from_bits(val as u8)
        }
        #[doc = "FMPI2C1 kernel clock source selection"]
        #[inline(always)]
        pub const fn set_fmpi2c1sel(&mut self, val: super::vals::Fmpi2csel) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
        }
        #[doc = "LPTIM1SEL"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim1sel(&self) -> super::vals::Lptimsel {
            let val = (self.0 >> 30usize) & 0x03;
            super::vals::Lptimsel::from_bits(val as u8)
        }
        #[doc = "LPTIM1SEL"]
        #[inline(always)]
        pub const fn set_lptim1sel(&mut self, val: super::vals::Lptimsel) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
        }
    }
    impl Default for Dckcfgr2 {
        #[inline(always)]
        fn default() -> Dckcfgr2 {
            Dckcfgr2(0)
        }
    }
    impl core::fmt::Debug for Dckcfgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dckcfgr2")
                .field("fmpi2c1sel", &self.fmpi2c1sel())
                .field("lptim1sel", &self.lptim1sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dckcfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dckcfgr2 {{ fmpi2c1sel: {:?}, lptim1sel: {:?} }}",
                self.fmpi2c1sel(),
                self.lptim1sel()
            )
        }
    }
    #[doc = "PLL configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pllcfgr(pub u32);
    impl Pllcfgr {
        #[doc = "Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
        #[must_use]
        #[inline(always)]
        pub const fn pllm(&self) -> super::vals::Pllm {
            let val = (self.0 >> 0usize) & 0x3f;
            super::vals::Pllm::from_bits(val as u8)
        }
        #[doc = "Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
        #[inline(always)]
        pub const fn set_pllm(&mut self, val: super::vals::Pllm) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
        }
        #[doc = "Main PLL (PLL) multiplication factor for VCO"]
        #[must_use]
        #[inline(always)]
        pub const fn plln(&self) -> super::vals::Plln {
            let val = (self.0 >> 6usize) & 0x01ff;
            super::vals::Plln::from_bits(val as u16)
        }
        #[doc = "Main PLL (PLL) multiplication factor for VCO"]
        #[inline(always)]
        pub const fn set_plln(&mut self, val: super::vals::Plln) {
            self.0 = (self.0 & !(0x01ff << 6usize)) | (((val.to_bits() as u32) & 0x01ff) << 6usize);
        }
        #[doc = "Main PLL (PLL) division factor for main system clock"]
        #[must_use]
        #[inline(always)]
        pub const fn pllp(&self) -> super::vals::Pllp {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Pllp::from_bits(val as u8)
        }
        #[doc = "Main PLL (PLL) division factor for main system clock"]
        #[inline(always)]
        pub const fn set_pllp(&mut self, val: super::vals::Pllp) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Main PLL(PLL) and audio PLL (PLLI2S) entry clock source"]
        #[must_use]
        #[inline(always)]
        pub const fn pllsrc(&self) -> super::vals::Pllsrc {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Pllsrc::from_bits(val as u8)
        }
        #[doc = "Main PLL(PLL) and audio PLL (PLLI2S) entry clock source"]
        #[inline(always)]
        pub const fn set_pllsrc(&mut self, val: super::vals::Pllsrc) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
        #[must_use]
        #[inline(always)]
        pub const fn pllq(&self) -> super::vals::Pllq {
            let val = (self.0 >> 24usize) & 0x0f;
            super::vals::Pllq::from_bits(val as u8)
        }
        #[doc = "Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks"]
        #[inline(always)]
        pub const fn set_pllq(&mut self, val: super::vals::Pllq) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
        }
        #[doc = "PLL division factor for I2S and System clocks"]
        #[must_use]
        #[inline(always)]
        pub const fn pllr(&self) -> super::vals::Pllr {
            let val = (self.0 >> 28usize) & 0x07;
            super::vals::Pllr::from_bits(val as u8)
        }
        #[doc = "PLL division factor for I2S and System clocks"]
        #[inline(always)]
        pub const fn set_pllr(&mut self, val: super::vals::Pllr) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
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
                .field("pllm", &self.pllm())
                .field("plln", &self.plln())
                .field("pllp", &self.pllp())
                .field("pllsrc", &self.pllsrc())
                .field("pllq", &self.pllq())
                .field("pllr", &self.pllr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pllcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pllcfgr {{ pllm: {:?}, plln: {:?}, pllp: {:?}, pllsrc: {:?}, pllq: {:?}, pllr: {:?} }}",
                self.pllm(),
                self.plln(),
                self.pllp(),
                self.pllsrc(),
                self.pllq(),
                self.pllr()
            )
        }
    }
    #[doc = "spread spectrum clock generation register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sscgr(pub u32);
    impl Sscgr {
        #[doc = "Modulation period"]
        #[must_use]
        #[inline(always)]
        pub const fn modper(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Modulation period"]
        #[inline(always)]
        pub const fn set_modper(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "Incrementation step"]
        #[must_use]
        #[inline(always)]
        pub const fn incstep(&self) -> u16 {
            let val = (self.0 >> 13usize) & 0x7fff;
            val as u16
        }
        #[doc = "Incrementation step"]
        #[inline(always)]
        pub const fn set_incstep(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 13usize)) | (((val as u32) & 0x7fff) << 13usize);
        }
        #[doc = "Spread Select"]
        #[must_use]
        #[inline(always)]
        pub const fn spreadsel(&self) -> super::vals::Spreadsel {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Spreadsel::from_bits(val as u8)
        }
        #[doc = "Spread Select"]
        #[inline(always)]
        pub const fn set_spreadsel(&mut self, val: super::vals::Spreadsel) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Spread spectrum modulation enable"]
        #[must_use]
        #[inline(always)]
        pub const fn sscgen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Spread spectrum modulation enable"]
        #[inline(always)]
        pub const fn set_sscgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sscgr {
        #[inline(always)]
        fn default() -> Sscgr {
            Sscgr(0)
        }
    }
    impl core::fmt::Debug for Sscgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sscgr")
                .field("modper", &self.modper())
                .field("incstep", &self.incstep())
                .field("spreadsel", &self.spreadsel())
                .field("sscgen", &self.sscgen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sscgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sscgr {{ modper: {=u16:?}, incstep: {=u16:?}, spreadsel: {:?}, sscgen: {=bool:?} }}",
                self.modper(),
                self.incstep(),
                self.spreadsel(),
                self.sscgen()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fmpi2csel {
        #[doc = "APB clock selected as I2C clock"]
        Pclk1 = 0x0,
        #[doc = "System clock selected as I2C clock"]
        Sys = 0x01,
        #[doc = "HSI clock selected as I2C clock"]
        Hsi = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Fmpi2csel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fmpi2csel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fmpi2csel {
        #[inline(always)]
        fn from(val: u8) -> Fmpi2csel {
            Fmpi2csel::from_bits(val)
        }
    }
    impl From<Fmpi2csel> for u8 {
        #[inline(always)]
        fn from(val: Fmpi2csel) -> u8 {
            Fmpi2csel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hpre {
        #[doc = "SYSCLK not divided"]
        Div1 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "SYSCLK divided by 2"]
        Div2 = 0x08,
        #[doc = "SYSCLK divided by 4"]
        Div4 = 0x09,
        #[doc = "SYSCLK divided by 8"]
        Div8 = 0x0a,
        #[doc = "SYSCLK divided by 16"]
        Div16 = 0x0b,
        #[doc = "SYSCLK divided by 64"]
        Div64 = 0x0c,
        #[doc = "SYSCLK divided by 128"]
        Div128 = 0x0d,
        #[doc = "SYSCLK divided by 256"]
        Div256 = 0x0e,
        #[doc = "SYSCLK divided by 512"]
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
    pub enum Issrc {
        #[doc = "I2Sx clock frequency = f(PLLCLK_R)"]
        Pllclkr = 0x0,
        #[doc = "I2Sx clock frequency = I2S_CKIN Alternate function input frequency"]
        I2sCkin = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "I2Sx clock frequency = HSI/HSE depends on PLLSRC bit (PLLCFGR\\[22\\])"]
        HsiHse = 0x03,
    }
    impl Issrc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Issrc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Issrc {
        #[inline(always)]
        fn from(val: u8) -> Issrc {
            Issrc::from_bits(val)
        }
    }
    impl From<Issrc> for u8 {
        #[inline(always)]
        fn from(val: Issrc) -> u8 {
            Issrc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lptimsel {
        #[doc = "APB1 clock (PCLK1) selected as LPTILM1 clock"]
        Pclk1 = 0x0,
        #[doc = "LSI clock is selected as LPTILM1 clock"]
        Lsi = 0x01,
        #[doc = "HSI clock is selected as LPTILM1 clock"]
        Hsi = 0x02,
        #[doc = "LSE clock is selected as LPTILM1 clock"]
        Lse = 0x03,
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
    pub enum Mco1sel {
        #[doc = "HSI clock selected"]
        Hsi = 0x0,
        #[doc = "LSE oscillator selected"]
        Lse = 0x01,
        #[doc = "HSE oscillator clock selected"]
        Hse = 0x02,
        #[doc = "PLL clock selected"]
        Pll = 0x03,
    }
    impl Mco1sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mco1sel {
            unsafe { core::mem::transmute(val & 0x03) }
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
        #[doc = "System clock (SYSCLK) selected"]
        Sys = 0x0,
        #[doc = "PLLI2S clock selected"]
        Plli2s = 0x01,
        #[doc = "HSE oscillator clock selected"]
        Hse = 0x02,
        #[doc = "PLL clock selected"]
        Pll = 0x03,
    }
    impl Mco2sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mco2sel {
            unsafe { core::mem::transmute(val & 0x03) }
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
        #[doc = "No division"]
        Div1 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "Division by 2"]
        Div2 = 0x04,
        #[doc = "Division by 3"]
        Div3 = 0x05,
        #[doc = "Division by 4"]
        Div4 = 0x06,
        #[doc = "Division by 5"]
        Div5 = 0x07,
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
    pub enum Pllm {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Plln {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
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
        Mul50 = 0x32,
        Mul51 = 0x33,
        Mul52 = 0x34,
        Mul53 = 0x35,
        Mul54 = 0x36,
        Mul55 = 0x37,
        Mul56 = 0x38,
        Mul57 = 0x39,
        Mul58 = 0x3a,
        Mul59 = 0x3b,
        Mul60 = 0x3c,
        Mul61 = 0x3d,
        Mul62 = 0x3e,
        Mul63 = 0x3f,
        Mul64 = 0x40,
        Mul65 = 0x41,
        Mul66 = 0x42,
        Mul67 = 0x43,
        Mul68 = 0x44,
        Mul69 = 0x45,
        Mul70 = 0x46,
        Mul71 = 0x47,
        Mul72 = 0x48,
        Mul73 = 0x49,
        Mul74 = 0x4a,
        Mul75 = 0x4b,
        Mul76 = 0x4c,
        Mul77 = 0x4d,
        Mul78 = 0x4e,
        Mul79 = 0x4f,
        Mul80 = 0x50,
        Mul81 = 0x51,
        Mul82 = 0x52,
        Mul83 = 0x53,
        Mul84 = 0x54,
        Mul85 = 0x55,
        Mul86 = 0x56,
        Mul87 = 0x57,
        Mul88 = 0x58,
        Mul89 = 0x59,
        Mul90 = 0x5a,
        Mul91 = 0x5b,
        Mul92 = 0x5c,
        Mul93 = 0x5d,
        Mul94 = 0x5e,
        Mul95 = 0x5f,
        Mul96 = 0x60,
        Mul97 = 0x61,
        Mul98 = 0x62,
        Mul99 = 0x63,
        Mul100 = 0x64,
        Mul101 = 0x65,
        Mul102 = 0x66,
        Mul103 = 0x67,
        Mul104 = 0x68,
        Mul105 = 0x69,
        Mul106 = 0x6a,
        Mul107 = 0x6b,
        Mul108 = 0x6c,
        Mul109 = 0x6d,
        Mul110 = 0x6e,
        Mul111 = 0x6f,
        Mul112 = 0x70,
        Mul113 = 0x71,
        Mul114 = 0x72,
        Mul115 = 0x73,
        Mul116 = 0x74,
        Mul117 = 0x75,
        Mul118 = 0x76,
        Mul119 = 0x77,
        Mul120 = 0x78,
        Mul121 = 0x79,
        Mul122 = 0x7a,
        Mul123 = 0x7b,
        Mul124 = 0x7c,
        Mul125 = 0x7d,
        Mul126 = 0x7e,
        Mul127 = 0x7f,
        Mul128 = 0x80,
        Mul129 = 0x81,
        Mul130 = 0x82,
        Mul131 = 0x83,
        Mul132 = 0x84,
        Mul133 = 0x85,
        Mul134 = 0x86,
        Mul135 = 0x87,
        Mul136 = 0x88,
        Mul137 = 0x89,
        Mul138 = 0x8a,
        Mul139 = 0x8b,
        Mul140 = 0x8c,
        Mul141 = 0x8d,
        Mul142 = 0x8e,
        Mul143 = 0x8f,
        Mul144 = 0x90,
        Mul145 = 0x91,
        Mul146 = 0x92,
        Mul147 = 0x93,
        Mul148 = 0x94,
        Mul149 = 0x95,
        Mul150 = 0x96,
        Mul151 = 0x97,
        Mul152 = 0x98,
        Mul153 = 0x99,
        Mul154 = 0x9a,
        Mul155 = 0x9b,
        Mul156 = 0x9c,
        Mul157 = 0x9d,
        Mul158 = 0x9e,
        Mul159 = 0x9f,
        Mul160 = 0xa0,
        Mul161 = 0xa1,
        Mul162 = 0xa2,
        Mul163 = 0xa3,
        Mul164 = 0xa4,
        Mul165 = 0xa5,
        Mul166 = 0xa6,
        Mul167 = 0xa7,
        Mul168 = 0xa8,
        Mul169 = 0xa9,
        Mul170 = 0xaa,
        Mul171 = 0xab,
        Mul172 = 0xac,
        Mul173 = 0xad,
        Mul174 = 0xae,
        Mul175 = 0xaf,
        Mul176 = 0xb0,
        Mul177 = 0xb1,
        Mul178 = 0xb2,
        Mul179 = 0xb3,
        Mul180 = 0xb4,
        Mul181 = 0xb5,
        Mul182 = 0xb6,
        Mul183 = 0xb7,
        Mul184 = 0xb8,
        Mul185 = 0xb9,
        Mul186 = 0xba,
        Mul187 = 0xbb,
        Mul188 = 0xbc,
        Mul189 = 0xbd,
        Mul190 = 0xbe,
        Mul191 = 0xbf,
        Mul192 = 0xc0,
        Mul193 = 0xc1,
        Mul194 = 0xc2,
        Mul195 = 0xc3,
        Mul196 = 0xc4,
        Mul197 = 0xc5,
        Mul198 = 0xc6,
        Mul199 = 0xc7,
        Mul200 = 0xc8,
        Mul201 = 0xc9,
        Mul202 = 0xca,
        Mul203 = 0xcb,
        Mul204 = 0xcc,
        Mul205 = 0xcd,
        Mul206 = 0xce,
        Mul207 = 0xcf,
        Mul208 = 0xd0,
        Mul209 = 0xd1,
        Mul210 = 0xd2,
        Mul211 = 0xd3,
        Mul212 = 0xd4,
        Mul213 = 0xd5,
        Mul214 = 0xd6,
        Mul215 = 0xd7,
        Mul216 = 0xd8,
        Mul217 = 0xd9,
        Mul218 = 0xda,
        Mul219 = 0xdb,
        Mul220 = 0xdc,
        Mul221 = 0xdd,
        Mul222 = 0xde,
        Mul223 = 0xdf,
        Mul224 = 0xe0,
        Mul225 = 0xe1,
        Mul226 = 0xe2,
        Mul227 = 0xe3,
        Mul228 = 0xe4,
        Mul229 = 0xe5,
        Mul230 = 0xe6,
        Mul231 = 0xe7,
        Mul232 = 0xe8,
        Mul233 = 0xe9,
        Mul234 = 0xea,
        Mul235 = 0xeb,
        Mul236 = 0xec,
        Mul237 = 0xed,
        Mul238 = 0xee,
        Mul239 = 0xef,
        Mul240 = 0xf0,
        Mul241 = 0xf1,
        Mul242 = 0xf2,
        Mul243 = 0xf3,
        Mul244 = 0xf4,
        Mul245 = 0xf5,
        Mul246 = 0xf6,
        Mul247 = 0xf7,
        Mul248 = 0xf8,
        Mul249 = 0xf9,
        Mul250 = 0xfa,
        Mul251 = 0xfb,
        Mul252 = 0xfc,
        Mul253 = 0xfd,
        Mul254 = 0xfe,
        Mul255 = 0xff,
        Mul256 = 0x0100,
        Mul257 = 0x0101,
        Mul258 = 0x0102,
        Mul259 = 0x0103,
        Mul260 = 0x0104,
        Mul261 = 0x0105,
        Mul262 = 0x0106,
        Mul263 = 0x0107,
        Mul264 = 0x0108,
        Mul265 = 0x0109,
        Mul266 = 0x010a,
        Mul267 = 0x010b,
        Mul268 = 0x010c,
        Mul269 = 0x010d,
        Mul270 = 0x010e,
        Mul271 = 0x010f,
        Mul272 = 0x0110,
        Mul273 = 0x0111,
        Mul274 = 0x0112,
        Mul275 = 0x0113,
        Mul276 = 0x0114,
        Mul277 = 0x0115,
        Mul278 = 0x0116,
        Mul279 = 0x0117,
        Mul280 = 0x0118,
        Mul281 = 0x0119,
        Mul282 = 0x011a,
        Mul283 = 0x011b,
        Mul284 = 0x011c,
        Mul285 = 0x011d,
        Mul286 = 0x011e,
        Mul287 = 0x011f,
        Mul288 = 0x0120,
        Mul289 = 0x0121,
        Mul290 = 0x0122,
        Mul291 = 0x0123,
        Mul292 = 0x0124,
        Mul293 = 0x0125,
        Mul294 = 0x0126,
        Mul295 = 0x0127,
        Mul296 = 0x0128,
        Mul297 = 0x0129,
        Mul298 = 0x012a,
        Mul299 = 0x012b,
        Mul300 = 0x012c,
        Mul301 = 0x012d,
        Mul302 = 0x012e,
        Mul303 = 0x012f,
        Mul304 = 0x0130,
        Mul305 = 0x0131,
        Mul306 = 0x0132,
        Mul307 = 0x0133,
        Mul308 = 0x0134,
        Mul309 = 0x0135,
        Mul310 = 0x0136,
        Mul311 = 0x0137,
        Mul312 = 0x0138,
        Mul313 = 0x0139,
        Mul314 = 0x013a,
        Mul315 = 0x013b,
        Mul316 = 0x013c,
        Mul317 = 0x013d,
        Mul318 = 0x013e,
        Mul319 = 0x013f,
        Mul320 = 0x0140,
        Mul321 = 0x0141,
        Mul322 = 0x0142,
        Mul323 = 0x0143,
        Mul324 = 0x0144,
        Mul325 = 0x0145,
        Mul326 = 0x0146,
        Mul327 = 0x0147,
        Mul328 = 0x0148,
        Mul329 = 0x0149,
        Mul330 = 0x014a,
        Mul331 = 0x014b,
        Mul332 = 0x014c,
        Mul333 = 0x014d,
        Mul334 = 0x014e,
        Mul335 = 0x014f,
        Mul336 = 0x0150,
        Mul337 = 0x0151,
        Mul338 = 0x0152,
        Mul339 = 0x0153,
        Mul340 = 0x0154,
        Mul341 = 0x0155,
        Mul342 = 0x0156,
        Mul343 = 0x0157,
        Mul344 = 0x0158,
        Mul345 = 0x0159,
        Mul346 = 0x015a,
        Mul347 = 0x015b,
        Mul348 = 0x015c,
        Mul349 = 0x015d,
        Mul350 = 0x015e,
        Mul351 = 0x015f,
        Mul352 = 0x0160,
        Mul353 = 0x0161,
        Mul354 = 0x0162,
        Mul355 = 0x0163,
        Mul356 = 0x0164,
        Mul357 = 0x0165,
        Mul358 = 0x0166,
        Mul359 = 0x0167,
        Mul360 = 0x0168,
        Mul361 = 0x0169,
        Mul362 = 0x016a,
        Mul363 = 0x016b,
        Mul364 = 0x016c,
        Mul365 = 0x016d,
        Mul366 = 0x016e,
        Mul367 = 0x016f,
        Mul368 = 0x0170,
        Mul369 = 0x0171,
        Mul370 = 0x0172,
        Mul371 = 0x0173,
        Mul372 = 0x0174,
        Mul373 = 0x0175,
        Mul374 = 0x0176,
        Mul375 = 0x0177,
        Mul376 = 0x0178,
        Mul377 = 0x0179,
        Mul378 = 0x017a,
        Mul379 = 0x017b,
        Mul380 = 0x017c,
        Mul381 = 0x017d,
        Mul382 = 0x017e,
        Mul383 = 0x017f,
        Mul384 = 0x0180,
        Mul385 = 0x0181,
        Mul386 = 0x0182,
        Mul387 = 0x0183,
        Mul388 = 0x0184,
        Mul389 = 0x0185,
        Mul390 = 0x0186,
        Mul391 = 0x0187,
        Mul392 = 0x0188,
        Mul393 = 0x0189,
        Mul394 = 0x018a,
        Mul395 = 0x018b,
        Mul396 = 0x018c,
        Mul397 = 0x018d,
        Mul398 = 0x018e,
        Mul399 = 0x018f,
        Mul400 = 0x0190,
        Mul401 = 0x0191,
        Mul402 = 0x0192,
        Mul403 = 0x0193,
        Mul404 = 0x0194,
        Mul405 = 0x0195,
        Mul406 = 0x0196,
        Mul407 = 0x0197,
        Mul408 = 0x0198,
        Mul409 = 0x0199,
        Mul410 = 0x019a,
        Mul411 = 0x019b,
        Mul412 = 0x019c,
        Mul413 = 0x019d,
        Mul414 = 0x019e,
        Mul415 = 0x019f,
        Mul416 = 0x01a0,
        Mul417 = 0x01a1,
        Mul418 = 0x01a2,
        Mul419 = 0x01a3,
        Mul420 = 0x01a4,
        Mul421 = 0x01a5,
        Mul422 = 0x01a6,
        Mul423 = 0x01a7,
        Mul424 = 0x01a8,
        Mul425 = 0x01a9,
        Mul426 = 0x01aa,
        Mul427 = 0x01ab,
        Mul428 = 0x01ac,
        Mul429 = 0x01ad,
        Mul430 = 0x01ae,
        Mul431 = 0x01af,
        Mul432 = 0x01b0,
        _RESERVED_1b1 = 0x01b1,
        _RESERVED_1b2 = 0x01b2,
        _RESERVED_1b3 = 0x01b3,
        _RESERVED_1b4 = 0x01b4,
        _RESERVED_1b5 = 0x01b5,
        _RESERVED_1b6 = 0x01b6,
        _RESERVED_1b7 = 0x01b7,
        _RESERVED_1b8 = 0x01b8,
        _RESERVED_1b9 = 0x01b9,
        _RESERVED_1ba = 0x01ba,
        _RESERVED_1bb = 0x01bb,
        _RESERVED_1bc = 0x01bc,
        _RESERVED_1bd = 0x01bd,
        _RESERVED_1be = 0x01be,
        _RESERVED_1bf = 0x01bf,
        _RESERVED_1c0 = 0x01c0,
        _RESERVED_1c1 = 0x01c1,
        _RESERVED_1c2 = 0x01c2,
        _RESERVED_1c3 = 0x01c3,
        _RESERVED_1c4 = 0x01c4,
        _RESERVED_1c5 = 0x01c5,
        _RESERVED_1c6 = 0x01c6,
        _RESERVED_1c7 = 0x01c7,
        _RESERVED_1c8 = 0x01c8,
        _RESERVED_1c9 = 0x01c9,
        _RESERVED_1ca = 0x01ca,
        _RESERVED_1cb = 0x01cb,
        _RESERVED_1cc = 0x01cc,
        _RESERVED_1cd = 0x01cd,
        _RESERVED_1ce = 0x01ce,
        _RESERVED_1cf = 0x01cf,
        _RESERVED_1d0 = 0x01d0,
        _RESERVED_1d1 = 0x01d1,
        _RESERVED_1d2 = 0x01d2,
        _RESERVED_1d3 = 0x01d3,
        _RESERVED_1d4 = 0x01d4,
        _RESERVED_1d5 = 0x01d5,
        _RESERVED_1d6 = 0x01d6,
        _RESERVED_1d7 = 0x01d7,
        _RESERVED_1d8 = 0x01d8,
        _RESERVED_1d9 = 0x01d9,
        _RESERVED_1da = 0x01da,
        _RESERVED_1db = 0x01db,
        _RESERVED_1dc = 0x01dc,
        _RESERVED_1dd = 0x01dd,
        _RESERVED_1de = 0x01de,
        _RESERVED_1df = 0x01df,
        _RESERVED_1e0 = 0x01e0,
        _RESERVED_1e1 = 0x01e1,
        _RESERVED_1e2 = 0x01e2,
        _RESERVED_1e3 = 0x01e3,
        _RESERVED_1e4 = 0x01e4,
        _RESERVED_1e5 = 0x01e5,
        _RESERVED_1e6 = 0x01e6,
        _RESERVED_1e7 = 0x01e7,
        _RESERVED_1e8 = 0x01e8,
        _RESERVED_1e9 = 0x01e9,
        _RESERVED_1ea = 0x01ea,
        _RESERVED_1eb = 0x01eb,
        _RESERVED_1ec = 0x01ec,
        _RESERVED_1ed = 0x01ed,
        _RESERVED_1ee = 0x01ee,
        _RESERVED_1ef = 0x01ef,
        _RESERVED_1f0 = 0x01f0,
        _RESERVED_1f1 = 0x01f1,
        _RESERVED_1f2 = 0x01f2,
        _RESERVED_1f3 = 0x01f3,
        _RESERVED_1f4 = 0x01f4,
        _RESERVED_1f5 = 0x01f5,
        _RESERVED_1f6 = 0x01f6,
        _RESERVED_1f7 = 0x01f7,
        _RESERVED_1f8 = 0x01f8,
        _RESERVED_1f9 = 0x01f9,
        _RESERVED_1fa = 0x01fa,
        _RESERVED_1fb = 0x01fb,
        _RESERVED_1fc = 0x01fc,
        _RESERVED_1fd = 0x01fd,
        _RESERVED_1fe = 0x01fe,
        _RESERVED_1ff = 0x01ff,
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
    pub enum Pllp {
        #[doc = "PLLP=2"]
        Div2 = 0x0,
        #[doc = "PLLP=4"]
        Div4 = 0x01,
        #[doc = "PLLP=6"]
        Div6 = 0x02,
        #[doc = "PLLP=8"]
        Div8 = 0x03,
    }
    impl Pllp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllp {
            unsafe { core::mem::transmute(val & 0x03) }
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllq {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
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
    }
    impl Pllq {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllq {
            unsafe { core::mem::transmute(val & 0x0f) }
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllr {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        Div2 = 0x02,
        Div3 = 0x03,
        Div4 = 0x04,
        Div5 = 0x05,
        Div6 = 0x06,
        Div7 = 0x07,
    }
    impl Pllr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllr {
            unsafe { core::mem::transmute(val & 0x07) }
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllsrc {
        #[doc = "HSI clock selected as PLL and PLLI2S clock entry"]
        Hsi = 0x0,
        #[doc = "HSE oscillator clock selected as PLL and PLLI2S clock entry"]
        Hse = 0x01,
    }
    impl Pllsrc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllsrc {
            unsafe { core::mem::transmute(val & 0x01) }
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
        Div1 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "HCLK divided by 2"]
        Div2 = 0x04,
        #[doc = "HCLK divided by 4"]
        Div4 = 0x05,
        #[doc = "HCLK divided by 8"]
        Div8 = 0x06,
        #[doc = "HCLK divided by 16"]
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
    pub enum Spreadsel {
        #[doc = "Center spread"]
        Center = 0x0,
        #[doc = "Down spread"]
        Down = 0x01,
    }
    impl Spreadsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Spreadsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Spreadsel {
        #[inline(always)]
        fn from(val: u8) -> Spreadsel {
            Spreadsel::from_bits(val)
        }
    }
    impl From<Spreadsel> for u8 {
        #[inline(always)]
        fn from(val: Spreadsel) -> u8 {
            Spreadsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sw {
        #[doc = "HSI oscillator used as system clock"]
        Hsi = 0x0,
        #[doc = "HSE oscillator used as system clock"]
        Hse = 0x01,
        #[doc = "PLL used as system clock"]
        Pll1P = 0x02,
        _RESERVED_3 = 0x03,
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
    pub enum Timpre {
        #[doc = "If the APB prescaler is configured 1, TIMxCLK = PCLKx. Otherwise, TIMxCLK = 2xPCLKx"]
        Mul2 = 0x0,
        #[doc = "If the APB prescaler is configured 1, 2 or 4, TIMxCLK = HCLK. Otherwise, TIMxCLK = 4xPCLKx"]
        Mul4 = 0x01,
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
}
