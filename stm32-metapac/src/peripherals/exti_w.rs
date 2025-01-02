#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "CPU-specific registers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpu {
    ptr: *mut u8,
}
unsafe impl Send for Cpu {}
unsafe impl Sync for Cpu {}
impl Cpu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "CPU x interrupt mask register"]
    #[inline(always)]
    pub const fn imr(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 16usize) as _) }
    }
    #[doc = "CPU x event mask register"]
    #[inline(always)]
    pub const fn emr(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize + n * 16usize) as _) }
    }
}
#[doc = "External interrupt/event controller"]
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
    #[doc = "EXTI pending register"]
    #[inline(always)]
    pub const fn pr(self, n: usize) -> crate::common::Reg<regs::Lines, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize + n * 32usize) as _) }
    }
    #[doc = "CPU specific registers"]
    #[inline(always)]
    pub const fn cpu(self, n: usize) -> Cpu {
        assert!(n < 2usize);
        unsafe { Cpu::from_ptr(self.ptr.add(0x80usize + n * 64usize) as _) }
    }
}
pub mod regs {
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
}
