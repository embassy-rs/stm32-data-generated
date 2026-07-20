#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Memory cipher engine."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mce {
    ptr: *mut u8,
}
unsafe impl Send for Mce {}
unsafe impl Sync for Mce {}
impl Mce {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "MCE configuration register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "MCE status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "MCE illegal access status register."]
    #[inline(always)]
    pub const fn iasr(self) -> crate::common::Reg<regs::Iasr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "MCE illegal access clear register."]
    #[inline(always)]
    pub const fn iacr(self) -> crate::common::Reg<regs::Iacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "MCE illegal access interrupt enable register."]
    #[inline(always)]
    pub const fn iaier(self) -> crate::common::Reg<regs::Iaier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "MCE illegal address register."]
    #[inline(always)]
    pub const fn iaddr(self) -> crate::common::Reg<regs::Iaddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "MCE region 1 configuration register."]
    #[inline(always)]
    pub const fn regcr(self, n: usize) -> crate::common::Reg<regs::Regcr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 16usize) as _) }
    }
    #[doc = "MCE start address for region 1 register."]
    #[inline(always)]
    pub const fn saddr(self, n: usize) -> crate::common::Reg<regs::Saddr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize + n * 16usize) as _) }
    }
    #[doc = "MCE end address for region 1 register."]
    #[inline(always)]
    pub const fn eaddr(self, n: usize) -> crate::common::Reg<regs::Eaddr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize + n * 16usize) as _) }
    }
    #[doc = ".MCE master key 0."]
    #[inline(always)]
    pub const fn mkeyr(self, n: usize) -> crate::common::Reg<regs::Mkeyr, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize + n * 4usize) as _) }
    }
    #[doc = "MCE fast master key 0."]
    #[inline(always)]
    pub const fn fmkeyr(self, n: usize) -> crate::common::Reg<regs::Fmkeyr, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize + n * 4usize) as _) }
    }
    #[doc = "MCE cipher context 1 configuration register."]
    #[inline(always)]
    pub const fn cc1cfgr(self) -> crate::common::Reg<regs::Cc1cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "MCE cipher context 1 nonce register 0."]
    #[inline(always)]
    pub const fn cc1nr(self, n: usize) -> crate::common::Reg<regs::Cc1nr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize + n * 4usize) as _) }
    }
    #[doc = "MCE cipher context 1 key register 0."]
    #[inline(always)]
    pub const fn cc1keyr(self, n: usize) -> crate::common::Reg<regs::Cc1keyr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x024cusize + n * 4usize) as _) }
    }
    #[doc = "MCE cipher context 2 configuration register."]
    #[inline(always)]
    pub const fn cc2cfgr(self) -> crate::common::Reg<regs::Cc2cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0270usize) as _) }
    }
    #[doc = "MCE cipher context 2 nonce register 0."]
    #[inline(always)]
    pub const fn cc2nr(self, n: usize) -> crate::common::Reg<regs::Cc2nr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0274usize + n * 4usize) as _) }
    }
    #[doc = "MCE cipher context 2 key register 0."]
    #[inline(always)]
    pub const fn cc2keyr(self, n: usize) -> crate::common::Reg<regs::Cc2keyr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x027cusize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "MCE cipher context 1 configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cc1cfgr(pub u32);
    impl Cc1cfgr {
        #[doc = "Cipher context enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ccen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Cipher context enable."]
        #[inline(always)]
        pub const fn set_ccen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Cipher context lock."]
        #[must_use]
        #[inline(always)]
        pub const fn cclock(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Cipher context lock."]
        #[inline(always)]
        pub const fn set_cclock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Key lock."]
        #[must_use]
        #[inline(always)]
        pub const fn keylock(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Key lock."]
        #[inline(always)]
        pub const fn set_keylock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Authorized cipher mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Authorized cipher mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Key CRC."]
        #[must_use]
        #[inline(always)]
        pub const fn keycrc(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Key CRC."]
        #[inline(always)]
        pub const fn set_keycrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Version."]
        #[must_use]
        #[inline(always)]
        pub const fn version(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Version."]
        #[inline(always)]
        pub const fn set_version(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Cc1cfgr {
        #[inline(always)]
        fn default() -> Cc1cfgr {
            Cc1cfgr(0)
        }
    }
    impl core::fmt::Debug for Cc1cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cc1cfgr")
                .field("ccen", &self.ccen())
                .field("cclock", &self.cclock())
                .field("keylock", &self.keylock())
                .field("mode", &self.mode())
                .field("keycrc", &self.keycrc())
                .field("version", &self.version())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cc1cfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cc1cfgr {{ ccen: {=bool:?}, cclock: {=bool:?}, keylock: {=bool:?}, mode: {=u8:?}, keycrc: {=u8:?}, version: {=u16:?} }}",
                self.ccen(),
                self.cclock(),
                self.keylock(),
                self.mode(),
                self.keycrc(),
                self.version()
            )
        }
    }
    #[doc = "MCE cipher context 1 key register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cc1keyr(pub u32);
    impl Cc1keyr {
        #[doc = "cipher key, bits \\[31:0\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "cipher key, bits \\[31:0\\]."]
        #[inline(always)]
        pub const fn set_key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cc1keyr {
        #[inline(always)]
        fn default() -> Cc1keyr {
            Cc1keyr(0)
        }
    }
    impl core::fmt::Debug for Cc1keyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cc1keyr").field("key", &self.key()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cc1keyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cc1keyr {{ key: {=u32:?} }}", self.key())
        }
    }
    #[doc = "MCE cipher context 1 nonce register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cc1nr(pub u32);
    impl Cc1nr {
        #[doc = "Stream cipher nonce, bits \\[31:0\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn scnonce(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Stream cipher nonce, bits \\[31:0\\]."]
        #[inline(always)]
        pub const fn set_scnonce(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cc1nr {
        #[inline(always)]
        fn default() -> Cc1nr {
            Cc1nr(0)
        }
    }
    impl core::fmt::Debug for Cc1nr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cc1nr").field("scnonce", &self.scnonce()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cc1nr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cc1nr {{ scnonce: {=u32:?} }}", self.scnonce())
        }
    }
    #[doc = "MCE cipher context 2 configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cc2cfgr(pub u32);
    impl Cc2cfgr {
        #[doc = "Cipher context enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ccen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Cipher context enable."]
        #[inline(always)]
        pub const fn set_ccen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Cipher context lock."]
        #[must_use]
        #[inline(always)]
        pub const fn cclock(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Cipher context lock."]
        #[inline(always)]
        pub const fn set_cclock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Key lock."]
        #[must_use]
        #[inline(always)]
        pub const fn keylock(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Key lock."]
        #[inline(always)]
        pub const fn set_keylock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Authorized cipher mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Authorized cipher mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Key CRC."]
        #[must_use]
        #[inline(always)]
        pub const fn keycrc(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Key CRC."]
        #[inline(always)]
        pub const fn set_keycrc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Version."]
        #[must_use]
        #[inline(always)]
        pub const fn version(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Version."]
        #[inline(always)]
        pub const fn set_version(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Cc2cfgr {
        #[inline(always)]
        fn default() -> Cc2cfgr {
            Cc2cfgr(0)
        }
    }
    impl core::fmt::Debug for Cc2cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cc2cfgr")
                .field("ccen", &self.ccen())
                .field("cclock", &self.cclock())
                .field("keylock", &self.keylock())
                .field("mode", &self.mode())
                .field("keycrc", &self.keycrc())
                .field("version", &self.version())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cc2cfgr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cc2cfgr {{ ccen: {=bool:?}, cclock: {=bool:?}, keylock: {=bool:?}, mode: {=u8:?}, keycrc: {=u8:?}, version: {=u16:?} }}",
                self.ccen(),
                self.cclock(),
                self.keylock(),
                self.mode(),
                self.keycrc(),
                self.version()
            )
        }
    }
    #[doc = "MCE cipher context 2 key register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cc2keyr(pub u32);
    impl Cc2keyr {
        #[doc = "cipher key, bits \\[31:0\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "cipher key, bits \\[31:0\\]."]
        #[inline(always)]
        pub const fn set_key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cc2keyr {
        #[inline(always)]
        fn default() -> Cc2keyr {
            Cc2keyr(0)
        }
    }
    impl core::fmt::Debug for Cc2keyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cc2keyr").field("key", &self.key()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cc2keyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cc2keyr {{ key: {=u32:?} }}", self.key())
        }
    }
    #[doc = "MCE cipher context 2 nonce register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cc2nr(pub u32);
    impl Cc2nr {
        #[doc = "Stream cipher nonce, bits \\[31:0\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn scnonce(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Stream cipher nonce, bits \\[31:0\\]."]
        #[inline(always)]
        pub const fn set_scnonce(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cc2nr {
        #[inline(always)]
        fn default() -> Cc2nr {
            Cc2nr(0)
        }
    }
    impl core::fmt::Debug for Cc2nr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cc2nr").field("scnonce", &self.scnonce()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cc2nr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cc2nr {{ scnonce: {=u32:?} }}", self.scnonce())
        }
    }
    #[doc = "MCE configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Global lock."]
        #[must_use]
        #[inline(always)]
        pub const fn glock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Global lock."]
        #[inline(always)]
        pub const fn set_glock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Master keys lock."]
        #[must_use]
        #[inline(always)]
        pub const fn mklock(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Master keys lock."]
        #[inline(always)]
        pub const fn set_mklock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Cipher selection."]
        #[must_use]
        #[inline(always)]
        pub const fn ciphersel(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Cipher selection."]
        #[inline(always)]
        pub const fn set_ciphersel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
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
                .field("glock", &self.glock())
                .field("mklock", &self.mklock())
                .field("ciphersel", &self.ciphersel())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ glock: {=bool:?}, mklock: {=bool:?}, ciphersel: {=u8:?} }}",
                self.glock(),
                self.mklock(),
                self.ciphersel()
            )
        }
    }
    #[doc = "MCE end address for region 1 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eaddr(pub u32);
    impl Eaddr {
        #[doc = "Region address end."]
        #[must_use]
        #[inline(always)]
        pub const fn baddend(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Region address end."]
        #[inline(always)]
        pub const fn set_baddend(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for Eaddr {
        #[inline(always)]
        fn default() -> Eaddr {
            Eaddr(0)
        }
    }
    impl core::fmt::Debug for Eaddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Eaddr").field("baddend", &self.baddend()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Eaddr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Eaddr {{ baddend: {=u32:?} }}", self.baddend())
        }
    }
    #[doc = "MCE fast master key 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fmkeyr(pub u32);
    impl Fmkeyr {
        #[doc = "Fast master key bit 0 (i = 31 to 0)."]
        #[must_use]
        #[inline(always)]
        pub const fn fmkey(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Fast master key bit 0 (i = 31 to 0)."]
        #[inline(always)]
        pub const fn set_fmkey(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Fmkeyr {
        #[inline(always)]
        fn default() -> Fmkeyr {
            Fmkeyr(0)
        }
    }
    impl core::fmt::Debug for Fmkeyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fmkeyr")
                .field("fmkey[0]", &self.fmkey(0usize))
                .field("fmkey[1]", &self.fmkey(1usize))
                .field("fmkey[2]", &self.fmkey(2usize))
                .field("fmkey[3]", &self.fmkey(3usize))
                .field("fmkey[4]", &self.fmkey(4usize))
                .field("fmkey[5]", &self.fmkey(5usize))
                .field("fmkey[6]", &self.fmkey(6usize))
                .field("fmkey[7]", &self.fmkey(7usize))
                .field("fmkey[8]", &self.fmkey(8usize))
                .field("fmkey[9]", &self.fmkey(9usize))
                .field("fmkey[10]", &self.fmkey(10usize))
                .field("fmkey[11]", &self.fmkey(11usize))
                .field("fmkey[12]", &self.fmkey(12usize))
                .field("fmkey[13]", &self.fmkey(13usize))
                .field("fmkey[14]", &self.fmkey(14usize))
                .field("fmkey[15]", &self.fmkey(15usize))
                .field("fmkey[16]", &self.fmkey(16usize))
                .field("fmkey[17]", &self.fmkey(17usize))
                .field("fmkey[18]", &self.fmkey(18usize))
                .field("fmkey[19]", &self.fmkey(19usize))
                .field("fmkey[20]", &self.fmkey(20usize))
                .field("fmkey[21]", &self.fmkey(21usize))
                .field("fmkey[22]", &self.fmkey(22usize))
                .field("fmkey[23]", &self.fmkey(23usize))
                .field("fmkey[24]", &self.fmkey(24usize))
                .field("fmkey[25]", &self.fmkey(25usize))
                .field("fmkey[26]", &self.fmkey(26usize))
                .field("fmkey[27]", &self.fmkey(27usize))
                .field("fmkey[28]", &self.fmkey(28usize))
                .field("fmkey[29]", &self.fmkey(29usize))
                .field("fmkey[30]", &self.fmkey(30usize))
                .field("fmkey[31]", &self.fmkey(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fmkeyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Fmkeyr {{ fmkey[0]: {=bool:?}, fmkey[1]: {=bool:?}, fmkey[2]: {=bool:?}, fmkey[3]: {=bool:?}, fmkey[4]: {=bool:?}, fmkey[5]: {=bool:?}, fmkey[6]: {=bool:?}, fmkey[7]: {=bool:?}, fmkey[8]: {=bool:?}, fmkey[9]: {=bool:?}, fmkey[10]: {=bool:?}, fmkey[11]: {=bool:?}, fmkey[12]: {=bool:?}, fmkey[13]: {=bool:?}, fmkey[14]: {=bool:?}, fmkey[15]: {=bool:?}, fmkey[16]: {=bool:?}, fmkey[17]: {=bool:?}, fmkey[18]: {=bool:?}, fmkey[19]: {=bool:?}, fmkey[20]: {=bool:?}, fmkey[21]: {=bool:?}, fmkey[22]: {=bool:?}, fmkey[23]: {=bool:?}, fmkey[24]: {=bool:?}, fmkey[25]: {=bool:?}, fmkey[26]: {=bool:?}, fmkey[27]: {=bool:?}, fmkey[28]: {=bool:?}, fmkey[29]: {=bool:?}, fmkey[30]: {=bool:?}, fmkey[31]: {=bool:?} }}",
                self.fmkey(0usize),
                self.fmkey(1usize),
                self.fmkey(2usize),
                self.fmkey(3usize),
                self.fmkey(4usize),
                self.fmkey(5usize),
                self.fmkey(6usize),
                self.fmkey(7usize),
                self.fmkey(8usize),
                self.fmkey(9usize),
                self.fmkey(10usize),
                self.fmkey(11usize),
                self.fmkey(12usize),
                self.fmkey(13usize),
                self.fmkey(14usize),
                self.fmkey(15usize),
                self.fmkey(16usize),
                self.fmkey(17usize),
                self.fmkey(18usize),
                self.fmkey(19usize),
                self.fmkey(20usize),
                self.fmkey(21usize),
                self.fmkey(22usize),
                self.fmkey(23usize),
                self.fmkey(24usize),
                self.fmkey(25usize),
                self.fmkey(26usize),
                self.fmkey(27usize),
                self.fmkey(28usize),
                self.fmkey(29usize),
                self.fmkey(30usize),
                self.fmkey(31usize)
            )
        }
    }
    #[doc = "MCE illegal access clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iacr(pub u32);
    impl Iacr {
        #[doc = "Illegal access error flag clear."]
        #[must_use]
        #[inline(always)]
        pub const fn iaef(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Illegal access error flag clear."]
        #[inline(always)]
        pub const fn set_iaef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Iacr {
        #[inline(always)]
        fn default() -> Iacr {
            Iacr(0)
        }
    }
    impl core::fmt::Debug for Iacr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iacr").field("iaef", &self.iaef()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Iacr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Iacr {{ iaef: {=bool:?} }}", self.iaef())
        }
    }
    #[doc = "MCE illegal address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iaddr(pub u32);
    impl Iaddr {
        #[doc = "Illegal address."]
        #[must_use]
        #[inline(always)]
        pub const fn iadd(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Illegal address."]
        #[inline(always)]
        pub const fn set_iadd(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iaddr {
        #[inline(always)]
        fn default() -> Iaddr {
            Iaddr(0)
        }
    }
    impl core::fmt::Debug for Iaddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iaddr").field("iadd", &self.iadd()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Iaddr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Iaddr {{ iadd: {=u32:?} }}", self.iadd())
        }
    }
    #[doc = "MCE illegal access interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iaier(pub u32);
    impl Iaier {
        #[doc = "Illegal access error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn iaeie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Illegal access error interrupt enable."]
        #[inline(always)]
        pub const fn set_iaeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Iaier {
        #[inline(always)]
        fn default() -> Iaier {
            Iaier(0)
        }
    }
    impl core::fmt::Debug for Iaier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iaier").field("iaeie", &self.iaeie()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Iaier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Iaier {{ iaeie: {=bool:?} }}", self.iaeie())
        }
    }
    #[doc = "MCE illegal access status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iasr(pub u32);
    impl Iasr {
        #[doc = "Illegal access error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn iaef(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Illegal access error flag."]
        #[inline(always)]
        pub const fn set_iaef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Iasr {
        #[inline(always)]
        fn default() -> Iasr {
            Iasr(0)
        }
    }
    impl core::fmt::Debug for Iasr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iasr").field("iaef", &self.iaef()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Iasr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Iasr {{ iaef: {=bool:?} }}", self.iaef())
        }
    }
    #[doc = ".MCE master key 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mkeyr(pub u32);
    impl Mkeyr {
        #[doc = "Master key bit 0 (i = 31 to 0)."]
        #[must_use]
        #[inline(always)]
        pub const fn mkey(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Master key bit 0 (i = 31 to 0)."]
        #[inline(always)]
        pub const fn set_mkey(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Mkeyr {
        #[inline(always)]
        fn default() -> Mkeyr {
            Mkeyr(0)
        }
    }
    impl core::fmt::Debug for Mkeyr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mkeyr")
                .field("mkey[0]", &self.mkey(0usize))
                .field("mkey[1]", &self.mkey(1usize))
                .field("mkey[2]", &self.mkey(2usize))
                .field("mkey[3]", &self.mkey(3usize))
                .field("mkey[4]", &self.mkey(4usize))
                .field("mkey[5]", &self.mkey(5usize))
                .field("mkey[6]", &self.mkey(6usize))
                .field("mkey[7]", &self.mkey(7usize))
                .field("mkey[8]", &self.mkey(8usize))
                .field("mkey[9]", &self.mkey(9usize))
                .field("mkey[10]", &self.mkey(10usize))
                .field("mkey[11]", &self.mkey(11usize))
                .field("mkey[12]", &self.mkey(12usize))
                .field("mkey[13]", &self.mkey(13usize))
                .field("mkey[14]", &self.mkey(14usize))
                .field("mkey[15]", &self.mkey(15usize))
                .field("mkey[16]", &self.mkey(16usize))
                .field("mkey[17]", &self.mkey(17usize))
                .field("mkey[18]", &self.mkey(18usize))
                .field("mkey[19]", &self.mkey(19usize))
                .field("mkey[20]", &self.mkey(20usize))
                .field("mkey[21]", &self.mkey(21usize))
                .field("mkey[22]", &self.mkey(22usize))
                .field("mkey[23]", &self.mkey(23usize))
                .field("mkey[24]", &self.mkey(24usize))
                .field("mkey[25]", &self.mkey(25usize))
                .field("mkey[26]", &self.mkey(26usize))
                .field("mkey[27]", &self.mkey(27usize))
                .field("mkey[28]", &self.mkey(28usize))
                .field("mkey[29]", &self.mkey(29usize))
                .field("mkey[30]", &self.mkey(30usize))
                .field("mkey[31]", &self.mkey(31usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mkeyr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Mkeyr {{ mkey[0]: {=bool:?}, mkey[1]: {=bool:?}, mkey[2]: {=bool:?}, mkey[3]: {=bool:?}, mkey[4]: {=bool:?}, mkey[5]: {=bool:?}, mkey[6]: {=bool:?}, mkey[7]: {=bool:?}, mkey[8]: {=bool:?}, mkey[9]: {=bool:?}, mkey[10]: {=bool:?}, mkey[11]: {=bool:?}, mkey[12]: {=bool:?}, mkey[13]: {=bool:?}, mkey[14]: {=bool:?}, mkey[15]: {=bool:?}, mkey[16]: {=bool:?}, mkey[17]: {=bool:?}, mkey[18]: {=bool:?}, mkey[19]: {=bool:?}, mkey[20]: {=bool:?}, mkey[21]: {=bool:?}, mkey[22]: {=bool:?}, mkey[23]: {=bool:?}, mkey[24]: {=bool:?}, mkey[25]: {=bool:?}, mkey[26]: {=bool:?}, mkey[27]: {=bool:?}, mkey[28]: {=bool:?}, mkey[29]: {=bool:?}, mkey[30]: {=bool:?}, mkey[31]: {=bool:?} }}",
                self.mkey(0usize),
                self.mkey(1usize),
                self.mkey(2usize),
                self.mkey(3usize),
                self.mkey(4usize),
                self.mkey(5usize),
                self.mkey(6usize),
                self.mkey(7usize),
                self.mkey(8usize),
                self.mkey(9usize),
                self.mkey(10usize),
                self.mkey(11usize),
                self.mkey(12usize),
                self.mkey(13usize),
                self.mkey(14usize),
                self.mkey(15usize),
                self.mkey(16usize),
                self.mkey(17usize),
                self.mkey(18usize),
                self.mkey(19usize),
                self.mkey(20usize),
                self.mkey(21usize),
                self.mkey(22usize),
                self.mkey(23usize),
                self.mkey(24usize),
                self.mkey(25usize),
                self.mkey(26usize),
                self.mkey(27usize),
                self.mkey(28usize),
                self.mkey(29usize),
                self.mkey(30usize),
                self.mkey(31usize)
            )
        }
    }
    #[doc = "MCE region 1 configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Regcr(pub u32);
    impl Regcr {
        #[doc = "Base region enable."]
        #[must_use]
        #[inline(always)]
        pub const fn bren(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Base region enable."]
        #[inline(always)]
        pub const fn set_bren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Context ID."]
        #[must_use]
        #[inline(always)]
        pub const fn ctxid(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[doc = "Context ID."]
        #[inline(always)]
        pub const fn set_ctxid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
        }
        #[doc = "Encrypted region."]
        #[must_use]
        #[inline(always)]
        pub const fn enc(&self) -> u8 {
            let val = (self.0 >> 14usize) & 0x03;
            val as u8
        }
        #[doc = "Encrypted region."]
        #[inline(always)]
        pub const fn set_enc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
        }
    }
    impl Default for Regcr {
        #[inline(always)]
        fn default() -> Regcr {
            Regcr(0)
        }
    }
    impl core::fmt::Debug for Regcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Regcr")
                .field("bren", &self.bren())
                .field("ctxid", &self.ctxid())
                .field("enc", &self.enc())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Regcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Regcr {{ bren: {=bool:?}, ctxid: {=u8:?}, enc: {=u8:?} }}",
                self.bren(),
                self.ctxid(),
                self.enc()
            )
        }
    }
    #[doc = "MCE start address for region 1 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Saddr(pub u32);
    impl Saddr {
        #[doc = "Region address start."]
        #[must_use]
        #[inline(always)]
        pub const fn baddstart(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Region address start."]
        #[inline(always)]
        pub const fn set_baddstart(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for Saddr {
        #[inline(always)]
        fn default() -> Saddr {
            Saddr(0)
        }
    }
    impl core::fmt::Debug for Saddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Saddr").field("baddstart", &self.baddstart()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Saddr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Saddr {{ baddstart: {=u32:?} }}", self.baddstart())
        }
    }
    #[doc = "MCE status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Master key valid."]
        #[must_use]
        #[inline(always)]
        pub const fn mkvalid(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Master key valid."]
        #[inline(always)]
        pub const fn set_mkvalid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Fast master key valid."]
        #[must_use]
        #[inline(always)]
        pub const fn fmkvalid(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Fast master key valid."]
        #[inline(always)]
        pub const fn set_fmkvalid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "encryption disabled."]
        #[must_use]
        #[inline(always)]
        pub const fn encdis(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "encryption disabled."]
        #[inline(always)]
        pub const fn set_encdis(&mut self, val: bool) {
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
                .field("mkvalid", &self.mkvalid())
                .field("fmkvalid", &self.fmkvalid())
                .field("encdis", &self.encdis())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ mkvalid: {=bool:?}, fmkvalid: {=bool:?}, encdis: {=bool:?} }}",
                self.mkvalid(),
                self.fmkvalid(),
                self.encdis()
            )
        }
    }
}
