#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "AXI cache."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cacheaxi {
    ptr: *mut u8,
}
unsafe impl Send for Cacheaxi {}
unsafe impl Sync for Cacheaxi {}
impl Cacheaxi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "CACHEAXI control register 1."]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "CACHEAXI status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "CACHEAXI interrupt enable register."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "CACHEAXI flag clear register."]
    #[inline(always)]
    pub const fn fcr(self) -> crate::common::Reg<regs::Fcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "CACHEAXI read-hit monitor register."]
    #[inline(always)]
    pub const fn rhmonr(self) -> crate::common::Reg<regs::Rhmonr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "CACHEAXI read-miss monitor register."]
    #[inline(always)]
    pub const fn rmmonr(self) -> crate::common::Reg<regs::Rmmonr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "CACHEAXI read-allocate miss monitor register."]
    #[inline(always)]
    pub const fn rammonr(self) -> crate::common::Reg<regs::Rammonr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "CACHEAXI eviction monitor register."]
    #[inline(always)]
    pub const fn evimonr(self) -> crate::common::Reg<regs::Evimonr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "CACHEAXI write-hit monitor register."]
    #[inline(always)]
    pub const fn whmonr(self) -> crate::common::Reg<regs::Whmonr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "CACHEAXI write-miss monitor register."]
    #[inline(always)]
    pub const fn wmmonr(self) -> crate::common::Reg<regs::Wmmonr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "CACHEAXI write-allocate miss monitor register."]
    #[inline(always)]
    pub const fn wammonr(self) -> crate::common::Reg<regs::Wammonr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "CACHEAXI write-through monitor register."]
    #[inline(always)]
    pub const fn wtmonr(self) -> crate::common::Reg<regs::Wtmonr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "CACHEAXI control register 2."]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "CACHEAXI command range start address register."]
    #[inline(always)]
    pub const fn cmdrsaddrr(self) -> crate::common::Reg<regs::Cmdrsaddrr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "CACHEAXI command range end address register."]
    #[inline(always)]
    pub const fn cmdreaddrr(self) -> crate::common::Reg<regs::Cmdreaddrr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
}
pub mod regs {
    #[doc = "CACHEAXI command range end address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmdreaddrr(pub u32);
    impl Cmdreaddrr {
        #[doc = "end address of range to which the cache maintenance command specified in CACHEAXI_CR2.CACHECMD field applies."]
        #[must_use]
        #[inline(always)]
        pub const fn cmdendaddr(&self) -> u32 {
            let val = (self.0 >> 6usize) & 0x03ff_ffff;
            val as u32
        }
        #[doc = "end address of range to which the cache maintenance command specified in CACHEAXI_CR2.CACHECMD field applies."]
        #[inline(always)]
        pub const fn set_cmdendaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
        }
    }
    impl Default for Cmdreaddrr {
        #[inline(always)]
        fn default() -> Cmdreaddrr {
            Cmdreaddrr(0)
        }
    }
    impl core::fmt::Debug for Cmdreaddrr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmdreaddrr")
                .field("cmdendaddr", &self.cmdendaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmdreaddrr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cmdreaddrr {{ cmdendaddr: {=u32:?} }}", self.cmdendaddr())
        }
    }
    #[doc = "CACHEAXI command range start address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmdrsaddrr(pub u32);
    impl Cmdrsaddrr {
        #[doc = "start address of range to which the cache maintenance command specified in CACHEAXI_CR2.CACHECMD field applies."]
        #[must_use]
        #[inline(always)]
        pub const fn cmdstartaddr(&self) -> u32 {
            let val = (self.0 >> 6usize) & 0x03ff_ffff;
            val as u32
        }
        #[doc = "start address of range to which the cache maintenance command specified in CACHEAXI_CR2.CACHECMD field applies."]
        #[inline(always)]
        pub const fn set_cmdstartaddr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x03ff_ffff << 6usize)) | (((val as u32) & 0x03ff_ffff) << 6usize);
        }
    }
    impl Default for Cmdrsaddrr {
        #[inline(always)]
        fn default() -> Cmdrsaddrr {
            Cmdrsaddrr(0)
        }
    }
    impl core::fmt::Debug for Cmdrsaddrr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmdrsaddrr")
                .field("cmdstartaddr", &self.cmdstartaddr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cmdrsaddrr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cmdrsaddrr {{ cmdstartaddr: {=u32:?} }}", self.cmdstartaddr())
        }
    }
    #[doc = "CACHEAXI control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "full cache invalidation."]
        #[must_use]
        #[inline(always)]
        pub const fn cacheinv(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "full cache invalidation."]
        #[inline(always)]
        pub const fn set_cacheinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "read-hit monitor enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rhitmen(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "read-hit monitor enable."]
        #[inline(always)]
        pub const fn set_rhitmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "read-miss monitor enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rmissmen(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "read-miss monitor enable."]
        #[inline(always)]
        pub const fn set_rmissmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "read-hit monitor reset."]
        #[must_use]
        #[inline(always)]
        pub const fn rhitmrst(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "read-hit monitor reset."]
        #[inline(always)]
        pub const fn set_rhitmrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "read-miss monitor reset."]
        #[must_use]
        #[inline(always)]
        pub const fn rmissmrst(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "read-miss monitor reset."]
        #[inline(always)]
        pub const fn set_rmissmrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "write-hit monitor enable."]
        #[must_use]
        #[inline(always)]
        pub const fn whitmen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "write-hit monitor enable."]
        #[inline(always)]
        pub const fn set_whitmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "write-miss monitor enable."]
        #[must_use]
        #[inline(always)]
        pub const fn wmissmen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "write-miss monitor enable."]
        #[inline(always)]
        pub const fn set_wmissmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "write-hit monitor reset."]
        #[must_use]
        #[inline(always)]
        pub const fn whitmrst(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "write-hit monitor reset."]
        #[inline(always)]
        pub const fn set_whitmrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "write-miss monitor reset."]
        #[must_use]
        #[inline(always)]
        pub const fn wmissmrst(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "write-miss monitor reset."]
        #[inline(always)]
        pub const fn set_wmissmrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "read-allocate miss monitor enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rammen(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "read-allocate miss monitor enable."]
        #[inline(always)]
        pub const fn set_rammen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "write-allocate miss monitor enable."]
        #[must_use]
        #[inline(always)]
        pub const fn wammen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "write-allocate miss monitor enable."]
        #[inline(always)]
        pub const fn set_wammen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "read-allocate miss monitor reset."]
        #[must_use]
        #[inline(always)]
        pub const fn rammrst(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "read-allocate miss monitor reset."]
        #[inline(always)]
        pub const fn set_rammrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "write-allocate miss monitor reset."]
        #[must_use]
        #[inline(always)]
        pub const fn wammrst(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "write-allocate miss monitor reset."]
        #[inline(always)]
        pub const fn set_wammrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "write-through monitor enable."]
        #[must_use]
        #[inline(always)]
        pub const fn wtmen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "write-through monitor enable."]
        #[inline(always)]
        pub const fn set_wtmen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "eviction monitor enable."]
        #[must_use]
        #[inline(always)]
        pub const fn evimen(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "eviction monitor enable."]
        #[inline(always)]
        pub const fn set_evimen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "write-through monitor reset."]
        #[must_use]
        #[inline(always)]
        pub const fn wtmrst(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "write-through monitor reset."]
        #[inline(always)]
        pub const fn set_wtmrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "eviction monitor reset."]
        #[must_use]
        #[inline(always)]
        pub const fn evimrst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "eviction monitor reset."]
        #[inline(always)]
        pub const fn set_evimrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("en", &self.en())
                .field("cacheinv", &self.cacheinv())
                .field("rhitmen", &self.rhitmen())
                .field("rmissmen", &self.rmissmen())
                .field("rhitmrst", &self.rhitmrst())
                .field("rmissmrst", &self.rmissmrst())
                .field("whitmen", &self.whitmen())
                .field("wmissmen", &self.wmissmen())
                .field("whitmrst", &self.whitmrst())
                .field("wmissmrst", &self.wmissmrst())
                .field("rammen", &self.rammen())
                .field("wammen", &self.wammen())
                .field("rammrst", &self.rammrst())
                .field("wammrst", &self.wammrst())
                .field("wtmen", &self.wtmen())
                .field("evimen", &self.evimen())
                .field("wtmrst", &self.wtmrst())
                .field("evimrst", &self.evimrst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr1 {{ en: {=bool:?}, cacheinv: {=bool:?}, rhitmen: {=bool:?}, rmissmen: {=bool:?}, rhitmrst: {=bool:?}, rmissmrst: {=bool:?}, whitmen: {=bool:?}, wmissmen: {=bool:?}, whitmrst: {=bool:?}, wmissmrst: {=bool:?}, rammen: {=bool:?}, wammen: {=bool:?}, rammrst: {=bool:?}, wammrst: {=bool:?}, wtmen: {=bool:?}, evimen: {=bool:?}, wtmrst: {=bool:?}, evimrst: {=bool:?} }}",
                self.en(),
                self.cacheinv(),
                self.rhitmen(),
                self.rmissmen(),
                self.rhitmrst(),
                self.rmissmrst(),
                self.whitmen(),
                self.wmissmen(),
                self.whitmrst(),
                self.wmissmrst(),
                self.rammen(),
                self.wammen(),
                self.rammrst(),
                self.wammrst(),
                self.wtmen(),
                self.evimen(),
                self.wtmrst(),
                self.evimrst()
            )
        }
    }
    #[doc = "CACHEAXI control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "starts maintenance range command (maintenance operation defined in CACHECMD)."]
        #[must_use]
        #[inline(always)]
        pub const fn startcmd(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "starts maintenance range command (maintenance operation defined in CACHECMD)."]
        #[inline(always)]
        pub const fn set_startcmd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "cache command maintenance operation (clean or clean-and-invalidate an address range)."]
        #[must_use]
        #[inline(always)]
        pub const fn cachecmd(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "cache command maintenance operation (clean or clean-and-invalidate an address range)."]
        #[inline(always)]
        pub const fn set_cachecmd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
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
                .field("startcmd", &self.startcmd())
                .field("cachecmd", &self.cachecmd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr2 {{ startcmd: {=bool:?}, cachecmd: {=u8:?} }}",
                self.startcmd(),
                self.cachecmd()
            )
        }
    }
    #[doc = "CACHEAXI eviction monitor register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Evimonr(pub u32);
    impl Evimonr {
        #[doc = "cache eviction monitor counter."]
        #[must_use]
        #[inline(always)]
        pub const fn evimon(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "cache eviction monitor counter."]
        #[inline(always)]
        pub const fn set_evimon(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Evimonr {
        #[inline(always)]
        fn default() -> Evimonr {
            Evimonr(0)
        }
    }
    impl core::fmt::Debug for Evimonr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Evimonr").field("evimon", &self.evimon()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Evimonr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Evimonr {{ evimon: {=u32:?} }}", self.evimon())
        }
    }
    #[doc = "CACHEAXI flag clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fcr(pub u32);
    impl Fcr {
        #[doc = "clear full invalidate busy end flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cbsyendf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "clear full invalidate busy end flag."]
        #[inline(always)]
        pub const fn set_cbsyendf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "clear cache error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cerrf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "clear cache error flag."]
        #[inline(always)]
        pub const fn set_cerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "clear command end flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ccmdendf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "clear command end flag."]
        #[inline(always)]
        pub const fn set_ccmdendf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
                .field("cbsyendf", &self.cbsyendf())
                .field("cerrf", &self.cerrf())
                .field("ccmdendf", &self.ccmdendf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Fcr {{ cbsyendf: {=bool:?}, cerrf: {=bool:?}, ccmdendf: {=bool:?} }}",
                self.cbsyendf(),
                self.cerrf(),
                self.ccmdendf()
            )
        }
    }
    #[doc = "CACHEAXI interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "interrupt enable on busy end."]
        #[must_use]
        #[inline(always)]
        pub const fn bsyendie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt enable on busy end."]
        #[inline(always)]
        pub const fn set_bsyendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "interrupt enable on cache error."]
        #[must_use]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt enable on cache error."]
        #[inline(always)]
        pub const fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "interrupt enable on command end."]
        #[must_use]
        #[inline(always)]
        pub const fn cmdendie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt enable on command end."]
        #[inline(always)]
        pub const fn set_cmdendie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
                .field("bsyendie", &self.bsyendie())
                .field("errie", &self.errie())
                .field("cmdendie", &self.cmdendie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ier {{ bsyendie: {=bool:?}, errie: {=bool:?}, cmdendie: {=bool:?} }}",
                self.bsyendie(),
                self.errie(),
                self.cmdendie()
            )
        }
    }
    #[doc = "CACHEAXI read-allocate miss monitor register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rammonr(pub u32);
    impl Rammonr {
        #[doc = "cache read-allocate miss monitor counter."]
        #[must_use]
        #[inline(always)]
        pub const fn rammon(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "cache read-allocate miss monitor counter."]
        #[inline(always)]
        pub const fn set_rammon(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Rammonr {
        #[inline(always)]
        fn default() -> Rammonr {
            Rammonr(0)
        }
    }
    impl core::fmt::Debug for Rammonr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rammonr").field("rammon", &self.rammon()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rammonr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rammonr {{ rammon: {=u32:?} }}", self.rammon())
        }
    }
    #[doc = "CACHEAXI read-hit monitor register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rhmonr(pub u32);
    impl Rhmonr {
        #[doc = "cache read-hit monitor counter."]
        #[must_use]
        #[inline(always)]
        pub const fn rhitmon(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "cache read-hit monitor counter."]
        #[inline(always)]
        pub const fn set_rhitmon(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Rhmonr {
        #[inline(always)]
        fn default() -> Rhmonr {
            Rhmonr(0)
        }
    }
    impl core::fmt::Debug for Rhmonr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rhmonr").field("rhitmon", &self.rhitmon()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rhmonr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rhmonr {{ rhitmon: {=u32:?} }}", self.rhitmon())
        }
    }
    #[doc = "CACHEAXI read-miss monitor register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rmmonr(pub u32);
    impl Rmmonr {
        #[doc = "cache read-miss monitor counter."]
        #[must_use]
        #[inline(always)]
        pub const fn rmissmon(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "cache read-miss monitor counter."]
        #[inline(always)]
        pub const fn set_rmissmon(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Rmmonr {
        #[inline(always)]
        fn default() -> Rmmonr {
            Rmmonr(0)
        }
    }
    impl core::fmt::Debug for Rmmonr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rmmonr").field("rmissmon", &self.rmissmon()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rmmonr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rmmonr {{ rmissmon: {=u32:?} }}", self.rmissmon())
        }
    }
    #[doc = "CACHEAXI status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "full invalidate busy flag."]
        #[must_use]
        #[inline(always)]
        pub const fn busyf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "full invalidate busy flag."]
        #[inline(always)]
        pub const fn set_busyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "full invalidate busy end flag."]
        #[must_use]
        #[inline(always)]
        pub const fn bsyendf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "full invalidate busy end flag."]
        #[inline(always)]
        pub const fn set_bsyendf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "cache error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn errf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "cache error flag."]
        #[inline(always)]
        pub const fn set_errf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "command busy flag."]
        #[must_use]
        #[inline(always)]
        pub const fn busycmdf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "command busy flag."]
        #[inline(always)]
        pub const fn set_busycmdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "command end flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cmdendf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "command end flag."]
        #[inline(always)]
        pub const fn set_cmdendf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
                .field("busyf", &self.busyf())
                .field("bsyendf", &self.bsyendf())
                .field("errf", &self.errf())
                .field("busycmdf", &self.busycmdf())
                .field("cmdendf", &self.cmdendf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ busyf: {=bool:?}, bsyendf: {=bool:?}, errf: {=bool:?}, busycmdf: {=bool:?}, cmdendf: {=bool:?} }}",
                self.busyf(),
                self.bsyendf(),
                self.errf(),
                self.busycmdf(),
                self.cmdendf()
            )
        }
    }
    #[doc = "CACHEAXI write-allocate miss monitor register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wammonr(pub u32);
    impl Wammonr {
        #[doc = "cache write-allocate miss monitor counter."]
        #[must_use]
        #[inline(always)]
        pub const fn wammon(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "cache write-allocate miss monitor counter."]
        #[inline(always)]
        pub const fn set_wammon(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Wammonr {
        #[inline(always)]
        fn default() -> Wammonr {
            Wammonr(0)
        }
    }
    impl core::fmt::Debug for Wammonr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wammonr").field("wammon", &self.wammon()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wammonr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wammonr {{ wammon: {=u32:?} }}", self.wammon())
        }
    }
    #[doc = "CACHEAXI write-hit monitor register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Whmonr(pub u32);
    impl Whmonr {
        #[doc = "cache write-hit monitor counter."]
        #[must_use]
        #[inline(always)]
        pub const fn whitmon(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "cache write-hit monitor counter."]
        #[inline(always)]
        pub const fn set_whitmon(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Whmonr {
        #[inline(always)]
        fn default() -> Whmonr {
            Whmonr(0)
        }
    }
    impl core::fmt::Debug for Whmonr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Whmonr").field("whitmon", &self.whitmon()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Whmonr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Whmonr {{ whitmon: {=u32:?} }}", self.whitmon())
        }
    }
    #[doc = "CACHEAXI write-miss monitor register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wmmonr(pub u32);
    impl Wmmonr {
        #[doc = "cache write-miss monitor counter."]
        #[must_use]
        #[inline(always)]
        pub const fn wmissmon(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "cache write-miss monitor counter."]
        #[inline(always)]
        pub const fn set_wmissmon(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Wmmonr {
        #[inline(always)]
        fn default() -> Wmmonr {
            Wmmonr(0)
        }
    }
    impl core::fmt::Debug for Wmmonr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wmmonr").field("wmissmon", &self.wmissmon()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wmmonr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wmmonr {{ wmissmon: {=u32:?} }}", self.wmissmon())
        }
    }
    #[doc = "CACHEAXI write-through monitor register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wtmonr(pub u32);
    impl Wtmonr {
        #[doc = "cache write-through monitor counter."]
        #[must_use]
        #[inline(always)]
        pub const fn wtmon(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "cache write-through monitor counter."]
        #[inline(always)]
        pub const fn set_wtmon(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Wtmonr {
        #[inline(always)]
        fn default() -> Wtmonr {
            Wtmonr(0)
        }
    }
    impl core::fmt::Debug for Wtmonr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wtmonr").field("wtmon", &self.wtmon()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wtmonr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wtmonr {{ wtmon: {=u32:?} }}", self.wtmon())
        }
    }
}
