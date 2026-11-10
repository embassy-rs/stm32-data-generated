#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "PLAY register block."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Play {
    ptr: *mut u8,
}
unsafe impl Send for Play {}
unsafe impl Sync for Play {}
impl Play {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "PLAY configuration control register."]
    #[inline(always)]
    pub const fn cfgcr(self) -> crate::common::Reg<regs::Cfgcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "PLAY security attributes configuration register."]
    #[inline(always)]
    pub const fn seccfgr(self) -> crate::common::Reg<regs::Seccfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "PLAY privilege attributes configuration register."]
    #[inline(always)]
    pub const fn privcfgr(self) -> crate::common::Reg<regs::Privcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "PLAY filter 0 configuration register."]
    #[inline(always)]
    pub const fn filtcfg(self, n: usize) -> crate::common::Reg<regs::Filtcfg, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _) }
    }
    #[doc = "PLAY logic element 0 configuration register 1."]
    #[inline(always)]
    pub const fn lecfg1(self, n: usize) -> crate::common::Reg<regs::Lecfg1, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize + n * 4usize) as _) }
    }
    #[doc = "PLAY logic element x configuration register 2."]
    #[inline(always)]
    pub const fn lecfg2(self, n: usize) -> crate::common::Reg<regs::Lecfg2, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize + n * 4usize) as _) }
    }
    #[doc = "PLAY output 0 configuration register."]
    #[inline(always)]
    pub const fn outcfg(self, n: usize) -> crate::common::Reg<regs::Outcfg, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize + n * 4usize) as _) }
    }
    #[doc = "PLAY software input status register."]
    #[inline(always)]
    pub const fn swin(self) -> crate::common::Reg<regs::Swin, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "PLAY software input set register."]
    #[inline(always)]
    pub const fn swinset(self) -> crate::common::Reg<regs::Swinset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "PLAY software input clear register."]
    #[inline(always)]
    pub const fn swinclr(self) -> crate::common::Reg<regs::Swinclr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "PLAY output status register."]
    #[inline(always)]
    pub const fn osr(self) -> crate::common::Reg<regs::Osr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "PLAY flag status register."]
    #[inline(always)]
    pub const fn flstat(self) -> crate::common::Reg<regs::Flstat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "PLAY flag set register."]
    #[inline(always)]
    pub const fn flset(self) -> crate::common::Reg<regs::Flset, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "PLAY flag clear register."]
    #[inline(always)]
    pub const fn flclr(self) -> crate::common::Reg<regs::Flclr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "PLAY flag interrupt enable register."]
    #[inline(always)]
    pub const fn flier(self) -> crate::common::Reg<regs::Flier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x022cusize) as _) }
    }
    #[doc = "PLAY flag control register."]
    #[inline(always)]
    pub const fn flctl(self) -> crate::common::Reg<regs::Flctl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "PLAY miscellaneous status register."]
    #[inline(always)]
    pub const fn msr(self) -> crate::common::Reg<regs::Msr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0234usize) as _) }
    }
    #[doc = "PLAY interrupt status register."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0238usize) as _) }
    }
    #[doc = "PLAY interrupt clear register."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x023cusize) as _) }
    }
    #[doc = "PLAY interrupt enable register."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
}
pub mod regs {
    #[doc = "PLAY configuration control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgcr(pub u32);
    impl Cfgcr {
        #[doc = "Unlock configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn unlock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Unlock configuration."]
        #[inline(always)]
        pub const fn set_unlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Cfgcr {
        #[inline(always)]
        fn default() -> Cfgcr {
            Cfgcr(0)
        }
    }
    impl core::fmt::Debug for Cfgcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgcr").field("unlock", &self.unlock()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cfgcr {{ unlock: {=bool:?} }}", self.unlock())
        }
    }
    #[doc = "PLAY filter n configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Filtcfg(pub u32);
    impl Filtcfg {
        #[doc = "Minimum pulse width."]
        #[must_use]
        #[inline(always)]
        pub const fn width(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Minimum pulse width."]
        #[inline(always)]
        pub const fn set_width(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Edge detection mode."]
        #[must_use]
        #[inline(always)]
        pub const fn edgedet(&self) -> super::vals::FiltcfgEdgedet {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::FiltcfgEdgedet::from_bits(val as u8)
        }
        #[doc = "Edge detection mode."]
        #[inline(always)]
        pub const fn set_edgedet(&mut self, val: super::vals::FiltcfgEdgedet) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "Input pre-multiplexer select."]
        #[must_use]
        #[inline(always)]
        pub const fn premuxsel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Input pre-multiplexer select."]
        #[inline(always)]
        pub const fn set_premuxsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for Filtcfg {
        #[inline(always)]
        fn default() -> Filtcfg {
            Filtcfg(0)
        }
    }
    impl core::fmt::Debug for Filtcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Filtcfg")
                .field("width", &self.width())
                .field("edgedet", &self.edgedet())
                .field("premuxsel", &self.premuxsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Filtcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Filtcfg {{ width: {=u8:?}, edgedet: {:?}, premuxsel: {=u8:?} }}",
                self.width(),
                self.edgedet(),
                self.premuxsel()
            )
        }
    }
    #[doc = "PLAY flag clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Flclr(pub u32);
    impl Flclr {
        #[doc = "Flag clear for LEy direct output."]
        #[must_use]
        #[inline(always)]
        pub const fn flagd_clr(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Flag clear for LEy direct output."]
        #[inline(always)]
        pub const fn set_flagd_clr(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Flag clear for LEy registered output."]
        #[must_use]
        #[inline(always)]
        pub const fn flagr_clr(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Flag clear for LEy registered output."]
        #[inline(always)]
        pub const fn set_flagr_clr(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Flclr {
        #[inline(always)]
        fn default() -> Flclr {
            Flclr(0)
        }
    }
    impl core::fmt::Debug for Flclr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Flclr")
                .field("flagd_clr[0]", &self.flagd_clr(0usize))
                .field("flagd_clr[1]", &self.flagd_clr(1usize))
                .field("flagd_clr[2]", &self.flagd_clr(2usize))
                .field("flagd_clr[3]", &self.flagd_clr(3usize))
                .field("flagd_clr[4]", &self.flagd_clr(4usize))
                .field("flagd_clr[5]", &self.flagd_clr(5usize))
                .field("flagd_clr[6]", &self.flagd_clr(6usize))
                .field("flagd_clr[7]", &self.flagd_clr(7usize))
                .field("flagd_clr[8]", &self.flagd_clr(8usize))
                .field("flagd_clr[9]", &self.flagd_clr(9usize))
                .field("flagd_clr[10]", &self.flagd_clr(10usize))
                .field("flagd_clr[11]", &self.flagd_clr(11usize))
                .field("flagd_clr[12]", &self.flagd_clr(12usize))
                .field("flagd_clr[13]", &self.flagd_clr(13usize))
                .field("flagd_clr[14]", &self.flagd_clr(14usize))
                .field("flagd_clr[15]", &self.flagd_clr(15usize))
                .field("flagr_clr[0]", &self.flagr_clr(0usize))
                .field("flagr_clr[1]", &self.flagr_clr(1usize))
                .field("flagr_clr[2]", &self.flagr_clr(2usize))
                .field("flagr_clr[3]", &self.flagr_clr(3usize))
                .field("flagr_clr[4]", &self.flagr_clr(4usize))
                .field("flagr_clr[5]", &self.flagr_clr(5usize))
                .field("flagr_clr[6]", &self.flagr_clr(6usize))
                .field("flagr_clr[7]", &self.flagr_clr(7usize))
                .field("flagr_clr[8]", &self.flagr_clr(8usize))
                .field("flagr_clr[9]", &self.flagr_clr(9usize))
                .field("flagr_clr[10]", &self.flagr_clr(10usize))
                .field("flagr_clr[11]", &self.flagr_clr(11usize))
                .field("flagr_clr[12]", &self.flagr_clr(12usize))
                .field("flagr_clr[13]", &self.flagr_clr(13usize))
                .field("flagr_clr[14]", &self.flagr_clr(14usize))
                .field("flagr_clr[15]", &self.flagr_clr(15usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Flclr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Flclr {{ flagd_clr[0]: {=bool:?}, flagd_clr[1]: {=bool:?}, flagd_clr[2]: {=bool:?}, flagd_clr[3]: {=bool:?}, flagd_clr[4]: {=bool:?}, flagd_clr[5]: {=bool:?}, flagd_clr[6]: {=bool:?}, flagd_clr[7]: {=bool:?}, flagd_clr[8]: {=bool:?}, flagd_clr[9]: {=bool:?}, flagd_clr[10]: {=bool:?}, flagd_clr[11]: {=bool:?}, flagd_clr[12]: {=bool:?}, flagd_clr[13]: {=bool:?}, flagd_clr[14]: {=bool:?}, flagd_clr[15]: {=bool:?}, flagr_clr[0]: {=bool:?}, flagr_clr[1]: {=bool:?}, flagr_clr[2]: {=bool:?}, flagr_clr[3]: {=bool:?}, flagr_clr[4]: {=bool:?}, flagr_clr[5]: {=bool:?}, flagr_clr[6]: {=bool:?}, flagr_clr[7]: {=bool:?}, flagr_clr[8]: {=bool:?}, flagr_clr[9]: {=bool:?}, flagr_clr[10]: {=bool:?}, flagr_clr[11]: {=bool:?}, flagr_clr[12]: {=bool:?}, flagr_clr[13]: {=bool:?}, flagr_clr[14]: {=bool:?}, flagr_clr[15]: {=bool:?} }}",
                self.flagd_clr(0usize),
                self.flagd_clr(1usize),
                self.flagd_clr(2usize),
                self.flagd_clr(3usize),
                self.flagd_clr(4usize),
                self.flagd_clr(5usize),
                self.flagd_clr(6usize),
                self.flagd_clr(7usize),
                self.flagd_clr(8usize),
                self.flagd_clr(9usize),
                self.flagd_clr(10usize),
                self.flagd_clr(11usize),
                self.flagd_clr(12usize),
                self.flagd_clr(13usize),
                self.flagd_clr(14usize),
                self.flagd_clr(15usize),
                self.flagr_clr(0usize),
                self.flagr_clr(1usize),
                self.flagr_clr(2usize),
                self.flagr_clr(3usize),
                self.flagr_clr(4usize),
                self.flagr_clr(5usize),
                self.flagr_clr(6usize),
                self.flagr_clr(7usize),
                self.flagr_clr(8usize),
                self.flagr_clr(9usize),
                self.flagr_clr(10usize),
                self.flagr_clr(11usize),
                self.flagr_clr(12usize),
                self.flagr_clr(13usize),
                self.flagr_clr(14usize),
                self.flagr_clr(15usize)
            )
        }
    }
    #[doc = "PLAY flag control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Flctl(pub u32);
    impl Flctl {
        #[doc = "Edge direction for LEy direct output."]
        #[must_use]
        #[inline(always)]
        pub const fn flagd_edge(&self, n: usize) -> super::vals::FlagEdge {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::FlagEdge::from_bits(val as u8)
        }
        #[doc = "Edge direction for LEy direct output."]
        #[inline(always)]
        pub const fn set_flagd_edge(&mut self, n: usize, val: super::vals::FlagEdge) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Edge direction for LEy registered output."]
        #[must_use]
        #[inline(always)]
        pub const fn flagr_edge(&self, n: usize) -> super::vals::FlagEdge {
            assert!(n < 16usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::FlagEdge::from_bits(val as u8)
        }
        #[doc = "Edge direction for LEy registered output."]
        #[inline(always)]
        pub const fn set_flagr_edge(&mut self, n: usize, val: super::vals::FlagEdge) {
            assert!(n < 16usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
    }
    impl Default for Flctl {
        #[inline(always)]
        fn default() -> Flctl {
            Flctl(0)
        }
    }
    impl core::fmt::Debug for Flctl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Flctl")
                .field("flagd_edge[0]", &self.flagd_edge(0usize))
                .field("flagd_edge[1]", &self.flagd_edge(1usize))
                .field("flagd_edge[2]", &self.flagd_edge(2usize))
                .field("flagd_edge[3]", &self.flagd_edge(3usize))
                .field("flagd_edge[4]", &self.flagd_edge(4usize))
                .field("flagd_edge[5]", &self.flagd_edge(5usize))
                .field("flagd_edge[6]", &self.flagd_edge(6usize))
                .field("flagd_edge[7]", &self.flagd_edge(7usize))
                .field("flagd_edge[8]", &self.flagd_edge(8usize))
                .field("flagd_edge[9]", &self.flagd_edge(9usize))
                .field("flagd_edge[10]", &self.flagd_edge(10usize))
                .field("flagd_edge[11]", &self.flagd_edge(11usize))
                .field("flagd_edge[12]", &self.flagd_edge(12usize))
                .field("flagd_edge[13]", &self.flagd_edge(13usize))
                .field("flagd_edge[14]", &self.flagd_edge(14usize))
                .field("flagd_edge[15]", &self.flagd_edge(15usize))
                .field("flagr_edge[0]", &self.flagr_edge(0usize))
                .field("flagr_edge[1]", &self.flagr_edge(1usize))
                .field("flagr_edge[2]", &self.flagr_edge(2usize))
                .field("flagr_edge[3]", &self.flagr_edge(3usize))
                .field("flagr_edge[4]", &self.flagr_edge(4usize))
                .field("flagr_edge[5]", &self.flagr_edge(5usize))
                .field("flagr_edge[6]", &self.flagr_edge(6usize))
                .field("flagr_edge[7]", &self.flagr_edge(7usize))
                .field("flagr_edge[8]", &self.flagr_edge(8usize))
                .field("flagr_edge[9]", &self.flagr_edge(9usize))
                .field("flagr_edge[10]", &self.flagr_edge(10usize))
                .field("flagr_edge[11]", &self.flagr_edge(11usize))
                .field("flagr_edge[12]", &self.flagr_edge(12usize))
                .field("flagr_edge[13]", &self.flagr_edge(13usize))
                .field("flagr_edge[14]", &self.flagr_edge(14usize))
                .field("flagr_edge[15]", &self.flagr_edge(15usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Flctl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Flctl {{ flagd_edge[0]: {:?}, flagd_edge[1]: {:?}, flagd_edge[2]: {:?}, flagd_edge[3]: {:?}, flagd_edge[4]: {:?}, flagd_edge[5]: {:?}, flagd_edge[6]: {:?}, flagd_edge[7]: {:?}, flagd_edge[8]: {:?}, flagd_edge[9]: {:?}, flagd_edge[10]: {:?}, flagd_edge[11]: {:?}, flagd_edge[12]: {:?}, flagd_edge[13]: {:?}, flagd_edge[14]: {:?}, flagd_edge[15]: {:?}, flagr_edge[0]: {:?}, flagr_edge[1]: {:?}, flagr_edge[2]: {:?}, flagr_edge[3]: {:?}, flagr_edge[4]: {:?}, flagr_edge[5]: {:?}, flagr_edge[6]: {:?}, flagr_edge[7]: {:?}, flagr_edge[8]: {:?}, flagr_edge[9]: {:?}, flagr_edge[10]: {:?}, flagr_edge[11]: {:?}, flagr_edge[12]: {:?}, flagr_edge[13]: {:?}, flagr_edge[14]: {:?}, flagr_edge[15]: {:?} }}",
                self.flagd_edge(0usize),
                self.flagd_edge(1usize),
                self.flagd_edge(2usize),
                self.flagd_edge(3usize),
                self.flagd_edge(4usize),
                self.flagd_edge(5usize),
                self.flagd_edge(6usize),
                self.flagd_edge(7usize),
                self.flagd_edge(8usize),
                self.flagd_edge(9usize),
                self.flagd_edge(10usize),
                self.flagd_edge(11usize),
                self.flagd_edge(12usize),
                self.flagd_edge(13usize),
                self.flagd_edge(14usize),
                self.flagd_edge(15usize),
                self.flagr_edge(0usize),
                self.flagr_edge(1usize),
                self.flagr_edge(2usize),
                self.flagr_edge(3usize),
                self.flagr_edge(4usize),
                self.flagr_edge(5usize),
                self.flagr_edge(6usize),
                self.flagr_edge(7usize),
                self.flagr_edge(8usize),
                self.flagr_edge(9usize),
                self.flagr_edge(10usize),
                self.flagr_edge(11usize),
                self.flagr_edge(12usize),
                self.flagr_edge(13usize),
                self.flagr_edge(14usize),
                self.flagr_edge(15usize)
            )
        }
    }
    #[doc = "PLAY flag interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Flier(pub u32);
    impl Flier {
        #[doc = "Flag interrupt enable for LEy direct output."]
        #[must_use]
        #[inline(always)]
        pub const fn flagd_ien(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Flag interrupt enable for LEy direct output."]
        #[inline(always)]
        pub const fn set_flagd_ien(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Flag interrupt enable for LEy registered output."]
        #[must_use]
        #[inline(always)]
        pub const fn flagr_ien(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Flag interrupt enable for LEy registered output."]
        #[inline(always)]
        pub const fn set_flagr_ien(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Flier {
        #[inline(always)]
        fn default() -> Flier {
            Flier(0)
        }
    }
    impl core::fmt::Debug for Flier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Flier")
                .field("flagd_ien[0]", &self.flagd_ien(0usize))
                .field("flagd_ien[1]", &self.flagd_ien(1usize))
                .field("flagd_ien[2]", &self.flagd_ien(2usize))
                .field("flagd_ien[3]", &self.flagd_ien(3usize))
                .field("flagd_ien[4]", &self.flagd_ien(4usize))
                .field("flagd_ien[5]", &self.flagd_ien(5usize))
                .field("flagd_ien[6]", &self.flagd_ien(6usize))
                .field("flagd_ien[7]", &self.flagd_ien(7usize))
                .field("flagd_ien[8]", &self.flagd_ien(8usize))
                .field("flagd_ien[9]", &self.flagd_ien(9usize))
                .field("flagd_ien[10]", &self.flagd_ien(10usize))
                .field("flagd_ien[11]", &self.flagd_ien(11usize))
                .field("flagd_ien[12]", &self.flagd_ien(12usize))
                .field("flagd_ien[13]", &self.flagd_ien(13usize))
                .field("flagd_ien[14]", &self.flagd_ien(14usize))
                .field("flagd_ien[15]", &self.flagd_ien(15usize))
                .field("flagr_ien[0]", &self.flagr_ien(0usize))
                .field("flagr_ien[1]", &self.flagr_ien(1usize))
                .field("flagr_ien[2]", &self.flagr_ien(2usize))
                .field("flagr_ien[3]", &self.flagr_ien(3usize))
                .field("flagr_ien[4]", &self.flagr_ien(4usize))
                .field("flagr_ien[5]", &self.flagr_ien(5usize))
                .field("flagr_ien[6]", &self.flagr_ien(6usize))
                .field("flagr_ien[7]", &self.flagr_ien(7usize))
                .field("flagr_ien[8]", &self.flagr_ien(8usize))
                .field("flagr_ien[9]", &self.flagr_ien(9usize))
                .field("flagr_ien[10]", &self.flagr_ien(10usize))
                .field("flagr_ien[11]", &self.flagr_ien(11usize))
                .field("flagr_ien[12]", &self.flagr_ien(12usize))
                .field("flagr_ien[13]", &self.flagr_ien(13usize))
                .field("flagr_ien[14]", &self.flagr_ien(14usize))
                .field("flagr_ien[15]", &self.flagr_ien(15usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Flier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Flier {{ flagd_ien[0]: {=bool:?}, flagd_ien[1]: {=bool:?}, flagd_ien[2]: {=bool:?}, flagd_ien[3]: {=bool:?}, flagd_ien[4]: {=bool:?}, flagd_ien[5]: {=bool:?}, flagd_ien[6]: {=bool:?}, flagd_ien[7]: {=bool:?}, flagd_ien[8]: {=bool:?}, flagd_ien[9]: {=bool:?}, flagd_ien[10]: {=bool:?}, flagd_ien[11]: {=bool:?}, flagd_ien[12]: {=bool:?}, flagd_ien[13]: {=bool:?}, flagd_ien[14]: {=bool:?}, flagd_ien[15]: {=bool:?}, flagr_ien[0]: {=bool:?}, flagr_ien[1]: {=bool:?}, flagr_ien[2]: {=bool:?}, flagr_ien[3]: {=bool:?}, flagr_ien[4]: {=bool:?}, flagr_ien[5]: {=bool:?}, flagr_ien[6]: {=bool:?}, flagr_ien[7]: {=bool:?}, flagr_ien[8]: {=bool:?}, flagr_ien[9]: {=bool:?}, flagr_ien[10]: {=bool:?}, flagr_ien[11]: {=bool:?}, flagr_ien[12]: {=bool:?}, flagr_ien[13]: {=bool:?}, flagr_ien[14]: {=bool:?}, flagr_ien[15]: {=bool:?} }}",
                self.flagd_ien(0usize),
                self.flagd_ien(1usize),
                self.flagd_ien(2usize),
                self.flagd_ien(3usize),
                self.flagd_ien(4usize),
                self.flagd_ien(5usize),
                self.flagd_ien(6usize),
                self.flagd_ien(7usize),
                self.flagd_ien(8usize),
                self.flagd_ien(9usize),
                self.flagd_ien(10usize),
                self.flagd_ien(11usize),
                self.flagd_ien(12usize),
                self.flagd_ien(13usize),
                self.flagd_ien(14usize),
                self.flagd_ien(15usize),
                self.flagr_ien(0usize),
                self.flagr_ien(1usize),
                self.flagr_ien(2usize),
                self.flagr_ien(3usize),
                self.flagr_ien(4usize),
                self.flagr_ien(5usize),
                self.flagr_ien(6usize),
                self.flagr_ien(7usize),
                self.flagr_ien(8usize),
                self.flagr_ien(9usize),
                self.flagr_ien(10usize),
                self.flagr_ien(11usize),
                self.flagr_ien(12usize),
                self.flagr_ien(13usize),
                self.flagr_ien(14usize),
                self.flagr_ien(15usize)
            )
        }
    }
    #[doc = "PLAY flag set register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Flset(pub u32);
    impl Flset {
        #[doc = "Flag set for LEy direct output."]
        #[must_use]
        #[inline(always)]
        pub const fn flagd_set(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Flag set for LEy direct output."]
        #[inline(always)]
        pub const fn set_flagd_set(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Flag set for LEy registered output."]
        #[must_use]
        #[inline(always)]
        pub const fn flagr_set(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Flag set for LEy registered output."]
        #[inline(always)]
        pub const fn set_flagr_set(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Flset {
        #[inline(always)]
        fn default() -> Flset {
            Flset(0)
        }
    }
    impl core::fmt::Debug for Flset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Flset")
                .field("flagd_set[0]", &self.flagd_set(0usize))
                .field("flagd_set[1]", &self.flagd_set(1usize))
                .field("flagd_set[2]", &self.flagd_set(2usize))
                .field("flagd_set[3]", &self.flagd_set(3usize))
                .field("flagd_set[4]", &self.flagd_set(4usize))
                .field("flagd_set[5]", &self.flagd_set(5usize))
                .field("flagd_set[6]", &self.flagd_set(6usize))
                .field("flagd_set[7]", &self.flagd_set(7usize))
                .field("flagd_set[8]", &self.flagd_set(8usize))
                .field("flagd_set[9]", &self.flagd_set(9usize))
                .field("flagd_set[10]", &self.flagd_set(10usize))
                .field("flagd_set[11]", &self.flagd_set(11usize))
                .field("flagd_set[12]", &self.flagd_set(12usize))
                .field("flagd_set[13]", &self.flagd_set(13usize))
                .field("flagd_set[14]", &self.flagd_set(14usize))
                .field("flagd_set[15]", &self.flagd_set(15usize))
                .field("flagr_set[0]", &self.flagr_set(0usize))
                .field("flagr_set[1]", &self.flagr_set(1usize))
                .field("flagr_set[2]", &self.flagr_set(2usize))
                .field("flagr_set[3]", &self.flagr_set(3usize))
                .field("flagr_set[4]", &self.flagr_set(4usize))
                .field("flagr_set[5]", &self.flagr_set(5usize))
                .field("flagr_set[6]", &self.flagr_set(6usize))
                .field("flagr_set[7]", &self.flagr_set(7usize))
                .field("flagr_set[8]", &self.flagr_set(8usize))
                .field("flagr_set[9]", &self.flagr_set(9usize))
                .field("flagr_set[10]", &self.flagr_set(10usize))
                .field("flagr_set[11]", &self.flagr_set(11usize))
                .field("flagr_set[12]", &self.flagr_set(12usize))
                .field("flagr_set[13]", &self.flagr_set(13usize))
                .field("flagr_set[14]", &self.flagr_set(14usize))
                .field("flagr_set[15]", &self.flagr_set(15usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Flset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Flset {{ flagd_set[0]: {=bool:?}, flagd_set[1]: {=bool:?}, flagd_set[2]: {=bool:?}, flagd_set[3]: {=bool:?}, flagd_set[4]: {=bool:?}, flagd_set[5]: {=bool:?}, flagd_set[6]: {=bool:?}, flagd_set[7]: {=bool:?}, flagd_set[8]: {=bool:?}, flagd_set[9]: {=bool:?}, flagd_set[10]: {=bool:?}, flagd_set[11]: {=bool:?}, flagd_set[12]: {=bool:?}, flagd_set[13]: {=bool:?}, flagd_set[14]: {=bool:?}, flagd_set[15]: {=bool:?}, flagr_set[0]: {=bool:?}, flagr_set[1]: {=bool:?}, flagr_set[2]: {=bool:?}, flagr_set[3]: {=bool:?}, flagr_set[4]: {=bool:?}, flagr_set[5]: {=bool:?}, flagr_set[6]: {=bool:?}, flagr_set[7]: {=bool:?}, flagr_set[8]: {=bool:?}, flagr_set[9]: {=bool:?}, flagr_set[10]: {=bool:?}, flagr_set[11]: {=bool:?}, flagr_set[12]: {=bool:?}, flagr_set[13]: {=bool:?}, flagr_set[14]: {=bool:?}, flagr_set[15]: {=bool:?} }}",
                self.flagd_set(0usize),
                self.flagd_set(1usize),
                self.flagd_set(2usize),
                self.flagd_set(3usize),
                self.flagd_set(4usize),
                self.flagd_set(5usize),
                self.flagd_set(6usize),
                self.flagd_set(7usize),
                self.flagd_set(8usize),
                self.flagd_set(9usize),
                self.flagd_set(10usize),
                self.flagd_set(11usize),
                self.flagd_set(12usize),
                self.flagd_set(13usize),
                self.flagd_set(14usize),
                self.flagd_set(15usize),
                self.flagr_set(0usize),
                self.flagr_set(1usize),
                self.flagr_set(2usize),
                self.flagr_set(3usize),
                self.flagr_set(4usize),
                self.flagr_set(5usize),
                self.flagr_set(6usize),
                self.flagr_set(7usize),
                self.flagr_set(8usize),
                self.flagr_set(9usize),
                self.flagr_set(10usize),
                self.flagr_set(11usize),
                self.flagr_set(12usize),
                self.flagr_set(13usize),
                self.flagr_set(14usize),
                self.flagr_set(15usize)
            )
        }
    }
    #[doc = "PLAY flag status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Flstat(pub u32);
    impl Flstat {
        #[doc = "Flag state for LEy direct output."]
        #[must_use]
        #[inline(always)]
        pub const fn flagd(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Flag state for LEy direct output."]
        #[inline(always)]
        pub const fn set_flagd(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Flag state for LEy registered output."]
        #[must_use]
        #[inline(always)]
        pub const fn flagr(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Flag state for LEy registered output."]
        #[inline(always)]
        pub const fn set_flagr(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Flstat {
        #[inline(always)]
        fn default() -> Flstat {
            Flstat(0)
        }
    }
    impl core::fmt::Debug for Flstat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Flstat")
                .field("flagd[0]", &self.flagd(0usize))
                .field("flagd[1]", &self.flagd(1usize))
                .field("flagd[2]", &self.flagd(2usize))
                .field("flagd[3]", &self.flagd(3usize))
                .field("flagd[4]", &self.flagd(4usize))
                .field("flagd[5]", &self.flagd(5usize))
                .field("flagd[6]", &self.flagd(6usize))
                .field("flagd[7]", &self.flagd(7usize))
                .field("flagd[8]", &self.flagd(8usize))
                .field("flagd[9]", &self.flagd(9usize))
                .field("flagd[10]", &self.flagd(10usize))
                .field("flagd[11]", &self.flagd(11usize))
                .field("flagd[12]", &self.flagd(12usize))
                .field("flagd[13]", &self.flagd(13usize))
                .field("flagd[14]", &self.flagd(14usize))
                .field("flagd[15]", &self.flagd(15usize))
                .field("flagr[0]", &self.flagr(0usize))
                .field("flagr[1]", &self.flagr(1usize))
                .field("flagr[2]", &self.flagr(2usize))
                .field("flagr[3]", &self.flagr(3usize))
                .field("flagr[4]", &self.flagr(4usize))
                .field("flagr[5]", &self.flagr(5usize))
                .field("flagr[6]", &self.flagr(6usize))
                .field("flagr[7]", &self.flagr(7usize))
                .field("flagr[8]", &self.flagr(8usize))
                .field("flagr[9]", &self.flagr(9usize))
                .field("flagr[10]", &self.flagr(10usize))
                .field("flagr[11]", &self.flagr(11usize))
                .field("flagr[12]", &self.flagr(12usize))
                .field("flagr[13]", &self.flagr(13usize))
                .field("flagr[14]", &self.flagr(14usize))
                .field("flagr[15]", &self.flagr(15usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Flstat {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Flstat {{ flagd[0]: {=bool:?}, flagd[1]: {=bool:?}, flagd[2]: {=bool:?}, flagd[3]: {=bool:?}, flagd[4]: {=bool:?}, flagd[5]: {=bool:?}, flagd[6]: {=bool:?}, flagd[7]: {=bool:?}, flagd[8]: {=bool:?}, flagd[9]: {=bool:?}, flagd[10]: {=bool:?}, flagd[11]: {=bool:?}, flagd[12]: {=bool:?}, flagd[13]: {=bool:?}, flagd[14]: {=bool:?}, flagd[15]: {=bool:?}, flagr[0]: {=bool:?}, flagr[1]: {=bool:?}, flagr[2]: {=bool:?}, flagr[3]: {=bool:?}, flagr[4]: {=bool:?}, flagr[5]: {=bool:?}, flagr[6]: {=bool:?}, flagr[7]: {=bool:?}, flagr[8]: {=bool:?}, flagr[9]: {=bool:?}, flagr[10]: {=bool:?}, flagr[11]: {=bool:?}, flagr[12]: {=bool:?}, flagr[13]: {=bool:?}, flagr[14]: {=bool:?}, flagr[15]: {=bool:?} }}",
                self.flagd(0usize),
                self.flagd(1usize),
                self.flagd(2usize),
                self.flagd(3usize),
                self.flagd(4usize),
                self.flagd(5usize),
                self.flagd(6usize),
                self.flagd(7usize),
                self.flagd(8usize),
                self.flagd(9usize),
                self.flagd(10usize),
                self.flagd(11usize),
                self.flagd(12usize),
                self.flagd(13usize),
                self.flagd(14usize),
                self.flagd(15usize),
                self.flagr(0usize),
                self.flagr(1usize),
                self.flagr(2usize),
                self.flagr(3usize),
                self.flagr(4usize),
                self.flagr(5usize),
                self.flagr(6usize),
                self.flagr(7usize),
                self.flagr(8usize),
                self.flagr(9usize),
                self.flagr(10usize),
                self.flagr(11usize),
                self.flagr(12usize),
                self.flagr(13usize),
                self.flagr(14usize),
                self.flagr(15usize)
            )
        }
    }
    #[doc = "PLAY interrupt clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "SWIN register write complete interrupt clear."]
        #[must_use]
        #[inline(always)]
        pub const fn swinwc_clr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SWIN register write complete interrupt clear."]
        #[inline(always)]
        pub const fn set_swinwc_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "FLCTL register write complete interrupt clear."]
        #[must_use]
        #[inline(always)]
        pub const fn flctlwc_clr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "FLCTL register write complete interrupt clear."]
        #[inline(always)]
        pub const fn set_flctlwc_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
                .field("swinwc_clr", &self.swinwc_clr())
                .field("flctlwc_clr", &self.flctlwc_clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icr {{ swinwc_clr: {=bool:?}, flctlwc_clr: {=bool:?} }}",
                self.swinwc_clr(),
                self.flctlwc_clr()
            )
        }
    }
    #[doc = "PLAY interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "SWIN register write complete interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn swinwc_ien(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SWIN register write complete interrupt enable."]
        #[inline(always)]
        pub const fn set_swinwc_ien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "FLCTL register write complete interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn flctlwc_ien(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "FLCTL register write complete interrupt enable."]
        #[inline(always)]
        pub const fn set_flctlwc_ien(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
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
                .field("swinwc_ien", &self.swinwc_ien())
                .field("flctlwc_ien", &self.flctlwc_ien())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ier {{ swinwc_ien: {=bool:?}, flctlwc_ien: {=bool:?} }}",
                self.swinwc_ien(),
                self.flctlwc_ien()
            )
        }
    }
    #[doc = "PLAY interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "SWIN register write complete interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn swinwc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SWIN register write complete interrupt status."]
        #[inline(always)]
        pub const fn set_swinwc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "FLCTL register write complete interrupt status."]
        #[must_use]
        #[inline(always)]
        pub const fn flctlwc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "FLCTL register write complete interrupt status."]
        #[inline(always)]
        pub const fn set_flctlwc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Flag interrupts status."]
        #[must_use]
        #[inline(always)]
        pub const fn flags(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Flag interrupts status."]
        #[inline(always)]
        pub const fn set_flags(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
                .field("swinwc", &self.swinwc())
                .field("flctlwc", &self.flctlwc())
                .field("flags", &self.flags())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Isr {{ swinwc: {=bool:?}, flctlwc: {=bool:?}, flags: {=bool:?} }}",
                self.swinwc(),
                self.flctlwc(),
                self.flags()
            )
        }
    }
    #[doc = "PLAY logic element n configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lecfg1(pub u32);
    impl Lecfg1 {
        #[doc = "Look-up table contents."]
        #[must_use]
        #[inline(always)]
        pub const fn lut(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Look-up table contents."]
        #[inline(always)]
        pub const fn set_lut(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Lecfg1 {
        #[inline(always)]
        fn default() -> Lecfg1 {
            Lecfg1(0)
        }
    }
    impl core::fmt::Debug for Lecfg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lecfg1").field("lut", &self.lut()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lecfg1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Lecfg1 {{ lut: {=u16:?} }}", self.lut())
        }
    }
    #[doc = "PLAY logic element x configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lecfg2(pub u32);
    impl Lecfg2 {
        #[doc = "Logic element input y selection."]
        #[must_use]
        #[inline(always)]
        pub const fn in_sel(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 6usize;
            let val = (self.0 >> offs) & 0x3f;
            val as u8
        }
        #[doc = "Logic element input y selection."]
        #[inline(always)]
        pub const fn set_in_sel(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 6usize;
            self.0 = (self.0 & !(0x3f << offs)) | (((val as u32) & 0x3f) << offs);
        }
        #[doc = "Clock enable selection."]
        #[must_use]
        #[inline(always)]
        pub const fn ck_sel(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "Clock enable selection."]
        #[inline(always)]
        pub const fn set_ck_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
    }
    impl Default for Lecfg2 {
        #[inline(always)]
        fn default() -> Lecfg2 {
            Lecfg2(0)
        }
    }
    impl core::fmt::Debug for Lecfg2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lecfg2")
                .field("in_sel[0]", &self.in_sel(0usize))
                .field("in_sel[1]", &self.in_sel(1usize))
                .field("in_sel[2]", &self.in_sel(2usize))
                .field("in_sel[3]", &self.in_sel(3usize))
                .field("ck_sel", &self.ck_sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lecfg2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Lecfg2 {{ in_sel[0]: {=u8:?}, in_sel[1]: {=u8:?}, in_sel[2]: {=u8:?}, in_sel[3]: {=u8:?}, ck_sel: {=u8:?} }}",
                self.in_sel(0usize),
                self.in_sel(1usize),
                self.in_sel(2usize),
                self.in_sel(3usize),
                self.ck_sel()
            )
        }
    }
    #[doc = "PLAY miscellaneous status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Msr(pub u32);
    impl Msr {
        #[doc = "SWIN register write busy flag status."]
        #[must_use]
        #[inline(always)]
        pub const fn swinwbfs(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SWIN register write busy flag status."]
        #[inline(always)]
        pub const fn set_swinwbfs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "FLCTL register write busy flag status."]
        #[must_use]
        #[inline(always)]
        pub const fn flctlwbfs(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "FLCTL register write busy flag status."]
        #[inline(always)]
        pub const fn set_flctlwbfs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Msr {
        #[inline(always)]
        fn default() -> Msr {
            Msr(0)
        }
    }
    impl core::fmt::Debug for Msr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Msr")
                .field("swinwbfs", &self.swinwbfs())
                .field("flctlwbfs", &self.flctlwbfs())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Msr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Msr {{ swinwbfs: {=bool:?}, flctlwbfs: {=bool:?} }}",
                self.swinwbfs(),
                self.flctlwbfs()
            )
        }
    }
    #[doc = "PLAY output status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Osr(pub u32);
    impl Osr {
        #[doc = "State of LE direct outputs leoutdy."]
        #[must_use]
        #[inline(always)]
        pub const fn leoutd(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "State of LE direct outputs leoutdy."]
        #[inline(always)]
        pub const fn set_leoutd(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "State of LE registered outputs leoutry."]
        #[must_use]
        #[inline(always)]
        pub const fn leoutr(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "State of LE registered outputs leoutry."]
        #[inline(always)]
        pub const fn set_leoutr(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Osr {
        #[inline(always)]
        fn default() -> Osr {
            Osr(0)
        }
    }
    impl core::fmt::Debug for Osr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Osr")
                .field("leoutd[0]", &self.leoutd(0usize))
                .field("leoutd[1]", &self.leoutd(1usize))
                .field("leoutd[2]", &self.leoutd(2usize))
                .field("leoutd[3]", &self.leoutd(3usize))
                .field("leoutd[4]", &self.leoutd(4usize))
                .field("leoutd[5]", &self.leoutd(5usize))
                .field("leoutd[6]", &self.leoutd(6usize))
                .field("leoutd[7]", &self.leoutd(7usize))
                .field("leoutd[8]", &self.leoutd(8usize))
                .field("leoutd[9]", &self.leoutd(9usize))
                .field("leoutd[10]", &self.leoutd(10usize))
                .field("leoutd[11]", &self.leoutd(11usize))
                .field("leoutd[12]", &self.leoutd(12usize))
                .field("leoutd[13]", &self.leoutd(13usize))
                .field("leoutd[14]", &self.leoutd(14usize))
                .field("leoutd[15]", &self.leoutd(15usize))
                .field("leoutr[0]", &self.leoutr(0usize))
                .field("leoutr[1]", &self.leoutr(1usize))
                .field("leoutr[2]", &self.leoutr(2usize))
                .field("leoutr[3]", &self.leoutr(3usize))
                .field("leoutr[4]", &self.leoutr(4usize))
                .field("leoutr[5]", &self.leoutr(5usize))
                .field("leoutr[6]", &self.leoutr(6usize))
                .field("leoutr[7]", &self.leoutr(7usize))
                .field("leoutr[8]", &self.leoutr(8usize))
                .field("leoutr[9]", &self.leoutr(9usize))
                .field("leoutr[10]", &self.leoutr(10usize))
                .field("leoutr[11]", &self.leoutr(11usize))
                .field("leoutr[12]", &self.leoutr(12usize))
                .field("leoutr[13]", &self.leoutr(13usize))
                .field("leoutr[14]", &self.leoutr(14usize))
                .field("leoutr[15]", &self.leoutr(15usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Osr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Osr {{ leoutd[0]: {=bool:?}, leoutd[1]: {=bool:?}, leoutd[2]: {=bool:?}, leoutd[3]: {=bool:?}, leoutd[4]: {=bool:?}, leoutd[5]: {=bool:?}, leoutd[6]: {=bool:?}, leoutd[7]: {=bool:?}, leoutd[8]: {=bool:?}, leoutd[9]: {=bool:?}, leoutd[10]: {=bool:?}, leoutd[11]: {=bool:?}, leoutd[12]: {=bool:?}, leoutd[13]: {=bool:?}, leoutd[14]: {=bool:?}, leoutd[15]: {=bool:?}, leoutr[0]: {=bool:?}, leoutr[1]: {=bool:?}, leoutr[2]: {=bool:?}, leoutr[3]: {=bool:?}, leoutr[4]: {=bool:?}, leoutr[5]: {=bool:?}, leoutr[6]: {=bool:?}, leoutr[7]: {=bool:?}, leoutr[8]: {=bool:?}, leoutr[9]: {=bool:?}, leoutr[10]: {=bool:?}, leoutr[11]: {=bool:?}, leoutr[12]: {=bool:?}, leoutr[13]: {=bool:?}, leoutr[14]: {=bool:?}, leoutr[15]: {=bool:?} }}",
                self.leoutd(0usize),
                self.leoutd(1usize),
                self.leoutd(2usize),
                self.leoutd(3usize),
                self.leoutd(4usize),
                self.leoutd(5usize),
                self.leoutd(6usize),
                self.leoutd(7usize),
                self.leoutd(8usize),
                self.leoutd(9usize),
                self.leoutd(10usize),
                self.leoutd(11usize),
                self.leoutd(12usize),
                self.leoutd(13usize),
                self.leoutd(14usize),
                self.leoutd(15usize),
                self.leoutr(0usize),
                self.leoutr(1usize),
                self.leoutr(2usize),
                self.leoutr(3usize),
                self.leoutr(4usize),
                self.leoutr(5usize),
                self.leoutr(6usize),
                self.leoutr(7usize),
                self.leoutr(8usize),
                self.leoutr(9usize),
                self.leoutr(10usize),
                self.leoutr(11usize),
                self.leoutr(12usize),
                self.leoutr(13usize),
                self.leoutr(14usize),
                self.leoutr(15usize)
            )
        }
    }
    #[doc = "PLAY output x configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Outcfg(pub u32);
    impl Outcfg {
        #[doc = "Output source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Output source selection."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
    }
    impl Default for Outcfg {
        #[inline(always)]
        fn default() -> Outcfg {
            Outcfg(0)
        }
    }
    impl core::fmt::Debug for Outcfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Outcfg").field("sel", &self.sel()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Outcfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Outcfg {{ sel: {=u8:?} }}", self.sel())
        }
    }
    #[doc = "PLAY privilege attributes configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcfgr(pub u32);
    impl Privcfgr {
        #[doc = "Privilege access control."]
        #[must_use]
        #[inline(always)]
        pub const fn priv_(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Privilege access control."]
        #[inline(always)]
        pub const fn set_priv_(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Privcfgr {
        #[inline(always)]
        fn default() -> Privcfgr {
            Privcfgr(0)
        }
    }
    impl core::fmt::Debug for Privcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Privcfgr").field("priv_", &self.priv_()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Privcfgr {{ priv_: {=u8:?} }}", self.priv_())
        }
    }
    #[doc = "PLAY security attributes configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccfgr(pub u32);
    impl Seccfgr {
        #[doc = "Secure access control."]
        #[must_use]
        #[inline(always)]
        pub const fn sec(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Secure access control."]
        #[inline(always)]
        pub const fn set_sec(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Seccfgr {
        #[inline(always)]
        fn default() -> Seccfgr {
            Seccfgr(0)
        }
    }
    impl core::fmt::Debug for Seccfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Seccfgr").field("sec", &self.sec()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Seccfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Seccfgr {{ sec: {=u8:?} }}", self.sec())
        }
    }
    #[doc = "PLAY software input status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Swin(pub u32);
    impl Swin {
        #[doc = "Input swiny state."]
        #[must_use]
        #[inline(always)]
        pub const fn swin(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Input swiny state."]
        #[inline(always)]
        pub const fn set_swin(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Swin {
        #[inline(always)]
        fn default() -> Swin {
            Swin(0)
        }
    }
    impl core::fmt::Debug for Swin {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Swin")
                .field("swin[0]", &self.swin(0usize))
                .field("swin[1]", &self.swin(1usize))
                .field("swin[2]", &self.swin(2usize))
                .field("swin[3]", &self.swin(3usize))
                .field("swin[4]", &self.swin(4usize))
                .field("swin[5]", &self.swin(5usize))
                .field("swin[6]", &self.swin(6usize))
                .field("swin[7]", &self.swin(7usize))
                .field("swin[8]", &self.swin(8usize))
                .field("swin[9]", &self.swin(9usize))
                .field("swin[10]", &self.swin(10usize))
                .field("swin[11]", &self.swin(11usize))
                .field("swin[12]", &self.swin(12usize))
                .field("swin[13]", &self.swin(13usize))
                .field("swin[14]", &self.swin(14usize))
                .field("swin[15]", &self.swin(15usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Swin {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Swin {{ swin[0]: {=bool:?}, swin[1]: {=bool:?}, swin[2]: {=bool:?}, swin[3]: {=bool:?}, swin[4]: {=bool:?}, swin[5]: {=bool:?}, swin[6]: {=bool:?}, swin[7]: {=bool:?}, swin[8]: {=bool:?}, swin[9]: {=bool:?}, swin[10]: {=bool:?}, swin[11]: {=bool:?}, swin[12]: {=bool:?}, swin[13]: {=bool:?}, swin[14]: {=bool:?}, swin[15]: {=bool:?} }}",
                self.swin(0usize),
                self.swin(1usize),
                self.swin(2usize),
                self.swin(3usize),
                self.swin(4usize),
                self.swin(5usize),
                self.swin(6usize),
                self.swin(7usize),
                self.swin(8usize),
                self.swin(9usize),
                self.swin(10usize),
                self.swin(11usize),
                self.swin(12usize),
                self.swin(13usize),
                self.swin(14usize),
                self.swin(15usize)
            )
        }
    }
    #[doc = "PLAY software input clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Swinclr(pub u32);
    impl Swinclr {
        #[doc = "Reset PLAY_SWIN.SWINy."]
        #[must_use]
        #[inline(always)]
        pub const fn swin(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Reset PLAY_SWIN.SWINy."]
        #[inline(always)]
        pub const fn set_swin(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Swinclr {
        #[inline(always)]
        fn default() -> Swinclr {
            Swinclr(0)
        }
    }
    impl core::fmt::Debug for Swinclr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Swinclr")
                .field("swin[0]", &self.swin(0usize))
                .field("swin[1]", &self.swin(1usize))
                .field("swin[2]", &self.swin(2usize))
                .field("swin[3]", &self.swin(3usize))
                .field("swin[4]", &self.swin(4usize))
                .field("swin[5]", &self.swin(5usize))
                .field("swin[6]", &self.swin(6usize))
                .field("swin[7]", &self.swin(7usize))
                .field("swin[8]", &self.swin(8usize))
                .field("swin[9]", &self.swin(9usize))
                .field("swin[10]", &self.swin(10usize))
                .field("swin[11]", &self.swin(11usize))
                .field("swin[12]", &self.swin(12usize))
                .field("swin[13]", &self.swin(13usize))
                .field("swin[14]", &self.swin(14usize))
                .field("swin[15]", &self.swin(15usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Swinclr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Swinclr {{ swin[0]: {=bool:?}, swin[1]: {=bool:?}, swin[2]: {=bool:?}, swin[3]: {=bool:?}, swin[4]: {=bool:?}, swin[5]: {=bool:?}, swin[6]: {=bool:?}, swin[7]: {=bool:?}, swin[8]: {=bool:?}, swin[9]: {=bool:?}, swin[10]: {=bool:?}, swin[11]: {=bool:?}, swin[12]: {=bool:?}, swin[13]: {=bool:?}, swin[14]: {=bool:?}, swin[15]: {=bool:?} }}",
                self.swin(0usize),
                self.swin(1usize),
                self.swin(2usize),
                self.swin(3usize),
                self.swin(4usize),
                self.swin(5usize),
                self.swin(6usize),
                self.swin(7usize),
                self.swin(8usize),
                self.swin(9usize),
                self.swin(10usize),
                self.swin(11usize),
                self.swin(12usize),
                self.swin(13usize),
                self.swin(14usize),
                self.swin(15usize)
            )
        }
    }
    #[doc = "PLAY software input set register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Swinset(pub u32);
    impl Swinset {
        #[doc = "Set PLAY_SWIN.SWINy."]
        #[must_use]
        #[inline(always)]
        pub const fn swin(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Set PLAY_SWIN.SWINy."]
        #[inline(always)]
        pub const fn set_swin(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Swinset {
        #[inline(always)]
        fn default() -> Swinset {
            Swinset(0)
        }
    }
    impl core::fmt::Debug for Swinset {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Swinset")
                .field("swin[0]", &self.swin(0usize))
                .field("swin[1]", &self.swin(1usize))
                .field("swin[2]", &self.swin(2usize))
                .field("swin[3]", &self.swin(3usize))
                .field("swin[4]", &self.swin(4usize))
                .field("swin[5]", &self.swin(5usize))
                .field("swin[6]", &self.swin(6usize))
                .field("swin[7]", &self.swin(7usize))
                .field("swin[8]", &self.swin(8usize))
                .field("swin[9]", &self.swin(9usize))
                .field("swin[10]", &self.swin(10usize))
                .field("swin[11]", &self.swin(11usize))
                .field("swin[12]", &self.swin(12usize))
                .field("swin[13]", &self.swin(13usize))
                .field("swin[14]", &self.swin(14usize))
                .field("swin[15]", &self.swin(15usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Swinset {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Swinset {{ swin[0]: {=bool:?}, swin[1]: {=bool:?}, swin[2]: {=bool:?}, swin[3]: {=bool:?}, swin[4]: {=bool:?}, swin[5]: {=bool:?}, swin[6]: {=bool:?}, swin[7]: {=bool:?}, swin[8]: {=bool:?}, swin[9]: {=bool:?}, swin[10]: {=bool:?}, swin[11]: {=bool:?}, swin[12]: {=bool:?}, swin[13]: {=bool:?}, swin[14]: {=bool:?}, swin[15]: {=bool:?} }}",
                self.swin(0usize),
                self.swin(1usize),
                self.swin(2usize),
                self.swin(3usize),
                self.swin(4usize),
                self.swin(5usize),
                self.swin(6usize),
                self.swin(7usize),
                self.swin(8usize),
                self.swin(9usize),
                self.swin(10usize),
                self.swin(11usize),
                self.swin(12usize),
                self.swin(13usize),
                self.swin(14usize),
                self.swin(15usize)
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum FiltcfgEdgedet {
        #[doc = "Edge detection is bypassed."]
        Bypassed = 0x0,
        #[doc = "A rising edge transition on the input is converted into a positive pulse of 1 play_clk cycle."]
        Rising = 0x01,
        #[doc = "A falling edge transition on the input is converted into a positive pulse of 1 play_clk cycle."]
        Falling = 0x02,
        #[doc = "A rising or falling edge transition on the input is converted into a positive pulse of 1 play_clk cycle."]
        Both = 0x03,
    }
    impl FiltcfgEdgedet {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> FiltcfgEdgedet {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for FiltcfgEdgedet {
        #[inline(always)]
        fn from(val: u8) -> FiltcfgEdgedet {
            FiltcfgEdgedet::from_bits(val)
        }
    }
    impl From<FiltcfgEdgedet> for u8 {
        #[inline(always)]
        fn from(val: FiltcfgEdgedet) -> u8 {
            FiltcfgEdgedet::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum FlagEdge {
        #[doc = "Is set by a falling edge on leouty."]
        Falling = 0x0,
        #[doc = "Is set by a rising edge on leouty."]
        Rising = 0x01,
    }
    impl FlagEdge {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> FlagEdge {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for FlagEdge {
        #[inline(always)]
        fn from(val: u8) -> FlagEdge {
            FlagEdge::from_bits(val)
        }
    }
    impl From<FlagEdge> for u8 {
        #[inline(always)]
        fn from(val: FlagEdge) -> u8 {
            FlagEdge::to_bits(val)
        }
    }
}
