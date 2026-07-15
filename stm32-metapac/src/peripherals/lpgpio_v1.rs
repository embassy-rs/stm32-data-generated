#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "LPGPIO1."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lpgpio {
    ptr: *mut u8,
}
unsafe impl Send for Lpgpio {}
unsafe impl Sync for Lpgpio {}
impl Lpgpio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "LPGPIO port mode register."]
    #[inline(always)]
    pub const fn moder(self) -> crate::common::Reg<regs::Moder, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "LPGPIO port input data register."]
    #[inline(always)]
    pub const fn idr(self) -> crate::common::Reg<regs::Idr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "LPGPIO port output data register."]
    #[inline(always)]
    pub const fn odr(self) -> crate::common::Reg<regs::Odr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "LPGPIO port bit set/reset register."]
    #[inline(always)]
    pub const fn bsrr(self) -> crate::common::Reg<regs::Bsrr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "LPGPIO port bit reset register."]
    #[inline(always)]
    pub const fn brr(self) -> crate::common::Reg<regs::Brr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
}
pub mod regs {
    #[doc = "LPGPIO port bit reset register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Brr(pub u32);
    impl Brr {
        #[doc = "BR0."]
        #[must_use]
        #[inline(always)]
        pub const fn br0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "BR0."]
        #[inline(always)]
        pub const fn set_br0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "BR1."]
        #[must_use]
        #[inline(always)]
        pub const fn br1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "BR1."]
        #[inline(always)]
        pub const fn set_br1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "BR2."]
        #[must_use]
        #[inline(always)]
        pub const fn br2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "BR2."]
        #[inline(always)]
        pub const fn set_br2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "BR3."]
        #[must_use]
        #[inline(always)]
        pub const fn br3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "BR3."]
        #[inline(always)]
        pub const fn set_br3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "BR4."]
        #[must_use]
        #[inline(always)]
        pub const fn br4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "BR4."]
        #[inline(always)]
        pub const fn set_br4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "BR5."]
        #[must_use]
        #[inline(always)]
        pub const fn br5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "BR5."]
        #[inline(always)]
        pub const fn set_br5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "BR6."]
        #[must_use]
        #[inline(always)]
        pub const fn br6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "BR6."]
        #[inline(always)]
        pub const fn set_br6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "BR7."]
        #[must_use]
        #[inline(always)]
        pub const fn br7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "BR7."]
        #[inline(always)]
        pub const fn set_br7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "BR8."]
        #[must_use]
        #[inline(always)]
        pub const fn br8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "BR8."]
        #[inline(always)]
        pub const fn set_br8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "BR9."]
        #[must_use]
        #[inline(always)]
        pub const fn br9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "BR9."]
        #[inline(always)]
        pub const fn set_br9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "BR10."]
        #[must_use]
        #[inline(always)]
        pub const fn br10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "BR10."]
        #[inline(always)]
        pub const fn set_br10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "BR11."]
        #[must_use]
        #[inline(always)]
        pub const fn br11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "BR11."]
        #[inline(always)]
        pub const fn set_br11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "BR12."]
        #[must_use]
        #[inline(always)]
        pub const fn br12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "BR12."]
        #[inline(always)]
        pub const fn set_br12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "BR13."]
        #[must_use]
        #[inline(always)]
        pub const fn br13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "BR13."]
        #[inline(always)]
        pub const fn set_br13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "BR14."]
        #[must_use]
        #[inline(always)]
        pub const fn br14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "BR14."]
        #[inline(always)]
        pub const fn set_br14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "BR15."]
        #[must_use]
        #[inline(always)]
        pub const fn br15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "BR15."]
        #[inline(always)]
        pub const fn set_br15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Brr {
        #[inline(always)]
        fn default() -> Brr {
            Brr(0)
        }
    }
    impl core::fmt::Debug for Brr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Brr")
                .field("br0", &self.br0())
                .field("br1", &self.br1())
                .field("br2", &self.br2())
                .field("br3", &self.br3())
                .field("br4", &self.br4())
                .field("br5", &self.br5())
                .field("br6", &self.br6())
                .field("br7", &self.br7())
                .field("br8", &self.br8())
                .field("br9", &self.br9())
                .field("br10", &self.br10())
                .field("br11", &self.br11())
                .field("br12", &self.br12())
                .field("br13", &self.br13())
                .field("br14", &self.br14())
                .field("br15", &self.br15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Brr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Brr {{ br0: {=bool:?}, br1: {=bool:?}, br2: {=bool:?}, br3: {=bool:?}, br4: {=bool:?}, br5: {=bool:?}, br6: {=bool:?}, br7: {=bool:?}, br8: {=bool:?}, br9: {=bool:?}, br10: {=bool:?}, br11: {=bool:?}, br12: {=bool:?}, br13: {=bool:?}, br14: {=bool:?}, br15: {=bool:?} }}",
                self.br0(),
                self.br1(),
                self.br2(),
                self.br3(),
                self.br4(),
                self.br5(),
                self.br6(),
                self.br7(),
                self.br8(),
                self.br9(),
                self.br10(),
                self.br11(),
                self.br12(),
                self.br13(),
                self.br14(),
                self.br15()
            )
        }
    }
    #[doc = "LPGPIO port bit set/reset register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bsrr(pub u32);
    impl Bsrr {
        #[doc = "BS0."]
        #[must_use]
        #[inline(always)]
        pub const fn bs0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "BS0."]
        #[inline(always)]
        pub const fn set_bs0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "BS1."]
        #[must_use]
        #[inline(always)]
        pub const fn bs1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "BS1."]
        #[inline(always)]
        pub const fn set_bs1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "BS2."]
        #[must_use]
        #[inline(always)]
        pub const fn bs2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "BS2."]
        #[inline(always)]
        pub const fn set_bs2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "BS3."]
        #[must_use]
        #[inline(always)]
        pub const fn bs3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "BS3."]
        #[inline(always)]
        pub const fn set_bs3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "BS4."]
        #[must_use]
        #[inline(always)]
        pub const fn bs4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "BS4."]
        #[inline(always)]
        pub const fn set_bs4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "BS5."]
        #[must_use]
        #[inline(always)]
        pub const fn bs5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "BS5."]
        #[inline(always)]
        pub const fn set_bs5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "BS6."]
        #[must_use]
        #[inline(always)]
        pub const fn bs6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "BS6."]
        #[inline(always)]
        pub const fn set_bs6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "BS7."]
        #[must_use]
        #[inline(always)]
        pub const fn bs7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "BS7."]
        #[inline(always)]
        pub const fn set_bs7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "BS8."]
        #[must_use]
        #[inline(always)]
        pub const fn bs8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "BS8."]
        #[inline(always)]
        pub const fn set_bs8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "BS9."]
        #[must_use]
        #[inline(always)]
        pub const fn bs9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "BS9."]
        #[inline(always)]
        pub const fn set_bs9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "BS10."]
        #[must_use]
        #[inline(always)]
        pub const fn bs10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "BS10."]
        #[inline(always)]
        pub const fn set_bs10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "BS11."]
        #[must_use]
        #[inline(always)]
        pub const fn bs11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "BS11."]
        #[inline(always)]
        pub const fn set_bs11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "BS12."]
        #[must_use]
        #[inline(always)]
        pub const fn bs12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "BS12."]
        #[inline(always)]
        pub const fn set_bs12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "BS13."]
        #[must_use]
        #[inline(always)]
        pub const fn bs13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "BS13."]
        #[inline(always)]
        pub const fn set_bs13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "BS14."]
        #[must_use]
        #[inline(always)]
        pub const fn bs14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "BS14."]
        #[inline(always)]
        pub const fn set_bs14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "BS15."]
        #[must_use]
        #[inline(always)]
        pub const fn bs15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "BS15."]
        #[inline(always)]
        pub const fn set_bs15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "BR0."]
        #[must_use]
        #[inline(always)]
        pub const fn br0(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "BR0."]
        #[inline(always)]
        pub const fn set_br0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "BR1."]
        #[must_use]
        #[inline(always)]
        pub const fn br1(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "BR1."]
        #[inline(always)]
        pub const fn set_br1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "BR2."]
        #[must_use]
        #[inline(always)]
        pub const fn br2(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "BR2."]
        #[inline(always)]
        pub const fn set_br2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "BR3."]
        #[must_use]
        #[inline(always)]
        pub const fn br3(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "BR3."]
        #[inline(always)]
        pub const fn set_br3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "BR4."]
        #[must_use]
        #[inline(always)]
        pub const fn br4(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "BR4."]
        #[inline(always)]
        pub const fn set_br4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "BR5."]
        #[must_use]
        #[inline(always)]
        pub const fn br5(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "BR5."]
        #[inline(always)]
        pub const fn set_br5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "BR6."]
        #[must_use]
        #[inline(always)]
        pub const fn br6(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "BR6."]
        #[inline(always)]
        pub const fn set_br6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "BR7."]
        #[must_use]
        #[inline(always)]
        pub const fn br7(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "BR7."]
        #[inline(always)]
        pub const fn set_br7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "BR8."]
        #[must_use]
        #[inline(always)]
        pub const fn br8(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "BR8."]
        #[inline(always)]
        pub const fn set_br8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "BR9."]
        #[must_use]
        #[inline(always)]
        pub const fn br9(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "BR9."]
        #[inline(always)]
        pub const fn set_br9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "BR10."]
        #[must_use]
        #[inline(always)]
        pub const fn br10(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "BR10."]
        #[inline(always)]
        pub const fn set_br10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "BR11."]
        #[must_use]
        #[inline(always)]
        pub const fn br11(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "BR11."]
        #[inline(always)]
        pub const fn set_br11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "BR12."]
        #[must_use]
        #[inline(always)]
        pub const fn br12(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "BR12."]
        #[inline(always)]
        pub const fn set_br12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "BR13."]
        #[must_use]
        #[inline(always)]
        pub const fn br13(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "BR13."]
        #[inline(always)]
        pub const fn set_br13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "BR14."]
        #[must_use]
        #[inline(always)]
        pub const fn br14(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "BR14."]
        #[inline(always)]
        pub const fn set_br14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "BR15."]
        #[must_use]
        #[inline(always)]
        pub const fn br15(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "BR15."]
        #[inline(always)]
        pub const fn set_br15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Bsrr {
        #[inline(always)]
        fn default() -> Bsrr {
            Bsrr(0)
        }
    }
    impl core::fmt::Debug for Bsrr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bsrr")
                .field("bs0", &self.bs0())
                .field("bs1", &self.bs1())
                .field("bs2", &self.bs2())
                .field("bs3", &self.bs3())
                .field("bs4", &self.bs4())
                .field("bs5", &self.bs5())
                .field("bs6", &self.bs6())
                .field("bs7", &self.bs7())
                .field("bs8", &self.bs8())
                .field("bs9", &self.bs9())
                .field("bs10", &self.bs10())
                .field("bs11", &self.bs11())
                .field("bs12", &self.bs12())
                .field("bs13", &self.bs13())
                .field("bs14", &self.bs14())
                .field("bs15", &self.bs15())
                .field("br0", &self.br0())
                .field("br1", &self.br1())
                .field("br2", &self.br2())
                .field("br3", &self.br3())
                .field("br4", &self.br4())
                .field("br5", &self.br5())
                .field("br6", &self.br6())
                .field("br7", &self.br7())
                .field("br8", &self.br8())
                .field("br9", &self.br9())
                .field("br10", &self.br10())
                .field("br11", &self.br11())
                .field("br12", &self.br12())
                .field("br13", &self.br13())
                .field("br14", &self.br14())
                .field("br15", &self.br15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bsrr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Bsrr {{ bs0: {=bool:?}, bs1: {=bool:?}, bs2: {=bool:?}, bs3: {=bool:?}, bs4: {=bool:?}, bs5: {=bool:?}, bs6: {=bool:?}, bs7: {=bool:?}, bs8: {=bool:?}, bs9: {=bool:?}, bs10: {=bool:?}, bs11: {=bool:?}, bs12: {=bool:?}, bs13: {=bool:?}, bs14: {=bool:?}, bs15: {=bool:?}, br0: {=bool:?}, br1: {=bool:?}, br2: {=bool:?}, br3: {=bool:?}, br4: {=bool:?}, br5: {=bool:?}, br6: {=bool:?}, br7: {=bool:?}, br8: {=bool:?}, br9: {=bool:?}, br10: {=bool:?}, br11: {=bool:?}, br12: {=bool:?}, br13: {=bool:?}, br14: {=bool:?}, br15: {=bool:?} }}",
                self.bs0(),
                self.bs1(),
                self.bs2(),
                self.bs3(),
                self.bs4(),
                self.bs5(),
                self.bs6(),
                self.bs7(),
                self.bs8(),
                self.bs9(),
                self.bs10(),
                self.bs11(),
                self.bs12(),
                self.bs13(),
                self.bs14(),
                self.bs15(),
                self.br0(),
                self.br1(),
                self.br2(),
                self.br3(),
                self.br4(),
                self.br5(),
                self.br6(),
                self.br7(),
                self.br8(),
                self.br9(),
                self.br10(),
                self.br11(),
                self.br12(),
                self.br13(),
                self.br14(),
                self.br15()
            )
        }
    }
    #[doc = "LPGPIO port input data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idr(pub u32);
    impl Idr {
        #[doc = "ID0."]
        #[must_use]
        #[inline(always)]
        pub const fn id0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ID0."]
        #[inline(always)]
        pub const fn set_id0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "ID1."]
        #[must_use]
        #[inline(always)]
        pub const fn id1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ID1."]
        #[inline(always)]
        pub const fn set_id1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ID2."]
        #[must_use]
        #[inline(always)]
        pub const fn id2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ID2."]
        #[inline(always)]
        pub const fn set_id2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "ID3."]
        #[must_use]
        #[inline(always)]
        pub const fn id3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ID3."]
        #[inline(always)]
        pub const fn set_id3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "ID4."]
        #[must_use]
        #[inline(always)]
        pub const fn id4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ID4."]
        #[inline(always)]
        pub const fn set_id4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "ID5."]
        #[must_use]
        #[inline(always)]
        pub const fn id5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ID5."]
        #[inline(always)]
        pub const fn set_id5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ID6."]
        #[must_use]
        #[inline(always)]
        pub const fn id6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "ID6."]
        #[inline(always)]
        pub const fn set_id6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "ID7."]
        #[must_use]
        #[inline(always)]
        pub const fn id7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "ID7."]
        #[inline(always)]
        pub const fn set_id7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "ID8."]
        #[must_use]
        #[inline(always)]
        pub const fn id8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "ID8."]
        #[inline(always)]
        pub const fn set_id8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "ID9."]
        #[must_use]
        #[inline(always)]
        pub const fn id9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "ID9."]
        #[inline(always)]
        pub const fn set_id9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "ID10."]
        #[must_use]
        #[inline(always)]
        pub const fn id10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "ID10."]
        #[inline(always)]
        pub const fn set_id10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "ID11."]
        #[must_use]
        #[inline(always)]
        pub const fn id11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "ID11."]
        #[inline(always)]
        pub const fn set_id11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "ID12."]
        #[must_use]
        #[inline(always)]
        pub const fn id12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "ID12."]
        #[inline(always)]
        pub const fn set_id12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "ID13."]
        #[must_use]
        #[inline(always)]
        pub const fn id13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "ID13."]
        #[inline(always)]
        pub const fn set_id13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "ID14."]
        #[must_use]
        #[inline(always)]
        pub const fn id14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "ID14."]
        #[inline(always)]
        pub const fn set_id14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "ID15."]
        #[must_use]
        #[inline(always)]
        pub const fn id15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "ID15."]
        #[inline(always)]
        pub const fn set_id15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Idr {
        #[inline(always)]
        fn default() -> Idr {
            Idr(0)
        }
    }
    impl core::fmt::Debug for Idr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Idr")
                .field("id0", &self.id0())
                .field("id1", &self.id1())
                .field("id2", &self.id2())
                .field("id3", &self.id3())
                .field("id4", &self.id4())
                .field("id5", &self.id5())
                .field("id6", &self.id6())
                .field("id7", &self.id7())
                .field("id8", &self.id8())
                .field("id9", &self.id9())
                .field("id10", &self.id10())
                .field("id11", &self.id11())
                .field("id12", &self.id12())
                .field("id13", &self.id13())
                .field("id14", &self.id14())
                .field("id15", &self.id15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Idr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Idr {{ id0: {=bool:?}, id1: {=bool:?}, id2: {=bool:?}, id3: {=bool:?}, id4: {=bool:?}, id5: {=bool:?}, id6: {=bool:?}, id7: {=bool:?}, id8: {=bool:?}, id9: {=bool:?}, id10: {=bool:?}, id11: {=bool:?}, id12: {=bool:?}, id13: {=bool:?}, id14: {=bool:?}, id15: {=bool:?} }}",
                self.id0(),
                self.id1(),
                self.id2(),
                self.id3(),
                self.id4(),
                self.id5(),
                self.id6(),
                self.id7(),
                self.id8(),
                self.id9(),
                self.id10(),
                self.id11(),
                self.id12(),
                self.id13(),
                self.id14(),
                self.id15()
            )
        }
    }
    #[doc = "LPGPIO port mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Moder(pub u32);
    impl Moder {
        #[doc = "MODE0."]
        #[must_use]
        #[inline(always)]
        pub const fn mode0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MODE0."]
        #[inline(always)]
        pub const fn set_mode0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "MODE1."]
        #[must_use]
        #[inline(always)]
        pub const fn mode1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "MODE1."]
        #[inline(always)]
        pub const fn set_mode1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "MODE2."]
        #[must_use]
        #[inline(always)]
        pub const fn mode2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "MODE2."]
        #[inline(always)]
        pub const fn set_mode2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "MODE3."]
        #[must_use]
        #[inline(always)]
        pub const fn mode3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "MODE3."]
        #[inline(always)]
        pub const fn set_mode3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "MODE4."]
        #[must_use]
        #[inline(always)]
        pub const fn mode4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "MODE4."]
        #[inline(always)]
        pub const fn set_mode4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MODE5."]
        #[must_use]
        #[inline(always)]
        pub const fn mode5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MODE5."]
        #[inline(always)]
        pub const fn set_mode5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "MODE6."]
        #[must_use]
        #[inline(always)]
        pub const fn mode6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "MODE6."]
        #[inline(always)]
        pub const fn set_mode6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "MODE7."]
        #[must_use]
        #[inline(always)]
        pub const fn mode7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "MODE7."]
        #[inline(always)]
        pub const fn set_mode7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "MODE8."]
        #[must_use]
        #[inline(always)]
        pub const fn mode8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "MODE8."]
        #[inline(always)]
        pub const fn set_mode8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "MODE9."]
        #[must_use]
        #[inline(always)]
        pub const fn mode9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "MODE9."]
        #[inline(always)]
        pub const fn set_mode9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "MODE10."]
        #[must_use]
        #[inline(always)]
        pub const fn mode10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "MODE10."]
        #[inline(always)]
        pub const fn set_mode10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "MODE11."]
        #[must_use]
        #[inline(always)]
        pub const fn mode11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "MODE11."]
        #[inline(always)]
        pub const fn set_mode11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "MODE12."]
        #[must_use]
        #[inline(always)]
        pub const fn mode12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "MODE12."]
        #[inline(always)]
        pub const fn set_mode12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "MODE13."]
        #[must_use]
        #[inline(always)]
        pub const fn mode13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "MODE13."]
        #[inline(always)]
        pub const fn set_mode13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "MODE14."]
        #[must_use]
        #[inline(always)]
        pub const fn mode14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "MODE14."]
        #[inline(always)]
        pub const fn set_mode14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "MODE15."]
        #[must_use]
        #[inline(always)]
        pub const fn mode15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "MODE15."]
        #[inline(always)]
        pub const fn set_mode15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Moder {
        #[inline(always)]
        fn default() -> Moder {
            Moder(0)
        }
    }
    impl core::fmt::Debug for Moder {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Moder")
                .field("mode0", &self.mode0())
                .field("mode1", &self.mode1())
                .field("mode2", &self.mode2())
                .field("mode3", &self.mode3())
                .field("mode4", &self.mode4())
                .field("mode5", &self.mode5())
                .field("mode6", &self.mode6())
                .field("mode7", &self.mode7())
                .field("mode8", &self.mode8())
                .field("mode9", &self.mode9())
                .field("mode10", &self.mode10())
                .field("mode11", &self.mode11())
                .field("mode12", &self.mode12())
                .field("mode13", &self.mode13())
                .field("mode14", &self.mode14())
                .field("mode15", &self.mode15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Moder {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Moder {{ mode0: {=bool:?}, mode1: {=bool:?}, mode2: {=bool:?}, mode3: {=bool:?}, mode4: {=bool:?}, mode5: {=bool:?}, mode6: {=bool:?}, mode7: {=bool:?}, mode8: {=bool:?}, mode9: {=bool:?}, mode10: {=bool:?}, mode11: {=bool:?}, mode12: {=bool:?}, mode13: {=bool:?}, mode14: {=bool:?}, mode15: {=bool:?} }}",
                self.mode0(),
                self.mode1(),
                self.mode2(),
                self.mode3(),
                self.mode4(),
                self.mode5(),
                self.mode6(),
                self.mode7(),
                self.mode8(),
                self.mode9(),
                self.mode10(),
                self.mode11(),
                self.mode12(),
                self.mode13(),
                self.mode14(),
                self.mode15()
            )
        }
    }
    #[doc = "LPGPIO port output data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Odr(pub u32);
    impl Odr {
        #[doc = "OD0."]
        #[must_use]
        #[inline(always)]
        pub const fn od0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "OD0."]
        #[inline(always)]
        pub const fn set_od0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "OD1."]
        #[must_use]
        #[inline(always)]
        pub const fn od1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "OD1."]
        #[inline(always)]
        pub const fn set_od1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "OD2."]
        #[must_use]
        #[inline(always)]
        pub const fn od2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "OD2."]
        #[inline(always)]
        pub const fn set_od2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "OD3."]
        #[must_use]
        #[inline(always)]
        pub const fn od3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "OD3."]
        #[inline(always)]
        pub const fn set_od3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "OD4."]
        #[must_use]
        #[inline(always)]
        pub const fn od4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "OD4."]
        #[inline(always)]
        pub const fn set_od4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "OD5."]
        #[must_use]
        #[inline(always)]
        pub const fn od5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "OD5."]
        #[inline(always)]
        pub const fn set_od5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "OD6."]
        #[must_use]
        #[inline(always)]
        pub const fn od6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "OD6."]
        #[inline(always)]
        pub const fn set_od6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "OD7."]
        #[must_use]
        #[inline(always)]
        pub const fn od7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "OD7."]
        #[inline(always)]
        pub const fn set_od7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "OD8."]
        #[must_use]
        #[inline(always)]
        pub const fn od8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "OD8."]
        #[inline(always)]
        pub const fn set_od8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "OD9."]
        #[must_use]
        #[inline(always)]
        pub const fn od9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "OD9."]
        #[inline(always)]
        pub const fn set_od9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "OD10."]
        #[must_use]
        #[inline(always)]
        pub const fn od10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "OD10."]
        #[inline(always)]
        pub const fn set_od10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "OD11."]
        #[must_use]
        #[inline(always)]
        pub const fn od11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "OD11."]
        #[inline(always)]
        pub const fn set_od11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "OD12."]
        #[must_use]
        #[inline(always)]
        pub const fn od12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "OD12."]
        #[inline(always)]
        pub const fn set_od12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "OD13."]
        #[must_use]
        #[inline(always)]
        pub const fn od13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "OD13."]
        #[inline(always)]
        pub const fn set_od13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "OD14."]
        #[must_use]
        #[inline(always)]
        pub const fn od14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "OD14."]
        #[inline(always)]
        pub const fn set_od14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "OD15."]
        #[must_use]
        #[inline(always)]
        pub const fn od15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "OD15."]
        #[inline(always)]
        pub const fn set_od15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Odr {
        #[inline(always)]
        fn default() -> Odr {
            Odr(0)
        }
    }
    impl core::fmt::Debug for Odr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Odr")
                .field("od0", &self.od0())
                .field("od1", &self.od1())
                .field("od2", &self.od2())
                .field("od3", &self.od3())
                .field("od4", &self.od4())
                .field("od5", &self.od5())
                .field("od6", &self.od6())
                .field("od7", &self.od7())
                .field("od8", &self.od8())
                .field("od9", &self.od9())
                .field("od10", &self.od10())
                .field("od11", &self.od11())
                .field("od12", &self.od12())
                .field("od13", &self.od13())
                .field("od14", &self.od14())
                .field("od15", &self.od15())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Odr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Odr {{ od0: {=bool:?}, od1: {=bool:?}, od2: {=bool:?}, od3: {=bool:?}, od4: {=bool:?}, od5: {=bool:?}, od6: {=bool:?}, od7: {=bool:?}, od8: {=bool:?}, od9: {=bool:?}, od10: {=bool:?}, od11: {=bool:?}, od12: {=bool:?}, od13: {=bool:?}, od14: {=bool:?}, od15: {=bool:?} }}",
                self.od0(),
                self.od1(),
                self.od2(),
                self.od3(),
                self.od4(),
                self.od5(),
                self.od6(),
                self.od7(),
                self.od8(),
                self.od9(),
                self.od10(),
                self.od11(),
                self.od12(),
                self.od13(),
                self.od14(),
                self.od15()
            )
        }
    }
}
