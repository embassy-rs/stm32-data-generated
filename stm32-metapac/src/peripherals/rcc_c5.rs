#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "RCC register block"]
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "RCC clock control register"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "RCC clock configuration register1"]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "RCC CPU domain clock configuration register 2"]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "RCC clock source interrupt enable register"]
    #[inline(always)]
    pub const fn cier(self) -> crate::common::Reg<regs::Cier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "RCC clock source interrupt flag register"]
    #[inline(always)]
    pub const fn cifr(self) -> crate::common::Reg<regs::Cifr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "RCC clock source interrupt clear register"]
    #[inline(always)]
    pub const fn cicr(self) -> crate::common::Reg<regs::Cicr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "RCC AHB1 reset register"]
    #[inline(always)]
    pub const fn ahb1rstr(self) -> crate::common::Reg<regs::Ahb1rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "RCC AHB2 peripheral reset register"]
    #[inline(always)]
    pub const fn ahb2rstr(self) -> crate::common::Reg<regs::Ahb2rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "RCC AHB4 peripheral reset register"]
    #[inline(always)]
    pub const fn ahb4rstr(self) -> crate::common::Reg<regs::Ahb4rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "RCC APB1 peripheral low reset register"]
    #[inline(always)]
    pub const fn apb1lrstr(self) -> crate::common::Reg<regs::Apb1lrstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "RCC APB1 peripheral high reset register"]
    #[inline(always)]
    pub const fn apb1hrstr(self) -> crate::common::Reg<regs::Apb1hrstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "RCC APB2 peripheral reset register"]
    #[inline(always)]
    pub const fn apb2rstr(self) -> crate::common::Reg<regs::Apb2rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "RCC APB3 peripheral reset register"]
    #[inline(always)]
    pub const fn apb3rstr(self) -> crate::common::Reg<regs::Apb3rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "RCC AHB1 peripheral clock register"]
    #[inline(always)]
    pub const fn ahb1enr(self) -> crate::common::Reg<regs::Ahb1enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "RCC AHB2 peripheral clock register"]
    #[inline(always)]
    pub const fn ahb2enr(self) -> crate::common::Reg<regs::Ahb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "RCC AHB4 peripheral clock register"]
    #[inline(always)]
    pub const fn ahb4enr(self) -> crate::common::Reg<regs::Ahb4enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "RCC APB1 peripheral clock register"]
    #[inline(always)]
    pub const fn apb1lenr(self) -> crate::common::Reg<regs::Apb1lenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "RCC APB1 peripheral clock register"]
    #[inline(always)]
    pub const fn apb1henr(self) -> crate::common::Reg<regs::Apb1henr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "RCC APB2 peripheral clock register"]
    #[inline(always)]
    pub const fn apb2enr(self) -> crate::common::Reg<regs::Apb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "RCC APB3 peripheral clock register"]
    #[inline(always)]
    pub const fn apb3enr(self) -> crate::common::Reg<regs::Apb3enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "RCC AHB1 sleep clock register"]
    #[inline(always)]
    pub const fn ahb1lpenr(self) -> crate::common::Reg<regs::Ahb1lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "RCC AHB2 sleep clock register"]
    #[inline(always)]
    pub const fn ahb2lpenr(self) -> crate::common::Reg<regs::Ahb2lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "RCC AHB4 sleep clock register"]
    #[inline(always)]
    pub const fn ahb4lpenr(self) -> crate::common::Reg<regs::Ahb4lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "RCC APB1 sleep clock register"]
    #[inline(always)]
    pub const fn apb1llpenr(self) -> crate::common::Reg<regs::Apb1llpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "RCC APB1 sleep clock register"]
    #[inline(always)]
    pub const fn apb1hlpenr(self) -> crate::common::Reg<regs::Apb1hlpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "RCC APB2 sleep clock register"]
    #[inline(always)]
    pub const fn apb2lpenr(self) -> crate::common::Reg<regs::Apb2lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "RCC APB3 sleep clock register"]
    #[inline(always)]
    pub const fn apb3lpenr(self) -> crate::common::Reg<regs::Apb3lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "RCC kernel clock configuration register"]
    #[inline(always)]
    pub const fn ccipr1(self) -> crate::common::Reg<regs::Ccipr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "RCC kernel clock configuration register"]
    #[inline(always)]
    pub const fn ccipr2(self) -> crate::common::Reg<regs::Ccipr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "RCC kernel clock configuration register"]
    #[inline(always)]
    pub const fn ccipr3(self) -> crate::common::Reg<regs::Ccipr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "RCC RTC domain control register"]
    #[inline(always)]
    pub const fn rtccr(self) -> crate::common::Reg<regs::Rtccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "RCC reset status register"]
    #[inline(always)]
    pub const fn rsr(self) -> crate::common::Reg<regs::Rsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "RCC privilege configuration register"]
    #[inline(always)]
    pub const fn privcfgr(self) -> crate::common::Reg<regs::Privcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
}
pub mod regs {
    #[doc = "RCC AHB1 peripherals clock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1enr(pub u32);
    impl Ahb1enr {
        #[doc = "LPDMA1 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lpdma1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LPDMA1 clock enable"]
        #[inline(always)]
        pub const fn set_lpdma1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LPDMA2 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lpdma2en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LPDMA2 clock enable"]
        #[inline(always)]
        pub const fn set_lpdma2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Flash interface clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn flitfen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Flash interface clock enable"]
        #[inline(always)]
        pub const fn set_flitfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
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
        #[doc = "CORDIC clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn cordicen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "CORDIC clock enable"]
        #[inline(always)]
        pub const fn set_cordicen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "RAMCFG clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ramcfgen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "RAMCFG clock enable"]
        #[inline(always)]
        pub const fn set_ramcfgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "ETH internal clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ethcken(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "ETH internal clock enable"]
        #[inline(always)]
        pub const fn set_ethcken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "ETH clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ethen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "ETH clock enable"]
        #[inline(always)]
        pub const fn set_ethen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "ETHTX clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ethtxen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "ETHTX clock enable"]
        #[inline(always)]
        pub const fn set_ethtxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "ETHRX clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ethrxen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "ETHRX clock enable"]
        #[inline(always)]
        pub const fn set_ethrxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SRAM2 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn sram2en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 clock enable"]
        #[inline(always)]
        pub const fn set_sram2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SRAM1 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn sram1en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 clock enable"]
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
                .field("lpdma1en", &self.lpdma1en())
                .field("lpdma2en", &self.lpdma2en())
                .field("flitfen", &self.flitfen())
                .field("crcen", &self.crcen())
                .field("cordicen", &self.cordicen())
                .field("ramcfgen", &self.ramcfgen())
                .field("ethcken", &self.ethcken())
                .field("ethen", &self.ethen())
                .field("ethtxen", &self.ethtxen())
                .field("ethrxen", &self.ethrxen())
                .field("sram2en", &self.sram2en())
                .field("sram1en", &self.sram1en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1enr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb1enr {{ lpdma1en: {=bool:?}, lpdma2en: {=bool:?}, flitfen: {=bool:?}, crcen: {=bool:?}, cordicen: {=bool:?}, ramcfgen: {=bool:?}, ethcken: {=bool:?}, ethen: {=bool:?}, ethtxen: {=bool:?}, ethrxen: {=bool:?}, sram2en: {=bool:?}, sram1en: {=bool:?} }}",
                self.lpdma1en(),
                self.lpdma2en(),
                self.flitfen(),
                self.crcen(),
                self.cordicen(),
                self.ramcfgen(),
                self.ethcken(),
                self.ethen(),
                self.ethtxen(),
                self.ethrxen(),
                self.sram2en(),
                self.sram1en()
            )
        }
    }
    #[doc = "RCC AHB1 sleep clock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1lpenr(pub u32);
    impl Ahb1lpenr {
        #[doc = "LPDMA1 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn lpdma1lpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LPDMA1 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_lpdma1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LPDMA2 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn lpdma2lpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LPDMA2 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_lpdma2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Flash interface clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn flitflpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Flash interface clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_flitflpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
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
        #[doc = "CORDIC clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn cordiclpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "CORDIC clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_cordiclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "RAMCFG clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn ramcfglpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "RAMCFG clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_ramcfglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "ETH internal clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn ethcklpen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "ETH internal clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_ethcklpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "ETH clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn ethlpen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "ETH clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_ethlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "ETHTX clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn ethtxlpen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "ETHTX clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_ethtxlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "ETHRX clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn ethrxlpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "ETHRX clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_ethrxlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ICACHE clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn icachelpen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "ICACHE clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_icachelpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "SRAM2 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn sram2lpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_sram2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SRAM1 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn sram1lpen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 clock enable during Sleep mode"]
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
                .field("lpdma1lpen", &self.lpdma1lpen())
                .field("lpdma2lpen", &self.lpdma2lpen())
                .field("flitflpen", &self.flitflpen())
                .field("crclpen", &self.crclpen())
                .field("cordiclpen", &self.cordiclpen())
                .field("ramcfglpen", &self.ramcfglpen())
                .field("ethcklpen", &self.ethcklpen())
                .field("ethlpen", &self.ethlpen())
                .field("ethtxlpen", &self.ethtxlpen())
                .field("ethrxlpen", &self.ethrxlpen())
                .field("icachelpen", &self.icachelpen())
                .field("sram2lpen", &self.sram2lpen())
                .field("sram1lpen", &self.sram1lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1lpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb1lpenr {{ lpdma1lpen: {=bool:?}, lpdma2lpen: {=bool:?}, flitflpen: {=bool:?}, crclpen: {=bool:?}, cordiclpen: {=bool:?}, ramcfglpen: {=bool:?}, ethcklpen: {=bool:?}, ethlpen: {=bool:?}, ethtxlpen: {=bool:?}, ethrxlpen: {=bool:?}, icachelpen: {=bool:?}, sram2lpen: {=bool:?}, sram1lpen: {=bool:?} }}",
                self.lpdma1lpen(),
                self.lpdma2lpen(),
                self.flitflpen(),
                self.crclpen(),
                self.cordiclpen(),
                self.ramcfglpen(),
                self.ethcklpen(),
                self.ethlpen(),
                self.ethtxlpen(),
                self.ethrxlpen(),
                self.icachelpen(),
                self.sram2lpen(),
                self.sram1lpen()
            )
        }
    }
    #[doc = "RCC AHB1 reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1rstr(pub u32);
    impl Ahb1rstr {
        #[doc = "LPDMA1 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn lpdma1rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LPDMA1 reset"]
        #[inline(always)]
        pub const fn set_lpdma1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LPDMA2 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn lpdma2rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LPDMA2 reset"]
        #[inline(always)]
        pub const fn set_lpdma2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
        #[doc = "CORDIC reset"]
        #[must_use]
        #[inline(always)]
        pub const fn cordicrst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "CORDIC reset"]
        #[inline(always)]
        pub const fn set_cordicrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "RAMCFG reset"]
        #[must_use]
        #[inline(always)]
        pub const fn ramcfgrst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "RAMCFG reset"]
        #[inline(always)]
        pub const fn set_ramcfgrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "ETHERNET reset"]
        #[must_use]
        #[inline(always)]
        pub const fn ethrst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "ETHERNET reset"]
        #[inline(always)]
        pub const fn set_ethrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
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
                .field("lpdma1rst", &self.lpdma1rst())
                .field("lpdma2rst", &self.lpdma2rst())
                .field("crcrst", &self.crcrst())
                .field("cordicrst", &self.cordicrst())
                .field("ramcfgrst", &self.ramcfgrst())
                .field("ethrst", &self.ethrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb1rstr {{ lpdma1rst: {=bool:?}, lpdma2rst: {=bool:?}, crcrst: {=bool:?}, cordicrst: {=bool:?}, ramcfgrst: {=bool:?}, ethrst: {=bool:?} }}",
                self.lpdma1rst(),
                self.lpdma2rst(),
                self.crcrst(),
                self.cordicrst(),
                self.ramcfgrst(),
                self.ethrst()
            )
        }
    }
    #[doc = "RCC AHB2 peripheral clock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2enr(pub u32);
    impl Ahb2enr {
        #[doc = "GPIOA clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOA clock enable"]
        #[inline(always)]
        pub const fn set_gpioaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPIOB clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioben(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOB clock enable"]
        #[inline(always)]
        pub const fn set_gpioben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "GPIOC clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiocen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOC clock enable"]
        #[inline(always)]
        pub const fn set_gpiocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "GPIOD clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioden(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOD clock enable"]
        #[inline(always)]
        pub const fn set_gpioden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "GPIOE clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioeen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOE clock enable"]
        #[inline(always)]
        pub const fn set_gpioeen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "GPIOF clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiofen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOF clock enable"]
        #[inline(always)]
        pub const fn set_gpiofen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "GPIOG clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiogen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOG clock enable"]
        #[inline(always)]
        pub const fn set_gpiogen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "GPIOH clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiohen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOH clock enable"]
        #[inline(always)]
        pub const fn set_gpiohen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ADC1 and ADC2 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn adcen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1 and ADC2 clock enable"]
        #[inline(always)]
        pub const fn set_adcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "DAC clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn dac1en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "DAC clock enable"]
        #[inline(always)]
        pub const fn set_dac1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "AES clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn aesen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "AES clock enable"]
        #[inline(always)]
        pub const fn set_aesen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "HASH clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn hashen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "HASH clock enable"]
        #[inline(always)]
        pub const fn set_hashen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "RNG clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn rngen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "RNG clock enable"]
        #[inline(always)]
        pub const fn set_rngen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PKA clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn pkaen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "PKA clock enable"]
        #[inline(always)]
        pub const fn set_pkaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "SAES clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn saesen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SAES clock enable"]
        #[inline(always)]
        pub const fn set_saesen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "CCB clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ccben(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "CCB clock enable"]
        #[inline(always)]
        pub const fn set_ccben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ADC3 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn adc3en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ADC3 clock enable"]
        #[inline(always)]
        pub const fn set_adc3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
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
                .field("gpioeen", &self.gpioeen())
                .field("gpiofen", &self.gpiofen())
                .field("gpiogen", &self.gpiogen())
                .field("gpiohen", &self.gpiohen())
                .field("adcen", &self.adcen())
                .field("dac1en", &self.dac1en())
                .field("aesen", &self.aesen())
                .field("hashen", &self.hashen())
                .field("rngen", &self.rngen())
                .field("pkaen", &self.pkaen())
                .field("saesen", &self.saesen())
                .field("ccben", &self.ccben())
                .field("adc3en", &self.adc3en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2enr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb2enr {{ gpioaen: {=bool:?}, gpioben: {=bool:?}, gpiocen: {=bool:?}, gpioden: {=bool:?}, gpioeen: {=bool:?}, gpiofen: {=bool:?}, gpiogen: {=bool:?}, gpiohen: {=bool:?}, adcen: {=bool:?}, dac1en: {=bool:?}, aesen: {=bool:?}, hashen: {=bool:?}, rngen: {=bool:?}, pkaen: {=bool:?}, saesen: {=bool:?}, ccben: {=bool:?}, adc3en: {=bool:?} }}",
                self.gpioaen(),
                self.gpioben(),
                self.gpiocen(),
                self.gpioden(),
                self.gpioeen(),
                self.gpiofen(),
                self.gpiogen(),
                self.gpiohen(),
                self.adcen(),
                self.dac1en(),
                self.aesen(),
                self.hashen(),
                self.rngen(),
                self.pkaen(),
                self.saesen(),
                self.ccben(),
                self.adc3en()
            )
        }
    }
    #[doc = "RCC AHB2 sleep clock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2lpenr(pub u32);
    impl Ahb2lpenr {
        #[doc = "GPIOA clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioalpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOA clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_gpioalpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPIOB clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioblpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOB clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_gpioblpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "GPIOC clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioclpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOC clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_gpioclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "GPIOD clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiodlpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOD clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_gpiodlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "GPIOE clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioelpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOE clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_gpioelpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "GPIOF clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioflpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOF clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_gpioflpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "GPIOG clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioglpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOG clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_gpioglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "GPIOH clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiohlpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOH clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_gpiohlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ADC1 and ADC2 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn adclpen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1 and ADC2 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_adclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "DAC clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn dac1lpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "DAC clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_dac1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "AES clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn aeslpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "AES clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_aeslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "HASH clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn hashlpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "HASH clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_hashlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "RNG clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn rnglpen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "RNG clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_rnglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PKA clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn pkalpen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "PKA clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_pkalpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "SAES clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn saeslpen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SAES clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_saeslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "CCB clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn ccblpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "CCB clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_ccblpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ADC3 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn adc3lpen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ADC3 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_adc3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
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
                .field("gpioelpen", &self.gpioelpen())
                .field("gpioflpen", &self.gpioflpen())
                .field("gpioglpen", &self.gpioglpen())
                .field("gpiohlpen", &self.gpiohlpen())
                .field("adclpen", &self.adclpen())
                .field("dac1lpen", &self.dac1lpen())
                .field("aeslpen", &self.aeslpen())
                .field("hashlpen", &self.hashlpen())
                .field("rnglpen", &self.rnglpen())
                .field("pkalpen", &self.pkalpen())
                .field("saeslpen", &self.saeslpen())
                .field("ccblpen", &self.ccblpen())
                .field("adc3lpen", &self.adc3lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2lpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb2lpenr {{ gpioalpen: {=bool:?}, gpioblpen: {=bool:?}, gpioclpen: {=bool:?}, gpiodlpen: {=bool:?}, gpioelpen: {=bool:?}, gpioflpen: {=bool:?}, gpioglpen: {=bool:?}, gpiohlpen: {=bool:?}, adclpen: {=bool:?}, dac1lpen: {=bool:?}, aeslpen: {=bool:?}, hashlpen: {=bool:?}, rnglpen: {=bool:?}, pkalpen: {=bool:?}, saeslpen: {=bool:?}, ccblpen: {=bool:?}, adc3lpen: {=bool:?} }}",
                self.gpioalpen(),
                self.gpioblpen(),
                self.gpioclpen(),
                self.gpiodlpen(),
                self.gpioelpen(),
                self.gpioflpen(),
                self.gpioglpen(),
                self.gpiohlpen(),
                self.adclpen(),
                self.dac1lpen(),
                self.aeslpen(),
                self.hashlpen(),
                self.rnglpen(),
                self.pkalpen(),
                self.saeslpen(),
                self.ccblpen(),
                self.adc3lpen()
            )
        }
    }
    #[doc = "RCC AHB2 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2rstr(pub u32);
    impl Ahb2rstr {
        #[doc = "GPIOA reset"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioarst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOA reset"]
        #[inline(always)]
        pub const fn set_gpioarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPIOB reset"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiobrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOB reset"]
        #[inline(always)]
        pub const fn set_gpiobrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "GPIOC reset"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiocrst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOC reset"]
        #[inline(always)]
        pub const fn set_gpiocrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "GPIOD reset"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiodrst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOD reset"]
        #[inline(always)]
        pub const fn set_gpiodrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "GPIOE reset"]
        #[must_use]
        #[inline(always)]
        pub const fn gpioerst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOE reset"]
        #[inline(always)]
        pub const fn set_gpioerst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "GPIOF reset"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiofrst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOF reset"]
        #[inline(always)]
        pub const fn set_gpiofrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "GPIOG reset"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiogrst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOG reset"]
        #[inline(always)]
        pub const fn set_gpiogrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "GPIOH reset"]
        #[must_use]
        #[inline(always)]
        pub const fn gpiohrst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "GPIOH reset"]
        #[inline(always)]
        pub const fn set_gpiohrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ADC1 and ADC2 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn adcrst(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1 and ADC2 reset"]
        #[inline(always)]
        pub const fn set_adcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "DAC reset"]
        #[must_use]
        #[inline(always)]
        pub const fn dac1rst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "DAC reset"]
        #[inline(always)]
        pub const fn set_dac1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "AES reset"]
        #[must_use]
        #[inline(always)]
        pub const fn aesrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "AES reset"]
        #[inline(always)]
        pub const fn set_aesrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "HASH reset"]
        #[must_use]
        #[inline(always)]
        pub const fn hashrst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "HASH reset"]
        #[inline(always)]
        pub const fn set_hashrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "RNG reset"]
        #[must_use]
        #[inline(always)]
        pub const fn rngrst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "RNG reset"]
        #[inline(always)]
        pub const fn set_rngrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "PKA reset"]
        #[must_use]
        #[inline(always)]
        pub const fn pkarst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "PKA reset"]
        #[inline(always)]
        pub const fn set_pkarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "SAES reset"]
        #[must_use]
        #[inline(always)]
        pub const fn saesrst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SAES reset"]
        #[inline(always)]
        pub const fn set_saesrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "CCB reset"]
        #[must_use]
        #[inline(always)]
        pub const fn ccbrst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "CCB reset"]
        #[inline(always)]
        pub const fn set_ccbrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ADC3 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn adc3rst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ADC3 reset"]
        #[inline(always)]
        pub const fn set_adc3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
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
                .field("gpioerst", &self.gpioerst())
                .field("gpiofrst", &self.gpiofrst())
                .field("gpiogrst", &self.gpiogrst())
                .field("gpiohrst", &self.gpiohrst())
                .field("adcrst", &self.adcrst())
                .field("dac1rst", &self.dac1rst())
                .field("aesrst", &self.aesrst())
                .field("hashrst", &self.hashrst())
                .field("rngrst", &self.rngrst())
                .field("pkarst", &self.pkarst())
                .field("saesrst", &self.saesrst())
                .field("ccbrst", &self.ccbrst())
                .field("adc3rst", &self.adc3rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb2rstr {{ gpioarst: {=bool:?}, gpiobrst: {=bool:?}, gpiocrst: {=bool:?}, gpiodrst: {=bool:?}, gpioerst: {=bool:?}, gpiofrst: {=bool:?}, gpiogrst: {=bool:?}, gpiohrst: {=bool:?}, adcrst: {=bool:?}, dac1rst: {=bool:?}, aesrst: {=bool:?}, hashrst: {=bool:?}, rngrst: {=bool:?}, pkarst: {=bool:?}, saesrst: {=bool:?}, ccbrst: {=bool:?}, adc3rst: {=bool:?} }}",
                self.gpioarst(),
                self.gpiobrst(),
                self.gpiocrst(),
                self.gpiodrst(),
                self.gpioerst(),
                self.gpiofrst(),
                self.gpiogrst(),
                self.gpiohrst(),
                self.adcrst(),
                self.dac1rst(),
                self.aesrst(),
                self.hashrst(),
                self.rngrst(),
                self.pkarst(),
                self.saesrst(),
                self.ccbrst(),
                self.adc3rst()
            )
        }
    }
    #[doc = "RCC AHB4 peripheral clock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb4enr(pub u32);
    impl Ahb4enr {
        #[doc = "XSPI1 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn xspi1en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "XSPI1 clock enable"]
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
            f.debug_struct("Ahb4enr").field("xspi1en", &self.xspi1en()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb4enr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ahb4enr {{ xspi1en: {=bool:?} }}", self.xspi1en())
        }
    }
    #[doc = "RCC AHB4 peripheral sleep clock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb4lpenr(pub u32);
    impl Ahb4lpenr {
        #[doc = "XSPI1 clock enable during sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn xspi1lpen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "XSPI1 clock enable during sleep mode"]
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
                .field("xspi1lpen", &self.xspi1lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb4lpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ahb4lpenr {{ xspi1lpen: {=bool:?} }}", self.xspi1lpen())
        }
    }
    #[doc = "RCC AHB4 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb4rstr(pub u32);
    impl Ahb4rstr {
        #[doc = "XSPI1 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn xspi1rst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "XSPI1 reset"]
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
            f.debug_struct("Ahb4rstr").field("xspi1rst", &self.xspi1rst()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb4rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ahb4rstr {{ xspi1rst: {=bool:?} }}", self.xspi1rst())
        }
    }
    #[doc = "RCC APB1 peripheral clock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1henr(pub u32);
    impl Apb1henr {
        #[doc = "COMP clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn compen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "COMP clock enable"]
        #[inline(always)]
        pub const fn set_compen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "FDCAN1 and FDCAN2 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn fdcanen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN1 and FDCAN2 clock enable"]
        #[inline(always)]
        pub const fn set_fdcanen(&mut self, val: bool) {
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
                .field("compen", &self.compen())
                .field("fdcanen", &self.fdcanen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1henr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb1henr {{ compen: {=bool:?}, fdcanen: {=bool:?} }}",
                self.compen(),
                self.fdcanen()
            )
        }
    }
    #[doc = "RCC APB1 sleep clock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1hlpenr(pub u32);
    impl Apb1hlpenr {
        #[doc = "COMP clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn complpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "COMP clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_complpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "FDCAN1 and FDCAN2 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn fdcanlpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN1 and FDCAN2 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_fdcanlpen(&mut self, val: bool) {
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
                .field("complpen", &self.complpen())
                .field("fdcanlpen", &self.fdcanlpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1hlpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb1hlpenr {{ complpen: {=bool:?}, fdcanlpen: {=bool:?} }}",
                self.complpen(),
                self.fdcanlpen()
            )
        }
    }
    #[doc = "RCC APB1 peripheral high reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1hrstr(pub u32);
    impl Apb1hrstr {
        #[doc = "COMP reset"]
        #[must_use]
        #[inline(always)]
        pub const fn comprst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "COMP reset"]
        #[inline(always)]
        pub const fn set_comprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "FDCAN1 and FDCAN2 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn fdcanrst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "FDCAN1 and FDCAN2 reset"]
        #[inline(always)]
        pub const fn set_fdcanrst(&mut self, val: bool) {
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
                .field("comprst", &self.comprst())
                .field("fdcanrst", &self.fdcanrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1hrstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb1hrstr {{ comprst: {=bool:?}, fdcanrst: {=bool:?} }}",
                self.comprst(),
                self.fdcanrst()
            )
        }
    }
    #[doc = "RCC APB1 peripheral clock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1lenr(pub u32);
    impl Apb1lenr {
        #[doc = "TIM2 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim2en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 clock enable"]
        #[inline(always)]
        pub const fn set_tim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim3en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 clock enable"]
        #[inline(always)]
        pub const fn set_tim3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim4en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 clock enable"]
        #[inline(always)]
        pub const fn set_tim4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
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
        #[doc = "TIM7 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim7en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 clock enable"]
        #[inline(always)]
        pub const fn set_tim7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM12 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim12en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12 clock enable"]
        #[inline(always)]
        pub const fn set_tim12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "WWDG clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn wwdgen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG clock enable"]
        #[inline(always)]
        pub const fn set_wwdgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "OPAMP clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn opampen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP clock enable"]
        #[inline(always)]
        pub const fn set_opampen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
        #[doc = "SPI3 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn spi3en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 clock enable"]
        #[inline(always)]
        pub const fn set_spi3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "USART2 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn usart2en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 clock enable"]
        #[inline(always)]
        pub const fn set_usart2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn usart3en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 clock enable"]
        #[inline(always)]
        pub const fn set_usart3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn uart4en(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 clock enable"]
        #[inline(always)]
        pub const fn set_uart4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn uart5en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 clock enable"]
        #[inline(always)]
        pub const fn set_uart5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
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
        #[doc = "I3C1 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn i3c1en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I3C1 clock enable"]
        #[inline(always)]
        pub const fn set_i3c1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "CRS clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn crsen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "CRS clock enable"]
        #[inline(always)]
        pub const fn set_crsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "USART6 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn usart6en(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "USART6 clock enable"]
        #[inline(always)]
        pub const fn set_usart6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "UART7 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn uart7en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "UART7 clock enable"]
        #[inline(always)]
        pub const fn set_uart7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
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
                .field("wwdgen", &self.wwdgen())
                .field("opampen", &self.opampen())
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
                .field("usart6en", &self.usart6en())
                .field("uart7en", &self.uart7en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1lenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb1lenr {{ tim2en: {=bool:?}, tim3en: {=bool:?}, tim4en: {=bool:?}, tim5en: {=bool:?}, tim6en: {=bool:?}, tim7en: {=bool:?}, tim12en: {=bool:?}, wwdgen: {=bool:?}, opampen: {=bool:?}, spi2en: {=bool:?}, spi3en: {=bool:?}, usart2en: {=bool:?}, usart3en: {=bool:?}, uart4en: {=bool:?}, uart5en: {=bool:?}, i2c1en: {=bool:?}, i2c2en: {=bool:?}, i3c1en: {=bool:?}, crsen: {=bool:?}, usart6en: {=bool:?}, uart7en: {=bool:?} }}",
                self.tim2en(),
                self.tim3en(),
                self.tim4en(),
                self.tim5en(),
                self.tim6en(),
                self.tim7en(),
                self.tim12en(),
                self.wwdgen(),
                self.opampen(),
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
                self.usart6en(),
                self.uart7en()
            )
        }
    }
    #[doc = "RCC APB1 sleep clock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1llpenr(pub u32);
    impl Apb1llpenr {
        #[doc = "TIM2 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim2lpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_tim2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim3lpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_tim3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim4lpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_tim4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
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
        #[doc = "TIM7 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim7lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_tim7lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM12 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim12lpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_tim12lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "WWDG clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn wwdglpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_wwdglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "OPAMP clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn opamplpen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_opamplpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
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
        #[doc = "SPI3 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn spi3lpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_spi3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
        #[doc = "USART3 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn usart3lpen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_usart3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn uart4lpen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_uart4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn uart5lpen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_uart5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
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
        #[doc = "I3C1 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn i3c1lpen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I3C1 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_i3c1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "CRS clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn crslpen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "CRS clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_crslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "USART6 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn usart6lpen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "USART6 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_usart6lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "UART7 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn uart7lpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "UART7 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_uart7lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
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
                .field("wwdglpen", &self.wwdglpen())
                .field("opamplpen", &self.opamplpen())
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
                .field("usart6lpen", &self.usart6lpen())
                .field("uart7lpen", &self.uart7lpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1llpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb1llpenr {{ tim2lpen: {=bool:?}, tim3lpen: {=bool:?}, tim4lpen: {=bool:?}, tim5lpen: {=bool:?}, tim6lpen: {=bool:?}, tim7lpen: {=bool:?}, tim12lpen: {=bool:?}, wwdglpen: {=bool:?}, opamplpen: {=bool:?}, spi2lpen: {=bool:?}, spi3lpen: {=bool:?}, usart2lpen: {=bool:?}, usart3lpen: {=bool:?}, uart4lpen: {=bool:?}, uart5lpen: {=bool:?}, i2c1lpen: {=bool:?}, i2c2lpen: {=bool:?}, i3c1lpen: {=bool:?}, crslpen: {=bool:?}, usart6lpen: {=bool:?}, uart7lpen: {=bool:?} }}",
                self.tim2lpen(),
                self.tim3lpen(),
                self.tim4lpen(),
                self.tim5lpen(),
                self.tim6lpen(),
                self.tim7lpen(),
                self.tim12lpen(),
                self.wwdglpen(),
                self.opamplpen(),
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
                self.usart6lpen(),
                self.uart7lpen()
            )
        }
    }
    #[doc = "RCC APB1 peripheral low reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1lrstr(pub u32);
    impl Apb1lrstr {
        #[doc = "TIM2 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim2rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 reset"]
        #[inline(always)]
        pub const fn set_tim2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim3rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 reset"]
        #[inline(always)]
        pub const fn set_tim3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim4rst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 reset"]
        #[inline(always)]
        pub const fn set_tim4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
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
        #[doc = "TIM7 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim7rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 reset"]
        #[inline(always)]
        pub const fn set_tim7rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM12 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim12rst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12 reset"]
        #[inline(always)]
        pub const fn set_tim12rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "OPAMP reset"]
        #[must_use]
        #[inline(always)]
        pub const fn opamprst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "OPAMP reset"]
        #[inline(always)]
        pub const fn set_opamprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "SPI2 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn spi2rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 reset"]
        #[inline(always)]
        pub const fn set_spi2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn spi3rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 reset"]
        #[inline(always)]
        pub const fn set_spi3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "USART2 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn usart2rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 reset"]
        #[inline(always)]
        pub const fn set_usart2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn usart3rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 reset"]
        #[inline(always)]
        pub const fn set_usart3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn uart4rst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 reset"]
        #[inline(always)]
        pub const fn set_uart4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn uart5rst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 reset"]
        #[inline(always)]
        pub const fn set_uart5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 reset"]
        #[inline(always)]
        pub const fn set_i2c1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c2rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 reset"]
        #[inline(always)]
        pub const fn set_i2c2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I3C1 block reset"]
        #[must_use]
        #[inline(always)]
        pub const fn i3c1rst(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I3C1 block reset"]
        #[inline(always)]
        pub const fn set_i3c1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "CRS reset"]
        #[must_use]
        #[inline(always)]
        pub const fn crsrst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "CRS reset"]
        #[inline(always)]
        pub const fn set_crsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "USART6 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn usart6rst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "USART6 reset"]
        #[inline(always)]
        pub const fn set_usart6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "UART7 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn uart7rst(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "UART7 reset"]
        #[inline(always)]
        pub const fn set_uart7rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
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
                .field("opamprst", &self.opamprst())
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
                .field("usart6rst", &self.usart6rst())
                .field("uart7rst", &self.uart7rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1lrstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb1lrstr {{ tim2rst: {=bool:?}, tim3rst: {=bool:?}, tim4rst: {=bool:?}, tim5rst: {=bool:?}, tim6rst: {=bool:?}, tim7rst: {=bool:?}, tim12rst: {=bool:?}, opamprst: {=bool:?}, spi2rst: {=bool:?}, spi3rst: {=bool:?}, usart2rst: {=bool:?}, usart3rst: {=bool:?}, uart4rst: {=bool:?}, uart5rst: {=bool:?}, i2c1rst: {=bool:?}, i2c2rst: {=bool:?}, i3c1rst: {=bool:?}, crsrst: {=bool:?}, usart6rst: {=bool:?}, uart7rst: {=bool:?} }}",
                self.tim2rst(),
                self.tim3rst(),
                self.tim4rst(),
                self.tim5rst(),
                self.tim6rst(),
                self.tim7rst(),
                self.tim12rst(),
                self.opamprst(),
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
                self.usart6rst(),
                self.uart7rst()
            )
        }
    }
    #[doc = "RCC APB2 peripheral clock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2enr(pub u32);
    impl Apb2enr {
        #[doc = "TIM1 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim1en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 clock enable"]
        #[inline(always)]
        pub const fn set_tim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
        #[doc = "TIM8 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim8en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 clock enable"]
        #[inline(always)]
        pub const fn set_tim8en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "USART1 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn usart1en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 clock enable"]
        #[inline(always)]
        pub const fn set_usart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TIM15 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim15en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 clock enable"]
        #[inline(always)]
        pub const fn set_tim15en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim16en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 clock enable"]
        #[inline(always)]
        pub const fn set_tim16en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn tim17en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 clock enable"]
        #[inline(always)]
        pub const fn set_tim17en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "USB clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn usben(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "USB clock enable"]
        #[inline(always)]
        pub const fn set_usben(&mut self, val: bool) {
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
                .field("tim8en", &self.tim8en())
                .field("usart1en", &self.usart1en())
                .field("tim15en", &self.tim15en())
                .field("tim16en", &self.tim16en())
                .field("tim17en", &self.tim17en())
                .field("usben", &self.usben())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2enr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2enr {{ tim1en: {=bool:?}, spi1en: {=bool:?}, tim8en: {=bool:?}, usart1en: {=bool:?}, tim15en: {=bool:?}, tim16en: {=bool:?}, tim17en: {=bool:?}, usben: {=bool:?} }}",
                self.tim1en(),
                self.spi1en(),
                self.tim8en(),
                self.usart1en(),
                self.tim15en(),
                self.tim16en(),
                self.tim17en(),
                self.usben()
            )
        }
    }
    #[doc = "RCC APB2 sleep clock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2lpenr(pub u32);
    impl Apb2lpenr {
        #[doc = "TIM1 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim1lpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_tim1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI1 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn spi1lpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_spi1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TIM8 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim8lpen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_tim8lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "USART1 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn usart1lpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_usart1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TIM15 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim15lpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_tim15lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim16lpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_tim16lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn tim17lpen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_tim17lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "USB clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn usblpen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "USB clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_usblpen(&mut self, val: bool) {
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
                .field("tim8lpen", &self.tim8lpen())
                .field("usart1lpen", &self.usart1lpen())
                .field("tim15lpen", &self.tim15lpen())
                .field("tim16lpen", &self.tim16lpen())
                .field("tim17lpen", &self.tim17lpen())
                .field("usblpen", &self.usblpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2lpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2lpenr {{ tim1lpen: {=bool:?}, spi1lpen: {=bool:?}, tim8lpen: {=bool:?}, usart1lpen: {=bool:?}, tim15lpen: {=bool:?}, tim16lpen: {=bool:?}, tim17lpen: {=bool:?}, usblpen: {=bool:?} }}",
                self.tim1lpen(),
                self.spi1lpen(),
                self.tim8lpen(),
                self.usart1lpen(),
                self.tim15lpen(),
                self.tim16lpen(),
                self.tim17lpen(),
                self.usblpen()
            )
        }
    }
    #[doc = "RCC APB2 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2rstr(pub u32);
    impl Apb2rstr {
        #[doc = "TIM1 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim1rst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 reset"]
        #[inline(always)]
        pub const fn set_tim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI1 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn spi1rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 reset"]
        #[inline(always)]
        pub const fn set_spi1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TIM8 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim8rst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 reset"]
        #[inline(always)]
        pub const fn set_tim8rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "USART1 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn usart1rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 reset"]
        #[inline(always)]
        pub const fn set_usart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TIM15 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim15rst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 reset"]
        #[inline(always)]
        pub const fn set_tim15rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim16rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 reset"]
        #[inline(always)]
        pub const fn set_tim16rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn tim17rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 reset"]
        #[inline(always)]
        pub const fn set_tim17rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "USB reset"]
        #[must_use]
        #[inline(always)]
        pub const fn usbrst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "USB reset"]
        #[inline(always)]
        pub const fn set_usbrst(&mut self, val: bool) {
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
                .field("tim8rst", &self.tim8rst())
                .field("usart1rst", &self.usart1rst())
                .field("tim15rst", &self.tim15rst())
                .field("tim16rst", &self.tim16rst())
                .field("tim17rst", &self.tim17rst())
                .field("usbrst", &self.usbrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2rstr {{ tim1rst: {=bool:?}, spi1rst: {=bool:?}, tim8rst: {=bool:?}, usart1rst: {=bool:?}, tim15rst: {=bool:?}, tim16rst: {=bool:?}, tim17rst: {=bool:?}, usbrst: {=bool:?} }}",
                self.tim1rst(),
                self.spi1rst(),
                self.tim8rst(),
                self.usart1rst(),
                self.tim15rst(),
                self.tim16rst(),
                self.tim17rst(),
                self.usbrst()
            )
        }
    }
    #[doc = "RCC APB3 peripheral clock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb3enr(pub u32);
    impl Apb3enr {
        #[doc = "SBS clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn sbsen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SBS clock enable"]
        #[inline(always)]
        pub const fn set_sbsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPUART1 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lpuart1en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 clock enable"]
        #[inline(always)]
        pub const fn set_lpuart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LPTIM1 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim1en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 clock enable"]
        #[inline(always)]
        pub const fn set_lptim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "RTC APB interface clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn rtcapben(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "RTC APB interface clock enable"]
        #[inline(always)]
        pub const fn set_rtcapben(&mut self, val: bool) {
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
                .field("sbsen", &self.sbsen())
                .field("lpuart1en", &self.lpuart1en())
                .field("lptim1en", &self.lptim1en())
                .field("rtcapben", &self.rtcapben())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb3enr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb3enr {{ sbsen: {=bool:?}, lpuart1en: {=bool:?}, lptim1en: {=bool:?}, rtcapben: {=bool:?} }}",
                self.sbsen(),
                self.lpuart1en(),
                self.lptim1en(),
                self.rtcapben()
            )
        }
    }
    #[doc = "RCC APB3 sleep clock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb3lpenr(pub u32);
    impl Apb3lpenr {
        #[doc = "SBS clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn sbslpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SBS clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_sbslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPUART1 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn lpuart1lpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_lpuart1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LPTIM1 clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim1lpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_lptim1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "RTC APB interface clock enable during Sleep mode"]
        #[must_use]
        #[inline(always)]
        pub const fn rtcapblpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "RTC APB interface clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn set_rtcapblpen(&mut self, val: bool) {
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
                .field("sbslpen", &self.sbslpen())
                .field("lpuart1lpen", &self.lpuart1lpen())
                .field("lptim1lpen", &self.lptim1lpen())
                .field("rtcapblpen", &self.rtcapblpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb3lpenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb3lpenr {{ sbslpen: {=bool:?}, lpuart1lpen: {=bool:?}, lptim1lpen: {=bool:?}, rtcapblpen: {=bool:?} }}",
                self.sbslpen(),
                self.lpuart1lpen(),
                self.lptim1lpen(),
                self.rtcapblpen()
            )
        }
    }
    #[doc = "RCC APB3 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb3rstr(pub u32);
    impl Apb3rstr {
        #[doc = "SBS reset"]
        #[must_use]
        #[inline(always)]
        pub const fn sbsrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SBS reset"]
        #[inline(always)]
        pub const fn set_sbsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPUART1 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn lpuart1rst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 reset"]
        #[inline(always)]
        pub const fn set_lpuart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LPTIM1 reset"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim1rst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 reset"]
        #[inline(always)]
        pub const fn set_lptim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
                .field("sbsrst", &self.sbsrst())
                .field("lpuart1rst", &self.lpuart1rst())
                .field("lptim1rst", &self.lptim1rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb3rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb3rstr {{ sbsrst: {=bool:?}, lpuart1rst: {=bool:?}, lptim1rst: {=bool:?} }}",
                self.sbsrst(),
                self.lpuart1rst(),
                self.lptim1rst()
            )
        }
    }
    #[doc = "RCC kernel clock configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccipr1(pub u32);
    impl Ccipr1 {
        #[doc = "USART1 kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn usart1sel(&self) -> super::vals::Usartsel {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Usartsel::from_bits(val as u8)
        }
        #[doc = "USART1 kernel clock source selection"]
        #[inline(always)]
        pub const fn set_usart1sel(&mut self, val: super::vals::Usartsel) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "USART2 kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn usart2sel(&self) -> super::vals::Usartsel {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Usartsel::from_bits(val as u8)
        }
        #[doc = "USART2 kernel clock source selection"]
        #[inline(always)]
        pub const fn set_usart2sel(&mut self, val: super::vals::Usartsel) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "UART3 kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn usart3sel(&self) -> super::vals::Usartsel {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Usartsel::from_bits(val as u8)
        }
        #[doc = "UART3 kernel clock source selection"]
        #[inline(always)]
        pub const fn set_usart3sel(&mut self, val: super::vals::Usartsel) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "UART4 kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn uart4sel(&self) -> super::vals::Uartsel {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::Uartsel::from_bits(val as u8)
        }
        #[doc = "UART4 kernel clock source selection"]
        #[inline(always)]
        pub const fn set_uart4sel(&mut self, val: super::vals::Uartsel) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "UART5 kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn uart5sel(&self) -> super::vals::Uartsel {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Uartsel::from_bits(val as u8)
        }
        #[doc = "UART5 kernel clock source selection"]
        #[inline(always)]
        pub const fn set_uart5sel(&mut self, val: super::vals::Uartsel) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "USART6 kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn usart6sel(&self) -> super::vals::Usartsel {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::Usartsel::from_bits(val as u8)
        }
        #[doc = "USART6 kernel clock source selection"]
        #[inline(always)]
        pub const fn set_usart6sel(&mut self, val: super::vals::Usartsel) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
        #[doc = "UART7 kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn uart7sel(&self) -> super::vals::Uartsel {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Uartsel::from_bits(val as u8)
        }
        #[doc = "UART7 kernel clock source selection"]
        #[inline(always)]
        pub const fn set_uart7sel(&mut self, val: super::vals::Uartsel) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "LPUART1 kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn lpuart1sel(&self) -> super::vals::Lpuartsel {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::Lpuartsel::from_bits(val as u8)
        }
        #[doc = "LPUART1 kernel clock source selection"]
        #[inline(always)]
        pub const fn set_lpuart1sel(&mut self, val: super::vals::Lpuartsel) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
        #[doc = "SPI1 kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn spi1sel(&self) -> super::vals::Spisel {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Spisel::from_bits(val as u8)
        }
        #[doc = "SPI1 kernel clock source selection"]
        #[inline(always)]
        pub const fn set_spi1sel(&mut self, val: super::vals::Spisel) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "SPI2 kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn spi2sel(&self) -> super::vals::Spisel {
            let val = (self.0 >> 18usize) & 0x03;
            super::vals::Spisel::from_bits(val as u8)
        }
        #[doc = "SPI2 kernel clock source selection"]
        #[inline(always)]
        pub const fn set_spi2sel(&mut self, val: super::vals::Spisel) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
        }
        #[doc = "SPI3 kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn spi3sel(&self) -> super::vals::Spisel {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::Spisel::from_bits(val as u8)
        }
        #[doc = "SPI3 kernel clock source selection"]
        #[inline(always)]
        pub const fn set_spi3sel(&mut self, val: super::vals::Spisel) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "FDCAN1 and FDCAN2 kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn fdcansel(&self) -> super::vals::Fdcansel {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::Fdcansel::from_bits(val as u8)
        }
        #[doc = "FDCAN1 and FDCAN2 kernel clock source selection"]
        #[inline(always)]
        pub const fn set_fdcansel(&mut self, val: super::vals::Fdcansel) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
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
                .field("usart6sel", &self.usart6sel())
                .field("uart7sel", &self.uart7sel())
                .field("lpuart1sel", &self.lpuart1sel())
                .field("spi1sel", &self.spi1sel())
                .field("spi2sel", &self.spi2sel())
                .field("spi3sel", &self.spi3sel())
                .field("fdcansel", &self.fdcansel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccipr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccipr1 {{ usart1sel: {:?}, usart2sel: {:?}, usart3sel: {:?}, uart4sel: {:?}, uart5sel: {:?}, usart6sel: {:?}, uart7sel: {:?}, lpuart1sel: {:?}, spi1sel: {:?}, spi2sel: {:?}, spi3sel: {:?}, fdcansel: {:?} }}",
                self.usart1sel(),
                self.usart2sel(),
                self.usart3sel(),
                self.uart4sel(),
                self.uart5sel(),
                self.usart6sel(),
                self.uart7sel(),
                self.lpuart1sel(),
                self.spi1sel(),
                self.spi2sel(),
                self.spi3sel(),
                self.fdcansel()
            )
        }
    }
    #[doc = "RCC kernel clock configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccipr2(pub u32);
    impl Ccipr2 {
        #[doc = "I2C1 kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1sel(&self) -> super::vals::Icsel {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Icsel::from_bits(val as u8)
        }
        #[doc = "I2C1 kernel clock source selection"]
        #[inline(always)]
        pub const fn set_i2c1sel(&mut self, val: super::vals::Icsel) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "I2C2 kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c2sel(&self) -> super::vals::Icsel {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Icsel::from_bits(val as u8)
        }
        #[doc = "I2C2 kernel clock source selection"]
        #[inline(always)]
        pub const fn set_i2c2sel(&mut self, val: super::vals::Icsel) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "I3C1 kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn i3c1sel(&self) -> super::vals::Icsel {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::Icsel::from_bits(val as u8)
        }
        #[doc = "I3C1 kernel clock source selection"]
        #[inline(always)]
        pub const fn set_i3c1sel(&mut self, val: super::vals::Icsel) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "ADC and DAC kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn adcdacsel(&self) -> super::vals::Adcdacsel {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::Adcdacsel::from_bits(val as u8)
        }
        #[doc = "ADC and DAC kernel clock source selection"]
        #[inline(always)]
        pub const fn set_adcdacsel(&mut self, val: super::vals::Adcdacsel) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
        #[doc = "ADC and DAC prescaler for kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn adcdacpre(&self) -> super::vals::Adcdacpre {
            let val = (self.0 >> 12usize) & 0x07;
            super::vals::Adcdacpre::from_bits(val as u8)
        }
        #[doc = "ADC and DAC prescaler for kernel clock source selection"]
        #[inline(always)]
        pub const fn set_adcdacpre(&mut self, val: super::vals::Adcdacpre) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
        }
        #[doc = "DAC sample and hold clock"]
        #[must_use]
        #[inline(always)]
        pub const fn dacsel(&self) -> super::vals::Dacsel {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Dacsel::from_bits(val as u8)
        }
        #[doc = "DAC sample and hold clock"]
        #[inline(always)]
        pub const fn set_dacsel(&mut self, val: super::vals::Dacsel) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "LPTIM1 kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn lptim1sel(&self) -> super::vals::Lptimsel {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Lptimsel::from_bits(val as u8)
        }
        #[doc = "LPTIM1 kernel clock source selection"]
        #[inline(always)]
        pub const fn set_lptim1sel(&mut self, val: super::vals::Lptimsel) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "CK48 clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn clk48sel(&self) -> super::vals::Clksel {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Clksel::from_bits(val as u8)
        }
        #[doc = "CK48 clock source selection"]
        #[inline(always)]
        pub const fn set_clk48sel(&mut self, val: super::vals::Clksel) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "SYSTICK clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn systiclksel(&self) -> super::vals::Systiclksel {
            let val = (self.0 >> 30usize) & 0x03;
            super::vals::Systiclksel::from_bits(val as u8)
        }
        #[doc = "SYSTICK clock source selection"]
        #[inline(always)]
        pub const fn set_systiclksel(&mut self, val: super::vals::Systiclksel) {
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
                .field("i2c1sel", &self.i2c1sel())
                .field("i2c2sel", &self.i2c2sel())
                .field("i3c1sel", &self.i3c1sel())
                .field("adcdacsel", &self.adcdacsel())
                .field("adcdacpre", &self.adcdacpre())
                .field("dacsel", &self.dacsel())
                .field("lptim1sel", &self.lptim1sel())
                .field("clk48sel", &self.clk48sel())
                .field("systiclksel", &self.systiclksel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccipr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccipr2 {{ i2c1sel: {:?}, i2c2sel: {:?}, i3c1sel: {:?}, adcdacsel: {:?}, adcdacpre: {:?}, dacsel: {:?}, lptim1sel: {:?}, clk48sel: {:?}, systiclksel: {:?} }}",
                self.i2c1sel(),
                self.i2c2sel(),
                self.i3c1sel(),
                self.adcdacsel(),
                self.adcdacpre(),
                self.dacsel(),
                self.lptim1sel(),
                self.clk48sel(),
                self.systiclksel()
            )
        }
    }
    #[doc = "RCC kernel clock configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccipr3(pub u32);
    impl Ccipr3 {
        #[doc = "XSPI1 kernel clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn xspi1sel(&self) -> super::vals::Xspisel {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Xspisel::from_bits(val as u8)
        }
        #[doc = "XSPI1 kernel clock source selection"]
        #[inline(always)]
        pub const fn set_xspi1sel(&mut self, val: super::vals::Xspisel) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "ETH RMII reference clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn ethrefclksel(&self) -> super::vals::Ethrefclksel {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Ethrefclksel::from_bits(val as u8)
        }
        #[doc = "ETH RMII reference clock source selection"]
        #[inline(always)]
        pub const fn set_ethrefclksel(&mut self, val: super::vals::Ethrefclksel) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "ETH PTP clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn ethptpclksel(&self) -> super::vals::Ethptpclksel {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::Ethptpclksel::from_bits(val as u8)
        }
        #[doc = "ETH PTP clock source selection"]
        #[inline(always)]
        pub const fn set_ethptpclksel(&mut self, val: super::vals::Ethptpclksel) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
        #[doc = "ETH clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn ethclksel(&self) -> super::vals::Ethclksel {
            let val = (self.0 >> 13usize) & 0x03;
            super::vals::Ethclksel::from_bits(val as u8)
        }
        #[doc = "ETH clock source selection"]
        #[inline(always)]
        pub const fn set_ethclksel(&mut self, val: super::vals::Ethclksel) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
        }
        #[doc = "Ethernet clock division"]
        #[must_use]
        #[inline(always)]
        pub const fn ethclkdiv(&self) -> super::vals::Ethclkdiv {
            let val = (self.0 >> 26usize) & 0x03;
            super::vals::Ethclkdiv::from_bits(val as u8)
        }
        #[doc = "Ethernet clock division"]
        #[inline(always)]
        pub const fn set_ethclkdiv(&mut self, val: super::vals::Ethclkdiv) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
        }
        #[doc = "Ethernet PTP clock division"]
        #[must_use]
        #[inline(always)]
        pub const fn ethptpdiv(&self) -> super::vals::Ethptpdiv {
            let val = (self.0 >> 28usize) & 0x0f;
            super::vals::Ethptpdiv::from_bits(val as u8)
        }
        #[doc = "Ethernet PTP clock division"]
        #[inline(always)]
        pub const fn set_ethptpdiv(&mut self, val: super::vals::Ethptpdiv) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
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
                .field("xspi1sel", &self.xspi1sel())
                .field("ethrefclksel", &self.ethrefclksel())
                .field("ethptpclksel", &self.ethptpclksel())
                .field("ethclksel", &self.ethclksel())
                .field("ethclkdiv", &self.ethclkdiv())
                .field("ethptpdiv", &self.ethptpdiv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccipr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccipr3 {{ xspi1sel: {:?}, ethrefclksel: {:?}, ethptpclksel: {:?}, ethclksel: {:?}, ethclkdiv: {:?}, ethptpdiv: {:?} }}",
                self.xspi1sel(),
                self.ethrefclksel(),
                self.ethptpclksel(),
                self.ethclksel(),
                self.ethclkdiv(),
                self.ethptpdiv()
            )
        }
    }
    #[doc = "RCC clock configuration register1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "System clock and trace clock switch"]
        #[must_use]
        #[inline(always)]
        pub const fn sw(&self) -> super::vals::Sw {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "System clock and trace clock switch"]
        #[inline(always)]
        pub const fn set_sw(&mut self, val: super::vals::Sw) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "System clock switch status"]
        #[must_use]
        #[inline(always)]
        pub const fn sws(&self) -> super::vals::Sw {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "System clock switch status"]
        #[inline(always)]
        pub const fn set_sws(&mut self, val: super::vals::Sw) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "System clock selection after a wake-up from system Stop mode"]
        #[must_use]
        #[inline(always)]
        pub const fn stopwuck(&self) -> super::vals::Stopwuck {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Stopwuck::from_bits(val as u8)
        }
        #[doc = "System clock selection after a wake-up from system Stop mode"]
        #[inline(always)]
        pub const fn set_stopwuck(&mut self, val: super::vals::Stopwuck) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "HSE division factor for RTC clock (source of HSE_1MHz clock)"]
        #[must_use]
        #[inline(always)]
        pub const fn rtcpre(&self) -> u16 {
            let val = (self.0 >> 7usize) & 0x01ff;
            val as u16
        }
        #[doc = "HSE division factor for RTC clock (source of HSE_1MHz clock)"]
        #[inline(always)]
        pub const fn set_rtcpre(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 7usize)) | (((val as u32) & 0x01ff) << 7usize);
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
        #[doc = "Microcontroller clock output 1"]
        #[must_use]
        #[inline(always)]
        pub const fn mco1sel(&self) -> super::vals::Mcosel {
            let val = (self.0 >> 22usize) & 0x07;
            super::vals::Mcosel::from_bits(val as u8)
        }
        #[doc = "Microcontroller clock output 1"]
        #[inline(always)]
        pub const fn set_mco1sel(&mut self, val: super::vals::Mcosel) {
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
        #[doc = "Microcontroller clock output 2"]
        #[must_use]
        #[inline(always)]
        pub const fn mco2sel(&self) -> super::vals::Mcosel {
            let val = (self.0 >> 29usize) & 0x07;
            super::vals::Mcosel::from_bits(val as u8)
        }
        #[doc = "Microcontroller clock output 2"]
        #[inline(always)]
        pub const fn set_mco2sel(&mut self, val: super::vals::Mcosel) {
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
                "Cfgr {{ sw: {:?}, sws: {:?}, stopwuck: {:?}, rtcpre: {=u16:?}, mco1pre: {:?}, mco1sel: {:?}, mco2pre: {:?}, mco2sel: {:?} }}",
                self.sw(),
                self.sws(),
                self.stopwuck(),
                self.rtcpre(),
                self.mco1pre(),
                self.mco1sel(),
                self.mco2pre(),
                self.mco2sel()
            )
        }
    }
    #[doc = "RCC CPU domain clock configuration register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "AHB prescaler"]
        #[must_use]
        #[inline(always)]
        pub const fn hpre(&self) -> super::vals::Hpre {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Hpre::from_bits(val as u8)
        }
        #[doc = "AHB prescaler"]
        #[inline(always)]
        pub const fn set_hpre(&mut self, val: super::vals::Hpre) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "APB low-speed prescaler (APB1)"]
        #[must_use]
        #[inline(always)]
        pub const fn ppre1(&self) -> super::vals::Ppre {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB low-speed prescaler (APB1)"]
        #[inline(always)]
        pub const fn set_ppre1(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "APB high-speed prescaler (APB2)"]
        #[must_use]
        #[inline(always)]
        pub const fn ppre2(&self) -> super::vals::Ppre {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB high-speed prescaler (APB2)"]
        #[inline(always)]
        pub const fn set_ppre2(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "APB low-speed prescaler (APB3)"]
        #[must_use]
        #[inline(always)]
        pub const fn ppre3(&self) -> super::vals::Ppre {
            let val = (self.0 >> 12usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB low-speed prescaler (APB3)"]
        #[inline(always)]
        pub const fn set_ppre3(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
        }
        #[doc = "AHB1 clock disable"]
        #[must_use]
        #[inline(always)]
        pub const fn ahb1dis(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "AHB1 clock disable"]
        #[inline(always)]
        pub const fn set_ahb1dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "AHB2 clock disable"]
        #[must_use]
        #[inline(always)]
        pub const fn ahb2dis(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "AHB2 clock disable"]
        #[inline(always)]
        pub const fn set_ahb2dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "AHB4 clock disable"]
        #[must_use]
        #[inline(always)]
        pub const fn ahb4dis(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "AHB4 clock disable"]
        #[inline(always)]
        pub const fn set_ahb4dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "APB1 clock disable value"]
        #[must_use]
        #[inline(always)]
        pub const fn apb1dis(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "APB1 clock disable value"]
        #[inline(always)]
        pub const fn set_apb1dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "APB2 clock disable value"]
        #[must_use]
        #[inline(always)]
        pub const fn apb2dis(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "APB2 clock disable value"]
        #[inline(always)]
        pub const fn set_apb2dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "APB3 clock disable value"]
        #[must_use]
        #[inline(always)]
        pub const fn apb3dis(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "APB3 clock disable value"]
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
            defmt::write!(
                f,
                "Cfgr2 {{ hpre: {:?}, ppre1: {:?}, ppre2: {:?}, ppre3: {:?}, ahb1dis: {=bool:?}, ahb2dis: {=bool:?}, ahb4dis: {=bool:?}, apb1dis: {=bool:?}, apb2dis: {=bool:?}, apb3dis: {=bool:?} }}",
                self.hpre(),
                self.ppre1(),
                self.ppre2(),
                self.ppre3(),
                self.ahb1dis(),
                self.ahb2dis(),
                self.ahb4dis(),
                self.apb1dis(),
                self.apb2dis(),
                self.apb3dis()
            )
        }
    }
    #[doc = "RCC clock source interrupt clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cicr(pub u32);
    impl Cicr {
        #[doc = "LSI ready interrupt clear"]
        #[must_use]
        #[inline(always)]
        pub const fn lsirdyc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt clear"]
        #[inline(always)]
        pub const fn set_lsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt clear"]
        #[must_use]
        #[inline(always)]
        pub const fn lserdyc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt clear"]
        #[inline(always)]
        pub const fn set_lserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI ready interrupt clear"]
        #[must_use]
        #[inline(always)]
        pub const fn hsirdyc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt clear"]
        #[inline(always)]
        pub const fn set_hsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSIDIV3 ready interrupt clear"]
        #[must_use]
        #[inline(always)]
        pub const fn hsidiv3rdyc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSIDIV3 ready interrupt clear"]
        #[inline(always)]
        pub const fn set_hsidiv3rdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HSIK ready interrupt clear"]
        #[must_use]
        #[inline(always)]
        pub const fn hsikrdyc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HSIK ready interrupt clear"]
        #[inline(always)]
        pub const fn set_hsikrdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "PSI ready interrupt clear"]
        #[must_use]
        #[inline(always)]
        pub const fn psirdyc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "PSI ready interrupt clear"]
        #[inline(always)]
        pub const fn set_psirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PSIDIV3 ready interrupt clear"]
        #[must_use]
        #[inline(always)]
        pub const fn psidiv3rdyc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PSIDIV3 ready interrupt clear"]
        #[inline(always)]
        pub const fn set_psidiv3rdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "PSIK ready interrupt clear"]
        #[must_use]
        #[inline(always)]
        pub const fn psikrdyc(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "PSIK ready interrupt clear"]
        #[inline(always)]
        pub const fn set_psikrdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "HSE ready interrupt clear"]
        #[must_use]
        #[inline(always)]
        pub const fn hserdyc(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt clear"]
        #[inline(always)]
        pub const fn set_hserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "HSE clock security system interrupt clear"]
        #[must_use]
        #[inline(always)]
        pub const fn hsecssc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock security system interrupt clear"]
        #[inline(always)]
        pub const fn set_hsecssc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LSE clock security system interrupt clear"]
        #[must_use]
        #[inline(always)]
        pub const fn lsecssc(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock security system interrupt clear"]
        #[inline(always)]
        pub const fn set_lsecssc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
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
                .field("hsidiv3rdyc", &self.hsidiv3rdyc())
                .field("hsikrdyc", &self.hsikrdyc())
                .field("psirdyc", &self.psirdyc())
                .field("psidiv3rdyc", &self.psidiv3rdyc())
                .field("psikrdyc", &self.psikrdyc())
                .field("hserdyc", &self.hserdyc())
                .field("hsecssc", &self.hsecssc())
                .field("lsecssc", &self.lsecssc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cicr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cicr {{ lsirdyc: {=bool:?}, lserdyc: {=bool:?}, hsirdyc: {=bool:?}, hsidiv3rdyc: {=bool:?}, hsikrdyc: {=bool:?}, psirdyc: {=bool:?}, psidiv3rdyc: {=bool:?}, psikrdyc: {=bool:?}, hserdyc: {=bool:?}, hsecssc: {=bool:?}, lsecssc: {=bool:?} }}",
                self.lsirdyc(),
                self.lserdyc(),
                self.hsirdyc(),
                self.hsidiv3rdyc(),
                self.hsikrdyc(),
                self.psirdyc(),
                self.psidiv3rdyc(),
                self.psikrdyc(),
                self.hserdyc(),
                self.hsecssc(),
                self.lsecssc()
            )
        }
    }
    #[doc = "RCC clock source interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cier(pub u32);
    impl Cier {
        #[doc = "LSI ready interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lsirdyie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt enable"]
        #[inline(always)]
        pub const fn set_lsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lserdyie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt enable"]
        #[inline(always)]
        pub const fn set_lserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI ready interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn hsirdyie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt enable"]
        #[inline(always)]
        pub const fn set_hsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSIDIV3 ready interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn hsidiv3rdyie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSIDIV3 ready interrupt enable"]
        #[inline(always)]
        pub const fn set_hsidiv3rdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HSIK ready interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn hsikrdyie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HSIK ready interrupt enable"]
        #[inline(always)]
        pub const fn set_hsikrdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "PSI ready interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn psirdyie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "PSI ready interrupt enable"]
        #[inline(always)]
        pub const fn set_psirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PSIDIV3 ready interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn psidiv3rdyie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PSIDIV3 ready interrupt enable"]
        #[inline(always)]
        pub const fn set_psidiv3rdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "PSIK ready interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn psikrdyie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "PSIK ready interrupt enable"]
        #[inline(always)]
        pub const fn set_psikrdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "HSE ready interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn hserdyie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt enable"]
        #[inline(always)]
        pub const fn set_hserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
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
                .field("hsidiv3rdyie", &self.hsidiv3rdyie())
                .field("hsikrdyie", &self.hsikrdyie())
                .field("psirdyie", &self.psirdyie())
                .field("psidiv3rdyie", &self.psidiv3rdyie())
                .field("psikrdyie", &self.psikrdyie())
                .field("hserdyie", &self.hserdyie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cier {{ lsirdyie: {=bool:?}, lserdyie: {=bool:?}, hsirdyie: {=bool:?}, hsidiv3rdyie: {=bool:?}, hsikrdyie: {=bool:?}, psirdyie: {=bool:?}, psidiv3rdyie: {=bool:?}, psikrdyie: {=bool:?}, hserdyie: {=bool:?} }}",
                self.lsirdyie(),
                self.lserdyie(),
                self.hsirdyie(),
                self.hsidiv3rdyie(),
                self.hsikrdyie(),
                self.psirdyie(),
                self.psidiv3rdyie(),
                self.psikrdyie(),
                self.hserdyie()
            )
        }
    }
    #[doc = "RCC clock source interrupt flag register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cifr(pub u32);
    impl Cifr {
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
        #[doc = "HSIDIV3 ready interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn hsidiv3rdyf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSIDIV3 ready interrupt flag"]
        #[inline(always)]
        pub const fn set_hsidiv3rdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HSIK ready interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn hsikrdyf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HSIK ready interrupt flag"]
        #[inline(always)]
        pub const fn set_hsikrdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "PSI ready interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn psirdyf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "PSI ready interrupt flag"]
        #[inline(always)]
        pub const fn set_psirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PSIDIV3 ready interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn psidiv3rdyf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PSIDIV3 ready interrupt flag"]
        #[inline(always)]
        pub const fn set_psidiv3rdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "PSIK ready interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn psikrdyf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "PSIK ready interrupt flag"]
        #[inline(always)]
        pub const fn set_psikrdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "HSE ready interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn hserdyf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt flag"]
        #[inline(always)]
        pub const fn set_hserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "HSE clock security system interrupt flag"]
        #[must_use]
        #[inline(always)]
        pub const fn hsecssf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock security system interrupt flag"]
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
                .field("hsidiv3rdyf", &self.hsidiv3rdyf())
                .field("hsikrdyf", &self.hsikrdyf())
                .field("psirdyf", &self.psirdyf())
                .field("psidiv3rdyf", &self.psidiv3rdyf())
                .field("psikrdyf", &self.psikrdyf())
                .field("hserdyf", &self.hserdyf())
                .field("hsecssf", &self.hsecssf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cifr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cifr {{ lsirdyf: {=bool:?}, lserdyf: {=bool:?}, hsirdyf: {=bool:?}, hsidiv3rdyf: {=bool:?}, hsikrdyf: {=bool:?}, psirdyf: {=bool:?}, psidiv3rdyf: {=bool:?}, psikrdyf: {=bool:?}, hserdyf: {=bool:?}, hsecssf: {=bool:?} }}",
                self.lsirdyf(),
                self.lserdyf(),
                self.hsirdyf(),
                self.hsidiv3rdyf(),
                self.hsikrdyf(),
                self.psirdyf(),
                self.psidiv3rdyf(),
                self.psikrdyf(),
                self.hserdyf(),
                self.hsecssf()
            )
        }
    }
    #[doc = "RCC clock control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "HSI clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn hsion(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "HSI clock enable"]
        #[inline(always)]
        pub const fn set_hsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "HSIDIV3 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn hsidiv3on(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "HSIDIV3 clock enable"]
        #[inline(always)]
        pub const fn set_hsidiv3on(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSIK clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn hsikon(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSIK clock enable"]
        #[inline(always)]
        pub const fn set_hsikon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSI clock enable in Stop mode"]
        #[must_use]
        #[inline(always)]
        pub const fn hsikeron(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSI clock enable in Stop mode"]
        #[inline(always)]
        pub const fn set_hsikeron(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HSI clock ready flag"]
        #[must_use]
        #[inline(always)]
        pub const fn hsirdy(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HSI clock ready flag"]
        #[inline(always)]
        pub const fn set_hsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "HSIDIV3 clock ready flag"]
        #[must_use]
        #[inline(always)]
        pub const fn hsidiv3rdy(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "HSIDIV3 clock ready flag"]
        #[inline(always)]
        pub const fn set_hsidiv3rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "HSIK clock ready flag"]
        #[must_use]
        #[inline(always)]
        pub const fn hsikrdy(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "HSIK clock ready flag"]
        #[inline(always)]
        pub const fn set_hsikrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "PSI clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn psion(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "PSI clock enable"]
        #[inline(always)]
        pub const fn set_psion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "PSIDIV3 clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn psidiv3on(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "PSIDIV3 clock enable"]
        #[inline(always)]
        pub const fn set_psidiv3on(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "PSIK clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn psikon(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "PSIK clock enable"]
        #[inline(always)]
        pub const fn set_psikon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "PSI clock enable in Stop mode"]
        #[must_use]
        #[inline(always)]
        pub const fn psikeron(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "PSI clock enable in Stop mode"]
        #[inline(always)]
        pub const fn set_psikeron(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "PSI clock ready flag"]
        #[must_use]
        #[inline(always)]
        pub const fn psirdy(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "PSI clock ready flag"]
        #[inline(always)]
        pub const fn set_psirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "PSIDIV3 clock ready flag"]
        #[must_use]
        #[inline(always)]
        pub const fn psidiv3rdy(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "PSIDIV3 clock ready flag"]
        #[inline(always)]
        pub const fn set_psidiv3rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "PSIK clock ready flag"]
        #[must_use]
        #[inline(always)]
        pub const fn psikrdy(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "PSIK clock ready flag"]
        #[inline(always)]
        pub const fn set_psikrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
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
        #[doc = "HSE clock security system enable"]
        #[must_use]
        #[inline(always)]
        pub const fn hsecsson(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock security system enable"]
        #[inline(always)]
        pub const fn set_hsecsson(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "External high speed clock type in Bypass mode"]
        #[must_use]
        #[inline(always)]
        pub const fn hseext(&self) -> super::vals::Hseext {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Hseext::from_bits(val as u8)
        }
        #[doc = "External high speed clock type in Bypass mode"]
        #[inline(always)]
        pub const fn set_hseext(&mut self, val: super::vals::Hseext) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
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
                .field("hsidiv3on", &self.hsidiv3on())
                .field("hsikon", &self.hsikon())
                .field("hsikeron", &self.hsikeron())
                .field("hsirdy", &self.hsirdy())
                .field("hsidiv3rdy", &self.hsidiv3rdy())
                .field("hsikrdy", &self.hsikrdy())
                .field("psion", &self.psion())
                .field("psidiv3on", &self.psidiv3on())
                .field("psikon", &self.psikon())
                .field("psikeron", &self.psikeron())
                .field("psirdy", &self.psirdy())
                .field("psidiv3rdy", &self.psidiv3rdy())
                .field("psikrdy", &self.psikrdy())
                .field("hseon", &self.hseon())
                .field("hserdy", &self.hserdy())
                .field("hsebyp", &self.hsebyp())
                .field("hsecsson", &self.hsecsson())
                .field("hseext", &self.hseext())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ hsion: {=bool:?}, hsidiv3on: {=bool:?}, hsikon: {=bool:?}, hsikeron: {=bool:?}, hsirdy: {=bool:?}, hsidiv3rdy: {=bool:?}, hsikrdy: {=bool:?}, psion: {=bool:?}, psidiv3on: {=bool:?}, psikon: {=bool:?}, psikeron: {=bool:?}, psirdy: {=bool:?}, psidiv3rdy: {=bool:?}, psikrdy: {=bool:?}, hseon: {=bool:?}, hserdy: {=bool:?}, hsebyp: {=bool:?}, hsecsson: {=bool:?}, hseext: {:?} }}",
                self.hsion(),
                self.hsidiv3on(),
                self.hsikon(),
                self.hsikeron(),
                self.hsirdy(),
                self.hsidiv3rdy(),
                self.hsikrdy(),
                self.psion(),
                self.psidiv3on(),
                self.psikon(),
                self.psikeron(),
                self.psirdy(),
                self.psidiv3rdy(),
                self.psikrdy(),
                self.hseon(),
                self.hserdy(),
                self.hsebyp(),
                self.hsecsson(),
                self.hseext()
            )
        }
    }
    #[doc = "RCC clock control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "HSI clock out divider factor"]
        #[must_use]
        #[inline(always)]
        pub const fn hsikdiv(&self) -> super::vals::Hsikdiv {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Hsikdiv::from_bits(val as u8)
        }
        #[doc = "HSI clock out divider factor"]
        #[inline(always)]
        pub const fn set_hsikdiv(&mut self, val: super::vals::Hsikdiv) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "PSI clock out divider factor"]
        #[must_use]
        #[inline(always)]
        pub const fn psikdiv(&self) -> super::vals::Psikdiv {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::Psikdiv::from_bits(val as u8)
        }
        #[doc = "PSI clock out divider factor"]
        #[inline(always)]
        pub const fn set_psikdiv(&mut self, val: super::vals::Psikdiv) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
        }
        #[doc = "PSI reference clock source selection"]
        #[must_use]
        #[inline(always)]
        pub const fn psirefsrc(&self) -> super::vals::Psirefsrc {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Psirefsrc::from_bits(val as u8)
        }
        #[doc = "PSI reference clock source selection"]
        #[inline(always)]
        pub const fn set_psirefsrc(&mut self, val: super::vals::Psirefsrc) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "PSI reference clock frequency selection"]
        #[must_use]
        #[inline(always)]
        pub const fn psiref(&self) -> super::vals::Psiref {
            let val = (self.0 >> 20usize) & 0x07;
            super::vals::Psiref::from_bits(val as u8)
        }
        #[doc = "PSI reference clock frequency selection"]
        #[inline(always)]
        pub const fn set_psiref(&mut self, val: super::vals::Psiref) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
        }
        #[doc = "PSI target frequency configuration"]
        #[must_use]
        #[inline(always)]
        pub const fn psifreq(&self) -> super::vals::Psifreq {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Psifreq::from_bits(val as u8)
        }
        #[doc = "PSI target frequency configuration"]
        #[inline(always)]
        pub const fn set_psifreq(&mut self, val: super::vals::Psifreq) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Cr2 {
        #[inline(always)]
        fn default() -> Cr2 {
            Cr2(0)
        }
    }
    impl core::fmt::Debug for Cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr2")
                .field("hsikdiv", &self.hsikdiv())
                .field("psikdiv", &self.psikdiv())
                .field("psirefsrc", &self.psirefsrc())
                .field("psiref", &self.psiref())
                .field("psifreq", &self.psifreq())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr2 {{ hsikdiv: {:?}, psikdiv: {:?}, psirefsrc: {:?}, psiref: {:?}, psifreq: {:?} }}",
                self.hsikdiv(),
                self.psikdiv(),
                self.psirefsrc(),
                self.psiref(),
                self.psifreq()
            )
        }
    }
    #[doc = "RCC privilege configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcfgr(pub u32);
    impl Privcfgr {
        #[doc = "RCC function privileged configuration"]
        #[must_use]
        #[inline(always)]
        pub const fn priv_(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RCC function privileged configuration"]
        #[inline(always)]
        pub const fn set_priv_(&mut self, val: bool) {
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
            f.debug_struct("Privcfgr").field("priv_", &self.priv_()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Privcfgr {{ priv_: {=bool:?} }}", self.priv_())
        }
    }
    #[doc = "RCC reset status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rsr(pub u32);
    impl Rsr {
        #[doc = "Remove reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn rmvf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Remove reset flag"]
        #[inline(always)]
        pub const fn set_rmvf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Pin reset flag (NRST)"]
        #[must_use]
        #[inline(always)]
        pub const fn pinrstf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Pin reset flag (NRST)"]
        #[inline(always)]
        pub const fn set_pinrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "POR reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn borrstf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "POR reset flag"]
        #[inline(always)]
        pub const fn set_borrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "System reset from CPU reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn sftrstf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "System reset from CPU reset flag"]
        #[inline(always)]
        pub const fn set_sftrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Independent watchdog reset flag"]
        #[must_use]
        #[inline(always)]
        pub const fn iwdgrstf(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog reset flag"]
        #[inline(always)]
        pub const fn set_iwdgrstf(&mut self, val: bool) {
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
            defmt::write!(
                f,
                "Rsr {{ rmvf: {=bool:?}, pinrstf: {=bool:?}, borrstf: {=bool:?}, sftrstf: {=bool:?}, iwdgrstf: {=bool:?}, wwdgrstf: {=bool:?}, lpwrrstf: {=bool:?} }}",
                self.rmvf(),
                self.pinrstf(),
                self.borrstf(),
                self.sftrstf(),
                self.iwdgrstf(),
                self.wwdgrstf(),
                self.lpwrrstf()
            )
        }
    }
    #[doc = "RCC RTC domain control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rtccr(pub u32);
    impl Rtccr {
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
        #[doc = "Low-speed external clock type in bypass mode"]
        #[must_use]
        #[inline(always)]
        pub const fn lseext(&self) -> super::vals::Lseext {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Lseext::from_bits(val as u8)
        }
        #[doc = "Low-speed external clock type in bypass mode"]
        #[inline(always)]
        pub const fn set_lseext(&mut self, val: super::vals::Lseext) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
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
        #[doc = "RTC domain software reset"]
        #[must_use]
        #[inline(always)]
        pub const fn rtcdrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "RTC domain software reset"]
        #[inline(always)]
        pub const fn set_rtcdrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Low-speed clock output (LSCO) enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lscoen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Low-speed clock output (LSCO) enable"]
        #[inline(always)]
        pub const fn set_lscoen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Low-speed clock output selection"]
        #[must_use]
        #[inline(always)]
        pub const fn lscosel(&self) -> super::vals::Lscosel {
            let val = (self.0 >> 25usize) & 0x01;
            super::vals::Lscosel::from_bits(val as u8)
        }
        #[doc = "Low-speed clock output selection"]
        #[inline(always)]
        pub const fn set_lscosel(&mut self, val: super::vals::Lscosel) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
        }
        #[doc = "LSI oscillator enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lsion(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "LSI oscillator enable"]
        #[inline(always)]
        pub const fn set_lsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "LSI oscillator ready"]
        #[must_use]
        #[inline(always)]
        pub const fn lsirdy(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "LSI oscillator ready"]
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
    impl core::fmt::Debug for Rtccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rtccr")
                .field("lseon", &self.lseon())
                .field("lserdy", &self.lserdy())
                .field("lsebyp", &self.lsebyp())
                .field("lsedrv", &self.lsedrv())
                .field("lsecsson", &self.lsecsson())
                .field("lsecssd", &self.lsecssd())
                .field("lseext", &self.lseext())
                .field("rtcsel", &self.rtcsel())
                .field("rtcen", &self.rtcen())
                .field("rtcdrst", &self.rtcdrst())
                .field("lscoen", &self.lscoen())
                .field("lscosel", &self.lscosel())
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
                "Rtccr {{ lseon: {=bool:?}, lserdy: {=bool:?}, lsebyp: {=bool:?}, lsedrv: {:?}, lsecsson: {=bool:?}, lsecssd: {=bool:?}, lseext: {:?}, rtcsel: {:?}, rtcen: {=bool:?}, rtcdrst: {=bool:?}, lscoen: {=bool:?}, lscosel: {:?}, lsion: {=bool:?}, lsirdy: {=bool:?} }}",
                self.lseon(),
                self.lserdy(),
                self.lsebyp(),
                self.lsedrv(),
                self.lsecsson(),
                self.lsecssd(),
                self.lseext(),
                self.rtcsel(),
                self.rtcen(),
                self.rtcdrst(),
                self.lscoen(),
                self.lscosel(),
                self.lsion(),
                self.lsirdy()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adcdacpre {
        #[doc = "ADC and DAC kernel clock are not divided (default after reset)"]
        Div1 = 0x0,
        #[doc = "ADC and DAC kernel clock divided by 2"]
        Div2 = 0x01,
        #[doc = "ADC and DAC kernel clock divided by 4"]
        Div4 = 0x02,
        #[doc = "ADC and DAC kernel clock divided by 3"]
        Div8 = 0x03,
        #[doc = "ADC and DAC kernel clock divided by 4"]
        Div16 = 0x04,
        #[doc = "ADC and DAC kernel clock divided by 5"]
        Div32 = 0x05,
        #[doc = "ADC and DAC kernel clock divided by 6"]
        Div64 = 0x06,
        #[doc = "ADC and DAC kernel clock divided by 7"]
        Div128 = 0x07,
    }
    impl Adcdacpre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adcdacpre {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adcdacpre {
        #[inline(always)]
        fn from(val: u8) -> Adcdacpre {
            Adcdacpre::from_bits(val)
        }
    }
    impl From<Adcdacpre> for u8 {
        #[inline(always)]
        fn from(val: Adcdacpre) -> u8 {
            Adcdacpre::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adcdacsel {
        #[doc = "rcc_hclk selected as kernel clock"]
        Hclk1 = 0x0,
        #[doc = "psi_ck selected as kernel clock (default after reset)"]
        Psi = 0x01,
        #[doc = "psik_ck selected as kernel clock"]
        Psik = 0x02,
        #[doc = "hsik_ck selected as kernel clock"]
        Hsik = 0x03,
    }
    impl Adcdacsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adcdacsel {
            unsafe { core::mem::transmute(val & 0x03) }
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
    pub enum Clksel {
        _RESERVED_0 = 0x0,
        #[doc = "psi_div_3_ck selected as kernel clock"]
        PsiDiv3 = 0x01,
        #[doc = "hsi_div_3_ck selected as kernel clock"]
        HsiDiv3 = 0x02,
        #[doc = "hse_ck selected as kernel clock"]
        Hse = 0x03,
    }
    impl Clksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Clksel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Clksel {
        #[inline(always)]
        fn from(val: u8) -> Clksel {
            Clksel::from_bits(val)
        }
    }
    impl From<Clksel> for u8 {
        #[inline(always)]
        fn from(val: Clksel) -> u8 {
            Clksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dacsel {
        #[doc = "lse_ck selected as sample and hold clock (default after reset)"]
        Lse = 0x0,
        #[doc = "lsi_ck selected as sample and hold clock"]
        Lsi = 0x01,
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
    pub enum Ethclkdiv {
        #[doc = "divided by 1(default after reset)"]
        Div1 = 0x0,
        #[doc = "divided by 2"]
        Div2 = 0x01,
        #[doc = "divided by 4"]
        Div4 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ethclkdiv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ethclkdiv {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ethclkdiv {
        #[inline(always)]
        fn from(val: u8) -> Ethclkdiv {
            Ethclkdiv::from_bits(val)
        }
    }
    impl From<Ethclkdiv> for u8 {
        #[inline(always)]
        fn from(val: Ethclkdiv) -> u8 {
            Ethclkdiv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ethclksel {
        _RESERVED_0 = 0x0,
        #[doc = "psi_ck selected as kernel clock"]
        Psi = 0x01,
        #[doc = "psik_ck selected as kernel clock"]
        Psik = 0x02,
        #[doc = "hse_ck selected as kernel clock"]
        Hse = 0x03,
    }
    impl Ethclksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ethclksel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ethclksel {
        #[inline(always)]
        fn from(val: u8) -> Ethclksel {
            Ethclksel::from_bits(val)
        }
    }
    impl From<Ethclksel> for u8 {
        #[inline(always)]
        fn from(val: Ethclksel) -> u8 {
            Ethclksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ethptpclksel {
        _RESERVED_0 = 0x0,
        #[doc = "rcc_hclk1 selected as kernel clock"]
        Hclk1 = 0x01,
        #[doc = "psi_ck selected as kernel clock"]
        Psi = 0x02,
        #[doc = "psik_ck selected as kernel clock"]
        Psik = 0x03,
    }
    impl Ethptpclksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ethptpclksel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ethptpclksel {
        #[inline(always)]
        fn from(val: u8) -> Ethptpclksel {
            Ethptpclksel::from_bits(val)
        }
    }
    impl From<Ethptpclksel> for u8 {
        #[inline(always)]
        fn from(val: Ethptpclksel) -> u8 {
            Ethptpclksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ethptpdiv {
        #[doc = "divided by 1(default after reset)"]
        Div1 = 0x0,
        #[doc = "divided by 2"]
        Div2 = 0x01,
        #[doc = "divided by 3"]
        Div3 = 0x02,
        #[doc = "divided by 4"]
        Div4 = 0x03,
        #[doc = "divided by 5"]
        Div5 = 0x04,
        #[doc = "divided by 6"]
        Div6 = 0x05,
        #[doc = "divided by 7"]
        Div7 = 0x06,
        #[doc = "divided by 8"]
        Div8 = 0x07,
        #[doc = "divided by 9"]
        Div9 = 0x08,
        #[doc = "divided by 10"]
        Div10 = 0x09,
        #[doc = "divided by 11"]
        Div11 = 0x0a,
        #[doc = "divided by 12"]
        Div12 = 0x0b,
        #[doc = "divided by 13"]
        Div13 = 0x0c,
        #[doc = "divided by 14"]
        Div14 = 0x0d,
        #[doc = "divided by 15"]
        Div15 = 0x0e,
        #[doc = "divided by 16"]
        Div16 = 0x0f,
    }
    impl Ethptpdiv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ethptpdiv {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ethptpdiv {
        #[inline(always)]
        fn from(val: u8) -> Ethptpdiv {
            Ethptpdiv::from_bits(val)
        }
    }
    impl From<Ethptpdiv> for u8 {
        #[inline(always)]
        fn from(val: Ethptpdiv) -> u8 {
            Ethptpdiv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ethrefclksel {
        #[doc = "ETH_RMII_REF (default after reset)"]
        EthRmiiRef = 0x0,
        #[doc = "eth_clk_fb"]
        EthClkFb = 0x01,
    }
    impl Ethrefclksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ethrefclksel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ethrefclksel {
        #[inline(always)]
        fn from(val: u8) -> Ethrefclksel {
            Ethrefclksel::from_bits(val)
        }
    }
    impl From<Ethrefclksel> for u8 {
        #[inline(always)]
        fn from(val: Ethrefclksel) -> u8 {
            Ethrefclksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fdcansel {
        #[doc = "rcc_pclk1 selected as kernel clock (default after reset)"]
        Pclk1 = 0x0,
        #[doc = "psi_ck selected as kernel clock"]
        Psi = 0x01,
        #[doc = "psik_ck selected as kernel clock"]
        Psik = 0x02,
        #[doc = "hse_ck selected as kernel clock"]
        Hse = 0x03,
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
        #[doc = "rcc_hclk = sys_ck"]
        Div1 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "rcc_hclk = sys_ck / 2"]
        Div2 = 0x08,
        #[doc = "rcc_hclk = sys_ck / 4"]
        Div4 = 0x09,
        #[doc = "rcc_hclk = sys_ck / 8"]
        Div8 = 0x0a,
        #[doc = "rcc_hclk = sys_ck / 16"]
        Div16 = 0x0b,
        #[doc = "rcc_hclk = sys_ck / 64"]
        Div64 = 0x0c,
        #[doc = "rcc_hclk = sys_ck / 128"]
        Div128 = 0x0d,
        #[doc = "rcc_hclk = sys_ck / 256"]
        Div256 = 0x0e,
        #[doc = "rcc_hclk = sys_ck / 512"]
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
    pub enum Hseext {
        #[doc = "HSE in analog mode (default after reset)"]
        Analog = 0x0,
        #[doc = "HSE in digital mode"]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hsikdiv {
        #[doc = "HSI clock divided by 1"]
        Div1 = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "HSI clock divided by 1.5"]
        Div15 = 0x02,
        #[doc = "HSI clock divided by 2"]
        Div2 = 0x03,
        #[doc = "HSI clock divided by 2.5"]
        Div25 = 0x04,
        #[doc = "HSI clock divided by 3"]
        Div3 = 0x05,
        #[doc = "HSI clock divided by 3.5"]
        Div35 = 0x06,
        #[doc = "HSI clock divided by 4"]
        Div4 = 0x07,
        #[doc = "HSI clock divided by 4.5"]
        Div45 = 0x08,
        #[doc = "HSI clock divided by 5"]
        Div5 = 0x09,
        #[doc = "HSI clock divided by 5.5"]
        Div55 = 0x0a,
        #[doc = "HSI clock divided by 6"]
        Div6 = 0x0b,
        #[doc = "HSI clock divided by 6.5"]
        Div65 = 0x0c,
        #[doc = "HSI clock divided by 7"]
        Div7 = 0x0d,
        #[doc = "HSI clock divided by 7.5"]
        Div75 = 0x0e,
        #[doc = "HSI clock divided by 8"]
        Div8 = 0x0f,
    }
    impl Hsikdiv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hsikdiv {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hsikdiv {
        #[inline(always)]
        fn from(val: u8) -> Hsikdiv {
            Hsikdiv::from_bits(val)
        }
    }
    impl From<Hsikdiv> for u8 {
        #[inline(always)]
        fn from(val: Hsikdiv) -> u8 {
            Hsikdiv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Icsel {
        #[doc = "rcc_pclk1 selected as kernel clock (default after reset)"]
        Pclk1 = 0x0,
        #[doc = "psik_ck selected as kernel clock"]
        Psik = 0x01,
        #[doc = "hsik_ck selected as kernel clock"]
        Hsik = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Icsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Icsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Icsel {
        #[inline(always)]
        fn from(val: u8) -> Icsel {
            Icsel::from_bits(val)
        }
    }
    impl From<Icsel> for u8 {
        #[inline(always)]
        fn from(val: Icsel) -> u8 {
            Icsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lptimsel {
        #[doc = "rcc_pclk3 selected as kernel clock (default after reset)"]
        Pclk3 = 0x0,
        #[doc = "hsik_ck selected as kernel clock"]
        Hsik = 0x01,
        #[doc = "lse_ck selected as kernel clock"]
        Lse = 0x02,
        #[doc = "lsi_ck selected as kernel clock"]
        Lsi = 0x03,
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
    pub enum Lpuartsel {
        #[doc = "rcc_pclk3 selected as kernel clock (default after reset)"]
        Pclk3 = 0x0,
        #[doc = "hsik_ck selected as kernel clock"]
        Hsik = 0x01,
        #[doc = "lse_ck selected as kernel clock"]
        Lse = 0x02,
        #[doc = "lsi_ck selected as kernel clock"]
        Lsi = 0x03,
    }
    impl Lpuartsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lpuartsel {
            unsafe { core::mem::transmute(val & 0x03) }
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
        Lsi = 0x0,
        #[doc = "LSE clock selected"]
        Lse = 0x01,
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
        #[doc = "Lowest drive (default after backup domain reset)"]
        Low = 0x0,
        #[doc = "Medium low drive"]
        MediumLow = 0x01,
        #[doc = "Medium high drive"]
        MediumHigh = 0x02,
        #[doc = "Highest drive"]
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
    pub enum Lseext {
        #[doc = "LSE in analog mode (default after backup domain reset)"]
        Analog = 0x0,
        #[doc = "LSE in digital mode (do not use if RTC is active)"]
        Digital = 0x01,
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
    pub enum Mcopre {
        #[doc = "Prescaler disabled (default after reset)"]
        Disabled = 0x0,
        #[doc = "Division by 1 (bypass)"]
        Div1 = 0x01,
        #[doc = "Division by 2"]
        Div2 = 0x02,
        #[doc = "Division by 3"]
        Div3 = 0x03,
        #[doc = "Division by 4"]
        Div4 = 0x04,
        #[doc = "Division by 5"]
        Div5 = 0x05,
        #[doc = "Division by 6"]
        Div6 = 0x06,
        #[doc = "Division by 7"]
        Div7 = 0x07,
        #[doc = "Division by 8"]
        Div8 = 0x08,
        #[doc = "Division by 9"]
        Div9 = 0x09,
        #[doc = "Division by 10"]
        Div10 = 0x0a,
        #[doc = "Division by 11"]
        Div11 = 0x0b,
        #[doc = "Division by 12"]
        Div12 = 0x0c,
        #[doc = "Division by 13"]
        Div13 = 0x0d,
        #[doc = "Division by 14"]
        Div14 = 0x0e,
        #[doc = "Division by 15"]
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
    pub enum Mcosel {
        #[doc = "System clock selected (sys_ck) (default after reset)"]
        Sys = 0x0,
        #[doc = "HSE clock selected (hse_ck)"]
        Hse = 0x01,
        #[doc = "LSE clock selected (lse_ck)"]
        Lse = 0x02,
        #[doc = "LSI clock selected (lsi_ck)"]
        Lsi = 0x03,
        #[doc = "PSIK clock selected (psik_ck)"]
        Psik = 0x04,
        #[doc = "HSIK clock selected (hsik_ck)"]
        Hsik = 0x05,
        #[doc = "PSI clock selected (psi_ck)"]
        Psi = 0x06,
        #[doc = "HSI clock selected (hsi_ck)"]
        Hsi = 0x07,
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
    pub enum Ppre {
        #[doc = "rcc_pclk = rcc_hclk1"]
        Div1 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "rcc_pclk = rcc_hclk1 / 2"]
        Div2 = 0x04,
        #[doc = "rcc_pclk = rcc_hclk1 / 4"]
        Div4 = 0x05,
        #[doc = "rcc_pclk = rcc_hclk1 / 8"]
        Div8 = 0x06,
        #[doc = "rcc_pclk = rcc_hclk1 / 16"]
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
    pub enum Psifreq {
        #[doc = "PSI nominal frequency = 100 MHz"]
        F100mHz = 0x0,
        #[doc = "PSI nominal frequency = 144 MHz"]
        F144mHz = 0x01,
        #[doc = "PSI nominal frequency = 144 MHz"]
        F160mHz = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Psifreq {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Psifreq {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Psifreq {
        #[inline(always)]
        fn from(val: u8) -> Psifreq {
            Psifreq::from_bits(val)
        }
    }
    impl From<Psifreq> for u8 {
        #[inline(always)]
        fn from(val: Psifreq) -> u8 {
            Psifreq::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Psikdiv {
        #[doc = "PSI clock divided by 1"]
        Div1 = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "PSI clock divided by 1.5"]
        Div15 = 0x02,
        #[doc = "PSI clock divided by 2"]
        Div2 = 0x03,
        #[doc = "PSI clock divided by 2.5"]
        Div25 = 0x04,
        #[doc = "PSI clock divided by 3"]
        Div3 = 0x05,
        #[doc = "PSI clock divided by 3.5"]
        Div35 = 0x06,
        #[doc = "PSI clock divided by 4"]
        Div4 = 0x07,
        #[doc = "PSI clock divided by 4.5"]
        Div45 = 0x08,
        #[doc = "PSI clock divided by 5"]
        Div5 = 0x09,
        #[doc = "PSI clock divided by 5.5"]
        Div55 = 0x0a,
        #[doc = "PSI clock divided by 6"]
        Div6 = 0x0b,
        #[doc = "PSI clock divided by 6.5"]
        Div65 = 0x0c,
        #[doc = "PSI clock divided by 7"]
        Div7 = 0x0d,
        #[doc = "PSI clock divided by 7.5"]
        Div75 = 0x0e,
        #[doc = "PSI clock divided by 8"]
        Div8 = 0x0f,
    }
    impl Psikdiv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Psikdiv {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Psikdiv {
        #[inline(always)]
        fn from(val: u8) -> Psikdiv {
            Psikdiv::from_bits(val)
        }
    }
    impl From<Psikdiv> for u8 {
        #[inline(always)]
        fn from(val: Psikdiv) -> u8 {
            Psikdiv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Psiref {
        #[doc = "32.768 kHz"]
        F32768kHz = 0x0,
        #[doc = "8 MHz"]
        F8mHz = 0x01,
        #[doc = "16 MHz"]
        F16mHz = 0x02,
        #[doc = "24 MHz"]
        F24mHz = 0x03,
        #[doc = "25 MHz"]
        F25mHz = 0x04,
        #[doc = "32 MHz"]
        F32mHz = 0x05,
        #[doc = "48 MHz"]
        F48mHz = 0x06,
        #[doc = "50 MHz"]
        F50mHz = 0x07,
    }
    impl Psiref {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Psiref {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Psiref {
        #[inline(always)]
        fn from(val: u8) -> Psiref {
            Psiref::from_bits(val)
        }
    }
    impl From<Psiref> for u8 {
        #[inline(always)]
        fn from(val: Psiref) -> u8 {
            Psiref::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Psirefsrc {
        #[doc = "HSE clock used as reference for PSI"]
        Hse = 0x0,
        #[doc = "LSE clock used as reference for PSI"]
        Lse = 0x01,
        #[doc = "HSI clock divided by 18 used as reference for PSI"]
        Hsi = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Psirefsrc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Psirefsrc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Psirefsrc {
        #[inline(always)]
        fn from(val: u8) -> Psirefsrc {
            Psirefsrc::from_bits(val)
        }
    }
    impl From<Psirefsrc> for u8 {
        #[inline(always)]
        fn from(val: Psirefsrc) -> u8 {
            Psirefsrc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rtcsel {
        #[doc = "No clock (default after backup domain reset)"]
        Disable = 0x0,
        #[doc = "LSE selected as RTC clock"]
        Lse = 0x01,
        #[doc = "LSI selected as RTC clock"]
        Lsi = 0x02,
        #[doc = "HSE divided by RTCPRE value selected as RTC clock"]
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Spisel {
        #[doc = "rcc_pclk2 selected as kernel clock (default after reset)"]
        Pclk2 = 0x0,
        #[doc = "psik_ck selected as kernel clock"]
        Psik = 0x01,
        #[doc = "hsik_ck selected as kernel clock"]
        Hsik = 0x02,
        #[doc = "AUDIOCLK selected as kernel clock"]
        Audioclk = 0x03,
    }
    impl Spisel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Spisel {
            unsafe { core::mem::transmute(val & 0x03) }
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
    pub enum Stopwuck {
        #[doc = "HSIDIV3 selected as wake-up clock from system Stop mode (default after reset)"]
        Hsidiv3 = 0x0,
        #[doc = "HSI selected as wake-up clock from system Stop mode"]
        Hsi = 0x01,
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
        #[doc = "HSIDIV3 selected as system clock (hsidiv3_ck) (default after reset)"]
        Hsidiv3 = 0x0,
        #[doc = "HSI selected as system clock (hsi_ck)"]
        Hsi = 0x01,
        #[doc = "HSE selected as system clock (hse_ck)"]
        Hse = 0x02,
        #[doc = "PSI selected as system clock (psi_ck)"]
        Psi = 0x03,
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
    pub enum Systiclksel {
        #[doc = "rcc_hclk/8 selected as clock source (default after reset)"]
        HclkDiv8 = 0x0,
        #[doc = "lsi_ker_ck\\[1\\]
selected as clock source"]
        Lsi = 0x01,
        #[doc = "lse_ck\\[1\\]
selected as clock source"]
        Lse = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Systiclksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Systiclksel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Systiclksel {
        #[inline(always)]
        fn from(val: u8) -> Systiclksel {
            Systiclksel::from_bits(val)
        }
    }
    impl From<Systiclksel> for u8 {
        #[inline(always)]
        fn from(val: Systiclksel) -> u8 {
            Systiclksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Uartsel {
        #[doc = "rcc_pclk1 selected as kernel clock (default after reset)"]
        Pclk1 = 0x0,
        #[doc = "psik_ck selected as kernel clock"]
        Psik = 0x01,
        #[doc = "hsik_ck selected as kernel clock"]
        Hsik = 0x02,
        #[doc = "lse_ck selected as kernel clock"]
        Lse = 0x03,
    }
    impl Uartsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Uartsel {
            unsafe { core::mem::transmute(val & 0x03) }
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
    pub enum Usartsel {
        #[doc = "rcc_pclk2 selected as kernel clock (default after reset)"]
        Pclk2 = 0x0,
        #[doc = "psik_ck selected as kernel clock"]
        Psik = 0x01,
        #[doc = "hsik_ck selected as kernel clock"]
        Hsik = 0x02,
        #[doc = "lse_ck selected as kernel clock"]
        Lse = 0x03,
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
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Xspisel {
        #[doc = "rcc_hclk4 selected as kernel clock (default after reset)"]
        Hclk4 = 0x0,
        #[doc = "psik_ck selected as kernel clock"]
        Psik = 0x01,
        #[doc = "hsik_ck selected as kernel clock"]
        Hsik = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Xspisel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Xspisel {
            unsafe { core::mem::transmute(val & 0x03) }
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
