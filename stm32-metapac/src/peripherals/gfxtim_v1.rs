#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Graphic timer."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gfxtim {
    ptr: *mut u8,
}
unsafe impl Send for Gfxtim {}
unsafe impl Sync for Gfxtim {}
impl Gfxtim {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GFXTIM configuration register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "GFXTIM clock generator configuration register."]
    #[inline(always)]
    pub const fn cgcr(self) -> crate::common::Reg<regs::Cgcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "GFXTIM timers configuration register."]
    #[inline(always)]
    pub const fn tcr(self) -> crate::common::Reg<regs::Tcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "GFXTIM timers disable register."]
    #[inline(always)]
    pub const fn tdr(self) -> crate::common::Reg<regs::Tdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "GFXTIM events control register."]
    #[inline(always)]
    pub const fn evcr(self) -> crate::common::Reg<regs::Evcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "GFXTIM events selection register."]
    #[inline(always)]
    pub const fn evsr(self) -> crate::common::Reg<regs::Evsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "GFXTIM watchdog timer configuration register."]
    #[inline(always)]
    pub const fn wdgtcr(self) -> crate::common::Reg<regs::Wdgtcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "GFXTIM interrupt status register."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "GFXTIM interrupt clear register."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "GFXTIM interrupt enable register."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "GFXTIM timers status register."]
    #[inline(always)]
    pub const fn tsr(self) -> crate::common::Reg<regs::Tsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "GFXTIM line clock counter reload register."]
    #[inline(always)]
    pub const fn lccrr(self) -> crate::common::Reg<regs::Lccrr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "GFXTIM frame clock counter reload register."]
    #[inline(always)]
    pub const fn fccrr(self) -> crate::common::Reg<regs::Fccrr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "GFXTIM absolute time register."]
    #[inline(always)]
    pub const fn atr(self) -> crate::common::Reg<regs::Atr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "GFXTIM absolute frame counter register."]
    #[inline(always)]
    pub const fn afcr(self) -> crate::common::Reg<regs::Afcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "GFXTIM absolute line counter register."]
    #[inline(always)]
    pub const fn alcr(self) -> crate::common::Reg<regs::Alcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "GFXTIM absolute frame counter compare 1 register."]
    #[inline(always)]
    pub const fn afcc1r(self) -> crate::common::Reg<regs::Afcc1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "GFXTIM absolute line counter compare 1 register."]
    #[inline(always)]
    pub const fn alcc1r(self) -> crate::common::Reg<regs::Alcc1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "GFXTIM absolute line counter compare 2 register."]
    #[inline(always)]
    pub const fn alcc2r(self) -> crate::common::Reg<regs::Alcc2r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "GFXTIM relative frame counter 1 register."]
    #[inline(always)]
    pub const fn rfc1r(self) -> crate::common::Reg<regs::Rfc1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "GFXTIM relative frame counter 1 reload register."]
    #[inline(always)]
    pub const fn rfc1rr(self) -> crate::common::Reg<regs::Rfc1rr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "GFXTIM relative frame counter 2 register."]
    #[inline(always)]
    pub const fn rfc2r(self) -> crate::common::Reg<regs::Rfc2r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "GFXTIM relative frame counter 2 reload register."]
    #[inline(always)]
    pub const fn rfc2rr(self) -> crate::common::Reg<regs::Rfc2rr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "GFXTIM watchdog counter register."]
    #[inline(always)]
    pub const fn wdgcr(self) -> crate::common::Reg<regs::Wdgcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "GFXTIM watchdog reload register."]
    #[inline(always)]
    pub const fn wdgrr(self) -> crate::common::Reg<regs::Wdgrr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "GFXTIM watchdog pre-alarm register."]
    #[inline(always)]
    pub const fn wdgpar(self) -> crate::common::Reg<regs::Wdgpar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
}
pub mod regs {
    #[doc = "GFXTIM absolute frame counter compare 1 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Afcc1r(pub u32);
    impl Afcc1r {
        #[doc = "frame number Compare 1 value for the absolute frame counter."]
        #[must_use]
        #[inline(always)]
        pub const fn frame(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "frame number Compare 1 value for the absolute frame counter."]
        #[inline(always)]
        pub const fn set_frame(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for Afcc1r {
        #[inline(always)]
        fn default() -> Afcc1r {
            Afcc1r(0)
        }
    }
    impl core::fmt::Debug for Afcc1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Afcc1r").field("frame", &self.frame()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Afcc1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Afcc1r {{ frame: {=u32:?} }}", self.frame())
        }
    }
    #[doc = "GFXTIM absolute frame counter register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Afcr(pub u32);
    impl Afcr {
        #[doc = "frame number Current value of the absolute frame counter. Note: This field can only be written when the absolute frame counter is disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn frame(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "frame number Current value of the absolute frame counter. Note: This field can only be written when the absolute frame counter is disabled."]
        #[inline(always)]
        pub const fn set_frame(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
    }
    impl Default for Afcr {
        #[inline(always)]
        fn default() -> Afcr {
            Afcr(0)
        }
    }
    impl core::fmt::Debug for Afcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Afcr").field("frame", &self.frame()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Afcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Afcr {{ frame: {=u32:?} }}", self.frame())
        }
    }
    #[doc = "GFXTIM absolute line counter compare 1 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Alcc1r(pub u32);
    impl Alcc1r {
        #[doc = "line number Compare value 1 for the absolute line counter."]
        #[must_use]
        #[inline(always)]
        pub const fn line(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "line number Compare value 1 for the absolute line counter."]
        #[inline(always)]
        pub const fn set_line(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Alcc1r {
        #[inline(always)]
        fn default() -> Alcc1r {
            Alcc1r(0)
        }
    }
    impl core::fmt::Debug for Alcc1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Alcc1r").field("line", &self.line()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Alcc1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Alcc1r {{ line: {=u16:?} }}", self.line())
        }
    }
    #[doc = "GFXTIM absolute line counter compare 2 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Alcc2r(pub u32);
    impl Alcc2r {
        #[doc = "line number Compare value 2 for the absolute line counter."]
        #[must_use]
        #[inline(always)]
        pub const fn line(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "line number Compare value 2 for the absolute line counter."]
        #[inline(always)]
        pub const fn set_line(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Alcc2r {
        #[inline(always)]
        fn default() -> Alcc2r {
            Alcc2r(0)
        }
    }
    impl core::fmt::Debug for Alcc2r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Alcc2r").field("line", &self.line()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Alcc2r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Alcc2r {{ line: {=u16:?} }}", self.line())
        }
    }
    #[doc = "GFXTIM absolute line counter register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Alcr(pub u32);
    impl Alcr {
        #[doc = "line number Current value of the absolute line counter. Note: This field can only be written when the absolute frame counter is disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn line(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "line number Current value of the absolute line counter. Note: This field can only be written when the absolute frame counter is disabled."]
        #[inline(always)]
        pub const fn set_line(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Alcr {
        #[inline(always)]
        fn default() -> Alcr {
            Alcr(0)
        }
    }
    impl core::fmt::Debug for Alcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Alcr").field("line", &self.line()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Alcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Alcr {{ line: {=u16:?} }}", self.line())
        }
    }
    #[doc = "GFXTIM absolute time register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Atr(pub u32);
    impl Atr {
        #[doc = "line number Current value of the absolute line counter."]
        #[must_use]
        #[inline(always)]
        pub const fn line(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "line number Current value of the absolute line counter."]
        #[inline(always)]
        pub const fn set_line(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "fame number Current value of the absolute frame counter."]
        #[must_use]
        #[inline(always)]
        pub const fn frame(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "fame number Current value of the absolute frame counter."]
        #[inline(always)]
        pub const fn set_frame(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for Atr {
        #[inline(always)]
        fn default() -> Atr {
            Atr(0)
        }
    }
    impl core::fmt::Debug for Atr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Atr")
                .field("line", &self.line())
                .field("frame", &self.frame())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Atr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Atr {{ line: {=u16:?}, frame: {=u32:?} }}",
                self.line(),
                self.frame()
            )
        }
    }
    #[doc = "GFXTIM clock generator configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cgcr(pub u32);
    impl Cgcr {
        #[doc = "line clock source This field configures the line clock source."]
        #[must_use]
        #[inline(always)]
        pub const fn lcs(&self) -> super::vals::Lcs {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Lcs::from_bits(val as u8)
        }
        #[doc = "line clock source This field configures the line clock source."]
        #[inline(always)]
        pub const fn set_lcs(&mut self, val: super::vals::Lcs) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "line clock counter clock source This bit configures the clock source for the line clock counter."]
        #[must_use]
        #[inline(always)]
        pub const fn lcccs(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "line clock counter clock source This bit configures the clock source for the line clock counter."]
        #[inline(always)]
        pub const fn set_lcccs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "line clock counter force reload This bit forces line clock counter reload."]
        #[must_use]
        #[inline(always)]
        pub const fn lccfr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "line clock counter force reload This bit forces line clock counter reload."]
        #[inline(always)]
        pub const fn set_lccfr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "line clock counter hardware reload source This field configures the hardware reload source for the line clock counter."]
        #[must_use]
        #[inline(always)]
        pub const fn lcchrs(&self) -> super::vals::Lcchrs {
            let val = (self.0 >> 12usize) & 0x07;
            super::vals::Lcchrs::from_bits(val as u8)
        }
        #[doc = "line clock counter hardware reload source This field configures the hardware reload source for the line clock counter."]
        #[inline(always)]
        pub const fn set_lcchrs(&mut self, val: super::vals::Lcchrs) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
        }
        #[doc = "frame clock source This field configures the frame clock source."]
        #[must_use]
        #[inline(always)]
        pub const fn fcs(&self) -> super::vals::Fcs {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::Fcs::from_bits(val as u8)
        }
        #[doc = "frame clock source This field configures the frame clock source."]
        #[inline(always)]
        pub const fn set_fcs(&mut self, val: super::vals::Fcs) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
        #[doc = "frame clock counter clock source This field configures the clock source for the frame clock counter."]
        #[must_use]
        #[inline(always)]
        pub const fn fcccs(&self) -> super::vals::Fcccs {
            let val = (self.0 >> 20usize) & 0x07;
            super::vals::Fcccs::from_bits(val as u8)
        }
        #[doc = "frame clock counter clock source This field configures the clock source for the frame clock counter."]
        #[inline(always)]
        pub const fn set_fcccs(&mut self, val: super::vals::Fcccs) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
        }
        #[doc = "frame clock counter force reload This bit forces frame clock counter reload."]
        #[must_use]
        #[inline(always)]
        pub const fn fccfr(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "frame clock counter force reload This bit forces frame clock counter reload."]
        #[inline(always)]
        pub const fn set_fccfr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "frame- -clock counter hardware reload source This field configures the hardware reload source for the frame- -clock counter."]
        #[must_use]
        #[inline(always)]
        pub const fn fcchrs(&self) -> super::vals::Fcchrs {
            let val = (self.0 >> 28usize) & 0x07;
            super::vals::Fcchrs::from_bits(val as u8)
        }
        #[doc = "frame- -clock counter hardware reload source This field configures the hardware reload source for the frame- -clock counter."]
        #[inline(always)]
        pub const fn set_fcchrs(&mut self, val: super::vals::Fcchrs) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Cgcr {
        #[inline(always)]
        fn default() -> Cgcr {
            Cgcr(0)
        }
    }
    impl core::fmt::Debug for Cgcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cgcr")
                .field("lcs", &self.lcs())
                .field("lcccs", &self.lcccs())
                .field("lccfr", &self.lccfr())
                .field("lcchrs", &self.lcchrs())
                .field("fcs", &self.fcs())
                .field("fcccs", &self.fcccs())
                .field("fccfr", &self.fccfr())
                .field("fcchrs", &self.fcchrs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cgcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cgcr {{ lcs: {:?}, lcccs: {=bool:?}, lccfr: {=bool:?}, lcchrs: {:?}, fcs: {:?}, fcccs: {:?}, fccfr: {=bool:?}, fcchrs: {:?} }}",
                self.lcs(),
                self.lcccs(),
                self.lccfr(),
                self.lcchrs(),
                self.fcs(),
                self.fcccs(),
                self.fccfr(),
                self.fcchrs()
            )
        }
    }
    #[doc = "GFXTIM configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "tearing source This field selects the tearing-effect source."]
        #[must_use]
        #[inline(always)]
        pub const fn tes(&self) -> super::vals::Tes {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Tes::from_bits(val as u8)
        }
        #[doc = "tearing source This field selects the tearing-effect source."]
        #[inline(always)]
        pub const fn set_tes(&mut self, val: super::vals::Tes) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "tearing--effect polarity This bit selects the tearing-effect polarity."]
        #[must_use]
        #[inline(always)]
        pub const fn tepol(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "tearing--effect polarity This bit selects the tearing-effect polarity."]
        #[inline(always)]
        pub const fn set_tepol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "synchronization source This field selects the synchronization signals (HSYNC and VSYNC) sources."]
        #[must_use]
        #[inline(always)]
        pub const fn syncs(&self) -> super::vals::Syncs {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Syncs::from_bits(val as u8)
        }
        #[doc = "synchronization source This field selects the synchronization signals (HSYNC and VSYNC) sources."]
        #[inline(always)]
        pub const fn set_syncs(&mut self, val: super::vals::Syncs) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "frame-clock calibration output enable This bit enables the frame-clock output."]
        #[must_use]
        #[inline(always)]
        pub const fn fccoe(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "frame-clock calibration output enable This bit enables the frame-clock output."]
        #[inline(always)]
        pub const fn set_fccoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "line-clock calibration output enable This bit enables the line-clock output."]
        #[must_use]
        #[inline(always)]
        pub const fn lccoe(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "line-clock calibration output enable This bit enables the line-clock output."]
        #[inline(always)]
        pub const fn set_lccoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
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
                .field("tes", &self.tes())
                .field("tepol", &self.tepol())
                .field("syncs", &self.syncs())
                .field("fccoe", &self.fccoe())
                .field("lccoe", &self.lccoe())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ tes: {:?}, tepol: {=bool:?}, syncs: {:?}, fccoe: {=bool:?}, lccoe: {=bool:?} }}",
                self.tes(),
                self.tepol(),
                self.syncs(),
                self.fccoe(),
                self.lccoe()
            )
        }
    }
    #[doc = "GFXTIM events control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Evcr(pub u32);
    impl Evcr {
        #[doc = "event 1 enable This bit enables the complex event 1 generation."]
        #[must_use]
        #[inline(always)]
        pub const fn ev1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "event 1 enable This bit enables the complex event 1 generation."]
        #[inline(always)]
        pub const fn set_ev1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "event 2 enable This bit enables the complex event 2 generation."]
        #[must_use]
        #[inline(always)]
        pub const fn ev2en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "event 2 enable This bit enables the complex event 2 generation."]
        #[inline(always)]
        pub const fn set_ev2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "event 3 enable This bit enables the complex event 3 generation."]
        #[must_use]
        #[inline(always)]
        pub const fn ev3en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "event 3 enable This bit enables the complex event 3 generation."]
        #[inline(always)]
        pub const fn set_ev3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "event 4 enable This bit enables the complex event 4 generation."]
        #[must_use]
        #[inline(always)]
        pub const fn ev4en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "event 4 enable This bit enables the complex event 4 generation."]
        #[inline(always)]
        pub const fn set_ev4en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Evcr {
        #[inline(always)]
        fn default() -> Evcr {
            Evcr(0)
        }
    }
    impl core::fmt::Debug for Evcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Evcr")
                .field("ev1en", &self.ev1en())
                .field("ev2en", &self.ev2en())
                .field("ev3en", &self.ev3en())
                .field("ev4en", &self.ev4en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Evcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Evcr {{ ev1en: {=bool:?}, ev2en: {=bool:?}, ev3en: {=bool:?}, ev4en: {=bool:?} }}",
                self.ev1en(),
                self.ev2en(),
                self.ev3en(),
                self.ev4en()
            )
        }
    }
    #[doc = "GFXTIM events selection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Evsr(pub u32);
    impl Evsr {
        #[doc = "line-event selection 1 This field defines the line-event selection for complex event 1 generation. others: reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn les1(&self) -> super::vals::Les1 {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Les1::from_bits(val as u8)
        }
        #[doc = "line-event selection 1 This field defines the line-event selection for complex event 1 generation. others: reserved."]
        #[inline(always)]
        pub const fn set_les1(&mut self, val: super::vals::Les1) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "frame-event selection 1 This field defines the frame-event selection for complex event 1 generation. others: reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn fes1(&self) -> super::vals::Fes1 {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Fes1::from_bits(val as u8)
        }
        #[doc = "frame-event selection 1 This field defines the frame-event selection for complex event 1 generation. others: reserved."]
        #[inline(always)]
        pub const fn set_fes1(&mut self, val: super::vals::Fes1) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "line-event selection 2 This field defines the line-event selection for complex event 2 generation. others: reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn les2(&self) -> super::vals::Les2 {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Les2::from_bits(val as u8)
        }
        #[doc = "line-event selection 2 This field defines the line-event selection for complex event 2 generation. others: reserved."]
        #[inline(always)]
        pub const fn set_les2(&mut self, val: super::vals::Les2) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "frame-event selection 2 This field defines the frame-event selection for complex event 2 generation. others: reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn fes2(&self) -> super::vals::Fes2 {
            let val = (self.0 >> 12usize) & 0x07;
            super::vals::Fes2::from_bits(val as u8)
        }
        #[doc = "frame-event selection 2 This field defines the frame-event selection for complex event 2 generation. others: reserved."]
        #[inline(always)]
        pub const fn set_fes2(&mut self, val: super::vals::Fes2) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val.to_bits() as u32) & 0x07) << 12usize);
        }
        #[doc = "line-event selection 3 This field defines the line-event selection for complex event 3 generation. others: reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn les3(&self) -> super::vals::Les3 {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::Les3::from_bits(val as u8)
        }
        #[doc = "line-event selection 3 This field defines the line-event selection for complex event 3 generation. others: reserved."]
        #[inline(always)]
        pub const fn set_les3(&mut self, val: super::vals::Les3) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
        #[doc = "frame-event selection 3 This field defines the frame-event selection for complex event 3 generation. others: reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn fes3(&self) -> super::vals::Fes3 {
            let val = (self.0 >> 20usize) & 0x07;
            super::vals::Fes3::from_bits(val as u8)
        }
        #[doc = "frame-event selection 3 This field defines the frame-event selection for complex event 3 generation. others: reserved."]
        #[inline(always)]
        pub const fn set_fes3(&mut self, val: super::vals::Fes3) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val.to_bits() as u32) & 0x07) << 20usize);
        }
        #[doc = "line-event selection 4 This field defines the line-event selection for complex event 4 generation. others: Reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn les4(&self) -> super::vals::Les4 {
            let val = (self.0 >> 24usize) & 0x07;
            super::vals::Les4::from_bits(val as u8)
        }
        #[doc = "line-event selection 4 This field defines the line-event selection for complex event 4 generation. others: Reserved."]
        #[inline(always)]
        pub const fn set_les4(&mut self, val: super::vals::Les4) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
        }
        #[doc = "frame-event selection 4 This field defines the frame-event selection for complex event 4 generation. others: reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn fes4(&self) -> super::vals::Fes4 {
            let val = (self.0 >> 28usize) & 0x07;
            super::vals::Fes4::from_bits(val as u8)
        }
        #[doc = "frame-event selection 4 This field defines the frame-event selection for complex event 4 generation. others: reserved."]
        #[inline(always)]
        pub const fn set_fes4(&mut self, val: super::vals::Fes4) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val.to_bits() as u32) & 0x07) << 28usize);
        }
    }
    impl Default for Evsr {
        #[inline(always)]
        fn default() -> Evsr {
            Evsr(0)
        }
    }
    impl core::fmt::Debug for Evsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Evsr")
                .field("les1", &self.les1())
                .field("fes1", &self.fes1())
                .field("les2", &self.les2())
                .field("fes2", &self.fes2())
                .field("les3", &self.les3())
                .field("fes3", &self.fes3())
                .field("les4", &self.les4())
                .field("fes4", &self.fes4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Evsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Evsr {{ les1: {:?}, fes1: {:?}, les2: {:?}, fes2: {:?}, les3: {:?}, fes3: {:?}, les4: {:?}, fes4: {:?} }}",
                self.les1(),
                self.fes1(),
                self.les2(),
                self.fes2(),
                self.les3(),
                self.fes3(),
                self.les4(),
                self.fes4()
            )
        }
    }
    #[doc = "GFXTIM frame clock counter reload register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fccrr(pub u32);
    impl Fccrr {
        #[doc = "reload value Reload value of the frame clock counter."]
        #[must_use]
        #[inline(always)]
        pub const fn reload(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "reload value Reload value of the frame clock counter."]
        #[inline(always)]
        pub const fn set_reload(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Fccrr {
        #[inline(always)]
        fn default() -> Fccrr {
            Fccrr(0)
        }
    }
    impl core::fmt::Debug for Fccrr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fccrr").field("reload", &self.reload()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fccrr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Fccrr {{ reload: {=u16:?} }}", self.reload())
        }
    }
    #[doc = "GFXTIM interrupt clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "clear absolute frame counter overflow flag This bit clears AFCOF in GXTIM_ISR."]
        #[must_use]
        #[inline(always)]
        pub const fn cafcof(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "clear absolute frame counter overflow flag This bit clears AFCOF in GXTIM_ISR."]
        #[inline(always)]
        pub const fn set_cafcof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "clear absolute line counter overflow flag This bit clears ALCOF in GXTIM_ISR."]
        #[must_use]
        #[inline(always)]
        pub const fn calcof(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "clear absolute line counter overflow flag This bit clears ALCOF in GXTIM_ISR."]
        #[inline(always)]
        pub const fn set_calcof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "clear tearing-effect flag This bit clears TEF in GXTIM_ISR."]
        #[must_use]
        #[inline(always)]
        pub const fn ctef(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "clear tearing-effect flag This bit clears TEF in GXTIM_ISR."]
        #[inline(always)]
        pub const fn set_ctef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "clear absolute frame counter compare 1 flag This bit clears AFCC1F in GXTIM_ISR."]
        #[must_use]
        #[inline(always)]
        pub const fn cafcc1f(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "clear absolute frame counter compare 1 flag This bit clears AFCC1F in GXTIM_ISR."]
        #[inline(always)]
        pub const fn set_cafcc1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "clear absolute line counter compare 1 flag This bit clears ALCC1F in GXTIM_ISR."]
        #[must_use]
        #[inline(always)]
        pub const fn calcc1f(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "clear absolute line counter compare 1 flag This bit clears ALCC1F in GXTIM_ISR."]
        #[inline(always)]
        pub const fn set_calcc1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "clear absolute line counter compare 2 flag This bit clears ALCC2F in GXTIM_ISR."]
        #[must_use]
        #[inline(always)]
        pub const fn calcc2f(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "clear absolute line counter compare 2 flag This bit clears ALCC2F in GXTIM_ISR."]
        #[inline(always)]
        pub const fn set_calcc2f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "clear relative frame counter 1 reload flag This bit clears RFC1RF in GXTIM_ISR."]
        #[must_use]
        #[inline(always)]
        pub const fn crfc1rf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "clear relative frame counter 1 reload flag This bit clears RFC1RF in GXTIM_ISR."]
        #[inline(always)]
        pub const fn set_crfc1rf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "clear relative frame counter 2 reload flag This bit clears RFC2RF in GXFXTIM_ISR."]
        #[must_use]
        #[inline(always)]
        pub const fn crfc2rf(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "clear relative frame counter 2 reload flag This bit clears RFC2RF in GXFXTIM_ISR."]
        #[inline(always)]
        pub const fn set_crfc2rf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "clear event 1 flag This bit EV1F in GXFXTIM_ISR."]
        #[must_use]
        #[inline(always)]
        pub const fn cev1f(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "clear event 1 flag This bit EV1F in GXFXTIM_ISR."]
        #[inline(always)]
        pub const fn set_cev1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "clear event 2 flag This bit clears EV2F in GXFXTIM_ISR."]
        #[must_use]
        #[inline(always)]
        pub const fn cev2f(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "clear event 2 flag This bit clears EV2F in GXFXTIM_ISR."]
        #[inline(always)]
        pub const fn set_cev2f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "clear event 3 flag This bit clears EV3F in GXFXTIM_ISR."]
        #[must_use]
        #[inline(always)]
        pub const fn cev3f(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "clear event 3 flag This bit clears EV3F in GXFXTIM_ISR."]
        #[inline(always)]
        pub const fn set_cev3f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "clear event 4 flag This bit clears EV4F in GXFXTIM_ISR."]
        #[must_use]
        #[inline(always)]
        pub const fn cev4f(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "clear event 4 flag This bit clears EV4F in GXFXTIM_ISR."]
        #[inline(always)]
        pub const fn set_cev4f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "clear watchdog alarm flag This bit clears WDGAF in GXFXTIM_ISR."]
        #[must_use]
        #[inline(always)]
        pub const fn cwdgaf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "clear watchdog alarm flag This bit clears WDGAF in GXFXTIM_ISR."]
        #[inline(always)]
        pub const fn set_cwdgaf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "clear watchdog pre-alarm flag This bit clears WDGPF in GXFXTIM_ISR."]
        #[must_use]
        #[inline(always)]
        pub const fn cwdgpf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "clear watchdog pre-alarm flag This bit clears WDGPF in GXFXTIM_ISR."]
        #[inline(always)]
        pub const fn set_cwdgpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Icr {
        #[inline(always)]
        fn default() -> Icr {
            Icr(0)
        }
    }
    impl core::fmt::Debug for Icr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icr")
                .field("cafcof", &self.cafcof())
                .field("calcof", &self.calcof())
                .field("ctef", &self.ctef())
                .field("cafcc1f", &self.cafcc1f())
                .field("calcc1f", &self.calcc1f())
                .field("calcc2f", &self.calcc2f())
                .field("crfc1rf", &self.crfc1rf())
                .field("crfc2rf", &self.crfc2rf())
                .field("cev1f", &self.cev1f())
                .field("cev2f", &self.cev2f())
                .field("cev3f", &self.cev3f())
                .field("cev4f", &self.cev4f())
                .field("cwdgaf", &self.cwdgaf())
                .field("cwdgpf", &self.cwdgpf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icr {{ cafcof: {=bool:?}, calcof: {=bool:?}, ctef: {=bool:?}, cafcc1f: {=bool:?}, calcc1f: {=bool:?}, calcc2f: {=bool:?}, crfc1rf: {=bool:?}, crfc2rf: {=bool:?}, cev1f: {=bool:?}, cev2f: {=bool:?}, cev3f: {=bool:?}, cev4f: {=bool:?}, cwdgaf: {=bool:?}, cwdgpf: {=bool:?} }}",
                self.cafcof(),
                self.calcof(),
                self.ctef(),
                self.cafcc1f(),
                self.calcc1f(),
                self.calcc2f(),
                self.crfc1rf(),
                self.crfc2rf(),
                self.cev1f(),
                self.cev2f(),
                self.cev3f(),
                self.cev4f(),
                self.cwdgaf(),
                self.cwdgpf()
            )
        }
    }
    #[doc = "GFXTIM interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "absolute frame counter overflow interrupt enable This bit enables the absolute frame counter overflow interrupt generation."]
        #[must_use]
        #[inline(always)]
        pub const fn afcoie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "absolute frame counter overflow interrupt enable This bit enables the absolute frame counter overflow interrupt generation."]
        #[inline(always)]
        pub const fn set_afcoie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "absolute line counter overflow interrupt enable This bit enables the absolute line counter overflow interrupt generation."]
        #[must_use]
        #[inline(always)]
        pub const fn alcoie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "absolute line counter overflow interrupt enable This bit enables the absolute line counter overflow interrupt generation."]
        #[inline(always)]
        pub const fn set_alcoie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "tearing-effect interrupt enable This bit enables the Tearing Effect interrupt generation."]
        #[must_use]
        #[inline(always)]
        pub const fn teie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "tearing-effect interrupt enable This bit enables the Tearing Effect interrupt generation."]
        #[inline(always)]
        pub const fn set_teie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "absolute frame counter compare 1 interrupt enable This bit enables the absolute frame counter compare interrupt generation."]
        #[must_use]
        #[inline(always)]
        pub const fn afcc1ie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "absolute frame counter compare 1 interrupt enable This bit enables the absolute frame counter compare interrupt generation."]
        #[inline(always)]
        pub const fn set_afcc1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "absolute line counter compare 1 interrupt enable This bit enables the absolute line counter compare 1 interrupt generation."]
        #[must_use]
        #[inline(always)]
        pub const fn alcc1ie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "absolute line counter compare 1 interrupt enable This bit enables the absolute line counter compare 1 interrupt generation."]
        #[inline(always)]
        pub const fn set_alcc1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "absolute line counter compare 2 interrupt enable This bit enables the absolute line counter compare 2 interrupt generation."]
        #[must_use]
        #[inline(always)]
        pub const fn alcc2ie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "absolute line counter compare 2 interrupt enable This bit enables the absolute line counter compare 2 interrupt generation."]
        #[inline(always)]
        pub const fn set_alcc2ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "relative frame counter 1 reload interrupt enable This bit enables the relative frame counter 1 reload interrupt generation."]
        #[must_use]
        #[inline(always)]
        pub const fn rfc1rie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "relative frame counter 1 reload interrupt enable This bit enables the relative frame counter 1 reload interrupt generation."]
        #[inline(always)]
        pub const fn set_rfc1rie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "relative frame counter 2 reload interrupt enable This bit enables the relative frame counter 2 reload interrupt generation."]
        #[must_use]
        #[inline(always)]
        pub const fn rfc2rie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "relative frame counter 2 reload interrupt enable This bit enables the relative frame counter 2 reload interrupt generation."]
        #[inline(always)]
        pub const fn set_rfc2rie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "event 1 interrupt enable This bit enables the complex event 1 interrupt generation."]
        #[must_use]
        #[inline(always)]
        pub const fn ev1ie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "event 1 interrupt enable This bit enables the complex event 1 interrupt generation."]
        #[inline(always)]
        pub const fn set_ev1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "event 2 interrupt enable This bit enables the complex event 2 interrupt generation."]
        #[must_use]
        #[inline(always)]
        pub const fn ev2ie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "event 2 interrupt enable This bit enables the complex event 2 interrupt generation."]
        #[inline(always)]
        pub const fn set_ev2ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "event 3 interrupt enable This bit enables the complex event 3 interrupt generation."]
        #[must_use]
        #[inline(always)]
        pub const fn ev3ie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "event 3 interrupt enable This bit enables the complex event 3 interrupt generation."]
        #[inline(always)]
        pub const fn set_ev3ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "event 4 interrupt enable This bit enables the complex event 4 interrupt generation."]
        #[must_use]
        #[inline(always)]
        pub const fn ev4ie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "event 4 interrupt enable This bit enables the complex event 4 interrupt generation."]
        #[inline(always)]
        pub const fn set_ev4ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "watchdog alarm interrupt enable This bit enables the watchdog alarm interrupt generation."]
        #[must_use]
        #[inline(always)]
        pub const fn wdgaie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "watchdog alarm interrupt enable This bit enables the watchdog alarm interrupt generation."]
        #[inline(always)]
        pub const fn set_wdgaie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "watchdog pre-alarm interrupt enable This bit enables the watchdog pre-alarm interrupt generation."]
        #[must_use]
        #[inline(always)]
        pub const fn wdgpie(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "watchdog pre-alarm interrupt enable This bit enables the watchdog pre-alarm interrupt generation."]
        #[inline(always)]
        pub const fn set_wdgpie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
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
                .field("afcoie", &self.afcoie())
                .field("alcoie", &self.alcoie())
                .field("teie", &self.teie())
                .field("afcc1ie", &self.afcc1ie())
                .field("alcc1ie", &self.alcc1ie())
                .field("alcc2ie", &self.alcc2ie())
                .field("rfc1rie", &self.rfc1rie())
                .field("rfc2rie", &self.rfc2rie())
                .field("ev1ie", &self.ev1ie())
                .field("ev2ie", &self.ev2ie())
                .field("ev3ie", &self.ev3ie())
                .field("ev4ie", &self.ev4ie())
                .field("wdgaie", &self.wdgaie())
                .field("wdgpie", &self.wdgpie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ier {{ afcoie: {=bool:?}, alcoie: {=bool:?}, teie: {=bool:?}, afcc1ie: {=bool:?}, alcc1ie: {=bool:?}, alcc2ie: {=bool:?}, rfc1rie: {=bool:?}, rfc2rie: {=bool:?}, ev1ie: {=bool:?}, ev2ie: {=bool:?}, ev3ie: {=bool:?}, ev4ie: {=bool:?}, wdgaie: {=bool:?}, wdgpie: {=bool:?} }}",
                self.afcoie(),
                self.alcoie(),
                self.teie(),
                self.afcc1ie(),
                self.alcc1ie(),
                self.alcc2ie(),
                self.rfc1rie(),
                self.rfc2rie(),
                self.ev1ie(),
                self.ev2ie(),
                self.ev3ie(),
                self.ev4ie(),
                self.wdgaie(),
                self.wdgpie()
            )
        }
    }
    #[doc = "GFXTIM interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "absolute frame counter overflow flag This bit indicates an overflow occurred on the absolute frame counter."]
        #[must_use]
        #[inline(always)]
        pub const fn afcof(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "absolute frame counter overflow flag This bit indicates an overflow occurred on the absolute frame counter."]
        #[inline(always)]
        pub const fn set_afcof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "absolute line counter overflow flag This bit indicates an overflow occurred on the absolute line counter."]
        #[must_use]
        #[inline(always)]
        pub const fn alcof(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "absolute line counter overflow flag This bit indicates an overflow occurred on the absolute line counter."]
        #[inline(always)]
        pub const fn set_alcof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "tearing-effect flag This bit indicates a tearing effect event occurred."]
        #[must_use]
        #[inline(always)]
        pub const fn tef(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "tearing-effect flag This bit indicates a tearing effect event occurred."]
        #[inline(always)]
        pub const fn set_tef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "absolute frame counter compare 1 flag This bit indicates match on compare 1 of the absolute frame counter."]
        #[must_use]
        #[inline(always)]
        pub const fn afcc1f(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "absolute frame counter compare 1 flag This bit indicates match on compare 1 of the absolute frame counter."]
        #[inline(always)]
        pub const fn set_afcc1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "absolute line counter compare 1 flag This bit indicates match on compare 1 of the absolute line counter."]
        #[must_use]
        #[inline(always)]
        pub const fn alcc1f(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "absolute line counter compare 1 flag This bit indicates match on compare 1 of the absolute line counter."]
        #[inline(always)]
        pub const fn set_alcc1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "absolute line counter compare 2 flag This bit indicates match on compare 2 of the absolute line counter."]
        #[must_use]
        #[inline(always)]
        pub const fn alcc2f(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "absolute line counter compare 2 flag This bit indicates match on compare 2 of the absolute line counter."]
        #[inline(always)]
        pub const fn set_alcc2f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "relative frame counter 1 reload flag This bit indicates relative frame counter 1 has been reloaded."]
        #[must_use]
        #[inline(always)]
        pub const fn rfc1rf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "relative frame counter 1 reload flag This bit indicates relative frame counter 1 has been reloaded."]
        #[inline(always)]
        pub const fn set_rfc1rf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "relative frame counter 2 reload flag This bit indicates relative frame counter 2 has been reloaded."]
        #[must_use]
        #[inline(always)]
        pub const fn rfc2rf(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "relative frame counter 2 reload flag This bit indicates relative frame counter 2 has been reloaded."]
        #[inline(always)]
        pub const fn set_rfc2rf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "event 1 flag This bit indicates a complex event 1 occurred."]
        #[must_use]
        #[inline(always)]
        pub const fn ev1f(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "event 1 flag This bit indicates a complex event 1 occurred."]
        #[inline(always)]
        pub const fn set_ev1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "event 2 flag This bit indicates a complex event 2 occurred."]
        #[must_use]
        #[inline(always)]
        pub const fn ev2f(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "event 2 flag This bit indicates a complex event 2 occurred."]
        #[inline(always)]
        pub const fn set_ev2f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "event 3 flag This bit indicates a complex event 3 occurred."]
        #[must_use]
        #[inline(always)]
        pub const fn ev3f(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "event 3 flag This bit indicates a complex event 3 occurred."]
        #[inline(always)]
        pub const fn set_ev3f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "event 4 flag This bit indicates a complex event 4 occurred."]
        #[must_use]
        #[inline(always)]
        pub const fn ev4f(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "event 4 flag This bit indicates a complex event 4 occurred."]
        #[inline(always)]
        pub const fn set_ev4f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "watchdog alarm flag This bit indicates that a graphic watchdog alarm occurred."]
        #[must_use]
        #[inline(always)]
        pub const fn wdgaf(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "watchdog alarm flag This bit indicates that a graphic watchdog alarm occurred."]
        #[inline(always)]
        pub const fn set_wdgaf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "watchdog pre-alarm flag This bit indicates that a graphic watchdog pre-alarm occurred."]
        #[must_use]
        #[inline(always)]
        pub const fn wdgpf(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "watchdog pre-alarm flag This bit indicates that a graphic watchdog pre-alarm occurred."]
        #[inline(always)]
        pub const fn set_wdgpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Isr {
        #[inline(always)]
        fn default() -> Isr {
            Isr(0)
        }
    }
    impl core::fmt::Debug for Isr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Isr")
                .field("afcof", &self.afcof())
                .field("alcof", &self.alcof())
                .field("tef", &self.tef())
                .field("afcc1f", &self.afcc1f())
                .field("alcc1f", &self.alcc1f())
                .field("alcc2f", &self.alcc2f())
                .field("rfc1rf", &self.rfc1rf())
                .field("rfc2rf", &self.rfc2rf())
                .field("ev1f", &self.ev1f())
                .field("ev2f", &self.ev2f())
                .field("ev3f", &self.ev3f())
                .field("ev4f", &self.ev4f())
                .field("wdgaf", &self.wdgaf())
                .field("wdgpf", &self.wdgpf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Isr {{ afcof: {=bool:?}, alcof: {=bool:?}, tef: {=bool:?}, afcc1f: {=bool:?}, alcc1f: {=bool:?}, alcc2f: {=bool:?}, rfc1rf: {=bool:?}, rfc2rf: {=bool:?}, ev1f: {=bool:?}, ev2f: {=bool:?}, ev3f: {=bool:?}, ev4f: {=bool:?}, wdgaf: {=bool:?}, wdgpf: {=bool:?} }}",
                self.afcof(),
                self.alcof(),
                self.tef(),
                self.afcc1f(),
                self.alcc1f(),
                self.alcc2f(),
                self.rfc1rf(),
                self.rfc2rf(),
                self.ev1f(),
                self.ev2f(),
                self.ev3f(),
                self.ev4f(),
                self.wdgaf(),
                self.wdgpf()
            )
        }
    }
    #[doc = "GFXTIM line clock counter reload register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lccrr(pub u32);
    impl Lccrr {
        #[doc = "reload value Reload value of the line clock counter."]
        #[must_use]
        #[inline(always)]
        pub const fn reload(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x003f_ffff;
            val as u32
        }
        #[doc = "reload value Reload value of the line clock counter."]
        #[inline(always)]
        pub const fn set_reload(&mut self, val: u32) {
            self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
        }
    }
    impl Default for Lccrr {
        #[inline(always)]
        fn default() -> Lccrr {
            Lccrr(0)
        }
    }
    impl core::fmt::Debug for Lccrr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lccrr").field("reload", &self.reload()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lccrr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Lccrr {{ reload: {=u32:?} }}", self.reload())
        }
    }
    #[doc = "GFXTIM relative frame counter 1 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rfc1r(pub u32);
    impl Rfc1r {
        #[doc = "frame number Current value of the relative frame counter 1."]
        #[must_use]
        #[inline(always)]
        pub const fn frame(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "frame number Current value of the relative frame counter 1."]
        #[inline(always)]
        pub const fn set_frame(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Rfc1r {
        #[inline(always)]
        fn default() -> Rfc1r {
            Rfc1r(0)
        }
    }
    impl core::fmt::Debug for Rfc1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rfc1r").field("frame", &self.frame()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rfc1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rfc1r {{ frame: {=u16:?} }}", self.frame())
        }
    }
    #[doc = "GFXTIM relative frame counter 1 reload register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rfc1rr(pub u32);
    impl Rfc1rr {
        #[doc = "frame reload value Reload value for the relative frame counter 1."]
        #[must_use]
        #[inline(always)]
        pub const fn frame(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "frame reload value Reload value for the relative frame counter 1."]
        #[inline(always)]
        pub const fn set_frame(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Rfc1rr {
        #[inline(always)]
        fn default() -> Rfc1rr {
            Rfc1rr(0)
        }
    }
    impl core::fmt::Debug for Rfc1rr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rfc1rr").field("frame", &self.frame()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rfc1rr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rfc1rr {{ frame: {=u16:?} }}", self.frame())
        }
    }
    #[doc = "GFXTIM relative frame counter 2 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rfc2r(pub u32);
    impl Rfc2r {
        #[doc = "frame number Current value of the relative frame counter 2."]
        #[must_use]
        #[inline(always)]
        pub const fn frame(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "frame number Current value of the relative frame counter 2."]
        #[inline(always)]
        pub const fn set_frame(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Rfc2r {
        #[inline(always)]
        fn default() -> Rfc2r {
            Rfc2r(0)
        }
    }
    impl core::fmt::Debug for Rfc2r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rfc2r").field("frame", &self.frame()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rfc2r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rfc2r {{ frame: {=u16:?} }}", self.frame())
        }
    }
    #[doc = "GFXTIM relative frame counter 2 reload register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rfc2rr(pub u32);
    impl Rfc2rr {
        #[doc = "frame reload value Reload value for the relative frame counter 2."]
        #[must_use]
        #[inline(always)]
        pub const fn frame(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "frame reload value Reload value for the relative frame counter 2."]
        #[inline(always)]
        pub const fn set_frame(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Rfc2rr {
        #[inline(always)]
        fn default() -> Rfc2rr {
            Rfc2rr(0)
        }
    }
    impl core::fmt::Debug for Rfc2rr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rfc2rr").field("frame", &self.frame()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rfc2rr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rfc2rr {{ frame: {=u16:?} }}", self.frame())
        }
    }
    #[doc = "GFXTIM timers configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tcr(pub u32);
    impl Tcr {
        #[doc = "absolute frame counter enable This bit enables the absolute frame counter."]
        #[must_use]
        #[inline(always)]
        pub const fn afcen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "absolute frame counter enable This bit enables the absolute frame counter."]
        #[inline(always)]
        pub const fn set_afcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "force absolute frame counter reset This bit forces the reset of the absolute frame counter."]
        #[must_use]
        #[inline(always)]
        pub const fn fafcr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "force absolute frame counter reset This bit forces the reset of the absolute frame counter."]
        #[inline(always)]
        pub const fn set_fafcr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "absolute line counter enable This bit enables the absolute line counter."]
        #[must_use]
        #[inline(always)]
        pub const fn alcen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "absolute line counter enable This bit enables the absolute line counter."]
        #[inline(always)]
        pub const fn set_alcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "force absolute line counter reset This bit forces the reset of the absolute line counter."]
        #[must_use]
        #[inline(always)]
        pub const fn falcr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "force absolute line counter reset This bit forces the reset of the absolute line counter."]
        #[inline(always)]
        pub const fn set_falcr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "relative frame counter 1 enable This bit enables the relative frame counter 1."]
        #[must_use]
        #[inline(always)]
        pub const fn rfc1en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "relative frame counter 1 enable This bit enables the relative frame counter 1."]
        #[inline(always)]
        pub const fn set_rfc1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "relative frame counter 1 continuous mode This bit enables the continuous mode of the relative frame counter 1."]
        #[must_use]
        #[inline(always)]
        pub const fn rfc1cm(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "relative frame counter 1 continuous mode This bit enables the continuous mode of the relative frame counter 1."]
        #[inline(always)]
        pub const fn set_rfc1cm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "force relative frame counter 1 reload This bit forces the reload of the relative frame counter 1."]
        #[must_use]
        #[inline(always)]
        pub const fn frfc1r(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "force relative frame counter 1 reload This bit forces the reload of the relative frame counter 1."]
        #[inline(always)]
        pub const fn set_frfc1r(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "relative frame counter 2 enable This bit enables the relative frame counter 2."]
        #[must_use]
        #[inline(always)]
        pub const fn rfc2en(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "relative frame counter 2 enable This bit enables the relative frame counter 2."]
        #[inline(always)]
        pub const fn set_rfc2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "relative frame counter 2 continuous mode This bit enables the continuous mode of the relative frame counter 2."]
        #[must_use]
        #[inline(always)]
        pub const fn rfc2cm(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "relative frame counter 2 continuous mode This bit enables the continuous mode of the relative frame counter 2."]
        #[inline(always)]
        pub const fn set_rfc2cm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "force relative frame counter 2 reload This bit forces the reload of the relative frame counter 2."]
        #[must_use]
        #[inline(always)]
        pub const fn frfc2r(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "force relative frame counter 2 reload This bit forces the reload of the relative frame counter 2."]
        #[inline(always)]
        pub const fn set_frfc2r(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Tcr {
        #[inline(always)]
        fn default() -> Tcr {
            Tcr(0)
        }
    }
    impl core::fmt::Debug for Tcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tcr")
                .field("afcen", &self.afcen())
                .field("fafcr", &self.fafcr())
                .field("alcen", &self.alcen())
                .field("falcr", &self.falcr())
                .field("rfc1en", &self.rfc1en())
                .field("rfc1cm", &self.rfc1cm())
                .field("frfc1r", &self.frfc1r())
                .field("rfc2en", &self.rfc2en())
                .field("rfc2cm", &self.rfc2cm())
                .field("frfc2r", &self.frfc2r())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Tcr {{ afcen: {=bool:?}, fafcr: {=bool:?}, alcen: {=bool:?}, falcr: {=bool:?}, rfc1en: {=bool:?}, rfc1cm: {=bool:?}, frfc1r: {=bool:?}, rfc2en: {=bool:?}, rfc2cm: {=bool:?}, frfc2r: {=bool:?} }}",
                self.afcen(),
                self.fafcr(),
                self.alcen(),
                self.falcr(),
                self.rfc1en(),
                self.rfc1cm(),
                self.frfc1r(),
                self.rfc2en(),
                self.rfc2cm(),
                self.frfc2r()
            )
        }
    }
    #[doc = "GFXTIM timers disable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tdr(pub u32);
    impl Tdr {
        #[doc = "absolute frame counter disable This bit disables the absolute frame counter."]
        #[must_use]
        #[inline(always)]
        pub const fn afcdis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "absolute frame counter disable This bit disables the absolute frame counter."]
        #[inline(always)]
        pub const fn set_afcdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "absolute line counter disable This bit disables the absolute line counter."]
        #[must_use]
        #[inline(always)]
        pub const fn alcdis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "absolute line counter disable This bit disables the absolute line counter."]
        #[inline(always)]
        pub const fn set_alcdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "relative frame counter 1 disable This bit disables the relative frame counter 1."]
        #[must_use]
        #[inline(always)]
        pub const fn rfc1dis(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "relative frame counter 1 disable This bit disables the relative frame counter 1."]
        #[inline(always)]
        pub const fn set_rfc1dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "relative frame counter 2 disable This bit disables the relative frame counter 2."]
        #[must_use]
        #[inline(always)]
        pub const fn rfc2dis(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "relative frame counter 2 disable This bit disables the relative frame counter 2."]
        #[inline(always)]
        pub const fn set_rfc2dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Tdr {
        #[inline(always)]
        fn default() -> Tdr {
            Tdr(0)
        }
    }
    impl core::fmt::Debug for Tdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tdr")
                .field("afcdis", &self.afcdis())
                .field("alcdis", &self.alcdis())
                .field("rfc1dis", &self.rfc1dis())
                .field("rfc2dis", &self.rfc2dis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Tdr {{ afcdis: {=bool:?}, alcdis: {=bool:?}, rfc1dis: {=bool:?}, rfc2dis: {=bool:?} }}",
                self.afcdis(),
                self.alcdis(),
                self.rfc1dis(),
                self.rfc2dis()
            )
        }
    }
    #[doc = "GFXTIM timers status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tsr(pub u32);
    impl Tsr {
        #[doc = "absolute frame counter status This bit returns the status of the absolute frame counter."]
        #[must_use]
        #[inline(always)]
        pub const fn afcs(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "absolute frame counter status This bit returns the status of the absolute frame counter."]
        #[inline(always)]
        pub const fn set_afcs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "absolute line counter status This bit returns the status of the absolute line counter."]
        #[must_use]
        #[inline(always)]
        pub const fn alcs(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "absolute line counter status This bit returns the status of the absolute line counter."]
        #[inline(always)]
        pub const fn set_alcs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "relative frame counter 1 status This bit returns the status of the relative frame counter 1."]
        #[must_use]
        #[inline(always)]
        pub const fn rfc1s(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "relative frame counter 1 status This bit returns the status of the relative frame counter 1."]
        #[inline(always)]
        pub const fn set_rfc1s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "relative frame counter 2 status This bit returns the status of the relative frame counter 2."]
        #[must_use]
        #[inline(always)]
        pub const fn rfc2s(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "relative frame counter 2 status This bit returns the status of the relative frame counter 2."]
        #[inline(always)]
        pub const fn set_rfc2s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Tsr {
        #[inline(always)]
        fn default() -> Tsr {
            Tsr(0)
        }
    }
    impl core::fmt::Debug for Tsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tsr")
                .field("afcs", &self.afcs())
                .field("alcs", &self.alcs())
                .field("rfc1s", &self.rfc1s())
                .field("rfc2s", &self.rfc2s())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Tsr {{ afcs: {=bool:?}, alcs: {=bool:?}, rfc1s: {=bool:?}, rfc2s: {=bool:?} }}",
                self.afcs(),
                self.alcs(),
                self.rfc1s(),
                self.rfc2s()
            )
        }
    }
    #[doc = "GFXTIM watchdog counter register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wdgcr(pub u32);
    impl Wdgcr {
        #[doc = "value Current value of the watchdog counter."]
        #[must_use]
        #[inline(always)]
        pub const fn value(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "value Current value of the watchdog counter."]
        #[inline(always)]
        pub const fn set_value(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Wdgcr {
        #[inline(always)]
        fn default() -> Wdgcr {
            Wdgcr(0)
        }
    }
    impl core::fmt::Debug for Wdgcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wdgcr").field("value", &self.value()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wdgcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wdgcr {{ value: {=u16:?} }}", self.value())
        }
    }
    #[doc = "GFXTIM watchdog pre-alarm register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wdgpar(pub u32);
    impl Wdgpar {
        #[doc = "pre-alarm value Pre-alarm value of the watchdog counter."]
        #[must_use]
        #[inline(always)]
        pub const fn prealarm(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "pre-alarm value Pre-alarm value of the watchdog counter."]
        #[inline(always)]
        pub const fn set_prealarm(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Wdgpar {
        #[inline(always)]
        fn default() -> Wdgpar {
            Wdgpar(0)
        }
    }
    impl core::fmt::Debug for Wdgpar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wdgpar").field("prealarm", &self.prealarm()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wdgpar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wdgpar {{ prealarm: {=u16:?} }}", self.prealarm())
        }
    }
    #[doc = "GFXTIM watchdog reload register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wdgrr(pub u32);
    impl Wdgrr {
        #[doc = "reload value Reload value of the watchdog counter."]
        #[must_use]
        #[inline(always)]
        pub const fn reload(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "reload value Reload value of the watchdog counter."]
        #[inline(always)]
        pub const fn set_reload(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Wdgrr {
        #[inline(always)]
        fn default() -> Wdgrr {
            Wdgrr(0)
        }
    }
    impl core::fmt::Debug for Wdgrr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wdgrr").field("reload", &self.reload()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wdgrr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wdgrr {{ reload: {=u16:?} }}", self.reload())
        }
    }
    #[doc = "GFXTIM watchdog timer configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wdgtcr(pub u32);
    impl Wdgtcr {
        #[doc = "watchdog enable This bit enables the graphic watchdog."]
        #[must_use]
        #[inline(always)]
        pub const fn wdgen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "watchdog enable This bit enables the graphic watchdog."]
        #[inline(always)]
        pub const fn set_wdgen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "watchdog disable This bit disables the graphic watchdog."]
        #[must_use]
        #[inline(always)]
        pub const fn wdgdis(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "watchdog disable This bit disables the graphic watchdog."]
        #[inline(always)]
        pub const fn set_wdgdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "watchdog status This bit returns the status of the graphic watchdog."]
        #[must_use]
        #[inline(always)]
        pub const fn wdgs(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "watchdog status This bit returns the status of the graphic watchdog."]
        #[inline(always)]
        pub const fn set_wdgs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "watchdog hardware reload configuration This field configures the watchdog hardware reload."]
        #[must_use]
        #[inline(always)]
        pub const fn wdghrc(&self) -> super::vals::Wdghrc {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Wdghrc::from_bits(val as u8)
        }
        #[doc = "watchdog hardware reload configuration This field configures the watchdog hardware reload."]
        #[inline(always)]
        pub const fn set_wdghrc(&mut self, val: super::vals::Wdghrc) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "watchdog clock source This field selects the watchdog clock source. others: reserved."]
        #[must_use]
        #[inline(always)]
        pub const fn wdgcs(&self) -> super::vals::Wdgcs {
            let val = (self.0 >> 8usize) & 0x0f;
            super::vals::Wdgcs::from_bits(val as u8)
        }
        #[doc = "watchdog clock source This field selects the watchdog clock source. others: reserved."]
        #[inline(always)]
        pub const fn set_wdgcs(&mut self, val: super::vals::Wdgcs) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
        }
        #[doc = "force watchdog reload This bit forces the reload of the graphic watchdog."]
        #[must_use]
        #[inline(always)]
        pub const fn fwdgr(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "force watchdog reload This bit forces the reload of the graphic watchdog."]
        #[inline(always)]
        pub const fn set_fwdgr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Wdgtcr {
        #[inline(always)]
        fn default() -> Wdgtcr {
            Wdgtcr(0)
        }
    }
    impl core::fmt::Debug for Wdgtcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wdgtcr")
                .field("wdgen", &self.wdgen())
                .field("wdgdis", &self.wdgdis())
                .field("wdgs", &self.wdgs())
                .field("wdghrc", &self.wdghrc())
                .field("wdgcs", &self.wdgcs())
                .field("fwdgr", &self.fwdgr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wdgtcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Wdgtcr {{ wdgen: {=bool:?}, wdgdis: {=bool:?}, wdgs: {=bool:?}, wdghrc: {:?}, wdgcs: {:?}, fwdgr: {=bool:?} }}",
                self.wdgen(),
                self.wdgdis(),
                self.wdgs(),
                self.wdghrc(),
                self.wdgcs(),
                self.fwdgr()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fcccs {
        #[doc = "frame clock counter disabled."]
        Disabled = 0x0,
        #[doc = "line clock counter underflow."]
        LineClockCounterUnderflow = 0x01,
        #[doc = "HSYNC rising edge."]
        HsyncRising = 0x02,
        #[doc = "HSYNC falling edge."]
        HsyncFalling = 0x03,
        #[doc = "VSYNC rising edge."]
        VsyncRising = 0x04,
        #[doc = "VSYNC falling edge."]
        VsyncFalling = 0x05,
        #[doc = "TE rising edge."]
        TeRising = 0x06,
        #[doc = "TE falling edge."]
        TeFalling = 0x07,
    }
    impl Fcccs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fcccs {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fcccs {
        #[inline(always)]
        fn from(val: u8) -> Fcccs {
            Fcccs::from_bits(val)
        }
    }
    impl From<Fcccs> for u8 {
        #[inline(always)]
        fn from(val: Fcccs) -> u8 {
            Fcccs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fcchrs {
        #[doc = "no hardware reload."]
        NoHardwareReload = 0x0,
        #[doc = "line- -clock counter underflow."]
        LineClockCounterUnderflow = 0x01,
        #[doc = "HSYNC rising edge."]
        HsyncRising = 0x02,
        #[doc = "HSYNC falling edge."]
        HsyncFalling = 0x03,
        #[doc = "VSYNC rising edge."]
        VsyncRising = 0x04,
        #[doc = "VSYNC falling edge."]
        VsyncFalling = 0x05,
        #[doc = "TE rising edge."]
        TeRising = 0x06,
        #[doc = "TE falling edge."]
        TeFalling = 0x07,
    }
    impl Fcchrs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fcchrs {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fcchrs {
        #[inline(always)]
        fn from(val: u8) -> Fcchrs {
            Fcchrs::from_bits(val)
        }
    }
    impl From<Fcchrs> for u8 {
        #[inline(always)]
        fn from(val: Fcchrs) -> u8 {
            Fcchrs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fcs {
        #[doc = "line clock counter underflow."]
        LineClockCounterUnderflow = 0x0,
        #[doc = "frame clock counter underflow."]
        FrameClockCounterUnderflow = 0x01,
        #[doc = "HSYNC rising edge."]
        HsyncRising = 0x02,
        #[doc = "HSYNC falling edge."]
        HsyncFalling = 0x03,
        #[doc = "VSYNC rising edge."]
        VsyncRising = 0x04,
        #[doc = "VSYNC falling edge."]
        VsyncFalling = 0x05,
        #[doc = "TE rising edge."]
        TeRising = 0x06,
        #[doc = "TE falling edge."]
        TeFalling = 0x07,
    }
    impl Fcs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fcs {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fcs {
        #[inline(always)]
        fn from(val: u8) -> Fcs {
            Fcs::from_bits(val)
        }
    }
    impl From<Fcs> for u8 {
        #[inline(always)]
        fn from(val: Fcs) -> u8 {
            Fcs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fes1 {
        #[doc = "no frame event."]
        NoFrameEvent = 0x0,
        #[doc = "absolute frame counter overflow."]
        AbsoluteFrameCounterOverflow = 0x01,
        #[doc = "absolute frame counter compare."]
        AbsoluteFrameCounterCompare = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "relative frame counter 1 reload."]
        RelativeFrameCounter1reload = 0x04,
        #[doc = "relative frame counter 2 reload."]
        RelativeFrameCounter2reload = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Fes1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fes1 {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fes1 {
        #[inline(always)]
        fn from(val: u8) -> Fes1 {
            Fes1::from_bits(val)
        }
    }
    impl From<Fes1> for u8 {
        #[inline(always)]
        fn from(val: Fes1) -> u8 {
            Fes1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fes2 {
        #[doc = "no frame event."]
        NoFrameEvent = 0x0,
        #[doc = "absolute frame counter overflow."]
        AbsoluteFrameCounterOverflow = 0x01,
        #[doc = "absolute frame counter compare."]
        AbsoluteFrameCounterCompare = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "relative frame counter 1 reload."]
        RelativeFrameCounter1reload = 0x04,
        #[doc = "relative frame counter 2 reload."]
        RelativeFrameCounter2reload = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Fes2 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fes2 {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fes2 {
        #[inline(always)]
        fn from(val: u8) -> Fes2 {
            Fes2::from_bits(val)
        }
    }
    impl From<Fes2> for u8 {
        #[inline(always)]
        fn from(val: Fes2) -> u8 {
            Fes2::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fes3 {
        #[doc = "no frame event."]
        NoFrameEvent = 0x0,
        #[doc = "absolute frame counter overflow."]
        AbsoluteFrameCounterOverflow = 0x01,
        #[doc = "absolute frame counter compare."]
        AbsoluteFrameCounterCompare = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "relative frame counter 1 reload."]
        RelativeFrameCounter1reload = 0x04,
        #[doc = "relative frame counter 2 reload."]
        RelativeFrameCounter2reload = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Fes3 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fes3 {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fes3 {
        #[inline(always)]
        fn from(val: u8) -> Fes3 {
            Fes3::from_bits(val)
        }
    }
    impl From<Fes3> for u8 {
        #[inline(always)]
        fn from(val: Fes3) -> u8 {
            Fes3::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Fes4 {
        #[doc = "no frame event."]
        NoFrameEvent = 0x0,
        #[doc = "absolute frame counter overflow."]
        AbsoluteFrameCounterOverflow = 0x01,
        #[doc = "absolute frame counter compare."]
        AbsoluteFrameCounterCompare = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "relative frame counter 1 reload."]
        RelativeFrameCounter1reload = 0x04,
        #[doc = "relative frame counter 2 reload."]
        RelativeFrameCounter2reload = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Fes4 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fes4 {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fes4 {
        #[inline(always)]
        fn from(val: u8) -> Fes4 {
            Fes4::from_bits(val)
        }
    }
    impl From<Fes4> for u8 {
        #[inline(always)]
        fn from(val: Fes4) -> u8 {
            Fes4::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lcchrs {
        #[doc = "no hardware reload."]
        NoHardwareReload = 0x0,
        #[doc = "frame clock counter underflow."]
        FrameClockCounterUnderflow = 0x01,
        #[doc = "HSYNC rising edge."]
        HsyncRising = 0x02,
        #[doc = "HSYNC falling edge."]
        HsyncFalling = 0x03,
        #[doc = "VSYNC rising edge."]
        VsyncRising = 0x04,
        #[doc = "VSYNC falling edge."]
        VsyncFalling = 0x05,
        #[doc = "TE rising edge."]
        TeRising = 0x06,
        #[doc = "TE falling edge."]
        TeFalling = 0x07,
    }
    impl Lcchrs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lcchrs {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lcchrs {
        #[inline(always)]
        fn from(val: u8) -> Lcchrs {
            Lcchrs::from_bits(val)
        }
    }
    impl From<Lcchrs> for u8 {
        #[inline(always)]
        fn from(val: Lcchrs) -> u8 {
            Lcchrs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lcs {
        #[doc = "line clock counter underflow."]
        LineClockCounterUnderflow = 0x0,
        #[doc = "frame clock counter underflow."]
        FrameClockCounterUnderflow = 0x01,
        #[doc = "HSYNC rising edge."]
        HsyncRising = 0x02,
        #[doc = "HSYNC falling edge."]
        HsyncFalling = 0x03,
        #[doc = "VSYNC rising edge."]
        VsyncRising = 0x04,
        #[doc = "VSYNC falling edge."]
        VsyncFalling = 0x05,
        #[doc = "TE rising edge."]
        TeRising = 0x06,
        #[doc = "TE falling edge."]
        TeFalling = 0x07,
    }
    impl Lcs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lcs {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lcs {
        #[inline(always)]
        fn from(val: u8) -> Lcs {
            Lcs::from_bits(val)
        }
    }
    impl From<Lcs> for u8 {
        #[inline(always)]
        fn from(val: Lcs) -> u8 {
            Lcs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Les1 {
        #[doc = "no line event."]
        NoLineEvent = 0x0,
        #[doc = "absolute line counter overflow."]
        AbsoluteLineCounterOverflow = 0x01,
        #[doc = "tearing effect."]
        TearingEffect = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "absolute line counter 1 compare."]
        AbsoluteLineCounter1compare = 0x04,
        #[doc = "absolute line counter 2 compare."]
        AbsoluteLineCounter2compare = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Les1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Les1 {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Les1 {
        #[inline(always)]
        fn from(val: u8) -> Les1 {
            Les1::from_bits(val)
        }
    }
    impl From<Les1> for u8 {
        #[inline(always)]
        fn from(val: Les1) -> u8 {
            Les1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Les2 {
        #[doc = "no line event."]
        NoLineEvent = 0x0,
        #[doc = "absolute line counter overflow."]
        AbsoluteLineCounterOverflow = 0x01,
        #[doc = "tearing effect."]
        TearingEffect = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "absolute line counter 1 compare."]
        AbsoluteLineCounter1compare = 0x04,
        #[doc = "absolute line counter 2 compare."]
        AbsoluteLineCounter2compare = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Les2 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Les2 {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Les2 {
        #[inline(always)]
        fn from(val: u8) -> Les2 {
            Les2::from_bits(val)
        }
    }
    impl From<Les2> for u8 {
        #[inline(always)]
        fn from(val: Les2) -> u8 {
            Les2::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Les3 {
        #[doc = "no line event."]
        NoLineEvent = 0x0,
        #[doc = "absolute line counter overflow."]
        AbsoluteLineCounterOverflow = 0x01,
        #[doc = "tearing effect."]
        TearingEffect = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "absolute line counter 1 compare."]
        AbsoluteLineCounter1compare = 0x04,
        #[doc = "absolute line counter 2 compare."]
        AbsoluteLineCounter2compare = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Les3 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Les3 {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Les3 {
        #[inline(always)]
        fn from(val: u8) -> Les3 {
            Les3::from_bits(val)
        }
    }
    impl From<Les3> for u8 {
        #[inline(always)]
        fn from(val: Les3) -> u8 {
            Les3::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Les4 {
        #[doc = "no line event."]
        NoLineEvent = 0x0,
        #[doc = "absolute line counter overflow."]
        AbsoluteLineCounterOverflow = 0x01,
        #[doc = "tearing effect."]
        TearingEffect = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "absolute line counter 1 compare."]
        AbsoluteLineCounter1compare = 0x04,
        #[doc = "absolute line counter 2 compare."]
        AbsoluteLineCounter2compare = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Les4 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Les4 {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Les4 {
        #[inline(always)]
        fn from(val: u8) -> Les4 {
            Les4::from_bits(val)
        }
    }
    impl From<Les4> for u8 {
        #[inline(always)]
        fn from(val: Les4) -> u8 {
            Les4::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Syncs {
        #[doc = "gfxtim_hsync\\[0\\]
and gfxtim_vsync\\[0\\]
selected."]
        Hsync0vsync0 = 0x0,
        #[doc = "gfxtim_hsync\\[1\\]
and gfxtim_vsync\\[1\\]
selected."]
        Hsync1vsync1 = 0x01,
        #[doc = "gfxtim_hsync\\[2\\]
and gfxtim_vsync\\[2\\]
selected."]
        Hsync2vsync2 = 0x02,
        #[doc = "gfxtim_hsync\\[3\\]
and gfxtim_vsync\\[3\\]
selected."]
        Hsync3vsync3 = 0x03,
    }
    impl Syncs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Syncs {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Syncs {
        #[inline(always)]
        fn from(val: u8) -> Syncs {
            Syncs::from_bits(val)
        }
    }
    impl From<Syncs> for u8 {
        #[inline(always)]
        fn from(val: Syncs) -> u8 {
            Syncs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Tes {
        #[doc = "TE input pad selected."]
        TeInputPad = 0x0,
        #[doc = "gfxtim_ite selected."]
        InternalTe = 0x01,
        #[doc = "HSYNC input selected by SYNCS\\[1:0\\]."]
        HsyncFromSyncs = 0x02,
        #[doc = "VSYNC input selected by SYNCS\\[1:0\\]."]
        VsyncFromSyncs = 0x03,
    }
    impl Tes {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tes {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tes {
        #[inline(always)]
        fn from(val: u8) -> Tes {
            Tes::from_bits(val)
        }
    }
    impl From<Tes> for u8 {
        #[inline(always)]
        fn from(val: Tes) -> u8 {
            Tes::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wdgcs {
        #[doc = "line clock."]
        LineClock = 0x0,
        #[doc = "frame clock."]
        FrameClock = 0x01,
        #[doc = "HSYNC rising edge."]
        HsyncRising = 0x02,
        #[doc = "HSYNC falling edge."]
        HsyncFalling = 0x03,
        #[doc = "VSYNC rising edge."]
        VsyncRising = 0x04,
        #[doc = "VSYNC falling edge."]
        VsyncFalling = 0x05,
        #[doc = "TE rising edge."]
        TeRising = 0x06,
        #[doc = "TE falling edge."]
        TeFalling = 0x07,
        #[doc = "event 1."]
        Event1 = 0x08,
        #[doc = "event 2."]
        Event2 = 0x09,
        #[doc = "event 3."]
        Event3 = 0x0a,
        #[doc = "event 4."]
        Event4 = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl Wdgcs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wdgcs {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wdgcs {
        #[inline(always)]
        fn from(val: u8) -> Wdgcs {
            Wdgcs::from_bits(val)
        }
    }
    impl From<Wdgcs> for u8 {
        #[inline(always)]
        fn from(val: Wdgcs) -> u8 {
            Wdgcs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wdghrc {
        #[doc = "watchdog hardware reload disabled."]
        HardwareReloadDisabled = 0x0,
        #[doc = "watchdog reloaded a rising edge of gfxtim_wrld."]
        ReloadOnWrldRising = 0x01,
        #[doc = "watchdog reloaded a falling edge of gfxtim_wrld."]
        ReloadOnWrldFalling = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Wdghrc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wdghrc {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wdghrc {
        #[inline(always)]
        fn from(val: u8) -> Wdghrc {
            Wdghrc::from_bits(val)
        }
    }
    impl From<Wdghrc> for u8 {
        #[inline(always)]
        fn from(val: Wdghrc) -> u8 {
            Wdghrc::to_bits(val)
        }
    }
}
