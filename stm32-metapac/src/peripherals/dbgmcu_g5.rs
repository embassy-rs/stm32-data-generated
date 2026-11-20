#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Microcontroller debug unit."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgmcu {
    ptr: *mut u8,
}
unsafe impl Send for Dbgmcu {}
unsafe impl Sync for Dbgmcu {}
impl Dbgmcu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DBGMCU identity code register."]
    #[inline(always)]
    pub const fn idcode(self) -> crate::common::Reg<regs::Idcode, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "DBGMCU configuration register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "DBGMCU APB1 peripheral freeze register."]
    #[inline(always)]
    pub const fn apb1fzr(self) -> crate::common::Reg<regs::Apb1fzr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "DBGMCU APB2 peripheral freeze register."]
    #[inline(always)]
    pub const fn apb2fzr(self) -> crate::common::Reg<regs::Apb2fzr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "DBGMCU APB3 peripheral freeze register."]
    #[inline(always)]
    pub const fn apb3fzr(self) -> crate::common::Reg<regs::Apb3fzr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "DBGMCU AHB1 peripheral freeze register."]
    #[inline(always)]
    pub const fn ahb1fzr(self) -> crate::common::Reg<regs::Ahb1fzr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "DBGMCU EDBGRQ debug request control."]
    #[inline(always)]
    pub const fn edbgrq(self) -> crate::common::Reg<regs::Edbgrq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "DBGMCU status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "DBGMCU debug authentication mailbox host register."]
    #[inline(always)]
    pub const fn auth_host(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "DBGMCU debug authentication mailbox device register."]
    #[inline(always)]
    pub const fn auth_device(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "DBGMCU debug authentication mailbox acknowledge register."]
    #[inline(always)]
    pub const fn auth_ack(self) -> crate::common::Reg<regs::AuthAck, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "DBGMCU CoreSight peripheral identity register 4."]
    #[inline(always)]
    pub const fn pidr4(self) -> crate::common::Reg<regs::Pidr4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd0usize) as _) }
    }
    #[doc = "DBGMCU CoreSight peripheral identity register 0."]
    #[inline(always)]
    pub const fn pidr0(self) -> crate::common::Reg<regs::Pidr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe0usize) as _) }
    }
    #[doc = "DBGMCU CoreSight peripheral identity register 1."]
    #[inline(always)]
    pub const fn pidr1(self) -> crate::common::Reg<regs::Pidr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe4usize) as _) }
    }
    #[doc = "DBGMCU CoreSight peripheral identity register 2."]
    #[inline(always)]
    pub const fn pidr2(self) -> crate::common::Reg<regs::Pidr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe8usize) as _) }
    }
    #[doc = "DBGMCU CoreSight peripheral identity register 3."]
    #[inline(always)]
    pub const fn pidr3(self) -> crate::common::Reg<regs::Pidr3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fecusize) as _) }
    }
    #[doc = "DBGMCU CoreSight component identity register 0."]
    #[inline(always)]
    pub const fn cidr0(self) -> crate::common::Reg<regs::Cidr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff0usize) as _) }
    }
    #[doc = "DBGMCU CoreSight component identity register 1."]
    #[inline(always)]
    pub const fn cidr1(self) -> crate::common::Reg<regs::Cidr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff4usize) as _) }
    }
    #[doc = "DBGMCU CoreSight component identity register 2."]
    #[inline(always)]
    pub const fn cidr2(self) -> crate::common::Reg<regs::Cidr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff8usize) as _) }
    }
    #[doc = "DBGMCU CoreSight component identity register 3."]
    #[inline(always)]
    pub const fn cidr3(self) -> crate::common::Reg<regs::Cidr3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
