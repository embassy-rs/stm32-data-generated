#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "FLASH address block description."]
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
    #[doc = "FLASH access control register."]
    #[inline(always)]
    pub const fn acr(self) -> crate::common::Reg<regs::Acr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "FLASH non-secure key register."]
    #[inline(always)]
    pub const fn keyr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "FLASH secure key register."]
    #[inline(always)]
    pub const fn skeyr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "FLASH option key register."]
    #[inline(always)]
    pub const fn optkeyr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "FLASH non-secure OBK key register."]
    #[inline(always)]
    pub const fn obkkeyr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "FLASH secure OBK key register."]
    #[inline(always)]
    pub const fn sobkkeyr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
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
    #[doc = "FLASH non-secure status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "FLASH secure status register."]
    #[inline(always)]
    pub const fn ssr(self) -> crate::common::Reg<regs::Ssr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "FLASH non-secure control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "FLASH secure control register."]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::Scr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "FLASH non-secure clear control register."]
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::Ccr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "FLASH secure clear control register."]
    #[inline(always)]
    pub const fn sccr(self) -> crate::common::Reg<regs::Sccr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "FLASH privilege configuration register."]
    #[inline(always)]
    pub const fn privcfgr(self) -> crate::common::Reg<regs::Privcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "FLASH non-secure OBK configuration register."]
    #[inline(always)]
    pub const fn obkcfgr(self) -> crate::common::Reg<regs::Obkcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "FLASH secure OBK configuration register."]
    #[inline(always)]
    pub const fn sobkcfgr(self) -> crate::common::Reg<regs::Obkcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "FLASH HDP extension register."]
    #[inline(always)]
    pub const fn hdpextr(self) -> crate::common::Reg<regs::Hdpextr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "FLASH option status register."]
    #[inline(always)]
    pub const fn optsr_cur(self) -> crate::common::Reg<regs::Optsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "FLASH option status register."]
    #[inline(always)]
    pub const fn optsr_prg(self) -> crate::common::Reg<regs::Optsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "FLASH non-secure EPOCH register."]
    #[inline(always)]
    pub const fn epochr_cur(self) -> crate::common::Reg<regs::Epochr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "FLASH non-secure EPOCH register."]
    #[inline(always)]
    pub const fn epochr_prg(self) -> crate::common::Reg<regs::Epochr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "FLASH secure EPOCH register."]
    #[inline(always)]
    pub const fn sepochr_cur(self) -> crate::common::Reg<regs::Epochr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "FLASH secure EPOCH register."]
    #[inline(always)]
    pub const fn sepochr_prg(self) -> crate::common::Reg<regs::Epochr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "FLASH option status register 2."]
    #[inline(always)]
    pub const fn optsr2_cur(self) -> crate::common::Reg<regs::Optsr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "FLASH option status register 2."]
    #[inline(always)]
    pub const fn optsr2_prg(self) -> crate::common::Reg<regs::Optsr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "FLASH option status register 3."]
    #[inline(always)]
    pub const fn optsr3_cur(self) -> crate::common::Reg<regs::Optsr3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "FLASH option status register 3."]
    #[inline(always)]
    pub const fn optsr3_prg(self) -> crate::common::Reg<regs::Optsr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "FLASH non-secure boot register."]
    #[inline(always)]
    pub const fn bootr_cur(self) -> crate::common::Reg<regs::Bootr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "FLASH non-secure boot register."]
    #[inline(always)]
    pub const fn bootr_prg(self) -> crate::common::Reg<regs::Bootr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "FLASH secure boot register."]
    #[inline(always)]
    pub const fn sbootr_cur(self) -> crate::common::Reg<regs::Sbootr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "FLASH secure boot register."]
    #[inline(always)]
    pub const fn sbootr_prg(self) -> crate::common::Reg<regs::Sbootr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "FLASH non-secure OTP block lock."]
    #[inline(always)]
    pub const fn otpblr_cur(self) -> crate::common::Reg<regs::Otpblr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "FLASH non-secure OTP block lock."]
    #[inline(always)]
    pub const fn otpblr_prg(self) -> crate::common::Reg<regs::Otpblr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "FLASH bootloader interface selection."]
    #[inline(always)]
    pub const fn bl_com_cfg_cur(self) -> crate::common::Reg<regs::BlComCfg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "FLASH bootloader interface selection."]
    #[inline(always)]
    pub const fn bl_com_cfg_prg(self) -> crate::common::Reg<regs::BlComCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "FLASH secure block-based register for Bank 1."]
    #[inline(always)]
    pub const fn secbb1r(self) -> crate::common::Reg<regs::Secbb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "FLASH privilege block-based register for Bank 1."]
    #[inline(always)]
    pub const fn privbb1r(self) -> crate::common::Reg<regs::Privbb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "FLASH security watermark for Bank 1."]
    #[inline(always)]
    pub const fn secwm1r_cur(self) -> crate::common::Reg<regs::Secwm, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "FLASH security watermark for Bank 1."]
    #[inline(always)]
    pub const fn secwm1r_prg(self) -> crate::common::Reg<regs::Secwm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "FLASH write page group protection for Bank 1."]
    #[inline(always)]
    pub const fn wrp1r_cur(self) -> crate::common::Reg<regs::Wrp, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "FLASH write page group protection for Bank 1."]
    #[inline(always)]
    pub const fn wrp1r_prg(self) -> crate::common::Reg<regs::Wrp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "FLASH data page configuration Bank 1."]
    #[inline(always)]
    pub const fn edata1r_cur(self) -> crate::common::Reg<regs::Edata, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "FLASH data page configuration Bank 1."]
    #[inline(always)]
    pub const fn edata1r_prg(self) -> crate::common::Reg<regs::Edata, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "FLASH HDP Bank 1 configuration."]
    #[inline(always)]
    pub const fn hdp1r_cur(self) -> crate::common::Reg<regs::Hdp, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "FLASH HDP Bank 1 configuration."]
    #[inline(always)]
    pub const fn hdp1r_prg(self) -> crate::common::Reg<regs::Hdp, crate::common::RW> {
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
    #[doc = "FLASH secure block-based register for Bank 2."]
    #[inline(always)]
    pub const fn secbb2r(self) -> crate::common::Reg<regs::Secbb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "FLASH privilege block-based register for Bank 2."]
    #[inline(always)]
    pub const fn privbb2r(self) -> crate::common::Reg<regs::Privbb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "FLASH security watermark for Bank 2."]
    #[inline(always)]
    pub const fn secwm2r_cur(self) -> crate::common::Reg<regs::Secwm, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "FLASH security watermark for Bank 2."]
    #[inline(always)]
    pub const fn secwm2r_prg(self) -> crate::common::Reg<regs::Secwm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[doc = "FLASH write page group protection for Bank 2."]
    #[inline(always)]
    pub const fn wrp2r_cur(self) -> crate::common::Reg<regs::Wrp, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[doc = "FLASH write page group protection for Bank 2."]
    #[inline(always)]
    pub const fn wrp2r_prg(self) -> crate::common::Reg<regs::Wrp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ecusize) as _) }
    }
    #[doc = "FLASH data page configuration Bank 2."]
    #[inline(always)]
    pub const fn edata2r_cur(self) -> crate::common::Reg<regs::Edata, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize) as _) }
    }
    #[doc = "FLASH data page configuration Bank 2."]
    #[inline(always)]
    pub const fn edata2r_prg(self) -> crate::common::Reg<regs::Edata, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f4usize) as _) }
    }
    #[doc = "FLASH HDP Bank 2 configuration."]
    #[inline(always)]
    pub const fn hdp2r_cur(self) -> crate::common::Reg<regs::Hdp, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "FLASH HDP Bank 2 configuration."]
    #[inline(always)]
    pub const fn hdp2r_prg(self) -> crate::common::Reg<regs::Hdp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01fcusize) as _) }
    }
}
pub mod regs {
    #[doc = "FLASH access control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Acr(pub u32);
    impl Acr {
        #[doc = "Read latency."]
        #[must_use]
        #[inline(always)]
        pub const fn latency(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Read latency."]
        #[inline(always)]
        pub const fn set_latency(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Flash signal delay."]
        #[must_use]
        #[inline(always)]
        pub const fn wrhighfreq(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Flash signal delay."]
        #[inline(always)]
        pub const fn set_wrhighfreq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Prefetch enable."]
        #[must_use]
        #[inline(always)]
        pub const fn prften(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Prefetch enable."]
        #[inline(always)]
        pub const fn set_prften(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Restrict FLASH_ACR from non-secure write access."]
        #[must_use]
        #[inline(always)]
        pub const fn acr_sec_en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Restrict FLASH_ACR from non-secure write access."]
        #[inline(always)]
        pub const fn set_acr_sec_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Acr {
        #[inline(always)]
        fn default() -> Acr {
            Acr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Acr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Acr")
                .field("latency", &self.latency())
                .field("wrhighfreq", &self.wrhighfreq())
                .field("prften", &self.prften())
                .field("acr_sec_en", &self.acr_sec_en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Acr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Acr {{ latency: {=u8:?}, wrhighfreq: {=u8:?}, prften: {=bool:?}, acr_sec_en: {=bool:?} }}",
                self.latency(),
                self.wrhighfreq(),
                self.prften(),
                self.acr_sec_en()
            )
        }
    }
    #[doc = "FLASH bootloader interface selection."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BlComCfg(pub u32);
    impl BlComCfg {
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
    impl Default for BlComCfg {
        #[inline(always)]
        fn default() -> BlComCfg {
            BlComCfg(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for BlComCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BlComCfg")
                .field("bl_com_cfg", &self.bl_com_cfg())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BlComCfg {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BlComCfg {{ bl_com_cfg: {=u32:?} }}", self.bl_com_cfg())
        }
    }
    #[doc = "FLASH non-secure boot register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bootr(pub u32);
    impl Bootr {
        #[doc = "Field locking the value of BOOTADD."]
        #[must_use]
        #[inline(always)]
        pub const fn boot_lock(&self) -> super::vals::BootLock {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::BootLock::from_bits(val as u8)
        }
        #[doc = "Field locking the value of BOOTADD."]
        #[inline(always)]
        pub const fn set_boot_lock(&mut self, val: super::vals::BootLock) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
        #[doc = "Non-secure unique boot entry address."]
        #[must_use]
        #[inline(always)]
        pub const fn bootadd(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Non-secure unique boot entry address."]
        #[inline(always)]
        pub const fn set_bootadd(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for Bootr {
        #[inline(always)]
        fn default() -> Bootr {
            Bootr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Bootr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bootr")
                .field("boot_lock", &self.boot_lock())
                .field("bootadd", &self.bootadd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bootr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bootr {{ boot_lock: {:?}, bootadd: {=u32:?} }}",
                self.boot_lock(),
                self.bootadd()
            )
        }
    }
    #[doc = "FLASH non-secure clear control register."]
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
        #[doc = "OBKERR flag clear bit."]
        #[must_use]
        #[inline(always)]
        pub const fn clr_obkerr(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "OBKERR flag clear bit."]
        #[inline(always)]
        pub const fn set_clr_obkerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "OBKWERR flag clear bit."]
        #[must_use]
        #[inline(always)]
        pub const fn clr_obkwerr(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "OBKWERR flag clear bit."]
        #[inline(always)]
        pub const fn set_clr_obkwerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "OPTCHANGEERR flag clear bit."]
        #[must_use]
        #[inline(always)]
        pub const fn clr_optchangeerr(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "OPTCHANGEERR flag clear bit."]
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
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Ccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ccr")
                .field("clr_eop", &self.clr_eop())
                .field("clr_wrperr", &self.clr_wrperr())
                .field("clr_pgserr", &self.clr_pgserr())
                .field("clr_strberr", &self.clr_strberr())
                .field("clr_incerr", &self.clr_incerr())
                .field("clr_obkerr", &self.clr_obkerr())
                .field("clr_obkwerr", &self.clr_obkwerr())
                .field("clr_optchangeerr", &self.clr_optchangeerr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ccr {{ clr_eop: {=bool:?}, clr_wrperr: {=bool:?}, clr_pgserr: {=bool:?}, clr_strberr: {=bool:?}, clr_incerr: {=bool:?}, clr_obkerr: {=bool:?}, clr_obkwerr: {=bool:?}, clr_optchangeerr: {=bool:?} }}",
                self.clr_eop(),
                self.clr_wrperr(),
                self.clr_pgserr(),
                self.clr_strberr(),
                self.clr_incerr(),
                self.clr_obkerr(),
                self.clr_obkwerr(),
                self.clr_optchangeerr()
            )
        }
    }
    #[doc = "FLASH non-secure control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Configuration lock bit."]
        #[must_use]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Configuration lock bit."]
        #[inline(always)]
        pub const fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Programming control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn pg(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Programming control bit."]
        #[inline(always)]
        pub const fn set_pg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Page erase request."]
        #[must_use]
        #[inline(always)]
        pub const fn per(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Page erase request."]
        #[inline(always)]
        pub const fn set_per(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Bank erase request."]
        #[must_use]
        #[inline(always)]
        pub const fn ber(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Bank erase request."]
        #[inline(always)]
        pub const fn set_ber(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Write forcing control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn fw(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Write forcing control bit."]
        #[inline(always)]
        pub const fn set_fw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Erase start control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn strt(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Erase start control bit."]
        #[inline(always)]
        pub const fn set_strt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Page erase selection number."]
        #[must_use]
        #[inline(always)]
        pub const fn pnb(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x1f;
            val as u8
        }
        #[doc = "Page erase selection number."]
        #[inline(always)]
        pub const fn set_pnb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
        }
        #[doc = "PUF preparation start control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn puf_launch(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "PUF preparation start control bit."]
        #[inline(always)]
        pub const fn set_puf_launch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Mass erase request."]
        #[must_use]
        #[inline(always)]
        pub const fn mer(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Mass erase request."]
        #[inline(always)]
        pub const fn set_mer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "End of operation interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn eopie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "End of operation interrupt enable."]
        #[inline(always)]
        pub const fn set_eopie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Write protection error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn wrperrie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Write protection error interrupt enable."]
        #[inline(always)]
        pub const fn set_wrperrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Programming sequence error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pgserrie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Programming sequence error interrupt enable."]
        #[inline(always)]
        pub const fn set_pgserrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Strobe error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn strberrie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Strobe error interrupt enable."]
        #[inline(always)]
        pub const fn set_strberrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Inconsistency error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn incerrie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Inconsistency error interrupt enable."]
        #[inline(always)]
        pub const fn set_incerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "OBK general error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn obkerrie(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "OBK general error interrupt enable."]
        #[inline(always)]
        pub const fn set_obkerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "OBK write error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn obkwerrie(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "OBK write error interrupt enable."]
        #[inline(always)]
        pub const fn set_obkwerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Option byte change error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn optchangeerrie(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte change error interrupt enable."]
        #[inline(always)]
        pub const fn set_optchangeerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
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
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr")
                .field("lock", &self.lock())
                .field("pg", &self.pg())
                .field("per", &self.per())
                .field("ber", &self.ber())
                .field("fw", &self.fw())
                .field("strt", &self.strt())
                .field("pnb", &self.pnb())
                .field("puf_launch", &self.puf_launch())
                .field("mer", &self.mer())
                .field("eopie", &self.eopie())
                .field("wrperrie", &self.wrperrie())
                .field("pgserrie", &self.pgserrie())
                .field("strberrie", &self.strberrie())
                .field("incerrie", &self.incerrie())
                .field("obkerrie", &self.obkerrie())
                .field("obkwerrie", &self.obkwerrie())
                .field("optchangeerrie", &self.optchangeerrie())
                .field("bksel", &self.bksel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ lock: {=bool:?}, pg: {=bool:?}, per: {=bool:?}, ber: {=bool:?}, fw: {=bool:?}, strt: {=bool:?}, pnb: {=u8:?}, puf_launch: {=bool:?}, mer: {=bool:?}, eopie: {=bool:?}, wrperrie: {=bool:?}, pgserrie: {=bool:?}, strberrie: {=bool:?}, incerrie: {=bool:?}, obkerrie: {=bool:?}, obkwerrie: {=bool:?}, optchangeerrie: {=bool:?}, bksel: {:?} }}",
                self.lock(),
                self.pg(),
                self.per(),
                self.ber(),
                self.fw(),
                self.strt(),
                self.pnb(),
                self.puf_launch(),
                self.mer(),
                self.eopie(),
                self.wrperrie(),
                self.pgserrie(),
                self.strberrie(),
                self.incerrie(),
                self.obkerrie(),
                self.obkwerrie(),
                self.optchangeerrie(),
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
        #[doc = "Single ECC error corrected in flash OB Keys storage area."]
        #[must_use]
        #[inline(always)]
        pub const fn obk_ecc(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Single ECC error corrected in flash OB Keys storage area."]
        #[inline(always)]
        pub const fn set_obk_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Single ECC error corrected in flash high-cycle data area."]
        #[must_use]
        #[inline(always)]
        pub const fn edata_ecc(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Single ECC error corrected in flash high-cycle data area."]
        #[inline(always)]
        pub const fn set_edata_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ECC fail bank for corrected ECC error."]
        #[must_use]
        #[inline(always)]
        pub const fn bk_ecc(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "ECC fail bank for corrected ECC error."]
        #[inline(always)]
        pub const fn set_bk_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Single ECC error corrected in system flash memory."]
        #[must_use]
        #[inline(always)]
        pub const fn sysf_ecc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Single ECC error corrected in system flash memory."]
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
        #[doc = "ECC single correction error interrupt enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ecccie(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "ECC single correction error interrupt enable bit."]
        #[inline(always)]
        pub const fn set_ecccie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "ECC correction flag (cleared by writing 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn eccc(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "ECC correction flag (cleared by writing 1)."]
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
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Ecccorr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ecccorr")
                .field("addr_ecc", &self.addr_ecc())
                .field("obk_ecc", &self.obk_ecc())
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
                "Ecccorr {{ addr_ecc: {=u16:?}, obk_ecc: {=bool:?}, edata_ecc: {=bool:?}, bk_ecc: {=bool:?}, sysf_ecc: {=bool:?}, otp_ecc: {=bool:?}, ecccie: {=bool:?}, eccc: {=bool:?} }}",
                self.addr_ecc(),
                self.obk_ecc(),
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
        #[doc = "Double ECC error detected in flash OB Keys storage area."]
        #[must_use]
        #[inline(always)]
        pub const fn obk_ecc(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Double ECC error detected in flash OB Keys storage area."]
        #[inline(always)]
        pub const fn set_obk_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Double ECC error detected in flash high-cycle data area."]
        #[must_use]
        #[inline(always)]
        pub const fn edata_ecc(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Double ECC error detected in flash high-cycle data area."]
        #[inline(always)]
        pub const fn set_edata_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "ECC fail bank for double ECC error."]
        #[must_use]
        #[inline(always)]
        pub const fn bk_ecc(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "ECC fail bank for double ECC error."]
        #[inline(always)]
        pub const fn set_bk_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Double ECC error detected in system flash memory."]
        #[must_use]
        #[inline(always)]
        pub const fn sysf_ecc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Double ECC error detected in system flash memory."]
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
        #[doc = "ECC double detection flag (cleared by writing 1)."]
        #[must_use]
        #[inline(always)]
        pub const fn eccd(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "ECC double detection flag (cleared by writing 1)."]
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
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Eccdetr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccdetr")
                .field("addr_ecc", &self.addr_ecc())
                .field("obk_ecc", &self.obk_ecc())
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
                "Eccdetr {{ addr_ecc: {=u16:?}, obk_ecc: {=bool:?}, edata_ecc: {=bool:?}, bk_ecc: {=bool:?}, sysf_ecc: {=bool:?}, otp_ecc: {=bool:?}, eccd: {=bool:?} }}",
                self.addr_ecc(),
                self.obk_ecc(),
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
    }
    impl Default for Eccdr {
        #[inline(always)]
        fn default() -> Eccdr {
            Eccdr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Eccdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eccdr").field("data_ecc", &self.data_ecc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eccdr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Eccdr {{ data_ecc: {=u16:?} }}", self.data_ecc())
        }
    }
    #[doc = "FLASH data page configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Edata(pub u32);
    impl Edata {
        #[doc = "Start page count minus one of flash high-cycle data area."]
        #[must_use]
        #[inline(always)]
        pub const fn strt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Start page count minus one of flash high-cycle data area."]
        #[inline(always)]
        pub const fn set_strt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Flash high-cycle data enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Flash high-cycle data enable."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Edata {
        #[inline(always)]
        fn default() -> Edata {
            Edata(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Edata {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Edata")
                .field("strt", &self.strt())
                .field("en", &self.en())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Edata {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Edata {{ strt: {=u8:?}, en: {=bool:?} }}", self.strt(), self.en())
        }
    }
    #[doc = "FLASH EPOCH register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Epochr(pub u32);
    impl Epochr {
        #[doc = "Non-volatile EPOCH counter."]
        #[must_use]
        #[inline(always)]
        pub const fn epoch(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Non-volatile EPOCH counter."]
        #[inline(always)]
        pub const fn set_epoch(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
    }
    impl Default for Epochr {
        #[inline(always)]
        fn default() -> Epochr {
            Epochr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Epochr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Epochr").field("epoch", &self.epoch()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Epochr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Epochr {{ epoch: {=u32:?} }}", self.epoch())
        }
    }
    #[doc = "FLASH HDP Bank configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hdp(pub u32);
    impl Hdp {
        #[doc = "HDPL barrier start set in number of 8-Kbyte pages."]
        #[must_use]
        #[inline(always)]
        pub const fn strt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "HDPL barrier start set in number of 8-Kbyte pages."]
        #[inline(always)]
        pub const fn set_strt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "HDPL barrier end set in number of 8-Kbyte pages."]
        #[must_use]
        #[inline(always)]
        pub const fn end(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "HDPL barrier end set in number of 8-Kbyte pages."]
        #[inline(always)]
        pub const fn set_end(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
    }
    impl Default for Hdp {
        #[inline(always)]
        fn default() -> Hdp {
            Hdp(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Hdp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hdp")
                .field("strt", &self.strt())
                .field("end", &self.end())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hdp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Hdp {{ strt: {=u8:?}, end: {=u8:?} }}", self.strt(), self.end())
        }
    }
    #[doc = "FLASH HDP extension register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hdpextr(pub u32);
    impl Hdpextr {
        #[doc = "HDP area extension in 8-Kbyte pages in Bank 1."]
        #[must_use]
        #[inline(always)]
        pub const fn hdp1_ext(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "HDP area extension in 8-Kbyte pages in Bank 1."]
        #[inline(always)]
        pub const fn set_hdp1_ext(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "HDP area extension in 8-Kbyte pages in Bank 2."]
        #[must_use]
        #[inline(always)]
        pub const fn hdp2_ext(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "HDP area extension in 8-Kbyte pages in Bank 2."]
        #[inline(always)]
        pub const fn set_hdp2_ext(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
    }
    impl Default for Hdpextr {
        #[inline(always)]
        fn default() -> Hdpextr {
            Hdpextr(0)
        }
    }
    #[cfg(feature = "debug")]
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
    #[doc = "FLASH OBK configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Obkcfgr(pub u32);
    impl Obkcfgr {
        #[doc = "OBKCFGR lock option configuration bit."]
        #[must_use]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "OBKCFGR lock option configuration bit."]
        #[inline(always)]
        pub const fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "OBK swap page request bit."]
        #[must_use]
        #[inline(always)]
        pub const fn swap_sect_req(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "OBK swap page request bit."]
        #[inline(always)]
        pub const fn set_swap_sect_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Alternate page bit."]
        #[must_use]
        #[inline(always)]
        pub const fn alt_sect(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Alternate page bit."]
        #[inline(always)]
        pub const fn set_alt_sect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Alternate page erase bit."]
        #[must_use]
        #[inline(always)]
        pub const fn alt_sect_erase(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Alternate page erase bit."]
        #[inline(always)]
        pub const fn set_alt_sect_erase(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Key index (offset / 16 bits) pointing for next swap."]
        #[must_use]
        #[inline(always)]
        pub const fn swap_offset(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "Key index (offset / 16 bits) pointing for next swap."]
        #[inline(always)]
        pub const fn set_swap_offset(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Obkcfgr {
        #[inline(always)]
        fn default() -> Obkcfgr {
            Obkcfgr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Obkcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Obkcfgr")
                .field("lock", &self.lock())
                .field("swap_sect_req", &self.swap_sect_req())
                .field("alt_sect", &self.alt_sect())
                .field("alt_sect_erase", &self.alt_sect_erase())
                .field("swap_offset", &self.swap_offset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Obkcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Obkcfgr {{ lock: {=bool:?}, swap_sect_req: {=bool:?}, alt_sect: {=bool:?}, alt_sect_erase: {=bool:?}, swap_offset: {=u16:?} }}",
                self.lock(),
                self.swap_sect_req(),
                self.alt_sect(),
                self.alt_sect_erase(),
                self.swap_offset()
            )
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
        pub const fn addr_op(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Interrupted operation address."]
        #[inline(always)]
        pub const fn set_addr_op(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "Flash high-cycle data area operation interrupted."]
        #[must_use]
        #[inline(always)]
        pub const fn data_op(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Flash high-cycle data area operation interrupted."]
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
    #[cfg(feature = "debug")]
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
                "Opsr {{ addr_op: {=u32:?}, data_op: {=bool:?}, bk_op: {=bool:?}, sysf_op: {=bool:?}, otp_op: {=bool:?}, code_op: {:?} }}",
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
        #[doc = "FLASH_OPTCR lock option configuration bit."]
        #[must_use]
        #[inline(always)]
        pub const fn optlock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "FLASH_OPTCR lock option configuration bit."]
        #[inline(always)]
        pub const fn set_optlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Option byte start change option configuration bit."]
        #[must_use]
        #[inline(always)]
        pub const fn optstrt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte start change option configuration bit."]
        #[inline(always)]
        pub const fn set_optstrt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Optcr {
        #[inline(always)]
        fn default() -> Optcr {
            Optcr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Optcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Optcr")
                .field("optlock", &self.optlock())
                .field("optstrt", &self.optstrt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Optcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Optcr {{ optlock: {=bool:?}, optstrt: {=bool:?} }}",
                self.optlock(),
                self.optstrt()
            )
        }
    }
    #[doc = "FLASH option status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optsr(pub u32);
    impl Optsr {
        #[doc = "IWDG control mode option status bit."]
        #[must_use]
        #[inline(always)]
        pub const fn iwdg_sw(&self) -> super::vals::OptsrIwdgSw {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::OptsrIwdgSw::from_bits(val as u8)
        }
        #[doc = "IWDG control mode option status bit."]
        #[inline(always)]
        pub const fn set_iwdg_sw(&mut self, val: super::vals::OptsrIwdgSw) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "WWDG control mode option status bit."]
        #[must_use]
        #[inline(always)]
        pub const fn wwdg_sw(&self) -> super::vals::OptsrWwdgSw {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::OptsrWwdgSw::from_bits(val as u8)
        }
        #[doc = "WWDG control mode option status bit."]
        #[inline(always)]
        pub const fn set_wwdg_sw(&mut self, val: super::vals::OptsrWwdgSw) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Core domain Stop entry reset option status bit."]
        #[must_use]
        #[inline(always)]
        pub const fn nrst_stop(&self) -> super::vals::OptsrNrstStop {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::OptsrNrstStop::from_bits(val as u8)
        }
        #[doc = "Core domain Stop entry reset option status bit."]
        #[inline(always)]
        pub const fn set_nrst_stop(&mut self, val: super::vals::OptsrNrstStop) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "Core domain Standby entry reset option status bit."]
        #[must_use]
        #[inline(always)]
        pub const fn nrst_stdby(&self) -> super::vals::OptsrNrstStdby {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::OptsrNrstStdby::from_bits(val as u8)
        }
        #[doc = "Core domain Standby entry reset option status bit."]
        #[inline(always)]
        pub const fn set_nrst_stdby(&mut self, val: super::vals::OptsrNrstStdby) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "Life state code (based on Hamming 8,4)."]
        #[must_use]
        #[inline(always)]
        pub const fn product_state(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Life state code (based on Hamming 8,4)."]
        #[inline(always)]
        pub const fn set_product_state(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "IWDG Stop mode freeze option status bit."]
        #[must_use]
        #[inline(always)]
        pub const fn iwdg_stop(&self) -> super::vals::OptsrIwdgStop {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::OptsrIwdgStop::from_bits(val as u8)
        }
        #[doc = "IWDG Stop mode freeze option status bit."]
        #[inline(always)]
        pub const fn set_iwdg_stop(&mut self, val: super::vals::OptsrIwdgStop) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "IWDG Standby mode freeze option status bit."]
        #[must_use]
        #[inline(always)]
        pub const fn iwdg_stdby(&self) -> super::vals::OptsrIwdgStdby {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::OptsrIwdgStdby::from_bits(val as u8)
        }
        #[doc = "IWDG Standby mode freeze option status bit."]
        #[inline(always)]
        pub const fn set_iwdg_stdby(&mut self, val: super::vals::OptsrIwdgStdby) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Unique boot entry control, selects either ST or OEM iRoT for secure boot."]
        #[must_use]
        #[inline(always)]
        pub const fn boot_ube(&self) -> super::vals::OptsrBootUbe {
            let val = (self.0 >> 22usize) & 0xff;
            super::vals::OptsrBootUbe::from_bits(val as u8)
        }
        #[doc = "Unique boot entry control, selects either ST or OEM iRoT for secure boot."]
        #[inline(always)]
        pub const fn set_boot_ube(&mut self, val: super::vals::OptsrBootUbe) {
            self.0 = (self.0 & !(0xff << 22usize)) | (((val.to_bits() as u32) & 0xff) << 22usize);
        }
    }
    impl Default for Optsr {
        #[inline(always)]
        fn default() -> Optsr {
            Optsr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Optsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Optsr")
                .field("iwdg_sw", &self.iwdg_sw())
                .field("wwdg_sw", &self.wwdg_sw())
                .field("nrst_stop", &self.nrst_stop())
                .field("nrst_stdby", &self.nrst_stdby())
                .field("product_state", &self.product_state())
                .field("iwdg_stop", &self.iwdg_stop())
                .field("iwdg_stdby", &self.iwdg_stdby())
                .field("boot_ube", &self.boot_ube())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Optsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Optsr {{ iwdg_sw: {:?}, wwdg_sw: {:?}, nrst_stop: {:?}, nrst_stdby: {:?}, product_state: {=u8:?}, iwdg_stop: {:?}, iwdg_stdby: {:?}, boot_ube: {:?} }}",
                self.iwdg_sw(),
                self.wwdg_sw(),
                self.nrst_stop(),
                self.nrst_stdby(),
                self.product_state(),
                self.iwdg_stop(),
                self.iwdg_stdby(),
                self.boot_ube()
            )
        }
    }
    #[doc = "FLASH option status register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optsr2(pub u32);
    impl Optsr2 {
        #[doc = "SRAM1 erase upon system reset."]
        #[must_use]
        #[inline(always)]
        pub const fn sram1_rst(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 erase upon system reset."]
        #[inline(always)]
        pub const fn set_sram1_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "SRAM2 erase upon system reset."]
        #[must_use]
        #[inline(always)]
        pub const fn sram2_rst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 erase upon system reset."]
        #[inline(always)]
        pub const fn set_sram2_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SRAM3 erase upon system reset."]
        #[must_use]
        #[inline(always)]
        pub const fn sram3_rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM3 erase upon system reset."]
        #[inline(always)]
        pub const fn set_sram3_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SRAM1 ECC detection and correction disable."]
        #[must_use]
        #[inline(always)]
        pub const fn sram1_ecc(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 ECC detection and correction disable."]
        #[inline(always)]
        pub const fn set_sram1_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "SRAM2 ECC detection and correction disable."]
        #[must_use]
        #[inline(always)]
        pub const fn sram2_ecc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 ECC detection and correction disable."]
        #[inline(always)]
        pub const fn set_sram2_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "SRAM3 ECC detection and correction disable."]
        #[must_use]
        #[inline(always)]
        pub const fn sram3_ecc(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM3 ECC detection and correction disable."]
        #[inline(always)]
        pub const fn set_sram3_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TrustZone enable configuration bits."]
        #[must_use]
        #[inline(always)]
        pub const fn tzen(&self) -> super::vals::OptsrTzen {
            let val = (self.0 >> 24usize) & 0xff;
            super::vals::OptsrTzen::from_bits(val as u8)
        }
        #[doc = "TrustZone enable configuration bits."]
        #[inline(always)]
        pub const fn set_tzen(&mut self, val: super::vals::OptsrTzen) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val.to_bits() as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Optsr2 {
        #[inline(always)]
        fn default() -> Optsr2 {
            Optsr2(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Optsr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Optsr2")
                .field("sram1_rst", &self.sram1_rst())
                .field("sram2_rst", &self.sram2_rst())
                .field("sram3_rst", &self.sram3_rst())
                .field("sram1_ecc", &self.sram1_ecc())
                .field("sram2_ecc", &self.sram2_ecc())
                .field("sram3_ecc", &self.sram3_ecc())
                .field("tzen", &self.tzen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Optsr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Optsr2 {{ sram1_rst: {=bool:?}, sram2_rst: {=bool:?}, sram3_rst: {=bool:?}, sram1_ecc: {=bool:?}, sram2_ecc: {=bool:?}, sram3_ecc: {=bool:?}, tzen: {:?} }}",
                self.sram1_rst(),
                self.sram2_rst(),
                self.sram3_rst(),
                self.sram1_ecc(),
                self.sram2_ecc(),
                self.sram3_ecc(),
                self.tzen()
            )
        }
    }
    #[doc = "FLASH option status register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optsr3(pub u32);
    impl Optsr3 {
        #[doc = "Bank address space mapping after reset."]
        #[must_use]
        #[inline(always)]
        pub const fn swap_bank(&self) -> super::vals::SwapBank {
            let val = (self.0 >> 8usize) & 0xff;
            super::vals::SwapBank::from_bits(val as u8)
        }
        #[doc = "Bank address space mapping after reset."]
        #[inline(always)]
        pub const fn set_swap_bank(&mut self, val: super::vals::SwapBank) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u32) & 0xff) << 8usize);
        }
        #[doc = "Lock bank address space swapping."]
        #[must_use]
        #[inline(always)]
        pub const fn swap_lockout(&self) -> super::vals::SwapLockout {
            let val = (self.0 >> 16usize) & 0xff;
            super::vals::SwapLockout::from_bits(val as u8)
        }
        #[doc = "Lock bank address space swapping."]
        #[inline(always)]
        pub const fn set_swap_lockout(&mut self, val: super::vals::SwapLockout) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Optsr3 {
        #[inline(always)]
        fn default() -> Optsr3 {
            Optsr3(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Optsr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Optsr3")
                .field("swap_bank", &self.swap_bank())
                .field("swap_lockout", &self.swap_lockout())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Optsr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Optsr3 {{ swap_bank: {:?}, swap_lockout: {:?} }}",
                self.swap_bank(),
                self.swap_lockout()
            )
        }
    }
    #[doc = "FLASH non-secure OTP block lock."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Otpblr(pub u32);
    impl Otpblr {
        #[doc = "OTP block lock."]
        #[must_use]
        #[inline(always)]
        pub const fn lockbl(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "OTP block lock."]
        #[inline(always)]
        pub const fn set_lockbl(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Otpblr {
        #[inline(always)]
        fn default() -> Otpblr {
            Otpblr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Otpblr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Otpblr")
                .field("lockbl[0]", &self.lockbl(0usize))
                .field("lockbl[1]", &self.lockbl(1usize))
                .field("lockbl[2]", &self.lockbl(2usize))
                .field("lockbl[3]", &self.lockbl(3usize))
                .field("lockbl[4]", &self.lockbl(4usize))
                .field("lockbl[5]", &self.lockbl(5usize))
                .field("lockbl[6]", &self.lockbl(6usize))
                .field("lockbl[7]", &self.lockbl(7usize))
                .field("lockbl[8]", &self.lockbl(8usize))
                .field("lockbl[9]", &self.lockbl(9usize))
                .field("lockbl[10]", &self.lockbl(10usize))
                .field("lockbl[11]", &self.lockbl(11usize))
                .field("lockbl[12]", &self.lockbl(12usize))
                .field("lockbl[13]", &self.lockbl(13usize))
                .field("lockbl[14]", &self.lockbl(14usize))
                .field("lockbl[15]", &self.lockbl(15usize))
                .field("lockbl[16]", &self.lockbl(16usize))
                .field("lockbl[17]", &self.lockbl(17usize))
                .field("lockbl[18]", &self.lockbl(18usize))
                .field("lockbl[19]", &self.lockbl(19usize))
                .field("lockbl[20]", &self.lockbl(20usize))
                .field("lockbl[21]", &self.lockbl(21usize))
                .field("lockbl[22]", &self.lockbl(22usize))
                .field("lockbl[23]", &self.lockbl(23usize))
                .field("lockbl[24]", &self.lockbl(24usize))
                .field("lockbl[25]", &self.lockbl(25usize))
                .field("lockbl[26]", &self.lockbl(26usize))
                .field("lockbl[27]", &self.lockbl(27usize))
                .field("lockbl[28]", &self.lockbl(28usize))
                .field("lockbl[29]", &self.lockbl(29usize))
                .field("lockbl[30]", &self.lockbl(30usize))
                .field("lockbl[31]", &self.lockbl(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Otpblr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Otpblr {{ lockbl[0]: {=bool:?}, lockbl[1]: {=bool:?}, lockbl[2]: {=bool:?}, lockbl[3]: {=bool:?}, lockbl[4]: {=bool:?}, lockbl[5]: {=bool:?}, lockbl[6]: {=bool:?}, lockbl[7]: {=bool:?}, lockbl[8]: {=bool:?}, lockbl[9]: {=bool:?}, lockbl[10]: {=bool:?}, lockbl[11]: {=bool:?}, lockbl[12]: {=bool:?}, lockbl[13]: {=bool:?}, lockbl[14]: {=bool:?}, lockbl[15]: {=bool:?}, lockbl[16]: {=bool:?}, lockbl[17]: {=bool:?}, lockbl[18]: {=bool:?}, lockbl[19]: {=bool:?}, lockbl[20]: {=bool:?}, lockbl[21]: {=bool:?}, lockbl[22]: {=bool:?}, lockbl[23]: {=bool:?}, lockbl[24]: {=bool:?}, lockbl[25]: {=bool:?}, lockbl[26]: {=bool:?}, lockbl[27]: {=bool:?}, lockbl[28]: {=bool:?}, lockbl[29]: {=bool:?}, lockbl[30]: {=bool:?}, lockbl[31]: {=bool:?} }}",
                self.lockbl(0usize),
                self.lockbl(1usize),
                self.lockbl(2usize),
                self.lockbl(3usize),
                self.lockbl(4usize),
                self.lockbl(5usize),
                self.lockbl(6usize),
                self.lockbl(7usize),
                self.lockbl(8usize),
                self.lockbl(9usize),
                self.lockbl(10usize),
                self.lockbl(11usize),
                self.lockbl(12usize),
                self.lockbl(13usize),
                self.lockbl(14usize),
                self.lockbl(15usize),
                self.lockbl(16usize),
                self.lockbl(17usize),
                self.lockbl(18usize),
                self.lockbl(19usize),
                self.lockbl(20usize),
                self.lockbl(21usize),
                self.lockbl(22usize),
                self.lockbl(23usize),
                self.lockbl(24usize),
                self.lockbl(25usize),
                self.lockbl(26usize),
                self.lockbl(27usize),
                self.lockbl(28usize),
                self.lockbl(29usize),
                self.lockbl(30usize),
                self.lockbl(31usize)
            )
        }
    }
    #[doc = "FLASH privilege block-based register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privbb(pub u32);
    impl Privbb {
        #[doc = "Privileged 8-Kbyte flash page attribute."]
        #[must_use]
        #[inline(always)]
        pub const fn priv_(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Privileged 8-Kbyte flash page attribute."]
        #[inline(always)]
        pub const fn set_priv_(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Privbb {
        #[inline(always)]
        fn default() -> Privbb {
            Privbb(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Privbb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Privbb")
                .field("priv_[0]", &self.priv_(0usize))
                .field("priv_[1]", &self.priv_(1usize))
                .field("priv_[2]", &self.priv_(2usize))
                .field("priv_[3]", &self.priv_(3usize))
                .field("priv_[4]", &self.priv_(4usize))
                .field("priv_[5]", &self.priv_(5usize))
                .field("priv_[6]", &self.priv_(6usize))
                .field("priv_[7]", &self.priv_(7usize))
                .field("priv_[8]", &self.priv_(8usize))
                .field("priv_[9]", &self.priv_(9usize))
                .field("priv_[10]", &self.priv_(10usize))
                .field("priv_[11]", &self.priv_(11usize))
                .field("priv_[12]", &self.priv_(12usize))
                .field("priv_[13]", &self.priv_(13usize))
                .field("priv_[14]", &self.priv_(14usize))
                .field("priv_[15]", &self.priv_(15usize))
                .field("priv_[16]", &self.priv_(16usize))
                .field("priv_[17]", &self.priv_(17usize))
                .field("priv_[18]", &self.priv_(18usize))
                .field("priv_[19]", &self.priv_(19usize))
                .field("priv_[20]", &self.priv_(20usize))
                .field("priv_[21]", &self.priv_(21usize))
                .field("priv_[22]", &self.priv_(22usize))
                .field("priv_[23]", &self.priv_(23usize))
                .field("priv_[24]", &self.priv_(24usize))
                .field("priv_[25]", &self.priv_(25usize))
                .field("priv_[26]", &self.priv_(26usize))
                .field("priv_[27]", &self.priv_(27usize))
                .field("priv_[28]", &self.priv_(28usize))
                .field("priv_[29]", &self.priv_(29usize))
                .field("priv_[30]", &self.priv_(30usize))
                .field("priv_[31]", &self.priv_(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privbb {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Privbb {{ priv_[0]: {=bool:?}, priv_[1]: {=bool:?}, priv_[2]: {=bool:?}, priv_[3]: {=bool:?}, priv_[4]: {=bool:?}, priv_[5]: {=bool:?}, priv_[6]: {=bool:?}, priv_[7]: {=bool:?}, priv_[8]: {=bool:?}, priv_[9]: {=bool:?}, priv_[10]: {=bool:?}, priv_[11]: {=bool:?}, priv_[12]: {=bool:?}, priv_[13]: {=bool:?}, priv_[14]: {=bool:?}, priv_[15]: {=bool:?}, priv_[16]: {=bool:?}, priv_[17]: {=bool:?}, priv_[18]: {=bool:?}, priv_[19]: {=bool:?}, priv_[20]: {=bool:?}, priv_[21]: {=bool:?}, priv_[22]: {=bool:?}, priv_[23]: {=bool:?}, priv_[24]: {=bool:?}, priv_[25]: {=bool:?}, priv_[26]: {=bool:?}, priv_[27]: {=bool:?}, priv_[28]: {=bool:?}, priv_[29]: {=bool:?}, priv_[30]: {=bool:?}, priv_[31]: {=bool:?} }}",
                self.priv_(0usize),
                self.priv_(1usize),
                self.priv_(2usize),
                self.priv_(3usize),
                self.priv_(4usize),
                self.priv_(5usize),
                self.priv_(6usize),
                self.priv_(7usize),
                self.priv_(8usize),
                self.priv_(9usize),
                self.priv_(10usize),
                self.priv_(11usize),
                self.priv_(12usize),
                self.priv_(13usize),
                self.priv_(14usize),
                self.priv_(15usize),
                self.priv_(16usize),
                self.priv_(17usize),
                self.priv_(18usize),
                self.priv_(19usize),
                self.priv_(20usize),
                self.priv_(21usize),
                self.priv_(22usize),
                self.priv_(23usize),
                self.priv_(24usize),
                self.priv_(25usize),
                self.priv_(26usize),
                self.priv_(27usize),
                self.priv_(28usize),
                self.priv_(29usize),
                self.priv_(30usize),
                self.priv_(31usize)
            )
        }
    }
    #[doc = "FLASH privilege configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcfgr(pub u32);
    impl Privcfgr {
        #[doc = "Privilege attribute for secure registers."]
        #[must_use]
        #[inline(always)]
        pub const fn spriv(&self) -> super::vals::Spriv {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Spriv::from_bits(val as u8)
        }
        #[doc = "Privilege attribute for secure registers."]
        #[inline(always)]
        pub const fn set_spriv(&mut self, val: super::vals::Spriv) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Privilege attribute for non-secure registers."]
        #[must_use]
        #[inline(always)]
        pub const fn priv_(&self) -> super::vals::Nspriv {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Nspriv::from_bits(val as u8)
        }
        #[doc = "Privilege attribute for non-secure registers."]
        #[inline(always)]
        pub const fn set_priv_(&mut self, val: super::vals::Nspriv) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Privcfgr {
        #[inline(always)]
        fn default() -> Privcfgr {
            Privcfgr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Privcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Privcfgr")
                .field("spriv", &self.spriv())
                .field("priv_", &self.priv_())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privcfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Privcfgr {{ spriv: {:?}, priv_: {:?} }}", self.spriv(), self.priv_())
        }
    }
    #[doc = "FLASH secure boot register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sbootr(pub u32);
    impl Sbootr {
        #[doc = "Field locking the values of BOOT_UBE and SBOOTADD."]
        #[must_use]
        #[inline(always)]
        pub const fn sboot_lock(&self) -> super::vals::BootLock {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::BootLock::from_bits(val as u8)
        }
        #[doc = "Field locking the values of BOOT_UBE and SBOOTADD."]
        #[inline(always)]
        pub const fn set_sboot_lock(&mut self, val: super::vals::BootLock) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
        #[doc = "Secure unique boot entry address."]
        #[must_use]
        #[inline(always)]
        pub const fn sbootadd(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "Secure unique boot entry address."]
        #[inline(always)]
        pub const fn set_sbootadd(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for Sbootr {
        #[inline(always)]
        fn default() -> Sbootr {
            Sbootr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Sbootr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sbootr")
                .field("sboot_lock", &self.sboot_lock())
                .field("sbootadd", &self.sbootadd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sbootr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sbootr {{ sboot_lock: {:?}, sbootadd: {=u32:?} }}",
                self.sboot_lock(),
                self.sbootadd()
            )
        }
    }
    #[doc = "FLASH secure clear control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sccr(pub u32);
    impl Sccr {
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
        #[doc = "OBKERR flag clear bit."]
        #[must_use]
        #[inline(always)]
        pub const fn clr_obkerr(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "OBKERR flag clear bit."]
        #[inline(always)]
        pub const fn set_clr_obkerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "OBKWERR flag clear bit."]
        #[must_use]
        #[inline(always)]
        pub const fn clr_obkwerr(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "OBKWERR flag clear bit."]
        #[inline(always)]
        pub const fn set_clr_obkwerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Sccr {
        #[inline(always)]
        fn default() -> Sccr {
            Sccr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Sccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sccr")
                .field("clr_eop", &self.clr_eop())
                .field("clr_wrperr", &self.clr_wrperr())
                .field("clr_pgserr", &self.clr_pgserr())
                .field("clr_strberr", &self.clr_strberr())
                .field("clr_incerr", &self.clr_incerr())
                .field("clr_obkerr", &self.clr_obkerr())
                .field("clr_obkwerr", &self.clr_obkwerr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sccr {{ clr_eop: {=bool:?}, clr_wrperr: {=bool:?}, clr_pgserr: {=bool:?}, clr_strberr: {=bool:?}, clr_incerr: {=bool:?}, clr_obkerr: {=bool:?}, clr_obkwerr: {=bool:?} }}",
                self.clr_eop(),
                self.clr_wrperr(),
                self.clr_pgserr(),
                self.clr_strberr(),
                self.clr_incerr(),
                self.clr_obkerr(),
                self.clr_obkwerr()
            )
        }
    }
    #[doc = "FLASH secure control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scr(pub u32);
    impl Scr {
        #[doc = "Configuration lock bit."]
        #[must_use]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Configuration lock bit."]
        #[inline(always)]
        pub const fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Programming control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn pg(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Programming control bit."]
        #[inline(always)]
        pub const fn set_pg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Page erase request."]
        #[must_use]
        #[inline(always)]
        pub const fn per(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Page erase request."]
        #[inline(always)]
        pub const fn set_per(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Bank erase request."]
        #[must_use]
        #[inline(always)]
        pub const fn ber(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Bank erase request."]
        #[inline(always)]
        pub const fn set_ber(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Write forcing control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn fw(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Write forcing control bit."]
        #[inline(always)]
        pub const fn set_fw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Erase start control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn strt(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Erase start control bit."]
        #[inline(always)]
        pub const fn set_strt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Page erase selection number."]
        #[must_use]
        #[inline(always)]
        pub const fn pnb(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x1f;
            val as u8
        }
        #[doc = "Page erase selection number."]
        #[inline(always)]
        pub const fn set_pnb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
        }
        #[doc = "PUF preparation start control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn puf_launch(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "PUF preparation start control bit."]
        #[inline(always)]
        pub const fn set_puf_launch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Mass erase request."]
        #[must_use]
        #[inline(always)]
        pub const fn mer(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Mass erase request."]
        #[inline(always)]
        pub const fn set_mer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "End of operation interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn eopie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "End of operation interrupt enable."]
        #[inline(always)]
        pub const fn set_eopie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Write protection error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn wrperrie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Write protection error interrupt enable."]
        #[inline(always)]
        pub const fn set_wrperrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Programming sequence error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pgserrie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Programming sequence error interrupt enable."]
        #[inline(always)]
        pub const fn set_pgserrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Strobe error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn strberrie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Strobe error interrupt enable."]
        #[inline(always)]
        pub const fn set_strberrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Inconsistency error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn incerrie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Inconsistency error interrupt enable."]
        #[inline(always)]
        pub const fn set_incerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "OBK general error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn obkerrie(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "OBK general error interrupt enable."]
        #[inline(always)]
        pub const fn set_obkerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "OBK write error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn obkwerrie(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "OBK write error interrupt enable."]
        #[inline(always)]
        pub const fn set_obkwerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Flash memory security state invert."]
        #[must_use]
        #[inline(always)]
        pub const fn inv(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Flash memory security state invert."]
        #[inline(always)]
        pub const fn set_inv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
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
    impl Default for Scr {
        #[inline(always)]
        fn default() -> Scr {
            Scr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Scr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Scr")
                .field("lock", &self.lock())
                .field("pg", &self.pg())
                .field("per", &self.per())
                .field("ber", &self.ber())
                .field("fw", &self.fw())
                .field("strt", &self.strt())
                .field("pnb", &self.pnb())
                .field("puf_launch", &self.puf_launch())
                .field("mer", &self.mer())
                .field("eopie", &self.eopie())
                .field("wrperrie", &self.wrperrie())
                .field("pgserrie", &self.pgserrie())
                .field("strberrie", &self.strberrie())
                .field("incerrie", &self.incerrie())
                .field("obkerrie", &self.obkerrie())
                .field("obkwerrie", &self.obkwerrie())
                .field("inv", &self.inv())
                .field("bksel", &self.bksel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Scr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Scr {{ lock: {=bool:?}, pg: {=bool:?}, per: {=bool:?}, ber: {=bool:?}, fw: {=bool:?}, strt: {=bool:?}, pnb: {=u8:?}, puf_launch: {=bool:?}, mer: {=bool:?}, eopie: {=bool:?}, wrperrie: {=bool:?}, pgserrie: {=bool:?}, strberrie: {=bool:?}, incerrie: {=bool:?}, obkerrie: {=bool:?}, obkwerrie: {=bool:?}, inv: {=bool:?}, bksel: {:?} }}",
                self.lock(),
                self.pg(),
                self.per(),
                self.ber(),
                self.fw(),
                self.strt(),
                self.pnb(),
                self.puf_launch(),
                self.mer(),
                self.eopie(),
                self.wrperrie(),
                self.pgserrie(),
                self.strberrie(),
                self.incerrie(),
                self.obkerrie(),
                self.obkwerrie(),
                self.inv(),
                self.bksel()
            )
        }
    }
    #[doc = "FLASH secure block-based register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secbb(pub u32);
    impl Secbb {
        #[doc = "Secure 8-Kbyte flash page attribute."]
        #[must_use]
        #[inline(always)]
        pub const fn sec(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Secure 8-Kbyte flash page attribute."]
        #[inline(always)]
        pub const fn set_sec(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Secbb {
        #[inline(always)]
        fn default() -> Secbb {
            Secbb(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Secbb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secbb")
                .field("sec[0]", &self.sec(0usize))
                .field("sec[1]", &self.sec(1usize))
                .field("sec[2]", &self.sec(2usize))
                .field("sec[3]", &self.sec(3usize))
                .field("sec[4]", &self.sec(4usize))
                .field("sec[5]", &self.sec(5usize))
                .field("sec[6]", &self.sec(6usize))
                .field("sec[7]", &self.sec(7usize))
                .field("sec[8]", &self.sec(8usize))
                .field("sec[9]", &self.sec(9usize))
                .field("sec[10]", &self.sec(10usize))
                .field("sec[11]", &self.sec(11usize))
                .field("sec[12]", &self.sec(12usize))
                .field("sec[13]", &self.sec(13usize))
                .field("sec[14]", &self.sec(14usize))
                .field("sec[15]", &self.sec(15usize))
                .field("sec[16]", &self.sec(16usize))
                .field("sec[17]", &self.sec(17usize))
                .field("sec[18]", &self.sec(18usize))
                .field("sec[19]", &self.sec(19usize))
                .field("sec[20]", &self.sec(20usize))
                .field("sec[21]", &self.sec(21usize))
                .field("sec[22]", &self.sec(22usize))
                .field("sec[23]", &self.sec(23usize))
                .field("sec[24]", &self.sec(24usize))
                .field("sec[25]", &self.sec(25usize))
                .field("sec[26]", &self.sec(26usize))
                .field("sec[27]", &self.sec(27usize))
                .field("sec[28]", &self.sec(28usize))
                .field("sec[29]", &self.sec(29usize))
                .field("sec[30]", &self.sec(30usize))
                .field("sec[31]", &self.sec(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secbb {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Secbb {{ sec[0]: {=bool:?}, sec[1]: {=bool:?}, sec[2]: {=bool:?}, sec[3]: {=bool:?}, sec[4]: {=bool:?}, sec[5]: {=bool:?}, sec[6]: {=bool:?}, sec[7]: {=bool:?}, sec[8]: {=bool:?}, sec[9]: {=bool:?}, sec[10]: {=bool:?}, sec[11]: {=bool:?}, sec[12]: {=bool:?}, sec[13]: {=bool:?}, sec[14]: {=bool:?}, sec[15]: {=bool:?}, sec[16]: {=bool:?}, sec[17]: {=bool:?}, sec[18]: {=bool:?}, sec[19]: {=bool:?}, sec[20]: {=bool:?}, sec[21]: {=bool:?}, sec[22]: {=bool:?}, sec[23]: {=bool:?}, sec[24]: {=bool:?}, sec[25]: {=bool:?}, sec[26]: {=bool:?}, sec[27]: {=bool:?}, sec[28]: {=bool:?}, sec[29]: {=bool:?}, sec[30]: {=bool:?}, sec[31]: {=bool:?} }}",
                self.sec(0usize),
                self.sec(1usize),
                self.sec(2usize),
                self.sec(3usize),
                self.sec(4usize),
                self.sec(5usize),
                self.sec(6usize),
                self.sec(7usize),
                self.sec(8usize),
                self.sec(9usize),
                self.sec(10usize),
                self.sec(11usize),
                self.sec(12usize),
                self.sec(13usize),
                self.sec(14usize),
                self.sec(15usize),
                self.sec(16usize),
                self.sec(17usize),
                self.sec(18usize),
                self.sec(19usize),
                self.sec(20usize),
                self.sec(21usize),
                self.sec(22usize),
                self.sec(23usize),
                self.sec(24usize),
                self.sec(25usize),
                self.sec(26usize),
                self.sec(27usize),
                self.sec(28usize),
                self.sec(29usize),
                self.sec(30usize),
                self.sec(31usize)
            )
        }
    }
    #[doc = "FLASH security watermark register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secwm(pub u32);
    impl Secwm {
        #[doc = "Bank security watermark area start page."]
        #[must_use]
        #[inline(always)]
        pub const fn strt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Bank security watermark area start page."]
        #[inline(always)]
        pub const fn set_strt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Bank security watermark area end page."]
        #[must_use]
        #[inline(always)]
        pub const fn end(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x1f;
            val as u8
        }
        #[doc = "Bank security watermark area end page."]
        #[inline(always)]
        pub const fn set_end(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
        }
    }
    impl Default for Secwm {
        #[inline(always)]
        fn default() -> Secwm {
            Secwm(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Secwm {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Secwm")
                .field("strt", &self.strt())
                .field("end", &self.end())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Secwm {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Secwm {{ strt: {=u8:?}, end: {=u8:?} }}", self.strt(), self.end())
        }
    }
    #[doc = "FLASH non-secure status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Busy flag."]
        #[must_use]
        #[inline(always)]
        pub const fn bsy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Busy flag."]
        #[inline(always)]
        pub const fn set_bsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Write buffer not empty flag."]
        #[must_use]
        #[inline(always)]
        pub const fn wbne(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Write buffer not empty flag."]
        #[inline(always)]
        pub const fn set_wbne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PUF readiness."]
        #[must_use]
        #[inline(always)]
        pub const fn puf_state(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PUF readiness."]
        #[inline(always)]
        pub const fn set_puf_state(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Data buffer not empty flag."]
        #[must_use]
        #[inline(always)]
        pub const fn dbne(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Data buffer not empty flag."]
        #[inline(always)]
        pub const fn set_dbne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "End of operation flag."]
        #[must_use]
        #[inline(always)]
        pub const fn eop(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "End of operation flag."]
        #[inline(always)]
        pub const fn set_eop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Write protection error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn wrperr(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Write protection error flag."]
        #[inline(always)]
        pub const fn set_wrperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Programming sequence error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn pgserr(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Programming sequence error flag."]
        #[inline(always)]
        pub const fn set_pgserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Strobe error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn strberr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Strobe error flag."]
        #[inline(always)]
        pub const fn set_strberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Inconsistency error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn incerr(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Inconsistency error flag."]
        #[inline(always)]
        pub const fn set_incerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "OBK general error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn obkerr(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "OBK general error flag."]
        #[inline(always)]
        pub const fn set_obkerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "OBK write error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn obkwerr(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "OBK write error flag."]
        #[inline(always)]
        pub const fn set_obkwerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Option byte change error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn optchangeerr(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte change error flag."]
        #[inline(always)]
        pub const fn set_optchangeerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr")
                .field("bsy", &self.bsy())
                .field("wbne", &self.wbne())
                .field("puf_state", &self.puf_state())
                .field("dbne", &self.dbne())
                .field("eop", &self.eop())
                .field("wrperr", &self.wrperr())
                .field("pgserr", &self.pgserr())
                .field("strberr", &self.strberr())
                .field("incerr", &self.incerr())
                .field("obkerr", &self.obkerr())
                .field("obkwerr", &self.obkwerr())
                .field("optchangeerr", &self.optchangeerr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ bsy: {=bool:?}, wbne: {=bool:?}, puf_state: {=bool:?}, dbne: {=bool:?}, eop: {=bool:?}, wrperr: {=bool:?}, pgserr: {=bool:?}, strberr: {=bool:?}, incerr: {=bool:?}, obkerr: {=bool:?}, obkwerr: {=bool:?}, optchangeerr: {=bool:?} }}",
                self.bsy(),
                self.wbne(),
                self.puf_state(),
                self.dbne(),
                self.eop(),
                self.wrperr(),
                self.pgserr(),
                self.strberr(),
                self.incerr(),
                self.obkerr(),
                self.obkwerr(),
                self.optchangeerr()
            )
        }
    }
    #[doc = "FLASH secure status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ssr(pub u32);
    impl Ssr {
        #[doc = "Busy flag."]
        #[must_use]
        #[inline(always)]
        pub const fn bsy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Busy flag."]
        #[inline(always)]
        pub const fn set_bsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Write buffer not empty flag."]
        #[must_use]
        #[inline(always)]
        pub const fn wbne(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Write buffer not empty flag."]
        #[inline(always)]
        pub const fn set_wbne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PUF readiness."]
        #[must_use]
        #[inline(always)]
        pub const fn puf_state(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PUF readiness."]
        #[inline(always)]
        pub const fn set_puf_state(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Data buffer not empty flag."]
        #[must_use]
        #[inline(always)]
        pub const fn dbne(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Data buffer not empty flag."]
        #[inline(always)]
        pub const fn set_dbne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "End of operation flag."]
        #[must_use]
        #[inline(always)]
        pub const fn eop(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "End of operation flag."]
        #[inline(always)]
        pub const fn set_eop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Write protection error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn wrperr(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Write protection error flag."]
        #[inline(always)]
        pub const fn set_wrperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Programming sequence error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn pgserr(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Programming sequence error flag."]
        #[inline(always)]
        pub const fn set_pgserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Strobe error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn strberr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Strobe error flag."]
        #[inline(always)]
        pub const fn set_strberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Inconsistency error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn incerr(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Inconsistency error flag."]
        #[inline(always)]
        pub const fn set_incerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "OBK general error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn obkerr(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "OBK general error flag."]
        #[inline(always)]
        pub const fn set_obkerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "OBK write error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn obkwerr(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "OBK write error flag."]
        #[inline(always)]
        pub const fn set_obkwerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
    }
    impl Default for Ssr {
        #[inline(always)]
        fn default() -> Ssr {
            Ssr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Ssr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ssr")
                .field("bsy", &self.bsy())
                .field("wbne", &self.wbne())
                .field("puf_state", &self.puf_state())
                .field("dbne", &self.dbne())
                .field("eop", &self.eop())
                .field("wrperr", &self.wrperr())
                .field("pgserr", &self.pgserr())
                .field("strberr", &self.strberr())
                .field("incerr", &self.incerr())
                .field("obkerr", &self.obkerr())
                .field("obkwerr", &self.obkwerr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ssr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ssr {{ bsy: {=bool:?}, wbne: {=bool:?}, puf_state: {=bool:?}, dbne: {=bool:?}, eop: {=bool:?}, wrperr: {=bool:?}, pgserr: {=bool:?}, strberr: {=bool:?}, incerr: {=bool:?}, obkerr: {=bool:?}, obkwerr: {=bool:?} }}",
                self.bsy(),
                self.wbne(),
                self.puf_state(),
                self.dbne(),
                self.eop(),
                self.wrperr(),
                self.pgserr(),
                self.strberr(),
                self.incerr(),
                self.obkerr(),
                self.obkwerr()
            )
        }
    }
    #[doc = "FLASH write page group protection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrp(pub u32);
    impl Wrp {
        #[doc = "Bank page group protection option status byte."]
        #[must_use]
        #[inline(always)]
        pub const fn wrpg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Bank page group protection option status byte."]
        #[inline(always)]
        pub const fn set_wrpg(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Wrp {
        #[inline(always)]
        fn default() -> Wrp {
            Wrp(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Wrp {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wrp").field("wrpg", &self.wrpg()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrp {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wrp {{ wrpg: {=u8:?} }}", self.wrpg())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Bksel {
        #[doc = "Bank 1 is selected"]
        Bank1 = 0x0,
        #[doc = "Bank 2 is selected"]
        Bank2 = 0x01,
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
    pub struct BootLock(u8);
    impl BootLock {
        #[doc = "Boot address is frozen"]
        pub const Locked: Self = Self(0xb4);
        #[doc = "Boot address can still be modified following its individual rules"]
        pub const Unlocked: Self = Self(0xc3);
    }
    impl BootLock {
        pub const fn from_bits(val: u8) -> BootLock {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for BootLock {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0xb4 => f.write_str("Locked"),
                0xc3 => f.write_str("Unlocked"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BootLock {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0xb4 => defmt::write!(f, "Locked"),
                0xc3 => defmt::write!(f, "Unlocked"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for BootLock {
        #[inline(always)]
        fn from(val: u8) -> BootLock {
            BootLock::from_bits(val)
        }
    }
    impl From<BootLock> for u8 {
        #[inline(always)]
        fn from(val: BootLock) -> u8 {
            BootLock::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum CodeOp {
        #[doc = "No flash operation on going during previous reset"]
        NoFlashInt = 0x0,
        #[doc = "Single write operation interrupted"]
        SingleWrInt = 0x01,
        #[doc = "OBK alternate page erase"]
        ObkAltPageErase = 0x02,
        #[doc = "Page erase operation interrupted"]
        PageEraseInt = 0x03,
        #[doc = "Bank erase operation interrupted"]
        BankEraseInt = 0x04,
        #[doc = "Mass erase operation interrupted"]
        MassEraseInt = 0x05,
        #[doc = "Option change operation interrupted"]
        OptChangeInt = 0x06,
        #[doc = "OBK swap page request"]
        ObkSwapPageRequest = 0x07,
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Nspriv {
        #[doc = "Access to non-secure registers is always granted"]
        Any = 0x0,
        #[doc = "Access to non-secure registers is denied for unprivileged access"]
        Privileged = 0x01,
    }
    impl Nspriv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Nspriv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Nspriv {
        #[inline(always)]
        fn from(val: u8) -> Nspriv {
            Nspriv::from_bits(val)
        }
    }
    impl From<Nspriv> for u8 {
        #[inline(always)]
        fn from(val: Nspriv) -> u8 {
            Nspriv::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct OptsrBootUbe(u8);
    impl OptsrBootUbe {
        #[doc = "OEM-iRoT (user flash) selected"]
        pub const OemIrot: Self = Self(0xb4);
        #[doc = "ST-iRoT (system flash) selected"]
        pub const StIrot: Self = Self(0xc3);
    }
    impl OptsrBootUbe {
        pub const fn from_bits(val: u8) -> OptsrBootUbe {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for OptsrBootUbe {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0xb4 => f.write_str("OemIrot"),
                0xc3 => f.write_str("StIrot"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OptsrBootUbe {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0xb4 => defmt::write!(f, "OemIrot"),
                0xc3 => defmt::write!(f, "StIrot"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for OptsrBootUbe {
        #[inline(always)]
        fn from(val: u8) -> OptsrBootUbe {
            OptsrBootUbe::from_bits(val)
        }
    }
    impl From<OptsrBootUbe> for u8 {
        #[inline(always)]
        fn from(val: OptsrBootUbe) -> u8 {
            OptsrBootUbe::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrIwdgStdby {
        #[doc = "Independent watchdog frozen in Standby mode"]
        Frozen = 0x0,
        #[doc = "Independent watchdog keeps running in Standby mode"]
        Running = 0x01,
    }
    impl OptsrIwdgStdby {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrIwdgStdby {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrIwdgStdby {
        #[inline(always)]
        fn from(val: u8) -> OptsrIwdgStdby {
            OptsrIwdgStdby::from_bits(val)
        }
    }
    impl From<OptsrIwdgStdby> for u8 {
        #[inline(always)]
        fn from(val: OptsrIwdgStdby) -> u8 {
            OptsrIwdgStdby::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrIwdgStop {
        #[doc = "Independent watchdog frozen in Stop mode"]
        Frozen = 0x0,
        #[doc = "Independent watchdog keeps running in Stop mode"]
        Running = 0x01,
    }
    impl OptsrIwdgStop {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrIwdgStop {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrIwdgStop {
        #[inline(always)]
        fn from(val: u8) -> OptsrIwdgStop {
            OptsrIwdgStop::from_bits(val)
        }
    }
    impl From<OptsrIwdgStop> for u8 {
        #[inline(always)]
        fn from(val: OptsrIwdgStop) -> u8 {
            OptsrIwdgStop::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrIwdgSw {
        #[doc = "IWDG watchdog is controlled by hardware"]
        Hardware = 0x0,
        #[doc = "IWDG watchdog is controlled by software"]
        Software = 0x01,
    }
    impl OptsrIwdgSw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrIwdgSw {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrIwdgSw {
        #[inline(always)]
        fn from(val: u8) -> OptsrIwdgSw {
            OptsrIwdgSw::from_bits(val)
        }
    }
    impl From<OptsrIwdgSw> for u8 {
        #[inline(always)]
        fn from(val: OptsrIwdgSw) -> u8 {
            OptsrIwdgSw::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrNrstStdby {
        #[doc = "A reset is generated when entering Standby mode"]
        Reset = 0x0,
        #[doc = "No reset generated when entering Standby mode"]
        NoReset = 0x01,
    }
    impl OptsrNrstStdby {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrNrstStdby {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrNrstStdby {
        #[inline(always)]
        fn from(val: u8) -> OptsrNrstStdby {
            OptsrNrstStdby::from_bits(val)
        }
    }
    impl From<OptsrNrstStdby> for u8 {
        #[inline(always)]
        fn from(val: OptsrNrstStdby) -> u8 {
            OptsrNrstStdby::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrNrstStop {
        #[doc = "A reset is generated when entering Stop mode"]
        Reset = 0x0,
        #[doc = "No reset generated when entering Stop mode"]
        NoReset = 0x01,
    }
    impl OptsrNrstStop {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrNrstStop {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrNrstStop {
        #[inline(always)]
        fn from(val: u8) -> OptsrNrstStop {
            OptsrNrstStop::from_bits(val)
        }
    }
    impl From<OptsrNrstStop> for u8 {
        #[inline(always)]
        fn from(val: OptsrNrstStop) -> u8 {
            OptsrNrstStop::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct OptsrTzen(u8);
    impl OptsrTzen {
        #[doc = "TrustZone enabled"]
        pub const Enabled: Self = Self(0xb4);
        #[doc = "TrustZone disabled"]
        pub const Disabled: Self = Self(0xc3);
    }
    impl OptsrTzen {
        pub const fn from_bits(val: u8) -> OptsrTzen {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for OptsrTzen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0xb4 => f.write_str("Enabled"),
                0xc3 => f.write_str("Disabled"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for OptsrTzen {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0xb4 => defmt::write!(f, "Enabled"),
                0xc3 => defmt::write!(f, "Disabled"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for OptsrTzen {
        #[inline(always)]
        fn from(val: u8) -> OptsrTzen {
            OptsrTzen::from_bits(val)
        }
    }
    impl From<OptsrTzen> for u8 {
        #[inline(always)]
        fn from(val: OptsrTzen) -> u8 {
            OptsrTzen::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrWwdgSw {
        #[doc = "WWDG watchdog is controlled by hardware"]
        Hardware = 0x0,
        #[doc = "WWDG watchdog is controlled by software"]
        Software = 0x01,
    }
    impl OptsrWwdgSw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrWwdgSw {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrWwdgSw {
        #[inline(always)]
        fn from(val: u8) -> OptsrWwdgSw {
            OptsrWwdgSw::from_bits(val)
        }
    }
    impl From<OptsrWwdgSw> for u8 {
        #[inline(always)]
        fn from(val: OptsrWwdgSw) -> u8 {
            OptsrWwdgSw::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Spriv {
        #[doc = "Access to secure registers is always granted"]
        Any = 0x0,
        #[doc = "Access to secure registers is denied for unprivileged access"]
        Privileged = 0x01,
    }
    impl Spriv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Spriv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Spriv {
        #[inline(always)]
        fn from(val: u8) -> Spriv {
            Spriv::from_bits(val)
        }
    }
    impl From<Spriv> for u8 {
        #[inline(always)]
        fn from(val: Spriv) -> u8 {
            Spriv::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct SwapBank(u8);
    impl SwapBank {
        #[doc = "Bank 2 starts first"]
        pub const Altered: Self = Self(0xb4);
        #[doc = "Bank 1 starts first"]
        pub const Default: Self = Self(0xc3);
    }
    impl SwapBank {
        pub const fn from_bits(val: u8) -> SwapBank {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for SwapBank {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0xb4 => f.write_str("Altered"),
                0xc3 => f.write_str("Default"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SwapBank {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0xb4 => defmt::write!(f, "Altered"),
                0xc3 => defmt::write!(f, "Default"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for SwapBank {
        #[inline(always)]
        fn from(val: u8) -> SwapBank {
            SwapBank::from_bits(val)
        }
    }
    impl From<SwapBank> for u8 {
        #[inline(always)]
        fn from(val: SwapBank) -> u8 {
            SwapBank::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct SwapLockout(u8);
    impl SwapLockout {
        #[doc = "Swapping disabled by configuration"]
        pub const Disabled: Self = Self(0xb4);
        #[doc = "Swapping permitted"]
        pub const Permitted: Self = Self(0xc3);
    }
    impl SwapLockout {
        pub const fn from_bits(val: u8) -> SwapLockout {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for SwapLockout {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0xb4 => f.write_str("Disabled"),
                0xc3 => f.write_str("Permitted"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for SwapLockout {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0xb4 => defmt::write!(f, "Disabled"),
                0xc3 => defmt::write!(f, "Permitted"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u8> for SwapLockout {
        #[inline(always)]
        fn from(val: u8) -> SwapLockout {
            SwapLockout::from_bits(val)
        }
    }
    impl From<SwapLockout> for u8 {
        #[inline(always)]
        fn from(val: SwapLockout) -> u8 {
            SwapLockout::to_bits(val)
        }
    }
}
