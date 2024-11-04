#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Microcontroller debug unit"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgmcu {
    ptr: *mut u8,
}
unsafe impl Send for Dbgmcu {}
unsafe impl Sync for Dbgmcu {}
impl Dbgmcu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "identity code register"]
    #[inline(always)]
    pub const fn idcode(self) -> crate::common::Reg<regs::Idcode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "status and configuration register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "APB1L peripheral freeze register"]
    #[inline(always)]
    pub const fn apb1lfzr(self) -> crate::common::Reg<regs::Apb1lfzr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "APB1H peripheral freeze register"]
    #[inline(always)]
    pub const fn apb1hfzr(self) -> crate::common::Reg<regs::Apb1hfzr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "APB2 peripheral freeze register"]
    #[inline(always)]
    pub const fn apb2fzr(self) -> crate::common::Reg<regs::Apb2fzr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "APB7 peripheral freeze register"]
    #[inline(always)]
    pub const fn apb7fzr(self) -> crate::common::Reg<regs::Apb7fzr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "AHB1 peripheral freeze register"]
    #[inline(always)]
    pub const fn ahb1fzr(self) -> crate::common::Reg<regs::Ahb1fzr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
    #[doc = "debug host authentication register"]
    #[inline(always)]
    pub const fn dbg_auth_host(self) -> crate::common::Reg<regs::DbgAuthHost, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "debug device authentication register"]
    #[inline(always)]
    pub const fn dbg_auth_device(self) -> crate::common::Reg<regs::DbgAuthDevice, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "part number codification register"]
    #[inline(always)]
    pub const fn pncr(self) -> crate::common::Reg<regs::Pncr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07dcusize) as _) }
    }
    #[doc = "CoreSight peripheral identity register 4"]
    #[inline(always)]
    pub const fn pidr4(self) -> crate::common::Reg<regs::Pidr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fd0usize) as _) }
    }
    #[doc = "CoreSight peripheral identity register 0"]
    #[inline(always)]
    pub const fn pidr0(self) -> crate::common::Reg<regs::Pidr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fe0usize) as _) }
    }
    #[doc = "CoreSight peripheral identity register 1"]
    #[inline(always)]
    pub const fn pidr1(self) -> crate::common::Reg<regs::Pidr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fe4usize) as _) }
    }
    #[doc = "CoreSight peripheral identity register 2"]
    #[inline(always)]
    pub const fn pidr2(self) -> crate::common::Reg<regs::Pidr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fe8usize) as _) }
    }
    #[doc = "CoreSight peripheral identity register 3"]
    #[inline(always)]
    pub const fn pidr3(self) -> crate::common::Reg<regs::Pidr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0fecusize) as _) }
    }
    #[doc = "CoreSight component identity register 0"]
    #[inline(always)]
    pub const fn cidr0(self) -> crate::common::Reg<regs::Cidr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ff0usize) as _) }
    }
    #[doc = "CoreSight peripheral identity register 1"]
    #[inline(always)]
    pub const fn cidr1(self) -> crate::common::Reg<regs::Cidr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ff4usize) as _) }
    }
    #[doc = "CoreSight component identity register 2"]
    #[inline(always)]
    pub const fn cidr2(self) -> crate::common::Reg<regs::Cidr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ff8usize) as _) }
    }
    #[doc = "CoreSight component identity register 3"]
    #[inline(always)]
    pub const fn cidr3(self) -> crate::common::Reg<regs::Cidr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ffcusize) as _) }
    }
}
pub mod regs {
    #[doc = "AHB1 peripheral freeze register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1fzr(pub u32);
    impl Ahb1fzr {
        #[doc = "GPDMA 1 channel 0 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC0."]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch0_stop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA 1 channel 0 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC0."]
        #[inline(always)]
        pub fn set_dbg_gpdma1_ch0_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "GPDMA 1 channel 1 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC1."]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch1_stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA 1 channel 1 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC1."]
        #[inline(always)]
        pub fn set_dbg_gpdma1_ch1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "GPDMA 1 channel 2 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC2."]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch2_stop(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA 1 channel 2 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC2."]
        #[inline(always)]
        pub fn set_dbg_gpdma1_ch2_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "GPDMA 1 channel 3 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC3."]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch3_stop(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA 1 channel 3 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC3."]
        #[inline(always)]
        pub fn set_dbg_gpdma1_ch3_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "GPDMA 1 channel 4 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC4."]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch4_stop(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA 1 channel 4 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC4."]
        #[inline(always)]
        pub fn set_dbg_gpdma1_ch4_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "GPDMA 1 channel 5 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC5."]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch5_stop(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA 1 channel 5 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC5."]
        #[inline(always)]
        pub fn set_dbg_gpdma1_ch5_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "GPDMA 1 channel 6 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC6."]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch6_stop(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA 1 channel 6 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC6."]
        #[inline(always)]
        pub fn set_dbg_gpdma1_ch6_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "GPDMA 1 channel 7 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC7."]
        #[inline(always)]
        pub const fn dbg_gpdma1_ch7_stop(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "GPDMA 1 channel 7 stop in CPU debug Write access can be protected by GPDMA_SECCFGR.SEC7."]
        #[inline(always)]
        pub fn set_dbg_gpdma1_ch7_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Ahb1fzr {
        #[inline(always)]
        fn default() -> Ahb1fzr {
            Ahb1fzr(0)
        }
    }
    #[doc = "APB1H peripheral freeze register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1hfzr(pub u32);
    impl Apb1hfzr {
        #[doc = "LPTIM2 stop in CPU debug Write access can be protected by GTZC_TZSC.LPTIM2SEC."]
        #[inline(always)]
        pub const fn dbg_lptim2_stop(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM2 stop in CPU debug Write access can be protected by GTZC_TZSC.LPTIM2SEC."]
        #[inline(always)]
        pub fn set_dbg_lptim2_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Apb1hfzr {
        #[inline(always)]
        fn default() -> Apb1hfzr {
            Apb1hfzr(0)
        }
    }
    #[doc = "APB1L peripheral freeze register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1lfzr(pub u32);
    impl Apb1lfzr {
        #[doc = "TIM2 stop in CPU debug Write access can be protected by GTZC_TZSC.TIM2SEC."]
        #[inline(always)]
        pub const fn dbg_tim2_stop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 stop in CPU debug Write access can be protected by GTZC_TZSC.TIM2SEC."]
        #[inline(always)]
        pub fn set_dbg_tim2_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 stop in CPU debug Write access can be protected by GTZC_TZSC.TIM3SEC."]
        #[inline(always)]
        pub const fn dbg_tim3_stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 stop in CPU debug Write access can be protected by GTZC_TZSC.TIM3SEC."]
        #[inline(always)]
        pub fn set_dbg_tim3_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "WWDG stop in CPU debug Write access can be protected by GTZC_TZSC.WWDGSEC"]
        #[inline(always)]
        pub const fn dbg_wwdg_stop(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG stop in CPU debug Write access can be protected by GTZC_TZSC.WWDGSEC"]
        #[inline(always)]
        pub fn set_dbg_wwdg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "IWDG stop in CPU debug Write access can be protected by GTZC_TZSC.IWDGSEC."]
        #[inline(always)]
        pub const fn dbg_iwdg_stop(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "IWDG stop in CPU debug Write access can be protected by GTZC_TZSC.IWDGSEC."]
        #[inline(always)]
        pub fn set_dbg_iwdg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "I2C1 SMBUS timeout stop in CPU debug Write access can be protected by GTZC_TZSC.I2C1SEC."]
        #[inline(always)]
        pub const fn dbg_i2c1_stop(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 SMBUS timeout stop in CPU debug Write access can be protected by GTZC_TZSC.I2C1SEC."]
        #[inline(always)]
        pub fn set_dbg_i2c1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Apb1lfzr {
        #[inline(always)]
        fn default() -> Apb1lfzr {
            Apb1lfzr(0)
        }
    }
    #[doc = "APB2 peripheral freeze register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2fzr(pub u32);
    impl Apb2fzr {
        #[doc = "TIM1 stop in CPU debug Write access can be protected by GTZC_TZSC.TIM1SEC."]
        #[inline(always)]
        pub const fn dbg_tim1_stop(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 stop in CPU debug Write access can be protected by GTZC_TZSC.TIM1SEC."]
        #[inline(always)]
        pub fn set_dbg_tim1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "TIM16 stop in CPU debug Write access can be protected by GTZC_TZSC.TIM16SEC."]
        #[inline(always)]
        pub const fn dbg_tim16_stop(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 stop in CPU debug Write access can be protected by GTZC_TZSC.TIM16SEC."]
        #[inline(always)]
        pub fn set_dbg_tim16_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 stop in CPU debug Write access can be protected by GTZC_TZSC.TIM17SEC."]
        #[inline(always)]
        pub const fn dbg_tim17_stop(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 stop in CPU debug Write access can be protected by GTZC_TZSC.TIM17SEC."]
        #[inline(always)]
        pub fn set_dbg_tim17_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for Apb2fzr {
        #[inline(always)]
        fn default() -> Apb2fzr {
            Apb2fzr(0)
        }
    }
    #[doc = "APB7 peripheral freeze register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb7fzr(pub u32);
    impl Apb7fzr {
        #[doc = "I2C3 stop in CPU debug Access can be protected by GTZC_TZSC.I2C3SEC."]
        #[inline(always)]
        pub const fn dbg_i2c3_stop(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 stop in CPU debug Access can be protected by GTZC_TZSC.I2C3SEC."]
        #[inline(always)]
        pub fn set_dbg_i2c3_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LPTIM1 stop in CPU debug Access can be protected by GTZC_TZSC.LPTIM1SEC."]
        #[inline(always)]
        pub const fn dbg_lptim1_stop(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 stop in CPU debug Access can be protected by GTZC_TZSC.LPTIM1SEC."]
        #[inline(always)]
        pub fn set_dbg_lptim1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "RTC stop in CPU debug Access can be protected by GTZC_TZSC.TIM17SEC. Can only be accessed secure when one or more features in the RTC or TAMP is/are secure."]
        #[inline(always)]
        pub const fn dbg_rtc_stop(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "RTC stop in CPU debug Access can be protected by GTZC_TZSC.TIM17SEC. Can only be accessed secure when one or more features in the RTC or TAMP is/are secure."]
        #[inline(always)]
        pub fn set_dbg_rtc_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Apb7fzr {
        #[inline(always)]
        fn default() -> Apb7fzr {
            Apb7fzr(0)
        }
    }
    #[doc = "CoreSight component identity register 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cidr0(pub u32);
    impl Cidr0 {
        #[doc = "Component ID bits \\[7:0\\]"]
        #[inline(always)]
        pub const fn preamble(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Component ID bits \\[7:0\\]"]
        #[inline(always)]
        pub fn set_preamble(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Cidr0 {
        #[inline(always)]
        fn default() -> Cidr0 {
            Cidr0(0)
        }
    }
    #[doc = "CoreSight peripheral identity register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cidr1(pub u32);
    impl Cidr1 {
        #[doc = "Component ID bits \\[11:8\\]"]
        #[inline(always)]
        pub const fn preamble(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Component ID bits \\[11:8\\]"]
        #[inline(always)]
        pub fn set_preamble(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Component ID bits \\[15:12\\]
- component class"]
        #[inline(always)]
        pub const fn class(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Component ID bits \\[15:12\\]
- component class"]
        #[inline(always)]
        pub fn set_class(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Cidr1 {
        #[inline(always)]
        fn default() -> Cidr1 {
            Cidr1(0)
        }
    }
    #[doc = "CoreSight component identity register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cidr2(pub u32);
    impl Cidr2 {
        #[doc = "Component ID bits \\[23:16\\]"]
        #[inline(always)]
        pub const fn preamble(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Component ID bits \\[23:16\\]"]
        #[inline(always)]
        pub fn set_preamble(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Cidr2 {
        #[inline(always)]
        fn default() -> Cidr2 {
            Cidr2(0)
        }
    }
    #[doc = "CoreSight component identity register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cidr3(pub u32);
    impl Cidr3 {
        #[doc = "Component ID bits \\[31:24\\]"]
        #[inline(always)]
        pub const fn preamble(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Component ID bits \\[31:24\\]"]
        #[inline(always)]
        pub fn set_preamble(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Cidr3 {
        #[inline(always)]
        fn default() -> Cidr3 {
            Cidr3(0)
        }
    }
    #[doc = "status and configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Allows debug in Stop mode Write access can be protected by PWR_SECCFGR.LPMSEC. The CPU debug and clocks remain active and the HSI oscillators is used as system clock during Stop debug mode, allowing CPU debug capability. On exit from Stop mode, the clock settings are set to the Stop mode exit state."]
        #[inline(always)]
        pub const fn dbg_stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Allows debug in Stop mode Write access can be protected by PWR_SECCFGR.LPMSEC. The CPU debug and clocks remain active and the HSI oscillators is used as system clock during Stop debug mode, allowing CPU debug capability. On exit from Stop mode, the clock settings are set to the Stop mode exit state."]
        #[inline(always)]
        pub fn set_dbg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Allows debug in Standby mode Write access can be protected by PWR_SECCFGR.LPMSEC. The CPU debug and clocks remain active and the HSI oscillator is used as system clock, the supply and SRAM memory content is maintained during Standby debug mode, allowing CPU debug capability. On exit from Standby mode, a standby reset is performed."]
        #[inline(always)]
        pub const fn dbg_standby(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Allows debug in Standby mode Write access can be protected by PWR_SECCFGR.LPMSEC. The CPU debug and clocks remain active and the HSI oscillator is used as system clock, the supply and SRAM memory content is maintained during Standby debug mode, allowing CPU debug capability. On exit from Standby mode, a standby reset is performed."]
        #[inline(always)]
        pub fn set_dbg_standby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Device low power mode selected 10x: Standby mode others reserved"]
        #[inline(always)]
        pub const fn lpms(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Device low power mode selected 10x: Standby mode others reserved"]
        #[inline(always)]
        pub fn set_lpms(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "Device Stop flag"]
        #[inline(always)]
        pub const fn stopf(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Device Stop flag"]
        #[inline(always)]
        pub fn set_stopf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Device Standby flag"]
        #[inline(always)]
        pub const fn sbf(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Device Standby flag"]
        #[inline(always)]
        pub fn set_sbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "CPU Sleep"]
        #[inline(always)]
        pub const fn cs(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "CPU Sleep"]
        #[inline(always)]
        pub fn set_cs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "CPU DeepSleep"]
        #[inline(always)]
        pub const fn cds(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "CPU DeepSleep"]
        #[inline(always)]
        pub fn set_cds(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    #[doc = "debug device authentication register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgAuthDevice(pub u32);
    impl DbgAuthDevice {
        #[doc = "Device specific ID Device specific ID used for RDP regression."]
        #[inline(always)]
        pub const fn auth_id(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Device specific ID Device specific ID used for RDP regression."]
        #[inline(always)]
        pub fn set_auth_id(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DbgAuthDevice {
        #[inline(always)]
        fn default() -> DbgAuthDevice {
            DbgAuthDevice(0)
        }
    }
    #[doc = "debug host authentication register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgAuthHost(pub u32);
    impl DbgAuthHost {
        #[doc = "Device authentication key The device specific 64-bit authentication key (OEMn key) must be written to this register (in two successive 32-bit writes, least significant word first) to permit RDP regression. Writing a wrong key locks access to the device and prevent code execution from the Flash memory."]
        #[inline(always)]
        pub const fn auth_key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Device authentication key The device specific 64-bit authentication key (OEMn key) must be written to this register (in two successive 32-bit writes, least significant word first) to permit RDP regression. Writing a wrong key locks access to the device and prevent code execution from the Flash memory."]
        #[inline(always)]
        pub fn set_auth_key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DbgAuthHost {
        #[inline(always)]
        fn default() -> DbgAuthHost {
            DbgAuthHost(0)
        }
    }
    #[doc = "identity code register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idcode(pub u32);
    impl Idcode {
        #[doc = "Device ID"]
        #[inline(always)]
        pub const fn dev_id(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Device ID"]
        #[inline(always)]
        pub fn set_dev_id(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Revision ID"]
        #[inline(always)]
        pub const fn rev_id(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Revision ID"]
        #[inline(always)]
        pub fn set_rev_id(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Idcode {
        #[inline(always)]
        fn default() -> Idcode {
            Idcode(0)
        }
    }
    #[doc = "CoreSight peripheral identity register 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pidr0(pub u32);
    impl Pidr0 {
        #[doc = "Part number bits \\[7:0\\]"]
        #[inline(always)]
        pub const fn partnum(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Part number bits \\[7:0\\]"]
        #[inline(always)]
        pub fn set_partnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Pidr0 {
        #[inline(always)]
        fn default() -> Pidr0 {
            Pidr0(0)
        }
    }
    #[doc = "CoreSight peripheral identity register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pidr1(pub u32);
    impl Pidr1 {
        #[doc = "Part number bits \\[11:8\\]"]
        #[inline(always)]
        pub const fn partnum(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Part number bits \\[11:8\\]"]
        #[inline(always)]
        pub fn set_partnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "JEP106 identity code bits \\[3:0\\]"]
        #[inline(always)]
        pub const fn jep106id(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "JEP106 identity code bits \\[3:0\\]"]
        #[inline(always)]
        pub fn set_jep106id(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Pidr1 {
        #[inline(always)]
        fn default() -> Pidr1 {
            Pidr1(0)
        }
    }
    #[doc = "CoreSight peripheral identity register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pidr2(pub u32);
    impl Pidr2 {
        #[doc = "JEP106 identity code bits \\[6:4\\]"]
        #[inline(always)]
        pub const fn jep106id(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "JEP106 identity code bits \\[6:4\\]"]
        #[inline(always)]
        pub fn set_jep106id(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "JEDEC assigned value"]
        #[inline(always)]
        pub const fn jedec(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "JEDEC assigned value"]
        #[inline(always)]
        pub fn set_jedec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Component revision number"]
        #[inline(always)]
        pub const fn revision(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Component revision number"]
        #[inline(always)]
        pub fn set_revision(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Pidr2 {
        #[inline(always)]
        fn default() -> Pidr2 {
            Pidr2(0)
        }
    }
    #[doc = "CoreSight peripheral identity register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pidr3(pub u32);
    impl Pidr3 {
        #[doc = "Customer modified"]
        #[inline(always)]
        pub const fn cmod(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Customer modified"]
        #[inline(always)]
        pub fn set_cmod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Metal fix version"]
        #[inline(always)]
        pub const fn revand(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Metal fix version"]
        #[inline(always)]
        pub fn set_revand(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Pidr3 {
        #[inline(always)]
        fn default() -> Pidr3 {
            Pidr3(0)
        }
    }
    #[doc = "CoreSight peripheral identity register 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pidr4(pub u32);
    impl Pidr4 {
        #[doc = "JEP106 continuation code"]
        #[inline(always)]
        pub const fn jep106con(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "JEP106 continuation code"]
        #[inline(always)]
        pub fn set_jep106con(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Register file size"]
        #[inline(always)]
        pub const fn f4kcount(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Register file size"]
        #[inline(always)]
        pub fn set_f4kcount(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Pidr4 {
        #[inline(always)]
        fn default() -> Pidr4 {
            Pidr4(0)
        }
    }
    #[doc = "part number codification register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pncr(pub u32);
    impl Pncr {
        #[doc = "Part number codification"]
        #[inline(always)]
        pub const fn codification(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Part number codification"]
        #[inline(always)]
        pub fn set_codification(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Pncr {
        #[inline(always)]
        fn default() -> Pncr {
            Pncr(0)
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Bit n identifies whether access port APn is present in device Bit n�=�0: APn absent Bit n�=�1: APn present"]
        #[inline(always)]
        pub const fn ap_present(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Bit n identifies whether access port APn is present in device Bit n�=�0: APn absent Bit n�=�1: APn present"]
        #[inline(always)]
        pub fn set_ap_present(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Bit n identifies whether access port APn is open (can be accessed via the debug port) or locked (debug access to the APn is blocked, except for access) Bit n�=�0: APn locked (except for access to DBGMCU) Bit n�=�1: APn enabled"]
        #[inline(always)]
        pub const fn ap_enabled(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Bit n identifies whether access port APn is open (can be accessed via the debug port) or locked (debug access to the APn is blocked, except for access) Bit n�=�0: APn locked (except for access to DBGMCU) Bit n�=�1: APn enabled"]
        #[inline(always)]
        pub fn set_ap_enabled(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
}
