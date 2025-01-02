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
    #[doc = "Clock configuration register (RCC_CFGR)"]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Clock interrupt register (RCC_CIR)"]
    #[inline(always)]
    pub const fn cir(self) -> crate::common::Reg<regs::Cir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "APB2 peripheral reset register (RCC_APB2RSTR)"]
    #[inline(always)]
    pub const fn apb2rstr(self) -> crate::common::Reg<regs::Apb2rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "APB1 peripheral reset register (RCC_APB1RSTR)"]
    #[inline(always)]
    pub const fn apb1rstr(self) -> crate::common::Reg<regs::Apb1rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "AHB Peripheral Clock enable register (RCC_AHBENR)"]
    #[inline(always)]
    pub const fn ahbenr(self) -> crate::common::Reg<regs::Ahbenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "APB2 peripheral clock enable register (RCC_APB2ENR)"]
    #[inline(always)]
    pub const fn apb2enr(self) -> crate::common::Reg<regs::Apb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "APB1 peripheral clock enable register (RCC_APB1ENR)"]
    #[inline(always)]
    pub const fn apb1enr(self) -> crate::common::Reg<regs::Apb1enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Backup domain control register (RCC_BDCR)"]
    #[inline(always)]
    pub const fn bdcr(self) -> crate::common::Reg<regs::Bdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Control/status register (RCC_CSR)"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "AHB peripheral reset register"]
    #[inline(always)]
    pub const fn ahbrstr(self) -> crate::common::Reg<regs::Ahbrstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Clock configuration register 2"]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Clock configuration register 3"]
    #[inline(always)]
    pub const fn cfgr3(self) -> crate::common::Reg<regs::Cfgr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
}
pub mod regs {
    #[doc = "AHB Peripheral Clock enable register (RCC_AHBENR)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahbenr(pub u32);
    impl Ahbenr {
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
        #[doc = "SRAM interface clock enable"]
        #[inline(always)]
        pub const fn sramen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM interface clock enable"]
        #[inline(always)]
        pub fn set_sramen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "FLASH clock enable"]
        #[inline(always)]
        pub const fn flashen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "FLASH clock enable"]
        #[inline(always)]
        pub fn set_flashen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "FMC clock enable"]
        #[inline(always)]
        pub const fn fmcen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "FMC clock enable"]
        #[inline(always)]
        pub fn set_fmcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "CRC clock enable"]
        #[inline(always)]
        pub const fn crcen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "CRC clock enable"]
        #[inline(always)]
        pub fn set_crcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "IO port H clock enable"]
        #[inline(always)]
        pub const fn gpiohen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "IO port H clock enable"]
        #[inline(always)]
        pub fn set_gpiohen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "I/O port A clock enable"]
        #[inline(always)]
        pub const fn gpioaen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port A clock enable"]
        #[inline(always)]
        pub fn set_gpioaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "I/O port B clock enable"]
        #[inline(always)]
        pub const fn gpioben(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port B clock enable"]
        #[inline(always)]
        pub fn set_gpioben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "I/O port C clock enable"]
        #[inline(always)]
        pub const fn gpiocen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port C clock enable"]
        #[inline(always)]
        pub fn set_gpiocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "I/O port D clock enable"]
        #[inline(always)]
        pub const fn gpioden(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port D clock enable"]
        #[inline(always)]
        pub fn set_gpioden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I/O port E clock enable"]
        #[inline(always)]
        pub const fn gpioeen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port E clock enable"]
        #[inline(always)]
        pub fn set_gpioeen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I/O port F clock enable"]
        #[inline(always)]
        pub const fn gpiofen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port F clock enable"]
        #[inline(always)]
        pub fn set_gpiofen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "IO port G clock enable"]
        #[inline(always)]
        pub const fn gpiogen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "IO port G clock enable"]
        #[inline(always)]
        pub fn set_gpiogen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Touch sensing controller clock enable"]
        #[inline(always)]
        pub const fn tscen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Touch sensing controller clock enable"]
        #[inline(always)]
        pub fn set_tscen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "ADC1 and ADC2 clock enable"]
        #[inline(always)]
        pub const fn adc12en(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1 and ADC2 clock enable"]
        #[inline(always)]
        pub fn set_adc12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "ADC3 and ADC4 clock enable"]
        #[inline(always)]
        pub const fn adc34en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "ADC3 and ADC4 clock enable"]
        #[inline(always)]
        pub fn set_adc34en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Ahbenr {
        #[inline(always)]
        fn default() -> Ahbenr {
            Ahbenr(0)
        }
    }
    impl core::fmt::Debug for Ahbenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahbenr")
                .field("dma1en", &self.dma1en())
                .field("dma2en", &self.dma2en())
                .field("sramen", &self.sramen())
                .field("flashen", &self.flashen())
                .field("fmcen", &self.fmcen())
                .field("crcen", &self.crcen())
                .field("gpiohen", &self.gpiohen())
                .field("gpioaen", &self.gpioaen())
                .field("gpioben", &self.gpioben())
                .field("gpiocen", &self.gpiocen())
                .field("gpioden", &self.gpioden())
                .field("gpioeen", &self.gpioeen())
                .field("gpiofen", &self.gpiofen())
                .field("gpiogen", &self.gpiogen())
                .field("tscen", &self.tscen())
                .field("adc12en", &self.adc12en())
                .field("adc34en", &self.adc34en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahbenr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahbenr {
                dma1en: bool,
                dma2en: bool,
                sramen: bool,
                flashen: bool,
                fmcen: bool,
                crcen: bool,
                gpiohen: bool,
                gpioaen: bool,
                gpioben: bool,
                gpiocen: bool,
                gpioden: bool,
                gpioeen: bool,
                gpiofen: bool,
                gpiogen: bool,
                tscen: bool,
                adc12en: bool,
                adc34en: bool,
            }
            let proxy = Ahbenr {
                dma1en: self.dma1en(),
                dma2en: self.dma2en(),
                sramen: self.sramen(),
                flashen: self.flashen(),
                fmcen: self.fmcen(),
                crcen: self.crcen(),
                gpiohen: self.gpiohen(),
                gpioaen: self.gpioaen(),
                gpioben: self.gpioben(),
                gpiocen: self.gpiocen(),
                gpioden: self.gpioden(),
                gpioeen: self.gpioeen(),
                gpiofen: self.gpiofen(),
                gpiogen: self.gpiogen(),
                tscen: self.tscen(),
                adc12en: self.adc12en(),
                adc34en: self.adc34en(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "AHB peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahbrstr(pub u32);
    impl Ahbrstr {
        #[doc = "FMC reset"]
        #[inline(always)]
        pub const fn fmcrst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "FMC reset"]
        #[inline(always)]
        pub fn set_fmcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "IO port H reset"]
        #[inline(always)]
        pub const fn gpiohrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "IO port H reset"]
        #[inline(always)]
        pub fn set_gpiohrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "I/O port A reset"]
        #[inline(always)]
        pub const fn gpioarst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port A reset"]
        #[inline(always)]
        pub fn set_gpioarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "I/O port B reset"]
        #[inline(always)]
        pub const fn gpiobrst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port B reset"]
        #[inline(always)]
        pub fn set_gpiobrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "I/O port C reset"]
        #[inline(always)]
        pub const fn gpiocrst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port C reset"]
        #[inline(always)]
        pub fn set_gpiocrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "I/O port D reset"]
        #[inline(always)]
        pub const fn gpiodrst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port D reset"]
        #[inline(always)]
        pub fn set_gpiodrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I/O port E reset"]
        #[inline(always)]
        pub const fn gpioerst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port E reset"]
        #[inline(always)]
        pub fn set_gpioerst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I/O port F reset"]
        #[inline(always)]
        pub const fn gpiofrst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I/O port F reset"]
        #[inline(always)]
        pub fn set_gpiofrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "IO port G reset"]
        #[inline(always)]
        pub const fn gpiogrst(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "IO port G reset"]
        #[inline(always)]
        pub fn set_gpiogrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Touch sensing controller reset"]
        #[inline(always)]
        pub const fn tscrst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Touch sensing controller reset"]
        #[inline(always)]
        pub fn set_tscrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "ADC1 and ADC2 reset"]
        #[inline(always)]
        pub const fn adc12rst(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1 and ADC2 reset"]
        #[inline(always)]
        pub fn set_adc12rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "ADC3 and ADC4 reset"]
        #[inline(always)]
        pub const fn adc34rst(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "ADC3 and ADC4 reset"]
        #[inline(always)]
        pub fn set_adc34rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Ahbrstr {
        #[inline(always)]
        fn default() -> Ahbrstr {
            Ahbrstr(0)
        }
    }
    impl core::fmt::Debug for Ahbrstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahbrstr")
                .field("fmcrst", &self.fmcrst())
                .field("gpiohrst", &self.gpiohrst())
                .field("gpioarst", &self.gpioarst())
                .field("gpiobrst", &self.gpiobrst())
                .field("gpiocrst", &self.gpiocrst())
                .field("gpiodrst", &self.gpiodrst())
                .field("gpioerst", &self.gpioerst())
                .field("gpiofrst", &self.gpiofrst())
                .field("gpiogrst", &self.gpiogrst())
                .field("tscrst", &self.tscrst())
                .field("adc12rst", &self.adc12rst())
                .field("adc34rst", &self.adc34rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahbrstr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ahbrstr {
                fmcrst: bool,
                gpiohrst: bool,
                gpioarst: bool,
                gpiobrst: bool,
                gpiocrst: bool,
                gpiodrst: bool,
                gpioerst: bool,
                gpiofrst: bool,
                gpiogrst: bool,
                tscrst: bool,
                adc12rst: bool,
                adc34rst: bool,
            }
            let proxy = Ahbrstr {
                fmcrst: self.fmcrst(),
                gpiohrst: self.gpiohrst(),
                gpioarst: self.gpioarst(),
                gpiobrst: self.gpiobrst(),
                gpiocrst: self.gpiocrst(),
                gpiodrst: self.gpiodrst(),
                gpioerst: self.gpioerst(),
                gpiofrst: self.gpiofrst(),
                gpiogrst: self.gpiogrst(),
                tscrst: self.tscrst(),
                adc12rst: self.adc12rst(),
                adc34rst: self.adc34rst(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "APB1 peripheral clock enable register (RCC_APB1ENR)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1enr(pub u32);
    impl Apb1enr {
        #[doc = "Timer 2 clock enable"]
        #[inline(always)]
        pub const fn tim2en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 2 clock enable"]
        #[inline(always)]
        pub fn set_tim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timer 3 clock enable"]
        #[inline(always)]
        pub const fn tim3en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 3 clock enable"]
        #[inline(always)]
        pub fn set_tim3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer 4 clock enable"]
        #[inline(always)]
        pub const fn tim4en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 4 clock enable"]
        #[inline(always)]
        pub fn set_tim4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Timer 6 clock enable"]
        #[inline(always)]
        pub const fn tim6en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 6 clock enable"]
        #[inline(always)]
        pub fn set_tim6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Timer 7 clock enable"]
        #[inline(always)]
        pub const fn tim7en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 7 clock enable"]
        #[inline(always)]
        pub fn set_tim7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
        #[doc = "SPI 2 clock enable"]
        #[inline(always)]
        pub const fn spi2en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI 2 clock enable"]
        #[inline(always)]
        pub fn set_spi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI 3 clock enable"]
        #[inline(always)]
        pub const fn spi3en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI 3 clock enable"]
        #[inline(always)]
        pub fn set_spi3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "USART 2 clock enable"]
        #[inline(always)]
        pub const fn usart2en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART 2 clock enable"]
        #[inline(always)]
        pub fn set_usart2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART 3 clock enable"]
        #[inline(always)]
        pub const fn usart3en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART 3 clock enable"]
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
        #[doc = "I2C 1 clock enable"]
        #[inline(always)]
        pub const fn i2c1en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C 1 clock enable"]
        #[inline(always)]
        pub fn set_i2c1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C 2 clock enable"]
        #[inline(always)]
        pub const fn i2c2en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C 2 clock enable"]
        #[inline(always)]
        pub fn set_i2c2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "USB clock enable"]
        #[inline(always)]
        pub const fn usben(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "USB clock enable"]
        #[inline(always)]
        pub fn set_usben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "CAN clock enable"]
        #[inline(always)]
        pub const fn canen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "CAN clock enable"]
        #[inline(always)]
        pub fn set_canen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "DAC2 interface clock enable"]
        #[inline(always)]
        pub const fn dac2en(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "DAC2 interface clock enable"]
        #[inline(always)]
        pub fn set_dac2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
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
        #[doc = "DAC interface clock enable"]
        #[inline(always)]
        pub const fn dacen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC interface clock enable"]
        #[inline(always)]
        pub fn set_dacen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "I2C3 clock enable"]
        #[inline(always)]
        pub const fn i2c3en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 clock enable"]
        #[inline(always)]
        pub fn set_i2c3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
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
                .field("tim2en", &self.tim2en())
                .field("tim3en", &self.tim3en())
                .field("tim4en", &self.tim4en())
                .field("tim6en", &self.tim6en())
                .field("tim7en", &self.tim7en())
                .field("wwdgen", &self.wwdgen())
                .field("spi2en", &self.spi2en())
                .field("spi3en", &self.spi3en())
                .field("usart2en", &self.usart2en())
                .field("usart3en", &self.usart3en())
                .field("uart4en", &self.uart4en())
                .field("uart5en", &self.uart5en())
                .field("i2c1en", &self.i2c1en())
                .field("i2c2en", &self.i2c2en())
                .field("usben", &self.usben())
                .field("canen", &self.canen())
                .field("dac2en", &self.dac2en())
                .field("pwren", &self.pwren())
                .field("dacen", &self.dacen())
                .field("i2c3en", &self.i2c3en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1enr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb1enr {
                tim2en: bool,
                tim3en: bool,
                tim4en: bool,
                tim6en: bool,
                tim7en: bool,
                wwdgen: bool,
                spi2en: bool,
                spi3en: bool,
                usart2en: bool,
                usart3en: bool,
                uart4en: bool,
                uart5en: bool,
                i2c1en: bool,
                i2c2en: bool,
                usben: bool,
                canen: bool,
                dac2en: bool,
                pwren: bool,
                dacen: bool,
                i2c3en: bool,
            }
            let proxy = Apb1enr {
                tim2en: self.tim2en(),
                tim3en: self.tim3en(),
                tim4en: self.tim4en(),
                tim6en: self.tim6en(),
                tim7en: self.tim7en(),
                wwdgen: self.wwdgen(),
                spi2en: self.spi2en(),
                spi3en: self.spi3en(),
                usart2en: self.usart2en(),
                usart3en: self.usart3en(),
                uart4en: self.uart4en(),
                uart5en: self.uart5en(),
                i2c1en: self.i2c1en(),
                i2c2en: self.i2c2en(),
                usben: self.usben(),
                canen: self.canen(),
                dac2en: self.dac2en(),
                pwren: self.pwren(),
                dacen: self.dacen(),
                i2c3en: self.i2c3en(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "APB1 peripheral reset register (RCC_APB1RSTR)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1rstr(pub u32);
    impl Apb1rstr {
        #[doc = "Timer 2 reset"]
        #[inline(always)]
        pub const fn tim2rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 2 reset"]
        #[inline(always)]
        pub fn set_tim2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Timer 3 reset"]
        #[inline(always)]
        pub const fn tim3rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 3 reset"]
        #[inline(always)]
        pub fn set_tim3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Timer 14 reset"]
        #[inline(always)]
        pub const fn tim4rst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 14 reset"]
        #[inline(always)]
        pub fn set_tim4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Timer 6 reset"]
        #[inline(always)]
        pub const fn tim6rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 6 reset"]
        #[inline(always)]
        pub fn set_tim6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Timer 7 reset"]
        #[inline(always)]
        pub const fn tim7rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Timer 7 reset"]
        #[inline(always)]
        pub fn set_tim7rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Window watchdog reset"]
        #[inline(always)]
        pub const fn wwdgrst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog reset"]
        #[inline(always)]
        pub fn set_wwdgrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
        #[doc = "USART 2 reset"]
        #[inline(always)]
        pub const fn usart2rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART 2 reset"]
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
        #[doc = "UART 4 reset"]
        #[inline(always)]
        pub const fn uart4rst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART 4 reset"]
        #[inline(always)]
        pub fn set_uart4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART 5 reset"]
        #[inline(always)]
        pub const fn uart5rst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART 5 reset"]
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
        #[doc = "USB reset"]
        #[inline(always)]
        pub const fn usbrst(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "USB reset"]
        #[inline(always)]
        pub fn set_usbrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "CAN reset"]
        #[inline(always)]
        pub const fn canrst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "CAN reset"]
        #[inline(always)]
        pub fn set_canrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "DAC2 interface reset"]
        #[inline(always)]
        pub const fn dac2rst(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "DAC2 interface reset"]
        #[inline(always)]
        pub fn set_dac2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
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
        #[doc = "DAC interface reset"]
        #[inline(always)]
        pub const fn dacrst(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC interface reset"]
        #[inline(always)]
        pub fn set_dacrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "I2C3 reset"]
        #[inline(always)]
        pub const fn i2c3rst(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 reset"]
        #[inline(always)]
        pub fn set_i2c3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
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
                .field("tim2rst", &self.tim2rst())
                .field("tim3rst", &self.tim3rst())
                .field("tim4rst", &self.tim4rst())
                .field("tim6rst", &self.tim6rst())
                .field("tim7rst", &self.tim7rst())
                .field("wwdgrst", &self.wwdgrst())
                .field("spi2rst", &self.spi2rst())
                .field("spi3rst", &self.spi3rst())
                .field("usart2rst", &self.usart2rst())
                .field("usart3rst", &self.usart3rst())
                .field("uart4rst", &self.uart4rst())
                .field("uart5rst", &self.uart5rst())
                .field("i2c1rst", &self.i2c1rst())
                .field("i2c2rst", &self.i2c2rst())
                .field("usbrst", &self.usbrst())
                .field("canrst", &self.canrst())
                .field("dac2rst", &self.dac2rst())
                .field("pwrrst", &self.pwrrst())
                .field("dacrst", &self.dacrst())
                .field("i2c3rst", &self.i2c3rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1rstr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb1rstr {
                tim2rst: bool,
                tim3rst: bool,
                tim4rst: bool,
                tim6rst: bool,
                tim7rst: bool,
                wwdgrst: bool,
                spi2rst: bool,
                spi3rst: bool,
                usart2rst: bool,
                usart3rst: bool,
                uart4rst: bool,
                uart5rst: bool,
                i2c1rst: bool,
                i2c2rst: bool,
                usbrst: bool,
                canrst: bool,
                dac2rst: bool,
                pwrrst: bool,
                dacrst: bool,
                i2c3rst: bool,
            }
            let proxy = Apb1rstr {
                tim2rst: self.tim2rst(),
                tim3rst: self.tim3rst(),
                tim4rst: self.tim4rst(),
                tim6rst: self.tim6rst(),
                tim7rst: self.tim7rst(),
                wwdgrst: self.wwdgrst(),
                spi2rst: self.spi2rst(),
                spi3rst: self.spi3rst(),
                usart2rst: self.usart2rst(),
                usart3rst: self.usart3rst(),
                uart4rst: self.uart4rst(),
                uart5rst: self.uart5rst(),
                i2c1rst: self.i2c1rst(),
                i2c2rst: self.i2c2rst(),
                usbrst: self.usbrst(),
                canrst: self.canrst(),
                dac2rst: self.dac2rst(),
                pwrrst: self.pwrrst(),
                dacrst: self.dacrst(),
                i2c3rst: self.i2c3rst(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "APB2 peripheral clock enable register (RCC_APB2ENR)"]
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
        #[doc = "TIM1 Timer clock enable"]
        #[inline(always)]
        pub const fn tim1en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 Timer clock enable"]
        #[inline(always)]
        pub fn set_tim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI 1 clock enable"]
        #[inline(always)]
        pub const fn spi1en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI 1 clock enable"]
        #[inline(always)]
        pub fn set_spi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TIM8 Timer clock enable"]
        #[inline(always)]
        pub const fn tim8en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 Timer clock enable"]
        #[inline(always)]
        pub fn set_tim8en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "USART1 clock enable"]
        #[inline(always)]
        pub const fn usart1en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 clock enable"]
        #[inline(always)]
        pub fn set_usart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI4 clock enable"]
        #[inline(always)]
        pub const fn spi4en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI4 clock enable"]
        #[inline(always)]
        pub fn set_spi4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
        #[doc = "TIM19 timer clock enable"]
        #[inline(always)]
        pub const fn tim19en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "TIM19 timer clock enable"]
        #[inline(always)]
        pub fn set_tim19en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "TIM20 timer clock enable"]
        #[inline(always)]
        pub const fn tim20en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "TIM20 timer clock enable"]
        #[inline(always)]
        pub fn set_tim20en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "MCU debug module clock enable"]
        #[inline(always)]
        pub const fn dbgmcuen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "MCU debug module clock enable"]
        #[inline(always)]
        pub fn set_dbgmcuen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "High Resolution Timer 1 clock enable"]
        #[inline(always)]
        pub const fn hrtim1en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "High Resolution Timer 1 clock enable"]
        #[inline(always)]
        pub fn set_hrtim1en(&mut self, val: bool) {
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
                .field("syscfgen", &self.syscfgen())
                .field("tim1en", &self.tim1en())
                .field("spi1en", &self.spi1en())
                .field("tim8en", &self.tim8en())
                .field("usart1en", &self.usart1en())
                .field("spi4en", &self.spi4en())
                .field("tim15en", &self.tim15en())
                .field("tim16en", &self.tim16en())
                .field("tim17en", &self.tim17en())
                .field("tim19en", &self.tim19en())
                .field("tim20en", &self.tim20en())
                .field("dbgmcuen", &self.dbgmcuen())
                .field("hrtim1en", &self.hrtim1en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2enr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb2enr {
                syscfgen: bool,
                tim1en: bool,
                spi1en: bool,
                tim8en: bool,
                usart1en: bool,
                spi4en: bool,
                tim15en: bool,
                tim16en: bool,
                tim17en: bool,
                tim19en: bool,
                tim20en: bool,
                dbgmcuen: bool,
                hrtim1en: bool,
            }
            let proxy = Apb2enr {
                syscfgen: self.syscfgen(),
                tim1en: self.tim1en(),
                spi1en: self.spi1en(),
                tim8en: self.tim8en(),
                usart1en: self.usart1en(),
                spi4en: self.spi4en(),
                tim15en: self.tim15en(),
                tim16en: self.tim16en(),
                tim17en: self.tim17en(),
                tim19en: self.tim19en(),
                tim20en: self.tim20en(),
                dbgmcuen: self.dbgmcuen(),
                hrtim1en: self.hrtim1en(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "APB2 peripheral reset register (RCC_APB2RSTR)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2rstr(pub u32);
    impl Apb2rstr {
        #[doc = "SYSCFG and COMP reset"]
        #[inline(always)]
        pub const fn syscfgrst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG and COMP reset"]
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
        #[doc = "SPI 1 reset"]
        #[inline(always)]
        pub const fn spi1rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI 1 reset"]
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
        #[doc = "SPI4 reset"]
        #[inline(always)]
        pub const fn spi4rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI4 reset"]
        #[inline(always)]
        pub fn set_spi4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
        #[doc = "TIM19 timer reset"]
        #[inline(always)]
        pub const fn tim19rst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "TIM19 timer reset"]
        #[inline(always)]
        pub fn set_tim19rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "TIM20 timer reset"]
        #[inline(always)]
        pub const fn tim20rst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "TIM20 timer reset"]
        #[inline(always)]
        pub fn set_tim20rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Debug MCU reset"]
        #[inline(always)]
        pub const fn dbgmcurst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Debug MCU reset"]
        #[inline(always)]
        pub fn set_dbgmcurst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "High Resolution Timer1 reset"]
        #[inline(always)]
        pub const fn hrtim1rst(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "High Resolution Timer1 reset"]
        #[inline(always)]
        pub fn set_hrtim1rst(&mut self, val: bool) {
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
                .field("syscfgrst", &self.syscfgrst())
                .field("tim1rst", &self.tim1rst())
                .field("spi1rst", &self.spi1rst())
                .field("tim8rst", &self.tim8rst())
                .field("usart1rst", &self.usart1rst())
                .field("spi4rst", &self.spi4rst())
                .field("tim15rst", &self.tim15rst())
                .field("tim16rst", &self.tim16rst())
                .field("tim17rst", &self.tim17rst())
                .field("tim19rst", &self.tim19rst())
                .field("tim20rst", &self.tim20rst())
                .field("dbgmcurst", &self.dbgmcurst())
                .field("hrtim1rst", &self.hrtim1rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2rstr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Apb2rstr {
                syscfgrst: bool,
                tim1rst: bool,
                spi1rst: bool,
                tim8rst: bool,
                usart1rst: bool,
                spi4rst: bool,
                tim15rst: bool,
                tim16rst: bool,
                tim17rst: bool,
                tim19rst: bool,
                tim20rst: bool,
                dbgmcurst: bool,
                hrtim1rst: bool,
            }
            let proxy = Apb2rstr {
                syscfgrst: self.syscfgrst(),
                tim1rst: self.tim1rst(),
                spi1rst: self.spi1rst(),
                tim8rst: self.tim8rst(),
                usart1rst: self.usart1rst(),
                spi4rst: self.spi4rst(),
                tim15rst: self.tim15rst(),
                tim16rst: self.tim16rst(),
                tim17rst: self.tim17rst(),
                tim19rst: self.tim19rst(),
                tim20rst: self.tim20rst(),
                dbgmcurst: self.dbgmcurst(),
                hrtim1rst: self.hrtim1rst(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Backup domain control register (RCC_BDCR)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdcr(pub u32);
    impl Bdcr {
        #[doc = "External Low Speed oscillator enable"]
        #[inline(always)]
        pub const fn lseon(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "External Low Speed oscillator enable"]
        #[inline(always)]
        pub fn set_lseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "External Low Speed oscillator ready"]
        #[inline(always)]
        pub const fn lserdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "External Low Speed oscillator ready"]
        #[inline(always)]
        pub fn set_lserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "External Low Speed oscillator bypass"]
        #[inline(always)]
        pub const fn lsebyp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "External Low Speed oscillator bypass"]
        #[inline(always)]
        pub fn set_lsebyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "LSE oscillator drive capability"]
        #[inline(always)]
        pub const fn lsedrv(&self) -> super::vals::Lsedrv {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Lsedrv::from_bits(val as u8)
        }
        #[doc = "LSE oscillator drive capability"]
        #[inline(always)]
        pub fn set_lsedrv(&mut self, val: super::vals::Lsedrv) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
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
                rtcsel: super::vals::Rtcsel,
                rtcen: bool,
                bdrst: bool,
            }
            let proxy = Bdcr {
                lseon: self.lseon(),
                lserdy: self.lserdy(),
                lsebyp: self.lsebyp(),
                lsedrv: self.lsedrv(),
                rtcsel: self.rtcsel(),
                rtcen: self.rtcen(),
                bdrst: self.bdrst(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Clock configuration register (RCC_CFGR)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "System clock Switch"]
        #[inline(always)]
        pub const fn sw(&self) -> super::vals::Sw {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "System clock Switch"]
        #[inline(always)]
        pub fn set_sw(&mut self, val: super::vals::Sw) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "System Clock Switch Status"]
        #[inline(always)]
        pub const fn sws(&self) -> super::vals::Sw {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "System Clock Switch Status"]
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
        #[doc = "APB Low speed prescaler (APB1)"]
        #[inline(always)]
        pub const fn ppre1(&self) -> super::vals::Ppre {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB Low speed prescaler (APB1)"]
        #[inline(always)]
        pub fn set_ppre1(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "APB high speed prescaler (APB2)"]
        #[inline(always)]
        pub const fn ppre2(&self) -> super::vals::Ppre {
            let val = (self.0 >> 11usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB high speed prescaler (APB2)"]
        #[inline(always)]
        pub fn set_ppre2(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 11usize)) | (((val.to_bits() as u32) & 0x07) << 11usize);
        }
        #[doc = "ADC prescaler"]
        #[inline(always)]
        pub const fn adcpre(&self) -> super::vals::Adcpre {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::Adcpre::from_bits(val as u8)
        }
        #[doc = "ADC prescaler"]
        #[inline(always)]
        pub fn set_adcpre(&mut self, val: super::vals::Adcpre) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
        #[doc = "PLL entry clock source"]
        #[inline(always)]
        pub const fn pllsrc(&self) -> super::vals::Pllsrc {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Pllsrc::from_bits(val as u8)
        }
        #[doc = "PLL entry clock source"]
        #[inline(always)]
        pub fn set_pllsrc(&mut self, val: super::vals::Pllsrc) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "HSE divider for PLL entry. Note: This bit is the same as the LSB of PREDIV in CFGR2, for compatibility with other STM32 products."]
        #[inline(always)]
        pub const fn pllxtpre(&self) -> super::vals::Pllxtpre {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Pllxtpre::from_bits(val as u8)
        }
        #[doc = "HSE divider for PLL entry. Note: This bit is the same as the LSB of PREDIV in CFGR2, for compatibility with other STM32 products."]
        #[inline(always)]
        pub fn set_pllxtpre(&mut self, val: super::vals::Pllxtpre) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "PLL Multiplication Factor"]
        #[inline(always)]
        pub const fn pllmul(&self) -> super::vals::Pllmul {
            let val = (self.0 >> 18usize) & 0x0f;
            super::vals::Pllmul::from_bits(val as u8)
        }
        #[doc = "PLL Multiplication Factor"]
        #[inline(always)]
        pub fn set_pllmul(&mut self, val: super::vals::Pllmul) {
            self.0 = (self.0 & !(0x0f << 18usize)) | (((val.to_bits() as u32) & 0x0f) << 18usize);
        }
        #[doc = "USB prescaler"]
        #[inline(always)]
        pub const fn usbpre(&self) -> super::vals::Usbpre {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Usbpre::from_bits(val as u8)
        }
        #[doc = "USB prescaler"]
        #[inline(always)]
        pub fn set_usbpre(&mut self, val: super::vals::Usbpre) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "I2S external clock source selection"]
        #[inline(always)]
        pub const fn i2ssrc(&self) -> super::vals::Issrc {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Issrc::from_bits(val as u8)
        }
        #[doc = "I2S external clock source selection"]
        #[inline(always)]
        pub fn set_i2ssrc(&mut self, val: super::vals::Issrc) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "Microcontroller clock output"]
        #[inline(always)]
        pub const fn mcosel(&self) -> super::vals::Mcosel {
            let val = (self.0 >> 24usize) & 0x07;
            super::vals::Mcosel::from_bits(val as u8)
        }
        #[doc = "Microcontroller clock output"]
        #[inline(always)]
        pub fn set_mcosel(&mut self, val: super::vals::Mcosel) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
        }
        #[doc = "Microcontroller Clock Output Prescaler"]
        #[inline(always)]
        pub const fn mcopre(&self) -> super::vals::Mcopre {
            let val = (self.0 >> 28usize) & 0x07;
            super::vals::Mcopre::from_bits(val as u8)
        }
        #[doc = "Microcontroller Clock Output Prescaler"]
        #[inline(always)]
        pub fn set_mcopre(&mut self, val: super::vals::Mcopre) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
        }
        #[doc = "Do not divide PLL to MCO"]
        #[inline(always)]
        pub const fn pllmcodiv(&self) -> super::vals::Pllmcodiv {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Pllmcodiv::from_bits(val as u8)
        }
        #[doc = "Do not divide PLL to MCO"]
        #[inline(always)]
        pub fn set_pllmcodiv(&mut self, val: super::vals::Pllmcodiv) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
                .field("ppre1", &self.ppre1())
                .field("ppre2", &self.ppre2())
                .field("adcpre", &self.adcpre())
                .field("pllsrc", &self.pllsrc())
                .field("pllxtpre", &self.pllxtpre())
                .field("pllmul", &self.pllmul())
                .field("usbpre", &self.usbpre())
                .field("i2ssrc", &self.i2ssrc())
                .field("mcosel", &self.mcosel())
                .field("mcopre", &self.mcopre())
                .field("pllmcodiv", &self.pllmcodiv())
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
                hpre: super::vals::Hpre,
                ppre1: super::vals::Ppre,
                ppre2: super::vals::Ppre,
                adcpre: super::vals::Adcpre,
                pllsrc: super::vals::Pllsrc,
                pllxtpre: super::vals::Pllxtpre,
                pllmul: super::vals::Pllmul,
                usbpre: super::vals::Usbpre,
                i2ssrc: super::vals::Issrc,
                mcosel: super::vals::Mcosel,
                mcopre: super::vals::Mcopre,
                pllmcodiv: super::vals::Pllmcodiv,
            }
            let proxy = Cfgr {
                sw: self.sw(),
                sws: self.sws(),
                hpre: self.hpre(),
                ppre1: self.ppre1(),
                ppre2: self.ppre2(),
                adcpre: self.adcpre(),
                pllsrc: self.pllsrc(),
                pllxtpre: self.pllxtpre(),
                pllmul: self.pllmul(),
                usbpre: self.usbpre(),
                i2ssrc: self.i2ssrc(),
                mcosel: self.mcosel(),
                mcopre: self.mcopre(),
                pllmcodiv: self.pllmcodiv(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Clock configuration register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "PREDIV division factor"]
        #[inline(always)]
        pub const fn prediv(&self) -> super::vals::Prediv {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Prediv::from_bits(val as u8)
        }
        #[doc = "PREDIV division factor"]
        #[inline(always)]
        pub fn set_prediv(&mut self, val: super::vals::Prediv) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "ADC1 and ADC2 prescaler"]
        #[inline(always)]
        pub const fn adc12pres(&self) -> super::vals::Adcpres {
            let val = (self.0 >> 4usize) & 0x1f;
            super::vals::Adcpres::from_bits(val as u8)
        }
        #[doc = "ADC1 and ADC2 prescaler"]
        #[inline(always)]
        pub fn set_adc12pres(&mut self, val: super::vals::Adcpres) {
            self.0 = (self.0 & !(0x1f << 4usize)) | (((val.to_bits() as u32) & 0x1f) << 4usize);
        }
        #[doc = "ADC1 prescaler"]
        #[inline(always)]
        pub const fn adc1pres(&self) -> super::vals::Adcpres {
            let val = (self.0 >> 4usize) & 0x1f;
            super::vals::Adcpres::from_bits(val as u8)
        }
        #[doc = "ADC1 prescaler"]
        #[inline(always)]
        pub fn set_adc1pres(&mut self, val: super::vals::Adcpres) {
            self.0 = (self.0 & !(0x1f << 4usize)) | (((val.to_bits() as u32) & 0x1f) << 4usize);
        }
        #[doc = "ADC3 and ADC4 prescaler"]
        #[inline(always)]
        pub const fn adc34pres(&self) -> super::vals::Adcpres {
            let val = (self.0 >> 9usize) & 0x1f;
            super::vals::Adcpres::from_bits(val as u8)
        }
        #[doc = "ADC3 and ADC4 prescaler"]
        #[inline(always)]
        pub fn set_adc34pres(&mut self, val: super::vals::Adcpres) {
            self.0 = (self.0 & !(0x1f << 9usize)) | (((val.to_bits() as u32) & 0x1f) << 9usize);
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
                .field("prediv", &self.prediv())
                .field("adc12pres", &self.adc12pres())
                .field("adc1pres", &self.adc1pres())
                .field("adc34pres", &self.adc34pres())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cfgr2 {
                prediv: super::vals::Prediv,
                adc12pres: super::vals::Adcpres,
                adc1pres: super::vals::Adcpres,
                adc34pres: super::vals::Adcpres,
            }
            let proxy = Cfgr2 {
                prediv: self.prediv(),
                adc12pres: self.adc12pres(),
                adc1pres: self.adc1pres(),
                adc34pres: self.adc34pres(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Clock configuration register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr3(pub u32);
    impl Cfgr3 {
        #[doc = "USART1 clock source selection"]
        #[inline(always)]
        pub const fn usart1sw(&self) -> super::vals::Usart1sw {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Usart1sw::from_bits(val as u8)
        }
        #[doc = "USART1 clock source selection"]
        #[inline(always)]
        pub fn set_usart1sw(&mut self, val: super::vals::Usart1sw) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "I2C1 clock source selection"]
        #[inline(always)]
        pub const fn i2c1sw(&self) -> super::vals::Icsw {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Icsw::from_bits(val as u8)
        }
        #[doc = "I2C1 clock source selection"]
        #[inline(always)]
        pub fn set_i2c1sw(&mut self, val: super::vals::Icsw) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "I2C2 clock source selection"]
        #[inline(always)]
        pub const fn i2c2sw(&self) -> super::vals::Icsw {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Icsw::from_bits(val as u8)
        }
        #[doc = "I2C2 clock source selection"]
        #[inline(always)]
        pub fn set_i2c2sw(&mut self, val: super::vals::Icsw) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "HDMI CEC clock source selection"]
        #[inline(always)]
        pub const fn cecsw(&self) -> super::vals::Cecsw {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Cecsw::from_bits(val as u8)
        }
        #[doc = "HDMI CEC clock source selection"]
        #[inline(always)]
        pub fn set_cecsw(&mut self, val: super::vals::Cecsw) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "I2C3 clock source selection"]
        #[inline(always)]
        pub const fn i2c3sw(&self) -> super::vals::Icsw {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Icsw::from_bits(val as u8)
        }
        #[doc = "I2C3 clock source selection"]
        #[inline(always)]
        pub fn set_i2c3sw(&mut self, val: super::vals::Icsw) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "Timer1 clock source selection"]
        #[inline(always)]
        pub const fn tim1sw(&self) -> super::vals::Timsw {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Timsw::from_bits(val as u8)
        }
        #[doc = "Timer1 clock source selection"]
        #[inline(always)]
        pub fn set_tim1sw(&mut self, val: super::vals::Timsw) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "Timer8 clock source selection"]
        #[inline(always)]
        pub const fn tim8sw(&self) -> super::vals::Timsw {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Timsw::from_bits(val as u8)
        }
        #[doc = "Timer8 clock source selection"]
        #[inline(always)]
        pub fn set_tim8sw(&mut self, val: super::vals::Timsw) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "Timer15 clock source selection"]
        #[inline(always)]
        pub const fn tim15sw(&self) -> super::vals::Timsw {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Timsw::from_bits(val as u8)
        }
        #[doc = "Timer15 clock source selection"]
        #[inline(always)]
        pub fn set_tim15sw(&mut self, val: super::vals::Timsw) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "Timer16 clock source selection"]
        #[inline(always)]
        pub const fn tim16sw(&self) -> super::vals::Timsw {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Timsw::from_bits(val as u8)
        }
        #[doc = "Timer16 clock source selection"]
        #[inline(always)]
        pub fn set_tim16sw(&mut self, val: super::vals::Timsw) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Hrtim1 clock source selection"]
        #[inline(always)]
        pub const fn hrtim1sw(&self) -> super::vals::Timsw {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Timsw::from_bits(val as u8)
        }
        #[doc = "Hrtim1 clock source selection"]
        #[inline(always)]
        pub fn set_hrtim1sw(&mut self, val: super::vals::Timsw) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "Timer17 clock source selection"]
        #[inline(always)]
        pub const fn tim17sw(&self) -> super::vals::Timsw {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::Timsw::from_bits(val as u8)
        }
        #[doc = "Timer17 clock source selection"]
        #[inline(always)]
        pub fn set_tim17sw(&mut self, val: super::vals::Timsw) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
        #[doc = "Timer20 clock source selection"]
        #[inline(always)]
        pub const fn tim20sw(&self) -> super::vals::Timsw {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Timsw::from_bits(val as u8)
        }
        #[doc = "Timer20 clock source selection"]
        #[inline(always)]
        pub fn set_tim20sw(&mut self, val: super::vals::Timsw) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "USART2 clock source selection"]
        #[inline(always)]
        pub const fn usart2sw(&self) -> super::vals::Usartsw {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Usartsw::from_bits(val as u8)
        }
        #[doc = "USART2 clock source selection"]
        #[inline(always)]
        pub fn set_usart2sw(&mut self, val: super::vals::Usartsw) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "USART3 clock source selection"]
        #[inline(always)]
        pub const fn usart3sw(&self) -> super::vals::Usartsw {
            let val = (self.0 >> 18usize) & 0x03;
            super::vals::Usartsw::from_bits(val as u8)
        }
        #[doc = "USART3 clock source selection"]
        #[inline(always)]
        pub fn set_usart3sw(&mut self, val: super::vals::Usartsw) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
        }
        #[doc = "UART4 clock source selection"]
        #[inline(always)]
        pub const fn uart4sw(&self) -> super::vals::Usartsw {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::Usartsw::from_bits(val as u8)
        }
        #[doc = "UART4 clock source selection"]
        #[inline(always)]
        pub fn set_uart4sw(&mut self, val: super::vals::Usartsw) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "UART5 clock source selection"]
        #[inline(always)]
        pub const fn uart5sw(&self) -> super::vals::Usartsw {
            let val = (self.0 >> 22usize) & 0x03;
            super::vals::Usartsw::from_bits(val as u8)
        }
        #[doc = "UART5 clock source selection"]
        #[inline(always)]
        pub fn set_uart5sw(&mut self, val: super::vals::Usartsw) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
        }
        #[doc = "Timer2 clock source selection"]
        #[inline(always)]
        pub const fn tim2sw(&self) -> super::vals::Tim2sw {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Tim2sw::from_bits(val as u8)
        }
        #[doc = "Timer2 clock source selection"]
        #[inline(always)]
        pub fn set_tim2sw(&mut self, val: super::vals::Tim2sw) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
        #[doc = "Timer34 clock source selection"]
        #[inline(always)]
        pub const fn tim34sw(&self) -> super::vals::Timsw {
            let val = (self.0 >> 25usize) & 0x01;
            super::vals::Timsw::from_bits(val as u8)
        }
        #[doc = "Timer34 clock source selection"]
        #[inline(always)]
        pub fn set_tim34sw(&mut self, val: super::vals::Timsw) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
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
                .field("usart1sw", &self.usart1sw())
                .field("i2c1sw", &self.i2c1sw())
                .field("i2c2sw", &self.i2c2sw())
                .field("cecsw", &self.cecsw())
                .field("i2c3sw", &self.i2c3sw())
                .field("tim1sw", &self.tim1sw())
                .field("tim8sw", &self.tim8sw())
                .field("tim15sw", &self.tim15sw())
                .field("tim16sw", &self.tim16sw())
                .field("hrtim1sw", &self.hrtim1sw())
                .field("tim17sw", &self.tim17sw())
                .field("tim20sw", &self.tim20sw())
                .field("usart2sw", &self.usart2sw())
                .field("usart3sw", &self.usart3sw())
                .field("uart4sw", &self.uart4sw())
                .field("uart5sw", &self.uart5sw())
                .field("tim2sw", &self.tim2sw())
                .field("tim34sw", &self.tim34sw())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr3 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cfgr3 {
                usart1sw: super::vals::Usart1sw,
                i2c1sw: super::vals::Icsw,
                i2c2sw: super::vals::Icsw,
                cecsw: super::vals::Cecsw,
                i2c3sw: super::vals::Icsw,
                tim1sw: super::vals::Timsw,
                tim8sw: super::vals::Timsw,
                tim15sw: super::vals::Timsw,
                tim16sw: super::vals::Timsw,
                hrtim1sw: super::vals::Timsw,
                tim17sw: super::vals::Timsw,
                tim20sw: super::vals::Timsw,
                usart2sw: super::vals::Usartsw,
                usart3sw: super::vals::Usartsw,
                uart4sw: super::vals::Usartsw,
                uart5sw: super::vals::Usartsw,
                tim2sw: super::vals::Tim2sw,
                tim34sw: super::vals::Timsw,
            }
            let proxy = Cfgr3 {
                usart1sw: self.usart1sw(),
                i2c1sw: self.i2c1sw(),
                i2c2sw: self.i2c2sw(),
                cecsw: self.cecsw(),
                i2c3sw: self.i2c3sw(),
                tim1sw: self.tim1sw(),
                tim8sw: self.tim8sw(),
                tim15sw: self.tim15sw(),
                tim16sw: self.tim16sw(),
                hrtim1sw: self.hrtim1sw(),
                tim17sw: self.tim17sw(),
                tim20sw: self.tim20sw(),
                usart2sw: self.usart2sw(),
                usart3sw: self.usart3sw(),
                uart4sw: self.uart4sw(),
                uart5sw: self.uart5sw(),
                tim2sw: self.tim2sw(),
                tim34sw: self.tim34sw(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Clock interrupt register (RCC_CIR)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cir(pub u32);
    impl Cir {
        #[doc = "LSI Ready Interrupt flag"]
        #[inline(always)]
        pub const fn lsirdyf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI Ready Interrupt flag"]
        #[inline(always)]
        pub fn set_lsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE Ready Interrupt flag"]
        #[inline(always)]
        pub const fn lserdyf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE Ready Interrupt flag"]
        #[inline(always)]
        pub fn set_lserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI Ready Interrupt flag"]
        #[inline(always)]
        pub const fn hsirdyf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSI Ready Interrupt flag"]
        #[inline(always)]
        pub fn set_hsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSE Ready Interrupt flag"]
        #[inline(always)]
        pub const fn hserdyf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSE Ready Interrupt flag"]
        #[inline(always)]
        pub fn set_hserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "PLL Ready Interrupt flag"]
        #[inline(always)]
        pub const fn pllrdyf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PLL Ready Interrupt flag"]
        #[inline(always)]
        pub fn set_pllrdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Clock Security System Interrupt flag"]
        #[inline(always)]
        pub const fn cssf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Clock Security System Interrupt flag"]
        #[inline(always)]
        pub fn set_cssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LSI Ready Interrupt Enable"]
        #[inline(always)]
        pub const fn lsirdyie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "LSI Ready Interrupt Enable"]
        #[inline(always)]
        pub fn set_lsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LSE Ready Interrupt Enable"]
        #[inline(always)]
        pub const fn lserdyie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LSE Ready Interrupt Enable"]
        #[inline(always)]
        pub fn set_lserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "HSI Ready Interrupt Enable"]
        #[inline(always)]
        pub const fn hsirdyie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HSI Ready Interrupt Enable"]
        #[inline(always)]
        pub fn set_hsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "HSE Ready Interrupt Enable"]
        #[inline(always)]
        pub const fn hserdyie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "HSE Ready Interrupt Enable"]
        #[inline(always)]
        pub fn set_hserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "PLL Ready Interrupt Enable"]
        #[inline(always)]
        pub const fn pllrdyie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "PLL Ready Interrupt Enable"]
        #[inline(always)]
        pub fn set_pllrdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "LSI Ready Interrupt Clear"]
        #[inline(always)]
        pub const fn lsirdyc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "LSI Ready Interrupt Clear"]
        #[inline(always)]
        pub fn set_lsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "LSE Ready Interrupt Clear"]
        #[inline(always)]
        pub const fn lserdyc(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "LSE Ready Interrupt Clear"]
        #[inline(always)]
        pub fn set_lserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "HSI Ready Interrupt Clear"]
        #[inline(always)]
        pub const fn hsirdyc(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "HSI Ready Interrupt Clear"]
        #[inline(always)]
        pub fn set_hsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "HSE Ready Interrupt Clear"]
        #[inline(always)]
        pub const fn hserdyc(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "HSE Ready Interrupt Clear"]
        #[inline(always)]
        pub fn set_hserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "PLL Ready Interrupt Clear"]
        #[inline(always)]
        pub const fn pllrdyc(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "PLL Ready Interrupt Clear"]
        #[inline(always)]
        pub fn set_pllrdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Clock security system interrupt clear"]
        #[inline(always)]
        pub const fn cssc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Clock security system interrupt clear"]
        #[inline(always)]
        pub fn set_cssc(&mut self, val: bool) {
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
                .field("cssc", &self.cssc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cir {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cir {
                lsirdyf: bool,
                lserdyf: bool,
                hsirdyf: bool,
                hserdyf: bool,
                pllrdyf: bool,
                cssf: bool,
                lsirdyie: bool,
                lserdyie: bool,
                hsirdyie: bool,
                hserdyie: bool,
                pllrdyie: bool,
                lsirdyc: bool,
                lserdyc: bool,
                hsirdyc: bool,
                hserdyc: bool,
                pllrdyc: bool,
                cssc: bool,
            }
            let proxy = Cir {
                lsirdyf: self.lsirdyf(),
                lserdyf: self.lserdyf(),
                hsirdyf: self.hsirdyf(),
                hserdyf: self.hserdyf(),
                pllrdyf: self.pllrdyf(),
                cssf: self.cssf(),
                lsirdyie: self.lsirdyie(),
                lserdyie: self.lserdyie(),
                hsirdyie: self.hsirdyie(),
                hserdyie: self.hserdyie(),
                pllrdyie: self.pllrdyie(),
                lsirdyc: self.lsirdyc(),
                lserdyc: self.lserdyc(),
                hsirdyc: self.hsirdyc(),
                hserdyc: self.hserdyc(),
                pllrdyc: self.pllrdyc(),
                cssc: self.cssc(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Clock control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Internal High Speed clock enable"]
        #[inline(always)]
        pub const fn hsion(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Internal High Speed clock enable"]
        #[inline(always)]
        pub fn set_hsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Internal High Speed clock ready flag"]
        #[inline(always)]
        pub const fn hsirdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Internal High Speed clock ready flag"]
        #[inline(always)]
        pub fn set_hsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Internal High Speed clock trimming"]
        #[inline(always)]
        pub const fn hsitrim(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x1f;
            val as u8
        }
        #[doc = "Internal High Speed clock trimming"]
        #[inline(always)]
        pub fn set_hsitrim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
        }
        #[doc = "Internal High Speed clock Calibration"]
        #[inline(always)]
        pub const fn hsical(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Internal High Speed clock Calibration"]
        #[inline(always)]
        pub fn set_hsical(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "External High Speed clock enable"]
        #[inline(always)]
        pub const fn hseon(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "External High Speed clock enable"]
        #[inline(always)]
        pub fn set_hseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "External High Speed clock ready flag"]
        #[inline(always)]
        pub const fn hserdy(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "External High Speed clock ready flag"]
        #[inline(always)]
        pub fn set_hserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "External High Speed clock Bypass"]
        #[inline(always)]
        pub const fn hsebyp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "External High Speed clock Bypass"]
        #[inline(always)]
        pub fn set_hsebyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Clock Security System enable"]
        #[inline(always)]
        pub const fn csson(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Clock Security System enable"]
        #[inline(always)]
        pub fn set_csson(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "PLL enable"]
        #[inline(always)]
        pub const fn pllon(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "PLL enable"]
        #[inline(always)]
        pub fn set_pllon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "PLL clock ready flag"]
        #[inline(always)]
        pub const fn pllrdy(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "PLL clock ready flag"]
        #[inline(always)]
        pub fn set_pllrdy(&mut self, val: bool) {
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
            #[derive(defmt :: Format)]
            struct Cr {
                hsion: bool,
                hsirdy: bool,
                hsitrim: u8,
                hsical: u8,
                hseon: bool,
                hserdy: bool,
                hsebyp: bool,
                csson: bool,
                pllon: bool,
                pllrdy: bool,
            }
            let proxy = Cr {
                hsion: self.hsion(),
                hsirdy: self.hsirdy(),
                hsitrim: self.hsitrim(),
                hsical: self.hsical(),
                hseon: self.hseon(),
                hserdy: self.hserdy(),
                hsebyp: self.hsebyp(),
                csson: self.csson(),
                pllon: self.pllon(),
                pllrdy: self.pllrdy(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Control/status register (RCC_CSR)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "Internal low speed oscillator enable"]
        #[inline(always)]
        pub const fn lsion(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Internal low speed oscillator enable"]
        #[inline(always)]
        pub fn set_lsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Internal low speed oscillator ready"]
        #[inline(always)]
        pub const fn lsirdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Internal low speed oscillator ready"]
        #[inline(always)]
        pub fn set_lsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Reset flag of the 1.8 V domain"]
        #[inline(always)]
        pub const fn v18pwrrstf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Reset flag of the 1.8 V domain"]
        #[inline(always)]
        pub fn set_v18pwrrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Remove reset flag"]
        #[inline(always)]
        pub const fn rmvf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Remove reset flag"]
        #[inline(always)]
        pub fn set_rmvf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
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
        #[doc = "PIN reset flag"]
        #[inline(always)]
        pub const fn pinrstf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "PIN reset flag"]
        #[inline(always)]
        pub fn set_pinrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "POR/PDR reset flag"]
        #[inline(always)]
        pub const fn porrstf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "POR/PDR reset flag"]
        #[inline(always)]
        pub fn set_porrstf(&mut self, val: bool) {
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
        #[doc = "Independent watchdog reset flag"]
        #[inline(always)]
        pub const fn iwdgrstf(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog reset flag"]
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
    impl core::fmt::Debug for Csr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csr")
                .field("lsion", &self.lsion())
                .field("lsirdy", &self.lsirdy())
                .field("v18pwrrstf", &self.v18pwrrstf())
                .field("rmvf", &self.rmvf())
                .field("oblrstf", &self.oblrstf())
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
    impl defmt::Format for Csr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Csr {
                lsion: bool,
                lsirdy: bool,
                v18pwrrstf: bool,
                rmvf: bool,
                oblrstf: bool,
                pinrstf: bool,
                porrstf: bool,
                sftrstf: bool,
                iwdgrstf: bool,
                wwdgrstf: bool,
                lpwrrstf: bool,
            }
            let proxy = Csr {
                lsion: self.lsion(),
                lsirdy: self.lsirdy(),
                v18pwrrstf: self.v18pwrrstf(),
                rmvf: self.rmvf(),
                oblrstf: self.oblrstf(),
                pinrstf: self.pinrstf(),
                porrstf: self.porrstf(),
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
    pub enum Adcpre {
        #[doc = "PCLK divided by 2"]
        DIV2 = 0x0,
        #[doc = "PCLK divided by 4"]
        DIV4 = 0x01,
        #[doc = "PCLK divided by 6"]
        DIV6 = 0x02,
        #[doc = "PCLK divided by 8"]
        DIV8 = 0x03,
    }
    impl Adcpre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adcpre {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adcpre {
        #[inline(always)]
        fn from(val: u8) -> Adcpre {
            Adcpre::from_bits(val)
        }
    }
    impl From<Adcpre> for u8 {
        #[inline(always)]
        fn from(val: Adcpre) -> u8 {
            Adcpre::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adcpres {
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
        #[doc = "PLL clock not divided"]
        DIV1 = 0x10,
        #[doc = "PLL clock divided by 2"]
        DIV2 = 0x11,
        #[doc = "PLL clock divided by 4"]
        DIV4 = 0x12,
        #[doc = "PLL clock divided by 6"]
        DIV6 = 0x13,
        #[doc = "PLL clock divided by 8"]
        DIV8 = 0x14,
        #[doc = "PLL clock divided by 10"]
        DIV10 = 0x15,
        #[doc = "PLL clock divided by 12"]
        DIV12 = 0x16,
        #[doc = "PLL clock divided by 16"]
        DIV16 = 0x17,
        #[doc = "PLL clock divided by 32"]
        DIV32 = 0x18,
        #[doc = "PLL clock divided by 64"]
        DIV64 = 0x19,
        #[doc = "PLL clock divided by 128"]
        DIV128 = 0x1a,
        #[doc = "PLL clock divided by 256"]
        DIV256 = 0x1b,
        _RESERVED_1c = 0x1c,
        _RESERVED_1d = 0x1d,
        _RESERVED_1e = 0x1e,
        _RESERVED_1f = 0x1f,
    }
    impl Adcpres {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adcpres {
            unsafe { core::mem::transmute(val & 0x1f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adcpres {
        #[inline(always)]
        fn from(val: u8) -> Adcpres {
            Adcpres::from_bits(val)
        }
    }
    impl From<Adcpres> for u8 {
        #[inline(always)]
        fn from(val: Adcpres) -> u8 {
            Adcpres::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cecsw {
        #[doc = "HSI clock divided by 244 selected as CEC clock source"]
        HSI_DIV_244 = 0x0,
        #[doc = "LSE clock selected as CEC clock source"]
        LSE = 0x01,
    }
    impl Cecsw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cecsw {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cecsw {
        #[inline(always)]
        fn from(val: u8) -> Cecsw {
            Cecsw::from_bits(val)
        }
    }
    impl From<Cecsw> for u8 {
        #[inline(always)]
        fn from(val: Cecsw) -> u8 {
            Cecsw::to_bits(val)
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
    pub enum Icsw {
        #[doc = "HSI clock selected as I2C clock source"]
        HSI = 0x0,
        #[doc = "SYSCLK clock selected as I2C clock source"]
        SYS = 0x01,
    }
    impl Icsw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Icsw {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Icsw {
        #[inline(always)]
        fn from(val: u8) -> Icsw {
            Icsw::from_bits(val)
        }
    }
    impl From<Icsw> for u8 {
        #[inline(always)]
        fn from(val: Icsw) -> u8 {
            Icsw::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Issrc {
        #[doc = "System clock used as I2S clock source"]
        SYS = 0x0,
        #[doc = "External clock mapped on the I2S_CKIN pin used as I2S clock source"]
        CKIN = 0x01,
    }
    impl Issrc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Issrc {
            unsafe { core::mem::transmute(val & 0x01) }
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
    pub enum Mcopre {
        #[doc = "MCO is divided by 1"]
        DIV1 = 0x0,
        #[doc = "MCO is divided by 2"]
        DIV2 = 0x01,
        #[doc = "MCO is divided by 4"]
        DIV4 = 0x02,
        #[doc = "MCO is divided by 8"]
        DIV8 = 0x03,
        #[doc = "MCO is divided by 16"]
        DIV16 = 0x04,
        #[doc = "MCO is divided by 32"]
        DIV32 = 0x05,
        #[doc = "MCO is divided by 64"]
        DIV64 = 0x06,
        #[doc = "MCO is divided by 128"]
        DIV128 = 0x07,
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
        _RESERVED_1 = 0x01,
        #[doc = "Internal low speed (LSI) oscillator clock selected"]
        LSI = 0x02,
        #[doc = "External low speed (LSE) oscillator clock selected"]
        LSE = 0x03,
        #[doc = "System clock selected"]
        SYS = 0x04,
        #[doc = "Internal RC 8 MHz (HSI) oscillator clock selected"]
        HSI = 0x05,
        #[doc = "External 4-32 MHz (HSE) oscillator clock selected"]
        HSE = 0x06,
        #[doc = "PLL clock selected (divided by 1 or 2, depending en PLLMCODIV)"]
        PLL = 0x07,
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
    pub enum Pllmcodiv {
        #[doc = "PLL is divided by 2 for MCO"]
        DIV2 = 0x0,
        #[doc = "PLL is not divided for MCO"]
        DIV1 = 0x01,
    }
    impl Pllmcodiv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllmcodiv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllmcodiv {
        #[inline(always)]
        fn from(val: u8) -> Pllmcodiv {
            Pllmcodiv::from_bits(val)
        }
    }
    impl From<Pllmcodiv> for u8 {
        #[inline(always)]
        fn from(val: Pllmcodiv) -> u8 {
            Pllmcodiv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllmul {
        #[doc = "PLL input clock x2"]
        MUL2 = 0x0,
        #[doc = "PLL input clock x3"]
        MUL3 = 0x01,
        #[doc = "PLL input clock x4"]
        MUL4 = 0x02,
        #[doc = "PLL input clock x5"]
        MUL5 = 0x03,
        #[doc = "PLL input clock x6"]
        MUL6 = 0x04,
        #[doc = "PLL input clock x7"]
        MUL7 = 0x05,
        #[doc = "PLL input clock x8"]
        MUL8 = 0x06,
        #[doc = "PLL input clock x9"]
        MUL9 = 0x07,
        #[doc = "PLL input clock x10"]
        MUL10 = 0x08,
        #[doc = "PLL input clock x11"]
        MUL11 = 0x09,
        #[doc = "PLL input clock x12"]
        MUL12 = 0x0a,
        #[doc = "PLL input clock x13"]
        MUL13 = 0x0b,
        #[doc = "PLL input clock x14"]
        MUL14 = 0x0c,
        #[doc = "PLL input clock x15"]
        MUL15 = 0x0d,
        #[doc = "PLL input clock x16"]
        MUL16 = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Pllmul {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllmul {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllmul {
        #[inline(always)]
        fn from(val: u8) -> Pllmul {
            Pllmul::from_bits(val)
        }
    }
    impl From<Pllmul> for u8 {
        #[inline(always)]
        fn from(val: Pllmul) -> u8 {
            Pllmul::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllsrc {
        #[doc = "HSI divided by 2 selected as PLL input clock"]
        HSI_DIV2 = 0x0,
        #[doc = "HSE divided by PREDIV selected as PLL input clock"]
        HSE_DIV_PREDIV = 0x01,
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
    pub enum Pllxtpre {
        #[doc = "HSE clock not divided"]
        DIV1 = 0x0,
        #[doc = "HSE clock divided by 2"]
        DIV2 = 0x01,
    }
    impl Pllxtpre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllxtpre {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllxtpre {
        #[inline(always)]
        fn from(val: u8) -> Pllxtpre {
            Pllxtpre::from_bits(val)
        }
    }
    impl From<Pllxtpre> for u8 {
        #[inline(always)]
        fn from(val: Pllxtpre) -> u8 {
            Pllxtpre::to_bits(val)
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
    pub enum Prediv {
        #[doc = "PREDIV input clock not divided"]
        DIV1 = 0x0,
        #[doc = "PREDIV input clock divided by 2"]
        DIV2 = 0x01,
        #[doc = "PREDIV input clock divided by 3"]
        DIV3 = 0x02,
        #[doc = "PREDIV input clock divided by 4"]
        DIV4 = 0x03,
        #[doc = "PREDIV input clock divided by 5"]
        DIV5 = 0x04,
        #[doc = "PREDIV input clock divided by 6"]
        DIV6 = 0x05,
        #[doc = "PREDIV input clock divided by 7"]
        DIV7 = 0x06,
        #[doc = "PREDIV input clock divided by 8"]
        DIV8 = 0x07,
        #[doc = "PREDIV input clock divided by 9"]
        DIV9 = 0x08,
        #[doc = "PREDIV input clock divided by 10"]
        DIV10 = 0x09,
        #[doc = "PREDIV input clock divided by 11"]
        DIV11 = 0x0a,
        #[doc = "PREDIV input clock divided by 12"]
        DIV12 = 0x0b,
        #[doc = "PREDIV input clock divided by 13"]
        DIV13 = 0x0c,
        #[doc = "PREDIV input clock divided by 14"]
        DIV14 = 0x0d,
        #[doc = "PREDIV input clock divided by 15"]
        DIV15 = 0x0e,
        #[doc = "PREDIV input clock divided by 16"]
        DIV16 = 0x0f,
    }
    impl Prediv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Prediv {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Prediv {
        #[inline(always)]
        fn from(val: u8) -> Prediv {
            Prediv::from_bits(val)
        }
    }
    impl From<Prediv> for u8 {
        #[inline(always)]
        fn from(val: Prediv) -> u8 {
            Prediv::to_bits(val)
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
    pub enum Sw {
        #[doc = "HSI oscillator used as system clock"]
        HSI = 0x0,
        #[doc = "HSE oscillator used as system clock"]
        HSE = 0x01,
        #[doc = "PLL used as system clock"]
        PLL1_P = 0x02,
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
    pub enum Tim2sw {
        #[doc = "PCLK2 clock (doubled frequency when prescaled)"]
        PCLK1_TIM = 0x0,
        #[doc = "PLL vco output (running up to 144 MHz)"]
        PLL1_P = 0x01,
    }
    impl Tim2sw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tim2sw {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tim2sw {
        #[inline(always)]
        fn from(val: u8) -> Tim2sw {
            Tim2sw::from_bits(val)
        }
    }
    impl From<Tim2sw> for u8 {
        #[inline(always)]
        fn from(val: Tim2sw) -> u8 {
            Tim2sw::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Timsw {
        #[doc = "PCLK2 clock (doubled frequency when prescaled)"]
        PCLK2_TIM = 0x0,
        #[doc = "PLL vco output (running up to 144 MHz)"]
        PLL1_P = 0x01,
    }
    impl Timsw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Timsw {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Timsw {
        #[inline(always)]
        fn from(val: u8) -> Timsw {
            Timsw::from_bits(val)
        }
    }
    impl From<Timsw> for u8 {
        #[inline(always)]
        fn from(val: Timsw) -> u8 {
            Timsw::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Usart1sw {
        #[doc = "PCLK selected as USART clock source"]
        PCLK2 = 0x0,
        #[doc = "SYSCLK selected as USART clock source"]
        SYS = 0x01,
        #[doc = "LSE selected as USART clock source"]
        LSE = 0x02,
        #[doc = "HSI selected as USART clock source"]
        HSI = 0x03,
    }
    impl Usart1sw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usart1sw {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usart1sw {
        #[inline(always)]
        fn from(val: u8) -> Usart1sw {
            Usart1sw::from_bits(val)
        }
    }
    impl From<Usart1sw> for u8 {
        #[inline(always)]
        fn from(val: Usart1sw) -> u8 {
            Usart1sw::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Usartsw {
        #[doc = "PCLK selected as USART clock source"]
        PCLK1 = 0x0,
        #[doc = "SYSCLK selected as USART clock source"]
        SYS = 0x01,
        #[doc = "LSE selected as USART clock source"]
        LSE = 0x02,
        #[doc = "HSI selected as USART clock source"]
        HSI = 0x03,
    }
    impl Usartsw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usartsw {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usartsw {
        #[inline(always)]
        fn from(val: u8) -> Usartsw {
            Usartsw::from_bits(val)
        }
    }
    impl From<Usartsw> for u8 {
        #[inline(always)]
        fn from(val: Usartsw) -> u8 {
            Usartsw::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Usbpre {
        #[doc = "PLL clock is divided by 1.5"]
        DIV1_5 = 0x0,
        #[doc = "PLL clock is not divided"]
        DIV1 = 0x01,
    }
    impl Usbpre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usbpre {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Usbpre {
        #[inline(always)]
        fn from(val: u8) -> Usbpre {
            Usbpre::from_bits(val)
        }
    }
    impl From<Usbpre> for u8 {
        #[inline(always)]
        fn from(val: Usbpre) -> u8 {
            Usbpre::to_bits(val)
        }
    }
}
