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
    #[doc = "control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "control register 3"]
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
    pub const fn atseedr(self) -> crate::common::Reg<regs::Atseedr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "TAMP active tamper output register"]
    #[inline(always)]
    pub const fn ator(self) -> crate::common::Reg<regs::Ator, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "TAMP active tamper control register 2"]
    #[inline(always)]
    pub const fn atcr2(self) -> crate::common::Reg<regs::Atcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "TAMP secure mode register"]
    #[inline(always)]
    pub const fn smcr(self) -> crate::common::Reg<regs::Smcr, crate::common::RW> {
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
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "TAMP masked interrupt status register"]
    #[inline(always)]
    pub const fn misr(self) -> crate::common::Reg<regs::Misr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "TAMP secure masked interrupt status register"]
    #[inline(always)]
    pub const fn smisr(self) -> crate::common::Reg<regs::Smisr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "TAMP status clear register"]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::Scr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "TAMP monotonic counter register"]
    #[inline(always)]
    pub const fn countr(self) -> crate::common::Reg<regs::Countr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "TAMP configuration register"]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "TAMP backup register"]
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
        #[doc = "TAMPAM"]
        #[inline(always)]
        pub const fn tampam(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "TAMPAM"]
        #[inline(always)]
        pub fn set_tampam(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "ATOSEL"]
        #[inline(always)]
        pub const fn atosel(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 8usize + n * 2usize;
            let val = (self.0 >> offs) & 0x03;
            val as u8
        }
        #[doc = "ATOSEL"]
        #[inline(always)]
        pub fn set_atosel(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 8usize + n * 2usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val as u32) & 0x03) << offs);
        }
        #[doc = "ATCKSEL"]
        #[inline(always)]
        pub const fn atcksel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x03;
            val as u8
        }
        #[doc = "ATCKSEL"]
        #[inline(always)]
        pub fn set_atcksel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
        }
        #[doc = "ATPER"]
        #[inline(always)]
        pub const fn atper(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "ATPER"]
        #[inline(always)]
        pub fn set_atper(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "ATOSHARE"]
        #[inline(always)]
        pub const fn atoshare(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "ATOSHARE"]
        #[inline(always)]
        pub fn set_atoshare(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "FLTEN"]
        #[inline(always)]
        pub const fn flten(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "FLTEN"]
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
    impl core::fmt::Debug for Atcr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Atcr1")
                .field(
                    "tampam",
                    &[
                        self.tampam(0usize),
                        self.tampam(1usize),
                        self.tampam(2usize),
                        self.tampam(3usize),
                        self.tampam(4usize),
                        self.tampam(5usize),
                        self.tampam(6usize),
                        self.tampam(7usize),
                    ],
                )
                .field(
                    "atosel",
                    &[
                        self.atosel(0usize),
                        self.atosel(1usize),
                        self.atosel(2usize),
                        self.atosel(3usize),
                    ],
                )
                .field("atcksel", &self.atcksel())
                .field("atper", &self.atper())
                .field("atoshare", &self.atoshare())
                .field("flten", &self.flten())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Atcr1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Atcr1 {
                tampam: [bool; 8usize],
                atosel: [u8; 4usize],
                atcksel: u8,
                atper: u8,
                atoshare: bool,
                flten: bool,
            }
            let proxy = Atcr1 {
                tampam: [
                    self.tampam(0usize),
                    self.tampam(1usize),
                    self.tampam(2usize),
                    self.tampam(3usize),
                    self.tampam(4usize),
                    self.tampam(5usize),
                    self.tampam(6usize),
                    self.tampam(7usize),
                ],
                atosel: [
                    self.atosel(0usize),
                    self.atosel(1usize),
                    self.atosel(2usize),
                    self.atosel(3usize),
                ],
                atcksel: self.atcksel(),
                atper: self.atper(),
                atoshare: self.atoshare(),
                flten: self.flten(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "TAMP active tamper control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Atcr2(pub u32);
    impl Atcr2 {
        #[doc = "ATOSEL"]
        #[inline(always)]
        pub const fn atosel(&self, n: usize) -> u8 {
            assert!(n < 8usize);
            let offs = 8usize + n * 3usize;
            let val = (self.0 >> offs) & 0x07;
            val as u8
        }
        #[doc = "ATOSEL"]
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
    impl core::fmt::Debug for Atcr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Atcr2")
                .field(
                    "atosel",
                    &[
                        self.atosel(0usize),
                        self.atosel(1usize),
                        self.atosel(2usize),
                        self.atosel(3usize),
                        self.atosel(4usize),
                        self.atosel(5usize),
                        self.atosel(6usize),
                        self.atosel(7usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Atcr2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Atcr2 {
                atosel: [u8; 8usize],
            }
            let proxy = Atcr2 {
                atosel: [
                    self.atosel(0usize),
                    self.atosel(1usize),
                    self.atosel(2usize),
                    self.atosel(3usize),
                    self.atosel(4usize),
                    self.atosel(5usize),
                    self.atosel(6usize),
                    self.atosel(7usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "TAMP active tamper output register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ator(pub u32);
    impl Ator {
        #[doc = "Pseudo-random generator value"]
        #[inline(always)]
        pub const fn prng(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Pseudo-random generator value"]
        #[inline(always)]
        pub fn set_prng(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Seed running flag"]
        #[inline(always)]
        pub const fn seedf(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Seed running flag"]
        #[inline(always)]
        pub fn set_seedf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Active tamper initialization status"]
        #[inline(always)]
        pub const fn inits(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Active tamper initialization status"]
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
    impl core::fmt::Debug for Ator {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ator")
                .field("prng", &self.prng())
                .field("seedf", &self.seedf())
                .field("inits", &self.inits())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ator {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ator {
                prng: u8,
                seedf: bool,
                inits: bool,
            }
            let proxy = Ator {
                prng: self.prng(),
                seedf: self.seedf(),
                inits: self.inits(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "TAMP active tamper seed register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Atseedr(pub u32);
    impl Atseedr {
        #[doc = "Pseudo-random generator seed value"]
        #[inline(always)]
        pub const fn seed(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Pseudo-random generator seed value"]
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
    impl core::fmt::Debug for Atseedr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Atseedr").field("seed", &self.seed()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Atseedr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Atseedr {
                seed: u32,
            }
            let proxy = Atseedr { seed: self.seed() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "TAMP backup register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bkpr(pub u32);
    impl Bkpr {
        #[doc = "BKP"]
        #[inline(always)]
        pub const fn bkp(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "BKP"]
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
    impl core::fmt::Debug for Bkpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bkpr").field("bkp", &self.bkp()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bkpr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Bkpr {
                bkp: u32,
            }
            let proxy = Bkpr { bkp: self.bkp() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "TAMP configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "TMONEN"]
        #[inline(always)]
        pub const fn tmonen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TMONEN"]
        #[inline(always)]
        pub fn set_tmonen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "VMONEN"]
        #[inline(always)]
        pub const fn vmonen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "VMONEN"]
        #[inline(always)]
        pub fn set_vmonen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "WUTMONEN"]
        #[inline(always)]
        pub const fn wutmonen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "WUTMONEN"]
        #[inline(always)]
        pub fn set_wutmonen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
                .field("tmonen", &self.tmonen())
                .field("vmonen", &self.vmonen())
                .field("wutmonen", &self.wutmonen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cfgr {
                tmonen: bool,
                vmonen: bool,
                wutmonen: bool,
            }
            let proxy = Cfgr {
                tmonen: self.tmonen(),
                vmonen: self.vmonen(),
                wutmonen: self.wutmonen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "TAMP monotonic counter register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Countr(pub u32);
    impl Countr {
        #[doc = "COUNT"]
        #[inline(always)]
        pub const fn count(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "COUNT"]
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
    impl core::fmt::Debug for Countr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Countr").field("count", &self.count()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Countr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Countr {
                count: u32,
            }
            let proxy = Countr { count: self.count() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "TAMPE"]
        #[inline(always)]
        pub const fn tampe(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "TAMPE"]
        #[inline(always)]
        pub fn set_tampe(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "ITAMPE"]
        #[inline(always)]
        pub const fn itampe(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "ITAMPE"]
        #[inline(always)]
        pub fn set_itampe(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
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
    impl core::fmt::Debug for Cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr1")
                .field(
                    "tampe",
                    &[
                        self.tampe(0usize),
                        self.tampe(1usize),
                        self.tampe(2usize),
                        self.tampe(3usize),
                        self.tampe(4usize),
                        self.tampe(5usize),
                        self.tampe(6usize),
                        self.tampe(7usize),
                    ],
                )
                .field(
                    "itampe",
                    &[
                        self.itampe(0usize),
                        self.itampe(1usize),
                        self.itampe(2usize),
                        self.itampe(3usize),
                        self.itampe(4usize),
                        self.itampe(5usize),
                        self.itampe(6usize),
                        self.itampe(7usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cr1 {
                tampe: [bool; 8usize],
                itampe: [bool; 8usize],
            }
            let proxy = Cr1 {
                tampe: [
                    self.tampe(0usize),
                    self.tampe(1usize),
                    self.tampe(2usize),
                    self.tampe(3usize),
                    self.tampe(4usize),
                    self.tampe(5usize),
                    self.tampe(6usize),
                    self.tampe(7usize),
                ],
                itampe: [
                    self.itampe(0usize),
                    self.itampe(1usize),
                    self.itampe(2usize),
                    self.itampe(3usize),
                    self.itampe(4usize),
                    self.itampe(5usize),
                    self.itampe(6usize),
                    self.itampe(7usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "control register 2"]
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
        #[doc = "Tamper X mask"]
        #[inline(always)]
        pub const fn tampmsk(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper X mask"]
        #[inline(always)]
        pub fn set_tampmsk(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "BKERASE"]
        #[inline(always)]
        pub const fn bkerase(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "BKERASE"]
        #[inline(always)]
        pub fn set_bkerase(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Active level for tamper X input"]
        #[inline(always)]
        pub const fn tamptrg(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 24usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Active level for tamper X input"]
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
    impl core::fmt::Debug for Cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr2")
                .field(
                    "tampnoer",
                    &[
                        self.tampnoer(0usize),
                        self.tampnoer(1usize),
                        self.tampnoer(2usize),
                        self.tampnoer(3usize),
                        self.tampnoer(4usize),
                        self.tampnoer(5usize),
                        self.tampnoer(6usize),
                        self.tampnoer(7usize),
                    ],
                )
                .field(
                    "tampmsk",
                    &[self.tampmsk(0usize), self.tampmsk(1usize), self.tampmsk(2usize)],
                )
                .field("bkerase", &self.bkerase())
                .field(
                    "tamptrg",
                    &[
                        self.tamptrg(0usize),
                        self.tamptrg(1usize),
                        self.tamptrg(2usize),
                        self.tamptrg(3usize),
                        self.tamptrg(4usize),
                        self.tamptrg(5usize),
                        self.tamptrg(6usize),
                        self.tamptrg(7usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cr2 {
                tampnoer: [bool; 8usize],
                tampmsk: [bool; 3usize],
                bkerase: bool,
                tamptrg: [bool; 8usize],
            }
            let proxy = Cr2 {
                tampnoer: [
                    self.tampnoer(0usize),
                    self.tampnoer(1usize),
                    self.tampnoer(2usize),
                    self.tampnoer(3usize),
                    self.tampnoer(4usize),
                    self.tampnoer(5usize),
                    self.tampnoer(6usize),
                    self.tampnoer(7usize),
                ],
                tampmsk: [self.tampmsk(0usize), self.tampmsk(1usize), self.tampmsk(2usize)],
                bkerase: self.bkerase(),
                tamptrg: [
                    self.tamptrg(0usize),
                    self.tamptrg(1usize),
                    self.tamptrg(2usize),
                    self.tamptrg(3usize),
                    self.tamptrg(4usize),
                    self.tamptrg(5usize),
                    self.tamptrg(6usize),
                    self.tamptrg(7usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "control register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr3(pub u32);
    impl Cr3 {
        #[doc = "Internal Tamper X no erase"]
        #[inline(always)]
        pub const fn itampnoer(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Internal Tamper X no erase"]
        #[inline(always)]
        pub fn set_itampnoer(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
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
    impl core::fmt::Debug for Cr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr3")
                .field(
                    "itampnoer",
                    &[
                        self.itampnoer(0usize),
                        self.itampnoer(1usize),
                        self.itampnoer(2usize),
                        self.itampnoer(3usize),
                        self.itampnoer(4usize),
                        self.itampnoer(5usize),
                        self.itampnoer(6usize),
                        self.itampnoer(7usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr3 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cr3 {
                itampnoer: [bool; 8usize],
            }
            let proxy = Cr3 {
                itampnoer: [
                    self.itampnoer(0usize),
                    self.itampnoer(1usize),
                    self.itampnoer(2usize),
                    self.itampnoer(3usize),
                    self.itampnoer(4usize),
                    self.itampnoer(5usize),
                    self.itampnoer(6usize),
                    self.itampnoer(7usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "TAMP filter control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fltcr(pub u32);
    impl Fltcr {
        #[doc = "TAMPFREQ"]
        #[inline(always)]
        pub const fn tampfreq(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "TAMPFREQ"]
        #[inline(always)]
        pub fn set_tampfreq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "TAMPFLT"]
        #[inline(always)]
        pub const fn tampflt(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "TAMPFLT"]
        #[inline(always)]
        pub fn set_tampflt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "TAMPPRCH"]
        #[inline(always)]
        pub const fn tampprch(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "TAMPPRCH"]
        #[inline(always)]
        pub fn set_tampprch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "TAMPPUDIS"]
        #[inline(always)]
        pub const fn tamppudis(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TAMPPUDIS"]
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
    impl core::fmt::Debug for Fltcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fltcr")
                .field("tampfreq", &self.tampfreq())
                .field("tampflt", &self.tampflt())
                .field("tampprch", &self.tampprch())
                .field("tamppudis", &self.tamppudis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fltcr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Fltcr {
                tampfreq: u8,
                tampflt: u8,
                tampprch: u8,
                tamppudis: bool,
            }
            let proxy = Fltcr {
                tampfreq: self.tampfreq(),
                tampflt: self.tampflt(),
                tampprch: self.tampprch(),
                tamppudis: self.tamppudis(),
            };
            defmt::write!(f, "{}", proxy)
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
            assert!(n < 8usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper X interrupt enable"]
        #[inline(always)]
        pub fn set_itampie(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
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
    impl core::fmt::Debug for Ier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ier")
                .field(
                    "tampie",
                    &[
                        self.tampie(0usize),
                        self.tampie(1usize),
                        self.tampie(2usize),
                        self.tampie(3usize),
                        self.tampie(4usize),
                        self.tampie(5usize),
                        self.tampie(6usize),
                        self.tampie(7usize),
                    ],
                )
                .field(
                    "itampie",
                    &[
                        self.itampie(0usize),
                        self.itampie(1usize),
                        self.itampie(2usize),
                        self.itampie(3usize),
                        self.itampie(4usize),
                        self.itampie(5usize),
                        self.itampie(6usize),
                        self.itampie(7usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ier {
                tampie: [bool; 8usize],
                itampie: [bool; 8usize],
            }
            let proxy = Ier {
                tampie: [
                    self.tampie(0usize),
                    self.tampie(1usize),
                    self.tampie(2usize),
                    self.tampie(3usize),
                    self.tampie(4usize),
                    self.tampie(5usize),
                    self.tampie(6usize),
                    self.tampie(7usize),
                ],
                itampie: [
                    self.itampie(0usize),
                    self.itampie(1usize),
                    self.itampie(2usize),
                    self.itampie(3usize),
                    self.itampie(4usize),
                    self.itampie(5usize),
                    self.itampie(6usize),
                    self.itampie(7usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "TAMP masked interrupt status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Misr(pub u32);
    impl Misr {
        #[doc = "Tamper X interrupt masked flag"]
        #[inline(always)]
        pub const fn tampmf(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper X interrupt masked flag"]
        #[inline(always)]
        pub fn set_tampmf(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Internal tamper X interrupt masked flag"]
        #[inline(always)]
        pub const fn itampmf(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper X interrupt masked flag"]
        #[inline(always)]
        pub fn set_itampmf(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
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
    impl core::fmt::Debug for Misr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Misr")
                .field(
                    "tampmf",
                    &[
                        self.tampmf(0usize),
                        self.tampmf(1usize),
                        self.tampmf(2usize),
                        self.tampmf(3usize),
                        self.tampmf(4usize),
                        self.tampmf(5usize),
                        self.tampmf(6usize),
                        self.tampmf(7usize),
                    ],
                )
                .field(
                    "itampmf",
                    &[
                        self.itampmf(0usize),
                        self.itampmf(1usize),
                        self.itampmf(2usize),
                        self.itampmf(3usize),
                        self.itampmf(4usize),
                        self.itampmf(5usize),
                        self.itampmf(6usize),
                        self.itampmf(7usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Misr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Misr {
                tampmf: [bool; 8usize],
                itampmf: [bool; 8usize],
            }
            let proxy = Misr {
                tampmf: [
                    self.tampmf(0usize),
                    self.tampmf(1usize),
                    self.tampmf(2usize),
                    self.tampmf(3usize),
                    self.tampmf(4usize),
                    self.tampmf(5usize),
                    self.tampmf(6usize),
                    self.tampmf(7usize),
                ],
                itampmf: [
                    self.itampmf(0usize),
                    self.itampmf(1usize),
                    self.itampmf(2usize),
                    self.itampmf(3usize),
                    self.itampmf(4usize),
                    self.itampmf(5usize),
                    self.itampmf(6usize),
                    self.itampmf(7usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "TAMP privilege mode control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcr(pub u32);
    impl Privcr {
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
        #[doc = "Tamper privilege protection"]
        #[inline(always)]
        pub const fn tamppriv(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Tamper privilege protection"]
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
    impl core::fmt::Debug for Privcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Privcr")
                .field("bkprwpriv", &self.bkprwpriv())
                .field("bkpwpriv", &self.bkpwpriv())
                .field("tamppriv", &self.tamppriv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privcr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Privcr {
                bkprwpriv: bool,
                bkpwpriv: bool,
                tamppriv: bool,
            }
            let proxy = Privcr {
                bkprwpriv: self.bkprwpriv(),
                bkpwpriv: self.bkpwpriv(),
                tamppriv: self.tamppriv(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "TAMP status clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scr(pub u32);
    impl Scr {
        #[doc = "Clear tamper X detection flag"]
        #[inline(always)]
        pub const fn ctampf(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Clear tamper X detection flag"]
        #[inline(always)]
        pub fn set_ctampf(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Clear internal tamper X detection flag"]
        #[inline(always)]
        pub const fn citampf(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Clear internal tamper X detection flag"]
        #[inline(always)]
        pub fn set_citampf(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
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
    impl core::fmt::Debug for Scr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scr")
                .field(
                    "ctampf",
                    &[
                        self.ctampf(0usize),
                        self.ctampf(1usize),
                        self.ctampf(2usize),
                        self.ctampf(3usize),
                        self.ctampf(4usize),
                        self.ctampf(5usize),
                        self.ctampf(6usize),
                        self.ctampf(7usize),
                    ],
                )
                .field(
                    "citampf",
                    &[
                        self.citampf(0usize),
                        self.citampf(1usize),
                        self.citampf(2usize),
                        self.citampf(3usize),
                        self.citampf(4usize),
                        self.citampf(5usize),
                        self.citampf(6usize),
                        self.citampf(7usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Scr {
                ctampf: [bool; 8usize],
                citampf: [bool; 8usize],
            }
            let proxy = Scr {
                ctampf: [
                    self.ctampf(0usize),
                    self.ctampf(1usize),
                    self.ctampf(2usize),
                    self.ctampf(3usize),
                    self.ctampf(4usize),
                    self.ctampf(5usize),
                    self.ctampf(6usize),
                    self.ctampf(7usize),
                ],
                citampf: [
                    self.citampf(0usize),
                    self.citampf(1usize),
                    self.citampf(2usize),
                    self.citampf(3usize),
                    self.citampf(4usize),
                    self.citampf(5usize),
                    self.citampf(6usize),
                    self.citampf(7usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "TAMP secure mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smcr(pub u32);
    impl Smcr {
        #[doc = "Backup registers read/write protection offset"]
        #[inline(always)]
        pub const fn bkprwdprot(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Backup registers read/write protection offset"]
        #[inline(always)]
        pub fn set_bkprwdprot(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Backup registers write protection offset"]
        #[inline(always)]
        pub const fn bkpwdprot(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Backup registers write protection offset"]
        #[inline(always)]
        pub fn set_bkpwdprot(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Tamper protection"]
        #[inline(always)]
        pub const fn tampdprot(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Tamper protection"]
        #[inline(always)]
        pub fn set_tampdprot(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Smcr {
        #[inline(always)]
        fn default() -> Smcr {
            Smcr(0)
        }
    }
    impl core::fmt::Debug for Smcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Smcr")
                .field("bkprwdprot", &self.bkprwdprot())
                .field("bkpwdprot", &self.bkpwdprot())
                .field("tampdprot", &self.tampdprot())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Smcr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Smcr {
                bkprwdprot: u8,
                bkpwdprot: u8,
                tampdprot: bool,
            }
            let proxy = Smcr {
                bkprwdprot: self.bkprwdprot(),
                bkpwdprot: self.bkpwdprot(),
                tampdprot: self.tampdprot(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "TAMP secure masked interrupt status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Smisr(pub u32);
    impl Smisr {
        #[doc = "Tamper X interrupt masked flag"]
        #[inline(always)]
        pub const fn tampmf(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper X interrupt masked flag"]
        #[inline(always)]
        pub fn set_tampmf(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Internal tamper X interrupt masked flag"]
        #[inline(always)]
        pub const fn itampmf(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper X interrupt masked flag"]
        #[inline(always)]
        pub fn set_itampmf(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
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
    impl core::fmt::Debug for Smisr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Smisr")
                .field(
                    "tampmf",
                    &[
                        self.tampmf(0usize),
                        self.tampmf(1usize),
                        self.tampmf(2usize),
                        self.tampmf(3usize),
                        self.tampmf(4usize),
                        self.tampmf(5usize),
                        self.tampmf(6usize),
                        self.tampmf(7usize),
                    ],
                )
                .field(
                    "itampmf",
                    &[
                        self.itampmf(0usize),
                        self.itampmf(1usize),
                        self.itampmf(2usize),
                        self.itampmf(3usize),
                        self.itampmf(4usize),
                        self.itampmf(5usize),
                        self.itampmf(6usize),
                        self.itampmf(7usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Smisr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Smisr {
                tampmf: [bool; 8usize],
                itampmf: [bool; 8usize],
            }
            let proxy = Smisr {
                tampmf: [
                    self.tampmf(0usize),
                    self.tampmf(1usize),
                    self.tampmf(2usize),
                    self.tampmf(3usize),
                    self.tampmf(4usize),
                    self.tampmf(5usize),
                    self.tampmf(6usize),
                    self.tampmf(7usize),
                ],
                itampmf: [
                    self.itampmf(0usize),
                    self.itampmf(1usize),
                    self.itampmf(2usize),
                    self.itampmf(3usize),
                    self.itampmf(4usize),
                    self.itampmf(5usize),
                    self.itampmf(6usize),
                    self.itampmf(7usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "TAMP status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Tamper X detection flag"]
        #[inline(always)]
        pub const fn tampf(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper X detection flag"]
        #[inline(always)]
        pub fn set_tampf(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Internal tamper X detection flag"]
        #[inline(always)]
        pub const fn itampf(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper X detection flag"]
        #[inline(always)]
        pub fn set_itampf(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
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
    impl core::fmt::Debug for Sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr")
                .field(
                    "tampf",
                    &[
                        self.tampf(0usize),
                        self.tampf(1usize),
                        self.tampf(2usize),
                        self.tampf(3usize),
                        self.tampf(4usize),
                        self.tampf(5usize),
                        self.tampf(6usize),
                        self.tampf(7usize),
                    ],
                )
                .field(
                    "itampf",
                    &[
                        self.itampf(0usize),
                        self.itampf(1usize),
                        self.itampf(2usize),
                        self.itampf(3usize),
                        self.itampf(4usize),
                        self.itampf(5usize),
                        self.itampf(6usize),
                        self.itampf(7usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Sr {
                tampf: [bool; 8usize],
                itampf: [bool; 8usize],
            }
            let proxy = Sr {
                tampf: [
                    self.tampf(0usize),
                    self.tampf(1usize),
                    self.tampf(2usize),
                    self.tampf(3usize),
                    self.tampf(4usize),
                    self.tampf(5usize),
                    self.tampf(6usize),
                    self.tampf(7usize),
                ],
                itampf: [
                    self.itampf(0usize),
                    self.itampf(1usize),
                    self.itampf(2usize),
                    self.itampf(3usize),
                    self.itampf(4usize),
                    self.itampf(5usize),
                    self.itampf(6usize),
                    self.itampf(7usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
