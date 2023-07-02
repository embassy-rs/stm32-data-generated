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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "master status register"]
    #[inline(always)]
    pub const fn msr(self) -> crate::common::Reg<regs::Msr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "transmit status register"]
    #[inline(always)]
    pub const fn tsr(self) -> crate::common::Reg<regs::Tsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "receive FIFO 0 register"]
    #[inline(always)]
    pub const fn rfr(self, n: usize) -> crate::common::Reg<regs::Rfr, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize + n * 4usize) as _) }
    }
    #[doc = "interrupt enable register"]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "error status register"]
    #[inline(always)]
    pub const fn esr(self) -> crate::common::Reg<regs::Esr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "bit timing register"]
    #[inline(always)]
    pub const fn btr(self) -> crate::common::Reg<regs::Btr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "CAN Transmit cluster"]
    #[inline(always)]
    pub const fn tx(self, n: usize) -> Tx {
        assert!(n < 3usize);
        unsafe { Tx::from_ptr(self.ptr.add(384usize + n * 16usize) as _) }
    }
    #[doc = "CAN Receive cluster"]
    #[inline(always)]
    pub const fn rx(self, n: usize) -> Rx {
        assert!(n < 2usize);
        unsafe { Rx::from_ptr(self.ptr.add(432usize + n * 16usize) as _) }
    }
    #[doc = "filter master register"]
    #[inline(always)]
    pub const fn fmr(self) -> crate::common::Reg<regs::Fmr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(512usize) as _) }
    }
    #[doc = "filter mode register"]
    #[inline(always)]
    pub const fn fm1r(self) -> crate::common::Reg<regs::Fm1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(516usize) as _) }
    }
    #[doc = "filter scale register"]
    #[inline(always)]
    pub const fn fs1r(self) -> crate::common::Reg<regs::Fs1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(524usize) as _) }
    }
    #[doc = "filter FIFO assignment register"]
    #[inline(always)]
    pub const fn ffa1r(self) -> crate::common::Reg<regs::Ffa1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(532usize) as _) }
    }
    #[doc = "filter activation register"]
    #[inline(always)]
    pub const fn fa1r(self) -> crate::common::Reg<regs::Fa1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(540usize) as _) }
    }
    #[doc = "CAN Filter Bank cluster"]
    #[inline(always)]
    pub const fn fb(self, n: usize) -> Fb {
        assert!(n < 28usize);
        unsafe { Fb::from_ptr(self.ptr.add(576usize + n * 8usize) as _) }
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Filter bank 0 register 2"]
    #[inline(always)]
    pub const fn fr2(self) -> crate::common::Reg<regs::Fr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "mailbox data high register"]
    #[inline(always)]
    pub const fn rdtr(self) -> crate::common::Reg<regs::Rdtr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "mailbox data high register"]
    #[inline(always)]
    pub const fn rdlr(self) -> crate::common::Reg<regs::Rdlr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "receive FIFO mailbox data high register"]
    #[inline(always)]
    pub const fn rdhr(self) -> crate::common::Reg<regs::Rdhr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "mailbox data length control and time stamp register"]
    #[inline(always)]
    pub const fn tdtr(self) -> crate::common::Reg<regs::Tdtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "mailbox data low register"]
    #[inline(always)]
    pub const fn tdlr(self) -> crate::common::Reg<regs::Tdlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "mailbox data high register"]
    #[inline(always)]
    pub const fn tdhr(self) -> crate::common::Reg<regs::Tdhr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
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
        #[doc = "LBKM"]
        #[inline(always)]
        pub const fn lbkm(&self) -> super::vals::Lbkm {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Lbkm::from_bits(val as u8)
        }
        #[doc = "LBKM"]
        #[inline(always)]
        pub fn set_lbkm(&mut self, val: super::vals::Lbkm) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
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
    #[doc = "interrupt enable register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "TMEIE"]
        #[inline(always)]
        pub const fn tmeie(&self) -> super::vals::Tmeie {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Tmeie::from_bits(val as u8)
        }
        #[doc = "TMEIE"]
        #[inline(always)]
        pub fn set_tmeie(&mut self, val: super::vals::Tmeie) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "FMPIE0"]
        #[inline(always)]
        pub const fn fmpie(&self, n: usize) -> super::vals::Fmpie {
            assert!(n < 2usize);
            let offs = 1usize + n * 3usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Fmpie::from_bits(val as u8)
        }
        #[doc = "FMPIE0"]
        #[inline(always)]
        pub fn set_fmpie(&mut self, n: usize, val: super::vals::Fmpie) {
            assert!(n < 2usize);
            let offs = 1usize + n * 3usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "FFIE0"]
        #[inline(always)]
        pub const fn ffie(&self, n: usize) -> super::vals::Ffie {
            assert!(n < 2usize);
            let offs = 2usize + n * 3usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Ffie::from_bits(val as u8)
        }
        #[doc = "FFIE0"]
        #[inline(always)]
        pub fn set_ffie(&mut self, n: usize, val: super::vals::Ffie) {
            assert!(n < 2usize);
            let offs = 2usize + n * 3usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "FOVIE0"]
        #[inline(always)]
        pub const fn fovie(&self, n: usize) -> super::vals::Fovie {
            assert!(n < 2usize);
            let offs = 3usize + n * 3usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Fovie::from_bits(val as u8)
        }
        #[doc = "FOVIE0"]
        #[inline(always)]
        pub fn set_fovie(&mut self, n: usize, val: super::vals::Fovie) {
            assert!(n < 2usize);
            let offs = 3usize + n * 3usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "EWGIE"]
        #[inline(always)]
        pub const fn ewgie(&self) -> super::vals::Ewgie {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Ewgie::from_bits(val as u8)
        }
        #[doc = "EWGIE"]
        #[inline(always)]
        pub fn set_ewgie(&mut self, val: super::vals::Ewgie) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "EPVIE"]
        #[inline(always)]
        pub const fn epvie(&self) -> super::vals::Epvie {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Epvie::from_bits(val as u8)
        }
        #[doc = "EPVIE"]
        #[inline(always)]
        pub fn set_epvie(&mut self, val: super::vals::Epvie) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "BOFIE"]
        #[inline(always)]
        pub const fn bofie(&self) -> super::vals::Bofie {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Bofie::from_bits(val as u8)
        }
        #[doc = "BOFIE"]
        #[inline(always)]
        pub fn set_bofie(&mut self, val: super::vals::Bofie) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "LECIE"]
        #[inline(always)]
        pub const fn lecie(&self) -> super::vals::Lecie {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Lecie::from_bits(val as u8)
        }
        #[doc = "LECIE"]
        #[inline(always)]
        pub fn set_lecie(&mut self, val: super::vals::Lecie) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "ERRIE"]
        #[inline(always)]
        pub const fn errie(&self) -> super::vals::Errie {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Errie::from_bits(val as u8)
        }
        #[doc = "ERRIE"]
        #[inline(always)]
        pub fn set_errie(&mut self, val: super::vals::Errie) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
        #[doc = "WKUIE"]
        #[inline(always)]
        pub const fn wkuie(&self) -> super::vals::Wkuie {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Wkuie::from_bits(val as u8)
        }
        #[doc = "WKUIE"]
        #[inline(always)]
        pub fn set_wkuie(&mut self, val: super::vals::Wkuie) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "SLKIE"]
        #[inline(always)]
        pub const fn slkie(&self) -> super::vals::Slkie {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Slkie::from_bits(val as u8)
        }
        #[doc = "SLKIE"]
        #[inline(always)]
        pub fn set_slkie(&mut self, val: super::vals::Slkie) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Ier {
        #[inline(always)]
        fn default() -> Ier {
            Ier(0)
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
    #[doc = "receive FIFO mailbox identifier register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rir(pub u32);
    impl Rir {
        #[doc = "RTR"]
        #[inline(always)]
        pub const fn rtr(&self) -> super::vals::RirRtr {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::RirRtr::from_bits(val as u8)
        }
        #[doc = "RTR"]
        #[inline(always)]
        pub fn set_rtr(&mut self, val: super::vals::RirRtr) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "IDE"]
        #[inline(always)]
        pub const fn ide(&self) -> super::vals::RirIde {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::RirIde::from_bits(val as u8)
        }
        #[doc = "IDE"]
        #[inline(always)]
        pub fn set_ide(&mut self, val: super::vals::RirIde) {
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
        pub const fn rtr(&self) -> super::vals::TirRtr {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::TirRtr::from_bits(val as u8)
        }
        #[doc = "RTR"]
        #[inline(always)]
        pub fn set_rtr(&mut self, val: super::vals::TirRtr) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "IDE"]
        #[inline(always)]
        pub const fn ide(&self) -> super::vals::TirIde {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::TirIde::from_bits(val as u8)
        }
        #[doc = "IDE"]
        #[inline(always)]
        pub fn set_ide(&mut self, val: super::vals::TirIde) {
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
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Bofie {
        #[doc = "ERRI bit will not be set when BOFF is set"]
        DISABLED = 0,
        #[doc = "ERRI bit will be set when BOFF is set"]
        ENABLED = 0x01,
    }
    impl Bofie {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bofie {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Bofie {
        #[inline(always)]
        fn from(val: u8) -> Bofie {
            Bofie::from_bits(val)
        }
    }
    impl From<Bofie> for u8 {
        #[inline(always)]
        fn from(val: Bofie) -> u8 {
            Bofie::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Epvie {
        #[doc = "ERRI bit will not be set when EPVF is set"]
        DISABLED = 0,
        #[doc = "ERRI bit will be set when EPVF is set"]
        ENABLED = 0x01,
    }
    impl Epvie {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Epvie {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Epvie {
        #[inline(always)]
        fn from(val: u8) -> Epvie {
            Epvie::from_bits(val)
        }
    }
    impl From<Epvie> for u8 {
        #[inline(always)]
        fn from(val: Epvie) -> u8 {
            Epvie::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Errie {
        #[doc = "No interrupt will be generated when an error condition is pending in the CAN_ESR"]
        DISABLED = 0,
        #[doc = "An interrupt will be generation when an error condition is pending in the CAN_ESR"]
        ENABLED = 0x01,
    }
    impl Errie {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Errie {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Errie {
        #[inline(always)]
        fn from(val: u8) -> Errie {
            Errie::from_bits(val)
        }
    }
    impl From<Errie> for u8 {
        #[inline(always)]
        fn from(val: Errie) -> u8 {
            Errie::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ewgie {
        #[doc = "ERRI bit will not be set when EWGF is set"]
        DISABLED = 0,
        #[doc = "ERRI bit will be set when EWGF is set"]
        ENABLED = 0x01,
    }
    impl Ewgie {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ewgie {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ewgie {
        #[inline(always)]
        fn from(val: u8) -> Ewgie {
            Ewgie::from_bits(val)
        }
    }
    impl From<Ewgie> for u8 {
        #[inline(always)]
        fn from(val: Ewgie) -> u8 {
            Ewgie::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ffie {
        #[doc = "No interrupt when FULL bit is set"]
        DISABLED = 0,
        #[doc = "Interrupt generated when FULL bit is set"]
        ENABLED = 0x01,
    }
    impl Ffie {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ffie {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ffie {
        #[inline(always)]
        fn from(val: u8) -> Ffie {
            Ffie::from_bits(val)
        }
    }
    impl From<Ffie> for u8 {
        #[inline(always)]
        fn from(val: Ffie) -> u8 {
            Ffie::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Fmpie {
        #[doc = "No interrupt generated when state of FMP\\[1:0\\]
bits are not 00b"]
        DISABLED = 0,
        #[doc = "Interrupt generated when state of FMP\\[1:0\\]
bits are not 00b"]
        ENABLED = 0x01,
    }
    impl Fmpie {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fmpie {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fmpie {
        #[inline(always)]
        fn from(val: u8) -> Fmpie {
            Fmpie::from_bits(val)
        }
    }
    impl From<Fmpie> for u8 {
        #[inline(always)]
        fn from(val: Fmpie) -> u8 {
            Fmpie::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Fovie {
        #[doc = "No interrupt when FOVR bit is set"]
        DISABLED = 0,
        #[doc = "Interrupt generated when FOVR bit is set"]
        ENABLED = 0x01,
    }
    impl Fovie {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fovie {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fovie {
        #[inline(always)]
        fn from(val: u8) -> Fovie {
            Fovie::from_bits(val)
        }
    }
    impl From<Fovie> for u8 {
        #[inline(always)]
        fn from(val: Fovie) -> u8 {
            Fovie::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Lbkm {
        #[doc = "Loop Back Mode disabled"]
        DISABLED = 0,
        #[doc = "Loop Back Mode enabled"]
        ENABLED = 0x01,
    }
    impl Lbkm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lbkm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lbkm {
        #[inline(always)]
        fn from(val: u8) -> Lbkm {
            Lbkm::from_bits(val)
        }
    }
    impl From<Lbkm> for u8 {
        #[inline(always)]
        fn from(val: Lbkm) -> u8 {
            Lbkm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Lec {
        #[doc = "No Error"]
        NOERROR = 0,
        #[doc = "Stuff Error"]
        STUFF = 0x01,
        #[doc = "Form Error"]
        FORM = 0x02,
        #[doc = "Acknowledgment Error"]
        ACK = 0x03,
        #[doc = "Bit recessive Error"]
        BITRECESSIVE = 0x04,
        #[doc = "Bit dominant Error"]
        BITDOMINANT = 0x05,
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
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Lecie {
        #[doc = "ERRI bit will not be set when the error code in LEC\\[2:0\\]
is set by hardware on error detection"]
        DISABLED = 0,
        #[doc = "ERRI bit will be set when the error code in LEC\\[2:0\\]
is set by hardware on error detection"]
        ENABLED = 0x01,
    }
    impl Lecie {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lecie {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lecie {
        #[inline(always)]
        fn from(val: u8) -> Lecie {
            Lecie::from_bits(val)
        }
    }
    impl From<Lecie> for u8 {
        #[inline(always)]
        fn from(val: Lecie) -> u8 {
            Lecie::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum RirIde {
        #[doc = "Standard identifier"]
        STANDARD = 0,
        #[doc = "Extended identifier"]
        EXTENDED = 0x01,
    }
    impl RirIde {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> RirIde {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for RirIde {
        #[inline(always)]
        fn from(val: u8) -> RirIde {
            RirIde::from_bits(val)
        }
    }
    impl From<RirIde> for u8 {
        #[inline(always)]
        fn from(val: RirIde) -> u8 {
            RirIde::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum RirRtr {
        #[doc = "Data frame"]
        DATA = 0,
        #[doc = "Remote frame"]
        REMOTE = 0x01,
    }
    impl RirRtr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> RirRtr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for RirRtr {
        #[inline(always)]
        fn from(val: u8) -> RirRtr {
            RirRtr::from_bits(val)
        }
    }
    impl From<RirRtr> for u8 {
        #[inline(always)]
        fn from(val: RirRtr) -> u8 {
            RirRtr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Silm {
        #[doc = "Normal operation"]
        NORMAL = 0,
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
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Slkie {
        #[doc = "No interrupt when SLAKI bit is set"]
        DISABLED = 0,
        #[doc = "Interrupt generated when SLAKI bit is set"]
        ENABLED = 0x01,
    }
    impl Slkie {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Slkie {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Slkie {
        #[inline(always)]
        fn from(val: u8) -> Slkie {
            Slkie::from_bits(val)
        }
    }
    impl From<Slkie> for u8 {
        #[inline(always)]
        fn from(val: Slkie) -> u8 {
            Slkie::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum TirIde {
        #[doc = "Standard identifier"]
        STANDARD = 0,
        #[doc = "Extended identifier"]
        EXTENDED = 0x01,
    }
    impl TirIde {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> TirIde {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for TirIde {
        #[inline(always)]
        fn from(val: u8) -> TirIde {
            TirIde::from_bits(val)
        }
    }
    impl From<TirIde> for u8 {
        #[inline(always)]
        fn from(val: TirIde) -> u8 {
            TirIde::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum TirRtr {
        #[doc = "Data frame"]
        DATA = 0,
        #[doc = "Remote frame"]
        REMOTE = 0x01,
    }
    impl TirRtr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> TirRtr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for TirRtr {
        #[inline(always)]
        fn from(val: u8) -> TirRtr {
            TirRtr::from_bits(val)
        }
    }
    impl From<TirRtr> for u8 {
        #[inline(always)]
        fn from(val: TirRtr) -> u8 {
            TirRtr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Tmeie {
        #[doc = "No interrupt when RQCPx bit is set"]
        DISABLED = 0,
        #[doc = "Interrupt generated when RQCPx bit is set"]
        ENABLED = 0x01,
    }
    impl Tmeie {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tmeie {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tmeie {
        #[inline(always)]
        fn from(val: u8) -> Tmeie {
            Tmeie::from_bits(val)
        }
    }
    impl From<Tmeie> for u8 {
        #[inline(always)]
        fn from(val: Tmeie) -> u8 {
            Tmeie::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Wkuie {
        #[doc = "No interrupt when WKUI is set"]
        DISABLED = 0,
        #[doc = "Interrupt generated when WKUI bit is set"]
        ENABLED = 0x01,
    }
    impl Wkuie {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wkuie {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wkuie {
        #[inline(always)]
        fn from(val: u8) -> Wkuie {
            Wkuie::from_bits(val)
        }
    }
    impl From<Wkuie> for u8 {
        #[inline(always)]
        fn from(val: Wkuie) -> u8 {
            Wkuie::to_bits(val)
        }
    }
}
