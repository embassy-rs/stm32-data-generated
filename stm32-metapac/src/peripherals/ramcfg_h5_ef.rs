#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "RAMs configuration controller."]
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
    #[doc = "RAMCFG memory interrupt status register."]
    #[inline(always)]
    pub const fn m1isr(self) -> crate::common::Reg<regs::M1isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "RAMCFG memory 1 erase key register."]
    #[inline(always)]
    pub const fn m1erkeyr(self) -> crate::common::Reg<regs::M1erkeyr, crate::common::RW> {
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
    #[doc = "RAMCFG memory interrupt status register."]
    #[inline(always)]
    pub const fn m2isr(self) -> crate::common::Reg<regs::M2isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "RAMCFG memory 2 ECC single error address register."]
    #[inline(always)]
    pub const fn m2sear(self) -> crate::common::Reg<regs::M2sear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "RAMCFG memory 2 ECC double error address register."]
    #[inline(always)]
    pub const fn m2dear(self) -> crate::common::Reg<regs::M2dear, crate::common::RW> {
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
    pub const fn m2ecckeyr(self) -> crate::common::Reg<regs::M2ecckeyr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "RAMCFG memory 2 erase key register."]
    #[inline(always)]
    pub const fn m2erkeyr(self) -> crate::common::Reg<regs::M2erkeyr, crate::common::RW> {
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
    #[doc = "RAMCFG memory 3 control register."]
    #[inline(always)]
    pub const fn m3cr(self) -> crate::common::Reg<regs::M3cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "RAMCFG memory 3 interrupt enable register."]
    #[inline(always)]
    pub const fn m3ier(self) -> crate::common::Reg<regs::M3ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "RAMCFG memory interrupt status register."]
    #[inline(always)]
    pub const fn m3isr(self) -> crate::common::Reg<regs::M3isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "RAMCFG memory 3 ECC single error address register."]
    #[inline(always)]
    pub const fn m3sear(self) -> crate::common::Reg<regs::M3sear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "RAMCFG memory 3 ECC double error address register."]
    #[inline(always)]
    pub const fn m3dear(self) -> crate::common::Reg<regs::M3dear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "RAMCFG memory 3 interrupt clear register 3."]
    #[inline(always)]
    pub const fn m3icr(self) -> crate::common::Reg<regs::M3icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "RAMCFG memory 3 ECC key register."]
    #[inline(always)]
    pub const fn m3ecckeyr(self) -> crate::common::Reg<regs::M3ecckeyr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "RAMCFG memory 3 erase key register."]
    #[inline(always)]
    pub const fn m3erkeyr(self) -> crate::common::Reg<regs::M3erkeyr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "RAMCFG memory 5 control register."]
    #[inline(always)]
    pub const fn m5cr(self) -> crate::common::Reg<regs::M5cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "RAMCFG memory 5 interrupt enable register."]
    #[inline(always)]
    pub const fn m5ier(self) -> crate::common::Reg<regs::M5ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "RAMCFG memory interrupt status register."]
    #[inline(always)]
    pub const fn m5isr(self) -> crate::common::Reg<regs::M5isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "RAMCFG memory 5 ECC single error address register."]
    #[inline(always)]
    pub const fn m5sear(self) -> crate::common::Reg<regs::M5sear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "RAMCFG memory 5 ECC double error address register."]
    #[inline(always)]
    pub const fn m5dear(self) -> crate::common::Reg<regs::M5dear, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "RAMCFG memory 5 interrupt clear register 5."]
    #[inline(always)]
    pub const fn m5icr(self) -> crate::common::Reg<regs::M5icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "RAMCFG memory 5 ECC key register."]
    #[inline(always)]
    pub const fn m5ecckeyr(self) -> crate::common::Reg<regs::M5ecckeyr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "RAMCFG memory 5 erase key register."]
    #[inline(always)]
    pub const fn m5erkeyr(self) -> crate::common::Reg<regs::M5erkeyr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "RAMCFG memory 6 control register."]
    #[inline(always)]
    pub const fn m6cr(self) -> crate::common::Reg<regs::M6cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "RAMCFG memory interrupt status register."]
    #[inline(always)]
    pub const fn m6isr(self) -> crate::common::Reg<regs::M6isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "RAMCFG memory 6 erase key register."]
    #[inline(always)]
    pub const fn m6erkeyr(self) -> crate::common::Reg<regs::M6erkeyr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "RAMCFG memory 7 control register."]
    #[inline(always)]
    pub const fn m7cr(self) -> crate::common::Reg<regs::M7cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "RAMCFG memory interrupt status register."]
    #[inline(always)]
    pub const fn m7isr(self) -> crate::common::Reg<regs::M7isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "RAMCFG memory 7 erase key register."]
    #[inline(always)]
    pub const fn m7erkeyr(self) -> crate::common::Reg<regs::M7erkeyr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
}
pub mod regs {
    #[doc = "RAMCFG memory 1 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M1cr(pub u32);
    impl M1cr {
        #[doc = "ECC enable. This bit reset value is defined by the user option bit configuration. When set, it can be cleared by software only after writing the unlock sequence in the MxECCKEYR register. Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
        #[must_use]
        #[inline(always)]
        pub const fn ecce(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ECC enable. This bit reset value is defined by the user option bit configuration. When set, it can be cleared by software only after writing the unlock sequence in the MxECCKEYR register. Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
        #[inline(always)]
        pub const fn set_ecce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Address latch enable Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
        #[must_use]
        #[inline(always)]
        pub const fn ale(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Address latch enable Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
        #[inline(always)]
        pub const fn set_ale(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SRAM erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the MxERKEYR register. Setting this bit starts the SRAM erase. This bit is automatically cleared by hardware at the end of the erase operation."]
        #[must_use]
        #[inline(always)]
        pub const fn sramer(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the MxERKEYR register. Setting this bit starts the SRAM erase. This bit is automatically cleared by hardware at the end of the erase operation."]
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
        #[doc = "Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the MxCR register. 1) Write 0xCA into ERASEKEY\\[7:0\\]. 2) Write 0x53 into ERASEKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn erasekey(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the MxCR register. 1) Write 0xCA into ERASEKEY\\[7:0\\]. 2) Write 0x53 into ERASEKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
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
    #[doc = "RAMCFG memory interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M1isr(pub u32);
    impl M1isr {
        #[doc = "ECC single error detected and corrected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
        #[must_use]
        #[inline(always)]
        pub const fn sedc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ECC single error detected and corrected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
        #[inline(always)]
        pub const fn set_sedc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ECC double error detected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
        #[must_use]
        #[inline(always)]
        pub const fn ded(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ECC double error detected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
        #[inline(always)]
        pub const fn set_ded(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SRAM busy with erase operation Note: Depending on the SRAM, the erase operation can be performed due to software request, system reset if the option bit is enabled, tamper detection or product state regression. Refer to Table 9: Internal SRAMs features."]
        #[must_use]
        #[inline(always)]
        pub const fn srambusy(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM busy with erase operation Note: Depending on the SRAM, the erase operation can be performed due to software request, system reset if the option bit is enabled, tamper detection or product state regression. Refer to Table 9: Internal SRAMs features."]
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
        #[doc = "ECC enable. This bit reset value is defined by the user option bit configuration. When set, it can be cleared by software only after writing the unlock sequence in the MxECCKEYR register. Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
        #[must_use]
        #[inline(always)]
        pub const fn ecce(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ECC enable. This bit reset value is defined by the user option bit configuration. When set, it can be cleared by software only after writing the unlock sequence in the MxECCKEYR register. Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
        #[inline(always)]
        pub const fn set_ecce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Address latch enable Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
        #[must_use]
        #[inline(always)]
        pub const fn ale(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Address latch enable Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
        #[inline(always)]
        pub const fn set_ale(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SRAM erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the MxERKEYR register. Setting this bit starts the SRAM erase. This bit is automatically cleared by hardware at the end of the erase operation."]
        #[must_use]
        #[inline(always)]
        pub const fn sramer(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the MxERKEYR register. Setting this bit starts the SRAM erase. This bit is automatically cleared by hardware at the end of the erase operation."]
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
        #[doc = "ECC double error address When the ALE bit is set in the MxCR register, this field is updated with the address corresponding to the ECC double error."]
        #[must_use]
        #[inline(always)]
        pub const fn edea(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "ECC double error address When the ALE bit is set in the MxCR register, this field is updated with the address corresponding to the ECC double error."]
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
    #[doc = "RAMCFG memory 2 ECC key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M2ecckeyr(pub u32);
    impl M2ecckeyr {
        #[doc = "ECC write protection key The following steps are required to unlock the write protection of the ECCE bit in the MxCR register. 1) Write 0xAE into ECCKEY\\[7:0\\]. 2) Write 0x75 into ECCKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn ecckey(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "ECC write protection key The following steps are required to unlock the write protection of the ECCE bit in the MxCR register. 1) Write 0xAE into ECCKEY\\[7:0\\]. 2) Write 0x75 into ECCKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
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
        #[doc = "Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the MxCR register. 1) Write 0xCA into ERASEKEY\\[7:0\\]. 2) Write 0x53 into ERASEKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn erasekey(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the MxCR register. 1) Write 0xCA into ERASEKEY\\[7:0\\]. 2) Write 0x53 into ERASEKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
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
    #[doc = "RAMCFG memory 2 interrupt clear register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M2icr(pub u32);
    impl M2icr {
        #[doc = "Clear ECC single error detected and corrected Writing 1 to this flag clears the SEDC bit in the MxISR register. Reading this flag returns the SEDC value."]
        #[must_use]
        #[inline(always)]
        pub const fn csedc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ECC single error detected and corrected Writing 1 to this flag clears the SEDC bit in the MxISR register. Reading this flag returns the SEDC value."]
        #[inline(always)]
        pub const fn set_csedc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear ECC double error detected Writing 1 to this flag clears the DED bit in the MxISR register. Reading this flag returns the DED value."]
        #[must_use]
        #[inline(always)]
        pub const fn cded(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ECC double error detected Writing 1 to this flag clears the DED bit in the MxISR register. Reading this flag returns the DED value."]
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
        #[doc = "Double error NMI This bit is set by software and cleared only by a global RAMCFG reset. Note: if ECCNMI is set, the RAMCFG maskable interrupt is not generated whatever DEIE bit value."]
        #[must_use]
        #[inline(always)]
        pub const fn eccnmi(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Double error NMI This bit is set by software and cleared only by a global RAMCFG reset. Note: if ECCNMI is set, the RAMCFG maskable interrupt is not generated whatever DEIE bit value."]
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
    #[doc = "RAMCFG memory interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M2isr(pub u32);
    impl M2isr {
        #[doc = "ECC single error detected and corrected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
        #[must_use]
        #[inline(always)]
        pub const fn sedc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ECC single error detected and corrected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
        #[inline(always)]
        pub const fn set_sedc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ECC double error detected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
        #[must_use]
        #[inline(always)]
        pub const fn ded(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ECC double error detected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
        #[inline(always)]
        pub const fn set_ded(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SRAM busy with erase operation Note: Depending on the SRAM, the erase operation can be performed due to software request, system reset if the option bit is enabled, tamper detection or product state regression. Refer to Table 9: Internal SRAMs features."]
        #[must_use]
        #[inline(always)]
        pub const fn srambusy(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM busy with erase operation Note: Depending on the SRAM, the erase operation can be performed due to software request, system reset if the option bit is enabled, tamper detection or product state regression. Refer to Table 9: Internal SRAMs features."]
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
        #[doc = "ECC single error address When the ALE bit is set in the MxCR register, this field is updated with the address corresponding to the ECC single error."]
        #[must_use]
        #[inline(always)]
        pub const fn esea(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "ECC single error address When the ALE bit is set in the MxCR register, this field is updated with the address corresponding to the ECC single error."]
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
        #[doc = "SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
        #[must_use]
        #[inline(always)]
        pub const fn pwp(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection These bits are set by software and cleared only by a global RAMCFG reset."]
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
        pub const fn p64wp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p64wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p65wp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p65wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p66wp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p66wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p67wp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p67wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p68wp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p68wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p69wp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p69wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p70wp(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p70wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p71wp(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p71wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p72wp(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p72wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p73wp(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p73wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p74wp(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p74wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p75wp(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p75wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p76wp(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p76wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p77wp(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p77wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p78wp(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p78wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p79wp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p79wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p80wp(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p80wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p81wp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p81wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p82wp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p82wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p83wp(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p83wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p84wp(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p84wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p85wp(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p85wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p86wp(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p86wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p87wp(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p87wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p88wp(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p88wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p89wp(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p89wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p90wp(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p90wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p91wp(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p91wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p92wp(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p92wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p93wp(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p93wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p94wp(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p94wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p95wp(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p95wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("p64wp", &self.p64wp())
                .field("p65wp", &self.p65wp())
                .field("p66wp", &self.p66wp())
                .field("p67wp", &self.p67wp())
                .field("p68wp", &self.p68wp())
                .field("p69wp", &self.p69wp())
                .field("p70wp", &self.p70wp())
                .field("p71wp", &self.p71wp())
                .field("p72wp", &self.p72wp())
                .field("p73wp", &self.p73wp())
                .field("p74wp", &self.p74wp())
                .field("p75wp", &self.p75wp())
                .field("p76wp", &self.p76wp())
                .field("p77wp", &self.p77wp())
                .field("p78wp", &self.p78wp())
                .field("p79wp", &self.p79wp())
                .field("p80wp", &self.p80wp())
                .field("p81wp", &self.p81wp())
                .field("p82wp", &self.p82wp())
                .field("p83wp", &self.p83wp())
                .field("p84wp", &self.p84wp())
                .field("p85wp", &self.p85wp())
                .field("p86wp", &self.p86wp())
                .field("p87wp", &self.p87wp())
                .field("p88wp", &self.p88wp())
                .field("p89wp", &self.p89wp())
                .field("p90wp", &self.p90wp())
                .field("p91wp", &self.p91wp())
                .field("p92wp", &self.p92wp())
                .field("p93wp", &self.p93wp())
                .field("p94wp", &self.p94wp())
                .field("p95wp", &self.p95wp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M2wpr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M2wpr3 {{ p64wp: {=bool:?}, p65wp: {=bool:?}, p66wp: {=bool:?}, p67wp: {=bool:?}, p68wp: {=bool:?}, p69wp: {=bool:?}, p70wp: {=bool:?}, p71wp: {=bool:?}, p72wp: {=bool:?}, p73wp: {=bool:?}, p74wp: {=bool:?}, p75wp: {=bool:?}, p76wp: {=bool:?}, p77wp: {=bool:?}, p78wp: {=bool:?}, p79wp: {=bool:?}, p80wp: {=bool:?}, p81wp: {=bool:?}, p82wp: {=bool:?}, p83wp: {=bool:?}, p84wp: {=bool:?}, p85wp: {=bool:?}, p86wp: {=bool:?}, p87wp: {=bool:?}, p88wp: {=bool:?}, p89wp: {=bool:?}, p90wp: {=bool:?}, p91wp: {=bool:?}, p92wp: {=bool:?}, p93wp: {=bool:?}, p94wp: {=bool:?}, p95wp: {=bool:?} }}",
                self.p64wp(),
                self.p65wp(),
                self.p66wp(),
                self.p67wp(),
                self.p68wp(),
                self.p69wp(),
                self.p70wp(),
                self.p71wp(),
                self.p72wp(),
                self.p73wp(),
                self.p74wp(),
                self.p75wp(),
                self.p76wp(),
                self.p77wp(),
                self.p78wp(),
                self.p79wp(),
                self.p80wp(),
                self.p81wp(),
                self.p82wp(),
                self.p83wp(),
                self.p84wp(),
                self.p85wp(),
                self.p86wp(),
                self.p87wp(),
                self.p88wp(),
                self.p89wp(),
                self.p90wp(),
                self.p91wp(),
                self.p92wp(),
                self.p93wp(),
                self.p94wp(),
                self.p95wp()
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
        pub const fn p96wp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p96wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p97wp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p97wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p98wp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p98wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p99wp(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p99wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p100wp(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p100wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p101wp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p101wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p102wp(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p102wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p103wp(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p103wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p104wp(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p104wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p105wp(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p105wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p106wp(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p106wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p107wp(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p107wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p108wp(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p108wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p109wp(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p109wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p110wp(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p110wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p111wp(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p111wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p112wp(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p112wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p113wp(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p113wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p114wp(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p114wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p115wp(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p115wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p116wp(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p116wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p117wp(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p117wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p118wp(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p118wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p119wp(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p119wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p120wp(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p120wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p121wp(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p121wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p122wp(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p122wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p123wp(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p123wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p124wp(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p124wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p125wp(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p125wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p126wp(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p126wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn p127wp(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 1-Kbyte page y write protection."]
        #[inline(always)]
        pub const fn set_p127wp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("p96wp", &self.p96wp())
                .field("p97wp", &self.p97wp())
                .field("p98wp", &self.p98wp())
                .field("p99wp", &self.p99wp())
                .field("p100wp", &self.p100wp())
                .field("p101wp", &self.p101wp())
                .field("p102wp", &self.p102wp())
                .field("p103wp", &self.p103wp())
                .field("p104wp", &self.p104wp())
                .field("p105wp", &self.p105wp())
                .field("p106wp", &self.p106wp())
                .field("p107wp", &self.p107wp())
                .field("p108wp", &self.p108wp())
                .field("p109wp", &self.p109wp())
                .field("p110wp", &self.p110wp())
                .field("p111wp", &self.p111wp())
                .field("p112wp", &self.p112wp())
                .field("p113wp", &self.p113wp())
                .field("p114wp", &self.p114wp())
                .field("p115wp", &self.p115wp())
                .field("p116wp", &self.p116wp())
                .field("p117wp", &self.p117wp())
                .field("p118wp", &self.p118wp())
                .field("p119wp", &self.p119wp())
                .field("p120wp", &self.p120wp())
                .field("p121wp", &self.p121wp())
                .field("p122wp", &self.p122wp())
                .field("p123wp", &self.p123wp())
                .field("p124wp", &self.p124wp())
                .field("p125wp", &self.p125wp())
                .field("p126wp", &self.p126wp())
                .field("p127wp", &self.p127wp())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M2wpr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M2wpr4 {{ p96wp: {=bool:?}, p97wp: {=bool:?}, p98wp: {=bool:?}, p99wp: {=bool:?}, p100wp: {=bool:?}, p101wp: {=bool:?}, p102wp: {=bool:?}, p103wp: {=bool:?}, p104wp: {=bool:?}, p105wp: {=bool:?}, p106wp: {=bool:?}, p107wp: {=bool:?}, p108wp: {=bool:?}, p109wp: {=bool:?}, p110wp: {=bool:?}, p111wp: {=bool:?}, p112wp: {=bool:?}, p113wp: {=bool:?}, p114wp: {=bool:?}, p115wp: {=bool:?}, p116wp: {=bool:?}, p117wp: {=bool:?}, p118wp: {=bool:?}, p119wp: {=bool:?}, p120wp: {=bool:?}, p121wp: {=bool:?}, p122wp: {=bool:?}, p123wp: {=bool:?}, p124wp: {=bool:?}, p125wp: {=bool:?}, p126wp: {=bool:?}, p127wp: {=bool:?} }}",
                self.p96wp(),
                self.p97wp(),
                self.p98wp(),
                self.p99wp(),
                self.p100wp(),
                self.p101wp(),
                self.p102wp(),
                self.p103wp(),
                self.p104wp(),
                self.p105wp(),
                self.p106wp(),
                self.p107wp(),
                self.p108wp(),
                self.p109wp(),
                self.p110wp(),
                self.p111wp(),
                self.p112wp(),
                self.p113wp(),
                self.p114wp(),
                self.p115wp(),
                self.p116wp(),
                self.p117wp(),
                self.p118wp(),
                self.p119wp(),
                self.p120wp(),
                self.p121wp(),
                self.p122wp(),
                self.p123wp(),
                self.p124wp(),
                self.p125wp(),
                self.p126wp(),
                self.p127wp()
            )
        }
    }
    #[doc = "RAMCFG memory 3 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M3cr(pub u32);
    impl M3cr {
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
    impl Default for M3cr {
        #[inline(always)]
        fn default() -> M3cr {
            M3cr(0)
        }
    }
    impl core::fmt::Debug for M3cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M3cr")
                .field("ecce", &self.ecce())
                .field("ale", &self.ale())
                .field("sramer", &self.sramer())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M3cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M3cr {{ ecce: {=bool:?}, ale: {=bool:?}, sramer: {=bool:?} }}",
                self.ecce(),
                self.ale(),
                self.sramer()
            )
        }
    }
    #[doc = "RAMCFG memory 3 ECC double error address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M3dear(pub u32);
    impl M3dear {
        #[doc = "ECC double error address When the ALE bit is set in the MxCR register, this field is updated with the address corresponding to the ECC double error."]
        #[must_use]
        #[inline(always)]
        pub const fn edea(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "ECC double error address When the ALE bit is set in the MxCR register, this field is updated with the address corresponding to the ECC double error."]
        #[inline(always)]
        pub const fn set_edea(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for M3dear {
        #[inline(always)]
        fn default() -> M3dear {
            M3dear(0)
        }
    }
    impl core::fmt::Debug for M3dear {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M3dear").field("edea", &self.edea()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M3dear {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "M3dear {{ edea: {=u32:?} }}", self.edea())
        }
    }
    #[doc = "RAMCFG memory 3 ECC key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M3ecckeyr(pub u32);
    impl M3ecckeyr {
        #[doc = "ECC write protection key The following steps are required to unlock the write protection of the ECCE bit in the MxCR register. 1) Write 0xAE into ECCKEY\\[7:0\\]. 2) Write 0x75 into ECCKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn ecckey(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "ECC write protection key The following steps are required to unlock the write protection of the ECCE bit in the MxCR register. 1) Write 0xAE into ECCKEY\\[7:0\\]. 2) Write 0x75 into ECCKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
        #[inline(always)]
        pub const fn set_ecckey(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for M3ecckeyr {
        #[inline(always)]
        fn default() -> M3ecckeyr {
            M3ecckeyr(0)
        }
    }
    impl core::fmt::Debug for M3ecckeyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M3ecckeyr").field("ecckey", &self.ecckey()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M3ecckeyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "M3ecckeyr {{ ecckey: {=u8:?} }}", self.ecckey())
        }
    }
    #[doc = "RAMCFG memory 3 erase key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M3erkeyr(pub u32);
    impl M3erkeyr {
        #[doc = "Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the MxCR register. 1) Write 0xCA into ERASEKEY\\[7:0\\]. 2) Write 0x53 into ERASEKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn erasekey(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the MxCR register. 1) Write 0xCA into ERASEKEY\\[7:0\\]. 2) Write 0x53 into ERASEKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
        #[inline(always)]
        pub const fn set_erasekey(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for M3erkeyr {
        #[inline(always)]
        fn default() -> M3erkeyr {
            M3erkeyr(0)
        }
    }
    impl core::fmt::Debug for M3erkeyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M3erkeyr").field("erasekey", &self.erasekey()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M3erkeyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "M3erkeyr {{ erasekey: {=u8:?} }}", self.erasekey())
        }
    }
    #[doc = "RAMCFG memory 3 interrupt clear register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M3icr(pub u32);
    impl M3icr {
        #[doc = "Clear ECC single error detected and corrected Writing 1 to this flag clears the SEDC bit in the MxISR register. Reading this flag returns the SEDC value."]
        #[must_use]
        #[inline(always)]
        pub const fn csedc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ECC single error detected and corrected Writing 1 to this flag clears the SEDC bit in the MxISR register. Reading this flag returns the SEDC value."]
        #[inline(always)]
        pub const fn set_csedc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear ECC double error detected Writing 1 to this flag clears the DED bit in the MxISR register. Reading this flag returns the DED value."]
        #[must_use]
        #[inline(always)]
        pub const fn cded(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ECC double error detected Writing 1 to this flag clears the DED bit in the MxISR register. Reading this flag returns the DED value."]
        #[inline(always)]
        pub const fn set_cded(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for M3icr {
        #[inline(always)]
        fn default() -> M3icr {
            M3icr(0)
        }
    }
    impl core::fmt::Debug for M3icr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M3icr")
                .field("csedc", &self.csedc())
                .field("cded", &self.cded())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M3icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M3icr {{ csedc: {=bool:?}, cded: {=bool:?} }}",
                self.csedc(),
                self.cded()
            )
        }
    }
    #[doc = "RAMCFG memory 3 interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M3ier(pub u32);
    impl M3ier {
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
        #[doc = "Double error NMI This bit is set by software and cleared only by a global RAMCFG reset. Note: if ECCNMI is set, the RAMCFG maskable interrupt is not generated whatever DEIE bit value."]
        #[must_use]
        #[inline(always)]
        pub const fn eccnmi(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Double error NMI This bit is set by software and cleared only by a global RAMCFG reset. Note: if ECCNMI is set, the RAMCFG maskable interrupt is not generated whatever DEIE bit value."]
        #[inline(always)]
        pub const fn set_eccnmi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for M3ier {
        #[inline(always)]
        fn default() -> M3ier {
            M3ier(0)
        }
    }
    impl core::fmt::Debug for M3ier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M3ier")
                .field("seie", &self.seie())
                .field("deie", &self.deie())
                .field("eccnmi", &self.eccnmi())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M3ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M3ier {{ seie: {=bool:?}, deie: {=bool:?}, eccnmi: {=bool:?} }}",
                self.seie(),
                self.deie(),
                self.eccnmi()
            )
        }
    }
    #[doc = "RAMCFG memory interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M3isr(pub u32);
    impl M3isr {
        #[doc = "ECC single error detected and corrected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
        #[must_use]
        #[inline(always)]
        pub const fn sedc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ECC single error detected and corrected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
        #[inline(always)]
        pub const fn set_sedc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ECC double error detected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
        #[must_use]
        #[inline(always)]
        pub const fn ded(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ECC double error detected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
        #[inline(always)]
        pub const fn set_ded(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SRAM busy with erase operation Note: Depending on the SRAM, the erase operation can be performed due to software request, system reset if the option bit is enabled, tamper detection or product state regression. Refer to Table 9: Internal SRAMs features."]
        #[must_use]
        #[inline(always)]
        pub const fn srambusy(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM busy with erase operation Note: Depending on the SRAM, the erase operation can be performed due to software request, system reset if the option bit is enabled, tamper detection or product state regression. Refer to Table 9: Internal SRAMs features."]
        #[inline(always)]
        pub const fn set_srambusy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for M3isr {
        #[inline(always)]
        fn default() -> M3isr {
            M3isr(0)
        }
    }
    impl core::fmt::Debug for M3isr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M3isr")
                .field("sedc", &self.sedc())
                .field("ded", &self.ded())
                .field("srambusy", &self.srambusy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M3isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M3isr {{ sedc: {=bool:?}, ded: {=bool:?}, srambusy: {=bool:?} }}",
                self.sedc(),
                self.ded(),
                self.srambusy()
            )
        }
    }
    #[doc = "RAMCFG memory 3 ECC single error address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M3sear(pub u32);
    impl M3sear {
        #[doc = "ECC single error address When the ALE bit is set in the MxCR register, this field is updated with the address corresponding to the ECC single error."]
        #[must_use]
        #[inline(always)]
        pub const fn esea(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "ECC single error address When the ALE bit is set in the MxCR register, this field is updated with the address corresponding to the ECC single error."]
        #[inline(always)]
        pub const fn set_esea(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for M3sear {
        #[inline(always)]
        fn default() -> M3sear {
            M3sear(0)
        }
    }
    impl core::fmt::Debug for M3sear {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M3sear").field("esea", &self.esea()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M3sear {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "M3sear {{ esea: {=u32:?} }}", self.esea())
        }
    }
    #[doc = "RAMCFG memory 5 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M5cr(pub u32);
    impl M5cr {
        #[doc = "ECC enable. This bit reset value is defined by the user option bit configuration. When set, it can be cleared by software only after writing the unlock sequence in the MxECCKEYR register. Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
        #[must_use]
        #[inline(always)]
        pub const fn ecce(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ECC enable. This bit reset value is defined by the user option bit configuration. When set, it can be cleared by software only after writing the unlock sequence in the MxECCKEYR register. Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
        #[inline(always)]
        pub const fn set_ecce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Address latch enable Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
        #[must_use]
        #[inline(always)]
        pub const fn ale(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Address latch enable Note: This bit is reserved and must be kept at reset value in SRAM1 control register."]
        #[inline(always)]
        pub const fn set_ale(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SRAM erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the MxERKEYR register. Setting this bit starts the SRAM erase. This bit is automatically cleared by hardware at the end of the erase operation."]
        #[must_use]
        #[inline(always)]
        pub const fn sramer(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM erase This bit can be set by software only after writing the unlock sequence in the ERASEKEY field of the MxERKEYR register. Setting this bit starts the SRAM erase. This bit is automatically cleared by hardware at the end of the erase operation."]
        #[inline(always)]
        pub const fn set_sramer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for M5cr {
        #[inline(always)]
        fn default() -> M5cr {
            M5cr(0)
        }
    }
    impl core::fmt::Debug for M5cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M5cr")
                .field("ecce", &self.ecce())
                .field("ale", &self.ale())
                .field("sramer", &self.sramer())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M5cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M5cr {{ ecce: {=bool:?}, ale: {=bool:?}, sramer: {=bool:?} }}",
                self.ecce(),
                self.ale(),
                self.sramer()
            )
        }
    }
    #[doc = "RAMCFG memory 5 ECC double error address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M5dear(pub u32);
    impl M5dear {
        #[doc = "ECC double error address When the ALE bit is set in the MxCR register, this field is updated with the address corresponding to the ECC double error."]
        #[must_use]
        #[inline(always)]
        pub const fn edea(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "ECC double error address When the ALE bit is set in the MxCR register, this field is updated with the address corresponding to the ECC double error."]
        #[inline(always)]
        pub const fn set_edea(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for M5dear {
        #[inline(always)]
        fn default() -> M5dear {
            M5dear(0)
        }
    }
    impl core::fmt::Debug for M5dear {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M5dear").field("edea", &self.edea()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M5dear {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "M5dear {{ edea: {=u32:?} }}", self.edea())
        }
    }
    #[doc = "RAMCFG memory 5 ECC key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M5ecckeyr(pub u32);
    impl M5ecckeyr {
        #[doc = "ECC write protection key The following steps are required to unlock the write protection of the ECCE bit in the MxCR register. 1) Write 0xAE into ECCKEY\\[7:0\\]. 2) Write 0x75 into ECCKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn ecckey(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "ECC write protection key The following steps are required to unlock the write protection of the ECCE bit in the MxCR register. 1) Write 0xAE into ECCKEY\\[7:0\\]. 2) Write 0x75 into ECCKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
        #[inline(always)]
        pub const fn set_ecckey(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for M5ecckeyr {
        #[inline(always)]
        fn default() -> M5ecckeyr {
            M5ecckeyr(0)
        }
    }
    impl core::fmt::Debug for M5ecckeyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M5ecckeyr").field("ecckey", &self.ecckey()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M5ecckeyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "M5ecckeyr {{ ecckey: {=u8:?} }}", self.ecckey())
        }
    }
    #[doc = "RAMCFG memory 5 erase key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M5erkeyr(pub u32);
    impl M5erkeyr {
        #[doc = "Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the MxCR register. 1) Write 0xCA into ERASEKEY\\[7:0\\]. 2) Write 0x53 into ERASEKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
        #[must_use]
        #[inline(always)]
        pub const fn erasekey(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Erase write protection key The following steps are required to unlock the write protection of the SRAMER bit in the MxCR register. 1) Write 0xCA into ERASEKEY\\[7:0\\]. 2) Write 0x53 into ERASEKEY\\[7:0\\]. Note: Writing a wrong key reactivates the write protection."]
        #[inline(always)]
        pub const fn set_erasekey(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for M5erkeyr {
        #[inline(always)]
        fn default() -> M5erkeyr {
            M5erkeyr(0)
        }
    }
    impl core::fmt::Debug for M5erkeyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M5erkeyr").field("erasekey", &self.erasekey()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M5erkeyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "M5erkeyr {{ erasekey: {=u8:?} }}", self.erasekey())
        }
    }
    #[doc = "RAMCFG memory 5 interrupt clear register 5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M5icr(pub u32);
    impl M5icr {
        #[doc = "Clear ECC single error detected and corrected Writing 1 to this flag clears the SEDC bit in the MxISR register. Reading this flag returns the SEDC value."]
        #[must_use]
        #[inline(always)]
        pub const fn csedc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ECC single error detected and corrected Writing 1 to this flag clears the SEDC bit in the MxISR register. Reading this flag returns the SEDC value."]
        #[inline(always)]
        pub const fn set_csedc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear ECC double error detected Writing 1 to this flag clears the DED bit in the MxISR register. Reading this flag returns the DED value."]
        #[must_use]
        #[inline(always)]
        pub const fn cded(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear ECC double error detected Writing 1 to this flag clears the DED bit in the MxISR register. Reading this flag returns the DED value."]
        #[inline(always)]
        pub const fn set_cded(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for M5icr {
        #[inline(always)]
        fn default() -> M5icr {
            M5icr(0)
        }
    }
    impl core::fmt::Debug for M5icr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M5icr")
                .field("csedc", &self.csedc())
                .field("cded", &self.cded())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M5icr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M5icr {{ csedc: {=bool:?}, cded: {=bool:?} }}",
                self.csedc(),
                self.cded()
            )
        }
    }
    #[doc = "RAMCFG memory 5 interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M5ier(pub u32);
    impl M5ier {
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
        #[doc = "Double error NMI This bit is set by software and cleared only by a global RAMCFG reset. Note: if ECCNMI is set, the RAMCFG maskable interrupt is not generated whatever DEIE bit value."]
        #[must_use]
        #[inline(always)]
        pub const fn eccnmi(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Double error NMI This bit is set by software and cleared only by a global RAMCFG reset. Note: if ECCNMI is set, the RAMCFG maskable interrupt is not generated whatever DEIE bit value."]
        #[inline(always)]
        pub const fn set_eccnmi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for M5ier {
        #[inline(always)]
        fn default() -> M5ier {
            M5ier(0)
        }
    }
    impl core::fmt::Debug for M5ier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M5ier")
                .field("seie", &self.seie())
                .field("deie", &self.deie())
                .field("eccnmi", &self.eccnmi())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M5ier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M5ier {{ seie: {=bool:?}, deie: {=bool:?}, eccnmi: {=bool:?} }}",
                self.seie(),
                self.deie(),
                self.eccnmi()
            )
        }
    }
    #[doc = "RAMCFG memory interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M5isr(pub u32);
    impl M5isr {
        #[doc = "ECC single error detected and corrected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
        #[must_use]
        #[inline(always)]
        pub const fn sedc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ECC single error detected and corrected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
        #[inline(always)]
        pub const fn set_sedc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ECC double error detected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
        #[must_use]
        #[inline(always)]
        pub const fn ded(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ECC double error detected Note: This bit is reserved and must be kept at reset value in SRAM1 interrupt status register."]
        #[inline(always)]
        pub const fn set_ded(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SRAM busy with erase operation Note: Depending on the SRAM, the erase operation can be performed due to software request, system reset if the option bit is enabled, tamper detection or product state regression. Refer to Table 9: Internal SRAMs features."]
        #[must_use]
        #[inline(always)]
        pub const fn srambusy(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM busy with erase operation Note: Depending on the SRAM, the erase operation can be performed due to software request, system reset if the option bit is enabled, tamper detection or product state regression. Refer to Table 9: Internal SRAMs features."]
        #[inline(always)]
        pub const fn set_srambusy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for M5isr {
        #[inline(always)]
        fn default() -> M5isr {
            M5isr(0)
        }
    }
    impl core::fmt::Debug for M5isr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M5isr")
                .field("sedc", &self.sedc())
                .field("ded", &self.ded())
                .field("srambusy", &self.srambusy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M5isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M5isr {{ sedc: {=bool:?}, ded: {=bool:?}, srambusy: {=bool:?} }}",
                self.sedc(),
                self.ded(),
                self.srambusy()
            )
        }
    }
    #[doc = "RAMCFG memory 5 ECC single error address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M5sear(pub u32);
    impl M5sear {
        #[doc = "ECC single error address When the ALE bit is set in the MxCR register, this field is updated with the address corresponding to the ECC single error."]
        #[must_use]
        #[inline(always)]
        pub const fn esea(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "ECC single error address When the ALE bit is set in the MxCR register, this field is updated with the address corresponding to the ECC single error."]
        #[inline(always)]
        pub const fn set_esea(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for M5sear {
        #[inline(always)]
        fn default() -> M5sear {
            M5sear(0)
        }
    }
    impl core::fmt::Debug for M5sear {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M5sear").field("esea", &self.esea()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M5sear {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "M5sear {{ esea: {=u32:?} }}", self.esea())
        }
    }
    #[doc = "RAMCFG memory 6 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M6cr(pub u32);
    impl M6cr {
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
    impl Default for M6cr {
        #[inline(always)]
        fn default() -> M6cr {
            M6cr(0)
        }
    }
    impl core::fmt::Debug for M6cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M6cr")
                .field("ecce", &self.ecce())
                .field("ale", &self.ale())
                .field("sramer", &self.sramer())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M6cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M6cr {{ ecce: {=bool:?}, ale: {=bool:?}, sramer: {=bool:?} }}",
                self.ecce(),
                self.ale(),
                self.sramer()
            )
        }
    }
    #[doc = "RAMCFG memory 6 erase key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M6erkeyr(pub u32);
    impl M6erkeyr {
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
    impl Default for M6erkeyr {
        #[inline(always)]
        fn default() -> M6erkeyr {
            M6erkeyr(0)
        }
    }
    impl core::fmt::Debug for M6erkeyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M6erkeyr").field("erasekey", &self.erasekey()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M6erkeyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "M6erkeyr {{ erasekey: {=u8:?} }}", self.erasekey())
        }
    }
    #[doc = "RAMCFG memory interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M6isr(pub u32);
    impl M6isr {
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
    impl Default for M6isr {
        #[inline(always)]
        fn default() -> M6isr {
            M6isr(0)
        }
    }
    impl core::fmt::Debug for M6isr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M6isr")
                .field("sedc", &self.sedc())
                .field("ded", &self.ded())
                .field("srambusy", &self.srambusy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M6isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M6isr {{ sedc: {=bool:?}, ded: {=bool:?}, srambusy: {=bool:?} }}",
                self.sedc(),
                self.ded(),
                self.srambusy()
            )
        }
    }
    #[doc = "RAMCFG memory 7 control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M7cr(pub u32);
    impl M7cr {
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
    impl Default for M7cr {
        #[inline(always)]
        fn default() -> M7cr {
            M7cr(0)
        }
    }
    impl core::fmt::Debug for M7cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M7cr")
                .field("ecce", &self.ecce())
                .field("ale", &self.ale())
                .field("sramer", &self.sramer())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M7cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M7cr {{ ecce: {=bool:?}, ale: {=bool:?}, sramer: {=bool:?} }}",
                self.ecce(),
                self.ale(),
                self.sramer()
            )
        }
    }
    #[doc = "RAMCFG memory 7 erase key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M7erkeyr(pub u32);
    impl M7erkeyr {
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
    impl Default for M7erkeyr {
        #[inline(always)]
        fn default() -> M7erkeyr {
            M7erkeyr(0)
        }
    }
    impl core::fmt::Debug for M7erkeyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M7erkeyr").field("erasekey", &self.erasekey()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M7erkeyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "M7erkeyr {{ erasekey: {=u8:?} }}", self.erasekey())
        }
    }
    #[doc = "RAMCFG memory interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct M7isr(pub u32);
    impl M7isr {
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
    impl Default for M7isr {
        #[inline(always)]
        fn default() -> M7isr {
            M7isr(0)
        }
    }
    impl core::fmt::Debug for M7isr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("M7isr")
                .field("sedc", &self.sedc())
                .field("ded", &self.ded())
                .field("srambusy", &self.srambusy())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for M7isr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "M7isr {{ sedc: {=bool:?}, ded: {=bool:?}, srambusy: {=bool:?} }}",
                self.sedc(),
                self.ded(),
                self.srambusy()
            )
        }
    }
}
