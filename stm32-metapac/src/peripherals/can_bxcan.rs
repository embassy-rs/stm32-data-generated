#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Controller area network"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Can {
    ptr: *mut u8,
}
unsafe impl Send for Can {}
unsafe impl Sync for Can {}
impl Can {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "master control register"]
    #[inline(always)]
    pub const fn mcr(self) -> crate::common::Reg<regs::Mcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "master status register"]
    #[inline(always)]
    pub const fn msr(self) -> crate::common::Reg<regs::Msr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "transmit status register"]
    #[inline(always)]
    pub const fn tsr(self) -> crate::common::Reg<regs::Tsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "receive FIFO 0 register"]
    #[inline(always)]
    pub const fn rfr(self, n: usize) -> crate::common::Reg<regs::Rfr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize + n * 4usize) as _) }
    }
    #[doc = "interrupt enable register"]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "error status register"]
    #[inline(always)]
    pub const fn esr(self) -> crate::common::Reg<regs::Esr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "bit timing register"]
    #[inline(always)]
    pub const fn btr(self) -> crate::common::Reg<regs::Btr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "CAN Transmit cluster"]
    #[inline(always)]
    pub const fn tx(self, n: usize) -> Tx {
        assert!(n < 3usize);
        unsafe { Tx::from_ptr(self.ptr.add(0x0180usize + n * 16usize) as _) }
    }
    #[doc = "CAN Receive cluster"]
    #[inline(always)]
    pub const fn rx(self, n: usize) -> Rx {
        assert!(n < 2usize);
        unsafe { Rx::from_ptr(self.ptr.add(0x01b0usize + n * 16usize) as _) }
    }
    #[doc = "filter master register"]
    #[inline(always)]
    pub const fn fmr(self) -> crate::common::Reg<regs::Fmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[doc = "filter mode register"]
    #[inline(always)]
    pub const fn fm1r(self) -> crate::common::Reg<regs::Fm1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0204usize) as _) }
    }
    #[doc = "filter scale register"]
    #[inline(always)]
    pub const fn fs1r(self) -> crate::common::Reg<regs::Fs1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x020cusize) as _) }
    }
    #[doc = "filter FIFO assignment register"]
    #[inline(always)]
    pub const fn ffa1r(self) -> crate::common::Reg<regs::Ffa1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0214usize) as _) }
    }
    #[doc = "filter activation register"]
    #[inline(always)]
    pub const fn fa1r(self) -> crate::common::Reg<regs::Fa1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x021cusize) as _) }
    }
    #[doc = "CAN Filter Bank cluster"]
    #[inline(always)]
    pub const fn fb(self, n: usize) -> Fb {
        assert!(n < 28usize);
        unsafe { Fb::from_ptr(self.ptr.add(0x0240usize + n * 8usize) as _) }
    }
}
#[doc = "CAN Filter Bank cluster"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fb {
    ptr: *mut u8,
}
unsafe impl Send for Fb {}
unsafe impl Sync for Fb {}
impl Fb {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Filter bank 0 register 1"]
    #[inline(always)]
    pub const fn fr1(self) -> crate::common::Reg<regs::Fr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Filter bank 0 register 2"]
    #[inline(always)]
    pub const fn fr2(self) -> crate::common::Reg<regs::Fr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
#[doc = "CAN Receive cluster"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rx {
    ptr: *mut u8,
}
unsafe impl Send for Rx {}
unsafe impl Sync for Rx {}
impl Rx {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "receive FIFO mailbox identifier register"]
    #[inline(always)]
    pub const fn rir(self) -> crate::common::Reg<regs::Rir, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "mailbox data high register"]
    #[inline(always)]
    pub const fn rdtr(self) -> crate::common::Reg<regs::Rdtr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "mailbox data high register"]
    #[inline(always)]
    pub const fn rdlr(self) -> crate::common::Reg<regs::Rdlr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "receive FIFO mailbox data high register"]
    #[inline(always)]
    pub const fn rdhr(self) -> crate::common::Reg<regs::Rdhr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
#[doc = "CAN Transmit cluster"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tx {
    ptr: *mut u8,
}
unsafe impl Send for Tx {}
unsafe impl Sync for Tx {}
impl Tx {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "TX mailbox identifier register"]
    #[inline(always)]
    pub const fn tir(self) -> crate::common::Reg<regs::Tir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "mailbox data length control and time stamp register"]
    #[inline(always)]
    pub const fn tdtr(self) -> crate::common::Reg<regs::Tdtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "mailbox data low register"]
    #[inline(always)]
    pub const fn tdlr(self) -> crate::common::Reg<regs::Tdlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "mailbox data high register"]
    #[inline(always)]
    pub const fn tdhr(self) -> crate::common::Reg<regs::Tdhr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
