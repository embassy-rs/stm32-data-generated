#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Coupling and chaining bridge"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccb {
    ptr: *mut u8,
}
unsafe impl Send for Ccb {}
unsafe impl Sync for Ccb {}
impl Ccb {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "CCB control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "CCB status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "CCB reference tag register"]
    #[inline(always)]
    pub const fn reftagr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "CCB control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Coupling and chaining operation"]
        #[must_use]
        #[inline(always)]
        pub const fn ccop(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Coupling and chaining operation"]
        #[inline(always)]
        pub const fn set_ccop(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "CCB reset"]
        #[must_use]
        #[inline(always)]
        pub const fn iprst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "CCB reset"]
        #[inline(always)]
        pub const fn set_iprst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    impl core::fmt::Debug for Cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr")
                .field("ccop", &self.ccop())
                .field("iprst", &self.iprst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cr {{ ccop: {=u8:?}, iprst: {=bool:?} }}", self.ccop(), self.iprst())
        }
    }
    #[doc = "CCB status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Operation step"]
        #[must_use]
        #[inline(always)]
        pub const fn opstep(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Operation step"]
        #[inline(always)]
        pub const fn set_opstep(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Operation error"]
        #[must_use]
        #[inline(always)]
        pub const fn operr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Operation error"]
        #[inline(always)]
        pub const fn set_operr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "CCB busy"]
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "CCB busy"]
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Tamper event flag"]
        #[must_use]
        #[inline(always)]
        pub const fn tamp_evt(&self, n: usize) -> bool {
            assert!(n < 5usize);
            let offs = 24usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Tamper event flag"]
        #[inline(always)]
        pub const fn set_tamp_evt(&mut self, n: usize, val: bool) {
            assert!(n < 5usize);
            let offs = 24usize + n * 1usize;
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
                .field("opstep", &self.opstep())
                .field("operr", &self.operr())
                .field("busy", &self.busy())
                .field("tamp_evt[0]", &self.tamp_evt(0usize))
                .field("tamp_evt[1]", &self.tamp_evt(1usize))
                .field("tamp_evt[2]", &self.tamp_evt(2usize))
                .field("tamp_evt[3]", &self.tamp_evt(3usize))
                .field("tamp_evt[4]", &self.tamp_evt(4usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ opstep: {=u8:?}, operr: {=u8:?}, busy: {=bool:?}, tamp_evt[0]: {=bool:?}, tamp_evt[1]: {=bool:?}, tamp_evt[2]: {=bool:?}, tamp_evt[3]: {=bool:?}, tamp_evt[4]: {=bool:?} }}",
                self.opstep(),
                self.operr(),
                self.busy(),
                self.tamp_evt(0usize),
                self.tamp_evt(1usize),
                self.tamp_evt(2usize),
                self.tamp_evt(3usize),
                self.tamp_evt(4usize)
            )
        }
    }
}
