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
    #[doc = "configuration register 1"]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::Cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "CFGR2"]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "external interrupt configuration register"]
    #[inline(always)]
    pub const fn exticr(self, n: usize) -> crate::common::Reg<regs::Exticr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 4usize) as _) }
    }
    #[doc = "CFGR3"]
    #[inline(always)]
    pub const fn cfgr3(self) -> crate::common::Reg<regs::Cfgr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
}
pub mod regs {
    #[doc = "configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr1(pub u32);
    impl Cfgr1 {
        #[doc = "Memory mapping selection bits"]
        #[inline(always)]
        pub const fn mem_mode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Memory mapping selection bits"]
        #[inline(always)]
        pub fn set_mem_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "User bank swapping"]
        #[inline(always)]
        pub const fn ufb(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "User bank swapping"]
        #[inline(always)]
        pub fn set_ufb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Boot mode selected by the boot pins status bits"]
        #[inline(always)]
        pub const fn boot_mode(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Boot mode selected by the boot pins status bits"]
        #[inline(always)]
        pub fn set_boot_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
    }
    impl Default for Cfgr1 {
        #[inline(always)]
        fn default() -> Cfgr1 {
            Cfgr1(0)
        }
    }
    #[doc = "CFGR2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "Firewall disable bit"]
        #[inline(always)]
        pub const fn fwdis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Firewall disable bit"]
        #[inline(always)]
        pub fn set_fwdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Fm+ drive capability on PB6 enable bit"]
        #[inline(always)]
        pub const fn i2c_pb6_fmp(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Fm+ drive capability on PB6 enable bit"]
        #[inline(always)]
        pub fn set_i2c_pb6_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Fm+ drive capability on PB7 enable bit"]
        #[inline(always)]
        pub const fn i2c_pb7_fmp(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Fm+ drive capability on PB7 enable bit"]
        #[inline(always)]
        pub fn set_i2c_pb7_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Fm+ drive capability on PB8 enable bit"]
        #[inline(always)]
        pub const fn i2c_pb8_fmp(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Fm+ drive capability on PB8 enable bit"]
        #[inline(always)]
        pub fn set_i2c_pb8_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Fm+ drive capability on PB9 enable bit"]
        #[inline(always)]
        pub const fn i2c_pb9_fmp(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Fm+ drive capability on PB9 enable bit"]
        #[inline(always)]
        pub fn set_i2c_pb9_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "I2C1 Fm+ drive capability enable bit"]
        #[inline(always)]
        pub const fn i2c1_fmp(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 Fm+ drive capability enable bit"]
        #[inline(always)]
        pub fn set_i2c1_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "I2C2 Fm+ drive capability enable bit"]
        #[inline(always)]
        pub const fn i2c2_fmp(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 Fm+ drive capability enable bit"]
        #[inline(always)]
        pub fn set_i2c2_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "I2C3 Fm+ drive capability enable bit"]
        #[inline(always)]
        pub const fn i2c3_fmp(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 Fm+ drive capability enable bit"]
        #[inline(always)]
        pub fn set_i2c3_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Cfgr2 {
        #[inline(always)]
        fn default() -> Cfgr2 {
            Cfgr2(0)
        }
    }
    #[doc = "CFGR3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr3(pub u32);
    impl Cfgr3 {
        #[doc = "VREFINT enable and scaler control for COMP2 enable bit"]
        #[inline(always)]
        pub const fn en_vrefint(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "VREFINT enable and scaler control for COMP2 enable bit"]
        #[inline(always)]
        pub fn set_en_vrefint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "VREFINT_ADC connection bit"]
        #[inline(always)]
        pub const fn sel_vref_out(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "VREFINT_ADC connection bit"]
        #[inline(always)]
        pub fn set_sel_vref_out(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "VREFINT reference for ADC enable bit"]
        #[inline(always)]
        pub const fn enbuf_vrefint_adc(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "VREFINT reference for ADC enable bit"]
        #[inline(always)]
        pub fn set_enbuf_vrefint_adc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Temperature sensor reference for ADC enable bit"]
        #[inline(always)]
        pub const fn enbuf_sensor_adc(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Temperature sensor reference for ADC enable bit"]
        #[inline(always)]
        pub fn set_enbuf_sensor_adc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "VREFINT reference for COMP2 scaler enable bit"]
        #[inline(always)]
        pub const fn enbuf_vrefint_comp2(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "VREFINT reference for COMP2 scaler enable bit"]
        #[inline(always)]
        pub fn set_enbuf_vrefint_comp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "VREFINT reference for HSI48 oscillator enable bit"]
        #[inline(always)]
        pub const fn enref_hsi48(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "VREFINT reference for HSI48 oscillator enable bit"]
        #[inline(always)]
        pub fn set_enref_hsi48(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "VREFINT ready flag"]
        #[inline(always)]
        pub const fn vrefint_rdyf(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "VREFINT ready flag"]
        #[inline(always)]
        pub fn set_vrefint_rdyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SYSCFG_CFGR3 lock bit"]
        #[inline(always)]
        pub const fn ref_lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SYSCFG_CFGR3 lock bit"]
        #[inline(always)]
        pub fn set_ref_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cfgr3 {
        #[inline(always)]
        fn default() -> Cfgr3 {
            Cfgr3(0)
        }
    }
    #[doc = "external interrupt configuration register 1-4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exticr(pub u32);
    impl Exticr {
        #[doc = "EXTI configuration bits"]
        #[inline(always)]
        pub const fn exti(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "EXTI configuration bits"]
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
}
