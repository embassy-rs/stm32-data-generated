#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Video encoder."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Venc {
    ptr: *mut u8,
}
unsafe impl Send for Venc {}
unsafe impl Sync for Venc {}
impl Venc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "VENC ID register."]
    #[inline(always)]
    pub const fn swreg(self, n: usize) -> crate::common::Reg<regs::Swreg, crate::common::RW> {
        assert!(n < 499usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "VENC ID register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Swreg(pub u32);
    impl Swreg {
        #[doc = "Interrupt register (all format mode)."]
        #[must_use]
        #[inline(always)]
        pub const fn swreg_field(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Interrupt register (all format mode)."]
        #[inline(always)]
        pub const fn set_swreg_field(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Swreg {
        #[inline(always)]
        fn default() -> Swreg {
            Swreg(0)
        }
    }
    impl core::fmt::Debug for Swreg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Swreg")
                .field("swreg_field", &self.swreg_field())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Swreg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Swreg {{ swreg_field: {=u32:?} }}", self.swreg_field())
        }
    }
}
