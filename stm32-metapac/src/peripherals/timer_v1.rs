#[doc = "Advanced-timers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimAdv {
    ptr: *mut u8,
}
unsafe impl Send for TimAdv {}
unsafe impl Sync for TimAdv {}
impl TimAdv {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1Gp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2Adv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "slave mode control register"]
    #[inline(always)]
    pub const fn smcr(self) -> crate::common::Reg<regs::Smcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(self) -> crate::common::Reg<regs::DierAdv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::SrAdv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "event generation register"]
    #[inline(always)]
    pub const fn egr(self) -> crate::common::Reg<regs::EgrAdv, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (input mode)"]
    #[inline(always)]
    pub const fn ccmr_input(self, n: usize) -> crate::common::Reg<regs::CcmrInput, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub const fn ccmr_output(self, n: usize) -> crate::common::Reg<regs::CcmrOutput, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare enable register"]
    #[inline(always)]
    pub const fn ccer(self) -> crate::common::Reg<regs::CcerAdv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "counter"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::Cnt16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "prescaler"]
    #[inline(always)]
    pub const fn psc(self) -> crate::common::Reg<regs::Psc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "auto-reload register"]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::Arr16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "repetition counter register"]
    #[inline(always)]
    pub const fn rcr(self) -> crate::common::Reg<regs::Rcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "capture/compare register"]
    #[inline(always)]
    pub const fn ccr(self, n: usize) -> crate::common::Reg<regs::Ccr16, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize + n * 4usize) as _) }
    }
    #[doc = "break and dead-time register"]
    #[inline(always)]
    pub const fn bdtr(self) -> crate::common::Reg<regs::Bdtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize) as _) }
    }
    #[doc = "DMA control register"]
    #[inline(always)]
    pub const fn dcr(self) -> crate::common::Reg<regs::Dcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(72usize) as _) }
    }
    #[doc = "DMA address for full transfer"]
    #[inline(always)]
    pub const fn dmar(self) -> crate::common::Reg<regs::Dmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(76usize) as _) }
    }
}
#[doc = "Basic timer"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimBasic {
    ptr: *mut u8,
}
unsafe impl Send for TimBasic {}
unsafe impl Sync for TimBasic {}
impl TimBasic {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1Basic, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2Basic, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(self) -> crate::common::Reg<regs::DierBasic, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::SrBasic, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "event generation register"]
    #[inline(always)]
    pub const fn egr(self) -> crate::common::Reg<regs::EgrBasic, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "counter"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::Cnt16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "prescaler"]
    #[inline(always)]
    pub const fn psc(self) -> crate::common::Reg<regs::Psc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "auto-reload register"]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::Arr16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
}
#[doc = "General purpose 16-bit timer"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimGp16 {
    ptr: *mut u8,
}
unsafe impl Send for TimGp16 {}
unsafe impl Sync for TimGp16 {}
impl TimGp16 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1Gp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2Gp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "slave mode control register"]
    #[inline(always)]
    pub const fn smcr(self) -> crate::common::Reg<regs::Smcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(self) -> crate::common::Reg<regs::DierGp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::SrGp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "event generation register"]
    #[inline(always)]
    pub const fn egr(self) -> crate::common::Reg<regs::EgrGp, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (input mode)"]
    #[inline(always)]
    pub const fn ccmr_input(self, n: usize) -> crate::common::Reg<regs::CcmrInput, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub const fn ccmr_output(self, n: usize) -> crate::common::Reg<regs::CcmrOutput, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare enable register"]
    #[inline(always)]
    pub const fn ccer(self) -> crate::common::Reg<regs::CcerGp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "counter"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::Cnt16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "prescaler"]
    #[inline(always)]
    pub const fn psc(self) -> crate::common::Reg<regs::Psc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "auto-reload register"]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::Arr16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "capture/compare register"]
    #[inline(always)]
    pub const fn ccr(self, n: usize) -> crate::common::Reg<regs::Ccr16, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize + n * 4usize) as _) }
    }
    #[doc = "DMA control register"]
    #[inline(always)]
    pub const fn dcr(self) -> crate::common::Reg<regs::Dcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(72usize) as _) }
    }
    #[doc = "DMA address for full transfer"]
    #[inline(always)]
    pub const fn dmar(self) -> crate::common::Reg<regs::Dmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(76usize) as _) }
    }
}
#[doc = "General purpose 32-bit timer"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TimGp32 {
    ptr: *mut u8,
}
unsafe impl Send for TimGp32 {}
unsafe impl Sync for TimGp32 {}
impl TimGp32 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1Gp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2Gp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "slave mode control register"]
    #[inline(always)]
    pub const fn smcr(self) -> crate::common::Reg<regs::Smcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[inline(always)]
    pub const fn dier(self) -> crate::common::Reg<regs::DierGp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::SrGp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "event generation register"]
    #[inline(always)]
    pub const fn egr(self) -> crate::common::Reg<regs::EgrGp, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (input mode)"]
    #[inline(always)]
    pub const fn ccmr_input(self, n: usize) -> crate::common::Reg<regs::CcmrInput, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub const fn ccmr_output(self, n: usize) -> crate::common::Reg<regs::CcmrOutput, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize + n * 4usize) as _) }
    }
    #[doc = "capture/compare enable register"]
    #[inline(always)]
    pub const fn ccer(self) -> crate::common::Reg<regs::CcerGp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "counter"]
    #[inline(always)]
    pub const fn cnt(self) -> crate::common::Reg<regs::Cnt32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "prescaler"]
    #[inline(always)]
    pub const fn psc(self) -> crate::common::Reg<regs::Psc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "auto-reload register"]
    #[inline(always)]
    pub const fn arr(self) -> crate::common::Reg<regs::Arr32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "capture/compare register"]
    #[inline(always)]
    pub const fn ccr(self, n: usize) -> crate::common::Reg<regs::Ccr32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize + n * 4usize) as _) }
    }
    #[doc = "DMA control register"]
    #[inline(always)]
    pub const fn dcr(self) -> crate::common::Reg<regs::Dcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(72usize) as _) }
    }
    #[doc = "DMA address for full transfer"]
    #[inline(always)]
    pub const fn dmar(self) -> crate::common::Reg<regs::Dmar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(76usize) as _) }
    }
}
pub mod regs {
    #[doc = "auto-reload register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Arr16(pub u32);
    impl Arr16 {
        #[doc = "Auto-reload value"]
        #[inline(always)]
        pub const fn arr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Auto-reload value"]
        #[inline(always)]
        pub fn set_arr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Arr16 {
        #[inline(always)]
        fn default() -> Arr16 {
            Arr16(0)
        }
    }
    #[doc = "auto-reload register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Arr32(pub u32);
    impl Arr32 {
        #[doc = "Auto-reload value"]
        #[inline(always)]
        pub const fn arr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Auto-reload value"]
        #[inline(always)]
        pub fn set_arr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Arr32 {
        #[inline(always)]
        fn default() -> Arr32 {
            Arr32(0)
        }
    }
    #[doc = "break and dead-time register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bdtr(pub u32);
    impl Bdtr {
        #[doc = "Dead-time generator setup"]
        #[inline(always)]
        pub const fn dtg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Dead-time generator setup"]
        #[inline(always)]
        pub fn set_dtg(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Lock configuration"]
        #[inline(always)]
        pub const fn lock(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Lock configuration"]
        #[inline(always)]
        pub fn set_lock(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "Off-state selection for Idle mode"]
        #[inline(always)]
        pub const fn ossi(&self) -> super::vals::Ossi {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Ossi::from_bits(val as u8)
        }
        #[doc = "Off-state selection for Idle mode"]
        #[inline(always)]
        pub fn set_ossi(&mut self, val: super::vals::Ossi) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "Off-state selection for Run mode"]
        #[inline(always)]
        pub const fn ossr(&self) -> super::vals::Ossr {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Ossr::from_bits(val as u8)
        }
        #[doc = "Off-state selection for Run mode"]
        #[inline(always)]
        pub fn set_ossr(&mut self, val: super::vals::Ossr) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Break enable"]
        #[inline(always)]
        pub const fn bke(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Break enable"]
        #[inline(always)]
        pub fn set_bke(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Break polarity"]
        #[inline(always)]
        pub const fn bkp(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Break polarity"]
        #[inline(always)]
        pub fn set_bkp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Automatic output enable"]
        #[inline(always)]
        pub const fn aoe(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic output enable"]
        #[inline(always)]
        pub fn set_aoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Main output enable"]
        #[inline(always)]
        pub const fn moe(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Main output enable"]
        #[inline(always)]
        pub fn set_moe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Bdtr {
        #[inline(always)]
        fn default() -> Bdtr {
            Bdtr(0)
        }
    }
    #[doc = "capture/compare enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CcerAdv(pub u32);
    impl CcerAdv {
        #[doc = "Capture/Compare 1 output enable"]
        #[inline(always)]
        pub const fn cce(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 output enable"]
        #[inline(always)]
        pub fn set_cce(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare 1 output Polarity"]
        #[inline(always)]
        pub const fn ccp(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 output Polarity"]
        #[inline(always)]
        pub fn set_ccp(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare 1 complementary output enable"]
        #[inline(always)]
        pub const fn ccne(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 2usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 complementary output enable"]
        #[inline(always)]
        pub fn set_ccne(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 2usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare 1 output Polarity"]
        #[inline(always)]
        pub const fn ccnp(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 3usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 output Polarity"]
        #[inline(always)]
        pub fn set_ccnp(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 3usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for CcerAdv {
        #[inline(always)]
        fn default() -> CcerAdv {
            CcerAdv(0)
        }
    }
    #[doc = "capture/compare enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CcerGp(pub u32);
    impl CcerGp {
        #[doc = "Capture/Compare 1 output enable"]
        #[inline(always)]
        pub const fn cce(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 output enable"]
        #[inline(always)]
        pub fn set_cce(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare 1 output Polarity"]
        #[inline(always)]
        pub const fn ccp(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 output Polarity"]
        #[inline(always)]
        pub fn set_ccp(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare 1 output Polarity"]
        #[inline(always)]
        pub const fn ccnp(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 3usize + n * 4usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 output Polarity"]
        #[inline(always)]
        pub fn set_ccnp(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 3usize + n * 4usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for CcerGp {
        #[inline(always)]
        fn default() -> CcerGp {
            CcerGp(0)
        }
    }
    #[doc = "capture/compare mode register 1 (input mode)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CcmrInput(pub u32);
    impl CcmrInput {
        #[doc = "Capture/Compare 1 selection"]
        #[inline(always)]
        pub const fn ccs(&self, n: usize) -> super::vals::CcmrInputCcs {
            assert!(n < 2usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::CcmrInputCcs::from_bits(val as u8)
        }
        #[doc = "Capture/Compare 1 selection"]
        #[inline(always)]
        pub fn set_ccs(&mut self, n: usize, val: super::vals::CcmrInputCcs) {
            assert!(n < 2usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Input capture 1 prescaler"]
        #[inline(always)]
        pub const fn icpsc(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 2usize + n * 8usize;
            let val = (self.0 >> offs) & 0x03;
            val as u8
        }
        #[doc = "Input capture 1 prescaler"]
        #[inline(always)]
        pub fn set_icpsc(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 2usize + n * 8usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
        }
        #[doc = "Input capture 1 filter"]
        #[inline(always)]
        pub const fn icf(&self, n: usize) -> super::vals::Icf {
            assert!(n < 2usize);
            let offs = 4usize + n * 8usize;
            let val = (self.0 >> offs) & 0x0f;
            super::vals::Icf::from_bits(val as u8)
        }
        #[doc = "Input capture 1 filter"]
        #[inline(always)]
        pub fn set_icf(&mut self, n: usize, val: super::vals::Icf) {
            assert!(n < 2usize);
            let offs = 4usize + n * 8usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val.to_bits() as u32) & 0x0f) << offs);
        }
    }
    impl Default for CcmrInput {
        #[inline(always)]
        fn default() -> CcmrInput {
            CcmrInput(0)
        }
    }
    #[doc = "capture/compare mode register 2 (output mode)"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CcmrOutput(pub u32);
    impl CcmrOutput {
        #[doc = "Capture/Compare 3 selection"]
        #[inline(always)]
        pub const fn ccs(&self, n: usize) -> super::vals::CcmrOutputCcs {
            assert!(n < 2usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::CcmrOutputCcs::from_bits(val as u8)
        }
        #[doc = "Capture/Compare 3 selection"]
        #[inline(always)]
        pub fn set_ccs(&mut self, n: usize, val: super::vals::CcmrOutputCcs) {
            assert!(n < 2usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "Output compare 3 fast enable"]
        #[inline(always)]
        pub const fn ocfe(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 2usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output compare 3 fast enable"]
        #[inline(always)]
        pub fn set_ocfe(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 2usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output compare 3 preload enable"]
        #[inline(always)]
        pub const fn ocpe(&self, n: usize) -> super::vals::Ocpe {
            assert!(n < 2usize);
            let offs = 3usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Ocpe::from_bits(val as u8)
        }
        #[doc = "Output compare 3 preload enable"]
        #[inline(always)]
        pub fn set_ocpe(&mut self, n: usize, val: super::vals::Ocpe) {
            assert!(n < 2usize);
            let offs = 3usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Output compare 3 mode"]
        #[inline(always)]
        pub const fn ocm(&self, n: usize) -> super::vals::Ocm {
            assert!(n < 2usize);
            let offs = 4usize + n * 8usize;
            let val = (self.0 >> offs) & 0x07;
            super::vals::Ocm::from_bits(val as u8)
        }
        #[doc = "Output compare 3 mode"]
        #[inline(always)]
        pub fn set_ocm(&mut self, n: usize, val: super::vals::Ocm) {
            assert!(n < 2usize);
            let offs = 4usize + n * 8usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val.to_bits() as u32) & 0x07) << offs);
        }
        #[doc = "Output compare 3 clear enable"]
        #[inline(always)]
        pub const fn occe(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 7usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output compare 3 clear enable"]
        #[inline(always)]
        pub fn set_occe(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 7usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for CcmrOutput {
        #[inline(always)]
        fn default() -> CcmrOutput {
            CcmrOutput(0)
        }
    }
    #[doc = "capture/compare register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr16(pub u32);
    impl Ccr16 {
        #[doc = "Capture/Compare 1 value"]
        #[inline(always)]
        pub const fn ccr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Capture/Compare 1 value"]
        #[inline(always)]
        pub fn set_ccr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ccr16 {
        #[inline(always)]
        fn default() -> Ccr16 {
            Ccr16(0)
        }
    }
    #[doc = "capture/compare register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr32(pub u32);
    impl Ccr32 {
        #[doc = "Capture/Compare 1 value"]
        #[inline(always)]
        pub const fn ccr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Capture/Compare 1 value"]
        #[inline(always)]
        pub fn set_ccr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ccr32 {
        #[inline(always)]
        fn default() -> Ccr32 {
            Ccr32(0)
        }
    }
    #[doc = "counter"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cnt16(pub u32);
    impl Cnt16 {
        #[doc = "counter value"]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "counter value"]
        #[inline(always)]
        pub fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Cnt16 {
        #[inline(always)]
        fn default() -> Cnt16 {
            Cnt16(0)
        }
    }
    #[doc = "counter"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cnt32(pub u32);
    impl Cnt32 {
        #[doc = "counter value"]
        #[inline(always)]
        pub const fn cnt(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "counter value"]
        #[inline(always)]
        pub fn set_cnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cnt32 {
        #[inline(always)]
        fn default() -> Cnt32 {
            Cnt32(0)
        }
    }
    #[doc = "control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1Basic(pub u32);
    impl Cr1Basic {
        #[doc = "Counter enable"]
        #[inline(always)]
        pub const fn cen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Counter enable"]
        #[inline(always)]
        pub fn set_cen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Update disable"]
        #[inline(always)]
        pub const fn udis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Update disable"]
        #[inline(always)]
        pub fn set_udis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Update request source"]
        #[inline(always)]
        pub const fn urs(&self) -> super::vals::Urs {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Urs::from_bits(val as u8)
        }
        #[doc = "Update request source"]
        #[inline(always)]
        pub fn set_urs(&mut self, val: super::vals::Urs) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "One-pulse mode"]
        #[inline(always)]
        pub const fn opm(&self) -> super::vals::Opm {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Opm::from_bits(val as u8)
        }
        #[doc = "One-pulse mode"]
        #[inline(always)]
        pub fn set_opm(&mut self, val: super::vals::Opm) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Auto-reload preload enable"]
        #[inline(always)]
        pub const fn arpe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Auto-reload preload enable"]
        #[inline(always)]
        pub fn set_arpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Cr1Basic {
        #[inline(always)]
        fn default() -> Cr1Basic {
            Cr1Basic(0)
        }
    }
    #[doc = "control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1Gp(pub u32);
    impl Cr1Gp {
        #[doc = "Counter enable"]
        #[inline(always)]
        pub const fn cen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Counter enable"]
        #[inline(always)]
        pub fn set_cen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Update disable"]
        #[inline(always)]
        pub const fn udis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Update disable"]
        #[inline(always)]
        pub fn set_udis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Update request source"]
        #[inline(always)]
        pub const fn urs(&self) -> super::vals::Urs {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Urs::from_bits(val as u8)
        }
        #[doc = "Update request source"]
        #[inline(always)]
        pub fn set_urs(&mut self, val: super::vals::Urs) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "One-pulse mode"]
        #[inline(always)]
        pub const fn opm(&self) -> super::vals::Opm {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Opm::from_bits(val as u8)
        }
        #[doc = "One-pulse mode"]
        #[inline(always)]
        pub fn set_opm(&mut self, val: super::vals::Opm) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Direction"]
        #[inline(always)]
        pub const fn dir(&self) -> super::vals::Dir {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Dir::from_bits(val as u8)
        }
        #[doc = "Direction"]
        #[inline(always)]
        pub fn set_dir(&mut self, val: super::vals::Dir) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Center-aligned mode selection"]
        #[inline(always)]
        pub const fn cms(&self) -> super::vals::Cms {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::Cms::from_bits(val as u8)
        }
        #[doc = "Center-aligned mode selection"]
        #[inline(always)]
        pub fn set_cms(&mut self, val: super::vals::Cms) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
        }
        #[doc = "Auto-reload preload enable"]
        #[inline(always)]
        pub const fn arpe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Auto-reload preload enable"]
        #[inline(always)]
        pub fn set_arpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Clock division"]
        #[inline(always)]
        pub const fn ckd(&self) -> super::vals::Ckd {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Ckd::from_bits(val as u8)
        }
        #[doc = "Clock division"]
        #[inline(always)]
        pub fn set_ckd(&mut self, val: super::vals::Ckd) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
    }
    impl Default for Cr1Gp {
        #[inline(always)]
        fn default() -> Cr1Gp {
            Cr1Gp(0)
        }
    }
    #[doc = "control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2Adv(pub u32);
    impl Cr2Adv {
        #[doc = "Capture/compare preloaded control"]
        #[inline(always)]
        pub const fn ccpc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare preloaded control"]
        #[inline(always)]
        pub fn set_ccpc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare control update selection"]
        #[inline(always)]
        pub const fn ccus(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare control update selection"]
        #[inline(always)]
        pub fn set_ccus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Capture/compare DMA selection"]
        #[inline(always)]
        pub const fn ccds(&self) -> super::vals::Ccds {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Ccds::from_bits(val as u8)
        }
        #[doc = "Capture/compare DMA selection"]
        #[inline(always)]
        pub fn set_ccds(&mut self, val: super::vals::Ccds) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Master mode selection"]
        #[inline(always)]
        pub const fn mms(&self) -> super::vals::Mms {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Mms::from_bits(val as u8)
        }
        #[doc = "Master mode selection"]
        #[inline(always)]
        pub fn set_mms(&mut self, val: super::vals::Mms) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "TI1 selection"]
        #[inline(always)]
        pub const fn ti1s(&self) -> super::vals::Tis {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Tis::from_bits(val as u8)
        }
        #[doc = "TI1 selection"]
        #[inline(always)]
        pub fn set_ti1s(&mut self, val: super::vals::Tis) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "Output Idle state 1"]
        #[inline(always)]
        pub const fn ois(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 8usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Output Idle state 1"]
        #[inline(always)]
        pub fn set_ois(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 8usize + n * 2usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Output Idle state 1"]
        #[inline(always)]
        pub const fn ois1n(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Output Idle state 1"]
        #[inline(always)]
        pub fn set_ois1n(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Output Idle state 2"]
        #[inline(always)]
        pub const fn ois2n(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Output Idle state 2"]
        #[inline(always)]
        pub fn set_ois2n(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Output Idle state 3"]
        #[inline(always)]
        pub const fn ois3n(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Output Idle state 3"]
        #[inline(always)]
        pub fn set_ois3n(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Cr2Adv {
        #[inline(always)]
        fn default() -> Cr2Adv {
            Cr2Adv(0)
        }
    }
    #[doc = "control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2Basic(pub u32);
    impl Cr2Basic {
        #[doc = "Master mode selection"]
        #[inline(always)]
        pub const fn mms(&self) -> super::vals::Mms {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Mms::from_bits(val as u8)
        }
        #[doc = "Master mode selection"]
        #[inline(always)]
        pub fn set_mms(&mut self, val: super::vals::Mms) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
    }
    impl Default for Cr2Basic {
        #[inline(always)]
        fn default() -> Cr2Basic {
            Cr2Basic(0)
        }
    }
    #[doc = "control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2Gp(pub u32);
    impl Cr2Gp {
        #[doc = "Capture/compare DMA selection"]
        #[inline(always)]
        pub const fn ccds(&self) -> super::vals::Ccds {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Ccds::from_bits(val as u8)
        }
        #[doc = "Capture/compare DMA selection"]
        #[inline(always)]
        pub fn set_ccds(&mut self, val: super::vals::Ccds) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Master mode selection"]
        #[inline(always)]
        pub const fn mms(&self) -> super::vals::Mms {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Mms::from_bits(val as u8)
        }
        #[doc = "Master mode selection"]
        #[inline(always)]
        pub fn set_mms(&mut self, val: super::vals::Mms) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "TI1 selection"]
        #[inline(always)]
        pub const fn ti1s(&self) -> super::vals::Tis {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Tis::from_bits(val as u8)
        }
        #[doc = "TI1 selection"]
        #[inline(always)]
        pub fn set_ti1s(&mut self, val: super::vals::Tis) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Cr2Gp {
        #[inline(always)]
        fn default() -> Cr2Gp {
            Cr2Gp(0)
        }
    }
    #[doc = "DMA control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dcr(pub u32);
    impl Dcr {
        #[doc = "DMA base address"]
        #[inline(always)]
        pub const fn dba(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "DMA base address"]
        #[inline(always)]
        pub fn set_dba(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "DMA burst length"]
        #[inline(always)]
        pub const fn dbl(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "DMA burst length"]
        #[inline(always)]
        pub fn set_dbl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
    }
    impl Default for Dcr {
        #[inline(always)]
        fn default() -> Dcr {
            Dcr(0)
        }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DierAdv(pub u32);
    impl DierAdv {
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub const fn uie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub fn set_uie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/Compare 1 interrupt enable"]
        #[inline(always)]
        pub const fn ccie(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 interrupt enable"]
        #[inline(always)]
        pub fn set_ccie(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "COM interrupt enable"]
        #[inline(always)]
        pub const fn comie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "COM interrupt enable"]
        #[inline(always)]
        pub fn set_comie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Trigger interrupt enable"]
        #[inline(always)]
        pub const fn tie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger interrupt enable"]
        #[inline(always)]
        pub fn set_tie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Break interrupt enable"]
        #[inline(always)]
        pub const fn bie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Break interrupt enable"]
        #[inline(always)]
        pub fn set_bie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Update DMA request enable"]
        #[inline(always)]
        pub const fn ude(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Update DMA request enable"]
        #[inline(always)]
        pub fn set_ude(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Capture/Compare 1 DMA request enable"]
        #[inline(always)]
        pub const fn ccde(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 9usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 DMA request enable"]
        #[inline(always)]
        pub fn set_ccde(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 9usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "COM DMA request enable"]
        #[inline(always)]
        pub const fn comde(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "COM DMA request enable"]
        #[inline(always)]
        pub fn set_comde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Trigger DMA request enable"]
        #[inline(always)]
        pub const fn tde(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger DMA request enable"]
        #[inline(always)]
        pub fn set_tde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for DierAdv {
        #[inline(always)]
        fn default() -> DierAdv {
            DierAdv(0)
        }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DierBasic(pub u32);
    impl DierBasic {
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub const fn uie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub fn set_uie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Update DMA request enable"]
        #[inline(always)]
        pub const fn ude(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Update DMA request enable"]
        #[inline(always)]
        pub fn set_ude(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for DierBasic {
        #[inline(always)]
        fn default() -> DierBasic {
            DierBasic(0)
        }
    }
    #[doc = "DMA/Interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DierGp(pub u32);
    impl DierGp {
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub const fn uie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt enable"]
        #[inline(always)]
        pub fn set_uie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/Compare 1 interrupt enable"]
        #[inline(always)]
        pub const fn ccie(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 interrupt enable"]
        #[inline(always)]
        pub fn set_ccie(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Trigger interrupt enable"]
        #[inline(always)]
        pub const fn tie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger interrupt enable"]
        #[inline(always)]
        pub fn set_tie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Update DMA request enable"]
        #[inline(always)]
        pub const fn ude(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Update DMA request enable"]
        #[inline(always)]
        pub fn set_ude(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Capture/Compare 1 DMA request enable"]
        #[inline(always)]
        pub const fn ccde(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 9usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 DMA request enable"]
        #[inline(always)]
        pub fn set_ccde(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 9usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Trigger DMA request enable"]
        #[inline(always)]
        pub const fn tde(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger DMA request enable"]
        #[inline(always)]
        pub fn set_tde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for DierGp {
        #[inline(always)]
        fn default() -> DierGp {
            DierGp(0)
        }
    }
    #[doc = "DMA address for full transfer"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dmar(pub u32);
    impl Dmar {
        #[doc = "DMA register for burst accesses"]
        #[inline(always)]
        pub const fn dmab(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "DMA register for burst accesses"]
        #[inline(always)]
        pub fn set_dmab(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Dmar {
        #[inline(always)]
        fn default() -> Dmar {
            Dmar(0)
        }
    }
    #[doc = "event generation register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EgrAdv(pub u32);
    impl EgrAdv {
        #[doc = "Update generation"]
        #[inline(always)]
        pub const fn ug(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update generation"]
        #[inline(always)]
        pub fn set_ug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare 1 generation"]
        #[inline(always)]
        pub const fn ccg(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 1 generation"]
        #[inline(always)]
        pub fn set_ccg(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare control update generation"]
        #[inline(always)]
        pub const fn comg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare control update generation"]
        #[inline(always)]
        pub fn set_comg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Trigger generation"]
        #[inline(always)]
        pub const fn tg(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger generation"]
        #[inline(always)]
        pub fn set_tg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Break generation"]
        #[inline(always)]
        pub const fn bg(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Break generation"]
        #[inline(always)]
        pub fn set_bg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for EgrAdv {
        #[inline(always)]
        fn default() -> EgrAdv {
            EgrAdv(0)
        }
    }
    #[doc = "event generation register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EgrBasic(pub u32);
    impl EgrBasic {
        #[doc = "Update generation"]
        #[inline(always)]
        pub const fn ug(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update generation"]
        #[inline(always)]
        pub fn set_ug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for EgrBasic {
        #[inline(always)]
        fn default() -> EgrBasic {
            EgrBasic(0)
        }
    }
    #[doc = "event generation register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EgrGp(pub u32);
    impl EgrGp {
        #[doc = "Update generation"]
        #[inline(always)]
        pub const fn ug(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update generation"]
        #[inline(always)]
        pub fn set_ug(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare 1 generation"]
        #[inline(always)]
        pub const fn ccg(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 1 generation"]
        #[inline(always)]
        pub fn set_ccg(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Capture/Compare control update generation"]
        #[inline(always)]
        pub const fn comg(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare control update generation"]
        #[inline(always)]
        pub fn set_comg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Trigger generation"]
        #[inline(always)]
        pub const fn tg(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger generation"]
        #[inline(always)]
        pub fn set_tg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Break generation"]
        #[inline(always)]
        pub const fn bg(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Break generation"]
        #[inline(always)]
        pub fn set_bg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for EgrGp {
        #[inline(always)]
        fn default() -> EgrGp {
            EgrGp(0)
        }
    }
    #[doc = "prescaler"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Psc(pub u32);
    impl Psc {
        #[doc = "Prescaler value"]
        #[inline(always)]
        pub const fn psc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Prescaler value"]
        #[inline(always)]
        pub fn set_psc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Psc {
        #[inline(always)]
        fn default() -> Psc {
            Psc(0)
        }
    }
    #[doc = "repetition counter register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rcr(pub u32);
    impl Rcr {
        #[doc = "Repetition counter value"]
        #[inline(always)]
        pub const fn rep(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Repetition counter value"]
        #[inline(always)]
        pub fn set_rep(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Rcr {
        #[inline(always)]
        fn default() -> Rcr {
            Rcr(0)
        }
    }
    #[doc = "slave mode control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smcr(pub u32);
    impl Smcr {
        #[doc = "Slave mode selection"]
        #[inline(always)]
        pub const fn sms(&self) -> super::vals::Sms {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Sms::from_bits(val as u8)
        }
        #[doc = "Slave mode selection"]
        #[inline(always)]
        pub fn set_sms(&mut self, val: super::vals::Sms) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Trigger selection"]
        #[inline(always)]
        pub const fn ts(&self) -> super::vals::Ts {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Ts::from_bits(val as u8)
        }
        #[doc = "Trigger selection"]
        #[inline(always)]
        pub fn set_ts(&mut self, val: super::vals::Ts) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "Master/Slave mode"]
        #[inline(always)]
        pub const fn msm(&self) -> super::vals::Msm {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Msm::from_bits(val as u8)
        }
        #[doc = "Master/Slave mode"]
        #[inline(always)]
        pub fn set_msm(&mut self, val: super::vals::Msm) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "External trigger filter"]
        #[inline(always)]
        pub const fn etf(&self) -> super::vals::Etf {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::Etf::from_bits(val as u8)
        }
        #[doc = "External trigger filter"]
        #[inline(always)]
        pub fn set_etf(&mut self, val: super::vals::Etf) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
        }
        #[doc = "External trigger prescaler"]
        #[inline(always)]
        pub const fn etps(&self) -> super::vals::Etps {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Etps::from_bits(val as u8)
        }
        #[doc = "External trigger prescaler"]
        #[inline(always)]
        pub fn set_etps(&mut self, val: super::vals::Etps) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "External clock enable"]
        #[inline(always)]
        pub const fn ece(&self) -> super::vals::Ece {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Ece::from_bits(val as u8)
        }
        #[doc = "External clock enable"]
        #[inline(always)]
        pub fn set_ece(&mut self, val: super::vals::Ece) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "External trigger polarity"]
        #[inline(always)]
        pub const fn etp(&self) -> super::vals::Etp {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Etp::from_bits(val as u8)
        }
        #[doc = "External trigger polarity"]
        #[inline(always)]
        pub fn set_etp(&mut self, val: super::vals::Etp) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Smcr {
        #[inline(always)]
        fn default() -> Smcr {
            Smcr(0)
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SrAdv(pub u32);
    impl SrAdv {
        #[doc = "Update interrupt flag"]
        #[inline(always)]
        pub const fn uif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt flag"]
        #[inline(always)]
        pub fn set_uif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare 1 interrupt flag"]
        #[inline(always)]
        pub const fn ccif(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 1 interrupt flag"]
        #[inline(always)]
        pub fn set_ccif(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "COM interrupt flag"]
        #[inline(always)]
        pub const fn comif(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "COM interrupt flag"]
        #[inline(always)]
        pub fn set_comif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Trigger interrupt flag"]
        #[inline(always)]
        pub const fn tif(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger interrupt flag"]
        #[inline(always)]
        pub fn set_tif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Break interrupt flag"]
        #[inline(always)]
        pub const fn bif(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Break interrupt flag"]
        #[inline(always)]
        pub fn set_bif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Capture/Compare 1 overcapture flag"]
        #[inline(always)]
        pub const fn ccof(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 9usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 overcapture flag"]
        #[inline(always)]
        pub fn set_ccof(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 9usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for SrAdv {
        #[inline(always)]
        fn default() -> SrAdv {
            SrAdv(0)
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SrBasic(pub u32);
    impl SrBasic {
        #[doc = "Update interrupt flag"]
        #[inline(always)]
        pub const fn uif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt flag"]
        #[inline(always)]
        pub fn set_uif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for SrBasic {
        #[inline(always)]
        fn default() -> SrBasic {
            SrBasic(0)
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SrGp(pub u32);
    impl SrGp {
        #[doc = "Update interrupt flag"]
        #[inline(always)]
        pub const fn uif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Update interrupt flag"]
        #[inline(always)]
        pub fn set_uif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Capture/compare 1 interrupt flag"]
        #[inline(always)]
        pub const fn ccif(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/compare 1 interrupt flag"]
        #[inline(always)]
        pub fn set_ccif(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 1usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "COM interrupt flag"]
        #[inline(always)]
        pub const fn comif(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "COM interrupt flag"]
        #[inline(always)]
        pub fn set_comif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Trigger interrupt flag"]
        #[inline(always)]
        pub const fn tif(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Trigger interrupt flag"]
        #[inline(always)]
        pub fn set_tif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Break interrupt flag"]
        #[inline(always)]
        pub const fn bif(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Break interrupt flag"]
        #[inline(always)]
        pub fn set_bif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Capture/Compare 1 overcapture flag"]
        #[inline(always)]
        pub const fn ccof(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 9usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Capture/Compare 1 overcapture flag"]
        #[inline(always)]
        pub fn set_ccof(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 9usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for SrGp {
        #[inline(always)]
        fn default() -> SrGp {
            SrGp(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ccds {
        #[doc = "CCx DMA request sent when CCx event occurs"]
        ONCOMPARE = 0,
        #[doc = "CCx DMA request sent when update event occurs"]
        ONUPDATE = 0x01,
    }
    impl Ccds {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ccds {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ccds {
        #[inline(always)]
        fn from(val: u8) -> Ccds {
            Ccds::from_bits(val)
        }
    }
    impl From<Ccds> for u8 {
        #[inline(always)]
        fn from(val: Ccds) -> u8 {
            Ccds::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum CcmrInputCcs {
        _RESERVED_0 = 0,
        #[doc = "CCx channel is configured as input, normal mapping: ICx mapped to TIx"]
        TI4 = 0x01,
        #[doc = "CCx channel is configured as input, alternate mapping (switches 1 with 2, 3 with 4)"]
        TI3 = 0x02,
        #[doc = "CCx channel is configured as input, ICx is mapped on TRC"]
        TRC = 0x03,
    }
    impl CcmrInputCcs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CcmrInputCcs {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CcmrInputCcs {
        #[inline(always)]
        fn from(val: u8) -> CcmrInputCcs {
            CcmrInputCcs::from_bits(val)
        }
    }
    impl From<CcmrInputCcs> for u8 {
        #[inline(always)]
        fn from(val: CcmrInputCcs) -> u8 {
            CcmrInputCcs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum CcmrOutputCcs {
        #[doc = "CCx channel is configured as output"]
        OUTPUT = 0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl CcmrOutputCcs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CcmrOutputCcs {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CcmrOutputCcs {
        #[inline(always)]
        fn from(val: u8) -> CcmrOutputCcs {
            CcmrOutputCcs::from_bits(val)
        }
    }
    impl From<CcmrOutputCcs> for u8 {
        #[inline(always)]
        fn from(val: CcmrOutputCcs) -> u8 {
            CcmrOutputCcs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ckd {
        #[doc = "t_DTS = t_CK_INT"]
        DIV1 = 0,
        #[doc = "t_DTS = 2 × t_CK_INT"]
        DIV2 = 0x01,
        #[doc = "t_DTS = 4 × t_CK_INT"]
        DIV4 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Ckd {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ckd {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ckd {
        #[inline(always)]
        fn from(val: u8) -> Ckd {
            Ckd::from_bits(val)
        }
    }
    impl From<Ckd> for u8 {
        #[inline(always)]
        fn from(val: Ckd) -> u8 {
            Ckd::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Cms {
        #[doc = "The counter counts up or down depending on the direction bit"]
        EDGEALIGNED = 0,
        #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting down."]
        CENTERALIGNED1 = 0x01,
        #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set only when the counter is counting up."]
        CENTERALIGNED2 = 0x02,
        #[doc = "The counter counts up and down alternatively. Output compare interrupt flags are set both when the counter is counting up or down."]
        CENTERALIGNED3 = 0x03,
    }
    impl Cms {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cms {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cms {
        #[inline(always)]
        fn from(val: u8) -> Cms {
            Cms::from_bits(val)
        }
    }
    impl From<Cms> for u8 {
        #[inline(always)]
        fn from(val: Cms) -> u8 {
            Cms::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Dir {
        #[doc = "Counter used as upcounter"]
        UP = 0,
        #[doc = "Counter used as downcounter"]
        DOWN = 0x01,
    }
    impl Dir {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dir {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dir {
        #[inline(always)]
        fn from(val: u8) -> Dir {
            Dir::from_bits(val)
        }
    }
    impl From<Dir> for u8 {
        #[inline(always)]
        fn from(val: Dir) -> u8 {
            Dir::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ece {
        #[doc = "External clock mode 2 disabled"]
        DISABLED = 0,
        #[doc = "External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal."]
        ENABLED = 0x01,
    }
    impl Ece {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ece {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ece {
        #[inline(always)]
        fn from(val: u8) -> Ece {
            Ece::from_bits(val)
        }
    }
    impl From<Ece> for u8 {
        #[inline(always)]
        fn from(val: Ece) -> u8 {
            Ece::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Etf {
        #[doc = "No filter, sampling is done at fDTS"]
        NOFILTER = 0,
        #[doc = "fSAMPLING=fCK_INT, N=2"]
        FCK_INT_N2 = 0x01,
        #[doc = "fSAMPLING=fCK_INT, N=4"]
        FCK_INT_N4 = 0x02,
        #[doc = "fSAMPLING=fCK_INT, N=8"]
        FCK_INT_N8 = 0x03,
        #[doc = "fSAMPLING=fDTS/2, N=6"]
        FDTS_DIV2_N6 = 0x04,
        #[doc = "fSAMPLING=fDTS/2, N=8"]
        FDTS_DIV2_N8 = 0x05,
        #[doc = "fSAMPLING=fDTS/4, N=6"]
        FDTS_DIV4_N6 = 0x06,
        #[doc = "fSAMPLING=fDTS/4, N=8"]
        FDTS_DIV4_N8 = 0x07,
        #[doc = "fSAMPLING=fDTS/8, N=6"]
        FDTS_DIV8_N6 = 0x08,
        #[doc = "fSAMPLING=fDTS/8, N=8"]
        FDTS_DIV8_N8 = 0x09,
        #[doc = "fSAMPLING=fDTS/16, N=5"]
        FDTS_DIV16_N5 = 0x0a,
        #[doc = "fSAMPLING=fDTS/16, N=6"]
        FDTS_DIV16_N6 = 0x0b,
        #[doc = "fSAMPLING=fDTS/16, N=8"]
        FDTS_DIV16_N8 = 0x0c,
        #[doc = "fSAMPLING=fDTS/32, N=5"]
        FDTS_DIV32_N5 = 0x0d,
        #[doc = "fSAMPLING=fDTS/32, N=6"]
        FDTS_DIV32_N6 = 0x0e,
        #[doc = "fSAMPLING=fDTS/32, N=8"]
        FDTS_DIV32_N8 = 0x0f,
    }
    impl Etf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Etf {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Etf {
        #[inline(always)]
        fn from(val: u8) -> Etf {
            Etf::from_bits(val)
        }
    }
    impl From<Etf> for u8 {
        #[inline(always)]
        fn from(val: Etf) -> u8 {
            Etf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Etp {
        #[doc = "ETR is noninverted, active at high level or rising edge"]
        NOTINVERTED = 0,
        #[doc = "ETR is inverted, active at low level or falling edge"]
        INVERTED = 0x01,
    }
    impl Etp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Etp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Etp {
        #[inline(always)]
        fn from(val: u8) -> Etp {
            Etp::from_bits(val)
        }
    }
    impl From<Etp> for u8 {
        #[inline(always)]
        fn from(val: Etp) -> u8 {
            Etp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Etps {
        #[doc = "Prescaler OFF"]
        DIV1 = 0,
        #[doc = "ETRP frequency divided by 2"]
        DIV2 = 0x01,
        #[doc = "ETRP frequency divided by 4"]
        DIV4 = 0x02,
        #[doc = "ETRP frequency divided by 8"]
        DIV8 = 0x03,
    }
    impl Etps {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Etps {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Etps {
        #[inline(always)]
        fn from(val: u8) -> Etps {
            Etps::from_bits(val)
        }
    }
    impl From<Etps> for u8 {
        #[inline(always)]
        fn from(val: Etps) -> u8 {
            Etps::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Icf {
        #[doc = "No filter, sampling is done at fDTS"]
        NOFILTER = 0,
        #[doc = "fSAMPLING=fCK_INT, N=2"]
        FCK_INT_N2 = 0x01,
        #[doc = "fSAMPLING=fCK_INT, N=4"]
        FCK_INT_N4 = 0x02,
        #[doc = "fSAMPLING=fCK_INT, N=8"]
        FCK_INT_N8 = 0x03,
        #[doc = "fSAMPLING=fDTS/2, N=6"]
        FDTS_DIV2_N6 = 0x04,
        #[doc = "fSAMPLING=fDTS/2, N=8"]
        FDTS_DIV2_N8 = 0x05,
        #[doc = "fSAMPLING=fDTS/4, N=6"]
        FDTS_DIV4_N6 = 0x06,
        #[doc = "fSAMPLING=fDTS/4, N=8"]
        FDTS_DIV4_N8 = 0x07,
        #[doc = "fSAMPLING=fDTS/8, N=6"]
        FDTS_DIV8_N6 = 0x08,
        #[doc = "fSAMPLING=fDTS/8, N=8"]
        FDTS_DIV8_N8 = 0x09,
        #[doc = "fSAMPLING=fDTS/16, N=5"]
        FDTS_DIV16_N5 = 0x0a,
        #[doc = "fSAMPLING=fDTS/16, N=6"]
        FDTS_DIV16_N6 = 0x0b,
        #[doc = "fSAMPLING=fDTS/16, N=8"]
        FDTS_DIV16_N8 = 0x0c,
        #[doc = "fSAMPLING=fDTS/32, N=5"]
        FDTS_DIV32_N5 = 0x0d,
        #[doc = "fSAMPLING=fDTS/32, N=6"]
        FDTS_DIV32_N6 = 0x0e,
        #[doc = "fSAMPLING=fDTS/32, N=8"]
        FDTS_DIV32_N8 = 0x0f,
    }
    impl Icf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Icf {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Icf {
        #[inline(always)]
        fn from(val: u8) -> Icf {
            Icf::from_bits(val)
        }
    }
    impl From<Icf> for u8 {
        #[inline(always)]
        fn from(val: Icf) -> u8 {
            Icf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Mms {
        #[doc = "The UG bit from the TIMx_EGR register is used as trigger output"]
        RESET = 0,
        #[doc = "The counter enable signal, CNT_EN, is used as trigger output"]
        ENABLE = 0x01,
        #[doc = "The update event is selected as trigger output"]
        UPDATE = 0x02,
        #[doc = "The trigger output send a positive pulse when the CC1IF flag it to be set, as soon as a capture or a compare match occurred"]
        COMPAREPULSE = 0x03,
        #[doc = "OC1REF signal is used as trigger output"]
        COMPAREOC1 = 0x04,
        #[doc = "OC2REF signal is used as trigger output"]
        COMPAREOC2 = 0x05,
        #[doc = "OC3REF signal is used as trigger output"]
        COMPAREOC3 = 0x06,
        #[doc = "OC4REF signal is used as trigger output"]
        COMPAREOC4 = 0x07,
    }
    impl Mms {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mms {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mms {
        #[inline(always)]
        fn from(val: u8) -> Mms {
            Mms::from_bits(val)
        }
    }
    impl From<Mms> for u8 {
        #[inline(always)]
        fn from(val: Mms) -> u8 {
            Mms::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Msm {
        #[doc = "No action"]
        NOSYNC = 0,
        #[doc = "The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
        SYNC = 0x01,
    }
    impl Msm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Msm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Msm {
        #[inline(always)]
        fn from(val: u8) -> Msm {
            Msm::from_bits(val)
        }
    }
    impl From<Msm> for u8 {
        #[inline(always)]
        fn from(val: Msm) -> u8 {
            Msm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ocm {
        #[doc = "The comparison between the output compare register TIMx_CCRy and the counter TIMx_CNT has no effect on the outputs"]
        FROZEN = 0,
        #[doc = "Set channel to active level on match. OCyREF signal is forced high when the counter matches the capture/compare register"]
        ACTIVEONMATCH = 0x01,
        #[doc = "Set channel to inactive level on match. OCyREF signal is forced low when the counter matches the capture/compare register"]
        INACTIVEONMATCH = 0x02,
        #[doc = "OCyREF toggles when TIMx_CNT=TIMx_CCRy"]
        TOGGLE = 0x03,
        #[doc = "OCyREF is forced low"]
        FORCEINACTIVE = 0x04,
        #[doc = "OCyREF is forced high"]
        FORCEACTIVE = 0x05,
        #[doc = "In upcounting, channel is active as long as TIMx_CNT<TIMx_CCRy else inactive. In downcounting, channel is inactive as long as TIMx_CNT>TIMx_CCRy else active"]
        PWMMODE1 = 0x06,
        #[doc = "Inversely to PwmMode1"]
        PWMMODE2 = 0x07,
    }
    impl Ocm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ocm {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ocm {
        #[inline(always)]
        fn from(val: u8) -> Ocm {
            Ocm::from_bits(val)
        }
    }
    impl From<Ocm> for u8 {
        #[inline(always)]
        fn from(val: Ocm) -> u8 {
            Ocm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ocpe {
        #[doc = "Preload register on CCR2 disabled. New values written to CCR2 are taken into account immediately"]
        DISABLED = 0,
        #[doc = "Preload register on CCR2 enabled. Preload value is loaded into active register on each update event"]
        ENABLED = 0x01,
    }
    impl Ocpe {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ocpe {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ocpe {
        #[inline(always)]
        fn from(val: u8) -> Ocpe {
            Ocpe::from_bits(val)
        }
    }
    impl From<Ocpe> for u8 {
        #[inline(always)]
        fn from(val: Ocpe) -> u8 {
            Ocpe::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Opm {
        #[doc = "Counter is not stopped at update event"]
        DISABLED = 0,
        #[doc = "Counter stops counting at the next update event (clearing the CEN bit)"]
        ENABLED = 0x01,
    }
    impl Opm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Opm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Opm {
        #[inline(always)]
        fn from(val: u8) -> Opm {
            Opm::from_bits(val)
        }
    }
    impl From<Opm> for u8 {
        #[inline(always)]
        fn from(val: Opm) -> u8 {
            Opm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ossi {
        #[doc = "When inactive, OC/OCN outputs are disabled"]
        DISABLED = 0,
        #[doc = "When inactive, OC/OCN outputs are forced to idle level"]
        IDLELEVEL = 0x01,
    }
    impl Ossi {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ossi {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ossi {
        #[inline(always)]
        fn from(val: u8) -> Ossi {
            Ossi::from_bits(val)
        }
    }
    impl From<Ossi> for u8 {
        #[inline(always)]
        fn from(val: Ossi) -> u8 {
            Ossi::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ossr {
        #[doc = "When inactive, OC/OCN outputs are disabled"]
        DISABLED = 0,
        #[doc = "When inactive, OC/OCN outputs are enabled with their inactive level"]
        IDLELEVEL = 0x01,
    }
    impl Ossr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ossr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ossr {
        #[inline(always)]
        fn from(val: u8) -> Ossr {
            Ossr::from_bits(val)
        }
    }
    impl From<Ossr> for u8 {
        #[inline(always)]
        fn from(val: Ossr) -> u8 {
            Ossr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Sms {
        #[doc = "Slave mode disabled - if CEN = ‘1 then the prescaler is clocked directly by the internal clock."]
        DISABLED = 0,
        #[doc = "Encoder mode 1 - Counter counts up/down on TI2FP1 edge depending on TI1FP2 level."]
        ENCODER_MODE_1 = 0x01,
        #[doc = "Encoder mode 2 - Counter counts up/down on TI1FP2 edge depending on TI2FP1 level."]
        ENCODER_MODE_2 = 0x02,
        #[doc = "Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input."]
        ENCODER_MODE_3 = 0x03,
        #[doc = "Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers."]
        RESET_MODE = 0x04,
        #[doc = "Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled."]
        GATED_MODE = 0x05,
        #[doc = "Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled."]
        TRIGGER_MODE = 0x06,
        #[doc = "External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter."]
        EXT_CLOCK_MODE = 0x07,
    }
    impl Sms {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sms {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sms {
        #[inline(always)]
        fn from(val: u8) -> Sms {
            Sms::from_bits(val)
        }
    }
    impl From<Sms> for u8 {
        #[inline(always)]
        fn from(val: Sms) -> u8 {
            Sms::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Tis {
        #[doc = "The TIMx_CH1 pin is connected to TI1 input"]
        NORMAL = 0,
        #[doc = "The TIMx_CH1, CH2, CH3 pins are connected to TI1 input"]
        XOR = 0x01,
    }
    impl Tis {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tis {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tis {
        #[inline(always)]
        fn from(val: u8) -> Tis {
            Tis::from_bits(val)
        }
    }
    impl From<Tis> for u8 {
        #[inline(always)]
        fn from(val: Tis) -> u8 {
            Tis::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ts {
        #[doc = "Internal Trigger 0 (ITR0)"]
        ITR0 = 0,
        #[doc = "Internal Trigger 1 (ITR1)"]
        ITR1 = 0x01,
        #[doc = "Internal Trigger 2 (ITR2)"]
        ITR2 = 0x02,
        #[doc = "Internal Trigger 3 (ITR3)"]
        ITR3 = 0x03,
        #[doc = "TI1 Edge Detector (TI1F_ED)"]
        TI1F_ED = 0x04,
        #[doc = "Filtered Timer Input 1 (TI1FP1)"]
        TI1FP1 = 0x05,
        #[doc = "Filtered Timer Input 2 (TI2FP2)"]
        TI2FP2 = 0x06,
        #[doc = "External Trigger input (ETRF)"]
        ETRF = 0x07,
    }
    impl Ts {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ts {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ts {
        #[inline(always)]
        fn from(val: u8) -> Ts {
            Ts::from_bits(val)
        }
    }
    impl From<Ts> for u8 {
        #[inline(always)]
        fn from(val: Ts) -> u8 {
            Ts::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Urs {
        #[doc = "Any of counter overflow/underflow, setting UG, or update through slave mode, generates an update interrupt or DMA request"]
        ANYEVENT = 0,
        #[doc = "Only counter overflow/underflow generates an update interrupt or DMA request"]
        COUNTERONLY = 0x01,
    }
    impl Urs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Urs {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Urs {
        #[inline(always)]
        fn from(val: u8) -> Urs {
            Urs::from_bits(val)
        }
    }
    impl From<Urs> for u8 {
        #[inline(always)]
        fn from(val: Urs) -> u8 {
            Urs::to_bits(val)
        }
    }
}
