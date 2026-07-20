#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Chrom-GRC."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gfxmmu {
    ptr: *mut u8,
}
unsafe impl Send for Gfxmmu {}
unsafe impl Sync for Gfxmmu {}
impl Gfxmmu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GFXMMU configuration register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "GFXMMU status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "GFXMMU flag clear register."]
    #[inline(always)]
    pub const fn fcr(self) -> crate::common::Reg<regs::Fcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "GFXMMU default value register."]
    #[inline(always)]
    pub const fn dvr(self) -> crate::common::Reg<regs::Dvr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "GFXMMU default alpha register."]
    #[inline(always)]
    pub const fn dar(self) -> crate::common::Reg<regs::Dar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "GFXMMU buffer 0 configuration register."]
    #[inline(always)]
    pub const fn bcr(self, n: usize) -> crate::common::Reg<regs::Bcr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _) }
    }
    #[doc = "GFXMMU LUT entry 0 low."]
    #[inline(always)]
    pub const fn lutl(self, n: usize) -> crate::common::Reg<regs::Lutl, crate::common::RW> {
        assert!(n < 1024usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1000usize + n * 8usize) as _) }
    }
    #[doc = "GFXMMU LUT entry 0 high."]
    #[inline(always)]
    pub const fn luth(self, n: usize) -> crate::common::Reg<regs::Luth, crate::common::RW> {
        assert!(n < 1024usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1004usize + n * 8usize) as _) }
    }
}
pub mod regs {
    #[doc = "GFXMMU buffer 0 configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bcr(pub u32);
    impl Bcr {
        #[doc = "Physical buffer offset."]
        #[must_use]
        #[inline(always)]
        pub const fn pbo(&self) -> u32 {
            let val = (self.0 >> 4usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "Physical buffer offset."]
        #[inline(always)]
        pub const fn set_pbo(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 4usize)) | (((val as u32) & 0x0007_ffff) << 4usize);
        }
        #[doc = "Physical buffer base address."]
        #[must_use]
        #[inline(always)]
        pub const fn pbba(&self) -> u16 {
            let val = (self.0 >> 23usize) & 0x01ff;
            val as u16
        }
        #[doc = "Physical buffer base address."]
        #[inline(always)]
        pub const fn set_pbba(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 23usize)) | (((val as u32) & 0x01ff) << 23usize);
        }
    }
    impl Default for Bcr {
        #[inline(always)]
        fn default() -> Bcr {
            Bcr(0)
        }
    }
    impl core::fmt::Debug for Bcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bcr")
                .field("pbo", &self.pbo())
                .field("pbba", &self.pbba())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bcr {{ pbo: {=u32:?}, pbba: {=u16:?} }}", self.pbo(), self.pbba())
        }
    }
    #[doc = "GFXMMU configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Buffer 0 overflow interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn boie(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Buffer 0 overflow interrupt enable."]
        #[inline(always)]
        pub const fn set_boie(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "AXI master error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ameie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "AXI master error interrupt enable."]
        #[inline(always)]
        pub const fn set_ameie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Block size."]
        #[must_use]
        #[inline(always)]
        pub const fn bs(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Block size."]
        #[inline(always)]
        pub const fn set_bs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Address translation enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ate(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Address translation enable."]
        #[inline(always)]
        pub const fn set_ate(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Buffer 0 packing enable."]
        #[must_use]
        #[inline(always)]
        pub const fn bpe(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 24usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Buffer 0 packing enable."]
        #[inline(always)]
        pub const fn set_bpe(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 24usize + n * 2usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Buffer 0 packing mode."]
        #[must_use]
        #[inline(always)]
        pub const fn bpm(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 25usize + n * 2usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Buffer 0 packing mode."]
        #[inline(always)]
        pub const fn set_bpm(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 25usize + n * 2usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
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
                .field("boie[0]", &self.boie(0usize))
                .field("boie[1]", &self.boie(1usize))
                .field("boie[2]", &self.boie(2usize))
                .field("boie[3]", &self.boie(3usize))
                .field("ameie", &self.ameie())
                .field("bs", &self.bs())
                .field("ate", &self.ate())
                .field("bpe[0]", &self.bpe(0usize))
                .field("bpe[1]", &self.bpe(1usize))
                .field("bpe[2]", &self.bpe(2usize))
                .field("bpe[3]", &self.bpe(3usize))
                .field("bpm[0]", &self.bpm(0usize))
                .field("bpm[1]", &self.bpm(1usize))
                .field("bpm[2]", &self.bpm(2usize))
                .field("bpm[3]", &self.bpm(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ boie[0]: {=bool:?}, boie[1]: {=bool:?}, boie[2]: {=bool:?}, boie[3]: {=bool:?}, ameie: {=bool:?}, bs: {=bool:?}, ate: {=bool:?}, bpe[0]: {=bool:?}, bpe[1]: {=bool:?}, bpe[2]: {=bool:?}, bpe[3]: {=bool:?}, bpm[0]: {=bool:?}, bpm[1]: {=bool:?}, bpm[2]: {=bool:?}, bpm[3]: {=bool:?} }}",
                self.boie(0usize),
                self.boie(1usize),
                self.boie(2usize),
                self.boie(3usize),
                self.ameie(),
                self.bs(),
                self.ate(),
                self.bpe(0usize),
                self.bpe(1usize),
                self.bpe(2usize),
                self.bpe(3usize),
                self.bpm(0usize),
                self.bpm(1usize),
                self.bpm(2usize),
                self.bpm(3usize)
            )
        }
    }
    #[doc = "GFXMMU default alpha register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dar(pub u32);
    impl Dar {
        #[doc = "Default alpha."]
        #[must_use]
        #[inline(always)]
        pub const fn da(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Default alpha."]
        #[inline(always)]
        pub const fn set_da(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Dar {
        #[inline(always)]
        fn default() -> Dar {
            Dar(0)
        }
    }
    impl core::fmt::Debug for Dar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dar").field("da", &self.da()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dar {{ da: {=u8:?} }}", self.da())
        }
    }
    #[doc = "GFXMMU default value register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dvr(pub u32);
    impl Dvr {
        #[doc = "Default value."]
        #[must_use]
        #[inline(always)]
        pub const fn dv(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Default value."]
        #[inline(always)]
        pub const fn set_dv(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dvr {
        #[inline(always)]
        fn default() -> Dvr {
            Dvr(0)
        }
    }
    impl core::fmt::Debug for Dvr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dvr").field("dv", &self.dv()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dvr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dvr {{ dv: {=u32:?} }}", self.dv())
        }
    }
    #[doc = "GFXMMU flag clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fcr(pub u32);
    impl Fcr {
        #[doc = "Clear buffer 0 overflow flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cbof(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Clear buffer 0 overflow flag."]
        #[inline(always)]
        pub const fn set_cbof(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Clear AXI master error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn camef(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Clear AXI master error flag."]
        #[inline(always)]
        pub const fn set_camef(&mut self, val: bool) {
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
                .field("cbof[0]", &self.cbof(0usize))
                .field("cbof[1]", &self.cbof(1usize))
                .field("cbof[2]", &self.cbof(2usize))
                .field("cbof[3]", &self.cbof(3usize))
                .field("camef", &self.camef())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Fcr {{ cbof[0]: {=bool:?}, cbof[1]: {=bool:?}, cbof[2]: {=bool:?}, cbof[3]: {=bool:?}, camef: {=bool:?} }}",
                self.cbof(0usize),
                self.cbof(1usize),
                self.cbof(2usize),
                self.cbof(3usize),
                self.camef()
            )
        }
    }
    #[doc = "GFXMMU LUT entry 0 high."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Luth(pub u32);
    impl Luth {
        #[doc = "Line offset."]
        #[must_use]
        #[inline(always)]
        pub const fn lo(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "Line offset."]
        #[inline(always)]
        pub const fn set_lo(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
        }
    }
    impl Default for Luth {
        #[inline(always)]
        fn default() -> Luth {
            Luth(0)
        }
    }
    impl core::fmt::Debug for Luth {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Luth").field("lo", &self.lo()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Luth {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Luth {{ lo: {=u32:?} }}", self.lo())
        }
    }
    #[doc = "GFXMMU LUT entry 0 low."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lutl(pub u32);
    impl Lutl {
        #[doc = "Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "First valid block."]
        #[must_use]
        #[inline(always)]
        pub const fn fvb(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "First valid block."]
        #[inline(always)]
        pub const fn set_fvb(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Last valid block."]
        #[must_use]
        #[inline(always)]
        pub const fn lvb(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Last valid block."]
        #[inline(always)]
        pub const fn set_lvb(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Lutl {
        #[inline(always)]
        fn default() -> Lutl {
            Lutl(0)
        }
    }
    impl core::fmt::Debug for Lutl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lutl")
                .field("en", &self.en())
                .field("fvb", &self.fvb())
                .field("lvb", &self.lvb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lutl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Lutl {{ en: {=bool:?}, fvb: {=u8:?}, lvb: {=u8:?} }}",
                self.en(),
                self.fvb(),
                self.lvb()
            )
        }
    }
    #[doc = "GFXMMU status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Buffer 0 overflow flag."]
        #[must_use]
        #[inline(always)]
        pub const fn bof(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Buffer 0 overflow flag."]
        #[inline(always)]
        pub const fn set_bof(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "AXI master error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn amef(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "AXI master error flag."]
        #[inline(always)]
        pub const fn set_amef(&mut self, val: bool) {
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
                .field("bof[0]", &self.bof(0usize))
                .field("bof[1]", &self.bof(1usize))
                .field("bof[2]", &self.bof(2usize))
                .field("bof[3]", &self.bof(3usize))
                .field("amef", &self.amef())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ bof[0]: {=bool:?}, bof[1]: {=bool:?}, bof[2]: {=bool:?}, bof[3]: {=bool:?}, amef: {=bool:?} }}",
                self.bof(0usize),
                self.bof(1usize),
                self.bof(2usize),
                self.bof(3usize),
                self.amef()
            )
        }
    }
}
