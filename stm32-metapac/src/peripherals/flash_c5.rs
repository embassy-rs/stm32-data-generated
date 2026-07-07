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
    pub const fn keyr(self) -> crate::common::Reg<u32, crate::common::W> {
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
    pub const fn optsr_cur(self) -> crate::common::Reg<regs::Optsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "FLASH option status register."]
    #[inline(always)]
    pub const fn optsr_prg(self) -> crate::common::Reg<regs::Optsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
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
    #[doc = "FLASH unique boot entry register."]
    #[inline(always)]
    pub const fn bootr_cur(self) -> crate::common::Reg<regs::Bootr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "FLASH unique boot entry address."]
    #[inline(always)]
    pub const fn bootr_prg(self) -> crate::common::Reg<regs::Bootr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "FLASH OTP block lock."]
    #[inline(always)]
    pub const fn otpblr_cur(self) -> crate::common::Reg<regs::Otpblr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "FLASH OTP block lock."]
    #[inline(always)]
    pub const fn otpblr_prg(self) -> crate::common::Reg<regs::Otpblr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "FLASH Bootloader interface selection."]
    #[inline(always)]
    pub const fn bl_com_cfg_cur(self) -> crate::common::Reg<regs::BlComCfg, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "FLASH Bootloader interface selection."]
    #[inline(always)]
    pub const fn bl_com_cfg_prg(self) -> crate::common::Reg<regs::BlComCfg, crate::common::R> {
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
    pub const fn wrp1r_cur(self) -> crate::common::Reg<regs::Wrp1r, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "FLASH write page protection for bank1."]
    #[inline(always)]
    pub const fn wrp1r_prg(self) -> crate::common::Reg<regs::Wrp1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "FLASH HDP bank1 register."]
    #[inline(always)]
    pub const fn hdp1r_cur(self) -> crate::common::Reg<regs::Hdp1r, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "FLASH HDP bank1 register."]
    #[inline(always)]
    pub const fn hdp1r_prg(self) -> crate::common::Reg<regs::Hdp1r, crate::common::RW> {
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
    pub const fn wrp2r_cur(self) -> crate::common::Reg<regs::Wrp2r, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[doc = "FLASH write page protection for bank2."]
    #[inline(always)]
    pub const fn wrp2r_prg(self) -> crate::common::Reg<regs::Wrp2r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ecusize) as _) }
    }
    #[doc = "FLASH HDP bank2 register."]
    #[inline(always)]
    pub const fn hdp2r_cur(self) -> crate::common::Reg<regs::Hdp2r, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "FLASH HDP bank2 register."]
    #[inline(always)]
    pub const fn hdp2r_prg(self) -> crate::common::Reg<regs::Hdp2r, crate::common::RW> {
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
        pub const fn latency(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Read latency."]
        #[inline(always)]
        pub const fn set_latency(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
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
        pub const fn prften(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Prefetch enable."]
        #[inline(always)]
        pub const fn set_prften(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Main Flash memory area empty (not reset by system reset)."]
        #[must_use]
        #[inline(always)]
        pub const fn empty(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Main Flash memory area empty (not reset by system reset)."]
        #[inline(always)]
        pub const fn set_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
                "Acr {{ latency: {=u8:?}, wrhighfreq: {=u8:?}, prften: {=bool:?}, empty: {=bool:?} }}",
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
    #[doc = "FLASH unique boot entry register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bootr(pub u32);
    impl Bootr {
        #[doc = "A field locking the values of BOOT0, BOOT_SEL, SWAP_BANK, and BOOTADD option settings."]
        #[must_use]
        #[inline(always)]
        pub const fn boot_lock(&self) -> super::vals::BootLock {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::BootLock::from_bits(val as u8)
        }
        #[doc = "A field locking the values of BOOT0, BOOT_SEL, SWAP_BANK, and BOOTADD option settings."]
        #[inline(always)]
        pub const fn set_boot_lock(&mut self, val: super::vals::BootLock) {
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
    impl Default for Bootr {
        #[inline(always)]
        fn default() -> Bootr {
            Bootr(0)
        }
    }
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
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "configuration lock bit."]
        #[inline(always)]
        pub const fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "programming control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn pg(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "programming control bit."]
        #[inline(always)]
        pub const fn set_pg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "page erase request."]
        #[must_use]
        #[inline(always)]
        pub const fn per(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "page erase request."]
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
        pub const fn pnb(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x3f;
            val as u8
        }
        #[doc = "page erase selection number."]
        #[inline(always)]
        pub const fn set_pnb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 6usize)) | (((val as u32) & 0x3f) << 6usize);
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
        #[doc = "end of operation interrupt control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn eopie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "end of operation interrupt control bit."]
        #[inline(always)]
        pub const fn set_eopie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "write protection error interrupt enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn wrperrie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "write protection error interrupt enable bit."]
        #[inline(always)]
        pub const fn set_wrperrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "programming sequence error interrupt enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn pgserrie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "programming sequence error interrupt enable bit."]
        #[inline(always)]
        pub const fn set_pgserrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "strobe error interrupt enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn strberrie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "strobe error interrupt enable bit."]
        #[inline(always)]
        pub const fn set_strberrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "inconsistency error interrupt enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn incerrie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "inconsistency error interrupt enable bit."]
        #[inline(always)]
        pub const fn set_incerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Option-byte change error interrupt enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn optchangeerrie(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Option-byte change error interrupt enable bit."]
        #[inline(always)]
        pub const fn set_optchangeerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
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
                .field("ber", &self.ber())
                .field("fw", &self.fw())
                .field("strt", &self.strt())
                .field("pnb", &self.pnb())
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
                "Cr {{ lock: {=bool:?}, pg: {=bool:?}, per: {=bool:?}, ber: {=bool:?}, fw: {=bool:?}, strt: {=bool:?}, pnb: {=u8:?}, mer: {=bool:?}, eopie: {=bool:?}, wrperrie: {=bool:?}, pgserrie: {=bool:?}, strberrie: {=bool:?}, incerrie: {=bool:?}, optchangeerrie: {=bool:?}, edatasel: {:?}, bksel: {:?} }}",
                self.lock(),
                self.pg(),
                self.per(),
                self.ber(),
                self.fw(),
                self.strt(),
                self.pnb(),
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
        pub const fn ecccie(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "ECC single correction error interrupt enable bit When ECCCIE bit is set to 1, an interrupt is generated when an ECC single correction error occurs during a read operation."]
        #[inline(always)]
        pub const fn set_ecccie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
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
                "Ecccorr {{ addr_ecc: {=u16:?}, data_ecc: {=bool:?}, edata_ecc: {=bool:?}, bk_ecc: {=bool:?}, sysf_ecc: {=bool:?}, otp_ecc: {=bool:?}, ecccie: {=bool:?}, eccc: {=bool:?} }}",
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
    pub struct Hdp1r(pub u32);
    impl Hdp1r {
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
    impl Default for Hdp1r {
        #[inline(always)]
        fn default() -> Hdp1r {
            Hdp1r(0)
        }
    }
    impl core::fmt::Debug for Hdp1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hdp1r")
                .field("hdp1_strt", &self.hdp1_strt())
                .field("hdp1_end", &self.hdp1_end())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hdp1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hdp1r {{ hdp1_strt: {=u8:?}, hdp1_end: {=u8:?} }}",
                self.hdp1_strt(),
                self.hdp1_end()
            )
        }
    }
    #[doc = "FLASH HDP bank2 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hdp2r(pub u32);
    impl Hdp2r {
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
    impl Default for Hdp2r {
        #[inline(always)]
        fn default() -> Hdp2r {
            Hdp2r(0)
        }
    }
    impl core::fmt::Debug for Hdp2r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hdp2r")
                .field("hdp2_strt", &self.hdp2_strt())
                .field("hdp2_end", &self.hdp2_end())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hdp2r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hdp2r {{ hdp2_strt: {=u8:?}, hdp2_end: {=u8:?} }}",
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
        pub const fn optlock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "OPTCR lock option configuration bit."]
        #[inline(always)]
        pub const fn set_optlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
                "Optcr {{ optlock: {=bool:?}, optstrt: {=bool:?}, swap_bank: {:?} }}",
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
        #[doc = "Boot 0 source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn boot_sel(&self) -> super::vals::OptsrBootSel {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::OptsrBootSel::from_bits(val as u8)
        }
        #[doc = "Boot 0 source selection."]
        #[inline(always)]
        pub const fn set_boot_sel(&mut self, val: super::vals::OptsrBootSel) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "Boot 0 option bit."]
        #[must_use]
        #[inline(always)]
        pub const fn boot0(&self) -> super::vals::OptsrBoot0 {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::OptsrBoot0::from_bits(val as u8)
        }
        #[doc = "Boot 0 option bit."]
        #[inline(always)]
        pub const fn set_boot0(&mut self, val: super::vals::OptsrBoot0) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "Flash data area enable."]
        #[must_use]
        #[inline(always)]
        pub const fn edata_en(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Flash data area enable."]
        #[inline(always)]
        pub const fn set_edata_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Dual bank selection option status bit."]
        #[must_use]
        #[inline(always)]
        pub const fn dual_bank(&self) -> super::vals::OptsrDualBank {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::OptsrDualBank::from_bits(val as u8)
        }
        #[doc = "Dual bank selection option status bit."]
        #[inline(always)]
        pub const fn set_dual_bank(&mut self, val: super::vals::OptsrDualBank) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Dual bank selection option status bit."]
        #[must_use]
        #[inline(always)]
        pub const fn single_bank(&self) -> super::vals::OptsrSingleBank {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::OptsrSingleBank::from_bits(val as u8)
        }
        #[doc = "Dual bank selection option status bit."]
        #[inline(always)]
        pub const fn set_single_bank(&mut self, val: super::vals::OptsrSingleBank) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Bank swapping option status bit."]
        #[must_use]
        #[inline(always)]
        pub const fn swap_bank(&self) -> super::vals::OptsrSwapBank {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::OptsrSwapBank::from_bits(val as u8)
        }
        #[doc = "Bank swapping option status bit."]
        #[inline(always)]
        pub const fn set_swap_bank(&mut self, val: super::vals::OptsrSwapBank) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Optsr {
        #[inline(always)]
        fn default() -> Optsr {
            Optsr(0)
        }
    }
    impl core::fmt::Debug for Optsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Optsr")
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
    impl defmt::Format for Optsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Optsr {{ iwdg_sw: {:?}, wwdg_sw: {:?}, nrst_stop: {:?}, nrst_stdby: {:?}, rdp_level: {=u8:?}, iwdg_stop: {:?}, iwdg_stdby: {:?}, boot_sel: {:?}, boot0: {:?}, edata_en: {=bool:?}, dual_bank: {:?}, single_bank: {:?}, swap_bank: {:?} }}",
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
    #[doc = "FLASH option status register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optsr2(pub u32);
    impl Optsr2 {
        #[doc = "SRAM1 erase upon system reset."]
        #[must_use]
        #[inline(always)]
        pub const fn sram1_rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 erase upon system reset."]
        #[inline(always)]
        pub const fn set_sram1_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SRAM2 erase when system reset."]
        #[must_use]
        #[inline(always)]
        pub const fn sram2_rst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 erase when system reset."]
        #[inline(always)]
        pub const fn set_sram2_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "SRAM2 ECC detection and correction disable."]
        #[must_use]
        #[inline(always)]
        pub const fn sram2_ecc(&self) -> super::vals::Optsr2Sram2Ecc {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Optsr2Sram2Ecc::from_bits(val as u8)
        }
        #[doc = "SRAM2 ECC detection and correction disable."]
        #[inline(always)]
        pub const fn set_sram2_ecc(&mut self, val: super::vals::Optsr2Sram2Ecc) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Optsr2 {
        #[inline(always)]
        fn default() -> Optsr2 {
            Optsr2(0)
        }
    }
    impl core::fmt::Debug for Optsr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Optsr2")
                .field("sram1_rst", &self.sram1_rst())
                .field("sram2_rst", &self.sram2_rst())
                .field("sram2_ecc", &self.sram2_ecc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Optsr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Optsr2 {{ sram1_rst: {=bool:?}, sram2_rst: {=bool:?}, sram2_ecc: {:?} }}",
                self.sram1_rst(),
                self.sram2_rst(),
                self.sram2_ecc()
            )
        }
    }
    #[doc = "FLASH OTP block lock."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Otpblr(pub u32);
    impl Otpblr {
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
    impl Default for Otpblr {
        #[inline(always)]
        fn default() -> Otpblr {
            Otpblr(0)
        }
    }
    impl core::fmt::Debug for Otpblr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Otpblr").field("lockbl", &self.lockbl()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Otpblr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Otpblr {{ lockbl: {=u32:?} }}", self.lockbl())
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
        pub const fn bsy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "busy flag."]
        #[inline(always)]
        pub const fn set_bsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
        pub const fn dbne(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "data buffer not empty flag."]
        #[inline(always)]
        pub const fn set_dbne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
        pub const fn eop(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "end of operation flag."]
        #[inline(always)]
        pub const fn set_eop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "write protection error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn wrperr(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "write protection error flag."]
        #[inline(always)]
        pub const fn set_wrperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "programming sequence error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn pgserr(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "programming sequence error flag."]
        #[inline(always)]
        pub const fn set_pgserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "strobe error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn strberr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "strobe error flag."]
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
        #[doc = "Option-byte change error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn optchangeerr(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Option-byte change error flag."]
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
                "Sr {{ bsy: {=bool:?}, wbne: {:?}, dbne: {=bool:?}, oemlock: {=bool:?}, bslock: {=bool:?}, eop: {=bool:?}, wrperr: {=bool:?}, pgserr: {=bool:?}, strberr: {=bool:?}, incerr: {=bool:?}, optchangeerr: {=bool:?} }}",
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
    pub struct Wrp1r(pub u32);
    impl Wrp1r {
        #[doc = "Bank1 page protection option status byte."]
        #[must_use]
        #[inline(always)]
        pub const fn wrpsg1(&self) -> super::vals::Wrp1rWrpsg1 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            super::vals::Wrp1rWrpsg1::from_bits(val as u32)
        }
        #[doc = "Bank1 page protection option status byte."]
        #[inline(always)]
        pub const fn set_wrpsg1(&mut self, val: super::vals::Wrp1rWrpsg1) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Wrp1r {
        #[inline(always)]
        fn default() -> Wrp1r {
            Wrp1r(0)
        }
    }
    impl core::fmt::Debug for Wrp1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wrp1r").field("wrpsg1", &self.wrpsg1()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrp1r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wrp1r {{ wrpsg1: {:?} }}", self.wrpsg1())
        }
    }
    #[doc = "FLASH write page protection for bank2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrp2r(pub u32);
    impl Wrp2r {
        #[doc = "Bank2 page protection option status byte."]
        #[must_use]
        #[inline(always)]
        pub const fn wrpsg2(&self) -> super::vals::Wrp2rWrpsg2 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            super::vals::Wrp2rWrpsg2::from_bits(val as u32)
        }
        #[doc = "Bank2 page protection option status byte."]
        #[inline(always)]
        pub const fn set_wrpsg2(&mut self, val: super::vals::Wrp2rWrpsg2) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Wrp2r {
        #[inline(always)]
        fn default() -> Wrp2r {
            Wrp2r(0)
        }
    }
    impl core::fmt::Debug for Wrp2r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Wrp2r").field("wrpsg2", &self.wrpsg2()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrp2r {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Wrp2r {{ wrpsg2: {:?} }}", self.wrpsg2())
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Bksel {
        #[doc = "Bank1 is selected for bank erase (BER)/page erase (PER)/interrupt enable."]
        Bank1 = 0x0,
        #[doc = "Bank2 is selected for BER/PER."]
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
        #[doc = "The BOOT0, BOOT_SEL, SWAP_BANK and BOOTADD are frozen."]
        pub const Locked: Self = Self(0xb4);
        #[doc = "The BOOT0, BOOT_SEL, SWAP_BANK and BOOTADD can still be modified following their individual rules."]
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
    pub enum OptcrSwapBank {
        #[doc = "Bank1 and bank2 not swapped."]
        NotSwapped = 0x0,
        #[doc = "Bank1 and bank2 swapped."]
        Swapped = 0x01,
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
    pub enum Optsr2Sram2Ecc {
        #[doc = "SRAM2 ECC check enabled."]
        Enabled = 0x0,
        #[doc = "SRAM2 ECC check disabled."]
        Disabled = 0x01,
    }
    impl Optsr2Sram2Ecc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Optsr2Sram2Ecc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Optsr2Sram2Ecc {
        #[inline(always)]
        fn from(val: u8) -> Optsr2Sram2Ecc {
            Optsr2Sram2Ecc::from_bits(val)
        }
    }
    impl From<Optsr2Sram2Ecc> for u8 {
        #[inline(always)]
        fn from(val: Optsr2Sram2Ecc) -> u8 {
            Optsr2Sram2Ecc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrBoot0 {
        #[doc = "BOOT0 = 0."]
        B0x0 = 0x0,
        #[doc = "BOOT0 = 1."]
        B0x1 = 0x01,
    }
    impl OptsrBoot0 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrBoot0 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrBoot0 {
        #[inline(always)]
        fn from(val: u8) -> OptsrBoot0 {
            OptsrBoot0::from_bits(val)
        }
    }
    impl From<OptsrBoot0> for u8 {
        #[inline(always)]
        fn from(val: OptsrBoot0) -> u8 {
            OptsrBoot0::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrBootSel {
        #[doc = "BOOT0 signal is defined by the BOOT0 option bit."]
        OptionBit = 0x0,
        #[doc = "BOOT0 signal is defined by BOOT0 pin value (legacy mode)."]
        Pin = 0x01,
    }
    impl OptsrBootSel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrBootSel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrBootSel {
        #[inline(always)]
        fn from(val: u8) -> OptsrBootSel {
            OptsrBootSel::from_bits(val)
        }
    }
    impl From<OptsrBootSel> for u8 {
        #[inline(always)]
        fn from(val: OptsrBootSel) -> u8 {
            OptsrBootSel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrDualBank {
        #[doc = "x Kbytes of user flash split with x/2 Kbytes in Bank 1 and x/2 Kbytes in Bank 2."]
        DualBank = 0x0,
        #[doc = "x Kbytes of user flash located in one bank."]
        SingleBank = 0x01,
    }
    impl OptsrDualBank {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrDualBank {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrDualBank {
        #[inline(always)]
        fn from(val: u8) -> OptsrDualBank {
            OptsrDualBank::from_bits(val)
        }
    }
    impl From<OptsrDualBank> for u8 {
        #[inline(always)]
        fn from(val: OptsrDualBank) -> u8 {
            OptsrDualBank::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrIwdgStdby {
        #[doc = "Independent watchdog frozen in Standby mode."]
        B0x0 = 0x0,
        #[doc = "Independent watchdog keep running in Standby mode."]
        B0x1 = 0x01,
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrIwdgStop {
        #[doc = "Independent watchdog frozen in system Stop mode."]
        B0x0 = 0x0,
        #[doc = "Independent watchdog keep running in system Stop mode."]
        B0x1 = 0x01,
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrIwdgSw {
        #[doc = "IWDG watchdog is controlled by hardware."]
        B0x0 = 0x0,
        #[doc = "IWDG watchdog is controlled by software."]
        B0x1 = 0x01,
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrNrstStdby {
        #[doc = "a reset is generated when entering Standby mode on core domain."]
        B0x0 = 0x0,
        #[doc = "no reset generated when entering Standby mode on core domain."]
        B0x1 = 0x01,
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
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrNrstStop {
        #[doc = "a reset is generated when entering Stop mode on core domain."]
        B0x0 = 0x0,
        #[doc = "no reset generated when entering Stop mode on core domain."]
        B0x1 = 0x01,
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
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrSingleBank {
        #[doc = "128 Kbytes of user flash split with 64 Kbytes in Bank 1 and 64 Kbytes in Bank 2."]
        B0x0 = 0x0,
        #[doc = "128 Kbytes of user flash located in one bank."]
        B0x1 = 0x01,
    }
    impl OptsrSingleBank {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrSingleBank {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrSingleBank {
        #[inline(always)]
        fn from(val: u8) -> OptsrSingleBank {
            OptsrSingleBank::from_bits(val)
        }
    }
    impl From<OptsrSingleBank> for u8 {
        #[inline(always)]
        fn from(val: OptsrSingleBank) -> u8 {
            OptsrSingleBank::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrSwapBank {
        #[doc = "Bank1 and bank2 not swapped."]
        B0x0 = 0x0,
        #[doc = "Bank1 and bank2 swapped."]
        B0x1 = 0x01,
    }
    impl OptsrSwapBank {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrSwapBank {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrSwapBank {
        #[inline(always)]
        fn from(val: u8) -> OptsrSwapBank {
            OptsrSwapBank::from_bits(val)
        }
    }
    impl From<OptsrSwapBank> for u8 {
        #[inline(always)]
        fn from(val: OptsrSwapBank) -> u8 {
            OptsrSwapBank::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum OptsrWwdgSw {
        #[doc = "WWDG watchdog is controlled by hardware."]
        B0x0 = 0x0,
        #[doc = "WWDG watchdog is controlled by software."]
        B0x1 = 0x01,
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
    pub struct Wrp1rWrpsg1(u32);
    impl Wrp1rWrpsg1 {
        #[doc = "write protected;."]
        pub const B0x0: Self = Self(0x0);
        #[doc = "not write protected."]
        pub const B0x1: Self = Self(0x01);
    }
    impl Wrp1rWrpsg1 {
        pub const fn from_bits(val: u32) -> Wrp1rWrpsg1 {
            Self(val & 0xffff_ffff)
        }
        pub const fn to_bits(self) -> u32 {
            self.0
        }
    }
    impl core::fmt::Debug for Wrp1rWrpsg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("B0x0"),
                0x01 => f.write_str("B0x1"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrp1rWrpsg1 {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "B0x0"),
                0x01 => defmt::write!(f, "B0x1"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u32> for Wrp1rWrpsg1 {
        #[inline(always)]
        fn from(val: u32) -> Wrp1rWrpsg1 {
            Wrp1rWrpsg1::from_bits(val)
        }
    }
    impl From<Wrp1rWrpsg1> for u32 {
        #[inline(always)]
        fn from(val: Wrp1rWrpsg1) -> u32 {
            Wrp1rWrpsg1::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Wrp2rWrpsg2(u32);
    impl Wrp2rWrpsg2 {
        #[doc = "write protected."]
        pub const B0x0: Self = Self(0x0);
        #[doc = "not write protected."]
        pub const B0x1: Self = Self(0x01);
    }
    impl Wrp2rWrpsg2 {
        pub const fn from_bits(val: u32) -> Wrp2rWrpsg2 {
            Self(val & 0xffff_ffff)
        }
        pub const fn to_bits(self) -> u32 {
            self.0
        }
    }
    impl core::fmt::Debug for Wrp2rWrpsg2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            match self.0 {
                0x0 => f.write_str("B0x0"),
                0x01 => f.write_str("B0x1"),
                other => core::write!(f, "0x{:02X}", other),
            }
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Wrp2rWrpsg2 {
        fn format(&self, f: defmt::Formatter) {
            match self.0 {
                0x0 => defmt::write!(f, "B0x0"),
                0x01 => defmt::write!(f, "B0x1"),
                other => defmt::write!(f, "0x{:02X}", other),
            }
        }
    }
    impl From<u32> for Wrp2rWrpsg2 {
        #[inline(always)]
        fn from(val: u32) -> Wrp2rWrpsg2 {
            Wrp2rWrpsg2::from_bits(val)
        }
    }
    impl From<Wrp2rWrpsg2> for u32 {
        #[inline(always)]
        fn from(val: Wrp2rWrpsg2) -> u32 {
            Wrp2rWrpsg2::to_bits(val)
        }
    }
}
