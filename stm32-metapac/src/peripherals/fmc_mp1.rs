#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "BCH error correction code."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bch {
    ptr: *mut u8,
}
unsafe impl Send for Bch {}
unsafe impl Sync for Bch {}
impl Bch {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "BCH interrupt enable register."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Bchier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "BCH interrupt status register."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Bchisr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "BCH interrupt clear register."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Bchicr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "BCH parity bits registers."]
    #[inline(always)]
    pub const fn pbr(self, n: usize) -> crate::common::Reg<regs::Bchpbr, crate::common::R> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize + n * 4usize) as _) }
    }
    #[doc = "BCH decoder status register 0."]
    #[inline(always)]
    pub const fn dsr0(self) -> crate::common::Reg<regs::Bchdsr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "BCH decoder status register 1."]
    #[inline(always)]
    pub const fn dsr1(self) -> crate::common::Reg<regs::Bchdsr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "BCH decoder status register 2."]
    #[inline(always)]
    pub const fn dsr2(self) -> crate::common::Reg<regs::Bchdsr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "BCH decoder status register 3."]
    #[inline(always)]
    pub const fn dsr3(self) -> crate::common::Reg<regs::Bchdsr3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "BCH decoder status register 4."]
    #[inline(always)]
    pub const fn dsr4(self) -> crate::common::Reg<regs::Bchdsr4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
}
#[doc = "NAND Command Sequencer."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csq {
    ptr: *mut u8,
}
unsafe impl Send for Csq {}
unsafe impl Sync for Csq {}
impl Csq {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Command Sequencer control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Csqcr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Command Sequencer configuration register 1."]
    #[inline(always)]
    pub const fn cfgr1(self) -> crate::common::Reg<regs::Csqcfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Command Sequencer configuration register 2."]
    #[inline(always)]
    pub const fn cfgr2(self) -> crate::common::Reg<regs::Csqcfgr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Command Sequencer configuration register 3."]
    #[inline(always)]
    pub const fn cfgr3(self) -> crate::common::Reg<regs::Csqcfgr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Command Sequencer address register 1."]
    #[inline(always)]
    pub const fn ar1(self) -> crate::common::Reg<regs::Csqar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Command Sequencer address register 2."]
    #[inline(always)]
    pub const fn ar2(self) -> crate::common::Reg<regs::Csqar2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Command Sequencer interrupt enable register."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Csqier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Command Sequencer interrupt status register."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Csqisr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Command Sequencer interrupt clear register."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Csqicr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Command Sequencer error mapping status register."]
    #[inline(always)]
    pub const fn emsr(self) -> crate::common::Reg<regs::Csqemsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
}
#[doc = "Flexible memory controller."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fmc {
    ptr: *mut u8,
}
unsafe impl Send for Fmc {}
unsafe impl Sync for Fmc {}
impl Fmc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn nor_psram(self) -> NorPsram {
        unsafe { NorPsram::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn nand(self) -> Nand {
        unsafe { Nand::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[inline(always)]
    pub const fn csq(self) -> Csq {
        unsafe { Csq::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[inline(always)]
    pub const fn bch(self) -> Bch {
        unsafe { Bch::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[doc = "Hardware configuration register 2."]
    #[inline(always)]
    pub const fn hwcfgr2(self) -> crate::common::Reg<regs::Hwcfgr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03ecusize) as _) }
    }
    #[doc = "Hardware configuration register 1."]
    #[inline(always)]
    pub const fn hwcfgr1(self) -> crate::common::Reg<regs::Hwcfgr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f0usize) as _) }
    }
    #[doc = "Version register."]
    #[inline(always)]
    pub const fn verr(self) -> crate::common::Reg<regs::Verr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f4usize) as _) }
    }
    #[doc = "Identification register."]
    #[inline(always)]
    pub const fn ipidr(self) -> crate::common::Reg<regs::Ipidr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f8usize) as _) }
    }
    #[doc = "Size ID register."]
    #[inline(always)]
    pub const fn sidr(self) -> crate::common::Reg<regs::Sidr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03fcusize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nand {
    ptr: *mut u8,
}
unsafe impl Send for Nand {}
unsafe impl Sync for Nand {}
impl Nand {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "NAND Flash control register."]
    #[inline(always)]
    pub const fn pcr(self) -> crate::common::Reg<regs::Pcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Status and interrupt register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Common memory space timing register."]
    #[inline(always)]
    pub const fn pmem(self) -> crate::common::Reg<regs::Pmem, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Attribute memory space timing register."]
    #[inline(always)]
    pub const fn patt(self) -> crate::common::Reg<regs::Patt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Hamming parity result register."]
    #[inline(always)]
    pub const fn hpr(self) -> crate::common::Reg<regs::Hpr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Hamming ECC result register."]
    #[inline(always)]
    pub const fn heccr(self) -> crate::common::Reg<regs::Heccr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NorPsram {
    ptr: *mut u8,
}
unsafe impl Send for NorPsram {}
unsafe impl Sync for NorPsram {}
impl NorPsram {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SRAM/NOR-Flash chip-select control register 1."]
    #[inline(always)]
    pub const fn bcr1(self) -> crate::common::Reg<regs::Bcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "SRAM/NOR-Flash chip-select timing registers 1-4."]
    #[inline(always)]
    pub const fn btr(self, n: usize) -> crate::common::Reg<regs::Btr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize + n * 8usize) as _) }
    }
    #[doc = "SRAM/NOR-Flash chip-select control registers 2-4."]
    #[inline(always)]
    pub const fn bcr(self, n: usize) -> crate::common::Reg<regs::Bcr, crate::common::RW> {
        assert!(n < 3usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize + n * 8usize) as _) }
    }
    #[doc = "PSRAM chip select counter register."]
    #[inline(always)]
    pub const fn pcscntr(self) -> crate::common::Reg<regs::Pcscntr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "SRAM/NOR-Flash write timing registers 1-4."]
    #[inline(always)]
    pub const fn bwtr(self, n: usize) -> crate::common::Reg<regs::Bwtr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize + n * 8usize) as _) }
    }
}
pub mod regs {
    #[doc = "BCH decoder status register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bchdsr0(pub u32);
    impl Bchdsr0 {
        #[doc = "Decoder uncorrectable error."]
        #[must_use]
        #[inline(always)]
        pub const fn due(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Decoder uncorrectable error."]
        #[inline(always)]
        pub const fn set_due(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Decoder error found."]
        #[must_use]
        #[inline(always)]
        pub const fn def(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Decoder error found."]
        #[inline(always)]
        pub const fn set_def(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Decoder error number."]
        #[must_use]
        #[inline(always)]
        pub const fn den(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Decoder error number."]
        #[inline(always)]
        pub const fn set_den(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Bchdsr0 {
        #[inline(always)]
        fn default() -> Bchdsr0 {
            Bchdsr0(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Bchdsr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bchdsr0")
                .field("due", &self.due())
                .field("def", &self.def())
                .field("den", &self.den())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bchdsr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bchdsr0 {{ due: {=bool:?}, def: {=bool:?}, den: {=u8:?} }}",
                self.due(),
                self.def(),
                self.den()
            )
        }
    }
    #[doc = "BCH decoder status register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bchdsr1(pub u32);
    impl Bchdsr1 {
        #[doc = "Error bit position 1."]
        #[must_use]
        #[inline(always)]
        pub const fn ebp1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Error bit position 1."]
        #[inline(always)]
        pub const fn set_ebp1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "Error bit position 2."]
        #[must_use]
        #[inline(always)]
        pub const fn ebp2(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "Error bit position 2."]
        #[inline(always)]
        pub const fn set_ebp2(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for Bchdsr1 {
        #[inline(always)]
        fn default() -> Bchdsr1 {
            Bchdsr1(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Bchdsr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bchdsr1")
                .field("ebp1", &self.ebp1())
                .field("ebp2", &self.ebp2())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bchdsr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bchdsr1 {{ ebp1: {=u16:?}, ebp2: {=u16:?} }}",
                self.ebp1(),
                self.ebp2()
            )
        }
    }
    #[doc = "BCH decoder status register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bchdsr2(pub u32);
    impl Bchdsr2 {
        #[doc = "Error bit position 3."]
        #[must_use]
        #[inline(always)]
        pub const fn ebp3(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Error bit position 3."]
        #[inline(always)]
        pub const fn set_ebp3(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "Error bit position 4."]
        #[must_use]
        #[inline(always)]
        pub const fn ebp4(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "Error bit position 4."]
        #[inline(always)]
        pub const fn set_ebp4(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for Bchdsr2 {
        #[inline(always)]
        fn default() -> Bchdsr2 {
            Bchdsr2(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Bchdsr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bchdsr2")
                .field("ebp3", &self.ebp3())
                .field("ebp4", &self.ebp4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bchdsr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bchdsr2 {{ ebp3: {=u16:?}, ebp4: {=u16:?} }}",
                self.ebp3(),
                self.ebp4()
            )
        }
    }
    #[doc = "BCH decoder status register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bchdsr3(pub u32);
    impl Bchdsr3 {
        #[doc = "Error bit position 5."]
        #[must_use]
        #[inline(always)]
        pub const fn ebp5(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Error bit position 5."]
        #[inline(always)]
        pub const fn set_ebp5(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "Error bit position 6."]
        #[must_use]
        #[inline(always)]
        pub const fn ebp6(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "Error bit position 6."]
        #[inline(always)]
        pub const fn set_ebp6(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for Bchdsr3 {
        #[inline(always)]
        fn default() -> Bchdsr3 {
            Bchdsr3(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Bchdsr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bchdsr3")
                .field("ebp5", &self.ebp5())
                .field("ebp6", &self.ebp6())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bchdsr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bchdsr3 {{ ebp5: {=u16:?}, ebp6: {=u16:?} }}",
                self.ebp5(),
                self.ebp6()
            )
        }
    }
    #[doc = "BCH decoder status register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bchdsr4(pub u32);
    impl Bchdsr4 {
        #[doc = "Error bit position 7."]
        #[must_use]
        #[inline(always)]
        pub const fn ebp7(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Error bit position 7."]
        #[inline(always)]
        pub const fn set_ebp7(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "Error bit position 8."]
        #[must_use]
        #[inline(always)]
        pub const fn ebp8(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "Error bit position 8."]
        #[inline(always)]
        pub const fn set_ebp8(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for Bchdsr4 {
        #[inline(always)]
        fn default() -> Bchdsr4 {
            Bchdsr4(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Bchdsr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bchdsr4")
                .field("ebp7", &self.ebp7())
                .field("ebp8", &self.ebp8())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bchdsr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bchdsr4 {{ ebp7: {=u16:?}, ebp8: {=u16:?} }}",
                self.ebp7(),
                self.ebp8()
            )
        }
    }
    #[doc = "BCH interrupt clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bchicr(pub u32);
    impl Bchicr {
        #[doc = "Clear decoder uncorrectable error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cduef(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear decoder uncorrectable error flag."]
        #[inline(always)]
        pub const fn set_cduef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear decoder error ready flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cderf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear decoder error ready flag."]
        #[inline(always)]
        pub const fn set_cderf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Clear decoder error found flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cdeff(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Clear decoder error found flag."]
        #[inline(always)]
        pub const fn set_cdeff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Clear decoder syndrome ready flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cdsrf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Clear decoder syndrome ready flag."]
        #[inline(always)]
        pub const fn set_cdsrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Clear encoder parity bits ready flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cepbrf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Clear encoder parity bits ready flag."]
        #[inline(always)]
        pub const fn set_cepbrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Bchicr {
        #[inline(always)]
        fn default() -> Bchicr {
            Bchicr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Bchicr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bchicr")
                .field("cduef", &self.cduef())
                .field("cderf", &self.cderf())
                .field("cdeff", &self.cdeff())
                .field("cdsrf", &self.cdsrf())
                .field("cepbrf", &self.cepbrf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bchicr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bchicr {{ cduef: {=bool:?}, cderf: {=bool:?}, cdeff: {=bool:?}, cdsrf: {=bool:?}, cepbrf: {=bool:?} }}",
                self.cduef(),
                self.cderf(),
                self.cdeff(),
                self.cdsrf(),
                self.cepbrf()
            )
        }
    }
    #[doc = "BCH interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bchier(pub u32);
    impl Bchier {
        #[doc = "Decoder uncorrectable error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dueie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Decoder uncorrectable error interrupt enable."]
        #[inline(always)]
        pub const fn set_dueie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Decoder error ready interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn derie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Decoder error ready interrupt enable."]
        #[inline(always)]
        pub const fn set_derie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Decoder error found interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn defie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Decoder error found interrupt enable."]
        #[inline(always)]
        pub const fn set_defie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Decoder syndrome ready interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dsrie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Decoder syndrome ready interrupt enable."]
        #[inline(always)]
        pub const fn set_dsrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Encoder parity bits ready interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn epbrie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Encoder parity bits ready interrupt enable."]
        #[inline(always)]
        pub const fn set_epbrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Bchier {
        #[inline(always)]
        fn default() -> Bchier {
            Bchier(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Bchier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bchier")
                .field("dueie", &self.dueie())
                .field("derie", &self.derie())
                .field("defie", &self.defie())
                .field("dsrie", &self.dsrie())
                .field("epbrie", &self.epbrie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bchier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bchier {{ dueie: {=bool:?}, derie: {=bool:?}, defie: {=bool:?}, dsrie: {=bool:?}, epbrie: {=bool:?} }}",
                self.dueie(),
                self.derie(),
                self.defie(),
                self.dsrie(),
                self.epbrie()
            )
        }
    }
    #[doc = "BCH interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bchisr(pub u32);
    impl Bchisr {
        #[doc = "Decoder uncorrectable error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn duef(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Decoder uncorrectable error flag."]
        #[inline(always)]
        pub const fn set_duef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Decoder error ready flag."]
        #[must_use]
        #[inline(always)]
        pub const fn derf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Decoder error ready flag."]
        #[inline(always)]
        pub const fn set_derf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Decoder error found flag."]
        #[must_use]
        #[inline(always)]
        pub const fn deff(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Decoder error found flag."]
        #[inline(always)]
        pub const fn set_deff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Decoder syndrome ready flag."]
        #[must_use]
        #[inline(always)]
        pub const fn dsrf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Decoder syndrome ready flag."]
        #[inline(always)]
        pub const fn set_dsrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Encoder parity bits ready flag."]
        #[must_use]
        #[inline(always)]
        pub const fn epbrf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Encoder parity bits ready flag."]
        #[inline(always)]
        pub const fn set_epbrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Bchisr {
        #[inline(always)]
        fn default() -> Bchisr {
            Bchisr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Bchisr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bchisr")
                .field("duef", &self.duef())
                .field("derf", &self.derf())
                .field("deff", &self.deff())
                .field("dsrf", &self.dsrf())
                .field("epbrf", &self.epbrf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bchisr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bchisr {{ duef: {=bool:?}, derf: {=bool:?}, deff: {=bool:?}, dsrf: {=bool:?}, epbrf: {=bool:?} }}",
                self.duef(),
                self.derf(),
                self.deff(),
                self.dsrf(),
                self.epbrf()
            )
        }
    }
    #[doc = "BCH parity bits register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bchpbr(pub u32);
    impl Bchpbr {
        #[doc = "BCH parity bits."]
        #[must_use]
        #[inline(always)]
        pub const fn bchpb(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "BCH parity bits."]
        #[inline(always)]
        pub const fn set_bchpb(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bchpbr {
        #[inline(always)]
        fn default() -> Bchpbr {
            Bchpbr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Bchpbr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bchpbr").field("bchpb", &self.bchpb()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bchpbr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bchpbr {{ bchpb: {=u32:?} }}", self.bchpb())
        }
    }
    #[doc = "SRAM/NOR-Flash chip-select control registers 2-4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bcr(pub u32);
    impl Bcr {
        #[doc = "Memory bank enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn mbken(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Memory bank enable bit."]
        #[inline(always)]
        pub const fn set_mbken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Address/data multiplexing enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn muxen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Address/data multiplexing enable bit."]
        #[inline(always)]
        pub const fn set_muxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Memory type."]
        #[must_use]
        #[inline(always)]
        pub const fn mtyp(&self) -> super::vals::Mtyp {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Mtyp::from_bits(val as u8)
        }
        #[doc = "Memory type."]
        #[inline(always)]
        pub const fn set_mtyp(&mut self, val: super::vals::Mtyp) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "Memory data bus width."]
        #[must_use]
        #[inline(always)]
        pub const fn mwid(&self) -> super::vals::Mwid {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Mwid::from_bits(val as u8)
        }
        #[doc = "Memory data bus width."]
        #[inline(always)]
        pub const fn set_mwid(&mut self, val: super::vals::Mwid) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Flash access enable."]
        #[must_use]
        #[inline(always)]
        pub const fn faccen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Flash access enable."]
        #[inline(always)]
        pub const fn set_faccen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Burst enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn bursten(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Burst enable bit."]
        #[inline(always)]
        pub const fn set_bursten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Wait signal polarity bit."]
        #[must_use]
        #[inline(always)]
        pub const fn waitpol(&self) -> super::vals::Waitpol {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Waitpol::from_bits(val as u8)
        }
        #[doc = "Wait signal polarity bit."]
        #[inline(always)]
        pub const fn set_waitpol(&mut self, val: super::vals::Waitpol) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "Wrapped burst mode support."]
        #[must_use]
        #[inline(always)]
        pub const fn wrapmod(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Wrapped burst mode support."]
        #[inline(always)]
        pub const fn set_wrapmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Wait timing configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn waitcfg(&self) -> super::vals::Waitcfg {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Waitcfg::from_bits(val as u8)
        }
        #[doc = "Wait timing configuration."]
        #[inline(always)]
        pub const fn set_waitcfg(&mut self, val: super::vals::Waitcfg) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Write enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn wren(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Write enable bit."]
        #[inline(always)]
        pub const fn set_wren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Wait enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn waiten(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Wait enable bit."]
        #[inline(always)]
        pub const fn set_waiten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Extended mode enable."]
        #[must_use]
        #[inline(always)]
        pub const fn extmod(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Extended mode enable."]
        #[inline(always)]
        pub const fn set_extmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Wait signal during asynchronous transfers."]
        #[must_use]
        #[inline(always)]
        pub const fn asyncwait(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Wait signal during asynchronous transfers."]
        #[inline(always)]
        pub const fn set_asyncwait(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "CRAM page size."]
        #[must_use]
        #[inline(always)]
        pub const fn cpsize(&self) -> super::vals::Cpsize {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::Cpsize::from_bits(val as u8)
        }
        #[doc = "CRAM page size."]
        #[inline(always)]
        pub const fn set_cpsize(&mut self, val: super::vals::Cpsize) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
        #[doc = "Write burst enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cburstrw(&self) -> super::vals::Cburstrw {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Cburstrw::from_bits(val as u8)
        }
        #[doc = "Write burst enable."]
        #[inline(always)]
        pub const fn set_cburstrw(&mut self, val: super::vals::Cburstrw) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "Byte lane (NBL) setup."]
        #[must_use]
        #[inline(always)]
        pub const fn nblset(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "Byte lane (NBL) setup."]
        #[inline(always)]
        pub const fn set_nblset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
    }
    impl Default for Bcr {
        #[inline(always)]
        fn default() -> Bcr {
            Bcr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Bcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bcr")
                .field("mbken", &self.mbken())
                .field("muxen", &self.muxen())
                .field("mtyp", &self.mtyp())
                .field("mwid", &self.mwid())
                .field("faccen", &self.faccen())
                .field("bursten", &self.bursten())
                .field("waitpol", &self.waitpol())
                .field("wrapmod", &self.wrapmod())
                .field("waitcfg", &self.waitcfg())
                .field("wren", &self.wren())
                .field("waiten", &self.waiten())
                .field("extmod", &self.extmod())
                .field("asyncwait", &self.asyncwait())
                .field("cpsize", &self.cpsize())
                .field("cburstrw", &self.cburstrw())
                .field("nblset", &self.nblset())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bcr {{ mbken: {=bool:?}, muxen: {=bool:?}, mtyp: {:?}, mwid: {:?}, faccen: {=bool:?}, bursten: {=bool:?}, waitpol: {:?}, wrapmod: {=bool:?}, waitcfg: {:?}, wren: {=bool:?}, waiten: {=bool:?}, extmod: {=bool:?}, asyncwait: {=bool:?}, cpsize: {:?}, cburstrw: {:?}, nblset: {=u8:?} }}",
                self.mbken(),
                self.muxen(),
                self.mtyp(),
                self.mwid(),
                self.faccen(),
                self.bursten(),
                self.waitpol(),
                self.wrapmod(),
                self.waitcfg(),
                self.wren(),
                self.waiten(),
                self.extmod(),
                self.asyncwait(),
                self.cpsize(),
                self.cburstrw(),
                self.nblset()
            )
        }
    }
    #[doc = "SRAM/NOR-Flash chip-select control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bcr1(pub u32);
    impl Bcr1 {
        #[doc = "Memory bank enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn mbken(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Memory bank enable bit."]
        #[inline(always)]
        pub const fn set_mbken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Address/data multiplexing enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn muxen(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Address/data multiplexing enable bit."]
        #[inline(always)]
        pub const fn set_muxen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Memory type."]
        #[must_use]
        #[inline(always)]
        pub const fn mtyp(&self) -> super::vals::Mtyp {
            let val = (self.0 >> 2usize) & 0x03;
            super::vals::Mtyp::from_bits(val as u8)
        }
        #[doc = "Memory type."]
        #[inline(always)]
        pub const fn set_mtyp(&mut self, val: super::vals::Mtyp) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
        }
        #[doc = "Memory data bus width."]
        #[must_use]
        #[inline(always)]
        pub const fn mwid(&self) -> super::vals::Mwid {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Mwid::from_bits(val as u8)
        }
        #[doc = "Memory data bus width."]
        #[inline(always)]
        pub const fn set_mwid(&mut self, val: super::vals::Mwid) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "Flash access enable."]
        #[must_use]
        #[inline(always)]
        pub const fn faccen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Flash access enable."]
        #[inline(always)]
        pub const fn set_faccen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Burst enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn bursten(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Burst enable bit."]
        #[inline(always)]
        pub const fn set_bursten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Wait signal polarity bit."]
        #[must_use]
        #[inline(always)]
        pub const fn waitpol(&self) -> super::vals::Waitpol {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Waitpol::from_bits(val as u8)
        }
        #[doc = "Wait signal polarity bit."]
        #[inline(always)]
        pub const fn set_waitpol(&mut self, val: super::vals::Waitpol) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "Wrapped burst mode support."]
        #[must_use]
        #[inline(always)]
        pub const fn wrapmod(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Wrapped burst mode support."]
        #[inline(always)]
        pub const fn set_wrapmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Wait timing configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn waitcfg(&self) -> super::vals::Waitcfg {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Waitcfg::from_bits(val as u8)
        }
        #[doc = "Wait timing configuration."]
        #[inline(always)]
        pub const fn set_waitcfg(&mut self, val: super::vals::Waitcfg) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Write enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn wren(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Write enable bit."]
        #[inline(always)]
        pub const fn set_wren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Wait enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn waiten(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Wait enable bit."]
        #[inline(always)]
        pub const fn set_waiten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Extended mode enable."]
        #[must_use]
        #[inline(always)]
        pub const fn extmod(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Extended mode enable."]
        #[inline(always)]
        pub const fn set_extmod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Wait signal during asynchronous transfers."]
        #[must_use]
        #[inline(always)]
        pub const fn asyncwait(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Wait signal during asynchronous transfers."]
        #[inline(always)]
        pub const fn set_asyncwait(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "CRAM page size."]
        #[must_use]
        #[inline(always)]
        pub const fn cpsize(&self) -> super::vals::Cpsize {
            let val = (self.0 >> 16usize) & 0x07;
            super::vals::Cpsize::from_bits(val as u8)
        }
        #[doc = "CRAM page size."]
        #[inline(always)]
        pub const fn set_cpsize(&mut self, val: super::vals::Cpsize) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
        }
        #[doc = "Write burst enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cburstrw(&self) -> super::vals::Cburstrw {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::Cburstrw::from_bits(val as u8)
        }
        #[doc = "Write burst enable."]
        #[inline(always)]
        pub const fn set_cburstrw(&mut self, val: super::vals::Cburstrw) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "Continuous clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cclken(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Continuous clock enable."]
        #[inline(always)]
        pub const fn set_cclken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Byte lane (NBL) setup."]
        #[must_use]
        #[inline(always)]
        pub const fn nblset(&self) -> u8 {
            let val = (self.0 >> 22usize) & 0x03;
            val as u8
        }
        #[doc = "Byte lane (NBL) setup."]
        #[inline(always)]
        pub const fn set_nblset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
        }
        #[doc = "FMC controller enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fmcen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "FMC controller enable."]
        #[inline(always)]
        pub const fn set_fmcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Bcr1 {
        #[inline(always)]
        fn default() -> Bcr1 {
            Bcr1(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Bcr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bcr1")
                .field("mbken", &self.mbken())
                .field("muxen", &self.muxen())
                .field("mtyp", &self.mtyp())
                .field("mwid", &self.mwid())
                .field("faccen", &self.faccen())
                .field("bursten", &self.bursten())
                .field("waitpol", &self.waitpol())
                .field("wrapmod", &self.wrapmod())
                .field("waitcfg", &self.waitcfg())
                .field("wren", &self.wren())
                .field("waiten", &self.waiten())
                .field("extmod", &self.extmod())
                .field("asyncwait", &self.asyncwait())
                .field("cpsize", &self.cpsize())
                .field("cburstrw", &self.cburstrw())
                .field("cclken", &self.cclken())
                .field("nblset", &self.nblset())
                .field("fmcen", &self.fmcen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bcr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bcr1 {{ mbken: {=bool:?}, muxen: {=bool:?}, mtyp: {:?}, mwid: {:?}, faccen: {=bool:?}, bursten: {=bool:?}, waitpol: {:?}, wrapmod: {=bool:?}, waitcfg: {:?}, wren: {=bool:?}, waiten: {=bool:?}, extmod: {=bool:?}, asyncwait: {=bool:?}, cpsize: {:?}, cburstrw: {:?}, cclken: {=bool:?}, nblset: {=u8:?}, fmcen: {=bool:?} }}",
                self.mbken(),
                self.muxen(),
                self.mtyp(),
                self.mwid(),
                self.faccen(),
                self.bursten(),
                self.waitpol(),
                self.wrapmod(),
                self.waitcfg(),
                self.wren(),
                self.waiten(),
                self.extmod(),
                self.asyncwait(),
                self.cpsize(),
                self.cburstrw(),
                self.cclken(),
                self.nblset(),
                self.fmcen()
            )
        }
    }
    #[doc = "SRAM/NOR-Flash chip-select timing register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Btr(pub u32);
    impl Btr {
        #[doc = "Address setup phase duration."]
        #[must_use]
        #[inline(always)]
        pub const fn addset(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Address setup phase duration."]
        #[inline(always)]
        pub const fn set_addset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Address-hold phase duration."]
        #[must_use]
        #[inline(always)]
        pub const fn addhld(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Address-hold phase duration."]
        #[inline(always)]
        pub const fn set_addhld(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Data-phase duration."]
        #[must_use]
        #[inline(always)]
        pub const fn datast(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Data-phase duration."]
        #[inline(always)]
        pub const fn set_datast(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Bus turnaround phase duration."]
        #[must_use]
        #[inline(always)]
        pub const fn busturn(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Bus turnaround phase duration."]
        #[inline(always)]
        pub const fn set_busturn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Clock divide ratio (for FMC_CLK signal)."]
        #[must_use]
        #[inline(always)]
        pub const fn clkdiv(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Clock divide ratio (for FMC_CLK signal)."]
        #[inline(always)]
        pub const fn set_clkdiv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "Data latency for synchronous memory."]
        #[must_use]
        #[inline(always)]
        pub const fn datlat(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Data latency for synchronous memory."]
        #[inline(always)]
        pub const fn set_datlat(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Access mode."]
        #[must_use]
        #[inline(always)]
        pub const fn accmod(&self) -> super::vals::Accmod {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Accmod::from_bits(val as u8)
        }
        #[doc = "Access mode."]
        #[inline(always)]
        pub const fn set_accmod(&mut self, val: super::vals::Accmod) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "Data hold phase duration."]
        #[must_use]
        #[inline(always)]
        pub const fn datahld(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "Data hold phase duration."]
        #[inline(always)]
        pub const fn set_datahld(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for Btr {
        #[inline(always)]
        fn default() -> Btr {
            Btr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Btr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Btr")
                .field("addset", &self.addset())
                .field("addhld", &self.addhld())
                .field("datast", &self.datast())
                .field("busturn", &self.busturn())
                .field("clkdiv", &self.clkdiv())
                .field("datlat", &self.datlat())
                .field("accmod", &self.accmod())
                .field("datahld", &self.datahld())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Btr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Btr {{ addset: {=u8:?}, addhld: {=u8:?}, datast: {=u8:?}, busturn: {=u8:?}, clkdiv: {=u8:?}, datlat: {=u8:?}, accmod: {:?}, datahld: {=u8:?} }}",
                self.addset(),
                self.addhld(),
                self.datast(),
                self.busturn(),
                self.clkdiv(),
                self.datlat(),
                self.accmod(),
                self.datahld()
            )
        }
    }
    #[doc = "SRAM/NOR-Flash write timing registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bwtr(pub u32);
    impl Bwtr {
        #[doc = "Address setup phase duration."]
        #[must_use]
        #[inline(always)]
        pub const fn addset(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Address setup phase duration."]
        #[inline(always)]
        pub const fn set_addset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Address-hold phase duration."]
        #[must_use]
        #[inline(always)]
        pub const fn addhld(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Address-hold phase duration."]
        #[inline(always)]
        pub const fn set_addhld(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Data-phase duration."]
        #[must_use]
        #[inline(always)]
        pub const fn datast(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Data-phase duration."]
        #[inline(always)]
        pub const fn set_datast(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Bus turnaround phase duration."]
        #[must_use]
        #[inline(always)]
        pub const fn busturn(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Bus turnaround phase duration."]
        #[inline(always)]
        pub const fn set_busturn(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Access mode."]
        #[must_use]
        #[inline(always)]
        pub const fn accmod(&self) -> super::vals::Accmod {
            let val = (self.0 >> 28usize) & 0x03;
            super::vals::Accmod::from_bits(val as u8)
        }
        #[doc = "Access mode."]
        #[inline(always)]
        pub const fn set_accmod(&mut self, val: super::vals::Accmod) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
        }
        #[doc = "Data hold phase duration."]
        #[must_use]
        #[inline(always)]
        pub const fn datahld(&self) -> u8 {
            let val = (self.0 >> 30usize) & 0x03;
            val as u8
        }
        #[doc = "Data hold phase duration."]
        #[inline(always)]
        pub const fn set_datahld(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
        }
    }
    impl Default for Bwtr {
        #[inline(always)]
        fn default() -> Bwtr {
            Bwtr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Bwtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bwtr")
                .field("addset", &self.addset())
                .field("addhld", &self.addhld())
                .field("datast", &self.datast())
                .field("busturn", &self.busturn())
                .field("accmod", &self.accmod())
                .field("datahld", &self.datahld())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bwtr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bwtr {{ addset: {=u8:?}, addhld: {=u8:?}, datast: {=u8:?}, busturn: {=u8:?}, accmod: {:?}, datahld: {=u8:?} }}",
                self.addset(),
                self.addhld(),
                self.datast(),
                self.busturn(),
                self.accmod(),
                self.datahld()
            )
        }
    }
    #[doc = "Command Sequencer address register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csqar1(pub u32);
    impl Csqar1 {
        #[doc = "Address cycle 1."]
        #[must_use]
        #[inline(always)]
        pub const fn addc1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Address cycle 1."]
        #[inline(always)]
        pub const fn set_addc1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Address cycle 2."]
        #[must_use]
        #[inline(always)]
        pub const fn addc2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Address cycle 2."]
        #[inline(always)]
        pub const fn set_addc2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Address cycle 3."]
        #[must_use]
        #[inline(always)]
        pub const fn addc3(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Address cycle 3."]
        #[inline(always)]
        pub const fn set_addc3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Address cycle 4."]
        #[must_use]
        #[inline(always)]
        pub const fn addc4(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Address cycle 4."]
        #[inline(always)]
        pub const fn set_addc4(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Csqar1 {
        #[inline(always)]
        fn default() -> Csqar1 {
            Csqar1(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Csqar1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csqar1")
                .field("addc1", &self.addc1())
                .field("addc2", &self.addc2())
                .field("addc3", &self.addc3())
                .field("addc4", &self.addc4())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csqar1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csqar1 {{ addc1: {=u8:?}, addc2: {=u8:?}, addc3: {=u8:?}, addc4: {=u8:?} }}",
                self.addc1(),
                self.addc2(),
                self.addc3(),
                self.addc4()
            )
        }
    }
    #[doc = "Command Sequencer address register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csqar2(pub u32);
    impl Csqar2 {
        #[doc = "Address cycle 5."]
        #[must_use]
        #[inline(always)]
        pub const fn addc5(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Address cycle 5."]
        #[inline(always)]
        pub const fn set_addc5(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "NAND Flash chip enable 0."]
        #[must_use]
        #[inline(always)]
        pub const fn nandcen0(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "NAND Flash chip enable 0."]
        #[inline(always)]
        pub const fn set_nandcen0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "NAND Flash chip enable 1."]
        #[must_use]
        #[inline(always)]
        pub const fn nandcen1(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "NAND Flash chip enable 1."]
        #[inline(always)]
        pub const fn set_nandcen1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Spare area address offset."]
        #[must_use]
        #[inline(always)]
        pub const fn sao(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Spare area address offset."]
        #[inline(always)]
        pub const fn set_sao(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Csqar2 {
        #[inline(always)]
        fn default() -> Csqar2 {
            Csqar2(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Csqar2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csqar2")
                .field("addc5", &self.addc5())
                .field("nandcen0", &self.nandcen0())
                .field("nandcen1", &self.nandcen1())
                .field("sao", &self.sao())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csqar2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csqar2 {{ addc5: {=u8:?}, nandcen0: {=bool:?}, nandcen1: {=bool:?}, sao: {=u16:?} }}",
                self.addc5(),
                self.nandcen0(),
                self.nandcen1(),
                self.sao()
            )
        }
    }
    #[doc = "Command Sequencer configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csqcfgr1(pub u32);
    impl Csqcfgr1 {
        #[doc = "Command cycle 2 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmd2en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Command cycle 2 enable."]
        #[inline(always)]
        pub const fn set_cmd2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DMA request data enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dmaden(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DMA request data enable."]
        #[inline(always)]
        pub const fn set_dmaden(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Address cycle number."]
        #[must_use]
        #[inline(always)]
        pub const fn acynbr(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Address cycle number."]
        #[inline(always)]
        pub const fn set_acynbr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Command 1."]
        #[must_use]
        #[inline(always)]
        pub const fn cmd1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Command 1."]
        #[inline(always)]
        pub const fn set_cmd1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Command 2."]
        #[must_use]
        #[inline(always)]
        pub const fn cmd2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Command 2."]
        #[inline(always)]
        pub const fn set_cmd2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Command 1 timing."]
        #[must_use]
        #[inline(always)]
        pub const fn cmd1t(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Command 1 timing."]
        #[inline(always)]
        pub const fn set_cmd1t(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Command 2 timing."]
        #[must_use]
        #[inline(always)]
        pub const fn cmd2t(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Command 2 timing."]
        #[inline(always)]
        pub const fn set_cmd2t(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Csqcfgr1 {
        #[inline(always)]
        fn default() -> Csqcfgr1 {
            Csqcfgr1(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Csqcfgr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csqcfgr1")
                .field("cmd2en", &self.cmd2en())
                .field("dmaden", &self.dmaden())
                .field("acynbr", &self.acynbr())
                .field("cmd1", &self.cmd1())
                .field("cmd2", &self.cmd2())
                .field("cmd1t", &self.cmd1t())
                .field("cmd2t", &self.cmd2t())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csqcfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csqcfgr1 {{ cmd2en: {=bool:?}, dmaden: {=bool:?}, acynbr: {=u8:?}, cmd1: {=u8:?}, cmd2: {=u8:?}, cmd1t: {=bool:?}, cmd2t: {=bool:?} }}",
                self.cmd2en(),
                self.dmaden(),
                self.acynbr(),
                self.cmd1(),
                self.cmd2(),
                self.cmd1t(),
                self.cmd2t()
            )
        }
    }
    #[doc = "Command Sequencer configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csqcfgr2(pub u32);
    impl Csqcfgr2 {
        #[doc = "Sequencer spare data transfer enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sqsdten(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Sequencer spare data transfer enable."]
        #[inline(always)]
        pub const fn set_sqsdten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Random command 2 sequencer enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rcmd2en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Random command 2 sequencer enable."]
        #[inline(always)]
        pub const fn set_rcmd2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "DMA request decoding status enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dmasen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DMA request decoding status enable."]
        #[inline(always)]
        pub const fn set_dmasen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Random command 1."]
        #[must_use]
        #[inline(always)]
        pub const fn rcmd1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Random command 1."]
        #[inline(always)]
        pub const fn set_rcmd1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Random command 2."]
        #[must_use]
        #[inline(always)]
        pub const fn rcmd2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Random command 2."]
        #[inline(always)]
        pub const fn set_rcmd2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Random command 1 timing."]
        #[must_use]
        #[inline(always)]
        pub const fn rcmd1t(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Random command 1 timing."]
        #[inline(always)]
        pub const fn set_rcmd1t(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Random command 2 timing."]
        #[must_use]
        #[inline(always)]
        pub const fn rcmd2t(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Random command 2 timing."]
        #[inline(always)]
        pub const fn set_rcmd2t(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Csqcfgr2 {
        #[inline(always)]
        fn default() -> Csqcfgr2 {
            Csqcfgr2(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Csqcfgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csqcfgr2")
                .field("sqsdten", &self.sqsdten())
                .field("rcmd2en", &self.rcmd2en())
                .field("dmasen", &self.dmasen())
                .field("rcmd1", &self.rcmd1())
                .field("rcmd2", &self.rcmd2())
                .field("rcmd1t", &self.rcmd1t())
                .field("rcmd2t", &self.rcmd2t())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csqcfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csqcfgr2 {{ sqsdten: {=bool:?}, rcmd2en: {=bool:?}, dmasen: {=bool:?}, rcmd1: {=u8:?}, rcmd2: {=u8:?}, rcmd1t: {=bool:?}, rcmd2t: {=bool:?} }}",
                self.sqsdten(),
                self.rcmd2en(),
                self.dmasen(),
                self.rcmd1(),
                self.rcmd2(),
                self.rcmd1t(),
                self.rcmd2t()
            )
        }
    }
    #[doc = "Command Sequencer configuration register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csqcfgr3(pub u32);
    impl Csqcfgr3 {
        #[doc = "Number of sectors to read/write."]
        #[must_use]
        #[inline(always)]
        pub const fn snbr(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "Number of sectors to read/write."]
        #[inline(always)]
        pub const fn set_snbr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
        #[doc = "Address cycle 1 timing."]
        #[must_use]
        #[inline(always)]
        pub const fn ac1t(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Address cycle 1 timing."]
        #[inline(always)]
        pub const fn set_ac1t(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Address cycle 2 timing."]
        #[must_use]
        #[inline(always)]
        pub const fn ac2t(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Address cycle 2 timing."]
        #[inline(always)]
        pub const fn set_ac2t(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Address cycle 3 timing."]
        #[must_use]
        #[inline(always)]
        pub const fn ac3t(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Address cycle 3 timing."]
        #[inline(always)]
        pub const fn set_ac3t(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Address cycle 4 timing."]
        #[must_use]
        #[inline(always)]
        pub const fn ac4t(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Address cycle 4 timing."]
        #[inline(always)]
        pub const fn set_ac4t(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Address cycle 5 timing."]
        #[must_use]
        #[inline(always)]
        pub const fn ac5t(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Address cycle 5 timing."]
        #[inline(always)]
        pub const fn set_ac5t(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Spare data transfer timing."]
        #[must_use]
        #[inline(always)]
        pub const fn sdt(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Spare data transfer timing."]
        #[inline(always)]
        pub const fn set_sdt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Random address cycle 1 timing."]
        #[must_use]
        #[inline(always)]
        pub const fn rac1t(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Random address cycle 1 timing."]
        #[inline(always)]
        pub const fn set_rac1t(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Random address cycle 2 timing."]
        #[must_use]
        #[inline(always)]
        pub const fn rac2t(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Random address cycle 2 timing."]
        #[inline(always)]
        pub const fn set_rac2t(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Csqcfgr3 {
        #[inline(always)]
        fn default() -> Csqcfgr3 {
            Csqcfgr3(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Csqcfgr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csqcfgr3")
                .field("snbr", &self.snbr())
                .field("ac1t", &self.ac1t())
                .field("ac2t", &self.ac2t())
                .field("ac3t", &self.ac3t())
                .field("ac4t", &self.ac4t())
                .field("ac5t", &self.ac5t())
                .field("sdt", &self.sdt())
                .field("rac1t", &self.rac1t())
                .field("rac2t", &self.rac2t())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csqcfgr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csqcfgr3 {{ snbr: {=u8:?}, ac1t: {=bool:?}, ac2t: {=bool:?}, ac3t: {=bool:?}, ac4t: {=bool:?}, ac5t: {=bool:?}, sdt: {=bool:?}, rac1t: {=bool:?}, rac2t: {=bool:?} }}",
                self.snbr(),
                self.ac1t(),
                self.ac2t(),
                self.ac3t(),
                self.ac4t(),
                self.ac5t(),
                self.sdt(),
                self.rac1t(),
                self.rac2t()
            )
        }
    }
    #[doc = "Command Sequencer control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csqcr(pub u32);
    impl Csqcr {
        #[doc = "Command Sequencer start."]
        #[must_use]
        #[inline(always)]
        pub const fn csqstart(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Command Sequencer start."]
        #[inline(always)]
        pub const fn set_csqstart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Csqcr {
        #[inline(always)]
        fn default() -> Csqcr {
            Csqcr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Csqcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csqcr").field("csqstart", &self.csqstart()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csqcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Csqcr {{ csqstart: {=bool:?} }}", self.csqstart())
        }
    }
    #[doc = "Command Sequencer error mapping status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csqemsr(pub u32);
    impl Csqemsr {
        #[doc = "Sector error mapping."]
        #[must_use]
        #[inline(always)]
        pub const fn sem(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Sector error mapping."]
        #[inline(always)]
        pub const fn set_sem(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Csqemsr {
        #[inline(always)]
        fn default() -> Csqemsr {
            Csqemsr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Csqemsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csqemsr").field("sem", &self.sem()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csqemsr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Csqemsr {{ sem: {=u16:?} }}", self.sem())
        }
    }
    #[doc = "Command Sequencer interrupt clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csqicr(pub u32);
    impl Csqicr {
        #[doc = "Clear transfer complete flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ctcf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear transfer complete flag."]
        #[inline(always)]
        pub const fn set_ctcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear sector complete flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cscf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear sector complete flag."]
        #[inline(always)]
        pub const fn set_cscf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Clear sector error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn csef(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Clear sector error flag."]
        #[inline(always)]
        pub const fn set_csef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Clear sector uncorrectable error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn csuef(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Clear sector uncorrectable error flag."]
        #[inline(always)]
        pub const fn set_csuef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Clear command transfer complete flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ccmdtcf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Clear command transfer complete flag."]
        #[inline(always)]
        pub const fn set_ccmdtcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Csqicr {
        #[inline(always)]
        fn default() -> Csqicr {
            Csqicr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Csqicr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csqicr")
                .field("ctcf", &self.ctcf())
                .field("cscf", &self.cscf())
                .field("csef", &self.csef())
                .field("csuef", &self.csuef())
                .field("ccmdtcf", &self.ccmdtcf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csqicr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csqicr {{ ctcf: {=bool:?}, cscf: {=bool:?}, csef: {=bool:?}, csuef: {=bool:?}, ccmdtcf: {=bool:?} }}",
                self.ctcf(),
                self.cscf(),
                self.csef(),
                self.csuef(),
                self.ccmdtcf()
            )
        }
    }
    #[doc = "Command Sequencer interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csqier(pub u32);
    impl Csqier {
        #[doc = "Transfer complete interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tcie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer complete interrupt enable."]
        #[inline(always)]
        pub const fn set_tcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Sector complete interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn scie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Sector complete interrupt enable."]
        #[inline(always)]
        pub const fn set_scie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Sector error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn seie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Sector error interrupt enable."]
        #[inline(always)]
        pub const fn set_seie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Sector uncorrectable error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sueie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Sector uncorrectable error interrupt enable."]
        #[inline(always)]
        pub const fn set_sueie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Command transfer complete interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cmdtcie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Command transfer complete interrupt enable."]
        #[inline(always)]
        pub const fn set_cmdtcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Csqier {
        #[inline(always)]
        fn default() -> Csqier {
            Csqier(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Csqier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csqier")
                .field("tcie", &self.tcie())
                .field("scie", &self.scie())
                .field("seie", &self.seie())
                .field("sueie", &self.sueie())
                .field("cmdtcie", &self.cmdtcie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csqier {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csqier {{ tcie: {=bool:?}, scie: {=bool:?}, seie: {=bool:?}, sueie: {=bool:?}, cmdtcie: {=bool:?} }}",
                self.tcie(),
                self.scie(),
                self.seie(),
                self.sueie(),
                self.cmdtcie()
            )
        }
    }
    #[doc = "Command Sequencer interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csqisr(pub u32);
    impl Csqisr {
        #[doc = "Transfer complete flag."]
        #[must_use]
        #[inline(always)]
        pub const fn tcf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer complete flag."]
        #[inline(always)]
        pub const fn set_tcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Sector complete flag."]
        #[must_use]
        #[inline(always)]
        pub const fn scf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Sector complete flag."]
        #[inline(always)]
        pub const fn set_scf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Sector error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn sef(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Sector error flag."]
        #[inline(always)]
        pub const fn set_sef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Sector uncorrectable error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn suef(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Sector uncorrectable error flag."]
        #[inline(always)]
        pub const fn set_suef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Command transfer complete flag."]
        #[must_use]
        #[inline(always)]
        pub const fn cmdtcf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Command transfer complete flag."]
        #[inline(always)]
        pub const fn set_cmdtcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Csqisr {
        #[inline(always)]
        fn default() -> Csqisr {
            Csqisr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Csqisr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csqisr")
                .field("tcf", &self.tcf())
                .field("scf", &self.scf())
                .field("sef", &self.sef())
                .field("suef", &self.suef())
                .field("cmdtcf", &self.cmdtcf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csqisr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csqisr {{ tcf: {=bool:?}, scf: {=bool:?}, sef: {=bool:?}, suef: {=bool:?}, cmdtcf: {=bool:?} }}",
                self.tcf(),
                self.scf(),
                self.sef(),
                self.suef(),
                self.cmdtcf()
            )
        }
    }
    #[doc = "Hamming ECC result register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Heccr(pub u32);
    impl Heccr {
        #[doc = "Hamming ECC result value."]
        #[must_use]
        #[inline(always)]
        pub const fn hecc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Hamming ECC result value."]
        #[inline(always)]
        pub const fn set_hecc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Heccr {
        #[inline(always)]
        fn default() -> Heccr {
            Heccr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Heccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Heccr").field("hecc", &self.hecc()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Heccr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Heccr {{ hecc: {=u32:?} }}", self.hecc())
        }
    }
    #[doc = "Hamming parity result register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hpr(pub u32);
    impl Hpr {
        #[doc = "Hamming parity result value."]
        #[must_use]
        #[inline(always)]
        pub const fn hpr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Hamming parity result value."]
        #[inline(always)]
        pub const fn set_hpr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Hpr {
        #[inline(always)]
        fn default() -> Hpr {
            Hpr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Hpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hpr").field("hpr", &self.hpr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hpr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Hpr {{ hpr: {=u32:?} }}", self.hpr())
        }
    }
    #[doc = "Hardware configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hwcfgr1(pub u32);
    impl Hwcfgr1 {
        #[doc = "NAND bank selection."]
        #[must_use]
        #[inline(always)]
        pub const fn nand_sel(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "NAND bank selection."]
        #[inline(always)]
        pub const fn set_nand_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "NAND ECC capability."]
        #[must_use]
        #[inline(always)]
        pub const fn nand_ecc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "NAND ECC capability."]
        #[inline(always)]
        pub const fn set_nand_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "SDRAM bank selection."]
        #[must_use]
        #[inline(always)]
        pub const fn sdram_sel(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SDRAM bank selection."]
        #[inline(always)]
        pub const fn set_sdram_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "AXI ID width."]
        #[must_use]
        #[inline(always)]
        pub const fn id_size(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "AXI ID width."]
        #[inline(always)]
        pub const fn set_id_size(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "Write address FIFO depth (log2)."]
        #[must_use]
        #[inline(always)]
        pub const fn wa_ln2dpth(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Write address FIFO depth (log2)."]
        #[inline(always)]
        pub const fn set_wa_ln2dpth(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Write data FIFO depth (log2)."]
        #[must_use]
        #[inline(always)]
        pub const fn wd_ln2dpth(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "Write data FIFO depth (log2)."]
        #[inline(always)]
        pub const fn set_wd_ln2dpth(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "Write response FIFO depth (log2)."]
        #[must_use]
        #[inline(always)]
        pub const fn wr_ln2dpth(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Write response FIFO depth (log2)."]
        #[inline(always)]
        pub const fn set_wr_ln2dpth(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Read address FIFO depth (log2)."]
        #[must_use]
        #[inline(always)]
        pub const fn ra_ln2dpth(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "Read address FIFO depth (log2)."]
        #[inline(always)]
        pub const fn set_ra_ln2dpth(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Hwcfgr1 {
        #[inline(always)]
        fn default() -> Hwcfgr1 {
            Hwcfgr1(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Hwcfgr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hwcfgr1")
                .field("nand_sel", &self.nand_sel())
                .field("nand_ecc", &self.nand_ecc())
                .field("sdram_sel", &self.sdram_sel())
                .field("id_size", &self.id_size())
                .field("wa_ln2dpth", &self.wa_ln2dpth())
                .field("wd_ln2dpth", &self.wd_ln2dpth())
                .field("wr_ln2dpth", &self.wr_ln2dpth())
                .field("ra_ln2dpth", &self.ra_ln2dpth())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hwcfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hwcfgr1 {{ nand_sel: {=bool:?}, nand_ecc: {=bool:?}, sdram_sel: {=bool:?}, id_size: {=u8:?}, wa_ln2dpth: {=u8:?}, wd_ln2dpth: {=u8:?}, wr_ln2dpth: {=u8:?}, ra_ln2dpth: {=u8:?} }}",
                self.nand_sel(),
                self.nand_ecc(),
                self.sdram_sel(),
                self.id_size(),
                self.wa_ln2dpth(),
                self.wd_ln2dpth(),
                self.wr_ln2dpth(),
                self.ra_ln2dpth()
            )
        }
    }
    #[doc = "Hardware configuration register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hwcfgr2(pub u32);
    impl Hwcfgr2 {
        #[doc = "Read data FIFO depth (log2)."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_ln2dpth(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Read data FIFO depth (log2)."]
        #[inline(always)]
        pub const fn set_rd_ln2dpth(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "NOR base address."]
        #[must_use]
        #[inline(always)]
        pub const fn nor_base(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "NOR base address."]
        #[inline(always)]
        pub const fn set_nor_base(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "SDRAM remapped base address."]
        #[must_use]
        #[inline(always)]
        pub const fn sdram_rbase(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "SDRAM remapped base address."]
        #[inline(always)]
        pub const fn set_sdram_rbase(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "NAND base address."]
        #[must_use]
        #[inline(always)]
        pub const fn nand_base(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "NAND base address."]
        #[inline(always)]
        pub const fn set_nand_base(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "SDRAM bank 1 base address."]
        #[must_use]
        #[inline(always)]
        pub const fn sdram1_base(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "SDRAM bank 1 base address."]
        #[inline(always)]
        pub const fn set_sdram1_base(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "SDRAM bank 2 base address."]
        #[must_use]
        #[inline(always)]
        pub const fn sdram2_base(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "SDRAM bank 2 base address."]
        #[inline(always)]
        pub const fn set_sdram2_base(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
    }
    impl Default for Hwcfgr2 {
        #[inline(always)]
        fn default() -> Hwcfgr2 {
            Hwcfgr2(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Hwcfgr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Hwcfgr2")
                .field("rd_ln2dpth", &self.rd_ln2dpth())
                .field("nor_base", &self.nor_base())
                .field("sdram_rbase", &self.sdram_rbase())
                .field("nand_base", &self.nand_base())
                .field("sdram1_base", &self.sdram1_base())
                .field("sdram2_base", &self.sdram2_base())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Hwcfgr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Hwcfgr2 {{ rd_ln2dpth: {=u8:?}, nor_base: {=u8:?}, sdram_rbase: {=u8:?}, nand_base: {=u8:?}, sdram1_base: {=u8:?}, sdram2_base: {=u8:?} }}",
                self.rd_ln2dpth(),
                self.nor_base(),
                self.sdram_rbase(),
                self.nand_base(),
                self.sdram1_base(),
                self.sdram2_base()
            )
        }
    }
    #[doc = "Identification register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipidr(pub u32);
    impl Ipidr {
        #[doc = "Peripheral identification."]
        #[must_use]
        #[inline(always)]
        pub const fn id(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Peripheral identification."]
        #[inline(always)]
        pub const fn set_id(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ipidr {
        #[inline(always)]
        fn default() -> Ipidr {
            Ipidr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Ipidr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ipidr").field("id", &self.id()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ipidr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Ipidr {{ id: {=u32:?} }}", self.id())
        }
    }
    #[doc = "Attribute memory space timing register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Patt(pub u32);
    impl Patt {
        #[doc = "Attribute memory setup time."]
        #[must_use]
        #[inline(always)]
        pub const fn attset(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Attribute memory setup time."]
        #[inline(always)]
        pub const fn set_attset(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Attribute memory wait time."]
        #[must_use]
        #[inline(always)]
        pub const fn attwait(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Attribute memory wait time."]
        #[inline(always)]
        pub const fn set_attwait(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Attribute memory hold time."]
        #[must_use]
        #[inline(always)]
        pub const fn atthold(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Attribute memory hold time."]
        #[inline(always)]
        pub const fn set_atthold(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Attribute memory data bus Hi-Z time."]
        #[must_use]
        #[inline(always)]
        pub const fn atthiz(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Attribute memory data bus Hi-Z time."]
        #[inline(always)]
        pub const fn set_atthiz(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Patt {
        #[inline(always)]
        fn default() -> Patt {
            Patt(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Patt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Patt")
                .field("attset", &self.attset())
                .field("attwait", &self.attwait())
                .field("atthold", &self.atthold())
                .field("atthiz", &self.atthiz())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Patt {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Patt {{ attset: {=u8:?}, attwait: {=u8:?}, atthold: {=u8:?}, atthiz: {=u8:?} }}",
                self.attset(),
                self.attwait(),
                self.atthold(),
                self.atthiz()
            )
        }
    }
    #[doc = "NAND Flash control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcr(pub u32);
    impl Pcr {
        #[doc = "Wait feature enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn pwaiten(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Wait feature enable bit."]
        #[inline(always)]
        pub const fn set_pwaiten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "NAND Flash memory bank enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn pbken(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "NAND Flash memory bank enable bit."]
        #[inline(always)]
        pub const fn set_pbken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Data bus width."]
        #[must_use]
        #[inline(always)]
        pub const fn pwid(&self) -> super::vals::Pwid {
            let val = (self.0 >> 4usize) & 0x03;
            super::vals::Pwid::from_bits(val as u8)
        }
        #[doc = "Data bus width."]
        #[inline(always)]
        pub const fn set_pwid(&mut self, val: super::vals::Pwid) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
        }
        #[doc = "ECC computation logic enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn eccen(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "ECC computation logic enable bit."]
        #[inline(always)]
        pub const fn set_eccen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "ECC algorithm."]
        #[must_use]
        #[inline(always)]
        pub const fn eccalg(&self) -> super::vals::Eccalg {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Eccalg::from_bits(val as u8)
        }
        #[doc = "ECC algorithm."]
        #[inline(always)]
        pub const fn set_eccalg(&mut self, val: super::vals::Eccalg) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "CLE to RE delay."]
        #[must_use]
        #[inline(always)]
        pub const fn tclr(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x0f;
            val as u8
        }
        #[doc = "CLE to RE delay."]
        #[inline(always)]
        pub const fn set_tclr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 9usize)) | (((val as u32) & 0x0f) << 9usize);
        }
        #[doc = "ALE to RE delay."]
        #[must_use]
        #[inline(always)]
        pub const fn tar(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x0f;
            val as u8
        }
        #[doc = "ALE to RE delay."]
        #[inline(always)]
        pub const fn set_tar(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 13usize)) | (((val as u32) & 0x0f) << 13usize);
        }
        #[doc = "ECC sector size."]
        #[must_use]
        #[inline(always)]
        pub const fn eccss(&self) -> super::vals::Eccss {
            let val = (self.0 >> 17usize) & 0x07;
            super::vals::Eccss::from_bits(val as u8)
        }
        #[doc = "ECC sector size."]
        #[inline(always)]
        pub const fn set_eccss(&mut self, val: super::vals::Eccss) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val.to_bits() as u32) & 0x07) << 17usize);
        }
        #[doc = "CLE to RE/CE hold timing."]
        #[must_use]
        #[inline(always)]
        pub const fn tceh(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "CLE to RE/CE hold timing."]
        #[inline(always)]
        pub const fn set_tceh(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "BCH error correction capability enable."]
        #[must_use]
        #[inline(always)]
        pub const fn bchecc(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "BCH error correction capability enable."]
        #[inline(always)]
        pub const fn set_bchecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Wait feature enable during write access."]
        #[must_use]
        #[inline(always)]
        pub const fn wen(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Wait feature enable during write access."]
        #[inline(always)]
        pub const fn set_wen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Pcr {
        #[inline(always)]
        fn default() -> Pcr {
            Pcr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Pcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcr")
                .field("pwaiten", &self.pwaiten())
                .field("pbken", &self.pbken())
                .field("pwid", &self.pwid())
                .field("eccen", &self.eccen())
                .field("eccalg", &self.eccalg())
                .field("tclr", &self.tclr())
                .field("tar", &self.tar())
                .field("eccss", &self.eccss())
                .field("tceh", &self.tceh())
                .field("bchecc", &self.bchecc())
                .field("wen", &self.wen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pcr {{ pwaiten: {=bool:?}, pbken: {=bool:?}, pwid: {:?}, eccen: {=bool:?}, eccalg: {:?}, tclr: {=u8:?}, tar: {=u8:?}, eccss: {:?}, tceh: {=u8:?}, bchecc: {=bool:?}, wen: {=bool:?} }}",
                self.pwaiten(),
                self.pbken(),
                self.pwid(),
                self.eccen(),
                self.eccalg(),
                self.tclr(),
                self.tar(),
                self.eccss(),
                self.tceh(),
                self.bchecc(),
                self.wen()
            )
        }
    }
    #[doc = "PSRAM chip select counter register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcscntr(pub u32);
    impl Pcscntr {
        #[doc = "Chip select counter."]
        #[must_use]
        #[inline(always)]
        pub const fn cscount(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Chip select counter."]
        #[inline(always)]
        pub const fn set_cscount(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Counter Bank 1-4 enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cntben(&self, n: usize) -> bool {
            assert!(n < 4usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Counter Bank 1-4 enable."]
        #[inline(always)]
        pub const fn set_cntben(&mut self, n: usize, val: bool) {
            assert!(n < 4usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Pcscntr {
        #[inline(always)]
        fn default() -> Pcscntr {
            Pcscntr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Pcscntr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pcscntr")
                .field("cscount", &self.cscount())
                .field("cntben[0]", &self.cntben(0usize))
                .field("cntben[1]", &self.cntben(1usize))
                .field("cntben[2]", &self.cntben(2usize))
                .field("cntben[3]", &self.cntben(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pcscntr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pcscntr {{ cscount: {=u16:?}, cntben[0]: {=bool:?}, cntben[1]: {=bool:?}, cntben[2]: {=bool:?}, cntben[3]: {=bool:?} }}",
                self.cscount(),
                self.cntben(0usize),
                self.cntben(1usize),
                self.cntben(2usize),
                self.cntben(3usize)
            )
        }
    }
    #[doc = "Common memory space timing register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pmem(pub u32);
    impl Pmem {
        #[doc = "Common memory x setup time."]
        #[must_use]
        #[inline(always)]
        pub const fn memset(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Common memory x setup time."]
        #[inline(always)]
        pub const fn set_memset(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Common memory wait time."]
        #[must_use]
        #[inline(always)]
        pub const fn memwait(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Common memory wait time."]
        #[inline(always)]
        pub const fn set_memwait(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Common memory hold time."]
        #[must_use]
        #[inline(always)]
        pub const fn memhold(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Common memory hold time."]
        #[inline(always)]
        pub const fn set_memhold(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Common memory x data bus Hi-Z time."]
        #[must_use]
        #[inline(always)]
        pub const fn memhiz(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Common memory x data bus Hi-Z time."]
        #[inline(always)]
        pub const fn set_memhiz(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Pmem {
        #[inline(always)]
        fn default() -> Pmem {
            Pmem(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Pmem {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pmem")
                .field("memset", &self.memset())
                .field("memwait", &self.memwait())
                .field("memhold", &self.memhold())
                .field("memhiz", &self.memhiz())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pmem {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pmem {{ memset: {=u8:?}, memwait: {=u8:?}, memhold: {=u8:?}, memhiz: {=u8:?} }}",
                self.memset(),
                self.memwait(),
                self.memhold(),
                self.memhiz()
            )
        }
    }
    #[doc = "Size ID register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sidr(pub u32);
    impl Sidr {
        #[doc = "Peripheral size identification."]
        #[must_use]
        #[inline(always)]
        pub const fn sid(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Peripheral size identification."]
        #[inline(always)]
        pub const fn set_sid(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Sidr {
        #[inline(always)]
        fn default() -> Sidr {
            Sidr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Sidr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sidr").field("sid", &self.sid()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sidr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Sidr {{ sid: {=u32:?} }}", self.sid())
        }
    }
    #[doc = "Status and interrupt register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Isolation state with respect to the AXI interface."]
        #[must_use]
        #[inline(always)]
        pub const fn isost(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Isolation state with respect to the AXI interface."]
        #[inline(always)]
        pub const fn set_isost(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Pipe Empty Flag."]
        #[must_use]
        #[inline(always)]
        pub const fn pef(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Pipe Empty Flag."]
        #[inline(always)]
        pub const fn set_pef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "NAND write request flag."]
        #[must_use]
        #[inline(always)]
        pub const fn nwrf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "NAND write request flag."]
        #[inline(always)]
        pub const fn set_nwrf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
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
                .field("isost", &self.isost())
                .field("pef", &self.pef())
                .field("nwrf", &self.nwrf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ isost: {=u8:?}, pef: {=bool:?}, nwrf: {=bool:?} }}",
                self.isost(),
                self.pef(),
                self.nwrf()
            )
        }
    }
    #[doc = "Version register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Verr(pub u32);
    impl Verr {
        #[doc = "Minor Revision number."]
        #[must_use]
        #[inline(always)]
        pub const fn minrev(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Minor Revision number."]
        #[inline(always)]
        pub const fn set_minrev(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Major Revision number."]
        #[must_use]
        #[inline(always)]
        pub const fn majrev(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Major Revision number."]
        #[inline(always)]
        pub const fn set_majrev(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Verr {
        #[inline(always)]
        fn default() -> Verr {
            Verr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Verr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Verr")
                .field("minrev", &self.minrev())
                .field("majrev", &self.majrev())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Verr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Verr {{ minrev: {=u8:?}, majrev: {=u8:?} }}",
                self.minrev(),
                self.majrev()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Accmod {
        #[doc = "Access mode A"]
        A = 0x0,
        #[doc = "Access mode B"]
        B = 0x01,
        #[doc = "Access mode C"]
        C = 0x02,
        #[doc = "Access mode D"]
        D = 0x03,
    }
    impl Accmod {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Accmod {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Accmod {
        #[inline(always)]
        fn from(val: u8) -> Accmod {
            Accmod::from_bits(val)
        }
    }
    impl From<Accmod> for u8 {
        #[inline(always)]
        fn from(val: Accmod) -> u8 {
            Accmod::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cburstrw {
        #[doc = "Write operations are always performed in Asynchronous mode."]
        Asynchronous = 0x0,
        #[doc = "Write operations are performed in Synchronous mode."]
        Synchronous = 0x01,
    }
    impl Cburstrw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cburstrw {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cburstrw {
        #[inline(always)]
        fn from(val: u8) -> Cburstrw {
            Cburstrw::from_bits(val)
        }
    }
    impl From<Cburstrw> for u8 {
        #[inline(always)]
        fn from(val: Cburstrw) -> u8 {
            Cburstrw::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cpsize {
        #[doc = "No burst split when crossing page boundary"]
        NoBurstSplit = 0x0,
        #[doc = "128 bytes CRAM page size"]
        Bytes128 = 0x01,
        #[doc = "256 bytes CRAM page size"]
        Bytes256 = 0x02,
        #[doc = "512 bytes CRAM page size"]
        Bytes512 = 0x03,
        #[doc = "1024 bytes CRAM page size"]
        Bytes1024 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Cpsize {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cpsize {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cpsize {
        #[inline(always)]
        fn from(val: u8) -> Cpsize {
            Cpsize::from_bits(val)
        }
    }
    impl From<Cpsize> for u8 {
        #[inline(always)]
        fn from(val: Cpsize) -> u8 {
            Cpsize::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Eccalg {
        #[doc = "Hamming algorithm"]
        Hamming = 0x0,
        #[doc = "BCH algorithm"]
        Bch = 0x01,
    }
    impl Eccalg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Eccalg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Eccalg {
        #[inline(always)]
        fn from(val: u8) -> Eccalg {
            Eccalg::from_bits(val)
        }
    }
    impl From<Eccalg> for u8 {
        #[inline(always)]
        fn from(val: Eccalg) -> u8 {
            Eccalg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Eccss {
        #[doc = "ECC page size 256 bytes"]
        Bytes256 = 0x0,
        #[doc = "ECC page size 512 bytes"]
        Bytes512 = 0x01,
        #[doc = "ECC page size 1024 bytes"]
        Bytes1024 = 0x02,
        #[doc = "ECC page size 2048 bytes"]
        Bytes2048 = 0x03,
        #[doc = "ECC page size 4096 bytes"]
        Bytes4096 = 0x04,
        #[doc = "ECC page size 8192 bytes"]
        Bytes8192 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Eccss {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Eccss {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Eccss {
        #[inline(always)]
        fn from(val: u8) -> Eccss {
            Eccss::from_bits(val)
        }
    }
    impl From<Eccss> for u8 {
        #[inline(always)]
        fn from(val: Eccss) -> u8 {
            Eccss::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mtyp {
        #[doc = "SRAM memory type"]
        Sram = 0x0,
        #[doc = "PSRAM (CRAM) memory type"]
        Psram = 0x01,
        #[doc = "NOR Flash/OneNAND Flash"]
        Flash = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Mtyp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mtyp {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mtyp {
        #[inline(always)]
        fn from(val: u8) -> Mtyp {
            Mtyp::from_bits(val)
        }
    }
    impl From<Mtyp> for u8 {
        #[inline(always)]
        fn from(val: Mtyp) -> u8 {
            Mtyp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Mwid {
        #[doc = "Memory data bus width 8 bits"]
        Bits8 = 0x0,
        #[doc = "Memory data bus width 16 bits"]
        Bits16 = 0x01,
        #[doc = "Memory data bus width 32 bits"]
        Bits32 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Mwid {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mwid {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Mwid {
        #[inline(always)]
        fn from(val: u8) -> Mwid {
            Mwid::from_bits(val)
        }
    }
    impl From<Mwid> for u8 {
        #[inline(always)]
        fn from(val: Mwid) -> u8 {
            Mwid::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Pwid {
        #[doc = "External memory device width 8 bits"]
        Bits8 = 0x0,
        #[doc = "External memory device width 16 bits"]
        Bits16 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Pwid {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pwid {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pwid {
        #[inline(always)]
        fn from(val: u8) -> Pwid {
            Pwid::from_bits(val)
        }
    }
    impl From<Pwid> for u8 {
        #[inline(always)]
        fn from(val: Pwid) -> u8 {
            Pwid::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Waitcfg {
        #[doc = "NWAIT signal is active one data cycle before wait state"]
        BeforeWaitState = 0x0,
        #[doc = "NWAIT signal is active during wait state"]
        DuringWaitState = 0x01,
    }
    impl Waitcfg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Waitcfg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Waitcfg {
        #[inline(always)]
        fn from(val: u8) -> Waitcfg {
            Waitcfg::from_bits(val)
        }
    }
    impl From<Waitcfg> for u8 {
        #[inline(always)]
        fn from(val: Waitcfg) -> u8 {
            Waitcfg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "debug", derive(Debug))]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Waitpol {
        #[doc = "NWAIT active low"]
        ActiveLow = 0x0,
        #[doc = "NWAIT active high"]
        ActiveHigh = 0x01,
    }
    impl Waitpol {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Waitpol {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Waitpol {
        #[inline(always)]
        fn from(val: u8) -> Waitpol {
            Waitpol::from_bits(val)
        }
    }
    impl From<Waitpol> for u8 {
        #[inline(always)]
        fn from(val: Waitpol) -> u8 {
            Waitpol::to_bits(val)
        }
    }
}
