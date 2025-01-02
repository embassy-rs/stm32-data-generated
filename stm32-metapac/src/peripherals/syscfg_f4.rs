#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "System configuration controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syscfg {
    ptr: *mut u8,
}
unsafe impl Send for Syscfg {}
unsafe impl Sync for Syscfg {}
impl Syscfg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "memory remap register"]
    #[inline(always)]
    pub const fn memrm(self) -> crate::common::Reg<regs::Memrm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "peripheral mode configuration register"]
    #[inline(always)]
    pub const fn pmc(self) -> crate::common::Reg<regs::Pmc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "external interrupt configuration register"]
    #[inline(always)]
    pub const fn exticr(self, n: usize) -> crate::common::Reg<regs::Exticr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 4usize) as _) }
    }
    #[doc = "Compensation cell control register"]
    #[inline(always)]
    pub const fn cmpcr(self) -> crate::common::Reg<regs::Cmpcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
}
pub mod regs {
    #[doc = "Compensation cell control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmpcr(pub u32);
    impl Cmpcr {
        #[doc = "Compensation cell power-down"]
        #[inline(always)]
        pub const fn cmp_pd(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Compensation cell power-down"]
        #[inline(always)]
        pub fn set_cmp_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "READY"]
        #[inline(always)]
        pub const fn ready(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "READY"]
        #[inline(always)]
        pub fn set_ready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Cmpcr {
        #[inline(always)]
        fn default() -> Cmpcr {
            Cmpcr(0)
        }
    }
    impl core::fmt::Debug for Cmpcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmpcr")
                .field("cmp_pd", &self.cmp_pd())
                .field("ready", &self.ready())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmpcr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cmpcr {
                cmp_pd: bool,
                ready: bool,
            }
            let proxy = Cmpcr {
                cmp_pd: self.cmp_pd(),
                ready: self.ready(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "external interrupt configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exticr(pub u32);
    impl Exticr {
        #[doc = "EXTI x configuration"]
        #[inline(always)]
        pub const fn exti(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "EXTI x configuration"]
        #[inline(always)]
        pub fn set_exti(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
    }
    impl Default for Exticr {
        #[inline(always)]
        fn default() -> Exticr {
            Exticr(0)
        }
    }
    impl core::fmt::Debug for Exticr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Exticr")
                .field(
                    "exti",
                    &[
                        self.exti(0usize),
                        self.exti(1usize),
                        self.exti(2usize),
                        self.exti(3usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Exticr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Exticr {
                exti: [u8; 4usize],
            }
            let proxy = Exticr {
                exti: [
                    self.exti(0usize),
                    self.exti(1usize),
                    self.exti(2usize),
                    self.exti(3usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "memory remap register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Memrm(pub u32);
    impl Memrm {
        #[doc = "Memory mapping selection"]
        #[inline(always)]
        pub const fn mem_mode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Memory mapping selection"]
        #[inline(always)]
        pub fn set_mem_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Flash bank mode selection"]
        #[inline(always)]
        pub const fn fb_mode(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Flash bank mode selection"]
        #[inline(always)]
        pub fn set_fb_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "FMC memory mapping swap"]
        #[inline(always)]
        pub const fn swp_fmc(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "FMC memory mapping swap"]
        #[inline(always)]
        pub fn set_swp_fmc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
    }
    impl Default for Memrm {
        #[inline(always)]
        fn default() -> Memrm {
            Memrm(0)
        }
    }
    impl core::fmt::Debug for Memrm {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Memrm")
                .field("mem_mode", &self.mem_mode())
                .field("fb_mode", &self.fb_mode())
                .field("swp_fmc", &self.swp_fmc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Memrm {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Memrm {
                mem_mode: u8,
                fb_mode: bool,
                swp_fmc: u8,
            }
            let proxy = Memrm {
                mem_mode: self.mem_mode(),
                fb_mode: self.fb_mode(),
                swp_fmc: self.swp_fmc(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "peripheral mode configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pmc(pub u32);
    impl Pmc {
        #[doc = "ADC1DC2"]
        #[inline(always)]
        pub const fn adc1dc2(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "ADC1DC2"]
        #[inline(always)]
        pub fn set_adc1dc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "ADC2DC2"]
        #[inline(always)]
        pub const fn adc2dc2(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "ADC2DC2"]
        #[inline(always)]
        pub fn set_adc2dc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "ADC3DC2"]
        #[inline(always)]
        pub const fn adc3dc2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "ADC3DC2"]
        #[inline(always)]
        pub fn set_adc3dc2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Ethernet PHY interface selection"]
        #[inline(always)]
        pub const fn mii_rmii_sel(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet PHY interface selection"]
        #[inline(always)]
        pub fn set_mii_rmii_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Pmc {
        #[inline(always)]
        fn default() -> Pmc {
            Pmc(0)
        }
    }
    impl core::fmt::Debug for Pmc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pmc")
                .field("adc1dc2", &self.adc1dc2())
                .field("adc2dc2", &self.adc2dc2())
                .field("adc3dc2", &self.adc3dc2())
                .field("mii_rmii_sel", &self.mii_rmii_sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pmc {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Pmc {
                adc1dc2: bool,
                adc2dc2: bool,
                adc3dc2: bool,
                mii_rmii_sel: bool,
            }
            let proxy = Pmc {
                adc1dc2: self.adc1dc2(),
                adc2dc2: self.adc2dc2(),
                adc3dc2: self.adc3dc2(),
                mii_rmii_sel: self.mii_rmii_sel(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
