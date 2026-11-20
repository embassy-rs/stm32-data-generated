#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "SBS register block"]
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
    #[doc = "SBS temporal isolation control register"]
    #[inline(always)]
    pub const fn hdplcr(self) -> crate::common::Reg<regs::Hdplcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "SBS temporal isolation status register"]
    #[inline(always)]
    pub const fn hdplsr(self) -> crate::common::Reg<regs::Hdplsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "SBS next HDPL control register"]
    #[inline(always)]
    pub const fn nexthdplcr(self) -> crate::common::Reg<regs::Nexthdplcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "SBS debug control register"]
    #[inline(always)]
    pub const fn dbgcr(self) -> crate::common::Reg<regs::Dbgcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "SBS debug lock register"]
    #[inline(always)]
    pub const fn dbglockr(self) -> crate::common::Reg<regs::Dbglockr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "SBS debug control register 2"]
    #[inline(always)]
    pub const fn dbgcr2(self) -> crate::common::Reg<regs::Dbgcr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "SBS RSS command register"]
    #[inline(always)]
    pub const fn rsscmdr(self) -> crate::common::Reg<regs::Rsscmdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "SBS EPOCH selection control register"]
    #[inline(always)]
    pub const fn epochselcr(self) -> crate::common::Reg<regs::Epochselcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "SBS security mode configuration control register"]
    #[inline(always)]
    pub const fn seccfgr(self) -> crate::common::Reg<regs::Seccfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "SBS product mode and configuration register"]
    #[inline(always)]
    pub const fn pmcr(self) -> crate::common::Reg<regs::Pmcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "SBS FPU interrupt mask register"]
    #[inline(always)]
    pub const fn fpuimr(self) -> crate::common::Reg<regs::Fpuimr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "SBS memory erase status register"]
    #[inline(always)]
    pub const fn mesr(self) -> crate::common::Reg<regs::Mesr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "SBS compensation cell for I/Os control and status register"]
    #[inline(always)]
    pub const fn cccsr(self) -> crate::common::Reg<regs::Cccsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "SBS compensation cell for I/Os value register"]
    #[inline(always)]
    pub const fn ccvalr(self) -> crate::common::Reg<regs::Ccvalr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "SBS compensation cell for I/Os software code register"]
    #[inline(always)]
    pub const fn ccswcr(self) -> crate::common::Reg<regs::Ccswcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "SBS Class B register"]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Cfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "SBS CPU non-secure lock register"]
    #[inline(always)]
    pub const fn cnslckr(self) -> crate::common::Reg<regs::Cnslckr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "SBS CPU secure lock register"]
    #[inline(always)]
    pub const fn cslckr(self) -> crate::common::Reg<regs::Cslckr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "SBS FLASH ECC NMI mask register"]
    #[inline(always)]
    pub const fn eccnmir(self) -> crate::common::Reg<regs::Eccnmir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "SBS flash bank swap lock register"]
    #[inline(always)]
    pub const fn swaplockr(self) -> crate::common::Reg<regs::Swaplockr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "SBS flash bank mode register"]
    #[inline(always)]
    pub const fn fbmoder(self) -> crate::common::Reg<regs::Fbmoder, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
}
pub mod regs {
    #[doc = "SBS compensation cell for I/Os control and status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cccsr(pub u32);
    impl Cccsr {
        #[doc = "Enable compensation cell for VDD power rail."]
        #[must_use]
        #[inline(always)]
        pub const fn en1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable compensation cell for VDD power rail."]
        #[inline(always)]
        pub const fn set_en1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Code selection for VDD power rail (reset value set to 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn cs1(&self) -> super::vals::Cs {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Cs::from_bits(val as u8)
        }
        #[doc = "Code selection for VDD power rail (reset value set to 1)."]
        #[inline(always)]
        pub const fn set_cs1(&mut self, val: super::vals::Cs) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "VDD compensation cell ready flag."]
        #[must_use]
        #[inline(always)]
        pub const fn rdy1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "VDD compensation cell ready flag."]
        #[inline(always)]
        pub const fn set_rdy1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Cccsr {
        #[inline(always)]
        fn default() -> Cccsr {
            Cccsr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Cccsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cccsr")
                .field("en1", &self.en1())
                .field("cs1", &self.cs1())
                .field("rdy1", &self.rdy1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cccsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cccsr {{ en1: {=bool:?}, cs1: {:?}, rdy1: {=bool:?} }}",
                self.en1(),
                self.cs1(),
                self.rdy1()
            )
        }
    }
    #[doc = "SBS compensation cell for I/Os software code register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccswcr(pub u32);
    impl Ccswcr {
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
        #[doc = "PMOS compensation code for VDD power rails."]
        #[must_use]
        #[inline(always)]
        pub const fn sw_apsrc1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "PMOS compensation code for VDD power rails."]
        #[inline(always)]
        pub const fn set_sw_apsrc1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Ccswcr {
        #[inline(always)]
        fn default() -> Ccswcr {
            Ccswcr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Ccswcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccswcr")
                .field("sw_ansrc1", &self.sw_ansrc1())
                .field("sw_apsrc1", &self.sw_apsrc1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccswcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccswcr {{ sw_ansrc1: {=u8:?}, sw_apsrc1: {=u8:?} }}",
                self.sw_ansrc1(),
                self.sw_apsrc1()
            )
        }
    }
    #[doc = "SBS compensation cell for I/Os value register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccvalr(pub u32);
    impl Ccvalr {
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
    impl Default for Ccvalr {
        #[inline(always)]
        fn default() -> Ccvalr {
            Ccvalr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Ccvalr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccvalr")
                .field("ansrc1", &self.ansrc1())
                .field("apsrc1", &self.apsrc1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccvalr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccvalr {{ ansrc1: {=u8:?}, apsrc1: {=u8:?} }}",
                self.ansrc1(),
                self.apsrc1()
            )
        }
    }
    #[doc = "SBS Class B register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr2(pub u32);
    impl Cfgr2 {
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
        pub const fn sel(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM ECC error lock."]
        #[inline(always)]
        pub const fn set_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PVD lock."]
        #[must_use]
        #[inline(always)]
        pub const fn pvdl(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PVD lock."]
        #[inline(always)]
        pub const fn set_pvdl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Flash ECC lock."]
        #[must_use]
        #[inline(always)]
        pub const fn eccl(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Flash ECC lock."]
        #[inline(always)]
        pub const fn set_eccl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Cortex-M33 D-CACHE ECC lock."]
        #[must_use]
        #[inline(always)]
        pub const fn dceccl(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Cortex-M33 D-CACHE ECC lock."]
        #[inline(always)]
        pub const fn set_dceccl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Cfgr2 {
        #[inline(always)]
        fn default() -> Cfgr2 {
            Cfgr2(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Cfgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr2")
                .field("cll", &self.cll())
                .field("sel", &self.sel())
                .field("pvdl", &self.pvdl())
                .field("eccl", &self.eccl())
                .field("dceccl", &self.dceccl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cfgr2 {{ cll: {=bool:?}, sel: {=bool:?}, pvdl: {=bool:?}, eccl: {=bool:?}, dceccl: {=bool:?} }}",
                self.cll(),
                self.sel(),
                self.pvdl(),
                self.eccl(),
                self.dceccl()
            )
        }
    }
    #[doc = "SBS CPU non-secure lock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cnslckr(pub u32);
    impl Cnslckr {
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
        #[doc = "Non-secure MPU register lock."]
        #[must_use]
        #[inline(always)]
        pub const fn locknsmpu(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure MPU register lock."]
        #[inline(always)]
        pub const fn set_locknsmpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Cnslckr {
        #[inline(always)]
        fn default() -> Cnslckr {
            Cnslckr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Cnslckr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cnslckr")
                .field("locknsvtor", &self.locknsvtor())
                .field("locknsmpu", &self.locknsmpu())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cnslckr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cnslckr {{ locknsvtor: {=bool:?}, locknsmpu: {=bool:?} }}",
                self.locknsvtor(),
                self.locknsmpu()
            )
        }
    }
    #[doc = "SBS CPU secure lock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cslckr(pub u32);
    impl Cslckr {
        #[doc = "VTOR_S and AIRCR register lock."]
        #[must_use]
        #[inline(always)]
        pub const fn locksvtaircr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "VTOR_S and AIRCR register lock."]
        #[inline(always)]
        pub const fn set_locksvtaircr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Secure MPU registers lock."]
        #[must_use]
        #[inline(always)]
        pub const fn locksmpu(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Secure MPU registers lock."]
        #[inline(always)]
        pub const fn set_locksmpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SAU registers lock."]
        #[must_use]
        #[inline(always)]
        pub const fn locksau(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SAU registers lock."]
        #[inline(always)]
        pub const fn set_locksau(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
    }
    impl Default for Cslckr {
        #[inline(always)]
        fn default() -> Cslckr {
            Cslckr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Cslckr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cslckr")
                .field("locksvtaircr", &self.locksvtaircr())
                .field("locksmpu", &self.locksmpu())
                .field("locksau", &self.locksau())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cslckr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cslckr {{ locksvtaircr: {=bool:?}, locksmpu: {=bool:?}, locksau: {=bool:?} }}",
                self.locksvtaircr(),
                self.locksmpu(),
                self.locksau()
            )
        }
    }
    #[doc = "SBS debug control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dbgcr(pub u32);
    impl Dbgcr {
        #[doc = "Access port unlock. Write 0xB4 to open the device access port."]
        #[must_use]
        #[inline(always)]
        pub const fn ap_unlock(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Access port unlock. Write 0xB4 to open the device access port."]
        #[inline(always)]
        pub const fn set_ap_unlock(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Debug unlock when DBG_AUTH_HDPL is reached. Write 0xB4 to open debug."]
        #[must_use]
        #[inline(always)]
        pub const fn dbg_unlock(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Debug unlock when DBG_AUTH_HDPL is reached. Write 0xB4 to open debug."]
        #[inline(always)]
        pub const fn set_dbg_unlock(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Authenticated debug temporal isolation level."]
        #[must_use]
        #[inline(always)]
        pub const fn dbg_auth_hdpl(&self) -> super::vals::DbgAuthHdpl {
            let val = (self.0 >> 16usize) & 0xff;
            super::vals::DbgAuthHdpl::from_bits(val as u8)
        }
        #[doc = "Authenticated debug temporal isolation level."]
        #[inline(always)]
        pub const fn set_dbg_auth_hdpl(&mut self, val: super::vals::DbgAuthHdpl) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
        }
        #[doc = "Control debug opening secure/non-secure. Write 0xB4 to open secure and non-secure."]
        #[must_use]
        #[inline(always)]
        pub const fn dbg_auth_sec(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Control debug opening secure/non-secure. Write 0xB4 to open secure and non-secure."]
        #[inline(always)]
        pub const fn set_dbg_auth_sec(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Dbgcr {
        #[inline(always)]
        fn default() -> Dbgcr {
            Dbgcr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dbgcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dbgcr")
                .field("ap_unlock", &self.ap_unlock())
                .field("dbg_unlock", &self.dbg_unlock())
                .field("dbg_auth_hdpl", &self.dbg_auth_hdpl())
                .field("dbg_auth_sec", &self.dbg_auth_sec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dbgcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dbgcr {{ ap_unlock: {=u8:?}, dbg_unlock: {=u8:?}, dbg_auth_hdpl: {:?}, dbg_auth_sec: {=u8:?} }}",
                self.ap_unlock(),
                self.dbg_unlock(),
                self.dbg_auth_hdpl(),
                self.dbg_auth_sec()
            )
        }
    }
    #[doc = "SBS debug control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dbgcr2(pub u32);
    impl Dbgcr2 {
        #[doc = "Secure AHB access port unlock."]
        #[must_use]
        #[inline(always)]
        pub const fn sec_ap_unlock(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Secure AHB access port unlock."]
        #[inline(always)]
        pub const fn set_sec_ap_unlock(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Dbgcr2 {
        #[inline(always)]
        fn default() -> Dbgcr2 {
            Dbgcr2(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dbgcr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dbgcr2")
                .field("sec_ap_unlock", &self.sec_ap_unlock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dbgcr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dbgcr2 {{ sec_ap_unlock: {=u8:?} }}", self.sec_ap_unlock())
        }
    }
    #[doc = "SBS debug lock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dbglockr(pub u32);
    impl Dbglockr {
        #[doc = "Debug configuration lock."]
        #[must_use]
        #[inline(always)]
        pub const fn dbgcfg_lock(&self) -> super::vals::DbgcfgLock {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::DbgcfgLock::from_bits(val as u8)
        }
        #[doc = "Debug configuration lock."]
        #[inline(always)]
        pub const fn set_dbgcfg_lock(&mut self, val: super::vals::DbgcfgLock) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Dbglockr {
        #[inline(always)]
        fn default() -> Dbglockr {
            Dbglockr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dbglockr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dbglockr")
                .field("dbgcfg_lock", &self.dbgcfg_lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dbglockr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dbglockr {{ dbgcfg_lock: {:?} }}", self.dbgcfg_lock())
        }
    }
    #[doc = "SBS FLASH ECC NMI mask register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccnmir(pub u32);
    impl Eccnmir {
        #[doc = "NMI behavior setup when a double ECC error occurs on FLASH data part."]
        #[must_use]
        #[inline(always)]
        pub const fn eccnmi_mask_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "NMI behavior setup when a double ECC error occurs on FLASH data part."]
        #[inline(always)]
        pub const fn set_eccnmi_mask_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Eccnmir {
        #[inline(always)]
        fn default() -> Eccnmir {
            Eccnmir(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Eccnmir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccnmir")
                .field("eccnmi_mask_en", &self.eccnmi_mask_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eccnmir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Eccnmir {{ eccnmi_mask_en: {=bool:?} }}", self.eccnmi_mask_en())
        }
    }
    #[doc = "SBS EPOCH selection control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Epochselcr(pub u32);
    impl Epochselcr {
        #[doc = "Select EPOCH value to be sent to the SAES."]
        #[must_use]
        #[inline(always)]
        pub const fn epoch_sel(&self) -> super::vals::EpochSel {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::EpochSel::from_bits(val as u8)
        }
        #[doc = "Select EPOCH value to be sent to the SAES."]
        #[inline(always)]
        pub const fn set_epoch_sel(&mut self, val: super::vals::EpochSel) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Epochselcr {
        #[inline(always)]
        fn default() -> Epochselcr {
            Epochselcr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Epochselcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Epochselcr")
                .field("epoch_sel", &self.epoch_sel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Epochselcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Epochselcr {{ epoch_sel: {:?} }}", self.epoch_sel())
        }
    }
    #[doc = "SBS flash bank mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fbmoder(pub u32);
    impl Fbmoder {
        #[doc = "Flash bank mode selection."]
        #[must_use]
        #[inline(always)]
        pub const fn fb_mode(&self) -> super::vals::FbMode {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::FbMode::from_bits(val as u8)
        }
        #[doc = "Flash bank mode selection."]
        #[inline(always)]
        pub const fn set_fb_mode(&mut self, val: super::vals::FbMode) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Fbmoder {
        #[inline(always)]
        fn default() -> Fbmoder {
            Fbmoder(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Fbmoder {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fbmoder").field("fb_mode", &self.fb_mode()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fbmoder {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Fbmoder {{ fb_mode: {:?} }}", self.fb_mode())
        }
    }
    #[doc = "SBS FPU interrupt mask register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fpuimr(pub u32);
    impl Fpuimr {
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
    impl Default for Fpuimr {
        #[inline(always)]
        fn default() -> Fpuimr {
            Fpuimr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Fpuimr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fpuimr").field("fpu_ie", &self.fpu_ie()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fpuimr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Fpuimr {{ fpu_ie: {=u8:?} }}", self.fpu_ie())
        }
    }
    #[doc = "SBS temporal isolation control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hdplcr(pub u32);
    impl Hdplcr {
        #[doc = "Increment HDPL value."]
        #[must_use]
        #[inline(always)]
        pub const fn incr_hdpl(&self) -> super::vals::IncrHdpl {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::IncrHdpl::from_bits(val as u8)
        }
        #[doc = "Increment HDPL value."]
        #[inline(always)]
        pub const fn set_incr_hdpl(&mut self, val: super::vals::IncrHdpl) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Hdplcr {
        #[inline(always)]
        fn default() -> Hdplcr {
            Hdplcr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Hdplcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hdplcr").field("incr_hdpl", &self.incr_hdpl()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hdplcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Hdplcr {{ incr_hdpl: {:?} }}", self.incr_hdpl())
        }
    }
    #[doc = "SBS temporal isolation status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hdplsr(pub u32);
    impl Hdplsr {
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
    impl Default for Hdplsr {
        #[inline(always)]
        fn default() -> Hdplsr {
            Hdplsr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Hdplsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hdplsr").field("hdpl", &self.hdpl()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hdplsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Hdplsr {{ hdpl: {:?} }}", self.hdpl())
        }
    }
    #[doc = "SBS memory erase status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mesr(pub u32);
    impl Mesr {
        #[doc = "Device memories erase status."]
        #[must_use]
        #[inline(always)]
        pub const fn mclr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Device memories erase status."]
        #[inline(always)]
        pub const fn set_mclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ICACHE and PKA RAM erase status."]
        #[must_use]
        #[inline(always)]
        pub const fn ipmee(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "ICACHE and PKA RAM erase status."]
        #[inline(always)]
        pub const fn set_ipmee(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Mesr {
        #[inline(always)]
        fn default() -> Mesr {
            Mesr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Mesr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mesr")
                .field("mclr", &self.mclr())
                .field("ipmee", &self.ipmee())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mesr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mesr {{ mclr: {=bool:?}, ipmee: {=bool:?} }}",
                self.mclr(),
                self.ipmee()
            )
        }
    }
    #[doc = "SBS next HDPL control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nexthdplcr(pub u32);
    impl Nexthdplcr {
        #[doc = "Index to point to a higher HDPL than the current one."]
        #[must_use]
        #[inline(always)]
        pub const fn nexthdpl(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Index to point to a higher HDPL than the current one."]
        #[inline(always)]
        pub const fn set_nexthdpl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
    }
    impl Default for Nexthdplcr {
        #[inline(always)]
        fn default() -> Nexthdplcr {
            Nexthdplcr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Nexthdplcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Nexthdplcr")
                .field("nexthdpl", &self.nexthdpl())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Nexthdplcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Nexthdplcr {{ nexthdpl: {=u8:?} }}", self.nexthdpl())
        }
    }
    #[doc = "SBS product mode and configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pmcr(pub u32);
    impl Pmcr {
        #[doc = "SMBus 3.0 on I2C1 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1smb3en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SMBus 3.0 on I2C1 enable."]
        #[inline(always)]
        pub const fn set_i2c1smb3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SMBus 3.0 on I2C3 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c3smb3en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SMBus 3.0 on I2C3 enable."]
        #[inline(always)]
        pub const fn set_i2c3smb3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "SMBus 3.0 on I2C4 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c4smb3en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "SMBus 3.0 on I2C4 enable."]
        #[inline(always)]
        pub const fn set_i2c4smb3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Fast-mode Plus driving capability activation on PB6."]
        #[must_use]
        #[inline(always)]
        pub const fn pb6_fmp(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus driving capability activation on PB6."]
        #[inline(always)]
        pub const fn set_pb6_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Fast-mode Plus driving capability activation on PB7."]
        #[must_use]
        #[inline(always)]
        pub const fn pb7_fmp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus driving capability activation on PB7."]
        #[inline(always)]
        pub const fn set_pb7_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Fast-mode Plus driving capability activation on PB9."]
        #[must_use]
        #[inline(always)]
        pub const fn pb9_fmp(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Fast-mode Plus driving capability activation on PB9."]
        #[inline(always)]
        pub const fn set_pb9_fmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for Pmcr {
        #[inline(always)]
        fn default() -> Pmcr {
            Pmcr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Pmcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pmcr")
                .field("i2c1smb3en", &self.i2c1smb3en())
                .field("i2c3smb3en", &self.i2c3smb3en())
                .field("i2c4smb3en", &self.i2c4smb3en())
                .field("pb6_fmp", &self.pb6_fmp())
                .field("pb7_fmp", &self.pb7_fmp())
                .field("pb9_fmp", &self.pb9_fmp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pmcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pmcr {{ i2c1smb3en: {=bool:?}, i2c3smb3en: {=bool:?}, i2c4smb3en: {=bool:?}, pb6_fmp: {=bool:?}, pb7_fmp: {=bool:?}, pb9_fmp: {=bool:?} }}",
                self.i2c1smb3en(),
                self.i2c3smb3en(),
                self.i2c4smb3en(),
                self.pb6_fmp(),
                self.pb7_fmp(),
                self.pb9_fmp()
            )
        }
    }
    #[doc = "SBS RSS command register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rsscmdr(pub u32);
    impl Rsscmdr {
        #[doc = "RSS command."]
        #[must_use]
        #[inline(always)]
        pub const fn rsscmd(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "RSS command."]
        #[inline(always)]
        pub const fn set_rsscmd(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Rsscmdr {
        #[inline(always)]
        fn default() -> Rsscmdr {
            Rsscmdr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Rsscmdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rsscmdr").field("rsscmd", &self.rsscmd()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rsscmdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Rsscmdr {{ rsscmd: {=u16:?} }}", self.rsscmd())
        }
    }
    #[doc = "SBS security mode configuration control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccfgr(pub u32);
    impl Seccfgr {
        #[doc = "SBS clock control, memory-erase status register and compensation cell register security enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sbssec(&self) -> super::vals::Sec {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Sec::from_bits(val as u8)
        }
        #[doc = "SBS clock control, memory-erase status register and compensation cell register security enable."]
        #[inline(always)]
        pub const fn set_sbssec(&mut self, val: super::vals::Sec) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "ClassB security enable."]
        #[must_use]
        #[inline(always)]
        pub const fn classbsec(&self) -> super::vals::Sec {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Sec::from_bits(val as u8)
        }
        #[doc = "ClassB security enable."]
        #[inline(always)]
        pub const fn set_classbsec(&mut self, val: super::vals::Sec) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "FPU security enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fpusec(&self) -> super::vals::Sec {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Sec::from_bits(val as u8)
        }
        #[doc = "FPU security enable."]
        #[inline(always)]
        pub const fn set_fpusec(&mut self, val: super::vals::Sec) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Seccfgr {
        #[inline(always)]
        fn default() -> Seccfgr {
            Seccfgr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Seccfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Seccfgr")
                .field("sbssec", &self.sbssec())
                .field("classbsec", &self.classbsec())
                .field("fpusec", &self.fpusec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Seccfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Seccfgr {{ sbssec: {:?}, classbsec: {:?}, fpusec: {:?} }}",
                self.sbssec(),
                self.classbsec(),
                self.fpusec()
            )
        }
    }
    #[doc = "SBS flash bank swap lock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Swaplockr(pub u32);
    impl Swaplockr {
        #[doc = "Flash bank swap operation lock."]
        #[must_use]
        #[inline(always)]
        pub const fn swap_lock(&self) -> super::vals::SwapLock {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::SwapLock::from_bits(val as u8)
        }
        #[doc = "Flash bank swap operation lock."]
        #[inline(always)]
        pub const fn set_swap_lock(&mut self, val: super::vals::SwapLock) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Swaplockr {
        #[inline(always)]
        fn default() -> Swaplockr {
            Swaplockr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Swaplockr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Swaplockr")
                .field("swap_lock", &self.swap_lock())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Swaplockr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Swaplockr {{ swap_lock: {:?} }}", self.swap_lock())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cs {
        #[doc = "Code from the cell (available in SBS_CCVALR)"]
        Cell = 0x0,
        #[doc = "Code from SBS_CCSWCR"]
        Software = 0x01,
    }
    impl Cs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cs {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cs {
        #[inline(always)]
        fn from(val: u8) -> Cs {
            Cs::from_bits(val)
        }
    }
    impl From<Cs> for u8 {
        #[inline(always)]
        fn from(val: Cs) -> u8 {
            Cs::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct DbgAuthHdpl(u8);
    impl DbgAuthHdpl {
        #[doc = "HDPL1"]
        pub const Hdpl1: Self = Self(0x51);
        #[doc = "HDPL3"]
        pub const Hdpl3: Self = Self(0x6f);
        #[doc = "HDPL2"]
        pub const Hdpl2: Self = Self(0x8a);
    }
    impl DbgAuthHdpl {
        pub const fn from_bits(val: u8) -> DbgAuthHdpl {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for DbgAuthHdpl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x51 => f.write_str("Hdpl1"),
                0x6f => f.write_str("Hdpl3"),
                0x8a => f.write_str("Hdpl2"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgAuthHdpl {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x51 => defmt::write!(f, "Hdpl1"),
                0x6f => defmt::write!(f, "Hdpl3"),
                0x8a => defmt::write!(f, "Hdpl2"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for DbgAuthHdpl {
        #[inline(always)]
        fn from(val: u8) -> DbgAuthHdpl {
            DbgAuthHdpl::from_bits(val)
        }
    }
    impl From<DbgAuthHdpl> for u8 {
        #[inline(always)]
        fn from(val: DbgAuthHdpl) -> u8 {
            DbgAuthHdpl::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct DbgcfgLock(u8);
    impl DbgcfgLock {
        #[doc = "Writes to SBS_DBGCR and SBS_DBGCR2 allowed (default)"]
        pub const Allowed: Self = Self(0xb4);
    }
    impl DbgcfgLock {
        pub const fn from_bits(val: u8) -> DbgcfgLock {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for DbgcfgLock {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0xb4 => f.write_str("Allowed"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgcfgLock {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0xb4 => defmt::write!(f, "Allowed"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for DbgcfgLock {
        #[inline(always)]
        fn from(val: u8) -> DbgcfgLock {
            DbgcfgLock::from_bits(val)
        }
    }
    impl From<DbgcfgLock> for u8 {
        #[inline(always)]
        fn from(val: DbgcfgLock) -> u8 {
            DbgcfgLock::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum EpochSel {
        #[doc = "NS_EPOCH (non-secure) counter input selected"]
        NsEpoch = 0x0,
        #[doc = "SEC_EPOCH counter input selected"]
        SecEpoch = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl EpochSel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> EpochSel {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for EpochSel {
        #[inline(always)]
        fn from(val: u8) -> EpochSel {
            EpochSel::from_bits(val)
        }
    }
    impl From<EpochSel> for u8 {
        #[inline(always)]
        fn from(val: EpochSel) -> u8 {
            EpochSel::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct FbMode(u8);
    impl FbMode {
        #[doc = "Flash Bank 2 mapped at 0x0800 0000 and Bank 1 at 0x0804 0000"]
        pub const Bank2first: Self = Self(0xb4);
        #[doc = "Flash Bank 1 mapped at 0x0800 0000 and Bank 2 at 0x0804 0000"]
        pub const Bank1first: Self = Self(0xc3);
    }
    impl FbMode {
        pub const fn from_bits(val: u8) -> FbMode {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for FbMode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0xb4 => f.write_str("Bank2first"),
                0xc3 => f.write_str("Bank1first"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for FbMode {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0xb4 => defmt::write!(f, "Bank2first"),
                0xc3 => defmt::write!(f, "Bank1first"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for FbMode {
        #[inline(always)]
        fn from(val: u8) -> FbMode {
            FbMode::from_bits(val)
        }
    }
    impl From<FbMode> for u8 {
        #[inline(always)]
        fn from(val: FbMode) -> u8 {
            FbMode::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Hdpl(u8);
    impl Hdpl {
        #[doc = "HDPL1, iRoT"]
        pub const Hdpl1: Self = Self(0x51);
        #[doc = "HDPL3, application (secure/non-secure)"]
        pub const Hdpl3: Self = Self(0x6f);
        #[doc = "HDPL2, uRoT"]
        pub const Hdpl2: Self = Self(0x8a);
        #[doc = "HDPL0, RSS"]
        pub const Hdpl0: Self = Self(0xb4);
    }
    impl Hdpl {
        pub const fn from_bits(val: u8) -> Hdpl {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Hdpl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x51 => f.write_str("Hdpl1"),
                0x6f => f.write_str("Hdpl3"),
                0x8a => f.write_str("Hdpl2"),
                0xb4 => f.write_str("Hdpl0"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hdpl {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x51 => defmt::write!(f, "Hdpl1"),
                0x6f => defmt::write!(f, "Hdpl3"),
                0x8a => defmt::write!(f, "Hdpl2"),
                0xb4 => defmt::write!(f, "Hdpl0"),
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
        #[doc = "Recommended value to increment HDPL level by one"]
        pub const Increment: Self = Self(0x6a);
        #[doc = "No increment"]
        pub const NoIncrement: Self = Self(0xb4);
    }
    impl IncrHdpl {
        pub const fn from_bits(val: u8) -> IncrHdpl {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for IncrHdpl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x6a => f.write_str("Increment"),
                0xb4 => f.write_str("NoIncrement"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IncrHdpl {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x6a => defmt::write!(f, "Increment"),
                0xb4 => defmt::write!(f, "NoIncrement"),
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Sec {
        #[doc = "Register accessible through secure or non-secure transaction"]
        Any = 0x0,
        #[doc = "Register only accessible through secure transaction"]
        Secure = 0x01,
    }
    impl Sec {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Sec {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Sec {
        #[inline(always)]
        fn from(val: u8) -> Sec {
            Sec::from_bits(val)
        }
    }
    impl From<Sec> for u8 {
        #[inline(always)]
        fn from(val: Sec) -> u8 {
            Sec::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct SwapLock(u8);
    impl SwapLock {
        #[doc = "Flash bank swap operation not allowed"]
        pub const Blocked: Self = Self(0xb4);
        #[doc = "Flash bank swap operation allowed"]
        pub const Allowed: Self = Self(0xc3);
    }
    impl SwapLock {
        pub const fn from_bits(val: u8) -> SwapLock {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for SwapLock {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0xb4 => f.write_str("Blocked"),
                0xc3 => f.write_str("Allowed"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SwapLock {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0xb4 => defmt::write!(f, "Blocked"),
                0xc3 => defmt::write!(f, "Allowed"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for SwapLock {
        #[inline(always)]
        fn from(val: u8) -> SwapLock {
            SwapLock::from_bits(val)
        }
    }
    impl From<SwapLock> for u8 {
        #[inline(always)]
        fn from(val: SwapLock) -> u8 {
            SwapLock::to_bits(val)
        }
    }
}
