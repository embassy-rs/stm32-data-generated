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
    #[doc = "PLL configuration register"]
    #[inline(always)]
    pub const fn pllcfgr(self) -> crate::common::Reg<regs::Pllcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "clock configuration register"]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "clock interrupt register"]
    #[inline(always)]
    pub const fn cir(self) -> crate::common::Reg<regs::Cir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "AHB1 peripheral reset register"]
    #[inline(always)]
    pub const fn ahb1rstr(self) -> crate::common::Reg<regs::Ahb1rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "AHB2 peripheral reset register"]
    #[inline(always)]
    pub const fn ahb2rstr(self) -> crate::common::Reg<regs::Ahb2rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "AHB3 peripheral reset register"]
    #[inline(always)]
    pub const fn ahb3rstr(self) -> crate::common::Reg<regs::Ahb3rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "APB1 peripheral reset register"]
    #[inline(always)]
    pub const fn apb1rstr(self) -> crate::common::Reg<regs::Apb1rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "APB2 peripheral reset register"]
    #[inline(always)]
    pub const fn apb2rstr(self) -> crate::common::Reg<regs::Apb2rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "AHB1 peripheral clock register"]
    #[inline(always)]
    pub const fn ahb1enr(self) -> crate::common::Reg<regs::Ahb1enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "AHB2 peripheral clock enable register"]
    #[inline(always)]
    pub const fn ahb2enr(self) -> crate::common::Reg<regs::Ahb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "AHB3 peripheral clock enable register"]
    #[inline(always)]
    pub const fn ahb3enr(self) -> crate::common::Reg<regs::Ahb3enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "APB1 peripheral clock enable register"]
    #[inline(always)]
    pub const fn apb1enr(self) -> crate::common::Reg<regs::Apb1enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "APB2 peripheral clock enable register"]
    #[inline(always)]
    pub const fn apb2enr(self) -> crate::common::Reg<regs::Apb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "AHB1 peripheral clock enable in low power mode register"]
    #[inline(always)]
    pub const fn ahb1lpenr(self) -> crate::common::Reg<regs::Ahb1lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "AHB2 peripheral clock enable in low power mode register"]
    #[inline(always)]
    pub const fn ahb2lpenr(self) -> crate::common::Reg<regs::Ahb2lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "AHB3 peripheral clock enable in low power mode register"]
    #[inline(always)]
    pub const fn ahb3lpenr(self) -> crate::common::Reg<regs::Ahb3lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "APB1 peripheral clock enable in low power mode register"]
    #[inline(always)]
    pub const fn apb1lpenr(self) -> crate::common::Reg<regs::Apb1lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "APB2 peripheral clock enabled in low power mode register"]
    #[inline(always)]
    pub const fn apb2lpenr(self) -> crate::common::Reg<regs::Apb2lpenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "Backup domain control register"]
    #[inline(always)]
    pub const fn bdcr(self) -> crate::common::Reg<regs::Bdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "clock control & status register"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "spread spectrum clock generation register"]
    #[inline(always)]
    pub const fn sscgr(self) -> crate::common::Reg<regs::Sscgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "PLLI2S configuration register"]
    #[inline(always)]
    pub const fn plli2scfgr(self) -> crate::common::Reg<regs::Plli2scfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "PLL configuration register"]
    #[inline(always)]
    pub const fn pllsaicfgr(self) -> crate::common::Reg<regs::Pllsaicfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "dedicated clocks configuration register"]
    #[inline(always)]
    pub const fn dckcfgr1(self) -> crate::common::Reg<regs::Dckcfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "dedicated clocks configuration register"]
    #[inline(always)]
    pub const fn dckcfgr2(self) -> crate::common::Reg<regs::Dckcfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
}
pub mod regs {
    #[doc = "AHB1 peripheral clock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1enr(pub u32);
    impl Ahb1enr {
        #[doc = "IO port A clock enable"]
        #[inline(always)]
        pub const fn gpioaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IO port A clock enable"]
        #[inline(always)]
        pub fn set_gpioaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IO port B clock enable"]
        #[inline(always)]
        pub const fn gpioben(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IO port B clock enable"]
        #[inline(always)]
        pub fn set_gpioben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IO port C clock enable"]
        #[inline(always)]
        pub const fn gpiocen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IO port C clock enable"]
        #[inline(always)]
        pub fn set_gpiocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "IO port D clock enable"]
        #[inline(always)]
        pub const fn gpioden(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IO port D clock enable"]
        #[inline(always)]
        pub fn set_gpioden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "IO port E clock enable"]
        #[inline(always)]
        pub const fn gpioeen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IO port E clock enable"]
        #[inline(always)]
        pub fn set_gpioeen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "IO port F clock enable"]
        #[inline(always)]
        pub const fn gpiofen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "IO port F clock enable"]
        #[inline(always)]
        pub fn set_gpiofen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "IO port G clock enable"]
        #[inline(always)]
        pub const fn gpiogen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "IO port G clock enable"]
        #[inline(always)]
        pub fn set_gpiogen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "IO port H clock enable"]
        #[inline(always)]
        pub const fn gpiohen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "IO port H clock enable"]
        #[inline(always)]
        pub fn set_gpiohen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "IO port I clock enable"]
        #[inline(always)]
        pub const fn gpioien(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "IO port I clock enable"]
        #[inline(always)]
        pub fn set_gpioien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "IO port J clock enable"]
        #[inline(always)]
        pub const fn gpiojen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "IO port J clock enable"]
        #[inline(always)]
        pub fn set_gpiojen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "IO port K clock enable"]
        #[inline(always)]
        pub const fn gpioken(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "IO port K clock enable"]
        #[inline(always)]
        pub fn set_gpioken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "CRC clock enable"]
        #[inline(always)]
        pub const fn crcen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC clock enable"]
        #[inline(always)]
        pub fn set_crcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Backup SRAM interface clock enable"]
        #[inline(always)]
        pub const fn bkpsramen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Backup SRAM interface clock enable"]
        #[inline(always)]
        pub fn set_bkpsramen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "CCM data RAM clock enable"]
        #[inline(always)]
        pub const fn dtcmramen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "CCM data RAM clock enable"]
        #[inline(always)]
        pub fn set_dtcmramen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "DMA1 clock enable"]
        #[inline(always)]
        pub const fn dma1en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 clock enable"]
        #[inline(always)]
        pub fn set_dma1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "DMA2 clock enable"]
        #[inline(always)]
        pub const fn dma2en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 clock enable"]
        #[inline(always)]
        pub fn set_dma2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "DMA2D clock enable"]
        #[inline(always)]
        pub const fn dma2den(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2D clock enable"]
        #[inline(always)]
        pub fn set_dma2den(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Ethernet MAC clock enable"]
        #[inline(always)]
        pub const fn ethen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet MAC clock enable"]
        #[inline(always)]
        pub fn set_ethen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Ethernet Transmission clock enable"]
        #[inline(always)]
        pub const fn ethtxen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet Transmission clock enable"]
        #[inline(always)]
        pub fn set_ethtxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Ethernet Reception clock enable"]
        #[inline(always)]
        pub const fn ethrxen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet Reception clock enable"]
        #[inline(always)]
        pub fn set_ethrxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Ethernet PTP clock enable"]
        #[inline(always)]
        pub const fn ethptpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet PTP clock enable"]
        #[inline(always)]
        pub fn set_ethptpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "USB OTG HS clock enable"]
        #[inline(always)]
        pub const fn usb_otg_hsen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "USB OTG HS clock enable"]
        #[inline(always)]
        pub fn set_usb_otg_hsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "USB OTG HSULPI clock enable"]
        #[inline(always)]
        pub const fn usb_otg_hsulpien(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "USB OTG HSULPI clock enable"]
        #[inline(always)]
        pub fn set_usb_otg_hsulpien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Ahb1enr {
        #[inline(always)]
        fn default() -> Ahb1enr {
            Ahb1enr(0)
        }
    }
    #[doc = "AHB1 peripheral clock enable in low power mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1lpenr(pub u32);
    impl Ahb1lpenr {
        #[doc = "IO port A clock enable during sleep mode"]
        #[inline(always)]
        pub const fn gpioalpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IO port A clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_gpioalpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IO port B clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn gpioblpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IO port B clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_gpioblpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IO port C clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn gpioclpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IO port C clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_gpioclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "IO port D clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn gpiodlpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IO port D clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_gpiodlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "IO port E clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn gpioelpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IO port E clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_gpioelpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "IO port F clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn gpioflpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "IO port F clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_gpioflpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "IO port G clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn gpioglpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "IO port G clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_gpioglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "IO port H clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn gpiohlpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "IO port H clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_gpiohlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "IO port I clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn gpioilpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "IO port I clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_gpioilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "IO port J clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn gpiojlpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "IO port J clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_gpiojlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "IO port K clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn gpioklpen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "IO port K clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_gpioklpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "CRC clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn crclpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_crclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "AXI to AHB bridge clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn axilpen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "AXI to AHB bridge clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_axilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Flash interface clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn flashlpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Flash interface clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_flashlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "SRAM 1interface clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn sram1lpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM 1interface clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_sram1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "SRAM 2 interface clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn sram2lpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM 2 interface clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_sram2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Backup SRAM interface clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn bkpsramlpen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Backup SRAM interface clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_bkpsramlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SRAM 3 interface clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn sram3lpen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM 3 interface clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_sram3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "DTCM RAM interface clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn dtcmlpen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "DTCM RAM interface clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_dtcmlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "DMA1 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn dma1lpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "DMA1 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_dma1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "DMA2 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn dma2lpen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_dma2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "DMA2D clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn dma2dlpen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2D clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_dma2dlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Ethernet MAC clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn ethlpen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet MAC clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_ethlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Ethernet transmission clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn ethtxlpen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet transmission clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_ethtxlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Ethernet reception clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn ethrxlpen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet reception clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_ethrxlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Ethernet PTP clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn ethptplpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet PTP clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_ethptplpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "USB OTG HS clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn usb_otg_hslpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "USB OTG HS clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_usb_otg_hslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "USB OTG HS ULPI clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn usb_otg_hsulpilpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "USB OTG HS ULPI clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_usb_otg_hsulpilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Ahb1lpenr {
        #[inline(always)]
        fn default() -> Ahb1lpenr {
            Ahb1lpenr(0)
        }
    }
    #[doc = "AHB1 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1rstr(pub u32);
    impl Ahb1rstr {
        #[doc = "IO port A reset"]
        #[inline(always)]
        pub const fn gpioarst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IO port A reset"]
        #[inline(always)]
        pub fn set_gpioarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IO port B reset"]
        #[inline(always)]
        pub const fn gpiobrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IO port B reset"]
        #[inline(always)]
        pub fn set_gpiobrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IO port C reset"]
        #[inline(always)]
        pub const fn gpiocrst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IO port C reset"]
        #[inline(always)]
        pub fn set_gpiocrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "IO port D reset"]
        #[inline(always)]
        pub const fn gpiodrst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IO port D reset"]
        #[inline(always)]
        pub fn set_gpiodrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "IO port E reset"]
        #[inline(always)]
        pub const fn gpioerst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IO port E reset"]
        #[inline(always)]
        pub fn set_gpioerst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "IO port F reset"]
        #[inline(always)]
        pub const fn gpiofrst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "IO port F reset"]
        #[inline(always)]
        pub fn set_gpiofrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "IO port G reset"]
        #[inline(always)]
        pub const fn gpiogrst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "IO port G reset"]
        #[inline(always)]
        pub fn set_gpiogrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "IO port H reset"]
        #[inline(always)]
        pub const fn gpiohrst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "IO port H reset"]
        #[inline(always)]
        pub fn set_gpiohrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "IO port I reset"]
        #[inline(always)]
        pub const fn gpioirst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "IO port I reset"]
        #[inline(always)]
        pub fn set_gpioirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "IO port J reset"]
        #[inline(always)]
        pub const fn gpiojrst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "IO port J reset"]
        #[inline(always)]
        pub fn set_gpiojrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "IO port K reset"]
        #[inline(always)]
        pub const fn gpiokrst(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "IO port K reset"]
        #[inline(always)]
        pub fn set_gpiokrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "CRC reset"]
        #[inline(always)]
        pub const fn crcrst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC reset"]
        #[inline(always)]
        pub fn set_crcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "DMA2 reset"]
        #[inline(always)]
        pub const fn dma1rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 reset"]
        #[inline(always)]
        pub fn set_dma1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "DMA2 reset"]
        #[inline(always)]
        pub const fn dma2rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2 reset"]
        #[inline(always)]
        pub fn set_dma2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "DMA2D reset"]
        #[inline(always)]
        pub const fn dma2drst(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "DMA2D reset"]
        #[inline(always)]
        pub fn set_dma2drst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Ethernet MAC reset"]
        #[inline(always)]
        pub const fn ethrst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet MAC reset"]
        #[inline(always)]
        pub fn set_ethrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "USB OTG HS module reset"]
        #[inline(always)]
        pub const fn usb_otg_hsrst(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "USB OTG HS module reset"]
        #[inline(always)]
        pub fn set_usb_otg_hsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Ahb1rstr {
        #[inline(always)]
        fn default() -> Ahb1rstr {
            Ahb1rstr(0)
        }
    }
    #[doc = "AHB2 peripheral clock enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2enr(pub u32);
    impl Ahb2enr {
        #[doc = "Camera interface enable"]
        #[inline(always)]
        pub const fn dcmien(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Camera interface enable"]
        #[inline(always)]
        pub fn set_dcmien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "JPEG enable"]
        #[inline(always)]
        pub const fn jpegen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "JPEG enable"]
        #[inline(always)]
        pub fn set_jpegen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "AES module clock enable"]
        #[inline(always)]
        pub const fn aesen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "AES module clock enable"]
        #[inline(always)]
        pub fn set_aesen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Cryptographic modules clock enable"]
        #[inline(always)]
        pub const fn crypen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Cryptographic modules clock enable"]
        #[inline(always)]
        pub fn set_crypen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Hash modules clock enable"]
        #[inline(always)]
        pub const fn hashen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Hash modules clock enable"]
        #[inline(always)]
        pub fn set_hashen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Random number generator clock enable"]
        #[inline(always)]
        pub const fn rngen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Random number generator clock enable"]
        #[inline(always)]
        pub fn set_rngen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "USB OTG FS clock enable"]
        #[inline(always)]
        pub const fn usb_otg_fsen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "USB OTG FS clock enable"]
        #[inline(always)]
        pub fn set_usb_otg_fsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ahb2enr {
        #[inline(always)]
        fn default() -> Ahb2enr {
            Ahb2enr(0)
        }
    }
    #[doc = "AHB2 peripheral clock enable in low power mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2lpenr(pub u32);
    impl Ahb2lpenr {
        #[doc = "Camera interface enable during Sleep mode"]
        #[inline(always)]
        pub const fn dcmilpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Camera interface enable during Sleep mode"]
        #[inline(always)]
        pub fn set_dcmilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "JPEG module enabled during Sleep mode"]
        #[inline(always)]
        pub const fn jpeglpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "JPEG module enabled during Sleep mode"]
        #[inline(always)]
        pub fn set_jpeglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "AES module clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn aeslpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "AES module clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_aeslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Cryptography modules clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn cryplpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Cryptography modules clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_cryplpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Hash modules clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn hashlpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Hash modules clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_hashlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Random number generator clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn rnglpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Random number generator clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_rnglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "USB OTG FS clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn usb_otg_fslpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "USB OTG FS clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_usb_otg_fslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ahb2lpenr {
        #[inline(always)]
        fn default() -> Ahb2lpenr {
            Ahb2lpenr(0)
        }
    }
    #[doc = "AHB2 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2rstr(pub u32);
    impl Ahb2rstr {
        #[doc = "Camera interface reset"]
        #[inline(always)]
        pub const fn dcmirst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Camera interface reset"]
        #[inline(always)]
        pub fn set_dcmirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "AES module reset"]
        #[inline(always)]
        pub const fn aesrst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "AES module reset"]
        #[inline(always)]
        pub fn set_aesrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Cryptographic module reset"]
        #[inline(always)]
        pub const fn cryprst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Cryptographic module reset"]
        #[inline(always)]
        pub fn set_cryprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Hash module reset"]
        #[inline(always)]
        pub const fn hsahrst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Hash module reset"]
        #[inline(always)]
        pub fn set_hsahrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Random number generator module reset"]
        #[inline(always)]
        pub const fn rngrst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Random number generator module reset"]
        #[inline(always)]
        pub fn set_rngrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "USB OTG FS module reset"]
        #[inline(always)]
        pub const fn usb_otg_fsrst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "USB OTG FS module reset"]
        #[inline(always)]
        pub fn set_usb_otg_fsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ahb2rstr {
        #[inline(always)]
        fn default() -> Ahb2rstr {
            Ahb2rstr(0)
        }
    }
    #[doc = "AHB3 peripheral clock enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3enr(pub u32);
    impl Ahb3enr {
        #[doc = "Flexible memory controller module clock enable"]
        #[inline(always)]
        pub const fn fmcen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Flexible memory controller module clock enable"]
        #[inline(always)]
        pub fn set_fmcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Quad SPI memory controller clock enable"]
        #[inline(always)]
        pub const fn quadspien(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Quad SPI memory controller clock enable"]
        #[inline(always)]
        pub fn set_quadspien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Ahb3enr {
        #[inline(always)]
        fn default() -> Ahb3enr {
            Ahb3enr(0)
        }
    }
    #[doc = "AHB3 peripheral clock enable in low power mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3lpenr(pub u32);
    impl Ahb3lpenr {
        #[doc = "Flexible memory controller module clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn fmclpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Flexible memory controller module clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_fmclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Quand SPI memory controller clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn quadspilpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Quand SPI memory controller clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_quadspilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Ahb3lpenr {
        #[inline(always)]
        fn default() -> Ahb3lpenr {
            Ahb3lpenr(0)
        }
    }
    #[doc = "AHB3 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb3rstr(pub u32);
    impl Ahb3rstr {
        #[doc = "Flexible memory controller module reset"]
        #[inline(always)]
        pub const fn fmcrst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Flexible memory controller module reset"]
        #[inline(always)]
        pub fn set_fmcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Quad SPI memory controller reset"]
        #[inline(always)]
        pub const fn quadspirst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Quad SPI memory controller reset"]
        #[inline(always)]
        pub fn set_quadspirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Ahb3rstr {
        #[inline(always)]
        fn default() -> Ahb3rstr {
            Ahb3rstr(0)
        }
    }
    #[doc = "APB1 peripheral clock enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1enr(pub u32);
    impl Apb1enr {
        #[doc = "TIM2 clock enable"]
        #[inline(always)]
        pub const fn tim2en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 clock enable"]
        #[inline(always)]
        pub fn set_tim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 clock enable"]
        #[inline(always)]
        pub const fn tim3en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 clock enable"]
        #[inline(always)]
        pub fn set_tim3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 clock enable"]
        #[inline(always)]
        pub const fn tim4en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 clock enable"]
        #[inline(always)]
        pub fn set_tim4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5 clock enable"]
        #[inline(always)]
        pub const fn tim5en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 clock enable"]
        #[inline(always)]
        pub fn set_tim5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 clock enable"]
        #[inline(always)]
        pub const fn tim6en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 clock enable"]
        #[inline(always)]
        pub fn set_tim6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 clock enable"]
        #[inline(always)]
        pub const fn tim7en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 clock enable"]
        #[inline(always)]
        pub fn set_tim7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM12 clock enable"]
        #[inline(always)]
        pub const fn tim12en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12 clock enable"]
        #[inline(always)]
        pub fn set_tim12en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIM13 clock enable"]
        #[inline(always)]
        pub const fn tim13en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIM13 clock enable"]
        #[inline(always)]
        pub fn set_tim13en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIM14 clock enable"]
        #[inline(always)]
        pub const fn tim14en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM14 clock enable"]
        #[inline(always)]
        pub fn set_tim14en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Low power timer 1 clock enable"]
        #[inline(always)]
        pub const fn lptim1en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Low power timer 1 clock enable"]
        #[inline(always)]
        pub fn set_lptim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "RTCAPB clock enable"]
        #[inline(always)]
        pub const fn rtcen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "RTCAPB clock enable"]
        #[inline(always)]
        pub fn set_rtcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
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
        #[doc = "CAN 3 enable"]
        #[inline(always)]
        pub const fn can3en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "CAN 3 enable"]
        #[inline(always)]
        pub fn set_can3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "SPI2 clock enable"]
        #[inline(always)]
        pub const fn spi2en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 clock enable"]
        #[inline(always)]
        pub fn set_spi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 clock enable"]
        #[inline(always)]
        pub const fn spi3en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 clock enable"]
        #[inline(always)]
        pub fn set_spi3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "SPDIF-RX clock enable"]
        #[inline(always)]
        pub const fn spdifrxen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SPDIF-RX clock enable"]
        #[inline(always)]
        pub fn set_spdifrxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
        #[doc = "USART3 clock enable"]
        #[inline(always)]
        pub const fn usart3en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 clock enable"]
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
        #[doc = "I2C1 clock enable"]
        #[inline(always)]
        pub const fn i2c1en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 clock enable"]
        #[inline(always)]
        pub fn set_i2c1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 clock enable"]
        #[inline(always)]
        pub const fn i2c2en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 clock enable"]
        #[inline(always)]
        pub fn set_i2c2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I2C3 clock enable"]
        #[inline(always)]
        pub const fn i2c3en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 clock enable"]
        #[inline(always)]
        pub fn set_i2c3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "I2C4 clock enable"]
        #[inline(always)]
        pub const fn i2c4en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 clock enable"]
        #[inline(always)]
        pub fn set_i2c4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "CAN 1 clock enable"]
        #[inline(always)]
        pub const fn can1en(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "CAN 1 clock enable"]
        #[inline(always)]
        pub fn set_can1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "CAN 2 clock enable"]
        #[inline(always)]
        pub const fn can2en(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "CAN 2 clock enable"]
        #[inline(always)]
        pub fn set_can2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "HDMI-CEN clock enable"]
        #[inline(always)]
        pub const fn cecen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "HDMI-CEN clock enable"]
        #[inline(always)]
        pub fn set_cecen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
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
        #[doc = "UART7 clock enable"]
        #[inline(always)]
        pub const fn uart7en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "UART7 clock enable"]
        #[inline(always)]
        pub fn set_uart7en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "UART8 clock enable"]
        #[inline(always)]
        pub const fn uart8en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "UART8 clock enable"]
        #[inline(always)]
        pub fn set_uart8en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Apb1enr {
        #[inline(always)]
        fn default() -> Apb1enr {
            Apb1enr(0)
        }
    }
    #[doc = "APB1 peripheral clock enable in low power mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1lpenr(pub u32);
    impl Apb1lpenr {
        #[doc = "TIM2 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn tim2lpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_tim2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn tim3lpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_tim3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn tim4lpen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_tim4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn tim5lpen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_tim5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn tim6lpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_tim6lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn tim7lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_tim7lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM12 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn tim12lpen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_tim12lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIM13 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn tim13lpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIM13 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_tim13lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIM14 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn tim14lpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM14 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_tim14lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "low power timer 1 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn lptim1lpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "low power timer 1 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_lptim1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "RTCAPB clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn rtclpen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "RTCAPB clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_rtclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Window watchdog clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn wwdglpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_wwdglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "CAN 3 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn can3lpen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "CAN 3 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_can3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "SPI2 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn spi2lpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_spi2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI3 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn spi3lpen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_spi3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "SPDIF-RX clock enable during sleep mode"]
        #[inline(always)]
        pub const fn spdifrxlpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SPDIF-RX clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_spdifrxlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "USART2 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn usart2lpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_usart2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn usart3lpen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_usart3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "UART4 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn uart4lpen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "UART4 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_uart4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "UART5 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn uart5lpen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "UART5 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_uart5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn i2c1lpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_i2c1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn i2c2lpen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_i2c2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I2C3 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn i2c3lpen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_i2c3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "I2C4 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn i2c4lpen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_i2c4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "CAN 1 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn can1lpen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "CAN 1 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_can1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "CAN 2 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn can2lpen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "CAN 2 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_can2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "HDMI-CEN clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn ceclpen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "HDMI-CEN clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_ceclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Power interface clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn pwrlpen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Power interface clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_pwrlpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "DAC interface clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn daclpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC interface clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_daclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "UART7 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn uart7lpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "UART7 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_uart7lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "UART8 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn uart8lpen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "UART8 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_uart8lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Apb1lpenr {
        #[inline(always)]
        fn default() -> Apb1lpenr {
            Apb1lpenr(0)
        }
    }
    #[doc = "APB1 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1rstr(pub u32);
    impl Apb1rstr {
        #[doc = "TIM2 reset"]
        #[inline(always)]
        pub const fn tim2rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 reset"]
        #[inline(always)]
        pub fn set_tim2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 reset"]
        #[inline(always)]
        pub const fn tim3rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 reset"]
        #[inline(always)]
        pub fn set_tim3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 reset"]
        #[inline(always)]
        pub const fn tim4rst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 reset"]
        #[inline(always)]
        pub fn set_tim4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5 reset"]
        #[inline(always)]
        pub const fn tim5rst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 reset"]
        #[inline(always)]
        pub fn set_tim5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM6 reset"]
        #[inline(always)]
        pub const fn tim6rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 reset"]
        #[inline(always)]
        pub fn set_tim6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 reset"]
        #[inline(always)]
        pub const fn tim7rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 reset"]
        #[inline(always)]
        pub fn set_tim7rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM12 reset"]
        #[inline(always)]
        pub const fn tim12rst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12 reset"]
        #[inline(always)]
        pub fn set_tim12rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIM13 reset"]
        #[inline(always)]
        pub const fn tim13rst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIM13 reset"]
        #[inline(always)]
        pub fn set_tim13rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIM14 reset"]
        #[inline(always)]
        pub const fn tim14rst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM14 reset"]
        #[inline(always)]
        pub fn set_tim14rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Low power timer 1 reset"]
        #[inline(always)]
        pub const fn lptim1rst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Low power timer 1 reset"]
        #[inline(always)]
        pub fn set_lptim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
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
        #[doc = "CAN 3 reset"]
        #[inline(always)]
        pub const fn can3rst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "CAN 3 reset"]
        #[inline(always)]
        pub fn set_can3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "SPI 2 reset"]
        #[inline(always)]
        pub const fn spi2rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI 2 reset"]
        #[inline(always)]
        pub fn set_spi2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPI 3 reset"]
        #[inline(always)]
        pub const fn spi3rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SPI 3 reset"]
        #[inline(always)]
        pub fn set_spi3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "SPDIF-RX reset"]
        #[inline(always)]
        pub const fn spdifrxrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SPDIF-RX reset"]
        #[inline(always)]
        pub fn set_spdifrxrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
        #[doc = "USART 3 reset"]
        #[inline(always)]
        pub const fn usart3rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART 3 reset"]
        #[inline(always)]
        pub fn set_usart3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "USART 4 reset"]
        #[inline(always)]
        pub const fn uart4rst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "USART 4 reset"]
        #[inline(always)]
        pub fn set_uart4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "USART 5 reset"]
        #[inline(always)]
        pub const fn uart5rst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "USART 5 reset"]
        #[inline(always)]
        pub fn set_uart5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C 1 reset"]
        #[inline(always)]
        pub const fn i2c1rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C 1 reset"]
        #[inline(always)]
        pub fn set_i2c1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C 2 reset"]
        #[inline(always)]
        pub const fn i2c2rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C 2 reset"]
        #[inline(always)]
        pub fn set_i2c2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I2C3 reset"]
        #[inline(always)]
        pub const fn i2c3rst(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 reset"]
        #[inline(always)]
        pub fn set_i2c3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "I2C 4 reset"]
        #[inline(always)]
        pub const fn i2c4rst(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "I2C 4 reset"]
        #[inline(always)]
        pub fn set_i2c4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "CAN1 reset"]
        #[inline(always)]
        pub const fn can1rst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "CAN1 reset"]
        #[inline(always)]
        pub fn set_can1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "CAN2 reset"]
        #[inline(always)]
        pub const fn can2rst(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "CAN2 reset"]
        #[inline(always)]
        pub fn set_can2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "HDMI-CEC reset"]
        #[inline(always)]
        pub const fn cecrst(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "HDMI-CEC reset"]
        #[inline(always)]
        pub fn set_cecrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
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
        #[doc = "DAC reset"]
        #[inline(always)]
        pub const fn dacrst(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DAC reset"]
        #[inline(always)]
        pub fn set_dacrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "UART7 reset"]
        #[inline(always)]
        pub const fn uart7rst(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "UART7 reset"]
        #[inline(always)]
        pub fn set_uart7rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "UART8 reset"]
        #[inline(always)]
        pub const fn uart8rst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "UART8 reset"]
        #[inline(always)]
        pub fn set_uart8rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Apb1rstr {
        #[inline(always)]
        fn default() -> Apb1rstr {
            Apb1rstr(0)
        }
    }
    #[doc = "APB2 peripheral clock enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2enr(pub u32);
    impl Apb2enr {
        #[doc = "TIM1 clock enable"]
        #[inline(always)]
        pub const fn tim1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 clock enable"]
        #[inline(always)]
        pub fn set_tim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM8 clock enable"]
        #[inline(always)]
        pub const fn tim8en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 clock enable"]
        #[inline(always)]
        pub fn set_tim8en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "USART1 clock enable"]
        #[inline(always)]
        pub const fn usart1en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 clock enable"]
        #[inline(always)]
        pub fn set_usart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "USART6 clock enable"]
        #[inline(always)]
        pub const fn usart6en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USART6 clock enable"]
        #[inline(always)]
        pub fn set_usart6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SDMMC2 clock enable"]
        #[inline(always)]
        pub const fn sdmmc2en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC2 clock enable"]
        #[inline(always)]
        pub fn set_sdmmc2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ADC1 clock enable"]
        #[inline(always)]
        pub const fn adc1en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1 clock enable"]
        #[inline(always)]
        pub fn set_adc1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "ADC2 clock enable"]
        #[inline(always)]
        pub const fn adc2en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "ADC2 clock enable"]
        #[inline(always)]
        pub fn set_adc2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "ADC3 clock enable"]
        #[inline(always)]
        pub const fn adc3en(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADC3 clock enable"]
        #[inline(always)]
        pub fn set_adc3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "SDMMC1 clock enable"]
        #[inline(always)]
        pub const fn sdmmc1en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC1 clock enable"]
        #[inline(always)]
        pub fn set_sdmmc1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI1 clock enable"]
        #[inline(always)]
        pub const fn spi1en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 clock enable"]
        #[inline(always)]
        pub fn set_spi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SPI4 clock enable"]
        #[inline(always)]
        pub const fn spi4en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "SPI4 clock enable"]
        #[inline(always)]
        pub fn set_spi4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "System configuration controller clock enable"]
        #[inline(always)]
        pub const fn syscfgen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "System configuration controller clock enable"]
        #[inline(always)]
        pub fn set_syscfgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TIM9 clock enable"]
        #[inline(always)]
        pub const fn tim9en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM9 clock enable"]
        #[inline(always)]
        pub fn set_tim9en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM10 clock enable"]
        #[inline(always)]
        pub const fn tim10en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM10 clock enable"]
        #[inline(always)]
        pub fn set_tim10en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM11 clock enable"]
        #[inline(always)]
        pub const fn tim11en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM11 clock enable"]
        #[inline(always)]
        pub fn set_tim11en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SPI5 clock enable"]
        #[inline(always)]
        pub const fn spi5en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SPI5 clock enable"]
        #[inline(always)]
        pub fn set_spi5en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "SPI6 clock enable"]
        #[inline(always)]
        pub const fn spi6en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SPI6 clock enable"]
        #[inline(always)]
        pub fn set_spi6en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SAI1 clock enable"]
        #[inline(always)]
        pub const fn sai1en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 clock enable"]
        #[inline(always)]
        pub fn set_sai1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "SAI2 clock enable"]
        #[inline(always)]
        pub const fn sai2en(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "SAI2 clock enable"]
        #[inline(always)]
        pub fn set_sai2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "LTDC clock enable"]
        #[inline(always)]
        pub const fn ltdcen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC clock enable"]
        #[inline(always)]
        pub fn set_ltdcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "DSI clock enable"]
        #[inline(always)]
        pub const fn dsien(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "DSI clock enable"]
        #[inline(always)]
        pub fn set_dsien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "DFSDM1 clock enable"]
        #[inline(always)]
        pub const fn dfsdm1en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DFSDM1 clock enable"]
        #[inline(always)]
        pub fn set_dfsdm1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "MDIO clock enable"]
        #[inline(always)]
        pub const fn mdiosen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "MDIO clock enable"]
        #[inline(always)]
        pub fn set_mdiosen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "USB OTG HS PHY controller clock enable"]
        #[inline(always)]
        pub const fn usbphycen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "USB OTG HS PHY controller clock enable"]
        #[inline(always)]
        pub fn set_usbphycen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Apb2enr {
        #[inline(always)]
        fn default() -> Apb2enr {
            Apb2enr(0)
        }
    }
    #[doc = "APB2 peripheral clock enabled in low power mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2lpenr(pub u32);
    impl Apb2lpenr {
        #[doc = "TIM1 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn tim1lpen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_tim1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM8 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn tim8lpen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_tim8lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "USART1 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn usart1lpen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_usart1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "USART6 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn usart6lpen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USART6 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_usart6lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SDMMC2 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn sdmmc2lpen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC2 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_sdmmc2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ADC1 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn adc1lpen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_adc1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "ADC2 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn adc2lpen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "ADC2 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_adc2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "ADC 3 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn adc3lpen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ADC 3 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_adc3lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "SDMMC1 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn sdmmc1lpen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC1 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_sdmmc1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI 1 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn spi1lpen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI 1 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_spi1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SPI 4 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn spi4lpen(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "SPI 4 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_spi4lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "System configuration controller clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn syscfglpen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "System configuration controller clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_syscfglpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TIM9 clock enable during sleep mode"]
        #[inline(always)]
        pub const fn tim9lpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM9 clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_tim9lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM10 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn tim10lpen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM10 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_tim10lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM11 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn tim11lpen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM11 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_tim11lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SPI 5 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn spi5lpen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SPI 5 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_spi5lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "SPI 6 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn spi6lpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SPI 6 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_spi6lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SAI1 clock enable during sleep mode"]
        #[inline(always)]
        pub const fn sai1lpen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_sai1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "SAI2 clock enable during sleep mode"]
        #[inline(always)]
        pub const fn sai2lpen(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "SAI2 clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_sai2lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "LTDC clock enable during sleep mode"]
        #[inline(always)]
        pub const fn ltdclpen(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC clock enable during sleep mode"]
        #[inline(always)]
        pub fn set_ltdclpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "DSI clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn dsilpen(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "DSI clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_dsilpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "DFSDM1 clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn dfsdm1lpen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DFSDM1 clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_dfsdm1lpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "MDIO clock enable during Sleep mode"]
        #[inline(always)]
        pub const fn mdioslpen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "MDIO clock enable during Sleep mode"]
        #[inline(always)]
        pub fn set_mdioslpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Apb2lpenr {
        #[inline(always)]
        fn default() -> Apb2lpenr {
            Apb2lpenr(0)
        }
    }
    #[doc = "APB2 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2rstr(pub u32);
    impl Apb2rstr {
        #[doc = "TIM1 reset"]
        #[inline(always)]
        pub const fn tim1rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 reset"]
        #[inline(always)]
        pub fn set_tim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM8 reset"]
        #[inline(always)]
        pub const fn tim8rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 reset"]
        #[inline(always)]
        pub fn set_tim8rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "USART1 reset"]
        #[inline(always)]
        pub const fn usart1rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 reset"]
        #[inline(always)]
        pub fn set_usart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "USART6 reset"]
        #[inline(always)]
        pub const fn usart6rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USART6 reset"]
        #[inline(always)]
        pub fn set_usart6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SDMMC2 reset"]
        #[inline(always)]
        pub const fn sdmmc2rst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC2 reset"]
        #[inline(always)]
        pub fn set_sdmmc2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ADC interface reset (common to all ADCs)"]
        #[inline(always)]
        pub const fn adcrst(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "ADC interface reset (common to all ADCs)"]
        #[inline(always)]
        pub fn set_adcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "SDMMC1 reset"]
        #[inline(always)]
        pub const fn sdmmc1rst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "SDMMC1 reset"]
        #[inline(always)]
        pub fn set_sdmmc1rst(&mut self, val: bool) {
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
        #[doc = "SPI4 reset"]
        #[inline(always)]
        pub const fn spi4rst(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "SPI4 reset"]
        #[inline(always)]
        pub fn set_spi4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "System configuration controller reset"]
        #[inline(always)]
        pub const fn syscfgrst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "System configuration controller reset"]
        #[inline(always)]
        pub fn set_syscfgrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TIM9 reset"]
        #[inline(always)]
        pub const fn tim9rst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM9 reset"]
        #[inline(always)]
        pub fn set_tim9rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM10 reset"]
        #[inline(always)]
        pub const fn tim10rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM10 reset"]
        #[inline(always)]
        pub fn set_tim10rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM11 reset"]
        #[inline(always)]
        pub const fn tim11rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM11 reset"]
        #[inline(always)]
        pub fn set_tim11rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SPI5 reset"]
        #[inline(always)]
        pub const fn spi5rst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SPI5 reset"]
        #[inline(always)]
        pub fn set_spi5rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "SPI6 reset"]
        #[inline(always)]
        pub const fn spi6rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SPI6 reset"]
        #[inline(always)]
        pub fn set_spi6rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SAI1 reset"]
        #[inline(always)]
        pub const fn sai1rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 reset"]
        #[inline(always)]
        pub fn set_sai1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "SAI2 reset"]
        #[inline(always)]
        pub const fn sai2rst(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "SAI2 reset"]
        #[inline(always)]
        pub fn set_sai2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "LTDC reset"]
        #[inline(always)]
        pub const fn ltdcrst(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "LTDC reset"]
        #[inline(always)]
        pub fn set_ltdcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "DSI reset"]
        #[inline(always)]
        pub const fn dsirst(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "DSI reset"]
        #[inline(always)]
        pub fn set_dsirst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "DFSDM 1 reset"]
        #[inline(always)]
        pub const fn dfsdm1rst(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "DFSDM 1 reset"]
        #[inline(always)]
        pub fn set_dfsdm1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "MDIOS reset"]
        #[inline(always)]
        pub const fn mdiosrst(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "MDIOS reset"]
        #[inline(always)]
        pub fn set_mdiosrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "USB OTG HS PHY controller reset"]
        #[inline(always)]
        pub const fn usbphycrst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "USB OTG HS PHY controller reset"]
        #[inline(always)]
        pub fn set_usbphycrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Apb2rstr {
        #[inline(always)]
        fn default() -> Apb2rstr {
            Apb2rstr(0)
        }
    }
    #[doc = "Backup domain control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdcr(pub u32);
    impl Bdcr {
        #[doc = "External low-speed oscillator enable"]
        #[inline(always)]
        pub const fn lseon(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "External low-speed oscillator enable"]
        #[inline(always)]
        pub fn set_lseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "External low-speed oscillator ready"]
        #[inline(always)]
        pub const fn lserdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "External low-speed oscillator ready"]
        #[inline(always)]
        pub fn set_lserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "External low-speed oscillator bypass"]
        #[inline(always)]
        pub const fn lsebyp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "External low-speed oscillator bypass"]
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
    #[doc = "clock configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "System clock switch"]
        #[inline(always)]
        pub const fn sw(&self) -> super::vals::Sw {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "System clock switch"]
        #[inline(always)]
        pub fn set_sw(&mut self, val: super::vals::Sw) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "System clock switch status"]
        #[inline(always)]
        pub const fn sws(&self) -> super::vals::Sw {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "System clock switch status"]
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
            let val = (self.0 >> 10usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB Low speed prescaler (APB1)"]
        #[inline(always)]
        pub fn set_ppre1(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 10usize)) | (((val.to_bits() as u32) & 0x07) << 10usize);
        }
        #[doc = "APB high-speed prescaler (APB2)"]
        #[inline(always)]
        pub const fn ppre2(&self) -> super::vals::Ppre {
            let val = (self.0 >> 13usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB high-speed prescaler (APB2)"]
        #[inline(always)]
        pub fn set_ppre2(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val.to_bits() as u32) & 0x07) << 13usize);
        }
        #[doc = "HSE division factor for RTC clock"]
        #[inline(always)]
        pub const fn rtcpre(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "HSE division factor for RTC clock"]
        #[inline(always)]
        pub fn set_rtcpre(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Microcontroller clock output 1"]
        #[inline(always)]
        pub const fn mco1sel(&self) -> super::vals::Mco1sel {
            let val = (self.0 >> 21usize) & 0x03;
            super::vals::Mco1sel::from_bits(val as u8)
        }
        #[doc = "Microcontroller clock output 1"]
        #[inline(always)]
        pub fn set_mco1sel(&mut self, val: super::vals::Mco1sel) {
            self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
        }
        #[doc = "I2S clock selection"]
        #[inline(always)]
        pub const fn i2ssrc(&self) -> super::vals::Issrc {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Issrc::from_bits(val as u8)
        }
        #[doc = "I2S clock selection"]
        #[inline(always)]
        pub fn set_i2ssrc(&mut self, val: super::vals::Issrc) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "MCO1 prescaler"]
        #[inline(always)]
        pub const fn mco1pre(&self) -> super::vals::Mcopre {
            let val = (self.0 >> 24usize) & 0x07;
            super::vals::Mcopre::from_bits(val as u8)
        }
        #[doc = "MCO1 prescaler"]
        #[inline(always)]
        pub fn set_mco1pre(&mut self, val: super::vals::Mcopre) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
        }
        #[doc = "MCO2 prescaler"]
        #[inline(always)]
        pub const fn mco2pre(&self) -> super::vals::Mcopre {
            let val = (self.0 >> 27usize) & 0x07;
            super::vals::Mcopre::from_bits(val as u8)
        }
        #[doc = "MCO2 prescaler"]
        #[inline(always)]
        pub fn set_mco2pre(&mut self, val: super::vals::Mcopre) {
            self.0 = (self.0 & !(0x07 << 27usize)) | (((val.to_bits() as u32) & 0x07) << 27usize);
        }
        #[doc = "Microcontroller clock output 2"]
        #[inline(always)]
        pub const fn mco2sel(&self) -> super::vals::Mco2sel {
            let val = (self.0 >> 30usize) & 0x03;
            super::vals::Mco2sel::from_bits(val as u8)
        }
        #[doc = "Microcontroller clock output 2"]
        #[inline(always)]
        pub fn set_mco2sel(&mut self, val: super::vals::Mco2sel) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
        }
    }
    impl Default for Cfgr {
        #[inline(always)]
        fn default() -> Cfgr {
            Cfgr(0)
        }
    }
    #[doc = "clock interrupt register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cir(pub u32);
    impl Cir {
        #[doc = "LSI ready interrupt flag"]
        #[inline(always)]
        pub const fn lsirdyf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt flag"]
        #[inline(always)]
        pub fn set_lsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt flag"]
        #[inline(always)]
        pub const fn lserdyf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt flag"]
        #[inline(always)]
        pub fn set_lserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI ready interrupt flag"]
        #[inline(always)]
        pub const fn hsirdyf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt flag"]
        #[inline(always)]
        pub fn set_hsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "HSE ready interrupt flag"]
        #[inline(always)]
        pub const fn hserdyf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt flag"]
        #[inline(always)]
        pub fn set_hserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "PLL ready interrupt flag"]
        #[inline(always)]
        pub const fn pllrdyf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PLL ready interrupt flag"]
        #[inline(always)]
        pub fn set_pllrdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "PLLI2S ready interrupt flag"]
        #[inline(always)]
        pub const fn plli2srdyf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "PLLI2S ready interrupt flag"]
        #[inline(always)]
        pub fn set_plli2srdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PLLSAI ready interrupt flag"]
        #[inline(always)]
        pub const fn pllsairdyf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PLLSAI ready interrupt flag"]
        #[inline(always)]
        pub fn set_pllsairdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Clock security system interrupt flag"]
        #[inline(always)]
        pub const fn cssf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Clock security system interrupt flag"]
        #[inline(always)]
        pub fn set_cssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LSI ready interrupt enable"]
        #[inline(always)]
        pub const fn lsirdyie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt enable"]
        #[inline(always)]
        pub fn set_lsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "LSE ready interrupt enable"]
        #[inline(always)]
        pub const fn lserdyie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt enable"]
        #[inline(always)]
        pub fn set_lserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "HSI ready interrupt enable"]
        #[inline(always)]
        pub const fn hsirdyie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt enable"]
        #[inline(always)]
        pub fn set_hsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "HSE ready interrupt enable"]
        #[inline(always)]
        pub const fn hserdyie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt enable"]
        #[inline(always)]
        pub fn set_hserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "PLL ready interrupt enable"]
        #[inline(always)]
        pub const fn pllrdyie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "PLL ready interrupt enable"]
        #[inline(always)]
        pub fn set_pllrdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "PLLI2S ready interrupt enable"]
        #[inline(always)]
        pub const fn plli2srdyie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "PLLI2S ready interrupt enable"]
        #[inline(always)]
        pub fn set_plli2srdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "PLLSAI Ready Interrupt Enable"]
        #[inline(always)]
        pub const fn pllsairdyie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "PLLSAI Ready Interrupt Enable"]
        #[inline(always)]
        pub fn set_pllsairdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "LSI ready interrupt clear"]
        #[inline(always)]
        pub const fn lsirdyc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "LSI ready interrupt clear"]
        #[inline(always)]
        pub fn set_lsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "LSE ready interrupt clear"]
        #[inline(always)]
        pub const fn lserdyc(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt clear"]
        #[inline(always)]
        pub fn set_lserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "HSI ready interrupt clear"]
        #[inline(always)]
        pub const fn hsirdyc(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt clear"]
        #[inline(always)]
        pub fn set_hsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "HSE ready interrupt clear"]
        #[inline(always)]
        pub const fn hserdyc(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt clear"]
        #[inline(always)]
        pub fn set_hserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Main PLL(PLL) ready interrupt clear"]
        #[inline(always)]
        pub const fn pllrdyc(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Main PLL(PLL) ready interrupt clear"]
        #[inline(always)]
        pub fn set_pllrdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "PLLI2S ready interrupt clear"]
        #[inline(always)]
        pub const fn plli2srdyc(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "PLLI2S ready interrupt clear"]
        #[inline(always)]
        pub fn set_plli2srdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "PLLSAI Ready Interrupt Clear"]
        #[inline(always)]
        pub const fn pllsairdyc(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "PLLSAI Ready Interrupt Clear"]
        #[inline(always)]
        pub fn set_pllsairdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
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
        #[doc = "Internal high-speed clock ready flag"]
        #[inline(always)]
        pub const fn hsirdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Internal high-speed clock ready flag"]
        #[inline(always)]
        pub fn set_hsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Internal high-speed clock trimming"]
        #[inline(always)]
        pub const fn hsitrim(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x1f;
            val as u8
        }
        #[doc = "Internal high-speed clock trimming"]
        #[inline(always)]
        pub fn set_hsitrim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
        }
        #[doc = "Internal high-speed clock calibration"]
        #[inline(always)]
        pub const fn hsical(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Internal high-speed clock calibration"]
        #[inline(always)]
        pub fn set_hsical(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
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
        #[doc = "Clock security system enable"]
        #[inline(always)]
        pub const fn csson(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Clock security system enable"]
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
        #[doc = "PLLI2S enable"]
        #[inline(always)]
        pub const fn plli2son(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "PLLI2S enable"]
        #[inline(always)]
        pub fn set_plli2son(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "PLLI2S clock ready flag"]
        #[inline(always)]
        pub const fn plli2srdy(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "PLLI2S clock ready flag"]
        #[inline(always)]
        pub fn set_plli2srdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "PLLSAI enable"]
        #[inline(always)]
        pub const fn pllsaion(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "PLLSAI enable"]
        #[inline(always)]
        pub fn set_pllsaion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "PLLSAI clock ready flag"]
        #[inline(always)]
        pub const fn pllsairdy(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "PLLSAI clock ready flag"]
        #[inline(always)]
        pub fn set_pllsairdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    #[doc = "clock control & status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "Internal low-speed oscillator enable"]
        #[inline(always)]
        pub const fn lsion(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Internal low-speed oscillator enable"]
        #[inline(always)]
        pub fn set_lsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Internal low-speed oscillator ready"]
        #[inline(always)]
        pub const fn lsirdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Internal low-speed oscillator ready"]
        #[inline(always)]
        pub fn set_lsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
        #[doc = "BOR reset flag"]
        #[inline(always)]
        pub const fn borrstf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "BOR reset flag"]
        #[inline(always)]
        pub fn set_borrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "PIN reset flag"]
        #[inline(always)]
        pub const fn padrstf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "PIN reset flag"]
        #[inline(always)]
        pub fn set_padrstf(&mut self, val: bool) {
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
        pub const fn wdgrstf(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog reset flag"]
        #[inline(always)]
        pub fn set_wdgrstf(&mut self, val: bool) {
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
    #[doc = "dedicated clocks configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dckcfgr1(pub u32);
    impl Dckcfgr1 {
        #[doc = "PLLI2S division factor for SAI1 clock"]
        #[inline(always)]
        pub const fn plli2sdivq(&self) -> super::vals::Plli2sdivq {
            let val = (self.0 >> 0usize) & 0x1f;
            super::vals::Plli2sdivq::from_bits(val as u8)
        }
        #[doc = "PLLI2S division factor for SAI1 clock"]
        #[inline(always)]
        pub fn set_plli2sdivq(&mut self, val: super::vals::Plli2sdivq) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
        }
        #[doc = "PLLSAI division factor for SAI1 clock"]
        #[inline(always)]
        pub const fn pllsaidivq(&self) -> super::vals::Pllsaidivq {
            let val = (self.0 >> 8usize) & 0x1f;
            super::vals::Pllsaidivq::from_bits(val as u8)
        }
        #[doc = "PLLSAI division factor for SAI1 clock"]
        #[inline(always)]
        pub fn set_pllsaidivq(&mut self, val: super::vals::Pllsaidivq) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val.to_bits() as u32) & 0x1f) << 8usize);
        }
        #[doc = "division factor for LCD_CLK"]
        #[inline(always)]
        pub const fn pllsaidivr(&self) -> super::vals::Pllsaidivr {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Pllsaidivr::from_bits(val as u8)
        }
        #[doc = "division factor for LCD_CLK"]
        #[inline(always)]
        pub fn set_pllsaidivr(&mut self, val: super::vals::Pllsaidivr) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "SAI1 clock source selection"]
        #[inline(always)]
        pub const fn sai1sel(&self) -> super::vals::Saisel {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::Saisel::from_bits(val as u8)
        }
        #[doc = "SAI1 clock source selection"]
        #[inline(always)]
        pub fn set_sai1sel(&mut self, val: super::vals::Saisel) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "SAI2 clock source selection"]
        #[inline(always)]
        pub const fn sai2sel(&self) -> super::vals::Saisel {
            let val = (self.0 >> 22usize) & 0x03;
            super::vals::Saisel::from_bits(val as u8)
        }
        #[doc = "SAI2 clock source selection"]
        #[inline(always)]
        pub fn set_sai2sel(&mut self, val: super::vals::Saisel) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
        }
        #[doc = "Timers clocks prescalers selection"]
        #[inline(always)]
        pub const fn timpre(&self) -> super::vals::Timpre {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Timpre::from_bits(val as u8)
        }
        #[doc = "Timers clocks prescalers selection"]
        #[inline(always)]
        pub fn set_timpre(&mut self, val: super::vals::Timpre) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
        #[doc = "DFSDM1 clock source selection"]
        #[inline(always)]
        pub const fn dfsdm1sel(&self) -> super::vals::Dfsdmsel {
            let val = (self.0 >> 25usize) & 0x01;
            super::vals::Dfsdmsel::from_bits(val as u8)
        }
        #[doc = "DFSDM1 clock source selection"]
        #[inline(always)]
        pub fn set_dfsdm1sel(&mut self, val: super::vals::Dfsdmsel) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
        }
        #[doc = "DFSDM1 AUDIO clock source selection"]
        #[inline(always)]
        pub const fn adfsdm1sel(&self) -> super::vals::Adfsdmsel {
            let val = (self.0 >> 26usize) & 0x01;
            super::vals::Adfsdmsel::from_bits(val as u8)
        }
        #[doc = "DFSDM1 AUDIO clock source selection"]
        #[inline(always)]
        pub fn set_adfsdm1sel(&mut self, val: super::vals::Adfsdmsel) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
        }
    }
    impl Default for Dckcfgr1 {
        #[inline(always)]
        fn default() -> Dckcfgr1 {
            Dckcfgr1(0)
        }
    }
    #[doc = "dedicated clocks configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dckcfgr2(pub u32);
    impl Dckcfgr2 {
        #[doc = "USART 1 clock source selection"]
        #[inline(always)]
        pub const fn usart1sel(&self) -> super::vals::Usart1sel {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Usart1sel::from_bits(val as u8)
        }
        #[doc = "USART 1 clock source selection"]
        #[inline(always)]
        pub fn set_usart1sel(&mut self, val: super::vals::Usart1sel) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "USART 2 clock source selection"]
        #[inline(always)]
        pub const fn usart2sel(&self) -> super::vals::Usart2sel {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Usart2sel::from_bits(val as u8)
        }
        #[doc = "USART 2 clock source selection"]
        #[inline(always)]
        pub fn set_usart2sel(&mut self, val: super::vals::Usart2sel) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "USART 3 clock source selection"]
        #[inline(always)]
        pub const fn usart3sel(&self) -> super::vals::Usart2sel {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Usart2sel::from_bits(val as u8)
        }
        #[doc = "USART 3 clock source selection"]
        #[inline(always)]
        pub fn set_usart3sel(&mut self, val: super::vals::Usart2sel) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "UART 4 clock source selection"]
        #[inline(always)]
        pub const fn uart4sel(&self) -> super::vals::Usart2sel {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::Usart2sel::from_bits(val as u8)
        }
        #[doc = "UART 4 clock source selection"]
        #[inline(always)]
        pub fn set_uart4sel(&mut self, val: super::vals::Usart2sel) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "UART 5 clock source selection"]
        #[inline(always)]
        pub const fn uart5sel(&self) -> super::vals::Usart2sel {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Usart2sel::from_bits(val as u8)
        }
        #[doc = "UART 5 clock source selection"]
        #[inline(always)]
        pub fn set_uart5sel(&mut self, val: super::vals::Usart2sel) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "USART 6 clock source selection"]
        #[inline(always)]
        pub const fn usart6sel(&self) -> super::vals::Usart1sel {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::Usart1sel::from_bits(val as u8)
        }
        #[doc = "USART 6 clock source selection"]
        #[inline(always)]
        pub fn set_usart6sel(&mut self, val: super::vals::Usart1sel) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
        #[doc = "UART 7 clock source selection"]
        #[inline(always)]
        pub const fn uart7sel(&self) -> super::vals::Usart2sel {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Usart2sel::from_bits(val as u8)
        }
        #[doc = "UART 7 clock source selection"]
        #[inline(always)]
        pub fn set_uart7sel(&mut self, val: super::vals::Usart2sel) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "UART 8 clock source selection"]
        #[inline(always)]
        pub const fn uart8sel(&self) -> super::vals::Usart2sel {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::Usart2sel::from_bits(val as u8)
        }
        #[doc = "UART 8 clock source selection"]
        #[inline(always)]
        pub fn set_uart8sel(&mut self, val: super::vals::Usart2sel) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
        #[doc = "I2C1 clock source selection"]
        #[inline(always)]
        pub const fn i2c1sel(&self) -> super::vals::I2csel {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::I2csel::from_bits(val as u8)
        }
        #[doc = "I2C1 clock source selection"]
        #[inline(always)]
        pub fn set_i2c1sel(&mut self, val: super::vals::I2csel) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "I2C2 clock source selection"]
        #[inline(always)]
        pub const fn i2c2sel(&self) -> super::vals::I2csel {
            let val = (self.0 >> 18usize) & 0x03;
            super::vals::I2csel::from_bits(val as u8)
        }
        #[doc = "I2C2 clock source selection"]
        #[inline(always)]
        pub fn set_i2c2sel(&mut self, val: super::vals::I2csel) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
        }
        #[doc = "I2C3 clock source selection"]
        #[inline(always)]
        pub const fn i2c3sel(&self) -> super::vals::I2csel {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::I2csel::from_bits(val as u8)
        }
        #[doc = "I2C3 clock source selection"]
        #[inline(always)]
        pub fn set_i2c3sel(&mut self, val: super::vals::I2csel) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "I2C4 clock source selection"]
        #[inline(always)]
        pub const fn i2c4sel(&self) -> super::vals::I2csel {
            let val = (self.0 >> 22usize) & 0x03;
            super::vals::I2csel::from_bits(val as u8)
        }
        #[doc = "I2C4 clock source selection"]
        #[inline(always)]
        pub fn set_i2c4sel(&mut self, val: super::vals::I2csel) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
        }
        #[doc = "Low power timer 1 clock source selection"]
        #[inline(always)]
        pub const fn lptim1sel(&self) -> super::vals::Lptimsel {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Lptimsel::from_bits(val as u8)
        }
        #[doc = "Low power timer 1 clock source selection"]
        #[inline(always)]
        pub fn set_lptim1sel(&mut self, val: super::vals::Lptimsel) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "HDMI-CEC clock source selection"]
        #[inline(always)]
        pub const fn cecsel(&self) -> super::vals::Cecsel {
            let val = (self.0 >> 26usize) & 0x01;
            super::vals::Cecsel::from_bits(val as u8)
        }
        #[doc = "HDMI-CEC clock source selection"]
        #[inline(always)]
        pub fn set_cecsel(&mut self, val: super::vals::Cecsel) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
        }
        #[doc = "48MHz clock source selection"]
        #[inline(always)]
        pub const fn clk48sel(&self) -> super::vals::Clk48sel {
            let val = (self.0 >> 27usize) & 0x01;
            super::vals::Clk48sel::from_bits(val as u8)
        }
        #[doc = "48MHz clock source selection"]
        #[inline(always)]
        pub fn set_clk48sel(&mut self, val: super::vals::Clk48sel) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
        }
        #[doc = "SDMMC1 clock source selection"]
        #[inline(always)]
        pub const fn sdmmc1sel(&self) -> super::vals::Sdmmcsel {
            let val = (self.0 >> 28usize) & 0x01;
            super::vals::Sdmmcsel::from_bits(val as u8)
        }
        #[doc = "SDMMC1 clock source selection"]
        #[inline(always)]
        pub fn set_sdmmc1sel(&mut self, val: super::vals::Sdmmcsel) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
        }
        #[doc = "SDMMC2 clock source selection"]
        #[inline(always)]
        pub const fn sdmmc2sel(&self) -> super::vals::Sdmmcsel {
            let val = (self.0 >> 29usize) & 0x01;
            super::vals::Sdmmcsel::from_bits(val as u8)
        }
        #[doc = "SDMMC2 clock source selection"]
        #[inline(always)]
        pub fn set_sdmmc2sel(&mut self, val: super::vals::Sdmmcsel) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
        }
        #[doc = "DSI clock source selection"]
        #[inline(always)]
        pub const fn dsisel(&self) -> super::vals::Dsisel {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Dsisel::from_bits(val as u8)
        }
        #[doc = "DSI clock source selection"]
        #[inline(always)]
        pub fn set_dsisel(&mut self, val: super::vals::Dsisel) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Dckcfgr2 {
        #[inline(always)]
        fn default() -> Dckcfgr2 {
            Dckcfgr2(0)
        }
    }
    #[doc = "PLL configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pllcfgr(pub u32);
    impl Pllcfgr {
        #[doc = "Division factor for the PLL and audio PLL (PLLI2S) input clock"]
        #[inline(always)]
        pub const fn pllm(&self) -> super::vals::Pllm {
            let val = (self.0 >> 0usize) & 0x3f;
            super::vals::Pllm::from_bits(val as u8)
        }
        #[doc = "Division factor for the PLL and audio PLL (PLLI2S) input clock"]
        #[inline(always)]
        pub fn set_pllm(&mut self, val: super::vals::Pllm) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
        }
        #[doc = "PLL multiplication factor for VCO"]
        #[inline(always)]
        pub const fn plln(&self) -> super::vals::Plln {
            let val = (self.0 >> 6usize) & 0x01ff;
            super::vals::Plln::from_bits(val as u16)
        }
        #[doc = "PLL multiplication factor for VCO"]
        #[inline(always)]
        pub fn set_plln(&mut self, val: super::vals::Plln) {
            self.0 = (self.0 & !(0x01ff << 6usize)) | (((val.to_bits() as u32) & 0x01ff) << 6usize);
        }
        #[doc = "PLL division factor for main system clock"]
        #[inline(always)]
        pub const fn pllp(&self) -> super::vals::Pllp {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Pllp::from_bits(val as u8)
        }
        #[doc = "PLL division factor for main system clock"]
        #[inline(always)]
        pub fn set_pllp(&mut self, val: super::vals::Pllp) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "PLL and audio PLL (PLLI2S, PLLSAI) entry clock source"]
        #[inline(always)]
        pub const fn pllsrc(&self) -> super::vals::Pllsrc {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::Pllsrc::from_bits(val as u8)
        }
        #[doc = "PLL and audio PLL (PLLI2S, PLLSAI) entry clock source"]
        #[inline(always)]
        pub fn set_pllsrc(&mut self, val: super::vals::Pllsrc) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "PLL division factor for USB OTG FS, SDIO and random number generator clocks"]
        #[inline(always)]
        pub const fn pllq(&self) -> super::vals::Pllq {
            let val = (self.0 >> 24usize) & 0x0f;
            super::vals::Pllq::from_bits(val as u8)
        }
        #[doc = "PLL division factor for USB OTG FS, SDIO and random number generator clocks"]
        #[inline(always)]
        pub fn set_pllq(&mut self, val: super::vals::Pllq) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
        }
        #[doc = "PLL division factor for DSI clock"]
        #[inline(always)]
        pub const fn pllr(&self) -> super::vals::Pllr {
            let val = (self.0 >> 28usize) & 0x07;
            super::vals::Pllr::from_bits(val as u8)
        }
        #[doc = "PLL division factor for DSI clock"]
        #[inline(always)]
        pub fn set_pllr(&mut self, val: super::vals::Pllr) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Pllcfgr {
        #[inline(always)]
        fn default() -> Pllcfgr {
            Pllcfgr(0)
        }
    }
    #[doc = "PLLI2S configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Plli2scfgr(pub u32);
    impl Plli2scfgr {
        #[doc = "PLL multiplication factor for VCO"]
        #[inline(always)]
        pub const fn plln(&self) -> super::vals::Plln {
            let val = (self.0 >> 6usize) & 0x01ff;
            super::vals::Plln::from_bits(val as u16)
        }
        #[doc = "PLL multiplication factor for VCO"]
        #[inline(always)]
        pub fn set_plln(&mut self, val: super::vals::Plln) {
            self.0 = (self.0 & !(0x01ff << 6usize)) | (((val.to_bits() as u32) & 0x01ff) << 6usize);
        }
        #[doc = "PLL division factor for main system clock"]
        #[inline(always)]
        pub const fn pllp(&self) -> super::vals::Pllp {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Pllp::from_bits(val as u8)
        }
        #[doc = "PLL division factor for main system clock"]
        #[inline(always)]
        pub fn set_pllp(&mut self, val: super::vals::Pllp) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "PLL division factor for USB OTG FS, SDIO and random number generator clocks"]
        #[inline(always)]
        pub const fn pllq(&self) -> super::vals::Pllq {
            let val = (self.0 >> 24usize) & 0x0f;
            super::vals::Pllq::from_bits(val as u8)
        }
        #[doc = "PLL division factor for USB OTG FS, SDIO and random number generator clocks"]
        #[inline(always)]
        pub fn set_pllq(&mut self, val: super::vals::Pllq) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
        }
        #[doc = "PLL division factor for DSI clock"]
        #[inline(always)]
        pub const fn pllr(&self) -> super::vals::Pllr {
            let val = (self.0 >> 28usize) & 0x07;
            super::vals::Pllr::from_bits(val as u8)
        }
        #[doc = "PLL division factor for DSI clock"]
        #[inline(always)]
        pub fn set_pllr(&mut self, val: super::vals::Pllr) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Plli2scfgr {
        #[inline(always)]
        fn default() -> Plli2scfgr {
            Plli2scfgr(0)
        }
    }
    #[doc = "PLL configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pllsaicfgr(pub u32);
    impl Pllsaicfgr {
        #[doc = "PLL multiplication factor for VCO"]
        #[inline(always)]
        pub const fn plln(&self) -> super::vals::Plln {
            let val = (self.0 >> 6usize) & 0x01ff;
            super::vals::Plln::from_bits(val as u16)
        }
        #[doc = "PLL multiplication factor for VCO"]
        #[inline(always)]
        pub fn set_plln(&mut self, val: super::vals::Plln) {
            self.0 = (self.0 & !(0x01ff << 6usize)) | (((val.to_bits() as u32) & 0x01ff) << 6usize);
        }
        #[doc = "PLL division factor for main system clock"]
        #[inline(always)]
        pub const fn pllp(&self) -> super::vals::Pllp {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Pllp::from_bits(val as u8)
        }
        #[doc = "PLL division factor for main system clock"]
        #[inline(always)]
        pub fn set_pllp(&mut self, val: super::vals::Pllp) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "PLL division factor for USB OTG FS, SDIO and random number generator clocks"]
        #[inline(always)]
        pub const fn pllq(&self) -> super::vals::Pllq {
            let val = (self.0 >> 24usize) & 0x0f;
            super::vals::Pllq::from_bits(val as u8)
        }
        #[doc = "PLL division factor for USB OTG FS, SDIO and random number generator clocks"]
        #[inline(always)]
        pub fn set_pllq(&mut self, val: super::vals::Pllq) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
        }
        #[doc = "PLL division factor for DSI clock"]
        #[inline(always)]
        pub const fn pllr(&self) -> super::vals::Pllr {
            let val = (self.0 >> 28usize) & 0x07;
            super::vals::Pllr::from_bits(val as u8)
        }
        #[doc = "PLL division factor for DSI clock"]
        #[inline(always)]
        pub fn set_pllr(&mut self, val: super::vals::Pllr) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Pllsaicfgr {
        #[inline(always)]
        fn default() -> Pllsaicfgr {
            Pllsaicfgr(0)
        }
    }
    #[doc = "spread spectrum clock generation register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sscgr(pub u32);
    impl Sscgr {
        #[doc = "Modulation period"]
        #[inline(always)]
        pub const fn modper(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Modulation period"]
        #[inline(always)]
        pub fn set_modper(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "Incrementation step"]
        #[inline(always)]
        pub const fn incstep(&self) -> u16 {
            let val = (self.0 >> 13usize) & 0x7fff;
            val as u16
        }
        #[doc = "Incrementation step"]
        #[inline(always)]
        pub fn set_incstep(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 13usize)) | (((val as u32) & 0x7fff) << 13usize);
        }
        #[doc = "Spread Select"]
        #[inline(always)]
        pub const fn spreadsel(&self) -> super::vals::Spreadsel {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Spreadsel::from_bits(val as u8)
        }
        #[doc = "Spread Select"]
        #[inline(always)]
        pub fn set_spreadsel(&mut self, val: super::vals::Spreadsel) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Spread spectrum modulation enable"]
        #[inline(always)]
        pub const fn sscgen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Spread spectrum modulation enable"]
        #[inline(always)]
        pub fn set_sscgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sscgr {
        #[inline(always)]
        fn default() -> Sscgr {
            Sscgr(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Adfsdmsel {
        #[doc = "SAI1 clock selected as DFSDM1 Audio clock source"]
        SAI1 = 0x0,
        #[doc = "SAI2 clock selected as DFSDM1 Audio clock source"]
        SAI2 = 0x01,
    }
    impl Adfsdmsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Adfsdmsel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Adfsdmsel {
        #[inline(always)]
        fn from(val: u8) -> Adfsdmsel {
            Adfsdmsel::from_bits(val)
        }
    }
    impl From<Adfsdmsel> for u8 {
        #[inline(always)]
        fn from(val: Adfsdmsel) -> u8 {
            Adfsdmsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Cecsel {
        #[doc = "LSE clock is selected as HDMI-CEC clock"]
        LSE = 0x0,
        #[doc = "HSI divided by 488 clock is selected as HDMI-CEC clock"]
        HSI_DIV488 = 0x01,
    }
    impl Cecsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cecsel {
            unsafe { core::mem::transmute(val & 0x01) }
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Clk48sel {
        #[doc = "48MHz clock from PLL is selected"]
        PLL1_Q = 0x0,
        #[doc = "48MHz clock from PLLSAI is selected"]
        PLLSAI1_P = 0x01,
    }
    impl Clk48sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Clk48sel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Clk48sel {
        #[inline(always)]
        fn from(val: u8) -> Clk48sel {
            Clk48sel::from_bits(val)
        }
    }
    impl From<Clk48sel> for u8 {
        #[inline(always)]
        fn from(val: Clk48sel) -> u8 {
            Clk48sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Dfsdmsel {
        #[doc = "APB2 clock (PCLK2) selected as DFSDM1 Kernel clock source"]
        PCLK2 = 0x0,
        #[doc = "System clock (SYSCLK) clock selected as DFSDM1 Kernel clock source"]
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Dsisel {
        #[doc = "DSI-PHY used as DSI byte lane clock source (usual case)"]
        DSI_PHY = 0x0,
        #[doc = "PLLR used as DSI byte lane clock source, used in case DSI PLL and DSI-PHY are off (low power mode)"]
        PLL1_R = 0x01,
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum I2csel {
        #[doc = "APB clock selected as I2C clock"]
        PCLK1 = 0x0,
        #[doc = "System clock selected as I2C clock"]
        SYS = 0x01,
        #[doc = "HSI clock selected as I2C clock"]
        HSI = 0x02,
        _RESERVED_3 = 0x03,
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Issrc {
        #[doc = "PLLI2S clock used as I2S clock source"]
        PLLI2S = 0x0,
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Lptimsel {
        #[doc = "APB1 clock (PCLK1) selected as LPTILM1 clock"]
        PCLK1 = 0x0,
        #[doc = "LSI clock is selected as LPTILM1 clock"]
        LSI = 0x01,
        #[doc = "HSI clock is selected as LPTILM1 clock"]
        HSI = 0x02,
        #[doc = "LSE clock is selected as LPTILM1 clock"]
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Lsedrv {
        #[doc = "Low driving capability"]
        LOW = 0x0,
        #[doc = "Medium high driving capability"]
        MEDIUMHIGH = 0x01,
        #[doc = "Medium low driving capability"]
        MEDIUMLOW = 0x02,
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Mco1sel {
        #[doc = "HSI clock selected"]
        HSI = 0x0,
        #[doc = "LSE oscillator selected"]
        LSE = 0x01,
        #[doc = "HSE oscillator clock selected"]
        HSE = 0x02,
        #[doc = "PLL clock selected"]
        PLL = 0x03,
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Mco2sel {
        #[doc = "System clock (SYSCLK) selected"]
        SYS = 0x0,
        #[doc = "PLLI2S clock selected"]
        PLLI2S = 0x01,
        #[doc = "HSE oscillator clock selected"]
        HSE = 0x02,
        #[doc = "PLL clock selected"]
        PLL = 0x03,
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Mcopre {
        #[doc = "No division"]
        DIV1 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "Division by 2"]
        DIV2 = 0x04,
        #[doc = "Division by 3"]
        DIV3 = 0x05,
        #[doc = "Division by 4"]
        DIV4 = 0x06,
        #[doc = "Division by 5"]
        DIV5 = 0x07,
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Plli2sdivq {
        #[doc = "PLLI2SDIVQ = /1"]
        DIV1 = 0x0,
        #[doc = "PLLI2SDIVQ = /2"]
        DIV2 = 0x01,
        #[doc = "PLLI2SDIVQ = /3"]
        DIV3 = 0x02,
        #[doc = "PLLI2SDIVQ = /4"]
        DIV4 = 0x03,
        #[doc = "PLLI2SDIVQ = /5"]
        DIV5 = 0x04,
        #[doc = "PLLI2SDIVQ = /6"]
        DIV6 = 0x05,
        #[doc = "PLLI2SDIVQ = /7"]
        DIV7 = 0x06,
        #[doc = "PLLI2SDIVQ = /8"]
        DIV8 = 0x07,
        #[doc = "PLLI2SDIVQ = /9"]
        DIV9 = 0x08,
        #[doc = "PLLI2SDIVQ = /10"]
        DIV10 = 0x09,
        #[doc = "PLLI2SDIVQ = /11"]
        DIV11 = 0x0a,
        #[doc = "PLLI2SDIVQ = /12"]
        DIV12 = 0x0b,
        #[doc = "PLLI2SDIVQ = /13"]
        DIV13 = 0x0c,
        #[doc = "PLLI2SDIVQ = /14"]
        DIV14 = 0x0d,
        #[doc = "PLLI2SDIVQ = /15"]
        DIV15 = 0x0e,
        #[doc = "PLLI2SDIVQ = /16"]
        DIV16 = 0x0f,
        #[doc = "PLLI2SDIVQ = /17"]
        DIV17 = 0x10,
        #[doc = "PLLI2SDIVQ = /18"]
        DIV18 = 0x11,
        #[doc = "PLLI2SDIVQ = /19"]
        DIV19 = 0x12,
        #[doc = "PLLI2SDIVQ = /20"]
        DIV20 = 0x13,
        #[doc = "PLLI2SDIVQ = /21"]
        DIV21 = 0x14,
        #[doc = "PLLI2SDIVQ = /22"]
        DIV22 = 0x15,
        #[doc = "PLLI2SDIVQ = /23"]
        DIV23 = 0x16,
        #[doc = "PLLI2SDIVQ = /24"]
        DIV24 = 0x17,
        #[doc = "PLLI2SDIVQ = /25"]
        DIV25 = 0x18,
        #[doc = "PLLI2SDIVQ = /26"]
        DIV26 = 0x19,
        #[doc = "PLLI2SDIVQ = /27"]
        DIV27 = 0x1a,
        #[doc = "PLLI2SDIVQ = /28"]
        DIV28 = 0x1b,
        #[doc = "PLLI2SDIVQ = /29"]
        DIV29 = 0x1c,
        #[doc = "PLLI2SDIVQ = /30"]
        DIV30 = 0x1d,
        #[doc = "PLLI2SDIVQ = /31"]
        DIV31 = 0x1e,
        #[doc = "PLLI2SDIVQ = /32"]
        DIV32 = 0x1f,
    }
    impl Plli2sdivq {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Plli2sdivq {
            unsafe { core::mem::transmute(val & 0x1f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Plli2sdivq {
        #[inline(always)]
        fn from(val: u8) -> Plli2sdivq {
            Plli2sdivq::from_bits(val)
        }
    }
    impl From<Plli2sdivq> for u8 {
        #[inline(always)]
        fn from(val: Plli2sdivq) -> u8 {
            Plli2sdivq::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Pllm {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
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
        DIV63 = 0x3f,
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
        pub const MUL50: Self = Self(0x32);
        pub const MUL51: Self = Self(0x33);
        pub const MUL52: Self = Self(0x34);
        pub const MUL53: Self = Self(0x35);
        pub const MUL54: Self = Self(0x36);
        pub const MUL55: Self = Self(0x37);
        pub const MUL56: Self = Self(0x38);
        pub const MUL57: Self = Self(0x39);
        pub const MUL58: Self = Self(0x3a);
        pub const MUL59: Self = Self(0x3b);
        pub const MUL60: Self = Self(0x3c);
        pub const MUL61: Self = Self(0x3d);
        pub const MUL62: Self = Self(0x3e);
        pub const MUL63: Self = Self(0x3f);
        pub const MUL64: Self = Self(0x40);
        pub const MUL65: Self = Self(0x41);
        pub const MUL66: Self = Self(0x42);
        pub const MUL67: Self = Self(0x43);
        pub const MUL68: Self = Self(0x44);
        pub const MUL69: Self = Self(0x45);
        pub const MUL70: Self = Self(0x46);
        pub const MUL71: Self = Self(0x47);
        pub const MUL72: Self = Self(0x48);
        pub const MUL73: Self = Self(0x49);
        pub const MUL74: Self = Self(0x4a);
        pub const MUL75: Self = Self(0x4b);
        pub const MUL76: Self = Self(0x4c);
        pub const MUL77: Self = Self(0x4d);
        pub const MUL78: Self = Self(0x4e);
        pub const MUL79: Self = Self(0x4f);
        pub const MUL80: Self = Self(0x50);
        pub const MUL81: Self = Self(0x51);
        pub const MUL82: Self = Self(0x52);
        pub const MUL83: Self = Self(0x53);
        pub const MUL84: Self = Self(0x54);
        pub const MUL85: Self = Self(0x55);
        pub const MUL86: Self = Self(0x56);
        pub const MUL87: Self = Self(0x57);
        pub const MUL88: Self = Self(0x58);
        pub const MUL89: Self = Self(0x59);
        pub const MUL90: Self = Self(0x5a);
        pub const MUL91: Self = Self(0x5b);
        pub const MUL92: Self = Self(0x5c);
        pub const MUL93: Self = Self(0x5d);
        pub const MUL94: Self = Self(0x5e);
        pub const MUL95: Self = Self(0x5f);
        pub const MUL96: Self = Self(0x60);
        pub const MUL97: Self = Self(0x61);
        pub const MUL98: Self = Self(0x62);
        pub const MUL99: Self = Self(0x63);
        pub const MUL100: Self = Self(0x64);
        pub const MUL101: Self = Self(0x65);
        pub const MUL102: Self = Self(0x66);
        pub const MUL103: Self = Self(0x67);
        pub const MUL104: Self = Self(0x68);
        pub const MUL105: Self = Self(0x69);
        pub const MUL106: Self = Self(0x6a);
        pub const MUL107: Self = Self(0x6b);
        pub const MUL108: Self = Self(0x6c);
        pub const MUL109: Self = Self(0x6d);
        pub const MUL110: Self = Self(0x6e);
        pub const MUL111: Self = Self(0x6f);
        pub const MUL112: Self = Self(0x70);
        pub const MUL113: Self = Self(0x71);
        pub const MUL114: Self = Self(0x72);
        pub const MUL115: Self = Self(0x73);
        pub const MUL116: Self = Self(0x74);
        pub const MUL117: Self = Self(0x75);
        pub const MUL118: Self = Self(0x76);
        pub const MUL119: Self = Self(0x77);
        pub const MUL120: Self = Self(0x78);
        pub const MUL121: Self = Self(0x79);
        pub const MUL122: Self = Self(0x7a);
        pub const MUL123: Self = Self(0x7b);
        pub const MUL124: Self = Self(0x7c);
        pub const MUL125: Self = Self(0x7d);
        pub const MUL126: Self = Self(0x7e);
        pub const MUL127: Self = Self(0x7f);
        pub const MUL128: Self = Self(0x80);
        pub const MUL129: Self = Self(0x81);
        pub const MUL130: Self = Self(0x82);
        pub const MUL131: Self = Self(0x83);
        pub const MUL132: Self = Self(0x84);
        pub const MUL133: Self = Self(0x85);
        pub const MUL134: Self = Self(0x86);
        pub const MUL135: Self = Self(0x87);
        pub const MUL136: Self = Self(0x88);
        pub const MUL137: Self = Self(0x89);
        pub const MUL138: Self = Self(0x8a);
        pub const MUL139: Self = Self(0x8b);
        pub const MUL140: Self = Self(0x8c);
        pub const MUL141: Self = Self(0x8d);
        pub const MUL142: Self = Self(0x8e);
        pub const MUL143: Self = Self(0x8f);
        pub const MUL144: Self = Self(0x90);
        pub const MUL145: Self = Self(0x91);
        pub const MUL146: Self = Self(0x92);
        pub const MUL147: Self = Self(0x93);
        pub const MUL148: Self = Self(0x94);
        pub const MUL149: Self = Self(0x95);
        pub const MUL150: Self = Self(0x96);
        pub const MUL151: Self = Self(0x97);
        pub const MUL152: Self = Self(0x98);
        pub const MUL153: Self = Self(0x99);
        pub const MUL154: Self = Self(0x9a);
        pub const MUL155: Self = Self(0x9b);
        pub const MUL156: Self = Self(0x9c);
        pub const MUL157: Self = Self(0x9d);
        pub const MUL158: Self = Self(0x9e);
        pub const MUL159: Self = Self(0x9f);
        pub const MUL160: Self = Self(0xa0);
        pub const MUL161: Self = Self(0xa1);
        pub const MUL162: Self = Self(0xa2);
        pub const MUL163: Self = Self(0xa3);
        pub const MUL164: Self = Self(0xa4);
        pub const MUL165: Self = Self(0xa5);
        pub const MUL166: Self = Self(0xa6);
        pub const MUL167: Self = Self(0xa7);
        pub const MUL168: Self = Self(0xa8);
        pub const MUL169: Self = Self(0xa9);
        pub const MUL170: Self = Self(0xaa);
        pub const MUL171: Self = Self(0xab);
        pub const MUL172: Self = Self(0xac);
        pub const MUL173: Self = Self(0xad);
        pub const MUL174: Self = Self(0xae);
        pub const MUL175: Self = Self(0xaf);
        pub const MUL176: Self = Self(0xb0);
        pub const MUL177: Self = Self(0xb1);
        pub const MUL178: Self = Self(0xb2);
        pub const MUL179: Self = Self(0xb3);
        pub const MUL180: Self = Self(0xb4);
        pub const MUL181: Self = Self(0xb5);
        pub const MUL182: Self = Self(0xb6);
        pub const MUL183: Self = Self(0xb7);
        pub const MUL184: Self = Self(0xb8);
        pub const MUL185: Self = Self(0xb9);
        pub const MUL186: Self = Self(0xba);
        pub const MUL187: Self = Self(0xbb);
        pub const MUL188: Self = Self(0xbc);
        pub const MUL189: Self = Self(0xbd);
        pub const MUL190: Self = Self(0xbe);
        pub const MUL191: Self = Self(0xbf);
        pub const MUL192: Self = Self(0xc0);
        pub const MUL193: Self = Self(0xc1);
        pub const MUL194: Self = Self(0xc2);
        pub const MUL195: Self = Self(0xc3);
        pub const MUL196: Self = Self(0xc4);
        pub const MUL197: Self = Self(0xc5);
        pub const MUL198: Self = Self(0xc6);
        pub const MUL199: Self = Self(0xc7);
        pub const MUL200: Self = Self(0xc8);
        pub const MUL201: Self = Self(0xc9);
        pub const MUL202: Self = Self(0xca);
        pub const MUL203: Self = Self(0xcb);
        pub const MUL204: Self = Self(0xcc);
        pub const MUL205: Self = Self(0xcd);
        pub const MUL206: Self = Self(0xce);
        pub const MUL207: Self = Self(0xcf);
        pub const MUL208: Self = Self(0xd0);
        pub const MUL209: Self = Self(0xd1);
        pub const MUL210: Self = Self(0xd2);
        pub const MUL211: Self = Self(0xd3);
        pub const MUL212: Self = Self(0xd4);
        pub const MUL213: Self = Self(0xd5);
        pub const MUL214: Self = Self(0xd6);
        pub const MUL215: Self = Self(0xd7);
        pub const MUL216: Self = Self(0xd8);
        pub const MUL217: Self = Self(0xd9);
        pub const MUL218: Self = Self(0xda);
        pub const MUL219: Self = Self(0xdb);
        pub const MUL220: Self = Self(0xdc);
        pub const MUL221: Self = Self(0xdd);
        pub const MUL222: Self = Self(0xde);
        pub const MUL223: Self = Self(0xdf);
        pub const MUL224: Self = Self(0xe0);
        pub const MUL225: Self = Self(0xe1);
        pub const MUL226: Self = Self(0xe2);
        pub const MUL227: Self = Self(0xe3);
        pub const MUL228: Self = Self(0xe4);
        pub const MUL229: Self = Self(0xe5);
        pub const MUL230: Self = Self(0xe6);
        pub const MUL231: Self = Self(0xe7);
        pub const MUL232: Self = Self(0xe8);
        pub const MUL233: Self = Self(0xe9);
        pub const MUL234: Self = Self(0xea);
        pub const MUL235: Self = Self(0xeb);
        pub const MUL236: Self = Self(0xec);
        pub const MUL237: Self = Self(0xed);
        pub const MUL238: Self = Self(0xee);
        pub const MUL239: Self = Self(0xef);
        pub const MUL240: Self = Self(0xf0);
        pub const MUL241: Self = Self(0xf1);
        pub const MUL242: Self = Self(0xf2);
        pub const MUL243: Self = Self(0xf3);
        pub const MUL244: Self = Self(0xf4);
        pub const MUL245: Self = Self(0xf5);
        pub const MUL246: Self = Self(0xf6);
        pub const MUL247: Self = Self(0xf7);
        pub const MUL248: Self = Self(0xf8);
        pub const MUL249: Self = Self(0xf9);
        pub const MUL250: Self = Self(0xfa);
        pub const MUL251: Self = Self(0xfb);
        pub const MUL252: Self = Self(0xfc);
        pub const MUL253: Self = Self(0xfd);
        pub const MUL254: Self = Self(0xfe);
        pub const MUL255: Self = Self(0xff);
        pub const MUL256: Self = Self(0x0100);
        pub const MUL257: Self = Self(0x0101);
        pub const MUL258: Self = Self(0x0102);
        pub const MUL259: Self = Self(0x0103);
        pub const MUL260: Self = Self(0x0104);
        pub const MUL261: Self = Self(0x0105);
        pub const MUL262: Self = Self(0x0106);
        pub const MUL263: Self = Self(0x0107);
        pub const MUL264: Self = Self(0x0108);
        pub const MUL265: Self = Self(0x0109);
        pub const MUL266: Self = Self(0x010a);
        pub const MUL267: Self = Self(0x010b);
        pub const MUL268: Self = Self(0x010c);
        pub const MUL269: Self = Self(0x010d);
        pub const MUL270: Self = Self(0x010e);
        pub const MUL271: Self = Self(0x010f);
        pub const MUL272: Self = Self(0x0110);
        pub const MUL273: Self = Self(0x0111);
        pub const MUL274: Self = Self(0x0112);
        pub const MUL275: Self = Self(0x0113);
        pub const MUL276: Self = Self(0x0114);
        pub const MUL277: Self = Self(0x0115);
        pub const MUL278: Self = Self(0x0116);
        pub const MUL279: Self = Self(0x0117);
        pub const MUL280: Self = Self(0x0118);
        pub const MUL281: Self = Self(0x0119);
        pub const MUL282: Self = Self(0x011a);
        pub const MUL283: Self = Self(0x011b);
        pub const MUL284: Self = Self(0x011c);
        pub const MUL285: Self = Self(0x011d);
        pub const MUL286: Self = Self(0x011e);
        pub const MUL287: Self = Self(0x011f);
        pub const MUL288: Self = Self(0x0120);
        pub const MUL289: Self = Self(0x0121);
        pub const MUL290: Self = Self(0x0122);
        pub const MUL291: Self = Self(0x0123);
        pub const MUL292: Self = Self(0x0124);
        pub const MUL293: Self = Self(0x0125);
        pub const MUL294: Self = Self(0x0126);
        pub const MUL295: Self = Self(0x0127);
        pub const MUL296: Self = Self(0x0128);
        pub const MUL297: Self = Self(0x0129);
        pub const MUL298: Self = Self(0x012a);
        pub const MUL299: Self = Self(0x012b);
        pub const MUL300: Self = Self(0x012c);
        pub const MUL301: Self = Self(0x012d);
        pub const MUL302: Self = Self(0x012e);
        pub const MUL303: Self = Self(0x012f);
        pub const MUL304: Self = Self(0x0130);
        pub const MUL305: Self = Self(0x0131);
        pub const MUL306: Self = Self(0x0132);
        pub const MUL307: Self = Self(0x0133);
        pub const MUL308: Self = Self(0x0134);
        pub const MUL309: Self = Self(0x0135);
        pub const MUL310: Self = Self(0x0136);
        pub const MUL311: Self = Self(0x0137);
        pub const MUL312: Self = Self(0x0138);
        pub const MUL313: Self = Self(0x0139);
        pub const MUL314: Self = Self(0x013a);
        pub const MUL315: Self = Self(0x013b);
        pub const MUL316: Self = Self(0x013c);
        pub const MUL317: Self = Self(0x013d);
        pub const MUL318: Self = Self(0x013e);
        pub const MUL319: Self = Self(0x013f);
        pub const MUL320: Self = Self(0x0140);
        pub const MUL321: Self = Self(0x0141);
        pub const MUL322: Self = Self(0x0142);
        pub const MUL323: Self = Self(0x0143);
        pub const MUL324: Self = Self(0x0144);
        pub const MUL325: Self = Self(0x0145);
        pub const MUL326: Self = Self(0x0146);
        pub const MUL327: Self = Self(0x0147);
        pub const MUL328: Self = Self(0x0148);
        pub const MUL329: Self = Self(0x0149);
        pub const MUL330: Self = Self(0x014a);
        pub const MUL331: Self = Self(0x014b);
        pub const MUL332: Self = Self(0x014c);
        pub const MUL333: Self = Self(0x014d);
        pub const MUL334: Self = Self(0x014e);
        pub const MUL335: Self = Self(0x014f);
        pub const MUL336: Self = Self(0x0150);
        pub const MUL337: Self = Self(0x0151);
        pub const MUL338: Self = Self(0x0152);
        pub const MUL339: Self = Self(0x0153);
        pub const MUL340: Self = Self(0x0154);
        pub const MUL341: Self = Self(0x0155);
        pub const MUL342: Self = Self(0x0156);
        pub const MUL343: Self = Self(0x0157);
        pub const MUL344: Self = Self(0x0158);
        pub const MUL345: Self = Self(0x0159);
        pub const MUL346: Self = Self(0x015a);
        pub const MUL347: Self = Self(0x015b);
        pub const MUL348: Self = Self(0x015c);
        pub const MUL349: Self = Self(0x015d);
        pub const MUL350: Self = Self(0x015e);
        pub const MUL351: Self = Self(0x015f);
        pub const MUL352: Self = Self(0x0160);
        pub const MUL353: Self = Self(0x0161);
        pub const MUL354: Self = Self(0x0162);
        pub const MUL355: Self = Self(0x0163);
        pub const MUL356: Self = Self(0x0164);
        pub const MUL357: Self = Self(0x0165);
        pub const MUL358: Self = Self(0x0166);
        pub const MUL359: Self = Self(0x0167);
        pub const MUL360: Self = Self(0x0168);
        pub const MUL361: Self = Self(0x0169);
        pub const MUL362: Self = Self(0x016a);
        pub const MUL363: Self = Self(0x016b);
        pub const MUL364: Self = Self(0x016c);
        pub const MUL365: Self = Self(0x016d);
        pub const MUL366: Self = Self(0x016e);
        pub const MUL367: Self = Self(0x016f);
        pub const MUL368: Self = Self(0x0170);
        pub const MUL369: Self = Self(0x0171);
        pub const MUL370: Self = Self(0x0172);
        pub const MUL371: Self = Self(0x0173);
        pub const MUL372: Self = Self(0x0174);
        pub const MUL373: Self = Self(0x0175);
        pub const MUL374: Self = Self(0x0176);
        pub const MUL375: Self = Self(0x0177);
        pub const MUL376: Self = Self(0x0178);
        pub const MUL377: Self = Self(0x0179);
        pub const MUL378: Self = Self(0x017a);
        pub const MUL379: Self = Self(0x017b);
        pub const MUL380: Self = Self(0x017c);
        pub const MUL381: Self = Self(0x017d);
        pub const MUL382: Self = Self(0x017e);
        pub const MUL383: Self = Self(0x017f);
        pub const MUL384: Self = Self(0x0180);
        pub const MUL385: Self = Self(0x0181);
        pub const MUL386: Self = Self(0x0182);
        pub const MUL387: Self = Self(0x0183);
        pub const MUL388: Self = Self(0x0184);
        pub const MUL389: Self = Self(0x0185);
        pub const MUL390: Self = Self(0x0186);
        pub const MUL391: Self = Self(0x0187);
        pub const MUL392: Self = Self(0x0188);
        pub const MUL393: Self = Self(0x0189);
        pub const MUL394: Self = Self(0x018a);
        pub const MUL395: Self = Self(0x018b);
        pub const MUL396: Self = Self(0x018c);
        pub const MUL397: Self = Self(0x018d);
        pub const MUL398: Self = Self(0x018e);
        pub const MUL399: Self = Self(0x018f);
        pub const MUL400: Self = Self(0x0190);
        pub const MUL401: Self = Self(0x0191);
        pub const MUL402: Self = Self(0x0192);
        pub const MUL403: Self = Self(0x0193);
        pub const MUL404: Self = Self(0x0194);
        pub const MUL405: Self = Self(0x0195);
        pub const MUL406: Self = Self(0x0196);
        pub const MUL407: Self = Self(0x0197);
        pub const MUL408: Self = Self(0x0198);
        pub const MUL409: Self = Self(0x0199);
        pub const MUL410: Self = Self(0x019a);
        pub const MUL411: Self = Self(0x019b);
        pub const MUL412: Self = Self(0x019c);
        pub const MUL413: Self = Self(0x019d);
        pub const MUL414: Self = Self(0x019e);
        pub const MUL415: Self = Self(0x019f);
        pub const MUL416: Self = Self(0x01a0);
        pub const MUL417: Self = Self(0x01a1);
        pub const MUL418: Self = Self(0x01a2);
        pub const MUL419: Self = Self(0x01a3);
        pub const MUL420: Self = Self(0x01a4);
        pub const MUL421: Self = Self(0x01a5);
        pub const MUL422: Self = Self(0x01a6);
        pub const MUL423: Self = Self(0x01a7);
        pub const MUL424: Self = Self(0x01a8);
        pub const MUL425: Self = Self(0x01a9);
        pub const MUL426: Self = Self(0x01aa);
        pub const MUL427: Self = Self(0x01ab);
        pub const MUL428: Self = Self(0x01ac);
        pub const MUL429: Self = Self(0x01ad);
        pub const MUL430: Self = Self(0x01ae);
        pub const MUL431: Self = Self(0x01af);
        pub const MUL432: Self = Self(0x01b0);
    }
    impl Plln {
        pub const fn from_bits(val: u16) -> Plln {
            Self(val & 0x01ff)
        }
        pub const fn to_bits(self) -> u16 {
            self.0
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
    pub enum Pllp {
        #[doc = "PLLP=2"]
        DIV2 = 0x0,
        #[doc = "PLLP=4"]
        DIV4 = 0x01,
        #[doc = "PLLP=6"]
        DIV6 = 0x02,
        #[doc = "PLLP=8"]
        DIV8 = 0x03,
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Pllq {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Pllr {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        DIV2 = 0x02,
        DIV3 = 0x03,
        DIV4 = 0x04,
        DIV5 = 0x05,
        DIV6 = 0x06,
        DIV7 = 0x07,
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Pllsaidivq {
        #[doc = "PLLSAIDIVQ = /1"]
        DIV1 = 0x0,
        #[doc = "PLLSAIDIVQ = /2"]
        DIV2 = 0x01,
        #[doc = "PLLSAIDIVQ = /3"]
        DIV3 = 0x02,
        #[doc = "PLLSAIDIVQ = /4"]
        DIV4 = 0x03,
        #[doc = "PLLSAIDIVQ = /5"]
        DIV5 = 0x04,
        #[doc = "PLLSAIDIVQ = /6"]
        DIV6 = 0x05,
        #[doc = "PLLSAIDIVQ = /7"]
        DIV7 = 0x06,
        #[doc = "PLLSAIDIVQ = /8"]
        DIV8 = 0x07,
        #[doc = "PLLSAIDIVQ = /9"]
        DIV9 = 0x08,
        #[doc = "PLLSAIDIVQ = /10"]
        DIV10 = 0x09,
        #[doc = "PLLSAIDIVQ = /11"]
        DIV11 = 0x0a,
        #[doc = "PLLSAIDIVQ = /12"]
        DIV12 = 0x0b,
        #[doc = "PLLSAIDIVQ = /13"]
        DIV13 = 0x0c,
        #[doc = "PLLSAIDIVQ = /14"]
        DIV14 = 0x0d,
        #[doc = "PLLSAIDIVQ = /15"]
        DIV15 = 0x0e,
        #[doc = "PLLSAIDIVQ = /16"]
        DIV16 = 0x0f,
        #[doc = "PLLSAIDIVQ = /17"]
        DIV17 = 0x10,
        #[doc = "PLLSAIDIVQ = /18"]
        DIV18 = 0x11,
        #[doc = "PLLSAIDIVQ = /19"]
        DIV19 = 0x12,
        #[doc = "PLLSAIDIVQ = /20"]
        DIV20 = 0x13,
        #[doc = "PLLSAIDIVQ = /21"]
        DIV21 = 0x14,
        #[doc = "PLLSAIDIVQ = /22"]
        DIV22 = 0x15,
        #[doc = "PLLSAIDIVQ = /23"]
        DIV23 = 0x16,
        #[doc = "PLLSAIDIVQ = /24"]
        DIV24 = 0x17,
        #[doc = "PLLSAIDIVQ = /25"]
        DIV25 = 0x18,
        #[doc = "PLLSAIDIVQ = /26"]
        DIV26 = 0x19,
        #[doc = "PLLSAIDIVQ = /27"]
        DIV27 = 0x1a,
        #[doc = "PLLSAIDIVQ = /28"]
        DIV28 = 0x1b,
        #[doc = "PLLSAIDIVQ = /29"]
        DIV29 = 0x1c,
        #[doc = "PLLSAIDIVQ = /30"]
        DIV30 = 0x1d,
        #[doc = "PLLSAIDIVQ = /31"]
        DIV31 = 0x1e,
        #[doc = "PLLSAIDIVQ = /32"]
        DIV32 = 0x1f,
    }
    impl Pllsaidivq {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllsaidivq {
            unsafe { core::mem::transmute(val & 0x1f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllsaidivq {
        #[inline(always)]
        fn from(val: u8) -> Pllsaidivq {
            Pllsaidivq::from_bits(val)
        }
    }
    impl From<Pllsaidivq> for u8 {
        #[inline(always)]
        fn from(val: Pllsaidivq) -> u8 {
            Pllsaidivq::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Pllsaidivr {
        #[doc = "PLLSAIDIVR = /2"]
        DIV2 = 0x0,
        #[doc = "PLLSAIDIVR = /4"]
        DIV4 = 0x01,
        #[doc = "PLLSAIDIVR = /8"]
        DIV8 = 0x02,
        #[doc = "PLLSAIDIVR = /16"]
        DIV16 = 0x03,
    }
    impl Pllsaidivr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllsaidivr {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllsaidivr {
        #[inline(always)]
        fn from(val: u8) -> Pllsaidivr {
            Pllsaidivr::from_bits(val)
        }
    }
    impl From<Pllsaidivr> for u8 {
        #[inline(always)]
        fn from(val: Pllsaidivr) -> u8 {
            Pllsaidivr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Pllsrc {
        #[doc = "HSI clock selected as PLL and PLLI2S clock entry"]
        HSI = 0x0,
        #[doc = "HSE oscillator clock selected as PLL and PLLI2S clock entry"]
        HSE = 0x01,
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Saisel {
        #[doc = "SAI2 clock frequency = f(PLLSAI_Q) / PLLSAIDIVQ"]
        PLLSAI1_Q = 0x0,
        #[doc = "SAI2 clock frequency = f(PLLI2S_Q) / PLLI2SDIVQ"]
        PLLI2S1_Q = 0x01,
        #[doc = "SAI2 clock frequency = Alternate function input frequency"]
        AFIF = 0x02,
        #[doc = "SAI2 clock frequency = HSI or HSE"]
        HSI_HSE = 0x03,
    }
    impl Saisel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Saisel {
            unsafe { core::mem::transmute(val & 0x03) }
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Sdmmcsel {
        #[doc = "48 MHz clock is selected as SD clock"]
        CLK48 = 0x0,
        #[doc = "System clock is selected as SD clock"]
        SYS = 0x01,
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Spreadsel {
        #[doc = "Center spread"]
        CENTER = 0x0,
        #[doc = "Down spread"]
        DOWN = 0x01,
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Timpre {
        #[doc = "If the APB prescaler is configured 1, TIMxCLK = PCLKx. Otherwise, TIMxCLK = 2xPCLKx"]
        MUL2 = 0x0,
        #[doc = "If the APB prescaler is configured 1, 2 or 4, TIMxCLK = HCLK. Otherwise, TIMxCLK = 4xPCLKx"]
        MUL4 = 0x01,
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Usart1sel {
        #[doc = "APB2 clock (PCLK2) is selected as USART clock"]
        PCLK2 = 0x0,
        #[doc = "System clock is selected as USART clock"]
        SYS = 0x01,
        #[doc = "HSI clock is selected as USART clock"]
        HSI = 0x02,
        #[doc = "LSE clock is selected as USART clock"]
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Usart2sel {
        #[doc = "APB1 clock (PCLK1) is selected as USART clock"]
        PCLK1 = 0x0,
        #[doc = "System clock is selected as USART clock"]
        SYS = 0x01,
        #[doc = "HSI clock is selected as USART clock"]
        HSI = 0x02,
        #[doc = "LSE clock is selected as USART clock"]
        LSE = 0x03,
    }
    impl Usart2sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Usart2sel {
            unsafe { core::mem::transmute(val & 0x03) }
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
}