pub mod regs {
    #[doc = "bit timing register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Btr(pub u32);
    impl Btr {
        #[doc = "BRP"]
        #[inline(always)]
        pub const fn brp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "BRP"]
        #[inline(always)]
        pub fn set_brp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "TS1"]
        #[inline(always)]
        pub const fn ts(&self, n: usize) -> u8 {
            assert!(n < 2usize);
            let offs = 16usize + n * 4usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "TS1"]
        #[inline(always)]
        pub fn set_ts(&mut self, n: usize, val: u8) {
            assert!(n < 2usize);
            let offs = 16usize + n * 4usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
        #[doc = "SJW"]
        #[inline(always)]
        pub const fn sjw(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "SJW"]
        #[inline(always)]
        pub fn set_sjw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "Loop Back Mode enabled"]
        #[inline(always)]
        pub const fn lbkm(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Loop Back Mode enabled"]
        #[inline(always)]
        pub fn set_lbkm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "SILM"]
        #[inline(always)]
        pub const fn silm(&self) -> super::vals::Silm {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Silm::from_bits(val as u8)
        }
        #[doc = "SILM"]
        #[inline(always)]
        pub fn set_silm(&mut self, val: super::vals::Silm) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Btr {
        #[inline(always)]
        fn default() -> Btr {
            Btr(0)
        }
    }
    impl core::fmt::Debug for Btr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Btr")
                .field("brp", &self.brp())
                .field("ts", &[self.ts(0usize), self.ts(1usize)])
                .field("sjw", &self.sjw())
                .field("lbkm", &self.lbkm())
                .field("silm", &self.silm())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Btr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Btr {
                brp: u16,
                ts: [u8; 2usize],
                sjw: u8,
                lbkm: bool,
                silm: super::vals::Silm,
            }
            let proxy = Btr {
                brp: self.brp(),
                ts: [self.ts(0usize), self.ts(1usize)],
                sjw: self.sjw(),
                lbkm: self.lbkm(),
                silm: self.silm(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Esr(pub u32);
    impl Esr {
        #[doc = "EWGF"]
        #[inline(always)]
        pub const fn ewgf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "EWGF"]
        #[inline(always)]
        pub fn set_ewgf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "EPVF"]
        #[inline(always)]
        pub const fn epvf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "EPVF"]
        #[inline(always)]
        pub fn set_epvf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "BOFF"]
        #[inline(always)]
        pub const fn boff(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "BOFF"]
        #[inline(always)]
        pub fn set_boff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "LEC"]
        #[inline(always)]
        pub const fn lec(&self) -> super::vals::Lec {
            let val = (self.0 >> 4usize) & 0x07;
            super::vals::Lec::from_bits(val as u8)
        }
        #[doc = "LEC"]
        #[inline(always)]
        pub fn set_lec(&mut self, val: super::vals::Lec) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
        }
        #[doc = "TEC"]
        #[inline(always)]
        pub const fn tec(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "TEC"]
        #[inline(always)]
        pub fn set_tec(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "REC"]
        #[inline(always)]
        pub const fn rec(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "REC"]
        #[inline(always)]
        pub fn set_rec(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Esr {
        #[inline(always)]
        fn default() -> Esr {
            Esr(0)
        }
    }
    impl core::fmt::Debug for Esr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Esr")
                .field("ewgf", &self.ewgf())
                .field("epvf", &self.epvf())
                .field("boff", &self.boff())
                .field("lec", &self.lec())
                .field("tec", &self.tec())
                .field("rec", &self.rec())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Esr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Esr {
                ewgf: bool,
                epvf: bool,
                boff: bool,
                lec: super::vals::Lec,
                tec: u8,
                rec: u8,
            }
            let proxy = Esr {
                ewgf: self.ewgf(),
                epvf: self.epvf(),
                boff: self.boff(),
                lec: self.lec(),
                tec: self.tec(),
                rec: self.rec(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "filter activation register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fa1r(pub u32);
    impl Fa1r {
        #[doc = "Filter active"]
        #[inline(always)]
        pub const fn fact(&self, n: usize) -> bool {
            assert!(n < 28usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Filter active"]
        #[inline(always)]
        pub fn set_fact(&mut self, n: usize, val: bool) {
            assert!(n < 28usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Fa1r {
        #[inline(always)]
        fn default() -> Fa1r {
            Fa1r(0)
        }
    }
    impl core::fmt::Debug for Fa1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fa1r")
                .field(
                    "fact",
                    &[
                        self.fact(0usize),
                        self.fact(1usize),
                        self.fact(2usize),
                        self.fact(3usize),
                        self.fact(4usize),
                        self.fact(5usize),
                        self.fact(6usize),
                        self.fact(7usize),
                        self.fact(8usize),
                        self.fact(9usize),
                        self.fact(10usize),
                        self.fact(11usize),
                        self.fact(12usize),
                        self.fact(13usize),
                        self.fact(14usize),
                        self.fact(15usize),
                        self.fact(16usize),
                        self.fact(17usize),
                        self.fact(18usize),
                        self.fact(19usize),
                        self.fact(20usize),
                        self.fact(21usize),
                        self.fact(22usize),
                        self.fact(23usize),
                        self.fact(24usize),
                        self.fact(25usize),
                        self.fact(26usize),
                        self.fact(27usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fa1r {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Fa1r {
                fact: [bool; 28usize],
            }
            let proxy = Fa1r {
                fact: [
                    self.fact(0usize),
                    self.fact(1usize),
                    self.fact(2usize),
                    self.fact(3usize),
                    self.fact(4usize),
                    self.fact(5usize),
                    self.fact(6usize),
                    self.fact(7usize),
                    self.fact(8usize),
                    self.fact(9usize),
                    self.fact(10usize),
                    self.fact(11usize),
                    self.fact(12usize),
                    self.fact(13usize),
                    self.fact(14usize),
                    self.fact(15usize),
                    self.fact(16usize),
                    self.fact(17usize),
                    self.fact(18usize),
                    self.fact(19usize),
                    self.fact(20usize),
                    self.fact(21usize),
                    self.fact(22usize),
                    self.fact(23usize),
                    self.fact(24usize),
                    self.fact(25usize),
                    self.fact(26usize),
                    self.fact(27usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "filter FIFO assignment register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ffa1r(pub u32);
    impl Ffa1r {
        #[doc = "Filter FIFO assignment for filter 0"]
        #[inline(always)]
        pub const fn ffa(&self, n: usize) -> bool {
            assert!(n < 28usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Filter FIFO assignment for filter 0"]
        #[inline(always)]
        pub fn set_ffa(&mut self, n: usize, val: bool) {
            assert!(n < 28usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Ffa1r {
        #[inline(always)]
        fn default() -> Ffa1r {
            Ffa1r(0)
        }
    }
    impl core::fmt::Debug for Ffa1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ffa1r")
                .field(
                    "ffa",
                    &[
                        self.ffa(0usize),
                        self.ffa(1usize),
                        self.ffa(2usize),
                        self.ffa(3usize),
                        self.ffa(4usize),
                        self.ffa(5usize),
                        self.ffa(6usize),
                        self.ffa(7usize),
                        self.ffa(8usize),
                        self.ffa(9usize),
                        self.ffa(10usize),
                        self.ffa(11usize),
                        self.ffa(12usize),
                        self.ffa(13usize),
                        self.ffa(14usize),
                        self.ffa(15usize),
                        self.ffa(16usize),
                        self.ffa(17usize),
                        self.ffa(18usize),
                        self.ffa(19usize),
                        self.ffa(20usize),
                        self.ffa(21usize),
                        self.ffa(22usize),
                        self.ffa(23usize),
                        self.ffa(24usize),
                        self.ffa(25usize),
                        self.ffa(26usize),
                        self.ffa(27usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ffa1r {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ffa1r {
                ffa: [bool; 28usize],
            }
            let proxy = Ffa1r {
                ffa: [
                    self.ffa(0usize),
                    self.ffa(1usize),
                    self.ffa(2usize),
                    self.ffa(3usize),
                    self.ffa(4usize),
                    self.ffa(5usize),
                    self.ffa(6usize),
                    self.ffa(7usize),
                    self.ffa(8usize),
                    self.ffa(9usize),
                    self.ffa(10usize),
                    self.ffa(11usize),
                    self.ffa(12usize),
                    self.ffa(13usize),
                    self.ffa(14usize),
                    self.ffa(15usize),
                    self.ffa(16usize),
                    self.ffa(17usize),
                    self.ffa(18usize),
                    self.ffa(19usize),
                    self.ffa(20usize),
                    self.ffa(21usize),
                    self.ffa(22usize),
                    self.ffa(23usize),
                    self.ffa(24usize),
                    self.ffa(25usize),
                    self.ffa(26usize),
                    self.ffa(27usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "filter mode register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fm1r(pub u32);
    impl Fm1r {
        #[doc = "Filter mode"]
        #[inline(always)]
        pub const fn fbm(&self, n: usize) -> bool {
            assert!(n < 28usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Filter mode"]
        #[inline(always)]
        pub fn set_fbm(&mut self, n: usize, val: bool) {
            assert!(n < 28usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Fm1r {
        #[inline(always)]
        fn default() -> Fm1r {
            Fm1r(0)
        }
    }
    impl core::fmt::Debug for Fm1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fm1r")
                .field(
                    "fbm",
                    &[
                        self.fbm(0usize),
                        self.fbm(1usize),
                        self.fbm(2usize),
                        self.fbm(3usize),
                        self.fbm(4usize),
                        self.fbm(5usize),
                        self.fbm(6usize),
                        self.fbm(7usize),
                        self.fbm(8usize),
                        self.fbm(9usize),
                        self.fbm(10usize),
                        self.fbm(11usize),
                        self.fbm(12usize),
                        self.fbm(13usize),
                        self.fbm(14usize),
                        self.fbm(15usize),
                        self.fbm(16usize),
                        self.fbm(17usize),
                        self.fbm(18usize),
                        self.fbm(19usize),
                        self.fbm(20usize),
                        self.fbm(21usize),
                        self.fbm(22usize),
                        self.fbm(23usize),
                        self.fbm(24usize),
                        self.fbm(25usize),
                        self.fbm(26usize),
                        self.fbm(27usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fm1r {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Fm1r {
                fbm: [bool; 28usize],
            }
            let proxy = Fm1r {
                fbm: [
                    self.fbm(0usize),
                    self.fbm(1usize),
                    self.fbm(2usize),
                    self.fbm(3usize),
                    self.fbm(4usize),
                    self.fbm(5usize),
                    self.fbm(6usize),
                    self.fbm(7usize),
                    self.fbm(8usize),
                    self.fbm(9usize),
                    self.fbm(10usize),
                    self.fbm(11usize),
                    self.fbm(12usize),
                    self.fbm(13usize),
                    self.fbm(14usize),
                    self.fbm(15usize),
                    self.fbm(16usize),
                    self.fbm(17usize),
                    self.fbm(18usize),
                    self.fbm(19usize),
                    self.fbm(20usize),
                    self.fbm(21usize),
                    self.fbm(22usize),
                    self.fbm(23usize),
                    self.fbm(24usize),
                    self.fbm(25usize),
                    self.fbm(26usize),
                    self.fbm(27usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "filter master register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fmr(pub u32);
    impl Fmr {
        #[doc = "FINIT"]
        #[inline(always)]
        pub const fn finit(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "FINIT"]
        #[inline(always)]
        pub fn set_finit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "CAN2SB"]
        #[inline(always)]
        pub const fn can2sb(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x3f;
            val as u8
        }
        #[doc = "CAN2SB"]
        #[inline(always)]
        pub fn set_can2sb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
        }
    }
    impl Default for Fmr {
        #[inline(always)]
        fn default() -> Fmr {
            Fmr(0)
        }
    }
    impl core::fmt::Debug for Fmr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fmr")
                .field("finit", &self.finit())
                .field("can2sb", &self.can2sb())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fmr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Fmr {
                finit: bool,
                can2sb: u8,
            }
            let proxy = Fmr {
                finit: self.finit(),
                can2sb: self.can2sb(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Filter bank 0 register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fr1(pub u32);
    impl Fr1 {
        #[doc = "Filter bits"]
        #[inline(always)]
        pub const fn fb(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Filter bits"]
        #[inline(always)]
        pub fn set_fb(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Fr1 {
        #[inline(always)]
        fn default() -> Fr1 {
            Fr1(0)
        }
    }
    impl core::fmt::Debug for Fr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fr1")
                .field(
                    "fb",
                    &[
                        self.fb(0usize),
                        self.fb(1usize),
                        self.fb(2usize),
                        self.fb(3usize),
                        self.fb(4usize),
                        self.fb(5usize),
                        self.fb(6usize),
                        self.fb(7usize),
                        self.fb(8usize),
                        self.fb(9usize),
                        self.fb(10usize),
                        self.fb(11usize),
                        self.fb(12usize),
                        self.fb(13usize),
                        self.fb(14usize),
                        self.fb(15usize),
                        self.fb(16usize),
                        self.fb(17usize),
                        self.fb(18usize),
                        self.fb(19usize),
                        self.fb(20usize),
                        self.fb(21usize),
                        self.fb(22usize),
                        self.fb(23usize),
                        self.fb(24usize),
                        self.fb(25usize),
                        self.fb(26usize),
                        self.fb(27usize),
                        self.fb(28usize),
                        self.fb(29usize),
                        self.fb(30usize),
                        self.fb(31usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fr1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Fr1 {
                fb: [bool; 32usize],
            }
            let proxy = Fr1 {
                fb: [
                    self.fb(0usize),
                    self.fb(1usize),
                    self.fb(2usize),
                    self.fb(3usize),
                    self.fb(4usize),
                    self.fb(5usize),
                    self.fb(6usize),
                    self.fb(7usize),
                    self.fb(8usize),
                    self.fb(9usize),
                    self.fb(10usize),
                    self.fb(11usize),
                    self.fb(12usize),
                    self.fb(13usize),
                    self.fb(14usize),
                    self.fb(15usize),
                    self.fb(16usize),
                    self.fb(17usize),
                    self.fb(18usize),
                    self.fb(19usize),
                    self.fb(20usize),
                    self.fb(21usize),
                    self.fb(22usize),
                    self.fb(23usize),
                    self.fb(24usize),
                    self.fb(25usize),
                    self.fb(26usize),
                    self.fb(27usize),
                    self.fb(28usize),
                    self.fb(29usize),
                    self.fb(30usize),
                    self.fb(31usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Filter bank 0 register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fr2(pub u32);
    impl Fr2 {
        #[doc = "Filter bits"]
        #[inline(always)]
        pub const fn fb(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Filter bits"]
        #[inline(always)]
        pub fn set_fb(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Fr2 {
        #[inline(always)]
        fn default() -> Fr2 {
            Fr2(0)
        }
    }
    impl core::fmt::Debug for Fr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fr2")
                .field(
                    "fb",
                    &[
                        self.fb(0usize),
                        self.fb(1usize),
                        self.fb(2usize),
                        self.fb(3usize),
                        self.fb(4usize),
                        self.fb(5usize),
                        self.fb(6usize),
                        self.fb(7usize),
                        self.fb(8usize),
                        self.fb(9usize),
                        self.fb(10usize),
                        self.fb(11usize),
                        self.fb(12usize),
                        self.fb(13usize),
                        self.fb(14usize),
                        self.fb(15usize),
                        self.fb(16usize),
                        self.fb(17usize),
                        self.fb(18usize),
                        self.fb(19usize),
                        self.fb(20usize),
                        self.fb(21usize),
                        self.fb(22usize),
                        self.fb(23usize),
                        self.fb(24usize),
                        self.fb(25usize),
                        self.fb(26usize),
                        self.fb(27usize),
                        self.fb(28usize),
                        self.fb(29usize),
                        self.fb(30usize),
                        self.fb(31usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fr2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Fr2 {
                fb: [bool; 32usize],
            }
            let proxy = Fr2 {
                fb: [
                    self.fb(0usize),
                    self.fb(1usize),
                    self.fb(2usize),
                    self.fb(3usize),
                    self.fb(4usize),
                    self.fb(5usize),
                    self.fb(6usize),
                    self.fb(7usize),
                    self.fb(8usize),
                    self.fb(9usize),
                    self.fb(10usize),
                    self.fb(11usize),
                    self.fb(12usize),
                    self.fb(13usize),
                    self.fb(14usize),
                    self.fb(15usize),
                    self.fb(16usize),
                    self.fb(17usize),
                    self.fb(18usize),
                    self.fb(19usize),
                    self.fb(20usize),
                    self.fb(21usize),
                    self.fb(22usize),
                    self.fb(23usize),
                    self.fb(24usize),
                    self.fb(25usize),
                    self.fb(26usize),
                    self.fb(27usize),
                    self.fb(28usize),
                    self.fb(29usize),
                    self.fb(30usize),
                    self.fb(31usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "filter scale register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fs1r(pub u32);
    impl Fs1r {
        #[doc = "Filter scale configuration"]
        #[inline(always)]
        pub const fn fsc(&self, n: usize) -> bool {
            assert!(n < 28usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Filter scale configuration"]
        #[inline(always)]
        pub fn set_fsc(&mut self, n: usize, val: bool) {
            assert!(n < 28usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Fs1r {
        #[inline(always)]
        fn default() -> Fs1r {
            Fs1r(0)
        }
    }
    impl core::fmt::Debug for Fs1r {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fs1r")
                .field(
                    "fsc",
                    &[
                        self.fsc(0usize),
                        self.fsc(1usize),
                        self.fsc(2usize),
                        self.fsc(3usize),
                        self.fsc(4usize),
                        self.fsc(5usize),
                        self.fsc(6usize),
                        self.fsc(7usize),
                        self.fsc(8usize),
                        self.fsc(9usize),
                        self.fsc(10usize),
                        self.fsc(11usize),
                        self.fsc(12usize),
                        self.fsc(13usize),
                        self.fsc(14usize),
                        self.fsc(15usize),
                        self.fsc(16usize),
                        self.fsc(17usize),
                        self.fsc(18usize),
                        self.fsc(19usize),
                        self.fsc(20usize),
                        self.fsc(21usize),
                        self.fsc(22usize),
                        self.fsc(23usize),
                        self.fsc(24usize),
                        self.fsc(25usize),
                        self.fsc(26usize),
                        self.fsc(27usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Fs1r {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Fs1r {
                fsc: [bool; 28usize],
            }
            let proxy = Fs1r {
                fsc: [
                    self.fsc(0usize),
                    self.fsc(1usize),
                    self.fsc(2usize),
                    self.fsc(3usize),
                    self.fsc(4usize),
                    self.fsc(5usize),
                    self.fsc(6usize),
                    self.fsc(7usize),
                    self.fsc(8usize),
                    self.fsc(9usize),
                    self.fsc(10usize),
                    self.fsc(11usize),
                    self.fsc(12usize),
                    self.fsc(13usize),
                    self.fsc(14usize),
                    self.fsc(15usize),
                    self.fsc(16usize),
                    self.fsc(17usize),
                    self.fsc(18usize),
                    self.fsc(19usize),
                    self.fsc(20usize),
                    self.fsc(21usize),
                    self.fsc(22usize),
                    self.fsc(23usize),
                    self.fsc(24usize),
                    self.fsc(25usize),
                    self.fsc(26usize),
                    self.fsc(27usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "TMEIE"]
        #[inline(always)]
        pub const fn tmeie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TMEIE"]
        #[inline(always)]
        pub fn set_tmeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "FMPIE0"]
        #[inline(always)]
        pub const fn fmpie(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 1usize + n * 3usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "FMPIE0"]
        #[inline(always)]
        pub fn set_fmpie(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 1usize + n * 3usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "FFIE0"]
        #[inline(always)]
        pub const fn ffie(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 2usize + n * 3usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "FFIE0"]
        #[inline(always)]
        pub fn set_ffie(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 2usize + n * 3usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "FOVIE0"]
        #[inline(always)]
        pub const fn fovie(&self, n: usize) -> bool {
            assert!(n < 2usize);
            let offs = 3usize + n * 3usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "FOVIE0"]
        #[inline(always)]
        pub fn set_fovie(&mut self, n: usize, val: bool) {
            assert!(n < 2usize);
            let offs = 3usize + n * 3usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "EWGIE"]
        #[inline(always)]
        pub const fn ewgie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "EWGIE"]
        #[inline(always)]
        pub fn set_ewgie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "EPVIE"]
        #[inline(always)]
        pub const fn epvie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "EPVIE"]
        #[inline(always)]
        pub fn set_epvie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "BOFIE"]
        #[inline(always)]
        pub const fn bofie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "BOFIE"]
        #[inline(always)]
        pub fn set_bofie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "LECIE"]
        #[inline(always)]
        pub const fn lecie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "LECIE"]
        #[inline(always)]
        pub fn set_lecie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "ERRIE"]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "ERRIE"]
        #[inline(always)]
        pub fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "WKUIE"]
        #[inline(always)]
        pub const fn wkuie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "WKUIE"]
        #[inline(always)]
        pub fn set_wkuie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "SLKIE"]
        #[inline(always)]
        pub const fn slkie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "SLKIE"]
        #[inline(always)]
        pub fn set_slkie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Ier {
        #[inline(always)]
        fn default() -> Ier {
            Ier(0)
        }
    }
    impl core::fmt::Debug for Ier {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ier")
                .field("tmeie", &self.tmeie())
                .field("fmpie", &[self.fmpie(0usize), self.fmpie(1usize)])
                .field("ffie", &[self.ffie(0usize), self.ffie(1usize)])
                .field("fovie", &[self.fovie(0usize), self.fovie(1usize)])
                .field("ewgie", &self.ewgie())
                .field("epvie", &self.epvie())
                .field("bofie", &self.bofie())
                .field("lecie", &self.lecie())
                .field("errie", &self.errie())
                .field("wkuie", &self.wkuie())
                .field("slkie", &self.slkie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ier {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Ier {
                tmeie: bool,
                fmpie: [bool; 2usize],
                ffie: [bool; 2usize],
                fovie: [bool; 2usize],
                ewgie: bool,
                epvie: bool,
                bofie: bool,
                lecie: bool,
                errie: bool,
                wkuie: bool,
                slkie: bool,
            }
            let proxy = Ier {
                tmeie: self.tmeie(),
                fmpie: [self.fmpie(0usize), self.fmpie(1usize)],
                ffie: [self.ffie(0usize), self.ffie(1usize)],
                fovie: [self.fovie(0usize), self.fovie(1usize)],
                ewgie: self.ewgie(),
                epvie: self.epvie(),
                bofie: self.bofie(),
                lecie: self.lecie(),
                errie: self.errie(),
                wkuie: self.wkuie(),
                slkie: self.slkie(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "master control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mcr(pub u32);
    impl Mcr {
        #[doc = "INRQ"]
        #[inline(always)]
        pub const fn inrq(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "INRQ"]
        #[inline(always)]
        pub fn set_inrq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SLEEP"]
        #[inline(always)]
        pub const fn sleep(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SLEEP"]
        #[inline(always)]
        pub fn set_sleep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "TXFP"]
        #[inline(always)]
        pub const fn txfp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TXFP"]
        #[inline(always)]
        pub fn set_txfp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "RFLM"]
        #[inline(always)]
        pub const fn rflm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RFLM"]
        #[inline(always)]
        pub fn set_rflm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "NART"]
        #[inline(always)]
        pub const fn nart(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "NART"]
        #[inline(always)]
        pub fn set_nart(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "AWUM"]
        #[inline(always)]
        pub const fn awum(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "AWUM"]
        #[inline(always)]
        pub fn set_awum(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "ABOM"]
        #[inline(always)]
        pub const fn abom(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "ABOM"]
        #[inline(always)]
        pub fn set_abom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TTCM"]
        #[inline(always)]
        pub const fn ttcm(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TTCM"]
        #[inline(always)]
        pub fn set_ttcm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "RESET"]
        #[inline(always)]
        pub const fn reset(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "RESET"]
        #[inline(always)]
        pub fn set_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "DBF"]
        #[inline(always)]
        pub const fn dbf(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "DBF"]
        #[inline(always)]
        pub fn set_dbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Mcr {
        #[inline(always)]
        fn default() -> Mcr {
            Mcr(0)
        }
    }
    impl core::fmt::Debug for Mcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Mcr")
                .field("inrq", &self.inrq())
                .field("sleep", &self.sleep())
                .field("txfp", &self.txfp())
                .field("rflm", &self.rflm())
                .field("nart", &self.nart())
                .field("awum", &self.awum())
                .field("abom", &self.abom())
                .field("ttcm", &self.ttcm())
                .field("reset", &self.reset())
                .field("dbf", &self.dbf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Mcr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Mcr {
                inrq: bool,
                sleep: bool,
                txfp: bool,
                rflm: bool,
                nart: bool,
                awum: bool,
                abom: bool,
                ttcm: bool,
                reset: bool,
                dbf: bool,
            }
            let proxy = Mcr {
                inrq: self.inrq(),
                sleep: self.sleep(),
                txfp: self.txfp(),
                rflm: self.rflm(),
                nart: self.nart(),
                awum: self.awum(),
                abom: self.abom(),
                ttcm: self.ttcm(),
                reset: self.reset(),
                dbf: self.dbf(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "master status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Msr(pub u32);
    impl Msr {
        #[doc = "INAK"]
        #[inline(always)]
        pub const fn inak(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "INAK"]
        #[inline(always)]
        pub fn set_inak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "SLAK"]
        #[inline(always)]
        pub const fn slak(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "SLAK"]
        #[inline(always)]
        pub fn set_slak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "ERRI"]
        #[inline(always)]
        pub const fn erri(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "ERRI"]
        #[inline(always)]
        pub fn set_erri(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "WKUI"]
        #[inline(always)]
        pub const fn wkui(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "WKUI"]
        #[inline(always)]
        pub fn set_wkui(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "SLAKI"]
        #[inline(always)]
        pub const fn slaki(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "SLAKI"]
        #[inline(always)]
        pub fn set_slaki(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "TXM"]
        #[inline(always)]
        pub const fn txm(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TXM"]
        #[inline(always)]
        pub fn set_txm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "RXM"]
        #[inline(always)]
        pub const fn rxm(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "RXM"]
        #[inline(always)]
        pub fn set_rxm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "SAMP"]
        #[inline(always)]
        pub const fn samp(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "SAMP"]
        #[inline(always)]
        pub fn set_samp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "RX"]
        #[inline(always)]
        pub const fn rx(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "RX"]
        #[inline(always)]
        pub fn set_rx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
    }
    impl Default for Msr {
        #[inline(always)]
        fn default() -> Msr {
            Msr(0)
        }
    }
    impl core::fmt::Debug for Msr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Msr")
                .field("inak", &self.inak())
                .field("slak", &self.slak())
                .field("erri", &self.erri())
                .field("wkui", &self.wkui())
                .field("slaki", &self.slaki())
                .field("txm", &self.txm())
                .field("rxm", &self.rxm())
                .field("samp", &self.samp())
                .field("rx", &self.rx())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Msr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Msr {
                inak: bool,
                slak: bool,
                erri: bool,
                wkui: bool,
                slaki: bool,
                txm: bool,
                rxm: bool,
                samp: bool,
                rx: bool,
            }
            let proxy = Msr {
                inak: self.inak(),
                slak: self.slak(),
                erri: self.erri(),
                wkui: self.wkui(),
                slaki: self.slaki(),
                txm: self.txm(),
                rxm: self.rxm(),
                samp: self.samp(),
                rx: self.rx(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "receive FIFO mailbox data high register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rdhr(pub u32);
    impl Rdhr {
        #[doc = "DATA4"]
        #[inline(always)]
        pub const fn data(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0xff;
            val as u8
        }
        #[doc = "DATA4"]
        #[inline(always)]
        pub fn set_data(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0xff << offs)) | (((val as u32) & 0xff) << offs);
        }
    }
    impl Default for Rdhr {
        #[inline(always)]
        fn default() -> Rdhr {
            Rdhr(0)
        }
    }
    impl core::fmt::Debug for Rdhr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rdhr")
                .field(
                    "data",
                    &[
                        self.data(0usize),
                        self.data(1usize),
                        self.data(2usize),
                        self.data(3usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rdhr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Rdhr {
                data: [u8; 4usize],
            }
            let proxy = Rdhr {
                data: [
                    self.data(0usize),
                    self.data(1usize),
                    self.data(2usize),
                    self.data(3usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "mailbox data high register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rdlr(pub u32);
    impl Rdlr {
        #[doc = "DATA0"]
        #[inline(always)]
        pub const fn data(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0xff;
            val as u8
        }
        #[doc = "DATA0"]
        #[inline(always)]
        pub fn set_data(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0xff << offs)) | (((val as u32) & 0xff) << offs);
        }
    }
    impl Default for Rdlr {
        #[inline(always)]
        fn default() -> Rdlr {
            Rdlr(0)
        }
    }
    impl core::fmt::Debug for Rdlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rdlr")
                .field(
                    "data",
                    &[
                        self.data(0usize),
                        self.data(1usize),
                        self.data(2usize),
                        self.data(3usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rdlr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Rdlr {
                data: [u8; 4usize],
            }
            let proxy = Rdlr {
                data: [
                    self.data(0usize),
                    self.data(1usize),
                    self.data(2usize),
                    self.data(3usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "mailbox data high register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rdtr(pub u32);
    impl Rdtr {
        #[doc = "DLC"]
        #[inline(always)]
        pub const fn dlc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DLC"]
        #[inline(always)]
        pub fn set_dlc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "FMI"]
        #[inline(always)]
        pub const fn fmi(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "FMI"]
        #[inline(always)]
        pub fn set_fmi(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "TIME"]
        #[inline(always)]
        pub const fn time(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "TIME"]
        #[inline(always)]
        pub fn set_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Rdtr {
        #[inline(always)]
        fn default() -> Rdtr {
            Rdtr(0)
        }
    }
    impl core::fmt::Debug for Rdtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rdtr")
                .field("dlc", &self.dlc())
                .field("fmi", &self.fmi())
                .field("time", &self.time())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rdtr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Rdtr {
                dlc: u8,
                fmi: u8,
                time: u16,
            }
            let proxy = Rdtr {
                dlc: self.dlc(),
                fmi: self.fmi(),
                time: self.time(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "receive FIFO 0 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rfr(pub u32);
    impl Rfr {
        #[doc = "FMP0"]
        #[inline(always)]
        pub const fn fmp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "FMP0"]
        #[inline(always)]
        pub fn set_fmp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "FULL0"]
        #[inline(always)]
        pub const fn full(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "FULL0"]
        #[inline(always)]
        pub fn set_full(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "FOVR0"]
        #[inline(always)]
        pub const fn fovr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "FOVR0"]
        #[inline(always)]
        pub fn set_fovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "RFOM0"]
        #[inline(always)]
        pub const fn rfom(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RFOM0"]
        #[inline(always)]
        pub fn set_rfom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Rfr {
        #[inline(always)]
        fn default() -> Rfr {
            Rfr(0)
        }
    }
    impl core::fmt::Debug for Rfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rfr")
                .field("fmp", &self.fmp())
                .field("full", &self.full())
                .field("fovr", &self.fovr())
                .field("rfom", &self.rfom())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rfr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Rfr {
                fmp: u8,
                full: bool,
                fovr: bool,
                rfom: bool,
            }
            let proxy = Rfr {
                fmp: self.fmp(),
                full: self.full(),
                fovr: self.fovr(),
                rfom: self.rfom(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "receive FIFO mailbox identifier register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rir(pub u32);
    impl Rir {
        #[doc = "RTR"]
        #[inline(always)]
        pub const fn rtr(&self) -> super::vals::Rtr {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Rtr::from_bits(val as u8)
        }
        #[doc = "RTR"]
        #[inline(always)]
        pub fn set_rtr(&mut self, val: super::vals::Rtr) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "IDE"]
        #[inline(always)]
        pub const fn ide(&self) -> super::vals::Ide {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Ide::from_bits(val as u8)
        }
        #[doc = "IDE"]
        #[inline(always)]
        pub fn set_ide(&mut self, val: super::vals::Ide) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "EXID"]
        #[inline(always)]
        pub const fn exid(&self) -> u32 {
            let val = (self.0 >> 3usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "EXID"]
        #[inline(always)]
        pub fn set_exid(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 3usize)) | (((val as u32) & 0x0003_ffff) << 3usize);
        }
        #[doc = "STID"]
        #[inline(always)]
        pub const fn stid(&self) -> u16 {
            let val = (self.0 >> 21usize) & 0x07ff;
            val as u16
        }
        #[doc = "STID"]
        #[inline(always)]
        pub fn set_stid(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 21usize)) | (((val as u32) & 0x07ff) << 21usize);
        }
    }
    impl Default for Rir {
        #[inline(always)]
        fn default() -> Rir {
            Rir(0)
        }
    }
    impl core::fmt::Debug for Rir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Rir")
                .field("rtr", &self.rtr())
                .field("ide", &self.ide())
                .field("exid", &self.exid())
                .field("stid", &self.stid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Rir {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Rir {
                rtr: super::vals::Rtr,
                ide: super::vals::Ide,
                exid: u32,
                stid: u16,
            }
            let proxy = Rir {
                rtr: self.rtr(),
                ide: self.ide(),
                exid: self.exid(),
                stid: self.stid(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "mailbox data high register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tdhr(pub u32);
    impl Tdhr {
        #[doc = "DATA4"]
        #[inline(always)]
        pub const fn data(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0xff;
            val as u8
        }
        #[doc = "DATA4"]
        #[inline(always)]
        pub fn set_data(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0xff << offs)) | (((val as u32) & 0xff) << offs);
        }
    }
    impl Default for Tdhr {
        #[inline(always)]
        fn default() -> Tdhr {
            Tdhr(0)
        }
    }
    impl core::fmt::Debug for Tdhr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tdhr")
                .field(
                    "data",
                    &[
                        self.data(0usize),
                        self.data(1usize),
                        self.data(2usize),
                        self.data(3usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tdhr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Tdhr {
                data: [u8; 4usize],
            }
            let proxy = Tdhr {
                data: [
                    self.data(0usize),
                    self.data(1usize),
                    self.data(2usize),
                    self.data(3usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "mailbox data low register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tdlr(pub u32);
    impl Tdlr {
        #[doc = "DATA0"]
        #[inline(always)]
        pub const fn data(&self, n: usize) -> u8 {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0xff;
            val as u8
        }
        #[doc = "DATA0"]
        #[inline(always)]
        pub fn set_data(&mut self, n: usize, val: u8) {
            assert!(n < 4usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0xff << offs)) | (((val as u32) & 0xff) << offs);
        }
    }
    impl Default for Tdlr {
        #[inline(always)]
        fn default() -> Tdlr {
            Tdlr(0)
        }
    }
    impl core::fmt::Debug for Tdlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tdlr")
                .field(
                    "data",
                    &[
                        self.data(0usize),
                        self.data(1usize),
                        self.data(2usize),
                        self.data(3usize),
                    ],
                )
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tdlr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Tdlr {
                data: [u8; 4usize],
            }
            let proxy = Tdlr {
                data: [
                    self.data(0usize),
                    self.data(1usize),
                    self.data(2usize),
                    self.data(3usize),
                ],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "mailbox data length control and time stamp register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tdtr(pub u32);
    impl Tdtr {
        #[doc = "DLC"]
        #[inline(always)]
        pub const fn dlc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "DLC"]
        #[inline(always)]
        pub fn set_dlc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "TGT"]
        #[inline(always)]
        pub const fn tgt(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "TGT"]
        #[inline(always)]
        pub fn set_tgt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "TIME"]
        #[inline(always)]
        pub const fn time(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "TIME"]
        #[inline(always)]
        pub fn set_time(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Tdtr {
        #[inline(always)]
        fn default() -> Tdtr {
            Tdtr(0)
        }
    }
    impl core::fmt::Debug for Tdtr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tdtr")
                .field("dlc", &self.dlc())
                .field("tgt", &self.tgt())
                .field("time", &self.time())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tdtr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Tdtr {
                dlc: u8,
                tgt: bool,
                time: u16,
            }
            let proxy = Tdtr {
                dlc: self.dlc(),
                tgt: self.tgt(),
                time: self.time(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "TX mailbox identifier register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tir(pub u32);
    impl Tir {
        #[doc = "TXRQ"]
        #[inline(always)]
        pub const fn txrq(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "TXRQ"]
        #[inline(always)]
        pub fn set_txrq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RTR"]
        #[inline(always)]
        pub const fn rtr(&self) -> super::vals::Rtr {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Rtr::from_bits(val as u8)
        }
        #[doc = "RTR"]
        #[inline(always)]
        pub fn set_rtr(&mut self, val: super::vals::Rtr) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "IDE"]
        #[inline(always)]
        pub const fn ide(&self) -> super::vals::Ide {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Ide::from_bits(val as u8)
        }
        #[doc = "IDE"]
        #[inline(always)]
        pub fn set_ide(&mut self, val: super::vals::Ide) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "EXID"]
        #[inline(always)]
        pub const fn exid(&self) -> u32 {
            let val = (self.0 >> 3usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "EXID"]
        #[inline(always)]
        pub fn set_exid(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 3usize)) | (((val as u32) & 0x0003_ffff) << 3usize);
        }
        #[doc = "STID"]
        #[inline(always)]
        pub const fn stid(&self) -> u16 {
            let val = (self.0 >> 21usize) & 0x07ff;
            val as u16
        }
        #[doc = "STID"]
        #[inline(always)]
        pub fn set_stid(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 21usize)) | (((val as u32) & 0x07ff) << 21usize);
        }
    }
    impl Default for Tir {
        #[inline(always)]
        fn default() -> Tir {
            Tir(0)
        }
    }
    impl core::fmt::Debug for Tir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tir")
                .field("txrq", &self.txrq())
                .field("rtr", &self.rtr())
                .field("ide", &self.ide())
                .field("exid", &self.exid())
                .field("stid", &self.stid())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tir {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Tir {
                txrq: bool,
                rtr: super::vals::Rtr,
                ide: super::vals::Ide,
                exid: u32,
                stid: u16,
            }
            let proxy = Tir {
                txrq: self.txrq(),
                rtr: self.rtr(),
                ide: self.ide(),
                exid: self.exid(),
                stid: self.stid(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "transmit status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Tsr(pub u32);
    impl Tsr {
        #[doc = "RQCP0"]
        #[inline(always)]
        pub const fn rqcp(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 0usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "RQCP0"]
        #[inline(always)]
        pub fn set_rqcp(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 0usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "TXOK0"]
        #[inline(always)]
        pub const fn txok(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 1usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "TXOK0"]
        #[inline(always)]
        pub fn set_txok(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 1usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "ALST0"]
        #[inline(always)]
        pub const fn alst(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 2usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "ALST0"]
        #[inline(always)]
        pub fn set_alst(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 2usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "TERR0"]
        #[inline(always)]
        pub const fn terr(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 3usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "TERR0"]
        #[inline(always)]
        pub fn set_terr(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 3usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "ABRQ0"]
        #[inline(always)]
        pub const fn abrq(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 7usize + n * 8usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "ABRQ0"]
        #[inline(always)]
        pub fn set_abrq(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 7usize + n * 8usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "CODE"]
        #[inline(always)]
        pub const fn code(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x03;
            val as u8
        }
        #[doc = "CODE"]
        #[inline(always)]
        pub fn set_code(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
        }
        #[doc = "Lowest priority flag for mailbox 0"]
        #[inline(always)]
        pub const fn tme(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 26usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Lowest priority flag for mailbox 0"]
        #[inline(always)]
        pub fn set_tme(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 26usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Lowest priority flag for mailbox 0"]
        #[inline(always)]
        pub const fn low(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 29usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Lowest priority flag for mailbox 0"]
        #[inline(always)]
        pub fn set_low(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 29usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Tsr {
        #[inline(always)]
        fn default() -> Tsr {
            Tsr(0)
        }
    }
    impl core::fmt::Debug for Tsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Tsr")
                .field("rqcp", &[self.rqcp(0usize), self.rqcp(1usize), self.rqcp(2usize)])
                .field("txok", &[self.txok(0usize), self.txok(1usize), self.txok(2usize)])
                .field("alst", &[self.alst(0usize), self.alst(1usize), self.alst(2usize)])
                .field("terr", &[self.terr(0usize), self.terr(1usize), self.terr(2usize)])
                .field("abrq", &[self.abrq(0usize), self.abrq(1usize), self.abrq(2usize)])
                .field("code", &self.code())
                .field("tme", &[self.tme(0usize), self.tme(1usize), self.tme(2usize)])
                .field("low", &[self.low(0usize), self.low(1usize), self.low(2usize)])
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Tsr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Tsr {
                rqcp: [bool; 3usize],
                txok: [bool; 3usize],
                alst: [bool; 3usize],
                terr: [bool; 3usize],
                abrq: [bool; 3usize],
                code: u8,
                tme: [bool; 3usize],
                low: [bool; 3usize],
            }
            let proxy = Tsr {
                rqcp: [self.rqcp(0usize), self.rqcp(1usize), self.rqcp(2usize)],
                txok: [self.txok(0usize), self.txok(1usize), self.txok(2usize)],
                alst: [self.alst(0usize), self.alst(1usize), self.alst(2usize)],
                terr: [self.terr(0usize), self.terr(1usize), self.terr(2usize)],
                abrq: [self.abrq(0usize), self.abrq(1usize), self.abrq(2usize)],
                code: self.code(),
                tme: [self.tme(0usize), self.tme(1usize), self.tme(2usize)],
                low: [self.low(0usize), self.low(1usize), self.low(2usize)],
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ide {
        #[doc = "Standard identifier"]
        STANDARD = 0x0,
        #[doc = "Extended identifier"]
        EXTENDED = 0x01,
    }
    impl Ide {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ide {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ide {
        #[inline(always)]
        fn from(val: u8) -> Ide {
            Ide::from_bits(val)
        }
    }
    impl From<Ide> for u8 {
        #[inline(always)]
        fn from(val: Ide) -> u8 {
            Ide::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lec {
        #[doc = "No Error"]
        NO_ERROR = 0x0,
        #[doc = "Stuff Error"]
        STUFF = 0x01,
        #[doc = "Form Error"]
        FORM = 0x02,
        #[doc = "Acknowledgment Error"]
        ACK = 0x03,
        #[doc = "Bit recessive Error"]
        BIT_RECESSIVE = 0x04,
        #[doc = "Bit dominant Error"]
        BIT_DOMINANT = 0x05,
        #[doc = "CRC Error"]
        CRC = 0x06,
        #[doc = "Set by software"]
        CUSTOM = 0x07,
    }
    impl Lec {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lec {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lec {
        #[inline(always)]
        fn from(val: u8) -> Lec {
            Lec::from_bits(val)
        }
    }
    impl From<Lec> for u8 {
        #[inline(always)]
        fn from(val: Lec) -> u8 {
            Lec::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rtr {
        #[doc = "Data frame"]
        DATA = 0x0,
        #[doc = "Remote frame"]
        REMOTE = 0x01,
    }
    impl Rtr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rtr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rtr {
        #[inline(always)]
        fn from(val: u8) -> Rtr {
            Rtr::from_bits(val)
        }
    }
    impl From<Rtr> for u8 {
        #[inline(always)]
        fn from(val: Rtr) -> u8 {
            Rtr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Silm {
        #[doc = "Normal operation"]
        NORMAL = 0x0,
        #[doc = "Silent Mode"]
        SILENT = 0x01,
    }
    impl Silm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Silm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Silm {
        #[inline(always)]
        fn from(val: u8) -> Silm {
            Silm::from_bits(val)
        }
    }
    impl From<Silm> for u8 {
        #[inline(always)]
        fn from(val: Silm) -> u8 {
            Silm::to_bits(val)
        }
    }
}
