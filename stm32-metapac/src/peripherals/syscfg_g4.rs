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
    #[doc = "Remap Memory register"]
    #[inline(always)]
    pub const fn memrmp(self) -> crate::common::Reg<regs::Memrmp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "peripheral mode configuration register"]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::Cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "external interrupt configuration register 1"]
    #[inline(always)]
    pub const fn exticr(self, n: usize) -> crate::common::Reg<regs::Exticr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 4usize) as _) }
    }
    #[doc = "CCM SRAM control and status register"]
    #[inline(always)]
    pub const fn scsr(self) -> crate::common::Reg<regs::Scsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "configuration register 2"]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "SRAM Write protection register 1"]
    #[inline(always)]
    pub const fn swpr(self) -> crate::common::Reg<regs::Swpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "SRAM2 Key Register"]
    #[inline(always)]
    pub const fn skr(self) -> crate::common::Reg<regs::Skr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
}
pub mod regs {
    #[doc = "peripheral mode configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr1(pub u32);
    impl Cfgr1 {
        #[doc = "BOOSTEN"]
        #[inline(always)]
        pub const fn boosten(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "BOOSTEN"]
        #[inline(always)]
        pub fn set_boosten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "GPIO analog switch control voltage selection"]
        #[inline(always)]
        pub const fn anaswvdd(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "GPIO analog switch control voltage selection"]
        #[inline(always)]
        pub fn set_anaswvdd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "FM+ drive capability on PB6"]
        #[inline(always)]
        pub const fn i2c_pb6_fmp(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "FM+ drive capability on PB6"]
        #[inline(always)]
        pub fn set_i2c_pb6_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "FM+ drive capability on PB6"]
        #[inline(always)]
        pub const fn i2c_pb7_fmp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "FM+ drive capability on PB6"]
        #[inline(always)]
        pub fn set_i2c_pb7_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "FM+ drive capability on PB6"]
        #[inline(always)]
        pub const fn i2c_pb8_fmp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "FM+ drive capability on PB6"]
        #[inline(always)]
        pub fn set_i2c_pb8_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "FM+ drive capability on PB6"]
        #[inline(always)]
        pub const fn i2c_pb9_fmp(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "FM+ drive capability on PB6"]
        #[inline(always)]
        pub fn set_i2c_pb9_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "I2C1 FM+ drive capability enable"]
        #[inline(always)]
        pub const fn i2c1_fmp(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 FM+ drive capability enable"]
        #[inline(always)]
        pub fn set_i2c1_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "I2C1 FM+ drive capability enable"]
        #[inline(always)]
        pub const fn i2c2_fmp(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 FM+ drive capability enable"]
        #[inline(always)]
        pub fn set_i2c2_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C1 FM+ drive capability enable"]
        #[inline(always)]
        pub const fn i2c3_fmp(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 FM+ drive capability enable"]
        #[inline(always)]
        pub fn set_i2c3_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I2C1 FM+ drive capability enable"]
        #[inline(always)]
        pub const fn i2c4_fmp(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 FM+ drive capability enable"]
        #[inline(always)]
        pub fn set_i2c4_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "FPU Interrupts Enable"]
        #[inline(always)]
        pub const fn fpu_ie(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x3f;
            val as u8
        }
        #[doc = "FPU Interrupts Enable"]
        #[inline(always)]
        pub fn set_fpu_ie(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 26usize)) | (((val as u32) & 0x3f) << 26usize);
        }
    }
    impl Default for Cfgr1 {
        #[inline(always)]
        fn default() -> Cfgr1 {
            Cfgr1(0)
        }
    }
    #[doc = "configuration register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "Core Lockup Lock"]
        #[inline(always)]
        pub const fn cll(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Core Lockup Lock"]
        #[inline(always)]
        pub fn set_cll(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SRAM Parity Lock"]
        #[inline(always)]
        pub const fn spl(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM Parity Lock"]
        #[inline(always)]
        pub fn set_spl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PVD Lock"]
        #[inline(always)]
        pub const fn pvdl(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PVD Lock"]
        #[inline(always)]
        pub fn set_pvdl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ECC Lock"]
        #[inline(always)]
        pub const fn eccl(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ECC Lock"]
        #[inline(always)]
        pub fn set_eccl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SRAM Parity Flag"]
        #[inline(always)]
        pub const fn spf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM Parity Flag"]
        #[inline(always)]
        pub fn set_spf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Cfgr2 {
        #[inline(always)]
        fn default() -> Cfgr2 {
            Cfgr2(0)
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
    #[doc = "Remap Memory register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Memrmp(pub u32);
    impl Memrmp {
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
        #[doc = "User Flash Bank mode"]
        #[inline(always)]
        pub const fn fb_mode(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "User Flash Bank mode"]
        #[inline(always)]
        pub fn set_fb_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Memrmp {
        #[inline(always)]
        fn default() -> Memrmp {
            Memrmp(0)
        }
    }
    #[doc = "CCM SRAM control and status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scsr(pub u32);
    impl Scsr {
        #[doc = "CCM SRAM Erase"]
        #[inline(always)]
        pub const fn ccmer(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "CCM SRAM Erase"]
        #[inline(always)]
        pub fn set_ccmer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CCM SRAM busy by erase operation"]
        #[inline(always)]
        pub const fn ccmbsy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CCM SRAM busy by erase operation"]
        #[inline(always)]
        pub fn set_ccmbsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Scsr {
        #[inline(always)]
        fn default() -> Scsr {
            Scsr(0)
        }
    }
    #[doc = "SRAM2 Key Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Skr(pub u32);
    impl Skr {
        #[doc = "SRAM2 Key for software erase"]
        #[inline(always)]
        pub const fn key(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "SRAM2 Key for software erase"]
        #[inline(always)]
        pub fn set_key(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Skr {
        #[inline(always)]
        fn default() -> Skr {
            Skr(0)
        }
    }
    #[doc = "SRAM Write protection register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Swpr(pub u32);
    impl Swpr {
        #[doc = "Write protection"]
        #[inline(always)]
        pub const fn page_wp(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Write protection"]
        #[inline(always)]
        pub fn set_page_wp(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Swpr {
        #[inline(always)]
        fn default() -> Swpr {
            Swpr(0)
        }
    }
}
