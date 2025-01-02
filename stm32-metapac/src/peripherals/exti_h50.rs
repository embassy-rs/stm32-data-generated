#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Extended interrupt and event controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Exti {
    ptr: *mut u8,
}
unsafe impl Send for Exti {}
unsafe impl Sync for Exti {}
impl Exti {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "rising trigger selection register"]
    #[inline(always)]
    pub const fn rtsr(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 32usize) as _) }
    }
    #[doc = "falling trigger selection register"]
    #[inline(always)]
    pub const fn ftsr(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize + n * 32usize) as _) }
    }
    #[doc = "software interrupt event register"]
    #[inline(always)]
    pub const fn swier(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 32usize) as _) }
    }
    #[doc = "rising edge pending register"]
    #[inline(always)]
    pub const fn rpr(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize + n * 32usize) as _) }
    }
    #[doc = "falling edge pending register"]
    #[inline(always)]
    pub const fn fpr(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize + n * 32usize) as _) }
    }
    #[doc = "privilege configuration register"]
    #[inline(always)]
    pub const fn privcfgr(self, n: usize) -> crate::common::Reg<regs::Priv, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize + n * 32usize) as _) }
    }
    #[doc = "external interrupt selection register"]
    #[inline(always)]
    pub const fn exticr(self, n: usize) -> crate::common::Reg<regs::Exti, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize + n * 4usize) as _) }
    }
    #[doc = "CPU wakeup with interrupt mask register"]
    #[inline(always)]
    pub const fn imr(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 16usize) as _) }
    }
    #[doc = "CPU wakeup with event mask register"]
    #[inline(always)]
    pub const fn emr(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize + n * 16usize) as _) }
    }
}
pub mod regs {
    #[doc = "EXTI external interrupt selection register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exti(pub u32);
    impl Exti {
        #[doc = "EXTI12 GPIO port selection These bits are written by software to select the source input for EXTI12 external interrupt. When EXTI_PRIVCFGR.PRIV12 is disabled, EXTI12 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV12 is enabled, EXTI12 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
        #[inline(always)]
        pub const fn exti(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0xff;
            val as u8
        }
        #[doc = "EXTI12 GPIO port selection These bits are written by software to select the source input for EXTI12 external interrupt. When EXTI_PRIVCFGR.PRIV12 is disabled, EXTI12 can be accessed with privileged and unprivileged access. When EXTI_PRIVCFGR.PRIV12 is enabled, EXTI12 can only be accessed with privileged access. Unprivileged write to this bit is discarded. Others: reserved"]
        #[inline(always)]
        pub fn set_exti(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0xff << offs)) | (((val as u32) & 0xff) << offs);
        }
    }
    impl Default for Exti {
        #[inline(always)]
        fn default() -> Exti {
            Exti(0)
        }
    }
    impl core::fmt::Debug for Exti {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Exti")
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
    impl defmt::Format for Exti {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Exti {
                exti: [u8; 4usize],
            }
            let proxy = Exti {
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
    #[doc = "EXTI lines register, 1 bit per line"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lines(pub u32);
    impl Lines {
        #[doc = "EXTI line"]
        #[inline(always)]
        pub const fn line(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "EXTI line"]
        #[inline(always)]
        pub fn set_line(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Lines {
        #[inline(always)]
        fn default() -> Lines {
            Lines(0)
        }
    }
    impl core::fmt::Debug for Lines {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lines")
                .field(
                    "line",
                    &[
                        self.line(0usize),
                        self.line(1usize),
                        self.line(2usize),
                        self.line(3usize),
                        self.line(4usize),
                        self.line(5usize),
                        self.line(6usize),
                        self.line(7usize),
                        self.line(8usize),
                        self.line(9usize),
                        self.line(10usize),
                        self.line(11usize),
                        self.line(12usize),
                        self.line(13usize),
                        self.line(14usize),
                        self.line(15usize),
                        self.line(16usize),
                        self.line(17usize),
                        self.line(18usize),
                        self.line(19usize),
                        self.line(20usize),
                        self.line(21usize),
                        self.line(22usize),
                        self.line(23usize),
                        self.line(24usize),
                        self.line(25usize),
                        self.line(26usize),
                        self.line(27usize),
                        self.line(28usize),
                        self.line(29usize),
                        self.line(30usize),
                        self.line(31usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lines {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Lines {
                line: [bool; 32usize],
            }
            let proxy = Lines {
                line: [
                    self.line(0usize),
                    self.line(1usize),
                    self.line(2usize),
                    self.line(3usize),
                    self.line(4usize),
                    self.line(5usize),
                    self.line(6usize),
                    self.line(7usize),
                    self.line(8usize),
                    self.line(9usize),
                    self.line(10usize),
                    self.line(11usize),
                    self.line(12usize),
                    self.line(13usize),
                    self.line(14usize),
                    self.line(15usize),
                    self.line(16usize),
                    self.line(17usize),
                    self.line(18usize),
                    self.line(19usize),
                    self.line(20usize),
                    self.line(21usize),
                    self.line(22usize),
                    self.line(23usize),
                    self.line(24usize),
                    self.line(25usize),
                    self.line(26usize),
                    self.line(27usize),
                    self.line(28usize),
                    self.line(29usize),
                    self.line(30usize),
                    self.line(31usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "privilege configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Priv(pub u32);
    impl Priv {
        #[doc = "Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
        #[inline(always)]
        pub const fn priv_(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Security enable on event input x When EXTI_SECCFGR.SECx is disabled, PRIVx can be accessed with secure and non-secure access. When EXTI_SECCFGR.SECx is enabled, PRIVx can only be written with secure access. Non-secure write to this PRIVx is discarded."]
        #[inline(always)]
        pub fn set_priv_(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Priv {
        #[inline(always)]
        fn default() -> Priv {
            Priv(0)
        }
    }
    impl core::fmt::Debug for Priv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Priv")
                .field(
                    "priv_",
                    &[
                        self.priv_(0usize),
                        self.priv_(1usize),
                        self.priv_(2usize),
                        self.priv_(3usize),
                        self.priv_(4usize),
                        self.priv_(5usize),
                        self.priv_(6usize),
                        self.priv_(7usize),
                        self.priv_(8usize),
                        self.priv_(9usize),
                        self.priv_(10usize),
                        self.priv_(11usize),
                        self.priv_(12usize),
                        self.priv_(13usize),
                        self.priv_(14usize),
                        self.priv_(15usize),
                        self.priv_(16usize),
                        self.priv_(17usize),
                        self.priv_(18usize),
                        self.priv_(19usize),
                        self.priv_(20usize),
                        self.priv_(21usize),
                        self.priv_(22usize),
                        self.priv_(23usize),
                        self.priv_(24usize),
                        self.priv_(25usize),
                        self.priv_(26usize),
                        self.priv_(27usize),
                        self.priv_(28usize),
                        self.priv_(29usize),
                        self.priv_(30usize),
                        self.priv_(31usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Priv {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Priv {
                priv_: [bool; 32usize],
            }
            let proxy = Priv {
                priv_: [
                    self.priv_(0usize),
                    self.priv_(1usize),
                    self.priv_(2usize),
                    self.priv_(3usize),
                    self.priv_(4usize),
                    self.priv_(5usize),
                    self.priv_(6usize),
                    self.priv_(7usize),
                    self.priv_(8usize),
                    self.priv_(9usize),
                    self.priv_(10usize),
                    self.priv_(11usize),
                    self.priv_(12usize),
                    self.priv_(13usize),
                    self.priv_(14usize),
                    self.priv_(15usize),
                    self.priv_(16usize),
                    self.priv_(17usize),
                    self.priv_(18usize),
                    self.priv_(19usize),
                    self.priv_(20usize),
                    self.priv_(21usize),
                    self.priv_(22usize),
                    self.priv_(23usize),
                    self.priv_(24usize),
                    self.priv_(25usize),
                    self.priv_(26usize),
                    self.priv_(27usize),
                    self.priv_(28usize),
                    self.priv_(29usize),
                    self.priv_(30usize),
                    self.priv_(31usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
