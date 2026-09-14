#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch {
    ptr: *mut u8,
}
unsafe impl Send for Ch {}
unsafe impl Sync for Ch {}
impl Ch {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DFSDM channel y configuration register."]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::Cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "DFSDM channel y configuration register."]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "DFSDM channel y analog watchdog and short-circuit detector register."]
    #[inline(always)]
    pub const fn awscdr(self) -> crate::common::Reg<regs::Awscdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "DFSDM channel y watchdog filter data register."]
    #[inline(always)]
    pub const fn wdatr(self) -> crate::common::Reg<regs::Wdatr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "DFSDM channel y data input register."]
    #[inline(always)]
    pub const fn datinr(self) -> crate::common::Reg<regs::Datinr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
}
#[doc = "Basic timers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChDly {
    ptr: *mut u8,
}
unsafe impl Send for ChDly {}
unsafe impl Sync for ChDly {}
impl ChDly {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DFSDM channel y configuration register."]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::Cfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "DFSDM channel y configuration register."]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "DFSDM channel y analog watchdog and short-circuit detector register."]
    #[inline(always)]
    pub const fn awscdr(self) -> crate::common::Reg<regs::Awscdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "DFSDM channel y watchdog filter data register."]
    #[inline(always)]
    pub const fn wdatr(self) -> crate::common::Reg<regs::Wdatr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "DFSDM channel y data input register."]
    #[inline(always)]
    pub const fn datinr(self) -> crate::common::Reg<regs::Datinr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn dlyr(self) -> crate::common::Reg<regs::Dlyr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
}
#[doc = "Digital filter for sigma delta modulators."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfsdm2ch1fltTrg5 {
    ptr: *mut u8,
}
unsafe impl Send for Dfsdm2ch1fltTrg5 {}
unsafe impl Sync for Dfsdm2ch1fltTrg5 {}
impl Dfsdm2ch1fltTrg5 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR."]
    #[inline(always)]
    pub const fn ch(self, n: usize) -> Ch {
        assert!(n < 2usize);
        unsafe { Ch::from_ptr(self.ptr.wrapping_add(0x0usize + n * 32usize) as _) }
    }
    #[doc = "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR."]
    #[inline(always)]
    pub const fn flt(self, n: usize) -> Flt {
        assert!(n < 1usize);
        unsafe { Flt::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 128usize) as _) }
    }
}
#[doc = "Digital filter for sigma delta modulators."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfsdm4ch2fltDlyTrg3 {
    ptr: *mut u8,
}
unsafe impl Send for Dfsdm4ch2fltDlyTrg3 {}
unsafe impl Sync for Dfsdm4ch2fltDlyTrg3 {}
impl Dfsdm4ch2fltDlyTrg3 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR."]
    #[inline(always)]
    pub const fn ch(self, n: usize) -> ChDly {
        assert!(n < 4usize);
        unsafe { ChDly::from_ptr(self.ptr.wrapping_add(0x0usize + n * 32usize) as _) }
    }
    #[doc = "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR."]
    #[inline(always)]
    pub const fn flt(self, n: usize) -> Flt {
        assert!(n < 2usize);
        unsafe { Flt::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 128usize) as _) }
    }
}
#[doc = "Digital filter for sigma delta modulators."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfsdm4ch2fltDlyTrg5Adc {
    ptr: *mut u8,
}
unsafe impl Send for Dfsdm4ch2fltDlyTrg5Adc {}
unsafe impl Sync for Dfsdm4ch2fltDlyTrg5Adc {}
impl Dfsdm4ch2fltDlyTrg5Adc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR."]
    #[inline(always)]
    pub const fn ch(self, n: usize) -> ChDly {
        assert!(n < 4usize);
        unsafe { ChDly::from_ptr(self.ptr.wrapping_add(0x0usize + n * 32usize) as _) }
    }
    #[doc = "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR."]
    #[inline(always)]
    pub const fn flt(self, n: usize) -> Flt {
        assert!(n < 2usize);
        unsafe { Flt::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 128usize) as _) }
    }
}
#[doc = "Digital filter for sigma delta modulators."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfsdm4ch2fltDlyTrg5AdcHwid {
    ptr: *mut u8,
}
unsafe impl Send for Dfsdm4ch2fltDlyTrg5AdcHwid {}
unsafe impl Sync for Dfsdm4ch2fltDlyTrg5AdcHwid {}
impl Dfsdm4ch2fltDlyTrg5AdcHwid {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR."]
    #[inline(always)]
    pub const fn ch(self, n: usize) -> ChDly {
        assert!(n < 4usize);
        unsafe { ChDly::from_ptr(self.ptr.wrapping_add(0x0usize + n * 32usize) as _) }
    }
    #[doc = "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR."]
    #[inline(always)]
    pub const fn flt(self, n: usize) -> Flt {
        assert!(n < 2usize);
        unsafe { Flt::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 128usize) as _) }
    }
    #[doc = "Cluster HWID, containing version registers"]
    #[inline(always)]
    pub const fn hwid(self) -> Hwid {
        unsafe { Hwid::from_ptr(self.ptr.wrapping_add(0x07f0usize) as _) }
    }
}
#[doc = "Digital filter for sigma delta modulators."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfsdm4ch2fltTrg3 {
    ptr: *mut u8,
}
unsafe impl Send for Dfsdm4ch2fltTrg3 {}
unsafe impl Sync for Dfsdm4ch2fltTrg3 {}
impl Dfsdm4ch2fltTrg3 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR."]
    #[inline(always)]
    pub const fn ch(self, n: usize) -> Ch {
        assert!(n < 4usize);
        unsafe { Ch::from_ptr(self.ptr.wrapping_add(0x0usize + n * 32usize) as _) }
    }
    #[doc = "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR."]
    #[inline(always)]
    pub const fn flt(self, n: usize) -> Flt {
        assert!(n < 2usize);
        unsafe { Flt::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 128usize) as _) }
    }
}
#[doc = "Digital filter for sigma delta modulators."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfsdm4ch4fltDlyTrg5Adc {
    ptr: *mut u8,
}
unsafe impl Send for Dfsdm4ch4fltDlyTrg5Adc {}
unsafe impl Sync for Dfsdm4ch4fltDlyTrg5Adc {}
impl Dfsdm4ch4fltDlyTrg5Adc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR."]
    #[inline(always)]
    pub const fn ch(self, n: usize) -> ChDly {
        assert!(n < 4usize);
        unsafe { ChDly::from_ptr(self.ptr.wrapping_add(0x0usize + n * 32usize) as _) }
    }
    #[doc = "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR."]
    #[inline(always)]
    pub const fn flt(self, n: usize) -> Flt {
        assert!(n < 4usize);
        unsafe { Flt::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 128usize) as _) }
    }
}
#[doc = "Digital filter for sigma delta modulators."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfsdm8ch4fltDlyTrg3 {
    ptr: *mut u8,
}
unsafe impl Send for Dfsdm8ch4fltDlyTrg3 {}
unsafe impl Sync for Dfsdm8ch4fltDlyTrg3 {}
impl Dfsdm8ch4fltDlyTrg3 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR."]
    #[inline(always)]
    pub const fn ch(self, n: usize) -> ChDly {
        assert!(n < 8usize);
        unsafe { ChDly::from_ptr(self.ptr.wrapping_add(0x0usize + n * 32usize) as _) }
    }
    #[doc = "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR."]
    #[inline(always)]
    pub const fn flt(self, n: usize) -> Flt {
        assert!(n < 4usize);
        unsafe { Flt::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 128usize) as _) }
    }
}
#[doc = "Digital filter for sigma delta modulators."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfsdm8ch4fltDlyTrg5Adc {
    ptr: *mut u8,
}
unsafe impl Send for Dfsdm8ch4fltDlyTrg5Adc {}
unsafe impl Sync for Dfsdm8ch4fltDlyTrg5Adc {}
impl Dfsdm8ch4fltDlyTrg5Adc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR."]
    #[inline(always)]
    pub const fn ch(self, n: usize) -> ChDly {
        assert!(n < 8usize);
        unsafe { ChDly::from_ptr(self.ptr.wrapping_add(0x0usize + n * 32usize) as _) }
    }
    #[doc = "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR."]
    #[inline(always)]
    pub const fn flt(self, n: usize) -> Flt {
        assert!(n < 4usize);
        unsafe { Flt::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 128usize) as _) }
    }
}
#[doc = "Digital filter for sigma delta modulators."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfsdm8ch4fltTrg3 {
    ptr: *mut u8,
}
unsafe impl Send for Dfsdm8ch4fltTrg3 {}
unsafe impl Sync for Dfsdm8ch4fltTrg3 {}
impl Dfsdm8ch4fltTrg3 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR."]
    #[inline(always)]
    pub const fn ch(self, n: usize) -> Ch {
        assert!(n < 8usize);
        unsafe { Ch::from_ptr(self.ptr.wrapping_add(0x0usize + n * 32usize) as _) }
    }
    #[doc = "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR."]
    #[inline(always)]
    pub const fn flt(self, n: usize) -> Flt {
        assert!(n < 4usize);
        unsafe { Flt::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 128usize) as _) }
    }
}
#[doc = "Digital filter for sigma delta modulators."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfsdm8ch4fltTrg3Adc {
    ptr: *mut u8,
}
unsafe impl Send for Dfsdm8ch4fltTrg3Adc {}
unsafe impl Sync for Dfsdm8ch4fltTrg3Adc {}
impl Dfsdm8ch4fltTrg3Adc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR."]
    #[inline(always)]
    pub const fn ch(self, n: usize) -> Ch {
        assert!(n < 8usize);
        unsafe { Ch::from_ptr(self.ptr.wrapping_add(0x0usize + n * 32usize) as _) }
    }
    #[doc = "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR."]
    #[inline(always)]
    pub const fn flt(self, n: usize) -> Flt {
        assert!(n < 4usize);
        unsafe { Flt::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 128usize) as _) }
    }
}
#[doc = "Digital filter for sigma delta modulators."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfsdm8ch4fltTrg5 {
    ptr: *mut u8,
}
unsafe impl Send for Dfsdm8ch4fltTrg5 {}
unsafe impl Sync for Dfsdm8ch4fltTrg5 {}
impl Dfsdm8ch4fltTrg5 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR."]
    #[inline(always)]
    pub const fn ch(self, n: usize) -> Ch {
        assert!(n < 8usize);
        unsafe { Ch::from_ptr(self.ptr.wrapping_add(0x0usize + n * 32usize) as _) }
    }
    #[doc = "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR."]
    #[inline(always)]
    pub const fn flt(self, n: usize) -> Flt {
        assert!(n < 4usize);
        unsafe { Flt::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 128usize) as _) }
    }
}
#[doc = "Digital filter for sigma delta modulators."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfsdm8ch4fltTrg5Adc {
    ptr: *mut u8,
}
unsafe impl Send for Dfsdm8ch4fltTrg5Adc {}
unsafe impl Sync for Dfsdm8ch4fltTrg5Adc {}
impl Dfsdm8ch4fltTrg5Adc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR."]
    #[inline(always)]
    pub const fn ch(self, n: usize) -> Ch {
        assert!(n < 8usize);
        unsafe { Ch::from_ptr(self.ptr.wrapping_add(0x0usize + n * 32usize) as _) }
    }
    #[doc = "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR."]
    #[inline(always)]
    pub const fn flt(self, n: usize) -> Flt {
        assert!(n < 4usize);
        unsafe { Flt::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 128usize) as _) }
    }
}
#[doc = "Digital filter for sigma delta modulators."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfsdm8ch6fltDlyTrg5AdcHwid {
    ptr: *mut u8,
}
unsafe impl Send for Dfsdm8ch6fltDlyTrg5AdcHwid {}
unsafe impl Sync for Dfsdm8ch6fltDlyTrg5AdcHwid {}
impl Dfsdm8ch6fltDlyTrg5AdcHwid {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR."]
    #[inline(always)]
    pub const fn ch(self, n: usize) -> ChDly {
        assert!(n < 8usize);
        unsafe { ChDly::from_ptr(self.ptr.wrapping_add(0x0usize + n * 32usize) as _) }
    }
    #[doc = "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR."]
    #[inline(always)]
    pub const fn flt(self, n: usize) -> Flt {
        assert!(n < 6usize);
        unsafe { Flt::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 128usize) as _) }
    }
    #[doc = "Cluster HWID, containing version registers"]
    #[inline(always)]
    pub const fn hwid(self) -> Hwid {
        unsafe { Hwid::from_ptr(self.ptr.wrapping_add(0x07f0usize) as _) }
    }
}
#[doc = "Digital filter for sigma delta modulators."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfsdm8ch8fltDlyTrg5Adc {
    ptr: *mut u8,
}
unsafe impl Send for Dfsdm8ch8fltDlyTrg5Adc {}
unsafe impl Sync for Dfsdm8ch8fltDlyTrg5Adc {}
impl Dfsdm8ch8fltDlyTrg5Adc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR."]
    #[inline(always)]
    pub const fn ch(self, n: usize) -> ChDly {
        assert!(n < 8usize);
        unsafe { ChDly::from_ptr(self.ptr.wrapping_add(0x0usize + n * 32usize) as _) }
    }
    #[doc = "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR."]
    #[inline(always)]
    pub const fn flt(self, n: usize) -> Flt {
        assert!(n < 8usize);
        unsafe { Flt::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 128usize) as _) }
    }
}
#[doc = "Digital filter for sigma delta modulators."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DfsdmSuperset {
    ptr: *mut u8,
}
unsafe impl Send for DfsdmSuperset {}
unsafe impl Sync for DfsdmSuperset {}
impl DfsdmSuperset {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Cluster CH%s, containing CH?CFGR1, CH?CFGR2, CH?AWSCDR, CH?WDATR, CH?DATINR, CH?DLYR."]
    #[inline(always)]
    pub const fn ch(self, n: usize) -> ChDly {
        assert!(n < 8usize);
        unsafe { ChDly::from_ptr(self.ptr.wrapping_add(0x0usize + n * 32usize) as _) }
    }
    #[doc = "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR."]
    #[inline(always)]
    pub const fn flt(self, n: usize) -> Flt {
        assert!(n < 6usize);
        unsafe { Flt::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 128usize) as _) }
    }
    #[doc = "Cluster HWID, containing version registers"]
    #[inline(always)]
    pub const fn hwid(self) -> Hwid {
        unsafe { Hwid::from_ptr(self.ptr.wrapping_add(0x07f0usize) as _) }
    }
}
#[doc = "Cluster FLT%s, containing FLT?CR1, FLT?CR2, FLT?ISR, FLT?ICR, FLT?JCHGR, FLT?FCR, FLT?JDATAR, FLT?RDATAR, FLT?AWHTR, FLT?AWLTR, FLT?AWSR, FLT?AWCFR, FLT?EXMAX, FLT?EXMIN, FLT?CNVTIMR."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flt {
    ptr: *mut u8,
}
unsafe impl Send for Flt {}
unsafe impl Sync for Flt {}
impl Flt {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[inline(always)]
    pub const fn jchgr(self) -> crate::common::Reg<regs::Jchgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn fcr(self) -> crate::common::Reg<regs::Fcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[inline(always)]
    pub const fn jdatar(self) -> crate::common::Reg<regs::Jdatar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[inline(always)]
    pub const fn rdatar(self) -> crate::common::Reg<regs::Rdatar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[inline(always)]
    pub const fn awhtr(self) -> crate::common::Reg<regs::Awhtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[inline(always)]
    pub const fn awltr(self) -> crate::common::Reg<regs::Awltr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[inline(always)]
    pub const fn awsr(self) -> crate::common::Reg<regs::Awsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[inline(always)]
    pub const fn awcfr(self) -> crate::common::Reg<regs::Awcfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[inline(always)]
    pub const fn exmax(self) -> crate::common::Reg<regs::Exmax, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[inline(always)]
    pub const fn exmin(self) -> crate::common::Reg<regs::Exmin, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[inline(always)]
    pub const fn cnvtimr(self) -> crate::common::Reg<regs::Cnvtimr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hwid {
    ptr: *mut u8,
}
unsafe impl Send for Hwid {}
unsafe impl Sync for Hwid {}
impl Hwid {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "This register specifies the hardware configuration of DFSDM peripheral."]
    #[inline(always)]
    pub const fn hwcfgr(self) -> crate::common::Reg<regs::Hwcfgr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "This register specifies the version of DFSDM peripheral."]
    #[inline(always)]
    pub const fn verr(self) -> crate::common::Reg<regs::Verr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "This register specifies the identification of DFSDM peripheral."]
    #[inline(always)]
    pub const fn ipidr(self) -> crate::common::Reg<regs::Ipidr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "This register specifies the size allocated to DFSDM registers."]
    #[inline(always)]
    pub const fn sidr(self) -> crate::common::Reg<regs::Sidr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
}
pub mod regs {
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awcfr(pub u32);
    impl Awcfr {
        #[doc = "Clear the analog watchdog low threshold flag CLRAWLTF\\[y\\]=0: Writing '0' has no effect CLRAWLTF\\[y\\]=1: Writing '1' to position y clears the corresponding AWLTF\\[y\\]
bit in the DFSDM_FLTxAWSR register."]
        #[must_use]
        #[inline(always)]
        pub const fn clrawltf(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Clear the analog watchdog low threshold flag CLRAWLTF\\[y\\]=0: Writing '0' has no effect CLRAWLTF\\[y\\]=1: Writing '1' to position y clears the corresponding AWLTF\\[y\\]
bit in the DFSDM_FLTxAWSR register."]
        #[inline(always)]
        pub const fn set_clrawltf(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Clear the analog watchdog high threshold flag CLRAWHTF\\[y\\]=0: Writing '0' has no effect CLRAWHTF\\[y\\]=1: Writing '1' to position y clears the corresponding AWHTF\\[y\\]
bit in the DFSDM_FLTxAWSR register."]
        #[must_use]
        #[inline(always)]
        pub const fn clrawhtf(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Clear the analog watchdog high threshold flag CLRAWHTF\\[y\\]=0: Writing '0' has no effect CLRAWHTF\\[y\\]=1: Writing '1' to position y clears the corresponding AWHTF\\[y\\]
bit in the DFSDM_FLTxAWSR register."]
        #[inline(always)]
        pub const fn set_clrawhtf(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Awcfr {
        #[inline(always)]
        fn default() -> Awcfr {
            Awcfr(0)
        }
    }
    impl core::fmt::Debug for Awcfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Awcfr")
                .field("clrawltf", &self.clrawltf())
                .field("clrawhtf", &self.clrawhtf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Awcfr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Awcfr {{ clrawltf: {=u8:?}, clrawhtf: {=u8:?} }}",
                self.clrawltf(),
                self.clrawhtf()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awhtr(pub u32);
    impl Awhtr {
        #[doc = "Break signal assignment to analog watchdog high threshold event BKAWH\\[i\\]
= 0: Break i signal is not assigned to an analog watchdog high threshold event BKAWH\\[i\\]
= 1: Break i signal is assigned to an analog watchdog high threshold event."]
        #[must_use]
        #[inline(always)]
        pub const fn bkawh(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Break signal assignment to analog watchdog high threshold event BKAWH\\[i\\]
= 0: Break i signal is not assigned to an analog watchdog high threshold event BKAWH\\[i\\]
= 1: Break i signal is assigned to an analog watchdog high threshold event."]
        #[inline(always)]
        pub const fn set_bkawh(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Analog watchdog high threshold These bits are written by software to define the high threshold for the analog watchdog. Note: In case channel transceivers monitor (AWFSEL=1), the higher 16 bits (AWHT\\[23:8\\]) define the 16-bit threshold as compared with the analog watchdog filter output (because data coming from the analog watchdog filter are up to a 16-bit resolution). Bits AWHT\\[7:0\\]
are not taken into comparison in this case."]
        #[must_use]
        #[inline(always)]
        pub const fn awht(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Analog watchdog high threshold These bits are written by software to define the high threshold for the analog watchdog. Note: In case channel transceivers monitor (AWFSEL=1), the higher 16 bits (AWHT\\[23:8\\]) define the 16-bit threshold as compared with the analog watchdog filter output (because data coming from the analog watchdog filter are up to a 16-bit resolution). Bits AWHT\\[7:0\\]
are not taken into comparison in this case."]
        #[inline(always)]
        pub const fn set_awht(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for Awhtr {
        #[inline(always)]
        fn default() -> Awhtr {
            Awhtr(0)
        }
    }
    impl core::fmt::Debug for Awhtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Awhtr")
                .field("bkawh", &self.bkawh())
                .field("awht", &self.awht())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Awhtr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Awhtr {{ bkawh: {=u8:?}, awht: {=u32:?} }}",
                self.bkawh(),
                self.awht()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awltr(pub u32);
    impl Awltr {
        #[doc = "Break signal assignment to analog watchdog low threshold event BKAWL\\[i\\]
= 0: Break i signal is not assigned to an analog watchdog low threshold event BKAWL\\[i\\]
= 1: Break i signal is assigned to an analog watchdog low threshold event."]
        #[must_use]
        #[inline(always)]
        pub const fn bkawl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Break signal assignment to analog watchdog low threshold event BKAWL\\[i\\]
= 0: Break i signal is not assigned to an analog watchdog low threshold event BKAWL\\[i\\]
= 1: Break i signal is assigned to an analog watchdog low threshold event."]
        #[inline(always)]
        pub const fn set_bkawl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Analog watchdog low threshold These bits are written by software to define the low threshold for the analog watchdog. Note: In case channel transceivers monitor (AWFSEL=1), only the higher 16 bits (AWLT\\[23:8\\]) define the 16-bit threshold as compared with the analog watchdog filter output (because data coming from the analog watchdog filter are up to a 16-bit resolution). Bits AWLT\\[7:0\\]
are not taken into comparison in this case."]
        #[must_use]
        #[inline(always)]
        pub const fn awlt(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Analog watchdog low threshold These bits are written by software to define the low threshold for the analog watchdog. Note: In case channel transceivers monitor (AWFSEL=1), only the higher 16 bits (AWLT\\[23:8\\]) define the 16-bit threshold as compared with the analog watchdog filter output (because data coming from the analog watchdog filter are up to a 16-bit resolution). Bits AWLT\\[7:0\\]
are not taken into comparison in this case."]
        #[inline(always)]
        pub const fn set_awlt(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for Awltr {
        #[inline(always)]
        fn default() -> Awltr {
            Awltr(0)
        }
    }
    impl core::fmt::Debug for Awltr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Awltr")
                .field("bkawl", &self.bkawl())
                .field("awlt", &self.awlt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Awltr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Awltr {{ bkawl: {=u8:?}, awlt: {=u32:?} }}",
                self.bkawl(),
                self.awlt()
            )
        }
    }
    #[doc = "DFSDM channel y analog watchdog and short-circuit detector register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awscdr(pub u32);
    impl Awscdr {
        #[doc = "Short-circuit detector threshold for channel y These bits are written by software to define the threshold counter for the short-circuit detector. If this value is reached, then a short-circuit detector event occurs on a given channel."]
        #[must_use]
        #[inline(always)]
        pub const fn scdt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Short-circuit detector threshold for channel y These bits are written by software to define the threshold counter for the short-circuit detector. If this value is reached, then a short-circuit detector event occurs on a given channel."]
        #[inline(always)]
        pub const fn set_scdt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Break signal assignment for short-circuit detector on channel y BKSCD\\[i\\]
= 0: Break i signal not assigned to short-circuit detector on channel y BKSCD\\[i\\]
= 1: Break i signal assigned to short-circuit detector on channel y."]
        #[must_use]
        #[inline(always)]
        pub const fn bkscd(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Break signal assignment for short-circuit detector on channel y BKSCD\\[i\\]
= 0: Break i signal not assigned to short-circuit detector on channel y BKSCD\\[i\\]
= 1: Break i signal assigned to short-circuit detector on channel y."]
        #[inline(always)]
        pub const fn set_bkscd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "Analog watchdog filter oversampling ratio (decimation rate) on channel y also the decimation ratio of the analog data rate. This bit can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register). Note: If AWFOSR = 0 then the filter has no effect (filter bypass). 0 - 31: Defines the length of the Sinc type filter in the range 1 - 32 (AWFOSR + 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn awfosr(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Analog watchdog filter oversampling ratio (decimation rate) on channel y also the decimation ratio of the analog data rate. This bit can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register). Note: If AWFOSR = 0 then the filter has no effect (filter bypass). 0 - 31: Defines the length of the Sinc type filter in the range 1 - 32 (AWFOSR + 1)."]
        #[inline(always)]
        pub const fn set_awfosr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
        #[doc = "Analog watchdog Sinc filter order on channel y. This bit can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
        #[must_use]
        #[inline(always)]
        pub const fn awford(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "Analog watchdog Sinc filter order on channel y. This bit can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
        #[inline(always)]
        pub const fn set_awford(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
    }
    impl Default for Awscdr {
        #[inline(always)]
        fn default() -> Awscdr {
            Awscdr(0)
        }
    }
    impl core::fmt::Debug for Awscdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Awscdr")
                .field("scdt", &self.scdt())
                .field("bkscd", &self.bkscd())
                .field("awfosr", &self.awfosr())
                .field("awford", &self.awford())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Awscdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Awscdr {{ scdt: {=u8:?}, bkscd: {=u8:?}, awfosr: {=u8:?}, awford: {=u8:?} }}",
                self.scdt(),
                self.bkscd(),
                self.awfosr(),
                self.awford()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awsr(pub u32);
    impl Awsr {
        #[doc = "Analog watchdog low threshold flag AWLTF\\[y\\]=1 indicates a low threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWLTF\\[y\\]
bit in the DFSDM_FLTxAWCFR register."]
        #[must_use]
        #[inline(always)]
        pub const fn awltf(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Analog watchdog low threshold flag AWLTF\\[y\\]=1 indicates a low threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWLTF\\[y\\]
bit in the DFSDM_FLTxAWCFR register."]
        #[inline(always)]
        pub const fn set_awltf(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Analog watchdog high threshold flag AWHTF\\[y\\]=1 indicates a high threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWHTF\\[y\\]
bit in the DFSDM_FLTxAWCFR register."]
        #[must_use]
        #[inline(always)]
        pub const fn awhtf(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Analog watchdog high threshold flag AWHTF\\[y\\]=1 indicates a high threshold error on channel y. It is set by hardware. It can be cleared by software using the corresponding CLRAWHTF\\[y\\]
bit in the DFSDM_FLTxAWCFR register."]
        #[inline(always)]
        pub const fn set_awhtf(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Awsr {
        #[inline(always)]
        fn default() -> Awsr {
            Awsr(0)
        }
    }
    impl core::fmt::Debug for Awsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Awsr")
                .field("awltf", &self.awltf())
                .field("awhtf", &self.awhtf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Awsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Awsr {{ awltf: {=u8:?}, awhtf: {=u8:?} }}",
                self.awltf(),
                self.awhtf()
            )
        }
    }
    #[doc = "DFSDM channel y configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr1(pub u32);
    impl Cfgr1 {
        #[doc = "Serial interface type for channel y This value can only be modified when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
        #[must_use]
        #[inline(always)]
        pub const fn sitp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Serial interface type for channel y This value can only be modified when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
        #[inline(always)]
        pub const fn set_sitp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "SPI clock select for channel y 2: clock coming from internal CKOUT - sampling point on each second CKOUT falling edge. For connection to external sigma-delta modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input rising edge). 3: clock coming from internal CKOUT output - sampling point on each second CKOUT rising edge. For connection to external sigma-delta modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input falling edge). This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
        #[must_use]
        #[inline(always)]
        pub const fn spicksel(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "SPI clock select for channel y 2: clock coming from internal CKOUT - sampling point on each second CKOUT falling edge. For connection to external sigma-delta modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input rising edge). 3: clock coming from internal CKOUT output - sampling point on each second CKOUT rising edge. For connection to external sigma-delta modulator which divides its clock input (from CKOUT) by 2 to generate its output serial communication clock (and this output clock change is active on each clock input falling edge). This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
        #[inline(always)]
        pub const fn set_spicksel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Short-circuit detector enable on channel y."]
        #[must_use]
        #[inline(always)]
        pub const fn scden(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Short-circuit detector enable on channel y."]
        #[inline(always)]
        pub const fn set_scden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Clock absence detector enable on channel y."]
        #[must_use]
        #[inline(always)]
        pub const fn ckaben(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Clock absence detector enable on channel y."]
        #[inline(always)]
        pub const fn set_ckaben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Channel y enable If channel y is enabled, then serial data receiving is started according to the given channel setting."]
        #[must_use]
        #[inline(always)]
        pub const fn chen(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Channel y enable If channel y is enabled, then serial data receiving is started according to the given channel setting."]
        #[inline(always)]
        pub const fn set_chen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Channel inputs selection This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
        #[must_use]
        #[inline(always)]
        pub const fn chinsel(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Channel inputs selection This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
        #[inline(always)]
        pub const fn set_chinsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Input data multiplexer for channel y 2: Data to channel y are taken from internal DFSDM_CHyDATINR register by direct CPU/DMA write. There can be written one or two 16-bit data samples according DATPACK\\[1:0\\]
bit field setting. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
        #[must_use]
        #[inline(always)]
        pub const fn datmpx(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Input data multiplexer for channel y 2: Data to channel y are taken from internal DFSDM_CHyDATINR register by direct CPU/DMA write. There can be written one or two 16-bit data samples according DATPACK\\[1:0\\]
bit field setting. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
        #[inline(always)]
        pub const fn set_datmpx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Data packing mode in DFSDM_CHyDATINR register. first sample in INDAT0\\[15:0\\]
(assigned to channel y) second sample INDAT1\\[15:0\\]
(assigned to channel y) To empty DFSDM_CHyDATINR register, two samples must be read by the digital filter from channel y (INDAT0\\[15:0\\]
part is read as first sample and then INDAT1\\[15:0\\]
part is read as next sample). 2: Dual: input data in DFSDM_CHyDATINR register are stored as two samples: first sample INDAT0\\[15:0\\]
(assigned to channel y) second sample INDAT1\\[15:0\\]
(assigned to channel y+1) To empty DFSDM_CHyDATINR register first sample must be read by the digital filter from channel y and second sample must be read by another digital filter from channel y+1. Dual mode is available only on even channel numbers (y = 0, 2, 4, 6), for odd channel numbers (y = 1, 3, 5, 7) DFSDM_CHyDATINR is write protected. If an even channel is set to dual mode then the following odd channel must be set into standard mode (DATPACK\\[1:0\\]=0) for correct cooperation with even channel. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
        #[must_use]
        #[inline(always)]
        pub const fn datpack(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Data packing mode in DFSDM_CHyDATINR register. first sample in INDAT0\\[15:0\\]
(assigned to channel y) second sample INDAT1\\[15:0\\]
(assigned to channel y) To empty DFSDM_CHyDATINR register, two samples must be read by the digital filter from channel y (INDAT0\\[15:0\\]
part is read as first sample and then INDAT1\\[15:0\\]
part is read as next sample). 2: Dual: input data in DFSDM_CHyDATINR register are stored as two samples: first sample INDAT0\\[15:0\\]
(assigned to channel y) second sample INDAT1\\[15:0\\]
(assigned to channel y+1) To empty DFSDM_CHyDATINR register first sample must be read by the digital filter from channel y and second sample must be read by another digital filter from channel y+1. Dual mode is available only on even channel numbers (y = 0, 2, 4, 6), for odd channel numbers (y = 1, 3, 5, 7) DFSDM_CHyDATINR is write protected. If an even channel is set to dual mode then the following odd channel must be set into standard mode (DATPACK\\[1:0\\]=0) for correct cooperation with even channel. 3: Reserved This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register)."]
        #[inline(always)]
        pub const fn set_datpack(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
        #[doc = "Output serial clock divider 256 (Divider = CKOUTDIV+1). CKOUTDIV also defines the threshold for a clock absence detection. This value can only be modified when DFSDMEN=0 (in DFSDM_CH0CFGR1 register). If DFSDMEN=0 (in DFSDM_CH0CFGR1 register) then CKOUT signal is set to low state (setting is performed one DFSDM clock cycle after DFSDMEN=0). Note: CKOUTDIV is present only in DFSDM_CH0CFGR1 register (channel y=0) 1- 255: Defines the division of system clock for the serial clock output for CKOUT signal in range 2 - 256."]
        #[must_use]
        #[inline(always)]
        pub const fn ckoutdiv(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Output serial clock divider 256 (Divider = CKOUTDIV+1). CKOUTDIV also defines the threshold for a clock absence detection. This value can only be modified when DFSDMEN=0 (in DFSDM_CH0CFGR1 register). If DFSDMEN=0 (in DFSDM_CH0CFGR1 register) then CKOUT signal is set to low state (setting is performed one DFSDM clock cycle after DFSDMEN=0). Note: CKOUTDIV is present only in DFSDM_CH0CFGR1 register (channel y=0) 1- 255: Defines the division of system clock for the serial clock output for CKOUT signal in range 2 - 256."]
        #[inline(always)]
        pub const fn set_ckoutdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Output serial clock source selection This value can be modified only when DFSDMEN=0 (in DFSDM_CH0CFGR1 register). Note: CKOUTSRC is present only in DFSDM_CH0CFGR1 register (channel y=0)."]
        #[must_use]
        #[inline(always)]
        pub const fn ckoutsrc(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Output serial clock source selection This value can be modified only when DFSDMEN=0 (in DFSDM_CH0CFGR1 register). Note: CKOUTSRC is present only in DFSDM_CH0CFGR1 register (channel y=0)."]
        #[inline(always)]
        pub const fn set_ckoutsrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Global enable for DFSDM interface If DFSDM interface is enabled, then it is started to operate according to enabled y channels and enabled x filters settings (CHEN bit in DFSDM_CHyCFGR1 and DFEN bit in DFSDM_FLTxCR1). Data cleared by setting DFSDMEN=0: all registers DFSDM_FLTxISR are set to reset state (x = 0..7) all registers DFSDM_FLTxAWSR are set to reset state (x = 0..7) Note: DFSDMEN is present only in DFSDM_CH0CFGR1 register (channel y=0)."]
        #[must_use]
        #[inline(always)]
        pub const fn dfsdmen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Global enable for DFSDM interface If DFSDM interface is enabled, then it is started to operate according to enabled y channels and enabled x filters settings (CHEN bit in DFSDM_CHyCFGR1 and DFEN bit in DFSDM_FLTxCR1). Data cleared by setting DFSDMEN=0: all registers DFSDM_FLTxISR are set to reset state (x = 0..7) all registers DFSDM_FLTxAWSR are set to reset state (x = 0..7) Note: DFSDMEN is present only in DFSDM_CH0CFGR1 register (channel y=0)."]
        #[inline(always)]
        pub const fn set_dfsdmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cfgr1 {
        #[inline(always)]
        fn default() -> Cfgr1 {
            Cfgr1(0)
        }
    }
    impl core::fmt::Debug for Cfgr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr1")
                .field("sitp", &self.sitp())
                .field("spicksel", &self.spicksel())
                .field("scden", &self.scden())
                .field("ckaben", &self.ckaben())
                .field("chen", &self.chen())
                .field("chinsel", &self.chinsel())
                .field("datmpx", &self.datmpx())
                .field("datpack", &self.datpack())
                .field("ckoutdiv", &self.ckoutdiv())
                .field("ckoutsrc", &self.ckoutsrc())
                .field("dfsdmen", &self.dfsdmen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfgr1 {{ sitp: {=u8:?}, spicksel: {=u8:?}, scden: {=bool:?}, ckaben: {=bool:?}, chen: {=bool:?}, chinsel: {=bool:?}, datmpx: {=u8:?}, datpack: {=u8:?}, ckoutdiv: {=u8:?}, ckoutsrc: {=bool:?}, dfsdmen: {=bool:?} }}",
                self.sitp(),
                self.spicksel(),
                self.scden(),
                self.ckaben(),
                self.chen(),
                self.chinsel(),
                self.datmpx(),
                self.datpack(),
                self.ckoutdiv(),
                self.ckoutsrc(),
                self.dfsdmen()
            )
        }
    }
    #[doc = "DFSDM channel y configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
        #[doc = "Data right bit-shift for channel y will be performed to have final results. Bit-shift is performed before offset correction. The data shift is rounding the result to nearest integer value. The sign of shifted result is maintained (to have valid 24-bit signed format of result data). This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register). 0-31: Defines the shift of the data result coming from the integrator - how many bit shifts to the right."]
        #[must_use]
        #[inline(always)]
        pub const fn dtrbs(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x1f;
            val as u8
        }
        #[doc = "Data right bit-shift for channel y will be performed to have final results. Bit-shift is performed before offset correction. The data shift is rounding the result to nearest integer value. The sign of shifted result is maintained (to have valid 24-bit signed format of result data). This value can be modified only when CHEN=0 (in DFSDM_CHyCFGR1 register). 0-31: Defines the shift of the data result coming from the integrator - how many bit shifts to the right."]
        #[inline(always)]
        pub const fn set_dtrbs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 3usize)) | (((val as u32) & 0x1f) << 3usize);
        }
        #[doc = "24-bit calibration offset for channel y For channel y, OFFSET is applied to the results of each conversion from this channel. This value is set by software."]
        #[must_use]
        #[inline(always)]
        pub const fn offset(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "24-bit calibration offset for channel y For channel y, OFFSET is applied to the results of each conversion from this channel. This value is set by software."]
        #[inline(always)]
        pub const fn set_offset(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for Cfgr2 {
        #[inline(always)]
        fn default() -> Cfgr2 {
            Cfgr2(0)
        }
    }
    impl core::fmt::Debug for Cfgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr2")
                .field("dtrbs", &self.dtrbs())
                .field("offset", &self.offset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfgr2 {{ dtrbs: {=u8:?}, offset: {=u32:?} }}",
                self.dtrbs(),
                self.offset()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cnvtimr(pub u32);
    impl Cnvtimr {
        #[doc = "28-bit timer counting conversion time t = CNVCNT\\[27:0\\]
/ fDFSDMCLK The timer has an input clock from DFSDM clock (system clock fDFSDMCLK). Conversion time measurement is started on each conversion start and stopped when conversion finishes (interval between first and last serial sample). Only in case of filter bypass (FOSR\\[9:0\\]
= 0) is the conversion time measurement stopped and CNVCNT\\[27:0\\]
= 0. The counted time is: if FAST=0 (or first conversion in continuous mode if FAST=1): t = \\[FOSR * (IOSR-1 + FORD) + FORD\\]
/ fCKIN ..... for Sincx filters t = \\[FOSR * (IOSR-1 + 4) + 2\\]
/ fCKIN ..... for FastSinc filter if FAST=1 in continuous mode (except first conversion): t = \\[FOSR * IOSR\\]
/ fCKIN in case if FOSR = FOSR\\[9:0\\]+1 = 1 (filter bypassed, active only integrator): CNVCNT = 0 (counting is stopped, conversion time: t = IOSR / fCKIN) where: fCKIN is the channel input clock frequency (on given channel CKINy pin) or input data rate in case of parallel data input (from internal ADC or from CPU/DMA write) Note: When conversion is interrupted (e.g. by disable/enable selected channel) the timer counts also this interruption time."]
        #[must_use]
        #[inline(always)]
        pub const fn cnvcnt(&self) -> u32 {
            let val = (self.0 >> 4usize) & 0x0fff_ffff;
            val as u32
        }
        #[doc = "28-bit timer counting conversion time t = CNVCNT\\[27:0\\]
/ fDFSDMCLK The timer has an input clock from DFSDM clock (system clock fDFSDMCLK). Conversion time measurement is started on each conversion start and stopped when conversion finishes (interval between first and last serial sample). Only in case of filter bypass (FOSR\\[9:0\\]
= 0) is the conversion time measurement stopped and CNVCNT\\[27:0\\]
= 0. The counted time is: if FAST=0 (or first conversion in continuous mode if FAST=1): t = \\[FOSR * (IOSR-1 + FORD) + FORD\\]
/ fCKIN ..... for Sincx filters t = \\[FOSR * (IOSR-1 + 4) + 2\\]
/ fCKIN ..... for FastSinc filter if FAST=1 in continuous mode (except first conversion): t = \\[FOSR * IOSR\\]
/ fCKIN in case if FOSR = FOSR\\[9:0\\]+1 = 1 (filter bypassed, active only integrator): CNVCNT = 0 (counting is stopped, conversion time: t = IOSR / fCKIN) where: fCKIN is the channel input clock frequency (on given channel CKINy pin) or input data rate in case of parallel data input (from internal ADC or from CPU/DMA write) Note: When conversion is interrupted (e.g. by disable/enable selected channel) the timer counts also this interruption time."]
        #[inline(always)]
        pub const fn set_cnvcnt(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
        }
    }
    impl Default for Cnvtimr {
        #[inline(always)]
        fn default() -> Cnvtimr {
            Cnvtimr(0)
        }
    }
    impl core::fmt::Debug for Cnvtimr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cnvtimr").field("cnvcnt", &self.cnvcnt()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cnvtimr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cnvtimr {{ cnvcnt: {=u32:?} }}", self.cnvcnt())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "DFSDM_FLTx enable Data which are cleared by setting DFEN=0: register DFSDM_FLTxISR is set to the reset state register DFSDM_FLTxAWSR is set to the reset state."]
        #[must_use]
        #[inline(always)]
        pub const fn dfen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DFSDM_FLTx enable Data which are cleared by setting DFEN=0: register DFSDM_FLTxISR is set to the reset state register DFSDM_FLTxAWSR is set to the reset state."]
        #[inline(always)]
        pub const fn set_dfen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Start a conversion of the injected group of channels This bit is always read as '0'."]
        #[must_use]
        #[inline(always)]
        pub const fn jswstart(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Start a conversion of the injected group of channels This bit is always read as '0'."]
        #[inline(always)]
        pub const fn set_jswstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Launch an injected conversion synchronously with the DFSDM_FLT0 JSWSTART trigger This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1)."]
        #[must_use]
        #[inline(always)]
        pub const fn jsync(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Launch an injected conversion synchronously with the DFSDM_FLT0 JSWSTART trigger This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1)."]
        #[inline(always)]
        pub const fn set_jsync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Scanning conversion mode for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). Writing JCHG if JSCAN=0 resets the channel selection to the lowest selected channel."]
        #[must_use]
        #[inline(always)]
        pub const fn jscan(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Scanning conversion mode for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). Writing JCHG if JSCAN=0 resets the channel selection to the lowest selected channel."]
        #[inline(always)]
        pub const fn set_jscan(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "DMA channel enabled to read data for the injected channel group This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1)."]
        #[must_use]
        #[inline(always)]
        pub const fn jdmaen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "DMA channel enabled to read data for the injected channel group This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1)."]
        #[inline(always)]
        pub const fn set_jdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Trigger signal selection for launching injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). Note: synchronous trigger has latency up to one fDFSDMCLK clock cycle (with deterministic jitter), asynchronous trigger has latency 2-3 fDFSDMCLK clock cycles (with jitter up to 1 cycle). DFSDM_FLTx 0x00 dfsdm_jtrg0 0x01 dfsdm_jtrg1 ... 0x1E dfsdm_jtrg30 0x1F dfsdm_jtrg31 Refer to . 0x0-0x1F: Trigger inputs selected by the following table (internal or external trigger)."]
        #[must_use]
        #[inline(always)]
        pub const fn jextsel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Trigger signal selection for launching injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). Note: synchronous trigger has latency up to one fDFSDMCLK clock cycle (with deterministic jitter), asynchronous trigger has latency 2-3 fDFSDMCLK clock cycles (with jitter up to 1 cycle). DFSDM_FLTx 0x00 dfsdm_jtrg0 0x01 dfsdm_jtrg1 ... 0x1E dfsdm_jtrg30 0x1F dfsdm_jtrg31 Refer to . 0x0-0x1F: Trigger inputs selected by the following table (internal or external trigger)."]
        #[inline(always)]
        pub const fn set_jextsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
        #[doc = "Trigger enable and trigger edge selection for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1)."]
        #[must_use]
        #[inline(always)]
        pub const fn jexten(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[doc = "Trigger enable and trigger edge selection for injected conversions This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1)."]
        #[inline(always)]
        pub const fn set_jexten(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
        #[doc = "Software start of a conversion on the regular channel This bit is always read as '0'."]
        #[must_use]
        #[inline(always)]
        pub const fn rswstart(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Software start of a conversion on the regular channel This bit is always read as '0'."]
        #[inline(always)]
        pub const fn set_rswstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Continuous mode selection for regular conversions Writing '0' to this bit while a continuous regular conversion is already in progress stops the continuous mode immediately."]
        #[must_use]
        #[inline(always)]
        pub const fn rcont(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Continuous mode selection for regular conversions Writing '0' to this bit while a continuous regular conversion is already in progress stops the continuous mode immediately."]
        #[inline(always)]
        pub const fn set_rcont(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Launch regular conversion synchronously with DFSDM_FLT0 This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1)."]
        #[must_use]
        #[inline(always)]
        pub const fn rsync(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Launch regular conversion synchronously with DFSDM_FLT0 This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1)."]
        #[inline(always)]
        pub const fn set_rsync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "DMA channel enabled to read data for the regular conversion This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1)."]
        #[must_use]
        #[inline(always)]
        pub const fn rdmaen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "DMA channel enabled to read data for the regular conversion This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1)."]
        #[inline(always)]
        pub const fn set_rdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Regular channel selection ... 7: Channel 7 is selected as the regular channel Writing these bits when RCIP=1 takes effect when the next regular conversion begins. This is especially useful in continuous mode (when RCONT=1). It also affects regular conversions which are pending (due to ongoing injected conversion)."]
        #[must_use]
        #[inline(always)]
        pub const fn rch(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x07;
            val as u8
        }
        #[doc = "Regular channel selection ... 7: Channel 7 is selected as the regular channel Writing these bits when RCIP=1 takes effect when the next regular conversion begins. This is especially useful in continuous mode (when RCONT=1). It also affects regular conversions which are pending (due to ongoing injected conversion)."]
        #[inline(always)]
        pub const fn set_rch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 24usize)) | (((val as u32) & 0x07) << 24usize);
        }
        #[doc = "Fast conversion mode selection for regular conversions When converting a regular conversion in continuous mode, having enabled the fast mode causes each conversion (except the first) to execute faster than in standard mode. This bit has no effect on conversions which are not continuous. This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). if FAST=0 (or first conversion in continuous mode if FAST=1): t = \\[FOSR * (IOSR-1 + FORD) + FORD\\]
/ fCKIN ..... for Sincx filters t = \\[FOSR * (IOSR-1 + 4) + 2\\]
/ fCKIN ..... for FastSinc filter if FAST=1 in continuous mode (except first conversion): t = \\[FOSR * IOSR\\]
/ fCKIN in case if FOSR = FOSR\\[9:0\\]+1 = 1 (filter bypassed, active only integrator): t = IOSR / fCKIN (... but CNVCNT=0) where: fCKIN is the channel input clock frequency (on given channel CKINy pin) or input data rate in case of parallel data input."]
        #[must_use]
        #[inline(always)]
        pub const fn fast(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Fast conversion mode selection for regular conversions When converting a regular conversion in continuous mode, having enabled the fast mode causes each conversion (except the first) to execute faster than in standard mode. This bit has no effect on conversions which are not continuous. This bit can be modified only when DFEN=0 (DFSDM_FLTxCR1). if FAST=0 (or first conversion in continuous mode if FAST=1): t = \\[FOSR * (IOSR-1 + FORD) + FORD\\]
/ fCKIN ..... for Sincx filters t = \\[FOSR * (IOSR-1 + 4) + 2\\]
/ fCKIN ..... for FastSinc filter if FAST=1 in continuous mode (except first conversion): t = \\[FOSR * IOSR\\]
/ fCKIN in case if FOSR = FOSR\\[9:0\\]+1 = 1 (filter bypassed, active only integrator): t = IOSR / fCKIN (... but CNVCNT=0) where: fCKIN is the channel input clock frequency (on given channel CKINy pin) or input data rate in case of parallel data input."]
        #[inline(always)]
        pub const fn set_fast(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Analog watchdog fast mode select."]
        #[must_use]
        #[inline(always)]
        pub const fn awfsel(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog fast mode select."]
        #[inline(always)]
        pub const fn set_awfsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
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
                .field("dfen", &self.dfen())
                .field("jswstart", &self.jswstart())
                .field("jsync", &self.jsync())
                .field("jscan", &self.jscan())
                .field("jdmaen", &self.jdmaen())
                .field("jextsel", &self.jextsel())
                .field("jexten", &self.jexten())
                .field("rswstart", &self.rswstart())
                .field("rcont", &self.rcont())
                .field("rsync", &self.rsync())
                .field("rdmaen", &self.rdmaen())
                .field("rch", &self.rch())
                .field("fast", &self.fast())
                .field("awfsel", &self.awfsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr1 {{ dfen: {=bool:?}, jswstart: {=bool:?}, jsync: {=bool:?}, jscan: {=bool:?}, jdmaen: {=bool:?}, jextsel: {=u8:?}, jexten: {=u8:?}, rswstart: {=bool:?}, rcont: {=bool:?}, rsync: {=bool:?}, rdmaen: {=bool:?}, rch: {=u8:?}, fast: {=bool:?}, awfsel: {=bool:?} }}",
                self.dfen(),
                self.jswstart(),
                self.jsync(),
                self.jscan(),
                self.jdmaen(),
                self.jextsel(),
                self.jexten(),
                self.rswstart(),
                self.rcont(),
                self.rsync(),
                self.rdmaen(),
                self.rch(),
                self.fast(),
                self.awfsel()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "Injected end of conversion interrupt enable Please see the explanation of JEOCF in DFSDM_FLTxISR."]
        #[must_use]
        #[inline(always)]
        pub const fn jeocie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Injected end of conversion interrupt enable Please see the explanation of JEOCF in DFSDM_FLTxISR."]
        #[inline(always)]
        pub const fn set_jeocie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Regular end of conversion interrupt enable Please see the explanation of REOCF in DFSDM_FLTxISR."]
        #[must_use]
        #[inline(always)]
        pub const fn reocie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Regular end of conversion interrupt enable Please see the explanation of REOCF in DFSDM_FLTxISR."]
        #[inline(always)]
        pub const fn set_reocie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Injected data overrun interrupt enable Please see the explanation of JOVRF in DFSDM_FLTxISR."]
        #[must_use]
        #[inline(always)]
        pub const fn jovrie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Injected data overrun interrupt enable Please see the explanation of JOVRF in DFSDM_FLTxISR."]
        #[inline(always)]
        pub const fn set_jovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Regular data overrun interrupt enable Please see the explanation of ROVRF in DFSDM_FLTxISR."]
        #[must_use]
        #[inline(always)]
        pub const fn rovrie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Regular data overrun interrupt enable Please see the explanation of ROVRF in DFSDM_FLTxISR."]
        #[inline(always)]
        pub const fn set_rovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Analog watchdog interrupt enable Please see the explanation of AWDF in DFSDM_FLTxISR."]
        #[must_use]
        #[inline(always)]
        pub const fn awdie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog interrupt enable Please see the explanation of AWDF in DFSDM_FLTxISR."]
        #[inline(always)]
        pub const fn set_awdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Short-circuit detector interrupt enable Please see the explanation of SCDF\\[7:0\\]
in DFSDM_FLTxISR. Note: SCDIE is present only in DFSDM_FLT0CR2 register (filter x=0)."]
        #[must_use]
        #[inline(always)]
        pub const fn scdie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Short-circuit detector interrupt enable Please see the explanation of SCDF\\[7:0\\]
in DFSDM_FLTxISR. Note: SCDIE is present only in DFSDM_FLT0CR2 register (filter x=0)."]
        #[inline(always)]
        pub const fn set_scdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Clock absence interrupt enable Please see the explanation of CKABF\\[7:0\\]
in DFSDM_FLTxISR. Note: CKABIE is present only in DFSDM_FLT0CR2 register (filter x=0)."]
        #[must_use]
        #[inline(always)]
        pub const fn ckabie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Clock absence interrupt enable Please see the explanation of CKABF\\[7:0\\]
in DFSDM_FLTxISR. Note: CKABIE is present only in DFSDM_FLT0CR2 register (filter x=0)."]
        #[inline(always)]
        pub const fn set_ckabie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Extremes detector channel selection These bits select the input channels to be taken by the Extremes detector. EXCH\\[y\\]
= 0: Extremes detector does not accept data from channel y EXCH\\[y\\]
= 1: Extremes detector accepts data from channel y."]
        #[must_use]
        #[inline(always)]
        pub const fn exch(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Extremes detector channel selection These bits select the input channels to be taken by the Extremes detector. EXCH\\[y\\]
= 0: Extremes detector does not accept data from channel y EXCH\\[y\\]
= 1: Extremes detector accepts data from channel y."]
        #[inline(always)]
        pub const fn set_exch(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Analog watchdog channel selection These bits select the input channel to be guarded continuously by the analog watchdog. AWDCH\\[y\\]
= 0: Analog watchdog is disabled on channel y AWDCH\\[y\\]
= 1: Analog watchdog is enabled on channel y."]
        #[must_use]
        #[inline(always)]
        pub const fn awdch(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Analog watchdog channel selection These bits select the input channel to be guarded continuously by the analog watchdog. AWDCH\\[y\\]
= 0: Analog watchdog is disabled on channel y AWDCH\\[y\\]
= 1: Analog watchdog is enabled on channel y."]
        #[inline(always)]
        pub const fn set_awdch(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
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
                .field("jeocie", &self.jeocie())
                .field("reocie", &self.reocie())
                .field("jovrie", &self.jovrie())
                .field("rovrie", &self.rovrie())
                .field("awdie", &self.awdie())
                .field("scdie", &self.scdie())
                .field("ckabie", &self.ckabie())
                .field("exch", &self.exch())
                .field("awdch", &self.awdch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr2 {{ jeocie: {=bool:?}, reocie: {=bool:?}, jovrie: {=bool:?}, rovrie: {=bool:?}, awdie: {=bool:?}, scdie: {=bool:?}, ckabie: {=bool:?}, exch: {=u8:?}, awdch: {=u8:?} }}",
                self.jeocie(),
                self.reocie(),
                self.jovrie(),
                self.rovrie(),
                self.awdie(),
                self.scdie(),
                self.ckabie(),
                self.exch(),
                self.awdch()
            )
        }
    }
    #[doc = "DFSDM channel y data input register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Datinr(pub u32);
    impl Datinr {
        #[doc = "Input data for channel y Input parallel channel data to be processed by the digital filter if DATMPX\\[1:0\\]=1 or DATMPX\\[1:0\\]=2. Data can be written by CPU/DMA (if DATMPX\\[1:0\\]=2) or directly by internal ADC (if DATMPX\\[1:0\\]=1). If DATPACK\\[1:0\\]=0 (standard mode) Channel y data sample is stored into INDAT0\\[15:0\\]. If DATPACK\\[1:0\\]=1 (interleaved mode) First channel y data sample is stored into INDAT0\\[15:0\\]. Second channel y data sample is stored into INDAT1\\[15:0\\]. Both samples are read sequentially by DFSDM_FLTx filter as two channel y data samples. If DATPACK\\[1:0\\]=2 (dual mode). For even y channels: Channel y data sample is stored into INDAT0\\[15:0\\]. For odd y channels: INDAT0\\[15:0\\]
is write protected. See for more details. INDAT0\\[15:0\\]
is in the16-bit signed format."]
        #[must_use]
        #[inline(always)]
        pub const fn indat0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Input data for channel y Input parallel channel data to be processed by the digital filter if DATMPX\\[1:0\\]=1 or DATMPX\\[1:0\\]=2. Data can be written by CPU/DMA (if DATMPX\\[1:0\\]=2) or directly by internal ADC (if DATMPX\\[1:0\\]=1). If DATPACK\\[1:0\\]=0 (standard mode) Channel y data sample is stored into INDAT0\\[15:0\\]. If DATPACK\\[1:0\\]=1 (interleaved mode) First channel y data sample is stored into INDAT0\\[15:0\\]. Second channel y data sample is stored into INDAT1\\[15:0\\]. Both samples are read sequentially by DFSDM_FLTx filter as two channel y data samples. If DATPACK\\[1:0\\]=2 (dual mode). For even y channels: Channel y data sample is stored into INDAT0\\[15:0\\]. For odd y channels: INDAT0\\[15:0\\]
is write protected. See for more details. INDAT0\\[15:0\\]
is in the16-bit signed format."]
        #[inline(always)]
        pub const fn set_indat0(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Input data for channel y or channel y+1 Input parallel channel data to be processed by the digital filter if DATMPX\\[1:0\\]=1 or DATMPX\\[1:0\\]=2. Data can be written by CPU/DMA (if DATMPX\\[1:0\\]=2) or directly by internal ADC (if DATMPX\\[1:0\\]=1). If DATPACK\\[1:0\\]=0 (standard mode) INDAT0\\[15:0\\]
is write protected (not used for input sample). If DATPACK\\[1:0\\]=1 (interleaved mode) Second channel y data sample is stored into INDAT1\\[15:0\\]. First channel y data sample is stored into INDAT0\\[15:0\\]. Both samples are read sequentially by DFSDM_FLTx filter as two channel y data samples. If DATPACK\\[1:0\\]=2 (dual mode). For even y channels: sample in INDAT1\\[15:0\\]
is automatically copied into INDAT0\\[15:0\\]
of channel (y+1). For odd y channels: INDAT1\\[15:0\\]
is write protected. See for more details. INDAT0\\[15:1\\]
is in the16-bit signed format."]
        #[must_use]
        #[inline(always)]
        pub const fn indat1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Input data for channel y or channel y+1 Input parallel channel data to be processed by the digital filter if DATMPX\\[1:0\\]=1 or DATMPX\\[1:0\\]=2. Data can be written by CPU/DMA (if DATMPX\\[1:0\\]=2) or directly by internal ADC (if DATMPX\\[1:0\\]=1). If DATPACK\\[1:0\\]=0 (standard mode) INDAT0\\[15:0\\]
is write protected (not used for input sample). If DATPACK\\[1:0\\]=1 (interleaved mode) Second channel y data sample is stored into INDAT1\\[15:0\\]. First channel y data sample is stored into INDAT0\\[15:0\\]. Both samples are read sequentially by DFSDM_FLTx filter as two channel y data samples. If DATPACK\\[1:0\\]=2 (dual mode). For even y channels: sample in INDAT1\\[15:0\\]
is automatically copied into INDAT0\\[15:0\\]
of channel (y+1). For odd y channels: INDAT1\\[15:0\\]
is write protected. See for more details. INDAT0\\[15:1\\]
is in the16-bit signed format."]
        #[inline(always)]
        pub const fn set_indat1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Datinr {
        #[inline(always)]
        fn default() -> Datinr {
            Datinr(0)
        }
    }
    impl core::fmt::Debug for Datinr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Datinr")
                .field("indat0", &self.indat0())
                .field("indat1", &self.indat1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Datinr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Datinr {{ indat0: {=u16:?}, indat1: {=u16:?} }}",
                self.indat0(),
                self.indat1()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dlyr(pub u32);
    impl Dlyr {
        #[doc = "Pulses to skip for input data skipping function immediately after writing to this field. Reading of PLSSKP\\[5:0\\]
returns current value of pulses which will be skipped. If PLSSKP\\[5:0\\]=0 then all required data samples were already skipped. Note: User can update PLSSKP\\[5:0\\]
also when PLSSKP\\[5:0\\]
is not zero. 0-63: Defines the number of serial input samples that will be skipped. Skipping is applied."]
        #[must_use]
        #[inline(always)]
        pub const fn plsskp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "Pulses to skip for input data skipping function immediately after writing to this field. Reading of PLSSKP\\[5:0\\]
returns current value of pulses which will be skipped. If PLSSKP\\[5:0\\]=0 then all required data samples were already skipped. Note: User can update PLSSKP\\[5:0\\]
also when PLSSKP\\[5:0\\]
is not zero. 0-63: Defines the number of serial input samples that will be skipped. Skipping is applied."]
        #[inline(always)]
        pub const fn set_plsskp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for Dlyr {
        #[inline(always)]
        fn default() -> Dlyr {
            Dlyr(0)
        }
    }
    impl core::fmt::Debug for Dlyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dlyr").field("plsskp", &self.plsskp()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dlyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dlyr {{ plsskp: {=u8:?} }}", self.plsskp())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exmax(pub u32);
    impl Exmax {
        #[doc = "Extremes detector maximum data channel. These bits contains information about the channel on which the data is stored into EXMAX\\[23:0\\]. Bits are cleared by reading of this register."]
        #[must_use]
        #[inline(always)]
        pub const fn exmaxch(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Extremes detector maximum data channel. These bits contains information about the channel on which the data is stored into EXMAX\\[23:0\\]. Bits are cleared by reading of this register."]
        #[inline(always)]
        pub const fn set_exmaxch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Extremes detector maximum value These bits are set by hardware and indicate the highest value converted by DFSDM_FLTx. EXMAX\\[23:0\\]
bits are reset to value (0x800000) by reading of this register."]
        #[must_use]
        #[inline(always)]
        pub const fn exmax(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Extremes detector maximum value These bits are set by hardware and indicate the highest value converted by DFSDM_FLTx. EXMAX\\[23:0\\]
bits are reset to value (0x800000) by reading of this register."]
        #[inline(always)]
        pub const fn set_exmax(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for Exmax {
        #[inline(always)]
        fn default() -> Exmax {
            Exmax(0)
        }
    }
    impl core::fmt::Debug for Exmax {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Exmax")
                .field("exmaxch", &self.exmaxch())
                .field("exmax", &self.exmax())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Exmax {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Exmax {{ exmaxch: {=u8:?}, exmax: {=u32:?} }}",
                self.exmaxch(),
                self.exmax()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exmin(pub u32);
    impl Exmin {
        #[doc = "Extremes detector minimum data channel These bits contain information about the channel on which the data is stored into EXMIN\\[23:0\\]. Bits are cleared by reading of this register."]
        #[must_use]
        #[inline(always)]
        pub const fn exminch(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Extremes detector minimum data channel These bits contain information about the channel on which the data is stored into EXMIN\\[23:0\\]. Bits are cleared by reading of this register."]
        #[inline(always)]
        pub const fn set_exminch(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Extremes detector minimum value These bits are set by hardware and indicate the lowest value converted by DFSDM_FLTx. EXMIN\\[23:0\\]
bits are reset to value (0x7FFFFF) by reading of this register."]
        #[must_use]
        #[inline(always)]
        pub const fn exmin(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Extremes detector minimum value These bits are set by hardware and indicate the lowest value converted by DFSDM_FLTx. EXMIN\\[23:0\\]
bits are reset to value (0x7FFFFF) by reading of this register."]
        #[inline(always)]
        pub const fn set_exmin(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for Exmin {
        #[inline(always)]
        fn default() -> Exmin {
            Exmin(0)
        }
    }
    impl core::fmt::Debug for Exmin {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Exmin")
                .field("exminch", &self.exminch())
                .field("exmin", &self.exmin())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Exmin {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Exmin {{ exminch: {=u8:?}, exmin: {=u32:?} }}",
                self.exminch(),
                self.exmin()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fcr(pub u32);
    impl Fcr {
        #[doc = "Integrator oversampling ratio (averaging length) from Sinc filter will be summed into one output data sample from the integrator. The output data rate from the integrator will be decreased by this number (additional data decimation ratio). This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1) Note: If IOSR = 0, then the Integrator has no effect (Integrator bypass). 0- 255: The length of the Integrator in the range 1 - 256 (IOSR + 1). Defines how many samples."]
        #[must_use]
        #[inline(always)]
        pub const fn iosr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Integrator oversampling ratio (averaging length) from Sinc filter will be summed into one output data sample from the integrator. The output data rate from the integrator will be decreased by this number (additional data decimation ratio). This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1) Note: If IOSR = 0, then the Integrator has no effect (Integrator bypass). 0- 255: The length of the Integrator in the range 1 - 256 (IOSR + 1). Defines how many samples."]
        #[inline(always)]
        pub const fn set_iosr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Sinc filter oversampling ratio (decimation rate) number is also the decimation ratio of the output data rate from filter. This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1) Note: If FOSR = 0, then the filter has no effect (filter bypass). 0 - 1023: Defines the length of the Sinc type filter in the range 1 - 1024 (FOSR = FOSR\\[9:0\\]
+1). This."]
        #[must_use]
        #[inline(always)]
        pub const fn fosr(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x03ff;
            val as u16
        }
        #[doc = "Sinc filter oversampling ratio (decimation rate) number is also the decimation ratio of the output data rate from filter. This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1) Note: If FOSR = 0, then the filter has no effect (filter bypass). 0 - 1023: Defines the length of the Sinc type filter in the range 1 - 1024 (FOSR = FOSR\\[9:0\\]
+1). This."]
        #[inline(always)]
        pub const fn set_fosr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
        }
        #[doc = "Sinc filter order 2: Sinc2 filter type 3: Sinc3 filter type 4: Sinc4 filter type 5: Sinc5 filter type 6-7: Reserved Sincx filter type transfer function: FastSinc filter type transfer function: This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1)."]
        #[must_use]
        #[inline(always)]
        pub const fn ford(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x07;
            val as u8
        }
        #[doc = "Sinc filter order 2: Sinc2 filter type 3: Sinc3 filter type 4: Sinc4 filter type 5: Sinc5 filter type 6-7: Reserved Sincx filter type transfer function: FastSinc filter type transfer function: This bit can only be modified when DFEN=0 (DFSDM_FLTxCR1)."]
        #[inline(always)]
        pub const fn set_ford(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
        }
    }
    impl Default for Fcr {
        #[inline(always)]
        fn default() -> Fcr {
            Fcr(0)
        }
    }
    impl core::fmt::Debug for Fcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fcr")
                .field("iosr", &self.iosr())
                .field("fosr", &self.fosr())
                .field("ford", &self.ford())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Fcr {{ iosr: {=u8:?}, fosr: {=u16:?}, ford: {=u8:?} }}",
                self.iosr(),
                self.fosr(),
                self.ford()
            )
        }
    }
    #[doc = "This register specifies the hardware configuration of DFSDM peripheral."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hwcfgr(pub u32);
    impl Hwcfgr {
        #[doc = "Number of implemented transceivers. Defines how many transceivers (input channels) are implemented in DFSDM peripheral."]
        #[must_use]
        #[inline(always)]
        pub const fn nbt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Number of implemented transceivers. Defines how many transceivers (input channels) are implemented in DFSDM peripheral."]
        #[inline(always)]
        pub const fn set_nbt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Number of implemented filters. Defines how many filters are implemented in DFSDM peripheral."]
        #[must_use]
        #[inline(always)]
        pub const fn nbf(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Number of implemented filters. Defines how many filters are implemented in DFSDM peripheral."]
        #[inline(always)]
        pub const fn set_nbf(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Hwcfgr {
        #[inline(always)]
        fn default() -> Hwcfgr {
            Hwcfgr(0)
        }
    }
    impl core::fmt::Debug for Hwcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hwcfgr")
                .field("nbt", &self.nbt())
                .field("nbf", &self.nbf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hwcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Hwcfgr {{ nbt: {=u8:?}, nbf: {=u8:?} }}", self.nbt(), self.nbf())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "Clear the injected conversion overrun flag."]
        #[must_use]
        #[inline(always)]
        pub const fn clrjovrf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Clear the injected conversion overrun flag."]
        #[inline(always)]
        pub const fn set_clrjovrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Clear the regular conversion overrun flag."]
        #[must_use]
        #[inline(always)]
        pub const fn clrrovrf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Clear the regular conversion overrun flag."]
        #[inline(always)]
        pub const fn set_clrrovrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Clear the clock absence flag CLRCKABF\\[y\\]=0: Writing '0' has no effect CLRCKABF\\[y\\]=1: Writing '1' to position y clears the corresponding CKABF\\[y\\]
bit in the DFSDM_FLTxISR register. When the transceiver is not yet synchronized, the clock absence flag is set and cannot be cleared by CLRCKABF\\[y\\]. Note: CLRCKABF\\[7:0\\]
is present only in DFSDM_FLT0ICR register (filter x=0)."]
        #[must_use]
        #[inline(always)]
        pub const fn clrckabf(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Clear the clock absence flag CLRCKABF\\[y\\]=0: Writing '0' has no effect CLRCKABF\\[y\\]=1: Writing '1' to position y clears the corresponding CKABF\\[y\\]
bit in the DFSDM_FLTxISR register. When the transceiver is not yet synchronized, the clock absence flag is set and cannot be cleared by CLRCKABF\\[y\\]. Note: CLRCKABF\\[7:0\\]
is present only in DFSDM_FLT0ICR register (filter x=0)."]
        #[inline(always)]
        pub const fn set_clrckabf(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Clear the short-circuit detector flag CLRSCDF\\[y\\]=0: Writing '0' has no effect CLRSCDF\\[y\\]=1: Writing '1' to position y clears the corresponding SCDF\\[y\\]
bit in the DFSDM_FLTxISR register Note: CLRSCDF\\[7:0\\]
is present only in DFSDM_FLT0ICR register (filter x=0)."]
        #[must_use]
        #[inline(always)]
        pub const fn clrscdf(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Clear the short-circuit detector flag CLRSCDF\\[y\\]=0: Writing '0' has no effect CLRSCDF\\[y\\]=1: Writing '1' to position y clears the corresponding SCDF\\[y\\]
bit in the DFSDM_FLTxISR register Note: CLRSCDF\\[7:0\\]
is present only in DFSDM_FLT0ICR register (filter x=0)."]
        #[inline(always)]
        pub const fn set_clrscdf(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
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
                .field("clrjovrf", &self.clrjovrf())
                .field("clrrovrf", &self.clrrovrf())
                .field("clrckabf", &self.clrckabf())
                .field("clrscdf", &self.clrscdf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icr {{ clrjovrf: {=bool:?}, clrrovrf: {=bool:?}, clrckabf: {=u8:?}, clrscdf: {=u8:?} }}",
                self.clrjovrf(),
                self.clrrovrf(),
                self.clrckabf(),
                self.clrscdf()
            )
        }
    }
    #[doc = "This register specifies the identification of DFSDM peripheral."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipidr(pub u32);
    impl Ipidr {
        #[doc = "Peripheral identifier. Bits \\[31:0\\]: these bits returns the DFSDM identifier ID\\[31:0\\]
= 0x0011 0031"]
        #[must_use]
        #[inline(always)]
        pub const fn id(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Peripheral identifier. Bits \\[31:0\\]: these bits returns the DFSDM identifier ID\\[31:0\\]
= 0x0011 0031"]
        #[inline(always)]
        pub const fn set_id(&mut self, val: u32) {
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
            f.debug_struct("Ipidr").field("id", &self.id()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ipidr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ipidr {{ id: {=u32:?} }}", self.id())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "End of injected conversion flag This bit is set by hardware. It is cleared when the software or DMA reads DFSDM_FLTxJDATAR."]
        #[must_use]
        #[inline(always)]
        pub const fn jeocf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "End of injected conversion flag This bit is set by hardware. It is cleared when the software or DMA reads DFSDM_FLTxJDATAR."]
        #[inline(always)]
        pub const fn set_jeocf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "End of regular conversion flag This bit is set by hardware. It is cleared when the software or DMA reads DFSDM_FLTxRDATAR."]
        #[must_use]
        #[inline(always)]
        pub const fn reocf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "End of regular conversion flag This bit is set by hardware. It is cleared when the software or DMA reads DFSDM_FLTxRDATAR."]
        #[inline(always)]
        pub const fn set_reocf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Injected conversion overrun flag This bit is set by hardware. It can be cleared by software using the CLRJOVRF bit in the DFSDM_FLTxICR register."]
        #[must_use]
        #[inline(always)]
        pub const fn jovrf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Injected conversion overrun flag This bit is set by hardware. It can be cleared by software using the CLRJOVRF bit in the DFSDM_FLTxICR register."]
        #[inline(always)]
        pub const fn set_jovrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Regular conversion overrun flag This bit is set by hardware. It can be cleared by software using the CLRROVRF bit in the DFSDM_FLTxICR register."]
        #[must_use]
        #[inline(always)]
        pub const fn rovrf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Regular conversion overrun flag This bit is set by hardware. It can be cleared by software using the CLRROVRF bit in the DFSDM_FLTxICR register."]
        #[inline(always)]
        pub const fn set_rovrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Analog watchdog This bit is set by hardware. It is cleared by software by clearing all source flag bits AWHTF\\[7:0\\]
and AWLTF\\[7:0\\]
in DFSDM_FLTxAWSR register (by writing '1' into the clear bits in DFSDM_FLTxAWCFR register)."]
        #[must_use]
        #[inline(always)]
        pub const fn awdf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Analog watchdog This bit is set by hardware. It is cleared by software by clearing all source flag bits AWHTF\\[7:0\\]
and AWLTF\\[7:0\\]
in DFSDM_FLTxAWSR register (by writing '1' into the clear bits in DFSDM_FLTxAWCFR register)."]
        #[inline(always)]
        pub const fn set_awdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Injected conversion in progress status A request to start an injected conversion is ignored when JCIP=1."]
        #[must_use]
        #[inline(always)]
        pub const fn jcip(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Injected conversion in progress status A request to start an injected conversion is ignored when JCIP=1."]
        #[inline(always)]
        pub const fn set_jcip(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Regular conversion in progress status A request to start a regular conversion is ignored when RCIP=1."]
        #[must_use]
        #[inline(always)]
        pub const fn rcip(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Regular conversion in progress status A request to start a regular conversion is ignored when RCIP=1."]
        #[inline(always)]
        pub const fn set_rcip(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Clock absence flag CKABF\\[y\\]=0: Clock signal on channel y is present. CKABF\\[y\\]=1: Clock signal on channel y is not present. Given y bit is set by hardware when clock absence is detected on channel y. It is held at CKABF\\[y\\]=1 state by hardware when CHEN=0 (see DFSDM_CHyCFGR1 register). It is held at CKABF\\[y\\]=1 state by hardware when the transceiver is not yet synchronized.It can be cleared by software using the corresponding CLRCKABF\\[y\\]
bit in the DFSDM_FLTxICR register. Note: CKABF\\[7:0\\]
is present only in DFSDM_FLT0ISR register (filter x=0)."]
        #[must_use]
        #[inline(always)]
        pub const fn ckabf(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Clock absence flag CKABF\\[y\\]=0: Clock signal on channel y is present. CKABF\\[y\\]=1: Clock signal on channel y is not present. Given y bit is set by hardware when clock absence is detected on channel y. It is held at CKABF\\[y\\]=1 state by hardware when CHEN=0 (see DFSDM_CHyCFGR1 register). It is held at CKABF\\[y\\]=1 state by hardware when the transceiver is not yet synchronized.It can be cleared by software using the corresponding CLRCKABF\\[y\\]
bit in the DFSDM_FLTxICR register. Note: CKABF\\[7:0\\]
is present only in DFSDM_FLT0ISR register (filter x=0)."]
        #[inline(always)]
        pub const fn set_ckabf(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Short-circuit detector flag SDCF\\[y\\]=0: No short-circuit detector event occurred on channel y SDCF\\[y\\]=1: The short-circuit detector counter reaches, on channel y, the value programmed in the DFSDM_CHyAWSCDR registers This bit is set by hardware. It can be cleared by software using the corresponding CLRSCDF\\[y\\]
bit in the DFSDM_FLTxICR register. SCDF\\[y\\]
is cleared also by hardware when CHEN\\[y\\]
= 0 (given channel is disabled). Note: SCDF\\[7:0\\]
is present only in DFSDM_FLT0ISR register (filter x=0)."]
        #[must_use]
        #[inline(always)]
        pub const fn scdf(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Short-circuit detector flag SDCF\\[y\\]=0: No short-circuit detector event occurred on channel y SDCF\\[y\\]=1: The short-circuit detector counter reaches, on channel y, the value programmed in the DFSDM_CHyAWSCDR registers This bit is set by hardware. It can be cleared by software using the corresponding CLRSCDF\\[y\\]
bit in the DFSDM_FLTxICR register. SCDF\\[y\\]
is cleared also by hardware when CHEN\\[y\\]
= 0 (given channel is disabled). Note: SCDF\\[7:0\\]
is present only in DFSDM_FLT0ISR register (filter x=0)."]
        #[inline(always)]
        pub const fn set_scdf(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
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
                .field("jeocf", &self.jeocf())
                .field("reocf", &self.reocf())
                .field("jovrf", &self.jovrf())
                .field("rovrf", &self.rovrf())
                .field("awdf", &self.awdf())
                .field("jcip", &self.jcip())
                .field("rcip", &self.rcip())
                .field("ckabf", &self.ckabf())
                .field("scdf", &self.scdf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Isr {{ jeocf: {=bool:?}, reocf: {=bool:?}, jovrf: {=bool:?}, rovrf: {=bool:?}, awdf: {=bool:?}, jcip: {=bool:?}, rcip: {=bool:?}, ckabf: {=u8:?}, scdf: {=u8:?} }}",
                self.jeocf(),
                self.reocf(),
                self.jovrf(),
                self.rovrf(),
                self.awdf(),
                self.jcip(),
                self.rcip(),
                self.ckabf(),
                self.scdf()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jchgr(pub u32);
    impl Jchgr {
        #[doc = "Injected channel group selection JCHG\\[y\\]=0: channel y is not part of the injected group JCHG\\[y\\]=1: channel y is part of the injected group If JSCAN=1, each of the selected channels is converted, one after another. The lowest channel (channel 0, if selected) is converted first and the sequence ends at the highest selected channel. If JSCAN=0, then only one channel is converted from the selected channels, and the channel selection is moved to the next channel. Writing JCHG, if JSCAN=0, resets the channel selection to the lowest selected channel. At least one channel must always be selected for the injected group. Writes causing all JCHG bits to be zero are ignored."]
        #[must_use]
        #[inline(always)]
        pub const fn jchg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Injected channel group selection JCHG\\[y\\]=0: channel y is not part of the injected group JCHG\\[y\\]=1: channel y is part of the injected group If JSCAN=1, each of the selected channels is converted, one after another. The lowest channel (channel 0, if selected) is converted first and the sequence ends at the highest selected channel. If JSCAN=0, then only one channel is converted from the selected channels, and the channel selection is moved to the next channel. Writing JCHG, if JSCAN=0, resets the channel selection to the lowest selected channel. At least one channel must always be selected for the injected group. Writes causing all JCHG bits to be zero are ignored."]
        #[inline(always)]
        pub const fn set_jchg(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Jchgr {
        #[inline(always)]
        fn default() -> Jchgr {
            Jchgr(0)
        }
    }
    impl core::fmt::Debug for Jchgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Jchgr").field("jchg", &self.jchg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Jchgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Jchgr {{ jchg: {=u8:?} }}", self.jchg())
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Jdatar(pub u32);
    impl Jdatar {
        #[doc = "Injected channel most recently converted When each conversion of a channel in the injected group finishes, JDATACH\\[2:0\\]
is updated to indicate which channel was converted. Thus, JDATA\\[23:0\\]
holds the data that corresponds to the channel indicated by JDATACH\\[2:0\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn jdatach(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Injected channel most recently converted When each conversion of a channel in the injected group finishes, JDATACH\\[2:0\\]
is updated to indicate which channel was converted. Thus, JDATA\\[23:0\\]
holds the data that corresponds to the channel indicated by JDATACH\\[2:0\\]."]
        #[inline(always)]
        pub const fn set_jdatach(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Injected group conversion data When each conversion of a channel in the injected group finishes, its resulting data is stored in this field. The data is valid when JEOCF=1. Reading this register clears the corresponding JEOCF."]
        #[must_use]
        #[inline(always)]
        pub const fn jdata(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Injected group conversion data When each conversion of a channel in the injected group finishes, its resulting data is stored in this field. The data is valid when JEOCF=1. Reading this register clears the corresponding JEOCF."]
        #[inline(always)]
        pub const fn set_jdata(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for Jdatar {
        #[inline(always)]
        fn default() -> Jdatar {
            Jdatar(0)
        }
    }
    impl core::fmt::Debug for Jdatar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Jdatar")
                .field("jdatach", &self.jdatach())
                .field("jdata", &self.jdata())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Jdatar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Jdatar {{ jdatach: {=u8:?}, jdata: {=u32:?} }}",
                self.jdatach(),
                self.jdata()
            )
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rdatar(pub u32);
    impl Rdatar {
        #[doc = "Regular channel most recently converted When each regular conversion finishes, RDATACH\\[2:0\\]
is updated to indicate which channel was converted (because regular channel selection RCH\\[2:0\\]
in DFSDM_FLTxCR1 register can be updated during regular conversion). Thus RDATA\\[23:0\\]
holds the data that corresponds to the channel indicated by RDATACH\\[2:0\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn rdatach(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Regular channel most recently converted When each regular conversion finishes, RDATACH\\[2:0\\]
is updated to indicate which channel was converted (because regular channel selection RCH\\[2:0\\]
in DFSDM_FLTxCR1 register can be updated during regular conversion). Thus RDATA\\[23:0\\]
holds the data that corresponds to the channel indicated by RDATACH\\[2:0\\]."]
        #[inline(always)]
        pub const fn set_rdatach(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Regular channel pending data Regular data in RDATA\\[23:0\\]
was delayed due to an injected channel trigger during the conversion."]
        #[must_use]
        #[inline(always)]
        pub const fn rpend(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Regular channel pending data Regular data in RDATA\\[23:0\\]
was delayed due to an injected channel trigger during the conversion."]
        #[inline(always)]
        pub const fn set_rpend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Regular channel conversion data When each regular conversion finishes, its data is stored in this register. The data is valid when REOCF=1. Reading this register clears the corresponding REOCF."]
        #[must_use]
        #[inline(always)]
        pub const fn rdata(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Regular channel conversion data When each regular conversion finishes, its data is stored in this register. The data is valid when REOCF=1. Reading this register clears the corresponding REOCF."]
        #[inline(always)]
        pub const fn set_rdata(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for Rdatar {
        #[inline(always)]
        fn default() -> Rdatar {
            Rdatar(0)
        }
    }
    impl core::fmt::Debug for Rdatar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rdatar")
                .field("rdatach", &self.rdatach())
                .field("rpend", &self.rpend())
                .field("rdata", &self.rdata())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rdatar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Rdatar {{ rdatach: {=u8:?}, rpend: {=bool:?}, rdata: {=u32:?} }}",
                self.rdatach(),
                self.rpend(),
                self.rdata()
            )
        }
    }
    #[doc = "This register specifies the size allocated to DFSDM registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sidr(pub u32);
    impl Sidr {
        #[doc = "Bits \\[31:8\\]: fixed code = 0xA3C5DD. Bits \\[7:0\\]: these bits returns the size of the memory region allocated to DFSDM registers. 0x02: 2KB allocated by DFSDM peripheral (fixed value)."]
        #[must_use]
        #[inline(always)]
        pub const fn sid(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Bits \\[31:8\\]: fixed code = 0xA3C5DD. Bits \\[7:0\\]: these bits returns the size of the memory region allocated to DFSDM registers. 0x02: 2KB allocated by DFSDM peripheral (fixed value)."]
        #[inline(always)]
        pub const fn set_sid(&mut self, val: u32) {
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
            defmt::write!(f, "Sidr {{ sid: {=u32:?} }}", self.sid())
        }
    }
    #[doc = "This register specifies the version of DFSDM peripheral."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Verr(pub u32);
    impl Verr {
        #[doc = "Minor revision of the DFSDM peripheral."]
        #[must_use]
        #[inline(always)]
        pub const fn minrev(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Minor revision of the DFSDM peripheral."]
        #[inline(always)]
        pub const fn set_minrev(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Major revision of the DFSDM peripheral."]
        #[must_use]
        #[inline(always)]
        pub const fn majrev(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Major revision of the DFSDM peripheral."]
        #[inline(always)]
        pub const fn set_majrev(&mut self, val: u8) {
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
            defmt::write!(
                f,
                "Verr {{ minrev: {=u8:?}, majrev: {=u8:?} }}",
                self.minrev(),
                self.majrev()
            )
        }
    }
    #[doc = "DFSDM channel y watchdog filter data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wdatr(pub u32);
    impl Wdatr {
        #[doc = "Input channel y watchdog data Data converted by the analog watchdog filter for input channel y. This data is continuously converted (no trigger) for this channel, with a limited resolution (OSR=1..32/sinc order = 1..3)."]
        #[must_use]
        #[inline(always)]
        pub const fn wdata(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Input channel y watchdog data Data converted by the analog watchdog filter for input channel y. This data is continuously converted (no trigger) for this channel, with a limited resolution (OSR=1..32/sinc order = 1..3)."]
        #[inline(always)]
        pub const fn set_wdata(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Wdatr {
        #[inline(always)]
        fn default() -> Wdatr {
            Wdatr(0)
        }
    }
    impl core::fmt::Debug for Wdatr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wdatr").field("wdata", &self.wdata()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wdatr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wdatr {{ wdata: {=u16:?} }}", self.wdata())
        }
    }
}