pub mod regs {
    #[doc = "DBGMCU AHB1 peripheral freeze register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ahb1fzr(pub u32);
    impl Ahb1fzr {
        #[doc = "GPDMA1 channel x stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn gpdma1_stop(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPDMA1 channel x stop in debug."]
        #[inline(always)]
        pub const fn set_gpdma1_stop(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "GPDMA2 channel x stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn gpdma2_stop(&self, n: usize) -> bool {
            assert!(n < 8usize);
            let offs = 16usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "GPDMA2 channel x stop in debug."]
        #[inline(always)]
        pub const fn set_gpdma2_stop(&mut self, n: usize, val: bool) {
            assert!(n < 8usize);
            let offs = 16usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Ahb1fzr {
        #[inline(always)]
        fn default() -> Ahb1fzr {
            Ahb1fzr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Ahb1fzr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ahb1fzr")
                .field("gpdma1_stop[0]", &self.gpdma1_stop(0usize))
                .field("gpdma1_stop[1]", &self.gpdma1_stop(1usize))
                .field("gpdma1_stop[2]", &self.gpdma1_stop(2usize))
                .field("gpdma1_stop[3]", &self.gpdma1_stop(3usize))
                .field("gpdma1_stop[4]", &self.gpdma1_stop(4usize))
                .field("gpdma1_stop[5]", &self.gpdma1_stop(5usize))
                .field("gpdma1_stop[6]", &self.gpdma1_stop(6usize))
                .field("gpdma1_stop[7]", &self.gpdma1_stop(7usize))
                .field("gpdma2_stop[0]", &self.gpdma2_stop(0usize))
                .field("gpdma2_stop[1]", &self.gpdma2_stop(1usize))
                .field("gpdma2_stop[2]", &self.gpdma2_stop(2usize))
                .field("gpdma2_stop[3]", &self.gpdma2_stop(3usize))
                .field("gpdma2_stop[4]", &self.gpdma2_stop(4usize))
                .field("gpdma2_stop[5]", &self.gpdma2_stop(5usize))
                .field("gpdma2_stop[6]", &self.gpdma2_stop(6usize))
                .field("gpdma2_stop[7]", &self.gpdma2_stop(7usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ahb1fzr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ahb1fzr {{ gpdma1_stop[0]: {=bool:?}, gpdma1_stop[1]: {=bool:?}, gpdma1_stop[2]: {=bool:?}, gpdma1_stop[3]: {=bool:?}, gpdma1_stop[4]: {=bool:?}, gpdma1_stop[5]: {=bool:?}, gpdma1_stop[6]: {=bool:?}, gpdma1_stop[7]: {=bool:?}, gpdma2_stop[0]: {=bool:?}, gpdma2_stop[1]: {=bool:?}, gpdma2_stop[2]: {=bool:?}, gpdma2_stop[3]: {=bool:?}, gpdma2_stop[4]: {=bool:?}, gpdma2_stop[5]: {=bool:?}, gpdma2_stop[6]: {=bool:?}, gpdma2_stop[7]: {=bool:?} }}",
                self.gpdma1_stop(0usize),
                self.gpdma1_stop(1usize),
                self.gpdma1_stop(2usize),
                self.gpdma1_stop(3usize),
                self.gpdma1_stop(4usize),
                self.gpdma1_stop(5usize),
                self.gpdma1_stop(6usize),
                self.gpdma1_stop(7usize),
                self.gpdma2_stop(0usize),
                self.gpdma2_stop(1usize),
                self.gpdma2_stop(2usize),
                self.gpdma2_stop(3usize),
                self.gpdma2_stop(4usize),
                self.gpdma2_stop(5usize),
                self.gpdma2_stop(6usize),
                self.gpdma2_stop(7usize)
            )
        }
    }
    #[doc = "DBGMCU APB1 peripheral freeze register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb1fzr(pub u32);
    impl Apb1fzr {
        #[doc = "TIM2 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn tim2_stop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TIM2 stop in debug."]
        #[inline(always)]
        pub const fn set_tim2_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM3 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn tim3_stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TIM3 stop in debug."]
        #[inline(always)]
        pub const fn set_tim3_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TIM6 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn tim6_stop(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM6 stop in debug."]
        #[inline(always)]
        pub const fn set_tim6_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TIM7 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn tim7_stop(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "TIM7 stop in debug."]
        #[inline(always)]
        pub const fn set_tim7_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "TIM9 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn tim9_stop(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM9 stop in debug."]
        #[inline(always)]
        pub const fn set_tim9_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TIM10 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn tim10_stop(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TIM10 stop in debug."]
        #[inline(always)]
        pub const fn set_tim10_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "TIM11 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn tim11_stop(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TIM11 stop in debug."]
        #[inline(always)]
        pub const fn set_tim11_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "TIM12 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn tim12_stop(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12 stop in debug."]
        #[inline(always)]
        pub const fn set_tim12_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "TIM13 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn tim13_stop(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "TIM13 stop in debug."]
        #[inline(always)]
        pub const fn set_tim13_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "TIM14 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn tim14_stop(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM14 stop in debug."]
        #[inline(always)]
        pub const fn set_tim14_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "WWDG stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn wwdg_stop(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG stop in debug."]
        #[inline(always)]
        pub const fn set_wwdg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "IWDG stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn iwdg_stop(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "IWDG stop in debug."]
        #[inline(always)]
        pub const fn set_iwdg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "I2C1 SMBUS timeout stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c1_stop(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "I2C1 SMBUS timeout stop in debug."]
        #[inline(always)]
        pub const fn set_i2c1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "I2C2 SMBUS timeout stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c2_stop(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "I2C2 SMBUS timeout stop in debug."]
        #[inline(always)]
        pub const fn set_i2c2_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "I3C1 SCL stall counter stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn i3c1_stop(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "I3C1 SCL stall counter stop in debug."]
        #[inline(always)]
        pub const fn set_i3c1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Apb1fzr {
        #[inline(always)]
        fn default() -> Apb1fzr {
            Apb1fzr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Apb1fzr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb1fzr")
                .field("tim2_stop", &self.tim2_stop())
                .field("tim3_stop", &self.tim3_stop())
                .field("tim6_stop", &self.tim6_stop())
                .field("tim7_stop", &self.tim7_stop())
                .field("tim9_stop", &self.tim9_stop())
                .field("tim10_stop", &self.tim10_stop())
                .field("tim11_stop", &self.tim11_stop())
                .field("tim12_stop", &self.tim12_stop())
                .field("tim13_stop", &self.tim13_stop())
                .field("tim14_stop", &self.tim14_stop())
                .field("wwdg_stop", &self.wwdg_stop())
                .field("iwdg_stop", &self.iwdg_stop())
                .field("i2c1_stop", &self.i2c1_stop())
                .field("i2c2_stop", &self.i2c2_stop())
                .field("i3c1_stop", &self.i3c1_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb1fzr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb1fzr {{ tim2_stop: {=bool:?}, tim3_stop: {=bool:?}, tim6_stop: {=bool:?}, tim7_stop: {=bool:?}, tim9_stop: {=bool:?}, tim10_stop: {=bool:?}, tim11_stop: {=bool:?}, tim12_stop: {=bool:?}, tim13_stop: {=bool:?}, tim14_stop: {=bool:?}, wwdg_stop: {=bool:?}, iwdg_stop: {=bool:?}, i2c1_stop: {=bool:?}, i2c2_stop: {=bool:?}, i3c1_stop: {=bool:?} }}",
                self.tim2_stop(),
                self.tim3_stop(),
                self.tim6_stop(),
                self.tim7_stop(),
                self.tim9_stop(),
                self.tim10_stop(),
                self.tim11_stop(),
                self.tim12_stop(),
                self.tim13_stop(),
                self.tim14_stop(),
                self.wwdg_stop(),
                self.iwdg_stop(),
                self.i2c1_stop(),
                self.i2c2_stop(),
                self.i3c1_stop()
            )
        }
    }
    #[doc = "DBGMCU APB2 peripheral freeze register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb2fzr(pub u32);
    impl Apb2fzr {
        #[doc = "HRTIM1 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn hrtim1_stop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "HRTIM1 stop in debug."]
        #[inline(always)]
        pub const fn set_hrtim1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "TIM1 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn tim1_stop(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "TIM1 stop in debug."]
        #[inline(always)]
        pub const fn set_tim1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "TIM8 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn tim8_stop(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "TIM8 stop in debug."]
        #[inline(always)]
        pub const fn set_tim8_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "TIM15 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn tim15_stop(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "TIM15 stop in debug."]
        #[inline(always)]
        pub const fn set_tim15_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "TIM16 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn tim16_stop(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "TIM16 stop in debug."]
        #[inline(always)]
        pub const fn set_tim16_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "TIM17 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn tim17_stop(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "TIM17 stop in debug."]
        #[inline(always)]
        pub const fn set_tim17_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "TIM4 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn tim4_stop(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 stop in debug."]
        #[inline(always)]
        pub const fn set_tim4_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "TIM5 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn tim5_stop(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 stop in debug."]
        #[inline(always)]
        pub const fn set_tim5_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Apb2fzr {
        #[inline(always)]
        fn default() -> Apb2fzr {
            Apb2fzr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Apb2fzr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb2fzr")
                .field("hrtim1_stop", &self.hrtim1_stop())
                .field("tim1_stop", &self.tim1_stop())
                .field("tim8_stop", &self.tim8_stop())
                .field("tim15_stop", &self.tim15_stop())
                .field("tim16_stop", &self.tim16_stop())
                .field("tim17_stop", &self.tim17_stop())
                .field("tim4_stop", &self.tim4_stop())
                .field("tim5_stop", &self.tim5_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb2fzr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb2fzr {{ hrtim1_stop: {=bool:?}, tim1_stop: {=bool:?}, tim8_stop: {=bool:?}, tim15_stop: {=bool:?}, tim16_stop: {=bool:?}, tim17_stop: {=bool:?}, tim4_stop: {=bool:?}, tim5_stop: {=bool:?} }}",
                self.hrtim1_stop(),
                self.tim1_stop(),
                self.tim8_stop(),
                self.tim15_stop(),
                self.tim16_stop(),
                self.tim17_stop(),
                self.tim4_stop(),
                self.tim5_stop()
            )
        }
    }
    #[doc = "DBGMCU APB3 peripheral freeze register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Apb3fzr(pub u32);
    impl Apb3fzr {
        #[doc = "TIM20 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn tim20_stop(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM20 stop in debug."]
        #[inline(always)]
        pub const fn set_tim20_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM26 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn tim26_stop(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM26 stop in debug."]
        #[inline(always)]
        pub const fn set_tim26_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "TIM27 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn tim27_stop(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "TIM27 stop in debug."]
        #[inline(always)]
        pub const fn set_tim27_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "I2C3 SMBUS timeout stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c3_stop(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "I2C3 SMBUS timeout stop in debug."]
        #[inline(always)]
        pub const fn set_i2c3_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "I2C4 SMBUS timeout stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn i2c4_stop(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "I2C4 SMBUS timeout stop in debug."]
        #[inline(always)]
        pub const fn set_i2c4_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "LPTIM1 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn lptim1_stop(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "LPTIM1 stop in debug."]
        #[inline(always)]
        pub const fn set_lptim1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "RTC stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn rtc_stop(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "RTC stop in debug."]
        #[inline(always)]
        pub const fn set_rtc_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Apb3fzr {
        #[inline(always)]
        fn default() -> Apb3fzr {
            Apb3fzr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Apb3fzr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Apb3fzr")
                .field("tim20_stop", &self.tim20_stop())
                .field("tim26_stop", &self.tim26_stop())
                .field("tim27_stop", &self.tim27_stop())
                .field("i2c3_stop", &self.i2c3_stop())
                .field("i2c4_stop", &self.i2c4_stop())
                .field("lptim1_stop", &self.lptim1_stop())
                .field("rtc_stop", &self.rtc_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Apb3fzr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Apb3fzr {{ tim20_stop: {=bool:?}, tim26_stop: {=bool:?}, tim27_stop: {=bool:?}, i2c3_stop: {=bool:?}, i2c4_stop: {=bool:?}, lptim1_stop: {=bool:?}, rtc_stop: {=bool:?} }}",
                self.tim20_stop(),
                self.tim26_stop(),
                self.tim27_stop(),
                self.i2c3_stop(),
                self.i2c4_stop(),
                self.lptim1_stop(),
                self.rtc_stop()
            )
        }
    }
    #[doc = "DBGMCU debug authentication mailbox acknowledge register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AuthAck(pub u32);
    impl AuthAck {
        #[doc = "Host to device acknowledge."]
        #[must_use]
        #[inline(always)]
        pub const fn host_ack(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Host to device acknowledge."]
        #[inline(always)]
        pub const fn set_host_ack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Device to host acknowledge."]
        #[must_use]
        #[inline(always)]
        pub const fn dev_ack(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Device to host acknowledge."]
        #[inline(always)]
        pub const fn set_dev_ack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for AuthAck {
        #[inline(always)]
        fn default() -> AuthAck {
            AuthAck(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for AuthAck {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AuthAck")
                .field("host_ack", &self.host_ack())
                .field("dev_ack", &self.dev_ack())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for AuthAck {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "AuthAck {{ host_ack: {=bool:?}, dev_ack: {=bool:?} }}",
                self.host_ack(),
                self.dev_ack()
            )
        }
    }
    #[doc = "DBGMCU CoreSight component identity register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cidr0(pub u32);
    impl Cidr0 {
        #[doc = "Component identification bits \\[7:0\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn preamble(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Component identification bits \\[7:0\\]."]
        #[inline(always)]
        pub const fn set_preamble(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Cidr0 {
        #[inline(always)]
        fn default() -> Cidr0 {
            Cidr0(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Cidr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cidr0").field("preamble", &self.preamble()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cidr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cidr0 {{ preamble: {=u8:?} }}", self.preamble())
        }
    }
    #[doc = "DBGMCU CoreSight component identity register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cidr1(pub u32);
    impl Cidr1 {
        #[doc = "Component identification bits \\[11:8\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn preamble(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Component identification bits \\[11:8\\]."]
        #[inline(always)]
        pub const fn set_preamble(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Component identification bits \\[15:12\\]
- component class."]
        #[must_use]
        #[inline(always)]
        pub const fn class(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Component identification bits \\[15:12\\]
- component class."]
        #[inline(always)]
        pub const fn set_class(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Cidr1 {
        #[inline(always)]
        fn default() -> Cidr1 {
            Cidr1(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Cidr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cidr1")
                .field("preamble", &self.preamble())
                .field("class", &self.class())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cidr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cidr1 {{ preamble: {=u8:?}, class: {=u8:?} }}",
                self.preamble(),
                self.class()
            )
        }
    }
    #[doc = "DBGMCU CoreSight component identity register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cidr2(pub u32);
    impl Cidr2 {
        #[doc = "Component identification bits \\[23:16\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn preamble(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Component identification bits \\[23:16\\]."]
        #[inline(always)]
        pub const fn set_preamble(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Cidr2 {
        #[inline(always)]
        fn default() -> Cidr2 {
            Cidr2(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Cidr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cidr2").field("preamble", &self.preamble()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cidr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cidr2 {{ preamble: {=u8:?} }}", self.preamble())
        }
    }
    #[doc = "DBGMCU CoreSight component identity register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cidr3(pub u32);
    impl Cidr3 {
        #[doc = "Component identification bits \\[31:24\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn preamble(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Component identification bits \\[31:24\\]."]
        #[inline(always)]
        pub const fn set_preamble(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Cidr3 {
        #[inline(always)]
        fn default() -> Cidr3 {
            Cidr3(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Cidr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cidr3").field("preamble", &self.preamble()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cidr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Cidr3 {{ preamble: {=u8:?} }}", self.preamble())
        }
    }
    #[doc = "DBGMCU configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Allows debug in Stop mode."]
        #[must_use]
        #[inline(always)]
        pub const fn stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Allows debug in Stop mode."]
        #[inline(always)]
        pub const fn set_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Allows debug in Standby mode."]
        #[must_use]
        #[inline(always)]
        pub const fn standby(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Allows debug in Standby mode."]
        #[inline(always)]
        pub const fn set_standby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Trace pin enable."]
        #[must_use]
        #[inline(always)]
        pub const fn trace_ioen(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Trace pin enable."]
        #[inline(always)]
        pub const fn set_trace_ioen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Trace port and clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn trace_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Trace port and clock enable."]
        #[inline(always)]
        pub const fn set_trace_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Trace pin assignment."]
        #[must_use]
        #[inline(always)]
        pub const fn trace_mode(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Trace pin assignment."]
        #[inline(always)]
        pub const fn set_trace_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "Debug credentials reset type."]
        #[must_use]
        #[inline(always)]
        pub const fn dcrt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Debug credentials reset type."]
        #[inline(always)]
        pub const fn set_dcrt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
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
                .field("stop", &self.stop())
                .field("standby", &self.standby())
                .field("trace_ioen", &self.trace_ioen())
                .field("trace_en", &self.trace_en())
                .field("trace_mode", &self.trace_mode())
                .field("dcrt", &self.dcrt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ stop: {=bool:?}, standby: {=bool:?}, trace_ioen: {=bool:?}, trace_en: {=bool:?}, trace_mode: {=u8:?}, dcrt: {=bool:?} }}",
                self.stop(),
                self.standby(),
                self.trace_ioen(),
                self.trace_en(),
                self.trace_mode(),
                self.dcrt()
            )
        }
    }
    #[doc = "DBGMCU EDBGRQ debug request control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Edbgrq(pub u32);
    impl Edbgrq {
        #[doc = "Drives the CPU external debug request (EDBGRQ) signal."]
        #[must_use]
        #[inline(always)]
        pub const fn cpu(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Drives the CPU external debug request (EDBGRQ) signal."]
        #[inline(always)]
        pub const fn set_cpu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Edbgrq {
        #[inline(always)]
        fn default() -> Edbgrq {
            Edbgrq(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Edbgrq {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Edbgrq").field("cpu", &self.cpu()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Edbgrq {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Edbgrq {{ cpu: {=bool:?} }}", self.cpu())
        }
    }
    #[doc = "DBGMCU identity code register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idcode(pub u32);
    impl Idcode {
        #[doc = "Device identification."]
        #[must_use]
        #[inline(always)]
        pub const fn dev_id(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Device identification."]
        #[inline(always)]
        pub const fn set_dev_id(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Revision."]
        #[must_use]
        #[inline(always)]
        pub const fn rev_id(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Revision."]
        #[inline(always)]
        pub const fn set_rev_id(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Idcode {
        #[inline(always)]
        fn default() -> Idcode {
            Idcode(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Idcode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Idcode")
                .field("dev_id", &self.dev_id())
                .field("rev_id", &self.rev_id())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Idcode {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Idcode {{ dev_id: {=u16:?}, rev_id: {=u16:?} }}",
                self.dev_id(),
                self.rev_id()
            )
        }
    }
    #[doc = "DBGMCU CoreSight peripheral identity register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pidr0(pub u32);
    impl Pidr0 {
        #[doc = "Part number bits \\[7:0\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn partnum(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Part number bits \\[7:0\\]."]
        #[inline(always)]
        pub const fn set_partnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Pidr0 {
        #[inline(always)]
        fn default() -> Pidr0 {
            Pidr0(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Pidr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pidr0").field("partnum", &self.partnum()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pidr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Pidr0 {{ partnum: {=u8:?} }}", self.partnum())
        }
    }
    #[doc = "DBGMCU CoreSight peripheral identity register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pidr1(pub u32);
    impl Pidr1 {
        #[doc = "Part number bits \\[11:8\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn partnum(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Part number bits \\[11:8\\]."]
        #[inline(always)]
        pub const fn set_partnum(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "JEP106 identity code bits \\[3:0\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn jep106id(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "JEP106 identity code bits \\[3:0\\]."]
        #[inline(always)]
        pub const fn set_jep106id(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Pidr1 {
        #[inline(always)]
        fn default() -> Pidr1 {
            Pidr1(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Pidr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pidr1")
                .field("partnum", &self.partnum())
                .field("jep106id", &self.jep106id())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pidr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pidr1 {{ partnum: {=u8:?}, jep106id: {=u8:?} }}",
                self.partnum(),
                self.jep106id()
            )
        }
    }
    #[doc = "DBGMCU CoreSight peripheral identity register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pidr2(pub u32);
    impl Pidr2 {
        #[doc = "JEP106 identity code bits \\[6:4\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn jep106id(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "JEP106 identity code bits \\[6:4\\]."]
        #[inline(always)]
        pub const fn set_jep106id(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "JEDEC assigned value."]
        #[must_use]
        #[inline(always)]
        pub const fn jedec(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "JEDEC assigned value."]
        #[inline(always)]
        pub const fn set_jedec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Component revision number."]
        #[must_use]
        #[inline(always)]
        pub const fn revision(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Component revision number."]
        #[inline(always)]
        pub const fn set_revision(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Pidr2 {
        #[inline(always)]
        fn default() -> Pidr2 {
            Pidr2(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Pidr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pidr2")
                .field("jep106id", &self.jep106id())
                .field("jedec", &self.jedec())
                .field("revision", &self.revision())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pidr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pidr2 {{ jep106id: {=u8:?}, jedec: {=bool:?}, revision: {=u8:?} }}",
                self.jep106id(),
                self.jedec(),
                self.revision()
            )
        }
    }
    #[doc = "DBGMCU CoreSight peripheral identity register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pidr3(pub u32);
    impl Pidr3 {
        #[doc = "Customer modified."]
        #[must_use]
        #[inline(always)]
        pub const fn cmod(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Customer modified."]
        #[inline(always)]
        pub const fn set_cmod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Metal fix version."]
        #[must_use]
        #[inline(always)]
        pub const fn revand(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Metal fix version."]
        #[inline(always)]
        pub const fn set_revand(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Pidr3 {
        #[inline(always)]
        fn default() -> Pidr3 {
            Pidr3(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Pidr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pidr3")
                .field("cmod", &self.cmod())
                .field("revand", &self.revand())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pidr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pidr3 {{ cmod: {=u8:?}, revand: {=u8:?} }}",
                self.cmod(),
                self.revand()
            )
        }
    }
    #[doc = "DBGMCU CoreSight peripheral identity register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pidr4(pub u32);
    impl Pidr4 {
        #[doc = "JEP106 continuation code."]
        #[must_use]
        #[inline(always)]
        pub const fn jep106con(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "JEP106 continuation code."]
        #[inline(always)]
        pub const fn set_jep106con(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Register file size."]
        #[must_use]
        #[inline(always)]
        pub const fn size(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Register file size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for Pidr4 {
        #[inline(always)]
        fn default() -> Pidr4 {
            Pidr4(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Pidr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pidr4")
                .field("jep106con", &self.jep106con())
                .field("size", &self.size())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Pidr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Pidr4 {{ jep106con: {=u8:?}, size: {=u8:?} }}",
                self.jep106con(),
                self.size()
            )
        }
    }
    #[doc = "DBGMCU status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Bit n identifies whether access port AP n is present in device."]
        #[must_use]
        #[inline(always)]
        pub const fn ap_present(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Bit n identifies whether access port AP n is present in device."]
        #[inline(always)]
        pub const fn set_ap_present(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Bit n identifies whether access port AP n is open or locked."]
        #[must_use]
        #[inline(always)]
        pub const fn ap_enabled(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Bit n identifies whether access port AP n is open or locked."]
        #[inline(always)]
        pub const fn set_ap_enabled(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
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
                .field("ap_present", &self.ap_present())
                .field("ap_enabled", &self.ap_enabled())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ ap_present: {=u16:?}, ap_enabled: {=u16:?} }}",
                self.ap_present(),
                self.ap_enabled()
            )
        }
    }
}
