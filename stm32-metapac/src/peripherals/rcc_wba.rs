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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "RCC internal clock sources calibration register 3"]
    #[inline(always)]
    pub const fn icscr3(self) -> crate::common::Reg<regs::Icscr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "RCC clock configuration register 1"]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::Cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "RCC clock configuration register 2"]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "RCC clock configuration register 3"]
    #[inline(always)]
    pub const fn cfgr3(self) -> crate::common::Reg<regs::Cfgr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "RCC PLL1 configuration register"]
    #[inline(always)]
    pub const fn pll1cfgr(self) -> crate::common::Reg<regs::Pll1cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "RCC PLL1 dividers register"]
    #[inline(always)]
    pub const fn pll1divr(self) -> crate::common::Reg<regs::Pll1divr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "RCC PLL1 fractional divider register"]
    #[inline(always)]
    pub const fn pll1fracr(self) -> crate::common::Reg<regs::Pll1fracr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "RCC clock interrupt enable register"]
    #[inline(always)]
    pub const fn cier(self) -> crate::common::Reg<regs::Cier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "RCC clock interrupt flag register"]
    #[inline(always)]
    pub const fn cifr(self) -> crate::common::Reg<regs::Cifr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "RCC clock interrupt clear register"]
    #[inline(always)]
    pub const fn cicr(self) -> crate::common::Reg<regs::Cicr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "RCC AHB1 peripheral reset register"]
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
    #[doc = "RCC AHB5 peripheral reset register"]
    #[inline(always)]
    pub const fn ahb5rstr(self) -> crate::common::Reg<regs::Ahb5rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "RCC APB1 peripheral reset register 1"]
    #[inline(always)]
    pub const fn apb1rstr1(self) -> crate::common::Reg<regs::Apb1rstr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "RCC APB1 peripheral reset register 2"]
    #[inline(always)]
    pub const fn apb1rstr2(self) -> crate::common::Reg<regs::Apb1rstr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "RCC APB2 peripheral reset register"]
    #[inline(always)]
    pub const fn apb2rstr(self) -> crate::common::Reg<regs::Apb2rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "RCC APB7 peripheral reset register"]
    #[inline(always)]
    pub const fn apb7rstr(self) -> crate::common::Reg<regs::Apb7rstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "RCC AHB1 peripheral clock enable register"]
    #[inline(always)]
    pub const fn ahb1enr(self) -> crate::common::Reg<regs::Ahb1enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "RCC AHB2 peripheral clock enable register"]
    #[inline(always)]
    pub const fn ahb2enr(self) -> crate::common::Reg<regs::Ahb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "RCC AHB4 peripheral clock enable register"]
    #[inline(always)]
    pub const fn ahb4enr(self) -> crate::common::Reg<regs::Ahb4enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "RCC AHB5 peripheral clock enable register"]
    #[inline(always)]
    pub const fn ahb5enr(self) -> crate::common::Reg<regs::Ahb5enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "RCC APB1 peripheral clock enable register 1"]
    #[inline(always)]
    pub const fn apb1enr1(self) -> crate::common::Reg<regs::Apb1enr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "RCC APB1 peripheral clock enable register 2"]
    #[inline(always)]
    pub const fn apb1enr2(self) -> crate::common::Reg<regs::Apb1enr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "RCC APB2 peripheral clock enable register"]
    #[inline(always)]
    pub const fn apb2enr(self) -> crate::common::Reg<regs::Apb2enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "RCC APB7 peripheral clock enable register"]
    #[inline(always)]
    pub const fn apb7enr(self) -> crate::common::Reg<regs::Apb7enr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "RCC AHB1 peripheral clocks enable in Sleep and Stop modes register"]
    #[inline(always)]
    pub const fn ahb1smenr(self) -> crate::common::Reg<regs::Ahb1smenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "RCC AHB2 peripheral clocks enable in Sleep and Stop modes register"]
    #[inline(always)]
    pub const fn ahb2smenr(self) -> crate::common::Reg<regs::Ahb2smenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "RCC AHB4 peripheral clocks enable in Sleep and Stop modes register"]
    #[inline(always)]
    pub const fn ahb4smenr(self) -> crate::common::Reg<regs::Ahb4smenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "RCC AHB5 peripheral clocks enable in Sleep and Stop modes register"]
    #[inline(always)]
    pub const fn ahb5smenr(self) -> crate::common::Reg<regs::Ahb5smenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "RCC APB1 peripheral clocks enable in Sleep and Stop modes register 1"]
    #[inline(always)]
    pub const fn apb1smenr1(self) -> crate::common::Reg<regs::Apb1smenr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "RCC APB1 peripheral clocks enable in Sleep and Stop modes register 2"]
    #[inline(always)]
    pub const fn apb1smenr2(self) -> crate::common::Reg<regs::Apb1smenr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "RCC APB2 peripheral clocks enable in Sleep and Stop modes register"]
    #[inline(always)]
    pub const fn apb2smenr(self) -> crate::common::Reg<regs::Apb2smenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "RCC APB7 peripheral clock enable in Sleep and Stop modes register"]
    #[inline(always)]
    pub const fn apb7smenr(self) -> crate::common::Reg<regs::Apb7smenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "RCC peripherals independent clock configuration register 1"]
    #[inline(always)]
    pub const fn ccipr1(self) -> crate::common::Reg<regs::Ccipr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "RCC peripherals independent clock configuration register 2"]
    #[inline(always)]
    pub const fn ccipr2(self) -> crate::common::Reg<regs::Ccipr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "RCC peripherals independent clock configuration register 3"]
    #[inline(always)]
    pub const fn ccipr3(self) -> crate::common::Reg<regs::Ccipr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "RCC backup domain control register"]
    #[inline(always)]
    pub const fn bdcr(self) -> crate::common::Reg<regs::Bdcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "RCC control/status register"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "RCC Backup domain control register"]
    #[inline(always)]
    pub const fn bdcr2(self) -> crate::common::Reg<regs::Bdcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "RCC secure configuration register"]
    #[inline(always)]
    pub const fn seccfgr(self) -> crate::common::Reg<regs::Seccfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "RCC privilege configuration register"]
    #[inline(always)]
    pub const fn privcfgr(self) -> crate::common::Reg<regs::Privcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "RCC audio synchronization control register"]
    #[inline(always)]
    pub const fn ascr(self) -> crate::common::Reg<regs::Ascr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "RCC audio synchronization interrupt enable register"]
    #[inline(always)]
    pub const fn asier(self) -> crate::common::Reg<regs::Asier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "RCC audio synchronization status register"]
    #[inline(always)]
    pub const fn assr(self) -> crate::common::Reg<regs::Assr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
    #[doc = "RCC audio synchronization counter register"]
    #[inline(always)]
    pub const fn ascntr(self) -> crate::common::Reg<regs::Ascntr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
    }
    #[doc = "RCC audio synchronization auto-reload register"]
    #[inline(always)]
    pub const fn asarr(self) -> crate::common::Reg<regs::Asarr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "RCC audio synchronization capture register"]
    #[inline(always)]
    pub const fn ascar(self) -> crate::common::Reg<regs::Ascar, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d4usize) as _) }
    }
    #[doc = "RCC audio synchronization compare register"]
    #[inline(always)]
    pub const fn ascor(self) -> crate::common::Reg<regs::Ascor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d8usize) as _) }
    }
    #[doc = "RCC clock configuration register 2"]
    #[inline(always)]
    pub const fn cfgr4(self) -> crate::common::Reg<regs::Cfgr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "RCC RADIO peripheral clock enable register"]
    #[inline(always)]
    pub const fn radioenr(self) -> crate::common::Reg<regs::Radioenr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "RCC external clock sources calibration register 1"]
    #[inline(always)]
    pub const fn ecscr1(self) -> crate::common::Reg<regs::Ecscr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
}
pub mod regs {
    #[doc = "RCC AHB1 peripheral clock enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1enr(pub u32);
    impl Ahb1enr {
        #[doc = "GPDMA1 bus clock enable Set and cleared by software. Access can be secured by GPDMA1 SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn gpdma1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1 bus clock enable Set and cleared by software. Access can be secured by GPDMA1 SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_gpdma1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "FLASH bus clock enable Set and cleared by software. This bit can be disabled only when the Flash memory is in power down mode. Can only be accessed secured when the Flash security state is secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn flashen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FLASH bus clock enable Set and cleared by software. This bit can be disabled only when the Flash memory is in power down mode. Can only be accessed secured when the Flash security state is secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_flashen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CRC bus clock enable Set and cleared by software. Access can be secured by GTZC_TZSC CRCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn crcen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC bus clock enable Set and cleared by software. Access can be secured by GTZC_TZSC CRCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_crcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Touch sensing controller bus clock enable Set and cleared by software. Access can be secured by GTZC_TZSC TSCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn tscen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Touch sensing controller bus clock enable Set and cleared by software. Access can be secured by GTZC_TZSC TSCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_tscen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "RAMCFG bus clock enable Set and cleared by software. Access can be secured by GTZC_TZSC RAMCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn ramcfgen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "RAMCFG bus clock enable Set and cleared by software. Access can be secured by GTZC_TZSC RAMCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_ramcfgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "GTZC1 bus clock enable Set and reset by software. Can only be accessed secure when device is secure (TZEN = 1). When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn gtzc1en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "GTZC1 bus clock enable Set and reset by software. Can only be accessed secure when device is secure (TZEN = 1). When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_gtzc1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "SRAM1 bus clock enable Set and reset by software. Access can be secured by GTZC_MPCBB1 SECx, INVSECSTATE. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn sram1en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 bus clock enable Set and reset by software. Access can be secured by GTZC_MPCBB1 SECx, INVSECSTATE. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
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
                .field("flashen", &self.flashen())
                .field("crcen", &self.crcen())
                .field("tscen", &self.tscen())
                .field("ramcfgen", &self.ramcfgen())
                .field("gtzc1en", &self.gtzc1en())
                .field("sram1en", &self.sram1en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1enr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb1enr {{ gpdma1en: {=bool:?}, flashen: {=bool:?}, crcen: {=bool:?}, tscen: {=bool:?}, ramcfgen: {=bool:?}, gtzc1en: {=bool:?}, sram1en: {=bool:?} }}" , self . gpdma1en () , self . flashen () , self . crcen () , self . tscen () , self . ramcfgen () , self . gtzc1en () , self . sram1en ())
        }
    }
    #[doc = "RCC AHB1 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1rstr(pub u32);
    impl Ahb1rstr {
        #[doc = "GPDMA1 reset Set and cleared by software. Access can be secured by GPDMA1 SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn gpdma1rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1 reset Set and cleared by software. Access can be secured by GPDMA1 SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_gpdma1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CRC reset Set and cleared by software. Access can be secured by GTZC_TZSC CRCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn crcrst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC reset Set and cleared by software. Access can be secured by GTZC_TZSC CRCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_crcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TSC reset Set and cleared by software. Access can be secured by GTZC_TZSC TSCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn tscrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TSC reset Set and cleared by software. Access can be secured by GTZC_TZSC TSCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_tscrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
                .field("crcrst", &self.crcrst())
                .field("tscrst", &self.tscrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb1rstr {{ gpdma1rst: {=bool:?}, crcrst: {=bool:?}, tscrst: {=bool:?} }}",
                self.gpdma1rst(),
                self.crcrst(),
                self.tscrst()
            )
        }
    }
    #[doc = "RCC AHB1 peripheral clocks enable in Sleep and Stop modes register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1smenr(pub u32);
    impl Ahb1smenr {
        #[doc = "GPDMA1 bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPDMA1 SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[must_use]
        #[inline(always)]
        pub const fn gpdma1smen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1 bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPDMA1 SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn set_gpdma1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "FLASH bus clock enable during Sleep and Stop modes Set and cleared by software. Can only be accessed secured when the Flash security state is secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn flashsmen(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FLASH bus clock enable during Sleep and Stop modes Set and cleared by software. Can only be accessed secured when the Flash security state is secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_flashsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CRC bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC CRCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn crcsmen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "CRC bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC CRCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_crcsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "TSC bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC TSCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.."]
        #[must_use]
        #[inline(always)]
        pub const fn tscsmen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TSC bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC TSCSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.."]
        #[inline(always)]
        pub const fn set_tscsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "RAMCFG bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC RAMCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn ramcfgsmen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "RAMCFG bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC RAMCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_ramcfgsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "GTZC1 bus clock enable during Sleep and Stop modes Set and cleared by software. Can only be accessed secure when one device is secure (TZEN = 1). When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn gtzc1smen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "GTZC1 bus clock enable during Sleep and Stop modes Set and cleared by software. Can only be accessed secure when one device is secure (TZEN = 1). When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_gtzc1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "ICACHE bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC ICACHE_REGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.."]
        #[must_use]
        #[inline(always)]
        pub const fn icachesmen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "ICACHE bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC ICACHE_REGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.."]
        #[inline(always)]
        pub const fn set_icachesmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "SRAM1 bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_MPCBB1 SECx, INVSECSTATE. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn sram1smen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_MPCBB1 SECx, INVSECSTATE. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_sram1smen(&mut self, val: bool) {
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
                .field("flashsmen", &self.flashsmen())
                .field("crcsmen", &self.crcsmen())
                .field("tscsmen", &self.tscsmen())
                .field("ramcfgsmen", &self.ramcfgsmen())
                .field("gtzc1smen", &self.gtzc1smen())
                .field("icachesmen", &self.icachesmen())
                .field("sram1smen", &self.sram1smen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1smenr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb1smenr {{ gpdma1smen: {=bool:?}, flashsmen: {=bool:?}, crcsmen: {=bool:?}, tscsmen: {=bool:?}, ramcfgsmen: {=bool:?}, gtzc1smen: {=bool:?}, icachesmen: {=bool:?}, sram1smen: {=bool:?} }}" , self . gpdma1smen () , self . flashsmen () , self . crcsmen () , self . tscsmen () , self . ramcfgsmen () , self . gtzc1smen () , self . icachesmen () , self . sram1smen ())
        }
    }
    #[doc = "RCC AHB2 peripheral clock enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2enr(pub u32);
    impl Ahb2enr {
        #[doc = "IO port A bus clock enable Set and cleared by software. Access can be secured by GPIOA SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioaen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IO port A bus clock enable Set and cleared by software. Access can be secured by GPIOA SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_gpioaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IO port B bus clock enable Set and cleared by software. Access can be secured by GPIOB SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioben(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IO port B bus clock enable Set and cleared by software. Access can be secured by GPIOB SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_gpioben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IO port C bus clock enable Set and cleared by software. Access can be secured by GPIOC SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiocen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IO port C bus clock enable Set and cleared by software. Access can be secured by GPIOC SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_gpiocen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "IO port D bus clock enable Set and cleared by software. Access can be secured by GPIOD SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioden(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IO port D bus clock enable Set and cleared by software. Access can be secured by GPIOD SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_gpioden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "IO port E bus clock enable Set and cleared by software. Access can be secured by GPIOE SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioeen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IO port E bus clock enable Set and cleared by software. Access can be secured by GPIOE SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_gpioeen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "IO port G bus clock enable Set and cleared by software. Access can be secured by GPIOG SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiogen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "IO port G bus clock enable Set and cleared by software. Access can be secured by GPIOG SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_gpiogen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "IO port H bus clock enable Set and cleared by software. Access can be secured by GPIOH SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiohen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "IO port H bus clock enable Set and cleared by software. Access can be secured by GPIOH SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_gpiohen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "USB OTG_HS bus and kernel clock enable Set and cleared by software. Access can be secured by GTZC_TZSC OTGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn usb_otg_hsen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USB OTG_HS bus and kernel clock enable Set and cleared by software. Access can be secured by GTZC_TZSC OTGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_usb_otg_hsen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "USB OTG_HS PHY kernel clock enable Set and cleared by software. Access can be secured by GTZC_TZSC OTGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn usb_otg_hs_phyen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "USB OTG_HS PHY kernel clock enable Set and cleared by software. Access can be secured by GTZC_TZSC OTGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_usb_otg_hs_phyen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "AES bus clock enable Set and cleared by software. Access can be secured by GTZC_TZSC AESSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn aesen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "AES bus clock enable Set and cleared by software. Access can be secured by GTZC_TZSC AESSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_aesen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "HASH bus clock enable Set and cleared by software. Access can be secured by GTZC_TZSC HASHSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn hashen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "HASH bus clock enable Set and cleared by software. Access can be secured by GTZC_TZSC HASHSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_hashen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "RNG bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC RNGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn rngen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "RNG bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC RNGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_rngen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SAES bus clock enable Set and cleared by software. Access can be secured by GTZC_TZSC SAESSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn saesen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "SAES bus clock enable Set and cleared by software. Access can be secured by GTZC_TZSC SAESSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_saesen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "HSEM bus clock enable Set and cleared by software. Can only be accessed secure when one or more features in the HSEM is secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn hsemen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "HSEM bus clock enable Set and cleared by software. Can only be accessed secure when one or more features in the HSEM is secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_hsemen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "PKA bus clock enable Set and cleared by software. Access can be secured by GTZC_TZSC PKASEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn pkaen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "PKA bus clock enable Set and cleared by software. Access can be secured by GTZC_TZSC PKASEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_pkaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SRAM2 bus clock enable Set and cleared by software. Access can be secured by GTZC_MPCBB2 SECx, INVSECSTATE. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn sram2en(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 bus clock enable Set and cleared by software. Access can be secured by GTZC_MPCBB2 SECx, INVSECSTATE. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_sram2en(&mut self, val: bool) {
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
                .field("gpioeen", &self.gpioeen())
                .field("gpiogen", &self.gpiogen())
                .field("gpiohen", &self.gpiohen())
                .field("usb_otg_hsen", &self.usb_otg_hsen())
                .field("usb_otg_hs_phyen", &self.usb_otg_hs_phyen())
                .field("aesen", &self.aesen())
                .field("hashen", &self.hashen())
                .field("rngen", &self.rngen())
                .field("saesen", &self.saesen())
                .field("hsemen", &self.hsemen())
                .field("pkaen", &self.pkaen())
                .field("sram2en", &self.sram2en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2enr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb2enr {{ gpioaen: {=bool:?}, gpioben: {=bool:?}, gpiocen: {=bool:?}, gpioden: {=bool:?}, gpioeen: {=bool:?}, gpiogen: {=bool:?}, gpiohen: {=bool:?}, usb_otg_hsen: {=bool:?}, usb_otg_hs_phyen: {=bool:?}, aesen: {=bool:?}, hashen: {=bool:?}, rngen: {=bool:?}, saesen: {=bool:?}, hsemen: {=bool:?}, pkaen: {=bool:?}, sram2en: {=bool:?} }}" , self . gpioaen () , self . gpioben () , self . gpiocen () , self . gpioden () , self . gpioeen () , self . gpiogen () , self . gpiohen () , self . usb_otg_hsen () , self . usb_otg_hs_phyen () , self . aesen () , self . hashen () , self . rngen () , self . saesen () , self . hsemen () , self . pkaen () , self . sram2en ())
        }
    }
    #[doc = "RCC AHB2 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2rstr(pub u32);
    impl Ahb2rstr {
        #[doc = "IO port A reset Set and cleared by software. Access can be secured by GPIOA SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioarst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IO port A reset Set and cleared by software. Access can be secured by GPIOA SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_gpioarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IO port B reset Set and cleared by software. Access can be secured by GPIOB SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiobrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IO port B reset Set and cleared by software. Access can be secured by GPIOB SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_gpiobrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IO port C reset Set and cleared by software. Access can be secured by GPIOC SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiocrst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IO port C reset Set and cleared by software. Access can be secured by GPIOC SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_gpiocrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "IO port D reset Set and cleared by software. Access can be secured by GPIOD SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiodrst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IO port D reset Set and cleared by software. Access can be secured by GPIOD SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_gpiodrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "IO port E reset Set and cleared by software. Access can be secured by GPIOE SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioerst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IO port E reset Set and cleared by software. Access can be secured by GPIOE SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_gpioerst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "IO port G reset Set and cleared by software. Access can be secured by GPIOG SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiogrst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "IO port G reset Set and cleared by software. Access can be secured by GPIOG SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_gpiogrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "IO port H reset Set and cleared by software. Access can be secured by GPIOH SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiohrst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "IO port H reset Set and cleared by software. Access can be secured by GPIOH SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_gpiohrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "USB OTG_HS reset Set and cleared by software. Access can be secured by GTZC_TZSC OTGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn usb_otg_hsrst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USB OTG_HS reset Set and cleared by software. Access can be secured by GTZC_TZSC OTGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_usb_otg_hsrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "AES hardware accelerator reset Set and cleared by software. Access can be secured by GTZC_TZSC AESSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn aesrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "AES hardware accelerator reset Set and cleared by software. Access can be secured by GTZC_TZSC AESSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_aesrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Hash reset Set and cleared by software. Access can be secured by GTZC_TZSC HASHSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn hashrst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Hash reset Set and cleared by software. Access can be secured by GTZC_TZSC HASHSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_hashrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Random number generator reset Set and cleared by software. Access can be secured by GTZC_TZSC RNGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn rngrst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Random number generator reset Set and cleared by software. Access can be secured by GTZC_TZSC RNGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_rngrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SAES hardware accelerator reset Set and cleared by software. Access can be secured by GTZC_TZSC SAESSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn saesrst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "SAES hardware accelerator reset Set and cleared by software. Access can be secured by GTZC_TZSC SAESSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_saesrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "HSEM hardware accelerator reset Set and cleared by software. Can only be accessed secure when one or more features in the HSEM is secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn hsemrst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "HSEM hardware accelerator reset Set and cleared by software. Can only be accessed secure when one or more features in the HSEM is secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_hsemrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "PKA reset Set and cleared by software. Access can be secured by GTZC_TZSC PKASEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn pkarst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "PKA reset Set and cleared by software. Access can be secured by GTZC_TZSC PKASEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_pkarst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
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
                .field("gpiogrst", &self.gpiogrst())
                .field("gpiohrst", &self.gpiohrst())
                .field("usb_otg_hsrst", &self.usb_otg_hsrst())
                .field("aesrst", &self.aesrst())
                .field("hashrst", &self.hashrst())
                .field("rngrst", &self.rngrst())
                .field("saesrst", &self.saesrst())
                .field("hsemrst", &self.hsemrst())
                .field("pkarst", &self.pkarst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb2rstr {{ gpioarst: {=bool:?}, gpiobrst: {=bool:?}, gpiocrst: {=bool:?}, gpiodrst: {=bool:?}, gpioerst: {=bool:?}, gpiogrst: {=bool:?}, gpiohrst: {=bool:?}, usb_otg_hsrst: {=bool:?}, aesrst: {=bool:?}, hashrst: {=bool:?}, rngrst: {=bool:?}, saesrst: {=bool:?}, hsemrst: {=bool:?}, pkarst: {=bool:?} }}" , self . gpioarst () , self . gpiobrst () , self . gpiocrst () , self . gpiodrst () , self . gpioerst () , self . gpiogrst () , self . gpiohrst () , self . usb_otg_hsrst () , self . aesrst () , self . hashrst () , self . rngrst () , self . saesrst () , self . hsemrst () , self . pkarst ())
        }
    }
    #[doc = "RCC AHB2 peripheral clocks enable in Sleep and Stop modes register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb2smenr(pub u32);
    impl Ahb2smenr {
        #[doc = "IO port A bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPIOA SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioasmen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "IO port A bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPIOA SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_gpioasmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IO port B bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPIOB SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiobsmen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IO port B bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPIOB SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_gpiobsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IO port C bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPIOC SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiocsmen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "IO port C bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPIOC SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_gpiocsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "IO port D bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPIOD SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiodsmen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "IO port D bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPIOD SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_gpiodsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "IO port E bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPIOE SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn gpioesmen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IO port E bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPIOE SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_gpioesmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "IO port G bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPIOG SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiogsmen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "IO port G bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPIOG SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_gpiogsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "IO port H bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPIOH SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn gpiohsmen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "IO port H bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GPIOH SECx. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_gpiohsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "USB OTG_HS bus and kernel clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC OTGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn usb_otg_hssmen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USB OTG_HS bus and kernel clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC OTGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_usb_otg_hssmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "USB OTG_HS PHY kernel clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC OTGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn usb_otg_hs_physmen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "USB OTG_HS PHY kernel clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC OTGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_usb_otg_hs_physmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "AES bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC AESSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn aessmen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "AES bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC AESSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_aessmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "HASH bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC HASHSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn hashsmen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "HASH bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC HASHSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_hashsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Random number generator (RNG) bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC RNGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn rngsmen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Random number generator (RNG) bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC RNGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_rngsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SAES accelerator bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC SAESSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn saessmen(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "SAES accelerator bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC SAESSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_saessmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "PKA bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC PKASEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn pkasmen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "PKA bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC PKASEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_pkasmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SRAM2 bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_MPCBB2 SECx, INVSECSTATE. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn sram2smen(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_MPCBB2 SECx, INVSECSTATE. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_sram2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Ahb2smenr {
        #[inline(always)]
        fn default() -> Ahb2smenr {
            Ahb2smenr(0)
        }
    }
    impl core::fmt::Debug for Ahb2smenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb2smenr")
                .field("gpioasmen", &self.gpioasmen())
                .field("gpiobsmen", &self.gpiobsmen())
                .field("gpiocsmen", &self.gpiocsmen())
                .field("gpiodsmen", &self.gpiodsmen())
                .field("gpioesmen", &self.gpioesmen())
                .field("gpiogsmen", &self.gpiogsmen())
                .field("gpiohsmen", &self.gpiohsmen())
                .field("usb_otg_hssmen", &self.usb_otg_hssmen())
                .field("usb_otg_hs_physmen", &self.usb_otg_hs_physmen())
                .field("aessmen", &self.aessmen())
                .field("hashsmen", &self.hashsmen())
                .field("rngsmen", &self.rngsmen())
                .field("saessmen", &self.saessmen())
                .field("pkasmen", &self.pkasmen())
                .field("sram2smen", &self.sram2smen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb2smenr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ahb2smenr {{ gpioasmen: {=bool:?}, gpiobsmen: {=bool:?}, gpiocsmen: {=bool:?}, gpiodsmen: {=bool:?}, gpioesmen: {=bool:?}, gpiogsmen: {=bool:?}, gpiohsmen: {=bool:?}, usb_otg_hssmen: {=bool:?}, usb_otg_hs_physmen: {=bool:?}, aessmen: {=bool:?}, hashsmen: {=bool:?}, rngsmen: {=bool:?}, saessmen: {=bool:?}, pkasmen: {=bool:?}, sram2smen: {=bool:?} }}" , self . gpioasmen () , self . gpiobsmen () , self . gpiocsmen () , self . gpiodsmen () , self . gpioesmen () , self . gpiogsmen () , self . gpiohsmen () , self . usb_otg_hssmen () , self . usb_otg_hs_physmen () , self . aessmen () , self . hashsmen () , self . rngsmen () , self . saessmen () , self . pkasmen () , self . sram2smen ())
        }
    }
    #[doc = "RCC AHB4 peripheral clock enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb4enr(pub u32);
    impl Ahb4enr {
        #[doc = "PWR bus clock enable Set and cleared by software. Can only be accessed secure when one or more features in the PWR is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn pwren(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PWR bus clock enable Set and cleared by software. Can only be accessed secure when one or more features in the PWR is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_pwren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ADC4 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC ADC4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn adc4en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ADC4 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC ADC4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_adc4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("pwren", &self.pwren())
                .field("adc4en", &self.adc4en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb4enr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb4enr {{ pwren: {=bool:?}, adc4en: {=bool:?} }}",
                self.pwren(),
                self.adc4en()
            )
        }
    }
    #[doc = "RCC AHB4 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb4rstr(pub u32);
    impl Ahb4rstr {
        #[doc = "ADC4 reset Set and cleared by software. Access can be secred by GTZC_TZSC ADC4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn adc4rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ADC4 reset Set and cleared by software. Access can be secred by GTZC_TZSC ADC4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_adc4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
            f.debug_struct("Ahb4rstr").field("adc4rst", &self.adc4rst()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb4rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ahb4rstr {{ adc4rst: {=bool:?} }}", self.adc4rst())
        }
    }
    #[doc = "RCC AHB4 peripheral clocks enable in Sleep and Stop modes register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb4smenr(pub u32);
    impl Ahb4smenr {
        #[doc = "PWR bus clock enable during Sleep and Stop modes Set and cleared by software. Can only be accessed secure when one or more features in the PWR is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn pwrsmen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PWR bus clock enable during Sleep and Stop modes Set and cleared by software. Can only be accessed secure when one or more features in the PWR is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_pwrsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ADC4 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC ADC4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[must_use]
        #[inline(always)]
        pub const fn adc4smen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ADC4 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC ADC4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn set_adc4smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Ahb4smenr {
        #[inline(always)]
        fn default() -> Ahb4smenr {
            Ahb4smenr(0)
        }
    }
    impl core::fmt::Debug for Ahb4smenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb4smenr")
                .field("pwrsmen", &self.pwrsmen())
                .field("adc4smen", &self.adc4smen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb4smenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb4smenr {{ pwrsmen: {=bool:?}, adc4smen: {=bool:?} }}",
                self.pwrsmen(),
                self.adc4smen()
            )
        }
    }
    #[doc = "RCC AHB5 peripheral clock enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb5enr(pub u32);
    impl Ahb5enr {
        #[doc = "2.4 GHz RADIO bus clock enable Set and cleared by software. Access can be secured by GTZC_TZSC RADIOSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Before accessing the 2.4 GHz RADIO sleep timers registers the RADIOCLKRDY bit must be checked. Note: When RADIOSMEN and STRADIOCLKON are both cleared, RADIOCLKRDY bit must be re-checked when exiting low-power modes (Sleep and Stop)."]
        #[must_use]
        #[inline(always)]
        pub const fn radioen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "2.4 GHz RADIO bus clock enable Set and cleared by software. Access can be secured by GTZC_TZSC RADIOSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Before accessing the 2.4 GHz RADIO sleep timers registers the RADIOCLKRDY bit must be checked. Note: When RADIOSMEN and STRADIOCLKON are both cleared, RADIOCLKRDY bit must be re-checked when exiting low-power modes (Sleep and Stop)."]
        #[inline(always)]
        pub const fn set_radioen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "PTACONV bus clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ptaconven(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "PTACONV bus clock enable"]
        #[inline(always)]
        pub const fn set_ptaconven(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
                .field("radioen", &self.radioen())
                .field("ptaconven", &self.ptaconven())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb5enr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb5enr {{ radioen: {=bool:?}, ptaconven: {=bool:?} }}",
                self.radioen(),
                self.ptaconven()
            )
        }
    }
    #[doc = "RCC AHB5 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb5rstr(pub u32);
    impl Ahb5rstr {
        #[doc = "2.4 GHz RADIO reset Set and cleared by software. Access can be secured by GTZC_TZSC RADIOSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn radiorst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "2.4 GHz RADIO reset Set and cleared by software. Access can be secured by GTZC_TZSC RADIOSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_radiorst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "PTACONV reset."]
        #[must_use]
        #[inline(always)]
        pub const fn ptaconvrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "PTACONV reset."]
        #[inline(always)]
        pub const fn set_ptaconvrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
                .field("radiorst", &self.radiorst())
                .field("ptaconvrst", &self.ptaconvrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb5rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb5rstr {{ radiorst: {=bool:?}, ptaconvrst: {=bool:?} }}",
                self.radiorst(),
                self.ptaconvrst()
            )
        }
    }
    #[doc = "RCC AHB5 peripheral clocks enable in Sleep and Stop modes register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb5smenr(pub u32);
    impl Ahb5smenr {
        #[doc = "2.4 GHz RADIO bus clock enable during Sleep and Stop modes when the 2.4 GHz RADIO is active. Set and cleared by software. Access can be secured by GTZC_TZSC RADIOSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn radiosmen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "2.4 GHz RADIO bus clock enable during Sleep and Stop modes when the 2.4 GHz RADIO is active. Set and cleared by software. Access can be secured by GTZC_TZSC RADIOSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_radiosmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "PTACONV bus clock enable during Sleep and Stop modes."]
        #[must_use]
        #[inline(always)]
        pub const fn ptaconvsmen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "PTACONV bus clock enable during Sleep and Stop modes."]
        #[inline(always)]
        pub const fn set_ptaconvsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Ahb5smenr {
        #[inline(always)]
        fn default() -> Ahb5smenr {
            Ahb5smenr(0)
        }
    }
    impl core::fmt::Debug for Ahb5smenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb5smenr")
                .field("radiosmen", &self.radiosmen())
                .field("ptaconvsmen", &self.ptaconvsmen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb5smenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb5smenr {{ radiosmen: {=bool:?}, ptaconvsmen: {=bool:?} }}",
                self.radiosmen(),
                self.ptaconvsmen()
            )
        }
    }
    #[doc = "RCC APB1 peripheral clock enable register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1enr1(pub u32);
    impl Apb1enr1 {
        #[doc = "TIM2 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC TIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn tim2en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC TIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_tim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC TIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn tim3en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC TIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_tim3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC TIM4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn tim4en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC TIM4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_tim4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "WWDG bus clock enable Set by software to enable the window watchdog bus clock. Reset by hardware system reset. This bit can also be set by hardware if the WWDG_SW option bit is reset. Access can be secured by GTZC_TZSC WWDGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn wwdgen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG bus clock enable Set by software to enable the window watchdog bus clock. Reset by hardware system reset. This bit can also be set by hardware if the WWDG_SW option bit is reset. Access can be secured by GTZC_TZSC WWDGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_wwdgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI2 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC SPI2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn spi2en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC SPI2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_spi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "USART2 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC USART2SEC When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.."]
        #[must_use]
        #[inline(always)]
        pub const fn usart2en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC USART2SEC When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.."]
        #[inline(always)]
        pub const fn set_usart2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC USART3SEC When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.."]
        #[must_use]
        #[inline(always)]
        pub const fn usart3en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC USART3SEC When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV.."]
        #[inline(always)]
        pub const fn set_usart3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "I2C1 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC I2C1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC I2C1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_i2c1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC I2C2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c2en(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC I2C2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_i2c2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
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
                .field("wwdgen", &self.wwdgen())
                .field("spi2en", &self.spi2en())
                .field("usart2en", &self.usart2en())
                .field("usart3en", &self.usart3en())
                .field("i2c1en", &self.i2c1en())
                .field("i2c2en", &self.i2c2en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1enr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb1enr1 {{ tim2en: {=bool:?}, tim3en: {=bool:?}, tim4en: {=bool:?}, wwdgen: {=bool:?}, spi2en: {=bool:?}, usart2en: {=bool:?}, usart3en: {=bool:?}, i2c1en: {=bool:?}, i2c2en: {=bool:?} }}" , self . tim2en () , self . tim3en () , self . tim4en () , self . wwdgen () , self . spi2en () , self . usart2en () , self . usart3en () , self . i2c1en () , self . i2c2en ())
        }
    }
    #[doc = "RCC APB1 peripheral clock enable register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1enr2(pub u32);
    impl Apb1enr2 {
        #[doc = "I2C4 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC I2C4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c4en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC I2C4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_i2c4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPTIM2 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC LPTIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim2en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC LPTIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_lptim2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1enr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb1enr2 {{ i2c4en: {=bool:?}, lptim2en: {=bool:?} }}",
                self.i2c4en(),
                self.lptim2en()
            )
        }
    }
    #[doc = "RCC APB1 peripheral reset register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1rstr1(pub u32);
    impl Apb1rstr1 {
        #[doc = "TIM2 reset Set and cleared by software. Access can be secured by GTZC_TZSC TIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn tim2rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 reset Set and cleared by software. Access can be secured by GTZC_TZSC TIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_tim2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 reset Set and cleared by software. Access can be secured by GTZC_TZSC TIM3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn tim3rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 reset Set and cleared by software. Access can be secured by GTZC_TZSC TIM3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_tim3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 reset Set and cleared by software. Access can be secured by GTZC_TZSC TIM4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn tim4rst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 reset Set and cleared by software. Access can be secured by GTZC_TZSC TIM4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_tim4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "SPI2 reset Set and cleared by software. Access can be secured by GTZC_TZSC SPI2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn spi2rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 reset Set and cleared by software. Access can be secured by GTZC_TZSC SPI2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_spi2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "USART2 reset Set and cleared by software. Access can be secured by GTZC_TZSC UART2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn usart2rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 reset Set and cleared by software. Access can be secured by GTZC_TZSC UART2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_usart2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 reset Set and cleared by software. Access can be secured by GTZC_TZSC UART3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn usart3rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 reset Set and cleared by software. Access can be secured by GTZC_TZSC UART3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_usart3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "I2C1 reset Set and cleared by software. Access can be secured by GTZC_TZSC I2C1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 reset Set and cleared by software. Access can be secured by GTZC_TZSC I2C1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_i2c1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 reset Set and cleared by software. Access can be secured by GTZC_TZSC I2C2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c2rst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 reset Set and cleared by software. Access can be secured by GTZC_TZSC I2C2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_i2c2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
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
                .field("spi2rst", &self.spi2rst())
                .field("usart2rst", &self.usart2rst())
                .field("usart3rst", &self.usart3rst())
                .field("i2c1rst", &self.i2c1rst())
                .field("i2c2rst", &self.i2c2rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1rstr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb1rstr1 {{ tim2rst: {=bool:?}, tim3rst: {=bool:?}, tim4rst: {=bool:?}, spi2rst: {=bool:?}, usart2rst: {=bool:?}, usart3rst: {=bool:?}, i2c1rst: {=bool:?}, i2c2rst: {=bool:?} }}" , self . tim2rst () , self . tim3rst () , self . tim4rst () , self . spi2rst () , self . usart2rst () , self . usart3rst () , self . i2c1rst () , self . i2c2rst ())
        }
    }
    #[doc = "RCC APB1 peripheral reset register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1rstr2(pub u32);
    impl Apb1rstr2 {
        #[doc = "I2C4 reset."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c4rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 reset."]
        #[inline(always)]
        pub const fn set_i2c4rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPTIM2 reset Set and cleared by software. Access can be secured by GTZC_TZSC LPTIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim2rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 reset Set and cleared by software. Access can be secured by GTZC_TZSC LPTIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_lptim2rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1rstr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb1rstr2 {{ i2c4rst: {=bool:?}, lptim2rst: {=bool:?} }}",
                self.i2c4rst(),
                self.lptim2rst()
            )
        }
    }
    #[doc = "RCC APB1 peripheral clocks enable in Sleep and Stop modes register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1smenr1(pub u32);
    impl Apb1smenr1 {
        #[doc = "TIM2 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC TIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn tim2smen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC TIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_tim2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC TIM3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn tim3smen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC TIM3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_tim3smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM4 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC TIM4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn tim4smen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC TIM4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_tim4smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Window watchdog bus clock enable during Sleep and Stop modes Set and cleared by software. This bit is forced to 1 by hardware when the hardware WWDG option is activated. Access can be secured by GTZC_TZSC WWDGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn wwdgsmen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog bus clock enable during Sleep and Stop modes Set and cleared by software. This bit is forced to 1 by hardware when the hardware WWDG option is activated. Access can be secured by GTZC_TZSC WWDGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_wwdgsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI2 bus and kernel clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC SPI2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn spi2smen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SPI2 bus and kernel clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC SPI2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_spi2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "USART2 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC USART2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[must_use]
        #[inline(always)]
        pub const fn usart2smen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "USART2 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC USART2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn set_usart2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "USART3 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC USART3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[must_use]
        #[inline(always)]
        pub const fn usart3smen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "USART3 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC USART3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn set_usart3smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "I2C1 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC I2C1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1smen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC I2C1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn set_i2c1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC I2C2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c2smen(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC I2C2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn set_i2c2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
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
                .field("wwdgsmen", &self.wwdgsmen())
                .field("spi2smen", &self.spi2smen())
                .field("usart2smen", &self.usart2smen())
                .field("usart3smen", &self.usart3smen())
                .field("i2c1smen", &self.i2c1smen())
                .field("i2c2smen", &self.i2c2smen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1smenr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb1smenr1 {{ tim2smen: {=bool:?}, tim3smen: {=bool:?}, tim4smen: {=bool:?}, wwdgsmen: {=bool:?}, spi2smen: {=bool:?}, usart2smen: {=bool:?}, usart3smen: {=bool:?}, i2c1smen: {=bool:?}, i2c2smen: {=bool:?} }}" , self . tim2smen () , self . tim3smen () , self . tim4smen () , self . wwdgsmen () , self . spi2smen () , self . usart2smen () , self . usart3smen () , self . i2c1smen () , self . i2c2smen ())
        }
    }
    #[doc = "RCC APB1 peripheral clocks enable in Sleep and Stop modes register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1smenr2(pub u32);
    impl Apb1smenr2 {
        #[doc = "I2C4 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC I2C4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c4smen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC I2C4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn set_i2c4smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPTIM2 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC LPTIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim2smen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC LPTIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn set_lptim2smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1smenr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb1smenr2 {{ i2c4smen: {=bool:?}, lptim2smen: {=bool:?} }}",
                self.i2c4smen(),
                self.lptim2smen()
            )
        }
    }
    #[doc = "RCC APB2 peripheral clock enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2enr(pub u32);
    impl Apb2enr {
        #[doc = "TIM1 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC TIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn tim1en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC TIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_tim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI1 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC SPI1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn spi1en(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC SPI1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_spi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "USART1bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC USART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn usart1en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC USART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_usart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TIM16 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC TIM16SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn tim16en(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC TIM16SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_tim16en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC TIM17SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn tim17en(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC TIM17SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_tim17en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SAI1 bus and kernel clocks enable"]
        #[must_use]
        #[inline(always)]
        pub const fn sai1en(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 bus and kernel clocks enable"]
        #[inline(always)]
        pub const fn set_sai1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
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
                .field("tim16en", &self.tim16en())
                .field("tim17en", &self.tim17en())
                .field("sai1en", &self.sai1en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2enr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb2enr {{ tim1en: {=bool:?}, spi1en: {=bool:?}, usart1en: {=bool:?}, tim16en: {=bool:?}, tim17en: {=bool:?}, sai1en: {=bool:?} }}" , self . tim1en () , self . spi1en () , self . usart1en () , self . tim16en () , self . tim17en () , self . sai1en ())
        }
    }
    #[doc = "RCC APB2 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2rstr(pub u32);
    impl Apb2rstr {
        #[doc = "TIM1 reset Set and cleared by software. Access can be secured by GTZC_TZSC TIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn tim1rst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 reset Set and cleared by software. Access can be secured by GTZC_TZSC TIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_tim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI1 reset Set and cleared by software. Access can be secured by GTZC_TZSC SPI1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn spi1rst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 reset Set and cleared by software. Access can be secured by GTZC_TZSC SPI1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_spi1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "USART1 reset Set and cleared by software. Access can be secured by GTZC_TZSC USART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn usart1rst(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 reset Set and cleared by software. Access can be secured by GTZC_TZSC USART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_usart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TIM16 reset Set and cleared by software. Access can be secured by GTZC_TZSC TIM16SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn tim16rst(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 reset Set and cleared by software. Access can be secured by GTZC_TZSC TIM16SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_tim16rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 reset Set and cleared by software. Access can be secured by GTZC_TZSC TIM17SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn tim17rst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 reset Set and cleared by software. Access can be secured by GTZC_TZSC TIM17SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_tim17rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SAI1 reset."]
        #[must_use]
        #[inline(always)]
        pub const fn sai1rst(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 reset."]
        #[inline(always)]
        pub const fn set_sai1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
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
                .field("tim16rst", &self.tim16rst())
                .field("tim17rst", &self.tim17rst())
                .field("sai1rst", &self.sai1rst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb2rstr {{ tim1rst: {=bool:?}, spi1rst: {=bool:?}, usart1rst: {=bool:?}, tim16rst: {=bool:?}, tim17rst: {=bool:?}, sai1rst: {=bool:?} }}" , self . tim1rst () , self . spi1rst () , self . usart1rst () , self . tim16rst () , self . tim17rst () , self . sai1rst ())
        }
    }
    #[doc = "RCC APB2 peripheral clocks enable in Sleep and Stop modes register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2smenr(pub u32);
    impl Apb2smenr {
        #[doc = "TIM1 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC TIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn tim1smen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC TIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_tim1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SPI1 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC SPI1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[must_use]
        #[inline(always)]
        pub const fn spi1smen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SPI1 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC SPI1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn set_spi1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "USART1 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC USART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[must_use]
        #[inline(always)]
        pub const fn usart1smen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "USART1 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC USART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn set_usart1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "TIM16 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC TIM16SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn tim16smen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC TIM16SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_tim16smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC TIM17SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn tim17smen(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC TIM17SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_tim17smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SAI1 bus and kernel clocks enable during Sleep and Stop modes."]
        #[must_use]
        #[inline(always)]
        pub const fn sai1smen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SAI1 bus and kernel clocks enable during Sleep and Stop modes."]
        #[inline(always)]
        pub const fn set_sai1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
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
                .field("usart1smen", &self.usart1smen())
                .field("tim16smen", &self.tim16smen())
                .field("tim17smen", &self.tim17smen())
                .field("sai1smen", &self.sai1smen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2smenr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb2smenr {{ tim1smen: {=bool:?}, spi1smen: {=bool:?}, usart1smen: {=bool:?}, tim16smen: {=bool:?}, tim17smen: {=bool:?}, sai1smen: {=bool:?} }}" , self . tim1smen () , self . spi1smen () , self . usart1smen () , self . tim16smen () , self . tim17smen () , self . sai1smen ())
        }
    }
    #[doc = "RCC APB7 peripheral clock enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb7enr(pub u32);
    impl Apb7enr {
        #[doc = "SYSCFG bus clock enable Set and cleared by software. Access can be secured by SYSCFG SYSCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn syscfgen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG bus clock enable Set and cleared by software. Access can be secured by SYSCFG SYSCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_syscfgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SPI3 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC SPI3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn spi3en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC SPI3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_spi3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LPUART1 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC LPUART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn lpuart1en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC LPUART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_lpuart1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I2C3 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC I2C3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c3en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC I2C3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_i2c3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LPTIM1 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC LPTIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim1en(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 bus and kernel clocks enable Set and cleared by software. Access can be secured by GTZC_TZSC LPTIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_lptim1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "COMP bus clock enable"]
        #[must_use]
        #[inline(always)]
        pub const fn compen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "COMP bus clock enable"]
        #[inline(always)]
        pub const fn set_compen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "VREFBUF bus clock enable Set and cleared by software. Access can be secured by GTZC_TZSC VREFBUFSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn vrefen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "VREFBUF bus clock enable Set and cleared by software. Access can be secured by GTZC_TZSC VREFBUFSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_vrefen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "RTC and TAMP bus clock enable Set and cleared by software. Can only be accessed secure when one or more features in the RTC or TAMP is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn rtcapben(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "RTC and TAMP bus clock enable Set and cleared by software. Can only be accessed secure when one or more features in the RTC or TAMP is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_rtcapben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Apb7enr {
        #[inline(always)]
        fn default() -> Apb7enr {
            Apb7enr(0)
        }
    }
    impl core::fmt::Debug for Apb7enr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb7enr")
                .field("syscfgen", &self.syscfgen())
                .field("spi3en", &self.spi3en())
                .field("lpuart1en", &self.lpuart1en())
                .field("i2c3en", &self.i2c3en())
                .field("lptim1en", &self.lptim1en())
                .field("compen", &self.compen())
                .field("vrefen", &self.vrefen())
                .field("rtcapben", &self.rtcapben())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb7enr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb7enr {{ syscfgen: {=bool:?}, spi3en: {=bool:?}, lpuart1en: {=bool:?}, i2c3en: {=bool:?}, lptim1en: {=bool:?}, compen: {=bool:?}, vrefen: {=bool:?}, rtcapben: {=bool:?} }}" , self . syscfgen () , self . spi3en () , self . lpuart1en () , self . i2c3en () , self . lptim1en () , self . compen () , self . vrefen () , self . rtcapben ())
        }
    }
    #[doc = "RCC APB7 peripheral reset register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb7rstr(pub u32);
    impl Apb7rstr {
        #[doc = "SYSCFG reset Set and cleared by software. Access can be secured by SYSCFG SYSCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn syscfgrst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG reset Set and cleared by software. Access can be secured by SYSCFG SYSCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_syscfgrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SPI3 reset Set and cleared by software. Access can be secured by GTZC_TZSC SPI3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn spi3rst(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 reset Set and cleared by software. Access can be secured by GTZC_TZSC SPI3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_spi3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LPUART1 reset Set and cleared by software. Access can be secured by GTZC_TZSC LPUART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn lpuart1rst(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 reset Set and cleared by software. Access can be secured by GTZC_TZSC LPUART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_lpuart1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I2C3 reset Set and cleared by software. Access can be secured by GTZC_TZSC I2C3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c3rst(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 reset Set and cleared by software. Access can be secured by GTZC_TZSC I2C3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_i2c3rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LPTIM1 reset Set and cleared by software. Access can be secured by GTZC_TZSC LPTIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim1rst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 reset Set and cleared by software. Access can be secured by GTZC_TZSC LPTIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_lptim1rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "COMP reset Set and cleared by software. Access can be secured by GTZC_TZSC COMPSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn comprst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "COMP reset Set and cleared by software. Access can be secured by GTZC_TZSC COMPSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_comprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "VREF reset Set and cleared by software. Access can be secured by GTZC_TZSC VREFSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn vrefrst(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "VREF reset Set and cleared by software. Access can be secured by GTZC_TZSC VREFSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_vrefrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Apb7rstr {
        #[inline(always)]
        fn default() -> Apb7rstr {
            Apb7rstr(0)
        }
    }
    impl core::fmt::Debug for Apb7rstr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb7rstr")
                .field("syscfgrst", &self.syscfgrst())
                .field("spi3rst", &self.spi3rst())
                .field("lpuart1rst", &self.lpuart1rst())
                .field("i2c3rst", &self.i2c3rst())
                .field("lptim1rst", &self.lptim1rst())
                .field("comprst", &self.comprst())
                .field("vrefrst", &self.vrefrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb7rstr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb7rstr {{ syscfgrst: {=bool:?}, spi3rst: {=bool:?}, lpuart1rst: {=bool:?}, i2c3rst: {=bool:?}, lptim1rst: {=bool:?}, comprst: {=bool:?}, vrefrst: {=bool:?} }}" , self . syscfgrst () , self . spi3rst () , self . lpuart1rst () , self . i2c3rst () , self . lptim1rst () , self . comprst () , self . vrefrst ())
        }
    }
    #[doc = "RCC APB7 peripheral clock enable in Sleep and Stop modes register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb7smenr(pub u32);
    impl Apb7smenr {
        #[doc = "SYSCFG bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by SYSCFG SYSCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn syscfgsmen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG bus clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by SYSCFG SYSCFGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_syscfgsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SPI3 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC SPI3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[must_use]
        #[inline(always)]
        pub const fn spi3smen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI3 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC SPI3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn set_spi3smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LPUART1 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC LPUART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[must_use]
        #[inline(always)]
        pub const fn lpuart1smen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LPUART1 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC LPUART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn set_lpuart1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "I2C3 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC I2C3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c3smen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC I2C3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn set_i2c3smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LPTIM1 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC LPTIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim1smen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 bus and kernel clocks enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC LPTIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn set_lptim1smen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "COMP bus clock enable during Sleep and Stop modes."]
        #[must_use]
        #[inline(always)]
        pub const fn compsmen(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "COMP bus clock enable during Sleep and Stop modes."]
        #[inline(always)]
        pub const fn set_compsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "VREFBUF clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC VREFBUFSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[must_use]
        #[inline(always)]
        pub const fn vrefsmen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "VREFBUF clock enable during Sleep and Stop modes Set and cleared by software. Access can be secured by GTZC_TZSC VREFBUFSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn set_vrefsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "RTC and TAMP APB clock enable during Sleep and Stop modes Set and cleared by software. Can only be accessed secure when one or more features in the RTC or TAMP is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[must_use]
        #[inline(always)]
        pub const fn rtcapbsmen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "RTC and TAMP APB clock enable during Sleep and Stop modes Set and cleared by software. Can only be accessed secure when one or more features in the RTC or TAMP is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: This bit must be set to allow the peripheral to wake up from Stop modes."]
        #[inline(always)]
        pub const fn set_rtcapbsmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Apb7smenr {
        #[inline(always)]
        fn default() -> Apb7smenr {
            Apb7smenr(0)
        }
    }
    impl core::fmt::Debug for Apb7smenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb7smenr")
                .field("syscfgsmen", &self.syscfgsmen())
                .field("spi3smen", &self.spi3smen())
                .field("lpuart1smen", &self.lpuart1smen())
                .field("i2c3smen", &self.i2c3smen())
                .field("lptim1smen", &self.lptim1smen())
                .field("compsmen", &self.compsmen())
                .field("vrefsmen", &self.vrefsmen())
                .field("rtcapbsmen", &self.rtcapbsmen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb7smenr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Apb7smenr {{ syscfgsmen: {=bool:?}, spi3smen: {=bool:?}, lpuart1smen: {=bool:?}, i2c3smen: {=bool:?}, lptim1smen: {=bool:?}, compsmen: {=bool:?}, vrefsmen: {=bool:?}, rtcapbsmen: {=bool:?} }}" , self . syscfgsmen () , self . spi3smen () , self . lpuart1smen () , self . i2c3smen () , self . lptim1smen () , self . compsmen () , self . vrefsmen () , self . rtcapbsmen ())
        }
    }
    #[doc = "RCC audio synchronization auto-reload register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Asarr(pub u32);
    impl Asarr {
        #[doc = "Auto-reload value This field is set by software. CA\\[19:0\\]
is the counter auto-reload value at which to restart the audio synchronization counter from value 0. It defines the counter period."]
        #[must_use]
        #[inline(always)]
        pub const fn ar(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Auto-reload value This field is set by software. CA\\[19:0\\]
is the counter auto-reload value at which to restart the audio synchronization counter from value 0. It defines the counter period."]
        #[inline(always)]
        pub const fn set_ar(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for Asarr {
        #[inline(always)]
        fn default() -> Asarr {
            Asarr(0)
        }
    }
    impl core::fmt::Debug for Asarr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Asarr").field("ar", &self.ar()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Asarr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Asarr {{ ar: {=u32:?} }}", self.ar())
        }
    }
    #[doc = "RCC audio synchronization capture register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ascar(pub u32);
    impl Ascar {
        #[doc = "Capture value This field is set by hardware. CA\\[26:20\\]
is the capture period counter value loaded on the trigger event. CA\\[19:0\\]
is the audio synchronization counter value loaded on the trigger event."]
        #[must_use]
        #[inline(always)]
        pub const fn ca(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x07ff_ffff;
            val as u32
        }
        #[doc = "Capture value This field is set by hardware. CA\\[26:20\\]
is the capture period counter value loaded on the trigger event. CA\\[19:0\\]
is the audio synchronization counter value loaded on the trigger event."]
        #[inline(always)]
        pub const fn set_ca(&mut self, val: u32) {
            self.0 = (self.0 & !(0x07ff_ffff << 0usize)) | (((val as u32) & 0x07ff_ffff) << 0usize);
        }
    }
    impl Default for Ascar {
        #[inline(always)]
        fn default() -> Ascar {
            Ascar(0)
        }
    }
    impl core::fmt::Debug for Ascar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ascar").field("ca", &self.ca()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ascar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ascar {{ ca: {=u32:?} }}", self.ca())
        }
    }
    #[doc = "RCC audio synchronization counter register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ascntr(pub u32);
    impl Ascntr {
        #[doc = "Counter value This field is set by hardware. CNT\\[19:0\\]
is the counter value at the time this register is read."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Counter value This field is set by hardware. CNT\\[19:0\\]
is the counter value at the time this register is read."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for Ascntr {
        #[inline(always)]
        fn default() -> Ascntr {
            Ascntr(0)
        }
    }
    impl core::fmt::Debug for Ascntr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ascntr").field("cnt", &self.cnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ascntr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ascntr {{ cnt: {=u32:?} }}", self.cnt())
        }
    }
    #[doc = "RCC audio synchronization compare register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ascor(pub u32);
    impl Ascor {
        #[doc = "Compare value This field is set by software. CO\\[19:0\\]
is the value to be compared to the audio synchronization counter to generate an compare interrupt event."]
        #[must_use]
        #[inline(always)]
        pub const fn co(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Compare value This field is set by software. CO\\[19:0\\]
is the value to be compared to the audio synchronization counter to generate an compare interrupt event."]
        #[inline(always)]
        pub const fn set_co(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for Ascor {
        #[inline(always)]
        fn default() -> Ascor {
            Ascor(0)
        }
    }
    impl core::fmt::Debug for Ascor {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ascor").field("co", &self.co()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ascor {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ascor {{ co: {=u32:?} }}", self.co())
        }
    }
    #[doc = "RCC audio synchronization control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ascr(pub u32);
    impl Ascr {
        #[doc = "Counter enable This bit is set and cleared by software. Clearing this bit will reset the audio synchronization counter and capture prescaler and all associated registers ASCR, ASIER, ASSR, ASCNTR, ASARR, ASCAR, and ASCOR."]
        #[must_use]
        #[inline(always)]
        pub const fn cen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Counter enable This bit is set and cleared by software. Clearing this bit will reset the audio synchronization counter and capture prescaler and all associated registers ASCR, ASIER, ASSR, ASCNTR, ASARR, ASCAR, and ASCOR."]
        #[inline(always)]
        pub const fn set_cen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clock prescaler This field is set and cleared by software. Counter clock frequency = f_audiosync_ker_ck / (PSC + 1)"]
        #[must_use]
        #[inline(always)]
        pub const fn psc(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "Clock prescaler This field is set and cleared by software. Counter clock frequency = f_audiosync_ker_ck / (PSC + 1)"]
        #[inline(always)]
        pub const fn set_psc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "Capture prescaler This field is set and cleared by software. Capture period in number of counter periods. Capture period = counter period * (TPS + 1)"]
        #[must_use]
        #[inline(always)]
        pub const fn cps(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Capture prescaler This field is set and cleared by software. Capture period in number of counter periods. Capture period = counter period * (TPS + 1)"]
        #[inline(always)]
        pub const fn set_cps(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for Ascr {
        #[inline(always)]
        fn default() -> Ascr {
            Ascr(0)
        }
    }
    impl core::fmt::Debug for Ascr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ascr")
                .field("cen", &self.cen())
                .field("psc", &self.psc())
                .field("cps", &self.cps())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ascr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ascr {{ cen: {=bool:?}, psc: {=u8:?}, cps: {=u8:?} }}",
                self.cen(),
                self.psc(),
                self.cps()
            )
        }
    }
    #[doc = "RCC audio synchronization interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Asier(pub u32);
    impl Asier {
        #[doc = "Capture trigger interrupt enable This bit is set and cleared by software."]
        #[must_use]
        #[inline(always)]
        pub const fn caie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Capture trigger interrupt enable This bit is set and cleared by software."]
        #[inline(always)]
        pub const fn set_caie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Comparer interrupt enable This field is set and cleared by software."]
        #[must_use]
        #[inline(always)]
        pub const fn coie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Comparer interrupt enable This field is set and cleared by software."]
        #[inline(always)]
        pub const fn set_coie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Capture error interrupt enable This field is set and cleared by software."]
        #[must_use]
        #[inline(always)]
        pub const fn caeie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Capture error interrupt enable This field is set and cleared by software."]
        #[inline(always)]
        pub const fn set_caeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Asier {
        #[inline(always)]
        fn default() -> Asier {
            Asier(0)
        }
    }
    impl core::fmt::Debug for Asier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Asier")
                .field("caie", &self.caie())
                .field("coie", &self.coie())
                .field("caeie", &self.caeie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Asier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Asier {{ caie: {=bool:?}, coie: {=bool:?}, caeie: {=bool:?} }}",
                self.caie(),
                self.coie(),
                self.caeie()
            )
        }
    }
    #[doc = "RCC audio synchronization status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Assr(pub u32);
    impl Assr {
        #[doc = "Capture trigger interrupt flag This field is set by hardware, only when CAIE is enabled. This bit is cleared by software by writing it to 0 or masked when CAIE is 0."]
        #[must_use]
        #[inline(always)]
        pub const fn caf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Capture trigger interrupt flag This field is set by hardware, only when CAIE is enabled. This bit is cleared by software by writing it to 0 or masked when CAIE is 0."]
        #[inline(always)]
        pub const fn set_caf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Comparer interrupt flag This field is set by hardware, only when COIE is enabled. This bit is cleared by software by writing it to 0 or masked when COIE is 0."]
        #[must_use]
        #[inline(always)]
        pub const fn cof(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Comparer interrupt flag This field is set by hardware, only when COIE is enabled. This bit is cleared by software by writing it to 0 or masked when COIE is 0."]
        #[inline(always)]
        pub const fn set_cof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Capture error interrupt flag This field is set by hardware, only when CAEIE is enabled. This bit is cleared by software by writing it to 0 or masked when CAEIE is 0."]
        #[must_use]
        #[inline(always)]
        pub const fn caef(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Capture error interrupt flag This field is set by hardware, only when CAEIE is enabled. This bit is cleared by software by writing it to 0 or masked when CAEIE is 0."]
        #[inline(always)]
        pub const fn set_caef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Assr {
        #[inline(always)]
        fn default() -> Assr {
            Assr(0)
        }
    }
    impl core::fmt::Debug for Assr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Assr")
                .field("caf", &self.caf())
                .field("cof", &self.cof())
                .field("caef", &self.caef())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Assr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Assr {{ caf: {=bool:?}, cof: {=bool:?}, caef: {=bool:?} }}",
                self.caf(),
                self.cof(),
                self.caef()
            )
        }
    }
    #[doc = "RCC backup domain control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdcr(pub u32);
    impl Bdcr {
        #[doc = "LSE oscillator enable Set and cleared by software. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn lseon(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSE oscillator enable Set and cleared by software. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_lseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE oscillator ready Set and cleared by hardware to indicate when the external 32�kHz oscillator is stable. After the LSEON bit is cleared, LSERDY goes low after six external low-speed oscillator clock cycles. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn lserdy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE oscillator ready Set and cleared by hardware to indicate when the external 32�kHz oscillator is stable. After the LSEON bit is cleared, LSERDY goes low after six external low-speed oscillator clock cycles. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_lserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LSE oscillator bypass Set and cleared by software to bypass oscillator in debug mode. This bit can be written only when the external 32�kHz oscillator is disabled (LSEON = 0 and LSERDY = 0). Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn lsebyp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "LSE oscillator bypass Set and cleared by software to bypass oscillator in debug mode. This bit can be written only when the external 32�kHz oscillator is disabled (LSEON = 0 and LSERDY = 0). Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_lsebyp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "LSE oscillator drive capability Set by software to modulate the drive capability of the LSE oscillator. LSEDRV must be programmed to a different value than 0 before enabling the LSE oscillator in ‘Xtal’ mode. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The oscillator is in ‘Xtal mode’ when it is not in bypass mode."]
        #[must_use]
        #[inline(always)]
        pub const fn lsedrv(&self) -> super::vals::Lsedrv {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Lsedrv::from_bits(val as u8)
        }
        #[doc = "LSE oscillator drive capability Set by software to modulate the drive capability of the LSE oscillator. LSEDRV must be programmed to a different value than 0 before enabling the LSE oscillator in ‘Xtal’ mode. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The oscillator is in ‘Xtal mode’ when it is not in bypass mode."]
        #[inline(always)]
        pub const fn set_lsedrv(&mut self, val: super::vals::Lsedrv) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "Low speed external clock security enable Set by software to enable the LSECSS. LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware) and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD�=�1). In that case, the software must disable the LSECSSON bit. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn lsecsson(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Low speed external clock security enable Set by software to enable the LSECSS. LSECSSON must be enabled after the LSE oscillator is enabled (LSEON bit enabled) and ready (LSERDY flag set by hardware) and after the RTCSEL bit is selected. Once enabled, this bit cannot be disabled, except after a LSE failure detection (LSECSSD�=�1). In that case, the software must disable the LSECSSON bit. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_lsecsson(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Low speed external clock security, LSE failure Detection Set by hardware to indicate when a failure is detected by the LSECCS on the external 32�kHz oscillator. Reset when LSCSSON bit is cleared. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn lsecssd(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Low speed external clock security, LSE failure Detection Set by hardware to indicate when a failure is detected by the LSECCS on the external 32�kHz oscillator. Reset when LSCSSON bit is cleared. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_lsecssd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LSE system clock (LSESYS) enable Set by software to enable the LSE system clock generated by RCC. The lsesys clock is used for peripherals (USART, LPUART, LPTIM, RNG, 2.4 GHz RADIO) and functions (LSCO, MCO, TIM triggers, LPTIM trigger) excluding the RTC, TAMP and LSECSS. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn lsesysen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LSE system clock (LSESYS) enable Set by software to enable the LSE system clock generated by RCC. The lsesys clock is used for peripherals (USART, LPUART, LPTIM, RNG, 2.4 GHz RADIO) and functions (LSCO, MCO, TIM triggers, LPTIM trigger) excluding the RTC, TAMP and LSECSS. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_lsesysen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "RTC and TAMP kernel clock source enable and selection Set by software to enable and select the clock source for the RTC. Can only be accessed secure when one or more features in the RTC or TAMP is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn rtcsel(&self) -> super::vals::Rtcsel {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Rtcsel::from_bits(val as u8)
        }
        #[doc = "RTC and TAMP kernel clock source enable and selection Set by software to enable and select the clock source for the RTC. Can only be accessed secure when one or more features in the RTC or TAMP is/are secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_rtcsel(&mut self, val: super::vals::Rtcsel) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "LSE system clock (LSESYS) ready Set and cleared by hardware to indicate when the LSE system clock is stable.When the LSESYSEN bit is set, the LSESYSRDY flag is set after two LSE clock cycles. The LSE clock must be already enabled and stable (LSEON and LSERDY are set). When the LSEON bit is cleared, LSERDY goes low after six external low-speed oscillator clock cycles. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn lsesysrdy(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LSE system clock (LSESYS) ready Set and cleared by hardware to indicate when the LSE system clock is stable.When the LSESYSEN bit is set, the LSESYSRDY flag is set after two LSE clock cycles. The LSE clock must be already enabled and stable (LSEON and LSERDY are set). When the LSEON bit is cleared, LSERDY goes low after six external low-speed oscillator clock cycles. Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_lsesysrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LSE clock glitch filter enable Set and cleared by hardware to enable the LSE glitch filter. This bit can be written only when the LSE is disabled (LSEON = 0 and LSERDY = 0). Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn lsegfon(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock glitch filter enable Set and cleared by hardware to enable the LSE glitch filter. This bit can be written only when the LSE is disabled (LSEON = 0 and LSERDY = 0). Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_lsegfon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "LSE trimming These bits are initialized at startup and after OBL_LAUNCH with SBF cleared with the factory-programmed LSE calibration value. Set and cleared by software. These bits must be modified only once after a BOR reset or an OBL_LAUNCH and before enabling LSE with LSEON (when both LSEON = 0 and LSERDY�= 0). Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: OBL_LAUNCH of this field occurs only when SBF is cleared and must then only be started by software when LSE oscillator is disabled, LSEON = 0 and LSERDY = 0."]
        #[must_use]
        #[inline(always)]
        pub const fn lsetrim(&self) -> super::vals::Lsetrim {
            let val = (self.0 >> 13usize) & 0x03;
            super::vals::Lsetrim::from_bits(val as u8)
        }
        #[doc = "LSE trimming These bits are initialized at startup and after OBL_LAUNCH with SBF cleared with the factory-programmed LSE calibration value. Set and cleared by software. These bits must be modified only once after a BOR reset or an OBL_LAUNCH and before enabling LSE with LSEON (when both LSEON = 0 and LSERDY�= 0). Access can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: OBL_LAUNCH of this field occurs only when SBF is cleared and must then only be started by software when LSE oscillator is disabled, LSEON = 0 and LSERDY = 0."]
        #[inline(always)]
        pub const fn set_lsetrim(&mut self, val: super::vals::Lsetrim) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
        }
        #[doc = "Backup domain software reset Set and cleared by software. Can only be accessed secure when one or more features in the RTC or TAMP is secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn bdrst(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Backup domain software reset Set and cleared by software. Can only be accessed secure when one or more features in the RTC or TAMP is secure. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_bdrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "2.4 GHz RADIO sleep timer kernel clock enable and selection Set and cleared by software. Access can be secured by GTZC_TZSC RADIOSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn radiostsel(&self) -> super::vals::Radiostsel {
            let val = (self.0 >> 18usize) & 0x03;
            super::vals::Radiostsel::from_bits(val as u8)
        }
        #[doc = "2.4 GHz RADIO sleep timer kernel clock enable and selection Set and cleared by software. Access can be secured by GTZC_TZSC RADIOSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_radiostsel(&mut self, val: super::vals::Radiostsel) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
        }
        #[doc = "Low-speed clock output (LSCO) enable Set and cleared by software. Access can be secured by RCC LSISEC and/or RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn lscoen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Low-speed clock output (LSCO) enable Set and cleared by software. Access can be secured by RCC LSISEC and/or RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_lscoen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Low-speed clock output selection Set and cleared by software. Access can be secured by RCC LSISEC and/or RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn lscosel(&self) -> super::vals::Lscosel {
            let val = (self.0 >> 25usize) & 0x01;
            super::vals::Lscosel::from_bits(val as u8)
        }
        #[doc = "Low-speed clock output selection Set and cleared by software. Access can be secured by RCC LSISEC and/or RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_lscosel(&mut self, val: super::vals::Lscosel) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
        }
        #[doc = "LSI1 oscillator enable Set and cleared by software. Access can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn lsi1on(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "LSI1 oscillator enable Set and cleared by software. Access can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_lsi1on(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "LSI1 oscillator ready Set and cleared by hardware to indicate when the LSI1 oscillator is stable. After the LSI1ON bit is cleared, LSI1RDY goes low after three internal low-speed oscillator clock cycles. This bit is set when the LSI1 is used by IWDG or RTC, even if LSI1ON = 0. Access can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn lsi1rdy(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "LSI1 oscillator ready Set and cleared by hardware to indicate when the LSI1 oscillator is stable. After the LSI1ON bit is cleared, LSI1RDY goes low after three internal low-speed oscillator clock cycles. This bit is set when the LSI1 is used by IWDG or RTC, even if LSI1ON = 0. Access can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_lsi1rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "LSI1 Low-speed clock divider configuration Set and cleared by software to enable the LSI1 division. This bit can be written only when the LSI1 is disabled (LSI1ON = 0 and LSI1RDY = 0). The LSI1PREDIV cannot be changed if the LSI1 is used by the IWDG or by the RTC. Access can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn lsi1prediv(&self) -> super::vals::Lsiprediv {
            let val = (self.0 >> 28usize) & 0x01;
            super::vals::Lsiprediv::from_bits(val as u8)
        }
        #[doc = "LSI1 Low-speed clock divider configuration Set and cleared by software to enable the LSI1 division. This bit can be written only when the LSI1 is disabled (LSI1ON = 0 and LSI1RDY = 0). The LSI1PREDIV cannot be changed if the LSI1 is used by the IWDG or by the RTC. Access can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_lsi1prediv(&mut self, val: super::vals::Lsiprediv) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
        }
        #[doc = "LSI2 oscillator enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lsi2on(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "LSI2 oscillator enable"]
        #[inline(always)]
        pub const fn set_lsi2on(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "LSI2 oscillator ready."]
        #[must_use]
        #[inline(always)]
        pub const fn lsi2rdy(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "LSI2 oscillator ready."]
        #[inline(always)]
        pub const fn set_lsi2rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
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
                .field("lsetrim", &self.lsetrim())
                .field("bdrst", &self.bdrst())
                .field("radiostsel", &self.radiostsel())
                .field("lscoen", &self.lscoen())
                .field("lscosel", &self.lscosel())
                .field("lsi1on", &self.lsi1on())
                .field("lsi1rdy", &self.lsi1rdy())
                .field("lsi1prediv", &self.lsi1prediv())
                .field("lsi2on", &self.lsi2on())
                .field("lsi2rdy", &self.lsi2rdy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bdcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Bdcr {{ lseon: {=bool:?}, lserdy: {=bool:?}, lsebyp: {=bool:?}, lsedrv: {:?}, lsecsson: {=bool:?}, lsecssd: {=bool:?}, lsesysen: {=bool:?}, rtcsel: {:?}, lsesysrdy: {=bool:?}, lsegfon: {=bool:?}, lsetrim: {:?}, bdrst: {=bool:?}, radiostsel: {:?}, lscoen: {=bool:?}, lscosel: {:?}, lsi1on: {=bool:?}, lsi1rdy: {=bool:?}, lsi1prediv: {:?}, lsi2on: {=bool:?}, lsi2rdy: {=bool:?} }}" , self . lseon () , self . lserdy () , self . lsebyp () , self . lsedrv () , self . lsecsson () , self . lsecssd () , self . lsesysen () , self . rtcsel () , self . lsesysrdy () , self . lsegfon () , self . lsetrim () , self . bdrst () , self . radiostsel () , self . lscoen () , self . lscosel () , self . lsi1on () , self . lsi1rdy () , self . lsi1prediv () , self . lsi2on () , self . lsi2rdy ())
        }
    }
    #[doc = "RCC Backup domain control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdcr2(pub u32);
    impl Bdcr2 {
        #[doc = "LSI2 oscillator operating mode configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn lsi2mode(&self) -> super::vals::Lsi2mode {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Lsi2mode::from_bits(val as u8)
        }
        #[doc = "LSI2 oscillator operating mode configuration."]
        #[inline(always)]
        pub const fn set_lsi2mode(&mut self, val: super::vals::Lsi2mode) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "LSI2 oscillator configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn lsi2cfg(&self) -> super::vals::Lsi2cfg {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::Lsi2cfg::from_bits(val as u8)
        }
        #[doc = "LSI2 oscillator configuration."]
        #[inline(always)]
        pub const fn set_lsi2cfg(&mut self, val: super::vals::Lsi2cfg) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Bdcr2 {
        #[inline(always)]
        fn default() -> Bdcr2 {
            Bdcr2(0)
        }
    }
    impl core::fmt::Debug for Bdcr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bdcr2")
                .field("lsi2mode", &self.lsi2mode())
                .field("lsi2cfg", &self.lsi2cfg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bdcr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bdcr2 {{ lsi2mode: {:?}, lsi2cfg: {:?} }}",
                self.lsi2mode(),
                self.lsi2cfg()
            )
        }
    }
    #[doc = "RCC peripherals independent clock configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccipr1(pub u32);
    impl Ccipr1 {
        #[doc = "USART1 kernel clock source selection This bits are used to select the USART1 kernel clock source. Access can be secured by GTZC_TZSC USART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The USART1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or LSE."]
        #[must_use]
        #[inline(always)]
        pub const fn usart1sel(&self) -> super::vals::Usart1sel {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Usart1sel::from_bits(val as u8)
        }
        #[doc = "USART1 kernel clock source selection This bits are used to select the USART1 kernel clock source. Access can be secured by GTZC_TZSC USART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The USART1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or LSE."]
        #[inline(always)]
        pub const fn set_usart1sel(&mut self, val: super::vals::Usart1sel) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "USART2 kernel clock source selection This bits are used to select the USART2 kernel clock source. Access can be secured by GTZC_TZSC USART2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The USART2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or LSE."]
        #[must_use]
        #[inline(always)]
        pub const fn usart2sel(&self) -> super::vals::Usartsel {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Usartsel::from_bits(val as u8)
        }
        #[doc = "USART2 kernel clock source selection This bits are used to select the USART2 kernel clock source. Access can be secured by GTZC_TZSC USART2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The USART2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or LSE."]
        #[inline(always)]
        pub const fn set_usart2sel(&mut self, val: super::vals::Usartsel) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "USART3 kernel clock source selection This bits are used to select the USART3 kernel clock source. Access can be secured by GTZC_TZSC USART2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The USART3 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or LSE."]
        #[must_use]
        #[inline(always)]
        pub const fn usart3sel(&self) -> super::vals::Usartsel {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Usartsel::from_bits(val as u8)
        }
        #[doc = "USART3 kernel clock source selection This bits are used to select the USART3 kernel clock source. Access can be secured by GTZC_TZSC USART2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The USART3 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI or LSE."]
        #[inline(always)]
        pub const fn set_usart3sel(&mut self, val: super::vals::Usartsel) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "I2C1 kernel clock source selection These bits are used to select the I2C1 kernel clock source. Access can be secured by GTZC_TZSC I2C1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The I2C1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1sel(&self) -> super::vals::I2c1sel {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::I2c1sel::from_bits(val as u8)
        }
        #[doc = "I2C1 kernel clock source selection These bits are used to select the I2C1 kernel clock source. Access can be secured by GTZC_TZSC I2C1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The I2C1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI."]
        #[inline(always)]
        pub const fn set_i2c1sel(&mut self, val: super::vals::I2c1sel) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
        #[doc = "I2C2 kernel clock source selection These bits are used to select the I2C2 kernel clock source. Access can be secured by GTZC_TZSC I2C2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The I2C2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c2sel(&self) -> super::vals::I2c1sel {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::I2c1sel::from_bits(val as u8)
        }
        #[doc = "I2C2 kernel clock source selection These bits are used to select the I2C2 kernel clock source. Access can be secured by GTZC_TZSC I2C2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The I2C2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI."]
        #[inline(always)]
        pub const fn set_i2c2sel(&mut self, val: super::vals::I2c1sel) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "I2C4 kernel clock source selection These bits are used to select the I2C4 kernel clock source. Access can be secured by GTZC_TZSC I2C4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The I2C4 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c4sel(&self) -> super::vals::I2c1sel {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::I2c1sel::from_bits(val as u8)
        }
        #[doc = "I2C4 kernel clock source selection These bits are used to select the I2C4 kernel clock source. Access can be secured by GTZC_TZSC I2C4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The I2C4 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI."]
        #[inline(always)]
        pub const fn set_i2c4sel(&mut self, val: super::vals::I2c1sel) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
        #[doc = "SPI2 kernel clock source selection These bits are used to select the SPI2 kernel clock source. Access can be secured by GTZC_TZSC SPI2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The SPI2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI."]
        #[must_use]
        #[inline(always)]
        pub const fn spi2sel(&self) -> super::vals::Spi2sel {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Spi2sel::from_bits(val as u8)
        }
        #[doc = "SPI2 kernel clock source selection These bits are used to select the SPI2 kernel clock source. Access can be secured by GTZC_TZSC SPI2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The SPI2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI."]
        #[inline(always)]
        pub const fn set_spi2sel(&mut self, val: super::vals::Spi2sel) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "Low-power timer 2 kernel clock source selection These bits are used to select the LPTIM2 kernel clock source. Access can be secured by GTZC_TZSC LPTIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The LPTIM2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is LSI, LSE or HSI if HSIKERON = 1."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim2sel(&self) -> super::vals::Lptim2sel {
            let val = (self.0 >> 18usize) & 0x03;
            super::vals::Lptim2sel::from_bits(val as u8)
        }
        #[doc = "Low-power timer 2 kernel clock source selection These bits are used to select the LPTIM2 kernel clock source. Access can be secured by GTZC_TZSC LPTIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The LPTIM2 is functional in Stop 0 and Stop 1 mode only when the kernel clock is LSI, LSE or HSI if HSIKERON = 1."]
        #[inline(always)]
        pub const fn set_lptim2sel(&mut self, val: super::vals::Lptim2sel) {
            self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
        }
        #[doc = "SPI1 kernel clock source selection These bits are used to select the SPI1 kernel clock source. Access can be secured by GTZC_TZSC SPI1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The SPI1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI."]
        #[must_use]
        #[inline(always)]
        pub const fn spi1sel(&self) -> super::vals::Spi1sel {
            let val = (self.0 >> 20usize) & 0x03;
            super::vals::Spi1sel::from_bits(val as u8)
        }
        #[doc = "SPI1 kernel clock source selection These bits are used to select the SPI1 kernel clock source. Access can be secured by GTZC_TZSC SPI1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The SPI1 is functional in Stop 0 and Stop 1 mode only when the kernel clock is HSI."]
        #[inline(always)]
        pub const fn set_spi1sel(&mut self, val: super::vals::Spi1sel) {
            self.0 = (self.0 & !(0x03 << 20usize)) | (((val.to_bits() as u32) & 0x03) << 20usize);
        }
        #[doc = "SysTick clock source selection These bits are used to select the SysTick clock source. Access can be secured by RCC SYSCLKSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: When LSE or LSI is selected, the AHB frequency must be at least four times higher than the LSI or LSE frequency. In addition, a jitter up to one hclk1 cycle is introduced, due to the LSE or LSI sampling with hclk1 in the SysTick circuitry."]
        #[must_use]
        #[inline(always)]
        pub const fn systicksel(&self) -> super::vals::Systicksel {
            let val = (self.0 >> 22usize) & 0x03;
            super::vals::Systicksel::from_bits(val as u8)
        }
        #[doc = "SysTick clock source selection These bits are used to select the SysTick clock source. Access can be secured by RCC SYSCLKSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: When LSE or LSI is selected, the AHB frequency must be at least four times higher than the LSI or LSE frequency. In addition, a jitter up to one hclk1 cycle is introduced, due to the LSE or LSI sampling with hclk1 in the SysTick circuitry."]
        #[inline(always)]
        pub const fn set_systicksel(&mut self, val: super::vals::Systicksel) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
        }
        #[doc = "Clocks sources for TIM16,TIM17 and LPTIM2 internal input capture When the TIMICSEL bit is set, the TIM16, TIM17 and LPTIM2 internal input capture can be connected to HSI/256. When TIMICSEL is cleared, the HSI, clock sources cannot be selected as TIM16, TIM17 or LPTIM2 internal input capture. Access can be secured by GTZC_TZSC TIM16SEC, TIM17SEC, or LPTIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The clock division must be disabled (TIMICSEL configured to 0) before selecting or changing a clock sources division."]
        #[must_use]
        #[inline(always)]
        pub const fn timicsel(&self) -> super::vals::Timicsel {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Timicsel::from_bits(val as u8)
        }
        #[doc = "Clocks sources for TIM16,TIM17 and LPTIM2 internal input capture When the TIMICSEL bit is set, the TIM16, TIM17 and LPTIM2 internal input capture can be connected to HSI/256. When TIMICSEL is cleared, the HSI, clock sources cannot be selected as TIM16, TIM17 or LPTIM2 internal input capture. Access can be secured by GTZC_TZSC TIM16SEC, TIM17SEC, or LPTIM2SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The clock division must be disabled (TIMICSEL configured to 0) before selecting or changing a clock sources division."]
        #[inline(always)]
        pub const fn set_timicsel(&mut self, val: super::vals::Timicsel) {
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
                .field("i2c1sel", &self.i2c1sel())
                .field("i2c2sel", &self.i2c2sel())
                .field("i2c4sel", &self.i2c4sel())
                .field("spi2sel", &self.spi2sel())
                .field("lptim2sel", &self.lptim2sel())
                .field("spi1sel", &self.spi1sel())
                .field("systicksel", &self.systicksel())
                .field("timicsel", &self.timicsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccipr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ccipr1 {{ usart1sel: {:?}, usart2sel: {:?}, usart3sel: {:?}, i2c1sel: {:?}, i2c2sel: {:?}, i2c4sel: {:?}, spi2sel: {:?}, lptim2sel: {:?}, spi1sel: {:?}, systicksel: {:?}, timicsel: {:?} }}" , self . usart1sel () , self . usart2sel () , self . usart3sel () , self . i2c1sel () , self . i2c2sel () , self . i2c4sel () , self . spi2sel () , self . lptim2sel () , self . spi1sel () , self . systicksel () , self . timicsel ())
        }
    }
    #[doc = "RCC peripherals independent clock configuration register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccipr2(pub u32);
    impl Ccipr2 {
        #[doc = "SAI1 kernel clock source selection These bits allow to select the SAI1 kernel clock source. Access can be secured by GTZC_TZSC SAI1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn sai1sel(&self) -> super::vals::Sai1sel {
            let val = (self.0 >> 5usize) & 0x07;
            super::vals::Sai1sel::from_bits(val as u8)
        }
        #[doc = "SAI1 kernel clock source selection These bits allow to select the SAI1 kernel clock source. Access can be secured by GTZC_TZSC SAI1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_sai1sel(&mut self, val: super::vals::Sai1sel) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
        }
        #[doc = "RNGSEL kernel clock source selection These bits allow to select the RNG kernel clock source. Access can be secured by GTZC_TZSC RNGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn rngsel(&self) -> super::vals::Rngsel {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Rngsel::from_bits(val as u8)
        }
        #[doc = "RNGSEL kernel clock source selection These bits allow to select the RNG kernel clock source. Access can be secured by GTZC_TZSC RNGSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_rngsel(&mut self, val: super::vals::Rngsel) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "USB OTG_HS PHY kernel clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn otghssel(&self) -> super::vals::Otghssel {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Otghssel::from_bits(val as u8)
        }
        #[doc = "USB OTG_HS PHY kernel clock source selection."]
        #[inline(always)]
        pub const fn set_otghssel(&mut self, val: super::vals::Otghssel) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "RCC audio synchronization kernel clock source selection This bit allows to select the audio synchronization kernel clock source. Access can be secured by GTZC_TZSC SAI1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn assel(&self) -> super::vals::Assel {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Assel::from_bits(val as u8)
        }
        #[doc = "RCC audio synchronization kernel clock source selection This bit allows to select the audio synchronization kernel clock source. Access can be secured by GTZC_TZSC SAI1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_assel(&mut self, val: super::vals::Assel) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
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
                .field("sai1sel", &self.sai1sel())
                .field("rngsel", &self.rngsel())
                .field("otghssel", &self.otghssel())
                .field("assel", &self.assel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccipr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccipr2 {{ sai1sel: {:?}, rngsel: {:?}, otghssel: {:?}, assel: {:?} }}",
                self.sai1sel(),
                self.rngsel(),
                self.otghssel(),
                self.assel()
            )
        }
    }
    #[doc = "RCC peripherals independent clock configuration register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccipr3(pub u32);
    impl Ccipr3 {
        #[doc = "LPUART1 kernel clock source selection These bits are used to select the LPUART1 kernel clock source. Access can be secured by GTZC_TZSC LPUART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The LPUART1 is functional in Stop modes only when the kernel clock is HSI or LSE."]
        #[must_use]
        #[inline(always)]
        pub const fn lpuart1sel(&self) -> super::vals::Lpuartsel {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Lpuartsel::from_bits(val as u8)
        }
        #[doc = "LPUART1 kernel clock source selection These bits are used to select the LPUART1 kernel clock source. Access can be secured by GTZC_TZSC LPUART1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The LPUART1 is functional in Stop modes only when the kernel clock is HSI or LSE."]
        #[inline(always)]
        pub const fn set_lpuart1sel(&mut self, val: super::vals::Lpuartsel) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "SPI3 kernel clock source selection These bits are used to select the SPI3 kernel clock source. Access can be secured by GTZC_TZSC SPI3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The SPI3 is functional in Stop modes only when the kernel clock is HSI."]
        #[must_use]
        #[inline(always)]
        pub const fn spi3sel(&self) -> super::vals::Spi3sel {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Spi3sel::from_bits(val as u8)
        }
        #[doc = "SPI3 kernel clock source selection These bits are used to select the SPI3 kernel clock source. Access can be secured by GTZC_TZSC SPI3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The SPI3 is functional in Stop modes only when the kernel clock is HSI."]
        #[inline(always)]
        pub const fn set_spi3sel(&mut self, val: super::vals::Spi3sel) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "I2C3 kernel clock source selection These bits are used to select the I2C3 kernel clock source. Access can be secured by GTZC_TZSC I2C3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The I2C3 is functional in Stop modes only when the kernel clock is HSI"]
        #[must_use]
        #[inline(always)]
        pub const fn i2c3sel(&self) -> super::vals::I2c3sel {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::I2c3sel::from_bits(val as u8)
        }
        #[doc = "I2C3 kernel clock source selection These bits are used to select the I2C3 kernel clock source. Access can be secured by GTZC_TZSC I2C3SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The I2C3 is functional in Stop modes only when the kernel clock is HSI"]
        #[inline(always)]
        pub const fn set_i2c3sel(&mut self, val: super::vals::I2c3sel) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
        #[doc = "LPTIM1 kernel clock source selection These bits are used to select the LPTIM1 kernel clock source. Access can be secured by GTZC_TZSC LPTIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The LPTIM1 is functional in Stop modes only when the kernel clock is LSI, LSE, HSI with HSIKERON = 1."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim1sel(&self) -> super::vals::Lptim1sel {
            let val = (self.0 >> 10usize) & 0x03;
            super::vals::Lptim1sel::from_bits(val as u8)
        }
        #[doc = "LPTIM1 kernel clock source selection These bits are used to select the LPTIM1 kernel clock source. Access can be secured by GTZC_TZSC LPTIM1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: The LPTIM1 is functional in Stop modes only when the kernel clock is LSI, LSE, HSI with HSIKERON = 1."]
        #[inline(always)]
        pub const fn set_lptim1sel(&mut self, val: super::vals::Lptim1sel) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val.to_bits() as u32) & 0x03) << 10usize);
        }
        #[doc = "ADC4 kernel clock source selection These bits are used to select the ADC4 kernel clock source. Access can be secured by GTZC_TZSC ADC4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. others: reserved Note: The ADC4 is functional in Stop modes only when the kernel clock is HSI."]
        #[must_use]
        #[inline(always)]
        pub const fn adcsel(&self) -> super::vals::Adcsel {
            let val = (self.0 >> 12usize) & 0x07;
            super::vals::Adcsel::from_bits(val as u8)
        }
        #[doc = "ADC4 kernel clock source selection These bits are used to select the ADC4 kernel clock source. Access can be secured by GTZC_TZSC ADC4SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. others: reserved Note: The ADC4 is functional in Stop modes only when the kernel clock is HSI."]
        #[inline(always)]
        pub const fn set_adcsel(&mut self, val: super::vals::Adcsel) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
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
                .field("lptim1sel", &self.lptim1sel())
                .field("adcsel", &self.adcsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccipr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccipr3 {{ lpuart1sel: {:?}, spi3sel: {:?}, i2c3sel: {:?}, lptim1sel: {:?}, adcsel: {:?} }}",
                self.lpuart1sel(),
                self.spi3sel(),
                self.i2c3sel(),
                self.lptim1sel(),
                self.adcsel()
            )
        }
    }
    #[doc = "RCC clock configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr1(pub u32);
    impl Cfgr1 {
        #[doc = "system clock switch Set and cleared by software to select system clock source (SYSCLK). Cleared by hardware when entering Stop and Standby modes When selecting HSE directly or indirectly as system clock and HSE oscillator clock security fails, cleared by hardware."]
        #[must_use]
        #[inline(always)]
        pub const fn sw(&self) -> super::vals::Sw {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "system clock switch Set and cleared by software to select system clock source (SYSCLK). Cleared by hardware when entering Stop and Standby modes When selecting HSE directly or indirectly as system clock and HSE oscillator clock security fails, cleared by hardware."]
        #[inline(always)]
        pub const fn set_sw(&mut self, val: super::vals::Sw) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "system clock switch status Set and cleared by hardware to indicate which clock source is used as system clock."]
        #[must_use]
        #[inline(always)]
        pub const fn sws(&self) -> super::vals::Sw {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Sw::from_bits(val as u8)
        }
        #[doc = "system clock switch status Set and cleared by hardware to indicate which clock source is used as system clock."]
        #[inline(always)]
        pub const fn set_sws(&mut self, val: super::vals::Sw) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "microcontroller clock output Set and cleared by software. others: reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching."]
        #[must_use]
        #[inline(always)]
        pub const fn mcosel(&self) -> super::vals::Mcosel {
            let val = (self.0 >> 24usize) & 0x0f;
            super::vals::Mcosel::from_bits(val as u8)
        }
        #[doc = "microcontroller clock output Set and cleared by software. others: reserved Note: This clock output may have some truncated cycles at startup or during MCO clock source switching."]
        #[inline(always)]
        pub const fn set_mcosel(&mut self, val: super::vals::Mcosel) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
        }
        #[doc = "microcontroller clock output prescaler Set and cleared by software. It is highly recommended to change this prescaler before MCO output is enabled. others: not allowed"]
        #[must_use]
        #[inline(always)]
        pub const fn mcopre(&self) -> super::vals::Mcopre {
            let val = (self.0 >> 28usize) & 0x07;
            super::vals::Mcopre::from_bits(val as u8)
        }
        #[doc = "microcontroller clock output prescaler Set and cleared by software. It is highly recommended to change this prescaler before MCO output is enabled. others: not allowed"]
        #[inline(always)]
        pub const fn set_mcopre(&mut self, val: super::vals::Mcopre) {
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
                .field("mcosel", &self.mcosel())
                .field("mcopre", &self.mcopre())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfgr1 {{ sw: {:?}, sws: {:?}, mcosel: {:?}, mcopre: {:?} }}",
                self.sw(),
                self.sws(),
                self.mcosel(),
                self.mcopre()
            )
        }
    }
    #[doc = "RCC clock configuration register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "AHB1, AHB2 and AHB4 prescaler Set and cleared by software to control the division factor of the AHB1, AHB2 and AHB4 clock (hclk1). The software must limit the incremental frequency step by setting these bits correctly to ensure that the hclk1 maximum incremental frequency step does not exceed the maximum allowed incremental frequency step (for more details, refer to Table�99: SYSCLK and bus maximum frequency). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account. 0xx: hclk1 = SYSCLK not divided"]
        #[must_use]
        #[inline(always)]
        pub const fn hpre(&self) -> super::vals::Hpre {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Hpre::from_bits(val as u8)
        }
        #[doc = "AHB1, AHB2 and AHB4 prescaler Set and cleared by software to control the division factor of the AHB1, AHB2 and AHB4 clock (hclk1). The software must limit the incremental frequency step by setting these bits correctly to ensure that the hclk1 maximum incremental frequency step does not exceed the maximum allowed incremental frequency step (for more details, refer to Table�99: SYSCLK and bus maximum frequency). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account. 0xx: hclk1 = SYSCLK not divided"]
        #[inline(always)]
        pub const fn set_hpre(&mut self, val: super::vals::Hpre) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "APB1 prescaler Set and cleared by software to control the division factor of the APB1 clock (pclk1). 0xx: pclk1 = hclk1 not divided"]
        #[must_use]
        #[inline(always)]
        pub const fn ppre1(&self) -> super::vals::Ppre {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB1 prescaler Set and cleared by software to control the division factor of the APB1 clock (pclk1). 0xx: pclk1 = hclk1 not divided"]
        #[inline(always)]
        pub const fn set_ppre1(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "APB2 prescaler Set and cleared by software to control the division factor of the APB2 clock (pclk2). 0xx: pclk2 = hclk1 not divided"]
        #[must_use]
        #[inline(always)]
        pub const fn ppre2(&self) -> super::vals::Ppre {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB2 prescaler Set and cleared by software to control the division factor of the APB2 clock (pclk2). 0xx: pclk2 = hclk1 not divided"]
        #[inline(always)]
        pub const fn set_ppre2(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfgr2 {{ hpre: {:?}, ppre1: {:?}, ppre2: {:?} }}",
                self.hpre(),
                self.ppre1(),
                self.ppre2()
            )
        }
    }
    #[doc = "RCC clock configuration register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr3(pub u32);
    impl Cfgr3 {
        #[doc = "APB7 prescaler Set and cleared by software to control the division factor of the APB7 clock (pclk7). 0xx: hclk1 not divided"]
        #[must_use]
        #[inline(always)]
        pub const fn ppre7(&self) -> super::vals::Ppre {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Ppre::from_bits(val as u8)
        }
        #[doc = "APB7 prescaler Set and cleared by software to control the division factor of the APB7 clock (pclk7). 0xx: hclk1 not divided"]
        #[inline(always)]
        pub const fn set_ppre7(&mut self, val: super::vals::Ppre) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
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
            f.debug_struct("Cfgr3").field("ppre7", &self.ppre7()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cfgr3 {{ ppre7: {:?} }}", self.ppre7())
        }
    }
    #[doc = "RCC clock configuration register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr4(pub u32);
    impl Cfgr4 {
        #[doc = "AHB5 prescaler when SWS select PLL1 Set and cleared by software to control the division factor of the AHB5 clock (hclk5). Must not be changed when SYSCLK source indicated by SWS is PLL1. When SYSCLK source indicated by SWS is not PLL1: HPRE5 is not taken into account. When SYSCLK source indicated by SWS is PLL1: HPRE5 is taken into account, from the moment the system clock switch occurs Depending on the device voltage range, the software must set these bits correctly to ensure that the AHB5 frequency does not exceed the maximum allowed frequency (for more details, refer to Table�99: SYSCLK and bus maximum frequency). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account. 0xx: hclk5 = SYSCLK not divided"]
        #[must_use]
        #[inline(always)]
        pub const fn hpre5(&self) -> super::vals::Hpre5 {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Hpre5::from_bits(val as u8)
        }
        #[doc = "AHB5 prescaler when SWS select PLL1 Set and cleared by software to control the division factor of the AHB5 clock (hclk5). Must not be changed when SYSCLK source indicated by SWS is PLL1. When SYSCLK source indicated by SWS is not PLL1: HPRE5 is not taken into account. When SYSCLK source indicated by SWS is PLL1: HPRE5 is taken into account, from the moment the system clock switch occurs Depending on the device voltage range, the software must set these bits correctly to ensure that the AHB5 frequency does not exceed the maximum allowed frequency (for more details, refer to Table�99: SYSCLK and bus maximum frequency). After a write operation to these bits and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account. 0xx: hclk5 = SYSCLK not divided"]
        #[inline(always)]
        pub const fn set_hpre5(&mut self, val: super::vals::Hpre5) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "AHB5 divider when SWS select HSI or HSE Set and reset by software. Set to 1 by hardware when entering Stop 1 mode. When SYSCLK source indicated by SWS is HSI or HSE: HDIV5 is taken into account When SYSCLK source indicated by SWS is PLL1: HDIV5 is taken not taken into account Depending on the device voltage range, the software must set this bit correctly to ensure that the AHB5 frequency does not exceed the maximum allowed frequency (for more details, refer to Table�99). After a write operation to this bit and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account."]
        #[must_use]
        #[inline(always)]
        pub const fn hdiv5(&self) -> super::vals::Hdiv5 {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Hdiv5::from_bits(val as u8)
        }
        #[doc = "AHB5 divider when SWS select HSI or HSE Set and reset by software. Set to 1 by hardware when entering Stop 1 mode. When SYSCLK source indicated by SWS is HSI or HSE: HDIV5 is taken into account When SYSCLK source indicated by SWS is PLL1: HDIV5 is taken not taken into account Depending on the device voltage range, the software must set this bit correctly to ensure that the AHB5 frequency does not exceed the maximum allowed frequency (for more details, refer to Table�99). After a write operation to this bit and before decreasing the voltage range, this register must be read to be sure that the new value is taken into account."]
        #[inline(always)]
        pub const fn set_hdiv5(&mut self, val: super::vals::Hdiv5) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Cfgr4 {
        #[inline(always)]
        fn default() -> Cfgr4 {
            Cfgr4(0)
        }
    }
    impl core::fmt::Debug for Cfgr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr4")
                .field("hpre5", &self.hpre5())
                .field("hdiv5", &self.hdiv5())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cfgr4 {{ hpre5: {:?}, hdiv5: {:?} }}", self.hpre5(), self.hdiv5())
        }
    }
    #[doc = "RCC clock interrupt clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cicr(pub u32);
    impl Cicr {
        #[doc = "LSI1 ready interrupt clear Writing this bit to 1 clears the LSI1RDYF flag. Writing 0 has no effect. Access to the bit can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn lsi1rdyc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI1 ready interrupt clear Writing this bit to 1 clears the LSI1RDYF flag. Writing 0 has no effect. Access to the bit can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_lsi1rdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt clear Writing this bit to 1 clears the LSERDYF flag. Writing 0 has no effect. Access to the bit can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn lserdyc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt clear Writing this bit to 1 clears the LSERDYF flag. Writing 0 has no effect. Access to the bit can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_lserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI ready interrupt clear Writing this bit to 1 clears the HSIRDYF flag. Writing 0 has no effect.\\ Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn hsirdyc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt clear Writing this bit to 1 clears the HSIRDYF flag. Writing 0 has no effect.\\ Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_hsirdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HSE ready interrupt clear Writing this bit to 1 clears the HSERDYF flag. Writing 0 has no effect. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn hserdyc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt clear Writing this bit to 1 clears the HSERDYF flag. Writing 0 has no effect. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_hserdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "PLL1 ready interrupt clear Writing this bit to 1 clears the PLL1RDYF flag. Writing 0 has no effect. Access to the bit can be secured by RCC PLL1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn pllrdyc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PLL1 ready interrupt clear Writing this bit to 1 clears the PLL1RDYF flag. Writing 0 has no effect. Access to the bit can be secured by RCC PLL1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_pllrdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "High speed external clock security system interrupt clear Writing this bit to 1 clears the HSECSSF flag. Writing 0 has no effect. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn hsecssc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "High speed external clock security system interrupt clear Writing this bit to 1 clears the HSECSSF flag. Writing 0 has no effect. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_hsecssc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LSI2 ready interrupt clear."]
        #[must_use]
        #[inline(always)]
        pub const fn lsi2rdyc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "LSI2 ready interrupt clear."]
        #[inline(always)]
        pub const fn set_lsi2rdyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
                .field("lsi1rdyc", &self.lsi1rdyc())
                .field("lserdyc", &self.lserdyc())
                .field("hsirdyc", &self.hsirdyc())
                .field("hserdyc", &self.hserdyc())
                .field("pllrdyc", &self.pllrdyc())
                .field("hsecssc", &self.hsecssc())
                .field("lsi2rdyc", &self.lsi2rdyc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cicr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cicr {{ lsi1rdyc: {=bool:?}, lserdyc: {=bool:?}, hsirdyc: {=bool:?}, hserdyc: {=bool:?}, pllrdyc: {=bool:?}, hsecssc: {=bool:?}, lsi2rdyc: {=bool:?} }}" , self . lsi1rdyc () , self . lserdyc () , self . hsirdyc () , self . hserdyc () , self . pllrdyc () , self . hsecssc () , self . lsi2rdyc ())
        }
    }
    #[doc = "RCC clock interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cier(pub u32);
    impl Cier {
        #[doc = "LSI1 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI1 oscillator stabilization. Access to the bit can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn lsi1rdyie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI1 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSI1 oscillator stabilization. Access to the bit can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_lsi1rdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization. Access to the bit can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn lserdyie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the LSE oscillator stabilization. Access to the bit can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_lserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI oscillator stabilization. Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn hsirdyie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSI oscillator stabilization. Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_hsirdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn hserdyie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt enable Set and cleared by software to enable/disable interrupt caused by the HSE oscillator stabilization. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_hserdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "PLL1 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL1 lock. Access to the bit can be secured by RCC PLL1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn pllrdyie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PLL1 ready interrupt enable Set and cleared by software to enable/disable interrupt caused by PLL1 lock. Access to the bit can be secured by RCC PLL1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_pllrdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LSI2 ready interrupt enable"]
        #[must_use]
        #[inline(always)]
        pub const fn lsi2rdyie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "LSI2 ready interrupt enable"]
        #[inline(always)]
        pub const fn set_lsi2rdyie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
                .field("lsi1rdyie", &self.lsi1rdyie())
                .field("lserdyie", &self.lserdyie())
                .field("hsirdyie", &self.hsirdyie())
                .field("hserdyie", &self.hserdyie())
                .field("pllrdyie", &self.pllrdyie())
                .field("lsi2rdyie", &self.lsi2rdyie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cier {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cier {{ lsi1rdyie: {=bool:?}, lserdyie: {=bool:?}, hsirdyie: {=bool:?}, hserdyie: {=bool:?}, pllrdyie: {=bool:?}, lsi2rdyie: {=bool:?} }}" , self . lsi1rdyie () , self . lserdyie () , self . hsirdyie () , self . hserdyie () , self . pllrdyie () , self . lsi2rdyie ())
        }
    }
    #[doc = "RCC clock interrupt flag register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cifr(pub u32);
    impl Cifr {
        #[doc = "LSI1 ready interrupt flag Set by hardware when the LSI1 clock becomes stable and LSI1RDYIE is set. Cleared by software setting the LSI1RDYC bit. Access to the bit can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn lsi1rdyf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LSI1 ready interrupt flag Set by hardware when the LSI1 clock becomes stable and LSI1RDYIE is set. Cleared by software setting the LSI1RDYC bit. Access to the bit can be secured by RCC LSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_lsi1rdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LSE ready interrupt flag Set by hardware when the LSE clock becomes stable and LSERDYIE is set. Cleared by software setting the LSERDYC bit. Access to the bit can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn lserdyf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LSE ready interrupt flag Set by hardware when the LSE clock becomes stable and LSERDYIE is set. Cleared by software setting the LSERDYC bit. Access to the bit can be secured by RCC LSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_lserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "HSI ready interrupt flag Set by hardware when the HSI clock becomes stable and HSIRDYIE is set in a response to setting the HSION (see CR). When HSION is not set but the HSI oscillator is enabled by the peripheral through a clock request, this bit is not set and no interrupt is generated. Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Cleared by software setting the HSIRDYC bit."]
        #[must_use]
        #[inline(always)]
        pub const fn hsirdyf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "HSI ready interrupt flag Set by hardware when the HSI clock becomes stable and HSIRDYIE is set in a response to setting the HSION (see CR). When HSION is not set but the HSI oscillator is enabled by the peripheral through a clock request, this bit is not set and no interrupt is generated. Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Cleared by software setting the HSIRDYC bit."]
        #[inline(always)]
        pub const fn set_hsirdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "HSE ready interrupt flag Set by hardware when the HSE clock becomes stable and HSERDYIE is set. Cleared by software setting the HSERDYC bit. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn hserdyf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "HSE ready interrupt flag Set by hardware when the HSE clock becomes stable and HSERDYIE is set. Cleared by software setting the HSERDYC bit. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_hserdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "PLL1 ready interrupt flag Set by hardware when the PLL1 locks and PLL1RDYIE is set. Cleared by software setting the PLL1RDYC bit. Access to the bit can be secured by RCC PLL1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn pllrdyf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "PLL1 ready interrupt flag Set by hardware when the PLL1 locks and PLL1RDYIE is set. Cleared by software setting the PLL1RDYC bit. Access to the bit can be secured by RCC PLL1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_pllrdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "HSE clock security system interrupt flag Set by hardware when a clock security failure is detected in the HSE oscillator. Cleared by software setting the HSECSSC bit. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn hsecssf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock security system interrupt flag Set by hardware when a clock security failure is detected in the HSE oscillator. Cleared by software setting the HSECSSC bit. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_hsecssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LSI2 ready interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn lsi2rdyf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "LSI2 ready interrupt flag."]
        #[inline(always)]
        pub const fn set_lsi2rdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
                .field("lsi1rdyf", &self.lsi1rdyf())
                .field("lserdyf", &self.lserdyf())
                .field("hsirdyf", &self.hsirdyf())
                .field("hserdyf", &self.hserdyf())
                .field("pllrdyf", &self.pllrdyf())
                .field("hsecssf", &self.hsecssf())
                .field("lsi2rdyf", &self.lsi2rdyf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cifr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cifr {{ lsi1rdyf: {=bool:?}, lserdyf: {=bool:?}, hsirdyf: {=bool:?}, hserdyf: {=bool:?}, pllrdyf: {=bool:?}, hsecssf: {=bool:?}, lsi2rdyf: {=bool:?} }}" , self . lsi1rdyf () , self . lserdyf () , self . hsirdyf () , self . hserdyf () , self . pllrdyf () , self . hsecssf () , self . lsi2rdyf ())
        }
    }
    #[doc = "RCC clock control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "HSI clock enable Set and cleared by software. Cleared by hardware when entering Stop and Standby modes. Set by hardware to force the HSI oscillator on when exiting Stop and Standby modes. Set by hardware to force the HSI oscillator on in case of clock security failure of the HSE crystal oscillator. This bit is set by hardware if the HSI is used directly or indirectly as system clock. Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn hsion(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "HSI clock enable Set and cleared by software. Cleared by hardware when entering Stop and Standby modes. Set by hardware to force the HSI oscillator on when exiting Stop and Standby modes. Set by hardware to force the HSI oscillator on in case of clock security failure of the HSE crystal oscillator. This bit is set by hardware if the HSI is used directly or indirectly as system clock. Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_hsion(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "HSI enable for some peripheral kernels Set and cleared by software to force HSI oscillator on even in Stop modes. Keeping the HSI oscillator on in Stop modes allows the communication speed not to be reduced by the HSI oscillator startup time. This bit has no effect on register bit HSION value. Cleared by hardware when entering Standby modes. Refer to Peripherals clock gating and autonomous mode for more details. Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn hsikeron(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "HSI enable for some peripheral kernels Set and cleared by software to force HSI oscillator on even in Stop modes. Keeping the HSI oscillator on in Stop modes allows the communication speed not to be reduced by the HSI oscillator startup time. This bit has no effect on register bit HSION value. Cleared by hardware when entering Standby modes. Refer to Peripherals clock gating and autonomous mode for more details. Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_hsikeron(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "HSI clock ready flag Set by hardware to indicate that HSI oscillator is stable. This bit is set only when HSI is enabled by software by setting HSION. Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: Once the HSION bit is cleared, HSIRDY goes low after six HSI clock cycles."]
        #[must_use]
        #[inline(always)]
        pub const fn hsirdy(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "HSI clock ready flag Set by hardware to indicate that HSI oscillator is stable. This bit is set only when HSI is enabled by software by setting HSION. Access to the bit can be secured by RCC HSISEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV. Note: Once the HSION bit is cleared, HSIRDY goes low after six HSI clock cycles."]
        #[inline(always)]
        pub const fn set_hsirdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE clock for the CPU when entering Stop and Standby modes and on a HSECSS failure. When the HSE is used as 2.4 GHz RADIO kernel clock, enabled by RADIOEN and RADIOSMEN and the 2.4 GHz RADIO is active, HSEON is not be cleared when entering low power mode. In this case only Stop 0 mode is entered as low power mode. This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn hseon(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock enable Set and cleared by software. Cleared by hardware to stop the HSE clock for the CPU when entering Stop and Standby modes and on a HSECSS failure. When the HSE is used as 2.4 GHz RADIO kernel clock, enabled by RADIOEN and RADIOSMEN and the 2.4 GHz RADIO is active, HSEON is not be cleared when entering low power mode. In this case only Stop 0 mode is entered as low power mode. This bit cannot be reset if the HSE oscillator is used directly or indirectly as the system clock. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_hseon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "HSE clock ready flag Set by hardware to indicate that the HSE oscillator is stable. This bit is set both when HSE is enabled by software by setting HSEON and when requested as kernel clock by the 2.4 GHz RADIO. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn hserdy(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock ready flag Set by hardware to indicate that the HSE oscillator is stable. This bit is set both when HSE is enabled by software by setting HSEON and when requested as kernel clock by the 2.4 GHz RADIO. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_hserdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "HSE clock security system enable Set by software to enable the HSE clock security system. When HSECSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn hsecsson(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock security system enable Set by software to enable the HSE clock security system. When HSECSSON is set, the clock detector is enabled by hardware when the HSE oscillator is ready and disabled by hardware if a HSE clock failure is detected. This bit is set only and is cleared by reset. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_hsecsson(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "HSE clock for SYSCLK prescaler Set and cleared by software to control the division factor of the HSE clock for SYSCLK. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn hsepre(&self) -> super::vals::Hsepre {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Hsepre::from_bits(val as u8)
        }
        #[doc = "HSE clock for SYSCLK prescaler Set and cleared by software to control the division factor of the HSE clock for SYSCLK. Access to the bit can be secured by RCC HSESEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_hsepre(&mut self, val: super::vals::Hsepre) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "PLL1 enable Set and cleared by software to enable the main PLL. Cleared by hardware when entering Stop or Standby modes and when PLL1 on HSE is selected as sysclk, on a HSECSS failure. This bit cannot be reset if the PLL1 clock is used as the system clock. Access to the bit can be secured by RCC PLL1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn pllon(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "PLL1 enable Set and cleared by software to enable the main PLL. Cleared by hardware when entering Stop or Standby modes and when PLL1 on HSE is selected as sysclk, on a HSECSS failure. This bit cannot be reset if the PLL1 clock is used as the system clock. Access to the bit can be secured by RCC PLL1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_pllon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "PLL1 clock ready flag Set by hardware to indicate that the PLL1 is locked. Access to the bit can be secured by RCC PLL1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn pllrdy(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "PLL1 clock ready flag Set by hardware to indicate that the PLL1 is locked. Access to the bit can be secured by RCC PLL1SEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
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
                .field("hsikeron", &self.hsikeron())
                .field("hsirdy", &self.hsirdy())
                .field("hseon", &self.hseon())
                .field("hserdy", &self.hserdy())
                .field("hsecsson", &self.hsecsson())
                .field("hsepre", &self.hsepre())
                .field("pllon", &self.pllon())
                .field("pllrdy", &self.pllrdy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Cr {{ hsion: {=bool:?}, hsikeron: {=bool:?}, hsirdy: {=bool:?}, hseon: {=bool:?}, hserdy: {=bool:?}, hsecsson: {=bool:?}, hsepre: {:?}, pllon: {=bool:?}, pllrdy: {=bool:?} }}" , self . hsion () , self . hsikeron () , self . hsirdy () , self . hseon () , self . hserdy () , self . hsecsson () , self . hsepre () , self . pllon () , self . pllrdy ())
        }
    }
    #[doc = "RCC control/status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "Remove reset flag Set by software to clear the reset flags. Access can be secured by RCC RMVFSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn rmvf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Remove reset flag Set by software to clear the reset flags. Access can be secured by RCC RMVFSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with RCC SPRIV or when non-secure with RCC NSPRIV."]
        #[inline(always)]
        pub const fn set_rmvf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Option byte loader reset flag Set by hardware when a reset from the option byte loading occurs. Cleared by writing to the RMVF bit."]
        #[must_use]
        #[inline(always)]
        pub const fn oblrstf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte loader reset flag Set by hardware when a reset from the option byte loading occurs. Cleared by writing to the RMVF bit."]
        #[inline(always)]
        pub const fn set_oblrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "NRST pin reset flag Set by hardware when a reset from the NRST pin occurs. Cleared by writing to the RMVF bit."]
        #[must_use]
        #[inline(always)]
        pub const fn pinrstf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "NRST pin reset flag Set by hardware when a reset from the NRST pin occurs. Cleared by writing to the RMVF bit."]
        #[inline(always)]
        pub const fn set_pinrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "BOR flag Set by hardware when a BOR occurs. Cleared by writing to the RMVF bit."]
        #[must_use]
        #[inline(always)]
        pub const fn borrstf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "BOR flag Set by hardware when a BOR occurs. Cleared by writing to the RMVF bit."]
        #[inline(always)]
        pub const fn set_borrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Software reset flag Set by hardware when a software reset occurs. Cleared by writing to the RMVF bit."]
        #[must_use]
        #[inline(always)]
        pub const fn sftrstf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Software reset flag Set by hardware when a software reset occurs. Cleared by writing to the RMVF bit."]
        #[inline(always)]
        pub const fn set_sftrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Independent watchdog reset flag Set by hardware when an independent watchdog reset domain occurs. Cleared by writing to the RMVF bit."]
        #[must_use]
        #[inline(always)]
        pub const fn iwdgrstf(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog reset flag Set by hardware when an independent watchdog reset domain occurs. Cleared by writing to the RMVF bit."]
        #[inline(always)]
        pub const fn set_iwdgrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Window watchdog reset flag Set by hardware when a window watchdog reset occurs. Cleared by writing to the RMVF bit."]
        #[must_use]
        #[inline(always)]
        pub const fn wwdgrstf(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog reset flag Set by hardware when a window watchdog reset occurs. Cleared by writing to the RMVF bit."]
        #[inline(always)]
        pub const fn set_wwdgrstf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Low-power reset flag Set by hardware when a reset occurs due to illegal Stop and Standby modes entry. Cleared by writing to the RMVF bit."]
        #[must_use]
        #[inline(always)]
        pub const fn lpwrrstf(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power reset flag Set by hardware when a reset occurs due to illegal Stop and Standby modes entry. Cleared by writing to the RMVF bit."]
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
            defmt :: write ! (f , "Csr {{ rmvf: {=bool:?}, oblrstf: {=bool:?}, pinrstf: {=bool:?}, borrstf: {=bool:?}, sftrstf: {=bool:?}, iwdgrstf: {=bool:?}, wwdgrstf: {=bool:?}, lpwrrstf: {=bool:?} }}" , self . rmvf () , self . oblrstf () , self . pinrstf () , self . borrstf () , self . sftrstf () , self . iwdgrstf () , self . wwdgrstf () , self . lpwrrstf ())
        }
    }
    #[doc = "RCC external clock sources calibration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ecscr1(pub u32);
    impl Ecscr1 {
        #[doc = "HSE clock trimming These bits provide user-programmable capacitor trimming value. It can be programmed to adjust the HSE oscillator frequency."]
        #[must_use]
        #[inline(always)]
        pub const fn hsetrim(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "HSE clock trimming These bits provide user-programmable capacitor trimming value. It can be programmed to adjust the HSE oscillator frequency."]
        #[inline(always)]
        pub const fn set_hsetrim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
    }
    impl Default for Ecscr1 {
        #[inline(always)]
        fn default() -> Ecscr1 {
            Ecscr1(0)
        }
    }
    impl core::fmt::Debug for Ecscr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ecscr1").field("hsetrim", &self.hsetrim()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ecscr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ecscr1 {{ hsetrim: {=u8:?} }}", self.hsetrim())
        }
    }
    #[doc = "RCC internal clock sources calibration register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icscr3(pub u32);
    impl Icscr3 {
        #[doc = "HSI clock calibration These bits are initialized at startup with the factory-programmed HSI calibration value. When HSITRIM\\[4:0\\]
is written, HSICAL\\[11:0\\]
is updated with the sum of HSITRIM\\[4:0\\]
and the initial factory trim value."]
        #[must_use]
        #[inline(always)]
        pub const fn hsical(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "HSI clock calibration These bits are initialized at startup with the factory-programmed HSI calibration value. When HSITRIM\\[4:0\\]
is written, HSICAL\\[11:0\\]
is updated with the sum of HSITRIM\\[4:0\\]
and the initial factory trim value."]
        #[inline(always)]
        pub const fn set_hsical(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "HSI clock trimming These bits provide an additional user-programmable trimming value that is added to the HSICAL\\[11:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the HSI."]
        #[must_use]
        #[inline(always)]
        pub const fn hsitrim(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "HSI clock trimming These bits provide an additional user-programmable trimming value that is added to the HSICAL\\[11:0\\]
bits. It can be programmed to adjust to voltage and temperature variations that influence the frequency of the HSI."]
        #[inline(always)]
        pub const fn set_hsitrim(&mut self, val: u8) {
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
            defmt::write!(
                f,
                "Icscr3 {{ hsical: {=u16:?}, hsitrim: {=u8:?} }}",
                self.hsical(),
                self.hsitrim()
            )
        }
    }
    #[doc = "RCC PLL1 configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pll1cfgr(pub u32);
    impl Pll1cfgr {
        #[doc = "PLL1 entry clock source Set and cleared by software to select PLL1 clock source. These bits can be written only when the PLL1 is disabled. Cleared by hardware when entering Stop or Standby modes. Note: In order to save power, when no PLL1 clock is used, the value of PLL1SRC must be 0."]
        #[must_use]
        #[inline(always)]
        pub const fn pllsrc(&self) -> super::vals::Pllsrc {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Pllsrc::from_bits(val as u8)
        }
        #[doc = "PLL1 entry clock source Set and cleared by software to select PLL1 clock source. These bits can be written only when the PLL1 is disabled. Cleared by hardware when entering Stop or Standby modes. Note: In order to save power, when no PLL1 clock is used, the value of PLL1SRC must be 0."]
        #[inline(always)]
        pub const fn set_pllsrc(&mut self, val: super::vals::Pllsrc) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1. 00-01-10: PLL1 input (ref1_ck) clock range frequency between 4 and 8 MHz"]
        #[must_use]
        #[inline(always)]
        pub const fn pllrge(&self) -> super::vals::Pllrge {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Pllrge::from_bits(val as u8)
        }
        #[doc = "PLL1 input frequency range Set and reset by software to select the proper reference frequency range used for PLL1. This bit must be written before enabling the PLL1. 00-01-10: PLL1 input (ref1_ck) clock range frequency between 4 and 8 MHz"]
        #[inline(always)]
        pub const fn set_pllrge(&mut self, val: super::vals::Pllrge) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "PLL1 fractional latch enable Set and reset by software to latch the content of PLL1FRACN into the ΣΔ modulator. In order to latch the PLL1FRACN value into the ΣΔ modulator, PLL1FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL1FRACN into the modulator (see PLL1 initialization phase for details)."]
        #[must_use]
        #[inline(always)]
        pub const fn pllfracen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PLL1 fractional latch enable Set and reset by software to latch the content of PLL1FRACN into the ΣΔ modulator. In order to latch the PLL1FRACN value into the ΣΔ modulator, PLL1FRACEN must be set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL1FRACN into the modulator (see PLL1 initialization phase for details)."]
        #[inline(always)]
        pub const fn set_pllfracen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The VCO1 input frequency is PLL1 input clock frequency/PLL1M. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
        #[must_use]
        #[inline(always)]
        pub const fn pllm(&self) -> super::vals::Pllm {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Pllm::from_bits(val as u8)
        }
        #[doc = "Prescaler for PLL1 Set and cleared by software to configure the prescaler of the PLL1. The VCO1 input frequency is PLL1 input clock frequency/PLL1M. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
        #[inline(always)]
        pub const fn set_pllm(&mut self, val: super::vals::Pllm) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "PLL1 DIVP divider output enable Set and reset by software to enable the pll1pclk output of the PLL1. To save power, PLL1PEN and PLL1P bits must be set to 0 when the pll1pclk is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
        #[must_use]
        #[inline(always)]
        pub const fn pllpen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "PLL1 DIVP divider output enable Set and reset by software to enable the pll1pclk output of the PLL1. To save power, PLL1PEN and PLL1P bits must be set to 0 when the pll1pclk is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
        #[inline(always)]
        pub const fn set_pllpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "PLL1 DIVQ divider output enable Set and reset by software to enable the pll1qclk output of the PLL1. To save power, PLL1QEN and PLL1Q bits must be set to 0 when the pll1qclk is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
        #[must_use]
        #[inline(always)]
        pub const fn pllqen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "PLL1 DIVQ divider output enable Set and reset by software to enable the pll1qclk output of the PLL1. To save power, PLL1QEN and PLL1Q bits must be set to 0 when the pll1qclk is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
        #[inline(always)]
        pub const fn set_pllqen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "PLL1 DIVR divider output enable Set and cleared by software to enable the pll1rclk output of the PLL1. To save power, PLL1REN and PLL1R bits must be set to 0 when the pll1rclk is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
        #[must_use]
        #[inline(always)]
        pub const fn pllren(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PLL1 DIVR divider output enable Set and cleared by software to enable the pll1rclk output of the PLL1. To save power, PLL1REN and PLL1R bits must be set to 0 when the pll1rclk is not used. This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0)."]
        #[inline(always)]
        pub const fn set_pllren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "pll1rclk clock for SYSCLK prescaler division enable Set and cleared by software to control the division of the pll1rclk clock for SYSCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn pllrclkpre(&self) -> super::vals::Pllrclkpre {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Pllrclkpre::from_bits(val as u8)
        }
        #[doc = "pll1rclk clock for SYSCLK prescaler division enable Set and cleared by software to control the division of the pll1rclk clock for SYSCLK."]
        #[inline(always)]
        pub const fn set_pllrclkpre(&mut self, val: super::vals::Pllrclkpre) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "pll1rclk clock for SYSCLK prescaler division step selection Set and cleared by software to control the division step of the pll1rclk clock for SYSCLK."]
        #[must_use]
        #[inline(always)]
        pub const fn pllrclkprestep(&self) -> super::vals::Pllrclkprestep {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Pllrclkprestep::from_bits(val as u8)
        }
        #[doc = "pll1rclk clock for SYSCLK prescaler division step selection Set and cleared by software to control the division step of the pll1rclk clock for SYSCLK."]
        #[inline(always)]
        pub const fn set_pllrclkprestep(&mut self, val: super::vals::Pllrclkprestep) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "pll1rclkpre not divided ready. Set by hardware after PLL1RCLKPRE has been set from divided to not divide, to indicate that the pll1rclk not divided is available on sysclkpre."]
        #[must_use]
        #[inline(always)]
        pub const fn pllrclkprerdy(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "pll1rclkpre not divided ready. Set by hardware after PLL1RCLKPRE has been set from divided to not divide, to indicate that the pll1rclk not divided is available on sysclkpre."]
        #[inline(always)]
        pub const fn set_pllrclkprerdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
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
                .field("pllpen", &self.pllpen())
                .field("pllqen", &self.pllqen())
                .field("pllren", &self.pllren())
                .field("pllrclkpre", &self.pllrclkpre())
                .field("pllrclkprestep", &self.pllrclkprestep())
                .field("pllrclkprerdy", &self.pllrclkprerdy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pll1cfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Pll1cfgr {{ pllsrc: {:?}, pllrge: {:?}, pllfracen: {=bool:?}, pllm: {:?}, pllpen: {=bool:?}, pllqen: {=bool:?}, pllren: {=bool:?}, pllrclkpre: {:?}, pllrclkprestep: {:?}, pllrclkprerdy: {=bool:?} }}" , self . pllsrc () , self . pllrge () , self . pllfracen () , self . pllm () , self . pllpen () , self . pllqen () , self . pllren () , self . pllrclkpre () , self . pllrclkprestep () , self . pllrclkprerdy ())
        }
    }
    #[doc = "RCC PLL1 dividers register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pll1divr(pub u32);
    impl Pll1divr {
        #[doc = "Multiplication factor for PLL1 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ... ... others: reserved VCO output frequency = F<sub>ref1_ck</sub> x multiplication factor for PLL1 VCO, when fractional value 0 has been loaded into PLL1FRACN, with: Multiplication factor for PLL1 VCO between 4 and 512 input frequency F<sub>ref1_ck</sub> between 4 and 16�MHz"]
        #[must_use]
        #[inline(always)]
        pub const fn plln(&self) -> super::vals::Plln {
            let val = (self.0 >> 0usize) & 0x01ff;
            super::vals::Plln::from_bits(val as u16)
        }
        #[doc = "Multiplication factor for PLL1 VCO Set and reset by software to control the multiplication factor of the VCO. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ... ... others: reserved VCO output frequency = F<sub>ref1_ck</sub> x multiplication factor for PLL1 VCO, when fractional value 0 has been loaded into PLL1FRACN, with: Multiplication factor for PLL1 VCO between 4 and 512 input frequency F<sub>ref1_ck</sub> between 4 and 16�MHz"]
        #[inline(always)]
        pub const fn set_plln(&mut self, val: super::vals::Plln) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val.to_bits() as u32) & 0x01ff) << 0usize);
        }
        #[doc = "PLL1 DIVP division factor Set and reset by software to control the frequency of the pll1pclk clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). Note that odd division factors are not allowed. ..."]
        #[must_use]
        #[inline(always)]
        pub const fn pllp(&self) -> super::vals::Plldiv {
            let val = (self.0 >> 9usize) & 0x7f;
            super::vals::Plldiv::from_bits(val as u8)
        }
        #[doc = "PLL1 DIVP division factor Set and reset by software to control the frequency of the pll1pclk clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). Note that odd division factors are not allowed. ..."]
        #[inline(always)]
        pub const fn set_pllp(&mut self, val: super::vals::Plldiv) {
            self.0 = (self.0 & !(0x7f << 9usize)) | (((val.to_bits() as u32) & 0x7f) << 9usize);
        }
        #[doc = "PLL1 DIVQ division factor Set and reset by software to control the frequency of the PLl1QCLK clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
        #[must_use]
        #[inline(always)]
        pub const fn pllq(&self) -> super::vals::Plldiv {
            let val = (self.0 >> 16usize) & 0x7f;
            super::vals::Plldiv::from_bits(val as u8)
        }
        #[doc = "PLL1 DIVQ division factor Set and reset by software to control the frequency of the PLl1QCLK clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
        #[inline(always)]
        pub const fn set_pllq(&mut self, val: super::vals::Plldiv) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val.to_bits() as u32) & 0x7f) << 16usize);
        }
        #[doc = "PLL1 DIVR division factor Set and reset by software to control the frequency of the pll1rclk clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
        #[must_use]
        #[inline(always)]
        pub const fn pllr(&self) -> super::vals::Plldiv {
            let val = (self.0 >> 24usize) & 0x7f;
            super::vals::Plldiv::from_bits(val as u8)
        }
        #[doc = "PLL1 DIVR division factor Set and reset by software to control the frequency of the pll1rclk clock. These bits can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). ..."]
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
    #[doc = "RCC PLL1 fractional divider register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pll1fracr(pub u32);
    impl Pll1fracr {
        #[doc = "Fractional part of the multiplication factor for PLL1 VCO Set and reset by software to control the fractional part of the multiplication factor of the VCO. These bits can be written at any time, allowing dynamic fine-tuning of the PLL1 VCO. VCO output frequency = F<sub>ref1_ck</sub> x \\[multiplication factor for PLL1 VCO + (PLL1FRACN / 2<sup>13</sup>)\\], with: Multiplication factor for PLL1 VCO must be between 4 and 512. PLL1FRACN can be between 0 and 2<sup>13</sup>- 1. The input frequency F<sub>ref1_ck</sub> must be between 4 and 16 MHz. To change the used fractional value on-the-fly even if the PLL1 is enabled, the application must proceed as follows: Set the bit PLL1FRACEN to 0. Write the new fractional value into PLL1FRACN. Set the bit PLL1FRACEN to 1."]
        #[must_use]
        #[inline(always)]
        pub const fn pllfracn(&self) -> u16 {
            let val = (self.0 >> 3usize) & 0x1fff;
            val as u16
        }
        #[doc = "Fractional part of the multiplication factor for PLL1 VCO Set and reset by software to control the fractional part of the multiplication factor of the VCO. These bits can be written at any time, allowing dynamic fine-tuning of the PLL1 VCO. VCO output frequency = F<sub>ref1_ck</sub> x \\[multiplication factor for PLL1 VCO + (PLL1FRACN / 2<sup>13</sup>)\\], with: Multiplication factor for PLL1 VCO must be between 4 and 512. PLL1FRACN can be between 0 and 2<sup>13</sup>- 1. The input frequency F<sub>ref1_ck</sub> must be between 4 and 16 MHz. To change the used fractional value on-the-fly even if the PLL1 is enabled, the application must proceed as follows: Set the bit PLL1FRACEN to 0. Write the new fractional value into PLL1FRACN. Set the bit PLL1FRACEN to 1."]
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
    #[doc = "RCC privilege configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcfgr(pub u32);
    impl Privcfgr {
        #[doc = "RCC secure functions privilege configuration Set and reset by software. This bit can be written only by a secure privileged access."]
        #[must_use]
        #[inline(always)]
        pub const fn spriv(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RCC secure functions privilege configuration Set and reset by software. This bit can be written only by a secure privileged access."]
        #[inline(always)]
        pub const fn set_spriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RCC non-secure functions privilege configuration Set and reset by software. This bit can be written only by privileged access, secure or non-secure."]
        #[must_use]
        #[inline(always)]
        pub const fn nspriv(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RCC non-secure functions privilege configuration Set and reset by software. This bit can be written only by privileged access, secure or non-secure."]
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
    #[doc = "RCC RADIO peripheral clock enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Radioenr(pub u32);
    impl Radioenr {
        #[doc = "2.4 GHz RADIO baseband kernel clock (aclk) enable Set and cleared by software. Note: The HSE oscillator needs to be enabled by either HSEON or STRADIOCLKON."]
        #[must_use]
        #[inline(always)]
        pub const fn bbclken(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "2.4 GHz RADIO baseband kernel clock (aclk) enable Set and cleared by software. Note: The HSE oscillator needs to be enabled by either HSEON or STRADIOCLKON."]
        #[inline(always)]
        pub const fn set_bbclken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "2.4 GHz RADIO bus clock enable and HSE oscillator enable by 2.4 GHz RADIO sleep timer wakeup event Set by hardware on a 2.4 GHz RADIO sleep timer wakeup event. Cleared by software writing zero to this bit. Note: Before accessing the 2.4 GHz RADIO registers the RADIOCLKRDY bit must be checked."]
        #[must_use]
        #[inline(always)]
        pub const fn stradioclkon(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "2.4 GHz RADIO bus clock enable and HSE oscillator enable by 2.4 GHz RADIO sleep timer wakeup event Set by hardware on a 2.4 GHz RADIO sleep timer wakeup event. Cleared by software writing zero to this bit. Note: Before accessing the 2.4 GHz RADIO registers the RADIOCLKRDY bit must be checked."]
        #[inline(always)]
        pub const fn set_stradioclkon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "2.4 GHz RADIO bus clock ready. Set and cleared by hardware to indicate that the 2.4 GHz RADIO bus clock is ready and the 2.4 GHz RADIO registers can be accessed. Note: Once both RADIOEN and STRADIOCLKON are cleared, RADIOCLKRDY goes low after three hclk5 clock cycles."]
        #[must_use]
        #[inline(always)]
        pub const fn radioclkrdy(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "2.4 GHz RADIO bus clock ready. Set and cleared by hardware to indicate that the 2.4 GHz RADIO bus clock is ready and the 2.4 GHz RADIO registers can be accessed. Note: Once both RADIOEN and STRADIOCLKON are cleared, RADIOCLKRDY goes low after three hclk5 clock cycles."]
        #[inline(always)]
        pub const fn set_radioclkrdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Radioenr {
        #[inline(always)]
        fn default() -> Radioenr {
            Radioenr(0)
        }
    }
    impl core::fmt::Debug for Radioenr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Radioenr")
                .field("bbclken", &self.bbclken())
                .field("stradioclkon", &self.stradioclkon())
                .field("radioclkrdy", &self.radioclkrdy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Radioenr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Radioenr {{ bbclken: {=bool:?}, stradioclkon: {=bool:?}, radioclkrdy: {=bool:?} }}",
                self.bbclken(),
                self.stradioclkon(),
                self.radioclkrdy()
            )
        }
    }
    #[doc = "RCC secure configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccfgr(pub u32);
    impl Seccfgr {
        #[doc = "HSI clock configuration and status bits security Set and reset by software."]
        #[must_use]
        #[inline(always)]
        pub const fn hsisec(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "HSI clock configuration and status bits security Set and reset by software."]
        #[inline(always)]
        pub const fn set_hsisec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "HSE clock configuration bits, status bits and HSECSS security Set and reset by software."]
        #[must_use]
        #[inline(always)]
        pub const fn hsesec(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "HSE clock configuration bits, status bits and HSECSS security Set and reset by software."]
        #[inline(always)]
        pub const fn set_hsesec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LSI clock configuration and status bits security Set and reset by software."]
        #[must_use]
        #[inline(always)]
        pub const fn lsisec(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LSI clock configuration and status bits security Set and reset by software."]
        #[inline(always)]
        pub const fn set_lsisec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "LSE clock configuration and status bits security Set and reset by software."]
        #[must_use]
        #[inline(always)]
        pub const fn lsesec(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "LSE clock configuration and status bits security Set and reset by software."]
        #[inline(always)]
        pub const fn set_lsesec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SYSCLK selection, clock output on MCO configuration security Set and reset by software."]
        #[must_use]
        #[inline(always)]
        pub const fn sysclksec(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCLK selection, clock output on MCO configuration security Set and reset by software."]
        #[inline(always)]
        pub const fn set_sysclksec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "AHBx/APBx prescaler configuration bits security Set and reset by software."]
        #[must_use]
        #[inline(always)]
        pub const fn prescsec(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "AHBx/APBx prescaler configuration bits security Set and reset by software."]
        #[inline(always)]
        pub const fn set_prescsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "PLL1 clock configuration and status bits security Set and reset by software."]
        #[must_use]
        #[inline(always)]
        pub const fn pllsec(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "PLL1 clock configuration and status bits security Set and reset by software."]
        #[inline(always)]
        pub const fn set_pllsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Remove reset flag security Set and reset by software."]
        #[must_use]
        #[inline(always)]
        pub const fn rmvfsec(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Remove reset flag security Set and reset by software."]
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
    impl core::fmt::Debug for Seccfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Seccfgr")
                .field("hsisec", &self.hsisec())
                .field("hsesec", &self.hsesec())
                .field("lsisec", &self.lsisec())
                .field("lsesec", &self.lsesec())
                .field("sysclksec", &self.sysclksec())
                .field("prescsec", &self.prescsec())
                .field("pllsec", &self.pllsec())
                .field("rmvfsec", &self.rmvfsec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Seccfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Seccfgr {{ hsisec: {=bool:?}, hsesec: {=bool:?}, lsisec: {=bool:?}, lsesec: {=bool:?}, sysclksec: {=bool:?}, prescsec: {=bool:?}, pllsec: {=bool:?}, rmvfsec: {=bool:?} }}" , self . hsisec () , self . hsesec () , self . lsisec () , self . lsesec () , self . sysclksec () , self . prescsec () , self . pllsec () , self . rmvfsec ())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Adcsel {
        #[doc = "hclk4 clock selected"]
        Hclk4 = 0x0,
        #[doc = "SYSCLK selected"]
        Sys = 0x01,
        #[doc = "pll1pclk selected"]
        Pll1P = 0x02,
        #[doc = "HSE clock selected"]
        Hse = 0x03,
        #[doc = "HSI clock selected"]
        Hsi = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
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
    pub enum Assel {
        #[doc = "pll1pclk selected."]
        Pll1P = 0x0,
        #[doc = "pll1qclk selected."]
        Pll1Q = 0x01,
    }
    impl Assel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Assel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Assel {
        #[inline(always)]
        fn from(val: u8) -> Assel {
            Assel::from_bits(val)
        }
    }
    impl From<Assel> for u8 {
        #[inline(always)]
        fn from(val: Assel) -> u8 {
            Assel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hdiv5 {
        #[doc = "hclk5 = SYSCLK not divided"]
        Div1 = 0x0,
        #[doc = "hclk5 = SYSCLK divided by 2"]
        Div2 = 0x01,
    }
    impl Hdiv5 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hdiv5 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hdiv5 {
        #[inline(always)]
        fn from(val: u8) -> Hdiv5 {
            Hdiv5::from_bits(val)
        }
    }
    impl From<Hdiv5> for u8 {
        #[inline(always)]
        fn from(val: Hdiv5) -> u8 {
            Hdiv5::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hpre {
        #[doc = "DCLK not divided"]
        Div1 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "hclk = SYSCLK divided by 2"]
        Div2 = 0x04,
        #[doc = "hclk = SYSCLK divided by 4"]
        Div4 = 0x05,
        #[doc = "hclk = SYSCLK divided by 8"]
        Div8 = 0x06,
        #[doc = "hclk = SYSCLK divided by 16"]
        Div16 = 0x07,
    }
    impl Hpre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hpre {
            unsafe { core::mem::transmute(val & 0x07) }
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
    pub enum Hpre5 {
        #[doc = "DCLK not divided"]
        Div1 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "hclk5 = SYSCLK divided by 2"]
        Div2 = 0x04,
        #[doc = "hclk5 = SYSCLK divided by 3"]
        Div3 = 0x05,
        #[doc = "hclk5 = SYSCLK divided by 4"]
        Div4 = 0x06,
        #[doc = "hclk5 = SYSCLK divided by 6"]
        Div6 = 0x07,
    }
    impl Hpre5 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hpre5 {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hpre5 {
        #[inline(always)]
        fn from(val: u8) -> Hpre5 {
            Hpre5::from_bits(val)
        }
    }
    impl From<Hpre5> for u8 {
        #[inline(always)]
        fn from(val: Hpre5) -> u8 {
            Hpre5::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hsepre {
        #[doc = "HSE not divided, SYSCLK = HSE"]
        Div1 = 0x0,
        #[doc = "HSE divided, SYSCLK = HSE/2"]
        Div2 = 0x01,
    }
    impl Hsepre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hsepre {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hsepre {
        #[inline(always)]
        fn from(val: u8) -> Hsepre {
            Hsepre::from_bits(val)
        }
    }
    impl From<Hsepre> for u8 {
        #[inline(always)]
        fn from(val: Hsepre) -> u8 {
            Hsepre::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum I2c1sel {
        #[doc = "pclk1 selected"]
        Pclk1 = 0x0,
        #[doc = "SYSCLK selected"]
        Sys = 0x01,
        #[doc = "HSI selected"]
        Hsi = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl I2c1sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> I2c1sel {
            unsafe { core::mem::transmute(val & 0x03) }
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
    pub enum I2c3sel {
        #[doc = "pclk7 selected"]
        Pclk7 = 0x0,
        #[doc = "SYSCLK selected"]
        Sys = 0x01,
        #[doc = "HSI selected"]
        Hsi = 0x02,
        _RESERVED_3 = 0x03,
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
    pub enum Lptim1sel {
        #[doc = "pclk7 selected."]
        Pclk7 = 0x0,
        #[doc = "LSI selected"]
        Lsi = 0x01,
        #[doc = "HSI selected"]
        Hsi = 0x02,
        #[doc = "LSE selected"]
        Lse = 0x03,
    }
    impl Lptim1sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lptim1sel {
            unsafe { core::mem::transmute(val & 0x03) }
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
        #[doc = "pclk7 selected."]
        Pclk1 = 0x0,
        #[doc = "LSI selected"]
        Lsi = 0x01,
        #[doc = "HSI selected"]
        Hsi = 0x02,
        #[doc = "LSE selected"]
        Lse = 0x03,
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
    pub enum Lpuartsel {
        #[doc = "pclk7 selected"]
        Pclk7 = 0x0,
        #[doc = "SYSCLK selected"]
        Sys = 0x01,
        #[doc = "HSI selected"]
        Hsi = 0x02,
        #[doc = "LSE selected"]
        Lse = 0x03,
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
    pub enum Lsetrim {
        #[doc = "current source resistance 5/4 x R"]
        R54 = 0x0,
        #[doc = "current source resistance R"]
        R = 0x01,
        #[doc = "current source resistance 3/4 x R"]
        R34 = 0x02,
        #[doc = "current source resistance 2/3 x R"]
        R23 = 0x03,
    }
    impl Lsetrim {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lsetrim {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lsetrim {
        #[inline(always)]
        fn from(val: u8) -> Lsetrim {
            Lsetrim::from_bits(val)
        }
    }
    impl From<Lsetrim> for u8 {
        #[inline(always)]
        fn from(val: Lsetrim) -> u8 {
            Lsetrim::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lsi2cfg {
        #[doc = "LSI2 frequency temperature sensitivity is close to 0 at +80 less thansup oless than/sup C."]
        Sensitivity0at80c = 0x0,
        #[doc = "LSI2 frequency temperature sensitivity is close to 0 at +50 less thansup oless than/sup C."]
        Sensitivity0at50c = 0x01,
        #[doc = "LSI2 frequency temperature sensitivity is close to 0 at +20 less thansup oless than/sup C."]
        Sensitivity0at20c = 0x02,
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
    }
    impl Lsi2cfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lsi2cfg {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lsi2cfg {
        #[inline(always)]
        fn from(val: u8) -> Lsi2cfg {
            Lsi2cfg::from_bits(val)
        }
    }
    impl From<Lsi2cfg> for u8 {
        #[inline(always)]
        fn from(val: Lsi2cfg) -> u8 {
            Lsi2cfg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lsi2mode {
        #[doc = "nominal-power, high accuracy."]
        NominalPower = 0x0,
        #[doc = "low-power, medium accuracy."]
        LowPower = 0x01,
        #[doc = "ultra-low-power, low accuracy."]
        UltraLowPower = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Lsi2mode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lsi2mode {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lsi2mode {
        #[inline(always)]
        fn from(val: u8) -> Lsi2mode {
            Lsi2mode::from_bits(val)
        }
    }
    impl From<Lsi2mode> for u8 {
        #[inline(always)]
        fn from(val: Lsi2mode) -> u8 {
            Lsi2mode::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lsiprediv {
        #[doc = "LSI not divided"]
        Div1 = 0x0,
        #[doc = "LSI divided by 128"]
        Div128 = 0x01,
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
    pub enum Mcopre {
        #[doc = "MCO divided by 1"]
        Div1 = 0x0,
        #[doc = "MCO divided by 2"]
        Div2 = 0x01,
        #[doc = "MCO divided by 4"]
        Div4 = 0x02,
        #[doc = "MCO divided by 8"]
        Div8 = 0x03,
        #[doc = "MCO divided by 16"]
        Div16 = 0x04,
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
        Disabled = 0x0,
        #[doc = "sysclkpre system clock after PLL1RCLKPRE division selected"]
        Sysclkpre = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "HSI clock selected"]
        Hsi = 0x03,
        #[doc = "HSE clock selected"]
        Hse = 0x04,
        #[doc = "pll1rclk clock selected"]
        Pll1R = 0x05,
        #[doc = "LSI clock selected"]
        Lsi = 0x06,
        #[doc = "LSE clock selected"]
        Lse = 0x07,
        #[doc = "pll1pclk clock selected"]
        Pll1P = 0x08,
        #[doc = "pll1qclk clock selected"]
        Pll1Q = 0x09,
        #[doc = "hclk5 clock selected"]
        Hclk5 = 0x0a,
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
    pub enum Otghssel {
        #[doc = "HSE32 selected."]
        Hse = 0x0,
        #[doc = "pll1pclk selected,."]
        Pll1P = 0x01,
        #[doc = "HSE32 divided by 2 selected."]
        HseDiv2 = 0x02,
        #[doc = "pll1pclk divided by 2 selected."]
        Pll1PDiv2 = 0x03,
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
        Div1 = 0x0,
        Div2 = 0x01,
        Div3 = 0x02,
        Div4 = 0x03,
        Div5 = 0x04,
        Div6 = 0x05,
        Div7 = 0x06,
        Div8 = 0x07,
    }
    impl Pllm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllm {
            unsafe { core::mem::transmute(val & 0x07) }
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
    pub enum Pllrclkpre {
        #[doc = "pll1rclk not divided, sysclkpre = pll1rclk"]
        Div1 = 0x0,
        #[doc = "pll1rclk divided, sysclkpre = pll1rclk divided"]
        Divided = 0x01,
    }
    impl Pllrclkpre {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllrclkpre {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllrclkpre {
        #[inline(always)]
        fn from(val: u8) -> Pllrclkpre {
            Pllrclkpre::from_bits(val)
        }
    }
    impl From<Pllrclkpre> for u8 {
        #[inline(always)]
        fn from(val: Pllrclkpre) -> u8 {
            Pllrclkpre::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllrclkprestep {
        #[doc = "pll1rclk 2-step division"]
        Step2 = 0x0,
        #[doc = "pll1rclk 3-step division"]
        Step3 = 0x01,
    }
    impl Pllrclkprestep {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pllrclkprestep {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pllrclkprestep {
        #[inline(always)]
        fn from(val: u8) -> Pllrclkprestep {
            Pllrclkprestep::from_bits(val)
        }
    }
    impl From<Pllrclkprestep> for u8 {
        #[inline(always)]
        fn from(val: Pllrclkprestep) -> u8 {
            Pllrclkprestep::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pllrge {
        #[doc = "PLL2 input (ref2_ck) clock range frequency between 4 and 8 MHz"]
        Freq4to8mhz = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "PLL2 input (ref2_ck) clock range frequency between 8 and 16 MHz"]
        Freq8to16mhz = 0x03,
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
        #[doc = "no clock sent to PLL1"]
        Disable = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "HSI clock selected as PLL1 clock entry"]
        Hsi = 0x02,
        #[doc = "HSE clock after HSEPRE divider selected as PLL1 clock entry"]
        Hse = 0x03,
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
    pub enum Radiostsel {
        #[doc = "no clock selected, 2.4 GHz RADIO sleep timer kernel clock disabled"]
        Disable = 0x0,
        #[doc = "LSE oscillator clock selected"]
        Lse = 0x01,
        #[doc = "LSI oscillator clock selected"]
        Lsi = 0x02,
        #[doc = "HSE oscillator clock divided by 1000 selected"]
        Hse = 0x03,
    }
    impl Radiostsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Radiostsel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Radiostsel {
        #[inline(always)]
        fn from(val: u8) -> Radiostsel {
            Radiostsel::from_bits(val)
        }
    }
    impl From<Radiostsel> for u8 {
        #[inline(always)]
        fn from(val: Radiostsel) -> u8 {
            Radiostsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rngsel {
        #[doc = "LSE selected"]
        Lse = 0x0,
        #[doc = "LSI selected"]
        Lsi = 0x01,
        #[doc = "HSI selected"]
        Hsi = 0x02,
        #[doc = "pll1qclk divide by 2 selected"]
        Pll1Q = 0x03,
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
        #[doc = "no clock selected, RTC and TAMP kernel clock disabled"]
        Disable = 0x0,
        #[doc = "LSE oscillator clock selected, and enabled"]
        Lse = 0x01,
        #[doc = "LSI oscillator clock selected, and enabled"]
        Lsi = 0x02,
        #[doc = "HSE oscillator clock divided by 32 selected, and enabled"]
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
    pub enum Sai1sel {
        #[doc = "pll1pclk selected."]
        Pll1P = 0x0,
        #[doc = "pll1qclk selected."]
        Pll1Q = 0x01,
        #[doc = "SYSCLK selected."]
        Sys = 0x02,
        #[doc = "input pin AUDIOCLK selected."]
        Audioclk = 0x03,
        #[doc = "HSI16 clock selected."]
        Hsi = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Sai1sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sai1sel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sai1sel {
        #[inline(always)]
        fn from(val: u8) -> Sai1sel {
            Sai1sel::from_bits(val)
        }
    }
    impl From<Sai1sel> for u8 {
        #[inline(always)]
        fn from(val: Sai1sel) -> u8 {
            Sai1sel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Spi1sel {
        #[doc = "pclk2 selected"]
        Pclk2 = 0x0,
        #[doc = "SYSCLK selected"]
        Sys = 0x01,
        #[doc = "HSI selected"]
        Hsi = 0x02,
        _RESERVED_3 = 0x03,
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
        #[doc = "pclk1 selected"]
        Pclk1 = 0x0,
        #[doc = "SYSCLK selected"]
        Sys = 0x01,
        #[doc = "HSI selected"]
        Hsi = 0x02,
        _RESERVED_3 = 0x03,
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
        #[doc = "pclk7 selected"]
        Pclk7 = 0x0,
        #[doc = "SYSCLK selected"]
        Sys = 0x01,
        #[doc = "HSI selected"]
        Hsi = 0x02,
        _RESERVED_3 = 0x03,
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
    pub enum Sw {
        #[doc = "HSI selected as system clock"]
        Hsi = 0x0,
        _RESERVED_1 = 0x01,
        #[doc = "HSE or HSE/2, as defined by HSEPRE, selected as system clock"]
        Hse = 0x02,
        #[doc = "pll1rclk selected as system clock"]
        Pll1R = 0x03,
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
        #[doc = "hclk1 divided by 8 selected"]
        Hclk1Div8 = 0x0,
        #[doc = "LSI selected"]
        Lsi = 0x01,
        #[doc = "LSE selected"]
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
    pub enum Timicsel {
        #[doc = "HSI divider disabled"]
        Hsi = 0x0,
        #[doc = "HSI/256 generated and can be selected by TIM16, TIM17 and LPTIM2 as internal input capture"]
        HsiDiv256 = 0x01,
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
    pub enum Usart1sel {
        #[doc = "pclk2 selected"]
        Pclk2 = 0x0,
        #[doc = "SYSCLK selected"]
        Sys = 0x01,
        #[doc = "HSI selected"]
        Hsi = 0x02,
        #[doc = "LSE selected"]
        Lse = 0x03,
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
        #[doc = "pclk1 selected"]
        Pclk1 = 0x0,
        #[doc = "SYSCLK selected"]
        Sys = 0x01,
        #[doc = "HSI selected"]
        Hsi = 0x02,
        #[doc = "LSE selected"]
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
}
