#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "FLASH register block."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flash {
    ptr: *mut u8,
}
unsafe impl Send for Flash {}
unsafe impl Sync for Flash {}
impl Flash {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Each register is assigned a offset address and a reset value. In case of registers representing option byte value, the reset value is determined by the OBL process. In case of success the reset value is loaded from OB. In case of OBL failure, a highly restrictive default value is set.FLASH access control register."]
    #[inline(always)]
    pub const fn acr(self) -> crate::common::Reg<regs::Acr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "FLASH key register."]
    #[inline(always)]
    pub const fn keyr(self) -> crate::common::Reg<regs::Keyr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "FLASH option key register."]
    #[inline(always)]
    pub const fn optkeyr(self) -> crate::common::Reg<regs::Optkeyr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "FLASH operation status register."]
    #[inline(always)]
    pub const fn opsr(self) -> crate::common::Reg<regs::Opsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "FLASH option control register."]
    #[inline(always)]
    pub const fn optcr(self) -> crate::common::Reg<regs::Optcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "FLASH status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "FLASH control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "FLASH clear control register."]
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::Ccr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "FLASH privilege configuration register."]
    #[inline(always)]
    pub const fn privcfgr(self) -> crate::common::Reg<regs::Privcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "FLASH HDP extension register."]
    #[inline(always)]
    pub const fn hdpextr(self) -> crate::common::Reg<regs::Hdpextr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "FLASH option status register."]
    #[inline(always)]
    pub const fn optsr_cur(self) -> crate::common::Reg<regs::OptsrCur, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "FLASH option status register."]
    #[inline(always)]
    pub const fn optsr_prg(self) -> crate::common::Reg<regs::OptsrPrg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "FLASH option status register 2."]
    #[inline(always)]
    pub const fn optsr2_cur(self) -> crate::common::Reg<regs::Optsr2Cur, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "FLASH option status register 2."]
    #[inline(always)]
    pub const fn optsr2_prg(self) -> crate::common::Reg<regs::Optsr2Prg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "FLASH unique boot entry register."]
    #[inline(always)]
    pub const fn bootr_cur(self) -> crate::common::Reg<regs::BootrCur, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "FLASH unique boot entry address."]
    #[inline(always)]
    pub const fn bootr_prg(self) -> crate::common::Reg<regs::BootrPrg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "FLASH OTP block lock."]
    #[inline(always)]
    pub const fn otpblr_cur(self) -> crate::common::Reg<regs::OtpblrCur, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "FLASH OTP block lock."]
    #[inline(always)]
    pub const fn otpblr_prg(self) -> crate::common::Reg<regs::OtpblrPrg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "FLASH Bootloader interface selection."]
    #[inline(always)]
    pub const fn bl_com_cfg_cur(self) -> crate::common::Reg<regs::BlComCfgCur, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "FLASH Bootloader interface selection."]
    #[inline(always)]
    pub const fn bl_com_cfg_prg(self) -> crate::common::Reg<regs::BlComCfgPrg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "FLASH OEM Key register 1."]
    #[inline(always)]
    pub const fn oemkeyr1_prg(self) -> crate::common::Reg<regs::Oemkeyr1Prg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "FLASH OEM Key register 2."]
    #[inline(always)]
    pub const fn oemkeyr2_prg(self) -> crate::common::Reg<regs::Oemkeyr2Prg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "FLASH OEM Key register 3."]
    #[inline(always)]
    pub const fn oemkeyr3_prg(self) -> crate::common::Reg<regs::Oemkeyr3Prg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "FLASH OEM Key register 4."]
    #[inline(always)]
    pub const fn oemkeyr4_prg(self) -> crate::common::Reg<regs::Oemkeyr4Prg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "FLASH Boundary Scan key register."]
    #[inline(always)]
    pub const fn bskeyr_prg(self) -> crate::common::Reg<regs::BskeyrPrg, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "FLASH write page protection for bank1."]
    #[inline(always)]
    pub const fn wrp1r_cur(self) -> crate::common::Reg<regs::Wrp1rCur, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "FLASH write page protection for bank1."]
    #[inline(always)]
    pub const fn wrp1r_prg(self) -> crate::common::Reg<regs::Wrp1rPrg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "FLASH HDP bank1 register."]
    #[inline(always)]
    pub const fn hdp1r_cur(self) -> crate::common::Reg<regs::Hdp1rCur, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "FLASH HDP bank1 register."]
    #[inline(always)]
    pub const fn hdp1r_prg(self) -> crate::common::Reg<regs::Hdp1rPrg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "FLASH ECC correction register."]
    #[inline(always)]
    pub const fn ecccorr(self) -> crate::common::Reg<regs::Ecccorr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "FLASH ECC detection register."]
    #[inline(always)]
    pub const fn eccdetr(self) -> crate::common::Reg<regs::Eccdetr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "FLASH ECC data."]
    #[inline(always)]
    pub const fn eccdr(self) -> crate::common::Reg<regs::Eccdr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "FLASH write page protection for bank2."]
    #[inline(always)]
    pub const fn wrp2r_cur(self) -> crate::common::Reg<regs::Wrp2rCur, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[doc = "FLASH write page protection for bank2."]
    #[inline(always)]
    pub const fn wrp2r_prg(self) -> crate::common::Reg<regs::Wrp2rPrg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ecusize) as _) }
    }
    #[doc = "FLASH HDP bank2 register."]
    #[inline(always)]
    pub const fn hdp2r_cur(self) -> crate::common::Reg<regs::Hdp2rCur, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "FLASH HDP bank2 register."]
    #[inline(always)]
    pub const fn hdp2r_prg(self) -> crate::common::Reg<regs::Hdp2rPrg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01fcusize) as _) }
    }
}
pub mod regs {
    #[doc = "Each register is assigned a offset address and a reset value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Acr(pub u32);
    impl Acr {
        #[doc = "Read latency."]
        #[must_use]
        #[inline(always)]
        pub const fn latency(&self) -> super::vals::Latency {
            let val = (self.0 >> 0usize) & 0x0f;
            super::vals::Latency::from_bits(val as u8)
        }
        #[doc = "Read latency."]
        #[inline(always)]
        pub const fn set_latency(&mut self, val: super::vals::Latency) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val.to_bits() as u32) & 0x0f) << 0usize);
        }
        #[doc = "FLASH signal delay."]
        #[must_use]
        #[inline(always)]
        pub const fn wrhighfreq(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "FLASH signal delay."]
        #[inline(always)]
        pub const fn set_wrhighfreq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Prefetch enable."]
        #[must_use]
        #[inline(always)]
        pub const fn prften(&self) -> super::vals::Prften {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Prften::from_bits(val as u8)
        }
        #[doc = "Prefetch enable."]
        #[inline(always)]
        pub const fn set_prften(&mut self, val: super::vals::Prften) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "Main Flash memory area empty (not reset by system reset)."]
        #[must_use]
        #[inline(always)]
        pub const fn empty(&self) -> super::vals::Empty {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Empty::from_bits(val as u8)
        }
        #[doc = "Main Flash memory area empty (not reset by system reset)."]
        #[inline(always)]
        pub const fn set_empty(&mut self, val: super::vals::Empty) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Acr {
        #[inline(always)]
        fn default() -> Acr {
            Acr(0)
        }
    }
    impl core::fmt::Debug for Acr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Acr")
                .field("latency", &self.latency())
                .field("wrhighfreq", &self.wrhighfreq())
                .field("prften", &self.prften())
                .field("empty", &self.empty())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Acr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Acr {{ latency: {:?}, wrhighfreq: {=u8:?}, prften: {:?}, empty: {:?} }}",
                self.latency(),
                self.wrhighfreq(),
                self.prften(),
                self.empty()
            )
        }
    }
    #[doc = "FLASH Bootloader interface selection."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BlComCfgCur(pub u32);
    impl BlComCfgCur {
        #[doc = "Bootloader interface selection/configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn bl_com_cfg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Bootloader interface selection/configuration."]
        #[inline(always)]
        pub const fn set_bl_com_cfg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BlComCfgCur {
        #[inline(always)]
        fn default() -> BlComCfgCur {
            BlComCfgCur(0)
        }
    }
    impl core::fmt::Debug for BlComCfgCur {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BlComCfgCur")
                .field("bl_com_cfg", &self.bl_com_cfg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BlComCfgCur {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BlComCfgCur {{ bl_com_cfg: {=u32:?} }}", self.bl_com_cfg())
        }
    }
    #[doc = "FLASH Bootloader interface selection."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BlComCfgPrg(pub u32);
    impl BlComCfgPrg {
        #[doc = "Bootloader interface selection/configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn bl_com_cfg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Bootloader interface selection/configuration."]
        #[inline(always)]
        pub const fn set_bl_com_cfg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BlComCfgPrg {
        #[inline(always)]
        fn default() -> BlComCfgPrg {
            BlComCfgPrg(0)
        }
    }
    impl core::fmt::Debug for BlComCfgPrg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BlComCfgPrg")
                .field("bl_com_cfg", &self.bl_com_cfg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BlComCfgPrg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BlComCfgPrg {{ bl_com_cfg: {=u32:?} }}", self.bl_com_cfg())
        }
    }
    #[doc = "FLASH unique boot entry register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BootrCur(pub u32);
    impl BootrCur {
        #[doc = "A field locking the values of BOOT0, BOOT_SEL, SWAP_BANK, and BOOTADD option settings."]
        #[must_use]
        #[inline(always)]
        pub const fn boot_lock(&self) -> super::vals::BootrCurBootLock {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::BootrCurBootLock::from_bits(val as u8)
        }
        #[doc = "A field locking the values of BOOT0, BOOT_SEL, SWAP_BANK, and BOOTADD option settings."]
        #[inline(always)]
        pub const fn set_boot_lock(&mut self, val: super::vals::BootrCurBootLock) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
        #[doc = "unique boot entry address."]
        #[must_use]
        #[inline(always)]
        pub const fn bootadd(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "unique boot entry address."]
        #[inline(always)]
        pub const fn set_bootadd(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for BootrCur {
        #[inline(always)]
        fn default() -> BootrCur {
            BootrCur(0)
        }
    }
    impl core::fmt::Debug for BootrCur {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BootrCur")
                .field("boot_lock", &self.boot_lock())
                .field("bootadd", &self.bootadd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BootrCur {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "BootrCur {{ boot_lock: {:?}, bootadd: {=u32:?} }}",
                self.boot_lock(),
                self.bootadd()
            )
        }
    }
    #[doc = "FLASH unique boot entry address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BootrPrg(pub u32);
    impl BootrPrg {
        #[doc = "A field locking the values of BOOT0, BOOT_SEL, SWAP_BANK, and BOOTADD option settings."]
        #[must_use]
        #[inline(always)]
        pub const fn boot_lock(&self) -> super::vals::BootrPrgBootLock {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::BootrPrgBootLock::from_bits(val as u8)
        }
        #[doc = "A field locking the values of BOOT0, BOOT_SEL, SWAP_BANK, and BOOTADD option settings."]
        #[inline(always)]
        pub const fn set_boot_lock(&mut self, val: super::vals::BootrPrgBootLock) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
        #[doc = "unique boot entry address."]
        #[must_use]
        #[inline(always)]
        pub const fn bootadd(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "unique boot entry address."]
        #[inline(always)]
        pub const fn set_bootadd(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for BootrPrg {
        #[inline(always)]
        fn default() -> BootrPrg {
            BootrPrg(0)
        }
    }
    impl core::fmt::Debug for BootrPrg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BootrPrg")
                .field("boot_lock", &self.boot_lock())
                .field("bootadd", &self.bootadd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BootrPrg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "BootrPrg {{ boot_lock: {:?}, bootadd: {=u32:?} }}",
                self.boot_lock(),
                self.bootadd()
            )
        }
    }
    #[doc = "FLASH Boundary Scan key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BskeyrPrg(pub u32);
    impl BskeyrPrg {
        #[doc = "Boundary Scan KEY."]
        #[must_use]
        #[inline(always)]
        pub const fn bskey(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Boundary Scan KEY."]
        #[inline(always)]
        pub const fn set_bskey(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BskeyrPrg {
        #[inline(always)]
        fn default() -> BskeyrPrg {
            BskeyrPrg(0)
        }
    }
    impl core::fmt::Debug for BskeyrPrg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BskeyrPrg").field("bskey", &self.bskey()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BskeyrPrg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BskeyrPrg {{ bskey: {=u32:?} }}", self.bskey())
        }
    }
    #[doc = "FLASH clear control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr(pub u32);
    impl Ccr {
        #[doc = "EOP flag clear bit."]
        #[must_use]
        #[inline(always)]
        pub const fn clr_eop(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "EOP flag clear bit."]
        #[inline(always)]
        pub const fn set_clr_eop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "WRPERR flag clear bit."]
        #[must_use]
        #[inline(always)]
        pub const fn clr_wrperr(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "WRPERR flag clear bit."]
        #[inline(always)]
        pub const fn set_clr_wrperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "PGSERR flag clear bit."]
        #[must_use]
        #[inline(always)]
        pub const fn clr_pgserr(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PGSERR flag clear bit."]
        #[inline(always)]
        pub const fn set_clr_pgserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "STRBERR flag clear bit."]
        #[must_use]
        #[inline(always)]
        pub const fn clr_strberr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "STRBERR flag clear bit."]
        #[inline(always)]
        pub const fn set_clr_strberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "INCERR flag clear bit."]
        #[must_use]
        #[inline(always)]
        pub const fn clr_incerr(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "INCERR flag clear bit."]
        #[inline(always)]
        pub const fn set_clr_incerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Clear the flag corresponding flag in SR by writing this bit."]
        #[must_use]
        #[inline(always)]
        pub const fn clr_optchangeerr(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Clear the flag corresponding flag in SR by writing this bit."]
        #[inline(always)]
        pub const fn set_clr_optchangeerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Ccr {
        #[inline(always)]
        fn default() -> Ccr {
            Ccr(0)
        }
    }
    impl core::fmt::Debug for Ccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccr")
                .field("clr_eop", &self.clr_eop())
                .field("clr_wrperr", &self.clr_wrperr())
                .field("clr_pgserr", &self.clr_pgserr())
                .field("clr_strberr", &self.clr_strberr())
                .field("clr_incerr", &self.clr_incerr())
                .field("clr_optchangeerr", &self.clr_optchangeerr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccr {{ clr_eop: {=bool:?}, clr_wrperr: {=bool:?}, clr_pgserr: {=bool:?}, clr_strberr: {=bool:?}, clr_incerr: {=bool:?}, clr_optchangeerr: {=bool:?} }}",
                self.clr_eop(),
                self.clr_wrperr(),
                self.clr_pgserr(),
                self.clr_strberr(),
                self.clr_incerr(),
                self.clr_optchangeerr()
            )
        }
    }
    #[doc = "FLASH control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "configuration lock bit."]
        #[must_use]
        #[inline(always)]
        pub const fn lock(&self) -> super::vals::Lock {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Lock::from_bits(val as u8)
        }
        #[doc = "configuration lock bit."]
        #[inline(always)]
        pub const fn set_lock(&mut self, val: super::vals::Lock) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "programming control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn pg(&self) -> super::vals::Pg {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Pg::from_bits(val as u8)
        }
        #[doc = "programming control bit."]
        #[inline(always)]
        pub const fn set_pg(&mut self, val: super::vals::Pg) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "page erase request."]
        #[must_use]
        #[inline(always)]
        pub const fn per(&self) -> super::vals::Per {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Per::from_bits(val as u8)
        }
        #[doc = "page erase request."]
        #[inline(always)]
        pub const fn set_per(&mut self, val: super::vals::Per) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "sector erase request."]
        #[must_use]
        #[inline(always)]
        pub const fn ser(&self) -> super::vals::Ser {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Ser::from_bits(val as u8)
        }
        #[doc = "sector erase request."]
        #[inline(always)]
        pub const fn set_ser(&mut self, val: super::vals::Ser) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Bank erase request."]
        #[must_use]
        #[inline(always)]
        pub const fn ber(&self) -> super::vals::Ber {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Ber::from_bits(val as u8)
        }
        #[doc = "Bank erase request."]
        #[inline(always)]
        pub const fn set_ber(&mut self, val: super::vals::Ber) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "write forcing control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn fw(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "write forcing control bit."]
        #[inline(always)]
        pub const fn set_fw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "erase start control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn strt(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "erase start control bit."]
        #[inline(always)]
        pub const fn set_strt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "page erase selection number."]
        #[must_use]
        #[inline(always)]
        pub const fn pnb(&self) -> super::vals::Pnb {
            let val = (self.0 >> 6usize) & 0x3f;
            super::vals::Pnb::from_bits(val as u8)
        }
        #[doc = "page erase selection number."]
        #[inline(always)]
        pub const fn set_pnb(&mut self, val: super::vals::Pnb) {
            self.0 = (self.0 & !(0x3f << 6usize)) | (((val.to_bits() as u32) & 0x3f) << 6usize);
        }
        #[doc = "sector erase selection number."]
        #[must_use]
        #[inline(always)]
        pub const fn snb(&self) -> super::vals::Snb {
            let val = (self.0 >> 6usize) & 0x3f;
            super::vals::Snb::from_bits(val as u8)
        }
        #[doc = "sector erase selection number."]
        #[inline(always)]
        pub const fn set_snb(&mut self, val: super::vals::Snb) {
            self.0 = (self.0 & !(0x3f << 6usize)) | (((val.to_bits() as u32) & 0x3f) << 6usize);
        }
        #[doc = "Mass erase request."]
        #[must_use]
        #[inline(always)]
        pub const fn mer(&self) -> super::vals::Mer {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Mer::from_bits(val as u8)
        }
        #[doc = "Mass erase request."]
        #[inline(always)]
        pub const fn set_mer(&mut self, val: super::vals::Mer) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "end of operation interrupt control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn eopie(&self) -> super::vals::Eopie {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Eopie::from_bits(val as u8)
        }
        #[doc = "end of operation interrupt control bit."]
        #[inline(always)]
        pub const fn set_eopie(&mut self, val: super::vals::Eopie) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "write protection error interrupt enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn wrperrie(&self) -> super::vals::Wrperrie {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Wrperrie::from_bits(val as u8)
        }
        #[doc = "write protection error interrupt enable bit."]
        #[inline(always)]
        pub const fn set_wrperrie(&mut self, val: super::vals::Wrperrie) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "programming sequence error interrupt enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn pgserrie(&self) -> super::vals::Pgserrie {
            let val = (self.0 >> 18usize) & 0x01;
            super::vals::Pgserrie::from_bits(val as u8)
        }
        #[doc = "programming sequence error interrupt enable bit."]
        #[inline(always)]
        pub const fn set_pgserrie(&mut self, val: super::vals::Pgserrie) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
        }
        #[doc = "strobe error interrupt enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn strberrie(&self) -> super::vals::Strberrie {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Strberrie::from_bits(val as u8)
        }
        #[doc = "strobe error interrupt enable bit."]
        #[inline(always)]
        pub const fn set_strberrie(&mut self, val: super::vals::Strberrie) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "inconsistency error interrupt enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn incerrie(&self) -> super::vals::Incerrie {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Incerrie::from_bits(val as u8)
        }
        #[doc = "inconsistency error interrupt enable bit."]
        #[inline(always)]
        pub const fn set_incerrie(&mut self, val: super::vals::Incerrie) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Option-byte change error interrupt enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn optchangeerrie(&self) -> super::vals::Optchangeerrie {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Optchangeerrie::from_bits(val as u8)
        }
        #[doc = "Option-byte change error interrupt enable bit."]
        #[inline(always)]
        pub const fn set_optchangeerrie(&mut self, val: super::vals::Optchangeerrie) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "EDATA erase selector bit."]
        #[must_use]
        #[inline(always)]
        pub const fn edatasel(&self) -> super::vals::Edatasel {
            let val = (self.0 >> 29usize) & 0x01;
            super::vals::Edatasel::from_bits(val as u8)
        }
        #[doc = "EDATA erase selector bit."]
        #[inline(always)]
        pub const fn set_edatasel(&mut self, val: super::vals::Edatasel) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
        }
        #[doc = "Bank selector bit."]
        #[must_use]
        #[inline(always)]
        pub const fn bksel(&self) -> super::vals::Bksel {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Bksel::from_bits(val as u8)
        }
        #[doc = "Bank selector bit."]
        #[inline(always)]
        pub const fn set_bksel(&mut self, val: super::vals::Bksel) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
                .field("lock", &self.lock())
                .field("pg", &self.pg())
                .field("per", &self.per())
                .field("ser", &self.ser())
                .field("ber", &self.ber())
                .field("fw", &self.fw())
                .field("strt", &self.strt())
                .field("pnb", &self.pnb())
                .field("snb", &self.snb())
                .field("mer", &self.mer())
                .field("eopie", &self.eopie())
                .field("wrperrie", &self.wrperrie())
                .field("pgserrie", &self.pgserrie())
                .field("strberrie", &self.strberrie())
                .field("incerrie", &self.incerrie())
                .field("optchangeerrie", &self.optchangeerrie())
                .field("edatasel", &self.edatasel())
                .field("bksel", &self.bksel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ lock: {:?}, pg: {:?}, per: {:?}, ser: {:?}, ber: {:?}, fw: {=bool:?}, strt: {=bool:?}, pnb: {:?}, snb: {:?}, mer: {:?}, eopie: {:?}, wrperrie: {:?}, pgserrie: {:?}, strberrie: {:?}, incerrie: {:?}, optchangeerrie: {:?}, edatasel: {:?}, bksel: {:?} }}",
                self.lock(),
                self.pg(),
                self.per(),
                self.ser(),
                self.ber(),
                self.fw(),
                self.strt(),
                self.pnb(),
                self.snb(),
                self.mer(),
                self.eopie(),
                self.wrperrie(),
                self.pgserrie(),
                self.strberrie(),
                self.incerrie(),
                self.optchangeerrie(),
                self.edatasel(),
                self.bksel()
            )
        }
    }
    #[doc = "FLASH ECC correction register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ecccorr(pub u32);
    impl Ecccorr {
        #[doc = "ECC error address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr_ecc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "ECC error address."]
        #[inline(always)]
        pub const fn set_addr_ecc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "ECC fail for corrected ECC error in flash data area."]
        #[must_use]
        #[inline(always)]
        pub const fn data_ecc(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "ECC fail for corrected ECC error in flash data area."]
        #[inline(always)]
        pub const fn set_data_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ECC fail for corrected ECC error in flash data area."]
        #[must_use]
        #[inline(always)]
        pub const fn edata_ecc(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "ECC fail for corrected ECC error in flash data area."]
        #[inline(always)]
        pub const fn set_edata_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ECC bank flag for corrected ECC error."]
        #[must_use]
        #[inline(always)]
        pub const fn bk_ecc(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "ECC bank flag for corrected ECC error."]
        #[inline(always)]
        pub const fn set_bk_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "ECC flag for corrected ECC error in system FLASH."]
        #[must_use]
        #[inline(always)]
        pub const fn sysf_ecc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "ECC flag for corrected ECC error in system FLASH."]
        #[inline(always)]
        pub const fn set_sysf_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "OTP ECC error bit."]
        #[must_use]
        #[inline(always)]
        pub const fn otp_ecc(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "OTP ECC error bit."]
        #[inline(always)]
        pub const fn set_otp_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "ECC single correction error interrupt enable bit When ECCCIE bit is set to 1, an interrupt is generated when an ECC single correction error occurs during a read operation."]
        #[must_use]
        #[inline(always)]
        pub const fn ecccie(&self) -> super::vals::Ecccie {
            let val = (self.0 >> 25usize) & 0x01;
            super::vals::Ecccie::from_bits(val as u8)
        }
        #[doc = "ECC single correction error interrupt enable bit When ECCCIE bit is set to 1, an interrupt is generated when an ECC single correction error occurs during a read operation."]
        #[inline(always)]
        pub const fn set_ecccie(&mut self, val: super::vals::Ecccie) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
        }
        #[doc = "ECC correction."]
        #[must_use]
        #[inline(always)]
        pub const fn eccc(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "ECC correction."]
        #[inline(always)]
        pub const fn set_eccc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Ecccorr {
        #[inline(always)]
        fn default() -> Ecccorr {
            Ecccorr(0)
        }
    }
    impl core::fmt::Debug for Ecccorr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ecccorr")
                .field("addr_ecc", &self.addr_ecc())
                .field("data_ecc", &self.data_ecc())
                .field("edata_ecc", &self.edata_ecc())
                .field("bk_ecc", &self.bk_ecc())
                .field("sysf_ecc", &self.sysf_ecc())
                .field("otp_ecc", &self.otp_ecc())
                .field("ecccie", &self.ecccie())
                .field("eccc", &self.eccc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ecccorr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ecccorr {{ addr_ecc: {=u16:?}, data_ecc: {=bool:?}, edata_ecc: {=bool:?}, bk_ecc: {=bool:?}, sysf_ecc: {=bool:?}, otp_ecc: {=bool:?}, ecccie: {:?}, eccc: {=bool:?} }}",
                self.addr_ecc(),
                self.data_ecc(),
                self.edata_ecc(),
                self.bk_ecc(),
                self.sysf_ecc(),
                self.otp_ecc(),
                self.ecccie(),
                self.eccc()
            )
        }
    }
    #[doc = "FLASH ECC detection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccdetr(pub u32);
    impl Eccdetr {
        #[doc = "ECC error address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr_ecc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "ECC error address."]
        #[inline(always)]
        pub const fn set_addr_ecc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "ECC fail for double ECC error in flash data area."]
        #[must_use]
        #[inline(always)]
        pub const fn data_ecc(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "ECC fail for double ECC error in flash data area."]
        #[inline(always)]
        pub const fn set_data_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ECC fail for double ECC error in flash data area."]
        #[must_use]
        #[inline(always)]
        pub const fn edata_ecc(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "ECC fail for double ECC error in flash data area."]
        #[inline(always)]
        pub const fn set_edata_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ECC fail bank for double ECC Error."]
        #[must_use]
        #[inline(always)]
        pub const fn bk_ecc(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "ECC fail bank for double ECC Error."]
        #[inline(always)]
        pub const fn set_bk_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "ECC fail for double ECC error in system flash memory."]
        #[must_use]
        #[inline(always)]
        pub const fn sysf_ecc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "ECC fail for double ECC error in system flash memory."]
        #[inline(always)]
        pub const fn set_sysf_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "OTP ECC error bit."]
        #[must_use]
        #[inline(always)]
        pub const fn otp_ecc(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "OTP ECC error bit."]
        #[inline(always)]
        pub const fn set_otp_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "ECC detection set by hardware when two ECC error has been detected."]
        #[must_use]
        #[inline(always)]
        pub const fn eccd(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "ECC detection set by hardware when two ECC error has been detected."]
        #[inline(always)]
        pub const fn set_eccd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Eccdetr {
        #[inline(always)]
        fn default() -> Eccdetr {
            Eccdetr(0)
        }
    }
    impl core::fmt::Debug for Eccdetr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccdetr")
                .field("addr_ecc", &self.addr_ecc())
                .field("data_ecc", &self.data_ecc())
                .field("edata_ecc", &self.edata_ecc())
                .field("bk_ecc", &self.bk_ecc())
                .field("sysf_ecc", &self.sysf_ecc())
                .field("otp_ecc", &self.otp_ecc())
                .field("eccd", &self.eccd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eccdetr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eccdetr {{ addr_ecc: {=u16:?}, data_ecc: {=bool:?}, edata_ecc: {=bool:?}, bk_ecc: {=bool:?}, sysf_ecc: {=bool:?}, otp_ecc: {=bool:?}, eccd: {=bool:?} }}",
                self.addr_ecc(),
                self.data_ecc(),
                self.edata_ecc(),
                self.bk_ecc(),
                self.sysf_ecc(),
                self.otp_ecc(),
                self.eccd()
            )
        }
    }
    #[doc = "FLASH ECC data."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccdr(pub u32);
    impl Eccdr {
        #[doc = "ECC error data."]
        #[must_use]
        #[inline(always)]
        pub const fn data_ecc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "ECC error data."]
        #[inline(always)]
        pub const fn set_data_ecc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "DATA ECC error address."]
        #[must_use]
        #[inline(always)]
        pub const fn data_addr_ecc(&self) -> super::vals::DataAddrEcc {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::DataAddrEcc::from_bits(val as u8)
        }
        #[doc = "DATA ECC error address."]
        #[inline(always)]
        pub const fn set_data_addr_ecc(&mut self, val: super::vals::DataAddrEcc) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
    }
    impl Default for Eccdr {
        #[inline(always)]
        fn default() -> Eccdr {
            Eccdr(0)
        }
    }
    impl core::fmt::Debug for Eccdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccdr")
                .field("data_ecc", &self.data_ecc())
                .field("data_addr_ecc", &self.data_addr_ecc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eccdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Eccdr {{ data_ecc: {=u16:?}, data_addr_ecc: {:?} }}",
                self.data_ecc(),
                self.data_addr_ecc()
            )
        }
    }
    #[doc = "FLASH HDP bank1 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hdp1rCur(pub u32);
    impl Hdp1rCur {
        #[doc = "Bank 1 HDPL barrier start set in number of 8Kbytes pages."]
        #[must_use]
        #[inline(always)]
        pub const fn hdp1_strt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Bank 1 HDPL barrier start set in number of 8Kbytes pages."]
        #[inline(always)]
        pub const fn set_hdp1_strt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Bank 1 HDPL barrier end set in number of 8Kbytes pages."]
        #[must_use]
        #[inline(always)]
        pub const fn hdp1_end(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Bank 1 HDPL barrier end set in number of 8Kbytes pages."]
        #[inline(always)]
        pub const fn set_hdp1_end(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Hdp1rCur {
        #[inline(always)]
        fn default() -> Hdp1rCur {
            Hdp1rCur(0)
        }
    }
    impl core::fmt::Debug for Hdp1rCur {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hdp1rCur")
                .field("hdp1_strt", &self.hdp1_strt())
                .field("hdp1_end", &self.hdp1_end())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hdp1rCur {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hdp1rCur {{ hdp1_strt: {=u8:?}, hdp1_end: {=u8:?} }}",
                self.hdp1_strt(),
                self.hdp1_end()
            )
        }
    }
    #[doc = "FLASH HDP bank1 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hdp1rPrg(pub u32);
    impl Hdp1rPrg {
        #[doc = "Bank 1 HDPL barrier start set in number of 8Kbytes pages."]
        #[must_use]
        #[inline(always)]
        pub const fn hdp1_strt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Bank 1 HDPL barrier start set in number of 8Kbytes pages."]
        #[inline(always)]
        pub const fn set_hdp1_strt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Bank 1 HDPL barrier end set in number of 8Kbytes pages."]
        #[must_use]
        #[inline(always)]
        pub const fn hdp1_end(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Bank 1 HDPL barrier end set in number of 8Kbytes pages."]
        #[inline(always)]
        pub const fn set_hdp1_end(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Hdp1rPrg {
        #[inline(always)]
        fn default() -> Hdp1rPrg {
            Hdp1rPrg(0)
        }
    }
    impl core::fmt::Debug for Hdp1rPrg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hdp1rPrg")
                .field("hdp1_strt", &self.hdp1_strt())
                .field("hdp1_end", &self.hdp1_end())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hdp1rPrg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hdp1rPrg {{ hdp1_strt: {=u8:?}, hdp1_end: {=u8:?} }}",
                self.hdp1_strt(),
                self.hdp1_end()
            )
        }
    }
    #[doc = "FLASH HDP bank2 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hdp2rCur(pub u32);
    impl Hdp2rCur {
        #[doc = "Bank 2 HDPL barrier start set in number of 8Kbytes pages."]
        #[must_use]
        #[inline(always)]
        pub const fn hdp2_strt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Bank 2 HDPL barrier start set in number of 8Kbytes pages."]
        #[inline(always)]
        pub const fn set_hdp2_strt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Bank 2 HDPL barrier end set in number of 8Kbytes pages."]
        #[must_use]
        #[inline(always)]
        pub const fn hdp2_end(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Bank 2 HDPL barrier end set in number of 8Kbytes pages."]
        #[inline(always)]
        pub const fn set_hdp2_end(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Hdp2rCur {
        #[inline(always)]
        fn default() -> Hdp2rCur {
            Hdp2rCur(0)
        }
    }
    impl core::fmt::Debug for Hdp2rCur {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hdp2rCur")
                .field("hdp2_strt", &self.hdp2_strt())
                .field("hdp2_end", &self.hdp2_end())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hdp2rCur {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hdp2rCur {{ hdp2_strt: {=u8:?}, hdp2_end: {=u8:?} }}",
                self.hdp2_strt(),
                self.hdp2_end()
            )
        }
    }
    #[doc = "FLASH HDP bank2 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hdp2rPrg(pub u32);
    impl Hdp2rPrg {
        #[doc = "Bank 2 HDPL barrier start set in number of 8Kbytes pages."]
        #[must_use]
        #[inline(always)]
        pub const fn hdp2_strt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Bank 2 HDPL barrier start set in number of 8Kbytes pages."]
        #[inline(always)]
        pub const fn set_hdp2_strt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Bank 2 HDPL barrier end set in number of 8Kbytes pages."]
        #[must_use]
        #[inline(always)]
        pub const fn hdp2_end(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Bank 2 HDPL barrier end set in number of 8Kbytes pages."]
        #[inline(always)]
        pub const fn set_hdp2_end(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Hdp2rPrg {
        #[inline(always)]
        fn default() -> Hdp2rPrg {
            Hdp2rPrg(0)
        }
    }
    impl core::fmt::Debug for Hdp2rPrg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hdp2rPrg")
                .field("hdp2_strt", &self.hdp2_strt())
                .field("hdp2_end", &self.hdp2_end())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hdp2rPrg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hdp2rPrg {{ hdp2_strt: {=u8:?}, hdp2_end: {=u8:?} }}",
                self.hdp2_strt(),
                self.hdp2_end()
            )
        }
    }
    #[doc = "FLASH HDP extension register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hdpextr(pub u32);
    impl Hdpextr {
        #[doc = "HDP area extension in 8Kbytes pages in bank1. Extension is added after the HDP1_END page (included)."]
        #[must_use]
        #[inline(always)]
        pub const fn hdp1_ext(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "HDP area extension in 8Kbytes pages in bank1. Extension is added after the HDP1_END page (included)."]
        #[inline(always)]
        pub const fn set_hdp1_ext(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "HDP area extension in 8Kbytes pages in bank2. Extension is added after the HDP2_END page (included)."]
        #[must_use]
        #[inline(always)]
        pub const fn hdp2_ext(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "HDP area extension in 8Kbytes pages in bank2. Extension is added after the HDP2_END page (included)."]
        #[inline(always)]
        pub const fn set_hdp2_ext(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
    }
    impl Default for Hdpextr {
        #[inline(always)]
        fn default() -> Hdpextr {
            Hdpextr(0)
        }
    }
    impl core::fmt::Debug for Hdpextr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hdpextr")
                .field("hdp1_ext", &self.hdp1_ext())
                .field("hdp2_ext", &self.hdp2_ext())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hdpextr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hdpextr {{ hdp1_ext: {=u8:?}, hdp2_ext: {=u8:?} }}",
                self.hdp1_ext(),
                self.hdp2_ext()
            )
        }
    }
    #[doc = "FLASH key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Keyr(pub u32);
    impl Keyr {
        #[doc = "Non-volatile memoryconfiguration access unlock key."]
        #[must_use]
        #[inline(always)]
        pub const fn key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Non-volatile memoryconfiguration access unlock key."]
        #[inline(always)]
        pub const fn set_key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Keyr {
        #[inline(always)]
        fn default() -> Keyr {
            Keyr(0)
        }
    }
    impl core::fmt::Debug for Keyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Keyr").field("key", &self.key()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Keyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Keyr {{ key: {=u32:?} }}", self.key())
        }
    }
    #[doc = "FLASH OEM Key register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oemkeyr1Prg(pub u32);
    impl Oemkeyr1Prg {
        #[doc = "Least significants bytes of OEMKEY."]
        #[must_use]
        #[inline(always)]
        pub const fn oemkey(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Least significants bytes of OEMKEY."]
        #[inline(always)]
        pub const fn set_oemkey(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Oemkeyr1Prg {
        #[inline(always)]
        fn default() -> Oemkeyr1Prg {
            Oemkeyr1Prg(0)
        }
    }
    impl core::fmt::Debug for Oemkeyr1Prg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Oemkeyr1Prg").field("oemkey", &self.oemkey()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Oemkeyr1Prg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Oemkeyr1Prg {{ oemkey: {=u32:?} }}", self.oemkey())
        }
    }
    #[doc = "FLASH OEM Key register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oemkeyr2Prg(pub u32);
    impl Oemkeyr2Prg {
        #[doc = "Mid-least significants bytes of OEMKEY."]
        #[must_use]
        #[inline(always)]
        pub const fn oemkey(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Mid-least significants bytes of OEMKEY."]
        #[inline(always)]
        pub const fn set_oemkey(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Oemkeyr2Prg {
        #[inline(always)]
        fn default() -> Oemkeyr2Prg {
            Oemkeyr2Prg(0)
        }
    }
    impl core::fmt::Debug for Oemkeyr2Prg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Oemkeyr2Prg").field("oemkey", &self.oemkey()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Oemkeyr2Prg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Oemkeyr2Prg {{ oemkey: {=u32:?} }}", self.oemkey())
        }
    }
    #[doc = "FLASH OEM Key register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oemkeyr3Prg(pub u32);
    impl Oemkeyr3Prg {
        #[doc = "Mid-most significants bytes of OEMKEY."]
        #[must_use]
        #[inline(always)]
        pub const fn oemkey(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Mid-most significants bytes of OEMKEY."]
        #[inline(always)]
        pub const fn set_oemkey(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Oemkeyr3Prg {
        #[inline(always)]
        fn default() -> Oemkeyr3Prg {
            Oemkeyr3Prg(0)
        }
    }
    impl core::fmt::Debug for Oemkeyr3Prg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Oemkeyr3Prg").field("oemkey", &self.oemkey()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Oemkeyr3Prg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Oemkeyr3Prg {{ oemkey: {=u32:?} }}", self.oemkey())
        }
    }
    #[doc = "FLASH OEM Key register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oemkeyr4Prg(pub u32);
    impl Oemkeyr4Prg {
        #[doc = "Most significants bytes of OEMKEY."]
        #[must_use]
        #[inline(always)]
        pub const fn oemkey(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Most significants bytes of OEMKEY."]
        #[inline(always)]
        pub const fn set_oemkey(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Oemkeyr4Prg {
        #[inline(always)]
        fn default() -> Oemkeyr4Prg {
            Oemkeyr4Prg(0)
        }
    }
    impl core::fmt::Debug for Oemkeyr4Prg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Oemkeyr4Prg").field("oemkey", &self.oemkey()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Oemkeyr4Prg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Oemkeyr4Prg {{ oemkey: {=u32:?} }}", self.oemkey())
        }
    }
    #[doc = "FLASH operation status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Opsr(pub u32);
    impl Opsr {
        #[doc = "Interrupted operation address."]
        #[must_use]
        #[inline(always)]
        pub const fn addr_op(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Interrupted operation address."]
        #[inline(always)]
        pub const fn set_addr_op(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Flash data area operation interrupted."]
        #[must_use]
        #[inline(always)]
        pub const fn data_op(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Flash data area operation interrupted."]
        #[inline(always)]
        pub const fn set_data_op(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Interrupted operation bank."]
        #[must_use]
        #[inline(always)]
        pub const fn bk_op(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupted operation bank."]
        #[inline(always)]
        pub const fn set_bk_op(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Operation in system flash memory interrupted."]
        #[must_use]
        #[inline(always)]
        pub const fn sysf_op(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Operation in system flash memory interrupted."]
        #[inline(always)]
        pub const fn set_sysf_op(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "OTP operation interrupted."]
        #[must_use]
        #[inline(always)]
        pub const fn otp_op(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "OTP operation interrupted."]
        #[inline(always)]
        pub const fn set_otp_op(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Flash memory operation code."]
        #[must_use]
        #[inline(always)]
        pub const fn code_op(&self) -> super::vals::CodeOp {
            let val = (self.0 >> 29usize) & 0x07;
            super::vals::CodeOp::from_bits(val as u8)
        }
        #[doc = "Flash memory operation code."]
        #[inline(always)]
        pub const fn set_code_op(&mut self, val: super::vals::CodeOp) {
            self.0 = (self.0 & !(0x07 << 29usize)) | (((val.to_bits() as u32) & 0x07) << 29usize);
        }
    }
    impl Default for Opsr {
        #[inline(always)]
        fn default() -> Opsr {
            Opsr(0)
        }
    }
    impl core::fmt::Debug for Opsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Opsr")
                .field("addr_op", &self.addr_op())
                .field("data_op", &self.data_op())
                .field("bk_op", &self.bk_op())
                .field("sysf_op", &self.sysf_op())
                .field("otp_op", &self.otp_op())
                .field("code_op", &self.code_op())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Opsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Opsr {{ addr_op: {=u16:?}, data_op: {=bool:?}, bk_op: {=bool:?}, sysf_op: {=bool:?}, otp_op: {=bool:?}, code_op: {:?} }}",
                self.addr_op(),
                self.data_op(),
                self.bk_op(),
                self.sysf_op(),
                self.otp_op(),
                self.code_op()
            )
        }
    }
    #[doc = "FLASH option control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optcr(pub u32);
    impl Optcr {
        #[doc = "OPTCR lock option configuration bit."]
        #[must_use]
        #[inline(always)]
        pub const fn optlock(&self) -> super::vals::Optlock {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Optlock::from_bits(val as u8)
        }
        #[doc = "OPTCR lock option configuration bit."]
        #[inline(always)]
        pub const fn set_optlock(&mut self, val: super::vals::Optlock) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Option-byte start change option configuration bit."]
        #[must_use]
        #[inline(always)]
        pub const fn optstrt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Option-byte start change option configuration bit."]
        #[inline(always)]
        pub const fn set_optstrt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Bank swapping option configuration bit."]
        #[must_use]
        #[inline(always)]
        pub const fn swap_bank(&self) -> super::vals::OptcrSwapBank {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::OptcrSwapBank::from_bits(val as u8)
        }
        #[doc = "Bank swapping option configuration bit."]
        #[inline(always)]
        pub const fn set_swap_bank(&mut self, val: super::vals::OptcrSwapBank) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Optcr {
        #[inline(always)]
        fn default() -> Optcr {
            Optcr(0)
        }
    }
    impl core::fmt::Debug for Optcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Optcr")
                .field("optlock", &self.optlock())
                .field("optstrt", &self.optstrt())
                .field("swap_bank", &self.swap_bank())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Optcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Optcr {{ optlock: {:?}, optstrt: {=bool:?}, swap_bank: {:?} }}",
                self.optlock(),
                self.optstrt(),
                self.swap_bank()
            )
        }
    }
    #[doc = "FLASH option key register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optkeyr(pub u32);
    impl Optkeyr {
        #[doc = "FLASH option-byte control access unlock key."]
        #[must_use]
        #[inline(always)]
        pub const fn optkey(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "FLASH option-byte control access unlock key."]
        #[inline(always)]
        pub const fn set_optkey(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Optkeyr {
        #[inline(always)]
        fn default() -> Optkeyr {
            Optkeyr(0)
        }
    }
    impl core::fmt::Debug for Optkeyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Optkeyr").field("optkey", &self.optkey()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Optkeyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Optkeyr {{ optkey: {=u32:?} }}", self.optkey())
        }
    }
    #[doc = "FLASH option status register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optsr2Cur(pub u32);
    impl Optsr2Cur {
        #[doc = "SRAM1 erase upon system reset."]
        #[must_use]
        #[inline(always)]
        pub const fn sram1_rst(&self) -> super::vals::Optsr2CurSram1Rst {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Optsr2CurSram1Rst::from_bits(val as u8)
        }
        #[doc = "SRAM1 erase upon system reset."]
        #[inline(always)]
        pub const fn set_sram1_rst(&mut self, val: super::vals::Optsr2CurSram1Rst) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "SRAM2 erase when system reset."]
        #[must_use]
        #[inline(always)]
        pub const fn sram2_rst(&self) -> super::vals::Optsr2CurSram2Rst {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Optsr2CurSram2Rst::from_bits(val as u8)
        }
        #[doc = "SRAM2 erase when system reset."]
        #[inline(always)]
        pub const fn set_sram2_rst(&mut self, val: super::vals::Optsr2CurSram2Rst) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "SRAM2 ECC detection and correction disable."]
        #[must_use]
        #[inline(always)]
        pub const fn sram2_ecc(&self) -> super::vals::Optsr2CurSram2Ecc {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Optsr2CurSram2Ecc::from_bits(val as u8)
        }
        #[doc = "SRAM2 ECC detection and correction disable."]
        #[inline(always)]
        pub const fn set_sram2_ecc(&mut self, val: super::vals::Optsr2CurSram2Ecc) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Optsr2Cur {
        #[inline(always)]
        fn default() -> Optsr2Cur {
            Optsr2Cur(0)
        }
    }
    impl core::fmt::Debug for Optsr2Cur {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Optsr2Cur")
                .field("sram1_rst", &self.sram1_rst())
                .field("sram2_rst", &self.sram2_rst())
                .field("sram2_ecc", &self.sram2_ecc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Optsr2Cur {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Optsr2Cur {{ sram1_rst: {:?}, sram2_rst: {:?}, sram2_ecc: {:?} }}",
                self.sram1_rst(),
                self.sram2_rst(),
                self.sram2_ecc()
            )
        }
    }
    #[doc = "FLASH option status register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optsr2Prg(pub u32);
    impl Optsr2Prg {
        #[doc = "SRAM1 erase upon system reset."]
        #[must_use]
        #[inline(always)]
        pub const fn sram1_rst(&self) -> super::vals::Optsr2PrgSram1Rst {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Optsr2PrgSram1Rst::from_bits(val as u8)
        }
        #[doc = "SRAM1 erase upon system reset."]
        #[inline(always)]
        pub const fn set_sram1_rst(&mut self, val: super::vals::Optsr2PrgSram1Rst) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "SRAM2 erase when system reset."]
        #[must_use]
        #[inline(always)]
        pub const fn sram2_rst(&self) -> super::vals::Optsr2PrgSram2Rst {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Optsr2PrgSram2Rst::from_bits(val as u8)
        }
        #[doc = "SRAM2 erase when system reset."]
        #[inline(always)]
        pub const fn set_sram2_rst(&mut self, val: super::vals::Optsr2PrgSram2Rst) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "SRAM2 ECC detection and correction disable."]
        #[must_use]
        #[inline(always)]
        pub const fn sram2_ecc(&self) -> super::vals::Optsr2PrgSram2Ecc {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Optsr2PrgSram2Ecc::from_bits(val as u8)
        }
        #[doc = "SRAM2 ECC detection and correction disable."]
        #[inline(always)]
        pub const fn set_sram2_ecc(&mut self, val: super::vals::Optsr2PrgSram2Ecc) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Optsr2Prg {
        #[inline(always)]
        fn default() -> Optsr2Prg {
            Optsr2Prg(0)
        }
    }
    impl core::fmt::Debug for Optsr2Prg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Optsr2Prg")
                .field("sram1_rst", &self.sram1_rst())
                .field("sram2_rst", &self.sram2_rst())
                .field("sram2_ecc", &self.sram2_ecc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Optsr2Prg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Optsr2Prg {{ sram1_rst: {:?}, sram2_rst: {:?}, sram2_ecc: {:?} }}",
                self.sram1_rst(),
                self.sram2_rst(),
                self.sram2_ecc()
            )
        }
    }
    #[doc = "FLASH option status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OptsrCur(pub u32);
    impl OptsrCur {
        #[doc = "IWDG control mode option status bit."]
        #[must_use]
        #[inline(always)]
        pub const fn iwdg_sw(&self) -> super::vals::OptsrCurIwdgSw {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::OptsrCurIwdgSw::from_bits(val as u8)
        }
        #[doc = "IWDG control mode option status bit."]
        #[inline(always)]
        pub const fn set_iwdg_sw(&mut self, val: super::vals::OptsrCurIwdgSw) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "WWDG control mode option status bit."]
        #[must_use]
        #[inline(always)]
        pub const fn wwdg_sw(&self) -> super::vals::OptsrCurWwdgSw {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::OptsrCurWwdgSw::from_bits(val as u8)
        }
        #[doc = "WWDG control mode option status bit."]
        #[inline(always)]
        pub const fn set_wwdg_sw(&mut self, val: super::vals::OptsrCurWwdgSw) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Core domain Stop entry reset option status bit."]
        #[must_use]
        #[inline(always)]
        pub const fn nrst_stop(&self) -> super::vals::OptsrCurNrstStop {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::OptsrCurNrstStop::from_bits(val as u8)
        }
        #[doc = "Core domain Stop entry reset option status bit."]
        #[inline(always)]
        pub const fn set_nrst_stop(&mut self, val: super::vals::OptsrCurNrstStop) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "Core domain Standby entry reset option status bit."]
        #[must_use]
        #[inline(always)]
        pub const fn nrst_stdby(&self) -> super::vals::OptsrCurNrstStdby {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::OptsrCurNrstStdby::from_bits(val as u8)
        }
        #[doc = "Core domain Standby entry reset option status bit."]
        #[inline(always)]
        pub const fn set_nrst_stdby(&mut self, val: super::vals::OptsrCurNrstStdby) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "RDP level code (based on Hamming 8,4). See Section7.5.8."]
        #[must_use]
        #[inline(always)]
        pub const fn rdp_level(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "RDP level code (based on Hamming 8,4). See Section7.5.8."]
        #[inline(always)]
        pub const fn set_rdp_level(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "IWDG Stop mode freeze option status bit."]
        #[must_use]
        #[inline(always)]
        pub const fn iwdg_stop(&self) -> super::vals::OptsrCurIwdgStop {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::OptsrCurIwdgStop::from_bits(val as u8)
        }
        #[doc = "IWDG Stop mode freeze option status bit."]
        #[inline(always)]
        pub const fn set_iwdg_stop(&mut self, val: super::vals::OptsrCurIwdgStop) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "IWDG Standby mode freeze option status bit."]
        #[must_use]
        #[inline(always)]
        pub const fn iwdg_stdby(&self) -> super::vals::OptsrCurIwdgStdby {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::OptsrCurIwdgStdby::from_bits(val as u8)
        }
        #[doc = "IWDG Standby mode freeze option status bit."]
        #[inline(always)]
        pub const fn set_iwdg_stdby(&mut self, val: super::vals::OptsrCurIwdgStdby) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Boot 0 source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn boot_sel(&self) -> super::vals::OptsrCurBootSel {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::OptsrCurBootSel::from_bits(val as u8)
        }
        #[doc = "Boot 0 source selection."]
        #[inline(always)]
        pub const fn set_boot_sel(&mut self, val: super::vals::OptsrCurBootSel) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Boot 0 option bit."]
        #[must_use]
        #[inline(always)]
        pub const fn boot0(&self) -> super::vals::OptsrCurBoot0 {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::OptsrCurBoot0::from_bits(val as u8)
        }
        #[doc = "Boot 0 option bit."]
        #[inline(always)]
        pub const fn set_boot0(&mut self, val: super::vals::OptsrCurBoot0) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "Flash data area enable."]
        #[must_use]
        #[inline(always)]
        pub const fn edata_en(&self) -> super::vals::OptsrCurEdataEn {
            let val = (self.0 >> 29usize) & 0x01;
            super::vals::OptsrCurEdataEn::from_bits(val as u8)
        }
        #[doc = "Flash data area enable."]
        #[inline(always)]
        pub const fn set_edata_en(&mut self, val: super::vals::OptsrCurEdataEn) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
        }
        #[doc = "Dual bank selection option status bit."]
        #[must_use]
        #[inline(always)]
        pub const fn dual_bank(&self) -> super::vals::OptsrCurDualBank {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::OptsrCurDualBank::from_bits(val as u8)
        }
        #[doc = "Dual bank selection option status bit."]
        #[inline(always)]
        pub const fn set_dual_bank(&mut self, val: super::vals::OptsrCurDualBank) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Dual bank selection option status bit."]
        #[must_use]
        #[inline(always)]
        pub const fn single_bank(&self) -> super::vals::OptsrCurSingleBank {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::OptsrCurSingleBank::from_bits(val as u8)
        }
        #[doc = "Dual bank selection option status bit."]
        #[inline(always)]
        pub const fn set_single_bank(&mut self, val: super::vals::OptsrCurSingleBank) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Bank swapping option status bit."]
        #[must_use]
        #[inline(always)]
        pub const fn swap_bank(&self) -> super::vals::OptsrCurSwapBank {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::OptsrCurSwapBank::from_bits(val as u8)
        }
        #[doc = "Bank swapping option status bit."]
        #[inline(always)]
        pub const fn set_swap_bank(&mut self, val: super::vals::OptsrCurSwapBank) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for OptsrCur {
        #[inline(always)]
        fn default() -> OptsrCur {
            OptsrCur(0)
        }
    }
    impl core::fmt::Debug for OptsrCur {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OptsrCur")
                .field("iwdg_sw", &self.iwdg_sw())
                .field("wwdg_sw", &self.wwdg_sw())
                .field("nrst_stop", &self.nrst_stop())
                .field("nrst_stdby", &self.nrst_stdby())
                .field("rdp_level", &self.rdp_level())
                .field("iwdg_stop", &self.iwdg_stop())
                .field("iwdg_stdby", &self.iwdg_stdby())
                .field("boot_sel", &self.boot_sel())
                .field("boot0", &self.boot0())
                .field("edata_en", &self.edata_en())
                .field("dual_bank", &self.dual_bank())
                .field("single_bank", &self.single_bank())
                .field("swap_bank", &self.swap_bank())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OptsrCur {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OptsrCur {{ iwdg_sw: {:?}, wwdg_sw: {:?}, nrst_stop: {:?}, nrst_stdby: {:?}, rdp_level: {=u8:?}, iwdg_stop: {:?}, iwdg_stdby: {:?}, boot_sel: {:?}, boot0: {:?}, edata_en: {:?}, dual_bank: {:?}, single_bank: {:?}, swap_bank: {:?} }}",
                self.iwdg_sw(),
                self.wwdg_sw(),
                self.nrst_stop(),
                self.nrst_stdby(),
                self.rdp_level(),
                self.iwdg_stop(),
                self.iwdg_stdby(),
                self.boot_sel(),
                self.boot0(),
                self.edata_en(),
                self.dual_bank(),
                self.single_bank(),
                self.swap_bank()
            )
        }
    }
    #[doc = "FLASH option status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OptsrPrg(pub u32);
    impl OptsrPrg {
        #[doc = "IWDG control mode option configuration bit."]
        #[must_use]
        #[inline(always)]
        pub const fn iwdg_sw(&self) -> super::vals::OptsrPrgIwdgSw {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::OptsrPrgIwdgSw::from_bits(val as u8)
        }
        #[doc = "IWDG control mode option configuration bit."]
        #[inline(always)]
        pub const fn set_iwdg_sw(&mut self, val: super::vals::OptsrPrgIwdgSw) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "WWDG control mode option configuration bit."]
        #[must_use]
        #[inline(always)]
        pub const fn wwdg_sw(&self) -> super::vals::OptsrPrgWwdgSw {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::OptsrPrgWwdgSw::from_bits(val as u8)
        }
        #[doc = "WWDG control mode option configuration bit."]
        #[inline(always)]
        pub const fn set_wwdg_sw(&mut self, val: super::vals::OptsrPrgWwdgSw) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Core domain Stop entry reset option configuration bit."]
        #[must_use]
        #[inline(always)]
        pub const fn nrst_stop(&self) -> super::vals::OptsrPrgNrstStop {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::OptsrPrgNrstStop::from_bits(val as u8)
        }
        #[doc = "Core domain Stop entry reset option configuration bit."]
        #[inline(always)]
        pub const fn set_nrst_stop(&mut self, val: super::vals::OptsrPrgNrstStop) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "Core domain Standby entry reset option configuration bit."]
        #[must_use]
        #[inline(always)]
        pub const fn nrst_stdby(&self) -> super::vals::OptsrPrgNrstStdby {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::OptsrPrgNrstStdby::from_bits(val as u8)
        }
        #[doc = "Core domain Standby entry reset option configuration bit."]
        #[inline(always)]
        pub const fn set_nrst_stdby(&mut self, val: super::vals::OptsrPrgNrstStdby) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "RDP level code (based on Hamming 8,4). See Section7.5.8."]
        #[must_use]
        #[inline(always)]
        pub const fn rdp_level(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "RDP level code (based on Hamming 8,4). See Section7.5.8."]
        #[inline(always)]
        pub const fn set_rdp_level(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "IWDG Stop mode freeze option configuration bit."]
        #[must_use]
        #[inline(always)]
        pub const fn iwdg_stop(&self) -> super::vals::OptsrPrgIwdgStop {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::OptsrPrgIwdgStop::from_bits(val as u8)
        }
        #[doc = "IWDG Stop mode freeze option configuration bit."]
        #[inline(always)]
        pub const fn set_iwdg_stop(&mut self, val: super::vals::OptsrPrgIwdgStop) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "IWDG Standby mode freeze option configuration bit."]
        #[must_use]
        #[inline(always)]
        pub const fn iwdg_stdby(&self) -> super::vals::OptsrPrgIwdgStdby {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::OptsrPrgIwdgStdby::from_bits(val as u8)
        }
        #[doc = "IWDG Standby mode freeze option configuration bit."]
        #[inline(always)]
        pub const fn set_iwdg_stdby(&mut self, val: super::vals::OptsrPrgIwdgStdby) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Boot 0 source configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn boot_sel(&self) -> super::vals::OptsrPrgBootSel {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::OptsrPrgBootSel::from_bits(val as u8)
        }
        #[doc = "Boot 0 source configuration."]
        #[inline(always)]
        pub const fn set_boot_sel(&mut self, val: super::vals::OptsrPrgBootSel) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Boot 0 option bit."]
        #[must_use]
        #[inline(always)]
        pub const fn boot0(&self) -> super::vals::OptsrPrgBoot0 {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::OptsrPrgBoot0::from_bits(val as u8)
        }
        #[doc = "Boot 0 option bit."]
        #[inline(always)]
        pub const fn set_boot0(&mut self, val: super::vals::OptsrPrgBoot0) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "Flash data area enable."]
        #[must_use]
        #[inline(always)]
        pub const fn edata_en(&self) -> super::vals::OptsrPrgEdataEn {
            let val = (self.0 >> 29usize) & 0x01;
            super::vals::OptsrPrgEdataEn::from_bits(val as u8)
        }
        #[doc = "Flash data area enable."]
        #[inline(always)]
        pub const fn set_edata_en(&mut self, val: super::vals::OptsrPrgEdataEn) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
        }
        #[doc = "Dual bank option configuration bit."]
        #[must_use]
        #[inline(always)]
        pub const fn dual_bank(&self) -> super::vals::OptsrPrgDualBank {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::OptsrPrgDualBank::from_bits(val as u8)
        }
        #[doc = "Dual bank option configuration bit."]
        #[inline(always)]
        pub const fn set_dual_bank(&mut self, val: super::vals::OptsrPrgDualBank) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Dual bank option configuration bit."]
        #[must_use]
        #[inline(always)]
        pub const fn single_bank(&self) -> super::vals::OptsrPrgSingleBank {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::OptsrPrgSingleBank::from_bits(val as u8)
        }
        #[doc = "Dual bank option configuration bit."]
        #[inline(always)]
        pub const fn set_single_bank(&mut self, val: super::vals::OptsrPrgSingleBank) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Bank swapping option configuration bit."]
        #[must_use]
        #[inline(always)]
        pub const fn swap_bank(&self) -> super::vals::OptsrPrgSwapBank {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::OptsrPrgSwapBank::from_bits(val as u8)
        }
        #[doc = "Bank swapping option configuration bit."]
        #[inline(always)]
        pub const fn set_swap_bank(&mut self, val: super::vals::OptsrPrgSwapBank) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for OptsrPrg {
        #[inline(always)]
        fn default() -> OptsrPrg {
            OptsrPrg(0)
        }
    }
    impl core::fmt::Debug for OptsrPrg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OptsrPrg")
                .field("iwdg_sw", &self.iwdg_sw())
                .field("wwdg_sw", &self.wwdg_sw())
                .field("nrst_stop", &self.nrst_stop())
                .field("nrst_stdby", &self.nrst_stdby())
                .field("rdp_level", &self.rdp_level())
                .field("iwdg_stop", &self.iwdg_stop())
                .field("iwdg_stdby", &self.iwdg_stdby())
                .field("boot_sel", &self.boot_sel())
                .field("boot0", &self.boot0())
                .field("edata_en", &self.edata_en())
                .field("dual_bank", &self.dual_bank())
                .field("single_bank", &self.single_bank())
                .field("swap_bank", &self.swap_bank())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OptsrPrg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "OptsrPrg {{ iwdg_sw: {:?}, wwdg_sw: {:?}, nrst_stop: {:?}, nrst_stdby: {:?}, rdp_level: {=u8:?}, iwdg_stop: {:?}, iwdg_stdby: {:?}, boot_sel: {:?}, boot0: {:?}, edata_en: {:?}, dual_bank: {:?}, single_bank: {:?}, swap_bank: {:?} }}",
                self.iwdg_sw(),
                self.wwdg_sw(),
                self.nrst_stop(),
                self.nrst_stdby(),
                self.rdp_level(),
                self.iwdg_stop(),
                self.iwdg_stdby(),
                self.boot_sel(),
                self.boot0(),
                self.edata_en(),
                self.dual_bank(),
                self.single_bank(),
                self.swap_bank()
            )
        }
    }
    #[doc = "FLASH OTP block lock."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OtpblrCur(pub u32);
    impl OtpblrCur {
        #[doc = "OTP block lock."]
        #[must_use]
        #[inline(always)]
        pub const fn lockbl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "OTP block lock."]
        #[inline(always)]
        pub const fn set_lockbl(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for OtpblrCur {
        #[inline(always)]
        fn default() -> OtpblrCur {
            OtpblrCur(0)
        }
    }
    impl core::fmt::Debug for OtpblrCur {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OtpblrCur").field("lockbl", &self.lockbl()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OtpblrCur {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OtpblrCur {{ lockbl: {=u32:?} }}", self.lockbl())
        }
    }
    #[doc = "FLASH OTP block lock."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OtpblrPrg(pub u32);
    impl OtpblrPrg {
        #[doc = "OTP block lock."]
        #[must_use]
        #[inline(always)]
        pub const fn lockbl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "OTP block lock."]
        #[inline(always)]
        pub const fn set_lockbl(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for OtpblrPrg {
        #[inline(always)]
        fn default() -> OtpblrPrg {
            OtpblrPrg(0)
        }
    }
    impl core::fmt::Debug for OtpblrPrg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OtpblrPrg").field("lockbl", &self.lockbl()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OtpblrPrg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "OtpblrPrg {{ lockbl: {=u32:?} }}", self.lockbl())
        }
    }
    #[doc = "FLASH privilege configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcfgr(pub u32);
    impl Privcfgr {
        #[doc = "privilege attribute."]
        #[must_use]
        #[inline(always)]
        pub const fn priv_(&self) -> super::vals::Priv {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Priv::from_bits(val as u8)
        }
        #[doc = "privilege attribute."]
        #[inline(always)]
        pub const fn set_priv_(&mut self, val: super::vals::Priv) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
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
            defmt::write!(f, "Privcfgr {{ priv_: {:?} }}", self.priv_())
        }
    }
    #[doc = "FLASH status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "busy flag."]
        #[must_use]
        #[inline(always)]
        pub const fn bsy(&self) -> super::vals::Bsy {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Bsy::from_bits(val as u8)
        }
        #[doc = "busy flag."]
        #[inline(always)]
        pub const fn set_bsy(&mut self, val: super::vals::Bsy) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "write buffer not empty flag."]
        #[must_use]
        #[inline(always)]
        pub const fn wbne(&self) -> super::vals::Wbne {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Wbne::from_bits(val as u8)
        }
        #[doc = "write buffer not empty flag."]
        #[inline(always)]
        pub const fn set_wbne(&mut self, val: super::vals::Wbne) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "data buffer not empty flag."]
        #[must_use]
        #[inline(always)]
        pub const fn dbne(&self) -> super::vals::Dbne {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Dbne::from_bits(val as u8)
        }
        #[doc = "data buffer not empty flag."]
        #[inline(always)]
        pub const fn set_dbne(&mut self, val: super::vals::Dbne) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "OEM lock."]
        #[must_use]
        #[inline(always)]
        pub const fn oemlock(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "OEM lock."]
        #[inline(always)]
        pub const fn set_oemlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "BS lock."]
        #[must_use]
        #[inline(always)]
        pub const fn bslock(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "BS lock."]
        #[inline(always)]
        pub const fn set_bslock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "end of operation flag."]
        #[must_use]
        #[inline(always)]
        pub const fn eop(&self) -> super::vals::Eop {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Eop::from_bits(val as u8)
        }
        #[doc = "end of operation flag."]
        #[inline(always)]
        pub const fn set_eop(&mut self, val: super::vals::Eop) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "write protection error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn wrperr(&self) -> super::vals::Wrperr {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Wrperr::from_bits(val as u8)
        }
        #[doc = "write protection error flag."]
        #[inline(always)]
        pub const fn set_wrperr(&mut self, val: super::vals::Wrperr) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "programming sequence error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn pgserr(&self) -> super::vals::Pgserr {
            let val = (self.0 >> 18usize) & 0x01;
            super::vals::Pgserr::from_bits(val as u8)
        }
        #[doc = "programming sequence error flag."]
        #[inline(always)]
        pub const fn set_pgserr(&mut self, val: super::vals::Pgserr) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
        }
        #[doc = "strobe error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn strberr(&self) -> super::vals::Strberr {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Strberr::from_bits(val as u8)
        }
        #[doc = "strobe error flag."]
        #[inline(always)]
        pub const fn set_strberr(&mut self, val: super::vals::Strberr) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "Inconsistency error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn incerr(&self) -> super::vals::Incerr {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::Incerr::from_bits(val as u8)
        }
        #[doc = "Inconsistency error flag."]
        #[inline(always)]
        pub const fn set_incerr(&mut self, val: super::vals::Incerr) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Option-byte change error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn optchangeerr(&self) -> super::vals::Optchangeerr {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::Optchangeerr::from_bits(val as u8)
        }
        #[doc = "Option-byte change error flag."]
        #[inline(always)]
        pub const fn set_optchangeerr(&mut self, val: super::vals::Optchangeerr) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
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
                .field("bsy", &self.bsy())
                .field("wbne", &self.wbne())
                .field("dbne", &self.dbne())
                .field("oemlock", &self.oemlock())
                .field("bslock", &self.bslock())
                .field("eop", &self.eop())
                .field("wrperr", &self.wrperr())
                .field("pgserr", &self.pgserr())
                .field("strberr", &self.strberr())
                .field("incerr", &self.incerr())
                .field("optchangeerr", &self.optchangeerr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ bsy: {:?}, wbne: {:?}, dbne: {:?}, oemlock: {=bool:?}, bslock: {=bool:?}, eop: {:?}, wrperr: {:?}, pgserr: {:?}, strberr: {:?}, incerr: {:?}, optchangeerr: {:?} }}",
                self.bsy(),
                self.wbne(),
                self.dbne(),
                self.oemlock(),
                self.bslock(),
                self.eop(),
                self.wrperr(),
                self.pgserr(),
                self.strberr(),
                self.incerr(),
                self.optchangeerr()
            )
        }
    }
    #[doc = "FLASH write page protection for bank1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrp1rCur(pub u32);
    impl Wrp1rCur {
        #[doc = "Bank1 page protection option status byte."]
        #[must_use]
        #[inline(always)]
        pub const fn wrpsg1(&self) -> super::vals::Wrp1rCurWrpsg1 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            super::vals::Wrp1rCurWrpsg1::from_bits(val as u32)
        }
        #[doc = "Bank1 page protection option status byte."]
        #[inline(always)]
        pub const fn set_wrpsg1(&mut self, val: super::vals::Wrp1rCurWrpsg1) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Wrp1rCur {
        #[inline(always)]
        fn default() -> Wrp1rCur {
            Wrp1rCur(0)
        }
    }
    impl core::fmt::Debug for Wrp1rCur {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wrp1rCur").field("wrpsg1", &self.wrpsg1()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrp1rCur {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wrp1rCur {{ wrpsg1: {:?} }}", self.wrpsg1())
        }
    }
    #[doc = "FLASH write page protection for bank1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrp1rPrg(pub u32);
    impl Wrp1rPrg {
        #[doc = "Bank1 page protection option status byte."]
        #[must_use]
        #[inline(always)]
        pub const fn wrpsg1(&self) -> super::vals::Wrp1rPrgWrpsg1 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            super::vals::Wrp1rPrgWrpsg1::from_bits(val as u32)
        }
        #[doc = "Bank1 page protection option status byte."]
        #[inline(always)]
        pub const fn set_wrpsg1(&mut self, val: super::vals::Wrp1rPrgWrpsg1) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Wrp1rPrg {
        #[inline(always)]
        fn default() -> Wrp1rPrg {
            Wrp1rPrg(0)
        }
    }
    impl core::fmt::Debug for Wrp1rPrg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wrp1rPrg").field("wrpsg1", &self.wrpsg1()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrp1rPrg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wrp1rPrg {{ wrpsg1: {:?} }}", self.wrpsg1())
        }
    }
    #[doc = "FLASH write page protection for bank2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrp2rCur(pub u32);
    impl Wrp2rCur {
        #[doc = "Bank2 page protection option status byte."]
        #[must_use]
        #[inline(always)]
        pub const fn wrpsg2(&self) -> super::vals::Wrp2rCurWrpsg2 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            super::vals::Wrp2rCurWrpsg2::from_bits(val as u32)
        }
        #[doc = "Bank2 page protection option status byte."]
        #[inline(always)]
        pub const fn set_wrpsg2(&mut self, val: super::vals::Wrp2rCurWrpsg2) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Wrp2rCur {
        #[inline(always)]
        fn default() -> Wrp2rCur {
            Wrp2rCur(0)
        }
    }
    impl core::fmt::Debug for Wrp2rCur {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wrp2rCur").field("wrpsg2", &self.wrpsg2()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrp2rCur {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wrp2rCur {{ wrpsg2: {:?} }}", self.wrpsg2())
        }
    }
    #[doc = "FLASH write page protection for bank2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrp2rPrg(pub u32);
    impl Wrp2rPrg {
        #[doc = "Bank2 page protection option status byte."]
        #[must_use]
        #[inline(always)]
        pub const fn wrpsg2(&self) -> super::vals::Wrp2rPrgWrpsg2 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            super::vals::Wrp2rPrgWrpsg2::from_bits(val as u32)
        }
        #[doc = "Bank2 page protection option status byte."]
        #[inline(always)]
        pub const fn set_wrpsg2(&mut self, val: super::vals::Wrp2rPrgWrpsg2) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Wrp2rPrg {
        #[inline(always)]
        fn default() -> Wrp2rPrg {
            Wrp2rPrg(0)
        }
    }
    impl core::fmt::Debug for Wrp2rPrg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wrp2rPrg").field("wrpsg2", &self.wrpsg2()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrp2rPrg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wrp2rPrg {{ wrpsg2: {:?} }}", self.wrpsg2())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ber {
        #[doc = "bank erase not requested."]
        B0x0 = 0x0,
        #[doc = "bank erase requested."]
        B0x1 = 0x01,
    }
    impl Ber {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ber {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ber {
        #[inline(always)]
        fn from(val: u8) -> Ber {
            Ber::from_bits(val)
        }
    }
    impl From<Ber> for u8 {
        #[inline(always)]
        fn from(val: Ber) -> u8 {
            Ber::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Bksel {
        #[doc = "Bank1 is selected for bank erase (BER)/page erase (PER)/interrupt enable."]
        B0x0 = 0x0,
        #[doc = "Bank2 is selected for BER/PER."]
        B0x1 = 0x01,
    }
    impl Bksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bksel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Bksel {
        #[inline(always)]
        fn from(val: u8) -> Bksel {
            Bksel::from_bits(val)
        }
    }
    impl From<Bksel> for u8 {
        #[inline(always)]
        fn from(val: Bksel) -> u8 {
            Bksel::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct BootrCurBootLock(u8);
    impl BootrCurBootLock {
        #[doc = "The BOOT0, BOOT_SEL, SWAP_BANK and BOOTADD are frozen."]
        pub const B0xB4: Self = Self(0xb4);
        #[doc = "The BOOT0, BOOT_SEL, SWAP_BANK and BOOTADD can still be modified following their individual rules."]
        pub const B0xC3: Self = Self(0xc3);
    }
    impl BootrCurBootLock {
        pub const fn from_bits(val: u8) -> BootrCurBootLock {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for BootrCurBootLock {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0xb4 => f.write_str("B0xB4"),
                0xc3 => f.write_str("B0xC3"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BootrCurBootLock {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0xb4 => defmt::write!(f, "B0xB4"),
                0xc3 => defmt::write!(f, "B0xC3"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for BootrCurBootLock {
        #[inline(always)]
        fn from(val: u8) -> BootrCurBootLock {
            BootrCurBootLock::from_bits(val)
        }
    }
    impl From<BootrCurBootLock> for u8 {
        #[inline(always)]
        fn from(val: BootrCurBootLock) -> u8 {
            BootrCurBootLock::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct BootrPrgBootLock(u8);
    impl BootrPrgBootLock {
        #[doc = "The BOOT0, BOOT_SEL, SWAP_BANK and BOOTADD are frozen."]
        pub const B0xB4: Self = Self(0xb4);
        #[doc = "The BOOT0, BOOT_SEL, SWAP_BANK and BOOTADD can still be modified following their individual rules."]
        pub const B0xC3: Self = Self(0xc3);
    }
    impl BootrPrgBootLock {
        pub const fn from_bits(val: u8) -> BootrPrgBootLock {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl core::fmt::Debug for BootrPrgBootLock {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0xb4 => f.write_str("B0xB4"),
                0xc3 => f.write_str("B0xC3"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BootrPrgBootLock {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0xb4 => defmt::write!(f, "B0xB4"),
                0xc3 => defmt::write!(f, "B0xC3"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for BootrPrgBootLock {
        #[inline(always)]
        fn from(val: u8) -> BootrPrgBootLock {
            BootrPrgBootLock::from_bits(val)
        }
    }
    impl From<BootrPrgBootLock> for u8 {
        #[inline(always)]
        fn from(val: BootrPrgBootLock) -> u8 {
            BootrPrgBootLock::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Bsy {
        #[doc = "no programming, erase or option byte change operation being executed."]
        B0x0 = 0x0,
        #[doc = "programming, erase or option byte change operation being executed."]
        B0x1 = 0x01,
    }
    impl Bsy {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bsy {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Bsy {
        #[inline(always)]
        fn from(val: u8) -> Bsy {
            Bsy::from_bits(val)
        }
    }
    impl From<Bsy> for u8 {
        #[inline(always)]
        fn from(val: Bsy) -> u8 {
            Bsy::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum CodeOp {
        #[doc = "No FLASH operation ongoing during previous reset."]
        B0x0 = 0x0,
        #[doc = "Single write operation interrupted."]
        B0x1 = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "Page erase operation interrupted."]
        B0x3 = 0x03,
        #[doc = "Bank erase operation interrupted."]
        B0x4 = 0x04,
        #[doc = "Mass erase operation interrupted."]
        B0x5 = 0x05,
        #[doc = "Option change operation interrupted."]
        B0x6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl CodeOp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CodeOp {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CodeOp {
        #[inline(always)]
        fn from(val: u8) -> CodeOp {
            CodeOp::from_bits(val)
        }
    }
    impl From<CodeOp> for u8 {
        #[inline(always)]
        fn from(val: CodeOp) -> u8 {
            CodeOp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum DataAddrEcc {
        #[doc = "Double error on first 16bits data or first 32 bits data accessed on the FLASH line."]
        B0x0 = 0x0,
        #[doc = "Double error on second 16bits data accessed on the FLASH line."]
        B0x1 = 0x01,
        #[doc = "Double error on third 16bits data or second 32 bits data accessed on the FLASH line."]
        B0x2 = 0x02,
        #[doc = "Double error on fourth 16bits data accessed on the FLASH line."]
        B0x3 = 0x03,
        #[doc = "Double error on fifth 16bits data or third 32 bits data accessed on the FLASH line."]
        B0x4 = 0x04,
        #[doc = "Double error on sixth 16bits data."]
        B0x5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl DataAddrEcc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> DataAddrEcc {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for DataAddrEcc {
        #[inline(always)]
        fn from(val: u8) -> DataAddrEcc {
            DataAddrEcc::from_bits(val)
        }
    }
    impl From<DataAddrEcc> for u8 {
        #[inline(always)]
        fn from(val: DataAddrEcc) -> u8 {
            DataAddrEcc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Dbne {
        #[doc = "data buffer not used."]
        B0x0 = 0x0,
        #[doc = "data buffer used, wait."]
        B0x1 = 0x01,
    }
    impl Dbne {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dbne {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dbne {
        #[inline(always)]
        fn from(val: u8) -> Dbne {
            Dbne::from_bits(val)
        }
    }
    impl From<Dbne> for u8 {
        #[inline(always)]
        fn from(val: Dbne) -> u8 {
            Dbne::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ecccie {
        #[doc = "no interrupt generated when an ECC single correction error occurs."]
        B0x0 = 0x0,
        #[doc = "interrupt generated when an ECC single correction error occurs."]
        B0x1 = 0x01,
    }
    impl Ecccie {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ecccie {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ecccie {
        #[inline(always)]
        fn from(val: u8) -> Ecccie {
            Ecccie::from_bits(val)
        }
    }
    impl From<Ecccie> for u8 {
        #[inline(always)]
        fn from(val: Ecccie) -> u8 {
            Ecccie::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Edatasel {
        #[doc = "Main flash page erase."]
        B0x0 = 0x0,
        #[doc = "Flash data area EDATA page erase."]
        B0x1 = 0x01,
    }
    impl Edatasel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Edatasel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Edatasel {
        #[inline(always)]
        fn from(val: u8) -> Edatasel {
            Edatasel::from_bits(val)
        }
    }
    impl From<Edatasel> for u8 {
        #[inline(always)]
        fn from(val: Edatasel) -> u8 {
            Edatasel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Empty {
        #[doc = "Boot address in Main Flash memory area programmed."]
        B0x0 = 0x0,
        #[doc = "Boot address in Main Flash memory area empty."]
        B0x1 = 0x01,
    }
    impl Empty {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Empty {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Empty {
        #[inline(always)]
        fn from(val: u8) -> Empty {
            Empty::from_bits(val)
        }
    }
    impl From<Empty> for u8 {
        #[inline(always)]
        fn from(val: Empty) -> u8 {
            Empty::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Eop {
        #[doc = "no operation completed."]
        B0x0 = 0x0,
        #[doc = "a operation completed."]
        B0x1 = 0x01,
    }
    impl Eop {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Eop {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Eop {
        #[inline(always)]
        fn from(val: u8) -> Eop {
            Eop::from_bits(val)
        }
    }
    impl From<Eop> for u8 {
        #[inline(always)]
        fn from(val: Eop) -> u8 {
            Eop::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Eopie {
        #[doc = "no interrupt generated at the end of operation."]
        B0x0 = 0x0,
        #[doc = "interrupt enabled when at the end of operation."]
        B0x1 = 0x01,
    }
    impl Eopie {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Eopie {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Eopie {
        #[inline(always)]
        fn from(val: u8) -> Eopie {
            Eopie::from_bits(val)
        }
    }
    impl From<Eopie> for u8 {
        #[inline(always)]
        fn from(val: Eopie) -> u8 {
            Eopie::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Incerr {
        #[doc = "no inconsistency error occurs."]
        B0x0 = 0x0,
        #[doc = "a inconsistency error occurs."]
        B0x1 = 0x01,
    }
    impl Incerr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Incerr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Incerr {
        #[inline(always)]
        fn from(val: u8) -> Incerr {
            Incerr::from_bits(val)
        }
    }
    impl From<Incerr> for u8 {
        #[inline(always)]
        fn from(val: Incerr) -> u8 {
            Incerr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Incerrie {
        #[doc = "no interrupt generated when a inconsistency error occurs."]
        B0x0 = 0x0,
        #[doc = "interrupt generated when a inconsistency error occurs."]
        B0x1 = 0x01,
    }
    impl Incerrie {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Incerrie {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Incerrie {
        #[inline(always)]
        fn from(val: u8) -> Incerrie {
            Incerrie::from_bits(val)
        }
    }
    impl From<Incerrie> for u8 {
        #[inline(always)]
        fn from(val: Incerrie) -> u8 {
            Incerrie::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Latency {
        #[doc = "zero wait state used to read a word from non-volatile memory."]
        B0x0 = 0x0,
        #[doc = "one wait state used to read a word from non-volatile memory."]
        B0x1 = 0x01,
        #[doc = "two wait states used to read a word from non-volatile memory."]
        B0x2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        #[doc = "seven wait states used to read a word from non-volatile memory."]
        B0x7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        #[doc = "15 wait states used to read from non-volatile memory."]
        B0xF = 0x0f,
    }
    impl Latency {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Latency {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Latency {
        #[inline(always)]
        fn from(val: u8) -> Latency {
            Latency::from_bits(val)
        }
    }
    impl From<Latency> for u8 {
        #[inline(always)]
        fn from(val: Latency) -> u8 {
            Latency::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lock {
        #[doc = "CR register unlocked."]
        B0x0 = 0x0,
        #[doc = "CR register locked."]
        B0x1 = 0x01,
    }
    impl Lock {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lock {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lock {
        #[inline(always)]
        fn from(val: u8) -> Lock {
            Lock::from_bits(val)
        }
    }
    impl From<Lock> for u8 {
        #[inline(always)]
        fn from(val: Lock) -> u8 {
            Lock::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mer {
        #[doc = "mass erase not requested."]
        B0x0 = 0x0,
        #[doc = "mass erase requested."]
        B0x1 = 0x01,
    }
    impl Mer {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mer {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mer {
        #[inline(always)]
        fn from(val: u8) -> Mer {
            Mer::from_bits(val)
        }
    }
    impl From<Mer> for u8 {
        #[inline(always)]
        fn from(val: Mer) -> u8 {
            Mer::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Optchangeerr {
        #[doc = "no option-byte change errors occurred."]
        B0x0 = 0x0,
        #[doc = "one or more errors occurred during an option-byte change operation."]
        B0x1 = 0x01,
    }
    impl Optchangeerr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Optchangeerr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Optchangeerr {
        #[inline(always)]
        fn from(val: u8) -> Optchangeerr {
            Optchangeerr::from_bits(val)
        }
    }
    impl From<Optchangeerr> for u8 {
        #[inline(always)]
        fn from(val: Optchangeerr) -> u8 {
            Optchangeerr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Optchangeerrie {
        #[doc = "no interrupt is generated when an error occurs during an option-byte change."]
        B0x0 = 0x0,
        #[doc = "an interrupt is generated when and error occurs during an option-byte change."]
        B0x1 = 0x01,
    }
    impl Optchangeerrie {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Optchangeerrie {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Optchangeerrie {
        #[inline(always)]
        fn from(val: u8) -> Optchangeerrie {
            Optchangeerrie::from_bits(val)
        }
    }
    impl From<Optchangeerrie> for u8 {
        #[inline(always)]
        fn from(val: Optchangeerrie) -> u8 {
            Optchangeerrie::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptcrSwapBank {
        #[doc = "Bank1 and bank2 not swapped."]
        B0x0 = 0x0,
        #[doc = "Bank1 and bank2 swapped."]
        B0x1 = 0x01,
    }
    impl OptcrSwapBank {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptcrSwapBank {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptcrSwapBank {
        #[inline(always)]
        fn from(val: u8) -> OptcrSwapBank {
            OptcrSwapBank::from_bits(val)
        }
    }
    impl From<OptcrSwapBank> for u8 {
        #[inline(always)]
        fn from(val: OptcrSwapBank) -> u8 {
            OptcrSwapBank::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Optlock {
        #[doc = "OPTCR register unlocked."]
        B0x0 = 0x0,
        #[doc = "OPTCR register locked."]
        B0x1 = 0x01,
    }
    impl Optlock {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Optlock {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Optlock {
        #[inline(always)]
        fn from(val: u8) -> Optlock {
            Optlock::from_bits(val)
        }
    }
    impl From<Optlock> for u8 {
        #[inline(always)]
        fn from(val: Optlock) -> u8 {
            Optlock::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Optsr2CurSram1Rst {
        #[doc = "SRAM1 erased when a system reset occurs."]
        B0x0 = 0x0,
        #[doc = "SRAM1 not erased when a system reset occurs."]
        B0x1 = 0x01,
    }
    impl Optsr2CurSram1Rst {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Optsr2CurSram1Rst {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Optsr2CurSram1Rst {
        #[inline(always)]
        fn from(val: u8) -> Optsr2CurSram1Rst {
            Optsr2CurSram1Rst::from_bits(val)
        }
    }
    impl From<Optsr2CurSram1Rst> for u8 {
        #[inline(always)]
        fn from(val: Optsr2CurSram1Rst) -> u8 {
            Optsr2CurSram1Rst::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Optsr2CurSram2Ecc {
        #[doc = "SRAM2 ECC check enabled."]
        B0x0 = 0x0,
        #[doc = "SRAM2 ECC check disabled."]
        B0x1 = 0x01,
    }
    impl Optsr2CurSram2Ecc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Optsr2CurSram2Ecc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Optsr2CurSram2Ecc {
        #[inline(always)]
        fn from(val: u8) -> Optsr2CurSram2Ecc {
            Optsr2CurSram2Ecc::from_bits(val)
        }
    }
    impl From<Optsr2CurSram2Ecc> for u8 {
        #[inline(always)]
        fn from(val: Optsr2CurSram2Ecc) -> u8 {
            Optsr2CurSram2Ecc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Optsr2CurSram2Rst {
        #[doc = "SRAM2 erased when a system reset occurs."]
        B0x0 = 0x0,
        #[doc = "SRAM2 not erased when a system reset occurs."]
        B0x1 = 0x01,
    }
    impl Optsr2CurSram2Rst {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Optsr2CurSram2Rst {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Optsr2CurSram2Rst {
        #[inline(always)]
        fn from(val: u8) -> Optsr2CurSram2Rst {
            Optsr2CurSram2Rst::from_bits(val)
        }
    }
    impl From<Optsr2CurSram2Rst> for u8 {
        #[inline(always)]
        fn from(val: Optsr2CurSram2Rst) -> u8 {
            Optsr2CurSram2Rst::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Optsr2PrgSram1Rst {
        #[doc = "SRAM1 erased when a system reset occurs."]
        B0x0 = 0x0,
        #[doc = "SRAM1 not erased when a system reset occurs."]
        B0x1 = 0x01,
    }
    impl Optsr2PrgSram1Rst {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Optsr2PrgSram1Rst {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Optsr2PrgSram1Rst {
        #[inline(always)]
        fn from(val: u8) -> Optsr2PrgSram1Rst {
            Optsr2PrgSram1Rst::from_bits(val)
        }
    }
    impl From<Optsr2PrgSram1Rst> for u8 {
        #[inline(always)]
        fn from(val: Optsr2PrgSram1Rst) -> u8 {
            Optsr2PrgSram1Rst::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Optsr2PrgSram2Ecc {
        #[doc = "SRAM2 ECC check enabled."]
        B0x0 = 0x0,
        #[doc = "SRAM2 ECC check disabled."]
        B0x1 = 0x01,
    }
    impl Optsr2PrgSram2Ecc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Optsr2PrgSram2Ecc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Optsr2PrgSram2Ecc {
        #[inline(always)]
        fn from(val: u8) -> Optsr2PrgSram2Ecc {
            Optsr2PrgSram2Ecc::from_bits(val)
        }
    }
    impl From<Optsr2PrgSram2Ecc> for u8 {
        #[inline(always)]
        fn from(val: Optsr2PrgSram2Ecc) -> u8 {
            Optsr2PrgSram2Ecc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Optsr2PrgSram2Rst {
        #[doc = "SRAM2 erased when a system reset occurs."]
        B0x0 = 0x0,
        #[doc = "SRAM2 not erased when a system reset occurs."]
        B0x1 = 0x01,
    }
    impl Optsr2PrgSram2Rst {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Optsr2PrgSram2Rst {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Optsr2PrgSram2Rst {
        #[inline(always)]
        fn from(val: u8) -> Optsr2PrgSram2Rst {
            Optsr2PrgSram2Rst::from_bits(val)
        }
    }
    impl From<Optsr2PrgSram2Rst> for u8 {
        #[inline(always)]
        fn from(val: Optsr2PrgSram2Rst) -> u8 {
            Optsr2PrgSram2Rst::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrCurBoot0 {
        #[doc = "BOOT0 = 0."]
        B0x0 = 0x0,
        #[doc = "BOOT0 = 1."]
        B0x1 = 0x01,
    }
    impl OptsrCurBoot0 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrCurBoot0 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrCurBoot0 {
        #[inline(always)]
        fn from(val: u8) -> OptsrCurBoot0 {
            OptsrCurBoot0::from_bits(val)
        }
    }
    impl From<OptsrCurBoot0> for u8 {
        #[inline(always)]
        fn from(val: OptsrCurBoot0) -> u8 {
            OptsrCurBoot0::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrCurBootSel {
        #[doc = "BOOT0 signal is defined by the BOOT0 option bit."]
        B0x0 = 0x0,
        #[doc = "BOOT0 signal is defined by BOOT0 pin value (legacy mode)."]
        B0x1 = 0x01,
    }
    impl OptsrCurBootSel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrCurBootSel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrCurBootSel {
        #[inline(always)]
        fn from(val: u8) -> OptsrCurBootSel {
            OptsrCurBootSel::from_bits(val)
        }
    }
    impl From<OptsrCurBootSel> for u8 {
        #[inline(always)]
        fn from(val: OptsrCurBootSel) -> u8 {
            OptsrCurBootSel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrCurDualBank {
        #[doc = "256Kbytes of user flash located in one bank."]
        B0x0 = 0x0,
        #[doc = "256Kbytes of user flash split with 128Kbytes in Bank 1 and 128Kbytes in Bank 2."]
        B0x1 = 0x01,
    }
    impl OptsrCurDualBank {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrCurDualBank {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrCurDualBank {
        #[inline(always)]
        fn from(val: u8) -> OptsrCurDualBank {
            OptsrCurDualBank::from_bits(val)
        }
    }
    impl From<OptsrCurDualBank> for u8 {
        #[inline(always)]
        fn from(val: OptsrCurDualBank) -> u8 {
            OptsrCurDualBank::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrCurEdataEn {
        #[doc = "No flash data area (EDATA pages are 128 bits writable)."]
        B0x0 = 0x0,
        #[doc = "Flash data area is enabled (EDATA pages are 16/32 bits writable)."]
        B0x1 = 0x01,
    }
    impl OptsrCurEdataEn {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrCurEdataEn {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrCurEdataEn {
        #[inline(always)]
        fn from(val: u8) -> OptsrCurEdataEn {
            OptsrCurEdataEn::from_bits(val)
        }
    }
    impl From<OptsrCurEdataEn> for u8 {
        #[inline(always)]
        fn from(val: OptsrCurEdataEn) -> u8 {
            OptsrCurEdataEn::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrCurIwdgStdby {
        #[doc = "Independent watchdog frozen in Standby mode."]
        B0x0 = 0x0,
        #[doc = "Independent watchdog keep running in Standby mode."]
        B0x1 = 0x01,
    }
    impl OptsrCurIwdgStdby {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrCurIwdgStdby {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrCurIwdgStdby {
        #[inline(always)]
        fn from(val: u8) -> OptsrCurIwdgStdby {
            OptsrCurIwdgStdby::from_bits(val)
        }
    }
    impl From<OptsrCurIwdgStdby> for u8 {
        #[inline(always)]
        fn from(val: OptsrCurIwdgStdby) -> u8 {
            OptsrCurIwdgStdby::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrCurIwdgStop {
        #[doc = "Independent watchdog frozen in system Stop mode."]
        B0x0 = 0x0,
        #[doc = "Independent watchdog keep running in system Stop mode."]
        B0x1 = 0x01,
    }
    impl OptsrCurIwdgStop {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrCurIwdgStop {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrCurIwdgStop {
        #[inline(always)]
        fn from(val: u8) -> OptsrCurIwdgStop {
            OptsrCurIwdgStop::from_bits(val)
        }
    }
    impl From<OptsrCurIwdgStop> for u8 {
        #[inline(always)]
        fn from(val: OptsrCurIwdgStop) -> u8 {
            OptsrCurIwdgStop::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrCurIwdgSw {
        #[doc = "IWDG watchdog is controlled by hardware."]
        B0x0 = 0x0,
        #[doc = "IWDG watchdog is controlled by software."]
        B0x1 = 0x01,
    }
    impl OptsrCurIwdgSw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrCurIwdgSw {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrCurIwdgSw {
        #[inline(always)]
        fn from(val: u8) -> OptsrCurIwdgSw {
            OptsrCurIwdgSw::from_bits(val)
        }
    }
    impl From<OptsrCurIwdgSw> for u8 {
        #[inline(always)]
        fn from(val: OptsrCurIwdgSw) -> u8 {
            OptsrCurIwdgSw::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrCurNrstStdby {
        #[doc = "a reset is generated when entering Standby mode on core domain."]
        B0x0 = 0x0,
        #[doc = "no reset generated when entering Standby mode on core domain."]
        B0x1 = 0x01,
    }
    impl OptsrCurNrstStdby {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrCurNrstStdby {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrCurNrstStdby {
        #[inline(always)]
        fn from(val: u8) -> OptsrCurNrstStdby {
            OptsrCurNrstStdby::from_bits(val)
        }
    }
    impl From<OptsrCurNrstStdby> for u8 {
        #[inline(always)]
        fn from(val: OptsrCurNrstStdby) -> u8 {
            OptsrCurNrstStdby::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrCurNrstStop {
        #[doc = "a reset is generated when entering Stop mode on core domain."]
        B0x0 = 0x0,
        #[doc = "no reset generated when entering Stop mode on core domain."]
        B0x1 = 0x01,
    }
    impl OptsrCurNrstStop {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrCurNrstStop {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrCurNrstStop {
        #[inline(always)]
        fn from(val: u8) -> OptsrCurNrstStop {
            OptsrCurNrstStop::from_bits(val)
        }
    }
    impl From<OptsrCurNrstStop> for u8 {
        #[inline(always)]
        fn from(val: OptsrCurNrstStop) -> u8 {
            OptsrCurNrstStop::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrCurSingleBank {
        #[doc = "128 Kbytes of user flash split with 64 Kbytes in Bank 1 and 64 Kbytes in Bank 2."]
        B0x0 = 0x0,
        #[doc = "128 Kbytes of user flash located in one bank."]
        B0x1 = 0x01,
    }
    impl OptsrCurSingleBank {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrCurSingleBank {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrCurSingleBank {
        #[inline(always)]
        fn from(val: u8) -> OptsrCurSingleBank {
            OptsrCurSingleBank::from_bits(val)
        }
    }
    impl From<OptsrCurSingleBank> for u8 {
        #[inline(always)]
        fn from(val: OptsrCurSingleBank) -> u8 {
            OptsrCurSingleBank::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrCurSwapBank {
        #[doc = "Bank1 and bank2 not swapped."]
        B0x0 = 0x0,
        #[doc = "Bank1 and bank2 swapped."]
        B0x1 = 0x01,
    }
    impl OptsrCurSwapBank {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrCurSwapBank {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrCurSwapBank {
        #[inline(always)]
        fn from(val: u8) -> OptsrCurSwapBank {
            OptsrCurSwapBank::from_bits(val)
        }
    }
    impl From<OptsrCurSwapBank> for u8 {
        #[inline(always)]
        fn from(val: OptsrCurSwapBank) -> u8 {
            OptsrCurSwapBank::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrCurWwdgSw {
        #[doc = "WWDG watchdog is controlled by hardware."]
        B0x0 = 0x0,
        #[doc = "WWDG watchdog is controlled by software."]
        B0x1 = 0x01,
    }
    impl OptsrCurWwdgSw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrCurWwdgSw {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrCurWwdgSw {
        #[inline(always)]
        fn from(val: u8) -> OptsrCurWwdgSw {
            OptsrCurWwdgSw::from_bits(val)
        }
    }
    impl From<OptsrCurWwdgSw> for u8 {
        #[inline(always)]
        fn from(val: OptsrCurWwdgSw) -> u8 {
            OptsrCurWwdgSw::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrPrgBoot0 {
        #[doc = "BOOT0 = 0."]
        B0x0 = 0x0,
        #[doc = "BOOT0 = 1."]
        B0x1 = 0x01,
    }
    impl OptsrPrgBoot0 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrPrgBoot0 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrPrgBoot0 {
        #[inline(always)]
        fn from(val: u8) -> OptsrPrgBoot0 {
            OptsrPrgBoot0::from_bits(val)
        }
    }
    impl From<OptsrPrgBoot0> for u8 {
        #[inline(always)]
        fn from(val: OptsrPrgBoot0) -> u8 {
            OptsrPrgBoot0::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrPrgBootSel {
        #[doc = "BOOT0 signal is defined by the BOOT0 option bit."]
        B0x0 = 0x0,
        #[doc = "BOOT0 signal is defined by BOOT0 pin value (legacy mode)."]
        B0x1 = 0x01,
    }
    impl OptsrPrgBootSel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrPrgBootSel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrPrgBootSel {
        #[inline(always)]
        fn from(val: u8) -> OptsrPrgBootSel {
            OptsrPrgBootSel::from_bits(val)
        }
    }
    impl From<OptsrPrgBootSel> for u8 {
        #[inline(always)]
        fn from(val: OptsrPrgBootSel) -> u8 {
            OptsrPrgBootSel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrPrgDualBank {
        #[doc = "256Kbytes of user flash located in one bank."]
        B0x0 = 0x0,
        #[doc = "256Kbytes of user flash split with 128Kbytes in Bank 1 and 128Kbytes in Bank 2."]
        B0x1 = 0x01,
    }
    impl OptsrPrgDualBank {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrPrgDualBank {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrPrgDualBank {
        #[inline(always)]
        fn from(val: u8) -> OptsrPrgDualBank {
            OptsrPrgDualBank::from_bits(val)
        }
    }
    impl From<OptsrPrgDualBank> for u8 {
        #[inline(always)]
        fn from(val: OptsrPrgDualBank) -> u8 {
            OptsrPrgDualBank::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrPrgEdataEn {
        #[doc = "No flash data area."]
        B0x0 = 0x0,
        #[doc = "Flash data area is enabled."]
        B0x1 = 0x01,
    }
    impl OptsrPrgEdataEn {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrPrgEdataEn {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrPrgEdataEn {
        #[inline(always)]
        fn from(val: u8) -> OptsrPrgEdataEn {
            OptsrPrgEdataEn::from_bits(val)
        }
    }
    impl From<OptsrPrgEdataEn> for u8 {
        #[inline(always)]
        fn from(val: OptsrPrgEdataEn) -> u8 {
            OptsrPrgEdataEn::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrPrgIwdgStdby {
        #[doc = "Independent watchdog frozen in Standby mode."]
        B0x0 = 0x0,
        #[doc = "Independent watchdog keep running in Standby mode."]
        B0x1 = 0x01,
    }
    impl OptsrPrgIwdgStdby {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrPrgIwdgStdby {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrPrgIwdgStdby {
        #[inline(always)]
        fn from(val: u8) -> OptsrPrgIwdgStdby {
            OptsrPrgIwdgStdby::from_bits(val)
        }
    }
    impl From<OptsrPrgIwdgStdby> for u8 {
        #[inline(always)]
        fn from(val: OptsrPrgIwdgStdby) -> u8 {
            OptsrPrgIwdgStdby::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrPrgIwdgStop {
        #[doc = "Independent watchdog frozen in system Stop mode."]
        B0x0 = 0x0,
        #[doc = "Independent watchdog keep running in system Stop mode."]
        B0x1 = 0x01,
    }
    impl OptsrPrgIwdgStop {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrPrgIwdgStop {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrPrgIwdgStop {
        #[inline(always)]
        fn from(val: u8) -> OptsrPrgIwdgStop {
            OptsrPrgIwdgStop::from_bits(val)
        }
    }
    impl From<OptsrPrgIwdgStop> for u8 {
        #[inline(always)]
        fn from(val: OptsrPrgIwdgStop) -> u8 {
            OptsrPrgIwdgStop::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrPrgIwdgSw {
        #[doc = "IWDG watchdog is controlled by hardware."]
        B0x0 = 0x0,
        #[doc = "IWDG watchdog is controlled by software."]
        B0x1 = 0x01,
    }
    impl OptsrPrgIwdgSw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrPrgIwdgSw {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrPrgIwdgSw {
        #[inline(always)]
        fn from(val: u8) -> OptsrPrgIwdgSw {
            OptsrPrgIwdgSw::from_bits(val)
        }
    }
    impl From<OptsrPrgIwdgSw> for u8 {
        #[inline(always)]
        fn from(val: OptsrPrgIwdgSw) -> u8 {
            OptsrPrgIwdgSw::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrPrgNrstStdby {
        #[doc = "a reset is generated when entering Standby mode on core domain."]
        B0x0 = 0x0,
        #[doc = "no reset generated when entering Standby mode on core domain."]
        B0x1 = 0x01,
    }
    impl OptsrPrgNrstStdby {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrPrgNrstStdby {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrPrgNrstStdby {
        #[inline(always)]
        fn from(val: u8) -> OptsrPrgNrstStdby {
            OptsrPrgNrstStdby::from_bits(val)
        }
    }
    impl From<OptsrPrgNrstStdby> for u8 {
        #[inline(always)]
        fn from(val: OptsrPrgNrstStdby) -> u8 {
            OptsrPrgNrstStdby::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrPrgNrstStop {
        #[doc = "a reset is generated when entering Stop mode on core domain."]
        B0x0 = 0x0,
        #[doc = "no reset generated when entering Stop mode on core domain."]
        B0x1 = 0x01,
    }
    impl OptsrPrgNrstStop {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrPrgNrstStop {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrPrgNrstStop {
        #[inline(always)]
        fn from(val: u8) -> OptsrPrgNrstStop {
            OptsrPrgNrstStop::from_bits(val)
        }
    }
    impl From<OptsrPrgNrstStop> for u8 {
        #[inline(always)]
        fn from(val: OptsrPrgNrstStop) -> u8 {
            OptsrPrgNrstStop::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrPrgSingleBank {
        #[doc = "128 Kbytes of user flash split with 64 Kbytes in Bank 1 and 64 Kbytes in Bank 2."]
        B0x0 = 0x0,
        #[doc = "128 Kbytes of user flash located in one bank."]
        B0x1 = 0x01,
    }
    impl OptsrPrgSingleBank {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrPrgSingleBank {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrPrgSingleBank {
        #[inline(always)]
        fn from(val: u8) -> OptsrPrgSingleBank {
            OptsrPrgSingleBank::from_bits(val)
        }
    }
    impl From<OptsrPrgSingleBank> for u8 {
        #[inline(always)]
        fn from(val: OptsrPrgSingleBank) -> u8 {
            OptsrPrgSingleBank::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrPrgSwapBank {
        #[doc = "Bank1 and bank2 not swapped."]
        B0x0 = 0x0,
        #[doc = "Bank1 and bank2 swapped."]
        B0x1 = 0x01,
    }
    impl OptsrPrgSwapBank {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrPrgSwapBank {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrPrgSwapBank {
        #[inline(always)]
        fn from(val: u8) -> OptsrPrgSwapBank {
            OptsrPrgSwapBank::from_bits(val)
        }
    }
    impl From<OptsrPrgSwapBank> for u8 {
        #[inline(always)]
        fn from(val: OptsrPrgSwapBank) -> u8 {
            OptsrPrgSwapBank::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrPrgWwdgSw {
        #[doc = "WWDG watchdog is controlled by hardware."]
        B0x0 = 0x0,
        #[doc = "WWDG watchdog is controlled by software."]
        B0x1 = 0x01,
    }
    impl OptsrPrgWwdgSw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrPrgWwdgSw {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrPrgWwdgSw {
        #[inline(always)]
        fn from(val: u8) -> OptsrPrgWwdgSw {
            OptsrPrgWwdgSw::from_bits(val)
        }
    }
    impl From<OptsrPrgWwdgSw> for u8 {
        #[inline(always)]
        fn from(val: OptsrPrgWwdgSw) -> u8 {
            OptsrPrgWwdgSw::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Per {
        #[doc = "page erase not requested."]
        B0x0 = 0x0,
        #[doc = "page erase requested."]
        B0x1 = 0x01,
    }
    impl Per {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Per {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Per {
        #[inline(always)]
        fn from(val: u8) -> Per {
            Per::from_bits(val)
        }
    }
    impl From<Per> for u8 {
        #[inline(always)]
        fn from(val: Per) -> u8 {
            Per::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pg {
        #[doc = "programming disabled."]
        B0x0 = 0x0,
        #[doc = "programming enabled."]
        B0x1 = 0x01,
    }
    impl Pg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pg {
        #[inline(always)]
        fn from(val: u8) -> Pg {
            Pg::from_bits(val)
        }
    }
    impl From<Pg> for u8 {
        #[inline(always)]
        fn from(val: Pg) -> u8 {
            Pg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pgserr {
        #[doc = "no sequence error occurred."]
        B0x0 = 0x0,
        #[doc = "a sequence error occurred."]
        B0x1 = 0x01,
    }
    impl Pgserr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pgserr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pgserr {
        #[inline(always)]
        fn from(val: u8) -> Pgserr {
            Pgserr::from_bits(val)
        }
    }
    impl From<Pgserr> for u8 {
        #[inline(always)]
        fn from(val: Pgserr) -> u8 {
            Pgserr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pgserrie {
        #[doc = "no interrupt generated when a sequence error occurs."]
        B0x0 = 0x0,
        #[doc = "interrupt generated when sequence error occurs."]
        B0x1 = 0x01,
    }
    impl Pgserrie {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pgserrie {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pgserrie {
        #[inline(always)]
        fn from(val: u8) -> Pgserrie {
            Pgserrie::from_bits(val)
        }
    }
    impl From<Pgserrie> for u8 {
        #[inline(always)]
        fn from(val: Pgserrie) -> u8 {
            Pgserrie::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pnb {
        #[doc = "Page 0 selected."]
        B0x00 = 0x0,
        #[doc = "Page 1 selected."]
        B0x01 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
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
        _RESERVED_f = 0x0f,
        _RESERVED_10 = 0x10,
        _RESERVED_11 = 0x11,
        _RESERVED_12 = 0x12,
        _RESERVED_13 = 0x13,
        _RESERVED_14 = 0x14,
        _RESERVED_15 = 0x15,
        _RESERVED_16 = 0x16,
        _RESERVED_17 = 0x17,
        _RESERVED_18 = 0x18,
        _RESERVED_19 = 0x19,
        _RESERVED_1a = 0x1a,
        _RESERVED_1b = 0x1b,
        _RESERVED_1c = 0x1c,
        _RESERVED_1d = 0x1d,
        _RESERVED_1e = 0x1e,
        _RESERVED_1f = 0x1f,
        _RESERVED_20 = 0x20,
        _RESERVED_21 = 0x21,
        _RESERVED_22 = 0x22,
        _RESERVED_23 = 0x23,
        _RESERVED_24 = 0x24,
        _RESERVED_25 = 0x25,
        _RESERVED_26 = 0x26,
        _RESERVED_27 = 0x27,
        _RESERVED_28 = 0x28,
        _RESERVED_29 = 0x29,
        _RESERVED_2a = 0x2a,
        _RESERVED_2b = 0x2b,
        _RESERVED_2c = 0x2c,
        _RESERVED_2d = 0x2d,
        _RESERVED_2e = 0x2e,
        _RESERVED_2f = 0x2f,
        _RESERVED_30 = 0x30,
        _RESERVED_31 = 0x31,
        _RESERVED_32 = 0x32,
        _RESERVED_33 = 0x33,
        _RESERVED_34 = 0x34,
        _RESERVED_35 = 0x35,
        _RESERVED_36 = 0x36,
        _RESERVED_37 = 0x37,
        _RESERVED_38 = 0x38,
        _RESERVED_39 = 0x39,
        _RESERVED_3a = 0x3a,
        _RESERVED_3b = 0x3b,
        _RESERVED_3c = 0x3c,
        _RESERVED_3d = 0x3d,
        _RESERVED_3e = 0x3e,
        _RESERVED_3f = 0x3f,
    }
    impl Pnb {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pnb {
            unsafe { core::mem::transmute(val & 0x3f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pnb {
        #[inline(always)]
        fn from(val: u8) -> Pnb {
            Pnb::from_bits(val)
        }
    }
    impl From<Pnb> for u8 {
        #[inline(always)]
        fn from(val: Pnb) -> u8 {
            Pnb::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Prften {
        #[doc = "prefetch disabled."]
        B0x0 = 0x0,
        #[doc = "prefetch enabled when latency is at least one wait-state."]
        B0x1 = 0x01,
    }
    impl Prften {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Prften {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Prften {
        #[inline(always)]
        fn from(val: u8) -> Prften {
            Prften::from_bits(val)
        }
    }
    impl From<Prften> for u8 {
        #[inline(always)]
        fn from(val: Prften) -> u8 {
            Prften::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Priv {
        #[doc = "Access to registers is always granted."]
        B0x0 = 0x0,
        #[doc = "Access to registers is denied in case of unprivileged access."]
        B0x1 = 0x01,
    }
    impl Priv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Priv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Priv {
        #[inline(always)]
        fn from(val: u8) -> Priv {
            Priv::from_bits(val)
        }
    }
    impl From<Priv> for u8 {
        #[inline(always)]
        fn from(val: Priv) -> u8 {
            Priv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ser {
        #[doc = "sector erase not requested."]
        B0x0 = 0x0,
        #[doc = "sector erase requested."]
        B0x1 = 0x01,
    }
    impl Ser {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ser {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ser {
        #[inline(always)]
        fn from(val: u8) -> Ser {
            Ser::from_bits(val)
        }
    }
    impl From<Ser> for u8 {
        #[inline(always)]
        fn from(val: Ser) -> u8 {
            Ser::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Snb {
        #[doc = "Sector 0 selected."]
        B0x00 = 0x0,
        #[doc = "Sector 1 selected."]
        B0x01 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
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
        _RESERVED_f = 0x0f,
        _RESERVED_10 = 0x10,
        _RESERVED_11 = 0x11,
        _RESERVED_12 = 0x12,
        _RESERVED_13 = 0x13,
        _RESERVED_14 = 0x14,
        _RESERVED_15 = 0x15,
        _RESERVED_16 = 0x16,
        _RESERVED_17 = 0x17,
        _RESERVED_18 = 0x18,
        _RESERVED_19 = 0x19,
        _RESERVED_1a = 0x1a,
        _RESERVED_1b = 0x1b,
        _RESERVED_1c = 0x1c,
        _RESERVED_1d = 0x1d,
        _RESERVED_1e = 0x1e,
        _RESERVED_1f = 0x1f,
        _RESERVED_20 = 0x20,
        _RESERVED_21 = 0x21,
        _RESERVED_22 = 0x22,
        _RESERVED_23 = 0x23,
        _RESERVED_24 = 0x24,
        _RESERVED_25 = 0x25,
        _RESERVED_26 = 0x26,
        _RESERVED_27 = 0x27,
        _RESERVED_28 = 0x28,
        _RESERVED_29 = 0x29,
        _RESERVED_2a = 0x2a,
        _RESERVED_2b = 0x2b,
        _RESERVED_2c = 0x2c,
        _RESERVED_2d = 0x2d,
        _RESERVED_2e = 0x2e,
        #[doc = "Sector 41 selected."]
        B0x2f = 0x2f,
        _RESERVED_30 = 0x30,
        _RESERVED_31 = 0x31,
        _RESERVED_32 = 0x32,
        _RESERVED_33 = 0x33,
        _RESERVED_34 = 0x34,
        _RESERVED_35 = 0x35,
        _RESERVED_36 = 0x36,
        _RESERVED_37 = 0x37,
        _RESERVED_38 = 0x38,
        _RESERVED_39 = 0x39,
        _RESERVED_3a = 0x3a,
        _RESERVED_3b = 0x3b,
        _RESERVED_3c = 0x3c,
        _RESERVED_3d = 0x3d,
        _RESERVED_3e = 0x3e,
        _RESERVED_3f = 0x3f,
    }
    impl Snb {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Snb {
            unsafe { core::mem::transmute(val & 0x3f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Snb {
        #[inline(always)]
        fn from(val: u8) -> Snb {
            Snb::from_bits(val)
        }
    }
    impl From<Snb> for u8 {
        #[inline(always)]
        fn from(val: Snb) -> u8 {
            Snb::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Strberr {
        #[doc = "no strobe error occurred."]
        B0x0 = 0x0,
        #[doc = "a strobe error occurred."]
        B0x1 = 0x01,
    }
    impl Strberr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Strberr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Strberr {
        #[inline(always)]
        fn from(val: u8) -> Strberr {
            Strberr::from_bits(val)
        }
    }
    impl From<Strberr> for u8 {
        #[inline(always)]
        fn from(val: Strberr) -> u8 {
            Strberr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Strberrie {
        #[doc = "no interrupt generated when a strobe error occurs."]
        B0x0 = 0x0,
        #[doc = "interrupt generated when strobe error occurs."]
        B0x1 = 0x01,
    }
    impl Strberrie {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Strberrie {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Strberrie {
        #[inline(always)]
        fn from(val: u8) -> Strberrie {
            Strberrie::from_bits(val)
        }
    }
    impl From<Strberrie> for u8 {
        #[inline(always)]
        fn from(val: Strberrie) -> u8 {
            Strberrie::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wbne {
        #[doc = "write buffer empty or full."]
        B0x0 = 0x0,
        #[doc = "write buffer waiting data to complete."]
        B0x1 = 0x01,
    }
    impl Wbne {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wbne {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wbne {
        #[inline(always)]
        fn from(val: u8) -> Wbne {
            Wbne::from_bits(val)
        }
    }
    impl From<Wbne> for u8 {
        #[inline(always)]
        fn from(val: Wbne) -> u8 {
            Wbne::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wrp1rCurWrpsg1(u32);
    impl Wrp1rCurWrpsg1 {
        #[doc = "write protected;."]
        pub const B0x0: Self = Self(0x0);
        #[doc = "not write protected."]
        pub const B0x1: Self = Self(0x01);
    }
    impl Wrp1rCurWrpsg1 {
        pub const fn from_bits(val: u32) -> Wrp1rCurWrpsg1 {
            Self(val & 0xffff_ffff)
        }
        pub const fn to_bits(self) -> u32 {
            self.0
        }
    }
    impl core::fmt::Debug for Wrp1rCurWrpsg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("B0x0"),
                0x01 => f.write_str("B0x1"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrp1rCurWrpsg1 {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "B0x0"),
                0x01 => defmt::write!(f, "B0x1"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u32> for Wrp1rCurWrpsg1 {
        #[inline(always)]
        fn from(val: u32) -> Wrp1rCurWrpsg1 {
            Wrp1rCurWrpsg1::from_bits(val)
        }
    }
    impl From<Wrp1rCurWrpsg1> for u32 {
        #[inline(always)]
        fn from(val: Wrp1rCurWrpsg1) -> u32 {
            Wrp1rCurWrpsg1::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wrp1rPrgWrpsg1(u32);
    impl Wrp1rPrgWrpsg1 {
        #[doc = "write protected;."]
        pub const B0x0: Self = Self(0x0);
        #[doc = "not write protected."]
        pub const B0x1: Self = Self(0x01);
    }
    impl Wrp1rPrgWrpsg1 {
        pub const fn from_bits(val: u32) -> Wrp1rPrgWrpsg1 {
            Self(val & 0xffff_ffff)
        }
        pub const fn to_bits(self) -> u32 {
            self.0
        }
    }
    impl core::fmt::Debug for Wrp1rPrgWrpsg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("B0x0"),
                0x01 => f.write_str("B0x1"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrp1rPrgWrpsg1 {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "B0x0"),
                0x01 => defmt::write!(f, "B0x1"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u32> for Wrp1rPrgWrpsg1 {
        #[inline(always)]
        fn from(val: u32) -> Wrp1rPrgWrpsg1 {
            Wrp1rPrgWrpsg1::from_bits(val)
        }
    }
    impl From<Wrp1rPrgWrpsg1> for u32 {
        #[inline(always)]
        fn from(val: Wrp1rPrgWrpsg1) -> u32 {
            Wrp1rPrgWrpsg1::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wrp2rCurWrpsg2(u32);
    impl Wrp2rCurWrpsg2 {
        #[doc = "write protected."]
        pub const B0x0: Self = Self(0x0);
        #[doc = "not write protected."]
        pub const B0x1: Self = Self(0x01);
    }
    impl Wrp2rCurWrpsg2 {
        pub const fn from_bits(val: u32) -> Wrp2rCurWrpsg2 {
            Self(val & 0xffff_ffff)
        }
        pub const fn to_bits(self) -> u32 {
            self.0
        }
    }
    impl core::fmt::Debug for Wrp2rCurWrpsg2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("B0x0"),
                0x01 => f.write_str("B0x1"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrp2rCurWrpsg2 {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "B0x0"),
                0x01 => defmt::write!(f, "B0x1"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u32> for Wrp2rCurWrpsg2 {
        #[inline(always)]
        fn from(val: u32) -> Wrp2rCurWrpsg2 {
            Wrp2rCurWrpsg2::from_bits(val)
        }
    }
    impl From<Wrp2rCurWrpsg2> for u32 {
        #[inline(always)]
        fn from(val: Wrp2rCurWrpsg2) -> u32 {
            Wrp2rCurWrpsg2::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wrp2rPrgWrpsg2(u32);
    impl Wrp2rPrgWrpsg2 {
        #[doc = "write protected;."]
        pub const B0x0: Self = Self(0x0);
        #[doc = "not write protected."]
        pub const B0x1: Self = Self(0x01);
    }
    impl Wrp2rPrgWrpsg2 {
        pub const fn from_bits(val: u32) -> Wrp2rPrgWrpsg2 {
            Self(val & 0xffff_ffff)
        }
        pub const fn to_bits(self) -> u32 {
            self.0
        }
    }
    impl core::fmt::Debug for Wrp2rPrgWrpsg2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("B0x0"),
                0x01 => f.write_str("B0x1"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrp2rPrgWrpsg2 {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "B0x0"),
                0x01 => defmt::write!(f, "B0x1"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u32> for Wrp2rPrgWrpsg2 {
        #[inline(always)]
        fn from(val: u32) -> Wrp2rPrgWrpsg2 {
            Wrp2rPrgWrpsg2::from_bits(val)
        }
    }
    impl From<Wrp2rPrgWrpsg2> for u32 {
        #[inline(always)]
        fn from(val: Wrp2rPrgWrpsg2) -> u32 {
            Wrp2rPrgWrpsg2::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wrperr {
        #[doc = "no write protection error occurred."]
        B0x0 = 0x0,
        #[doc = "a write protection error occurred."]
        B0x1 = 0x01,
    }
    impl Wrperr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wrperr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wrperr {
        #[inline(always)]
        fn from(val: u8) -> Wrperr {
            Wrperr::from_bits(val)
        }
    }
    impl From<Wrperr> for u8 {
        #[inline(always)]
        fn from(val: Wrperr) -> u8 {
            Wrperr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wrperrie {
        #[doc = "no interrupt generated when a protection error occurs."]
        B0x0 = 0x0,
        #[doc = "interrupt generated when a protection error occurs."]
        B0x1 = 0x01,
    }
    impl Wrperrie {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wrperrie {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wrperrie {
        #[inline(always)]
        fn from(val: u8) -> Wrperrie {
            Wrperrie::from_bits(val)
        }
    }
    impl From<Wrperrie> for u8 {
        #[inline(always)]
        fn from(val: Wrperrie) -> u8 {
            Wrperrie::to_bits(val)
        }
    }
}
