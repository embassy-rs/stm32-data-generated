#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Tamper and backup registers"]
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
    #[doc = "TAMP control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "TAMP control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "TAMP control register 3"]
    #[inline(always)]
    pub const fn cr3(self) -> crate::common::Reg<regs::Cr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "TAMP filter control register"]
    #[inline(always)]
    pub const fn fltcr(self) -> crate::common::Reg<regs::Fltcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "TAMP active tamper control register 1"]
    #[inline(always)]
    pub const fn atcr1(self) -> crate::common::Reg<regs::Atcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "TAMP active tamper seed register"]
    #[inline(always)]
    pub const fn atseedr(self) -> crate::common::Reg<regs::Atseedr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "TAMP active tamper output register"]
    #[inline(always)]
    pub const fn ator(self) -> crate::common::Reg<regs::Ator, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "TAMP active tamper control register 2"]
    #[inline(always)]
    pub const fn atcr2(self) -> crate::common::Reg<regs::Atcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "TAMP secure mode register"]
    #[inline(always)]
    pub const fn seccfgr(self) -> crate::common::Reg<regs::Seccfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "TAMP privilege mode control register"]
    #[inline(always)]
    pub const fn privcr(self) -> crate::common::Reg<regs::Privcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "TAMP interrupt enable register"]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "TAMP status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "TAMP non-secure masked interrupt status register"]
    #[inline(always)]
    pub const fn misr(self) -> crate::common::Reg<regs::Misr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "TAMP secure masked interrupt status register"]
    #[inline(always)]
    pub const fn smisr(self) -> crate::common::Reg<regs::Smisr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "TAMP status clear register"]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::Scr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "TAMP monotonic counter 1 register"]
    #[inline(always)]
    pub const fn countr(self) -> crate::common::Reg<regs::Countr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "TAMP erase configuration register"]
    #[inline(always)]
    pub const fn ercfgr(self) -> crate::common::Reg<regs::Ercfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "TAMP backup X register"]
    #[inline(always)]
    pub const fn bkpr(self, n: usize) -> crate::common::Reg<regs::Bkpr, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "TAMP active tamper control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Atcr1(pub u32);
    impl Atcr1 {
        #[doc = "Tamper X active mode"]
        #[inline(always)]
        pub const fn tampam(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper X active mode"]
        #[inline(always)]
        pub fn set_tampam(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Active tamper shared output X selection. The selected output must be available in the package pinout"]
        #[inline(always)]
        pub const fn atosel(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 8usize + n * 2usize;
            let val = (self.0 >> offs) & 0x03;
            val as u8
        }
        #[doc = "Active tamper shared output X selection. The selected output must be available in the package pinout"]
        #[inline(always)]
        pub fn set_atosel(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 8usize + n * 2usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
        }
        #[doc = "Active tamper RTC asynchronous prescaler clock selection. These bits selects the RTC asynchronous prescaler stage output.The selected clock is CK_ATPRE.. fCK_ATPRE = fRTCCLK / 2ATCKSEL when (PREDIV_A+1) = 128.. .... These bits can be written only when all active tampers are disabled. The write protection remains for up to 1.5 ck_atpre cycles after all the active tampers are disable."]
        #[inline(always)]
        pub const fn atcksel(&self) -> super::vals::Atcksel {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::Atcksel::from_bits(val as u8)
        }
        #[doc = "Active tamper RTC asynchronous prescaler clock selection. These bits selects the RTC asynchronous prescaler stage output.The selected clock is CK_ATPRE.. fCK_ATPRE = fRTCCLK / 2ATCKSEL when (PREDIV_A+1) = 128.. .... These bits can be written only when all active tampers are disabled. The write protection remains for up to 1.5 ck_atpre cycles after all the active tampers are disable."]
        #[inline(always)]
        pub fn set_atcksel(&mut self, val: super::vals::Atcksel) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
        #[doc = "Active tamper output change period. The tamper output is changed every CK_ATPER = (2ATPER x CK_ATPRE) cycles. Refer to ."]
        #[inline(always)]
        pub const fn atper(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Active tamper output change period. The tamper output is changed every CK_ATPER = (2ATPER x CK_ATPRE) cycles. Refer to ."]
        #[inline(always)]
        pub fn set_atper(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "Active tamper output sharing. IN1 is compared with TAMPOUTSEL1. IN2 is compared with TAMPOUTSEL2. IN3 is compared with TAMPOUTSEL3. IN4 is compared with TAMPOUTSEL4. IN5 is compared with TAMPOUTSEL5. IN6 is compared with TAMPOUTSEL6. IN7 is compared with TAMPOUTSEL7. IN8 is compared with TAMPOUTSEL8"]
        #[inline(always)]
        pub const fn atoshare(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Active tamper output sharing. IN1 is compared with TAMPOUTSEL1. IN2 is compared with TAMPOUTSEL2. IN3 is compared with TAMPOUTSEL3. IN4 is compared with TAMPOUTSEL4. IN5 is compared with TAMPOUTSEL5. IN6 is compared with TAMPOUTSEL6. IN7 is compared with TAMPOUTSEL7. IN8 is compared with TAMPOUTSEL8"]
        #[inline(always)]
        pub fn set_atoshare(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Active tamper filter enable"]
        #[inline(always)]
        pub const fn flten(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Active tamper filter enable"]
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
    #[doc = "TAMP active tamper control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Atcr2(pub u32);
    impl Atcr2 {
        #[doc = "Active tamper shared output X selection. The selected output must be available in the package pinout. Bits 9:8 are the mirror of ATOSEL1\\[1:0\\]
in the ATCR1, and so can also be read or. written through ATCR1."]
        #[inline(always)]
        pub const fn atosel(&self, n: usize) -> u8 {
            assert!(n < 8usize);
            let offs = 8usize + n * 3usize;
            let val = (self.0 >> offs) & 0x07;
            val as u8
        }
        #[doc = "Active tamper shared output X selection. The selected output must be available in the package pinout. Bits 9:8 are the mirror of ATOSEL1\\[1:0\\]
in the ATCR1, and so can also be read or. written through ATCR1."]
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
    #[doc = "TAMP active tamper output register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ator(pub u32);
    impl Ator {
        #[doc = "Pseudo-random generator value. This field provides the values of the PRNG output. Because of potential inconsistencies due to synchronization delays, PRNG must be read at least twice. The read value is correct if it is equal to previous read value. This field can only be read when the APB is in secure mode."]
        #[inline(always)]
        pub const fn prng(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Pseudo-random generator value. This field provides the values of the PRNG output. Because of potential inconsistencies due to synchronization delays, PRNG must be read at least twice. The read value is correct if it is equal to previous read value. This field can only be read when the APB is in secure mode."]
        #[inline(always)]
        pub fn set_prng(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Seed running flag. This flag is set by hardware when a new seed is written in the ATSEEDR. It is cleared by hardware when the PRNG has absorbed this new seed, and by system reset. The TAMP APB cock must not be switched off as long as SEEDF is set."]
        #[inline(always)]
        pub const fn seedf(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Seed running flag. This flag is set by hardware when a new seed is written in the ATSEEDR. It is cleared by hardware when the PRNG has absorbed this new seed, and by system reset. The TAMP APB cock must not be switched off as long as SEEDF is set."]
        #[inline(always)]
        pub fn set_seedf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Active tamper initialization status. This flag is set by hardware when the PRNG has absorbed the first 128-bit seed, meaning that the enabled active tampers are functional. This flag is cleared when the active tampers are disabled."]
        #[inline(always)]
        pub const fn inits(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Active tamper initialization status. This flag is set by hardware when the PRNG has absorbed the first 128-bit seed, meaning that the enabled active tampers are functional. This flag is cleared when the active tampers are disabled."]
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
    #[doc = "TAMP active tamper seed register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Atseedr(pub u32);
    impl Atseedr {
        #[doc = "Pseudo-random generator seed value. This register must be written four times with 32-bit values to provide the 128-bit seed to the PRNG. Writing to this register automatically sends the seed value to the PRNG."]
        #[inline(always)]
        pub const fn seed(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Pseudo-random generator seed value. This register must be written four times with 32-bit values to provide the 128-bit seed to the PRNG. Writing to this register automatically sends the seed value to the PRNG."]
        #[inline(always)]
        pub fn set_seed(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Atseedr {
        #[inline(always)]
        fn default() -> Atseedr {
            Atseedr(0)
        }
    }
    #[doc = "TAMP backup register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bkpr(pub u32);
    impl Bkpr {
        #[doc = "The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set. This register is also reset when the readout protection (RDP) is disabled."]
        #[inline(always)]
        pub const fn bkp(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The application can write or read data to and from these registers. In the default (ERASE) configuration this register is reset on a tamper detection event. It is forced to reset value as long as there is at least one internal or external tamper flag being set. This register is also reset when the readout protection (RDP) is disabled."]
        #[inline(always)]
        pub fn set_bkp(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bkpr {
        #[inline(always)]
        fn default() -> Bkpr {
            Bkpr(0)
        }
    }
    #[doc = "TAMP monotonic counter 1 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Countr(pub u32);
    impl Countr {
        #[doc = "This register is read-only only and is incremented by one when a write access is done to this register. This register cannot roll-over and is frozen when reaching the maximum value."]
        #[inline(always)]
        pub const fn count(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "This register is read-only only and is incremented by one when a write access is done to this register. This register cannot roll-over and is frozen when reaching the maximum value."]
        #[inline(always)]
        pub fn set_count(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Countr {
        #[inline(always)]
        fn default() -> Countr {
            Countr(0)
        }
    }
    #[doc = "TAMP control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "Tamper detection on INx enable"]
        #[inline(always)]
        pub const fn tampe(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper detection on INx enable"]
        #[inline(always)]
        pub fn set_tampe(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Internal tamper X enable"]
        #[inline(always)]
        pub const fn itampe(&self, n: usize) -> bool {
            assert!(n < 13usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper X enable"]
        #[inline(always)]
        pub fn set_itampe(&mut self, n: usize, val: bool) {
            assert!(n < 13usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Cr1 {
        #[inline(always)]
        fn default() -> Cr1 {
            Cr1(0)
        }
    }
    #[doc = "TAMP control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "Tamper X no erase"]
        #[inline(always)]
        pub const fn tampnoer(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper X no erase"]
        #[inline(always)]
        pub fn set_tampnoer(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Tamper X mask. The tamper 1 interrupt must not be enabled when TAMP1MSK is set."]
        #[inline(always)]
        pub const fn tampmsk(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper X mask. The tamper 1 interrupt must not be enabled when TAMP1MSK is set."]
        #[inline(always)]
        pub fn set_tampmsk(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Backup registers and device secrets access blocked"]
        #[inline(always)]
        pub const fn bkblock(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Backup registers and device secrets access blocked"]
        #[inline(always)]
        pub fn set_bkblock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Backup registers and device secrets erase. Writing '1 to this bit reset the backup registers and device secrets(1). Writing 0 has no effect. This bit is always read as 0."]
        #[inline(always)]
        pub const fn bkerase(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Backup registers and device secrets erase. Writing '1 to this bit reset the backup registers and device secrets(1). Writing 0 has no effect. This bit is always read as 0."]
        #[inline(always)]
        pub fn set_bkerase(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Active level for tamper 1 input."]
        #[inline(always)]
        pub const fn tamptrg(&self, n: usize) -> super::vals::Tamptrg {
            assert!(n < 8usize);
            let offs = 24usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Tamptrg::from_bits(val as u8)
        }
        #[doc = "Active level for tamper 1 input."]
        #[inline(always)]
        pub fn set_tamptrg(&mut self, n: usize, val: super::vals::Tamptrg) {
            assert!(n < 8usize);
            let offs = 24usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
    }
    impl Default for Cr2 {
        #[inline(always)]
        fn default() -> Cr2 {
            Cr2(0)
        }
    }
    #[doc = "TAMP control register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr3(pub u32);
    impl Cr3 {
        #[doc = "Internal Tamper X no erase"]
        #[inline(always)]
        pub const fn itampnoer(&self, n: usize) -> bool {
            assert!(n < 13usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Internal Tamper X no erase"]
        #[inline(always)]
        pub fn set_itampnoer(&mut self, n: usize, val: bool) {
            assert!(n < 13usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Cr3 {
        #[inline(always)]
        fn default() -> Cr3 {
            Cr3(0)
        }
    }
    #[doc = "TAMP erase configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ercfgr(pub u32);
    impl Ercfgr {
        #[doc = "Configurable device secrets configuration"]
        #[inline(always)]
        pub const fn ercfg0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Configurable device secrets configuration"]
        #[inline(always)]
        pub fn set_ercfg0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Ercfgr {
        #[inline(always)]
        fn default() -> Ercfgr {
            Ercfgr(0)
        }
    }
    #[doc = "TAMP filter control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fltcr(pub u32);
    impl Fltcr {
        #[doc = "Tamper sampling frequency. Determines the frequency at which each of the INx inputs are sampled."]
        #[inline(always)]
        pub const fn tampfreq(&self) -> super::vals::Tampfreq {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Tampfreq::from_bits(val as u8)
        }
        #[doc = "Tamper sampling frequency. Determines the frequency at which each of the INx inputs are sampled."]
        #[inline(always)]
        pub fn set_tampfreq(&mut self, val: super::vals::Tampfreq) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "INx filter count. These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the INx inputs."]
        #[inline(always)]
        pub const fn tampflt(&self) -> super::vals::Tampflt {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Tampflt::from_bits(val as u8)
        }
        #[doc = "INx filter count. These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the INx inputs."]
        #[inline(always)]
        pub fn set_tampflt(&mut self, val: super::vals::Tampflt) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "INx precharge duration. These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the INx inputs."]
        #[inline(always)]
        pub const fn tampprch(&self) -> super::vals::Tampprch {
            let val = (self.0 >> 5usize) & 0x03;
            super::vals::Tampprch::from_bits(val as u8)
        }
        #[doc = "INx precharge duration. These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the INx inputs."]
        #[inline(always)]
        pub fn set_tampprch(&mut self, val: super::vals::Tampprch) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
        }
        #[doc = "INx pull-up disable. This bit determines if each of the TAMPx pins are precharged before each sample."]
        #[inline(always)]
        pub const fn tamppudis(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "INx pull-up disable. This bit determines if each of the TAMPx pins are precharged before each sample."]
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
    #[doc = "TAMP interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "Tamper X interrupt enable"]
        #[inline(always)]
        pub const fn tampie(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper X interrupt enable"]
        #[inline(always)]
        pub fn set_tampie(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Internal tamper X interrupt enable"]
        #[inline(always)]
        pub const fn itampie(&self, n: usize) -> bool {
            assert!(n < 13usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper X interrupt enable"]
        #[inline(always)]
        pub fn set_itampie(&mut self, n: usize, val: bool) {
            assert!(n < 13usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Ier {
        #[inline(always)]
        fn default() -> Ier {
            Ier(0)
        }
    }
    #[doc = "TAMP non-secure masked interrupt status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Misr(pub u32);
    impl Misr {
        #[doc = "TAMPx non-secure interrupt masked flag. This flag is set by hardware when the tamper X non-secure interrupt is raised."]
        #[inline(always)]
        pub const fn tampmf(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "TAMPx non-secure interrupt masked flag. This flag is set by hardware when the tamper X non-secure interrupt is raised."]
        #[inline(always)]
        pub fn set_tampmf(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Internal tamper X non-secure interrupt masked flag. This flag is set by hardware when the internal tamper X non-secure interrupt is raised."]
        #[inline(always)]
        pub const fn itampmf(&self, n: usize) -> bool {
            assert!(n < 13usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper X non-secure interrupt masked flag. This flag is set by hardware when the internal tamper X non-secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itampmf(&mut self, n: usize, val: bool) {
            assert!(n < 13usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Misr {
        #[inline(always)]
        fn default() -> Misr {
            Misr(0)
        }
    }
    #[doc = "TAMP privilege mode control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcr(pub u32);
    impl Privcr {
        #[doc = "Monotonic counter 1 privilege protection"]
        #[inline(always)]
        pub const fn cnt1priv(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Monotonic counter 1 privilege protection"]
        #[inline(always)]
        pub fn set_cnt1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Backup registers zone 1 privilege protection"]
        #[inline(always)]
        pub const fn bkprwpriv(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Backup registers zone 1 privilege protection"]
        #[inline(always)]
        pub fn set_bkprwpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Backup registers zone 2 privilege protection"]
        #[inline(always)]
        pub const fn bkpwpriv(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Backup registers zone 2 privilege protection"]
        #[inline(always)]
        pub fn set_bkpwpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Tamper privilege protection (excluding backup registers). Note: Refer to for details on the read protection."]
        #[inline(always)]
        pub const fn tamppriv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Tamper privilege protection (excluding backup registers). Note: Refer to for details on the read protection."]
        #[inline(always)]
        pub fn set_tamppriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Privcr {
        #[inline(always)]
        fn default() -> Privcr {
            Privcr(0)
        }
    }
    #[doc = "TAMP status clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scr(pub u32);
    impl Scr {
        #[doc = "Clear TAMPx detection flag. Writing 1 in this bit clears the TAMPxF bit in the SR register."]
        #[inline(always)]
        pub const fn ctampf(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Clear TAMPx detection flag. Writing 1 in this bit clears the TAMPxF bit in the SR register."]
        #[inline(always)]
        pub fn set_ctampf(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Clear ITAMPx detection flag. Writing 1 in this bit clears the ITAMPxF bit in the SR register."]
        #[inline(always)]
        pub const fn citampf(&self, n: usize) -> bool {
            assert!(n < 13usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Clear ITAMPx detection flag. Writing 1 in this bit clears the ITAMPxF bit in the SR register."]
        #[inline(always)]
        pub fn set_citampf(&mut self, n: usize, val: bool) {
            assert!(n < 13usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Scr {
        #[inline(always)]
        fn default() -> Scr {
            Scr(0)
        }
    }
    #[doc = "TAMP secure mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccfgr(pub u32);
    impl Seccfgr {
        #[doc = "Backup registers read/write protection offset. Protection zone 1 is defined for backup registers from BKP0R to BKPxR (x = BKPRWSEC-1, from 0 to 128). if TZEN=1, these backup registers can be read and written only with secure access. If TZEN=0: the protection zone 1 can be read and written with non-secure access. If BKPRWSEC = 0: there is no protection zone 1. If BKPRWPRIV is set, BKPRWSEC\\[7:0\\]
can be written only in privileged mode."]
        #[inline(always)]
        pub const fn bkprwsec(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Backup registers read/write protection offset. Protection zone 1 is defined for backup registers from BKP0R to BKPxR (x = BKPRWSEC-1, from 0 to 128). if TZEN=1, these backup registers can be read and written only with secure access. If TZEN=0: the protection zone 1 can be read and written with non-secure access. If BKPRWSEC = 0: there is no protection zone 1. If BKPRWPRIV is set, BKPRWSEC\\[7:0\\]
can be written only in privileged mode."]
        #[inline(always)]
        pub fn set_bkprwsec(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Monotonic counter 1 secure protection"]
        #[inline(always)]
        pub const fn cnt1sec(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Monotonic counter 1 secure protection"]
        #[inline(always)]
        pub fn set_cnt1sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Backup registers write protection offset. Protection zone 2 is defined for backup registers from BKPyR (y = BKPRWSEC, from 0 to 128) to BKPzR (z = BKPWSEC-1, from 0 to 128, BKPWSECBKPRWSEC): if TZEN=1, these backup registers can be written only with secure access. They can be read with secure or non-secure access. Protection zone 3 defined for backup registers from BKPtR (t = BKPWSEC, from 0 to 127). They can be read or written with secure or non-secure access. If TZEN=0: the protection zone 2 can be read and written with non-secure access. If BKPWSEC = 0 or if BKPWSEC BKPRWSEC: there is no protection zone 2. If BKPWPRIV is set, BKPRWSEC\\[7:0\\]
can be written only in privileged mode."]
        #[inline(always)]
        pub const fn bkpwsec(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Backup registers write protection offset. Protection zone 2 is defined for backup registers from BKPyR (y = BKPRWSEC, from 0 to 128) to BKPzR (z = BKPWSEC-1, from 0 to 128, BKPWSECBKPRWSEC): if TZEN=1, these backup registers can be written only with secure access. They can be read with secure or non-secure access. Protection zone 3 defined for backup registers from BKPtR (t = BKPWSEC, from 0 to 127). They can be read or written with secure or non-secure access. If TZEN=0: the protection zone 2 can be read and written with non-secure access. If BKPWSEC = 0 or if BKPWSEC BKPRWSEC: there is no protection zone 2. If BKPWPRIV is set, BKPRWSEC\\[7:0\\]
can be written only in privileged mode."]
        #[inline(always)]
        pub fn set_bkpwsec(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Boot hardware key lock. This bit can be read and can only be written to 1 by software. It is cleared by hardware together with the backup registers following a tamper detection event or when the readout protection (RDP) is disabled."]
        #[inline(always)]
        pub const fn bhklock(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Boot hardware key lock. This bit can be read and can only be written to 1 by software. It is cleared by hardware together with the backup registers following a tamper detection event or when the readout protection (RDP) is disabled."]
        #[inline(always)]
        pub fn set_bhklock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Tamper protection (excluding monotonic counters and backup registers). Note: Refer to for details on the read protection."]
        #[inline(always)]
        pub const fn tampsec(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Tamper protection (excluding monotonic counters and backup registers). Note: Refer to for details on the read protection."]
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
    #[doc = "TAMP secure masked interrupt status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smisr(pub u32);
    impl Smisr {
        #[doc = "TAMPx secure interrupt masked flag. This flag is set by hardware when the tamper X secure interrupt is raised."]
        #[inline(always)]
        pub const fn tampmf(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "TAMPx secure interrupt masked flag. This flag is set by hardware when the tamper X secure interrupt is raised."]
        #[inline(always)]
        pub fn set_tampmf(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Internal tamper X secure interrupt masked flag. This flag is set by hardware when the internal tamper X secure interrupt is raised."]
        #[inline(always)]
        pub const fn itampmf(&self, n: usize) -> bool {
            assert!(n < 13usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper X secure interrupt masked flag. This flag is set by hardware when the internal tamper X secure interrupt is raised."]
        #[inline(always)]
        pub fn set_itampmf(&mut self, n: usize, val: bool) {
            assert!(n < 13usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Smisr {
        #[inline(always)]
        fn default() -> Smisr {
            Smisr(0)
        }
    }
    #[doc = "TAMP status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "TAMPx detection flag. This flag is set by hardware when a tamper detection event is detected on the TAMPx input."]
        #[inline(always)]
        pub const fn tampf(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "TAMPx detection flag. This flag is set by hardware when a tamper detection event is detected on the TAMPx input."]
        #[inline(always)]
        pub fn set_tampf(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Internal tamper X flag. This flag is set by hardware when a tamper detection event is detected on the internal tamper X."]
        #[inline(always)]
        pub const fn itampf(&self, n: usize) -> bool {
            assert!(n < 13usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper X flag. This flag is set by hardware when a tamper detection event is detected on the internal tamper X."]
        #[inline(always)]
        pub fn set_itampf(&mut self, n: usize, val: bool) {
            assert!(n < 13usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Atcksel {
        #[doc = "RTCCLK is selected"]
        DIV1 = 0x0,
        #[doc = "RTCCLK/2 is selected when (PREDIV_A+1) = 128 (actually selects 1st flip flop output)"]
        DIV2 = 0x01,
        #[doc = "RTCCLK/4 is selected when (PREDIV_A+1) = 128 (actually selects 2nd flip flop output)"]
        DIV4 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        #[doc = "RTCCLK/128 is selected when (PREDIV_A+1) = 128 (actually selects 7th flip flop output)"]
        DIV128 = 0x07,
    }
    impl Atcksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Atcksel {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Atcksel {
        #[inline(always)]
        fn from(val: u8) -> Atcksel {
            Atcksel::from_bits(val)
        }
    }
    impl From<Atcksel> for u8 {
        #[inline(always)]
        fn from(val: Atcksel) -> u8 {
            Atcksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Tampflt {
        #[doc = "Tamper event is activated on edge of INx input transitions to the active level (no internal pull-up on INx input)."]
        NOFILTER = 0x0,
        #[doc = "Tamper event is activated after 2 consecutive samples at the active level."]
        FILTER2 = 0x01,
        #[doc = "Tamper event is activated after 4 consecutive samples at the active level."]
        FILTER4 = 0x02,
        #[doc = "Tamper event is activated after 8 consecutive samples at the active level."]
        FILTER8 = 0x03,
    }
    impl Tampflt {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tampflt {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tampflt {
        #[inline(always)]
        fn from(val: u8) -> Tampflt {
            Tampflt::from_bits(val)
        }
    }
    impl From<Tampflt> for u8 {
        #[inline(always)]
        fn from(val: Tampflt) -> u8 {
            Tampflt::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Tampfreq {
        #[doc = "RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)"]
        HZ_1 = 0x0,
        #[doc = "RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)"]
        HZ_2 = 0x01,
        #[doc = "RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)"]
        HZ_4 = 0x02,
        #[doc = "RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)"]
        HZ_8 = 0x03,
        #[doc = "RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)"]
        HZ_16 = 0x04,
        #[doc = "RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)"]
        HZ_32 = 0x05,
        #[doc = "RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)"]
        HZ_64 = 0x06,
        #[doc = "RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)"]
        HZ_128 = 0x07,
    }
    impl Tampfreq {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tampfreq {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tampfreq {
        #[inline(always)]
        fn from(val: u8) -> Tampfreq {
            Tampfreq::from_bits(val)
        }
    }
    impl From<Tampfreq> for u8 {
        #[inline(always)]
        fn from(val: Tampfreq) -> u8 {
            Tampfreq::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Tampprch {
        #[doc = "1 RTCCLK cycle"]
        CYCLES1 = 0x0,
        #[doc = "2 RTCCLK cycles"]
        CYCLES2 = 0x01,
        #[doc = "4 RTCCLK cycles"]
        CYCLES4 = 0x02,
        #[doc = "8 RTCCLK cycles"]
        CYCLES8 = 0x03,
    }
    impl Tampprch {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tampprch {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tampprch {
        #[inline(always)]
        fn from(val: u8) -> Tampprch {
            Tampprch::from_bits(val)
        }
    }
    impl From<Tampprch> for u8 {
        #[inline(always)]
        fn from(val: Tampprch) -> u8 {
            Tampprch::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Tamptrg {
        #[doc = "If TAMPFLT 00 Tamper 2 input staying low triggers a tamper detection event."]
        FILTEREDLOWORUNFILTEREDHIGH = 0x0,
        #[doc = "If TAMPFLT 00 Tamper 2 input staying high triggers a tamper detection event."]
        FILTEREDHIGHORUNFILTEREDLOW = 0x01,
    }
    impl Tamptrg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tamptrg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tamptrg {
        #[inline(always)]
        fn from(val: u8) -> Tamptrg {
            Tamptrg::from_bits(val)
        }
    }
    impl From<Tamptrg> for u8 {
        #[inline(always)]
        fn from(val: Tamptrg) -> u8 {
            Tamptrg::to_bits(val)
        }
    }
}
