#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Advanced encryption standard hardware accelerator"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aes {
    ptr: *mut u8,
}
unsafe impl Send for Aes {}
unsafe impl Sync for Aes {}
impl Aes {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Data input register"]
    #[inline(always)]
    pub const fn dinr(self) -> crate::common::Reg<regs::Dinr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Data output register"]
    #[inline(always)]
    pub const fn doutr(self) -> crate::common::Reg<regs::Doutr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Key register"]
    #[inline(always)]
    pub const fn keyr(self, n: usize) -> crate::common::Reg<regs::Keyr, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(
                self.ptr.add(
                    0x10usize + ([0usize, 4usize, 8usize, 12usize, 32usize, 36usize, 40usize, 44usize][n] as usize),
                ) as _,
            )
        }
    }
    #[doc = "Initialization vector register"]
    #[inline(always)]
    pub const fn ivr(self, n: usize) -> crate::common::Reg<regs::Ivr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize + n * 4usize) as _) }
    }
    #[doc = "Suspend register"]
    #[inline(always)]
    pub const fn suspr(self, n: usize) -> crate::common::Reg<regs::Suspr, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "Control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "AES enable"]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "AES enable"]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Data type selection"]
        #[inline(always)]
        pub const fn datatype(&self) -> super::vals::Datatype {
            let val = (self.0 >> 1usize) & 0x03;
            super::vals::Datatype::from_bits(val as u8)
        }
        #[doc = "Data type selection"]
        #[inline(always)]
        pub fn set_datatype(&mut self, val: super::vals::Datatype) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
        }
        #[doc = "Operating mode"]
        #[inline(always)]
        pub const fn mode(&self) -> super::vals::Mode {
            let val = (self.0 >> 3usize) & 0x03;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "Operating mode"]
        #[inline(always)]
        pub fn set_mode(&mut self, val: super::vals::Mode) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val.to_bits() as u32) & 0x03) << 3usize);
        }
        #[doc = "Chaining mode bit1 bit0"]
        #[inline(always)]
        pub const fn chmod10(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "Chaining mode bit1 bit0"]
        #[inline(always)]
        pub fn set_chmod10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "Computation Complete Flag Clear"]
        #[inline(always)]
        pub const fn ccfc(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Computation Complete Flag Clear"]
        #[inline(always)]
        pub fn set_ccfc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Error clear"]
        #[inline(always)]
        pub const fn errc(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Error clear"]
        #[inline(always)]
        pub fn set_errc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CCF flag interrupt enable"]
        #[inline(always)]
        pub const fn ccfie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "CCF flag interrupt enable"]
        #[inline(always)]
        pub fn set_ccfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Error interrupt enable"]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt enable"]
        #[inline(always)]
        pub fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Enable DMA management of data input phase"]
        #[inline(always)]
        pub const fn dmainen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Enable DMA management of data input phase"]
        #[inline(always)]
        pub fn set_dmainen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Enable DMA management of data output phase"]
        #[inline(always)]
        pub const fn dmaouten(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Enable DMA management of data output phase"]
        #[inline(always)]
        pub fn set_dmaouten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "GCM or CCM phase selection"]
        #[inline(always)]
        pub const fn gcmph(&self) -> super::vals::Gcmph {
            let val = (self.0 >> 13usize) & 0x03;
            super::vals::Gcmph::from_bits(val as u8)
        }
        #[doc = "GCM or CCM phase selection"]
        #[inline(always)]
        pub fn set_gcmph(&mut self, val: super::vals::Gcmph) {
            self.0 = (self.0 & !(0x03 << 13usize)) | (((val.to_bits() as u32) & 0x03) << 13usize);
        }
        #[doc = "Chaining mode bit2"]
        #[inline(always)]
        pub const fn chmod2(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Chaining mode bit2"]
        #[inline(always)]
        pub fn set_chmod2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Key size selection"]
        #[inline(always)]
        pub const fn keysize(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Key size selection"]
        #[inline(always)]
        pub fn set_keysize(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    #[doc = "Data input register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dinr(pub u32);
    impl Dinr {
        #[doc = "Input data word"]
        #[inline(always)]
        pub const fn din(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Input data word"]
        #[inline(always)]
        pub fn set_din(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dinr {
        #[inline(always)]
        fn default() -> Dinr {
            Dinr(0)
        }
    }
    #[doc = "Data output register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Doutr(pub u32);
    impl Doutr {
        #[doc = "Output data word"]
        #[inline(always)]
        pub const fn dout(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Output data word"]
        #[inline(always)]
        pub fn set_dout(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Doutr {
        #[inline(always)]
        fn default() -> Doutr {
            Doutr(0)
        }
    }
    #[doc = "Initialization vector register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ivr(pub u32);
    impl Ivr {
        #[doc = "Initialization vector input"]
        #[inline(always)]
        pub const fn ivi(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Initialization vector input"]
        #[inline(always)]
        pub fn set_ivi(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ivr {
        #[inline(always)]
        fn default() -> Ivr {
            Ivr(0)
        }
    }
    #[doc = "Key register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Keyr(pub u32);
    impl Keyr {
        #[doc = "Cryptographic key"]
        #[inline(always)]
        pub const fn key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Cryptographic key"]
        #[inline(always)]
        pub fn set_key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Keyr {
        #[inline(always)]
        fn default() -> Keyr {
            Keyr(0)
        }
    }
    #[doc = "Status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Computation complete flag"]
        #[inline(always)]
        pub const fn ccf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Computation complete flag"]
        #[inline(always)]
        pub fn set_ccf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Read error flag"]
        #[inline(always)]
        pub const fn rderr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Read error flag"]
        #[inline(always)]
        pub fn set_rderr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Write error flag"]
        #[inline(always)]
        pub const fn wrerr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Write error flag"]
        #[inline(always)]
        pub fn set_wrerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Busy flag"]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Busy flag"]
        #[inline(always)]
        pub fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
    #[doc = "Suspend register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Suspr(pub u32);
    impl Suspr {
        #[doc = "AES suspend"]
        #[inline(always)]
        pub const fn susp(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "AES suspend"]
        #[inline(always)]
        pub fn set_susp(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Suspr {
        #[inline(always)]
        fn default() -> Suspr {
            Suspr(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Datatype {
        #[doc = "Word"]
        NONE = 0x0,
        #[doc = "Half-word (16-bit)"]
        HALFWORD = 0x01,
        #[doc = "Byte (8-bit)"]
        BYTE = 0x02,
        #[doc = "Bit"]
        BIT = 0x03,
    }
    impl Datatype {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Datatype {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Datatype {
        #[inline(always)]
        fn from(val: u8) -> Datatype {
            Datatype::from_bits(val)
        }
    }
    impl From<Datatype> for u8 {
        #[inline(always)]
        fn from(val: Datatype) -> u8 {
            Datatype::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Gcmph {
        #[doc = "Init phase"]
        INITPHASE = 0x0,
        #[doc = "Header phase"]
        HEADERPHASE = 0x01,
        #[doc = "Payload phase"]
        PAYLOADPHASE = 0x02,
        #[doc = "Final phase"]
        FINALPHASE = 0x03,
    }
    impl Gcmph {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Gcmph {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Gcmph {
        #[inline(always)]
        fn from(val: u8) -> Gcmph {
            Gcmph::from_bits(val)
        }
    }
    impl From<Gcmph> for u8 {
        #[inline(always)]
        fn from(val: Gcmph) -> u8 {
            Gcmph::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Mode {
        #[doc = "Encryption"]
        MODE1 = 0x0,
        #[doc = "Key derivation (or key preparation for ECB/CBC decryption)"]
        MODE2 = 0x01,
        #[doc = "Decryption"]
        MODE3 = 0x02,
        #[doc = "Key derivation then single decryption"]
        MODE4 = 0x03,
    }
    impl Mode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mode {
        #[inline(always)]
        fn from(val: u8) -> Mode {
            Mode::from_bits(val)
        }
    }
    impl From<Mode> for u8 {
        #[inline(always)]
        fn from(val: Mode) -> u8 {
            Mode::to_bits(val)
        }
    }
}
