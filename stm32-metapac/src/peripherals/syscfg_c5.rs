#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "System configuration controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syscfg {
    ptr: *mut u8,
}
unsafe impl Send for Syscfg {}
unsafe impl Sync for Syscfg {}
impl Syscfg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SBS temporal isolation control register."]
    #[inline(always)]
    pub const fn sbs_hdplcr(self) -> crate::common::Reg<regs::SbsHdplcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "SBS temporal isolation status register."]
    #[inline(always)]
    pub const fn sbs_hdplsr(self) -> crate::common::Reg<regs::SbsHdplsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "SBS next TIL control register."]
    #[inline(always)]
    pub const fn sbs_nexttilcr(self) -> crate::common::Reg<regs::SbsNexttilcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "SBS product mode and configuration register."]
    #[inline(always)]
    pub const fn sbs_pmcr(self) -> crate::common::Reg<regs::SbsPmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "SBS FPU interrupt mask register."]
    #[inline(always)]
    pub const fn sbs_fpuimr(self) -> crate::common::Reg<regs::SbsFpuimr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "SBS memory erase status register."]
    #[inline(always)]
    pub const fn sbs_mesr(self) -> crate::common::Reg<regs::SbsMesr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "SBS compensation cell for I/Os control and status register."]
    #[inline(always)]
    pub const fn sbs_cccsr(self) -> crate::common::Reg<regs::SbsCccsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "SBS compensation cell for I/Os value register."]
    #[inline(always)]
    pub const fn sbs_ccvalr(self) -> crate::common::Reg<regs::SbsCcvalr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "SBS compensation cell for I/Os software code register."]
    #[inline(always)]
    pub const fn sbs_ccswcr(self) -> crate::common::Reg<regs::SbsCcswcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "SBS Class B register."]
    #[inline(always)]
    pub const fn sbs_cfgr2(self) -> crate::common::Reg<regs::SbsCfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "SBS CPU lock register."]
    #[inline(always)]
    pub const fn sbs_clckr(self) -> crate::common::Reg<regs::SbsClckr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "SBS CPU lock register."]
    #[inline(always)]
    pub const fn sbs_cnslckr(self) -> crate::common::Reg<regs::SbsCnslckr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "SBS ECC NMI mask register."]
    #[inline(always)]
    pub const fn sbs_eccnmir(self) -> crate::common::Reg<regs::SbsEccnmir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
}
pub mod regs {
    #[doc = "SBS compensation cell for I/Os control and status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SbsCccsr(pub u32);
    impl SbsCccsr {
        #[doc = "Enable compensation cell for VDDIO power rail."]
        #[must_use]
        #[inline(always)]
        pub const fn en1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable compensation cell for VDDIO power rail."]
        #[inline(always)]
        pub const fn set_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Code selection for VDDIO power rail (reset value set to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn cs1(&self) -> super::vals::Cs1 {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Cs1::from_bits(val as u8)
        }
        #[doc = "Code selection for VDDIO power rail (reset value set to 1)."]
        #[inline(always)]
        pub const fn set_cs1(&mut self, val: super::vals::Cs1) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "VDDIO compensation cell ready flag."]
        #[must_use]
        #[inline(always)]
        pub const fn rdy1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "VDDIO compensation cell ready flag."]
        #[inline(always)]
        pub const fn set_rdy1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for SbsCccsr {
        #[inline(always)]
        fn default() -> SbsCccsr {
            SbsCccsr(0)
        }
    }
    impl core::fmt::Debug for SbsCccsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SbsCccsr")
                .field("en1", &self.en1())
                .field("cs1", &self.cs1())
                .field("rdy1", &self.rdy1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SbsCccsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SbsCccsr {{ en1: {=bool:?}, cs1: {:?}, rdy1: {=bool:?} }}",
                self.en1(),
                self.cs1(),
                self.rdy1()
            )
        }
    }
    #[doc = "SBS compensation cell for I/Os software code register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SbsCcswcr(pub u32);
    impl SbsCcswcr {
        #[doc = "NMOS compensation code for VDD power rails."]
        #[must_use]
        #[inline(always)]
        pub const fn sw_ansrc1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "NMOS compensation code for VDD power rails."]
        #[inline(always)]
        pub const fn set_sw_ansrc1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "PMOS compensation code for the VDD power rails."]
        #[must_use]
        #[inline(always)]
        pub const fn sw_apsrc1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "PMOS compensation code for the VDD power rails."]
        #[inline(always)]
        pub const fn set_sw_apsrc1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for SbsCcswcr {
        #[inline(always)]
        fn default() -> SbsCcswcr {
            SbsCcswcr(0)
        }
    }
    impl core::fmt::Debug for SbsCcswcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SbsCcswcr")
                .field("sw_ansrc1", &self.sw_ansrc1())
                .field("sw_apsrc1", &self.sw_apsrc1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SbsCcswcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SbsCcswcr {{ sw_ansrc1: {=u8:?}, sw_apsrc1: {=u8:?} }}",
                self.sw_ansrc1(),
                self.sw_apsrc1()
            )
        }
    }
    #[doc = "SBS compensation cell for I/Os value register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SbsCcvalr(pub u32);
    impl SbsCcvalr {
        #[doc = "Compensation value for the NMOS transistor."]
        #[must_use]
        #[inline(always)]
        pub const fn ansrc1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Compensation value for the NMOS transistor."]
        #[inline(always)]
        pub const fn set_ansrc1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Compensation value for the PMOS transistor."]
        #[must_use]
        #[inline(always)]
        pub const fn apsrc1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Compensation value for the PMOS transistor."]
        #[inline(always)]
        pub const fn set_apsrc1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for SbsCcvalr {
        #[inline(always)]
        fn default() -> SbsCcvalr {
            SbsCcvalr(0)
        }
    }
    impl core::fmt::Debug for SbsCcvalr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SbsCcvalr")
                .field("ansrc1", &self.ansrc1())
                .field("apsrc1", &self.apsrc1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SbsCcvalr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SbsCcvalr {{ ansrc1: {=u8:?}, apsrc1: {=u8:?} }}",
                self.ansrc1(),
                self.apsrc1()
            )
        }
    }
    #[doc = "SBS Class B register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SbsCfgr2(pub u32);
    impl SbsCfgr2 {
        #[doc = "Core lockup lock."]
        #[must_use]
        #[inline(always)]
        pub const fn cll(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Core lockup lock."]
        #[inline(always)]
        pub const fn set_cll(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SRAM ECC error lock."]
        #[must_use]
        #[inline(always)]
        pub const fn sel(&self) -> super::vals::Sel {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Sel::from_bits(val as u8)
        }
        #[doc = "SRAM ECC error lock."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: super::vals::Sel) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "PVD lock."]
        #[must_use]
        #[inline(always)]
        pub const fn pvdl(&self) -> super::vals::Pvdl {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Pvdl::from_bits(val as u8)
        }
        #[doc = "PVD lock."]
        #[inline(always)]
        pub const fn set_pvdl(&mut self, val: super::vals::Pvdl) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "ECC lock."]
        #[must_use]
        #[inline(always)]
        pub const fn eccl(&self) -> super::vals::Eccl {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Eccl::from_bits(val as u8)
        }
        #[doc = "ECC lock."]
        #[inline(always)]
        pub const fn set_eccl(&mut self, val: super::vals::Eccl) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
    }
    impl Default for SbsCfgr2 {
        #[inline(always)]
        fn default() -> SbsCfgr2 {
            SbsCfgr2(0)
        }
    }
    impl core::fmt::Debug for SbsCfgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SbsCfgr2")
                .field("cll", &self.cll())
                .field("sel", &self.sel())
                .field("pvdl", &self.pvdl())
                .field("eccl", &self.eccl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SbsCfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SbsCfgr2 {{ cll: {=bool:?}, sel: {:?}, pvdl: {:?}, eccl: {:?} }}",
                self.cll(),
                self.sel(),
                self.pvdl(),
                self.eccl()
            )
        }
    }
    #[doc = "SBS CPU lock register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SbsClckr(pub u32);
    impl SbsClckr {
        #[doc = "VTOR register lock."]
        #[must_use]
        #[inline(always)]
        pub const fn lockvtor(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "VTOR register lock."]
        #[inline(always)]
        pub const fn set_lockvtor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "MPU register lock."]
        #[must_use]
        #[inline(always)]
        pub const fn lockmpu(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "MPU register lock."]
        #[inline(always)]
        pub const fn set_lockmpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for SbsClckr {
        #[inline(always)]
        fn default() -> SbsClckr {
            SbsClckr(0)
        }
    }
    impl core::fmt::Debug for SbsClckr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SbsClckr")
                .field("lockvtor", &self.lockvtor())
                .field("lockmpu", &self.lockmpu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SbsClckr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SbsClckr {{ lockvtor: {=bool:?}, lockmpu: {=bool:?} }}",
                self.lockvtor(),
                self.lockmpu()
            )
        }
    }
    #[doc = "SBS CPU lock register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SbsCnslckr(pub u32);
    impl SbsCnslckr {
        #[doc = "VTOR_NS register lock."]
        #[must_use]
        #[inline(always)]
        pub const fn locknsvtor(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "VTOR_NS register lock."]
        #[inline(always)]
        pub const fn set_locknsvtor(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "MPU register lock."]
        #[must_use]
        #[inline(always)]
        pub const fn locknsmpu(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "MPU register lock."]
        #[inline(always)]
        pub const fn set_locknsmpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for SbsCnslckr {
        #[inline(always)]
        fn default() -> SbsCnslckr {
            SbsCnslckr(0)
        }
    }
    impl core::fmt::Debug for SbsCnslckr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SbsCnslckr")
                .field("locknsvtor", &self.locknsvtor())
                .field("locknsmpu", &self.locknsmpu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SbsCnslckr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "SbsCnslckr {{ locknsvtor: {=bool:?}, locknsmpu: {=bool:?} }}",
                self.locknsvtor(),
                self.locknsmpu()
            )
        }
    }
    #[doc = "SBS ECC NMI mask register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SbsEccnmir(pub u32);
    impl SbsEccnmir {
        #[doc = "NMI behavior setup when a double ECC error occurs on FLITF data part."]
        #[must_use]
        #[inline(always)]
        pub const fn eccnmi_mask_en(&self) -> super::vals::EccnmiMaskEn {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::EccnmiMaskEn::from_bits(val as u8)
        }
        #[doc = "NMI behavior setup when a double ECC error occurs on FLITF data part."]
        #[inline(always)]
        pub const fn set_eccnmi_mask_en(&mut self, val: super::vals::EccnmiMaskEn) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
    }
    impl Default for SbsEccnmir {
        #[inline(always)]
        fn default() -> SbsEccnmir {
            SbsEccnmir(0)
        }
    }
    impl core::fmt::Debug for SbsEccnmir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SbsEccnmir")
                .field("eccnmi_mask_en", &self.eccnmi_mask_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SbsEccnmir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SbsEccnmir {{ eccnmi_mask_en: {:?} }}", self.eccnmi_mask_en())
        }
    }
    #[doc = "SBS FPU interrupt mask register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SbsFpuimr(pub u32);
    impl SbsFpuimr {
        #[doc = "FPU interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fpu_ie(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "FPU interrupt enable."]
        #[inline(always)]
        pub const fn set_fpu_ie(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
        }
    }
    impl Default for SbsFpuimr {
        #[inline(always)]
        fn default() -> SbsFpuimr {
            SbsFpuimr(0)
        }
    }
    impl core::fmt::Debug for SbsFpuimr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SbsFpuimr").field("fpu_ie", &self.fpu_ie()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SbsFpuimr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SbsFpuimr {{ fpu_ie: {=u8:?} }}", self.fpu_ie())
        }
    }
    #[doc = "SBS temporal isolation control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SbsHdplcr(pub u32);
    impl SbsHdplcr {
        #[doc = "HDPL value increment."]
        #[must_use]
        #[inline(always)]
        pub const fn incr_hdpl(&self) -> super::vals::IncrHdpl {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::IncrHdpl::from_bits(val as u8)
        }
        #[doc = "HDPL value increment."]
        #[inline(always)]
        pub const fn set_incr_hdpl(&mut self, val: super::vals::IncrHdpl) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
    }
    impl Default for SbsHdplcr {
        #[inline(always)]
        fn default() -> SbsHdplcr {
            SbsHdplcr(0)
        }
    }
    impl core::fmt::Debug for SbsHdplcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SbsHdplcr")
                .field("incr_hdpl", &self.incr_hdpl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SbsHdplcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SbsHdplcr {{ incr_hdpl: {:?} }}", self.incr_hdpl())
        }
    }
    #[doc = "SBS temporal isolation status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SbsHdplsr(pub u32);
    impl SbsHdplsr {
        #[doc = "Temporal isolation level."]
        #[must_use]
        #[inline(always)]
        pub const fn hdpl(&self) -> super::vals::Hdpl {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::Hdpl::from_bits(val as u8)
        }
        #[doc = "Temporal isolation level."]
        #[inline(always)]
        pub const fn set_hdpl(&mut self, val: super::vals::Hdpl) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
    }
    impl Default for SbsHdplsr {
        #[inline(always)]
        fn default() -> SbsHdplsr {
            SbsHdplsr(0)
        }
    }
    impl core::fmt::Debug for SbsHdplsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SbsHdplsr").field("hdpl", &self.hdpl()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SbsHdplsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SbsHdplsr {{ hdpl: {:?} }}", self.hdpl())
        }
    }
    #[doc = "SBS memory erase status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SbsMesr(pub u32);
    impl SbsMesr {
        #[doc = "Device memories erase status."]
        #[must_use]
        #[inline(always)]
        pub const fn mclr(&self) -> super::vals::Mclr {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Mclr::from_bits(val as u8)
        }
        #[doc = "Device memories erase status."]
        #[inline(always)]
        pub const fn set_mclr(&mut self, val: super::vals::Mclr) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "ICACHE erase status."]
        #[must_use]
        #[inline(always)]
        pub const fn ipmee(&self) -> super::vals::Ipmee {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Ipmee::from_bits(val as u8)
        }
        #[doc = "ICACHE erase status."]
        #[inline(always)]
        pub const fn set_ipmee(&mut self, val: super::vals::Ipmee) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
    }
    impl Default for SbsMesr {
        #[inline(always)]
        fn default() -> SbsMesr {
            SbsMesr(0)
        }
    }
    impl core::fmt::Debug for SbsMesr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SbsMesr")
                .field("mclr", &self.mclr())
                .field("ipmee", &self.ipmee())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SbsMesr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SbsMesr {{ mclr: {:?}, ipmee: {:?} }}", self.mclr(), self.ipmee())
        }
    }
    #[doc = "SBS next TIL control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SbsNexttilcr(pub u32);
    impl SbsNexttilcr {
        #[doc = "Next TIL control register."]
        #[must_use]
        #[inline(always)]
        pub const fn nexttil(&self) -> super::vals::Nexttil {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Nexttil::from_bits(val as u8)
        }
        #[doc = "Next TIL control register."]
        #[inline(always)]
        pub const fn set_nexttil(&mut self, val: super::vals::Nexttil) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
    }
    impl Default for SbsNexttilcr {
        #[inline(always)]
        fn default() -> SbsNexttilcr {
            SbsNexttilcr(0)
        }
    }
    impl core::fmt::Debug for SbsNexttilcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SbsNexttilcr")
                .field("nexttil", &self.nexttil())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SbsNexttilcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SbsNexttilcr {{ nexttil: {:?} }}", self.nexttil())
        }
    }
    #[doc = "SBS product mode and configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SbsPmcr(pub u32);
    impl SbsPmcr {
        #[doc = "Fast-mode Plus command on PB(6)."]
        #[must_use]
        #[inline(always)]
        pub const fn pb6_fmplus(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus command on PB(6)."]
        #[inline(always)]
        pub const fn set_pb6_fmplus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Fast-mode Plus command on PB(7)."]
        #[must_use]
        #[inline(always)]
        pub const fn pb7_fmplus(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus command on PB(7)."]
        #[inline(always)]
        pub const fn set_pb7_fmplus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Fast-mode Plus command on PB(8)."]
        #[must_use]
        #[inline(always)]
        pub const fn pb8_fmplus(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus command on PB(8)."]
        #[inline(always)]
        pub const fn set_pb8_fmplus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Fast-mode Plus command on PB(9)."]
        #[must_use]
        #[inline(always)]
        pub const fn pb9_fmplus(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus command on PB(9)."]
        #[inline(always)]
        pub const fn set_pb9_fmplus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Fast-mode Plus command on PA(11)."]
        #[must_use]
        #[inline(always)]
        pub const fn pa11_fmplus(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus command on PA(11)."]
        #[inline(always)]
        pub const fn set_pa11_fmplus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Fast-mode Plus command on PA(12)."]
        #[must_use]
        #[inline(always)]
        pub const fn pa12_fmplus(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus command on PA(12)."]
        #[inline(always)]
        pub const fn set_pa12_fmplus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Fast-mode Plus command on PB(3)."]
        #[must_use]
        #[inline(always)]
        pub const fn pb3_fmplus(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus command on PB(3)."]
        #[inline(always)]
        pub const fn set_pb3_fmplus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Fast-mode Plus command on PB(4)."]
        #[must_use]
        #[inline(always)]
        pub const fn pb4_fmplus(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus command on PB(4)."]
        #[inline(always)]
        pub const fn set_pb4_fmplus(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Ethernet PHY interface selection."]
        #[must_use]
        #[inline(always)]
        pub const fn eth_sel_phy(&self) -> super::vals::EthSelPhy {
            let val = (self.0 >> 24usize) & 0x0f;
            super::vals::EthSelPhy::from_bits(val as u8)
        }
        #[doc = "Ethernet PHY interface selection."]
        #[inline(always)]
        pub const fn set_eth_sel_phy(&mut self, val: super::vals::EthSelPhy) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val.to_bits() as u32) & 0x0f) << 24usize);
        }
        #[doc = "Ethernet external PHY interrupt polarity configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn ethintpol(&self) -> super::vals::Ethintpol {
            let val = (self.0 >> 28usize) & 0x01;
            super::vals::Ethintpol::from_bits(val as u8)
        }
        #[doc = "Ethernet external PHY interrupt polarity configuration."]
        #[inline(always)]
        pub const fn set_ethintpol(&mut self, val: super::vals::Ethintpol) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
        }
        #[doc = "Ethernet power-down acknowledge."]
        #[must_use]
        #[inline(always)]
        pub const fn ethpdack(&self) -> super::vals::Ethpdack {
            let val = (self.0 >> 29usize) & 0x01;
            super::vals::Ethpdack::from_bits(val as u8)
        }
        #[doc = "Ethernet power-down acknowledge."]
        #[inline(always)]
        pub const fn set_ethpdack(&mut self, val: super::vals::Ethpdack) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
        }
        #[doc = "Ethernet TxLPI status."]
        #[must_use]
        #[inline(always)]
        pub const fn ethtxlpi(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet TxLPI status."]
        #[inline(always)]
        pub const fn set_ethtxlpi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for SbsPmcr {
        #[inline(always)]
        fn default() -> SbsPmcr {
            SbsPmcr(0)
        }
    }
    impl core::fmt::Debug for SbsPmcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SbsPmcr")
                .field("pb6_fmplus", &self.pb6_fmplus())
                .field("pb7_fmplus", &self.pb7_fmplus())
                .field("pb8_fmplus", &self.pb8_fmplus())
                .field("pb9_fmplus", &self.pb9_fmplus())
                .field("pa11_fmplus", &self.pa11_fmplus())
                .field("pa12_fmplus", &self.pa12_fmplus())
                .field("pb3_fmplus", &self.pb3_fmplus())
                .field("pb4_fmplus", &self.pb4_fmplus())
                .field("eth_sel_phy", &self.eth_sel_phy())
                .field("ethintpol", &self.ethintpol())
                .field("ethpdack", &self.ethpdack())
                .field("ethtxlpi", &self.ethtxlpi())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SbsPmcr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "SbsPmcr {{ pb6_fmplus: {=bool:?}, pb7_fmplus: {=bool:?}, pb8_fmplus: {=bool:?}, pb9_fmplus: {=bool:?}, pa11_fmplus: {=bool:?}, pa12_fmplus: {=bool:?}, pb3_fmplus: {=bool:?}, pb4_fmplus: {=bool:?}, eth_sel_phy: {:?}, ethintpol: {:?}, ethpdack: {:?}, ethtxlpi: {=bool:?} }}" , self . pb6_fmplus () , self . pb7_fmplus () , self . pb8_fmplus () , self . pb9_fmplus () , self . pa11_fmplus () , self . pa12_fmplus () , self . pb3_fmplus () , self . pb4_fmplus () , self . eth_sel_phy () , self . ethintpol () , self . ethpdack () , self . ethtxlpi ())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cs1 {
        #[doc = "Code from the cell (available in the SBS_CCVR)."]
        B0x0 = 0x0,
        #[doc = "Code from SBS_CCCR."]
        B0x1 = 0x01,
    }
    impl Cs1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cs1 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cs1 {
        #[inline(always)]
        fn from(val: u8) -> Cs1 {
            Cs1::from_bits(val)
        }
    }
    impl From<Cs1> for u8 {
        #[inline(always)]
        fn from(val: Cs1) -> u8 {
            Cs1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Eccl {
        #[doc = "Double ECC error flag disconnected to timer break inputs."]
        B0x0 = 0x0,
        #[doc = "Double ECC error flag connected to timer break inputs."]
        B0x1 = 0x01,
    }
    impl Eccl {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Eccl {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Eccl {
        #[inline(always)]
        fn from(val: u8) -> Eccl {
            Eccl::from_bits(val)
        }
    }
    impl From<Eccl> for u8 {
        #[inline(always)]
        fn from(val: Eccl) -> u8 {
            Eccl::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum EccnmiMaskEn {
        #[doc = "NMI generated if a double ECC error in the FLITF data part."]
        B0x0 = 0x0,
        #[doc = "NMI not generated if a double ECC error in the FLITF data part."]
        B0x1 = 0x01,
    }
    impl EccnmiMaskEn {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> EccnmiMaskEn {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for EccnmiMaskEn {
        #[inline(always)]
        fn from(val: u8) -> EccnmiMaskEn {
            EccnmiMaskEn::from_bits(val)
        }
    }
    impl From<EccnmiMaskEn> for u8 {
        #[inline(always)]
        fn from(val: EccnmiMaskEn) -> u8 {
            EccnmiMaskEn::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum EthSelPhy {
        #[doc = "GMII or MII."]
        B0x0 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "RMII."]
        B0x4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        #[doc = "10BT1S."]
        B0x8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
    }
    impl EthSelPhy {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> EthSelPhy {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for EthSelPhy {
        #[inline(always)]
        fn from(val: u8) -> EthSelPhy {
            EthSelPhy::from_bits(val)
        }
    }
    impl From<EthSelPhy> for u8 {
        #[inline(always)]
        fn from(val: EthSelPhy) -> u8 {
            EthSelPhy::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ethintpol {
        #[doc = "active high."]
        B0x0 = 0x0,
        #[doc = "active low."]
        B0x1 = 0x01,
    }
    impl Ethintpol {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ethintpol {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ethintpol {
        #[inline(always)]
        fn from(val: u8) -> Ethintpol {
            Ethintpol::from_bits(val)
        }
    }
    impl From<Ethintpol> for u8 {
        #[inline(always)]
        fn from(val: Ethintpol) -> u8 {
            Ethintpol::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ethpdack {
        #[doc = "Ethernet functional / power-down sequence not yet completed."]
        B0x0 = 0x0,
        #[doc = "Ethernet power-down sequence completed."]
        B0x1 = 0x01,
    }
    impl Ethpdack {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ethpdack {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ethpdack {
        #[inline(always)]
        fn from(val: u8) -> Ethpdack {
            Ethpdack::from_bits(val)
        }
    }
    impl From<Ethpdack> for u8 {
        #[inline(always)]
        fn from(val: Ethpdack) -> u8 {
            Ethpdack::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hdpl(u8);
    impl Hdpl {
        #[doc = "HDPL1, hide protection level to be used to execute and protect an immutable root of trust (IROT) stage."]
        pub const B0x51: Self = Self(0x51);
        #[doc = "HDPL3, hide protection level to be used to execute the application."]
        pub const B0x6f: Self = Self(0x6f);
        #[doc = "HDPL2, hide protection level to be used to execute and protect an updatable Root of Trust (UROT) stage."]
        pub const B0x8a: Self = Self(0x8a);
    }
    impl Hdpl {
        pub const fn from_bits(val: u8) -> Hdpl {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for Hdpl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x51 => f.write_str("B0x51"),
                0x6f => f.write_str("B0x6f"),
                0x8a => f.write_str("B0x8a"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hdpl {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x51 => defmt::write!(f, "B0x51"),
                0x6f => defmt::write!(f, "B0x6f"),
                0x8a => defmt::write!(f, "B0x8a"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for Hdpl {
        #[inline(always)]
        fn from(val: u8) -> Hdpl {
            Hdpl::from_bits(val)
        }
    }
    impl From<Hdpl> for u8 {
        #[inline(always)]
        fn from(val: Hdpl) -> u8 {
            Hdpl::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct IncrHdpl(u8);
    impl IncrHdpl {
        #[doc = "Recommended value to increment HDPL level by one."]
        pub const B0x6a: Self = Self(0x6a);
        #[doc = "No increment."]
        pub const B0xB4: Self = Self(0xb4);
    }
    impl IncrHdpl {
        pub const fn from_bits(val: u8) -> IncrHdpl {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for IncrHdpl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x6a => f.write_str("B0x6a"),
                0xb4 => f.write_str("B0xB4"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IncrHdpl {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x6a => defmt::write!(f, "B0x6a"),
                0xb4 => defmt::write!(f, "B0xB4"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for IncrHdpl {
        #[inline(always)]
        fn from(val: u8) -> IncrHdpl {
            IncrHdpl::from_bits(val)
        }
    }
    impl From<IncrHdpl> for u8 {
        #[inline(always)]
        fn from(val: IncrHdpl) -> u8 {
            IncrHdpl::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ipmee {
        #[doc = "ICACHE erase ongoing."]
        B0x0 = 0x0,
        #[doc = "ICACHE erase ended."]
        B0x1 = 0x01,
    }
    impl Ipmee {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ipmee {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ipmee {
        #[inline(always)]
        fn from(val: u8) -> Ipmee {
            Ipmee::from_bits(val)
        }
    }
    impl From<Ipmee> for u8 {
        #[inline(always)]
        fn from(val: Ipmee) -> u8 {
            Ipmee::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mclr {
        #[doc = "Memory erase ongoing if not yet cleared by software."]
        B0x0 = 0x0,
        #[doc = "Memory erase done."]
        B0x1 = 0x01,
    }
    impl Mclr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mclr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mclr {
        #[inline(always)]
        fn from(val: u8) -> Mclr {
            Mclr::from_bits(val)
        }
    }
    impl From<Mclr> for u8 {
        #[inline(always)]
        fn from(val: Mclr) -> u8 {
            Mclr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Nexttil {
        #[doc = "OBK-TIL = TIL."]
        B0x0 = 0x0,
        #[doc = "OBK-TIL = TIL + 1."]
        B0x1 = 0x01,
        #[doc = "OBK-TIL = TIL + 2."]
        B0x2 = 0x02,
        #[doc = "OBK-TIL = TIL + 3."]
        B0x3 = 0x03,
    }
    impl Nexttil {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Nexttil {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Nexttil {
        #[inline(always)]
        fn from(val: u8) -> Nexttil {
            Nexttil::from_bits(val)
        }
    }
    impl From<Nexttil> for u8 {
        #[inline(always)]
        fn from(val: Nexttil) -> u8 {
            Nexttil::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pvdl {
        #[doc = "PVD interrupt disconnected from timer break inputs."]
        B0x0 = 0x0,
        #[doc = "PVD interrupt is connected to timer break inputs."]
        B0x1 = 0x01,
    }
    impl Pvdl {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pvdl {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pvdl {
        #[inline(always)]
        fn from(val: u8) -> Pvdl {
            Pvdl::from_bits(val)
        }
    }
    impl From<Pvdl> for u8 {
        #[inline(always)]
        fn from(val: Pvdl) -> u8 {
            Pvdl::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sel {
        #[doc = "SRAM double ECC error flag disconnected from timer break inputs."]
        B0x0 = 0x0,
        #[doc = "SRAM double ECC error flag connected to timer break inputs."]
        B0x1 = 0x01,
    }
    impl Sel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sel {
        #[inline(always)]
        fn from(val: u8) -> Sel {
            Sel::from_bits(val)
        }
    }
    impl From<Sel> for u8 {
        #[inline(always)]
        fn from(val: Sel) -> u8 {
            Sel::to_bits(val)
        }
    }
}
