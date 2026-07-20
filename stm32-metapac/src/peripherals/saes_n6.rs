#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Secure AES coprocessor."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Saes {
    ptr: *mut u8,
}
unsafe impl Send for Saes {}
unsafe impl Sync for Saes {}
impl Saes {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SAES control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "SAES status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "SAES data input register."]
    #[inline(always)]
    pub const fn dinr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "SAES data output register."]
    #[inline(always)]
    pub const fn doutr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "SAES key registers."]
    #[inline(always)]
    pub const fn keyr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(
                0x10usize + ([0usize, 4usize, 8usize, 12usize, 32usize, 36usize, 40usize, 44usize][n] as usize),
            ) as _)
        }
    }
    #[doc = "SAES initialization vector registers."]
    #[inline(always)]
    pub const fn ivr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize + n * 4usize) as _) }
    }
    #[doc = "SAES suspend registers."]
    #[inline(always)]
    pub const fn suspr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _) }
    }
    #[doc = "SAES interrupt enable register."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "SAES interrupt status register."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[doc = "SAES interrupt clear register."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
    }
}
pub mod regs {
    #[doc = "SAES control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
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
        #[doc = "Data type."]
        #[must_use]
        #[inline(always)]
        pub const fn datatype(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "Data type."]
        #[inline(always)]
        pub const fn set_datatype(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[doc = "Operating mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "Operating mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "CHMOD\\[1:0\\]: Chaining mode."]
        #[must_use]
        #[inline(always)]
        pub const fn chmod(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "CHMOD\\[1:0\\]: Chaining mode."]
        #[inline(always)]
        pub const fn set_chmod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "DMA input enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dmainen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "DMA input enable."]
        #[inline(always)]
        pub const fn set_dmainen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "DMA output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dmaouten(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "DMA output enable."]
        #[inline(always)]
        pub const fn set_dmaouten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "GCM or CCM phase selection."]
        #[must_use]
        #[inline(always)]
        pub const fn gcmph(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x03;
            val as u8
        }
        #[doc = "GCM or CCM phase selection."]
        #[inline(always)]
        pub const fn set_gcmph(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val as u32) & 0x03) << 13usize);
        }
        #[doc = "CHMOD\\[2\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn chmod_1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "CHMOD\\[2\\]."]
        #[inline(always)]
        pub const fn set_chmod_1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Key size selection."]
        #[must_use]
        #[inline(always)]
        pub const fn keysize(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Key size selection."]
        #[inline(always)]
        pub const fn set_keysize(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Key protection."]
        #[must_use]
        #[inline(always)]
        pub const fn keyprot(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Key protection."]
        #[inline(always)]
        pub const fn set_keyprot(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Number of padding bytes in last block."]
        #[must_use]
        #[inline(always)]
        pub const fn npblb(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Number of padding bytes in last block."]
        #[inline(always)]
        pub const fn set_npblb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "Key mode selection."]
        #[must_use]
        #[inline(always)]
        pub const fn kmod(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "Key mode selection."]
        #[inline(always)]
        pub const fn set_kmod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "Key share identification."]
        #[must_use]
        #[inline(always)]
        pub const fn kshareid(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x03;
            val as u8
        }
        #[doc = "Key share identification."]
        #[inline(always)]
        pub const fn set_kshareid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
        }
        #[doc = "Key selection."]
        #[must_use]
        #[inline(always)]
        pub const fn keysel(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x07;
            val as u8
        }
        #[doc = "Key selection."]
        #[inline(always)]
        pub const fn set_keysel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 28usize)) | (((val as u32) & 0x07) << 28usize);
        }
        #[doc = "SAES peripheral software reset."]
        #[must_use]
        #[inline(always)]
        pub const fn iprst(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SAES peripheral software reset."]
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
                .field("en", &self.en())
                .field("datatype", &self.datatype())
                .field("mode", &self.mode())
                .field("chmod", &self.chmod())
                .field("dmainen", &self.dmainen())
                .field("dmaouten", &self.dmaouten())
                .field("gcmph", &self.gcmph())
                .field("chmod_1", &self.chmod_1())
                .field("keysize", &self.keysize())
                .field("keyprot", &self.keyprot())
                .field("npblb", &self.npblb())
                .field("kmod", &self.kmod())
                .field("kshareid", &self.kshareid())
                .field("keysel", &self.keysel())
                .field("iprst", &self.iprst())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ en: {=bool:?}, datatype: {=u8:?}, mode: {=u8:?}, chmod: {=u8:?}, dmainen: {=bool:?}, dmaouten: {=bool:?}, gcmph: {=u8:?}, chmod_1: {=bool:?}, keysize: {=bool:?}, keyprot: {=bool:?}, npblb: {=u8:?}, kmod: {=u8:?}, kshareid: {=u8:?}, keysel: {=u8:?}, iprst: {=bool:?} }}",
                self.en(),
                self.datatype(),
                self.mode(),
                self.chmod(),
                self.dmainen(),
                self.dmaouten(),
                self.gcmph(),
                self.chmod_1(),
                self.keysize(),
                self.keyprot(),
                self.npblb(),
                self.kmod(),
                self.kshareid(),
                self.keysel(),
                self.iprst()
            )
        }
    }
    #[doc = "SAES interrupt clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "Computation complete flag clear."]
        #[must_use]
        #[inline(always)]
        pub const fn ccf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Computation complete flag clear."]
        #[inline(always)]
        pub const fn set_ccf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Read or write error interrupt flag clear."]
        #[must_use]
        #[inline(always)]
        pub const fn rweif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Read or write error interrupt flag clear."]
        #[inline(always)]
        pub const fn set_rweif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Key error interrupt flag clear."]
        #[must_use]
        #[inline(always)]
        pub const fn keif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Key error interrupt flag clear."]
        #[inline(always)]
        pub const fn set_keif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "RNG error interrupt flag clear."]
        #[must_use]
        #[inline(always)]
        pub const fn rngeif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RNG error interrupt flag clear."]
        #[inline(always)]
        pub const fn set_rngeif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
                .field("ccf", &self.ccf())
                .field("rweif", &self.rweif())
                .field("keif", &self.keif())
                .field("rngeif", &self.rngeif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icr {{ ccf: {=bool:?}, rweif: {=bool:?}, keif: {=bool:?}, rngeif: {=bool:?} }}",
                self.ccf(),
                self.rweif(),
                self.keif(),
                self.rngeif()
            )
        }
    }
    #[doc = "SAES interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "Computation complete flag interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ccfie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Computation complete flag interrupt enable."]
        #[inline(always)]
        pub const fn set_ccfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Read or write error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rweie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Read or write error interrupt enable."]
        #[inline(always)]
        pub const fn set_rweie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Key error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn keie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Key error interrupt enable."]
        #[inline(always)]
        pub const fn set_keie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "RNG error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rngeie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RNG error interrupt enable."]
        #[inline(always)]
        pub const fn set_rngeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
                .field("ccfie", &self.ccfie())
                .field("rweie", &self.rweie())
                .field("keie", &self.keie())
                .field("rngeie", &self.rngeie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ier {{ ccfie: {=bool:?}, rweie: {=bool:?}, keie: {=bool:?}, rngeie: {=bool:?} }}",
                self.ccfie(),
                self.rweie(),
                self.keie(),
                self.rngeie()
            )
        }
    }
    #[doc = "SAES interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "Computation complete flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ccf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Computation complete flag."]
        #[inline(always)]
        pub const fn set_ccf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Read or write error interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn rweif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Read or write error interrupt flag."]
        #[inline(always)]
        pub const fn set_rweif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Key error interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn keif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Key error interrupt flag."]
        #[inline(always)]
        pub const fn set_keif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "RNG error interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn rngeif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RNG error interrupt flag."]
        #[inline(always)]
        pub const fn set_rngeif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
                .field("ccf", &self.ccf())
                .field("rweif", &self.rweif())
                .field("keif", &self.keif())
                .field("rngeif", &self.rngeif())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Isr {{ ccf: {=bool:?}, rweif: {=bool:?}, keif: {=bool:?}, rngeif: {=bool:?} }}",
                self.ccf(),
                self.rweif(),
                self.keif(),
                self.rngeif()
            )
        }
    }
    #[doc = "SAES status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Read error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn rderrf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Read error flag."]
        #[inline(always)]
        pub const fn set_rderrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Write error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn wrerrf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Write error flag."]
        #[inline(always)]
        pub const fn set_wrerrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Busy."]
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Busy."]
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Key valid flag."]
        #[must_use]
        #[inline(always)]
        pub const fn keyvalid(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Key valid flag."]
        #[inline(always)]
        pub const fn set_keyvalid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
                .field("rderrf", &self.rderrf())
                .field("wrerrf", &self.wrerrf())
                .field("busy", &self.busy())
                .field("keyvalid", &self.keyvalid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ rderrf: {=bool:?}, wrerrf: {=bool:?}, busy: {=bool:?}, keyvalid: {=bool:?} }}",
                self.rderrf(),
                self.wrerrf(),
                self.busy(),
                self.keyvalid()
            )
        }
    }
}
