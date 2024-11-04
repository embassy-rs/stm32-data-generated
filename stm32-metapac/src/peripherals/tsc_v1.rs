#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Touch sensing controller."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsc {
    ptr: *mut u8,
}
unsafe impl Send for Tsc {}
unsafe impl Sync for Tsc {}
impl Tsc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "interrupt enable register."]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "interrupt clear register."]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "interrupt status register."]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "I/O hysteresis control register."]
    #[inline(always)]
    pub const fn iohcr(self) -> crate::common::Reg<regs::Iohcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "I/O analog switch control register."]
    #[inline(always)]
    pub const fn ioascr(self) -> crate::common::Reg<regs::Ioascr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "I/O sampling control register."]
    #[inline(always)]
    pub const fn ioscr(self) -> crate::common::Reg<regs::Ioscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "I/O channel control register."]
    #[inline(always)]
    pub const fn ioccr(self) -> crate::common::Reg<regs::Ioccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "I/O group control status register."]
    #[inline(always)]
    pub const fn iogcsr(self) -> crate::common::Reg<regs::Iogcsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "I/O group x counter register."]
    #[inline(always)]
    pub const fn iogcr(self, n: usize) -> crate::common::Reg<regs::Iogcr, crate::common::R> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Touch sensing controller enable."]
        #[inline(always)]
        pub const fn tsce(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Touch sensing controller enable."]
        #[inline(always)]
        pub fn set_tsce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Start a new acquisition."]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Start a new acquisition."]
        #[inline(always)]
        pub fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Acquisition mode."]
        #[inline(always)]
        pub const fn am(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Acquisition mode."]
        #[inline(always)]
        pub fn set_am(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Synchronization pin polarity."]
        #[inline(always)]
        pub const fn syncpol(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronization pin polarity."]
        #[inline(always)]
        pub fn set_syncpol(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "I/O Default mode."]
        #[inline(always)]
        pub const fn iodef(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "I/O Default mode."]
        #[inline(always)]
        pub fn set_iodef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Max count value."]
        #[inline(always)]
        pub const fn mcv(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x07;
            val as u8
        }
        #[doc = "Max count value."]
        #[inline(always)]
        pub fn set_mcv(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
        }
        #[doc = "pulse generator prescaler."]
        #[inline(always)]
        pub const fn pgpsc(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "pulse generator prescaler."]
        #[inline(always)]
        pub fn set_pgpsc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "Spread spectrum prescaler."]
        #[inline(always)]
        pub const fn sspsc(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Spread spectrum prescaler."]
        #[inline(always)]
        pub fn set_sspsc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Spread spectrum enable."]
        #[inline(always)]
        pub const fn sse(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Spread spectrum enable."]
        #[inline(always)]
        pub fn set_sse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Spread spectrum deviation."]
        #[inline(always)]
        pub const fn ssd(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x7f;
            val as u8
        }
        #[doc = "Spread spectrum deviation."]
        #[inline(always)]
        pub fn set_ssd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 17usize)) | (((val as u32) & 0x7f) << 17usize);
        }
        #[doc = "Charge transfer pulse low."]
        #[inline(always)]
        pub const fn ctpl(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "Charge transfer pulse low."]
        #[inline(always)]
        pub fn set_ctpl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "Charge transfer pulse high."]
        #[inline(always)]
        pub const fn ctph(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "Charge transfer pulse high."]
        #[inline(always)]
        pub fn set_ctph(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    #[doc = "interrupt clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "End of acquisition interrupt clear."]
        #[inline(always)]
        pub const fn eoaic(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "End of acquisition interrupt clear."]
        #[inline(always)]
        pub fn set_eoaic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Max count error interrupt clear."]
        #[inline(always)]
        pub const fn mceic(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Max count error interrupt clear."]
        #[inline(always)]
        pub fn set_mceic(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Icr {
        #[inline(always)]
        fn default() -> Icr {
            Icr(0)
        }
    }
    #[doc = "interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "End of acquisition interrupt enable."]
        #[inline(always)]
        pub const fn eoaie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "End of acquisition interrupt enable."]
        #[inline(always)]
        pub fn set_eoaie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Max count error interrupt enable."]
        #[inline(always)]
        pub const fn mceie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Max count error interrupt enable."]
        #[inline(always)]
        pub fn set_mceie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Ier {
        #[inline(always)]
        fn default() -> Ier {
            Ier(0)
        }
    }
    #[doc = "I/O analog switch control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ioascr(pub u32);
    impl Ioascr {
        #[doc = "G1_IO1 analog switch enable."]
        #[inline(always)]
        pub const fn g1_io1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "G1_IO1 analog switch enable."]
        #[inline(always)]
        pub fn set_g1_io1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "G1_IO2 analog switch enable."]
        #[inline(always)]
        pub const fn g1_io2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "G1_IO2 analog switch enable."]
        #[inline(always)]
        pub fn set_g1_io2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "G1_IO3 analog switch enable."]
        #[inline(always)]
        pub const fn g1_io3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "G1_IO3 analog switch enable."]
        #[inline(always)]
        pub fn set_g1_io3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "G1_IO4 analog switch enable."]
        #[inline(always)]
        pub const fn g1_io4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "G1_IO4 analog switch enable."]
        #[inline(always)]
        pub fn set_g1_io4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "G2_IO1 analog switch enable."]
        #[inline(always)]
        pub const fn g2_io1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "G2_IO1 analog switch enable."]
        #[inline(always)]
        pub fn set_g2_io1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "G2_IO2 analog switch enable."]
        #[inline(always)]
        pub const fn g2_io2(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "G2_IO2 analog switch enable."]
        #[inline(always)]
        pub fn set_g2_io2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "G2_IO3 analog switch enable."]
        #[inline(always)]
        pub const fn g2_io3(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "G2_IO3 analog switch enable."]
        #[inline(always)]
        pub fn set_g2_io3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "G2_IO4 analog switch enable."]
        #[inline(always)]
        pub const fn g2_io4(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "G2_IO4 analog switch enable."]
        #[inline(always)]
        pub fn set_g2_io4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "G3_IO1 analog switch enable."]
        #[inline(always)]
        pub const fn g3_io1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "G3_IO1 analog switch enable."]
        #[inline(always)]
        pub fn set_g3_io1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "G3_IO2 analog switch enable."]
        #[inline(always)]
        pub const fn g3_io2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "G3_IO2 analog switch enable."]
        #[inline(always)]
        pub fn set_g3_io2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "G3_IO3 analog switch enable."]
        #[inline(always)]
        pub const fn g3_io3(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "G3_IO3 analog switch enable."]
        #[inline(always)]
        pub fn set_g3_io3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "G3_IO4 analog switch enable."]
        #[inline(always)]
        pub const fn g3_io4(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "G3_IO4 analog switch enable."]
        #[inline(always)]
        pub fn set_g3_io4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "G4_IO1 analog switch enable."]
        #[inline(always)]
        pub const fn g4_io1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "G4_IO1 analog switch enable."]
        #[inline(always)]
        pub fn set_g4_io1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "G4_IO2 analog switch enable."]
        #[inline(always)]
        pub const fn g4_io2(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "G4_IO2 analog switch enable."]
        #[inline(always)]
        pub fn set_g4_io2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "G4_IO3 analog switch enable."]
        #[inline(always)]
        pub const fn g4_io3(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "G4_IO3 analog switch enable."]
        #[inline(always)]
        pub fn set_g4_io3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "G4_IO4 analog switch enable."]
        #[inline(always)]
        pub const fn g4_io4(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "G4_IO4 analog switch enable."]
        #[inline(always)]
        pub fn set_g4_io4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "G5_IO1 analog switch enable."]
        #[inline(always)]
        pub const fn g5_io1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "G5_IO1 analog switch enable."]
        #[inline(always)]
        pub fn set_g5_io1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "G5_IO2 analog switch enable."]
        #[inline(always)]
        pub const fn g5_io2(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "G5_IO2 analog switch enable."]
        #[inline(always)]
        pub fn set_g5_io2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "G5_IO3 analog switch enable."]
        #[inline(always)]
        pub const fn g5_io3(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "G5_IO3 analog switch enable."]
        #[inline(always)]
        pub fn set_g5_io3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "G5_IO4 analog switch enable."]
        #[inline(always)]
        pub const fn g5_io4(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "G5_IO4 analog switch enable."]
        #[inline(always)]
        pub fn set_g5_io4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "G6_IO1 analog switch enable."]
        #[inline(always)]
        pub const fn g6_io1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "G6_IO1 analog switch enable."]
        #[inline(always)]
        pub fn set_g6_io1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "G6_IO2 analog switch enable."]
        #[inline(always)]
        pub const fn g6_io2(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "G6_IO2 analog switch enable."]
        #[inline(always)]
        pub fn set_g6_io2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "G6_IO3 analog switch enable."]
        #[inline(always)]
        pub const fn g6_io3(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "G6_IO3 analog switch enable."]
        #[inline(always)]
        pub fn set_g6_io3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "G6_IO4 analog switch enable."]
        #[inline(always)]
        pub const fn g6_io4(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "G6_IO4 analog switch enable."]
        #[inline(always)]
        pub fn set_g6_io4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Ioascr {
        #[inline(always)]
        fn default() -> Ioascr {
            Ioascr(0)
        }
    }
    #[doc = "I/O channel control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ioccr(pub u32);
    impl Ioccr {
        #[doc = "G1_IO1 channel mode."]
        #[inline(always)]
        pub const fn g1_io1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "G1_IO1 channel mode."]
        #[inline(always)]
        pub fn set_g1_io1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "G1_IO2 channel mode."]
        #[inline(always)]
        pub const fn g1_io2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "G1_IO2 channel mode."]
        #[inline(always)]
        pub fn set_g1_io2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "G1_IO3 channel mode."]
        #[inline(always)]
        pub const fn g1_io3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "G1_IO3 channel mode."]
        #[inline(always)]
        pub fn set_g1_io3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "G1_IO4 channel mode."]
        #[inline(always)]
        pub const fn g1_io4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "G1_IO4 channel mode."]
        #[inline(always)]
        pub fn set_g1_io4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "G2_IO1 channel mode."]
        #[inline(always)]
        pub const fn g2_io1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "G2_IO1 channel mode."]
        #[inline(always)]
        pub fn set_g2_io1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "G2_IO2 channel mode."]
        #[inline(always)]
        pub const fn g2_io2(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "G2_IO2 channel mode."]
        #[inline(always)]
        pub fn set_g2_io2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "G2_IO3 channel mode."]
        #[inline(always)]
        pub const fn g2_io3(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "G2_IO3 channel mode."]
        #[inline(always)]
        pub fn set_g2_io3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "G2_IO4 channel mode."]
        #[inline(always)]
        pub const fn g2_io4(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "G2_IO4 channel mode."]
        #[inline(always)]
        pub fn set_g2_io4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "G3_IO1 channel mode."]
        #[inline(always)]
        pub const fn g3_io1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "G3_IO1 channel mode."]
        #[inline(always)]
        pub fn set_g3_io1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "G3_IO2 channel mode."]
        #[inline(always)]
        pub const fn g3_io2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "G3_IO2 channel mode."]
        #[inline(always)]
        pub fn set_g3_io2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "G3_IO3 channel mode."]
        #[inline(always)]
        pub const fn g3_io3(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "G3_IO3 channel mode."]
        #[inline(always)]
        pub fn set_g3_io3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "G3_IO4 channel mode."]
        #[inline(always)]
        pub const fn g3_io4(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "G3_IO4 channel mode."]
        #[inline(always)]
        pub fn set_g3_io4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "G4_IO1 channel mode."]
        #[inline(always)]
        pub const fn g4_io1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "G4_IO1 channel mode."]
        #[inline(always)]
        pub fn set_g4_io1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "G4_IO2 channel mode."]
        #[inline(always)]
        pub const fn g4_io2(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "G4_IO2 channel mode."]
        #[inline(always)]
        pub fn set_g4_io2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "G4_IO3 channel mode."]
        #[inline(always)]
        pub const fn g4_io3(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "G4_IO3 channel mode."]
        #[inline(always)]
        pub fn set_g4_io3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "G4_IO4 channel mode."]
        #[inline(always)]
        pub const fn g4_io4(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "G4_IO4 channel mode."]
        #[inline(always)]
        pub fn set_g4_io4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "G5_IO1 channel mode."]
        #[inline(always)]
        pub const fn g5_io1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "G5_IO1 channel mode."]
        #[inline(always)]
        pub fn set_g5_io1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "G5_IO2 channel mode."]
        #[inline(always)]
        pub const fn g5_io2(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "G5_IO2 channel mode."]
        #[inline(always)]
        pub fn set_g5_io2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "G5_IO3 channel mode."]
        #[inline(always)]
        pub const fn g5_io3(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "G5_IO3 channel mode."]
        #[inline(always)]
        pub fn set_g5_io3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "G5_IO4 channel mode."]
        #[inline(always)]
        pub const fn g5_io4(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "G5_IO4 channel mode."]
        #[inline(always)]
        pub fn set_g5_io4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "G6_IO1 channel mode."]
        #[inline(always)]
        pub const fn g6_io1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "G6_IO1 channel mode."]
        #[inline(always)]
        pub fn set_g6_io1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "G6_IO2 channel mode."]
        #[inline(always)]
        pub const fn g6_io2(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "G6_IO2 channel mode."]
        #[inline(always)]
        pub fn set_g6_io2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "G6_IO3 channel mode."]
        #[inline(always)]
        pub const fn g6_io3(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "G6_IO3 channel mode."]
        #[inline(always)]
        pub fn set_g6_io3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "G6_IO4 channel mode."]
        #[inline(always)]
        pub const fn g6_io4(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "G6_IO4 channel mode."]
        #[inline(always)]
        pub fn set_g6_io4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Ioccr {
        #[inline(always)]
        fn default() -> Ioccr {
            Ioccr(0)
        }
    }
    #[doc = "I/O group x counter register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iogcr(pub u32);
    impl Iogcr {
        #[doc = "Counter value."]
        #[inline(always)]
        pub const fn cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "Counter value."]
        #[inline(always)]
        pub fn set_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
    }
    impl Default for Iogcr {
        #[inline(always)]
        fn default() -> Iogcr {
            Iogcr(0)
        }
    }
    #[doc = "I/O group control status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iogcsr(pub u32);
    impl Iogcsr {
        #[doc = "Analog I/O group x enable."]
        #[inline(always)]
        pub const fn g1e(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Analog I/O group x enable."]
        #[inline(always)]
        pub fn set_g1e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Analog I/O group x enable."]
        #[inline(always)]
        pub const fn g2e(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Analog I/O group x enable."]
        #[inline(always)]
        pub fn set_g2e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Analog I/O group x enable."]
        #[inline(always)]
        pub const fn g3e(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Analog I/O group x enable."]
        #[inline(always)]
        pub fn set_g3e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Analog I/O group x enable."]
        #[inline(always)]
        pub const fn g4e(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Analog I/O group x enable."]
        #[inline(always)]
        pub fn set_g4e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Analog I/O group x enable."]
        #[inline(always)]
        pub const fn g5e(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Analog I/O group x enable."]
        #[inline(always)]
        pub fn set_g5e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Analog I/O group x enable."]
        #[inline(always)]
        pub const fn g6e(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Analog I/O group x enable."]
        #[inline(always)]
        pub fn set_g6e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Analog I/O group x enable."]
        #[inline(always)]
        pub const fn g7e(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Analog I/O group x enable."]
        #[inline(always)]
        pub fn set_g7e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Analog I/O group x enable."]
        #[inline(always)]
        pub const fn g8e(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Analog I/O group x enable."]
        #[inline(always)]
        pub fn set_g8e(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Analog I/O group x status."]
        #[inline(always)]
        pub const fn g1s(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Analog I/O group x status."]
        #[inline(always)]
        pub fn set_g1s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Analog I/O group x status."]
        #[inline(always)]
        pub const fn g2s(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Analog I/O group x status."]
        #[inline(always)]
        pub fn set_g2s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Analog I/O group x status."]
        #[inline(always)]
        pub const fn g3s(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Analog I/O group x status."]
        #[inline(always)]
        pub fn set_g3s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Analog I/O group x status."]
        #[inline(always)]
        pub const fn g4s(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Analog I/O group x status."]
        #[inline(always)]
        pub fn set_g4s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Analog I/O group x status."]
        #[inline(always)]
        pub const fn g5s(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Analog I/O group x status."]
        #[inline(always)]
        pub fn set_g5s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Analog I/O group x status."]
        #[inline(always)]
        pub const fn g6s(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Analog I/O group x status."]
        #[inline(always)]
        pub fn set_g6s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Analog I/O group x status."]
        #[inline(always)]
        pub const fn g7s(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Analog I/O group x status."]
        #[inline(always)]
        pub fn set_g7s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Analog I/O group x status."]
        #[inline(always)]
        pub const fn g8s(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Analog I/O group x status."]
        #[inline(always)]
        pub fn set_g8s(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Iogcsr {
        #[inline(always)]
        fn default() -> Iogcsr {
            Iogcsr(0)
        }
    }
    #[doc = "I/O hysteresis control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iohcr(pub u32);
    impl Iohcr {
        #[doc = "G1_IO1 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub const fn g1_io1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "G1_IO1 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub fn set_g1_io1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "G1_IO2 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub const fn g1_io2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "G1_IO2 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub fn set_g1_io2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "G1_IO3 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub const fn g1_io3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "G1_IO3 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub fn set_g1_io3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "G1_IO4 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub const fn g1_io4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "G1_IO4 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub fn set_g1_io4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "G2_IO1 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub const fn g2_io1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "G2_IO1 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub fn set_g2_io1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "G2_IO2 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub const fn g2_io2(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "G2_IO2 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub fn set_g2_io2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "G2_IO3 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub const fn g2_io3(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "G2_IO3 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub fn set_g2_io3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "G2_IO4 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub const fn g2_io4(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "G2_IO4 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub fn set_g2_io4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "G3_IO1 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub const fn g3_io1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "G3_IO1 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub fn set_g3_io1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "G3_IO2 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub const fn g3_io2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "G3_IO2 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub fn set_g3_io2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "G3_IO3 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub const fn g3_io3(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "G3_IO3 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub fn set_g3_io3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "G3_IO4 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub const fn g3_io4(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "G3_IO4 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub fn set_g3_io4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "G4_IO1 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub const fn g4_io1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "G4_IO1 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub fn set_g4_io1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "G4_IO2 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub const fn g4_io2(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "G4_IO2 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub fn set_g4_io2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "G4_IO3 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub const fn g4_io3(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "G4_IO3 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub fn set_g4_io3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "G4_IO4 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub const fn g4_io4(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "G4_IO4 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub fn set_g4_io4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "G5_IO1 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub const fn g5_io1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "G5_IO1 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub fn set_g5_io1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "G5_IO2 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub const fn g5_io2(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "G5_IO2 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub fn set_g5_io2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "G5_IO3 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub const fn g5_io3(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "G5_IO3 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub fn set_g5_io3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "G5_IO4 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub const fn g5_io4(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "G5_IO4 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub fn set_g5_io4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "G6_IO1 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub const fn g6_io1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "G6_IO1 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub fn set_g6_io1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "G6_IO2 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub const fn g6_io2(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "G6_IO2 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub fn set_g6_io2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "G6_IO3 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub const fn g6_io3(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "G6_IO3 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub fn set_g6_io3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "G6_IO4 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub const fn g6_io4(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "G6_IO4 Schmitt trigger hysteresis mode."]
        #[inline(always)]
        pub fn set_g6_io4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Iohcr {
        #[inline(always)]
        fn default() -> Iohcr {
            Iohcr(0)
        }
    }
    #[doc = "I/O sampling control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ioscr(pub u32);
    impl Ioscr {
        #[doc = "G1_IO1 sampling mode."]
        #[inline(always)]
        pub const fn g1_io1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "G1_IO1 sampling mode."]
        #[inline(always)]
        pub fn set_g1_io1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "G1_IO2 sampling mode."]
        #[inline(always)]
        pub const fn g1_io2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "G1_IO2 sampling mode."]
        #[inline(always)]
        pub fn set_g1_io2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "G1_IO3 sampling mode."]
        #[inline(always)]
        pub const fn g1_io3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "G1_IO3 sampling mode."]
        #[inline(always)]
        pub fn set_g1_io3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "G1_IO4 sampling mode."]
        #[inline(always)]
        pub const fn g1_io4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "G1_IO4 sampling mode."]
        #[inline(always)]
        pub fn set_g1_io4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "G2_IO1 sampling mode."]
        #[inline(always)]
        pub const fn g2_io1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "G2_IO1 sampling mode."]
        #[inline(always)]
        pub fn set_g2_io1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "G2_IO2 sampling mode."]
        #[inline(always)]
        pub const fn g2_io2(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "G2_IO2 sampling mode."]
        #[inline(always)]
        pub fn set_g2_io2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "G2_IO3 sampling mode."]
        #[inline(always)]
        pub const fn g2_io3(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "G2_IO3 sampling mode."]
        #[inline(always)]
        pub fn set_g2_io3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "G2_IO4 sampling mode."]
        #[inline(always)]
        pub const fn g2_io4(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "G2_IO4 sampling mode."]
        #[inline(always)]
        pub fn set_g2_io4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "G3_IO1 sampling mode."]
        #[inline(always)]
        pub const fn g3_io1(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "G3_IO1 sampling mode."]
        #[inline(always)]
        pub fn set_g3_io1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "G3_IO2 sampling mode."]
        #[inline(always)]
        pub const fn g3_io2(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "G3_IO2 sampling mode."]
        #[inline(always)]
        pub fn set_g3_io2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "G3_IO3 sampling mode."]
        #[inline(always)]
        pub const fn g3_io3(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "G3_IO3 sampling mode."]
        #[inline(always)]
        pub fn set_g3_io3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "G3_IO4 sampling mode."]
        #[inline(always)]
        pub const fn g3_io4(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "G3_IO4 sampling mode."]
        #[inline(always)]
        pub fn set_g3_io4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "G4_IO1 sampling mode."]
        #[inline(always)]
        pub const fn g4_io1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "G4_IO1 sampling mode."]
        #[inline(always)]
        pub fn set_g4_io1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "G4_IO2 sampling mode."]
        #[inline(always)]
        pub const fn g4_io2(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "G4_IO2 sampling mode."]
        #[inline(always)]
        pub fn set_g4_io2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "G4_IO3 sampling mode."]
        #[inline(always)]
        pub const fn g4_io3(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "G4_IO3 sampling mode."]
        #[inline(always)]
        pub fn set_g4_io3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "G4_IO4 sampling mode."]
        #[inline(always)]
        pub const fn g4_io4(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "G4_IO4 sampling mode."]
        #[inline(always)]
        pub fn set_g4_io4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "G5_IO1 sampling mode."]
        #[inline(always)]
        pub const fn g5_io1(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "G5_IO1 sampling mode."]
        #[inline(always)]
        pub fn set_g5_io1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "G5_IO2 sampling mode."]
        #[inline(always)]
        pub const fn g5_io2(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "G5_IO2 sampling mode."]
        #[inline(always)]
        pub fn set_g5_io2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "G5_IO3 sampling mode."]
        #[inline(always)]
        pub const fn g5_io3(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "G5_IO3 sampling mode."]
        #[inline(always)]
        pub fn set_g5_io3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "G5_IO4 sampling mode."]
        #[inline(always)]
        pub const fn g5_io4(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "G5_IO4 sampling mode."]
        #[inline(always)]
        pub fn set_g5_io4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "G6_IO1 sampling mode."]
        #[inline(always)]
        pub const fn g6_io1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "G6_IO1 sampling mode."]
        #[inline(always)]
        pub fn set_g6_io1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "G6_IO2 sampling mode."]
        #[inline(always)]
        pub const fn g6_io2(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "G6_IO2 sampling mode."]
        #[inline(always)]
        pub fn set_g6_io2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "G6_IO3 sampling mode."]
        #[inline(always)]
        pub const fn g6_io3(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "G6_IO3 sampling mode."]
        #[inline(always)]
        pub fn set_g6_io3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "G6_IO4 sampling mode."]
        #[inline(always)]
        pub const fn g6_io4(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "G6_IO4 sampling mode."]
        #[inline(always)]
        pub fn set_g6_io4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Ioscr {
        #[inline(always)]
        fn default() -> Ioscr {
            Ioscr(0)
        }
    }
    #[doc = "interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "End of acquisition flag."]
        #[inline(always)]
        pub const fn eoaf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "End of acquisition flag."]
        #[inline(always)]
        pub fn set_eoaf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Max count error flag."]
        #[inline(always)]
        pub const fn mcef(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Max count error flag."]
        #[inline(always)]
        pub fn set_mcef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Isr {
        #[inline(always)]
        fn default() -> Isr {
            Isr(0)
        }
    }
}
