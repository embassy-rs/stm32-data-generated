#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "RAMCFG register block."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ramcfg {
    ptr: *mut u8,
}
unsafe impl Send for Ramcfg {}
unsafe impl Sync for Ramcfg {}
impl Ramcfg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "RAMCFG memory 1 control register."]
    #[inline(always)]
    pub const fn m1cr(self) -> crate::common::Reg<regs::M1cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "RAMCFG memory 1 interrupt status register."]
    #[inline(always)]
    pub const fn m1isr(self) -> crate::common::Reg<regs::M1isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "RAMCFG memory 1 erase key register."]
    #[inline(always)]
    pub const fn m1erkeyr(self) -> crate::common::Reg<regs::M1erkeyr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "RAMCFG memory 2 control register."]
    #[inline(always)]
    pub const fn m2cr(self) -> crate::common::Reg<regs::M2cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "RAMCFG memory 2 interrupt enable register."]
    #[inline(always)]
    pub const fn m2ier(self) -> crate::common::Reg<regs::M2ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "RAMCFG memory 2 interrupt status register."]
    #[inline(always)]
    pub const fn m2isr(self) -> crate::common::Reg<regs::M2isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "RAMCFG memory 2 ECC single error address register."]
    #[inline(always)]
    pub const fn m2sear(self) -> crate::common::Reg<regs::M2sear, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "RAMCFG memory 2 ECC double error address register."]
    #[inline(always)]
    pub const fn m2dear(self) -> crate::common::Reg<regs::M2dear, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "RAMCFG memory 2 interrupt clear register 2."]
    #[inline(always)]
    pub const fn m2icr(self) -> crate::common::Reg<regs::M2icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "RAMCFG memory 2 write protection register 1."]
    #[inline(always)]
    pub const fn m2wpr1(self) -> crate::common::Reg<regs::M2wpr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "RAMCFG memory 2 write protection register 2."]
    #[inline(always)]
    pub const fn m2wpr2(self) -> crate::common::Reg<regs::M2wpr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "RAMCFG memory 2 ECC key register."]
    #[inline(always)]
    pub const fn m2ecckeyr(self) -> crate::common::Reg<regs::M2ecckeyr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "RAMCFG memory 2 erase key register."]
    #[inline(always)]
    pub const fn m2erkeyr(self) -> crate::common::Reg<regs::M2erkeyr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "RAMCFG memory 2 write protection register 3."]
    #[inline(always)]
    pub const fn m2wpr3(self) -> crate::common::Reg<regs::M2wpr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "RAMCFG memory 2 write protection register 4."]
    #[inline(always)]
    pub const fn m2wpr4(self) -> crate::common::Reg<regs::M2wpr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
}
pub mod regs {
    #[doc = "RAMCFG memory 1 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M1cr(pub u32);
    impl M1cr {
        #[doc = "ECC enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ecce(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ECC enable."]
        #[inline(always)]
        pub const fn set_ecce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Address latch enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ale(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Address latch enable."]
        #[inline(always)]
        pub const fn set_ale(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SRAM erase."]
        #[must_use]
        #[inline(always)]
        pub const fn sramer(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM erase."]
        #[inline(always)]
        pub const fn set_sramer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for M1cr {
        #[inline(always)]
        fn default() -> M1cr {
            M1cr(0)
        }
    }
    impl core::fmt::Debug for M1cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M1cr")
                .field("ecce", &self.ecce())
                .field("ale", &self.ale())
                .field("sramer", &self.sramer())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M1cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M1cr {{ ecce: {=bool:?}, ale: {=bool:?}, sramer: {=bool:?} }}",
                self.ecce(),
                self.ale(),
                self.sramer()
            )
        }
    }
    #[doc = "RAMCFG memory 1 erase key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M1erkeyr(pub u32);
    impl M1erkeyr {
        #[doc = "Erase write protection key."]
        #[must_use]
        #[inline(always)]
        pub const fn erasekey(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Erase write protection key."]
        #[inline(always)]
        pub const fn set_erasekey(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for M1erkeyr {
        #[inline(always)]
        fn default() -> M1erkeyr {
            M1erkeyr(0)
        }
    }
    impl core::fmt::Debug for M1erkeyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M1erkeyr").field("erasekey", &self.erasekey()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M1erkeyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "M1erkeyr {{ erasekey: {=u8:?} }}", self.erasekey())
        }
    }
    #[doc = "RAMCFG memory 1 interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M1isr(pub u32);
    impl M1isr {
        #[doc = "ECC single error detected and corrected."]
        #[must_use]
        #[inline(always)]
        pub const fn sedc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ECC single error detected and corrected."]
        #[inline(always)]
        pub const fn set_sedc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ECC double error detected."]
        #[must_use]
        #[inline(always)]
        pub const fn ded(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ECC double error detected."]
        #[inline(always)]
        pub const fn set_ded(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SRAM busy with erase operation."]
        #[must_use]
        #[inline(always)]
        pub const fn srambusy(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM busy with erase operation."]
        #[inline(always)]
        pub const fn set_srambusy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for M1isr {
        #[inline(always)]
        fn default() -> M1isr {
            M1isr(0)
        }
    }
    impl core::fmt::Debug for M1isr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M1isr")
                .field("sedc", &self.sedc())
                .field("ded", &self.ded())
                .field("srambusy", &self.srambusy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M1isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M1isr {{ sedc: {=bool:?}, ded: {=bool:?}, srambusy: {=bool:?} }}",
                self.sedc(),
                self.ded(),
                self.srambusy()
            )
        }
    }
    #[doc = "RAMCFG memory 2 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M2cr(pub u32);
    impl M2cr {
        #[doc = "ECC enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ecce(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ECC enable."]
        #[inline(always)]
        pub const fn set_ecce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Address latch enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ale(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Address latch enable."]
        #[inline(always)]
        pub const fn set_ale(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SRAM erase."]
        #[must_use]
        #[inline(always)]
        pub const fn sramer(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM erase."]
        #[inline(always)]
        pub const fn set_sramer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for M2cr {
        #[inline(always)]
        fn default() -> M2cr {
            M2cr(0)
        }
    }
    impl core::fmt::Debug for M2cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M2cr")
                .field("ecce", &self.ecce())
                .field("ale", &self.ale())
                .field("sramer", &self.sramer())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M2cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M2cr {{ ecce: {=bool:?}, ale: {=bool:?}, sramer: {=bool:?} }}",
                self.ecce(),
                self.ale(),
                self.sramer()
            )
        }
    }
    #[doc = "RAMCFG memory 2 ECC double error address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M2dear(pub u32);
    impl M2dear {
        #[doc = "ECC double error address."]
        #[must_use]
        #[inline(always)]
        pub const fn edea(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "ECC double error address."]
        #[inline(always)]
        pub const fn set_edea(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for M2dear {
        #[inline(always)]
        fn default() -> M2dear {
            M2dear(0)
        }
    }
    impl core::fmt::Debug for M2dear {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M2dear").field("edea", &self.edea()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M2dear {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "M2dear {{ edea: {=u32:?} }}", self.edea())
        }
    }
    #[doc = "RAMCFG memory 2ECC key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M2ecckeyr(pub u32);
    impl M2ecckeyr {
        #[doc = "ECC write protection key."]
        #[must_use]
        #[inline(always)]
        pub const fn ecckey(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "ECC write protection key."]
        #[inline(always)]
        pub const fn set_ecckey(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for M2ecckeyr {
        #[inline(always)]
        fn default() -> M2ecckeyr {
            M2ecckeyr(0)
        }
    }
    impl core::fmt::Debug for M2ecckeyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M2ecckeyr").field("ecckey", &self.ecckey()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M2ecckeyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "M2ecckeyr {{ ecckey: {=u8:?} }}", self.ecckey())
        }
    }
    #[doc = "RAMCFG memory 2 erase key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M2erkeyr(pub u32);
    impl M2erkeyr {
        #[doc = "Erase write protection key."]
        #[must_use]
        #[inline(always)]
        pub const fn erasekey(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Erase write protection key."]
        #[inline(always)]
        pub const fn set_erasekey(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for M2erkeyr {
        #[inline(always)]
        fn default() -> M2erkeyr {
            M2erkeyr(0)
        }
    }
    impl core::fmt::Debug for M2erkeyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M2erkeyr").field("erasekey", &self.erasekey()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M2erkeyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "M2erkeyr {{ erasekey: {=u8:?} }}", self.erasekey())
        }
    }
    #[doc = "RAMCFG memory 2 interrupt clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M2icr(pub u32);
    impl M2icr {
        #[doc = "Clear ECC single error detected and corrected."]
        #[must_use]
        #[inline(always)]
        pub const fn csedc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ECC single error detected and corrected."]
        #[inline(always)]
        pub const fn set_csedc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear ECC double error detected."]
        #[must_use]
        #[inline(always)]
        pub const fn cded(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ECC double error detected."]
        #[inline(always)]
        pub const fn set_cded(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for M2icr {
        #[inline(always)]
        fn default() -> M2icr {
            M2icr(0)
        }
    }
    impl core::fmt::Debug for M2icr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M2icr")
                .field("csedc", &self.csedc())
                .field("cded", &self.cded())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M2icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M2icr {{ csedc: {=bool:?}, cded: {=bool:?} }}",
                self.csedc(),
                self.cded()
            )
        }
    }
    #[doc = "RAMCFG memory 2 interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M2ier(pub u32);
    impl M2ier {
        #[doc = "ECC single error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn seie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ECC single error interrupt enable."]
        #[inline(always)]
        pub const fn set_seie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ECC double error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn deie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ECC double error interrupt enable."]
        #[inline(always)]
        pub const fn set_deie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Double error NMI."]
        #[must_use]
        #[inline(always)]
        pub const fn eccnmi(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Double error NMI."]
        #[inline(always)]
        pub const fn set_eccnmi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for M2ier {
        #[inline(always)]
        fn default() -> M2ier {
            M2ier(0)
        }
    }
    impl core::fmt::Debug for M2ier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M2ier")
                .field("seie", &self.seie())
                .field("deie", &self.deie())
                .field("eccnmi", &self.eccnmi())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M2ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M2ier {{ seie: {=bool:?}, deie: {=bool:?}, eccnmi: {=bool:?} }}",
                self.seie(),
                self.deie(),
                self.eccnmi()
            )
        }
    }
    #[doc = "RAMCFG memory 2 interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M2isr(pub u32);
    impl M2isr {
        #[doc = "ECC single error detected and corrected."]
        #[must_use]
        #[inline(always)]
        pub const fn sedc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ECC single error detected and corrected."]
        #[inline(always)]
        pub const fn set_sedc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ECC double error detected."]
        #[must_use]
        #[inline(always)]
        pub const fn ded(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ECC double error detected."]
        #[inline(always)]
        pub const fn set_ded(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SRAM busy with erase operation."]
        #[must_use]
        #[inline(always)]
        pub const fn srambusy(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM busy with erase operation."]
        #[inline(always)]
        pub const fn set_srambusy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for M2isr {
        #[inline(always)]
        fn default() -> M2isr {
            M2isr(0)
        }
    }
    impl core::fmt::Debug for M2isr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M2isr")
                .field("sedc", &self.sedc())
                .field("ded", &self.ded())
                .field("srambusy", &self.srambusy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M2isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M2isr {{ sedc: {=bool:?}, ded: {=bool:?}, srambusy: {=bool:?} }}",
                self.sedc(),
                self.ded(),
                self.srambusy()
            )
        }
    }
    #[doc = "RAMCFG memory 2 ECC single error address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M2sear(pub u32);
    impl M2sear {
        #[doc = "ECC single error address."]
        #[must_use]
        #[inline(always)]
        pub const fn esea(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "ECC single error address."]
        #[inline(always)]
        pub const fn set_esea(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for M2sear {
        #[inline(always)]
        fn default() -> M2sear {
            M2sear(0)
        }
    }
    impl core::fmt::Debug for M2sear {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M2sear").field("esea", &self.esea()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M2sear {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "M2sear {{ esea: {=u32:?} }}", self.esea())
        }
    }
    #[doc = "RAMCFG memory 2 write protection register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M2wpr1(pub u32);
    impl M2wpr1 {
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn pwp(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_pwp(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for M2wpr1 {
        #[inline(always)]
        fn default() -> M2wpr1 {
            M2wpr1(0)
        }
    }
    impl core::fmt::Debug for M2wpr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M2wpr1")
                .field("pwp[0]", &self.pwp(0usize))
                .field("pwp[1]", &self.pwp(1usize))
                .field("pwp[2]", &self.pwp(2usize))
                .field("pwp[3]", &self.pwp(3usize))
                .field("pwp[4]", &self.pwp(4usize))
                .field("pwp[5]", &self.pwp(5usize))
                .field("pwp[6]", &self.pwp(6usize))
                .field("pwp[7]", &self.pwp(7usize))
                .field("pwp[8]", &self.pwp(8usize))
                .field("pwp[9]", &self.pwp(9usize))
                .field("pwp[10]", &self.pwp(10usize))
                .field("pwp[11]", &self.pwp(11usize))
                .field("pwp[12]", &self.pwp(12usize))
                .field("pwp[13]", &self.pwp(13usize))
                .field("pwp[14]", &self.pwp(14usize))
                .field("pwp[15]", &self.pwp(15usize))
                .field("pwp[16]", &self.pwp(16usize))
                .field("pwp[17]", &self.pwp(17usize))
                .field("pwp[18]", &self.pwp(18usize))
                .field("pwp[19]", &self.pwp(19usize))
                .field("pwp[20]", &self.pwp(20usize))
                .field("pwp[21]", &self.pwp(21usize))
                .field("pwp[22]", &self.pwp(22usize))
                .field("pwp[23]", &self.pwp(23usize))
                .field("pwp[24]", &self.pwp(24usize))
                .field("pwp[25]", &self.pwp(25usize))
                .field("pwp[26]", &self.pwp(26usize))
                .field("pwp[27]", &self.pwp(27usize))
                .field("pwp[28]", &self.pwp(28usize))
                .field("pwp[29]", &self.pwp(29usize))
                .field("pwp[30]", &self.pwp(30usize))
                .field("pwp[31]", &self.pwp(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M2wpr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M2wpr1 {{ pwp[0]: {=bool:?}, pwp[1]: {=bool:?}, pwp[2]: {=bool:?}, pwp[3]: {=bool:?}, pwp[4]: {=bool:?}, pwp[5]: {=bool:?}, pwp[6]: {=bool:?}, pwp[7]: {=bool:?}, pwp[8]: {=bool:?}, pwp[9]: {=bool:?}, pwp[10]: {=bool:?}, pwp[11]: {=bool:?}, pwp[12]: {=bool:?}, pwp[13]: {=bool:?}, pwp[14]: {=bool:?}, pwp[15]: {=bool:?}, pwp[16]: {=bool:?}, pwp[17]: {=bool:?}, pwp[18]: {=bool:?}, pwp[19]: {=bool:?}, pwp[20]: {=bool:?}, pwp[21]: {=bool:?}, pwp[22]: {=bool:?}, pwp[23]: {=bool:?}, pwp[24]: {=bool:?}, pwp[25]: {=bool:?}, pwp[26]: {=bool:?}, pwp[27]: {=bool:?}, pwp[28]: {=bool:?}, pwp[29]: {=bool:?}, pwp[30]: {=bool:?}, pwp[31]: {=bool:?} }}",
                self.pwp(0usize),
                self.pwp(1usize),
                self.pwp(2usize),
                self.pwp(3usize),
                self.pwp(4usize),
                self.pwp(5usize),
                self.pwp(6usize),
                self.pwp(7usize),
                self.pwp(8usize),
                self.pwp(9usize),
                self.pwp(10usize),
                self.pwp(11usize),
                self.pwp(12usize),
                self.pwp(13usize),
                self.pwp(14usize),
                self.pwp(15usize),
                self.pwp(16usize),
                self.pwp(17usize),
                self.pwp(18usize),
                self.pwp(19usize),
                self.pwp(20usize),
                self.pwp(21usize),
                self.pwp(22usize),
                self.pwp(23usize),
                self.pwp(24usize),
                self.pwp(25usize),
                self.pwp(26usize),
                self.pwp(27usize),
                self.pwp(28usize),
                self.pwp(29usize),
                self.pwp(30usize),
                self.pwp(31usize)
            )
        }
    }
    #[doc = "RAMCFG memory 2 write protection register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M2wpr2(pub u32);
    impl M2wpr2 {
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn pwp(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_pwp(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for M2wpr2 {
        #[inline(always)]
        fn default() -> M2wpr2 {
            M2wpr2(0)
        }
    }
    impl core::fmt::Debug for M2wpr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M2wpr2")
                .field("pwp[0]", &self.pwp(0usize))
                .field("pwp[1]", &self.pwp(1usize))
                .field("pwp[2]", &self.pwp(2usize))
                .field("pwp[3]", &self.pwp(3usize))
                .field("pwp[4]", &self.pwp(4usize))
                .field("pwp[5]", &self.pwp(5usize))
                .field("pwp[6]", &self.pwp(6usize))
                .field("pwp[7]", &self.pwp(7usize))
                .field("pwp[8]", &self.pwp(8usize))
                .field("pwp[9]", &self.pwp(9usize))
                .field("pwp[10]", &self.pwp(10usize))
                .field("pwp[11]", &self.pwp(11usize))
                .field("pwp[12]", &self.pwp(12usize))
                .field("pwp[13]", &self.pwp(13usize))
                .field("pwp[14]", &self.pwp(14usize))
                .field("pwp[15]", &self.pwp(15usize))
                .field("pwp[16]", &self.pwp(16usize))
                .field("pwp[17]", &self.pwp(17usize))
                .field("pwp[18]", &self.pwp(18usize))
                .field("pwp[19]", &self.pwp(19usize))
                .field("pwp[20]", &self.pwp(20usize))
                .field("pwp[21]", &self.pwp(21usize))
                .field("pwp[22]", &self.pwp(22usize))
                .field("pwp[23]", &self.pwp(23usize))
                .field("pwp[24]", &self.pwp(24usize))
                .field("pwp[25]", &self.pwp(25usize))
                .field("pwp[26]", &self.pwp(26usize))
                .field("pwp[27]", &self.pwp(27usize))
                .field("pwp[28]", &self.pwp(28usize))
                .field("pwp[29]", &self.pwp(29usize))
                .field("pwp[30]", &self.pwp(30usize))
                .field("pwp[31]", &self.pwp(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M2wpr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M2wpr2 {{ pwp[0]: {=bool:?}, pwp[1]: {=bool:?}, pwp[2]: {=bool:?}, pwp[3]: {=bool:?}, pwp[4]: {=bool:?}, pwp[5]: {=bool:?}, pwp[6]: {=bool:?}, pwp[7]: {=bool:?}, pwp[8]: {=bool:?}, pwp[9]: {=bool:?}, pwp[10]: {=bool:?}, pwp[11]: {=bool:?}, pwp[12]: {=bool:?}, pwp[13]: {=bool:?}, pwp[14]: {=bool:?}, pwp[15]: {=bool:?}, pwp[16]: {=bool:?}, pwp[17]: {=bool:?}, pwp[18]: {=bool:?}, pwp[19]: {=bool:?}, pwp[20]: {=bool:?}, pwp[21]: {=bool:?}, pwp[22]: {=bool:?}, pwp[23]: {=bool:?}, pwp[24]: {=bool:?}, pwp[25]: {=bool:?}, pwp[26]: {=bool:?}, pwp[27]: {=bool:?}, pwp[28]: {=bool:?}, pwp[29]: {=bool:?}, pwp[30]: {=bool:?}, pwp[31]: {=bool:?} }}",
                self.pwp(0usize),
                self.pwp(1usize),
                self.pwp(2usize),
                self.pwp(3usize),
                self.pwp(4usize),
                self.pwp(5usize),
                self.pwp(6usize),
                self.pwp(7usize),
                self.pwp(8usize),
                self.pwp(9usize),
                self.pwp(10usize),
                self.pwp(11usize),
                self.pwp(12usize),
                self.pwp(13usize),
                self.pwp(14usize),
                self.pwp(15usize),
                self.pwp(16usize),
                self.pwp(17usize),
                self.pwp(18usize),
                self.pwp(19usize),
                self.pwp(20usize),
                self.pwp(21usize),
                self.pwp(22usize),
                self.pwp(23usize),
                self.pwp(24usize),
                self.pwp(25usize),
                self.pwp(26usize),
                self.pwp(27usize),
                self.pwp(28usize),
                self.pwp(29usize),
                self.pwp(30usize),
                self.pwp(31usize)
            )
        }
    }
    #[doc = "RAMCFG memory 2 write protection register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M2wpr3(pub u32);
    impl M2wpr3 {
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn pwp(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_pwp(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for M2wpr3 {
        #[inline(always)]
        fn default() -> M2wpr3 {
            M2wpr3(0)
        }
    }
    impl core::fmt::Debug for M2wpr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M2wpr3")
                .field("pwp[0]", &self.pwp(0usize))
                .field("pwp[1]", &self.pwp(1usize))
                .field("pwp[2]", &self.pwp(2usize))
                .field("pwp[3]", &self.pwp(3usize))
                .field("pwp[4]", &self.pwp(4usize))
                .field("pwp[5]", &self.pwp(5usize))
                .field("pwp[6]", &self.pwp(6usize))
                .field("pwp[7]", &self.pwp(7usize))
                .field("pwp[8]", &self.pwp(8usize))
                .field("pwp[9]", &self.pwp(9usize))
                .field("pwp[10]", &self.pwp(10usize))
                .field("pwp[11]", &self.pwp(11usize))
                .field("pwp[12]", &self.pwp(12usize))
                .field("pwp[13]", &self.pwp(13usize))
                .field("pwp[14]", &self.pwp(14usize))
                .field("pwp[15]", &self.pwp(15usize))
                .field("pwp[16]", &self.pwp(16usize))
                .field("pwp[17]", &self.pwp(17usize))
                .field("pwp[18]", &self.pwp(18usize))
                .field("pwp[19]", &self.pwp(19usize))
                .field("pwp[20]", &self.pwp(20usize))
                .field("pwp[21]", &self.pwp(21usize))
                .field("pwp[22]", &self.pwp(22usize))
                .field("pwp[23]", &self.pwp(23usize))
                .field("pwp[24]", &self.pwp(24usize))
                .field("pwp[25]", &self.pwp(25usize))
                .field("pwp[26]", &self.pwp(26usize))
                .field("pwp[27]", &self.pwp(27usize))
                .field("pwp[28]", &self.pwp(28usize))
                .field("pwp[29]", &self.pwp(29usize))
                .field("pwp[30]", &self.pwp(30usize))
                .field("pwp[31]", &self.pwp(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M2wpr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M2wpr3 {{ pwp[0]: {=bool:?}, pwp[1]: {=bool:?}, pwp[2]: {=bool:?}, pwp[3]: {=bool:?}, pwp[4]: {=bool:?}, pwp[5]: {=bool:?}, pwp[6]: {=bool:?}, pwp[7]: {=bool:?}, pwp[8]: {=bool:?}, pwp[9]: {=bool:?}, pwp[10]: {=bool:?}, pwp[11]: {=bool:?}, pwp[12]: {=bool:?}, pwp[13]: {=bool:?}, pwp[14]: {=bool:?}, pwp[15]: {=bool:?}, pwp[16]: {=bool:?}, pwp[17]: {=bool:?}, pwp[18]: {=bool:?}, pwp[19]: {=bool:?}, pwp[20]: {=bool:?}, pwp[21]: {=bool:?}, pwp[22]: {=bool:?}, pwp[23]: {=bool:?}, pwp[24]: {=bool:?}, pwp[25]: {=bool:?}, pwp[26]: {=bool:?}, pwp[27]: {=bool:?}, pwp[28]: {=bool:?}, pwp[29]: {=bool:?}, pwp[30]: {=bool:?}, pwp[31]: {=bool:?} }}",
                self.pwp(0usize),
                self.pwp(1usize),
                self.pwp(2usize),
                self.pwp(3usize),
                self.pwp(4usize),
                self.pwp(5usize),
                self.pwp(6usize),
                self.pwp(7usize),
                self.pwp(8usize),
                self.pwp(9usize),
                self.pwp(10usize),
                self.pwp(11usize),
                self.pwp(12usize),
                self.pwp(13usize),
                self.pwp(14usize),
                self.pwp(15usize),
                self.pwp(16usize),
                self.pwp(17usize),
                self.pwp(18usize),
                self.pwp(19usize),
                self.pwp(20usize),
                self.pwp(21usize),
                self.pwp(22usize),
                self.pwp(23usize),
                self.pwp(24usize),
                self.pwp(25usize),
                self.pwp(26usize),
                self.pwp(27usize),
                self.pwp(28usize),
                self.pwp(29usize),
                self.pwp(30usize),
                self.pwp(31usize)
            )
        }
    }
    #[doc = "RAMCFG memory 2 write protection register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M2wpr4(pub u32);
    impl M2wpr4 {
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn pwp(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_pwp(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for M2wpr4 {
        #[inline(always)]
        fn default() -> M2wpr4 {
            M2wpr4(0)
        }
    }
    impl core::fmt::Debug for M2wpr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M2wpr4")
                .field("pwp[0]", &self.pwp(0usize))
                .field("pwp[1]", &self.pwp(1usize))
                .field("pwp[2]", &self.pwp(2usize))
                .field("pwp[3]", &self.pwp(3usize))
                .field("pwp[4]", &self.pwp(4usize))
                .field("pwp[5]", &self.pwp(5usize))
                .field("pwp[6]", &self.pwp(6usize))
                .field("pwp[7]", &self.pwp(7usize))
                .field("pwp[8]", &self.pwp(8usize))
                .field("pwp[9]", &self.pwp(9usize))
                .field("pwp[10]", &self.pwp(10usize))
                .field("pwp[11]", &self.pwp(11usize))
                .field("pwp[12]", &self.pwp(12usize))
                .field("pwp[13]", &self.pwp(13usize))
                .field("pwp[14]", &self.pwp(14usize))
                .field("pwp[15]", &self.pwp(15usize))
                .field("pwp[16]", &self.pwp(16usize))
                .field("pwp[17]", &self.pwp(17usize))
                .field("pwp[18]", &self.pwp(18usize))
                .field("pwp[19]", &self.pwp(19usize))
                .field("pwp[20]", &self.pwp(20usize))
                .field("pwp[21]", &self.pwp(21usize))
                .field("pwp[22]", &self.pwp(22usize))
                .field("pwp[23]", &self.pwp(23usize))
                .field("pwp[24]", &self.pwp(24usize))
                .field("pwp[25]", &self.pwp(25usize))
                .field("pwp[26]", &self.pwp(26usize))
                .field("pwp[27]", &self.pwp(27usize))
                .field("pwp[28]", &self.pwp(28usize))
                .field("pwp[29]", &self.pwp(29usize))
                .field("pwp[30]", &self.pwp(30usize))
                .field("pwp[31]", &self.pwp(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M2wpr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M2wpr4 {{ pwp[0]: {=bool:?}, pwp[1]: {=bool:?}, pwp[2]: {=bool:?}, pwp[3]: {=bool:?}, pwp[4]: {=bool:?}, pwp[5]: {=bool:?}, pwp[6]: {=bool:?}, pwp[7]: {=bool:?}, pwp[8]: {=bool:?}, pwp[9]: {=bool:?}, pwp[10]: {=bool:?}, pwp[11]: {=bool:?}, pwp[12]: {=bool:?}, pwp[13]: {=bool:?}, pwp[14]: {=bool:?}, pwp[15]: {=bool:?}, pwp[16]: {=bool:?}, pwp[17]: {=bool:?}, pwp[18]: {=bool:?}, pwp[19]: {=bool:?}, pwp[20]: {=bool:?}, pwp[21]: {=bool:?}, pwp[22]: {=bool:?}, pwp[23]: {=bool:?}, pwp[24]: {=bool:?}, pwp[25]: {=bool:?}, pwp[26]: {=bool:?}, pwp[27]: {=bool:?}, pwp[28]: {=bool:?}, pwp[29]: {=bool:?}, pwp[30]: {=bool:?}, pwp[31]: {=bool:?} }}",
                self.pwp(0usize),
                self.pwp(1usize),
                self.pwp(2usize),
                self.pwp(3usize),
                self.pwp(4usize),
                self.pwp(5usize),
                self.pwp(6usize),
                self.pwp(7usize),
                self.pwp(8usize),
                self.pwp(9usize),
                self.pwp(10usize),
                self.pwp(11usize),
                self.pwp(12usize),
                self.pwp(13usize),
                self.pwp(14usize),
                self.pwp(15usize),
                self.pwp(16usize),
                self.pwp(17usize),
                self.pwp(18usize),
                self.pwp(19usize),
                self.pwp(20usize),
                self.pwp(21usize),
                self.pwp(22usize),
                self.pwp(23usize),
                self.pwp(24usize),
                self.pwp(25usize),
                self.pwp(26usize),
                self.pwp(27usize),
                self.pwp(28usize),
                self.pwp(29usize),
                self.pwp(30usize),
                self.pwp(31usize)
            )
        }
    }
}
