#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "2D graphics accelerator."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpu2d {
    ptr: *mut u8,
}
unsafe impl Send for Gpu2d {}
unsafe impl Sync for Gpu2d {}
impl Gpu2d {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Breakpoint register."]
    #[inline(always)]
    pub const fn breakpoint(self) -> crate::common::Reg<regs::Breakpoint, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "Interrupt control register."]
    #[inline(always)]
    pub const fn itctrl(self) -> crate::common::Reg<regs::Itctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "Last command list identifier register."]
    #[inline(always)]
    pub const fn clid(self) -> crate::common::Reg<regs::Clid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "System interrupt register."]
    #[inline(always)]
    pub const fn sys_interrupt(self) -> crate::common::Reg<regs::SysInterrupt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff8usize) as _) }
    }
}
pub mod regs {
    #[doc = "Breakpoint register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Breakpoint(pub u32);
    impl Breakpoint {
        #[doc = "Breakpoint value."]
        #[must_use]
        #[inline(always)]
        pub const fn value(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Breakpoint value."]
        #[inline(always)]
        pub const fn set_value(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Breakpoint {
        #[inline(always)]
        fn default() -> Breakpoint {
            Breakpoint(0)
        }
    }
    impl core::fmt::Debug for Breakpoint {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Breakpoint").field("value", &self.value()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Breakpoint {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Breakpoint {{ value: {=u32:?} }}", self.value())
        }
    }
    #[doc = "Last command list identifier register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Clid(pub u32);
    impl Clid {
        #[doc = "Last completed command list identifier."]
        #[must_use]
        #[inline(always)]
        pub const fn id(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Last completed command list identifier."]
        #[inline(always)]
        pub const fn set_id(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Clid {
        #[inline(always)]
        fn default() -> Clid {
            Clid(0)
        }
    }
    impl core::fmt::Debug for Clid {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Clid").field("id", &self.id()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Clid {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Clid {{ id: {=u32:?} }}", self.id())
        }
    }
    #[doc = "Interrupt control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Itctrl(pub u32);
    impl Itctrl {
        #[doc = "Command list complete interrupt and flag."]
        #[must_use]
        #[inline(always)]
        pub const fn clc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Command list complete interrupt and flag."]
        #[inline(always)]
        pub const fn set_clc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Itctrl {
        #[inline(always)]
        fn default() -> Itctrl {
            Itctrl(0)
        }
    }
    impl core::fmt::Debug for Itctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Itctrl").field("clc", &self.clc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Itctrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Itctrl {{ clc: {=bool:?} }}", self.clc())
        }
    }
    #[doc = "System interrupt register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SysInterrupt(pub u32);
    impl SysInterrupt {
        #[doc = "Error interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn er(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt status."]
        #[inline(always)]
        pub const fn set_er(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for SysInterrupt {
        #[inline(always)]
        fn default() -> SysInterrupt {
            SysInterrupt(0)
        }
    }
    impl core::fmt::Debug for SysInterrupt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SysInterrupt").field("er", &self.er()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SysInterrupt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SysInterrupt {{ er: {=bool:?} }}", self.er())
        }
    }
}
