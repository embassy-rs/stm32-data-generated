#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Tamper and backup."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tamp {
    ptr: *mut u8,
}
unsafe impl Send for Tamp {}
unsafe impl Sync for Tamp {}
impl Tamp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "TAMP control register 1."]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "TAMP control register 2."]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "TAMP control register 3."]
    #[inline(always)]
    pub const fn cr3(self) -> crate::common::Reg<regs::Cr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "TAMP filter control register."]
    #[inline(always)]
    pub const fn fltcr(self) -> crate::common::Reg<regs::Fltcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "TAMP active tamper control register 1."]
    #[inline(always)]
    pub const fn atcr1(self) -> crate::common::Reg<regs::Atcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "TAMP active tamper seed register."]
    #[inline(always)]
    pub const fn atseedr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "TAMP active tamper output register."]
    #[inline(always)]
    pub const fn ator(self) -> crate::common::Reg<regs::Ator, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "TAMP active tamper control register 2."]
    #[inline(always)]
    pub const fn atcr2(self) -> crate::common::Reg<regs::Atcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "TAMP secure mode register."]
    #[inline(always)]
    pub const fn seccfgr(self) -> crate::common::Reg<regs::Seccfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "TAMP privilege mode control register."]
    #[inline(always)]
    pub const fn privcfgr(self) -> crate::common::Reg<regs::Privcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "TAMP interrupt enable register."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "TAMP status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "TAMP non-secure masked interrupt status register."]
    #[inline(always)]
    pub const fn misr(self) -> crate::common::Reg<regs::Misr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "TAMP secure masked interrupt status register."]
    #[inline(always)]
    pub const fn smisr(self) -> crate::common::Reg<regs::Smisr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "TAMP status clear register."]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::Scr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "TAMP monotonic counter 1 register."]
    #[inline(always)]
    pub const fn count1r(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "TAMP option register."]
    #[inline(always)]
    pub const fn or(self) -> crate::common::Reg<regs::Or, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "TAMP resources protection configuration register."]
    #[inline(always)]
    pub const fn rpcfgr(self) -> crate::common::Reg<regs::Rpcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "TAMP backup x register. (x=0-31)"]
    #[inline(always)]
    pub const fn bkpr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "TAMP active tamper control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Atcr1(pub u32);
    impl Atcr1 {
        #[doc = "Tamper x active mode. (x=1-8)"]
        #[inline(always)]
        pub const fn tampam(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper x active mode. (x=1-8)"]
        #[inline(always)]
        pub fn set_tampam(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Active tamper shared output x selection The selected output must be available in the package pinout. (x=1-4)"]
        #[inline(always)]
        pub const fn atosel(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 8usize + n * 2usize;
            let val = (self.0 >> offs) & 0x03;
            val as u8
        }
        #[doc = "Active tamper shared output x selection The selected output must be available in the package pinout. (x=1-4)"]
        #[inline(always)]
        pub fn set_atosel(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 8usize + n * 2usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
        }
        #[doc = "Active tamper RTC asynchronous prescaler clock selection These bits selects the RTC asynchronous prescaler stage output.The selected clock is CK_ATPRE. fCK_ATPRE = fRTCCLK / 2ATCKSEL when (PREDIV_A+1) = 128. ... These bits can be written only when all active tampers are disabled. The write protection remains for up to 1.5 ck_atpre cycles after all the active tampers are disable."]
        #[inline(always)]
        pub const fn atcksel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Active tamper RTC asynchronous prescaler clock selection These bits selects the RTC asynchronous prescaler stage output.The selected clock is CK_ATPRE. fCK_ATPRE = fRTCCLK / 2ATCKSEL when (PREDIV_A+1) = 128. ... These bits can be written only when all active tampers are disabled. The write protection remains for up to 1.5 ck_atpre cycles after all the active tampers are disable."]
        #[inline(always)]
        pub fn set_atcksel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "Active tamper output change period The tamper output is changed every CK_ATPER = (2ATPER x CK_ATPRE) cycles. Refer to."]
        #[inline(always)]
        pub const fn atper(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Active tamper output change period The tamper output is changed every CK_ATPER = (2ATPER x CK_ATPRE) cycles. Refer to."]
        #[inline(always)]
        pub fn set_atper(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "Active tamper output sharing TAMP_IN1 is compared with TAMPOUTSEL1 TAMP_IN2 is compared with TAMPOUTSEL2 TAMP_IN3 is compared with TAMPOUTSEL3 TAMP_IN4 is compared with TAMPOUTSEL4 TAMP_IN5 is compared with TAMPOUTSEL5 TAMP_IN6 is compared with TAMPOUTSEL6 TAMP_IN7 is compared with TAMPOUTSEL7 TAMP_IN8 is compared with TAMPOUTSEL8."]
        #[inline(always)]
        pub const fn atoshare(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Active tamper output sharing TAMP_IN1 is compared with TAMPOUTSEL1 TAMP_IN2 is compared with TAMPOUTSEL2 TAMP_IN3 is compared with TAMPOUTSEL3 TAMP_IN4 is compared with TAMPOUTSEL4 TAMP_IN5 is compared with TAMPOUTSEL5 TAMP_IN6 is compared with TAMPOUTSEL6 TAMP_IN7 is compared with TAMPOUTSEL7 TAMP_IN8 is compared with TAMPOUTSEL8."]
        #[inline(always)]
        pub fn set_atoshare(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Active tamper filter enable."]
        #[inline(always)]
        pub const fn flten(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Active tamper filter enable."]
        #[inline(always)]
        pub fn set_flten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Atcr1 {
        #[inline(always)]
        fn default() -> Atcr1 {
            Atcr1(0)
        }
    }
    #[doc = "TAMP active tamper control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Atcr2(pub u32);
    impl Atcr2 {
        #[doc = "Active tamper shared output x selection The selected output must be available in the package pinout. Bits 9:8 are the mirror of ATOSELx\\[1:0\\]
in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1. (x=1-8)"]
        #[inline(always)]
        pub const fn atosel(&self, n: usize) -> u8 {
            assert!(n < 8usize);
            let offs = 8usize + n * 3usize;
            let val = (self.0 >> offs) & 0x07;
            val as u8
        }
        #[doc = "Active tamper shared output x selection The selected output must be available in the package pinout. Bits 9:8 are the mirror of ATOSELx\\[1:0\\]
in the TAMP_ATCR1, and so can also be read or written through TAMP_ATCR1. (x=1-8)"]
        #[inline(always)]
        pub fn set_atosel(&mut self, n: usize, val: u8) {
            assert!(n < 8usize);
            let offs = 8usize + n * 3usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val as u32) & 0x07) << offs);
        }
    }
    impl Default for Atcr2 {
        #[inline(always)]
        fn default() -> Atcr2 {
            Atcr2(0)
        }
    }
    #[doc = "TAMP active tamper output register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ator(pub u32);
    impl Ator {
        #[doc = "Pseudo-random generator value This field provides the values of the PRNG output. Because of potential inconsistencies due to synchronization delays, PRNG must be read at least twice. The read value is correct if it is equal to previous read value. This field can only be read when the APB is in secure mode."]
        #[inline(always)]
        pub const fn prng(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Pseudo-random generator value This field provides the values of the PRNG output. Because of potential inconsistencies due to synchronization delays, PRNG must be read at least twice. The read value is correct if it is equal to previous read value. This field can only be read when the APB is in secure mode."]
        #[inline(always)]
        pub fn set_prng(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Seed running flag This flag is set by hardware when a new seed is written in the TAMP_ATSEEDR. It is cleared by hardware when the PRNG has absorbed this new seed, and by system reset. The TAMP APB cock must not be switched off as long as SEEDF is set."]
        #[inline(always)]
        pub const fn seedf(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Seed running flag This flag is set by hardware when a new seed is written in the TAMP_ATSEEDR. It is cleared by hardware when the PRNG has absorbed this new seed, and by system reset. The TAMP APB cock must not be switched off as long as SEEDF is set."]
        #[inline(always)]
        pub fn set_seedf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Active tamper initialization status This flag is set by hardware when the PRNG has absorbed the first 128-bit seed, meaning that the enabled active tampers are functional. This flag is cleared when the active tampers are disabled."]
        #[inline(always)]
        pub const fn inits(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Active tamper initialization status This flag is set by hardware when the PRNG has absorbed the first 128-bit seed, meaning that the enabled active tampers are functional. This flag is cleared when the active tampers are disabled."]
        #[inline(always)]
        pub fn set_inits(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Ator {
        #[inline(always)]
        fn default() -> Ator {
            Ator(0)
        }
    }
    #[doc = "TAMP control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "Tamper detection on TAMP_INx enable. (x=1-8)"]
        #[inline(always)]
        pub const fn tampe(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper detection on TAMP_INx enable. (x=1-8)"]
        #[inline(always)]
        pub fn set_tampe(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Internal tamper 1 enable."]
        #[inline(always)]
        pub const fn itamp1e(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 1 enable."]
        #[inline(always)]
        pub fn set_itamp1e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Internal tamper 2 enable."]
        #[inline(always)]
        pub const fn itamp2e(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 2 enable."]
        #[inline(always)]
        pub fn set_itamp2e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Internal tamper 3 enable."]
        #[inline(always)]
        pub const fn itamp3e(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 3 enable."]
        #[inline(always)]
        pub fn set_itamp3e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Internal tamper 4 enable."]
        #[inline(always)]
        pub const fn itamp4e(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 4 enable."]
        #[inline(always)]
        pub fn set_itamp4e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Internal tamper 5 enable."]
        #[inline(always)]
        pub const fn itamp5e(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 5 enable."]
        #[inline(always)]
        pub fn set_itamp5e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Internal tamper 6 enable."]
        #[inline(always)]
        pub const fn itamp6e(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 6 enable."]
        #[inline(always)]
        pub fn set_itamp6e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Internal tamper 7 enable."]
        #[inline(always)]
        pub const fn itamp7e(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 7 enable."]
        #[inline(always)]
        pub fn set_itamp7e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Internal tamper 8 enable."]
        #[inline(always)]
        pub const fn itamp8e(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 8 enable."]
        #[inline(always)]
        pub fn set_itamp8e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Internal tamper 9 enable."]
        #[inline(always)]
        pub const fn itamp9e(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 9 enable."]
        #[inline(always)]
        pub fn set_itamp9e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Internal tamper 11 enable."]
        #[inline(always)]
        pub const fn itamp11e(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 11 enable."]
        #[inline(always)]
        pub fn set_itamp11e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Internal tamper 12 enable."]
        #[inline(always)]
        pub const fn itamp12e(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 12 enable."]
        #[inline(always)]
        pub fn set_itamp12e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Internal tamper 13 enable."]
        #[inline(always)]
        pub const fn itamp13e(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 13 enable."]
        #[inline(always)]
        pub fn set_itamp13e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Internal tamper 15 enable."]
        #[inline(always)]
        pub const fn itamp15e(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 15 enable."]
        #[inline(always)]
        pub fn set_itamp15e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Cr1 {
        #[inline(always)]
        fn default() -> Cr1 {
            Cr1(0)
        }
    }
    #[doc = "TAMP control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "Tamper x potential mode. (x=1-8)"]
        #[inline(always)]
        pub const fn tamppom(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper x potential mode. (x=1-8)"]
        #[inline(always)]
        pub fn set_tamppom(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Tamper x mask. The tamper x interrupt must not be enabled when TAMPxMSK is set. (x=1-3)"]
        #[inline(always)]
        pub const fn tampmsk(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper x mask. The tamper x interrupt must not be enabled when TAMPxMSK is set. (x=1-3)"]
        #[inline(always)]
        pub fn set_tampmsk(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Backup registers and device secrets access blocked."]
        #[inline(always)]
        pub const fn bkblock(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Backup registers and device secrets access blocked."]
        #[inline(always)]
        pub fn set_bkblock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Backup registers and device secrets erase Writing ‘1’ to this bit reset the backup registers and device secrets(1). Writing 0 has no effect. This bit is always read as 0."]
        #[inline(always)]
        pub const fn bkerase(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Backup registers and device secrets erase Writing ‘1’ to this bit reset the backup registers and device secrets(1). Writing 0 has no effect. This bit is always read as 0."]
        #[inline(always)]
        pub fn set_bkerase(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Active level for tamper x input If TAMPFLT = 00 Tamper x input rising edge triggers a tamper detection event. If TAMPFLT = 00 Tamper x input falling edge triggers a tamper detection event. (x=1-8)"]
        #[inline(always)]
        pub const fn tamptrg(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 24usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Active level for tamper x input If TAMPFLT = 00 Tamper x input rising edge triggers a tamper detection event. If TAMPFLT = 00 Tamper x input falling edge triggers a tamper detection event. (x=1-8)"]
        #[inline(always)]
        pub fn set_tamptrg(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 24usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Cr2 {
        #[inline(always)]
        fn default() -> Cr2 {
            Cr2(0)
        }
    }
    #[doc = "TAMP control register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr3(pub u32);
    impl Cr3 {
        #[doc = "Internal tamper 1 potential mode."]
        #[inline(always)]
        pub const fn itamp1pom(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 1 potential mode."]
        #[inline(always)]
        pub fn set_itamp1pom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Internal tamper 2 potential mode."]
        #[inline(always)]
        pub const fn itamp2pom(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 2 potential mode."]
        #[inline(always)]
        pub fn set_itamp2pom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Internal tamper 3 potential mode."]
        #[inline(always)]
        pub const fn itamp3pom(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 3 potential mode."]
        #[inline(always)]
        pub fn set_itamp3pom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Internal tamper 4 potential mode."]
        #[inline(always)]
        pub const fn itamp4pom(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 4 potential mode."]
        #[inline(always)]
        pub fn set_itamp4pom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Internal tamper 5 potential mode."]
        #[inline(always)]
        pub const fn itamp5pom(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 5 potential mode."]
        #[inline(always)]
        pub fn set_itamp5pom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Internal tamper 6 potential mode."]
        #[inline(always)]
        pub const fn itamp6pom(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 6 potential mode."]
        #[inline(always)]
        pub fn set_itamp6pom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Internal tamper 7 potential mode."]
        #[inline(always)]
        pub const fn itamp7pom(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 7 potential mode."]
        #[inline(always)]
        pub fn set_itamp7pom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Internal tamper 8 potential mode."]
        #[inline(always)]
        pub const fn itamp8pom(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 8 potential mode."]
        #[inline(always)]
        pub fn set_itamp8pom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Internal tamper 9 potential mode."]
        #[inline(always)]
        pub const fn itamp9pom(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 9 potential mode."]
        #[inline(always)]
        pub fn set_itamp9pom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Internal tamper 11 potential mode."]
        #[inline(always)]
        pub const fn itamp11pom(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 11 potential mode."]
        #[inline(always)]
        pub fn set_itamp11pom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Internal tamper 12 potential mode."]
        #[inline(always)]
        pub const fn itamp12pom(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 12 potential mode."]
        #[inline(always)]
        pub fn set_itamp12pom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Internal tamper 13 potential mode."]
        #[inline(always)]
        pub const fn itamp13pom(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 13 potential mode."]
        #[inline(always)]
        pub fn set_itamp13pom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Internal tamper 15 potential mode."]
        #[inline(always)]
        pub const fn itamp15pom(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 15 potential mode."]
        #[inline(always)]
        pub fn set_itamp15pom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Cr3 {
        #[inline(always)]
        fn default() -> Cr3 {
            Cr3(0)
        }
    }
    #[doc = "TAMP filter control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fltcr(pub u32);
    impl Fltcr {
        #[doc = "Tamper sampling frequency Determines the frequency at which each of the TAMP_INx inputs are sampled."]
        #[inline(always)]
        pub const fn tampfreq(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Tamper sampling frequency Determines the frequency at which each of the TAMP_INx inputs are sampled."]
        #[inline(always)]
        pub fn set_tampfreq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "TAMP_INx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the TAMP_INx inputs."]
        #[inline(always)]
        pub const fn tampflt(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "TAMP_INx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the TAMP_INx inputs."]
        #[inline(always)]
        pub fn set_tampflt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "TAMP_INx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the TAMP_INx inputs."]
        #[inline(always)]
        pub const fn tampprch(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "TAMP_INx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the TAMP_INx inputs."]
        #[inline(always)]
        pub fn set_tampprch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "TAMP_INx pull-up disable This bit determines if each of the TAMPx pins are precharged before each sample."]
        #[inline(always)]
        pub const fn tamppudis(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TAMP_INx pull-up disable This bit determines if each of the TAMPx pins are precharged before each sample."]
        #[inline(always)]
        pub fn set_tamppudis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Fltcr {
        #[inline(always)]
        fn default() -> Fltcr {
            Fltcr(0)
        }
    }
    #[doc = "TAMP interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "Tamper x interrupt enable. (x=1-8)"]
        #[inline(always)]
        pub const fn tampie(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper x interrupt enable. (x=1-8)"]
        #[inline(always)]
        pub fn set_tampie(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Internal tamper 1 interrupt enable."]
        #[inline(always)]
        pub const fn itamp1ie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 1 interrupt enable."]
        #[inline(always)]
        pub fn set_itamp1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Internal tamper 2 interrupt enable."]
        #[inline(always)]
        pub const fn itamp2ie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 2 interrupt enable."]
        #[inline(always)]
        pub fn set_itamp2ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Internal tamper 3 interrupt enable."]
        #[inline(always)]
        pub const fn itamp3ie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 3 interrupt enable."]
        #[inline(always)]
        pub fn set_itamp3ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Internal tamper 4 interrupt enable."]
        #[inline(always)]
        pub const fn itamp4ie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 4 interrupt enable."]
        #[inline(always)]
        pub fn set_itamp4ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Internal tamper 5 interrupt enable."]
        #[inline(always)]
        pub const fn itamp5ie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 5 interrupt enable."]
        #[inline(always)]
        pub fn set_itamp5ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Internal tamper 6 interrupt enable."]
        #[inline(always)]
        pub const fn itamp6ie(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 6 interrupt enable."]
        #[inline(always)]
        pub fn set_itamp6ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Internal tamper 7 interrupt enable."]
        #[inline(always)]
        pub const fn itamp7ie(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 7 interrupt enable."]
        #[inline(always)]
        pub fn set_itamp7ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Internal tamper 8 interrupt enable."]
        #[inline(always)]
        pub const fn itamp8ie(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 8 interrupt enable."]
        #[inline(always)]
        pub fn set_itamp8ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Internal tamper 9 interrupt enable."]
        #[inline(always)]
        pub const fn itamp9ie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 9 interrupt enable."]
        #[inline(always)]
        pub fn set_itamp9ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Internal tamper 11 interrupt enable."]
        #[inline(always)]
        pub const fn itamp11ie(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 11 interrupt enable."]
        #[inline(always)]
        pub fn set_itamp11ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Internal tamper 12 interrupt enable."]
        #[inline(always)]
        pub const fn itamp12ie(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 12 interrupt enable."]
        #[inline(always)]
        pub fn set_itamp12ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Internal tamper 13 interrupt enable."]
        #[inline(always)]
        pub const fn itamp13ie(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 13 interrupt enable."]
        #[inline(always)]
        pub fn set_itamp13ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Internal tamper 15 interrupt enable."]
        #[inline(always)]
        pub const fn itamp15ie(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 15 interrupt enable."]
        #[inline(always)]
        pub fn set_itamp15ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Ier {
        #[inline(always)]
        fn default() -> Ier {
            Ier(0)
        }
    }
    #[doc = "TAMP non-secure masked interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Misr(pub u32);
    impl Misr {
        #[doc = "TAMP1 non-secure interrupt masked flag This flag is set by hardware when the tamper 1 non-secure interrupt is raised."]
        #[inline(always)]
        pub const fn tampmf(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "TAMP1 non-secure interrupt masked flag This flag is set by hardware when the tamper 1 non-secure interrupt is raised."]
        #[inline(always)]
        pub fn set_tampmf(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Internal tamper 1 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 1 non-secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp1mf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 1 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 1 non-secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp1mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Internal tamper 2 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 2 non-secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp2mf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 2 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 2 non-secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp2mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Internal tamper 3 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 3 non-secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp3mf(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 3 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 3 non-secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp3mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Internal tamper 4 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 4 non-secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp4mf(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 4 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 4 non-secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp4mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Internal tamper 5 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 5 non-secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp5mf(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 5 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 5 non-secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp5mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Internal tamper 6 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 6 non-secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp6mf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 6 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 6 non-secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp6mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Internal tamper 7 tamper non-secure interrupt masked flag This flag is set by hardware when the internal tamper 7 non-secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp7mf(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 7 tamper non-secure interrupt masked flag This flag is set by hardware when the internal tamper 7 non-secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp7mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Internal tamper 8 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 8 non-secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp8mf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 8 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 8 non-secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp8mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "internal tamper 9 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 9 non-secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp9mf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "internal tamper 9 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 9 non-secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp9mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "internal tamper 11 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 11 non-secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp11mf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "internal tamper 11 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 11 non-secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp11mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "internal tamper 12 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 12 non-secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp12mf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "internal tamper 12 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 12 non-secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp12mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "internal tamper 13 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 13 non-secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp13mf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "internal tamper 13 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 13 non-secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp13mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "internal tamper 15 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 15 non-secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp15mf(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "internal tamper 15 non-secure interrupt masked flag This flag is set by hardware when the internal tamper 15 non-secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp15mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Misr {
        #[inline(always)]
        fn default() -> Misr {
            Misr(0)
        }
    }
    #[doc = "TAMP option register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Or(pub u32);
    impl Or {
        #[doc = "TAMP_OUT3 mapping."]
        #[inline(always)]
        pub const fn out3_rmp(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "TAMP_OUT3 mapping."]
        #[inline(always)]
        pub fn set_out3_rmp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "TAMP_OUT5 mapping."]
        #[inline(always)]
        pub const fn out5_rmp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TAMP_OUT5 mapping."]
        #[inline(always)]
        pub fn set_out5_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TAMP_IN2 mapping."]
        #[inline(always)]
        pub const fn in2_rmp(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TAMP_IN2 mapping."]
        #[inline(always)]
        pub fn set_in2_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "TAMP_IN3 mapping."]
        #[inline(always)]
        pub const fn in3_rmp(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "TAMP_IN3 mapping."]
        #[inline(always)]
        pub fn set_in3_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "TAMP_IN4 mapping."]
        #[inline(always)]
        pub const fn in4_rmp(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "TAMP_IN4 mapping."]
        #[inline(always)]
        pub fn set_in4_rmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Or {
        #[inline(always)]
        fn default() -> Or {
            Or(0)
        }
    }
    #[doc = "TAMP privilege mode control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcfgr(pub u32);
    impl Privcfgr {
        #[doc = "Monotonic counter 1 privilege protection."]
        #[inline(always)]
        pub const fn cnt1priv(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Monotonic counter 1 privilege protection."]
        #[inline(always)]
        pub fn set_cnt1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Backup registers zone 1 privilege protection."]
        #[inline(always)]
        pub const fn bkprwpriv(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Backup registers zone 1 privilege protection."]
        #[inline(always)]
        pub fn set_bkprwpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Backup registers zone 2 privilege protection."]
        #[inline(always)]
        pub const fn bkpwpriv(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Backup registers zone 2 privilege protection."]
        #[inline(always)]
        pub fn set_bkpwpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Tamper privilege protection (excluding backup registers) Note: Refer to for details on the read protection."]
        #[inline(always)]
        pub const fn tamppriv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Tamper privilege protection (excluding backup registers) Note: Refer to for details on the read protection."]
        #[inline(always)]
        pub fn set_tamppriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Privcfgr {
        #[inline(always)]
        fn default() -> Privcfgr {
            Privcfgr(0)
        }
    }
    #[doc = "TAMP erase configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rpcfgr(pub u32);
    impl Rpcfgr {
        #[doc = "Configurable resource 0 protection."]
        #[inline(always)]
        pub const fn rpcfg0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Configurable resource 0 protection."]
        #[inline(always)]
        pub fn set_rpcfg0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Rpcfgr {
        #[inline(always)]
        fn default() -> Rpcfgr {
            Rpcfgr(0)
        }
    }
    #[doc = "TAMP status clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scr(pub u32);
    impl Scr {
        #[doc = "Clear TAMPx detection flag. Writing 1 in this bit clears the TAMPxF bit in the TAMP_SR register. (x=1-8)"]
        #[inline(always)]
        pub const fn ctampf(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Clear TAMPx detection flag. Writing 1 in this bit clears the TAMPxF bit in the TAMP_SR register. (x=1-8)"]
        #[inline(always)]
        pub fn set_ctampf(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Clear ITAMP1 detection flag Writing 1 in this bit clears the ITAMP1F bit in the TAMP_SR register."]
        #[inline(always)]
        pub const fn citamp1f(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ITAMP1 detection flag Writing 1 in this bit clears the ITAMP1F bit in the TAMP_SR register."]
        #[inline(always)]
        pub fn set_citamp1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Clear ITAMP2 detection flag Writing 1 in this bit clears the ITAMP2F bit in the TAMP_SR register."]
        #[inline(always)]
        pub const fn citamp2f(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ITAMP2 detection flag Writing 1 in this bit clears the ITAMP2F bit in the TAMP_SR register."]
        #[inline(always)]
        pub fn set_citamp2f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Clear ITAMP3 detection flag Writing 1 in this bit clears the ITAMP3F bit in the TAMP_SR register."]
        #[inline(always)]
        pub const fn citamp3f(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ITAMP3 detection flag Writing 1 in this bit clears the ITAMP3F bit in the TAMP_SR register."]
        #[inline(always)]
        pub fn set_citamp3f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Clear ITAMP4 detection flag Writing 1 in this bit clears the ITAMP4F bit in the TAMP_SR register."]
        #[inline(always)]
        pub const fn citamp4f(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ITAMP4 detection flag Writing 1 in this bit clears the ITAMP4F bit in the TAMP_SR register."]
        #[inline(always)]
        pub fn set_citamp4f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Clear ITAMP5 detection flag Writing 1 in this bit clears the ITAMP5F bit in the TAMP_SR register."]
        #[inline(always)]
        pub const fn citamp5f(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ITAMP5 detection flag Writing 1 in this bit clears the ITAMP5F bit in the TAMP_SR register."]
        #[inline(always)]
        pub fn set_citamp5f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Clear ITAMP6 detection flag Writing 1 in this bit clears the ITAMP6F bit in the TAMP_SR register."]
        #[inline(always)]
        pub const fn citamp6f(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ITAMP6 detection flag Writing 1 in this bit clears the ITAMP6F bit in the TAMP_SR register."]
        #[inline(always)]
        pub fn set_citamp6f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Clear ITAMP7 detection flag Writing 1 in this bit clears the ITAMP7F bit in the TAMP_SR register."]
        #[inline(always)]
        pub const fn citamp7f(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ITAMP7 detection flag Writing 1 in this bit clears the ITAMP7F bit in the TAMP_SR register."]
        #[inline(always)]
        pub fn set_citamp7f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Clear ITAMP8 detection flag Writing 1 in this bit clears the ITAMP8F bit in the TAMP_SR register."]
        #[inline(always)]
        pub const fn citamp8f(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ITAMP8 detection flag Writing 1 in this bit clears the ITAMP8F bit in the TAMP_SR register."]
        #[inline(always)]
        pub fn set_citamp8f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Clear ITAMP9 detection flag Writing 1 in this bit clears the ITAMP9F bit in the TAMP_SR register."]
        #[inline(always)]
        pub const fn citamp9f(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ITAMP9 detection flag Writing 1 in this bit clears the ITAMP9F bit in the TAMP_SR register."]
        #[inline(always)]
        pub fn set_citamp9f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Clear ITAMP11 detection flag Writing 1 in this bit clears the ITAMP11F bit in the TAMP_SR register."]
        #[inline(always)]
        pub const fn citamp11f(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ITAMP11 detection flag Writing 1 in this bit clears the ITAMP11F bit in the TAMP_SR register."]
        #[inline(always)]
        pub fn set_citamp11f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Clear ITAMP12 detection flag Writing 1 in this bit clears the ITAMP12F bit in the TAMP_SR register."]
        #[inline(always)]
        pub const fn citamp12f(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ITAMP12 detection flag Writing 1 in this bit clears the ITAMP12F bit in the TAMP_SR register."]
        #[inline(always)]
        pub fn set_citamp12f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Clear ITAMP13 detection flag Writing 1 in this bit clears the ITAMP13F bit in the TAMP_SR register."]
        #[inline(always)]
        pub const fn citamp13f(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ITAMP13 detection flag Writing 1 in this bit clears the ITAMP13F bit in the TAMP_SR register."]
        #[inline(always)]
        pub fn set_citamp13f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Clear ITAMP15 detection flag Writing 1 in this bit clears the ITAMP15F bit in the TAMP_SR register."]
        #[inline(always)]
        pub const fn citamp15f(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ITAMP15 detection flag Writing 1 in this bit clears the ITAMP15F bit in the TAMP_SR register."]
        #[inline(always)]
        pub fn set_citamp15f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Scr {
        #[inline(always)]
        fn default() -> Scr {
            Scr(0)
        }
    }
    #[doc = "TAMP secure mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccfgr(pub u32);
    impl Seccfgr {
        #[doc = "Backup registers read/write protection offset Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRWSEC-1, from 0 to 128). if TZEN=1, these backup registers can be read and written only with secure access. If TZEN=0: the protection zone 1 can be read and written with non-secure access. If BKPRWSEC = 0: there is no protection zone 1. If BKPRWPRIV is set, BKPRWSEC\\[7:0\\]
can be written only in privileged mode."]
        #[inline(always)]
        pub const fn bkprwsec(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Backup registers read/write protection offset Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRWSEC-1, from 0 to 128). if TZEN=1, these backup registers can be read and written only with secure access. If TZEN=0: the protection zone 1 can be read and written with non-secure access. If BKPRWSEC = 0: there is no protection zone 1. If BKPRWPRIV is set, BKPRWSEC\\[7:0\\]
can be written only in privileged mode."]
        #[inline(always)]
        pub fn set_bkprwsec(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Monotonic counter 1 secure protection."]
        #[inline(always)]
        pub const fn cnt1sec(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Monotonic counter 1 secure protection."]
        #[inline(always)]
        pub fn set_cnt1sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Backup registers write protection offset Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRWSEC, from 0 to 128) to TAMP_BKPzR (z = BKPWSEC-1, from 0 to 128, BKPWSEC ≥ BKPRWSEC): if TZEN=1, these backup registers can be written only with secure access. They can be read with secure or non-secure access. Protection zone 3 defined for backup registers from TAMP_BKPtR (t = BKPWSEC, from 0 to 127). They can be read or written with secure or non-secure access. If TZEN=0: the protection zone 2 can be read and written with non-secure access. If BKPWSEC = 0 or if BKPWSEC ≤ BKPRWSEC: there is no protection zone 2. If BKPWPRIV is set, BKPRWSEC\\[7:0\\]
can be written only in privileged mode."]
        #[inline(always)]
        pub const fn bkpwsec(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Backup registers write protection offset Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRWSEC, from 0 to 128) to TAMP_BKPzR (z = BKPWSEC-1, from 0 to 128, BKPWSEC ≥ BKPRWSEC): if TZEN=1, these backup registers can be written only with secure access. They can be read with secure or non-secure access. Protection zone 3 defined for backup registers from TAMP_BKPtR (t = BKPWSEC, from 0 to 127). They can be read or written with secure or non-secure access. If TZEN=0: the protection zone 2 can be read and written with non-secure access. If BKPWSEC = 0 or if BKPWSEC ≤ BKPRWSEC: there is no protection zone 2. If BKPWPRIV is set, BKPRWSEC\\[7:0\\]
can be written only in privileged mode."]
        #[inline(always)]
        pub fn set_bkpwsec(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Boot hardware key lock This bit can be read and can only be written to 1 by software. It is cleared by hardware together with the backup registers following a tamper detection event or when the readout protection (RDP) is disabled."]
        #[inline(always)]
        pub const fn bhklock(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Boot hardware key lock This bit can be read and can only be written to 1 by software. It is cleared by hardware together with the backup registers following a tamper detection event or when the readout protection (RDP) is disabled."]
        #[inline(always)]
        pub fn set_bhklock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Tamper protection (excluding monotonic counters and backup registers) Note: Refer to for details on the read protection."]
        #[inline(always)]
        pub const fn tampsec(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Tamper protection (excluding monotonic counters and backup registers) Note: Refer to for details on the read protection."]
        #[inline(always)]
        pub fn set_tampsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Seccfgr {
        #[inline(always)]
        fn default() -> Seccfgr {
            Seccfgr(0)
        }
    }
    #[doc = "TAMP secure masked interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smisr(pub u32);
    impl Smisr {
        #[doc = "TAMPx secure interrupt masked flag. This flag is set by hardware when the tamper x secure interrupt is raised. (x=1-8)"]
        #[inline(always)]
        pub const fn tampmf(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "TAMPx secure interrupt masked flag. This flag is set by hardware when the tamper x secure interrupt is raised. (x=1-8)"]
        #[inline(always)]
        pub fn set_tampmf(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Internal tamper 1 secure interrupt masked flag This flag is set by hardware when the internal tamper 1 secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp1mf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 1 secure interrupt masked flag This flag is set by hardware when the internal tamper 1 secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp1mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Internal tamper 2 secure interrupt masked flag This flag is set by hardware when the internal tamper 2 secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp2mf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 2 secure interrupt masked flag This flag is set by hardware when the internal tamper 2 secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp2mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Internal tamper 3 secure interrupt masked flag This flag is set by hardware when the internal tamper 3 secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp3mf(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 3 secure interrupt masked flag This flag is set by hardware when the internal tamper 3 secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp3mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Internal tamper 4 secure interrupt masked flag This flag is set by hardware when the internal tamper 4 secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp4mf(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 4 secure interrupt masked flag This flag is set by hardware when the internal tamper 4 secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp4mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Internal tamper 5 secure interrupt masked flag This flag is set by hardware when the internal tamper 5 secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp5mf(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 5 secure interrupt masked flag This flag is set by hardware when the internal tamper 5 secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp5mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Internal tamper 6 secure interrupt masked flag This flag is set by hardware when the internal tamper 6 secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp6mf(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 6 secure interrupt masked flag This flag is set by hardware when the internal tamper 6 secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp6mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Internal tamper 7 secure interrupt masked flag This flag is set by hardware when the internal tamper 7 secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp7mf(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 7 secure interrupt masked flag This flag is set by hardware when the internal tamper 7 secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp7mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Internal tamper 8 secure interrupt masked flag This flag is set by hardware when the internal tamper 8 secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp8mf(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 8 secure interrupt masked flag This flag is set by hardware when the internal tamper 8 secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp8mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "internal tamper 9 secure interrupt masked flag This flag is set by hardware when the internal tamper 9 secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp9mf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "internal tamper 9 secure interrupt masked flag This flag is set by hardware when the internal tamper 9 secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp9mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "internal tamper 11 secure interrupt masked flag This flag is set by hardware when the internal tamper 11 secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp11mf(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "internal tamper 11 secure interrupt masked flag This flag is set by hardware when the internal tamper 11 secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp11mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "internal tamper 12 secure interrupt masked flag This flag is set by hardware when the internal tamper 12 secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp12mf(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "internal tamper 12 secure interrupt masked flag This flag is set by hardware when the internal tamper 12 secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp12mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "internal tamper 13 secure interrupt masked flag This flag is set by hardware when the internal tamper 13 secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp13mf(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "internal tamper 13 secure interrupt masked flag This flag is set by hardware when the internal tamper 13 secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp13mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "internal tamper 15 secure interrupt masked flag This flag is set by hardware when the internal tamper 15 secure interrupt is raised."]
        #[inline(always)]
        pub const fn itamp15mf(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "internal tamper 15 secure interrupt masked flag This flag is set by hardware when the internal tamper 15 secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itamp15mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Smisr {
        #[inline(always)]
        fn default() -> Smisr {
            Smisr(0)
        }
    }
    #[doc = "TAMP status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "TAMPx detection flag. This flag is set by hardware when a tamper detection event is detected on the TAMPx input. (x=1-8)"]
        #[inline(always)]
        pub const fn tampf(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "TAMPx detection flag. This flag is set by hardware when a tamper detection event is detected on the TAMPx input. (x=1-8)"]
        #[inline(always)]
        pub fn set_tampf(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Internal tamper 1 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 1."]
        #[inline(always)]
        pub const fn itamp1f(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 1 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 1."]
        #[inline(always)]
        pub fn set_itamp1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Internal tamper 2 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 2."]
        #[inline(always)]
        pub const fn itamp2f(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 2 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 2."]
        #[inline(always)]
        pub fn set_itamp2f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Internal tamper 3 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 3."]
        #[inline(always)]
        pub const fn itamp3f(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 3 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 3."]
        #[inline(always)]
        pub fn set_itamp3f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Internal tamper 4 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 4."]
        #[inline(always)]
        pub const fn itamp4f(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 4 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 4."]
        #[inline(always)]
        pub fn set_itamp4f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Internal tamper 5 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 5."]
        #[inline(always)]
        pub const fn itamp5f(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 5 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 5."]
        #[inline(always)]
        pub fn set_itamp5f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Internal tamper 6 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 6."]
        #[inline(always)]
        pub const fn itamp6f(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 6 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 6."]
        #[inline(always)]
        pub fn set_itamp6f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Internal tamper 7 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 7."]
        #[inline(always)]
        pub const fn itamp7f(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 7 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 7."]
        #[inline(always)]
        pub fn set_itamp7f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Internal tamper 8 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 8."]
        #[inline(always)]
        pub const fn itamp8f(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 8 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 8."]
        #[inline(always)]
        pub fn set_itamp8f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Internal tamper 9 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 9."]
        #[inline(always)]
        pub const fn itamp9f(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 9 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 9."]
        #[inline(always)]
        pub fn set_itamp9f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Internal tamper 11 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 11."]
        #[inline(always)]
        pub const fn itamp11f(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 11 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 11."]
        #[inline(always)]
        pub fn set_itamp11f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Internal tamper 12 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 12."]
        #[inline(always)]
        pub const fn itamp12f(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 12 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 12."]
        #[inline(always)]
        pub fn set_itamp12f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Internal tamper 13 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 13."]
        #[inline(always)]
        pub const fn itamp13f(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 13 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 13."]
        #[inline(always)]
        pub fn set_itamp13f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Internal tamper 15 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 15."]
        #[inline(always)]
        pub const fn itamp15f(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper 15 flag This flag is set by hardware when a tamper detection event is detected on the internal tamper 15."]
        #[inline(always)]
        pub fn set_itamp15f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
}
