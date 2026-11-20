#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Position encoder interface."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Posenc {
    ptr: *mut u8,
}
unsafe impl Send for Posenc {}
unsafe impl Sync for Posenc {}
impl Posenc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "POSENC control register 1."]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "POSENC control register 2."]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "POSENC interrupt enable register."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "POSENC interrupt clear register."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "POSENC masked interrupt status register."]
    #[inline(always)]
    pub const fn misr(self) -> crate::common::Reg<regs::Misr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "POSENC interrupt status register."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "POSENC status register 1."]
    #[inline(always)]
    pub const fn sr1(self) -> crate::common::Reg<regs::Sr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "POSENC status register 2."]
    #[inline(always)]
    pub const fn sr2(self) -> crate::common::Reg<regs::Sr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "POSENC status register 3."]
    #[inline(always)]
    pub const fn sr3(self) -> crate::common::Reg<regs::Sr3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "POSENC data 0 address register."]
    #[inline(always)]
    pub const fn d0ar(self) -> crate::common::Reg<regs::D0ar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "POSENC data 1 address register."]
    #[inline(always)]
    pub const fn d1ar(self) -> crate::common::Reg<regs::D1ar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "POSENC data 0 register."]
    #[inline(always)]
    pub const fn d0r(self) -> crate::common::Reg<regs::D0r, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[doc = "POSENC data 1 register."]
    #[inline(always)]
    pub const fn d1r(self) -> crate::common::Reg<regs::D1r, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
    }
    #[doc = "POSENC Hiperface-DSL safe position 1 CRC register."]
    #[inline(always)]
    pub const fn hd_vposcrcr(self) -> crate::common::Reg<regs::HdVposcrcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x030cusize) as _) }
    }
    #[doc = "POSENC Hiperface-DSL safe position 1 VPOS4 register."]
    #[inline(always)]
    pub const fn hd_vpos4r(self) -> crate::common::Reg<regs::HdVpos4r, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0310usize) as _) }
    }
    #[doc = "POSENC Hiperface-DSL safe position 1 VPOS3_0 register."]
    #[inline(always)]
    pub const fn hd_vpos30r(self) -> crate::common::Reg<regs::HdVpos30r, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0314usize) as _) }
    }
    #[doc = "POSENC Hiperface-DSL safe position 1 VEL2_0 register."]
    #[inline(always)]
    pub const fn hd_vel20r(self) -> crate::common::Reg<regs::HdVel20r, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0318usize) as _) }
    }
    #[doc = "POSENC Hiperface-DSL fast position POS4 register."]
    #[inline(always)]
    pub const fn hd_pos4r(self) -> crate::common::Reg<regs::HdPos4r, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x031cusize) as _) }
    }
    #[doc = "POSENC Hiperface-DSL fast position POS3_0 register."]
    #[inline(always)]
    pub const fn hd_pos30r(self) -> crate::common::Reg<regs::HdPos30r, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0320usize) as _) }
    }
}
pub mod regs {
    #[doc = "POSENC control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "Encoder selection."]
        #[must_use]
        #[inline(always)]
        pub const fn encsel(&self) -> super::vals::Encsel {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Encsel::from_bits(val as u8)
        }
        #[doc = "Encoder selection."]
        #[inline(always)]
        pub const fn set_encsel(&mut self, val: super::vals::Encsel) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "Strobe re-synchronization disable."]
        #[must_use]
        #[inline(always)]
        pub const fn nstrbrsync(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Strobe re-synchronization disable."]
        #[inline(always)]
        pub const fn set_nstrbrsync(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Strobe selection."]
        #[must_use]
        #[inline(always)]
        pub const fn strbsel(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "Strobe selection."]
        #[inline(always)]
        pub const fn set_strbsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
        }
    }
    impl Default for Cr1 {
        #[inline(always)]
        fn default() -> Cr1 {
            Cr1(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr1")
                .field("encsel", &self.encsel())
                .field("nstrbrsync", &self.nstrbrsync())
                .field("strbsel", &self.strbsel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr1 {{ encsel: {:?}, nstrbrsync: {=bool:?}, strbsel: {=u8:?} }}",
                self.encsel(),
                self.nstrbrsync(),
                self.strbsel()
            )
        }
    }
    #[doc = "POSENC control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "Biss-C specific."]
        #[must_use]
        #[inline(always)]
        pub const fn bcprtckcr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Biss-C specific."]
        #[inline(always)]
        pub const fn set_bcprtckcr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Hiperface-DSL fast position event-driven accesses."]
        #[must_use]
        #[inline(always)]
        pub const fn hdpos(&self) -> super::vals::Hdpos {
            let val = (self.0 >> 8usize) & 0x03;
            super::vals::Hdpos::from_bits(val as u8)
        }
        #[doc = "Hiperface-DSL fast position event-driven accesses."]
        #[inline(always)]
        pub const fn set_hdpos(&mut self, val: super::vals::Hdpos) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
        }
        #[doc = "Hiperface-DSL velocity event-driven accesses."]
        #[must_use]
        #[inline(always)]
        pub const fn hdvel(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Hiperface-DSL velocity event-driven accesses."]
        #[inline(always)]
        pub const fn set_hdvel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Hiperface-DSL safe position 1 event-driven accesses."]
        #[must_use]
        #[inline(always)]
        pub const fn hdvpos(&self) -> super::vals::Hdpos {
            let val = (self.0 >> 11usize) & 0x03;
            super::vals::Hdpos::from_bits(val as u8)
        }
        #[doc = "Hiperface-DSL safe position 1 event-driven accesses."]
        #[inline(always)]
        pub const fn set_hdvpos(&mut self, val: super::vals::Hdpos) {
            self.0 = (self.0 & !(0x03 << 11usize)) | (((val.to_bits() as u32) & 0x03) << 11usize);
        }
        #[doc = "Hiperface-DSL safe position 1 CRC event-driven accesses."]
        #[must_use]
        #[inline(always)]
        pub const fn hdvposcrc(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Hiperface-DSL safe position 1 CRC event-driven accesses."]
        #[inline(always)]
        pub const fn set_hdvposcrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Hiperface-DSL freeze."]
        #[must_use]
        #[inline(always)]
        pub const fn hdfrz(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Hiperface-DSL freeze."]
        #[inline(always)]
        pub const fn set_hdfrz(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Cr2 {
        #[inline(always)]
        fn default() -> Cr2 {
            Cr2(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr2")
                .field("bcprtckcr", &self.bcprtckcr())
                .field("hdpos", &self.hdpos())
                .field("hdvel", &self.hdvel())
                .field("hdvpos", &self.hdvpos())
                .field("hdvposcrc", &self.hdvposcrc())
                .field("hdfrz", &self.hdfrz())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr2 {{ bcprtckcr: {=u8:?}, hdpos: {:?}, hdvel: {=bool:?}, hdvpos: {:?}, hdvposcrc: {=bool:?}, hdfrz: {=bool:?} }}",
                self.bcprtckcr(),
                self.hdpos(),
                self.hdvel(),
                self.hdvpos(),
                self.hdvposcrc(),
                self.hdfrz()
            )
        }
    }
    #[doc = "POSENC data 0 address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct D0ar(pub u32);
    impl D0ar {
        #[doc = "Data 0 address."]
        #[must_use]
        #[inline(always)]
        pub const fn d0add(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Data 0 address."]
        #[inline(always)]
        pub const fn set_d0add(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for D0ar {
        #[inline(always)]
        fn default() -> D0ar {
            D0ar(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for D0ar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("D0ar").field("d0add", &self.d0add()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for D0ar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "D0ar {{ d0add: {=u8:?} }}", self.d0add())
        }
    }
    #[doc = "POSENC data 0 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct D0r(pub u32);
    impl D0r {
        #[doc = "Data 0 value."]
        #[must_use]
        #[inline(always)]
        pub const fn d0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Data 0 value."]
        #[inline(always)]
        pub const fn set_d0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for D0r {
        #[inline(always)]
        fn default() -> D0r {
            D0r(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for D0r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("D0r").field("d0", &self.d0()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for D0r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "D0r {{ d0: {=u32:?} }}", self.d0())
        }
    }
    #[doc = "POSENC data 1 address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct D1ar(pub u32);
    impl D1ar {
        #[doc = "Data 1 address."]
        #[must_use]
        #[inline(always)]
        pub const fn d1add(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Data 1 address."]
        #[inline(always)]
        pub const fn set_d1add(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for D1ar {
        #[inline(always)]
        fn default() -> D1ar {
            D1ar(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for D1ar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("D1ar").field("d1add", &self.d1add()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for D1ar {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "D1ar {{ d1add: {=u8:?} }}", self.d1add())
        }
    }
    #[doc = "POSENC data 1 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct D1r(pub u32);
    impl D1r {
        #[doc = "Data 1 value."]
        #[must_use]
        #[inline(always)]
        pub const fn d1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Data 1 value."]
        #[inline(always)]
        pub const fn set_d1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for D1r {
        #[inline(always)]
        fn default() -> D1r {
            D1r(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for D1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("D1r").field("d1", &self.d1()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for D1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "D1r {{ d1: {=u32:?} }}", self.d1())
        }
    }
    #[doc = "POSENC Hiperface-DSL fast position POS3_0 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HdPos30r(pub u32);
    impl HdPos30r {
        #[doc = "Hiperface-DSL fast position value."]
        #[must_use]
        #[inline(always)]
        pub const fn pos(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0xff;
            val as u8
        }
        #[doc = "Hiperface-DSL fast position value."]
        #[inline(always)]
        pub const fn set_pos(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0xff << offs)) | (((val as u32) & 0xff) << offs);
        }
    }
    impl Default for HdPos30r {
        #[inline(always)]
        fn default() -> HdPos30r {
            HdPos30r(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HdPos30r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HdPos30r")
                .field("pos[0]", &self.pos(0usize))
                .field("pos[1]", &self.pos(1usize))
                .field("pos[2]", &self.pos(2usize))
                .field("pos[3]", &self.pos(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HdPos30r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HdPos30r {{ pos[0]: {=u8:?}, pos[1]: {=u8:?}, pos[2]: {=u8:?}, pos[3]: {=u8:?} }}",
                self.pos(0usize),
                self.pos(1usize),
                self.pos(2usize),
                self.pos(3usize)
            )
        }
    }
    #[doc = "POSENC Hiperface-DSL fast position POS4 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HdPos4r(pub u32);
    impl HdPos4r {
        #[doc = "Hiperface-DSL fast position POS4 value."]
        #[must_use]
        #[inline(always)]
        pub const fn pos4(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Hiperface-DSL fast position POS4 value."]
        #[inline(always)]
        pub const fn set_pos4(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for HdPos4r {
        #[inline(always)]
        fn default() -> HdPos4r {
            HdPos4r(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HdPos4r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HdPos4r").field("pos4", &self.pos4()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HdPos4r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HdPos4r {{ pos4: {=u8:?} }}", self.pos4())
        }
    }
    #[doc = "POSENC Hiperface-DSL safe position 1 VEL2_0 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HdVel20r(pub u32);
    impl HdVel20r {
        #[doc = "Hiperface-DSL velocity value."]
        #[must_use]
        #[inline(always)]
        pub const fn vel(&self, n: usize) -> u8 {
            assert!(n < 3usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0xff;
            val as u8
        }
        #[doc = "Hiperface-DSL velocity value."]
        #[inline(always)]
        pub const fn set_vel(&mut self, n: usize, val: u8) {
            assert!(n < 3usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0xff << offs)) | (((val as u32) & 0xff) << offs);
        }
    }
    impl Default for HdVel20r {
        #[inline(always)]
        fn default() -> HdVel20r {
            HdVel20r(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HdVel20r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HdVel20r")
                .field("vel[0]", &self.vel(0usize))
                .field("vel[1]", &self.vel(1usize))
                .field("vel[2]", &self.vel(2usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HdVel20r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HdVel20r {{ vel[0]: {=u8:?}, vel[1]: {=u8:?}, vel[2]: {=u8:?} }}",
                self.vel(0usize),
                self.vel(1usize),
                self.vel(2usize)
            )
        }
    }
    #[doc = "POSENC Hiperface-DSL safe position 1 VPOS3_0 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HdVpos30r(pub u32);
    impl HdVpos30r {
        #[doc = "Hiperface-DSL safe position 1 value."]
        #[must_use]
        #[inline(always)]
        pub const fn vpos(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0xff;
            val as u8
        }
        #[doc = "Hiperface-DSL safe position 1 value."]
        #[inline(always)]
        pub const fn set_vpos(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0xff << offs)) | (((val as u32) & 0xff) << offs);
        }
    }
    impl Default for HdVpos30r {
        #[inline(always)]
        fn default() -> HdVpos30r {
            HdVpos30r(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HdVpos30r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HdVpos30r")
                .field("vpos[0]", &self.vpos(0usize))
                .field("vpos[1]", &self.vpos(1usize))
                .field("vpos[2]", &self.vpos(2usize))
                .field("vpos[3]", &self.vpos(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HdVpos30r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HdVpos30r {{ vpos[0]: {=u8:?}, vpos[1]: {=u8:?}, vpos[2]: {=u8:?}, vpos[3]: {=u8:?} }}",
                self.vpos(0usize),
                self.vpos(1usize),
                self.vpos(2usize),
                self.vpos(3usize)
            )
        }
    }
    #[doc = "POSENC Hiperface-DSL safe position 1 VPOS4 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HdVpos4r(pub u32);
    impl HdVpos4r {
        #[doc = "Hiperface-DSL safe position 1 VPOS4 value."]
        #[must_use]
        #[inline(always)]
        pub const fn vpos4(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Hiperface-DSL safe position 1 VPOS4 value."]
        #[inline(always)]
        pub const fn set_vpos4(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for HdVpos4r {
        #[inline(always)]
        fn default() -> HdVpos4r {
            HdVpos4r(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HdVpos4r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HdVpos4r").field("vpos4", &self.vpos4()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HdVpos4r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HdVpos4r {{ vpos4: {=u8:?} }}", self.vpos4())
        }
    }
    #[doc = "POSENC Hiperface-DSL safe position 1 CRC register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HdVposcrcr(pub u32);
    impl HdVposcrcr {
        #[doc = "Hiperface-DSL safe position 1 CRC low byte."]
        #[must_use]
        #[inline(always)]
        pub const fn vposcrcl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Hiperface-DSL safe position 1 CRC low byte."]
        #[inline(always)]
        pub const fn set_vposcrcl(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Hiperface-DSL safe position 1 CRC high byte."]
        #[must_use]
        #[inline(always)]
        pub const fn vposcrch(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Hiperface-DSL safe position 1 CRC high byte."]
        #[inline(always)]
        pub const fn set_vposcrch(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for HdVposcrcr {
        #[inline(always)]
        fn default() -> HdVposcrcr {
            HdVposcrcr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HdVposcrcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HdVposcrcr")
                .field("vposcrcl", &self.vposcrcl())
                .field("vposcrch", &self.vposcrch())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HdVposcrcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "HdVposcrcr {{ vposcrcl: {=u8:?}, vposcrch: {=u8:?} }}",
                self.vposcrcl(),
                self.vposcrch()
            )
        }
    }
    #[doc = "POSENC interrupt clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "Clear transfer complete interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cgtcf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear transfer complete interrupt flag."]
        #[inline(always)]
        pub const fn set_cgtcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear write transfer complete interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cwrtcf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear write transfer complete interrupt flag."]
        #[inline(always)]
        pub const fn set_cwrtcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Clear data 0 read transfer complete interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cd0rtcf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Clear data 0 read transfer complete interrupt flag."]
        #[inline(always)]
        pub const fn set_cd0rtcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Clear data 1 read transfer complete interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cd1rtcf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Clear data 1 read transfer complete interrupt flag."]
        #[inline(always)]
        pub const fn set_cd1rtcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Clear Hiperface-DSL safe position 1 CRC transfer complete interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hdcvposcrctcf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Clear Hiperface-DSL safe position 1 CRC transfer complete interrupt flag."]
        #[inline(always)]
        pub const fn set_hdcvposcrctcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Clear Hiperface-DSL safe position 1 transfer complete interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hdcvpostcf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Clear Hiperface-DSL safe position 1 transfer complete interrupt flag."]
        #[inline(always)]
        pub const fn set_hdcvpostcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Clear Hiperface-DSL velocity transfer complete interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hdcveltcf(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Clear Hiperface-DSL velocity transfer complete interrupt flag."]
        #[inline(always)]
        pub const fn set_hdcveltcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Clear Hiperface-DSL fast position transfer complete interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hdcfpostcf(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Clear Hiperface-DSL fast position transfer complete interrupt flag."]
        #[inline(always)]
        pub const fn set_hdcfpostcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Clear Hiperface-DSL fast position interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hdcfposf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Clear Hiperface-DSL fast position interrupt flag."]
        #[inline(always)]
        pub const fn set_hdcfposf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Clear Hiperface-DSL safe position 1 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hdcvpos1f(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Clear Hiperface-DSL safe position 1 interrupt flag."]
        #[inline(always)]
        pub const fn set_hdcvpos1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Clear Hiperface-DSL safe position 2 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hdcvpos2f(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Clear Hiperface-DSL safe position 2 interrupt flag."]
        #[inline(always)]
        pub const fn set_hdcvpos2f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Clear Hiperface-DSL long message interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hdcfrelf(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Clear Hiperface-DSL long message interrupt flag."]
        #[inline(always)]
        pub const fn set_hdcfrelf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Clear Hiperface-DSL short message interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hdcfresf(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Clear Hiperface-DSL short message interrupt flag."]
        #[inline(always)]
        pub const fn set_hdcfresf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Icr {
        #[inline(always)]
        fn default() -> Icr {
            Icr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Icr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Icr")
                .field("cgtcf", &self.cgtcf())
                .field("cwrtcf", &self.cwrtcf())
                .field("cd0rtcf", &self.cd0rtcf())
                .field("cd1rtcf", &self.cd1rtcf())
                .field("hdcvposcrctcf", &self.hdcvposcrctcf())
                .field("hdcvpostcf", &self.hdcvpostcf())
                .field("hdcveltcf", &self.hdcveltcf())
                .field("hdcfpostcf", &self.hdcfpostcf())
                .field("hdcfposf", &self.hdcfposf())
                .field("hdcvpos1f", &self.hdcvpos1f())
                .field("hdcvpos2f", &self.hdcvpos2f())
                .field("hdcfrelf", &self.hdcfrelf())
                .field("hdcfresf", &self.hdcfresf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Icr {{ cgtcf: {=bool:?}, cwrtcf: {=bool:?}, cd0rtcf: {=bool:?}, cd1rtcf: {=bool:?}, hdcvposcrctcf: {=bool:?}, hdcvpostcf: {=bool:?}, hdcveltcf: {=bool:?}, hdcfpostcf: {=bool:?}, hdcfposf: {=bool:?}, hdcvpos1f: {=bool:?}, hdcvpos2f: {=bool:?}, hdcfrelf: {=bool:?}, hdcfresf: {=bool:?} }}",
                self.cgtcf(),
                self.cwrtcf(),
                self.cd0rtcf(),
                self.cd1rtcf(),
                self.hdcvposcrctcf(),
                self.hdcvpostcf(),
                self.hdcveltcf(),
                self.hdcfpostcf(),
                self.hdcfposf(),
                self.hdcvpos1f(),
                self.hdcvpos2f(),
                self.hdcfrelf(),
                self.hdcfresf()
            )
        }
    }
    #[doc = "POSENC interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "Global transfer complete interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn gtcie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Global transfer complete interrupt enable."]
        #[inline(always)]
        pub const fn set_gtcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Write transfer complete interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn wrtcie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Write transfer complete interrupt enable."]
        #[inline(always)]
        pub const fn set_wrtcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Read data 0 transfer complete interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn d0rtcie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Read data 0 transfer complete interrupt enable."]
        #[inline(always)]
        pub const fn set_d0rtcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Read data 1 transfer complete interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn d1rtcie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Read data 1 transfer complete interrupt enable."]
        #[inline(always)]
        pub const fn set_d1rtcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Hiperface-DSL safe position 1 CRC transfer complete interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hdvposcrctcie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Hiperface-DSL safe position 1 CRC transfer complete interrupt enable."]
        #[inline(always)]
        pub const fn set_hdvposcrctcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Hiperface-DSL safe position 1 transfer complete interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hdvpostcie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Hiperface-DSL safe position 1 transfer complete interrupt enable."]
        #[inline(always)]
        pub const fn set_hdvpostcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Hiperface-DSL velocity transfer complete interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hdveltcie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Hiperface-DSL velocity transfer complete interrupt enable."]
        #[inline(always)]
        pub const fn set_hdveltcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Hiperface-DSL fast position transfer complete interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hdfpostcie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Hiperface-DSL fast position transfer complete interrupt enable."]
        #[inline(always)]
        pub const fn set_hdfpostcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Hiperface-DSL fast position interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hdfposie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Hiperface-DSL fast position interrupt enable."]
        #[inline(always)]
        pub const fn set_hdfposie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Hiperface-DSL safe position 1 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hdvpos1ie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Hiperface-DSL safe position 1 interrupt enable."]
        #[inline(always)]
        pub const fn set_hdvpos1ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Hiperface-DSL safe position 2 interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hdvpos2ie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Hiperface-DSL safe position 2 interrupt enable."]
        #[inline(always)]
        pub const fn set_hdvpos2ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Hiperface-DSL long message interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hdfrelie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Hiperface-DSL long message interrupt enable."]
        #[inline(always)]
        pub const fn set_hdfrelie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Hiperface-DSL short message interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn hdfresie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Hiperface-DSL short message interrupt enable."]
        #[inline(always)]
        pub const fn set_hdfresie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Ier {
        #[inline(always)]
        fn default() -> Ier {
            Ier(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Ier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ier")
                .field("gtcie", &self.gtcie())
                .field("wrtcie", &self.wrtcie())
                .field("d0rtcie", &self.d0rtcie())
                .field("d1rtcie", &self.d1rtcie())
                .field("hdvposcrctcie", &self.hdvposcrctcie())
                .field("hdvpostcie", &self.hdvpostcie())
                .field("hdveltcie", &self.hdveltcie())
                .field("hdfpostcie", &self.hdfpostcie())
                .field("hdfposie", &self.hdfposie())
                .field("hdvpos1ie", &self.hdvpos1ie())
                .field("hdvpos2ie", &self.hdvpos2ie())
                .field("hdfrelie", &self.hdfrelie())
                .field("hdfresie", &self.hdfresie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ier {{ gtcie: {=bool:?}, wrtcie: {=bool:?}, d0rtcie: {=bool:?}, d1rtcie: {=bool:?}, hdvposcrctcie: {=bool:?}, hdvpostcie: {=bool:?}, hdveltcie: {=bool:?}, hdfpostcie: {=bool:?}, hdfposie: {=bool:?}, hdvpos1ie: {=bool:?}, hdvpos2ie: {=bool:?}, hdfrelie: {=bool:?}, hdfresie: {=bool:?} }}",
                self.gtcie(),
                self.wrtcie(),
                self.d0rtcie(),
                self.d1rtcie(),
                self.hdvposcrctcie(),
                self.hdvpostcie(),
                self.hdveltcie(),
                self.hdfpostcie(),
                self.hdfposie(),
                self.hdvpos1ie(),
                self.hdvpos2ie(),
                self.hdfrelie(),
                self.hdfresie()
            )
        }
    }
    #[doc = "POSENC interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "Global transfer complete flag."]
        #[must_use]
        #[inline(always)]
        pub const fn gtcf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Global transfer complete flag."]
        #[inline(always)]
        pub const fn set_gtcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Write transfer complete flag."]
        #[must_use]
        #[inline(always)]
        pub const fn wrtcf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Write transfer complete flag."]
        #[inline(always)]
        pub const fn set_wrtcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Data 0 read transfer complete flag."]
        #[must_use]
        #[inline(always)]
        pub const fn d0rtcf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Data 0 read transfer complete flag."]
        #[inline(always)]
        pub const fn set_d0rtcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Data 1 read transfer complete flag."]
        #[must_use]
        #[inline(always)]
        pub const fn d1rtcf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Data 1 read transfer complete flag."]
        #[inline(always)]
        pub const fn set_d1rtcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Hiperface-DSL safe position 1 CRC transfer complete interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hdvposcrctcf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Hiperface-DSL safe position 1 CRC transfer complete interrupt flag."]
        #[inline(always)]
        pub const fn set_hdvposcrctcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Hiperface-DSL safe position 1 transfer complete interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hdvposrtcf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Hiperface-DSL safe position 1 transfer complete interrupt flag."]
        #[inline(always)]
        pub const fn set_hdvposrtcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Hiperface-DSL velocity transfer complete interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hdvelrtcf(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Hiperface-DSL velocity transfer complete interrupt flag."]
        #[inline(always)]
        pub const fn set_hdvelrtcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Hiperface-DSL fast position transfer complete interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hdfposrtcf(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Hiperface-DSL fast position transfer complete interrupt flag."]
        #[inline(always)]
        pub const fn set_hdfposrtcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Hiperface-DSL fast position interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hdfposf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Hiperface-DSL fast position interrupt flag."]
        #[inline(always)]
        pub const fn set_hdfposf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Hiperface-DSL safe position 1 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hdvpos1f(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Hiperface-DSL safe position 1 interrupt flag."]
        #[inline(always)]
        pub const fn set_hdvpos1f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Hiperface-DSL safe position 2 interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hdvpos2f(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Hiperface-DSL safe position 2 interrupt flag."]
        #[inline(always)]
        pub const fn set_hdvpos2f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Hiperface-DSL long message interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hdfrelf(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Hiperface-DSL long message interrupt flag."]
        #[inline(always)]
        pub const fn set_hdfrelf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Hiperface-DSL short message interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hdfresf(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Hiperface-DSL short message interrupt flag."]
        #[inline(always)]
        pub const fn set_hdfresf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Isr {
        #[inline(always)]
        fn default() -> Isr {
            Isr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Isr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Isr")
                .field("gtcf", &self.gtcf())
                .field("wrtcf", &self.wrtcf())
                .field("d0rtcf", &self.d0rtcf())
                .field("d1rtcf", &self.d1rtcf())
                .field("hdvposcrctcf", &self.hdvposcrctcf())
                .field("hdvposrtcf", &self.hdvposrtcf())
                .field("hdvelrtcf", &self.hdvelrtcf())
                .field("hdfposrtcf", &self.hdfposrtcf())
                .field("hdfposf", &self.hdfposf())
                .field("hdvpos1f", &self.hdvpos1f())
                .field("hdvpos2f", &self.hdvpos2f())
                .field("hdfrelf", &self.hdfrelf())
                .field("hdfresf", &self.hdfresf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Isr {{ gtcf: {=bool:?}, wrtcf: {=bool:?}, d0rtcf: {=bool:?}, d1rtcf: {=bool:?}, hdvposcrctcf: {=bool:?}, hdvposrtcf: {=bool:?}, hdvelrtcf: {=bool:?}, hdfposrtcf: {=bool:?}, hdfposf: {=bool:?}, hdvpos1f: {=bool:?}, hdvpos2f: {=bool:?}, hdfrelf: {=bool:?}, hdfresf: {=bool:?} }}",
                self.gtcf(),
                self.wrtcf(),
                self.d0rtcf(),
                self.d1rtcf(),
                self.hdvposcrctcf(),
                self.hdvposrtcf(),
                self.hdvelrtcf(),
                self.hdfposrtcf(),
                self.hdfposf(),
                self.hdvpos1f(),
                self.hdvpos2f(),
                self.hdfrelf(),
                self.hdfresf()
            )
        }
    }
    #[doc = "POSENC masked interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Misr(pub u32);
    impl Misr {
        #[doc = "Global transfer complete masked flag."]
        #[must_use]
        #[inline(always)]
        pub const fn gtcmf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Global transfer complete masked flag."]
        #[inline(always)]
        pub const fn set_gtcmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Write transfer complete masked flag."]
        #[must_use]
        #[inline(always)]
        pub const fn wrtcmf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Write transfer complete masked flag."]
        #[inline(always)]
        pub const fn set_wrtcmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Data 0 read transfer complete masked flag."]
        #[must_use]
        #[inline(always)]
        pub const fn d0rtcmf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Data 0 read transfer complete masked flag."]
        #[inline(always)]
        pub const fn set_d0rtcmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Data 1 read transfer complete masked flag."]
        #[must_use]
        #[inline(always)]
        pub const fn d1rtcmf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Data 1 read transfer complete masked flag."]
        #[inline(always)]
        pub const fn set_d1rtcmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Hiperface-DSL safe position 1 CRC transfer complete interrupt masked flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hdvposcrctcmf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Hiperface-DSL safe position 1 CRC transfer complete interrupt masked flag."]
        #[inline(always)]
        pub const fn set_hdvposcrctcmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Hiperface-DSL safe position 1 transfer complete interrupt masked flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hdvposrtcmf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Hiperface-DSL safe position 1 transfer complete interrupt masked flag."]
        #[inline(always)]
        pub const fn set_hdvposrtcmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Hiperface-DSL velocity ready interrupt masked flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hdvelrtcmf(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Hiperface-DSL velocity ready interrupt masked flag."]
        #[inline(always)]
        pub const fn set_hdvelrtcmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Hiperface-DSL fast position 1 ready interrupt masked flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hdfposrtcmf(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Hiperface-DSL fast position 1 ready interrupt masked flag."]
        #[inline(always)]
        pub const fn set_hdfposrtcmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Hiperface-DSL fast position interrupt masked flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hdfposmf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Hiperface-DSL fast position interrupt masked flag."]
        #[inline(always)]
        pub const fn set_hdfposmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Hiperface-DSL safe position 1 interrupt masked flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hdvpos1mf(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Hiperface-DSL safe position 1 interrupt masked flag."]
        #[inline(always)]
        pub const fn set_hdvpos1mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Hiperface-DSL safe position 2 interrupt masked flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hdvpos2mf(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Hiperface-DSL safe position 2 interrupt masked flag."]
        #[inline(always)]
        pub const fn set_hdvpos2mf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Hiperface-DSL long message interrupt masked flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hdfrelmf(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Hiperface-DSL long message interrupt masked flag."]
        #[inline(always)]
        pub const fn set_hdfrelmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Hiperface-DSL short message interrupt masked flag."]
        #[must_use]
        #[inline(always)]
        pub const fn hdfresmf(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Hiperface-DSL short message interrupt masked flag."]
        #[inline(always)]
        pub const fn set_hdfresmf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Misr {
        #[inline(always)]
        fn default() -> Misr {
            Misr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Misr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Misr")
                .field("gtcmf", &self.gtcmf())
                .field("wrtcmf", &self.wrtcmf())
                .field("d0rtcmf", &self.d0rtcmf())
                .field("d1rtcmf", &self.d1rtcmf())
                .field("hdvposcrctcmf", &self.hdvposcrctcmf())
                .field("hdvposrtcmf", &self.hdvposrtcmf())
                .field("hdvelrtcmf", &self.hdvelrtcmf())
                .field("hdfposrtcmf", &self.hdfposrtcmf())
                .field("hdfposmf", &self.hdfposmf())
                .field("hdvpos1mf", &self.hdvpos1mf())
                .field("hdvpos2mf", &self.hdvpos2mf())
                .field("hdfrelmf", &self.hdfrelmf())
                .field("hdfresmf", &self.hdfresmf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Misr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Misr {{ gtcmf: {=bool:?}, wrtcmf: {=bool:?}, d0rtcmf: {=bool:?}, d1rtcmf: {=bool:?}, hdvposcrctcmf: {=bool:?}, hdvposrtcmf: {=bool:?}, hdvelrtcmf: {=bool:?}, hdfposrtcmf: {=bool:?}, hdfposmf: {=bool:?}, hdvpos1mf: {=bool:?}, hdvpos2mf: {=bool:?}, hdfrelmf: {=bool:?}, hdfresmf: {=bool:?} }}",
                self.gtcmf(),
                self.wrtcmf(),
                self.d0rtcmf(),
                self.d1rtcmf(),
                self.hdvposcrctcmf(),
                self.hdvposrtcmf(),
                self.hdvelrtcmf(),
                self.hdfposrtcmf(),
                self.hdfposmf(),
                self.hdvpos1mf(),
                self.hdvpos2mf(),
                self.hdfrelmf(),
                self.hdfresmf()
            )
        }
    }
    #[doc = "POSENC status register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr1(pub u32);
    impl Sr1 {
        #[doc = "Global busy."]
        #[must_use]
        #[inline(always)]
        pub const fn gbsy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Global busy."]
        #[inline(always)]
        pub const fn set_gbsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Write busy."]
        #[must_use]
        #[inline(always)]
        pub const fn wrbsy(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Write busy."]
        #[inline(always)]
        pub const fn set_wrbsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Data 0 read busy."]
        #[must_use]
        #[inline(always)]
        pub const fn d0rdbsy(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Data 0 read busy."]
        #[inline(always)]
        pub const fn set_d0rdbsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Data 1 read busy."]
        #[must_use]
        #[inline(always)]
        pub const fn d1rdbsy(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Data 1 read busy."]
        #[inline(always)]
        pub const fn set_d1rdbsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Hiperface-DSL interrupt line status."]
        #[must_use]
        #[inline(always)]
        pub const fn hdint(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Hiperface-DSL interrupt line status."]
        #[inline(always)]
        pub const fn set_hdint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Hiperface-DSL interrupt_s line status."]
        #[must_use]
        #[inline(always)]
        pub const fn hdints(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Hiperface-DSL interrupt_s line status."]
        #[inline(always)]
        pub const fn set_hdints(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Sr1 {
        #[inline(always)]
        fn default() -> Sr1 {
            Sr1(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Sr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr1")
                .field("gbsy", &self.gbsy())
                .field("wrbsy", &self.wrbsy())
                .field("d0rdbsy", &self.d0rdbsy())
                .field("d1rdbsy", &self.d1rdbsy())
                .field("hdint", &self.hdint())
                .field("hdints", &self.hdints())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr1 {{ gbsy: {=bool:?}, wrbsy: {=bool:?}, d0rdbsy: {=bool:?}, d1rdbsy: {=bool:?}, hdint: {=bool:?}, hdints: {=bool:?} }}",
                self.gbsy(),
                self.wrbsy(),
                self.d0rdbsy(),
                self.d1rdbsy(),
                self.hdint(),
                self.hdints()
            )
        }
    }
    #[doc = "POSENC status register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr2(pub u32);
    impl Sr2 {
        #[doc = "Online status D interface."]
        #[must_use]
        #[inline(always)]
        pub const fn onlinestatusd(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Online status D interface."]
        #[inline(always)]
        pub const fn set_onlinestatusd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Online status safe 1 interface."]
        #[must_use]
        #[inline(always)]
        pub const fn onlinestatus1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Online status safe 1 interface."]
        #[inline(always)]
        pub const fn set_onlinestatus1(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Sr2 {
        #[inline(always)]
        fn default() -> Sr2 {
            Sr2(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Sr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr2")
                .field("onlinestatusd", &self.onlinestatusd())
                .field("onlinestatus1", &self.onlinestatus1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr2 {{ onlinestatusd: {=u16:?}, onlinestatus1: {=u16:?} }}",
                self.onlinestatusd(),
                self.onlinestatus1()
            )
        }
    }
    #[doc = "POSENC status register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr3(pub u32);
    impl Sr3 {
        #[doc = "Online status safe 2 interface."]
        #[must_use]
        #[inline(always)]
        pub const fn onlinestatus2(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Online status safe 2 interface."]
        #[inline(always)]
        pub const fn set_onlinestatus2(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Sr3 {
        #[inline(always)]
        fn default() -> Sr3 {
            Sr3(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Sr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr3")
                .field("onlinestatus2", &self.onlinestatus2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Sr3 {{ onlinestatus2: {=u16:?} }}", self.onlinestatus2())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Encsel {
        #[doc = "EnDat3."]
        EnDat3 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "Hiperface-DSL."]
        HiperfaceDsl = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        #[doc = "No protocol selected."]
        None = 0x0f,
    }
    impl Encsel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Encsel {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Encsel {
        #[inline(always)]
        fn from(val: u8) -> Encsel {
            Encsel::from_bits(val)
        }
    }
    impl From<Encsel> for u8 {
        #[inline(always)]
        fn from(val: Encsel) -> u8 {
            Encsel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Hdpos {
        #[doc = "No reading."]
        Disabled = 0x0,
        #[doc = "Single-turn 17-bit reading."]
        Bits17 = 0x01,
        #[doc = "Single-turn 17-bit / multi-turn 12-bit reading."]
        Bits17multi12 = 0x02,
        #[doc = "Single-turn 23-bit / multi-turn 12-bit reading."]
        Bits23multi12 = 0x03,
    }
    impl Hdpos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hdpos {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hdpos {
        #[inline(always)]
        fn from(val: u8) -> Hdpos {
            Hdpos::from_bits(val)
        }
    }
    impl From<Hdpos> for u8 {
        #[inline(always)]
        fn from(val: Hdpos) -> u8 {
            Hdpos::to_bits(val)
        }
    }
}
