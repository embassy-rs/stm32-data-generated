#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Flash"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flash {
    ptr: *mut u8,
}
unsafe impl Send for Flash {}
unsafe impl Sync for Flash {}
impl Flash {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Access control register"]
    #[inline(always)]
    pub const fn acr(self) -> crate::common::Reg<regs::Acr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Flash key register"]
    #[inline(always)]
    pub const fn keyr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Option byte key register"]
    #[inline(always)]
    pub const fn optkeyr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Flash control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Flash ECC register"]
    #[inline(always)]
    pub const fn eccr(self) -> crate::common::Reg<regs::Eccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Flash option register"]
    #[inline(always)]
    pub const fn optr(self) -> crate::common::Reg<regs::Optr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Flash Bank 1 PCROP Start address zone A register"]
    #[inline(always)]
    pub const fn pcrop1asr(self) -> crate::common::Reg<regs::Pcrop1asr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Flash Bank 1 PCROP End address zone A register"]
    #[inline(always)]
    pub const fn pcrop1aer(self) -> crate::common::Reg<regs::Pcrop1aer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Flash Bank 1 WRP area A address register"]
    #[inline(always)]
    pub const fn wrp1ar(self) -> crate::common::Reg<regs::Wrp1ar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Flash Bank 1 WRP area B address register"]
    #[inline(always)]
    pub const fn wrp1br(self) -> crate::common::Reg<regs::Wrp1br, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Flash Bank 1 PCROP Start address area B register"]
    #[inline(always)]
    pub const fn pcrop1bsr(self) -> crate::common::Reg<regs::Pcrop1bsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Flash Bank 1 PCROP End address area B register"]
    #[inline(always)]
    pub const fn pcrop1ber(self) -> crate::common::Reg<regs::Pcrop1ber, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "IPCC mailbox data buffer address register"]
    #[inline(always)]
    pub const fn ipccbr(self) -> crate::common::Reg<regs::Ipccbr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "CPU2 cortex M0 access control register"]
    #[inline(always)]
    pub const fn c2acr(self) -> crate::common::Reg<regs::C2acr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "CPU2 cortex M0 status register"]
    #[inline(always)]
    pub const fn c2sr(self) -> crate::common::Reg<regs::C2sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "CPU2 cortex M0 control register"]
    #[inline(always)]
    pub const fn c2cr(self) -> crate::common::Reg<regs::C2cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "Secure flash start address register"]
    #[inline(always)]
    pub const fn sfr(self) -> crate::common::Reg<regs::Sfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "Secure SRAM2 start address and cortex M0 reset vector register"]
    #[inline(always)]
    pub const fn srrvr(self) -> crate::common::Reg<regs::Srrvr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
}
pub mod regs {
    #[doc = "Access control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Acr(pub u32);
    impl Acr {
        #[doc = "Latency"]
        #[inline(always)]
        pub const fn latency(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Latency"]
        #[inline(always)]
        pub fn set_latency(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Prefetch enable"]
        #[inline(always)]
        pub const fn prften(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Prefetch enable"]
        #[inline(always)]
        pub fn set_prften(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Instruction cache enable"]
        #[inline(always)]
        pub const fn icen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction cache enable"]
        #[inline(always)]
        pub fn set_icen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Data cache enable"]
        #[inline(always)]
        pub const fn dcen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Data cache enable"]
        #[inline(always)]
        pub fn set_dcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Instruction cache reset"]
        #[inline(always)]
        pub const fn icrst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Instruction cache reset"]
        #[inline(always)]
        pub fn set_icrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Data cache reset"]
        #[inline(always)]
        pub const fn dcrst(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Data cache reset"]
        #[inline(always)]
        pub fn set_dcrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "CPU1 CortexM4 program erase suspend request"]
        #[inline(always)]
        pub const fn pes(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "CPU1 CortexM4 program erase suspend request"]
        #[inline(always)]
        pub fn set_pes(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Flash User area empty"]
        #[inline(always)]
        pub const fn empty(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Flash User area empty"]
        #[inline(always)]
        pub fn set_empty(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Acr {
        #[inline(always)]
        fn default() -> Acr {
            Acr(0)
        }
    }
    #[doc = "CPU2 cortex M0 access control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C2acr(pub u32);
    impl C2acr {
        #[doc = "CPU2 cortex M0 prefetch enable"]
        #[inline(always)]
        pub const fn prften(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "CPU2 cortex M0 prefetch enable"]
        #[inline(always)]
        pub fn set_prften(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "CPU2 cortex M0 instruction cache enable"]
        #[inline(always)]
        pub const fn icen(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "CPU2 cortex M0 instruction cache enable"]
        #[inline(always)]
        pub fn set_icen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "CPU2 cortex M0 instruction cache reset"]
        #[inline(always)]
        pub const fn icrst(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "CPU2 cortex M0 instruction cache reset"]
        #[inline(always)]
        pub fn set_icrst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "CPU2 cortex M0 program erase suspend request"]
        #[inline(always)]
        pub const fn pes(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "CPU2 cortex M0 program erase suspend request"]
        #[inline(always)]
        pub fn set_pes(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for C2acr {
        #[inline(always)]
        fn default() -> C2acr {
            C2acr(0)
        }
    }
    #[doc = "CPU2 cortex M0 control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C2cr(pub u32);
    impl C2cr {
        #[doc = "Programming"]
        #[inline(always)]
        pub const fn pg(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Programming"]
        #[inline(always)]
        pub fn set_pg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Page erase"]
        #[inline(always)]
        pub const fn per(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Page erase"]
        #[inline(always)]
        pub fn set_per(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Masse erase"]
        #[inline(always)]
        pub const fn mer(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Masse erase"]
        #[inline(always)]
        pub fn set_mer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Page Number selection"]
        #[inline(always)]
        pub const fn pnb(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0xff;
            val as u8
        }
        #[doc = "Page Number selection"]
        #[inline(always)]
        pub fn set_pnb(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 3usize)) | (((val as u32) & 0xff) << 3usize);
        }
        #[doc = "Start"]
        #[inline(always)]
        pub const fn strt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Start"]
        #[inline(always)]
        pub fn set_strt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Fast programming"]
        #[inline(always)]
        pub const fn fstpg(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Fast programming"]
        #[inline(always)]
        pub fn set_fstpg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "End of operation interrupt enable"]
        #[inline(always)]
        pub const fn eopie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "End of operation interrupt enable"]
        #[inline(always)]
        pub fn set_eopie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Error interrupt enable"]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt enable"]
        #[inline(always)]
        pub fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "PCROP read error interrupt enable"]
        #[inline(always)]
        pub const fn rderrie(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "PCROP read error interrupt enable"]
        #[inline(always)]
        pub fn set_rderrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
    }
    impl Default for C2cr {
        #[inline(always)]
        fn default() -> C2cr {
            C2cr(0)
        }
    }
    #[doc = "CPU2 cortex M0 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C2sr(pub u32);
    impl C2sr {
        #[doc = "End of operation"]
        #[inline(always)]
        pub const fn eop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "End of operation"]
        #[inline(always)]
        pub fn set_eop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Operation error"]
        #[inline(always)]
        pub const fn operr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Operation error"]
        #[inline(always)]
        pub fn set_operr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Programming error"]
        #[inline(always)]
        pub const fn progerr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Programming error"]
        #[inline(always)]
        pub fn set_progerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "write protection error"]
        #[inline(always)]
        pub const fn wrperr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "write protection error"]
        #[inline(always)]
        pub fn set_wrperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Programming alignment error"]
        #[inline(always)]
        pub const fn pgaerr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Programming alignment error"]
        #[inline(always)]
        pub fn set_pgaerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Size error"]
        #[inline(always)]
        pub const fn sizerr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Size error"]
        #[inline(always)]
        pub fn set_sizerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Programming sequence error"]
        #[inline(always)]
        pub const fn pgserr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Programming sequence error"]
        #[inline(always)]
        pub fn set_pgserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Fast programming data miss error"]
        #[inline(always)]
        pub const fn misserr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Fast programming data miss error"]
        #[inline(always)]
        pub fn set_misserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Fast programming error"]
        #[inline(always)]
        pub const fn fasterr(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Fast programming error"]
        #[inline(always)]
        pub fn set_fasterr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "PCROP read error"]
        #[inline(always)]
        pub const fn rderr(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "PCROP read error"]
        #[inline(always)]
        pub fn set_rderr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Busy"]
        #[inline(always)]
        pub const fn bsy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Busy"]
        #[inline(always)]
        pub fn set_bsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Programming or erase configuration busy"]
        #[inline(always)]
        pub const fn cfgbsy(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Programming or erase configuration busy"]
        #[inline(always)]
        pub fn set_cfgbsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Programming or erase operation suspended"]
        #[inline(always)]
        pub const fn pesd(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Programming or erase operation suspended"]
        #[inline(always)]
        pub fn set_pesd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for C2sr {
        #[inline(always)]
        fn default() -> C2sr {
            C2sr(0)
        }
    }
    #[doc = "Flash control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Programming"]
        #[inline(always)]
        pub const fn pg(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Programming"]
        #[inline(always)]
        pub fn set_pg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Page erase"]
        #[inline(always)]
        pub const fn per(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Page erase"]
        #[inline(always)]
        pub fn set_per(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "This bit triggers the mass erase (all user pages) when set"]
        #[inline(always)]
        pub const fn mer(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "This bit triggers the mass erase (all user pages) when set"]
        #[inline(always)]
        pub fn set_mer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Page number selection"]
        #[inline(always)]
        pub const fn pnb(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0xff;
            val as u8
        }
        #[doc = "Page number selection"]
        #[inline(always)]
        pub fn set_pnb(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 3usize)) | (((val as u32) & 0xff) << 3usize);
        }
        #[doc = "Start"]
        #[inline(always)]
        pub const fn strt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Start"]
        #[inline(always)]
        pub fn set_strt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Options modification start"]
        #[inline(always)]
        pub const fn optstrt(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Options modification start"]
        #[inline(always)]
        pub fn set_optstrt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Fast programming"]
        #[inline(always)]
        pub const fn fstpg(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Fast programming"]
        #[inline(always)]
        pub fn set_fstpg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "End of operation interrupt enable"]
        #[inline(always)]
        pub const fn eopie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "End of operation interrupt enable"]
        #[inline(always)]
        pub fn set_eopie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Error interrupt enable"]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Error interrupt enable"]
        #[inline(always)]
        pub fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "PCROP read error interrupt enable"]
        #[inline(always)]
        pub const fn rderrie(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "PCROP read error interrupt enable"]
        #[inline(always)]
        pub fn set_rderrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Force the option byte loading"]
        #[inline(always)]
        pub const fn obl_launch(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Force the option byte loading"]
        #[inline(always)]
        pub fn set_obl_launch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Options Lock"]
        #[inline(always)]
        pub const fn optlock(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Options Lock"]
        #[inline(always)]
        pub fn set_optlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "FLASH_CR Lock"]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "FLASH_CR Lock"]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    #[doc = "Flash ECC register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccr(pub u32);
    impl Eccr {
        #[doc = "ECC fail address"]
        #[inline(always)]
        pub const fn addr_ecc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "ECC fail address"]
        #[inline(always)]
        pub fn set_addr_ecc(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
        #[doc = "System Flash ECC fail"]
        #[inline(always)]
        pub const fn sysf_ecc(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "System Flash ECC fail"]
        #[inline(always)]
        pub fn set_sysf_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "ECC correction interrupt enable"]
        #[inline(always)]
        pub const fn ecccie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ECC correction interrupt enable"]
        #[inline(always)]
        pub fn set_ecccie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "CPU identification"]
        #[inline(always)]
        pub const fn cpuid(&self) -> u8 {
            let val = (self.0 >> 26usize) & 0x07;
            val as u8
        }
        #[doc = "CPU identification"]
        #[inline(always)]
        pub fn set_cpuid(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 26usize)) | (((val as u32) & 0x07) << 26usize);
        }
        #[doc = "ECC correction"]
        #[inline(always)]
        pub const fn eccc(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "ECC correction"]
        #[inline(always)]
        pub fn set_eccc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "ECC detection"]
        #[inline(always)]
        pub const fn eccd(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "ECC detection"]
        #[inline(always)]
        pub fn set_eccd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Eccr {
        #[inline(always)]
        fn default() -> Eccr {
            Eccr(0)
        }
    }
    #[doc = "IPCC mailbox data buffer address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipccbr(pub u32);
    impl Ipccbr {
        #[doc = "PCC mailbox data buffer base address"]
        #[inline(always)]
        pub const fn ipccdba(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x3fff;
            val as u16
        }
        #[doc = "PCC mailbox data buffer base address"]
        #[inline(always)]
        pub fn set_ipccdba(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
        }
    }
    impl Default for Ipccbr {
        #[inline(always)]
        fn default() -> Ipccbr {
            Ipccbr(0)
        }
    }
    #[doc = "Flash option register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optr(pub u32);
    impl Optr {
        #[doc = "Read protection level"]
        #[inline(always)]
        pub const fn rdp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Read protection level"]
        #[inline(always)]
        pub fn set_rdp(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Security enabled"]
        #[inline(always)]
        pub const fn ese(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Security enabled"]
        #[inline(always)]
        pub fn set_ese(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "BOR reset Level"]
        #[inline(always)]
        pub const fn bor_lev(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x07;
            val as u8
        }
        #[doc = "BOR reset Level"]
        #[inline(always)]
        pub fn set_bor_lev(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
        }
        #[doc = "nRST_STOP"]
        #[inline(always)]
        pub const fn n_rst_stop(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "nRST_STOP"]
        #[inline(always)]
        pub fn set_n_rst_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "nRST_STDBY"]
        #[inline(always)]
        pub const fn n_rst_stdby(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "nRST_STDBY"]
        #[inline(always)]
        pub fn set_n_rst_stdby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "nRST_SHDW"]
        #[inline(always)]
        pub const fn n_rst_shdw(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "nRST_SHDW"]
        #[inline(always)]
        pub fn set_n_rst_shdw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Independent watchdog selection"]
        #[inline(always)]
        pub const fn idwg_sw(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog selection"]
        #[inline(always)]
        pub fn set_idwg_sw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Independent watchdog counter freeze in Stop mode"]
        #[inline(always)]
        pub const fn iwdg_stop(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog counter freeze in Stop mode"]
        #[inline(always)]
        pub fn set_iwdg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Independent watchdog counter freeze in Standby mode"]
        #[inline(always)]
        pub const fn iwdg_stdby(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog counter freeze in Standby mode"]
        #[inline(always)]
        pub fn set_iwdg_stdby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Window watchdog selection"]
        #[inline(always)]
        pub const fn wwdg_sw(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog selection"]
        #[inline(always)]
        pub fn set_wwdg_sw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Boot configuration"]
        #[inline(always)]
        pub const fn n_boot1(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Boot configuration"]
        #[inline(always)]
        pub fn set_n_boot1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "SRAM2 parity check enable"]
        #[inline(always)]
        pub const fn sram2_pe(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 parity check enable"]
        #[inline(always)]
        pub fn set_sram2_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "SRAM2 Erase when system reset"]
        #[inline(always)]
        pub const fn sram2_rst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 Erase when system reset"]
        #[inline(always)]
        pub fn set_sram2_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Software Boot0"]
        #[inline(always)]
        pub const fn n_swboot0(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Software Boot0"]
        #[inline(always)]
        pub fn set_n_swboot0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "nBoot0 option bit"]
        #[inline(always)]
        pub const fn n_boot0(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "nBoot0 option bit"]
        #[inline(always)]
        pub fn set_n_boot0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Radio Automatic Gain Control Trimming"]
        #[inline(always)]
        pub const fn agc_trim(&self) -> u8 {
            let val = (self.0 >> 29usize) & 0x07;
            val as u8
        }
        #[doc = "Radio Automatic Gain Control Trimming"]
        #[inline(always)]
        pub fn set_agc_trim(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
        }
    }
    impl Default for Optr {
        #[inline(always)]
        fn default() -> Optr {
            Optr(0)
        }
    }
    #[doc = "Flash Bank 1 PCROP End address zone A register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrop1aer(pub u32);
    impl Pcrop1aer {
        #[doc = "Bank 1 PCROP area end offset"]
        #[inline(always)]
        pub const fn pcrop1a_end(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "Bank 1 PCROP area end offset"]
        #[inline(always)]
        pub fn set_pcrop1a_end(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "PCROP area preserved when RDP level decreased"]
        #[inline(always)]
        pub const fn pcrop_rdp(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "PCROP area preserved when RDP level decreased"]
        #[inline(always)]
        pub fn set_pcrop_rdp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Pcrop1aer {
        #[inline(always)]
        fn default() -> Pcrop1aer {
            Pcrop1aer(0)
        }
    }
    #[doc = "Flash Bank 1 PCROP Start address zone A register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrop1asr(pub u32);
    impl Pcrop1asr {
        #[doc = "Bank 1 PCROPQ area start offset"]
        #[inline(always)]
        pub const fn pcrop1a_strt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "Bank 1 PCROPQ area start offset"]
        #[inline(always)]
        pub fn set_pcrop1a_strt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Pcrop1asr {
        #[inline(always)]
        fn default() -> Pcrop1asr {
            Pcrop1asr(0)
        }
    }
    #[doc = "Flash Bank 1 PCROP End address area B register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrop1ber(pub u32);
    impl Pcrop1ber {
        #[doc = "Bank 1 PCROP area end area B offset"]
        #[inline(always)]
        pub const fn pcrop1b_end(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "Bank 1 PCROP area end area B offset"]
        #[inline(always)]
        pub fn set_pcrop1b_end(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Pcrop1ber {
        #[inline(always)]
        fn default() -> Pcrop1ber {
            Pcrop1ber(0)
        }
    }
    #[doc = "Flash Bank 1 PCROP Start address area B register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcrop1bsr(pub u32);
    impl Pcrop1bsr {
        #[doc = "Bank 1 PCROP area B start offset"]
        #[inline(always)]
        pub const fn pcrop1b_strt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "Bank 1 PCROP area B start offset"]
        #[inline(always)]
        pub fn set_pcrop1b_strt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
    }
    impl Default for Pcrop1bsr {
        #[inline(always)]
        fn default() -> Pcrop1bsr {
            Pcrop1bsr(0)
        }
    }
    #[doc = "Secure flash start address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sfr(pub u32);
    impl Sfr {
        #[doc = "Secure flash start address"]
        #[inline(always)]
        pub const fn sfsa(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Secure flash start address"]
        #[inline(always)]
        pub fn set_sfsa(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Flash security disable"]
        #[inline(always)]
        pub const fn fsd(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Flash security disable"]
        #[inline(always)]
        pub fn set_fsd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Disable Cortex M0 debug access"]
        #[inline(always)]
        pub const fn dds(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Disable Cortex M0 debug access"]
        #[inline(always)]
        pub fn set_dds(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
    }
    impl Default for Sfr {
        #[inline(always)]
        fn default() -> Sfr {
            Sfr(0)
        }
    }
    #[doc = "Status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "End of operation"]
        #[inline(always)]
        pub const fn eop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "End of operation"]
        #[inline(always)]
        pub fn set_eop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Operation error"]
        #[inline(always)]
        pub const fn operr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Operation error"]
        #[inline(always)]
        pub fn set_operr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Programming error"]
        #[inline(always)]
        pub const fn progerr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Programming error"]
        #[inline(always)]
        pub fn set_progerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Write protected error"]
        #[inline(always)]
        pub const fn wrperr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Write protected error"]
        #[inline(always)]
        pub fn set_wrperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Programming alignment error"]
        #[inline(always)]
        pub const fn pgaerr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Programming alignment error"]
        #[inline(always)]
        pub fn set_pgaerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Size error"]
        #[inline(always)]
        pub const fn sizerr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Size error"]
        #[inline(always)]
        pub fn set_sizerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Programming sequence error"]
        #[inline(always)]
        pub const fn pgserr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Programming sequence error"]
        #[inline(always)]
        pub fn set_pgserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Fast programming data miss error"]
        #[inline(always)]
        pub const fn miserr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Fast programming data miss error"]
        #[inline(always)]
        pub fn set_miserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Fast programming error"]
        #[inline(always)]
        pub const fn fasterr(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Fast programming error"]
        #[inline(always)]
        pub fn set_fasterr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "User Option OPTVAL indication"]
        #[inline(always)]
        pub const fn optnv(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "User Option OPTVAL indication"]
        #[inline(always)]
        pub fn set_optnv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "PCROP read error"]
        #[inline(always)]
        pub const fn rderr(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "PCROP read error"]
        #[inline(always)]
        pub fn set_rderr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Option validity error"]
        #[inline(always)]
        pub const fn optverr(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Option validity error"]
        #[inline(always)]
        pub fn set_optverr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Busy"]
        #[inline(always)]
        pub const fn bsy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Busy"]
        #[inline(always)]
        pub fn set_bsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Programming or erase configuration busy"]
        #[inline(always)]
        pub const fn cfgbsy(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Programming or erase configuration busy"]
        #[inline(always)]
        pub fn set_cfgbsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Programming or erase operation suspended"]
        #[inline(always)]
        pub const fn pesd(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Programming or erase operation suspended"]
        #[inline(always)]
        pub fn set_pesd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
    #[doc = "Secure SRAM2 start address and cortex M0 reset vector register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Srrvr(pub u32);
    impl Srrvr {
        #[doc = "cortex M0 access control register"]
        #[inline(always)]
        pub const fn sbrv(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "cortex M0 access control register"]
        #[inline(always)]
        pub fn set_sbrv(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
        }
        #[doc = "Secure backup SRAM2a start address"]
        #[inline(always)]
        pub const fn sbrsa(&self) -> u8 {
            let val = (self.0 >> 18usize) & 0x1f;
            val as u8
        }
        #[doc = "Secure backup SRAM2a start address"]
        #[inline(always)]
        pub fn set_sbrsa(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 18usize)) | (((val as u32) & 0x1f) << 18usize);
        }
        #[doc = "backup SRAM2a security disable"]
        #[inline(always)]
        pub const fn brsd(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "backup SRAM2a security disable"]
        #[inline(always)]
        pub fn set_brsd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Secure non backup SRAM2a start address"]
        #[inline(always)]
        pub const fn snbrsa(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x1f;
            val as u8
        }
        #[doc = "Secure non backup SRAM2a start address"]
        #[inline(always)]
        pub fn set_snbrsa(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 25usize)) | (((val as u32) & 0x1f) << 25usize);
        }
        #[doc = "non-backup SRAM2b security disable"]
        #[inline(always)]
        pub const fn nbrsd(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "non-backup SRAM2b security disable"]
        #[inline(always)]
        pub fn set_nbrsd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "CPU2 cortex M0 boot reset vector memory selection"]
        #[inline(always)]
        pub const fn c2opt(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "CPU2 cortex M0 boot reset vector memory selection"]
        #[inline(always)]
        pub fn set_c2opt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Srrvr {
        #[inline(always)]
        fn default() -> Srrvr {
            Srrvr(0)
        }
    }
    #[doc = "Flash Bank 1 WRP area A address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrp1ar(pub u32);
    impl Wrp1ar {
        #[doc = "Bank 1 WRP first area A start offset"]
        #[inline(always)]
        pub const fn wrp1a_strt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Bank 1 WRP first area A start offset"]
        #[inline(always)]
        pub fn set_wrp1a_strt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Bank 1 WRP first area A end offset"]
        #[inline(always)]
        pub const fn wrp1a_end(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Bank 1 WRP first area A end offset"]
        #[inline(always)]
        pub fn set_wrp1a_end(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Wrp1ar {
        #[inline(always)]
        fn default() -> Wrp1ar {
            Wrp1ar(0)
        }
    }
    #[doc = "Flash Bank 1 WRP area B address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrp1br(pub u32);
    impl Wrp1br {
        #[doc = "Bank 1 WRP second area B start offset"]
        #[inline(always)]
        pub const fn wrp1b_end(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Bank 1 WRP second area B start offset"]
        #[inline(always)]
        pub fn set_wrp1b_end(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Bank 1 WRP second area B end offset"]
        #[inline(always)]
        pub const fn wrp1b_strt(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Bank 1 WRP second area B end offset"]
        #[inline(always)]
        pub fn set_wrp1b_strt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Wrp1br {
        #[inline(always)]
        fn default() -> Wrp1br {
            Wrp1br(0)
        }
    }
}
