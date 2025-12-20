#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "EXTI address block description."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Exti {
    ptr: *mut u8,
}
unsafe impl Send for Exti {}
unsafe impl Sync for Exti {}
impl Exti {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "EXTI rising trigger selection register."]
    #[inline(always)]
    pub const fn rtsr1(self) -> crate::common::Reg<regs::Rtsr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "EXTI falling trigger selection register."]
    #[inline(always)]
    pub const fn ftsr1(self) -> crate::common::Reg<regs::Ftsr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "EXTI software interrupt event register."]
    #[inline(always)]
    pub const fn swier1(self) -> crate::common::Reg<regs::Swier1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "EXTI rising edge pending register."]
    #[inline(always)]
    pub const fn rpr1(self) -> crate::common::Reg<regs::Rpr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "EXTI falling edge pending register."]
    #[inline(always)]
    pub const fn fpr1(self) -> crate::common::Reg<regs::Fpr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "EXTI security configuration register."]
    #[inline(always)]
    pub const fn seccfgr1(self) -> crate::common::Reg<regs::Seccfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "EXTI privilege configuration register."]
    #[inline(always)]
    pub const fn privcfgr1(self) -> crate::common::Reg<regs::Privcfgr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "EXTI external interrupt selection register."]
    #[inline(always)]
    pub const fn exticr1(self) -> crate::common::Reg<regs::Exticr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "EXTI external interrupt selection register."]
    #[inline(always)]
    pub const fn exticr2(self) -> crate::common::Reg<regs::Exticr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "EXTI external interrupt selection register."]
    #[inline(always)]
    pub const fn exticr3(self) -> crate::common::Reg<regs::Exticr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "EXTI external interrupt selection register."]
    #[inline(always)]
    pub const fn exticr4(self) -> crate::common::Reg<regs::Exticr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "EXTI lock register."]
    #[inline(always)]
    pub const fn lockr(self) -> crate::common::Reg<regs::Lockr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "EXTI CPU wake-up with interrupt mask register."]
    #[inline(always)]
    pub const fn imr1(self) -> crate::common::Reg<regs::Imr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "EXTI CPU wake-up with event mask register."]
    #[inline(always)]
    pub const fn emr1(self) -> crate::common::Reg<regs::Emr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
}
pub mod regs {
    #[doc = "EXTI CPU wake-up with event mask register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Emr1(pub u32);
    impl Emr1 {
        #[doc = "CPU wake-up with event generation mask on event input i."]
        #[inline(always)]
        pub const fn em(&self, n: usize) -> bool {
            assert!(n < 23usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "CPU wake-up with event generation mask on event input i."]
        #[inline(always)]
        pub fn set_em(&mut self, n: usize, val: bool) {
            assert!(n < 23usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Emr1 {
        #[inline(always)]
        fn default() -> Emr1 {
            Emr1(0)
        }
    }
    impl core::fmt::Debug for Emr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Emr1")
                .field("em[0]", &self.em(0usize))
                .field("em[1]", &self.em(1usize))
                .field("em[2]", &self.em(2usize))
                .field("em[3]", &self.em(3usize))
                .field("em[4]", &self.em(4usize))
                .field("em[5]", &self.em(5usize))
                .field("em[6]", &self.em(6usize))
                .field("em[7]", &self.em(7usize))
                .field("em[8]", &self.em(8usize))
                .field("em[9]", &self.em(9usize))
                .field("em[10]", &self.em(10usize))
                .field("em[11]", &self.em(11usize))
                .field("em[12]", &self.em(12usize))
                .field("em[13]", &self.em(13usize))
                .field("em[14]", &self.em(14usize))
                .field("em[15]", &self.em(15usize))
                .field("em[16]", &self.em(16usize))
                .field("em[17]", &self.em(17usize))
                .field("em[18]", &self.em(18usize))
                .field("em[19]", &self.em(19usize))
                .field("em[20]", &self.em(20usize))
                .field("em[21]", &self.em(21usize))
                .field("em[22]", &self.em(22usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Emr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Emr1 {{ em[0]: {=bool:?}, em[1]: {=bool:?}, em[2]: {=bool:?}, em[3]: {=bool:?}, em[4]: {=bool:?}, em[5]: {=bool:?}, em[6]: {=bool:?}, em[7]: {=bool:?}, em[8]: {=bool:?}, em[9]: {=bool:?}, em[10]: {=bool:?}, em[11]: {=bool:?}, em[12]: {=bool:?}, em[13]: {=bool:?}, em[14]: {=bool:?}, em[15]: {=bool:?}, em[16]: {=bool:?}, em[17]: {=bool:?}, em[18]: {=bool:?}, em[19]: {=bool:?}, em[20]: {=bool:?}, em[21]: {=bool:?}, em[22]: {=bool:?} }}" , self . em (0usize) , self . em (1usize) , self . em (2usize) , self . em (3usize) , self . em (4usize) , self . em (5usize) , self . em (6usize) , self . em (7usize) , self . em (8usize) , self . em (9usize) , self . em (10usize) , self . em (11usize) , self . em (12usize) , self . em (13usize) , self . em (14usize) , self . em (15usize) , self . em (16usize) , self . em (17usize) , self . em (18usize) , self . em (19usize) , self . em (20usize) , self . em (21usize) , self . em (22usize))
        }
    }
    #[doc = "EXTI external interrupt selection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exticr1(pub u32);
    impl Exticr1 {
        #[doc = "None."]
        #[inline(always)]
        pub const fn exti(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0xff;
            val as u8
        }
        #[doc = "None."]
        #[inline(always)]
        pub fn set_exti(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0xff << offs)) | (((val as u32) & 0xff) << offs);
        }
    }
    impl Default for Exticr1 {
        #[inline(always)]
        fn default() -> Exticr1 {
            Exticr1(0)
        }
    }
    impl core::fmt::Debug for Exticr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Exticr1")
                .field("exti[0]", &self.exti(0usize))
                .field("exti[1]", &self.exti(1usize))
                .field("exti[2]", &self.exti(2usize))
                .field("exti[3]", &self.exti(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Exticr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Exticr1 {{ exti[0]: {=u8:?}, exti[1]: {=u8:?}, exti[2]: {=u8:?}, exti[3]: {=u8:?} }}",
                self.exti(0usize),
                self.exti(1usize),
                self.exti(2usize),
                self.exti(3usize)
            )
        }
    }
    #[doc = "EXTI external interrupt selection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exticr2(pub u32);
    impl Exticr2 {
        #[doc = "None."]
        #[inline(always)]
        pub const fn exti(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0xff;
            val as u8
        }
        #[doc = "None."]
        #[inline(always)]
        pub fn set_exti(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0xff << offs)) | (((val as u32) & 0xff) << offs);
        }
    }
    impl Default for Exticr2 {
        #[inline(always)]
        fn default() -> Exticr2 {
            Exticr2(0)
        }
    }
    impl core::fmt::Debug for Exticr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Exticr2")
                .field("exti[0]", &self.exti(0usize))
                .field("exti[1]", &self.exti(1usize))
                .field("exti[2]", &self.exti(2usize))
                .field("exti[3]", &self.exti(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Exticr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Exticr2 {{ exti[0]: {=u8:?}, exti[1]: {=u8:?}, exti[2]: {=u8:?}, exti[3]: {=u8:?} }}",
                self.exti(0usize),
                self.exti(1usize),
                self.exti(2usize),
                self.exti(3usize)
            )
        }
    }
    #[doc = "EXTI external interrupt selection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exticr3(pub u32);
    impl Exticr3 {
        #[doc = "None."]
        #[inline(always)]
        pub const fn exti(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0xff;
            val as u8
        }
        #[doc = "None."]
        #[inline(always)]
        pub fn set_exti(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0xff << offs)) | (((val as u32) & 0xff) << offs);
        }
    }
    impl Default for Exticr3 {
        #[inline(always)]
        fn default() -> Exticr3 {
            Exticr3(0)
        }
    }
    impl core::fmt::Debug for Exticr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Exticr3")
                .field("exti[0]", &self.exti(0usize))
                .field("exti[1]", &self.exti(1usize))
                .field("exti[2]", &self.exti(2usize))
                .field("exti[3]", &self.exti(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Exticr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Exticr3 {{ exti[0]: {=u8:?}, exti[1]: {=u8:?}, exti[2]: {=u8:?}, exti[3]: {=u8:?} }}",
                self.exti(0usize),
                self.exti(1usize),
                self.exti(2usize),
                self.exti(3usize)
            )
        }
    }
    #[doc = "EXTI external interrupt selection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Exticr4(pub u32);
    impl Exticr4 {
        #[doc = "None."]
        #[inline(always)]
        pub const fn exti(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0xff;
            val as u8
        }
        #[doc = "None."]
        #[inline(always)]
        pub fn set_exti(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0xff << offs)) | (((val as u32) & 0xff) << offs);
        }
    }
    impl Default for Exticr4 {
        #[inline(always)]
        fn default() -> Exticr4 {
            Exticr4(0)
        }
    }
    impl core::fmt::Debug for Exticr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Exticr4")
                .field("exti[0]", &self.exti(0usize))
                .field("exti[1]", &self.exti(1usize))
                .field("exti[2]", &self.exti(2usize))
                .field("exti[3]", &self.exti(3usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Exticr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Exticr4 {{ exti[0]: {=u8:?}, exti[1]: {=u8:?}, exti[2]: {=u8:?}, exti[3]: {=u8:?} }}",
                self.exti(0usize),
                self.exti(1usize),
                self.exti(2usize),
                self.exti(3usize)
            )
        }
    }
    #[doc = "EXTI falling edge pending register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fpr1(pub u32);
    impl Fpr1 {
        #[doc = "configurable event inputs i falling edge pending bit."]
        #[inline(always)]
        pub const fn fpif(&self, n: usize) -> bool {
            assert!(n < 23usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "configurable event inputs i falling edge pending bit."]
        #[inline(always)]
        pub fn set_fpif(&mut self, n: usize, val: bool) {
            assert!(n < 23usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Fpr1 {
        #[inline(always)]
        fn default() -> Fpr1 {
            Fpr1(0)
        }
    }
    impl core::fmt::Debug for Fpr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fpr1")
                .field("fpif[0]", &self.fpif(0usize))
                .field("fpif[1]", &self.fpif(1usize))
                .field("fpif[2]", &self.fpif(2usize))
                .field("fpif[3]", &self.fpif(3usize))
                .field("fpif[4]", &self.fpif(4usize))
                .field("fpif[5]", &self.fpif(5usize))
                .field("fpif[6]", &self.fpif(6usize))
                .field("fpif[7]", &self.fpif(7usize))
                .field("fpif[8]", &self.fpif(8usize))
                .field("fpif[9]", &self.fpif(9usize))
                .field("fpif[10]", &self.fpif(10usize))
                .field("fpif[11]", &self.fpif(11usize))
                .field("fpif[12]", &self.fpif(12usize))
                .field("fpif[13]", &self.fpif(13usize))
                .field("fpif[14]", &self.fpif(14usize))
                .field("fpif[15]", &self.fpif(15usize))
                .field("fpif[16]", &self.fpif(16usize))
                .field("fpif[17]", &self.fpif(17usize))
                .field("fpif[18]", &self.fpif(18usize))
                .field("fpif[19]", &self.fpif(19usize))
                .field("fpif[20]", &self.fpif(20usize))
                .field("fpif[21]", &self.fpif(21usize))
                .field("fpif[22]", &self.fpif(22usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fpr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Fpr1 {{ fpif[0]: {=bool:?}, fpif[1]: {=bool:?}, fpif[2]: {=bool:?}, fpif[3]: {=bool:?}, fpif[4]: {=bool:?}, fpif[5]: {=bool:?}, fpif[6]: {=bool:?}, fpif[7]: {=bool:?}, fpif[8]: {=bool:?}, fpif[9]: {=bool:?}, fpif[10]: {=bool:?}, fpif[11]: {=bool:?}, fpif[12]: {=bool:?}, fpif[13]: {=bool:?}, fpif[14]: {=bool:?}, fpif[15]: {=bool:?}, fpif[16]: {=bool:?}, fpif[17]: {=bool:?}, fpif[18]: {=bool:?}, fpif[19]: {=bool:?}, fpif[20]: {=bool:?}, fpif[21]: {=bool:?}, fpif[22]: {=bool:?} }}" , self . fpif (0usize) , self . fpif (1usize) , self . fpif (2usize) , self . fpif (3usize) , self . fpif (4usize) , self . fpif (5usize) , self . fpif (6usize) , self . fpif (7usize) , self . fpif (8usize) , self . fpif (9usize) , self . fpif (10usize) , self . fpif (11usize) , self . fpif (12usize) , self . fpif (13usize) , self . fpif (14usize) , self . fpif (15usize) , self . fpif (16usize) , self . fpif (17usize) , self . fpif (18usize) , self . fpif (19usize) , self . fpif (20usize) , self . fpif (21usize) , self . fpif (22usize))
        }
    }
    #[doc = "EXTI falling trigger selection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ftsr1(pub u32);
    impl Ftsr1 {
        #[doc = "Falling trigger event configuration bit of configurable event input i."]
        #[inline(always)]
        pub const fn ft(&self, n: usize) -> bool {
            assert!(n < 23usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Falling trigger event configuration bit of configurable event input i."]
        #[inline(always)]
        pub fn set_ft(&mut self, n: usize, val: bool) {
            assert!(n < 23usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Ftsr1 {
        #[inline(always)]
        fn default() -> Ftsr1 {
            Ftsr1(0)
        }
    }
    impl core::fmt::Debug for Ftsr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ftsr1")
                .field("ft[0]", &self.ft(0usize))
                .field("ft[1]", &self.ft(1usize))
                .field("ft[2]", &self.ft(2usize))
                .field("ft[3]", &self.ft(3usize))
                .field("ft[4]", &self.ft(4usize))
                .field("ft[5]", &self.ft(5usize))
                .field("ft[6]", &self.ft(6usize))
                .field("ft[7]", &self.ft(7usize))
                .field("ft[8]", &self.ft(8usize))
                .field("ft[9]", &self.ft(9usize))
                .field("ft[10]", &self.ft(10usize))
                .field("ft[11]", &self.ft(11usize))
                .field("ft[12]", &self.ft(12usize))
                .field("ft[13]", &self.ft(13usize))
                .field("ft[14]", &self.ft(14usize))
                .field("ft[15]", &self.ft(15usize))
                .field("ft[16]", &self.ft(16usize))
                .field("ft[17]", &self.ft(17usize))
                .field("ft[18]", &self.ft(18usize))
                .field("ft[19]", &self.ft(19usize))
                .field("ft[20]", &self.ft(20usize))
                .field("ft[21]", &self.ft(21usize))
                .field("ft[22]", &self.ft(22usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ftsr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Ftsr1 {{ ft[0]: {=bool:?}, ft[1]: {=bool:?}, ft[2]: {=bool:?}, ft[3]: {=bool:?}, ft[4]: {=bool:?}, ft[5]: {=bool:?}, ft[6]: {=bool:?}, ft[7]: {=bool:?}, ft[8]: {=bool:?}, ft[9]: {=bool:?}, ft[10]: {=bool:?}, ft[11]: {=bool:?}, ft[12]: {=bool:?}, ft[13]: {=bool:?}, ft[14]: {=bool:?}, ft[15]: {=bool:?}, ft[16]: {=bool:?}, ft[17]: {=bool:?}, ft[18]: {=bool:?}, ft[19]: {=bool:?}, ft[20]: {=bool:?}, ft[21]: {=bool:?}, ft[22]: {=bool:?} }}" , self . ft (0usize) , self . ft (1usize) , self . ft (2usize) , self . ft (3usize) , self . ft (4usize) , self . ft (5usize) , self . ft (6usize) , self . ft (7usize) , self . ft (8usize) , self . ft (9usize) , self . ft (10usize) , self . ft (11usize) , self . ft (12usize) , self . ft (13usize) , self . ft (14usize) , self . ft (15usize) , self . ft (16usize) , self . ft (17usize) , self . ft (18usize) , self . ft (19usize) , self . ft (20usize) , self . ft (21usize) , self . ft (22usize))
        }
    }
    #[doc = "EXTI CPU wake-up with interrupt mask register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Imr1(pub u32);
    impl Imr1 {
        #[doc = "CPU wake-up with interrupt mask on event input i."]
        #[inline(always)]
        pub const fn im(&self, n: usize) -> bool {
            assert!(n < 23usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "CPU wake-up with interrupt mask on event input i."]
        #[inline(always)]
        pub fn set_im(&mut self, n: usize, val: bool) {
            assert!(n < 23usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Imr1 {
        #[inline(always)]
        fn default() -> Imr1 {
            Imr1(0)
        }
    }
    impl core::fmt::Debug for Imr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Imr1")
                .field("im[0]", &self.im(0usize))
                .field("im[1]", &self.im(1usize))
                .field("im[2]", &self.im(2usize))
                .field("im[3]", &self.im(3usize))
                .field("im[4]", &self.im(4usize))
                .field("im[5]", &self.im(5usize))
                .field("im[6]", &self.im(6usize))
                .field("im[7]", &self.im(7usize))
                .field("im[8]", &self.im(8usize))
                .field("im[9]", &self.im(9usize))
                .field("im[10]", &self.im(10usize))
                .field("im[11]", &self.im(11usize))
                .field("im[12]", &self.im(12usize))
                .field("im[13]", &self.im(13usize))
                .field("im[14]", &self.im(14usize))
                .field("im[15]", &self.im(15usize))
                .field("im[16]", &self.im(16usize))
                .field("im[17]", &self.im(17usize))
                .field("im[18]", &self.im(18usize))
                .field("im[19]", &self.im(19usize))
                .field("im[20]", &self.im(20usize))
                .field("im[21]", &self.im(21usize))
                .field("im[22]", &self.im(22usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Imr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Imr1 {{ im[0]: {=bool:?}, im[1]: {=bool:?}, im[2]: {=bool:?}, im[3]: {=bool:?}, im[4]: {=bool:?}, im[5]: {=bool:?}, im[6]: {=bool:?}, im[7]: {=bool:?}, im[8]: {=bool:?}, im[9]: {=bool:?}, im[10]: {=bool:?}, im[11]: {=bool:?}, im[12]: {=bool:?}, im[13]: {=bool:?}, im[14]: {=bool:?}, im[15]: {=bool:?}, im[16]: {=bool:?}, im[17]: {=bool:?}, im[18]: {=bool:?}, im[19]: {=bool:?}, im[20]: {=bool:?}, im[21]: {=bool:?}, im[22]: {=bool:?} }}" , self . im (0usize) , self . im (1usize) , self . im (2usize) , self . im (3usize) , self . im (4usize) , self . im (5usize) , self . im (6usize) , self . im (7usize) , self . im (8usize) , self . im (9usize) , self . im (10usize) , self . im (11usize) , self . im (12usize) , self . im (13usize) , self . im (14usize) , self . im (15usize) , self . im (16usize) , self . im (17usize) , self . im (18usize) , self . im (19usize) , self . im (20usize) , self . im (21usize) , self . im (22usize))
        }
    }
    #[doc = "EXTI lock register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lockr(pub u32);
    impl Lockr {
        #[doc = "Global security and privilege configuration registers (SECCFGR and PRIVCFGR) lock."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Global security and privilege configuration registers (SECCFGR and PRIVCFGR) lock."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Lockr {
        #[inline(always)]
        fn default() -> Lockr {
            Lockr(0)
        }
    }
    impl core::fmt::Debug for Lockr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lockr").field("lock", &self.lock()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Lockr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Lockr {{ lock: {=bool:?} }}", self.lock())
        }
    }
    #[doc = "EXTI privilege configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcfgr1(pub u32);
    impl Privcfgr1 {
        #[doc = "Security enable on event input i."]
        #[inline(always)]
        pub const fn priv_(&self, n: usize) -> bool {
            assert!(n < 23usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Security enable on event input i."]
        #[inline(always)]
        pub fn set_priv_(&mut self, n: usize, val: bool) {
            assert!(n < 23usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Privcfgr1 {
        #[inline(always)]
        fn default() -> Privcfgr1 {
            Privcfgr1(0)
        }
    }
    impl core::fmt::Debug for Privcfgr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Privcfgr1")
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Privcfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Privcfgr1 {{ priv_[0]: {=bool:?}, priv_[1]: {=bool:?}, priv_[2]: {=bool:?}, priv_[3]: {=bool:?}, priv_[4]: {=bool:?}, priv_[5]: {=bool:?}, priv_[6]: {=bool:?}, priv_[7]: {=bool:?}, priv_[8]: {=bool:?}, priv_[9]: {=bool:?}, priv_[10]: {=bool:?}, priv_[11]: {=bool:?}, priv_[12]: {=bool:?}, priv_[13]: {=bool:?}, priv_[14]: {=bool:?}, priv_[15]: {=bool:?}, priv_[16]: {=bool:?}, priv_[17]: {=bool:?}, priv_[18]: {=bool:?}, priv_[19]: {=bool:?}, priv_[20]: {=bool:?}, priv_[21]: {=bool:?}, priv_[22]: {=bool:?} }}" , self . priv_ (0usize) , self . priv_ (1usize) , self . priv_ (2usize) , self . priv_ (3usize) , self . priv_ (4usize) , self . priv_ (5usize) , self . priv_ (6usize) , self . priv_ (7usize) , self . priv_ (8usize) , self . priv_ (9usize) , self . priv_ (10usize) , self . priv_ (11usize) , self . priv_ (12usize) , self . priv_ (13usize) , self . priv_ (14usize) , self . priv_ (15usize) , self . priv_ (16usize) , self . priv_ (17usize) , self . priv_ (18usize) , self . priv_ (19usize) , self . priv_ (20usize) , self . priv_ (21usize) , self . priv_ (22usize))
        }
    }
    #[doc = "EXTI rising edge pending register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rpr1(pub u32);
    impl Rpr1 {
        #[doc = "Configurable event input i rising edge pending bit."]
        #[inline(always)]
        pub const fn rpif(&self, n: usize) -> bool {
            assert!(n < 23usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Configurable event input i rising edge pending bit."]
        #[inline(always)]
        pub fn set_rpif(&mut self, n: usize, val: bool) {
            assert!(n < 23usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Rpr1 {
        #[inline(always)]
        fn default() -> Rpr1 {
            Rpr1(0)
        }
    }
    impl core::fmt::Debug for Rpr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rpr1")
                .field("rpif[0]", &self.rpif(0usize))
                .field("rpif[1]", &self.rpif(1usize))
                .field("rpif[2]", &self.rpif(2usize))
                .field("rpif[3]", &self.rpif(3usize))
                .field("rpif[4]", &self.rpif(4usize))
                .field("rpif[5]", &self.rpif(5usize))
                .field("rpif[6]", &self.rpif(6usize))
                .field("rpif[7]", &self.rpif(7usize))
                .field("rpif[8]", &self.rpif(8usize))
                .field("rpif[9]", &self.rpif(9usize))
                .field("rpif[10]", &self.rpif(10usize))
                .field("rpif[11]", &self.rpif(11usize))
                .field("rpif[12]", &self.rpif(12usize))
                .field("rpif[13]", &self.rpif(13usize))
                .field("rpif[14]", &self.rpif(14usize))
                .field("rpif[15]", &self.rpif(15usize))
                .field("rpif[16]", &self.rpif(16usize))
                .field("rpif[17]", &self.rpif(17usize))
                .field("rpif[18]", &self.rpif(18usize))
                .field("rpif[19]", &self.rpif(19usize))
                .field("rpif[20]", &self.rpif(20usize))
                .field("rpif[21]", &self.rpif(21usize))
                .field("rpif[22]", &self.rpif(22usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rpr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Rpr1 {{ rpif[0]: {=bool:?}, rpif[1]: {=bool:?}, rpif[2]: {=bool:?}, rpif[3]: {=bool:?}, rpif[4]: {=bool:?}, rpif[5]: {=bool:?}, rpif[6]: {=bool:?}, rpif[7]: {=bool:?}, rpif[8]: {=bool:?}, rpif[9]: {=bool:?}, rpif[10]: {=bool:?}, rpif[11]: {=bool:?}, rpif[12]: {=bool:?}, rpif[13]: {=bool:?}, rpif[14]: {=bool:?}, rpif[15]: {=bool:?}, rpif[16]: {=bool:?}, rpif[17]: {=bool:?}, rpif[18]: {=bool:?}, rpif[19]: {=bool:?}, rpif[20]: {=bool:?}, rpif[21]: {=bool:?}, rpif[22]: {=bool:?} }}" , self . rpif (0usize) , self . rpif (1usize) , self . rpif (2usize) , self . rpif (3usize) , self . rpif (4usize) , self . rpif (5usize) , self . rpif (6usize) , self . rpif (7usize) , self . rpif (8usize) , self . rpif (9usize) , self . rpif (10usize) , self . rpif (11usize) , self . rpif (12usize) , self . rpif (13usize) , self . rpif (14usize) , self . rpif (15usize) , self . rpif (16usize) , self . rpif (17usize) , self . rpif (18usize) , self . rpif (19usize) , self . rpif (20usize) , self . rpif (21usize) , self . rpif (22usize))
        }
    }
    #[doc = "EXTI rising trigger selection register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rtsr1(pub u32);
    impl Rtsr1 {
        #[doc = "Rising trigger event configuration bit of configurable event input i."]
        #[inline(always)]
        pub const fn rt(&self, n: usize) -> bool {
            assert!(n < 23usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Rising trigger event configuration bit of configurable event input i."]
        #[inline(always)]
        pub fn set_rt(&mut self, n: usize, val: bool) {
            assert!(n < 23usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Rtsr1 {
        #[inline(always)]
        fn default() -> Rtsr1 {
            Rtsr1(0)
        }
    }
    impl core::fmt::Debug for Rtsr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rtsr1")
                .field("rt[0]", &self.rt(0usize))
                .field("rt[1]", &self.rt(1usize))
                .field("rt[2]", &self.rt(2usize))
                .field("rt[3]", &self.rt(3usize))
                .field("rt[4]", &self.rt(4usize))
                .field("rt[5]", &self.rt(5usize))
                .field("rt[6]", &self.rt(6usize))
                .field("rt[7]", &self.rt(7usize))
                .field("rt[8]", &self.rt(8usize))
                .field("rt[9]", &self.rt(9usize))
                .field("rt[10]", &self.rt(10usize))
                .field("rt[11]", &self.rt(11usize))
                .field("rt[12]", &self.rt(12usize))
                .field("rt[13]", &self.rt(13usize))
                .field("rt[14]", &self.rt(14usize))
                .field("rt[15]", &self.rt(15usize))
                .field("rt[16]", &self.rt(16usize))
                .field("rt[17]", &self.rt(17usize))
                .field("rt[18]", &self.rt(18usize))
                .field("rt[19]", &self.rt(19usize))
                .field("rt[20]", &self.rt(20usize))
                .field("rt[21]", &self.rt(21usize))
                .field("rt[22]", &self.rt(22usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rtsr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Rtsr1 {{ rt[0]: {=bool:?}, rt[1]: {=bool:?}, rt[2]: {=bool:?}, rt[3]: {=bool:?}, rt[4]: {=bool:?}, rt[5]: {=bool:?}, rt[6]: {=bool:?}, rt[7]: {=bool:?}, rt[8]: {=bool:?}, rt[9]: {=bool:?}, rt[10]: {=bool:?}, rt[11]: {=bool:?}, rt[12]: {=bool:?}, rt[13]: {=bool:?}, rt[14]: {=bool:?}, rt[15]: {=bool:?}, rt[16]: {=bool:?}, rt[17]: {=bool:?}, rt[18]: {=bool:?}, rt[19]: {=bool:?}, rt[20]: {=bool:?}, rt[21]: {=bool:?}, rt[22]: {=bool:?} }}" , self . rt (0usize) , self . rt (1usize) , self . rt (2usize) , self . rt (3usize) , self . rt (4usize) , self . rt (5usize) , self . rt (6usize) , self . rt (7usize) , self . rt (8usize) , self . rt (9usize) , self . rt (10usize) , self . rt (11usize) , self . rt (12usize) , self . rt (13usize) , self . rt (14usize) , self . rt (15usize) , self . rt (16usize) , self . rt (17usize) , self . rt (18usize) , self . rt (19usize) , self . rt (20usize) , self . rt (21usize) , self . rt (22usize))
        }
    }
    #[doc = "EXTI security configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccfgr1(pub u32);
    impl Seccfgr1 {
        #[doc = "Security enable on event input i."]
        #[inline(always)]
        pub const fn sec(&self, n: usize) -> bool {
            assert!(n < 23usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Security enable on event input i."]
        #[inline(always)]
        pub fn set_sec(&mut self, n: usize, val: bool) {
            assert!(n < 23usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Seccfgr1 {
        #[inline(always)]
        fn default() -> Seccfgr1 {
            Seccfgr1(0)
        }
    }
    impl core::fmt::Debug for Seccfgr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Seccfgr1")
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
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Seccfgr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Seccfgr1 {{ sec[0]: {=bool:?}, sec[1]: {=bool:?}, sec[2]: {=bool:?}, sec[3]: {=bool:?}, sec[4]: {=bool:?}, sec[5]: {=bool:?}, sec[6]: {=bool:?}, sec[7]: {=bool:?}, sec[8]: {=bool:?}, sec[9]: {=bool:?}, sec[10]: {=bool:?}, sec[11]: {=bool:?}, sec[12]: {=bool:?}, sec[13]: {=bool:?}, sec[14]: {=bool:?}, sec[15]: {=bool:?}, sec[16]: {=bool:?}, sec[17]: {=bool:?}, sec[18]: {=bool:?}, sec[19]: {=bool:?}, sec[20]: {=bool:?}, sec[21]: {=bool:?}, sec[22]: {=bool:?} }}" , self . sec (0usize) , self . sec (1usize) , self . sec (2usize) , self . sec (3usize) , self . sec (4usize) , self . sec (5usize) , self . sec (6usize) , self . sec (7usize) , self . sec (8usize) , self . sec (9usize) , self . sec (10usize) , self . sec (11usize) , self . sec (12usize) , self . sec (13usize) , self . sec (14usize) , self . sec (15usize) , self . sec (16usize) , self . sec (17usize) , self . sec (18usize) , self . sec (19usize) , self . sec (20usize) , self . sec (21usize) , self . sec (22usize))
        }
    }
    #[doc = "EXTI software interrupt event register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Swier1(pub u32);
    impl Swier1 {
        #[doc = "Software interrupt on event i."]
        #[inline(always)]
        pub const fn swi(&self, n: usize) -> bool {
            assert!(n < 23usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Software interrupt on event i."]
        #[inline(always)]
        pub fn set_swi(&mut self, n: usize, val: bool) {
            assert!(n < 23usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Swier1 {
        #[inline(always)]
        fn default() -> Swier1 {
            Swier1(0)
        }
    }
    impl core::fmt::Debug for Swier1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Swier1")
                .field("swi[0]", &self.swi(0usize))
                .field("swi[1]", &self.swi(1usize))
                .field("swi[2]", &self.swi(2usize))
                .field("swi[3]", &self.swi(3usize))
                .field("swi[4]", &self.swi(4usize))
                .field("swi[5]", &self.swi(5usize))
                .field("swi[6]", &self.swi(6usize))
                .field("swi[7]", &self.swi(7usize))
                .field("swi[8]", &self.swi(8usize))
                .field("swi[9]", &self.swi(9usize))
                .field("swi[10]", &self.swi(10usize))
                .field("swi[11]", &self.swi(11usize))
                .field("swi[12]", &self.swi(12usize))
                .field("swi[13]", &self.swi(13usize))
                .field("swi[14]", &self.swi(14usize))
                .field("swi[15]", &self.swi(15usize))
                .field("swi[16]", &self.swi(16usize))
                .field("swi[17]", &self.swi(17usize))
                .field("swi[18]", &self.swi(18usize))
                .field("swi[19]", &self.swi(19usize))
                .field("swi[20]", &self.swi(20usize))
                .field("swi[21]", &self.swi(21usize))
                .field("swi[22]", &self.swi(22usize))
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Swier1 {
        fn format(&self, f: defmt::Formatter) {
            defmt :: write ! (f , "Swier1 {{ swi[0]: {=bool:?}, swi[1]: {=bool:?}, swi[2]: {=bool:?}, swi[3]: {=bool:?}, swi[4]: {=bool:?}, swi[5]: {=bool:?}, swi[6]: {=bool:?}, swi[7]: {=bool:?}, swi[8]: {=bool:?}, swi[9]: {=bool:?}, swi[10]: {=bool:?}, swi[11]: {=bool:?}, swi[12]: {=bool:?}, swi[13]: {=bool:?}, swi[14]: {=bool:?}, swi[15]: {=bool:?}, swi[16]: {=bool:?}, swi[17]: {=bool:?}, swi[18]: {=bool:?}, swi[19]: {=bool:?}, swi[20]: {=bool:?}, swi[21]: {=bool:?}, swi[22]: {=bool:?} }}" , self . swi (0usize) , self . swi (1usize) , self . swi (2usize) , self . swi (3usize) , self . swi (4usize) , self . swi (5usize) , self . swi (6usize) , self . swi (7usize) , self . swi (8usize) , self . swi (9usize) , self . swi (10usize) , self . swi (11usize) , self . swi (12usize) , self . swi (13usize) , self . swi (14usize) , self . swi (15usize) , self . swi (16usize) , self . swi (17usize) , self . swi (18usize) , self . swi (19usize) , self . swi (20usize) , self . swi (21usize) , self . swi (22usize))
        }
    }
}
