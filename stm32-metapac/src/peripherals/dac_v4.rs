#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Digital-to-analog converter"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dac {
    ptr: *mut u8,
}
unsafe impl Send for Dac {}
unsafe impl Sync for Dac {}
impl Dac {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "software trigger register"]
    #[inline(always)]
    pub const fn swtrigr(self) -> crate::common::Reg<regs::Swtrigr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "channel 12-bit right-aligned data holding register"]
    #[inline(always)]
    pub const fn dhr12r(self, n: usize) -> crate::common::Reg<regs::Dhr12r, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 12usize) as _) }
    }
    #[doc = "channel 12-bit left-aligned data holding register"]
    #[inline(always)]
    pub const fn dhr12l(self, n: usize) -> crate::common::Reg<regs::Dhr12l, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize + n * 12usize) as _) }
    }
    #[doc = "channel 8-bit right-aligned data holding register"]
    #[inline(always)]
    pub const fn dhr8r(self, n: usize) -> crate::common::Reg<regs::Dhr8r, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize + n * 12usize) as _) }
    }
    #[doc = "dual 12-bit right-aligned data holding register"]
    #[inline(always)]
    pub const fn dhr12rd(self) -> crate::common::Reg<regs::Dhr12rd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "dual 12-bit left aligned data holding register"]
    #[inline(always)]
    pub const fn dhr12ld(self) -> crate::common::Reg<regs::Dhr12ld, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "dual 8-bit right aligned data holding register"]
    #[inline(always)]
    pub const fn dhr8rd(self) -> crate::common::Reg<regs::Dhr8rd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "channel data output register"]
    #[inline(always)]
    pub const fn dor(self, n: usize) -> crate::common::Reg<regs::Dor, crate::common::R> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize + n * 4usize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "calibration control register"]
    #[inline(always)]
    pub const fn ccr(self) -> crate::common::Reg<regs::Ccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "mode control register"]
    #[inline(always)]
    pub const fn mcr(self) -> crate::common::Reg<regs::Mcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "sample and hold sample time register"]
    #[inline(always)]
    pub const fn shsr(self, n: usize) -> crate::common::Reg<regs::Shsr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize + n * 4usize) as _) }
    }
    #[doc = "sample and hold hold time register"]
    #[inline(always)]
    pub const fn shhr(self) -> crate::common::Reg<regs::Shhr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "sample and hold refresh time register"]
    #[inline(always)]
    pub const fn shrr(self) -> crate::common::Reg<regs::Shrr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
}
pub mod regs {
    #[doc = "calibration control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ccr(pub u32);
    impl Ccr {
        #[doc = "channel offset trimming value"]
        #[inline(always)]
        pub const fn otrim(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 0usize + n * 16usize;
            let val = (self.0 >> offs) & 0x1f;
            val as u8
        }
        #[doc = "channel offset trimming value"]
        #[inline(always)]
        pub fn set_otrim(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 0usize + n * 16usize;
            self.0 = (self.0 & !(0x1f << offs)) | (((val as u32) & 0x1f) << offs);
        }
    }
    impl Default for Ccr {
        #[inline(always)]
        fn default() -> Ccr {
            Ccr(0)
        }
    }
    #[doc = "control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "channel enable"]
        #[inline(always)]
        pub const fn en(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 16usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "channel enable"]
        #[inline(always)]
        pub fn set_en(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 16usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "channel trigger enable"]
        #[inline(always)]
        pub const fn ten(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 1usize + n * 16usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "channel trigger enable"]
        #[inline(always)]
        pub fn set_ten(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 1usize + n * 16usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "channel trigger selection"]
        #[inline(always)]
        pub const fn tsel(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 2usize + n * 16usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "channel trigger selection"]
        #[inline(always)]
        pub fn set_tsel(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 2usize + n * 16usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
        #[doc = "channel noise/triangle wave generation enable"]
        #[inline(always)]
        pub const fn wave(&self, n: usize) -> super::vals::Wave {
            assert!(n < 2usize);
            let offs = 6usize + n * 16usize;
            let val = (self.0 >> offs) & 0x03;
            super::vals::Wave::from_bits(val as u8)
        }
        #[doc = "channel noise/triangle wave generation enable"]
        #[inline(always)]
        pub fn set_wave(&mut self, n: usize, val: super::vals::Wave) {
            assert!(n < 2usize);
            let offs = 6usize + n * 16usize;
            self.0 = (self.0 & !(0x03 << offs)) | (((val.to_bits() as u32) & 0x03) << offs);
        }
        #[doc = "channel mask/amplitude selector"]
        #[inline(always)]
        pub const fn mamp(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 8usize + n * 16usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "channel mask/amplitude selector"]
        #[inline(always)]
        pub fn set_mamp(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 8usize + n * 16usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
        #[doc = "channel DMA enable"]
        #[inline(always)]
        pub const fn dmaen(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 12usize + n * 16usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "channel DMA enable"]
        #[inline(always)]
        pub fn set_dmaen(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 12usize + n * 16usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "channel DMA Underrun Interrupt enable"]
        #[inline(always)]
        pub const fn dmaudrie(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 13usize + n * 16usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "channel DMA Underrun Interrupt enable"]
        #[inline(always)]
        pub fn set_dmaudrie(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 13usize + n * 16usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "DAC channel calibration enable"]
        #[inline(always)]
        pub const fn cen(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 14usize + n * 16usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "DAC channel calibration enable"]
        #[inline(always)]
        pub fn set_cen(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 14usize + n * 16usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    #[doc = "channel 12-bit left-aligned data holding register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhr12l(pub u32);
    impl Dhr12l {
        #[doc = "channel 12-bit left-aligned data"]
        #[inline(always)]
        pub const fn dhr(&self) -> u16 {
            let val = (self.0 >> 4usize) & 0x0fff;
            val as u16
        }
        #[doc = "channel 12-bit left-aligned data"]
        #[inline(always)]
        pub fn set_dhr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
        }
    }
    impl Default for Dhr12l {
        #[inline(always)]
        fn default() -> Dhr12l {
            Dhr12l(0)
        }
    }
    #[doc = "dual 12-bit left aligned data holding register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhr12ld(pub u32);
    impl Dhr12ld {
        #[doc = "channel 12-bit left-aligned data"]
        #[inline(always)]
        pub const fn dhr(&self, n: usize) -> u16 {
            assert!(n < 2usize);
            let offs = 4usize + n * 16usize;
            let val = (self.0 >> offs) & 0x0fff;
            val as u16
        }
        #[doc = "channel 12-bit left-aligned data"]
        #[inline(always)]
        pub fn set_dhr(&mut self, n: usize, val: u16) {
            assert!(n < 2usize);
            let offs = 4usize + n * 16usize;
            self.0 = (self.0 & !(0x0fff << offs)) | (((val as u32) & 0x0fff) << offs);
        }
    }
    impl Default for Dhr12ld {
        #[inline(always)]
        fn default() -> Dhr12ld {
            Dhr12ld(0)
        }
    }
    #[doc = "channel 12-bit right-aligned data holding register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhr12r(pub u32);
    impl Dhr12r {
        #[doc = "channel 12-bit right-aligned data"]
        #[inline(always)]
        pub const fn dhr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "channel 12-bit right-aligned data"]
        #[inline(always)]
        pub fn set_dhr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Dhr12r {
        #[inline(always)]
        fn default() -> Dhr12r {
            Dhr12r(0)
        }
    }
    #[doc = "dual 12-bit right-aligned data holding register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhr12rd(pub u32);
    impl Dhr12rd {
        #[doc = "channel 12-bit right-aligned data"]
        #[inline(always)]
        pub const fn dhr(&self, n: usize) -> u16 {
            assert!(n < 2usize);
            let offs = 0usize + n * 16usize;
            let val = (self.0 >> offs) & 0x0fff;
            val as u16
        }
        #[doc = "channel 12-bit right-aligned data"]
        #[inline(always)]
        pub fn set_dhr(&mut self, n: usize, val: u16) {
            assert!(n < 2usize);
            let offs = 0usize + n * 16usize;
            self.0 = (self.0 & !(0x0fff << offs)) | (((val as u32) & 0x0fff) << offs);
        }
    }
    impl Default for Dhr12rd {
        #[inline(always)]
        fn default() -> Dhr12rd {
            Dhr12rd(0)
        }
    }
    #[doc = "channel 8-bit right-aligned data holding register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhr8r(pub u32);
    impl Dhr8r {
        #[doc = "channel 8-bit right-aligned data"]
        #[inline(always)]
        pub const fn dhr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "channel 8-bit right-aligned data"]
        #[inline(always)]
        pub fn set_dhr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Dhr8r {
        #[inline(always)]
        fn default() -> Dhr8r {
            Dhr8r(0)
        }
    }
    #[doc = "dual 8-bit right aligned data holding register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhr8rd(pub u32);
    impl Dhr8rd {
        #[doc = "channel 8-bit right-aligned data"]
        #[inline(always)]
        pub const fn dhr(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0xff;
            val as u8
        }
        #[doc = "channel 8-bit right-aligned data"]
        #[inline(always)]
        pub fn set_dhr(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0xff << offs)) | (((val as u32) & 0xff) << offs);
        }
    }
    impl Default for Dhr8rd {
        #[inline(always)]
        fn default() -> Dhr8rd {
            Dhr8rd(0)
        }
    }
    #[doc = "channel data output register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dor(pub u32);
    impl Dor {
        #[doc = "channel data output"]
        #[inline(always)]
        pub const fn dor(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "channel data output"]
        #[inline(always)]
        pub fn set_dor(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
    }
    impl Default for Dor {
        #[inline(always)]
        fn default() -> Dor {
            Dor(0)
        }
    }
    #[doc = "mode control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mcr(pub u32);
    impl Mcr {
        #[doc = "DAC channel mode"]
        #[inline(always)]
        pub const fn mode(&self, n: usize) -> super::vals::Mode {
            assert!(n < 2usize);
            let offs = 0usize + n * 16usize;
            let val = (self.0 >> offs) & 0x07;
            super::vals::Mode::from_bits(val as u8)
        }
        #[doc = "DAC channel mode"]
        #[inline(always)]
        pub fn set_mode(&mut self, n: usize, val: super::vals::Mode) {
            assert!(n < 2usize);
            let offs = 0usize + n * 16usize;
            self.0 = (self.0 & !(0x07 << offs)) | (((val.to_bits() as u32) & 0x07) << offs);
        }
    }
    impl Default for Mcr {
        #[inline(always)]
        fn default() -> Mcr {
            Mcr(0)
        }
    }
    #[doc = "sample and hold hold time register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Shhr(pub u32);
    impl Shhr {
        #[doc = "channel hold time"]
        #[inline(always)]
        pub const fn thold(&self, n: usize) -> u16 {
            assert!(n < 2usize);
            let offs = 0usize + n * 16usize;
            let val = (self.0 >> offs) & 0x03ff;
            val as u16
        }
        #[doc = "channel hold time"]
        #[inline(always)]
        pub fn set_thold(&mut self, n: usize, val: u16) {
            assert!(n < 2usize);
            let offs = 0usize + n * 16usize;
            self.0 = (self.0 & !(0x03ff << offs)) | (((val as u32) & 0x03ff) << offs);
        }
    }
    impl Default for Shhr {
        #[inline(always)]
        fn default() -> Shhr {
            Shhr(0)
        }
    }
    #[doc = "sample and hold refresh time register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Shrr(pub u32);
    impl Shrr {
        #[doc = "channel refresh time"]
        #[inline(always)]
        pub const fn trefresh(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 0usize + n * 16usize;
            let val = (self.0 >> offs) & 0xff;
            val as u8
        }
        #[doc = "channel refresh time"]
        #[inline(always)]
        pub fn set_trefresh(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 0usize + n * 16usize;
            self.0 = (self.0 & !(0xff << offs)) | (((val as u32) & 0xff) << offs);
        }
    }
    impl Default for Shrr {
        #[inline(always)]
        fn default() -> Shrr {
            Shrr(0)
        }
    }
    #[doc = "sample and hold sample time register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Shsr(pub u32);
    impl Shsr {
        #[doc = "channel sample time"]
        #[inline(always)]
        pub const fn tsample(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "channel sample time"]
        #[inline(always)]
        pub fn set_tsample(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
    }
    impl Default for Shsr {
        #[inline(always)]
        fn default() -> Shsr {
            Shsr(0)
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "channel DMA underrun flag"]
        #[inline(always)]
        pub const fn dmaudr(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 13usize + n * 16usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "channel DMA underrun flag"]
        #[inline(always)]
        pub fn set_dmaudr(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 13usize + n * 16usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "channel calibration offset status"]
        #[inline(always)]
        pub const fn cal_flag(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 14usize + n * 16usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "channel calibration offset status"]
        #[inline(always)]
        pub fn set_cal_flag(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 14usize + n * 16usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "channel busy writing sample time flag"]
        #[inline(always)]
        pub const fn bwst(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 15usize + n * 16usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "channel busy writing sample time flag"]
        #[inline(always)]
        pub fn set_bwst(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 15usize + n * 16usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
    #[doc = "software trigger register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Swtrigr(pub u32);
    impl Swtrigr {
        #[doc = "channel software trigger"]
        #[inline(always)]
        pub const fn swtrig(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "channel software trigger"]
        #[inline(always)]
        pub fn set_swtrig(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Swtrigr {
        #[inline(always)]
        fn default() -> Swtrigr {
            Swtrigr(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Mode {
        #[doc = "Normal mode, external pin only, buffer enabled"]
        NORMAL_EXT_BUFEN = 0x0,
        #[doc = "Normal mode, external pin and internal peripherals, buffer enabled"]
        NORMAL_EXT_INT_BUFEN = 0x01,
        #[doc = "Normal mode, external pin only, buffer disabled"]
        NORMAL_EXT_BUFDIS = 0x02,
        #[doc = "Normal mode, internal peripherals only, buffer disabled"]
        NORMAL_INT_BUFDIS = 0x03,
        #[doc = "Sample and hold mode, external pin only, buffer enabled"]
        SAMPHOLD_EXT_BUFEN = 0x04,
        #[doc = "Sample and hold mode, external pin and internal peripherals, buffer enabled"]
        SAMPHOLD_EXT_INT_BUFEN = 0x05,
        #[doc = "Sample and hold mode, external pin and internal peripherals, buffer disabled"]
        SAMPHOLD_EXT_INT_BUFDIS = 0x06,
        #[doc = "Sample and hold mode, internal peripherals only, buffer disabled"]
        SAMPHOLD_INT_BUFDIS = 0x07,
    }
    impl Mode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Mode {
            unsafe { core::mem::transmute(val & 0x07) }
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
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Wave {
        #[doc = "Wave generation disabled"]
        DISABLED = 0x0,
        #[doc = "Noise wave generation enabled"]
        NOISE = 0x01,
        #[doc = "Triangle wave generation enabled"]
        TRIANGLE = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Wave {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wave {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wave {
        #[inline(always)]
        fn from(val: u8) -> Wave {
            Wave::from_bits(val)
        }
    }
    impl From<Wave> for u8 {
        #[inline(always)]
        fn from(val: Wave) -> u8 {
            Wave::to_bits(val)
        }
    }
}
