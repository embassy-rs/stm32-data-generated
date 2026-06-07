#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "DBGMCU address block description."]
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
    pub const fn dbgmcu_idcode(self) -> crate::common::Reg<regs::DbgmcuIdcode, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "DBGMCU configuration register."]
    #[inline(always)]
    pub const fn dbgmcu_cr(self) -> crate::common::Reg<regs::DbgmcuCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "DBGMCU APB1L peripheral freeze register."]
    #[inline(always)]
    pub const fn dbgmcu_apb1lfzr(self) -> crate::common::Reg<regs::DbgmcuApb1lfzr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "DBGMCU APB2 peripheral freeze register."]
    #[inline(always)]
    pub const fn dbgmcu_apb2fzr(self) -> crate::common::Reg<regs::DbgmcuApb2fzr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "DBGMCU APB3 peripheral freeze register."]
    #[inline(always)]
    pub const fn dbgmcu_apb3fzr(self) -> crate::common::Reg<regs::DbgmcuApb3fzr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "DBGMCU AHB1 peripheral freeze register."]
    #[inline(always)]
    pub const fn dbgmcu_ahb1fzr(self) -> crate::common::Reg<regs::DbgmcuAhb1fzr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "DBGMCU status register."]
    #[inline(always)]
    pub const fn dbgmcu_sr(self) -> crate::common::Reg<regs::DbgmcuSr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "DBGMCU debug authentication mailbox host register."]
    #[inline(always)]
    pub const fn dbgmcu_dbg_auth_host(self) -> crate::common::Reg<regs::DbgmcuDbgAuthHost, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "DBGMCU debug authentication mailbox device register."]
    #[inline(always)]
    pub const fn dbgmcu_dbg_auth_device(self) -> crate::common::Reg<regs::DbgmcuDbgAuthDevice, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "DBGMCU boundary-scan key password register."]
    #[inline(always)]
    pub const fn dbgmcu_dbg_bskey_pwd(self) -> crate::common::Reg<regs::DbgmcuDbgBskeyPwd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "DBGMCU debug OEMKEY validation register."]
    #[inline(always)]
    pub const fn dbgmcu_dbg_valr(self) -> crate::common::Reg<regs::DbgmcuDbgValr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "DBGMCU CoreSight peripheral identity register 4."]
    #[inline(always)]
    pub const fn dbgmcu_pidr4(self) -> crate::common::Reg<regs::DbgmcuPidr4, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fd0usize) as _) }
    }
    #[doc = "DBGMCU CoreSight peripheral identity register 0."]
    #[inline(always)]
    pub const fn dbgmcu_pidr0(self) -> crate::common::Reg<regs::DbgmcuPidr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe0usize) as _) }
    }
    #[doc = "DBGMCU CoreSight peripheral identity register 1."]
    #[inline(always)]
    pub const fn dbgmcu_pidr1(self) -> crate::common::Reg<regs::DbgmcuPidr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe4usize) as _) }
    }
    #[doc = "DBGMCU CoreSight peripheral identity register 2."]
    #[inline(always)]
    pub const fn dbgmcu_pidr2(self) -> crate::common::Reg<regs::DbgmcuPidr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fe8usize) as _) }
    }
    #[doc = "DBGMCU CoreSight peripheral identity register 3."]
    #[inline(always)]
    pub const fn dbgmcu_pidr3(self) -> crate::common::Reg<regs::DbgmcuPidr3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fecusize) as _) }
    }
    #[doc = "DBGMCU CoreSight component identity register 0."]
    #[inline(always)]
    pub const fn dbgmcu_cidr0(self) -> crate::common::Reg<regs::DbgmcuCidr0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff0usize) as _) }
    }
    #[doc = "DBGMCU CoreSight component identity register 1."]
    #[inline(always)]
    pub const fn dbgmcu_cidr1(self) -> crate::common::Reg<regs::DbgmcuCidr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff4usize) as _) }
    }
    #[doc = "DBGMCU CoreSight component identity register 2."]
    #[inline(always)]
    pub const fn dbgmcu_cidr2(self) -> crate::common::Reg<regs::DbgmcuCidr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ff8usize) as _) }
    }
    #[doc = "DBGMCU CoreSight component identity register 3."]
    #[inline(always)]
    pub const fn dbgmcu_cidr3(self) -> crate::common::Reg<regs::DbgmcuCidr3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ffcusize) as _) }
    }
}
pub mod regs {
    #[doc = "DBGMCU AHB1 peripheral freeze register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuAhb1fzr(pub u32);
    impl DbgmcuAhb1fzr {
        #[doc = "LPDMA1 channel 0 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn lpdma1_0_stop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LPDMA1 channel 0 stop in debug."]
        #[inline(always)]
        pub const fn set_lpdma1_0_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "LPDMA1 channel 1 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn lpdma1_1_stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LPDMA1 channel 1 stop in debug."]
        #[inline(always)]
        pub const fn set_lpdma1_1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "LPDMA1 channel 2 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn lpdma1_2_stop(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "LPDMA1 channel 2 stop in debug."]
        #[inline(always)]
        pub const fn set_lpdma1_2_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "LPDMA1 channel 3 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn lpdma1_3_stop(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LPDMA1 channel 3 stop in debug."]
        #[inline(always)]
        pub const fn set_lpdma1_3_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "LPDMA1 channel 4 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn lpdma1_4_stop(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "LPDMA1 channel 4 stop in debug."]
        #[inline(always)]
        pub const fn set_lpdma1_4_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "LPDMA1 channel 5 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn lpdma1_5_stop(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "LPDMA1 channel 5 stop in debug."]
        #[inline(always)]
        pub const fn set_lpdma1_5_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "LPDMA1 channel 6 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn lpdma1_6_stop(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LPDMA1 channel 6 stop in debug."]
        #[inline(always)]
        pub const fn set_lpdma1_6_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "LPDMA1 channel 7 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn lpdma1_7_stop(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LPDMA1 channel 7 stop in debug."]
        #[inline(always)]
        pub const fn set_lpdma1_7_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LPDMA2 channel 0 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn lpdma2_0_stop(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "LPDMA2 channel 0 stop in debug."]
        #[inline(always)]
        pub const fn set_lpdma2_0_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "LPDMA2 channel 1 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn lpdma2_1_stop(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "LPDMA2 channel 1 stop in debug."]
        #[inline(always)]
        pub const fn set_lpdma2_1_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "LPDMA2 channel 2 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn lpdma2_2_stop(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "LPDMA2 channel 2 stop in debug."]
        #[inline(always)]
        pub const fn set_lpdma2_2_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "LPDMA2 channel 3 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn lpdma2_3_stop(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "LPDMA2 channel 3 stop in debug."]
        #[inline(always)]
        pub const fn set_lpdma2_3_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "LPDMA2 channel 4 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn lpdma2_4_stop(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "LPDMA2 channel 4 stop in debug."]
        #[inline(always)]
        pub const fn set_lpdma2_4_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "LPDMA2 channel 5 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn lpdma2_5_stop(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "LPDMA2 channel 5 stop in debug."]
        #[inline(always)]
        pub const fn set_lpdma2_5_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "LPDMA2 channel 6 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn lpdma2_6_stop(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "LPDMA2 channel 6 stop in debug."]
        #[inline(always)]
        pub const fn set_lpdma2_6_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "LPDMA2 channel 7stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn lpdma2_7_stop(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "LPDMA2 channel 7stop in debug."]
        #[inline(always)]
        pub const fn set_lpdma2_7_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for DbgmcuAhb1fzr {
        #[inline(always)]
        fn default() -> DbgmcuAhb1fzr {
            DbgmcuAhb1fzr(0)
        }
    }
    impl core::fmt::Debug for DbgmcuAhb1fzr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuAhb1fzr")
                .field("lpdma1_0_stop", &self.lpdma1_0_stop())
                .field("lpdma1_1_stop", &self.lpdma1_1_stop())
                .field("lpdma1_2_stop", &self.lpdma1_2_stop())
                .field("lpdma1_3_stop", &self.lpdma1_3_stop())
                .field("lpdma1_4_stop", &self.lpdma1_4_stop())
                .field("lpdma1_5_stop", &self.lpdma1_5_stop())
                .field("lpdma1_6_stop", &self.lpdma1_6_stop())
                .field("lpdma1_7_stop", &self.lpdma1_7_stop())
                .field("lpdma2_0_stop", &self.lpdma2_0_stop())
                .field("lpdma2_1_stop", &self.lpdma2_1_stop())
                .field("lpdma2_2_stop", &self.lpdma2_2_stop())
                .field("lpdma2_3_stop", &self.lpdma2_3_stop())
                .field("lpdma2_4_stop", &self.lpdma2_4_stop())
                .field("lpdma2_5_stop", &self.lpdma2_5_stop())
                .field("lpdma2_6_stop", &self.lpdma2_6_stop())
                .field("lpdma2_7_stop", &self.lpdma2_7_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuAhb1fzr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DbgmcuAhb1fzr {{ lpdma1_0_stop: {=bool:?}, lpdma1_1_stop: {=bool:?}, lpdma1_2_stop: {=bool:?}, lpdma1_3_stop: {=bool:?}, lpdma1_4_stop: {=bool:?}, lpdma1_5_stop: {=bool:?}, lpdma1_6_stop: {=bool:?}, lpdma1_7_stop: {=bool:?}, lpdma2_0_stop: {=bool:?}, lpdma2_1_stop: {=bool:?}, lpdma2_2_stop: {=bool:?}, lpdma2_3_stop: {=bool:?}, lpdma2_4_stop: {=bool:?}, lpdma2_5_stop: {=bool:?}, lpdma2_6_stop: {=bool:?}, lpdma2_7_stop: {=bool:?} }}" , self . lpdma1_0_stop () , self . lpdma1_1_stop () , self . lpdma1_2_stop () , self . lpdma1_3_stop () , self . lpdma1_4_stop () , self . lpdma1_5_stop () , self . lpdma1_6_stop () , self . lpdma1_7_stop () , self . lpdma2_0_stop () , self . lpdma2_1_stop () , self . lpdma2_2_stop () , self . lpdma2_3_stop () , self . lpdma2_4_stop () , self . lpdma2_5_stop () , self . lpdma2_6_stop () , self . lpdma2_7_stop ())
        }
    }
    #[doc = "DBGMCU APB1L peripheral freeze register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuApb1lfzr(pub u32);
    impl DbgmcuApb1lfzr {
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
        #[doc = "TIM4 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn tim4_stop(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TIM4 stop in debug."]
        #[inline(always)]
        pub const fn set_tim4_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "TIM5 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn tim5_stop(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "TIM5 stop in debug."]
        #[inline(always)]
        pub const fn set_tim5_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
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
        #[doc = "TIM12 stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn tim12_stop(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "TIM12 stop in debug."]
        #[inline(always)]
        pub const fn set_tim12_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "WWDG stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn wwdg_stop(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "WWDG stop in debug."]
        #[inline(always)]
        pub const fn set_wwdg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "IWDG stop in debug."]
        #[must_use]
        #[inline(always)]
        pub const fn iwdg_stop(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "IWDG stop in debug."]
        #[inline(always)]
        pub const fn set_iwdg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
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
    impl Default for DbgmcuApb1lfzr {
        #[inline(always)]
        fn default() -> DbgmcuApb1lfzr {
            DbgmcuApb1lfzr(0)
        }
    }
    impl core::fmt::Debug for DbgmcuApb1lfzr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuApb1lfzr")
                .field("tim2_stop", &self.tim2_stop())
                .field("tim3_stop", &self.tim3_stop())
                .field("tim4_stop", &self.tim4_stop())
                .field("tim5_stop", &self.tim5_stop())
                .field("tim6_stop", &self.tim6_stop())
                .field("tim7_stop", &self.tim7_stop())
                .field("tim12_stop", &self.tim12_stop())
                .field("wwdg_stop", &self.wwdg_stop())
                .field("iwdg_stop", &self.iwdg_stop())
                .field("i2c1_stop", &self.i2c1_stop())
                .field("i2c2_stop", &self.i2c2_stop())
                .field("i3c1_stop", &self.i3c1_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuApb1lfzr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DbgmcuApb1lfzr {{ tim2_stop: {=bool:?}, tim3_stop: {=bool:?}, tim4_stop: {=bool:?}, tim5_stop: {=bool:?}, tim6_stop: {=bool:?}, tim7_stop: {=bool:?}, tim12_stop: {=bool:?}, wwdg_stop: {=bool:?}, iwdg_stop: {=bool:?}, i2c1_stop: {=bool:?}, i2c2_stop: {=bool:?}, i3c1_stop: {=bool:?} }}" , self . tim2_stop () , self . tim3_stop () , self . tim4_stop () , self . tim5_stop () , self . tim6_stop () , self . tim7_stop () , self . tim12_stop () , self . wwdg_stop () , self . iwdg_stop () , self . i2c1_stop () , self . i2c2_stop () , self . i3c1_stop ())
        }
    }
    #[doc = "DBGMCU APB2 peripheral freeze register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuApb2fzr(pub u32);
    impl DbgmcuApb2fzr {
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
    }
    impl Default for DbgmcuApb2fzr {
        #[inline(always)]
        fn default() -> DbgmcuApb2fzr {
            DbgmcuApb2fzr(0)
        }
    }
    impl core::fmt::Debug for DbgmcuApb2fzr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuApb2fzr")
                .field("tim1_stop", &self.tim1_stop())
                .field("tim8_stop", &self.tim8_stop())
                .field("tim15_stop", &self.tim15_stop())
                .field("tim16_stop", &self.tim16_stop())
                .field("tim17_stop", &self.tim17_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuApb2fzr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DbgmcuApb2fzr {{ tim1_stop: {=bool:?}, tim8_stop: {=bool:?}, tim15_stop: {=bool:?}, tim16_stop: {=bool:?}, tim17_stop: {=bool:?} }}" , self . tim1_stop () , self . tim8_stop () , self . tim15_stop () , self . tim16_stop () , self . tim17_stop ())
        }
    }
    #[doc = "DBGMCU APB3 peripheral freeze register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuApb3fzr(pub u32);
    impl DbgmcuApb3fzr {
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
    impl Default for DbgmcuApb3fzr {
        #[inline(always)]
        fn default() -> DbgmcuApb3fzr {
            DbgmcuApb3fzr(0)
        }
    }
    impl core::fmt::Debug for DbgmcuApb3fzr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuApb3fzr")
                .field("lptim1_stop", &self.lptim1_stop())
                .field("rtc_stop", &self.rtc_stop())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuApb3fzr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DbgmcuApb3fzr {{ lptim1_stop: {=bool:?}, rtc_stop: {=bool:?} }}",
                self.lptim1_stop(),
                self.rtc_stop()
            )
        }
    }
    #[doc = "DBGMCU CoreSight component identity register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuCidr0(pub u32);
    impl DbgmcuCidr0 {
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
    impl Default for DbgmcuCidr0 {
        #[inline(always)]
        fn default() -> DbgmcuCidr0 {
            DbgmcuCidr0(0)
        }
    }
    impl core::fmt::Debug for DbgmcuCidr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuCidr0")
                .field("preamble", &self.preamble())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuCidr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DbgmcuCidr0 {{ preamble: {=u8:?} }}", self.preamble())
        }
    }
    #[doc = "DBGMCU CoreSight component identity register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuCidr1(pub u32);
    impl DbgmcuCidr1 {
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
    impl Default for DbgmcuCidr1 {
        #[inline(always)]
        fn default() -> DbgmcuCidr1 {
            DbgmcuCidr1(0)
        }
    }
    impl core::fmt::Debug for DbgmcuCidr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuCidr1")
                .field("preamble", &self.preamble())
                .field("class", &self.class())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuCidr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DbgmcuCidr1 {{ preamble: {=u8:?}, class: {=u8:?} }}",
                self.preamble(),
                self.class()
            )
        }
    }
    #[doc = "DBGMCU CoreSight component identity register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuCidr2(pub u32);
    impl DbgmcuCidr2 {
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
    impl Default for DbgmcuCidr2 {
        #[inline(always)]
        fn default() -> DbgmcuCidr2 {
            DbgmcuCidr2(0)
        }
    }
    impl core::fmt::Debug for DbgmcuCidr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuCidr2")
                .field("preamble", &self.preamble())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuCidr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DbgmcuCidr2 {{ preamble: {=u8:?} }}", self.preamble())
        }
    }
    #[doc = "DBGMCU CoreSight component identity register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuCidr3(pub u32);
    impl DbgmcuCidr3 {
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
    impl Default for DbgmcuCidr3 {
        #[inline(always)]
        fn default() -> DbgmcuCidr3 {
            DbgmcuCidr3(0)
        }
    }
    impl core::fmt::Debug for DbgmcuCidr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuCidr3")
                .field("preamble", &self.preamble())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuCidr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DbgmcuCidr3 {{ preamble: {=u8:?} }}", self.preamble())
        }
    }
    #[doc = "DBGMCU configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuCr(pub u32);
    impl DbgmcuCr {
        #[doc = "Debug in Sleep mode."]
        #[must_use]
        #[inline(always)]
        pub const fn sleep(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Debug in Sleep mode."]
        #[inline(always)]
        pub const fn set_sleep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Debug in Stop mode."]
        #[must_use]
        #[inline(always)]
        pub const fn stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Debug in Stop mode."]
        #[inline(always)]
        pub const fn set_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Debug in Standby mode."]
        #[must_use]
        #[inline(always)]
        pub const fn standby(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Debug in Standby mode."]
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
        pub const fn trace_mode(&self) -> super::vals::TraceMode {
            let val = (self.0 >> 6usize) & 0x03;
            super::vals::TraceMode::from_bits(val as u8)
        }
        #[doc = "Trace pin assignment."]
        #[inline(always)]
        pub const fn set_trace_mode(&mut self, val: super::vals::TraceMode) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
        }
    }
    impl Default for DbgmcuCr {
        #[inline(always)]
        fn default() -> DbgmcuCr {
            DbgmcuCr(0)
        }
    }
    impl core::fmt::Debug for DbgmcuCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuCr")
                .field("sleep", &self.sleep())
                .field("stop", &self.stop())
                .field("standby", &self.standby())
                .field("trace_ioen", &self.trace_ioen())
                .field("trace_en", &self.trace_en())
                .field("trace_mode", &self.trace_mode())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuCr {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "DbgmcuCr {{ sleep: {=bool:?}, stop: {=bool:?}, standby: {=bool:?}, trace_ioen: {=bool:?}, trace_en: {=bool:?}, trace_mode: {:?} }}" , self . sleep () , self . stop () , self . standby () , self . trace_ioen () , self . trace_en () , self . trace_mode ())
        }
    }
    #[doc = "DBGMCU debug authentication mailbox device register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuDbgAuthDevice(pub u32);
    impl DbgmcuDbgAuthDevice {
        #[doc = "Device specific ID used for RDP regression."]
        #[must_use]
        #[inline(always)]
        pub const fn auth_id(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Device specific ID used for RDP regression."]
        #[inline(always)]
        pub const fn set_auth_id(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DbgmcuDbgAuthDevice {
        #[inline(always)]
        fn default() -> DbgmcuDbgAuthDevice {
            DbgmcuDbgAuthDevice(0)
        }
    }
    impl core::fmt::Debug for DbgmcuDbgAuthDevice {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuDbgAuthDevice")
                .field("auth_id", &self.auth_id())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuDbgAuthDevice {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DbgmcuDbgAuthDevice {{ auth_id: {=u32:?} }}", self.auth_id())
        }
    }
    #[doc = "DBGMCU debug authentication mailbox host register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuDbgAuthHost(pub u32);
    impl DbgmcuDbgAuthHost {
        #[doc = "Device authentication key."]
        #[must_use]
        #[inline(always)]
        pub const fn auth_key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Device authentication key."]
        #[inline(always)]
        pub const fn set_auth_key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DbgmcuDbgAuthHost {
        #[inline(always)]
        fn default() -> DbgmcuDbgAuthHost {
            DbgmcuDbgAuthHost(0)
        }
    }
    impl core::fmt::Debug for DbgmcuDbgAuthHost {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuDbgAuthHost")
                .field("auth_key", &self.auth_key())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuDbgAuthHost {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DbgmcuDbgAuthHost {{ auth_key: {=u32:?} }}", self.auth_key())
        }
    }
    #[doc = "DBGMCU boundary-scan key password register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuDbgBskeyPwd(pub u32);
    impl DbgmcuDbgBskeyPwd {
        #[doc = "Boundary-scan key (BS key)."]
        #[must_use]
        #[inline(always)]
        pub const fn bskey_pwd(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Boundary-scan key (BS key)."]
        #[inline(always)]
        pub const fn set_bskey_pwd(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for DbgmcuDbgBskeyPwd {
        #[inline(always)]
        fn default() -> DbgmcuDbgBskeyPwd {
            DbgmcuDbgBskeyPwd(0)
        }
    }
    impl core::fmt::Debug for DbgmcuDbgBskeyPwd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuDbgBskeyPwd")
                .field("bskey_pwd", &self.bskey_pwd())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuDbgBskeyPwd {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DbgmcuDbgBskeyPwd {{ bskey_pwd: {=u32:?} }}", self.bskey_pwd())
        }
    }
    #[doc = "DBGMCU debug OEMKEY validation register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuDbgValr(pub u32);
    impl DbgmcuDbgValr {
        #[doc = "Validation ready."]
        #[must_use]
        #[inline(always)]
        pub const fn val_rdy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Validation ready."]
        #[inline(always)]
        pub const fn set_val_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "OEMKEY validation."]
        #[must_use]
        #[inline(always)]
        pub const fn val_oemkey(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "OEMKEY validation."]
        #[inline(always)]
        pub const fn set_val_oemkey(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for DbgmcuDbgValr {
        #[inline(always)]
        fn default() -> DbgmcuDbgValr {
            DbgmcuDbgValr(0)
        }
    }
    impl core::fmt::Debug for DbgmcuDbgValr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuDbgValr")
                .field("val_rdy", &self.val_rdy())
                .field("val_oemkey", &self.val_oemkey())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuDbgValr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DbgmcuDbgValr {{ val_rdy: {=bool:?}, val_oemkey: {=bool:?} }}",
                self.val_rdy(),
                self.val_oemkey()
            )
        }
    }
    #[doc = "DBGMCU identity code register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuIdcode(pub u32);
    impl DbgmcuIdcode {
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
        #[doc = "Revision of the device."]
        #[must_use]
        #[inline(always)]
        pub const fn rev_id(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Revision of the device."]
        #[inline(always)]
        pub const fn set_rev_id(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for DbgmcuIdcode {
        #[inline(always)]
        fn default() -> DbgmcuIdcode {
            DbgmcuIdcode(0)
        }
    }
    impl core::fmt::Debug for DbgmcuIdcode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuIdcode")
                .field("dev_id", &self.dev_id())
                .field("rev_id", &self.rev_id())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuIdcode {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DbgmcuIdcode {{ dev_id: {=u16:?}, rev_id: {=u16:?} }}",
                self.dev_id(),
                self.rev_id()
            )
        }
    }
    #[doc = "DBGMCU CoreSight peripheral identity register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuPidr0(pub u32);
    impl DbgmcuPidr0 {
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
    impl Default for DbgmcuPidr0 {
        #[inline(always)]
        fn default() -> DbgmcuPidr0 {
            DbgmcuPidr0(0)
        }
    }
    impl core::fmt::Debug for DbgmcuPidr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuPidr0").field("partnum", &self.partnum()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuPidr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "DbgmcuPidr0 {{ partnum: {=u8:?} }}", self.partnum())
        }
    }
    #[doc = "DBGMCU CoreSight peripheral identity register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuPidr1(pub u32);
    impl DbgmcuPidr1 {
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
    impl Default for DbgmcuPidr1 {
        #[inline(always)]
        fn default() -> DbgmcuPidr1 {
            DbgmcuPidr1(0)
        }
    }
    impl core::fmt::Debug for DbgmcuPidr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuPidr1")
                .field("partnum", &self.partnum())
                .field("jep106id", &self.jep106id())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuPidr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DbgmcuPidr1 {{ partnum: {=u8:?}, jep106id: {=u8:?} }}",
                self.partnum(),
                self.jep106id()
            )
        }
    }
    #[doc = "DBGMCU CoreSight peripheral identity register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuPidr2(pub u32);
    impl DbgmcuPidr2 {
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
    impl Default for DbgmcuPidr2 {
        #[inline(always)]
        fn default() -> DbgmcuPidr2 {
            DbgmcuPidr2(0)
        }
    }
    impl core::fmt::Debug for DbgmcuPidr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuPidr2")
                .field("jep106id", &self.jep106id())
                .field("jedec", &self.jedec())
                .field("revision", &self.revision())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuPidr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DbgmcuPidr2 {{ jep106id: {=u8:?}, jedec: {=bool:?}, revision: {=u8:?} }}",
                self.jep106id(),
                self.jedec(),
                self.revision()
            )
        }
    }
    #[doc = "DBGMCU CoreSight peripheral identity register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuPidr3(pub u32);
    impl DbgmcuPidr3 {
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
    impl Default for DbgmcuPidr3 {
        #[inline(always)]
        fn default() -> DbgmcuPidr3 {
            DbgmcuPidr3(0)
        }
    }
    impl core::fmt::Debug for DbgmcuPidr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuPidr3")
                .field("cmod", &self.cmod())
                .field("revand", &self.revand())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuPidr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DbgmcuPidr3 {{ cmod: {=u8:?}, revand: {=u8:?} }}",
                self.cmod(),
                self.revand()
            )
        }
    }
    #[doc = "DBGMCU CoreSight peripheral identity register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuPidr4(pub u32);
    impl DbgmcuPidr4 {
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
        pub const fn size(&self) -> super::vals::Size {
            let val = (self.0 >> 4usize) & 0x0f;
            super::vals::Size::from_bits(val as u8)
        }
        #[doc = "Register file size."]
        #[inline(always)]
        pub const fn set_size(&mut self, val: super::vals::Size) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val.to_bits() as u32) & 0x0f) << 4usize);
        }
    }
    impl Default for DbgmcuPidr4 {
        #[inline(always)]
        fn default() -> DbgmcuPidr4 {
            DbgmcuPidr4(0)
        }
    }
    impl core::fmt::Debug for DbgmcuPidr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuPidr4")
                .field("jep106con", &self.jep106con())
                .field("size", &self.size())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuPidr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DbgmcuPidr4 {{ jep106con: {=u8:?}, size: {:?} }}",
                self.jep106con(),
                self.size()
            )
        }
    }
    #[doc = "DBGMCU status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DbgmcuSr(pub u32);
    impl DbgmcuSr {
        #[doc = "Access port present."]
        #[must_use]
        #[inline(always)]
        pub const fn ap_present(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Access port present."]
        #[inline(always)]
        pub const fn set_ap_present(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Access port enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ap_enabled(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Access port enable."]
        #[inline(always)]
        pub const fn set_ap_enabled(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for DbgmcuSr {
        #[inline(always)]
        fn default() -> DbgmcuSr {
            DbgmcuSr(0)
        }
    }
    impl core::fmt::Debug for DbgmcuSr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DbgmcuSr")
                .field("ap_present", &self.ap_present())
                .field("ap_enabled", &self.ap_enabled())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for DbgmcuSr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "DbgmcuSr {{ ap_present: {=u16:?}, ap_enabled: {=u16:?} }}",
                self.ap_present(),
                self.ap_enabled()
            )
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Size {
        #[doc = "The register file occupies a single 4-Kbyte region."]
        B0x0 = 0x0,
        _RESERVED_1 = 0x01,
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
    }
    impl Size {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Size {
            unsafe { core::mem::transmute(val & 0x0f) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Size {
        #[inline(always)]
        fn from(val: u8) -> Size {
            Size::from_bits(val)
        }
    }
    impl From<Size> for u8 {
        #[inline(always)]
        fn from(val: Size) -> u8 {
            Size::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum TraceMode {
        #[doc = "Trace pins assigned for asynchronous mode (TRACESWO)."]
        B0x0 = 0x0,
        #[doc = "Trace pins assigned for synchronous mode with a port width of 1 (TRACECK, TRACED0)."]
        B0x1 = 0x01,
        #[doc = "Trace pins assigned for synchronous mode with a port width of 2 ((TRACECK, TRACED0-1)."]
        B0x2 = 0x02,
        #[doc = "Trace pins assigned for synchronous mode with a port width of 4 ((TRACECK, TRACED0-3)."]
        B0x3 = 0x03,
    }
    impl TraceMode {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> TraceMode {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for TraceMode {
        #[inline(always)]
        fn from(val: u8) -> TraceMode {
            TraceMode::from_bits(val)
        }
    }
    impl From<TraceMode> for u8 {
        #[inline(always)]
        fn from(val: TraceMode) -> u8 {
            TraceMode::to_bits(val)
        }
    }
}
