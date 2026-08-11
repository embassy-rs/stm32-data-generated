#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Temperature Sensor Factory Calibration"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tscal {
    ptr: *mut u8,
}
unsafe impl Send for Tscal {}
unsafe impl Sync for Tscal {}
impl Tscal {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Factory calibration point 1"]
    #[inline(always)]
    pub const fn tscal1(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Factory calibration point 2"]
    #[inline(always)]
    pub const fn tscal2(self) -> crate::common::Reg<u16, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x22usize) as _) }
    }
}
