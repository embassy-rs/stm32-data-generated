#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Universal asynchronous receiver transmitter"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart {
    ptr: *mut u8,
}
unsafe impl Send for Uart {}
unsafe impl Sync for Uart {}
impl Uart {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Data register"]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Baud rate register"]
    #[inline(always)]
    pub const fn brr(self) -> crate::common::Reg<regs::Brr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Control register 3"]
    #[inline(always)]
    pub const fn cr3(self) -> crate::common::Reg<regs::Cr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
}
#[doc = "Universal synchronous asynchronous receiver transmitter"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usart {
    ptr: *mut u8,
}
unsafe impl Send for Usart {}
unsafe impl Sync for Usart {}
impl Usart {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Data register"]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Baud rate register"]
    #[inline(always)]
    pub const fn brr(self) -> crate::common::Reg<regs::Brr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2Usart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Control register 3"]
    #[inline(always)]
    pub const fn cr3(self) -> crate::common::Reg<regs::Cr3Usart, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Guard time and prescaler register"]
    #[inline(always)]
    pub const fn gtpr(self) -> crate::common::Reg<regs::Gtpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
}
pub mod regs {
    #[doc = "Baud rate register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Brr(pub u32);
    impl Brr {
        #[doc = "USARTDIV"]
        #[inline(always)]
        pub const fn brr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "USARTDIV"]
        #[inline(always)]
        pub fn set_brr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
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
            f.debug_struct("Brr").field("brr", &self.brr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Brr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Brr {
                brr: u16,
            }
            let proxy = Brr { brr: self.brr() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "Send break"]
        #[inline(always)]
        pub const fn sbk(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Send break"]
        #[inline(always)]
        pub fn set_sbk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Receiver wakeup"]
        #[inline(always)]
        pub const fn rwu(&self) -> super::vals::Rwu {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Rwu::from_bits(val as u8)
        }
        #[doc = "Receiver wakeup"]
        #[inline(always)]
        pub fn set_rwu(&mut self, val: super::vals::Rwu) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Receiver enable"]
        #[inline(always)]
        pub const fn re(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver enable"]
        #[inline(always)]
        pub fn set_re(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Transmitter enable"]
        #[inline(always)]
        pub const fn te(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter enable"]
        #[inline(always)]
        pub fn set_te(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "IDLE interrupt enable"]
        #[inline(always)]
        pub const fn idleie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "IDLE interrupt enable"]
        #[inline(always)]
        pub fn set_idleie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "RXNE interrupt enable"]
        #[inline(always)]
        pub const fn rxneie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RXNE interrupt enable"]
        #[inline(always)]
        pub fn set_rxneie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Transmission complete interrupt enable"]
        #[inline(always)]
        pub const fn tcie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission complete interrupt enable"]
        #[inline(always)]
        pub fn set_tcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "TXE interrupt enable"]
        #[inline(always)]
        pub const fn txeie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "TXE interrupt enable"]
        #[inline(always)]
        pub fn set_txeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "PE interrupt enable"]
        #[inline(always)]
        pub const fn peie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "PE interrupt enable"]
        #[inline(always)]
        pub fn set_peie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Parity selection"]
        #[inline(always)]
        pub const fn ps(&self) -> super::vals::Ps {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Ps::from_bits(val as u8)
        }
        #[doc = "Parity selection"]
        #[inline(always)]
        pub fn set_ps(&mut self, val: super::vals::Ps) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "Parity control enable"]
        #[inline(always)]
        pub const fn pce(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Parity control enable"]
        #[inline(always)]
        pub fn set_pce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Receiver wakeup method"]
        #[inline(always)]
        pub const fn wake(&self) -> super::vals::Wake {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Wake::from_bits(val as u8)
        }
        #[doc = "Receiver wakeup method"]
        #[inline(always)]
        pub fn set_wake(&mut self, val: super::vals::Wake) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Word length"]
        #[inline(always)]
        pub const fn m0(&self) -> super::vals::M0 {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::M0::from_bits(val as u8)
        }
        #[doc = "Word length"]
        #[inline(always)]
        pub fn set_m0(&mut self, val: super::vals::M0) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "USART enable"]
        #[inline(always)]
        pub const fn ue(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "USART enable"]
        #[inline(always)]
        pub fn set_ue(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Cr1 {
        #[inline(always)]
        fn default() -> Cr1 {
            Cr1(0)
        }
    }
    impl core::fmt::Debug for Cr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr1")
                .field("sbk", &self.sbk())
                .field("rwu", &self.rwu())
                .field("re", &self.re())
                .field("te", &self.te())
                .field("idleie", &self.idleie())
                .field("rxneie", &self.rxneie())
                .field("tcie", &self.tcie())
                .field("txeie", &self.txeie())
                .field("peie", &self.peie())
                .field("ps", &self.ps())
                .field("pce", &self.pce())
                .field("wake", &self.wake())
                .field("m0", &self.m0())
                .field("ue", &self.ue())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr1 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cr1 {
                sbk: bool,
                rwu: super::vals::Rwu,
                re: bool,
                te: bool,
                idleie: bool,
                rxneie: bool,
                tcie: bool,
                txeie: bool,
                peie: bool,
                ps: super::vals::Ps,
                pce: bool,
                wake: super::vals::Wake,
                m0: super::vals::M0,
                ue: bool,
            }
            let proxy = Cr1 {
                sbk: self.sbk(),
                rwu: self.rwu(),
                re: self.re(),
                te: self.te(),
                idleie: self.idleie(),
                rxneie: self.rxneie(),
                tcie: self.tcie(),
                txeie: self.txeie(),
                peie: self.peie(),
                ps: self.ps(),
                pce: self.pce(),
                wake: self.wake(),
                m0: self.m0(),
                ue: self.ue(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "Address of the USART node"]
        #[inline(always)]
        pub const fn add(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Address of the USART node"]
        #[inline(always)]
        pub fn set_add(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Line break detection length"]
        #[inline(always)]
        pub const fn lbdl(&self) -> super::vals::Lbdl {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Lbdl::from_bits(val as u8)
        }
        #[doc = "Line break detection length"]
        #[inline(always)]
        pub fn set_lbdl(&mut self, val: super::vals::Lbdl) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "LIN break detection interrupt enable"]
        #[inline(always)]
        pub const fn lbdie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LIN break detection interrupt enable"]
        #[inline(always)]
        pub fn set_lbdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "STOP bits"]
        #[inline(always)]
        pub const fn stop(&self) -> super::vals::Stop {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Stop::from_bits(val as u8)
        }
        #[doc = "STOP bits"]
        #[inline(always)]
        pub fn set_stop(&mut self, val: super::vals::Stop) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "LIN mode enable"]
        #[inline(always)]
        pub const fn linen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "LIN mode enable"]
        #[inline(always)]
        pub fn set_linen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Cr2 {
        #[inline(always)]
        fn default() -> Cr2 {
            Cr2(0)
        }
    }
    impl core::fmt::Debug for Cr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr2")
                .field("add", &self.add())
                .field("lbdl", &self.lbdl())
                .field("lbdie", &self.lbdie())
                .field("stop", &self.stop())
                .field("linen", &self.linen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cr2 {
                add: u8,
                lbdl: super::vals::Lbdl,
                lbdie: bool,
                stop: super::vals::Stop,
                linen: bool,
            }
            let proxy = Cr2 {
                add: self.add(),
                lbdl: self.lbdl(),
                lbdie: self.lbdie(),
                stop: self.stop(),
                linen: self.linen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2Usart(pub u32);
    impl Cr2Usart {
        #[doc = "Address of the USART node"]
        #[inline(always)]
        pub const fn add(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Address of the USART node"]
        #[inline(always)]
        pub fn set_add(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Line break detection length"]
        #[inline(always)]
        pub const fn lbdl(&self) -> super::vals::Lbdl {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Lbdl::from_bits(val as u8)
        }
        #[doc = "Line break detection length"]
        #[inline(always)]
        pub fn set_lbdl(&mut self, val: super::vals::Lbdl) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "LIN break detection interrupt enable"]
        #[inline(always)]
        pub const fn lbdie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "LIN break detection interrupt enable"]
        #[inline(always)]
        pub fn set_lbdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Last bit clock pulse"]
        #[inline(always)]
        pub const fn lbcl(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Last bit clock pulse"]
        #[inline(always)]
        pub fn set_lbcl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Clock phase"]
        #[inline(always)]
        pub const fn cpha(&self) -> super::vals::Cpha {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Cpha::from_bits(val as u8)
        }
        #[doc = "Clock phase"]
        #[inline(always)]
        pub fn set_cpha(&mut self, val: super::vals::Cpha) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "Clock polarity"]
        #[inline(always)]
        pub const fn cpol(&self) -> super::vals::Cpol {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Cpol::from_bits(val as u8)
        }
        #[doc = "Clock polarity"]
        #[inline(always)]
        pub fn set_cpol(&mut self, val: super::vals::Cpol) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "Clock enable"]
        #[inline(always)]
        pub const fn clken(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Clock enable"]
        #[inline(always)]
        pub fn set_clken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "STOP bits"]
        #[inline(always)]
        pub const fn stop(&self) -> super::vals::Stop {
            let val = (self.0 >> 12usize) & 0x03;
            super::vals::Stop::from_bits(val as u8)
        }
        #[doc = "STOP bits"]
        #[inline(always)]
        pub fn set_stop(&mut self, val: super::vals::Stop) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
        }
        #[doc = "LIN mode enable"]
        #[inline(always)]
        pub const fn linen(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "LIN mode enable"]
        #[inline(always)]
        pub fn set_linen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Cr2Usart {
        #[inline(always)]
        fn default() -> Cr2Usart {
            Cr2Usart(0)
        }
    }
    impl core::fmt::Debug for Cr2Usart {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr2Usart")
                .field("add", &self.add())
                .field("lbdl", &self.lbdl())
                .field("lbdie", &self.lbdie())
                .field("lbcl", &self.lbcl())
                .field("cpha", &self.cpha())
                .field("cpol", &self.cpol())
                .field("clken", &self.clken())
                .field("stop", &self.stop())
                .field("linen", &self.linen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr2Usart {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cr2Usart {
                add: u8,
                lbdl: super::vals::Lbdl,
                lbdie: bool,
                lbcl: bool,
                cpha: super::vals::Cpha,
                cpol: super::vals::Cpol,
                clken: bool,
                stop: super::vals::Stop,
                linen: bool,
            }
            let proxy = Cr2Usart {
                add: self.add(),
                lbdl: self.lbdl(),
                lbdie: self.lbdie(),
                lbcl: self.lbcl(),
                cpha: self.cpha(),
                cpol: self.cpol(),
                clken: self.clken(),
                stop: self.stop(),
                linen: self.linen(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Control register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr3(pub u32);
    impl Cr3 {
        #[doc = "Error interrupt enable"]
        #[inline(always)]
        pub const fn eie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt enable"]
        #[inline(always)]
        pub fn set_eie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IrDA mode enable"]
        #[inline(always)]
        pub const fn iren(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IrDA mode enable"]
        #[inline(always)]
        pub fn set_iren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IrDA low-power"]
        #[inline(always)]
        pub const fn irlp(&self) -> super::vals::Irlp {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Irlp::from_bits(val as u8)
        }
        #[doc = "IrDA low-power"]
        #[inline(always)]
        pub fn set_irlp(&mut self, val: super::vals::Irlp) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Half-duplex selection"]
        #[inline(always)]
        pub const fn hdsel(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Half-duplex selection"]
        #[inline(always)]
        pub fn set_hdsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "DMA enable receiver"]
        #[inline(always)]
        pub const fn dmar(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "DMA enable receiver"]
        #[inline(always)]
        pub fn set_dmar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "DMA enable transmitter"]
        #[inline(always)]
        pub const fn dmat(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "DMA enable transmitter"]
        #[inline(always)]
        pub fn set_dmat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for Cr3 {
        #[inline(always)]
        fn default() -> Cr3 {
            Cr3(0)
        }
    }
    impl core::fmt::Debug for Cr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr3")
                .field("eie", &self.eie())
                .field("iren", &self.iren())
                .field("irlp", &self.irlp())
                .field("hdsel", &self.hdsel())
                .field("dmar", &self.dmar())
                .field("dmat", &self.dmat())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr3 {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cr3 {
                eie: bool,
                iren: bool,
                irlp: super::vals::Irlp,
                hdsel: bool,
                dmar: bool,
                dmat: bool,
            }
            let proxy = Cr3 {
                eie: self.eie(),
                iren: self.iren(),
                irlp: self.irlp(),
                hdsel: self.hdsel(),
                dmar: self.dmar(),
                dmat: self.dmat(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Control register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr3Usart(pub u32);
    impl Cr3Usart {
        #[doc = "Error interrupt enable"]
        #[inline(always)]
        pub const fn eie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt enable"]
        #[inline(always)]
        pub fn set_eie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "IrDA mode enable"]
        #[inline(always)]
        pub const fn iren(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "IrDA mode enable"]
        #[inline(always)]
        pub fn set_iren(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "IrDA low-power"]
        #[inline(always)]
        pub const fn irlp(&self) -> super::vals::Irlp {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Irlp::from_bits(val as u8)
        }
        #[doc = "IrDA low-power"]
        #[inline(always)]
        pub fn set_irlp(&mut self, val: super::vals::Irlp) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Half-duplex selection"]
        #[inline(always)]
        pub const fn hdsel(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Half-duplex selection"]
        #[inline(always)]
        pub fn set_hdsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Smartcard NACK enable"]
        #[inline(always)]
        pub const fn nack(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Smartcard NACK enable"]
        #[inline(always)]
        pub fn set_nack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Smartcard mode enable"]
        #[inline(always)]
        pub const fn scen(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Smartcard mode enable"]
        #[inline(always)]
        pub fn set_scen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "DMA enable receiver"]
        #[inline(always)]
        pub const fn dmar(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "DMA enable receiver"]
        #[inline(always)]
        pub fn set_dmar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "DMA enable transmitter"]
        #[inline(always)]
        pub const fn dmat(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "DMA enable transmitter"]
        #[inline(always)]
        pub fn set_dmat(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "RTS enable"]
        #[inline(always)]
        pub const fn rtse(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "RTS enable"]
        #[inline(always)]
        pub fn set_rtse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CTS enable"]
        #[inline(always)]
        pub const fn ctse(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "CTS enable"]
        #[inline(always)]
        pub fn set_ctse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "CTS interrupt enable"]
        #[inline(always)]
        pub const fn ctsie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "CTS interrupt enable"]
        #[inline(always)]
        pub fn set_ctsie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Cr3Usart {
        #[inline(always)]
        fn default() -> Cr3Usart {
            Cr3Usart(0)
        }
    }
    impl core::fmt::Debug for Cr3Usart {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr3Usart")
                .field("eie", &self.eie())
                .field("iren", &self.iren())
                .field("irlp", &self.irlp())
                .field("hdsel", &self.hdsel())
                .field("nack", &self.nack())
                .field("scen", &self.scen())
                .field("dmar", &self.dmar())
                .field("dmat", &self.dmat())
                .field("rtse", &self.rtse())
                .field("ctse", &self.ctse())
                .field("ctsie", &self.ctsie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr3Usart {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Cr3Usart {
                eie: bool,
                iren: bool,
                irlp: super::vals::Irlp,
                hdsel: bool,
                nack: bool,
                scen: bool,
                dmar: bool,
                dmat: bool,
                rtse: bool,
                ctse: bool,
                ctsie: bool,
            }
            let proxy = Cr3Usart {
                eie: self.eie(),
                iren: self.iren(),
                irlp: self.irlp(),
                hdsel: self.hdsel(),
                nack: self.nack(),
                scen: self.scen(),
                dmar: self.dmar(),
                dmat: self.dmat(),
                rtse: self.rtse(),
                ctse: self.ctse(),
                ctsie: self.ctsie(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Data register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr(pub u32);
    impl Dr {
        #[doc = "Data value"]
        #[inline(always)]
        pub const fn dr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "Data value"]
        #[inline(always)]
        pub fn set_dr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Dr {
        #[inline(always)]
        fn default() -> Dr {
            Dr(0)
        }
    }
    impl core::fmt::Debug for Dr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dr").field("dr", &self.dr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Dr {
                dr: u16,
            }
            let proxy = Dr { dr: self.dr() };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Guard time and prescaler register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gtpr(pub u32);
    impl Gtpr {
        #[doc = "Prescaler value"]
        #[inline(always)]
        pub const fn psc(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Prescaler value"]
        #[inline(always)]
        pub fn set_psc(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Guard time value"]
        #[inline(always)]
        pub const fn gt(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Guard time value"]
        #[inline(always)]
        pub fn set_gt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
    }
    impl Default for Gtpr {
        #[inline(always)]
        fn default() -> Gtpr {
            Gtpr(0)
        }
    }
    impl core::fmt::Debug for Gtpr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gtpr")
                .field("psc", &self.psc())
                .field("gt", &self.gt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Gtpr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Gtpr {
                psc: u8,
                gt: u8,
            }
            let proxy = Gtpr {
                psc: self.psc(),
                gt: self.gt(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
    #[doc = "Status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Parity error"]
        #[inline(always)]
        pub const fn pe(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Parity error"]
        #[inline(always)]
        pub fn set_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Framing error"]
        #[inline(always)]
        pub const fn fe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Framing error"]
        #[inline(always)]
        pub fn set_fe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Noise error flag"]
        #[inline(always)]
        pub const fn ne(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Noise error flag"]
        #[inline(always)]
        pub fn set_ne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Overrun error"]
        #[inline(always)]
        pub const fn ore(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun error"]
        #[inline(always)]
        pub fn set_ore(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Idle line detected"]
        #[inline(always)]
        pub const fn idle(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Idle line detected"]
        #[inline(always)]
        pub fn set_idle(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Read data register not empty"]
        #[inline(always)]
        pub const fn rxne(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Read data register not empty"]
        #[inline(always)]
        pub fn set_rxne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Transmission complete"]
        #[inline(always)]
        pub const fn tc(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Transmission complete"]
        #[inline(always)]
        pub fn set_tc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Transmit data register empty"]
        #[inline(always)]
        pub const fn txe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit data register empty"]
        #[inline(always)]
        pub fn set_txe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "LIN break detection flag"]
        #[inline(always)]
        pub const fn lbd(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "LIN break detection flag"]
        #[inline(always)]
        pub fn set_lbd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CTS flag"]
        #[inline(always)]
        pub const fn cts(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "CTS flag"]
        #[inline(always)]
        pub fn set_cts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
    impl core::fmt::Debug for Sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr")
                .field("pe", &self.pe())
                .field("fe", &self.fe())
                .field("ne", &self.ne())
                .field("ore", &self.ore())
                .field("idle", &self.idle())
                .field("rxne", &self.rxne())
                .field("tc", &self.tc())
                .field("txe", &self.txe())
                .field("lbd", &self.lbd())
                .field("cts", &self.cts())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            #[derive(defmt :: Format)]
            struct Sr {
                pe: bool,
                fe: bool,
                ne: bool,
                ore: bool,
                idle: bool,
                rxne: bool,
                tc: bool,
                txe: bool,
                lbd: bool,
                cts: bool,
            }
            let proxy = Sr {
                pe: self.pe(),
                fe: self.fe(),
                ne: self.ne(),
                ore: self.ore(),
                idle: self.idle(),
                rxne: self.rxne(),
                tc: self.tc(),
                txe: self.txe(),
                lbd: self.lbd(),
                cts: self.cts(),
            };
            defmt::write!(f, "{}", proxy)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cpha {
        #[doc = "The first clock transition is the first data capture edge"]
        FIRST = 0x0,
        #[doc = "The second clock transition is the first data capture edge"]
        SECOND = 0x01,
    }
    impl Cpha {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cpha {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cpha {
        #[inline(always)]
        fn from(val: u8) -> Cpha {
            Cpha::from_bits(val)
        }
    }
    impl From<Cpha> for u8 {
        #[inline(always)]
        fn from(val: Cpha) -> u8 {
            Cpha::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Cpol {
        #[doc = "Steady low value on CK pin outside transmission window"]
        LOW = 0x0,
        #[doc = "Steady high value on CK pin outside transmission window"]
        HIGH = 0x01,
    }
    impl Cpol {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cpol {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cpol {
        #[inline(always)]
        fn from(val: u8) -> Cpol {
            Cpol::from_bits(val)
        }
    }
    impl From<Cpol> for u8 {
        #[inline(always)]
        fn from(val: Cpol) -> u8 {
            Cpol::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Irlp {
        #[doc = "Normal mode"]
        NORMAL = 0x0,
        #[doc = "Low-power mode"]
        LOW_POWER = 0x01,
    }
    impl Irlp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Irlp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Irlp {
        #[inline(always)]
        fn from(val: u8) -> Irlp {
            Irlp::from_bits(val)
        }
    }
    impl From<Irlp> for u8 {
        #[inline(always)]
        fn from(val: Irlp) -> u8 {
            Irlp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Lbdl {
        #[doc = "10-bit break detection"]
        BIT10 = 0x0,
        #[doc = "11-bit break detection"]
        BIT11 = 0x01,
    }
    impl Lbdl {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lbdl {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lbdl {
        #[inline(always)]
        fn from(val: u8) -> Lbdl {
            Lbdl::from_bits(val)
        }
    }
    impl From<Lbdl> for u8 {
        #[inline(always)]
        fn from(val: Lbdl) -> u8 {
            Lbdl::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum M0 {
        #[doc = "1 start bit, 8 data bits, n stop bits"]
        BIT8 = 0x0,
        #[doc = "1 start bit, 9 data bits, n stop bits"]
        BIT9 = 0x01,
    }
    impl M0 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> M0 {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for M0 {
        #[inline(always)]
        fn from(val: u8) -> M0 {
            M0::from_bits(val)
        }
    }
    impl From<M0> for u8 {
        #[inline(always)]
        fn from(val: M0) -> u8 {
            M0::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Ps {
        #[doc = "Even parity"]
        EVEN = 0x0,
        #[doc = "Odd parity"]
        ODD = 0x01,
    }
    impl Ps {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ps {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ps {
        #[inline(always)]
        fn from(val: u8) -> Ps {
            Ps::from_bits(val)
        }
    }
    impl From<Ps> for u8 {
        #[inline(always)]
        fn from(val: Ps) -> u8 {
            Ps::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Rwu {
        #[doc = "Receiver in active mode"]
        ACTIVE = 0x0,
        #[doc = "Receiver in mute mode"]
        MUTE = 0x01,
    }
    impl Rwu {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rwu {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rwu {
        #[inline(always)]
        fn from(val: u8) -> Rwu {
            Rwu::from_bits(val)
        }
    }
    impl From<Rwu> for u8 {
        #[inline(always)]
        fn from(val: Rwu) -> u8 {
            Rwu::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Stop {
        #[doc = "1 stop bit"]
        STOP1 = 0x0,
        #[doc = "0.5 stop bits"]
        STOP0P5 = 0x01,
        #[doc = "2 stop bits"]
        STOP2 = 0x02,
        #[doc = "1.5 stop bits"]
        STOP1P5 = 0x03,
    }
    impl Stop {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Stop {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Stop {
        #[inline(always)]
        fn from(val: u8) -> Stop {
            Stop::from_bits(val)
        }
    }
    impl From<Stop> for u8 {
        #[inline(always)]
        fn from(val: Stop) -> u8 {
            Stop::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub enum Wake {
        #[doc = "USART wakeup on idle line"]
        IDLE_LINE = 0x0,
        #[doc = "USART wakeup on address mark"]
        ADDRESS_MARK = 0x01,
    }
    impl Wake {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wake {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wake {
        #[inline(always)]
        fn from(val: u8) -> Wake {
            Wake::from_bits(val)
        }
    }
    impl From<Wake> for u8 {
        #[inline(always)]
        fn from(val: Wake) -> u8 {
            Wake::to_bits(val)
        }
    }
}
