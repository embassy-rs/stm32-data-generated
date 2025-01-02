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
    #[doc = "TAMP filter control register"]
    #[inline(always)]
    pub const fn fltcr(self) -> crate::common::Reg<regs::Fltcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
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
    #[doc = "TAMP status clear register"]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::Scr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "TAMP backup register"]
    #[inline(always)]
    pub const fn bkpr(self, n: usize) -> crate::common::Reg<regs::Bkpr, crate::common::RW> {
        assert!(n < 5usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
    }
    #[doc = "TAMP hardware configuration register 2"]
    #[inline(always)]
    pub const fn hwcfgr2(self) -> crate::common::Reg<regs::Hwcfgr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03ecusize) as _) }
    }
    #[doc = "TAMP hardware configuration register 1"]
    #[inline(always)]
    pub const fn hwcfgr1(self) -> crate::common::Reg<regs::Hwcfgr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03f0usize) as _) }
    }
    #[doc = "EXTI IP Version register"]
    #[inline(always)]
    pub const fn verr(self) -> crate::common::Reg<regs::Verr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03f4usize) as _) }
    }
    #[doc = "EXTI Identification register"]
    #[inline(always)]
    pub const fn ipidr(self) -> crate::common::Reg<regs::Ipidr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03f8usize) as _) }
    }
    #[doc = "EXTI Size ID register"]
    #[inline(always)]
    pub const fn sidr(self) -> crate::common::Reg<regs::Sidr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03fcusize) as _) }
    }
}
pub mod regs {
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
    #[doc = "control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "Tamper detection on IN X enable"]
        #[inline(always)]
        pub const fn tampe(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper detection on IN X enable"]
        #[inline(always)]
        pub fn set_tampe(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Internal tamper X enable"]
        #[inline(always)]
        pub const fn itampe(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper X enable"]
        #[inline(always)]
        pub fn set_itampe(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
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
                .field("tampe", &[self.tampe(0usize), self.tampe(1usize)])
                .field(
                    "itampe",
                    &[
                        self.itampe(0usize),
                        self.itampe(1usize),
                        self.itampe(2usize),
                        self.itampe(3usize),
                        self.itampe(4usize),
                        self.itampe(5usize),
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
                tampe: [bool; 2usize],
                itampe: [bool; 6usize],
            }
            let proxy = Cr1 {
                tampe: [self.tampe(0usize), self.tampe(1usize)],
                itampe: [
                    self.itampe(0usize),
                    self.itampe(1usize),
                    self.itampe(2usize),
                    self.itampe(3usize),
                    self.itampe(4usize),
                    self.itampe(5usize),
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
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper X no erase"]
        #[inline(always)]
        pub fn set_tampnoer(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Tamper X mask"]
        #[inline(always)]
        pub const fn tampmsk(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper X mask"]
        #[inline(always)]
        pub fn set_tampmsk(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Active level for tamper X input"]
        #[inline(always)]
        pub const fn tamptrg(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 24usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Active level for tamper X input"]
        #[inline(always)]
        pub fn set_tamptrg(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
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
                .field("tampnoer", &[self.tampnoer(0usize), self.tampnoer(1usize)])
                .field("tampmsk", &[self.tampmsk(0usize), self.tampmsk(1usize)])
                .field("tamptrg", &[self.tamptrg(0usize), self.tamptrg(1usize)])
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cr2 {
                tampnoer: [bool; 2usize],
                tampmsk: [bool; 2usize],
                tamptrg: [bool; 2usize],
            }
            let proxy = Cr2 {
                tampnoer: [self.tampnoer(0usize), self.tampnoer(1usize)],
                tampmsk: [self.tampmsk(0usize), self.tampmsk(1usize)],
                tamptrg: [self.tamptrg(0usize), self.tamptrg(1usize)],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "TAMP filter control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fltcr(pub u32);
    impl Fltcr {
        #[doc = "Tamper sampling frequency. Determines the frequency at which each of the INx inputs are sampled."]
        #[inline(always)]
        pub const fn tampfreq(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Tamper sampling frequency. Determines the frequency at which each of the INx inputs are sampled."]
        #[inline(always)]
        pub fn set_tampfreq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "INx filter count. These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the INx inputs."]
        #[inline(always)]
        pub const fn tampflt(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "INx filter count. These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the INx inputs."]
        #[inline(always)]
        pub fn set_tampflt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "INx precharge duration. These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the INx inputs."]
        #[inline(always)]
        pub const fn tampprch(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "INx precharge duration. These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the INx inputs."]
        #[inline(always)]
        pub fn set_tampprch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
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
    #[doc = "TAMP hardware configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hwcfgr1(pub u32);
    impl Hwcfgr1 {
        #[doc = "BACKUP_REGS"]
        #[inline(always)]
        pub const fn backup_regs(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "BACKUP_REGS"]
        #[inline(always)]
        pub fn set_backup_regs(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "TAMPER"]
        #[inline(always)]
        pub const fn tamper(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "TAMPER"]
        #[inline(always)]
        pub fn set_tamper(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "ACTIVE_TAMPER"]
        #[inline(always)]
        pub const fn active_tamper(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "ACTIVE_TAMPER"]
        #[inline(always)]
        pub fn set_active_tamper(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "INT_TAMPER"]
        #[inline(always)]
        pub const fn int_tamper(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "INT_TAMPER"]
        #[inline(always)]
        pub fn set_int_tamper(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Hwcfgr1 {
        #[inline(always)]
        fn default() -> Hwcfgr1 {
            Hwcfgr1(0)
        }
    }
    impl core::fmt::Debug for Hwcfgr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hwcfgr1")
                .field("backup_regs", &self.backup_regs())
                .field("tamper", &self.tamper())
                .field("active_tamper", &self.active_tamper())
                .field("int_tamper", &self.int_tamper())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hwcfgr1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Hwcfgr1 {
                backup_regs: u8,
                tamper: u8,
                active_tamper: u8,
                int_tamper: u16,
            }
            let proxy = Hwcfgr1 {
                backup_regs: self.backup_regs(),
                tamper: self.tamper(),
                active_tamper: self.active_tamper(),
                int_tamper: self.int_tamper(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "TAMP hardware configuration register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hwcfgr2(pub u32);
    impl Hwcfgr2 {
        #[doc = "PTIONREG_OUT"]
        #[inline(always)]
        pub const fn ptionreg_out(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "PTIONREG_OUT"]
        #[inline(always)]
        pub fn set_ptionreg_out(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "TRUST_ZONE"]
        #[inline(always)]
        pub const fn trust_zone(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "TRUST_ZONE"]
        #[inline(always)]
        pub fn set_trust_zone(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
    }
    impl Default for Hwcfgr2 {
        #[inline(always)]
        fn default() -> Hwcfgr2 {
            Hwcfgr2(0)
        }
    }
    impl core::fmt::Debug for Hwcfgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hwcfgr2")
                .field("ptionreg_out", &self.ptionreg_out())
                .field("trust_zone", &self.trust_zone())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hwcfgr2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Hwcfgr2 {
                ptionreg_out: u8,
                trust_zone: u8,
            }
            let proxy = Hwcfgr2 {
                ptionreg_out: self.ptionreg_out(),
                trust_zone: self.trust_zone(),
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
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper X interrupt enable"]
        #[inline(always)]
        pub fn set_tampie(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Internal tamper X interrupt enable"]
        #[inline(always)]
        pub const fn itampie(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper X interrupt enable"]
        #[inline(always)]
        pub fn set_itampie(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
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
                .field("tampie", &[self.tampie(0usize), self.tampie(1usize)])
                .field(
                    "itampie",
                    &[
                        self.itampie(0usize),
                        self.itampie(1usize),
                        self.itampie(2usize),
                        self.itampie(3usize),
                        self.itampie(4usize),
                        self.itampie(5usize),
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
                tampie: [bool; 2usize],
                itampie: [bool; 6usize],
            }
            let proxy = Ier {
                tampie: [self.tampie(0usize), self.tampie(1usize)],
                itampie: [
                    self.itampie(0usize),
                    self.itampie(1usize),
                    self.itampie(2usize),
                    self.itampie(3usize),
                    self.itampie(4usize),
                    self.itampie(5usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "EXTI Identification register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipidr(pub u32);
    impl Ipidr {
        #[doc = "IP Identification"]
        #[inline(always)]
        pub const fn ipid(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "IP Identification"]
        #[inline(always)]
        pub fn set_ipid(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ipidr {
        #[inline(always)]
        fn default() -> Ipidr {
            Ipidr(0)
        }
    }
    impl core::fmt::Debug for Ipidr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ipidr").field("ipid", &self.ipid()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ipidr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ipidr {
                ipid: u32,
            }
            let proxy = Ipidr { ipid: self.ipid() };
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
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper X interrupt masked flag"]
        #[inline(always)]
        pub fn set_tampmf(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Internal tamper X interrupt masked flag"]
        #[inline(always)]
        pub const fn itampmf(&self, n: usize) -> bool {
            assert!(n < 6usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper X interrupt masked flag"]
        #[inline(always)]
        pub fn set_itampmf(&mut self, n: usize, val: bool) {
            assert!(n < 6usize);
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
                .field("tampmf", &[self.tampmf(0usize), self.tampmf(1usize)])
                .field(
                    "itampmf",
                    &[
                        self.itampmf(0usize),
                        self.itampmf(1usize),
                        self.itampmf(2usize),
                        self.itampmf(3usize),
                        self.itampmf(4usize),
                        self.itampmf(5usize),
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
                tampmf: [bool; 2usize],
                itampmf: [bool; 6usize],
            }
            let proxy = Misr {
                tampmf: [self.tampmf(0usize), self.tampmf(1usize)],
                itampmf: [
                    self.itampmf(0usize),
                    self.itampmf(1usize),
                    self.itampmf(2usize),
                    self.itampmf(3usize),
                    self.itampmf(4usize),
                    self.itampmf(5usize),
                ],
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
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Clear tamper X detection flag"]
        #[inline(always)]
        pub fn set_ctampf(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Clear internal tamper X detection flag"]
        #[inline(always)]
        pub const fn citampf(&self, n: usize) -> bool {
            assert!(n < 7usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Clear internal tamper X detection flag"]
        #[inline(always)]
        pub fn set_citampf(&mut self, n: usize, val: bool) {
            assert!(n < 7usize);
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
                .field("ctampf", &[self.ctampf(0usize), self.ctampf(1usize)])
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
                ctampf: [bool; 2usize],
                citampf: [bool; 7usize],
            }
            let proxy = Scr {
                ctampf: [self.ctampf(0usize), self.ctampf(1usize)],
                citampf: [
                    self.citampf(0usize),
                    self.citampf(1usize),
                    self.citampf(2usize),
                    self.citampf(3usize),
                    self.citampf(4usize),
                    self.citampf(5usize),
                    self.citampf(6usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "EXTI Size ID register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sidr(pub u32);
    impl Sidr {
        #[doc = "Size Identification"]
        #[inline(always)]
        pub const fn sid(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Size Identification"]
        #[inline(always)]
        pub fn set_sid(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Sidr {
        #[inline(always)]
        fn default() -> Sidr {
            Sidr(0)
        }
    }
    impl core::fmt::Debug for Sidr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sidr").field("sid", &self.sid()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sidr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Sidr {
                sid: u32,
            }
            let proxy = Sidr { sid: self.sid() };
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
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper X detection flag"]
        #[inline(always)]
        pub fn set_tampf(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Internal tamper X detection flag"]
        #[inline(always)]
        pub const fn itampf(&self, n: usize) -> bool {
            assert!(n < 7usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Internal tamper X detection flag"]
        #[inline(always)]
        pub fn set_itampf(&mut self, n: usize, val: bool) {
            assert!(n < 7usize);
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
                .field("tampf", &[self.tampf(0usize), self.tampf(1usize)])
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
                tampf: [bool; 2usize],
                itampf: [bool; 7usize],
            }
            let proxy = Sr {
                tampf: [self.tampf(0usize), self.tampf(1usize)],
                itampf: [
                    self.itampf(0usize),
                    self.itampf(1usize),
                    self.itampf(2usize),
                    self.itampf(3usize),
                    self.itampf(4usize),
                    self.itampf(5usize),
                    self.itampf(6usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "EXTI IP Version register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Verr(pub u32);
    impl Verr {
        #[doc = "Minor Revision number"]
        #[inline(always)]
        pub const fn minrev(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Minor Revision number"]
        #[inline(always)]
        pub fn set_minrev(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Major Revision number"]
        #[inline(always)]
        pub const fn majrev(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Major Revision number"]
        #[inline(always)]
        pub fn set_majrev(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Verr {
        #[inline(always)]
        fn default() -> Verr {
            Verr(0)
        }
    }
    impl core::fmt::Debug for Verr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Verr")
                .field("minrev", &self.minrev())
                .field("majrev", &self.majrev())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Verr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Verr {
                minrev: u8,
                majrev: u8,
            }
            let proxy = Verr {
                minrev: self.minrev(),
                majrev: self.majrev(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
