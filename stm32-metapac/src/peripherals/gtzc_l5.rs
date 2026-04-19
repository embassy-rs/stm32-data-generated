#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "GTZC_TZSC address block description."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GtzcTzsc {
    ptr: *mut u8,
}
unsafe impl Send for GtzcTzsc {}
unsafe impl Sync for GtzcTzsc {}
impl GtzcTzsc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GTZC1 TZSC control register."]
    #[inline(always)]
    pub const fn tzsc_cr(self) -> crate::common::Reg<regs::TzscCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "GTZC1 TZSC secure configuration register 1."]
    #[inline(always)]
    pub const fn tzsc_seccfgr1(self) -> crate::common::Reg<regs::TzscSeccfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "GTZC1 TZSC secure configuration register 2."]
    #[inline(always)]
    pub const fn tzsc_seccfgr2(self) -> crate::common::Reg<regs::TzscSeccfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "GTZC1 TZSC privilege configuration register 1."]
    #[inline(always)]
    pub const fn tzsc_privcfgr1(self) -> crate::common::Reg<regs::TzscPrivcfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "GTZC1 TZSC privilege configuration register 2."]
    #[inline(always)]
    pub const fn tzsc_privcfgr2(self) -> crate::common::Reg<regs::TzscPrivcfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
}
#[doc = "Block-based memory protection controller."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mpcbb {
    ptr: *mut u8,
}
unsafe impl Send for Mpcbb {}
unsafe impl Sync for Mpcbb {}
impl Mpcbb {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "MPCBB control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::MpcbbCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "MPCBB lock vector register."]
    #[inline(always)]
    pub const fn lckvtr(self, n: usize) -> crate::common::Reg<regs::Lckvtr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize + n * 4usize) as _) }
    }
    #[doc = "MPCBB security configuration register."]
    #[inline(always)]
    pub const fn vctr(self, n: usize) -> crate::common::Reg<regs::Vctr, crate::common::RW> {
        assert!(n < 64usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _) }
    }
}
#[doc = "TrustZone interrupt controller."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tzic {
    ptr: *mut u8,
}
unsafe impl Send for Tzic {}
unsafe impl Sync for Tzic {}
impl Tzic {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "TZIC interrupt enable register."]
    #[inline(always)]
    pub const fn ier(self, n: usize) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "TZIC status register."]
    #[inline(always)]
    pub const fn sr(self, n: usize) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize + n * 4usize) as _) }
    }
    #[doc = "TZIC flag clear register."]
    #[inline(always)]
    pub const fn fcr(self, n: usize) -> crate::common::Reg<regs::Fcr, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "TZIC flag clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fcr(pub u32);
    impl Fcr {
        #[must_use]
        #[inline(always)]
        pub const fn cf(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_cf(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
            f.debug_struct("Fcr").field("cf", &self.cf()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Fcr {{ cf: {=u32:?} }}", self.cf())
        }
    }
    #[doc = "TZIC interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[must_use]
        #[inline(always)]
        pub const fn ie(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_ie(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
            f.debug_struct("Ier").field("ie", &self.ie()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ier {{ ie: {=u32:?} }}", self.ie())
        }
    }
    #[doc = "MPCBB lock vector register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lckvtr(pub u32);
    impl Lckvtr {
        #[must_use]
        #[inline(always)]
        pub const fn lcksb(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_lcksb(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Lckvtr {
        #[inline(always)]
        fn default() -> Lckvtr {
            Lckvtr(0)
        }
    }
    impl core::fmt::Debug for Lckvtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lckvtr").field("lcksb", &self.lcksb()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lckvtr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Lckvtr {{ lcksb: {=u32:?} }}", self.lcksb())
        }
    }
    #[doc = "MPCBB control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MpcbbCr(pub u32);
    impl MpcbbCr {
        #[must_use]
        #[inline(always)]
        pub const fn lck(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_lck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn invsecstate(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_invsecstate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn srwiladis(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_srwiladis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for MpcbbCr {
        #[inline(always)]
        fn default() -> MpcbbCr {
            MpcbbCr(0)
        }
    }
    impl core::fmt::Debug for MpcbbCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MpcbbCr")
                .field("lck", &self.lck())
                .field("invsecstate", &self.invsecstate())
                .field("srwiladis", &self.srwiladis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for MpcbbCr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "MpcbbCr {{ lck: {=bool:?}, invsecstate: {=bool:?}, srwiladis: {=bool:?} }}",
                self.lck(),
                self.invsecstate(),
                self.srwiladis()
            )
        }
    }
    #[doc = "TZIC status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[must_use]
        #[inline(always)]
        pub const fn f(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_f(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
            f.debug_struct("Sr").field("f", &self.f()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Sr {{ f: {=u32:?} }}", self.f())
        }
    }
    #[doc = "GTZC1 TZSC control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TzscCr(pub u32);
    impl TzscCr {
        #[doc = "Lock the configuration of TZSC_SECCFGRn and TZSC_PRIVCFGRn registers until next reset."]
        #[must_use]
        #[inline(always)]
        pub const fn lck(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Lock the configuration of TZSC_SECCFGRn and TZSC_PRIVCFGRn registers until next reset."]
        #[inline(always)]
        pub const fn set_lck(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for TzscCr {
        #[inline(always)]
        fn default() -> TzscCr {
            TzscCr(0)
        }
    }
    impl core::fmt::Debug for TzscCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TzscCr").field("lck", &self.lck()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TzscCr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TzscCr {{ lck: {=bool:?} }}", self.lck())
        }
    }
    #[doc = "GTZC1 TZSC privilege configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TzscPrivcfgr1(pub u32);
    impl TzscPrivcfgr1 {
        #[doc = "Privileged access mode for TIM2."]
        #[must_use]
        #[inline(always)]
        pub const fn tim2priv(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for TIM2."]
        #[inline(always)]
        pub const fn set_tim2priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Privileged access mode for TIM3."]
        #[must_use]
        #[inline(always)]
        pub const fn tim3priv(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for TIM3."]
        #[inline(always)]
        pub const fn set_tim3priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Privileged access mode for TIM4."]
        #[must_use]
        #[inline(always)]
        pub const fn tim4priv(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for TIM4."]
        #[inline(always)]
        pub const fn set_tim4priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Privileged access mode for WWDG."]
        #[must_use]
        #[inline(always)]
        pub const fn wwdgpriv(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for WWDG."]
        #[inline(always)]
        pub const fn set_wwdgpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Privileged access mode for IWDG."]
        #[must_use]
        #[inline(always)]
        pub const fn iwdgpriv(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for IWDG."]
        #[inline(always)]
        pub const fn set_iwdgpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Privileged access mode for SPI2."]
        #[must_use]
        #[inline(always)]
        pub const fn spi2priv(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for SPI2."]
        #[inline(always)]
        pub const fn set_spi2priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Privileged access mode for USART2."]
        #[must_use]
        #[inline(always)]
        pub const fn usart2priv(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for USART2."]
        #[inline(always)]
        pub const fn set_usart2priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Privileged access mode for USART3."]
        #[must_use]
        #[inline(always)]
        pub const fn usart3priv(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for USART3."]
        #[inline(always)]
        pub const fn set_usart3priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Privileged access mode for I2C1."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1priv(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for I2C1."]
        #[inline(always)]
        pub const fn set_i2c1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Privileged access mode for I2C2."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c2priv(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for I2C2."]
        #[inline(always)]
        pub const fn set_i2c2priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Privileged access mode for I2C4."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c4priv(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for I2C4."]
        #[inline(always)]
        pub const fn set_i2c4priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Privileged access mode for LPTIM2."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim2priv(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for LPTIM2."]
        #[inline(always)]
        pub const fn set_lptim2priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for TzscPrivcfgr1 {
        #[inline(always)]
        fn default() -> TzscPrivcfgr1 {
            TzscPrivcfgr1(0)
        }
    }
    impl core::fmt::Debug for TzscPrivcfgr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TzscPrivcfgr1")
                .field("tim2priv", &self.tim2priv())
                .field("tim3priv", &self.tim3priv())
                .field("tim4priv", &self.tim4priv())
                .field("wwdgpriv", &self.wwdgpriv())
                .field("iwdgpriv", &self.iwdgpriv())
                .field("spi2priv", &self.spi2priv())
                .field("usart2priv", &self.usart2priv())
                .field("usart3priv", &self.usart3priv())
                .field("i2c1priv", &self.i2c1priv())
                .field("i2c2priv", &self.i2c2priv())
                .field("i2c4priv", &self.i2c4priv())
                .field("lptim2priv", &self.lptim2priv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TzscPrivcfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TzscPrivcfgr1 {{ tim2priv: {=bool:?}, tim3priv: {=bool:?}, tim4priv: {=bool:?}, wwdgpriv: {=bool:?}, iwdgpriv: {=bool:?}, spi2priv: {=bool:?}, usart2priv: {=bool:?}, usart3priv: {=bool:?}, i2c1priv: {=bool:?}, i2c2priv: {=bool:?}, i2c4priv: {=bool:?}, lptim2priv: {=bool:?} }}" , self . tim2priv () , self . tim3priv () , self . tim4priv () , self . wwdgpriv () , self . iwdgpriv () , self . spi2priv () , self . usart2priv () , self . usart3priv () , self . i2c1priv () , self . i2c2priv () , self . i2c4priv () , self . lptim2priv ())
        }
    }
    #[doc = "GTZC1 TZSC privilege configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TzscPrivcfgr2(pub u32);
    impl TzscPrivcfgr2 {
        #[doc = "Privileged access mode for TIM1."]
        #[must_use]
        #[inline(always)]
        pub const fn tim1priv(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for TIM1."]
        #[inline(always)]
        pub const fn set_tim1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Privileged access mode for SPI1PRIV."]
        #[must_use]
        #[inline(always)]
        pub const fn spi1priv(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for SPI1PRIV."]
        #[inline(always)]
        pub const fn set_spi1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Privileged access mode for USART1."]
        #[must_use]
        #[inline(always)]
        pub const fn usart1priv(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for USART1."]
        #[inline(always)]
        pub const fn set_usart1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Privileged access mode for TIM16."]
        #[must_use]
        #[inline(always)]
        pub const fn tim16priv(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for TIM16."]
        #[inline(always)]
        pub const fn set_tim16priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Privileged access mode for TIM17."]
        #[must_use]
        #[inline(always)]
        pub const fn tim17priv(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for TIM17."]
        #[inline(always)]
        pub const fn set_tim17priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Privileged access mode for SAI1."]
        #[must_use]
        #[inline(always)]
        pub const fn sai1priv(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for SAI1."]
        #[inline(always)]
        pub const fn set_sai1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Privileged access mode for SPI3."]
        #[must_use]
        #[inline(always)]
        pub const fn spi3priv(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for SPI3."]
        #[inline(always)]
        pub const fn set_spi3priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Privileged access mode for LPUART1."]
        #[must_use]
        #[inline(always)]
        pub const fn lpuart1priv(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for LPUART1."]
        #[inline(always)]
        pub const fn set_lpuart1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Privileged access mode for I2C3."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c3priv(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for I2C3."]
        #[inline(always)]
        pub const fn set_i2c3priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Privileged access mode for LPTIM1."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim1priv(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for LPTIM1."]
        #[inline(always)]
        pub const fn set_lptim1priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Privileged access mode for COMP."]
        #[must_use]
        #[inline(always)]
        pub const fn comppriv(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for COMP."]
        #[inline(always)]
        pub const fn set_comppriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Privileged access mode for ADC4."]
        #[must_use]
        #[inline(always)]
        pub const fn adc4priv(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for ADC4."]
        #[inline(always)]
        pub const fn set_adc4priv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Privileged access mode for VREFBUF."]
        #[must_use]
        #[inline(always)]
        pub const fn vrefbufpriv(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged access mode for VREFBUF."]
        #[inline(always)]
        pub const fn set_vrefbufpriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for TzscPrivcfgr2 {
        #[inline(always)]
        fn default() -> TzscPrivcfgr2 {
            TzscPrivcfgr2(0)
        }
    }
    impl core::fmt::Debug for TzscPrivcfgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TzscPrivcfgr2")
                .field("tim1priv", &self.tim1priv())
                .field("spi1priv", &self.spi1priv())
                .field("usart1priv", &self.usart1priv())
                .field("tim16priv", &self.tim16priv())
                .field("tim17priv", &self.tim17priv())
                .field("sai1priv", &self.sai1priv())
                .field("spi3priv", &self.spi3priv())
                .field("lpuart1priv", &self.lpuart1priv())
                .field("i2c3priv", &self.i2c3priv())
                .field("lptim1priv", &self.lptim1priv())
                .field("comppriv", &self.comppriv())
                .field("adc4priv", &self.adc4priv())
                .field("vrefbufpriv", &self.vrefbufpriv())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TzscPrivcfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TzscPrivcfgr2 {{ tim1priv: {=bool:?}, spi1priv: {=bool:?}, usart1priv: {=bool:?}, tim16priv: {=bool:?}, tim17priv: {=bool:?}, sai1priv: {=bool:?}, spi3priv: {=bool:?}, lpuart1priv: {=bool:?}, i2c3priv: {=bool:?}, lptim1priv: {=bool:?}, comppriv: {=bool:?}, adc4priv: {=bool:?}, vrefbufpriv: {=bool:?} }}" , self . tim1priv () , self . spi1priv () , self . usart1priv () , self . tim16priv () , self . tim17priv () , self . sai1priv () , self . spi3priv () , self . lpuart1priv () , self . i2c3priv () , self . lptim1priv () , self . comppriv () , self . adc4priv () , self . vrefbufpriv ())
        }
    }
    #[doc = "GTZC1 TZSC secure configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TzscSeccfgr1(pub u32);
    impl TzscSeccfgr1 {
        #[doc = "Secure access mode for TIM2."]
        #[must_use]
        #[inline(always)]
        pub const fn tim2sec(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for TIM2."]
        #[inline(always)]
        pub const fn set_tim2sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Secure access mode for TIM3."]
        #[must_use]
        #[inline(always)]
        pub const fn tim3sec(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for TIM3."]
        #[inline(always)]
        pub const fn set_tim3sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Secure access mode for TIM4."]
        #[must_use]
        #[inline(always)]
        pub const fn tim4sec(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for TIM4."]
        #[inline(always)]
        pub const fn set_tim4sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Secure access mode for WWDG."]
        #[must_use]
        #[inline(always)]
        pub const fn wwdgsec(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for WWDG."]
        #[inline(always)]
        pub const fn set_wwdgsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Secure access mode for IWDG."]
        #[must_use]
        #[inline(always)]
        pub const fn iwdgsec(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for IWDG."]
        #[inline(always)]
        pub const fn set_iwdgsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Secure access mode for SPI2."]
        #[must_use]
        #[inline(always)]
        pub const fn spi2sec(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for SPI2."]
        #[inline(always)]
        pub const fn set_spi2sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Secure access mode for USART2."]
        #[must_use]
        #[inline(always)]
        pub const fn usart2sec(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for USART2."]
        #[inline(always)]
        pub const fn set_usart2sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Secure access mode for USART3."]
        #[must_use]
        #[inline(always)]
        pub const fn usart3sec(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for USART3."]
        #[inline(always)]
        pub const fn set_usart3sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Secure access mode for I2C1."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1sec(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for I2C1."]
        #[inline(always)]
        pub const fn set_i2c1sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Secure access mode for I2C2."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c2sec(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for I2C2."]
        #[inline(always)]
        pub const fn set_i2c2sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Secure access mode for I2C4."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c4sec(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for I2C4."]
        #[inline(always)]
        pub const fn set_i2c4sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Secure access mode for LPTIM2."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim2sec(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for LPTIM2."]
        #[inline(always)]
        pub const fn set_lptim2sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for TzscSeccfgr1 {
        #[inline(always)]
        fn default() -> TzscSeccfgr1 {
            TzscSeccfgr1(0)
        }
    }
    impl core::fmt::Debug for TzscSeccfgr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TzscSeccfgr1")
                .field("tim2sec", &self.tim2sec())
                .field("tim3sec", &self.tim3sec())
                .field("tim4sec", &self.tim4sec())
                .field("wwdgsec", &self.wwdgsec())
                .field("iwdgsec", &self.iwdgsec())
                .field("spi2sec", &self.spi2sec())
                .field("usart2sec", &self.usart2sec())
                .field("usart3sec", &self.usart3sec())
                .field("i2c1sec", &self.i2c1sec())
                .field("i2c2sec", &self.i2c2sec())
                .field("i2c4sec", &self.i2c4sec())
                .field("lptim2sec", &self.lptim2sec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TzscSeccfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TzscSeccfgr1 {{ tim2sec: {=bool:?}, tim3sec: {=bool:?}, tim4sec: {=bool:?}, wwdgsec: {=bool:?}, iwdgsec: {=bool:?}, spi2sec: {=bool:?}, usart2sec: {=bool:?}, usart3sec: {=bool:?}, i2c1sec: {=bool:?}, i2c2sec: {=bool:?}, i2c4sec: {=bool:?}, lptim2sec: {=bool:?} }}" , self . tim2sec () , self . tim3sec () , self . tim4sec () , self . wwdgsec () , self . iwdgsec () , self . spi2sec () , self . usart2sec () , self . usart3sec () , self . i2c1sec () , self . i2c2sec () , self . i2c4sec () , self . lptim2sec ())
        }
    }
    #[doc = "GTZC1 TZSC secure configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TzscSeccfgr2(pub u32);
    impl TzscSeccfgr2 {
        #[doc = "Secure access mode for TIM1."]
        #[must_use]
        #[inline(always)]
        pub const fn tim1sec(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for TIM1."]
        #[inline(always)]
        pub const fn set_tim1sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Secure access mode for SPI1."]
        #[must_use]
        #[inline(always)]
        pub const fn spi1sec(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for SPI1."]
        #[inline(always)]
        pub const fn set_spi1sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Secure access mode for USART1."]
        #[must_use]
        #[inline(always)]
        pub const fn usart1sec(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for USART1."]
        #[inline(always)]
        pub const fn set_usart1sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Secure access mode for TIM16."]
        #[must_use]
        #[inline(always)]
        pub const fn tim16sec(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for TIM16."]
        #[inline(always)]
        pub const fn set_tim16sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Secure access mode for TIM17."]
        #[must_use]
        #[inline(always)]
        pub const fn tim17sec(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for TIM17."]
        #[inline(always)]
        pub const fn set_tim17sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Secure access mode for SAI1."]
        #[must_use]
        #[inline(always)]
        pub const fn sai1sec(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for SAI1."]
        #[inline(always)]
        pub const fn set_sai1sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Secure access mode for SPI3."]
        #[must_use]
        #[inline(always)]
        pub const fn spi3sec(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for SPI3."]
        #[inline(always)]
        pub const fn set_spi3sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Secure access mode for LPUART1."]
        #[must_use]
        #[inline(always)]
        pub const fn lpuart1sec(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for LPUART1."]
        #[inline(always)]
        pub const fn set_lpuart1sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Secure access mode for I2C3."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c3sec(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for I2C3."]
        #[inline(always)]
        pub const fn set_i2c3sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Secure access mode for LPTIM1."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim1sec(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for LPTIM1."]
        #[inline(always)]
        pub const fn set_lptim1sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Secure access mode for COMP."]
        #[must_use]
        #[inline(always)]
        pub const fn compsec(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for COMP."]
        #[inline(always)]
        pub const fn set_compsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Secure access mode for ADC4."]
        #[must_use]
        #[inline(always)]
        pub const fn adc4sec(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for ADC4."]
        #[inline(always)]
        pub const fn set_adc4sec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Secure access mode for VREFBUF."]
        #[must_use]
        #[inline(always)]
        pub const fn vrefbufsec(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Secure access mode for VREFBUF."]
        #[inline(always)]
        pub const fn set_vrefbufsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for TzscSeccfgr2 {
        #[inline(always)]
        fn default() -> TzscSeccfgr2 {
            TzscSeccfgr2(0)
        }
    }
    impl core::fmt::Debug for TzscSeccfgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TzscSeccfgr2")
                .field("tim1sec", &self.tim1sec())
                .field("spi1sec", &self.spi1sec())
                .field("usart1sec", &self.usart1sec())
                .field("tim16sec", &self.tim16sec())
                .field("tim17sec", &self.tim17sec())
                .field("sai1sec", &self.sai1sec())
                .field("spi3sec", &self.spi3sec())
                .field("lpuart1sec", &self.lpuart1sec())
                .field("i2c3sec", &self.i2c3sec())
                .field("lptim1sec", &self.lptim1sec())
                .field("compsec", &self.compsec())
                .field("adc4sec", &self.adc4sec())
                .field("vrefbufsec", &self.vrefbufsec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for TzscSeccfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "TzscSeccfgr2 {{ tim1sec: {=bool:?}, spi1sec: {=bool:?}, usart1sec: {=bool:?}, tim16sec: {=bool:?}, tim17sec: {=bool:?}, sai1sec: {=bool:?}, spi3sec: {=bool:?}, lpuart1sec: {=bool:?}, i2c3sec: {=bool:?}, lptim1sec: {=bool:?}, compsec: {=bool:?}, adc4sec: {=bool:?}, vrefbufsec: {=bool:?} }}" , self . tim1sec () , self . spi1sec () , self . usart1sec () , self . tim16sec () , self . tim17sec () , self . sai1sec () , self . spi3sec () , self . lpuart1sec () , self . i2c3sec () , self . lptim1sec () , self . compsec () , self . adc4sec () , self . vrefbufsec ())
        }
    }
    #[doc = "MPCBB security configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vctr(pub u32);
    impl Vctr {
        #[must_use]
        #[inline(always)]
        pub const fn b(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[inline(always)]
        pub const fn set_b(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Vctr {
        #[inline(always)]
        fn default() -> Vctr {
            Vctr(0)
        }
    }
    impl core::fmt::Debug for Vctr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Vctr").field("b", &self.b()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Vctr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Vctr {{ b: {=u32:?} }}", self.b())
        }
    }
}
